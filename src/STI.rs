use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::fs::{File, read};
use fltk::image::RgbImage;
use fltk::enums::ColorDepth;
use glob::{glob, glob_with};

// Sir-Tech's Crazy Image (STCI) file format specifications.	Each file is composed of:
// 1		ImageFileHeader, uncompressed
// *		Palette (STCI_INDEXED, size = uiNumberOfColours * PALETTE_ELEMENT_SIZE), uncompressed
// *		SubRectInfo's (usNumberOfRects > 0, size = usNumberOfSubRects * sizeof(SubRectInfo) ), uncompressed
// *		Bytes of image data, possibly compressed
const STCI_ID_STRING: &[u8] =  b"STCI";
const STCI_ID_LEN: u32 = 4;

const STCI_ETRLE_COMPRESSED: u32 = 0x0020;
const STCI_ZLIB_COMPRESSED: u32 = 0x0010;
const STCI_INDEXED: u32 = 0x0008;
const STCI_RGB: u32 = 0x0004;
const STCI_ALPHA: u32 = 0x0002;
const STCI_TRANSPARENT: u32 = 0x0001;

// ETRLE defines
const COMPRESS_TRANSPARENT: u32 = 0x80;
const COMPRESS_NON_TRANSPARENT: u32 = 0x00;
const COMPRESS_RUN_LIMIT: u32 = 0x7F;

// NB if you're going to change the header definition:
// - make sure that everything in this header is nicely aligned
// - don't exceed the 64-byte maximum
#[derive(Copy, Clone, Debug)]
struct RGB
{
    redMask: u32,
    greenMask: u32,
    blueMask: u32,
    alphaMask: u32,
    redDepth: u8,
    greenDepth: u8,
    blueDepth: u8,
    alphaDepth: u8
}
#[derive(Copy, Clone, Debug)]
struct Indexed
{ // For indexed files, the palette will contain 3 separate bytes for red, green, and blue
    numberOfColours: u32,
    numberOfSubImages: u16,
    redDepth: u8,
    geenDepth: u8,
    blueDepth: u8,
    indexedUnused: [u8; 11]
}

union ColorData
{
    f1: RGB,
    f2: Indexed
}

struct STCIHeader
{
	ID: [u8; STCI_ID_LEN as usize],
	originalSize: u32,
	storedSize: u32, // equal to uiOriginalSize if data uncompressed
	transparentValue: u32,
	flags: u32,
	height: u16,
	width: u16,
    colordata: ColorData,
	depth: u8,	// size in bits of one pixel as stored in the file
	appDataSize: u32,
	unused: [u8; 15],
}
const STCI_HEADER_SIZE: usize = 64;
impl STCIHeader
{
    pub fn new(data: &[u8; STCI_HEADER_SIZE]) -> STCIHeader
    {
        let mut ID = [0; STCI_ID_LEN as usize];
        ID.copy_from_slice(&data[0..STCI_ID_LEN as usize]);
        
        let originalSize = readu32(&data[4..8]);
        let storedSize = readu32(&data[8..12]);
        let transparentValue = readu32(&data[12..16]);
        let flags = readu32(&data[16..20]);
        let height = readu16(&data[20..22]);
        let width = readu16(&data[22..24]);
        
        let redMask = readu32(&data[24..28]);
        let greenMask = readu32(&data[28..32]);
        let blueMask = readu32(&data[32..36]);
        let alphaMask = readu32(&data[36..40]);
        let redDepth = data[40];
        let greenDepth = data[41];
        let blueDepth = data[42];
        let alphaDepth = data[43];
        let colordata = ColorData { f1: RGB { redMask, greenMask, blueMask, alphaMask, redDepth, greenDepth, blueDepth, alphaDepth, } };

        let depth = data[44];
        let appDataSize = readu32(&data[45..49]);
        // depth is packed into a 4 byte boundary so skip three bytes ahead
        // let appDataSize = readu32(&data[48..52]);
        let unused =  [0; 15];

        STCIHeader { ID, originalSize, storedSize, transparentValue, flags, height, width, colordata, depth, appDataSize, unused }
    }
}


struct STCISubImage
{
	dataOffset: u32,
	dataLength: u32,
	offsetX: i16,
	offsetY: i16,
	height: u16,
	width: u16,
}
const STCI_SUBIMAGE_SIZE: usize = 16;

#[derive(Copy, Clone, Debug)]
struct STCIPaletteElement
{
	red: u8,
	green: u8,
	blue: u8,
}
const STCI_PALETTE_ELEMENT_SIZE: usize = 3;
const STCI_8BIT_PALETTE_SIZE: usize = 768;

const IS_COMPRESSED_BIT_MASK: u8 = 0x80;
const COMPRESSED_SEQUENCE_LENGTH_MASK: u8 = 0x7F;
const MAX_SEQUENCE_LENGTH: u8 = 127;

pub fn loadSTI(f: PathBuf) -> Vec<RgbImage>
{
    let mut rgbImages: Vec<RgbImage> = Vec::new();
    let mut fileResult = File::open(&f);
    match fileResult
    {
        Err(e) => 
        {
            println!("Couldn't find file {}", f.to_str().unwrap());
        }
        Ok(mut file) => 
        {
            let mut buffer = [0 as u8; STCI_HEADER_SIZE];
            file.read(&mut buffer);
            let header = STCIHeader::new(&buffer);
            assert_eq!( &header.ID, STCI_ID_STRING);
        
            if header.flags & STCI_INDEXED == STCI_INDEXED
            {
                if unsafe{header.colordata.f2.numberOfColours} != 256
                {
                    println!("Indexed image palette size is incorrect!");
                    return rgbImages;
                }
                
                let mut buffer = [0u8; STCI_8BIT_PALETTE_SIZE];
                file.read(&mut buffer);
                let mut palette: Vec<STCIPaletteElement> = Vec::new();
                for i in 0..256
                {
                    palette.push(STCIPaletteElement { red: buffer[i*3], green: buffer[i*3 + 1], blue: buffer[i*3 + 2] });
                }
                // Read subimageheaders
                let mut subHeaders: Vec<STCISubImage> = Vec::new();
                let imageCount = unsafe{header.colordata.f2.numberOfSubImages};
                for i in 0..imageCount
                {
                    let mut buffer = [0u8; STCI_SUBIMAGE_SIZE];
                    file.read(&mut buffer);
                    
                    let subheader = STCISubImage{
                        dataOffset: readu32(&buffer[0..4]),
                        dataLength: readu32(&buffer[4..8]),
                        offsetX: readu16(&buffer[8..10]) as i16,
                        offsetY: readu16(&buffer[10..12]) as i16,
	                   height: readu16(&buffer[12..14]),
	                   width: readu16(&buffer[14..16])
                    };
                    
                    subHeaders.push(subheader);
                }
                
                // Save image data section position in the file
                let dataSection = file.stream_position().unwrap();
                // Read image data for all images
                let mut images: Vec<IndexedImage> = Vec::new();
                for subheader in subHeaders
                {
                    let start = subheader.dataOffset;
                    file.seek(SeekFrom::Start(dataSection + start as u64));
                    let mut buffer = vec![0u8; subheader.dataLength as usize];
                    file.read_exact(&mut buffer);
                    
                    let mut newImage = IndexedImage{
                        width: subheader.width,
                        height: subheader.height,
                        palette: palette.clone(),
                        indices: Vec::new()
                    };
                    
                    let mut i = 0;
                    loop 
                    {
                        let byte = buffer[i];
                        // println!("{}", byte);
        
                        if byte == 0 { 
                            i +=1; 
                            if i >= buffer.len() 
                            { break; }
                            else 
                            { continue; }
                        }
                        
                        let isCompressed = ((byte & IS_COMPRESSED_BIT_MASK) >> 7) == 1;
                        if isCompressed
                        {
                            let compressedLength = byte & COMPRESSED_SEQUENCE_LENGTH_MASK;
                            for _ in 0..compressedLength 
                            { 
                                // First index is used as transparent color in JA palettes
                                &newImage.indices.push(0); 
                            }
                            i += 1;
                        }
                        else
                        {
                            let uncompressedLength = byte;
                            for _ in 0..uncompressedLength
                            {
                                i += 1;
                                &newImage.indices.push(buffer[i]);
                            }
                            i += 1;
                        }
                        
                        if i >= buffer.len() { break; }
                    }
                    
                    images.push(newImage);
                }
                
                for image in images
                {
                    let mut data: Vec<u8> = Vec::new();
                    for i in image.indices.clone()
                    {
                        data.push(image.palette[i as usize].red);
                        data.push(image.palette[i as usize].green);
                        data.push(image.palette[i as usize].blue);
                        // Alpha value
                        match i
                        {
                            0 => data.push(0),
                            _ => data.push(255)
                        }
                    }
                    
                    // println!("width = {}", image.width);
                    // println!("height = {}", image.height);
                    // println!("indices length should be WxH = {}", image.width*image.height);
                    // println!("indices length = {}", image.indices.len());
                    let rgbImage = RgbImage::new(&data, image.width as i32, image.height as i32, ColorDepth::Rgba8).unwrap();
                    rgbImages.push(rgbImage);
                }
            }
            else if header.flags & STCI_RGB == STCI_RGB
            {
                println!("RGB .stis are not implemented yet!");
            }
        }
    }
    return rgbImages;
}


fn readu32(data: &[u8]) -> u32
{
    let mut temp = [0; 4];
    temp.copy_from_slice(data);
    u32::from_le_bytes(temp)
}
fn readu16(data: &[u8]) -> u16
{
    let mut temp = [0; 2];
    temp.copy_from_slice(data);
    u16::from_le_bytes(temp)
}

#[derive(Clone, Debug)]
struct IndexedImage {
    width: u16,
    height: u16,
    palette: Vec<STCIPaletteElement>,
    indices: Vec<u8>
}

pub fn loadBigGunImages(dataPath: &PathBuf, size: usize) -> Vec<RgbImage>
{
    let mut rgbImages: Vec<RgbImage> = Vec::new();
    
    for i in 0..size
    {
        let mut f = dataPath.clone();
        f.push("BigItems");
        
        if i < 10
        {
            f.push(format!("Gun0{}.STI", i));
        }
        else
        {
            f.push(format!("Gun{}.STI", i));
        }
        
        let sti = loadSTI(f);
        // Only use the very first image for big items.
        // Several mods have incorrect sti files where bigitems contain several subimages, 
        // even though the game only ever loads the first one.
        rgbImages.push(sti[0].clone());
        
        // for s in sti
        // {
            // rgbImages.push(s);
        // }
    }
    
    return rgbImages;
}

pub struct Images
{
    pub big: Vec<Vec<RgbImage>>,
    pub med: Vec<Vec<RgbImage>>,
    pub small: Vec<Vec<RgbImage>>
}
impl Images
{
    pub fn new() -> Images
    {
        Images { big: Vec::new(), med: Vec::new(), small: Vec::new() }
    }
    
    pub fn loadImages(&mut self, dataPath: &PathBuf)
    {
        self.big.clear();
        self.med.clear();
        self.small.clear();
        // Small & medium images first as they are saved inside a single .sti file and their size is then used to determine
        // how many individual bigitem sti graphics are loaded
        
        // Guns
        let mut mdpath = dataPath.clone();
        mdpath.push("tilesets/0/smguns.sti");
	    self.small.push(loadSTI(mdpath));
        
        mdpath = dataPath.clone();
        mdpath.push("Interface/mdguns.sti");
	    self.med.push(loadSTI(mdpath));
        
        self.big.push(loadBigGunImages(dataPath, self.med[0].len()));
        
        // Items
        let options = glob::MatchOptions { case_sensitive: false, require_literal_separator: false, require_literal_leading_dot: false };

        mdpath = dataPath.clone();
        mdpath.push("interface/MDP*.sti");
        for path in glob_with(&mdpath.to_str().unwrap(), options).unwrap().filter_map(Result::ok) {
            // println!("{}", path.display());
            self.med.push(loadSTI(path));
        }
        
        mdpath = dataPath.clone();
        mdpath.push("tilesets/0/smp*items.sti");
        for path in glob_with(&mdpath.to_str().unwrap(), options).unwrap().filter_map(Result::ok) {
            // println!("{}", path.display());
            self.small.push(loadSTI(path));
        }
        
        for i in 1..self.med.len()
        {
            let mut bigitems: Vec<RgbImage> = Vec::new();

            for j in 0..self.med[i].len()
            {
                let mut f = dataPath.clone();
                f.push("BigItems");
                
                if j < 10
                {
                    f.push(format!("P{}ITEM0{}.STI", i, j));
                }
                else
                {
                    f.push(format!("P{}ITEM{}.STI", i, j));
                }
                
                let sti = loadSTI(f);
                for s in sti
                {
                    bigitems.push(s);
                }
            }
            self.big.push(bigitems);
        } 
        
        // println!("Big/med/small images lengths");
        // for i in 0..self.big.len()
        // {
        //     println!("index = {}", i);
        //     println!("{}", self.big[i].len());
        //     println!("{}", self.med[i].len());
        //     println!("{}", self.small[i].len());
        // }
    }

    pub fn getbig(&self, stiType: usize, stiIndex: usize) -> Option<RgbImage>
    {
        if stiType < self.big.len() && stiIndex < self.big[stiType].len()
        {
            return Some(self.big[stiType][stiIndex].clone());
        }
        println!("!!! Graphic index out of graphic vector bounds !!!");
        println!("Tried to access big image [{}][{}]", stiType, stiIndex);
        return None;
    }
    pub fn getmed(&self, stiType: usize, stiIndex: usize) -> Option<RgbImage>
    {
        if stiType < self.med.len() && stiIndex < self.med[stiType].len()
        {
            return Some(self.med[stiType][stiIndex].clone());
        }
        println!("!!! Graphic index out of graphic vector bounds !!!");
        println!("Tried to access medium image [{}][{}]", stiType, stiIndex);
        return None;
    }
    pub fn getsmall(&self, stiType: usize, stiIndex: usize) -> Option<RgbImage>
    {
        if stiType < self.small.len() && stiIndex < self.small[stiType].len()
        {
            return Some(self.small[stiType][stiIndex].clone());
        }
        println!("!!! Graphic index out of graphic vector bounds !!!");
        println!("Tried to access small image [{}][{}]", stiType, stiIndex);
        return None;
    }
}