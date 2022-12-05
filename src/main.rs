#![allow(non_snake_case)]
#![allow(unused)]
use std::env::current_dir;
// use std::io::{BufReader, Write, Read};
// use std::fs::{File, read};
// use std::fmt;
// use std::str;
use std::path::PathBuf;
use std::time::{Instant};
use fltk::button::{RadioButton, ToggleButton, CheckButton, LightButton, RepeatButton, RadioLightButton, RadioRoundButton};
use fltk::enums::{Color, Align};
use fltk::group::{Tabs, Group, FlexType, Pack};
use fltk::input::{IntInput, Input};
use fltk::menu::{MenuFlag, SysMenuBar, Choice};
use fltk::valuator::{Scrollbar, ScrollbarType};
// use quick_xml::events::{Event, BytesStart};
// use quick_xml::events::attributes::{Attributes, Attribute};
// use quick_xml::{Reader, Writer};
use fltk::{
	enums::{Shortcut, FrameType},
	button::{Button, RoundButton},
	app, frame::Frame, group::Flex, prelude::*, window::Window, image::*, *
};
use fltk_evented::Listener;
mod JAxml;
mod STI;

// TODO
// Build item info layout
// Display existing items' data
// Allow editing item data
// Add/Delete/Duplicate items
// Change item uiIndex
// Prompt to save work upon quitting if needed
// Clear item image when switching treeview / deselecting everything. No clear_image() method for widgets?
// Inventories
// Merchants?
// Error checking
// Only allow saving of valid data


fn main() 
{
	let dataPath = PathBuf::from("H:\\JA2 Dev\\Data-1.13"); // <-- Temporary start path while developing
	let mut xmldata = JAxml::JAxmlState::new();
	let mut images = STI::Images::new();
	loadData(&mut xmldata, &mut images, &dataPath);
	//-----------------------------------------------------------------------------
	// App Layouting
	//-----------------------------------------------------------------------------
	let a = app::App::default().with_scheme(app::Scheme::Base);
    app::set_font_size(14);
	let (s, r) = app::channel::<Message>();
    
    let mut mainWindow = Window::default()
        .with_size(1280, 720)
        .center_screen()
        .with_label("JA2 xml editor");

 	// Toolbar of sorts
	let menu = createMenuBar(&s);

	// Tree browser that is used to list editable items
	let mut tree: Listener<_> = tree::Tree::default().with_size(300, 700).with_pos(0, 20).into();
	tree.set_show_root(false);
	fillTree(&mut tree, &xmldata, Message::ShowAll);
	
	// Item info
	let mut itemWindow = Window::default()
		.with_size(980, 720)
		.with_pos(300, 0);
	
	let mut tabs = Tabs::new(0, 0, itemWindow.w(), 20, "");
	tabs.emit(s, Message::Redraw);
	
	let w = itemWindow.w(); let h = itemWindow.h() - tabs.h();
	
	let tab1 = Group::default().with_size(w, h).below_of(&tabs, 0).with_label("Tab1\t\t");
	// Item Graphics section
	let x = 0;
	let y = 25;
	let mut itemGraphics = ItemGraphicsArea::initialize(x, y, &s, &images);
	let mut itemStats = ItemStatsArea::initialize(x, 485);
	let mut itemDescription = ItemDescriptionArea::initialize(310, y);
	let mut itemProperties = ItemPropertiesArea::initialize(310, y + 210);
	let mut itemKit = ItemKitArea::initialize(310, 485);
	let mut itemVision = ItemVisionArea::initialize(310+235+10, 485);
	tab1.end();
    

    let tab2 = Group::default().with_size(w, h).right_of(&tab1, 0).with_label("Tab2\t\t");
    let _but1 = Button::default().with_size(0, 30).with_label("Button").center_of(&itemWindow);
    tab2.end();


    let tab3 = Group::default().with_size(w, h).right_of(&tab2, 0).with_label("Tab3\t\t");
    let _but2 = RoundButton::default().with_size(0, 30).with_label("Round").center_of(&itemWindow);
    tab3.end();


    let tab4 = Group::default().with_size(w, h).right_of(&tab3, 0).with_label("Tab4\t\t");
    let _but3 = RoundButton::default().with_size(0, 30).with_label("Round2").center_of(&itemWindow);
    tab4.end();


    tabs.end();
	itemWindow.end();
 	
	mainWindow.end();
	mainWindow.make_resizable(true);
	mainWindow.show();

	//-----------------------------------------------------------------------------
	// Main loop
	//-----------------------------------------------------------------------------    
    let mut index = 0;
    while a.wait() 
    {
		if tree.triggered()
		{
 			if let Some(item) = tree.first_selected_item() 
 			{
                println!("{} selected", item.label().unwrap());
                let uiIndex = unsafe{item.user_data::<u32>()}.unwrap();
                println!("uiIndex {}", uiIndex);
                
                let stiType = xmldata.items.items[uiIndex as usize].ubGraphicType as usize;
                let stiIndex = xmldata.items.items[uiIndex as usize].ubGraphicNum as usize;
                println!("Graphic index {}", stiIndex);
                if stiType < images.big.len() && stiIndex < images.big[stiType].len()
                {
					itemGraphics.updateItemGraphics(&images, stiType, stiIndex);

					if stiType != itemGraphics.itemType.value() as usize
					{
						itemGraphics.itemType.set_value(stiType as i32);
						itemGraphics.updateScrollBarBounds(&images);
						itemGraphics.redrawScrollAreaImages(&images);
					}
					itemGraphics.itemIndex.set_value(&format!("{}", stiIndex));

				}
				else 
				{
					println!("Graphic index out of graphic vector bounds!");
					println!("Tried to access image [{}][{}]", stiType, stiIndex);
				}
			
				itemDescription.update(&xmldata, uiIndex as usize);

				itemWindow.redraw()
			}
		}
    	
        if let Some(msg) = r.recv() 
        {
			use Message::*;
            match msg 
            {
				// Toolbar menus
				Open =>
				{
					openFileDialog(&mut xmldata, &mut images, &mut tree);
				}
				Save =>
				{
					saveFileDialog(&xmldata);
				}
				Quit =>
				{
					a.quit();
				}
				ShowAll | ShowGuns | ShowAmmo | ShowLaunchers | ShowGrenades | ShowExplosives | ShowKnives | 
				ShowOther | ShowArmor | ShowFaceGear | ShowKits | ShowKeys | ShowLBE | ShowMisc | ShowNone | 
				ShowRandom | ShowMerges | ShowAttachmentMerges | ShowLaunchables | ShowCompatibleFaceGear | 
				ShowTransforms | ShowRandomItems | ShowAttachmentList | ShowAttachmentInfo | ShowIncompatibleAttachments | 
				ShowMedical | ShowScifi | ShowNonScifi | ShowTonsOfGuns | ShowReducedGuns | ShowAttachments |
				ShowDrugs => 
				{
					fillTree(&mut tree, &xmldata, msg);
				}
				// Item Window
				Redraw => 
				{
					itemWindow.redraw();
				}
				GraphicScroll =>
				{
					itemGraphics.redrawScrollAreaImages(&images);
					itemWindow.redraw();
				}
				GraphicType =>
				{
					itemGraphics.updateScrollBarBounds(&images);
					itemGraphics.redrawScrollAreaImages(&images);

				}
				_ => {}
	        }
        }
    }
}


//-----------------------------------------------------------------------------
// Functions
//-----------------------------------------------------------------------------
fn openFileDialog(xmldata: &mut JAxml::JAxmlState, images: &mut STI::Images, tree: &mut Listener<tree::Tree>)
{
	let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
	dialog.set_directory(&current_dir().unwrap());
	dialog.show();
	println!("{:?}", dialog.filename());
	if dialog.filename().is_dir()
	{
		loadData(xmldata, images, &dialog.filename());
		fillTree(tree, &xmldata, Message::ShowGuns);
	}
}

fn saveFileDialog(xmldata: &JAxml::JAxmlState)
{
	let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
	dialog.set_directory(&current_dir().unwrap());
	dialog.show();
	println!("{:?}", dialog.filename());
	if dialog.filename().is_dir()
	{
		saveData(&dialog.filename(), xmldata);
	}
}

fn loadData(xmldata: &mut JAxml::JAxmlState, images: &mut STI::Images, dataPath: &PathBuf)
{
	let t = Instant::now();
	xmldata.loadData(&dataPath);
	println!("Loading xml data took: {:?}", t.elapsed());
	let t = Instant::now();
	images.loadImages(&dataPath);
	println!("Loading sti files took: {:?}", t.elapsed());
}

fn saveData(dataPath: &PathBuf, xmldata: &JAxml::JAxmlState)
{
	let t = Instant::now();
	xmldata.saveData(&dataPath);
	println!("Saving xml data took: {:?}", t.elapsed());
}

fn fillTree(tree: &mut Listener<tree::Tree>, xmldata: &JAxml::JAxmlState, msg: Message)
{
  	tree.clear();
  	match msg
  	{
		Message::ShowAll =>
		{
			for item in &xmldata.items.items
			{
				if item.szLongItemName.contains("/")
				{
					let name = item.szLongItemName.replace("/", "\\/");
					tree.add(&name);
				}
				else
				{
					tree.add(&item.szLongItemName);
				}
				    
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(item.uiIndex);
		    }
		}
		Message::ShowGuns =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Gun)
		}
		Message::ShowAmmo =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Ammo)
		}
		Message::ShowArmor =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Armor)
		}
    	Message::ShowLaunchers =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Launcher)
    	}
    	Message::ShowGrenades =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Grenade)
    	}
    	Message::ShowExplosives =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Bomb)
    	}
    	Message::ShowKnives =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Blade)
    	}
    	Message::ShowOther =>
    	{
			for item in &xmldata.items.items
			{
				if item.usItemClass == JAxml::ItemClass::Thrown as u32 || item.usItemClass == JAxml::ItemClass::Punch as u32
				{
					if item.szLongItemName.contains("/")
					{
						let name = item.szLongItemName.replace("/", "\\/");
						tree.add(&name);
					}
					else
					{
						tree.add(&item.szLongItemName);
					}
				      		
		      		let mut treeitem = tree.last().unwrap();
					treeitem.set_user_data(item.uiIndex);
				}
			}
    	}
    	Message::ShowFaceGear =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Face)
    	}
    	Message::ShowKits =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Kit)
    	}
    	Message::ShowMedical =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Medkit)
    	}
    	Message::ShowKeys =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Key)
    	}
    	Message::ShowLBE =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::LBE)
    	}
    	Message::ShowMisc =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Misc)
    	}
    	Message::ShowNone =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::None)
    	}
    	Message::ShowRandom =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Random)
    	}
    	Message::ShowScifi =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.scifi
			    {
					if item.szLongItemName.contains("/")
					{
						let name = item.szLongItemName.replace("/", "\\/");
						tree.add(&name);
					}
					else
					{
						tree.add(&item.szLongItemName);
					}
						
					let mut treeitem = tree.last().unwrap();
					treeitem.set_user_data(item.uiIndex);
				}
			}
    	}
    	Message::ShowNonScifi =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.scifi == false
			    {
					if item.szLongItemName.contains("/")
					{
						let name = item.szLongItemName.replace("/", "\\/");
						tree.add(&name);
					}
					else
					{
						tree.add(&item.szLongItemName);
					}
				              	
					let mut treeitem = tree.last().unwrap();
					treeitem.set_user_data(item.uiIndex);
				}
			}
    	}
    	Message::ShowTonsOfGuns =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.biggunlist
			    {
					if item.szLongItemName.contains("/")
					{
						let name = item.szLongItemName.replace("/", "\\/");
						tree.add(&name);
					}
					else
					{
						tree.add(&item.szLongItemName);
					}
							
					let mut treeitem = tree.last().unwrap();
					treeitem.set_user_data(item.uiIndex);
				}
			}
    	}
    	Message::ShowReducedGuns =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.biggunlist == false
			    {
					if item.szLongItemName.contains("/")
					{
						let name = item.szLongItemName.replace("/", "\\/");
						tree.add(&name);
					}
					else
					{
						tree.add(&item.szLongItemName);
					}
						
					let mut treeitem = tree.last().unwrap();
					treeitem.set_user_data(item.uiIndex);
				}
			}
    	}
    	Message::ShowAttachments =>
    	{
    		
    	}
    	Message::ShowDrugs =>
    	{
    		
    	}
		_ => {}
	}

	tree.redraw();
}

fn matchItemClass(xmldata: &JAxml::JAxmlState, tree: &mut Listener<tree::Tree>, itemClass: JAxml::ItemClass)
{
	for item in &xmldata.items.items
	{
		if item.usItemClass == itemClass as u32
		{
			if item.szLongItemName.contains("/")
			{
				let name = item.szLongItemName.replace("/", "\\/");
				tree.add(&name);
			}
			else
			{
				tree.add(&item.szLongItemName);
			}
				      
			let mut treeitem = tree.last().unwrap();
			treeitem.set_user_data(item.uiIndex);
		}
	}
}

fn createMenuBar(s: &app::Sender<Message>) -> menu::SysMenuBar
{
 	let mut menu = menu::SysMenuBar::default().with_size(800, 20);
	menu.set_frame(FrameType::FlatBox);
    menu.add_emit(
        "&File/Load XML data\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        *s,
        Message::Open,
    );
    menu.add_emit(
        "&File/Save XML data\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        *s,
        Message::Save,
    ); 	
    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        *s,
        Message::Quit,
    );
	menu.add_emit(
	    "&Items/Show/By Tag/Sci-fi\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowScifi
	);
	menu.add_emit(
	    "&Items/Show/By Tag/Non Sci-fi\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowNonScifi
	);
	menu.add_emit(
	    "&Items/Show/By Tag/Tons Of Guns\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowTonsOfGuns
	);
	menu.add_emit(
	    "&Items/Show/By Tag/Reduced Guns\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowReducedGuns
	);
	menu.add_emit(
	    "&Items/Show/By Tag/Attachments\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowAttachments
	);
	menu.add_emit(
	    "&Items/Show/By Tag/Drugs\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowDrugs
	);
	menu.add_emit(
	    "&Items/Show/All\t",
		Shortcut::None,
		MenuFlag::MenuDivider,
		*s,
		Message::ShowAll
	);
	menu.add_emit(
	    "&Items/Show/Guns\t",
		Shortcut::Alt | 'g',
		MenuFlag::Normal,
		*s,
		Message::ShowGuns
	);
	menu.add_emit(
	    "&Items/Show/Ammo\t",
		Shortcut::Alt | 'a',
		MenuFlag::Normal,
		*s,
		Message::ShowAmmo
	);
	menu.add_emit(
	    "&Items/Show/Launchers\t",
		Shortcut::Alt | 'l',
		MenuFlag::Normal,
		*s,
		Message::ShowLaunchers
	);
	menu.add_emit(
	    "&Items/Show/Grenades\t",
		Shortcut::Alt | 'n',
		MenuFlag::Normal,
		*s,
		Message::ShowGrenades
	);
	menu.add_emit(
	    "&Items/Show/Explosives\t",
		Shortcut::Alt | 'e',
		MenuFlag::Normal,
		*s,
		Message::ShowExplosives
	);
	menu.add_emit(
	    "&Items/Show/Knives\t",
		Shortcut::Alt | 'k',
		MenuFlag::Normal,
		*s,
		Message::ShowKnives
	);
	menu.add_emit(
	    "&Items/Show/Other Weapons\t",
		Shortcut::Alt | 'o',
		MenuFlag::Normal,
		*s,
		Message::ShowOther
	);
	menu.add_emit(
	    "&Items/Show/Armor\t",
		Shortcut::Alt | 'r',
		MenuFlag::Normal,
		*s,
		Message::ShowArmor
	);
	menu.add_emit(
	    "&Items/Show/Facial Gear\t",
		Shortcut::Alt | 'f',
		MenuFlag::Normal,
		*s,
		Message::ShowFaceGear
	);
	menu.add_emit(
	    "&Items/Show/Kits\t",
		Shortcut::Alt | 'i',
		MenuFlag::Normal,
		*s,
		Message::ShowKits
	);
	menu.add_emit(
	    "&Items/Show/Medical Gear\t",
		Shortcut::Alt | 'h',
		MenuFlag::Normal,
		*s,
		Message::ShowMedical
	);
	menu.add_emit(
	    "&Items/Show/Keys\t",
		Shortcut::Alt | 'y',
		MenuFlag::Normal,
		*s,
		Message::ShowKeys
	);
	menu.add_emit(
	    "&Items/Show/LBE\t",
		Shortcut::Alt | 'b',
		MenuFlag::Normal,
		*s,
		Message::ShowLBE
	);
	menu.add_emit(
	    "&Items/Show/Misc\t",
		Shortcut::Alt | 'm',
		MenuFlag::Normal,
		*s,
		Message::ShowMisc
	);
	menu.add_emit(
	    "&Items/Show/Empty\\/None\t",
		Shortcut::Alt | 'v',
		MenuFlag::Normal,
		*s,
		Message::ShowNone
	);
	menu.add_emit(
	    "&Items/Show/Random Items\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowRandom
	);
	menu.add_emit(
	    "&Items/Merges/Standard\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowMerges
	);
	menu.add_emit(
	    "&Items/Merges/Attachment\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowAttachmentMerges
	);
	menu.add_emit(
	    "&Items/Attachments/Full List\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowAttachmentList
	);
	menu.add_emit(
	    "&Items/Attachments/Info\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowAttachmentInfo
	);
	menu.add_emit(
	    "&Items/Attachments/Incompatibilities\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowIncompatibleAttachments
	);
	menu.add_emit(
	    "&Items/Launchables\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowLaunchables
	);
	menu.add_emit(
	    "&Items/Compatible Face Items\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowCompatibleFaceGear
	);
	menu.add_emit(
	    "&Items/Transformations\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowTransforms
	);
	menu.add_emit(
	    "&Items/Random Items\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowRandomItems
	);
	
	return menu;
}



struct ItemGraphicsArea
{
	big: Frame,
	med: Frame,
	small: Frame,
	images: Vec<Frame>,
	scrollbar: Scrollbar,
	itemType: Choice,
	itemIndex: IntInput
}
impl ItemGraphicsArea
{
	fn initialize(x: i32, y: i32, s: &app::Sender<Message>, imagesSTI: &STI::Images) -> ItemGraphicsArea
	{
		let mainWidth = 300; let mainHeight = 450;

		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(60, 20).with_pos(x + 130, y - 10).with_label("Graphics").set_frame(FrameType::FlatBox);
		
		// Item images
		let bigw = 104; let bigh = 74;
		let medw = 74; let medh = 74;
		let smallw = 34; let smallh = 34;
	
		let mut big = Frame::default().with_size(bigw, bigh).with_pos(x + 10, y + 20);
		big.set_frame(FrameType::EngravedBox);
		let mut med = Frame::default().with_size(medw, medh).below_of(&big, 20);
		med.set_frame(FrameType::EngravedBox);
		let mut small = Frame::default().with_size(smallw, smallh).below_of(&med, 20);
		small.set_frame(FrameType::EngravedBox);
		
		let _ = Frame::default().with_size(20, 20).with_pos(x + 32, big.y() - 20).with_label("Big Image");
		let _ = Frame::default().with_size(20, 20).with_pos(x + 50, med.y() - 20).with_label("Inventory Image");
		let _ = Frame::default().with_size(20, 20).with_pos(x + 42, small.y() - 20).with_label("Ground Image");
		let _ = Frame::default().with_size(20, 20).with_pos(x + 42, small.y() + small.h() + 5).with_label("Graphic Type");
		let _ = Frame::default().with_size(20, 20).with_pos(x + 42, small.y() + small.h() + 50).with_label("Graphic Index");
		
		// Item graphic type & graphic index
		let mut itemType = Choice::default().with_pos(x + 10, small.y() + small.h() + 25).with_size(100, 20);
		itemType.emit(*s, Message::GraphicType);
		itemType.add_choice("Guns");
		for i in 1..imagesSTI.big.len()
		{
			let text = format!("P{}items", i);
			itemType.add_choice(&text);
		}

		let mut itemIndex = input::IntInput::default().with_size(100, 20).with_pos(x + 10, small.y() + small.h() + 70);
	
		// Item scroll area
		let mut scrollArea = Frame::default().with_size(150, 420).with_pos(x + 140, y + 20);
		scrollArea.set_frame(FrameType::EmbossedBox);
		scrollArea.set_color(Color::White);

		let mut images = Vec::new();
		let w = 104; let h = 54;
		let padding = 5;
		for i in 0..7
		{
			let mut image = Frame::default().with_size(w, h).with_pos(scrollArea.x() + 5, scrollArea.y() + 5 + (h+5)*i);
			image.set_frame(FrameType::BorderBox);
			image.set_color(Color::White);

			images.push(image);
		}
		
		let w = 20;
		let mut scrollbar = Scrollbar::default().with_pos(scrollArea.x() + scrollArea.w() - w, scrollArea.y()).with_size(w, scrollArea.h());
		scrollbar.emit(*s, Message::GraphicScroll);

		return ItemGraphicsArea{big, med, small, images, scrollbar, itemType, itemIndex};
	}

	
	fn updateScrollBarBounds(&mut self, sti: &STI::Images)
	{
		let mut i = self.itemType.value() as usize;
		if i >= sti.big.len()
		{
			println!("!!! In updateScrollBarBounds !!!");
    		println!("Tried to access nonexistent graphtype! images[{}]", i);
    		println!("Defaulting to guns");
			i = 0;
		}
		let max = sti.big[i].len() - self.images.len();

		self.scrollbar.set_maximum(max as f64);
		self.scrollbar.set_minimum(0.0);
		self.scrollbar.set_step(1.0, 1); // increment by 1.0 at each 1 step
    	self.scrollbar.set_value(0.0);
	}

	fn redrawScrollAreaImages(&mut self, sti: &STI::Images)
	{
		let w = self.images[0].w(); let h = self.images[0].h();
		let start = self.scrollbar.value() as usize;
		
		let mut graphType = self.itemType.value() as usize;
		if graphType >= sti.big.len()
		{
			println!("!!! In redrawScrollAreaImages !!!");
    		println!("Tried to access nonexistent graphtype! images[{}]", graphType);
    		println!("Defaulting to guns");
			graphType = 0;
		}

		for j in 0..7
		{
			let index = start + j;
			if index < sti.big[graphType].len()
			{
				let mut image = sti.big[graphType][index].clone();
				image.scale(w-4, h-4, true, true);
				
				self.images[j].set_image(Some(image));
			}
			else 
			{
				self.images[j].set_image(None::<RgbImage>);
			}
		}
	}

	fn updateItemGraphics(&mut self, images: &STI::Images, stiType: usize, stiIndex: usize)
	{
		let margin = 4;
		
		let mut image = images.big[stiType][stiIndex].clone();
		
		let width = self.big.w() - margin;
		let height = self.big.h() - margin;
		image.scale(width, height, true, true);
		self.big.set_image(Some(image));
		
		let mut image = images.med[stiType][stiIndex].clone();
		let width = self.med.w() - margin;
		let height = self.med.h() - margin;
		image.scale(width, height, true, true);
		self.med.set_image(Some(image));
		
		let mut image = images.small[stiType][stiIndex].clone();
		let width = self.small.w() - margin;
		let height = self.small.h() - margin;
		image.scale(width, height, true, true);
		self.small.set_image(Some(image));

	}
	
	fn addGraphTypeChoices(&mut self, images: &STI::Images)
	{
		self.itemType.clear();
		
		self.itemType.add_choice("Guns");
		for i in 1..images.big.len()
		{
			let text = format!("P{}items", i);
			self.itemType.add_choice(&text);
		}
	}
}


struct ItemStatsArea
{
	ints: Vec<Listener<IntInput>>,
	// price: IntInput,
	// weight: IntInput,
	// nperpocket: IntInput,
	// size: IntInput,
	// reliability: IntInput,
	// repairease: IntInput,
	cursor: Listener<Choice>,
}
impl ItemStatsArea
{
	fn initialize(x: i32, y: i32) -> ItemStatsArea
	{
		let mainWidth = 300; let mainHeight = 230;

		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(60, 20).with_pos(x + 130, y - 10).with_label("Stats").set_frame(FrameType::FlatBox);

		let xMargin = 5; let yMargin = 10;
		let w = mainWidth/2 - 2*xMargin; let h = mainHeight - 2*yMargin;

		// let mut flex = Flex::new(x + xMargin, y + yMargin, w, h, None);
		// flex.set_type(FlexType::Column);
		let mut flex = Pack::new(x + xMargin, y + yMargin, w, h, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(60, 20).with_label("Price");
		let _ = Frame::default().with_size(60, 20).with_label("Weight");
		let _ = Frame::default().with_size(60, 20).with_label("# per pocket");
		let _ = Frame::default().with_size(60, 20).with_label("Size");
		let _ = Frame::default().with_size(60, 20).with_label("Reliability");
		let _ = Frame::default().with_size(60, 20).with_label("Repair Ease");
		let _ = Frame::default().with_size(60, 20).with_label("Cursor");
		flex.end();

		let mut ints = Vec::new();

		let mut flex = Flex::default().with_pos(x + xMargin + w, y + yMargin).with_size(w, h);
		flex.set_type(FlexType::Column);
		// let mut flex = Pack::new(x + xMargin + w, y + yMargin, w, h, None);
		// flex.set_spacing(5);
		for i in 0..6
		{
			let mut input = IntInput::default();
			flex.set_size(&mut input, 20);
			ints.push(input.into());
		}
		let mut cursor = Choice::default();
		flex.set_size(&mut cursor, 20);
		flex.end();

		let cursor = cursor.into();

		return ItemStatsArea { ints, cursor }
	}
}

struct ItemDescriptionArea
{
	inputs: Vec<Listener<Input>>
}
impl ItemDescriptionArea
{
	fn initialize(x: i32, y: i32) -> ItemDescriptionArea
	{
		let mainWidth = 660; let mainHeight = 200;
		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(80, 20).with_pos(x + 130, y - 10).with_label("Description").set_frame(FrameType::FlatBox);

		let mut inputs: Vec<Listener<Input>> = Vec::new();
		let xOffset = 80;
		let h1 = 30; let h2 = 100;
		let w = 240;
		
		let mut flex = Pack::new(x + xOffset, y + 10, w, 180, None);
		flex.set_spacing(10);
		inputs.push(Input::default().with_size(0, h1).with_label("Name\n[80]").into());
		inputs.push(Input::default().with_size(0, h1).with_label("Long Name\n[80]").into());
		inputs.push(Input::default().with_size(0, h2).with_label("Description\n[400]").into());
		flex.end();
		inputs.last_mut().unwrap().set_wrap(true);


		let mut flex = Pack::new(flex.x()+flex.w() + 80, y + 10, w, 180, None);
		flex.set_spacing(10);
		let _ = Frame::default().with_size(0, h1).with_label("Bobby Ray's");
		inputs.push(Input::default().with_size(0, h1).with_label("Name\n[80]").into());
		inputs.push(Input::default().with_size(0, h2).with_label("Description\n[400]").into());
		inputs.last_mut().unwrap().set_wrap(true);
		flex.end();



		return ItemDescriptionArea { inputs };
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, index: usize)
	{
		if index < xmldata.items.items.len()
		{
			let item = &xmldata.items.items[index];
			self.inputs[0].set_value(&item.szItemName);
			self.inputs[1].set_value(&item.szLongItemName);
			self.inputs[2].set_value(&item.szItemDesc);
			self.inputs[3].set_value(&item.szBRName);
			self.inputs[4].set_value(&item.szBRDesc);

			let label = format!("Name\n[{}]", 80 - item.szItemName.len());
			self.inputs[0].set_label(&label);
			let label = format!("Long Name\n[{}]", 80 - item.szLongItemName.len());
			self.inputs[1].set_label(&label);
			let label = format!("Description\n[{}]", 400 - item.szItemDesc.len());
			self.inputs[2].set_label(&label);
			let label = format!("Name\n[{}]", 80 - item.szBRName.len());
			self.inputs[3].set_label(&label);
			let label = format!("Description\n[{}]", 400 - item.szBRDesc.len());
			self.inputs[4].set_label(&label);
		}
		else 
		{
			println!("!!! Out of bounds access!!! ITEMLIST [{}] ", index);
		}
	}
}


struct ItemPropertiesArea
{

}
impl ItemPropertiesArea
{
	fn initialize(x: i32, y: i32) -> ItemPropertiesArea
	{
		let mainWidth = 660; let mainHeight = 240;
		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(80, 20).with_pos(x + 130, y - 10).with_label("Properties").set_frame(FrameType::FlatBox);

		let xOffset = 10;
		let h1 = 20; let h2 = 100;
		let w = 165;
		
		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(w, h1).with_label("Show Status");
		let _ = CheckButton::default().with_size(w, h1).with_label("Damageable");
		let _ = CheckButton::default().with_size(w, h1).with_label("Repairable");
		let _ = CheckButton::default().with_size(w, h1).with_label("Damaged by water");
		let _ = CheckButton::default().with_size(w, h1).with_label("Sinks");
		let _ = CheckButton::default().with_size(w, h1).with_label("Unaerodynamic");
		let _ = CheckButton::default().with_size(w, h1).with_label("Electronic");
		let _ = CheckButton::default().with_size(w, h1).with_label("Metal");
		let _ = CheckButton::default().with_size(w, h1).with_label("Two-Handed");
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(w, h1).with_label("Tons of Guns");
		let _ = CheckButton::default().with_size(w, h1).with_label("Sci-Fi");
		let _ = CheckButton::default().with_size(w, h1).with_label("Nonbuyable");
		let _ = CheckButton::default().with_size(w, h1).with_label("Undroppable");
		let _ = CheckButton::default().with_size(w, h1).with_label("Not in editor");
		let _ = CheckButton::default().with_size(w, h1).with_label("New Inventory Only");
		let _ = CheckButton::default().with_size(w, h1).with_label("Tripwire");
		let _ = CheckButton::default().with_size(w, h1).with_label("Activated by tripwire");
		let _ = CheckButton::default().with_size(w, h1).with_label("Remote trigger");
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(w, h1).with_label("Contains Liquid");
		let _ = CheckButton::default().with_size(w, h1).with_label("Canteen");
		let _ = CheckButton::default().with_size(w, h1).with_label("Gas Can");
		let _ = CheckButton::default().with_size(w, h1).with_label("Alcohol");
		let _ = CheckButton::default().with_size(w, h1).with_label("Jar");
		let _ = CheckButton::default().with_size(w, h1).with_label("Medicine / Drug");
		let _ = CheckButton::default().with_size(w, h1).with_label("Gasmask");
		let _ = CheckButton::default().with_size(w, h1).with_label("Robot remote control");
		let _ = CheckButton::default().with_size(w, h1).with_label("Walkman");
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(w, h1).with_label("Rock");
		let _ = CheckButton::default().with_size(w, h1).with_label("Can and String");
		let _ = CheckButton::default().with_size(w, h1).with_label("Marbles");
		let _ = CheckButton::default().with_size(w, h1).with_label("Duckbill");
		let _ = CheckButton::default().with_size(w, h1).with_label("Wire Cutters");
		let _ = CheckButton::default().with_size(w, h1).with_label("X-Ray scanner");
		let _ = CheckButton::default().with_size(w, h1).with_label("Metal Detector");
		let _ = CheckButton::default().with_size(w, h1).with_label("Is Battery");
		let _ = CheckButton::default().with_size(w, h1).with_label("Needs batteries");
		flex.end();

		return ItemPropertiesArea {  };
	}
}


struct ItemKitArea
{

}
impl ItemKitArea
{
	fn initialize(x: i32, y: i32) -> ItemKitArea
	{
		let mainWidth = 235; let mainHeight = 230;
		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(60, 20).with_pos(x + 130, y - 10).with_label("Kits").set_frame(FrameType::FlatBox);

		let xOffset = 10;
		let h1 = 20; let h2 = 100;
		let w = 100;

		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(w, h1).with_label("Hardware");
		let _ = CheckButton::default().with_size(w, h1).with_label("Tool Kit");
		let _ = CheckButton::default().with_size(w, h1).with_label("Locksmith Kit");
		let _ = CheckButton::default().with_size(w, h1).with_label("Camouflage Kit");
		let _ = CheckButton::default().with_size(w, h1).with_label("Medical Kit");
		let _ = CheckButton::default().with_size(w, h1).with_label("First Aid Kit");
		// let _ = Frame::default().with_size(w, h1).with_label("Defusal Kit Bonus");
		// let _ = Frame::default().with_size(w, h1).with_label("Sleep modifier");
		// let _ = CheckButton::default().with_size(w, h1).with_label("");
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 10, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(w, h1);
		let _ = IntInput::default().with_size(w, h1);
		let _ = IntInput::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		let _ = IntInput::default().with_size(w, h1).with_label("Defusal Kit Bonus");
		let _ = IntInput::default().with_size(w, h1).with_label("Sleep Modifier");
		flex.end();


		return ItemKitArea {  };
	}
}

struct ItemVisionArea
{

}
impl ItemVisionArea
{
	fn initialize(x: i32, y: i32) -> ItemVisionArea
	{
		let mainWidth = 660-245; let mainHeight = 230;
		// Main framed box. Everything else is located relative to this
		let _ = Frame::default().with_size(mainWidth, mainHeight).with_pos(x, y).set_frame(FrameType::EngravedBox);
		let _ = Frame::default().with_size(150, 20).with_pos(x + 100, y - 10).with_label("Vision and Camouflage").set_frame(FrameType::FlatBox);

		let xOffset = 120;
		let h1 = 20; let h2 = 100;
		let w = 60;

		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = IntInput::default().with_size(w, h1).with_label("General");
		let _ = IntInput::default().with_size(w, h1).with_label("Nighttime");
		let _ = IntInput::default().with_size(w, h1).with_label("Daytime");
		let _ = IntInput::default().with_size(w, h1).with_label("Cave");
		let _ = IntInput::default().with_size(w, h1).with_label("Bright Light");
		let _ = IntInput::default().with_size(w, h1).with_label("Tunnelvision");
		let _ = IntInput::default().with_size(w, h1).with_label("Flashlight Range");
		let _ = IntInput::default().with_size(w, h1).with_label("Spotting Modifier");
		let _ = CheckButton::default().with_size(w, h1).with_label("Thermal Optics");
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 100, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = IntInput::default().with_size(w, h1).with_label("Woodland");
		let _ = IntInput::default().with_size(w, h1).with_label("Urban");
		let _ = IntInput::default().with_size(w, h1).with_label("Desert");
		let _ = IntInput::default().with_size(w, h1).with_label("Snow");
		let _ = IntInput::default().with_size(w, h1).with_label("Stealth");
		let _ = Choice::default().with_size(w, h1).with_label("Clothes Type");
		flex.end();

		return ItemVisionArea {  };
	}
}
//-----------------------------------------------------------------------------
// Enums
//-----------------------------------------------------------------------------
#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    New,
    Open,
    Save,
    SaveAs,
    Print,
    Quit,
    Cut,
    Copy,
    Paste,
    About,
    ShowAll,
    ShowGuns,
    ShowAmmo,
    ShowLaunchers,
    ShowGrenades,
    ShowExplosives,
    ShowKnives,
    ShowOther,
    ShowArmor,
    ShowFaceGear,
    ShowKits,
    ShowMedical,
    ShowKeys,
    ShowLBE,
    ShowMisc,
    ShowNone,
    ShowRandom,
    ShowMerges,
    ShowAttachmentMerges,
    ShowLaunchables,
    ShowCompatibleFaceGear,
    ShowTransforms,
    ShowRandomItems,
    ShowAttachmentList,
    ShowAttachmentInfo,
    ShowIncompatibleAttachments,
    ShowScifi,
    ShowNonScifi,
    ShowTonsOfGuns,
    ShowReducedGuns,
    ShowDrugs,
    ShowAttachments,
    Redraw,
    GraphicScroll,
	GraphicType,
}

    
