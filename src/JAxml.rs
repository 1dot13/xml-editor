#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::io::{BufReader, Write};
use std::fs::{File};
use std::fmt;
use std::str;
use std::path::PathBuf;
use quick_xml::events::{Event};
// use quick_xml::events::attributes::{Attributes, Attribute};
// use quick_xml::name::QName;
use quick_xml::{Reader};

//-----------------------------------------------------------------------------
// Macros
//-----------------------------------------------------------------------------
macro_rules! write_tag_i {
	($file:tt, $value:tt, $tag:tt, $forcewrite:tt) => {{
		
		let mut empty = false;
		if $value == 0 {empty = true;}

		if empty == false || $forcewrite == true
		{
			match write!($file, "\t\t<{}>{}</{}>\n", $tag, $value, $tag)
			{
				Ok(_) => {}
				Err(e) => {panic!("Error writing value {} for xml tag {}\n {:?}", $value, $tag, e)}
			}
		}
	}}
}
macro_rules! write_tag_s {
	($file:tt, $value:tt, $tag:tt, $forcewrite:tt) => {{
		
		let mut empty = false;
		if $value == "" {empty = true;}

		if empty == false || $forcewrite == true
		{
			match write!($file, "\t\t<{}>{}</{}>\n", $tag, $value, $tag)
			{
				Ok(_) => {}
				Err(e) => {panic!("Error writing value {} for xml tag {}\n {:?}", $value, $tag, e)}
			}
		}
	}}
}
macro_rules! write_tag_f {
	($file:tt, $value:tt, $tag:tt, $forcewrite:tt) => {{
		
		let mut empty = false;
		if $value == 0.0 {empty = true;}

		if empty == false || $forcewrite == true
		{
			match write!($file, "\t\t<{}>{}</{}>\n", $tag, $value, $tag)
			{
				Ok(_) => {}
				Err(e) => {panic!("Error writing value {} for xml tag {}\n {:?}", $value, $tag, e)}
			}
		}
	}}
}

macro_rules! generateListStructs {
    ($($listname:ident, $itemname:ident, $matchvalue:tt)*) => {

        $(pub struct $listname
        {
            pub items: Vec<$itemname>
        }
        impl $listname {
            pub fn new() -> $listname
            {
                $listname{items: Vec::new()}
            }

            pub fn loadItems(filepath: &PathBuf) -> $listname
            {
                let mut il = $listname::new();
                let items = &mut il.items;

                let mut reader = Reader::from_file(filepath).unwrap();
                reader.trim_text(true);
                let mut buf = Vec::new();
                loop 
                {
                    match reader.read_event_into(&mut buf) 
                    {
                        Err(element) => panic!("Error at position {}: {:?}", reader.buffer_position(), element),
                        Ok(Event::Eof) => break,

                        Ok(Event::Start(ref element)) => 
                        {
                            match element.name().as_ref() 
                            {			
                                $matchvalue =>
                                {
                                    items.push($itemname::new());
                                    items.last_mut().unwrap().readItem(&mut reader, &mut buf);
                                }
                                _ => {}
                            }
                        }
                        _ => ()
                    }
                    buf.clear();
                }
                return il;
            }

            pub fn save(&self, filepath: &PathBuf)
            {
                let mut buffer = Vec::new();
                // Write xml header before the xml data
                write!(buffer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n").unwrap();
                write!(buffer, "<{}>\n", stringify!($listname)).unwrap();
                
                let mut forcewrite = true; // write first item completely
                for i in &self.items
                {
                    i.save(&mut buffer, forcewrite);
                    forcewrite = false;
                }

                write!(buffer, "</{}>\n", stringify!($listname)).unwrap();

                println!("{}", &filepath.to_str().unwrap());
                std::fs::create_dir_all(filepath.parent().unwrap());
                let mut file = File::create(filepath).unwrap();
                file.write_all(&buffer);
            }
        })*
    }
}
//-----------------------------------------------------------------------------
// Structs
//-----------------------------------------------------------------------------
generateListStructs!{
    ITEMLIST, ITEM, b"ITEM"
    WEAPONLIST, WEAPON, b"WEAPON"
    ATTACHMENTSLOTLIST, ATTACHMENTSLOT, b"ATTACHMENTSLOT"
    ATTACHMENTLIST, ATTACHMENT, b"ATTACHMENT"
    ATTACHMENTINFOLIST, ATTACHMENTINFO, b"ATTACHMENTINFO"
    ATTACHMENTCOMBOMERGELIST, ATTACHMENTCOMBOMERGE, b"ATTACHMENTCOMBOMERGE"
    ARMOURLIST, ARMOUR, b"ARMOUR"
    AMMOTYPELIST, AMMOTYPE, b"AMMOTYPE"
    AMMOLIST, AMMOSTRING, b"AMMO"
    MAGAZINELIST, MAGAZINE, b"MAGAZINE"
    CLOTHESLIST, CLOTHES, b"CLOTHES"
    COMPATIBLEFACEITEMLIST, COMPATIBLEFACEITEM, b"COMPATIBLEFACEITEM"
    DRUGSLIST, DRUG, b"DRUG"
    EXPDATALIST, EXPDATA, b"EXPDATA"
    EXPLOSIVELIST, EXPLOSIVE, b"EXPLOSIVE"
    FOODSLIST, FOOD, b"FOOD"
    FOODOPINIONSLIST, FOODOPINION, b"FOODOPINION"
    INCOMPATIBLEATTACHMENTLIST, INCOMPATIBLEATTACHMENT, b"INCOMPATIBLEATTACHMENT"
    TRANSFORMATIONS_LIST, TRANSFORM, b"TRANSFORM"
    LAUNCHABLELIST, LAUNCHABLE, b"LAUNCHABLE"
    LOADBEARINGEQUIPMENTLIST, LOADBEARINGEQUIPMENT, b"LOADBEARINGEQUIPMENT"
    MERGELIST, MERGE, b"MERGE"
    POCKETLIST, POCKET, b"POCKET"
    RANDOMITEMLIST, RANDOMITEM, b"RANDOMITEM"
    STRUCTURECONSTRUCTLIST, STRUCTURECONSTRUCT, b"STRUCTURE"
    STRUCTUREDECONSTRUCTLIST, STRUCTUREDECONSTRUCT, b"STRUCTURE"
    STRUCTUREMOVELIST, STRUCTUREMOVE, b"STRUCTURE"
	SPREADPATTERNLIST, SPREADPATTERN, b"SPREADPATTERN"
}

pub struct Data
{
	pub items: ITEMLIST,
	pub weapons: WEAPONLIST,
	pub armors: ARMOURLIST,
	pub clothes: CLOTHESLIST,
	pub calibers: AMMOLIST,
	pub ammotypes: AMMOTYPELIST,
	pub magazines: MAGAZINELIST,
	pub attachments: ATTACHMENTLIST,
	pub attachmentslots: ATTACHMENTSLOTLIST,
	pub attachmentinfo: ATTACHMENTINFOLIST,
	pub attachmentmerges: ATTACHMENTCOMBOMERGELIST,
	pub incompatibleattachments: INCOMPATIBLEATTACHMENTLIST,
	pub compatiblefaceitems: COMPATIBLEFACEITEMLIST,
	pub drugs: DRUGSLIST,
	pub explosiondata: EXPDATALIST,
	pub explosives: EXPLOSIVELIST,
	pub foods: FOODSLIST,
	pub foodopinions: FOODOPINIONSLIST,
	pub transformations: TRANSFORMATIONS_LIST,
	pub launchables: LAUNCHABLELIST,
	pub lbe: LOADBEARINGEQUIPMENTLIST,
	pub merges: MERGELIST,
	pub pockets: POCKETLIST,
	pub randomitems: RANDOMITEMLIST,
	pub structconstructs: STRUCTURECONSTRUCTLIST,
	pub structdeconstructs: STRUCTUREDECONSTRUCTLIST,
	pub structmoves: STRUCTUREMOVELIST,
	pub spreadpatterns: SPREADPATTERNLIST,
	pub sounds: SOUNDLIST,
	pub burstsounds: SOUNDLIST,
}
impl Data
{
	pub fn new() -> Data
	{
		Data { 
			items: ITEMLIST::new(), weapons: WEAPONLIST::new(), armors: ARMOURLIST::new(), clothes: CLOTHESLIST::new(), calibers: AMMOLIST::new(),
			ammotypes: AMMOTYPELIST::new(), attachmentinfo: ATTACHMENTINFOLIST::new(), attachmentmerges: ATTACHMENTCOMBOMERGELIST::new(),
			attachments: ATTACHMENTLIST::new(), attachmentslots: ATTACHMENTSLOTLIST::new(), compatiblefaceitems: COMPATIBLEFACEITEMLIST::new(),
			drugs: DRUGSLIST::new(), explosiondata: EXPDATALIST::new(), explosives: EXPLOSIVELIST::new(), foods: FOODSLIST::new(),
			foodopinions: FOODOPINIONSLIST::new(), incompatibleattachments: INCOMPATIBLEATTACHMENTLIST::new(), launchables: LAUNCHABLELIST::new(),
			lbe: LOADBEARINGEQUIPMENTLIST::new(), magazines: MAGAZINELIST::new(), merges: MERGELIST::new(), pockets: POCKETLIST::new(),
			randomitems: RANDOMITEMLIST::new(), transformations: TRANSFORMATIONS_LIST::new(), structconstructs: STRUCTURECONSTRUCTLIST::new(),
			structdeconstructs: STRUCTUREDECONSTRUCTLIST::new(), structmoves: STRUCTUREMOVELIST::new(), spreadpatterns: SPREADPATTERNLIST::new(),
			sounds: SOUNDLIST::new(), burstsounds: SOUNDLIST::new() 
		}
	}
	
	fn paths(dataFolder: &PathBuf) -> Vec<PathBuf>
	{
		let mut tableDataPath = dataFolder.clone();
		tableDataPath.push("TableData");
		
		let mut paths: Vec<PathBuf> = Vec::new();
		for _ in 0..30
		{
			paths.push(tableDataPath.clone());
		}
		paths[0].push("Items/Items.xml");
		paths[1].push("Items/Weapons.xml");
		paths[2].push("Items/Armours.xml");
		paths[3].push("Items/Clothes.xml");
		paths[4].push("Items/AmmoStrings.xml");
		paths[5].push("Items/AmmoTypes.xml");
		paths[6].push("Items/Magazines.xml");
		paths[7].push("Items/Attachments.xml");
		paths[8].push("Items/AttachmentSlots.xml");
		paths[9].push("Items/AttachmentInfo.xml");
		paths[10].push("Items/AttachmentComboMerges.xml");
		paths[11].push("Items/IncompatibleAttachments.xml");
		paths[12].push("Items/CompatibleFaceItems.xml");
		paths[13].push("Items/Drugs.xml");
		paths[14].push("Items/ExplosionData.xml");
		paths[15].push("Items/Explosives.xml");
		paths[16].push("Items/Food.xml");
		paths[17].push("Items/FoodOpinion.xml");
		paths[18].push("Items/Item_Transformations.xml");
		paths[19].push("Items/Launchables.xml");
		paths[20].push("Items/LoadBearingEquipment.xml");
		paths[21].push("Items/Merges.xml");
		paths[22].push("Items/Pockets.xml");
		paths[23].push("Items/RandomItem.xml");
		paths[24].push("Items/StructureConstruct.xml");
		paths[25].push("Items/StructureDeconstruct.xml");
		paths[26].push("Items/StructureMove.xml");
		paths[27].push("SpreadPatterns.xml");
		paths[28].push("Sounds/Sounds.xml");
		paths[29].push("Sounds/BurstSounds.xml");

		return paths;
	}

	pub fn loadData(&mut self, dataFolder: &PathBuf)
	{
		let paths = Data::paths(dataFolder);

		let items = ITEMLIST::loadItems(&paths[0]);
		let weapons= WEAPONLIST::loadItems(&paths[1]);
		let armors= ARMOURLIST::loadItems(&paths[2]);
		let clothes = CLOTHESLIST::loadItems(&paths[3]);
		let calibers = AMMOLIST::loadItems(&paths[4]);
		let ammotypes = AMMOTYPELIST::loadItems(&paths[5]);
		let magazines = MAGAZINELIST::loadItems(&paths[6]);
		let attachments = ATTACHMENTLIST::loadItems(&paths[7]);
		let attachmentslots = ATTACHMENTSLOTLIST::loadItems(&paths[8]);
		let attachmentinfo = ATTACHMENTINFOLIST::loadItems(&paths[9]);
		let attachmentmerges = ATTACHMENTCOMBOMERGELIST::loadItems(&paths[10]);
		let incompatibleattachments = INCOMPATIBLEATTACHMENTLIST::loadItems(&paths[11]);
		let compatiblefaceitems = COMPATIBLEFACEITEMLIST::loadItems(&paths[12]);
		let drugs = DRUGSLIST::loadItems(&paths[13]);
		let explosiondata = EXPDATALIST::loadItems(&paths[14]);
		let explosives = EXPLOSIVELIST::loadItems(&paths[15]);
		let foods = FOODSLIST::loadItems(&paths[16]);
		let foodopinions = FOODOPINIONSLIST::loadItems(&paths[17]);
		let transformations = TRANSFORMATIONS_LIST::loadItems(&paths[18]);
		let launchables = LAUNCHABLELIST::loadItems(&paths[19]);
		let lbe = LOADBEARINGEQUIPMENTLIST::loadItems(&paths[20]);
		let merges = MERGELIST::loadItems(&paths[21]);
		let pockets = POCKETLIST::loadItems(&paths[22]);
		let randomitems = RANDOMITEMLIST::loadItems(&paths[23]);
		let structconstructs = STRUCTURECONSTRUCTLIST::loadItems(&paths[24]);
		let structdeconstructs = STRUCTUREDECONSTRUCTLIST::loadItems(&paths[25]);
		let structmoves = STRUCTUREMOVELIST::loadItems(&paths[26]);
		let spreadpatterns = SPREADPATTERNLIST::loadItems(&paths[27]);
		let sounds = SOUNDLIST::loadItems(&paths[28]);
		let burstsounds = SOUNDLIST::loadItems(&paths[29]);

		self.items = items;
		self.weapons = weapons;
		self.armors = armors;
		self.clothes = clothes;
		self.calibers = calibers;
		self.ammotypes = ammotypes;
		self.magazines = magazines;
		self.attachments = attachments;
		self.attachmentslots = attachmentslots;
		self.attachmentinfo = attachmentinfo;
		self.attachmentmerges = attachmentmerges;
		self.incompatibleattachments = incompatibleattachments;
		self.compatiblefaceitems = compatiblefaceitems;
		self.drugs = drugs;
		self.explosiondata = explosiondata;
		self.explosives = explosives;
		self.foods = foods;
		self.foodopinions = foodopinions;
		self.transformations = transformations;
		self.launchables = launchables;
		self.lbe = lbe;
		self.merges = merges;
		self.pockets = pockets;
		self.randomitems = randomitems;
		self.structconstructs = structconstructs;
		self.structdeconstructs = structdeconstructs;
		self.structmoves = structmoves;
		self.spreadpatterns = spreadpatterns;
		self.sounds = sounds;
		self.burstsounds = burstsounds;
	}
	
	pub fn saveData(&self, dataFolder: &PathBuf)
	{
		let paths = Data::paths(dataFolder);
		
		self.items.save(&paths[0]);
		self.weapons.save(&paths[1]);
		self.armors.save(&paths[2]);
		self.clothes.save(&paths[3]);
		self.calibers.save(&paths[4]);
		self.ammotypes.save(&paths[5]);
		self.magazines.save(&paths[6]);
		self.attachments.save(&paths[7]);
		self.attachmentslots.save(&paths[8]);
		self.attachmentinfo.save(&paths[9]);
		self.attachmentmerges.save(&paths[10]);
		self.incompatibleattachments.save(&paths[11]);
		self.compatiblefaceitems.save(&paths[12]);
		self.drugs.save(&paths[13]);
		self.explosiondata.save(&paths[14]);
		self.explosives.save(&paths[15]);
		self.foods.save(&paths[16]);
		self.foodopinions.save(&paths[17]);
		self.transformations.save(&paths[18]);
		self.launchables.save(&paths[19]);
		self.lbe.save(&paths[20]);
		self.merges.save(&paths[21]);
		self.pockets.save(&paths[22]);
		self.randomitems.save(&paths[23]);
		self.structconstructs.save(&paths[24]);
		self.structdeconstructs.save(&paths[25]);
		self.structmoves.save(&paths[26]);
	}
	
	pub fn findNamebyIndex(&self, uiIndex: u32) -> Option<String>
	{
		for item in &self.items.items
		{
			if item.uiIndex == uiIndex
			{
				return Some(item.szLongItemName.clone());
			}
		}
		
		return None;
	}

	pub fn getWeapon(&self, uiIndex: u32) -> Option<&WEAPON>
	{
		for weapon in &self.weapons.items
		{
			if weapon.uiIndex == uiIndex { return Some(weapon); }
		}
		
		return None;
	}

	pub fn getWeapon_mut(&mut self, uiIndex: u32) -> Option<&mut WEAPON>
	{
		for weapon in &mut self.weapons.items
		{
			if weapon.uiIndex == uiIndex { return Some(weapon); }
		}
		
		return None;
	}
}


impl CLOTHESLIST
{
	pub fn findNamebyIndex(&self, uiIndex: u32) -> Option<String>
	{
		for item in &self.items
		{
			if item.uiIndex == uiIndex
			{
				return Some(item.szName.clone());
			}
		}
		
		return None;
	}	
}
impl AMMOLIST
{
	pub fn findNamebyIndex(&self, uiIndex: u32) -> Option<String>
	{
		for item in &self.items
		{
			if item.uiIndex == uiIndex
			{
				return Some(item.AmmoCaliber.clone());
			}
		}
		
		return None;
	}	
}



pub struct SOUNDLIST
{
	pub sounds: Vec<String>,
}
impl SOUNDLIST
{
	pub fn new() -> SOUNDLIST
    {
        SOUNDLIST { sounds: Vec::new() }
    }


	pub fn loadItems(filepath: &PathBuf) -> SOUNDLIST
	{
		let mut il = SOUNDLIST::new();

		let mut reader = Reader::from_file(filepath).unwrap();
		reader.trim_text(true);
		let mut buf = Vec::new();
		loop 
		{
			match reader.read_event_into(&mut buf) 
			{
				Err(element) => panic!("Error at position {}: {:?}", reader.buffer_position(), element),
				Ok(Event::Eof) => break,

				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"SOUND" => {il.sounds.push( parseString(&mut reader, &mut buf) );}
						_ => {}
					}
				}
				_ => ()
			}
			buf.clear();
		}
		return il;
	}


	pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		// EMPTY BY DESIGN! 
		// Macro for generating list structs requires the function definition. 
		// Not a problem as long as we never save sounds.xml until the complete struct is implemented.
	}
}


pub struct SPREADPATTERN
{
	pub name: String,
}
impl SPREADPATTERN
{
	pub fn new() -> SPREADPATTERN
    {
        SPREADPATTERN { name: "".to_string() }
    }


	pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"NAME" => {self.name = parseString(reader, buf);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"SPREADPATTERN" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}


	pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		// EMPTY BY DESIGN! 
		// Macro for generating list structs requires the function definition. 
		// Not a problem as long as we never save spreadpatterns.xml until the complete struct is implemented.
	}
}


pub struct STRUCTUREMOVE
{
    szTileSetDisplayName: String,
    szTileSetName: String,
    allowedtiles: Vec<u32>,
}
impl STRUCTUREMOVE {
    pub fn new() -> STRUCTUREMOVE
    {
        STRUCTUREMOVE { szTileSetDisplayName: "".to_string(), szTileSetName: "".to_string(), allowedtiles: Vec::new() }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"szTileSetDisplayName" => {self.szTileSetDisplayName = parseString(reader, buf);}
                        b"szTileSetName" => {self.szTileSetName = parseString(reader, buf);}
                        b"allowedtiles" => {self.allowedtiles.push(parseu32(reader, buf, &name));}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"STRUCTURE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<STRUCTURE>\n").unwrap();

        let value = &self.szTileSetDisplayName;
        write_tag_s!(file, value, "szTileSetDisplayName", forcewrite);
        let value = &self.szTileSetName;
        write_tag_s!(file, value, "szTileSetName", forcewrite);
        for tile in &self.allowedtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "allowedtile", forcewrite);
        }

        write!(file, "\t</STRUCTURE>\n").unwrap();
	}
}


pub struct STRUCTUREDECONSTRUCT
{
    usDeconstructItem: u32,
    usItemToCreate: u32,
    usCreatedItemStatus: u32,
    szTileSetDisplayName: String,
    szTileSetName: String,
    dCreationCost: f32,
    allowedtiles: Vec<u32>,
}
impl STRUCTUREDECONSTRUCT {
    pub fn new() -> STRUCTUREDECONSTRUCT
    {
        STRUCTUREDECONSTRUCT { usDeconstructItem: 0, szTileSetDisplayName: "".to_string(), szTileSetName: "".to_string(), usItemToCreate: 0, dCreationCost: 0.0,
        usCreatedItemStatus: 0, allowedtiles: Vec::new()
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"usDeconstructItem" => {self.usDeconstructItem = parseu32(reader, buf, &name);}
                        b"usCreatedItemStatus" => {self.usCreatedItemStatus = parseu32(reader, buf, &name);}
                        b"szTileSetDisplayName" => {self.szTileSetDisplayName = parseString(reader, buf);}
                        b"szTileSetName" => {self.szTileSetName = parseString(reader, buf);}
                        b"usItemToCreate" => {self.usItemToCreate = parseu32(reader, buf, &name);}
                        b"dCreationCost" => {self.dCreationCost = parsef32(reader, buf, &name);}
                        b"allowedtiles" => {self.allowedtiles.push(parseu32(reader, buf, &name));}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"STRUCTURE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<STRUCTURE>\n").unwrap();

        let value = self.usDeconstructItem;
        write_tag_i!(file, value, "usDeconstructItem", forcewrite);
        let value = self.usItemToCreate;
        write_tag_i!(file, value, "usItemToCreate", forcewrite);
        let value = &self.szTileSetDisplayName;
        write_tag_s!(file, value, "szTileSetDisplayName", forcewrite);
        let value = &self.szTileSetName;
        write_tag_s!(file, value, "szTileSetName", forcewrite);
        let value = self.dCreationCost;
        write_tag_f!(file, value, "dCreationCost", forcewrite);
        let value = self.usCreatedItemStatus as u32;
        write_tag_i!(file, value, "usCreatedItemStatus", forcewrite);
        for tile in &self.allowedtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "allowedtile", forcewrite);
        }

        write!(file, "\t</STRUCTURE>\n").unwrap();
	}
}


pub struct STRUCTURECONSTRUCT
{
    usCreationItem: u32,
    usItemStatusLoss: u32,
    szTileSetDisplayName: String,
    szTileSetName: String,
    dCreationCost: f32,
    fFortifyAdjacentAdjustment: bool,
    northfacingtiles: Vec<u32>,
    southfacingtiles: Vec<u32>,
    eastfacingtiles: Vec<u32>,
    westfacingtiles: Vec<u32>,
}
impl STRUCTURECONSTRUCT {
    pub fn new() -> STRUCTURECONSTRUCT
    {
        STRUCTURECONSTRUCT { usCreationItem: 0, szTileSetDisplayName: "".to_string(), szTileSetName: "".to_string(), usItemStatusLoss: 0, dCreationCost: 0.0,
        fFortifyAdjacentAdjustment: false, northfacingtiles: Vec::new(), southfacingtiles: Vec::new(), eastfacingtiles: Vec::new(), westfacingtiles: Vec::new()
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"usCreationItem" => {self.usCreationItem = parseu32(reader, buf, &name);}
                        b"szTileSetDisplayName" => {self.szTileSetDisplayName = parseString(reader, buf);}
                        b"szTileSetName" => {self.szTileSetName = parseString(reader, buf);}
                        b"usItemStatusLoss" => {self.usItemStatusLoss = parseu32(reader, buf, &name);}
                        b"dCreationCost" => {self.dCreationCost = parsef32(reader, buf, &name);}
                        b"fFortifyAdjacentAdjustment" => {self.fFortifyAdjacentAdjustment = parsebool(reader, buf, &name);}
                        b"northfacingtiles" => {self.northfacingtiles.push(parseu32(reader, buf, &name));}
                        b"southfacingtiles" => {self.southfacingtiles.push(parseu32(reader, buf, &name));}
                        b"eastfacingtiles" => {self.eastfacingtiles.push(parseu32(reader, buf, &name));}
                        b"westfacingtiles" => {self.westfacingtiles.push(parseu32(reader, buf, &name));}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"STRUCTURE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<STRUCTURE>\n").unwrap();

        let value = self.usCreationItem;
        write_tag_i!(file, value, "usCreationItem", forcewrite);
        let value = self.usItemStatusLoss;
        write_tag_i!(file, value, "usItemStatusLoss", forcewrite);
        let value = &self.szTileSetDisplayName;
        write_tag_s!(file, value, "szTileSetDisplayName", forcewrite);
        let value = &self.szTileSetName;
        write_tag_s!(file, value, "szTileSetName", forcewrite);
        let value = self.dCreationCost;
        write_tag_f!(file, value, "dCreationCost", forcewrite);
        let value = self.fFortifyAdjacentAdjustment as u32;
        write_tag_i!(file, value, "fFortifyAdjacentAdjustment", forcewrite);
        for tile in &self.northfacingtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "northfacingtile", forcewrite);
        }
        for tile in &self.southfacingtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "southfacingtile", forcewrite);
        }
        for tile in &self.eastfacingtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "eastfacingtile", forcewrite);
        }
        for tile in &self.westfacingtiles
        {
            let value = tile.clone();
            write_tag_i!(file, value, "westfacingtile", forcewrite);
        }

        write!(file, "\t</STRUCTURE>\n").unwrap();
	}
}


pub struct RANDOMITEM
{
    uiIndex: u32,
    szName: String,
    randomitem1: u32,
    randomitem2: u32,
    randomitem3: u32,
    randomitem4: u32,
    randomitem5: u32,
    randomitem6: u32,
    randomitem7: u32,
    randomitem8: u32,
    randomitem9: u32,
    randomitem10: u32,
    item1: u32,
    item2: u32,
    item3: u32,
    item4: u32,
    item5: u32,
    item6: u32,
    item7: u32,
    item8: u32,
    item9: u32,
    item10: u32,
}
impl RANDOMITEM {
    pub fn new() -> RANDOMITEM
    {
        RANDOMITEM { uiIndex: 0, szName: "".to_string(), randomitem1: 0, randomitem2: 0, randomitem3: 0, randomitem4: 0, randomitem5: 0, randomitem6: 0, randomitem7: 0, randomitem8: 0, randomitem9: 0, randomitem10: 0, item1: 0, item2: 0, item3: 0, item4: 0, item5: 0, item6: 0, item7: 0, item8: 0, item9: 0, item10: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"uiIndex" => {self.uiIndex = parseu32(reader, buf, &name);}
                        b"szName" => {self.szName = parseString(reader, buf);}
                        b"randomitem1" => {self.randomitem1 = parseu32(reader, buf, &name);}
                        b"randomitem2" => {self.randomitem2 = parseu32(reader, buf, &name);}
                        b"randomitem3" => {self.randomitem3 = parseu32(reader, buf, &name);}
                        b"randomitem4" => {self.randomitem4 = parseu32(reader, buf, &name);}
                        b"randomitem5" => {self.randomitem5 = parseu32(reader, buf, &name);}
                        b"randomitem6" => {self.randomitem6 = parseu32(reader, buf, &name);}
                        b"randomitem7" => {self.randomitem7 = parseu32(reader, buf, &name);}
                        b"randomitem8" => {self.randomitem8 = parseu32(reader, buf, &name);}
                        b"randomitem9" => {self.randomitem9 = parseu32(reader, buf, &name);}
                        b"randomitem10" => {self.randomitem10 = parseu32(reader, buf, &name);}
                        b"item1" => {self.item1 = parseu32(reader, buf, &name);}
                        b"item2" => {self.item2 = parseu32(reader, buf, &name);}
                        b"item3" => {self.item3 = parseu32(reader, buf, &name);}
                        b"item4" => {self.item4 = parseu32(reader, buf, &name);}
                        b"item5" => {self.item5 = parseu32(reader, buf, &name);}
                        b"item6" => {self.item6 = parseu32(reader, buf, &name);}
                        b"item7" => {self.item7 = parseu32(reader, buf, &name);}
                        b"item8" => {self.item8 = parseu32(reader, buf, &name);}
                        b"item9" => {self.item9 = parseu32(reader, buf, &name);}
                        b"item10" => {self.item10 = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"RANDOMITEM" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<RANDOMITEM>\n").unwrap();

        let value = self.uiIndex;
        write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.szName;
        write_tag_s!(file, value, "szName", forcewrite);
        let value = self.randomitem1;
        write_tag_i!(file, value, "randomitem1", forcewrite);
        let value = self.randomitem2;
        write_tag_i!(file, value, "randomitem2", forcewrite);
        let value = self.randomitem3;
        write_tag_i!(file, value, "randomitem3", forcewrite);
        let value = self.randomitem4;
        write_tag_i!(file, value, "randomitem4", forcewrite);
        let value = self.randomitem5;
        write_tag_i!(file, value, "randomitem5", forcewrite);
        let value = self.randomitem6;
        write_tag_i!(file, value, "randomitem6", forcewrite);
        let value = self.randomitem7;
        write_tag_i!(file, value, "randomitem7", forcewrite);
        let value = self.randomitem8;
        write_tag_i!(file, value, "randomitem8", forcewrite);
        let value = self.randomitem9;
        write_tag_i!(file, value, "randomitem9", forcewrite);
        let value = self.randomitem10;
        write_tag_i!(file, value, "randomitem10", forcewrite);
        let value = self.item1;
        write_tag_i!(file, value, "item1", forcewrite);
        let value = self.item2;
        write_tag_i!(file, value, "item2", forcewrite);
        let value = self.item3;
        write_tag_i!(file, value, "item3", forcewrite);
        let value = self.item4;
        write_tag_i!(file, value, "item4", forcewrite);
        let value = self.item5;
        write_tag_i!(file, value, "item5", forcewrite);
        let value = self.item6;
        write_tag_i!(file, value, "item6", forcewrite);
        let value = self.item7;
        write_tag_i!(file, value, "item7", forcewrite);
        let value = self.item8;
        write_tag_i!(file, value, "item8", forcewrite);
        let value = self.item9;
        write_tag_i!(file, value, "item9", forcewrite);
        let value = self.item10;
        write_tag_i!(file, value, "item10", forcewrite);


        write!(file, "\t</RANDOMITEM>\n").unwrap();
	}
}


pub struct POCKET
{
    pIndex: u32,
    pName: String,
    pSilhouette: u32,
    pType: u32,
    pRestriction: u32,
    pVolume: u32,
    ItemCapacityPerSize0: u32,
    ItemCapacityPerSize1: u32,
    ItemCapacityPerSize2: u32,
    ItemCapacityPerSize3: u32,
    ItemCapacityPerSize4: u32,
    ItemCapacityPerSize5: u32,
    ItemCapacityPerSize6: u32,
    ItemCapacityPerSize7: u32,
    ItemCapacityPerSize8: u32,
    ItemCapacityPerSize9: u32,
    ItemCapacityPerSize10: u32,
    ItemCapacityPerSize11: u32,
    ItemCapacityPerSize12: u32,
    ItemCapacityPerSize13: u32,
    ItemCapacityPerSize14: u32,
    ItemCapacityPerSize15: u32,
    ItemCapacityPerSize16: u32,
    ItemCapacityPerSize17: u32,
    ItemCapacityPerSize18: u32,
    ItemCapacityPerSize19: u32,
    ItemCapacityPerSize20: u32,
    ItemCapacityPerSize21: u32,
    ItemCapacityPerSize22: u32,
    ItemCapacityPerSize23: u32,
    ItemCapacityPerSize24: u32,
    ItemCapacityPerSize25: u32,
    ItemCapacityPerSize26: u32,
    ItemCapacityPerSize27: u32,
    ItemCapacityPerSize28: u32,
    ItemCapacityPerSize29: u32,
    ItemCapacityPerSize30: u32,
    ItemCapacityPerSize31: u32,
    ItemCapacityPerSize32: u32,
    ItemCapacityPerSize33: u32,
    ItemCapacityPerSize34: u32
}
impl POCKET {
    pub fn new() -> POCKET
    {
        POCKET { pIndex: 0, pName: "".to_string(), pSilhouette: 0, pType: 0, pRestriction: 0, pVolume: 0,
        ItemCapacityPerSize0: 0, ItemCapacityPerSize1: 0, ItemCapacityPerSize2: 0, ItemCapacityPerSize3: 0, ItemCapacityPerSize4: 0, ItemCapacityPerSize5: 0,
        ItemCapacityPerSize6: 0, ItemCapacityPerSize7: 0, ItemCapacityPerSize8: 0, ItemCapacityPerSize9: 0, ItemCapacityPerSize10: 0, ItemCapacityPerSize11: 0,
        ItemCapacityPerSize12: 0, ItemCapacityPerSize13: 0, ItemCapacityPerSize14: 0, ItemCapacityPerSize15: 0, ItemCapacityPerSize16: 0, ItemCapacityPerSize17: 0,
        ItemCapacityPerSize18: 0, ItemCapacityPerSize19: 0, ItemCapacityPerSize20: 0, ItemCapacityPerSize21: 0, ItemCapacityPerSize22: 0, ItemCapacityPerSize23: 0,
        ItemCapacityPerSize24: 0, ItemCapacityPerSize25: 0, ItemCapacityPerSize26: 0, ItemCapacityPerSize27: 0, ItemCapacityPerSize28: 0, ItemCapacityPerSize29: 0,
        ItemCapacityPerSize30: 0, ItemCapacityPerSize31: 0, ItemCapacityPerSize32: 0, ItemCapacityPerSize33: 0, ItemCapacityPerSize34: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"pIndex" => {self.pIndex = parseu32(reader, buf, &name);}
                        b"pName" => {self.pName = parseString(reader, buf);}
                        b"pSilhouette" => {self.pSilhouette = parseu32(reader, buf, &name);}
                        b"pType" => {self.pType = parseu32(reader, buf, &name);}
                        b"pRestriction" => {self.pRestriction = parseu32(reader, buf, &name);}
                        b"pVolume" => {self.pVolume = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize0" => {self.ItemCapacityPerSize0 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize1" => {self.ItemCapacityPerSize1 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize2" => {self.ItemCapacityPerSize2 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize3" => {self.ItemCapacityPerSize3 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize4" => {self.ItemCapacityPerSize4 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize5" => {self.ItemCapacityPerSize5 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize6" => {self.ItemCapacityPerSize6 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize7" => {self.ItemCapacityPerSize7 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize8" => {self.ItemCapacityPerSize8 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize9" => {self.ItemCapacityPerSize9 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize10" => {self.ItemCapacityPerSize10 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize11" => {self.ItemCapacityPerSize11 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize12" => {self.ItemCapacityPerSize12 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize13" => {self.ItemCapacityPerSize13 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize14" => {self.ItemCapacityPerSize14 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize15" => {self.ItemCapacityPerSize15 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize16" => {self.ItemCapacityPerSize16 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize17" => {self.ItemCapacityPerSize17 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize18" => {self.ItemCapacityPerSize18 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize19" => {self.ItemCapacityPerSize19 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize20" => {self.ItemCapacityPerSize20 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize21" => {self.ItemCapacityPerSize21 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize22" => {self.ItemCapacityPerSize22 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize23" => {self.ItemCapacityPerSize23 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize24" => {self.ItemCapacityPerSize24 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize25" => {self.ItemCapacityPerSize25 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize26" => {self.ItemCapacityPerSize26 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize27" => {self.ItemCapacityPerSize27 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize28" => {self.ItemCapacityPerSize28 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize29" => {self.ItemCapacityPerSize29 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize30" => {self.ItemCapacityPerSize30 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize31" => {self.ItemCapacityPerSize31 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize32" => {self.ItemCapacityPerSize32 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize33" => {self.ItemCapacityPerSize33 = parseu32(reader, buf, &name);}
                        b"ItemCapacityPerSize34" => {self.ItemCapacityPerSize34 = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"POCKET" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<POCKET>\n").unwrap();

        let value = self.pIndex;
        write_tag_i!(file, value, "pIndex", forcewrite);
        let value = &self.pName;
        write_tag_s!(file, value, "pName", forcewrite);
        let value = self.pSilhouette;
        write_tag_i!(file, value, "pSilhouette", forcewrite);
        let value = self.pType;
        write_tag_i!(file, value, "pType", forcewrite);
        let value = self.pRestriction;
        write_tag_i!(file, value, "pRestriction", forcewrite);
        let value = self.pVolume;
        write_tag_i!(file, value, "pVolume", forcewrite);
        let value = self.ItemCapacityPerSize0;
        write_tag_i!(file, value, "ItemCapacityPerSize0", forcewrite);
        let value = self.ItemCapacityPerSize1;
        write_tag_i!(file, value, "ItemCapacityPerSize1", forcewrite);
        let value = self.ItemCapacityPerSize2;
        write_tag_i!(file, value, "ItemCapacityPerSize2", forcewrite);
        let value = self.ItemCapacityPerSize3;
        write_tag_i!(file, value, "ItemCapacityPerSize3", forcewrite);
        let value = self.ItemCapacityPerSize4;
        write_tag_i!(file, value, "ItemCapacityPerSize4", forcewrite);
        let value = self.ItemCapacityPerSize5;
        write_tag_i!(file, value, "ItemCapacityPerSize5", forcewrite);
        let value = self.ItemCapacityPerSize6;
        write_tag_i!(file, value, "ItemCapacityPerSize6", forcewrite);
        let value = self.ItemCapacityPerSize7;
        write_tag_i!(file, value, "ItemCapacityPerSize7", forcewrite);
        let value = self.ItemCapacityPerSize8;
        write_tag_i!(file, value, "ItemCapacityPerSize8", forcewrite);
        let value = self.ItemCapacityPerSize9;
        write_tag_i!(file, value, "ItemCapacityPerSize9", forcewrite);
        let value = self.ItemCapacityPerSize10;
        write_tag_i!(file, value, "ItemCapacityPerSize10", forcewrite);
        let value = self.ItemCapacityPerSize11;
        write_tag_i!(file, value, "ItemCapacityPerSize11", forcewrite);
        let value = self.ItemCapacityPerSize12;
        write_tag_i!(file, value, "ItemCapacityPerSize12", forcewrite);
        let value = self.ItemCapacityPerSize13;
        write_tag_i!(file, value, "ItemCapacityPerSize13", forcewrite);
        let value = self.ItemCapacityPerSize14;
        write_tag_i!(file, value, "ItemCapacityPerSize14", forcewrite);
        let value = self.ItemCapacityPerSize15;
        write_tag_i!(file, value, "ItemCapacityPerSize15", forcewrite);
        let value = self.ItemCapacityPerSize16;
        write_tag_i!(file, value, "ItemCapacityPerSize16", forcewrite);
        let value = self.ItemCapacityPerSize17;
        write_tag_i!(file, value, "ItemCapacityPerSize17", forcewrite);
        let value = self.ItemCapacityPerSize18;
        write_tag_i!(file, value, "ItemCapacityPerSize18", forcewrite);
        let value = self.ItemCapacityPerSize19;
        write_tag_i!(file, value, "ItemCapacityPerSize19", forcewrite);
        let value = self.ItemCapacityPerSize20;
        write_tag_i!(file, value, "ItemCapacityPerSize20", forcewrite);
        let value = self.ItemCapacityPerSize21;
        write_tag_i!(file, value, "ItemCapacityPerSize21", forcewrite);
        let value = self.ItemCapacityPerSize22;
        write_tag_i!(file, value, "ItemCapacityPerSize22", forcewrite);
        let value = self.ItemCapacityPerSize23;
        write_tag_i!(file, value, "ItemCapacityPerSize23", forcewrite);
        let value = self.ItemCapacityPerSize24;
        write_tag_i!(file, value, "ItemCapacityPerSize24", forcewrite);
        let value = self.ItemCapacityPerSize25;
        write_tag_i!(file, value, "ItemCapacityPerSize25", forcewrite);
        let value = self.ItemCapacityPerSize26;
        write_tag_i!(file, value, "ItemCapacityPerSize26", forcewrite);
        write_tag_i!(file, value, "ItemCapacityPerSize27", forcewrite);
        let value = self.ItemCapacityPerSize27;
        write_tag_i!(file, value, "ItemCapacityPerSize28", forcewrite);
        let value = self.ItemCapacityPerSize28;
        write_tag_i!(file, value, "ItemCapacityPerSize29", forcewrite);
        let value = self.ItemCapacityPerSize29;
        write_tag_i!(file, value, "ItemCapacityPerSize30", forcewrite);
        let value = self.ItemCapacityPerSize30;
        write_tag_i!(file, value, "ItemCapacityPerSize31", forcewrite);
        let value = self.ItemCapacityPerSize31;
        write_tag_i!(file, value, "ItemCapacityPerSize32", forcewrite);
        let value = self.ItemCapacityPerSize32;
        write_tag_i!(file, value, "ItemCapacityPerSize33", forcewrite);
        let value = self.ItemCapacityPerSize33;
        write_tag_i!(file, value, "ItemCapacityPerSize34", forcewrite);
        let value = self.ItemCapacityPerSize34;

        write!(file, "\t</POCKET>\n").unwrap();
	}
}


pub struct MERGE
{
    firstItemIndex: u32,
    secondItemIndex: u32,
    firstResultingItemIndex: u32,
    secondResultingItemIndex: u32,
    mergeType: u32,
    APCost: u32,
}
impl MERGE {
    pub fn new() -> MERGE
    {
        MERGE { firstItemIndex: 0, secondItemIndex: 0, firstResultingItemIndex: 0, secondResultingItemIndex: 0, mergeType: 0, APCost: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"firstItemIndex" => {self.firstItemIndex = parseu32(reader, buf, &name);}
                        b"secondItemIndex" => {self.secondItemIndex = parseu32(reader, buf, &name);}
                        b"firstResultingItemIndex" => {self.firstResultingItemIndex = parseu32(reader, buf, &name);}
                        b"secondResultingItemIndex" => {self.secondResultingItemIndex = parseu32(reader, buf, &name);}
                        b"mergeType" => {self.mergeType = parseu32(reader, buf, &name);}
                        b"APCost" => {self.APCost = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"MERGE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<MERGE>\n").unwrap();

        let value = self.firstItemIndex;
        write_tag_i!(file, value, "firstItemIndex", forcewrite);
        let value = self.secondItemIndex;
        write_tag_i!(file, value, "secondItemIndex", forcewrite);
        let value = self.firstResultingItemIndex;
        write_tag_i!(file, value, "firstResultingItemIndex", forcewrite);
        let value = self.secondResultingItemIndex;
        write_tag_i!(file, value, "secondResultingItemIndex", forcewrite);
        let value = self.mergeType;
        write_tag_i!(file, value, "mergeType", forcewrite);
        let value = self.APCost;
        write_tag_i!(file, value, "APCost", forcewrite);

		write!(file, "\t</MERGE>\n").unwrap();
	}
}


pub struct LOADBEARINGEQUIPMENT
{
    lbeIndex: u32,
    lbeClass: u32,
    lbeCombo: u32,
    lbeFilledSize: u32,
    lbeAvailableVolume: u32,
    lbePocketsAvailable: u32,
    lbePocketIndex1: u32,
    lbePocketIndex2: u32,
    lbePocketIndex3: u32,
    lbePocketIndex4: u32,
    lbePocketIndex5: u32,
    lbePocketIndex6: u32,
    lbePocketIndex7: u32,
    lbePocketIndex8: u32,
    lbePocketIndex9: u32,
    lbePocketIndex10: u32,
    lbePocketIndex11: u32,
    lbePocketIndex12: u32
}
impl LOADBEARINGEQUIPMENT {
    pub fn new() -> LOADBEARINGEQUIPMENT
    {
        LOADBEARINGEQUIPMENT { lbeIndex: 0, lbeClass: 0, lbeCombo: 0, lbeFilledSize: 0, lbeAvailableVolume: 0, lbePocketsAvailable: 0, lbePocketIndex1: 0, lbePocketIndex2: 0, lbePocketIndex3: 0, lbePocketIndex4: 0, lbePocketIndex5: 0, lbePocketIndex6: 0, lbePocketIndex7: 0, lbePocketIndex8: 0, lbePocketIndex9: 0, lbePocketIndex10: 0, lbePocketIndex11: 0, lbePocketIndex12: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"lbeIndex" => {self.lbeIndex = parseu32(reader, buf, &name);}
                        b"lbeClass" => {self.lbeClass = parseu32(reader, buf, &name);}
                        b"lbeCombo" => {self.lbeCombo = parseu32(reader, buf, &name);}
                        b"lbeFilledSize" => {self.lbeFilledSize = parseu32(reader, buf, &name);}
                        b"lbeAvailableVolume" => {self.lbeAvailableVolume = parseu32(reader, buf, &name);}
                        b"lbePocketsAvailable" => {self.lbePocketsAvailable = parseu32(reader, buf, &name);}
                        b"lbePocketIndex1" => {self.lbePocketIndex1 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex2" => {self.lbePocketIndex2 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex3" => {self.lbePocketIndex3 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex4" => {self.lbePocketIndex4 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex5" => {self.lbePocketIndex5 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex6" => {self.lbePocketIndex6 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex7" => {self.lbePocketIndex7 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex8" => {self.lbePocketIndex8 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex9" => {self.lbePocketIndex9 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex10" => {self.lbePocketIndex10 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex11" => {self.lbePocketIndex11 = parseu32(reader, buf, &name);}
                        b"lbePocketIndex12" => {self.lbePocketIndex12 = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"LOADBEARINGEQUIPMENT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<LOADBEARINGEQUIPMENT>\n").unwrap();

        let value = self.lbeIndex;
        write_tag_i!(file, value, "lbeIndex", forcewrite);
        let value = self.lbeClass;
        write_tag_i!(file, value, "lbeClass", forcewrite);
        let value = self.lbeCombo;
        write_tag_i!(file, value, "lbeCombo", forcewrite);
        let value = self.lbeFilledSize;
        write_tag_i!(file, value, "lbeFilledSize", forcewrite);
        let value = self.lbeAvailableVolume;
        write_tag_i!(file, value, "lbeAvailableVolume", forcewrite);
        let value = self.lbePocketsAvailable;
        write_tag_i!(file, value, "lbePocketsAvailable", forcewrite);
        let value = self.lbePocketIndex1;
        write_tag_i!(file, value, "lbePocketIndex1", forcewrite);
        let value = self.lbePocketIndex2;
        write_tag_i!(file, value, "lbePocketIndex2", forcewrite);
        let value = self.lbePocketIndex3;
        write_tag_i!(file, value, "lbePocketIndex3", forcewrite);
        let value = self.lbePocketIndex4;
        write_tag_i!(file, value, "lbePocketIndex4", forcewrite);
        let value = self.lbePocketIndex5;
        write_tag_i!(file, value, "lbePocketIndex5", forcewrite);
        let value = self.lbePocketIndex6;
        write_tag_i!(file, value, "lbePocketIndex6", forcewrite);
        let value = self.lbePocketIndex7;
        write_tag_i!(file, value, "lbePocketIndex7", forcewrite);
        let value = self.lbePocketIndex8;
        write_tag_i!(file, value, "lbePocketIndex8", forcewrite);
        let value = self.lbePocketIndex9;
        write_tag_i!(file, value, "lbePocketIndex9", forcewrite);
        let value = self.lbePocketIndex10;
        write_tag_i!(file, value, "lbePocketIndex10", forcewrite);
        let value = self.lbePocketIndex11;
        write_tag_i!(file, value, "lbePocketIndex11", forcewrite);
        let value = self.lbePocketIndex12;
        write_tag_i!(file, value, "lbePocketIndex12", forcewrite);

		write!(file, "\t</LOADBEARINGEQUIPMENT>\n").unwrap();
	}
}



pub struct LAUNCHABLE
{
    launchableIndex: u32,
    itemIndex: u32,
}
impl LAUNCHABLE {
    pub fn new() -> LAUNCHABLE
    {
        LAUNCHABLE { launchableIndex: 0, itemIndex: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"launchableIndex" => {self.launchableIndex = parseu32(reader, buf, &name);}
                        b"itemIndex" => {self.itemIndex = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"LAUNCHABLE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<LAUNCHABLE>\n").unwrap();

        let value = self.launchableIndex;
        write_tag_i!(file, value, "launchableIndex", forcewrite);
        let value = self.itemIndex;
        write_tag_i!(file, value, "itemIndex", forcewrite);

        write!(file, "\t</LAUNCHABLE>\n").unwrap();
	}
}



pub struct TRANSFORM
{
    usItem: u32,
    usResult1: u32,
    usResult2: u32,
    usResult3: u32,
    usResult4: u32,
    usResult5: u32,
    usResult6: u32,
    usResult7: u32,
    usResult8: u32,
    usResult9: u32,
    usResult10: u32,
    usAPCost: u32,
    iBPCost: u32,
    szMenuRowText: String,
    szTooltipText: String,
}
impl TRANSFORM {
    pub fn new() -> TRANSFORM
    {
        TRANSFORM { usItem: 0, usResult1: 0, usResult2: 0, usResult3: 0, usResult4: 0, usResult5: 0, usResult6: 0, usResult7: 0, usResult8: 0, usResult9: 0, usResult10: 0, usAPCost: 0, iBPCost: 0, szMenuRowText: "".to_string(), szTooltipText: "".to_string() }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"usItem" => {self.usItem = parseu32(reader, buf, &name);}
                        b"usResult1" => {self.usResult1 = parseu32(reader, buf, &name);}
                        b"usResult2" => {self.usResult2 = parseu32(reader, buf, &name);}
                        b"usResult3" => {self.usResult3 = parseu32(reader, buf, &name);}
                        b"usResult4" => {self.usResult4 = parseu32(reader, buf, &name);}
                        b"usResult5" => {self.usResult5 = parseu32(reader, buf, &name);}
                        b"usResult6" => {self.usResult6 = parseu32(reader, buf, &name);}
                        b"usResult7" => {self.usResult7 = parseu32(reader, buf, &name);}
                        b"usResult8" => {self.usResult8 = parseu32(reader, buf, &name);}
                        b"usResult9" => {self.usResult9 = parseu32(reader, buf, &name);}
                        b"usResult10" => {self.usResult10 = parseu32(reader, buf, &name);}
                        b"usAPCost" => {self.usAPCost = parseu32(reader, buf, &name);}
                        b"iBPCost" => {self.iBPCost = parseu32(reader, buf, &name);}
                        b"szMenuRowText" => {self.szMenuRowText = parseString(reader, buf);}
                        b"szTooltipText" => {self.szTooltipText = parseString(reader, buf);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"TRANSFORM" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<TRANSFORM>\n").unwrap();

        let value = self.usItem;
        write_tag_i!(file, value, "usItem", forcewrite);
        let value = self.usResult1;
        write_tag_i!(file, value, "usResult1", forcewrite);
        let value = self.usResult2;
        write_tag_i!(file, value, "usResult2", forcewrite);
        let value = self.usResult3;
        write_tag_i!(file, value, "usResult3", forcewrite);
        let value = self.usResult4;
        write_tag_i!(file, value, "usResult4", forcewrite);
        let value = self.usResult5;
        write_tag_i!(file, value, "usResult5", forcewrite);
        let value = self.usResult6;
        write_tag_i!(file, value, "usResult6", forcewrite);
        let value = self.usResult7;
        write_tag_i!(file, value, "usResult7", forcewrite);
        let value = self.usResult8;
        write_tag_i!(file, value, "usResult8", forcewrite);
        let value = self.usResult9;
        write_tag_i!(file, value, "usResult9", forcewrite);
        let value = self.usResult10;
        write_tag_i!(file, value, "usResult10", forcewrite);
        let value = self.usAPCost;
        write_tag_i!(file, value, "usAPCost", forcewrite);
        let value = self.iBPCost;
        write_tag_i!(file, value, "iBPCost", forcewrite);
        let value = &self.szMenuRowText;
        write_tag_s!(file, value, "szMenuRowText", forcewrite);
        let value = &self.szTooltipText;
        write_tag_s!(file, value, "szTooltipText", forcewrite);

		write!(file, "\t</TRANSFORM>\n").unwrap();
	}
}


pub struct INCOMPATIBLEATTACHMENT
{
    itemIndex: u32,
    incompatibleattachmentIndex: u32,
}
impl INCOMPATIBLEATTACHMENT {
    pub fn new() -> INCOMPATIBLEATTACHMENT
    {
        INCOMPATIBLEATTACHMENT { itemIndex: 0, incompatibleattachmentIndex: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"itemIndex" => {self.itemIndex = parseu32(reader, buf, &name);}
                        b"incompatibleattachmentIndex" => {self.incompatibleattachmentIndex = parseu32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"INCOMPATIBLEATTACHMENT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<INCOMPATIBLEATTACHMENT>\n").unwrap();

        let value = self.itemIndex;
        write_tag_i!(file, value, "itemIndex", forcewrite);
        let value = self.incompatibleattachmentIndex;
        write_tag_i!(file, value, "incompatibleattachmentIndex", forcewrite);

        write!(file, "\t</INCOMPATIBLEATTACHMENT>\n").unwrap();
	}
}


pub struct FOODOPINION
{
    uProfile: u32,
    opinions: Vec<PROFILE_OPINION>,
}
impl FOODOPINION {
    pub fn new() -> FOODOPINION
    {
        FOODOPINION { uProfile: 0, opinions: Vec::new()}
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uProfile" => { self.uProfile = parseu32(reader, buf, &name); }
                        b"PROFILE_OPINION" => {
                            self.opinions.push(PROFILE_OPINION::new());
                            self.opinions.last_mut().unwrap().readItem(reader, buf);
                        }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"FOODOPINION" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<FOODOPINION>\n").unwrap();

		let value = self.uProfile;
		write_tag_i!(file, value, "uProfile", forcewrite);

        for opinion in &self.opinions
        {
            opinion.save(file, forcewrite);
        }

        write!(file, "\t</FOODOPINION>\n").unwrap();
	}
}


pub struct PROFILE_OPINION
{
    FoodNumber: u32,
    MoraleMod: i32,
}
impl PROFILE_OPINION
{
    pub fn new() -> PROFILE_OPINION
    {
        PROFILE_OPINION { FoodNumber: 0, MoraleMod: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"FoodNumber" => { self.FoodNumber = parseu32(reader, buf, &name); }
                        b"MoraleMod" => { self.MoraleMod = parsei32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"PROFILE_OPINION" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t\t<PROFILE_OPINION>\n").unwrap();

		let value = self.FoodNumber;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "FoodNumber", forcewrite);
        }
        
        let value = self.MoraleMod;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "MoraleMod", forcewrite);
        }
		
        write!(file, "\t\t</PROFILE_OPINION>\n").unwrap();
	}
}


pub struct FOOD
{
    uiIndex: u32,
    szName: String,
    bFoodPoints: u32,
    bDrinkPoints: u32,
    usDecayRate: f32,
}
impl FOOD {
    pub fn new() -> FOOD
    {
        FOOD { uiIndex: 0, szName: "".to_string(), bFoodPoints: 0, bDrinkPoints: 0, usDecayRate: 0.0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"uiIndex" => {self.uiIndex = parseu32(reader, buf, &name);}
                        b"szName" => {self.szName = parseString(reader, buf);}
                        b"bFoodPoints" => {self.bFoodPoints = parseu32(reader, buf, &name);}
                        b"bDrinkPoints" => {self.bDrinkPoints = parseu32(reader, buf, &name);}
                        b"usDecayRate" => {self.usDecayRate = parsef32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"FOOD" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<FOOD>\n").unwrap();

        let value = self.uiIndex;
        write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.szName;
        write_tag_s!(file, value, "szName", forcewrite);
        let value = self.bFoodPoints;
        write_tag_i!(file, value, "bFoodPoints", forcewrite);
        let value = self.bDrinkPoints;
        write_tag_i!(file, value, "bDrinkPoints", forcewrite);
        let value = self.usDecayRate;
        write_tag_f!(file, value, "usDecayRate", forcewrite);

        write!(file, "\t</FOOD>\n").unwrap();
	}
}


pub struct EXPLOSIVE
{
    pub uiIndex: u32,
    pub ubType: u32,
    pub ubDamage: u32,
    pub ubStunDamage: u32,
    pub ubRadius: u32,
    pub ubVolume: u32,
    pub ubVolatility: u32,
    pub ubAnimationID: u32,
    pub ubDuration: u32,
    pub ubStartRadius: u32,
    pub ubMagSize: u32,
    pub fExplodeOnImpact: bool,
    pub usNumFragments: u32,
    pub ubFragType: u32,
    pub ubFragDamage: u32,
    pub ubFragRange: u32,
    pub ubHorizontalDegree: u32,
    pub ubVerticalDegree: u32,
	pub bIndoorModifier: f32,
}
impl EXPLOSIVE {
    pub fn new() -> EXPLOSIVE
    {
        EXPLOSIVE { uiIndex: 0, ubType: 0, ubDamage: 0, ubStunDamage: 0, ubRadius: 0, ubVolume: 0, ubVolatility: 0, ubAnimationID: 0, ubDuration: 0, ubStartRadius: 0, ubMagSize: 0, fExplodeOnImpact: false, usNumFragments: 0, ubFragType: 0, ubFragDamage: 0, ubFragRange: 0, ubHorizontalDegree: 0, ubVerticalDegree: 0, bIndoorModifier: 0.0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
                        b"uiIndex" => {self.uiIndex = parseu32(reader, buf, &name);}
                        b"ubType" => {self.ubType = parseu32(reader, buf, &name);}
                        b"ubDamage" => {self.ubDamage = parseu32(reader, buf, &name);}
                        b"ubStunDamage" => {self.ubStunDamage = parseu32(reader, buf, &name);}
                        b"ubRadius" => {self.ubRadius = parseu32(reader, buf, &name);}
                        b"ubVolume" => {self.ubVolume = parseu32(reader, buf, &name);}
                        b"ubVolatility" => {self.ubVolatility = parseu32(reader, buf, &name);}
                        b"ubAnimationID" => {self.ubAnimationID = parseu32(reader, buf, &name);}
                        b"ubDuration" => {self.ubDuration = parseu32(reader, buf, &name);}
                        b"ubStartRadius" => {self.ubStartRadius = parseu32(reader, buf, &name);}
                        b"ubMagSize" => {self.ubMagSize = parseu32(reader, buf, &name);}
                        b"fExplodeOnImpact" => {self.fExplodeOnImpact = parsebool(reader, buf, &name);}
                        b"usNumFragments" => {self.usNumFragments = parseu32(reader, buf, &name);}
                        b"ubFragType" => {self.ubFragType = parseu32(reader, buf, &name);}
                        b"ubFragDamage" => {self.ubFragDamage = parseu32(reader, buf, &name);}
                        b"ubFragRange" => {self.ubFragRange = parseu32(reader, buf, &name);}
                        b"ubHorizontalDegree" => {self.ubHorizontalDegree = parseu32(reader, buf, &name);}
                        b"ubVerticalDegree" => {self.ubVerticalDegree = parseu32(reader, buf, &name);}
                        b"bIndoorModifier" => {self.bIndoorModifier = parsef32(reader, buf, &name);}
                        _ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"EXPLOSIVE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<EXPLOSIVE>\n").unwrap();

        let value = self.uiIndex;
        write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = self.ubType;
        write_tag_i!(file, value, "ubType", forcewrite);
        let value = self.ubDamage;
        write_tag_i!(file, value, "ubDamage", forcewrite);
        let value = self.ubStunDamage;
        write_tag_i!(file, value, "ubStunDamage", forcewrite);
        let value = self.ubRadius;
        write_tag_i!(file, value, "ubRadius", forcewrite);
        let value = self.ubVolume;
        write_tag_i!(file, value, "ubVolume", forcewrite);
        let value = self.ubVolatility;
        write_tag_i!(file, value, "ubVolatility", forcewrite);
        let value = self.ubAnimationID;
        write_tag_i!(file, value, "ubAnimationID", forcewrite);
        let value = self.ubDuration;
        write_tag_i!(file, value, "ubDuration", forcewrite);
        let value = self.ubStartRadius;
        write_tag_i!(file, value, "ubStartRadius", forcewrite);
        let value = self.ubMagSize;
        write_tag_i!(file, value, "ubMagSize", forcewrite);
        let value = self.fExplodeOnImpact as u32;
        write_tag_i!(file, value, "fExplodeOnImpact", forcewrite);
        let value = self.usNumFragments;
        write_tag_i!(file, value, "usNumFragments", forcewrite);
        let value = self.ubFragType;
        write_tag_i!(file, value, "ubFragType", forcewrite);
        let value = self.ubFragDamage;
        write_tag_i!(file, value, "ubFragDamage", forcewrite);
        let value = self.ubFragRange;
        write_tag_i!(file, value, "ubFragRange", forcewrite);
        let value = self.ubHorizontalDegree;
        write_tag_i!(file, value, "ubHorizontalDegree", forcewrite);
        let value = self.ubVerticalDegree;
        write_tag_i!(file, value, "ubVerticalDegree", forcewrite);
        let value = self.bIndoorModifier;
        write_tag_f!(file, value, "bIndoorModifier", forcewrite);

		write!(file, "\t</EXPLOSIVE>\n").unwrap();
	}
}


pub struct EXPDATA
{
    pub uiIndex: u32,
    pub name: String,
    pub TransKeyFrame: u32,
    pub DamageKeyFrame: u32,
    pub ExplosionSoundID: u32,
    pub AltExplosionSoundID: i32,
    pub BlastFilename: String,
    pub BlastSpeed: u32,
}
impl EXPDATA {
    pub fn new() -> EXPDATA
    {
        EXPDATA { uiIndex: 0, name: "".to_string(), TransKeyFrame: 0, DamageKeyFrame: 0, ExplosionSoundID: 0, AltExplosionSoundID: 0, BlastFilename: "".to_string(), BlastSpeed: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
                        b"name" => { self.name = parseString(reader, buf); }
                        b"TransKeyFrame" => { self.TransKeyFrame = parseu32(reader, buf, &name); }
                        b"DamageKeyFrame" => { self.DamageKeyFrame = parseu32(reader, buf, &name); }
                        b"ExplosionSoundID" => { self.ExplosionSoundID = parseu32(reader, buf, &name); }
                        b"AltExplosionSoundID" => { self.AltExplosionSoundID = parsei32(reader, buf, &name); }
                        b"BlastFilename" => { self.BlastFilename = parseString(reader, buf); }
                        b"BlastSpeed" => { self.BlastSpeed = parseu32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"EXPDATA" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<EXPDATA>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.name;
        write_tag_s!(file, value, "name", forcewrite);
        let value = self.TransKeyFrame;
        write_tag_i!(file, value, "TransKeyFrame", forcewrite);
        let value = self.DamageKeyFrame;
        write_tag_i!(file, value, "DamageKeyFrame", forcewrite);
        let value = self.ExplosionSoundID;
        write_tag_i!(file, value, "ExplosionSoundID", forcewrite);
        let value = self.AltExplosionSoundID;
        write_tag_i!(file, value, "AltExplosionSoundID", forcewrite);
        let value = &self.BlastFilename;
        write_tag_s!(file, value, "BlastFilename", forcewrite);
        let value = self.BlastSpeed;
        write_tag_i!(file, value, "BlastSpeed", forcewrite);

		write!(file, "\t</EXPDATA>\n").unwrap();
	}
}


pub struct DRUG
{
    uiIndex: u32,
    szName: String,
    opinionevent: bool,
    drugEffects: Vec<DRUGEFFECT>,
    diseaseEffects: Vec<DISEASEEFFECT>,
    disabilityEffects: Vec<DISABILITYEFFECT>,
    personalityEffects: Vec<PERSONALITYEFFECT>
}
impl DRUG {
    pub fn new() -> DRUG
    {
        DRUG { uiIndex: 0, szName: "".to_string(), opinionevent: false, drugEffects: Vec::new(),
        diseaseEffects: Vec::new(), disabilityEffects: Vec::new(), personalityEffects: Vec::new()}
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
                        b"szName" => { self.szName = parseString(reader, buf); }
						b"opinionevent" => { self.opinionevent = parsebool(reader, buf, &name); }
                        b"DRUG_EFFECT" => {
                            self.drugEffects.push(DRUGEFFECT::new());
                            self.drugEffects.last_mut().unwrap().readItem(reader, buf);
                        }
                        b"DISEASE_EFFECT" => {
                            self.diseaseEffects.push(DISEASEEFFECT::new());
                            self.diseaseEffects.last_mut().unwrap().readItem(reader, buf);
                        }
                        b"DISABILITY_EFFECT" => {
                            self.disabilityEffects.push(DISABILITYEFFECT::new());
                            self.disabilityEffects.last_mut().unwrap().readItem(reader, buf);
                        }
                        b"PERSONALITY_EFFECT" => {
                            self.personalityEffects.push(PERSONALITYEFFECT::new());
                            self.personalityEffects.last_mut().unwrap().readItem(reader, buf);
                        }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"DRUG" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<DRUG>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.szName;
        write_tag_s!(file, value, "szName", forcewrite);
		let value = self.opinionevent as u32;
		write_tag_i!(file, value, "opinionevent", forcewrite);

        for effect in &self.drugEffects
        {
            effect.save(file, forcewrite);
        }
        for effect in &self.diseaseEffects
        {
            effect.save(file, forcewrite);
        }
        for effect in &self.disabilityEffects
        {
            effect.save(file, forcewrite);
        }
        for effect in &self.personalityEffects
        {
            effect.save(file, forcewrite);
        }

        write!(file, "\t</DRUG>\n").unwrap();
	}
}


pub struct DRUGEFFECT
{
    effect: u8,
    duration: u32,
    size: i32,
    chance: u8
}
impl DRUGEFFECT
{
    pub fn new() -> DRUGEFFECT
    {
        DRUGEFFECT { effect: 0, duration: 0, size: 0, chance: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"effect" => { self.effect = parseu8(reader, buf, &name); }
                        b"duration" => { self.duration = parseu32(reader, buf, &name); }
                        b"size" => { self.size = parsei32(reader, buf, &name); }
                        b"chance" => { self.chance = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"DRUG_EFFECT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t\t<DRUG_EFFECT>\n").unwrap();

		let value = self.effect;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "effect", forcewrite);
        }
        
        let value = self.duration;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "duration", forcewrite);
        }
		
        let value = self.size;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "size", forcewrite);
        }

        let value = self.chance;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "chance", forcewrite);
        }

        write!(file, "\t\t</DRUG_EFFECT>\n").unwrap();
	}
}


pub struct DISEASEEFFECT
{
    disease: u8,
    size: i32,
    chance: u8
}
impl DISEASEEFFECT
{
    pub fn new() -> DISEASEEFFECT
    {
        DISEASEEFFECT { disease: 0, size: 0, chance: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"disease" => { self.disease = parseu8(reader, buf, &name); }
                        b"size" => { self.size = parsei32(reader, buf, &name); }
                        b"chance" => { self.chance = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"DISEASE_EFFECT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t\t<DISEASE_EFFECT>\n").unwrap();

		let value = self.disease;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "disease", forcewrite);
        }
        		
        let value = self.size;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "size", forcewrite);
        }

        let value = self.chance;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "chance", forcewrite);
        }

        write!(file, "\t\t</DISEASE_EFFECT>\n").unwrap();
	}
}


pub struct DISABILITYEFFECT
{
    disability: u8,
    duration: u32,
    chance: u8
}
impl DISABILITYEFFECT
{
    pub fn new() -> DISABILITYEFFECT
    {
        DISABILITYEFFECT { disability: 0, duration: 0, chance: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"disability" => { self.disability = parseu8(reader, buf, &name); }
                        b"duration" => { self.duration = parseu32(reader, buf, &name); }
                        b"chance" => { self.chance = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"DISABILITY_EFFECT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t\t<DISABILITY_EFFECT>\n").unwrap();

		let value = self.disability;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "disability", forcewrite);
        }
        
        let value = self.duration;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "duration", forcewrite);
        }
		
        let value = self.chance;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "chance", forcewrite);
        }

        write!(file, "\t\t</DISABILITY_EFFECT>\n").unwrap();
	}
}


pub struct PERSONALITYEFFECT
{
    personality: u8,
    duration: u32,
    chance: u8
}
impl PERSONALITYEFFECT
{
    pub fn new() -> PERSONALITYEFFECT
    {
        PERSONALITYEFFECT { personality: 0, duration: 0, chance: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"personality" => { self.personality = parseu8(reader, buf, &name); }
                        b"duration" => { self.duration = parseu32(reader, buf, &name); }
                        b"chance" => { self.chance = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"PERSONALITY_EFFECT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t\t<PERSONALITY_EFFECT>\n").unwrap();

		let value = self.personality;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "personality", forcewrite);
        }
        
        let value = self.duration;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "duration", forcewrite);
        }
		
        let value = self.chance;
        if value != 0 || forcewrite == true
        {
    		write!(file, "\t").unwrap();
            write_tag_i!(file, value, "chance", forcewrite);
        }

        write!(file, "\t\t</PERSONALITY_EFFECT>\n").unwrap();
	}
}


pub struct COMPATIBLEFACEITEM
{
    compatiblefaceitemIndex: u32,
    itemIndex: u32,
}
impl COMPATIBLEFACEITEM {
    pub fn new() -> COMPATIBLEFACEITEM
    {
        COMPATIBLEFACEITEM { compatiblefaceitemIndex: 0, itemIndex: 0}
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"compatiblefaceitemIndex" => { self.compatiblefaceitemIndex = parseu32(reader, buf, &name); }
                        b"itemIndex" => { self.itemIndex = parseu32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"COMPATIBLEFACEITEM" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<COMPATIBLEFACEITEM>\n").unwrap();

		let value = self.compatiblefaceitemIndex;
		write_tag_i!(file, value, "compatiblefaceitemIndex", forcewrite);
        let value = self.itemIndex;
        write_tag_i!(file, value, "itemIndex", forcewrite);

		write!(file, "\t</COMPATIBLEFACEITEM>\n").unwrap();
	}
}


pub struct CLOTHES
{
    pub uiIndex: u32,
    pub szName: String,
    pub Vest: String,
    pub Pants: String,
}
impl CLOTHES {
    pub fn new() -> CLOTHES
    {
        CLOTHES { uiIndex: 0, szName: "".to_string(), Vest: "".to_string(), Pants: "".to_string()}
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
                        b"szName" => { self.szName = parseString(reader, buf); }
                        b"Vest" => { self.Vest = parseString(reader, buf); }
                        b"Pants" => { self.Pants = parseString(reader, buf); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"CLOTHES" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<CLOTHES>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.szName;
        write_tag_s!(file, value, "szName", forcewrite);
        let value = &self.Vest;
        write_tag_s!(file, value, "Vest", forcewrite);
        let value = &self.Pants;
        write_tag_s!(file, value, "Pants", forcewrite);

		write!(file, "\t</CLOTHES>\n").unwrap();
	}
}


pub struct ATTACHMENTSLOT
{
    uiSlotIndex: u32,
    szSlotName: String,
    nasAttachmentClass: u32,
    nasLayoutClass: u32,
    usDescPanelPosX: u32,
    usDescPanelPosY: u32,
    fMultiShot: bool,
    fBigSlot: bool,
    ubPocketMapping: u32,
}
impl ATTACHMENTSLOT {
    pub fn new() -> ATTACHMENTSLOT
    {
        ATTACHMENTSLOT { uiSlotIndex: 0, szSlotName: "".to_string(), nasAttachmentClass: 0, nasLayoutClass: 0, usDescPanelPosX: 0, usDescPanelPosY: 0, fMultiShot: false, fBigSlot: false,
            ubPocketMapping: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiSlotIndex" => { self.uiSlotIndex = parseu32(reader, buf, &name); }
                        b"szSlotName" => { self.szSlotName = parseString(reader, buf); }
                        b"nasAttachmentClass" => { self.nasAttachmentClass = parseu32(reader, buf, &name); }
                        b"nasLayoutClass" => { self.nasLayoutClass = parseu32(reader, buf, &name); }
                        b"usDescPanelPosX" => { self.usDescPanelPosX = parseu32(reader, buf, &name); }
                        b"usDescPanelPosY" => { self.usDescPanelPosY = parseu32(reader, buf, &name); }
                        b"fMultiShot" => { self.fMultiShot = parsebool(reader, buf, &name); }
                        b"fBigSlot" => { self.fBigSlot = parsebool(reader, buf, &name); }
                        b"ubPocketMapping" => { self.ubPocketMapping = parseu32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ATTACHMENTSLOT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ATTACHMENTSLOT>\n").unwrap();

		let value = self.uiSlotIndex;
		write_tag_i!(file, value, "uiSlotIndex", forcewrite);
        let value = &self.szSlotName;
        write_tag_s!(file, value, "szSlotName", forcewrite);
        let value = self.nasAttachmentClass;
        write_tag_i!(file, value, "nasAttachmentClass", forcewrite);
        let value = self.nasLayoutClass;
        write_tag_i!(file, value, "nasLayoutClass", forcewrite);
        let value = self.usDescPanelPosX;
        write_tag_i!(file, value, "usDescPanelPosX", forcewrite);
        let value = self.usDescPanelPosY;
        write_tag_i!(file, value, "usDescPanelPosY", forcewrite);
        let value = self.fMultiShot as u32;
        write_tag_i!(file, value, "fMultiShot", forcewrite);
        let value = self.fBigSlot as u32;
        write_tag_i!(file, value, "fBigSlot", forcewrite);
        let value = self.ubPocketMapping;
        write_tag_i!(file, value, "ubPocketMapping", forcewrite);

		write!(file, "\t</ATTACHMENTSLOT>\n").unwrap();
	}
}


pub struct ATTACHMENT
{
    pub attachmentIndex: u32,
    pub itemIndex: u32,
    pub APCost: u32,
    pub NASOnly: bool
}
impl ATTACHMENT {
    pub fn new() -> ATTACHMENT
    {
        ATTACHMENT { attachmentIndex: 0, itemIndex: 0, APCost: 0, NASOnly: false}
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"attachmentIndex" => { self.attachmentIndex = parseu32(reader, buf, &name); }
                        b"itemIndex" => { self.itemIndex = parseu32(reader, buf, &name); }
                        b"APCost" => { self.APCost = parseu32(reader, buf, &name); }
                        b"NASOnly" => { self.NASOnly = parsebool(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ATTACHMENT" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ATTACHMENT>\n").unwrap();

		let value = self.attachmentIndex;
		write_tag_i!(file, value, "attachmentIndex", forcewrite);
        let value = self.itemIndex;
        write_tag_i!(file, value, "itemIndex", forcewrite);
        let value = self.APCost;
        write_tag_i!(file, value, "APCost", forcewrite);
        let value = self.NASOnly as u32;
        write_tag_i!(file, value, "NASOnly", forcewrite);

        write!(file, "\t</ATTACHMENT>\n").unwrap();
	}
}


pub struct ATTACHMENTINFO
{
    uiIndex: u32,
    usItem: u32,
    uiItemClass: u32,
    bAttachmentSkillCheck: u32,
    bAttachmentSkillCheckMod: i32,
}
impl ATTACHMENTINFO {
    pub fn new() -> ATTACHMENTINFO
    {
        ATTACHMENTINFO { uiIndex: 0, usItem: 0, uiItemClass: 0, bAttachmentSkillCheck: 0, bAttachmentSkillCheckMod: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
                        b"usItem" => { self.usItem = parseu32(reader, buf, &name); }
                        b"uiItemClass" => { self.uiItemClass = parseu32(reader, buf, &name); }
                        b"bAttachmentSkillCheck" => { self.bAttachmentSkillCheck = parseu32(reader, buf, &name); }
                        b"bAttachmentSkillCheckMod" => { self.bAttachmentSkillCheckMod = parsei32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ATTACHMENTINFO" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ATTACHMENTINFO>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = self.usItem;
        write_tag_i!(file, value, "usItem", forcewrite);
        let value = self.uiItemClass;
        write_tag_i!(file, value, "uiItemClass", forcewrite);
        let value = self.bAttachmentSkillCheck;
        write_tag_i!(file, value, "bAttachmentSkillCheck", forcewrite);
        let value = self.bAttachmentSkillCheckMod;
        write_tag_i!(file, value, "bAttachmentSkillCheckMod", forcewrite);

        write!(file, "\t</ATTACHMENTINFO>\n").unwrap();
	}
}


pub struct ATTACHMENTCOMBOMERGE
{
    uiIndex: u32,
    usItem: u32,
    usAttachment1: u32,
    usAttachment2: u32,
    usAttachment3: u32,
    usAttachment4: u32,
    usAttachment5: u32,
    usAttachment6: u32,
    usAttachment7: u32,
    usAttachment8: u32,
    usAttachment9: u32,
    usAttachment10: u32,
    usAttachment11: u32,
    usAttachment12: u32,
    usAttachment13: u32,
    usAttachment14: u32,
    usAttachment15: u32,
    usAttachment16: u32,
    usAttachment17: u32,
    usAttachment18: u32,
    usAttachment19: u32,
    usAttachment20: u32,
    usResult: u32
}
impl ATTACHMENTCOMBOMERGE {
    pub fn new() -> ATTACHMENTCOMBOMERGE
    {
        ATTACHMENTCOMBOMERGE { uiIndex: 0, usItem: 0, usAttachment1: 0, usAttachment2: 0, usAttachment3: 0, usAttachment4: 0, usAttachment5: 0, usAttachment6: 0,
            usAttachment7: 0, usAttachment8: 0, usAttachment9: 0, usAttachment10: 0, usAttachment11: 0, usAttachment12: 0, usAttachment13: 0, usAttachment14: 0,
            usAttachment15: 0, usAttachment16: 0, usAttachment17: 0, usAttachment18: 0, usAttachment19: 0, usAttachment20: 0, usResult: 0 }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
                        b"usItem" => { self.usItem = parseu32(reader, buf, &name); }
                        b"usAttachment1" => { self.usAttachment1 = parseu32(reader, buf, &name); }
                        b"usAttachment2" => { self.usAttachment2 = parseu32(reader, buf, &name); }
                        b"usAttachment3" => { self.usAttachment3 = parseu32(reader, buf, &name); }
                        b"usAttachment4" => { self.usAttachment4 = parseu32(reader, buf, &name); }
                        b"usAttachment5" => { self.usAttachment5 = parseu32(reader, buf, &name); }
                        b"usAttachment6" => { self.usAttachment6 = parseu32(reader, buf, &name); }
                        b"usAttachment7" => { self.usAttachment7 = parseu32(reader, buf, &name); }
                        b"usAttachment8" => { self.usAttachment8 = parseu32(reader, buf, &name); }
                        b"usAttachment9" => { self.usAttachment9 = parseu32(reader, buf, &name); }
                        b"usAttachment10" => { self.usAttachment10 = parseu32(reader, buf, &name); }
                        b"usAttachment11" => { self.usAttachment11 = parseu32(reader, buf, &name); }
                        b"usAttachment12" => { self.usAttachment12 = parseu32(reader, buf, &name); }
                        b"usAttachment13" => { self.usAttachment13 = parseu32(reader, buf, &name); }
                        b"usAttachment14" => { self.usAttachment14 = parseu32(reader, buf, &name); }
                        b"usAttachment15" => { self.usAttachment15 = parseu32(reader, buf, &name); }
                        b"usAttachment16" => { self.usAttachment16 = parseu32(reader, buf, &name); }
                        b"usAttachment17" => { self.usAttachment17 = parseu32(reader, buf, &name); }
                        b"usAttachment18" => { self.usAttachment18 = parseu32(reader, buf, &name); }
                        b"usAttachment19" => { self.usAttachment19 = parseu32(reader, buf, &name); }
                        b"usAttachment20" => { self.usAttachment20 = parseu32(reader, buf, &name); }
                        b"usResult" => { self.usResult = parseu32(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ATTACHMENTCOMBOMERGE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ATTACHMENTCOMBOMERGE>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = self.usItem;
        write_tag_i!(file, value, "usItem", forcewrite);
        let value = self.usAttachment1;
        write_tag_i!(file, value, "usAttachment1", forcewrite);
        let value = self.usAttachment2;
        write_tag_i!(file, value, "usAttachment2", forcewrite);
        let value = self.usAttachment3;
        write_tag_i!(file, value, "usAttachment3", forcewrite);
        let value = self.usAttachment4;
        write_tag_i!(file, value, "usAttachment4", forcewrite);
        let value = self.usAttachment5;
        write_tag_i!(file, value, "usAttachment5", forcewrite);
        let value = self.usAttachment6;
        write_tag_i!(file, value, "usAttachment6", forcewrite);
        let value = self.usAttachment7;
        write_tag_i!(file, value, "usAttachment7", forcewrite);
        let value = self.usAttachment8;
        write_tag_i!(file, value, "usAttachment8", forcewrite);
        let value = self.usAttachment9;
        write_tag_i!(file, value, "usAttachment9", forcewrite);
        let value = self.usAttachment10;
        write_tag_i!(file, value, "usAttachment10", forcewrite);
        let value = self.usAttachment11;
        write_tag_i!(file, value, "usAttachment11", forcewrite);
        let value = self.usAttachment12;
        write_tag_i!(file, value, "usAttachment12", forcewrite);
        let value = self.usAttachment13;
        write_tag_i!(file, value, "usAttachment13", forcewrite);
        let value = self.usAttachment14;
        write_tag_i!(file, value, "usAttachment14", forcewrite);
        let value = self.usAttachment15;
        write_tag_i!(file, value, "usAttachment15", forcewrite);
        let value = self.usAttachment16;
        write_tag_i!(file, value, "usAttachment16", forcewrite);
        let value = self.usAttachment17;
        write_tag_i!(file, value, "usAttachment17", forcewrite);
        let value = self.usAttachment18;
        write_tag_i!(file, value, "usAttachment18", forcewrite);
        let value = self.usAttachment19;
        write_tag_i!(file, value, "usAttachment19", forcewrite);
        let value = self.usAttachment20;
        write_tag_i!(file, value, "usAttachment20", forcewrite);
        let value = self.usResult;
        write_tag_i!(file, value, "usResult", forcewrite);

		write!(file, "\t</ATTACHMENTCOMBOMERGE>\n").unwrap();
	}
}


pub struct ARMOUR
{
    pub uiIndex: u32,
    ubArmourClass: u8,
    ubProtection: u8, 
    ubCoverage: u8,
    ubDegradePercent: u8
}
impl ARMOUR {
    pub fn new() -> ARMOUR
    {
        ARMOUR { 
            uiIndex: 0, ubArmourClass: 0, ubProtection: 0, ubCoverage: 0, ubDegradePercent:0
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						b"ubArmourClass" => { self.ubArmourClass = parseu8(reader, buf, &name); }
						b"ubProtection" => { self.ubProtection = parseu8(reader, buf, &name); }
						b"ubCoverage" => { self.ubCoverage = parseu8(reader, buf, &name); }
						b"ubDegradePercent" => { self.ubDegradePercent = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ARMOUR" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ARMOUR>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = self.ubArmourClass;
        write_tag_i!(file, value, "ubArmourClass", forcewrite);
        let value = self.ubProtection;
        write_tag_i!(file, value, "ubProtection", forcewrite);
        let value = self.ubCoverage;
        write_tag_i!(file, value, "ubCoverage", forcewrite);
        let value = self.ubDegradePercent;
        write_tag_i!(file, value, "ubDegradePercent", forcewrite);

		write!(file, "\t</ARMOUR>\n").unwrap();
	}
}


pub struct WEAPON
{
    pub uiIndex: u32,
    pub szWeaponName: String, // Unused in 1.13 source. Copy item longname upon renaming
    pub ubWeaponClass: u8, // handgun/shotgun/rifle/knife
    pub ubWeaponType: u8, // exact type (for display purposes)
    pub ubCalibre: u8, // type of ammunition needed
    pub ubReadyTime: u8, // APs to ready/unready weapon
    pub ubShotsPer4Turns: f32, // maximum (mechanical) firing rate
    pub ubShotsPerBurst: u8,
    pub ubBurstPenalty: u8, // % penalty per shot after first
    pub ubBulletSpeed: u8, // bullet's travelling speed
    pub ubImpact: u8, // weapon's max damage impact (size & speed)
    pub ubDeadliness: u8, // comparative ratings of guns
    pub bAccuracy: i8, // accuracy or penalty used by OCTH
    pub ubMagSize: u16,
    pub usRange: u16,
    pub usReloadDelay: u16,
    pub ubAttackVolume: u8,
    pub ubHitVolume: u8,
    pub sSound: u16,
    pub sBurstSound: u16,
    pub sSilencedBurstSound: u16,
    pub sReloadSound: u16,
    pub sLocknLoadSound: u16,
    pub bBurstAP: u8, // Snap: Burst AP cost replaces bBaseAutofireCost
    pub bAutofireShotsPerFiveAP: u8, 
    pub swapClips: bool,
    pub silencedSound: u16,
    pub APsToReload: u8,
    pub maxdistformessydeath: u8,
    pub NoSemiAuto: bool,
    pub AutoPenalty: u8,
    pub sAniDelay: i16, // Lesh: for burst animation delay
    pub APsToReloadManually: u8,
    pub ManualReloadSound: u16,
    pub nAccuracy: i8, // accuracy or penalty used by NCTH
    pub EasyUnjam: bool, // Guns where each bullet has its own chamber (like revolvers) are easyer to unjam 
    pub bRecoilX: f32, // Recoil now measured in points of muzzle deviation X and Y.
    pub bRecoilY: f32, // Positive values indicated upwards (Y) and rightwards (X). Negatives are down (-Y) and left (-X).
                    // Note that each value is an array. Each item in the array determines recoil
                    // for a different bullet in the sequence. Not all values have to be filled,
                    // but the last filled value will determine the recoil for longer volleys.
    pub ubRecoilDelay: u8,
    pub ubAimLevels: u8, // Dictates how many aiming levels this gun supports. If 0, the program
                     // chooses automatically based on the type of gun (see AllowedAimingLevels() ).
    pub ubHandling: u8,	// This value replaces ubReadyTime for determining a weapons base handling characteristics.
    pub usOverheatingJamThreshold: f32, // if a gun's temperature is above this treshold, it is increasingly prone to jamming
    pub usOverheatingDamageThreshold: f32, // if a gun is fired while its temperature is above this value, it degrades much faster
    pub usOverheatingSingleShotTemperature: f32, // a single shot raises a gun's temperature by this amount
    pub HeavyGun: bool,	// a gun with this cannot be shouldered in standing position, part of shooting from hip feature
    // if the wielder possesses the 'GUNSLINGER_NT' trait, is using alternate hold scope mode, and has the second hand free, this gun can be used in burst mode
    // this is intended for guns that normally don't possess burst mode, like revolvers
    pub fBurstOnlyByFanTheHammer: bool,
    // Multi-barrel weapons can fire a variety of barrels at once in all firemodes.
    // This vector stores the possible configurations
    pub barrelconfigurations: Vec<u8>,
    // NWSS data
    pub ubNWSSCase: u8,
    pub ubNWSSLast: u8,
    pub szNWSSSound: String,
}
impl WEAPON 
{
    pub fn new() -> WEAPON
    {
        WEAPON { 
            uiIndex: 0, szWeaponName: "".to_string(), ubWeaponClass: 0, ubWeaponType: 0, ubCalibre: 0, ubReadyTime: 0, 
            ubShotsPer4Turns: 0.0, ubShotsPerBurst: 0, ubBurstPenalty: 0, ubBulletSpeed: 0, ubImpact: 0, ubDeadliness: 0,
            bAccuracy: 0, ubMagSize: 0, usRange: 0, usReloadDelay: 0, ubAttackVolume: 0, ubHitVolume: 0, sSound: 0, 
            sBurstSound: 0, sSilencedBurstSound: 0, sReloadSound: 0, sLocknLoadSound: 0, bBurstAP: 0, bAutofireShotsPerFiveAP: 0, 
            swapClips: false, silencedSound: 0, APsToReload: 0, maxdistformessydeath: 0, NoSemiAuto: false, AutoPenalty: 0, 
            sAniDelay: 0, APsToReloadManually: 0, ManualReloadSound: 0, nAccuracy: 0, EasyUnjam: false, 
            bRecoilX: 0.0, bRecoilY: 0.0, ubRecoilDelay: 0, ubAimLevels: 0, ubHandling: 0, 
            usOverheatingJamThreshold: 0.0, usOverheatingDamageThreshold: 0.0, usOverheatingSingleShotTemperature: 0.0, 
            HeavyGun: false, fBurstOnlyByFanTheHammer: false, barrelconfigurations: Vec::new(), 
            ubNWSSCase: 0, ubNWSSLast: 0, szNWSSSound: "".to_string() 
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop 
		{
			match reader.read_event_into(buf) 
			{
				    Ok(Event::Start(e)) => 
				    {
					        let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					        match e.name().as_ref()
					        {
			            		b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						        b"szWeaponName" => { self.szWeaponName = parseString(reader, buf); }
			            		b"ubWeaponClass" => { self.ubWeaponClass = parseu8(reader, buf, &name); }
								b"ubWeaponType" => { self.ubWeaponType = parseu8(reader, buf, &name); }
								b"ubCalibre" => { self.ubCalibre = parseu8(reader, buf, &name); }
								b"ubReadyTime" => { self.ubReadyTime = parseu8(reader, buf, &name); }
								b"ubShotsPer4Turns" => { self.ubShotsPer4Turns = parsef32(reader, buf, &name); }
								b"ubShotsPerBurst" => { self.ubShotsPerBurst = parseu8(reader, buf, &name); }
								b"ubBurstPenalty" => { self.ubBurstPenalty = parseu8(reader, buf, &name); }
								b"ubBulletSpeed" => { self.ubBulletSpeed = parseu8(reader, buf, &name); }
								b"ubImpact" => { self.ubImpact = parseu8(reader, buf, &name); }
								b"ubDeadliness" => { self.ubDeadliness = parseu8(reader, buf, &name); }
								b"bAccuracy" => { self.bAccuracy = parsei8(reader, buf, &name); }
								b"ubMagSize" => { self.ubMagSize = parseu16(reader, buf, &name); }
								b"usRange" => { self.usRange = parseu16(reader, buf, &name); }
								b"usReloadDelay" => { self.usReloadDelay = parseu16(reader, buf, &name); }
								b"BurstAniDelay" => { self.sAniDelay = parsei16(reader, buf, &name); }
								b"ubAttackVolume" => { self.ubAttackVolume = parseu8(reader, buf, &name); }
								b"ubHitVolume" => { self.ubHitVolume = parseu8(reader, buf, &name); }
								b"sSound" => { self.sSound = parseu16(reader, buf, &name); }
								b"sBurstSound" => { self.sBurstSound = parseu16(reader, buf, &name); }
								b"sSilencedBurstSound" => { self.sSilencedBurstSound = parseu16(reader, buf, &name); }
								b"sReloadSound" => { self.sReloadSound = parseu16(reader, buf, &name); }
								b"sLocknLoadSound" => { self.sLocknLoadSound = parseu16(reader, buf, &name); }
								b"SilencedSound" => { self.silencedSound = parseu16(reader, buf, &name); }
								b"bBurstAP" => { self.bBurstAP = parseu8(reader, buf, &name); }
								b"bAutofireShotsPerFiveAP" => { self.bAutofireShotsPerFiveAP = parseu8(reader, buf, &name); }
								b"APsToReload" => {self.APsToReload = parseu8(reader, buf, &name); }
								b"SwapClips" => {self.swapClips = parsebool(reader, buf, &name); }
								b"MaxDistForMessyDeath" => {self.maxdistformessydeath = parseu8(reader, buf, &name); }
								b"AutoPenalty" => {self.AutoPenalty = parseu8(reader, buf, &name); }
								b"NoSemiAuto" => {self.NoSemiAuto = parsebool(reader, buf, &name); }
								b"EasyUnjam" => {self.EasyUnjam = parsebool(reader, buf, &name); }
								b"APsToReloadManually" => {self.APsToReloadManually = parseu8(reader, buf, &name); }
								b"ManualReloadSound" => {self.ManualReloadSound = parseu16(reader, buf, &name); }
								b"nAccuracy" => {self.nAccuracy = parsei8(reader, buf, &name); }
								b"bRecoilX" => {self.bRecoilX = parsef32(reader, buf, &name); }
								b"bRecoilY" => {self.bRecoilY = parsef32(reader, buf, &name); }
								b"ubAimLevels" => {self.ubAimLevels = parseu8(reader, buf, &name); }
								b"ubRecoilDelay" => {self.ubRecoilDelay = parseu8(reader, buf, &name); }
								b"Handling" => { self.ubHandling = parseu8(reader, buf, &name); }
								b"usOverheatingJamThreshold" => {self.usOverheatingJamThreshold = parsef32(reader, buf, &name);}
								b"usOverheatingDamageThreshold" => {self.usOverheatingDamageThreshold = parsef32(reader, buf, &name);}
								b"usOverheatingSingleShotTemperature" => {self.usOverheatingSingleShotTemperature = parsef32(reader, buf, &name);}
								b"HeavyGun" => {self.HeavyGun = parsebool(reader, buf, &name);}
								b"fBurstOnlyByFanTheHammer" => {self.fBurstOnlyByFanTheHammer = parsebool(reader, buf, &name);}
                        		b"BarrelConfiguration" => 
								{
			                		let value = parseu8(reader, buf, &name);
									self.barrelconfigurations.push(value);
			    				}
								b"ubNWSSCase" => { self.ubNWSSCase = parseu8(reader, buf, &name); }
								b"ubNWSSLast" => { self.ubNWSSLast = parseu8(reader, buf, &name); }
								b"szNWSSSound" => { self.szNWSSSound = parseString(reader, buf); }
								_ => {}
						        }
				    }

				    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				    Ok(Event::End(ref element)) => 
				    {
					        match element.name().as_ref()
					        {
						            b"WEAPON" => break,
						            _ => ()
					        }
				    }
				    _ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<WEAPON>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.szWeaponName;
        write_tag_s!(file, value, "szWeaponName", forcewrite);
        let value = self.ubWeaponClass;
        write_tag_i!(file, value, "ubWeaponClass", forcewrite);
        let value = self.ubWeaponType;
        write_tag_i!(file, value, "ubWeaponType", forcewrite);
        let value = self.ubCalibre;
        write_tag_i!(file, value, "ubCalibre", forcewrite);
        let value = self.ubReadyTime;
        write_tag_i!(file, value, "ubReadyTime", forcewrite);
        let value = self.ubShotsPer4Turns;
        write_tag_f!(file, value, "ubShotsPer4Turns", forcewrite);
        let value = self.ubShotsPerBurst;
        write_tag_i!(file, value, "ubShotsPerBurst", forcewrite);
        let value = self.ubBurstPenalty;
        write_tag_i!(file, value, "ubBurstPenalty", forcewrite);
        let value = self.ubBulletSpeed;
        write_tag_i!(file, value, "ubBulletSpeed", forcewrite);
        let value = self.ubImpact;
        write_tag_i!(file, value, "ubImpact", forcewrite);
        let value = self.ubDeadliness;
        write_tag_i!(file, value, "ubDeadliness", forcewrite);
        let value = self.bAccuracy;
        write_tag_i!(file, value, "bAccuracy", forcewrite);
        let value = self.ubMagSize;
        write_tag_i!(file, value, "ubMagSize", forcewrite);
        let value = self.usRange;
        write_tag_i!(file, value, "usRange", forcewrite);
        let value = self.usReloadDelay;
        write_tag_i!(file, value, "usReloadDelay", forcewrite);
        let value = self.sAniDelay;
        write_tag_i!(file, value, "BurstAniDelay", forcewrite);
        let value = self.ubAttackVolume;
        write_tag_i!(file, value, "ubAttackVolume", forcewrite);
        let value = self.ubHitVolume;
        write_tag_i!(file, value, "ubHitVolume", forcewrite);
        let value = self.sSound;
        write_tag_i!(file, value, "sSound", forcewrite);
        let value = self.sBurstSound;
        write_tag_i!(file, value, "sBurstSound", forcewrite);
        let value = self.sSilencedBurstSound;
        write_tag_i!(file, value, "sSilencedBurstSound", forcewrite);
        let value = self.sReloadSound;
        write_tag_i!(file, value, "sReloadSound", forcewrite);
        let value = self.sLocknLoadSound;
        write_tag_i!(file, value, "sLocknLoadSound", forcewrite);
        let value = self.silencedSound;
        write_tag_i!(file, value, "SilencedSound", forcewrite);
        let value = self.bBurstAP;
        write_tag_i!(file, value, "bBurstAP", forcewrite);
        let value = self.bAutofireShotsPerFiveAP;
        write_tag_i!(file, value, "bAutofireShotsPerFiveAP", forcewrite);
        let value = self.APsToReload;
        write_tag_i!(file, value, "APsToReload", forcewrite);
        let value = self.swapClips as u32;
        write_tag_i!(file, value, "SwapClips", forcewrite);
        let value = self.maxdistformessydeath;
        write_tag_i!(file, value, "MaxDistForMessyDeath", forcewrite);
        let value = self.AutoPenalty;
        write_tag_i!(file, value, "AutoPenalty", forcewrite);
        let value = self.NoSemiAuto as u32;
        write_tag_i!(file, value, "NoSemiAuto", forcewrite);
        let value = self.EasyUnjam as u32;
        write_tag_i!(file, value, "EasyUnjam", forcewrite);
        let value = self.APsToReloadManually;
        write_tag_i!(file, value, "APsToReloadManually", forcewrite);
        let value = self.ManualReloadSound;
        write_tag_i!(file, value, "ManualReloadSound", forcewrite);
        let value = self.nAccuracy;
        write_tag_i!(file, value, "nAccuracy", forcewrite);
        let value = self.bRecoilX;
        write_tag_f!(file, value, "bRecoilX", forcewrite);
        let value = self.bRecoilY;
        write_tag_f!(file, value, "bRecoilY", forcewrite);
        let value = self.ubAimLevels;
        write_tag_i!(file, value, "ubAimLevels", forcewrite);
        let value = self.ubRecoilDelay;
        write_tag_i!(file, value, "ubRecoilDelay", forcewrite);
        let value = self.ubHandling;
        write_tag_i!(file, value, "Handling", forcewrite);
        let value = self.usOverheatingJamThreshold;
        write_tag_f!(file, value, "usOverheatingJamThreshold", forcewrite);
        let value = self.usOverheatingDamageThreshold;
        write_tag_f!(file, value, "usOverheatingDamageThreshold", forcewrite);
        let value = self.usOverheatingSingleShotTemperature;
        write_tag_f!(file, value, "usOverheatingSingleShotTemperature", forcewrite);
        let value = self.HeavyGun as u32;
        write_tag_i!(file, value, "HeavyGun", forcewrite);
        let value = self.fBurstOnlyByFanTheHammer as u32;
        write_tag_i!(file, value, "fBurstOnlyByFanTheHammer", forcewrite);

		for p in &self.barrelconfigurations
		{
			let p = p.clone();
			write_tag_i!(file, p, "BarrelConfiguration", forcewrite);
		}

        let value = self.ubNWSSCase;
		write_tag_i!(file, value, "ubNWSSCase", forcewrite);
		let value = self.ubNWSSLast;
		write_tag_i!(file, value, "ubNWSSLast", forcewrite);
        let value = &self.szNWSSSound;
        write_tag_s!(file, value, "szNWSSSound", forcewrite);

		write!(file, "\t</WEAPON>\n").unwrap();
	}

}

pub struct MAGAZINE
{
    pub uiIndex: u32,
    pub ubCalibre: u8,
    pub ubMagSize: u16, 
    pub ubAmmoType: u8,
    pub ubMagType: u8
}
impl MAGAZINE {
    pub fn new() -> MAGAZINE
    {
        MAGAZINE { 
            uiIndex: 0, ubCalibre: 0, ubMagSize: 0, ubAmmoType: 0, ubMagType:0
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						b"ubCalibre" => { self.ubCalibre = parseu8(reader, buf, &name); }
						b"ubMagSize" => { self.ubMagSize = parseu16(reader, buf, &name); }
						b"ubAmmoType" => { self.ubAmmoType = parseu8(reader, buf, &name); }
						b"ubMagType" => { self.ubMagType = parseu8(reader, buf, &name); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"MAGAZINE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<MAGAZINE>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = self.ubCalibre;
        write_tag_i!(file, value, "ubCalibre", forcewrite);
        let value = self.ubMagSize;
        write_tag_i!(file, value, "ubMagSize", forcewrite);
        let value = self.ubAmmoType;
        write_tag_i!(file, value, "ubAmmoType", forcewrite);
        let value = self.ubMagType;
        write_tag_i!(file, value, "ubMagType", forcewrite);

		write!(file, "\t</MAGAZINE>\n").unwrap();
	}
}

pub struct AMMOSTRING
{
    pub uiIndex: u32,
    pub AmmoCaliber: String,
    pub BRCaliber: String,
    pub NWSSCaliber: String,
}
impl AMMOSTRING {
    pub fn new() -> AMMOSTRING
    {
        AMMOSTRING { 
            uiIndex: 0, AmmoCaliber: "".to_string(), BRCaliber: "".to_string(), NWSSCaliber: "".to_string()
        }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						b"AmmoCaliber" => { self.AmmoCaliber = parseString(reader, buf); }
						b"BRCaliber" => { self.BRCaliber = parseString(reader, buf); }
						b"NWSSCaliber" => { self.NWSSCaliber = parseString(reader, buf); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"AMMO" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<AMMO>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.AmmoCaliber;
        write_tag_s!(file, value, "AmmoCaliber", forcewrite);
        let value = &self.BRCaliber;
        write_tag_s!(file, value, "BRCaliber", forcewrite);
        let value = &self.NWSSCaliber;
        write_tag_s!(file, value, "NWSSCaliber", forcewrite);

		write!(file, "\t</AMMO>\n").unwrap();
	}
}

pub struct AMMOTYPE
{
    pub uiIndex: u32,
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub structureImpactReductionMultiplier: u8,
    pub structureImpactReductionDivisor: u8,
    pub armourImpactReductionMultiplier: u8,
    pub armourImpactReductionDivisor: u8,
    pub beforeArmourDamageMultiplier: u8,
    pub beforeArmourDamageDivisor: u8,
    pub afterArmourDamageMultiplier: u8,
    pub afterArmourDamageDivisor: u8,
    pub zeroMinimumDamage: bool,
    pub usPiercePersonChanceModifier: u16,
    pub standardIssue: bool,
    pub numberOfBullets: u16,
    pub multipleBulletDamageMultiplier: u8,
    pub multipleBulletDamageDivisor: u8,
    pub highExplosive: u32,
    pub explosionSize: u8,
    pub dart: bool,
    pub knife: bool,
    pub monsterSpit: bool,
    pub acidic: bool, 
    pub ignoreArmour: bool,
    pub lockBustingPower: u16,
    pub tracerEffect: bool,
    pub spreadPattern: String,
    pub temperatureModificator: f32,
    pub dirtModificator: f32,
    pub ammoflag: u32,
    pub dDamageModifierLife: f32,
    pub dDamageModifierBreath: f32,
    pub dDamageModifierTank: f32,
    pub dDamageModifierArmouredVehicle: f32,
    pub dDamageModifierCivilianVehicle: f32,
    pub dDamageModifierZombie: f32,
    pub shotAnimation: String,
}
impl AMMOTYPE {
    pub fn new() -> AMMOTYPE
    {
        AMMOTYPE { uiIndex: 0, name: "".to_string(), red: 0, green: 0, blue: 0, structureImpactReductionMultiplier: 0, structureImpactReductionDivisor: 0,
            armourImpactReductionMultiplier: 0, armourImpactReductionDivisor: 0, beforeArmourDamageMultiplier: 0, beforeArmourDamageDivisor: 0,
            afterArmourDamageMultiplier: 0, afterArmourDamageDivisor: 0, zeroMinimumDamage: false, usPiercePersonChanceModifier: 0, standardIssue: false,
            numberOfBullets: 0, multipleBulletDamageMultiplier: 0, multipleBulletDamageDivisor: 0, highExplosive: 0, explosionSize: 0,
            dart: false, knife: false, monsterSpit: false, acidic: false, ignoreArmour: false, lockBustingPower: 0, tracerEffect: false, spreadPattern: "".to_string(),
            temperatureModificator: 0.0, dirtModificator: 0.0, ammoflag: 0, dDamageModifierLife: 0.0, dDamageModifierBreath: 0.0, dDamageModifierTank: 0.0,
            dDamageModifierArmouredVehicle: 0.0, dDamageModifierCivilianVehicle: 0.0, dDamageModifierZombie: 0.0, shotAnimation: "".to_string() }
    }

    pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						b"name" => { self.name = parseString(reader, buf); }
						b"red" => { self.red = parseu8(reader, buf, &name); }
						b"green" => { self.green = parseu8(reader, buf, &name); }
						b"blue" => { self.blue = parseu8(reader, buf, &name); }
						b"structureImpactReductionMultiplier" => { self.structureImpactReductionMultiplier = parseu8(reader, buf, &name); }
						b"structureImpactReductionDivisor" => { self.structureImpactReductionDivisor = parseu8(reader, buf, &name); }
						b"armourImpactReductionMultiplier" => { self.armourImpactReductionMultiplier = parseu8(reader, buf, &name); }
						b"armourImpactReductionDivisor" => { self.armourImpactReductionDivisor = parseu8(reader, buf, &name); }
						b"beforeArmourDamageMultiplier" => { self.beforeArmourDamageMultiplier = parseu8(reader, buf, &name); }
						b"beforeArmourDamageDivisor" => { self.beforeArmourDamageDivisor = parseu8(reader, buf, &name); }
						b"afterArmourDamageMultiplier" => { self.afterArmourDamageMultiplier = parseu8(reader, buf, &name); }
						b"afterArmourDamageDivisor" => { self.afterArmourDamageDivisor = parseu8(reader, buf, &name); }
						b"zeroMinimumDamage" => { self.zeroMinimumDamage = parsebool(reader, buf, &name); }
						b"usPiercePersonChanceModifier" => { self.usPiercePersonChanceModifier = parseu16(reader, buf, &name); }
						b"standardIssue" => { self.standardIssue = parsebool(reader, buf, &name); }
						b"numberOfBullets" => { self.numberOfBullets = parseu16(reader, buf, &name); }
						b"multipleBulletDamageMultiplier" => { self.multipleBulletDamageMultiplier = parseu8(reader, buf, &name); }
						b"multipleBulletDamageDivisor" => { self.multipleBulletDamageDivisor = parseu8(reader, buf, &name); }
						b"highExplosive" => { self.highExplosive = parseu32(reader, buf, &name); }
						b"explosionSize" => { self.explosionSize = parseu8(reader, buf, &name); }
						b"dart" => { self.dart = parsebool(reader, buf, &name); }
						b"knife" => { self.knife = parsebool(reader, buf, &name); }
						b"monsterSpit" => { self.monsterSpit = parsebool(reader, buf, &name); }
						b"acidic" => { self.acidic = parsebool(reader, buf, &name); }
						b"ignoreArmour" => { self.ignoreArmour = parsebool(reader, buf, &name); }
						b"lockBustingPower" => { self.lockBustingPower = parseu16(reader, buf, &name); }
						b"tracerEffect" => {self.tracerEffect = parsebool(reader, buf, &name); }
						b"spreadPattern" => {self.spreadPattern = parseString(reader, buf); }
						b"temperatureModificator" => {self.temperatureModificator = parsef32(reader, buf, &name); }
						b"dirtModificator" => {self.dirtModificator = parsef32(reader, buf, &name); }
						b"ammoflag" => {self.ammoflag = parseu32(reader, buf, &name); }
						b"dDamageModifierLife" => {self.dDamageModifierLife = parsef32(reader, buf, &name); }
						b"dDamageModifierBreath" => {self.dDamageModifierBreath = parsef32(reader, buf, &name); }
						b"dDamageModifierTank" => {self.dDamageModifierTank = parsef32(reader, buf, &name); }
						b"dDamageModifierArmouredVehicle" => {self.dDamageModifierArmouredVehicle = parsef32(reader, buf, &name); }
						b"dDamageModifierCivilianVehicle" => {self.dDamageModifierCivilianVehicle = parsef32(reader, buf, &name); }
						b"dDamageModifierZombie" => {self.dDamageModifierZombie = parsef32(reader, buf, &name); }
						b"shotAnimation" => {self.shotAnimation = parseString(reader, buf); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"AMMOTYPE" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

    pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<AMMOTYPE>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
        let value = &self.name;
        write_tag_s!(file, value, "name", forcewrite);
        let value = self.red;
        write_tag_i!(file, value, "red", forcewrite);
        let value = self.green;
        write_tag_i!(file, value, "green", forcewrite);
        let value = self.blue;
        write_tag_i!(file, value, "blue", forcewrite);

        let value = self.structureImpactReductionMultiplier;
        write_tag_i!(file, value, "structureImpactReductionMultiplier", forcewrite);
        let value = self.structureImpactReductionDivisor;
        write_tag_i!(file, value, "structureImpactReductionDivisor", forcewrite);
        let value = self.armourImpactReductionMultiplier;
        write_tag_i!(file, value, "armourImpactReductionMultiplier", forcewrite);
        let value = self.armourImpactReductionDivisor;
        write_tag_i!(file, value, "armourImpactReductionDivisor", forcewrite);
        let value = self.beforeArmourDamageMultiplier;
        write_tag_i!(file, value, "beforeArmourDamageMultiplier", forcewrite);
        let value = self.beforeArmourDamageDivisor;
        write_tag_i!(file, value, "beforeArmourDamageDivisor", forcewrite);
        let value = self.afterArmourDamageMultiplier;
        write_tag_i!(file, value, "afterArmourDamageMultiplier", forcewrite);
        let value = self.afterArmourDamageDivisor;
        write_tag_i!(file, value, "afterArmourDamageDivisor", forcewrite);
        let value = self.zeroMinimumDamage as u32;
        write_tag_i!(file, value, "zeroMinimumDamage", forcewrite);
        let value = self.usPiercePersonChanceModifier;
        write_tag_i!(file, value, "usPiercePersonChanceModifier", forcewrite);
        let value = self.standardIssue as u32;
        write_tag_i!(file, value, "standardIssue", forcewrite);
        let value = self.numberOfBullets;
        write_tag_i!(file, value, "numberOfBullets", forcewrite);
        let value = self.multipleBulletDamageMultiplier;
        write_tag_i!(file, value, "multipleBulletDamageMultiplier", forcewrite);
        let value = self.multipleBulletDamageDivisor;
        write_tag_i!(file, value, "multipleBulletDamageDivisor", forcewrite);
        let value = self.highExplosive;
        write_tag_i!(file, value, "highExplosive", forcewrite);
        let value = self.explosionSize;
        write_tag_i!(file, value, "explosionSize", forcewrite);
        let value = self.dart as u32;
        write_tag_i!(file, value, "dart", forcewrite);
        let value = self.knife as u32;
        write_tag_i!(file, value, "knife", forcewrite);
        let value = self.monsterSpit as u32;
        write_tag_i!(file, value, "monsterSpit", forcewrite);
        let value = self.acidic as u32;
        write_tag_i!(file, value, "acidic", forcewrite);
        let value = self.ignoreArmour as u32;
        write_tag_i!(file, value, "ignoreArmour", forcewrite);
        let value = self.lockBustingPower;
        write_tag_i!(file, value, "lockBustingPower", forcewrite);
        let value = self.tracerEffect as u32;
        write_tag_i!(file, value, "tracerEffect", forcewrite);
        let value = &self.spreadPattern;
        write_tag_s!(file, value, "spreadPattern", forcewrite);
        let value = self.temperatureModificator;
        write_tag_f!(file, value, "temperatureModificator", forcewrite);
        let value = self.dirtModificator;
        write_tag_f!(file, value, "dirtModificator", forcewrite);
        let value = self.ammoflag;
        write_tag_i!(file, value, "ammoflag", forcewrite);
        let value = self.dDamageModifierLife;
        write_tag_f!(file, value, "dDamageModifierLife", forcewrite);
        let value = self.dDamageModifierBreath;
        write_tag_f!(file, value, "dDamageModifierBreath", forcewrite);
        let value = self.dDamageModifierTank;
        write_tag_f!(file, value, "dDamageModifierTank", forcewrite);
        let value = self.dDamageModifierArmouredVehicle;
        write_tag_f!(file, value, "dDamageModifierArmouredVehicle", forcewrite);
        let value = self.dDamageModifierCivilianVehicle;
        write_tag_f!(file, value, "dDamageModifierCivilianVehicle", forcewrite);
        let value = self.dDamageModifierZombie;
        write_tag_f!(file, value, "dDamageModifierZombie", forcewrite);
        let value = &self.shotAnimation;
        write_tag_s!(file, value, "shotAnimation", forcewrite);

		write!(file, "\t</AMMOTYPE>\n").unwrap();
	}
}


pub struct ITEM {
	pub uiIndex: u32,
	pub szItemName: String,
	pub szLongItemName: String,
	pub szItemDesc: String,
	pub szBRName: String,
	pub szBRDesc: String,
	pub usItemClass: u32,
	pub AttachmentClass: u32,
    pub nasAttachmentClass: u64,
	pub nasLayoutClass: u64,
	// ulAvailableAttachmentPoints: Vec<u64>, 
	pub AvailableAttachmentPoint: AttachmentPoints, 
	pub ulAttachmentPoint: u64, 
	pub ubAttachToPointAPCost: u8,
	pub ubClassIndex: u16,
	pub usItemFlag: u64,
	pub ubCursor: u8,
	pub bSoundType: i8,
	pub ubGraphicType: u8,
	pub ubGraphicNum: u16,
	pub ubWeight: u16,
	pub ubPerPocket: u8,
	pub ItemSize: u16,
	pub ItemSizeBonus: i16,
	pub usPrice: u16,
	pub ubCoolness: u8,
	pub bReliability: i8,
	pub bRepairEase: i8,
	pub Damageable: bool,
	pub Repairable: bool,
	pub WaterDamages: bool,
	pub Metal: bool,
	pub Sinks: bool,
	pub showstatus: bool,
	pub hiddenaddon: bool,
	pub twohanded: bool,
	pub notbuyable: bool,
	pub attachment: bool,
	pub hiddenattachment: bool,
	pub blockironsight: bool,
	pub biggunlist: bool,
	pub scifi: bool,
	pub notineditor: bool,
	pub defaultundroppable: bool,
	pub unaerodynamic: bool,
	pub electronic: bool,
	pub inseparable: u8, //Madd:Normally, an inseparable attachment can never be removed.  
						//But now we will make it so that these items can be replaced, but still not removed directly.
						//0 = removeable (as before)
						//1 = inseparable (as before)
						//2 = inseparable, but replaceable
	pub BR_NewInventory: u8,
	pub BR_UsedInventory: u8,
	pub BR_ROF: i16,
	pub percentnoisereduction: i16,
	pub hidemuzzleflash: bool,
	pub bipod: i16,
	pub rangebonus: i16,
	pub percentrangebonus: i16,
	pub tohitbonus: i16,
	pub bestlaserrange: i16,
	pub aimbonus: i16,
	pub minrangeforaimbonus: i16,
	pub magsizebonus: i16,
	pub rateoffirebonus: i16,
	pub bulletspeedbonus: i16,
	pub burstsizebonus: i16,
	pub bursttohitbonus: i16,
	pub autofiretohitbonus: i16,
	pub APBonus: i16,
	pub percentburstfireapreduction: i16,
	pub percentautofireapreduction: i16,
	pub percentreadytimeapreduction: i16,
	pub percentreloadtimeapreduction: i16,
	pub percentapreduction: i16,
	pub percentstatusdrainreduction: i16,
	pub damagebonus: i16,
	pub meleedamagebonus: i16,
	pub grenadelauncher: bool,
	pub duckbill: bool,
	pub glgrenade: bool,
	pub mine: bool,
	pub mortar: bool,
	pub rocketlauncher: bool,
	pub singleshotrocketlauncher: bool,
	pub discardedlauncheritem: u16,
	pub rocketrifle: bool,
	pub cannon: bool,
	pub defaultattachments: Vec<u16>,
	pub brassknuckles: bool,
	pub crowbar: bool,
	pub bloodieditem: i16,
	pub rock: bool,
	pub camobonus: i16,
	pub urbanCamobonus: i16,
	pub desertCamobonus: i16,
	pub snowCamobonus: i16,
	pub stealthbonus: i16,
	pub flakjacket: bool,
	pub leatherjacket: bool,
	pub directional: bool, // item is a directional mine/bomb (actual direction is set upon planting)
	pub remotetrigger: bool,
	pub lockbomb: bool,
	pub flare: bool,
	pub robotremotecontrol: bool,
	pub walkman: bool,
	pub hearingrangebonus: i16,
	pub visionrangebonus: i16,
	pub nightvisionrangebonus: i16,
	pub dayvisionrangebonus: i16,
	pub cavevisionrangebonus: i16,
	pub brightlightvisionrangebonus: i16,
	pub percenttunnelvision: u8,
	pub usFlashLightRange: u8, //  range of a flashlight (an item with usFlashLightRange > 0 is deemed a flashlight)
	pub thermaloptics: bool,
	pub gasmask: bool,
	pub alcohol: f32,
	pub hardware: bool,
	pub medical: bool,
	pub drugtype: u32, // this flagmask determines what different components are used in a drug, which results in different effects
	pub camouflagekit: bool,
	pub locksmithkit: bool,
	pub toolkit: bool,
	pub firstaidkit: bool,
	pub medicalkit: bool,
	pub wirecutters: bool,
	pub canteen: bool,
	pub gascan: bool,
	pub marbles: bool,
	pub canandstring: bool,
	pub jar: bool,
	pub xray: bool,
	pub batteries: bool,
	pub needsbatteries: bool,
	pub containsliquid: bool,
	pub metaldetector: bool,
	pub usSpotting: i16, //  spotting effectiveness
	pub fingerprintid: bool,
	pub tripwireactivation: bool, // item (mine) can be activated by nearby tripwire
	pub tripwire: bool, // item is tripwire
	pub newinv: bool, // item only available in new inventory mode
	pub ubAttachmentSystem: u8, //Item availability per attachment system: 0 = both, 1 = OAS, 2 = NAS
	pub scopemagfactor: f32,
	pub projectionfactor: f32,
	pub RecoilModifierX: f32,
	pub RecoilModifierY: f32,
	pub PercentRecoilModifier: i16,
	pub percentaccuracymodifier: i16,
	// spreadPattern: i32, //zilpin: pellet spread patterns externalized in XML
	pub barrel: bool, // item can be used on some guns as an exchange barrel
	pub usOverheatingCooldownFactor: f32, // every turn/5 seconds, a gun's temperature is lowered by this amount
	pub overheatTemperatureModificator: f32, // percentage modifier of heat a shot generates (read from attachments)
	pub overheatCooldownModificator: f32, // percentage modifier of cooldown amount (read from attachments, applies to guns & barrels)
	pub overheatJamThresholdModificator: f32, // percentage modifier of a gun's jam threshold (read from attachments)
	pub overheatDamageThresholdModificator: f32, // percentage modifier of a gun's damage threshold (read from attachments)
	pub foodtype: u32,
	pub LockPickModifier: i8,
	pub CrowbarModifier: u8,
	pub DisarmModifier: u8,
	pub RepairModifier: i8,
	pub usHackingModifier: u8,
	pub usBurialModifier: u8,
	pub usDamageChance: u8, // chance that damage to the status will also damage the repair threshold
	pub dirtIncreaseFactor: f32, // one shot causes this much dirt on a gun
	pub clothestype: u32, // clothes type that 'links' to an entry in Clothes.xml
	pub usActionItemFlag: u32, // a flag that is necessary for transforming action items to objects with new abilities (for now, tripwire networks and directional explosives)
	pub randomitem: u16, //  a link to RandomItemsClass.xml. Out of such an item, a random object is created, depending on the entries in the xml
	pub randomitemcoolnessmodificator: i8, // alters the allowed maximum coolness a random item can have
	pub usItemChoiceTimeSetting: u8, // determine whether the AI should pick this item for its choices only at certain times
	pub usBuddyItem: u16, //  item is connected to another item. Type of connection depends on item specifics
	pub ubSleepModifier: u8, //  item provides breath regeneration bonus while resting
	pub sBackpackWeightModifier: i16, //modifier to weight calculation to climb.
	pub fAllowClimbing: bool, //does item allow climbing while wearing it
	pub antitankmine: bool,
	pub cigarette: bool, //  this item can be smoked
	pub usPortionSize: u8, //  for consumables: how much of this item is consumed at once
	pub usRiotShieldStrength: u16, // strength of shield
	pub usRiotShieldGraphic: u16, // graphic of shield (when deployed in tactical, taken from Tilecache/riotshield.sti)
	pub sFireResistance: i16,
	pub fRobotDamageReductionModifier: f32,
	pub bRobotStrBonus: i8,
	pub bRobotAgiBonus: i8,
	pub bRobotDexBonus: i8,
	pub fProvidesRobotCamo: bool,
	pub fProvidesRobotNightVision: bool,
	pub fProvidesRobotLaserBonus: bool,
	pub bRobotChassisSkillGrant: i8,
	pub bRobotTargetingSkillGrant: i8,
	pub bRobotUtilitySkillGrant: i8,
	// STAND/CROUCH/PRONE_MODIFIERS
	pub flatbasemodifier: [i16; 3],
	pub percentbasemodifier: [i16; 3],
	pub flataimmodifier: [i16; 3],
	pub counterforcefrequency: [i16; 3],
	pub percentcapmodifier: [i16; 3],
	pub percenthandlingmodifier: [i16; 3],
	pub percentdropcompensationmodifier: [i16; 3],
	pub maxcounterforcemodifier: [i16; 3],
	pub counterforceaccuracymodifier: [i16; 3],
	pub targettrackingmodifier: [i16; 3],
	pub aimlevelsmodifier: [i16; 3],
}

impl ITEM
{
	
	pub fn new() -> ITEM
	{
		return ITEM{
			uiIndex: 0,
			szItemName: "".to_string(),
			szLongItemName: "".to_string(),
			szItemDesc: "".to_string(),
			szBRName: "".to_string(),
			szBRDesc: "".to_string(),
			usItemClass: 0,
			AttachmentClass: 0,
			nasAttachmentClass: 0,
			nasLayoutClass: 0,
			AvailableAttachmentPoint: AttachmentPoints { points: Vec::new() }, 
			ulAttachmentPoint: 0,
			ubAttachToPointAPCost: 0,
			ubClassIndex: 0,
			usItemFlag: 0,
			ubCursor: 0,
			bSoundType: 0,
			ubGraphicType: 0,
			ubGraphicNum: 0,
			ubWeight: 0,
			ubPerPocket: 0,
			ItemSize: 0,
			ItemSizeBonus: 0,
			usPrice: 0,
			ubCoolness: 0,
			bReliability: 0,
			bRepairEase: 0,
			Damageable: false,
			Repairable: false,
			WaterDamages: false,
			Metal: false,
			Sinks: false,
			showstatus: false,
			hiddenaddon: false,
			twohanded: false,
			notbuyable: false,
			attachment: false,
			hiddenattachment: false,
			blockironsight: false,
			biggunlist: false,
			scifi: false,
			notineditor: false,
			defaultundroppable: false,
			unaerodynamic: false,
			electronic: false,
			inseparable: 0,
			BR_NewInventory: 0,
			BR_UsedInventory: 0,
			BR_ROF: 0,
			percentnoisereduction: 0,
			hidemuzzleflash: false,
			bipod: 0,
			rangebonus: 0,
			percentrangebonus: 0,
			tohitbonus: 0,
			bestlaserrange: 0,
			aimbonus: 0,
			minrangeforaimbonus: 0,
			magsizebonus: 0,
			rateoffirebonus: 0,
			bulletspeedbonus: 0,
			burstsizebonus: 0,
			bursttohitbonus: 0,
			autofiretohitbonus: 0,
			APBonus: 0,
			percentburstfireapreduction: 0,
			percentautofireapreduction: 0,
			percentreadytimeapreduction: 0,
			percentreloadtimeapreduction: 0,
			percentapreduction: 0,
			percentstatusdrainreduction: 0,
			damagebonus: 0,
			meleedamagebonus: 0,
			grenadelauncher: false,
			duckbill: false,
			glgrenade: false,
			mine: false,
			mortar: false,
			rocketlauncher: false,
			singleshotrocketlauncher: false,
			discardedlauncheritem: 0,
			rocketrifle: false,
			cannon: false,
			defaultattachments: Vec::new(),
			brassknuckles: false,
			crowbar: false,
			bloodieditem: 0,
			rock: false,
			camobonus: 0,
			urbanCamobonus: 0,
			desertCamobonus: 0,
			snowCamobonus: 0,
			stealthbonus: 0,
			flakjacket: false,
			leatherjacket: false,
			directional: false,
			remotetrigger: false,
			lockbomb: false,
			flare: false,
			robotremotecontrol: false,
			walkman: false,
			hearingrangebonus: 0,
			visionrangebonus: 0,
			nightvisionrangebonus: 0,
			dayvisionrangebonus: 0,
			cavevisionrangebonus: 0,
			brightlightvisionrangebonus: 0,
			percenttunnelvision: 0,
			usFlashLightRange: 0,
			thermaloptics: false,
			gasmask: false,
			alcohol: 0.0,
			hardware: false,
			medical: false,
			drugtype: 0,
			camouflagekit: false,
			locksmithkit: false,
			toolkit: false,
			firstaidkit: false,
			medicalkit: false,
			wirecutters: false,
			canteen: false,
			gascan: false,
			marbles: false,
			canandstring: false,
			jar: false,
			xray: false,
			batteries: false,
			needsbatteries: false,
			containsliquid: false,
			metaldetector: false,
			usSpotting: 0,
			fingerprintid: false,
			tripwireactivation: false,
			tripwire: false,
			newinv: false,
			ubAttachmentSystem: 0,
			scopemagfactor: 0.0,
			projectionfactor: 0.0,
			RecoilModifierX: 0.0,
			RecoilModifierY: 0.0,
			PercentRecoilModifier: 0,
			percentaccuracymodifier: 0,
			barrel: false, 
			usOverheatingCooldownFactor: 0.0,
			overheatTemperatureModificator: 0.0,
			overheatCooldownModificator: 0.0,
			overheatJamThresholdModificator: 0.0,
			overheatDamageThresholdModificator: 0.0,
			foodtype: 0,
			LockPickModifier: 0,
			CrowbarModifier: 0,
			DisarmModifier: 0,
			RepairModifier: 0,
			usHackingModifier: 0,
			usBurialModifier: 0,
			usDamageChance: 0,
			dirtIncreaseFactor: 0.0,
			clothestype: 0,
			usActionItemFlag: 0,
			randomitem: 0,
			randomitemcoolnessmodificator: 0,
			usItemChoiceTimeSetting: 0,
			usBuddyItem: 0,
			ubSleepModifier: 0,
			sBackpackWeightModifier: 0,
			fAllowClimbing: false,
			antitankmine: false,
			cigarette: false,
			usPortionSize: 0,
			usRiotShieldStrength: 0,
			usRiotShieldGraphic: 0,
			sFireResistance: 0,
			fRobotDamageReductionModifier: 0.0,
			bRobotStrBonus: 0,
			bRobotAgiBonus: 0,
			bRobotDexBonus: 0,
			fProvidesRobotCamo: false,
			fProvidesRobotNightVision: false,
			fProvidesRobotLaserBonus: false,
			bRobotChassisSkillGrant: 0,
			bRobotTargetingSkillGrant: 0,
			bRobotUtilitySkillGrant: 0,
			// STAND/CROUCH/PRONE_MODIFIERS
			flatbasemodifier: [0; 3],
			percentbasemodifier: [0; 3],
			flataimmodifier: [0; 3],
			counterforcefrequency: [0; 3],
			percentcapmodifier: [0; 3],
			percenthandlingmodifier: [0; 3],
			percentdropcompensationmodifier: [0; 3],
			maxcounterforcemodifier: [0; 3],
			counterforceaccuracymodifier: [0; 3],
			targettrackingmodifier: [0; 3],
			aimlevelsmodifier: [0; 3],
		};
	}

	pub fn readItem(&mut self, reader: &mut Reader<BufReader<std::fs::File>>, buf: &mut Vec<u8>)
	{
		loop {
			match reader.read_event_into(buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"uiIndex" => { self.uiIndex = parseu32(reader, buf, &name); }
						b"szItemName" => { self.szItemName = parseString(reader, buf); }
						b"szLongItemName" => { self.szLongItemName = parseString(reader, buf); }
						b"szItemDesc" => { self.szItemDesc = parseString(reader, buf); }
						b"szBRName" => { self.szBRName = parseString(reader, buf); }
						b"szBRDesc" => { self.szBRDesc = parseString(reader, buf); }
						b"usItemClass" => { self.usItemClass = parseu32(reader, buf, &name); }
						b"AttachmentClass" => { self.AttachmentClass = parseu32(reader, buf, &name); }
						b"nasAttachmentClass" => { self.nasAttachmentClass = parseu64(reader, buf, &name); }
						b"nasLayoutClass" => { self.nasLayoutClass = parseu64(reader, buf, &name); }
						b"ulAttachmentPoint" => { self.ulAttachmentPoint = parseu64(reader, buf, &name); }

						b"AvailableAttachmentPoint" => {
							let value = parseu64(reader, buf, &name);
							self.AvailableAttachmentPoint.points.push(value);
						}

						b"ubAttachToPointAPCost" => { self.ubAttachToPointAPCost = parseu8(reader, buf, &name); }
						b"ubClassIndex" => { self.ubClassIndex = parseu16(reader, buf, &name); }
						b"usItemFlag" => { self.usItemFlag = parseu64(reader, buf, &name); }
						b"ubCursor" => { self.ubCursor = parseu8(reader, buf, &name); }
						b"bSoundType" => { self.bSoundType = parsei8(reader, buf, &name); }
						b"ubGraphicType" => { self.ubGraphicType = parseu8(reader, buf, &name); }
						b"ubGraphicNum" => { self.ubGraphicNum = parseu16(reader, buf, &name); }
						b"ubWeight" => { self.ubWeight = parseu16(reader, buf, &name); }
						b"ubPerPocket" => { self.ubPerPocket = parseu8(reader, buf, &name); }
						b"ItemSize" => { self.ItemSize = parseu16(reader, buf, &name); }
						b"ItemSizeBonus" => { self.ItemSizeBonus = parsei16(reader, buf, &name); }
						b"usPrice" => { self.usPrice = parseu16(reader, buf, &name); }
						b"ubCoolness" => { self.ubCoolness = parseu8(reader, buf, &name); }
						b"bReliability" => { self.bReliability = parsei8(reader, buf, &name); }
						b"bRepairEase" => { self.bRepairEase = parsei8(reader, buf, &name); }

						b"Damageable" => { self.Damageable = parsebool(reader, buf, &name); }
						b"Repairable" => { self.Repairable = parsebool(reader, buf, &name); }
						b"WaterDamages" => { self.WaterDamages = parsebool(reader, buf, &name); }
						b"Metal" => { self.Metal = parsebool(reader, buf, &name); }
						b"Sinks" => { self.Sinks = parsebool(reader, buf, &name); }
						b"ShowStatus" => {self.showstatus = parsebool(reader, buf, &name); }
						b"HiddenAddon" => {self.hiddenaddon = parsebool(reader, buf, &name); }
						b"TwoHanded" => {self.twohanded = parsebool(reader, buf, &name); }
						b"NotBuyable" => {self.notbuyable = parsebool(reader, buf, &name); }
						b"Attachment" => {self.attachment = parsebool(reader, buf, &name); }
						b"HiddenAttachment" => {self.hiddenattachment = parsebool(reader, buf, &name); }
						b"BlockIronSight" => {self.blockironsight = parsebool(reader, buf, &name); }
						b"BigGunList" => {self.biggunlist = parsebool(reader, buf, &name); }
						b"SciFi" => {self.scifi = parsebool(reader, buf, &name); }
						b"NotInEditor" => {self.notineditor = parsebool(reader, buf, &name); }
						b"DefaultUndroppable" => {self.defaultundroppable = parsebool(reader, buf, &name); }
						b"Unaerodynamic" => {self.unaerodynamic = parsebool(reader, buf, &name); }
						b"Electronic" => {self.electronic = parsebool(reader, buf, &name); }
					
						b"Inseparable" => { self.inseparable = parseu8(reader, buf, &name); }
						b"BR_NewInventory" => {self.BR_NewInventory = parseu8(reader, buf, &name);}
						b"BR_UsedInventory" => {self.BR_UsedInventory = parseu8(reader, buf, &name);}
						b"BR_ROF" => {self.BR_ROF = parsei16(reader, buf, &name);}
						b"PercentNoiseReduction" => {self.percentnoisereduction = parsei16(reader, buf, &name);}
						b"HideMuzzleFlash" => {self.hidemuzzleflash = parsebool(reader, buf, &name);}
						b"Bipod" => {self.bipod = parsei16(reader, buf, &name);}
						b"RangeBonus" => {self.rangebonus = parsei16(reader, buf, &name);}
						b"PercentRangeBonus" => {self.percentrangebonus = parsei16(reader, buf, &name);}
						b"ToHitBonus" => {self.tohitbonus = parsei16(reader, buf, &name);}
						b"BestLaserRange" => {self.bestlaserrange = parsei16(reader, buf, &name);}
						b"AimBonus" => {self.aimbonus = parsei16(reader, buf, &name);}
						b"MinRangeForAimBonus" => {self.minrangeforaimbonus = parsei16(reader, buf, &name);}
						b"MagSizeBonus" => {self.magsizebonus = parsei16(reader, buf, &name);}
						b"RateOfFireBonus" => {self.rateoffirebonus = parsei16(reader, buf, &name);}
						b"BulletSpeedBonus" => {self.bulletspeedbonus = parsei16(reader, buf, &name);}
						b"BurstSizeBonus" => {self.burstsizebonus = parsei16(reader, buf, &name);}
						b"BurstToHitBonus" => {self.bursttohitbonus = parsei16(reader, buf, &name);}
						b"AutoFireToHitBonus" => {self.autofiretohitbonus = parsei16(reader, buf, &name);}
						b"APBonus" => {self.APBonus = parsei16(reader, buf, &name);}
						b"PercentBurstFireAPReduction" => {self.percentburstfireapreduction = parsei16(reader, buf, &name);}
						b"PercentAutofireAPReduction" => {self.percentautofireapreduction = parsei16(reader, buf, &name);}
						b"PercentReadyTimeAPReduction" => {self.percentreadytimeapreduction = parsei16(reader, buf, &name);}
						b"PercentReloadTimeAPReduction" => {self.percentreloadtimeapreduction = parsei16(reader, buf, &name);}
						b"PercentAPReduction" => {self.percentapreduction = parsei16(reader, buf, &name);}
						b"PercentStatusDrainReduction" => {self.percentstatusdrainreduction = parsei16(reader, buf, &name);}
						b"DamageBonus" => {self.damagebonus = parsei16(reader, buf, &name);}
						b"MeleeDamageBonus" => {self.meleedamagebonus = parsei16(reader, buf, &name);}
						b"GrenadeLauncher" => {self.grenadelauncher = parsebool(reader, buf, &name);}
						b"Duckbill" => {self.duckbill = parsebool(reader, buf, &name);}
						b"GLGrenade" => {self.glgrenade = parsebool(reader, buf, &name);}
						b"Mine" => {self.mine = parsebool(reader, buf, &name);}
						b"Mortar" => {self.mortar = parsebool(reader, buf, &name);}
						b"RocketLauncher" => {self.rocketlauncher = parsebool(reader, buf, &name);}
						b"SingleShotRocketLauncher" => {self.singleshotrocketlauncher = parsebool(reader, buf, &name);}
						b"DiscardedLauncherItem" => {self.discardedlauncheritem = parseu16(reader, buf, &name);}
						b"RocketRifle" => {self.rocketrifle = parsebool(reader, buf, &name);}
						b"Cannon" => {self.cannon = parsebool(reader, buf, &name);}

						b"DefaultAttachment" => {
							let value = parseu16(reader, buf, &name);
							self.defaultattachments.push(value);
						}

						b"BrassKnuckles" => {self.brassknuckles = parsebool(reader, buf, &name);}
						b"Crowbar" => {self.crowbar = parsebool(reader, buf, &name);}
						b"BloodiedItem" => {self.bloodieditem = parsei16(reader, buf, &name);}
						b"Rock" => {self.rock = parsebool(reader, buf, &name);}
						b"CamoBonus" => {self.camobonus = parsei16(reader, buf, &name);}
						b"UrbanCamoBonus" => {self.urbanCamobonus = parsei16(reader, buf, &name);}
						b"DesertCamoBonus" => {self.desertCamobonus = parsei16(reader, buf, &name);}
						b"SnowCamoBonus" => {self.snowCamobonus = parsei16(reader, buf, &name);}
						b"StealthBonus" => {self.stealthbonus = parsei16(reader, buf, &name);}
						b"FlakJacket" => {self.flakjacket = parsebool(reader, buf, &name);}
						b"LeatherJacket" => {self.leatherjacket = parsebool(reader, buf, &name);}
						b"Directional" => {self.directional = parsebool(reader, buf, &name);}
						b"RemoteTrigger" => {self.remotetrigger = parsebool(reader, buf, &name);}
						b"LockBomb" => {self.lockbomb = parsebool(reader, buf, &name);}
						b"Flare" => {self.flare = parsebool(reader, buf, &name);}
						b"RobotRemoteControl" => {self.robotremotecontrol = parsebool(reader, buf, &name);}
						b"Walkman" => {self.walkman = parsebool(reader, buf, &name);}
						b"HearingRangeBonus" => {self.hearingrangebonus = parsei16(reader, buf, &name);}
						b"VisionRangeBonus" => {self.visionrangebonus = parsei16(reader, buf, &name);}
						b"NightVisionRangeBonus" => {self.nightvisionrangebonus = parsei16(reader, buf, &name);}
						b"DayVisionRangeBonus" => {self.dayvisionrangebonus = parsei16(reader, buf, &name);}
						b"CaveVisionRangeBonus" => {self.cavevisionrangebonus = parsei16(reader, buf, &name);}
						b"BrightLightVisionRangeBonus" => {self.brightlightvisionrangebonus = parsei16(reader, buf, &name);}
						b"PercentTunnelVision" => {self.percenttunnelvision = parseu8(reader, buf, &name);}
						b"FlashLightRange" => {self.usFlashLightRange = parseu8(reader, buf, &name);}
						b"ThermalOptics" => {self.thermaloptics = parsebool(reader, buf, &name);}
						b"GasMask" => {self.gasmask = parsebool(reader, buf, &name);}
						b"Alcohol" => {self.alcohol = parsef32(reader, buf, &name);}
						b"Hardware" => {self.hardware = parsebool(reader, buf, &name);}
						b"Medical" => {self.medical = parsebool(reader, buf, &name);}
						b"DrugType" => {self.drugtype = parseu32(reader, buf, &name);}
						b"CamouflageKit" => {self.camouflagekit = parsebool(reader, buf, &name);}
						b"LocksmithKit" => {self.locksmithkit = parsebool(reader, buf, &name);}
						b"Toolkit" => {self.toolkit = parsebool(reader, buf, &name);}
						b"FirstAidKit" => {self.firstaidkit = parsebool(reader, buf, &name);}
						b"MedicalKit" => {self.medicalkit = parsebool(reader, buf, &name);}
						b"WireCutters" => {self.wirecutters = parsebool(reader, buf, &name);}
						b"Canteen" => {self.canteen = parsebool(reader, buf, &name);}
						b"GasCan" => {self.gascan = parsebool(reader, buf, &name);}
						b"Marbles" => {self.marbles = parsebool(reader, buf, &name);}
						b"CanAndString" => {self.canandstring = parsebool(reader, buf, &name);}
						b"Jar" => {self.jar = parsebool(reader, buf, &name);}
						b"XRay" => {self.xray = parsebool(reader, buf, &name);}
			
						b"Batteries" => {self.batteries = parsebool(reader, buf, &name);}
						b"NeedsBatteries" => {self.needsbatteries = parsebool(reader, buf, &name);}
						b"ContainsLiquid" => {self.containsliquid = parsebool(reader, buf, &name);}
						b"MetalDetector" => {self.metaldetector = parsebool(reader, buf, &name);}
						b"usSpotting" => {self.usSpotting = parsei16(reader, buf, &name);}
						b"FingerPrintID" => {self.fingerprintid = parsebool(reader, buf, &name);}
						b"TripWireActivation" => {self.tripwireactivation = parsebool(reader, buf, &name);}
						b"TripWire" => {self.tripwire = parsebool(reader, buf, &name);}
						b"NewInv" => {self.newinv = parsebool(reader, buf, &name);}
						b"AttachmentSystem" => {self.ubAttachmentSystem = parseu8(reader, buf, &name);}
						b"ScopeMagFactor" => {self.scopemagfactor = parsef32(reader, buf, &name);}
						b"ProjectionFactor" => {self.projectionfactor = parsef32(reader, buf, &name);}
						b"RecoilModifierX" => {self.RecoilModifierX = parsef32(reader, buf, &name);}
						b"RecoilModifierY" => {self.RecoilModifierY = parsef32(reader, buf, &name);}
						b"PercentRecoilModifier" => {self.PercentRecoilModifier = parsei16(reader, buf, &name);}
						b"PercentAccuracyModifier" => {self.percentaccuracymodifier = parsei16(reader, buf, &name);}
						b"barrel" => {self.barrel = parsebool(reader, buf, &name);}
						b"usOverheatingCooldownFactor" => {self.usOverheatingCooldownFactor = parsef32(reader, buf, &name);}
						b"overheatTemperatureModificator" => {self.overheatTemperatureModificator = parsef32(reader, buf, &name);}
						b"overheatCooldownModificator" => {self.overheatCooldownModificator = parsef32(reader, buf, &name);}
						b"overheatJamThresholdModificator" => {self.overheatJamThresholdModificator = parsef32(reader, buf, &name);}
						b"overheatDamageThresholdModificator" => {self.overheatDamageThresholdModificator = parsef32(reader, buf, &name);}
						b"FoodType" => {self.foodtype = parseu32(reader, buf, &name);}
						b"LockPickModifier" => {self.LockPickModifier = parsei8(reader, buf, &name);}
						b"CrowbarModifier" => {self.CrowbarModifier = parseu8(reader, buf, &name);}
						b"DisarmModifier" => {self.DisarmModifier = parseu8(reader, buf, &name);}
						b"RepairModifier" => {self.RepairModifier = parsei8(reader, buf, &name);}
						b"usHackingModifier" => {self.usHackingModifier = parseu8(reader, buf, &name);}
						b"usBurialModifier" => {self.usBurialModifier = parseu8(reader, buf, &name);}
						b"DamageChance" => {self.usDamageChance = parseu8(reader, buf, &name);}
						b"DirtIncreaseFactor" => {self.dirtIncreaseFactor = parsef32(reader, buf, &name);}
						b"clothestype" => {self.clothestype = parseu32(reader, buf, &name);}
						b"usActionItemFlag" => {self.usActionItemFlag = parseu32(reader, buf, &name);}
						b"randomitem" => {self.randomitem = parseu16(reader, buf, &name);}
						b"randomitemcoolnessmodificator" => {self.randomitemcoolnessmodificator = parsei8(reader, buf, &name);}
						b"ItemChoiceTimeSetting" => {self.usItemChoiceTimeSetting = parseu8(reader, buf, &name);}
						b"buddyitem" => {self.usBuddyItem = parseu16(reader, buf, &name);}
						b"SleepModifier" => {self.ubSleepModifier = parseu8(reader, buf, &name);}
						b"sBackpackWeightModifier" => {self.sBackpackWeightModifier = parsei16(reader, buf, &name);}
						b"fAllowClimbing" => {self.fAllowClimbing = parsebool(reader, buf, &name);}
						b"antitankmine" => {self.antitankmine = parsebool(reader, buf, &name);}
						b"cigarette" => {self.cigarette = parsebool(reader, buf, &name);}
						b"usPortionSize" => {self.usPortionSize = parseu8(reader, buf, &name);}
						b"usRiotShieldStrength" => {self.usRiotShieldStrength = parseu16(reader, buf, &name);}
						b"usRiotShieldGraphic" => {self.usRiotShieldGraphic = parseu16(reader, buf, &name);}
						b"sFireResistance" => {self.sFireResistance = parsei16(reader, buf, &name);}
						b"RobotDamageReduction" => {self.fRobotDamageReductionModifier = parsef32(reader, buf, &name);}
						b"RobotStrBonus" => {self.bRobotStrBonus = parsei8(reader, buf, &name);}
						b"RobotAgiBonus" => {self.bRobotAgiBonus = parsei8(reader, buf, &name);}
						b"RobotDexBonus" => {self.bRobotDexBonus = parsei8(reader, buf, &name);}
						b"ProvidesRobotCamo" => {self.fProvidesRobotCamo = parsebool(reader, buf, &name);}
						b"ProvidesRobotNightVision" => {self.fProvidesRobotNightVision = parsebool(reader, buf, &name);}
						b"ProvidesRobotLaserBonus" => {self.fProvidesRobotLaserBonus = parsebool(reader, buf, &name);}
						b"RobotChassisSkillGrant" => {self.bRobotChassisSkillGrant = parsei8(reader, buf, &name);}
						b"RobotTargetingSkillGrant" => {self.bRobotTargetingSkillGrant = parsei8(reader, buf, &name);}
						b"RobotUtilitySkillGrant" => {self.bRobotUtilitySkillGrant = parsei8(reader, buf, &name);}
						// STAND/CROUCH/PRONE_MODIFIERS
						b"STAND_MODIFIERS" => { self.readStanceModifiers(reader, 0); }
						b"CROUCH_MODIFIERS" => { self.readStanceModifiers(reader, 1); }
						b"PRONE_MODIFIERS" => { self.readStanceModifiers(reader, 2); }
						_ => {}
					}
				}

				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"ITEM" => break,
						_ => ()
					}
				}
				_ => (),
			}
			buf.clear();
		}	
	}

	fn readStanceModifiers(&mut self, reader: &mut Reader<BufReader<File>>, i: usize)
	{
		let mut buf = Vec::new();
		loop {
			match reader.read_event_into(&mut buf) 
			{
				Ok(Event::Start(e)) => 
				{
					let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
					match e.name().as_ref()
					{
						b"FlatBase" => {
							let value = parsei16(reader, &mut buf, &name);
							self.flatbasemodifier[i] = value;
						}
						b"PercentBase" => {
							let value = parsei16(reader, &mut buf, &name);
							self.percentbasemodifier[i] = value;
						}
						b"FlatAim" => {
							let value = parsei16(reader, &mut buf, &name);
							self.flataimmodifier[i] = value;
						}
						b"PercentCap" => {
							let value = parsei16(reader, &mut buf, &name);
							self.percentcapmodifier[i] = value;
						}
						b"PercentHandling" => {
							let value = parsei16(reader, &mut buf, &name);
							self.percenthandlingmodifier[i] = value;
						}
						b"PercentTargetTrackingSpeed" => {
							let value = parsei16(reader, &mut buf, &name);
							self.targettrackingmodifier[i] = value;
						}
						b"PercentDropCompensation" => {
							let value = parsei16(reader, &mut buf, &name);
							self.percentdropcompensationmodifier[i] = value;
						}
						b"PercentMaxCounterForce" => {
							let value = parsei16(reader, &mut buf, &name);
							self.maxcounterforcemodifier[i] = value;
						}
						b"PercentCounterForceAccuracy" => {
							let value = parsei16(reader, &mut buf, &name);
							self.counterforceaccuracymodifier[i] = value;
						}
						b"PercentCounterForceFrequency" => {
							let value = parsei16(reader, &mut buf, &name);
							self.counterforcefrequency[i] = value;
						}
						b"AimLevels" => {
							let value = parsei16(reader, &mut buf, &name);
							self.aimlevelsmodifier[i] = value;
						}
						_ => ()
					}
				}
				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				Ok(Event::End(ref element)) => 
				{
					match element.name().as_ref()
					{
						b"STAND_MODIFIERS" => break,
						b"CROUCH_MODIFIERS" => break,
						b"PRONE_MODIFIERS" => break,
						_ => ()
					}
				}
				_ => ()
			}
			buf.clear();
		}
	}

	pub fn save(&self, file: &mut Vec<u8>, forcewrite: bool)
	{
		write!(file, "\t<ITEM>\n").unwrap();

		let value = self.uiIndex;
		write_tag_i!(file, value, "uiIndex", forcewrite);
		
		let value = &self.szItemName;
		write_tag_s!(file, value, "szItemName", forcewrite);
		
		let value = &self.szLongItemName;
		write_tag_s!(file, value, "szLongItemName", forcewrite);
		
		let value = &self.szItemDesc;
		write_tag_s!(file, value, "szItemDesc", forcewrite);
		
		let value = &self.szBRName;
		write_tag_s!(file, value, "szBRName", forcewrite);
		
		let value = &self.szBRDesc;
		write_tag_s!(file, value, "szBRDesc", forcewrite);
		
		let value = self.usItemClass;
		write_tag_i!(file, value, "usItemClass", forcewrite);
		
		let value = self.AttachmentClass;
		write_tag_i!(file, value, "AttachmentClass", forcewrite);
		
		let value = self.nasAttachmentClass;
		write_tag_i!(file, value, "nasAttachmentClass", forcewrite);
		
		let value = self.nasLayoutClass;
		write_tag_i!(file, value, "nasLayoutClass", forcewrite);

		for p in &self.AvailableAttachmentPoint.points
		{
			let p = p.clone();
			write_tag_i!(file, p, "AvailableAttachmentPoint", forcewrite);
		}
		let value = self.ulAttachmentPoint;
		write_tag_i!(file, value, "AttachmentPoint", forcewrite);

		let value = self.ubAttachToPointAPCost;
		write_tag_i!(file, value, "AttachToPointAPCost", forcewrite);

		let value = self.ubClassIndex;
		write_tag_i!(file, value, "ubClassIndex", forcewrite);

		let value = self.usItemFlag;
		write_tag_i!(file, value, "ItemFlag", forcewrite);

		let value = self.ubCursor;
		write_tag_i!(file, value, "ubCursor", forcewrite);

		let value = self.bSoundType;
		write_tag_i!(file, value, "bSoundType", forcewrite);

		let value = self.ubGraphicType;
		write_tag_i!(file, value, "ubGraphicType", forcewrite);

		let value = self.ubGraphicNum;
		write_tag_i!(file, value, "ubGraphicNum", forcewrite);

		let value = self.ubWeight;
		write_tag_i!(file, value, "ubWeight", forcewrite);

		let value = self.ubPerPocket;
		write_tag_i!(file, value, "ubPerPocket", forcewrite);

		let value = self.ItemSize;
		write_tag_i!(file, value, "ItemSize", forcewrite);

		let value = self.ItemSizeBonus;
		write_tag_i!(file, value, "ItemSizeBonus", forcewrite);

		let value = self.usPrice;
		write_tag_i!(file, value, "usPrice", forcewrite);

		let value = self.ubCoolness;
		write_tag_i!(file, value, "ubCoolness", forcewrite);

		let value = self.bReliability;
		write_tag_i!(file, value, "bReliability", forcewrite);

		let value = self.bRepairEase;
		write_tag_i!(file, value, "bRepairEase", forcewrite);

		let value = self.Damageable as u32;
		write_tag_i!(file, value, "Damageable", forcewrite);

		let value = self.Repairable as u32;
		write_tag_i!(file, value, "Repairable", forcewrite);

		let value = self.WaterDamages as u32;
		write_tag_i!(file, value, "WaterDamages", forcewrite);

		let value = self.Metal as u32;
		write_tag_i!(file, value, "Metal", forcewrite);

		let value = self.Sinks as u32;
		write_tag_i!(file, value, "Sinks", forcewrite);


		let value = self.showstatus as u32;
		write_tag_i!(file, value, "ShowStatus", forcewrite);

		let value = self.hiddenaddon as u32;
		write_tag_i!(file, value, "HiddenAddon", forcewrite);

		let value = self.twohanded as u32;
		write_tag_i!(file, value, "TwoHanded", forcewrite);

		let value = self.notbuyable as u32;
		write_tag_i!(file, value, "NotBuyable", forcewrite);

		let value = self.attachment as u32;
		write_tag_i!(file, value, "Attachment", forcewrite);

		let value = self.hiddenattachment as u32;
		write_tag_i!(file, value, "HiddenAttachment", forcewrite);

		let value = self.blockironsight as u32;
		write_tag_i!(file, value, "BlockIronSight", forcewrite);

		let value = self.biggunlist as u32;
		write_tag_i!(file, value, "BigGunList", forcewrite);

		let value = self.scifi as u32;
		write_tag_i!(file, value, "SciFi", forcewrite);

		let value = self.notineditor as u32;
		write_tag_i!(file, value, "NotInEditor", forcewrite);

		let value = self.defaultundroppable as u32;
		write_tag_i!(file, value, "DefaultUndroppable", forcewrite);

		let value = self.unaerodynamic as u32;
		write_tag_i!(file, value, "Unaerodynamic", forcewrite);

		let value = self.electronic as u32;
		write_tag_i!(file, value, "Electronic", forcewrite);


		let value = self.inseparable;
		write_tag_i!(file, value, "Inseparable", forcewrite);

		let value = self.BR_NewInventory;
		write_tag_i!(file, value, "BR_NewInventory", forcewrite);

		let value = self.BR_UsedInventory;
		write_tag_i!(file, value, "BR_UsedInventory", forcewrite);

		let value = self.BR_ROF;
		write_tag_i!(file, value, "BR_ROF", forcewrite);

		let value = self.percentnoisereduction;
		write_tag_i!(file, value, "PercentNoiseReduction", forcewrite);

		let value = self.hidemuzzleflash as u32;
		write_tag_i!(file, value, "HideMuzzleFlash", forcewrite);

		let value = self.bipod;
		write_tag_i!(file, value, "Bipod", forcewrite);

		let value = self.rangebonus;
		write_tag_i!(file, value, "RangeBonus", forcewrite);

		let value = self.percentrangebonus;
		write_tag_i!(file, value, "PercentRangeBonus", forcewrite);

		let value = self.tohitbonus;
		write_tag_i!(file, value, "ToHitBonus", forcewrite);

		let value = self.bestlaserrange;
		write_tag_i!(file, value, "BestLaserRange", forcewrite);

		let value = self.aimbonus;
		write_tag_i!(file, value, "AimBonus", forcewrite);

		let value = self.minrangeforaimbonus;
		write_tag_i!(file, value, "MinRangeForAimBonus", forcewrite);

		let value = self.magsizebonus;
		write_tag_i!(file, value, "MagSizeBonus", forcewrite);

		let value = self.rateoffirebonus;
		write_tag_i!(file, value, "RateOfFireBonus", forcewrite);

		let value = self.bulletspeedbonus;
		write_tag_i!(file, value, "BulletSpeedBonus", forcewrite);

		let value = self.burstsizebonus;
		write_tag_i!(file, value, "BurstSizeBonus", forcewrite);

		let value = self.bursttohitbonus;
		write_tag_i!(file, value, "BurstToHitBonus", forcewrite);

		let value = self.autofiretohitbonus;
		write_tag_i!(file, value, "AutoFireToHitBonus", forcewrite);

		let value = self.APBonus;
		write_tag_i!(file, value, "APBonus", forcewrite);

		let value = self.percentburstfireapreduction;
		write_tag_i!(file, value, "PercentBurstFireAPReduction", forcewrite);

		let value = self.percentautofireapreduction;
		write_tag_i!(file, value, "PercentAutofireAPReduction", forcewrite);

		let value = self.percentreadytimeapreduction;
		write_tag_i!(file, value, "PercentReadyTimeAPReduction", forcewrite);

		let value = self.percentreloadtimeapreduction;
		write_tag_i!(file, value, "PercentReloadTimeAPReduction", forcewrite);

		let value = self.percentapreduction;
		write_tag_i!(file, value, "PercentAPReduction", forcewrite);

		let value = self.percentstatusdrainreduction;
		write_tag_i!(file, value, "PercentStatusDrainReduction", forcewrite);

		let value = self.damagebonus;
		write_tag_i!(file, value, "DamageBonus", forcewrite);

		let value = self.meleedamagebonus;
		write_tag_i!(file, value, "MeleeDamageBonus", forcewrite);

		let value = self.grenadelauncher as u32;
		write_tag_i!(file, value, "GrenadeLauncher", forcewrite);

		let value = self.duckbill as u32;
		write_tag_i!(file, value, "Duckbill", forcewrite);

		let value = self.glgrenade as u32;
		write_tag_i!(file, value, "GLGrenade", forcewrite);

		let value = self.mine as u32;
		write_tag_i!(file, value, "Mine", forcewrite);

		let value = self.mortar as u32;
		write_tag_i!(file, value, "Mortar", forcewrite);

		let value = self.rocketlauncher as u32;
		write_tag_i!(file, value, "RocketLauncher", forcewrite);

		let value = self.singleshotrocketlauncher as u32;
		write_tag_i!(file, value, "SingleShotRocketLauncher", forcewrite);

		let value = self.discardedlauncheritem;
		write_tag_i!(file, value, "DiscardedLauncherItem", forcewrite);

		let value = self.rocketrifle as u32;
		write_tag_i!(file, value, "RocketRifle", forcewrite);

		let value = self.cannon as u32;
		write_tag_i!(file, value, "Cannon", forcewrite);


		for p in &self.defaultattachments
		{
			let p = p.clone();
			write_tag_i!(file, p, "DefaultAttachment", forcewrite);
		}

		let value = self.brassknuckles as u32;
		write_tag_i!(file, value, "BrassKnuckles", forcewrite);

		let value = self.crowbar as u32;
		write_tag_i!(file, value, "Crowbar", forcewrite);

		let value = self.bloodieditem;
		write_tag_i!(file, value, "BloodiedItem", forcewrite);

		let value = self.rock as u32;
		write_tag_i!(file, value, "Rock", forcewrite);

		let value = self.camobonus;
		write_tag_i!(file, value, "CamoBonus", forcewrite);

		let value = self.urbanCamobonus;
		write_tag_i!(file, value, "UrbanCamoBonus", forcewrite);

		let value = self.desertCamobonus;
		write_tag_i!(file, value, "DesertCamoBonus", forcewrite);

		let value = self.snowCamobonus;
		write_tag_i!(file, value, "SnowCamoBonus", forcewrite);

		let value = self.stealthbonus;
		write_tag_i!(file, value, "StealthBonus", forcewrite);

		let value = self.flakjacket as u32;
		write_tag_i!(file, value, "FlakJacket", forcewrite);

		let value = self.leatherjacket as u32;
		write_tag_i!(file, value, "LeatherJacket", forcewrite);

		let value = self.directional as u32;
		write_tag_i!(file, value, "Directional", forcewrite);

		let value = self.remotetrigger as u32;
		write_tag_i!(file, value, "RemoteTrigger", forcewrite);

		let value = self.lockbomb as u32;
		write_tag_i!(file, value, "LockBomb", forcewrite);

		let value = self.flare as u32;
		write_tag_i!(file, value, "Flare", forcewrite);

		let value = self.robotremotecontrol as u32;
		write_tag_i!(file, value, "RobotRemoteControl", forcewrite);

		let value = self.walkman as u32;
		write_tag_i!(file, value, "Walkman", forcewrite);

		let value = self.hearingrangebonus;
		write_tag_i!(file, value, "HearingRangeBonus", forcewrite);

		let value = self.visionrangebonus;
		write_tag_i!(file, value, "VisionRangeBonus", forcewrite);

		let value = self.nightvisionrangebonus;
		write_tag_i!(file, value, "NightVisionRangeBonus", forcewrite);

		let value = self.dayvisionrangebonus;
		write_tag_i!(file, value, "DayVisionRangeBonus", forcewrite);

		let value = self.cavevisionrangebonus;
		write_tag_i!(file, value, "CaveVisionRangeBonus", forcewrite);

		let value = self.brightlightvisionrangebonus;
		write_tag_i!(file, value, "BrightLightVisionRangeBonus", forcewrite);

		let value = self.percenttunnelvision;
		write_tag_i!(file, value, "PercentTunnelVision", forcewrite);

		let value = self.usFlashLightRange;
		write_tag_i!(file, value, "FlashLightRange", forcewrite);

		let value = self.thermaloptics as u32;
		write_tag_i!(file, value, "ThermalOptics", forcewrite);

		let value = self.gasmask as u32;
		write_tag_i!(file, value, "GasMask", forcewrite);

		let value = self.alcohol as u32;
		write_tag_i!(file, value, "Alcohol", forcewrite);

		let value = self.hardware as u32;
		write_tag_i!(file, value, "Hardware", forcewrite);

		let value = self.medical as u32;
		write_tag_i!(file, value, "Medical", forcewrite);

		let value = self.drugtype;
		write_tag_i!(file, value, "DrugType", forcewrite);

		let value = self.camouflagekit as u32;
		write_tag_i!(file, value, "CamouflageKit", forcewrite);

		let value = self.locksmithkit as u32;
		write_tag_i!(file, value, "LocksmithKit", forcewrite);

		let value = self.toolkit as u32;
		write_tag_i!(file, value, "Toolkit", forcewrite);

		let value = self.firstaidkit as u32;
		write_tag_i!(file, value, "FirstAidKit", forcewrite);

		let value = self.medicalkit as u32;
		write_tag_i!(file, value, "MedicalKit", forcewrite);

		let value = self.wirecutters as u32;
		write_tag_i!(file, value, "WireCutters", forcewrite);

		let value = self.canteen as u32;
		write_tag_i!(file, value, "Canteen", forcewrite);

		let value = self.gascan as u32;
		write_tag_i!(file, value, "GasCan", forcewrite);

		let value = self.marbles as u32;
		write_tag_i!(file, value, "Marbles", forcewrite);

		let value = self.canandstring as u32;
		write_tag_i!(file, value, "CanAndString", forcewrite);

		let value = self.jar as u32;
		write_tag_i!(file, value, "Jar", forcewrite);

		let value = self.xray as u32;
		write_tag_i!(file, value, "XRay", forcewrite);

		let value = self.batteries as u32;
		write_tag_i!(file, value, "Batteries", forcewrite);
		let value = self.needsbatteries as u32;
		write_tag_i!(file, value, "NeedsBatteries", forcewrite);
		let value = self.containsliquid as u32;
		write_tag_i!(file, value, "ContainsLiquid", forcewrite);  
		let value = self.metaldetector as u32;
		write_tag_i!(file, value, "MetalDetector", forcewrite);  
		let value = self.usSpotting;
		write_tag_i!(file, value, "usSpotting", forcewrite);  
		let value = self.fingerprintid as u32;
		write_tag_i!(file, value, "FingerPrintID", forcewrite);  
		let value = self.tripwireactivation as u32;
		write_tag_i!(file, value, "TripWireActivation", forcewrite);  
		let value = self.tripwire as u32;
		write_tag_i!(file, value, "TripWire", forcewrite);  
		let value = self.newinv as u32;
		write_tag_i!(file, value, "NewInv", forcewrite);  
		let value = self.ubAttachmentSystem;
		write_tag_i!(file, value, "AttachmentSystem", forcewrite);  
		let value = self.scopemagfactor;
		write_tag_f!(file, value, "ScopeMagFactor", forcewrite);  
		let value = self.projectionfactor;
		write_tag_f!(file, value, "ProjectionFactor", forcewrite);  
		let value = self.RecoilModifierX;
		write_tag_f!(file, value, "RecoilModifierX", forcewrite);  
		let value = self.RecoilModifierY;
		write_tag_f!(file, value, "RecoilModifierY", forcewrite);  
		let value = self.PercentRecoilModifier;
		write_tag_i!(file, value, "PercentRecoilModifier", forcewrite);  
		let value = self.percentaccuracymodifier;
		write_tag_i!(file, value, "PercentAccuracyModifier", forcewrite);  
		let value = self.barrel as u32;
		write_tag_i!(file, value, "barrel", forcewrite);  
		let value = self.usOverheatingCooldownFactor;
		write_tag_f!(file, value, "usOverheatingCooldownFactor", forcewrite);  
		let value = self.overheatTemperatureModificator;
		write_tag_f!(file, value, "overheatTemperatureModificator", forcewrite);  
		let value = self.overheatCooldownModificator;
		write_tag_f!(file, value, "overheatCooldownModificator", forcewrite);  
		let value = self.overheatJamThresholdModificator;
		write_tag_f!(file, value, "overheatJamThresholdModificator", forcewrite);  
		let value = self.overheatDamageThresholdModificator;
		write_tag_f!(file, value, "overheatDamageThresholdModificator", forcewrite);  
		let value = self.foodtype;
		write_tag_i!(file, value, "FoodType", forcewrite);  
		let value = self.LockPickModifier;
		write_tag_i!(file, value, "LockPickModifier", forcewrite);  
		let value = self.CrowbarModifier;
		write_tag_i!(file, value, "CrowbarModifier", forcewrite);  
		let value = self.DisarmModifier;
		write_tag_i!(file, value, "DisarmModifier", forcewrite);  
		let value = self.RepairModifier;
		write_tag_i!(file, value, "RepairModifier", forcewrite);  
		let value = self.usHackingModifier;
		write_tag_i!(file, value, "usHackingModifier", forcewrite);  
		let value = self.usBurialModifier;
		write_tag_i!(file, value, "usBurialModifier", forcewrite);  
		let value = self.usDamageChance;
		write_tag_i!(file, value, "DamageChance", forcewrite);  
		let value = self.dirtIncreaseFactor;
		write_tag_f!(file, value, "DirtIncreaseFactor", forcewrite);  
		let value = self.clothestype;
		write_tag_i!(file, value, "clothestype", forcewrite);  
		let value = self.usActionItemFlag;
		write_tag_i!(file, value, "usActionItemFlag", forcewrite);  
		let value = self.randomitem;
		write_tag_i!(file, value, "randomitem", forcewrite);  
		let value = self.randomitemcoolnessmodificator;
		write_tag_i!(file, value, "randomitemcoolnessmodificator", forcewrite);  
		let value = self.usItemChoiceTimeSetting;
		write_tag_i!(file, value, "ItemChoiceTimeSetting", forcewrite);  
		let value = self.usBuddyItem;
		write_tag_i!(file, value, "buddyitem", forcewrite);  
		let value = self.ubSleepModifier;
		write_tag_i!(file, value, "SleepModifier", forcewrite);  
		let value = self.sBackpackWeightModifier;
		write_tag_i!(file, value, "sBackpackWeightModifier", forcewrite);  
		let value = self.fAllowClimbing as u32;
		write_tag_i!(file, value, "fAllowClimbing", forcewrite);  
		let value = self.antitankmine as u32;
		write_tag_i!(file, value, "antitankmine", forcewrite);  
		let value = self.cigarette as u32;
		write_tag_i!(file, value, "cigarette", forcewrite);  
		let value = self.usPortionSize;
		write_tag_i!(file, value, "usPortionSize", forcewrite);  
		let value = self.usRiotShieldStrength;
		write_tag_i!(file, value, "usRiotShieldStrength", forcewrite);  
		let value = self.usRiotShieldGraphic;
		write_tag_i!(file, value, "usRiotShieldGraphic", forcewrite);  
		let value = self.sFireResistance;
		write_tag_i!(file, value, "sFireResistance", forcewrite);  
		let value = self.fRobotDamageReductionModifier;
		write_tag_f!(file, value, "RobotDamageReduction", forcewrite);  
		let value = self.bRobotStrBonus;
		write_tag_i!(file, value, "RobotStrBonus", forcewrite);  
		let value = self.bRobotAgiBonus;
		write_tag_i!(file, value, "RobotAgiBonus", forcewrite);  
		let value = self.bRobotDexBonus;
		write_tag_i!(file, value, "RobotDexBonus", forcewrite);  
		let value = self.fProvidesRobotCamo as u32;
		write_tag_i!(file, value, "ProvidesRobotCamo", forcewrite);  
		let value = self.fProvidesRobotNightVision as u32;
		write_tag_i!(file, value, "ProvidesRobotNightVision", forcewrite);  
		let value = self.fProvidesRobotLaserBonus as u32;
		write_tag_i!(file, value, "ProvidesRobotLaserBonus", forcewrite);  
		let value = self.bRobotChassisSkillGrant;
		write_tag_i!(file, value, "RobotChassisSkillGrant", forcewrite);  
		let value = self.bRobotTargetingSkillGrant;
		write_tag_i!(file, value, "RobotTargetingSkillGrant", forcewrite);  
		let value = self.bRobotUtilitySkillGrant;
		write_tag_i!(file, value, "RobotUtilitySkillGrant", forcewrite);  

		let s = ["STAND_MODIFIERS", "CROUCH_MODIFIERS", "PRONE_MODIFIERS"];
		for i in 0..3
		{
			let flatbase = self.flatbasemodifier[i];
			let percentbase = self.percentbasemodifier[i];
			let flataim = self.flataimmodifier[i];
			let percentcap = self.percentcapmodifier[i];
			let percenthandling = self.percenthandlingmodifier[i];
			let targettracking = self.targettrackingmodifier[i];
			let dropcompensation = self.percentdropcompensationmodifier[i];
			let maxcounterforce = self.maxcounterforcemodifier[i];
			let counterforceaccuracy = self.counterforceaccuracymodifier[i];
			let counterforcefreq = self.counterforcefrequency[i];
			let aimlevels = self.aimlevelsmodifier[i];

			if flatbase == 0 && percentbase == 0 && flataim == 0 && percentcap == 0 && percenthandling == 0 && targettracking == 0 && dropcompensation == 0 && maxcounterforce == 0 && counterforceaccuracy == 0 && counterforcefreq == 0 && aimlevels == 0
			{
				write!(file, "\t\t<{} />\n", s[i]).unwrap();
			}
			else
			{
				write!(file, "\t\t<{}>\n", s[i]).unwrap();

				if flatbase != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, flatbase, "FlatBase", forcewrite);
				}
				if percentbase != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, percentbase, "PercentBase", forcewrite);
				}
				if flataim != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, flataim, "FlatAim", forcewrite);
				}
				if percentcap != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, percentcap, "PercentCap", forcewrite);
				}
				if percenthandling != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, percenthandling, "PercentHandling", forcewrite);
				}
				if targettracking != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, targettracking, "PercentTargetTrackingSpeed", forcewrite);
				}
				if dropcompensation != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, dropcompensation, "PercentDropCompensation", forcewrite);
				}
				if maxcounterforce != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, maxcounterforce, "PercentMaxCounterForce", forcewrite);
				}
				if counterforceaccuracy != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, counterforceaccuracy, "PercentCounterForceAccuracy", forcewrite);
				}
				if counterforcefreq != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, counterforcefreq, "PercentCounterForceFrequency", forcewrite);
				}
				if aimlevels != 0 || forcewrite == true {
					write!(file, "\t").unwrap();
					write_tag_i!(file, aimlevels, "AimLevels", forcewrite);
				}

				write!(file, "\t\t</{}>\n", s[i]).unwrap();
			}
		}
		write!(file, "\t</ITEM>\n").unwrap();
	}

}


pub struct AttachmentPoints
{
	points: Vec<u64>
}
impl fmt::Debug for AttachmentPoints
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		for p in &self.points
		{
			write!(f, "\t\t<AvailableAttachmentPoint>{}</AvailableAttachmentPoint>\n", p);
		}
		Ok(())
	}
}


//-----------------------------------------------------------------------------
// Functions
//-----------------------------------------------------------------------------
fn parseString(reader: &mut Reader<BufReader<File>>, buf: &mut Vec<u8>) -> String
{
	loop {
		match reader.read_event_into(buf) 
		{
			Ok(Event::Text(e)) => {
				let value = e.unescape().unwrap().into_owned();
				return value;
			}
			Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
			_ => {}
		}
	}
}

fn parsebool(reader: &mut Reader<BufReader<File>>, buf: &mut Vec<u8>, name: &str) -> bool
{
	loop {
		match reader.read_event_into(buf) 
		{
			Ok(Event::Text(e)) => {
				let value = e.unescape().unwrap().into_owned().parse::<u32>();
				match value
				{
					Ok(value) => {return value != 0;}
					_ => {println!("Error parsing value for tag {}", name); return false;}
				}
			}
			Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
			_ => {}
		}
	}
}

macro_rules! parsers {
	($($name:ident, $type:ty),*) => {
		
		$(fn $name(reader: &mut Reader<BufReader<File>>, buf: &mut Vec<u8>, name: &str) -> $type
		{
			loop {
				match reader.read_event_into(buf) 
				{
					Ok(Event::Text(e)) => {
						let value = e.unescape().unwrap().into_owned().parse::<$type>();
						match value
						{
							Ok(value) => {return value;}
							_ => {println!("Error parsing value for tag {} at position {}", name, reader.buffer_position()); return Default::default();}
						}
					}
					Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
					_ => {}
				}
			}
		})*
	};
}
parsers!(parseu8, u8, parsei8, i8, parseu16, u16, parsei16, i16, parseu32, u32, parsei32, i32, parseu64, u64, parsei64, i64, parsef32, f32);


//-----------------------------------------------------------------------------
// Enums
//-----------------------------------------------------------------------------
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<usize> for $name {
            type Error = ();

            fn try_from(v: usize) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as usize => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
#[derive(Copy, Clone)]
	pub enum ItemClass {
		None = 0x00000001,
		Gun = 0x00000002,
		Blade = 0x00000004,
		ThrowingKnife = 0x00000008,
		Launcher = 0x00000010,
		Tentacle = 0x00000020,
		Thrown = 0x00000040,
		Punch = 0x00000080,
		Grenade = 0x00000100,
		Bomb = 0x00000200,
		Ammo = 0x00000400,
		Armor = 0x00000800,
		Medkit = 0x00001000,
		Kit = 0x00002000,
		Appliable = 0x00004000,
		Face = 0x00008000,
		Key = 0x00010000,
		LBE = 0x00020000,
		BeltClip = 0x00040000,
		Misc = 0x10000000,
		Money = 0x20000000,
		Random = 0x40000000,
	}
}

back_to_enum! {
	#[derive(Copy, Clone)]
	pub enum Cursor {
		Invalid = 0,
		Quest,
		Punch,
		Target,
		Knife,
		Aid,
		Throw,
		Mine,
		Lockpick,
		MineDetector,
		Crowbar,
		CCTV,
		Camera,
		Key,
		Saw,
		WireCutters,
		Remote,
		Bomb,
		Repair,
		Trajectory,
		Jar,
		Tincan,
		Refuel,
		Fortification,
		Handcuffs,
		ApplyItem,
		InteractiveAction,
		Bloodbag,
		Splint,
	}
}

back_to_enum! {
	#[derive(Copy, Clone)]
	pub enum MagazineType {
		Magazine = 0,
		Bullets,
		Box,
		Crate,
	}
}

impl fmt::Display for MagazineType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            MagazineType::Magazine => write!(f, "Magazine"),
            MagazineType::Bullets => write!(f, "Bullet(s)"),
            MagazineType::Box => write!(f, "Box"),
            MagazineType::Crate => write!(f, "Crate"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ExplosionType {
	Normal = 0,
	Stun,
	Teargas,
	Mustardgas,
	Flare,
	Noise,
	Smoke,
	Creaturegas,
	Burnablegas,
	Flashbang,
	SignalSmoke,
	SmokeDebris,
	SmokeFireRetardant,
	AnyType,
}

pub enum ExplosionAnimationID {
	NO_BLAST,
	BLAST_1,
	BLAST_2,
	BLAST_3,
	STUN_BLAST,	
	WATER_BLAST,
	TARGAS_EXP,
	SMOKE_EXP,
	MUSTARD_EXP,
	BURN_EXP,
	THERMOBARIC_EXP,
	FLASHBANG_EXP,
	ROOF_COLLAPSE,
	ROOF_COLLAPSE_SMOKE,
	NUM_EXP_TYPES=50
}