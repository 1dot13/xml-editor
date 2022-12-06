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
use fltk::enums::{Color, Align, Font};
use fltk::group::{Tabs, Group, FlexType, Pack};
use fltk::input::{IntInput, Input, FloatInput};
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
	
	let mut tab1 = Group::default().with_size(w, h).below_of(&tabs, 0).with_label("General\t\t");
	let x = 0;
	let y = 25;
	let mut itemGraphics = ItemGraphicsArea::initialize(x, y, &s, &images);
	let mut itemStats = ItemStatsArea::initialize(x, 485);
	let mut itemDescription = ItemDescriptionArea::initialize(310, y);
	let mut itemProperties = ItemPropertiesArea::initialize(310, y + 210);
	let mut itemKit = ItemKitArea::initialize(310, 485);
	let mut itemVision = ItemVisionArea::initialize(310+235+10, 485);
	tab1.end();


    let mut tab2 = Group::default().with_size(w, h).right_of(&tab1, 0).with_label("Weapon\t\t");
	let x = 0;
	let y = 25;
	let mut weaponArea = WeaponArea::initialize(x, y);
    tab2.end();


    let mut tab3 = Group::default().with_size(w, h).right_of(&tab2, 0).with_label("Tab3\t\t");
    let _but2 = RoundButton::default().with_size(0, 30).with_label("Round").center_of(&itemWindow);
    tab3.end();


    let mut tab4 = Group::default().with_size(w, h).right_of(&tab3, 0).with_label("Tab4\t\t");
    let _but3 = RoundButton::default().with_size(0, 30).with_label("Round2").center_of(&itemWindow);
    tab4.end();

	tab1.emit(s, Message::Tab1);
	tab2.emit(s, Message::Tab2);
	tab3.emit(s, Message::Tab3);
	tab4.emit(s, Message::Tab4);

    tabs.end();
	itemWindow.end();
 	
	mainWindow.end();
	// mainWindow.make_resizable(true);
	mainWindow.show();


	itemVision.addChoicesToClothesTypes(&xmldata);
	weaponArea.addChoices(&xmldata);
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
                let uiIndex = unsafe{item.user_data::<u32>()}.unwrap() as usize;
                println!("uiIndex {}", uiIndex);
                
				itemGraphics.update(&xmldata, &images, uiIndex);
				itemDescription.update(&xmldata, uiIndex);
				itemProperties.update(&xmldata, uiIndex);
				itemStats.update(&xmldata, uiIndex);
				itemKit.update(&xmldata, uiIndex);
				itemVision.update(&xmldata, uiIndex);

				weaponArea.update(&xmldata, uiIndex);

				itemWindow.redraw()
			}
			else 
			{
				itemGraphics.clearImages();
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
				Tab1  | Tab2 | Tab3 | Tab4 => { switchTab(&xmldata, msg); }
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

fn switchTab(xmldata: &JAxml::JAxmlState, msg: Message)
{
	match msg
	{
		Message::Tab1 => 
		{

		}	
		Message::Tab2 => 
		{

		}
		Message::Tab3 =>
		{

		}
		Message::Tab4 =>
		{

		}
		_ => ()
	}
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
	itemIndex: IntInput,
	itemClass: Choice,
	uiIndex: IntInput
}
impl ItemGraphicsArea
{
	fn initialize(x: i32, y: i32, s: &app::Sender<Message>, imagesSTI: &STI::Images) -> ItemGraphicsArea
	{
		let mainWidth = 300; let mainHeight = 450;

		// Main framed box. Everything else is located relative to this
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			130, 60, "Graphics"
		);
		
		//-------------------------------------------------
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
		
		//-------------------------------------------------
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

		//-------------------------------------------------
		// Item class & uiIndex
		let (frame2, _) = createBox(
			x + 5, small.y() + small.h() + 100,
			120, 80,
			12, 80, "Item Class"
		);

		let _ = Frame::default().with_size(20, 20).with_pos(x + 42, small.y() + small.h() + 130).with_label("Item Index");

		let mut itemClass = Choice::default().with_pos(x + 10, small.y() + small.h() + 110).with_size(100, 20);
		itemClass.emit(*s, Message::ItemClass);

		let classes = vec![
			"None",
			"Gun",
			"Blade",
			"ThrowingKnife",
			"Launcher",
			"Tentacle",
			"Thrown",
			"Punch",
			"Grenade",
			"Bomb",
			"Ammo",
			"Armor",
			"Medkit",
			"Kit",
			"Appliable",
			"Face",
			"Key",
			"LBE",
			"BeltClip",
			"Misc",
			"Money",
			"Random",
		];

		for class in classes
		{
			itemClass.add_choice(class);
		}

		let mut uiIndex = input::IntInput::default().with_size(100, 20).with_pos(x + 10, small.y() + small.h() + 150);

		//-------------------------------------------------
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

		return ItemGraphicsArea{big, med, small, images, scrollbar, itemType, itemIndex, uiIndex, itemClass};
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

	fn update(&mut self, xmldata: &JAxml::JAxmlState, images: &STI::Images, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];

		let stiType = item.ubGraphicType as usize;
		let stiIndex = item.ubGraphicNum as usize;
		println!("Graphic Type {}", stiType);
		println!("Graphic index {}", stiIndex);

		if stiType < images.big.len() && stiIndex < images.big[stiType].len()
		{
			self.updateItemGraphics(&images, stiType, stiIndex);

			if stiType != self.itemType.value() as usize
			{
				self.itemType.set_value(stiType as i32);
				self.updateScrollBarBounds(&images);
				self.redrawScrollAreaImages(&images);
			}
			self.itemIndex.set_value(&format!("{}", stiIndex));
			
		}
		else 
		{
			println!("Graphic index out of graphic vector bounds!");
			println!("Tried to access image [{}][{}]", stiType, stiIndex);
		}

		self.uiIndex.set_value(&format!("{}", uiIndex));
		self.updateItemClass(item.usItemClass as usize);
	}

	fn updateItemClass(&mut self, itemClass: usize)
	{
		match itemClass.try_into()
		{
			Ok(JAxml::ItemClass::None) => { self.itemClass.set_value(0); }
			Ok(JAxml::ItemClass::Gun) => { self.itemClass.set_value(1); }
			Ok(JAxml::ItemClass::Blade) => { self.itemClass.set_value(2); }
			Ok(JAxml::ItemClass::ThrowingKnife) => { self.itemClass.set_value(3); }
			Ok(JAxml::ItemClass::Launcher) => { self.itemClass.set_value(4); }
			Ok(JAxml::ItemClass::Tentacle) => { self.itemClass.set_value(5); }
			Ok(JAxml::ItemClass::Thrown) => { self.itemClass.set_value(6); }
			Ok(JAxml::ItemClass::Punch) => { self.itemClass.set_value(7); }
			Ok(JAxml::ItemClass::Grenade) => { self.itemClass.set_value(8); }
			Ok(JAxml::ItemClass::Bomb) => { self.itemClass.set_value(9); }
			Ok(JAxml::ItemClass::Ammo) => { self.itemClass.set_value(10); }
			Ok(JAxml::ItemClass::Armor) => { self.itemClass.set_value(11); }
			Ok(JAxml::ItemClass::Medkit) => { self.itemClass.set_value(12); }
			Ok(JAxml::ItemClass::Kit) => { self.itemClass.set_value(13); }
			Ok(JAxml::ItemClass::Appliable) => { self.itemClass.set_value(14); }
			Ok(JAxml::ItemClass::Face) => { self.itemClass.set_value(15); }
			Ok(JAxml::ItemClass::Key) => { self.itemClass.set_value(16); }
			Ok(JAxml::ItemClass::LBE) => { self.itemClass.set_value(17); }
			Ok(JAxml::ItemClass::BeltClip) => { self.itemClass.set_value(18); }
			Ok(JAxml::ItemClass::Misc) => { self.itemClass.set_value(19); }
			Ok(JAxml::ItemClass::Money) => { self.itemClass.set_value(20); }
			Ok(JAxml::ItemClass::Random) => { self.itemClass.set_value(21); }
			_ => { println!("!!! UNKNOWN ITEM CLASS !!!"); self.itemClass.set_value(-1); }
		}
	}

	fn clearImages(&mut self)
	{
		self.big.set_image(None::<RgbImage>);
		self.med.set_image(None::<RgbImage>);
		self.small.set_image(None::<RgbImage>);
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
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			130, 60, "Stats"
		);

		let xMargin = 5; let yMargin = 10;
		let w = mainWidth/2 - 2*xMargin; let h = mainHeight - 2*yMargin;

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
		for i in 0..6
		{
			let mut input = IntInput::default();
			flex.set_size(&mut input, 20);
			ints.push(input.into());
		}
		let mut cursor = Choice::default();
		flex.set_size(&mut cursor, 20);
		flex.end();

		// Cursor choices. Must match with Jaxml::enum::Cursor
		cursor.add_choice("Invalid");
		cursor.add_choice("Quest");
		cursor.add_choice("Punch");
		cursor.add_choice("Target");
		cursor.add_choice("Knife");
		cursor.add_choice("Aid");
		cursor.add_choice("Throw");
		cursor.add_choice("Mine");
		cursor.add_choice("Lockpick");
		cursor.add_choice("MineDetector");
		cursor.add_choice("Crowbar");
		cursor.add_choice("CCTV");
		cursor.add_choice("Camera");
		cursor.add_choice("Key");
		cursor.add_choice("Saw");
		cursor.add_choice("WireCutters");
		cursor.add_choice("Remote");
		cursor.add_choice("Bomb");
		cursor.add_choice("Repair");
		cursor.add_choice("Trajectory");
		cursor.add_choice("Jar");
		cursor.add_choice("Tincan");
		cursor.add_choice("Refuel");
		cursor.add_choice("Fortification");
		cursor.add_choice("Handcuffs");
		cursor.add_choice("ApplyItem");
		cursor.add_choice("InteractiveAction");
		cursor.add_choice("Bloodbag");
		cursor.add_choice("Splint");

		let cursor = cursor.into();

		return ItemStatsArea { ints, cursor }
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		self.ints[0].set_value(&format!("{}", item.usPrice));
		self.ints[1].set_value(&format!("{}", item.ubWeight));
		self.ints[2].set_value(&format!("{}", item.ubPerPocket));
		self.ints[3].set_value(&format!("{}", item.ItemSize));
		self.ints[4].set_value(&format!("{}", item.bReliability));
		self.ints[5].set_value(&format!("{}", item.bRepairEase));

		self.cursor.set_value(item.ubCursor as i32);
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
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			130, 80, "Description"
		);

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

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		if uiIndex < xmldata.items.items.len()
		{
			let item = &xmldata.items.items[uiIndex];
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
			println!("!!! Out of bounds access!!! ITEMLIST [{}] ", uiIndex);
		}
	}
}


struct ItemPropertiesArea
{
	inputs: Vec<Listener<CheckButton>>
}
impl ItemPropertiesArea
{
	fn initialize(x: i32, y: i32) -> ItemPropertiesArea
	{
		let mainWidth = 660; let mainHeight = 240;
		// Main framed box. Everything else is located relative to this
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			130, 80, "Properties"
		);

		let xOffset = 10;
		let h1 = 20; let h2 = 100;
		let w = 165;
		let mut inputs = Vec::new();

		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Show Status").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Damageable").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Repairable").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Damaged by water").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Sinks").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Unaerodynamic").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Electronic").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Metal").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Two-Handed").into());
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Tons of Guns").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Sci-Fi").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Nonbuyable").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Undroppable").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Not in editor").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("New Inventory Only").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Tripwire").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Activated by tripwire").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Remote trigger").into());
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Contains Liquid").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Canteen").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Gas Can").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Alcohol").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Jar").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Medicine / Drug").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Gasmask").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Robot remote control").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Walkman").into());
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w(), y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Rock").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Can and String").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Marbles").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Duckbill").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Wire Cutters").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("X-Ray scanner").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Metal Detector").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Is Battery").into());
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Needs batteries").into());
		flex.end();

		return ItemPropertiesArea { inputs };
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];

		self.inputs[0].set_value(item.showstatus);
		self.inputs[1].set_value(item.Damageable);
		self.inputs[2].set_value(item.Repairable);
		self.inputs[3].set_value(item.WaterDamages);
		self.inputs[4].set_value(item.Sinks);
		self.inputs[5].set_value(item.unaerodynamic);
		self.inputs[6].set_value(item.electronic);
		self.inputs[7].set_value(item.Metal);
		self.inputs[8].set_value(item.twohanded);

		self.inputs[9].set_value(item.biggunlist);
		self.inputs[10].set_value(item.scifi);
		self.inputs[11].set_value(item.notbuyable);
		self.inputs[12].set_value(item.defaultundroppable);
		self.inputs[13].set_value(item.notineditor);
		self.inputs[14].set_value(item.newinv);
		self.inputs[15].set_value(item.tripwire);
		self.inputs[16].set_value(item.tripwireactivation);
		self.inputs[17].set_value(item.remotetrigger);

		self.inputs[18].set_value(item.containsliquid);
		self.inputs[19].set_value(item.canteen);
		self.inputs[20].set_value(item.gascan);
		self.inputs[21].set_value(item.alcohol != 0.0);
		self.inputs[22].set_value(item.jar);
		self.inputs[23].set_value(item.medical);
		self.inputs[24].set_value(item.gasmask);
		self.inputs[25].set_value(item.robotremotecontrol);
		self.inputs[26].set_value(item.walkman);

		self.inputs[27].set_value(item.rock);
		self.inputs[28].set_value(item.canandstring);
		self.inputs[29].set_value(item.marbles);
		self.inputs[30].set_value(item.duckbill);
		self.inputs[31].set_value(item.wirecutters);
		self.inputs[32].set_value(item.xray);
		self.inputs[33].set_value(item.metaldetector);
		self.inputs[34].set_value(item.batteries);
		self.inputs[35].set_value(item.needsbatteries);

	}
}


struct ItemKitArea
{
	inputs: Vec<Listener<CheckButton>>,
	ints: Vec<Listener<IntInput>>
}
impl ItemKitArea
{
	fn initialize(x: i32, y: i32) -> ItemKitArea
	{
		let mainWidth = 235; let mainHeight = 230;
		// Main framed box. Everything else is located relative to this
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			130, 60, "Kits"
		);

		let xOffset = 10;
		let h1 = 20; let h2 = 100;
		let w = 100;

		let mut inputs = Vec::new();
		let mut ints = Vec::new();

		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		inputs.push( CheckButton::default().with_size(w, h1).with_label("Hardware").into() );
		inputs.push( CheckButton::default().with_size(w, h1).with_label("Tool Kit").into() );
		inputs.push( CheckButton::default().with_size(w, h1).with_label("Locksmith Kit").into() );
		inputs.push( CheckButton::default().with_size(w, h1).with_label("Camouflage Kit").into() );
		inputs.push( CheckButton::default().with_size(w, h1).with_label("Medical Kit").into() );
		inputs.push( CheckButton::default().with_size(w, h1).with_label("First Aid Kit").into() );
		// let _ = Frame::default().with_size(w, h1).with_label("Defusal Kit Bonus");
		// let _ = Frame::default().with_size(w, h1).with_label("Sleep modifier");
		// let _ = CheckButton::default().with_size(w, h1).with_label("");
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 10, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(w, h1);
		ints.push( IntInput::default().with_size(w, h1).into() );
		ints.push( IntInput::default().with_size(w, h1).into() );
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		ints.push( IntInput::default().with_size(w, h1).with_label("Defusal Kit Bonus").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Sleep Modifier").into() );
		flex.end();


		return ItemKitArea { inputs, ints };
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];

		self.inputs[0].set_value(item.hardware);
		self.inputs[1].set_value(item.toolkit);
		self.inputs[2].set_value(item.locksmithkit);
		self.inputs[3].set_value(item.camouflagekit);
		self.inputs[4].set_value(item.medicalkit);
		self.inputs[5].set_value(item.firstaidkit);

		self.ints[0].set_value(&format!("{}", item.RepairModifier));
		self.ints[1].set_value(&format!("{}", item.LockPickModifier));
		self.ints[2].set_value(&format!("{}", item.DisarmModifier));
		self.ints[3].set_value(&format!("{}", item.ubSleepModifier));
	}
}

struct ItemVisionArea
{
	ints: Vec<Listener<IntInput>>,
	thermal: Listener<CheckButton>,
	clothesType: Listener<Choice>
}
impl ItemVisionArea
{
	fn initialize(x: i32, y: i32) -> ItemVisionArea
	{
		let mainWidth = 660-245; let mainHeight = 230;
		// Main framed box. Everything else is located relative to this
		let (_, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			100, 150, "Vision and Camouflage"
		);

		let xOffset = 120;
		let h1 = 20; let h2 = 100;
		let w = 60;

		let mut ints = Vec::new();

		let mut flex = Pack::new(x + xOffset, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		ints.push( IntInput::default().with_size(w, h1).with_label("General").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Nighttime").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Daytime").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Cave").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Bright Light").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Tunnelvision").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Flashlight Range").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Spotting Modifier").into() );
		let thermal = CheckButton::default().with_size(w, h1).with_label("Thermal Optics").with_align(Align::Left).into();
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 100, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		ints.push( IntInput::default().with_size(w, h1).with_label("Woodland").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Urban").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Desert").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Snow").into() );
		ints.push( IntInput::default().with_size(w, h1).with_label("Stealth").into() );
		let clothesType = Choice::default().with_size(w, h1).with_label("Clothes Type").into();
		flex.end();

		return ItemVisionArea { ints, thermal, clothesType };
	}

	fn addChoicesToClothesTypes(&mut self, xmldata: &JAxml::JAxmlState)
	{
		self.clothesType.clear();
		for cloth in &xmldata.clothes.items
		{
			self.clothesType.add_choice(&format!("{}", cloth.szName));
		}
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];

		self.ints[0].set_value(&format!("{}", item.visionrangebonus));
		self.ints[1].set_value(&format!("{}", item.nightvisionrangebonus));
		self.ints[2].set_value(&format!("{}", item.dayvisionrangebonus));
		self.ints[3].set_value(&format!("{}", item.cavevisionrangebonus));
		self.ints[4].set_value(&format!("{}", item.brightlightvisionrangebonus));
		self.ints[5].set_value(&format!("{}", item.percenttunnelvision));
		self.ints[6].set_value(&format!("{}", item.usFlashLightRange));
		self.ints[7].set_value(&format!("{}", item.usSpotting));

		self.ints[8].set_value(&format!("{}", item.camobonus));
		self.ints[9].set_value(&format!("{}", item.urbanCamobonus));
		self.ints[10].set_value(&format!("{}", item.desertCamobonus));
		self.ints[11].set_value(&format!("{}", item.snowCamobonus));
		self.ints[12].set_value(&format!("{}", item.stealthbonus));
		
		self.thermal.set_value(item.thermaloptics);
		self.clothesType.set_value(item.clothestype as i32);
	}
}


struct WeaponAreaGeneral
{
	class: Listener<Choice>,
	guntype: Listener<Choice>,
	caliber: Listener<Choice>,
	magsize: Listener<IntInput>
}

struct WeaponAreaStats
{
	range: Listener<IntInput>,
	accuracy: Listener<IntInput>,
	damage: Listener<IntInput>,
	deadliness: Listener<IntInput>,
	messydeath: Listener<IntInput>,
	meleeDamage: Listener<IntInput>,
	crowbarBonus: Listener<IntInput>,
	autofirespeed: Listener<IntInput>,
	autofirepenalty: Listener<IntInput>,
	burstshots: Listener<IntInput>,
	burstpenalty: Listener<IntInput>,
	burstAPcost: Listener<IntInput>,
	reloadAP: Listener<IntInput>,
	manualreloadAP: Listener<IntInput>,
	readyAP: Listener<IntInput>,
	shotsper4turns: Listener<IntInput>,
	brRateOfFire: Listener<IntInput>,
	reloadAnimDelay: Listener<IntInput>,
	burstfireAnimDelay: Listener<IntInput>,
	bulletspeed: Listener<IntInput>,
}
struct WeaponAreaProperties
{
	crowbar: Listener<CheckButton>,
	brassknuckles: Listener<CheckButton>,
	fullauto: Listener<CheckButton>,
	rocketrifle: Listener<CheckButton>,
	fingerprintid: Listener<CheckButton>,
	easyunjam: Listener<CheckButton>,
	heavyweapon: Listener<CheckButton>,
	hidemuzzleflash: Listener<CheckButton>,
	barrel: Listener<CheckButton>,
}
struct WeaponAreaNCTH
{
	flatbase: Vec<Listener<IntInput>>,
	flataim: Vec<Listener<IntInput>>,
	base: Vec<Listener<IntInput>>,
	cap: Vec<Listener<IntInput>>,
	handling: Vec<Listener<IntInput>>,
	tracking: Vec<Listener<IntInput>>,
	dropCompensation: Vec<Listener<IntInput>>,
	maxCounterforce: Vec<Listener<IntInput>>,
	CFaccuracy: Vec<Listener<IntInput>>,
	CFfrequency: Vec<Listener<IntInput>>,
	aimlevel: Vec<Listener<IntInput>>,
	// Items.xml
	scopeMagFactor: Listener<FloatInput>,
	laserProjFactor: Listener<IntInput>,
	recoilXmodifier: Listener<FloatInput>,
	recoilYmodifier: Listener<FloatInput>,
	recoilModifier: Listener<IntInput>,
	accuracyModifier: Listener<IntInput>,
	// Weapons.xml
	NCTHaccuracy: Listener<IntInput>,
	recoilX: Listener<FloatInput>,
	recoilY: Listener<FloatInput>,
	recoilDelay: Listener<IntInput>,
	defaultAimLevels: Listener<IntInput>,
	weaponHandling: Listener<IntInput>,
}
struct WeaponAreaTemperature
{
	jamThreshold: Listener<FloatInput>,
	dmgThreshold: Listener<FloatInput>,
	increasePerShot: Listener<FloatInput>,
	cooldownFactor: Listener<FloatInput>,
	cooldownModifier: Listener<FloatInput>,
	tempModifier: Listener<FloatInput>,
	jamThresholdModifier: Listener<FloatInput>,
	damageThresholdModifier: Listener<FloatInput>,
}
struct WeaponAreaModifiers
{
	// ranged
	damage: Listener<IntInput>,
	range: Listener<IntInput>,
	magSize: Listener<IntInput>,
	burstSize: Listener<IntInput>,
	shotsper4turns: Listener<IntInput>,
	bulletspeed: Listener<IntInput>,
	noiseReduction: Listener<IntInput>,
	// to hit
	general: Listener<IntInput>,
	aimedShot: Listener<IntInput>,
	bipodProne: Listener<IntInput>,
	burst: Listener<IntInput>,
	autofire: Listener<IntInput>,
	laserRange: Listener<IntInput>,
	minRange: Listener<IntInput>,
	// AP reductions
	generalAP: Listener<IntInput>,
	readyAP: Listener<IntInput>,
	reloadAP: Listener<IntInput>,
	burstAP: Listener<IntInput>,
	autofireAP: Listener<IntInput>,
	// bonuses
	bonusAP: Listener<IntInput>,
	bonusHearing: Listener<IntInput>,
	bonusKitStatus: Listener<IntInput>,
	bonusSize: Listener<IntInput>,
}
struct WeaponArea
{
	general: WeaponAreaGeneral,
	stats: WeaponAreaStats,
	properties: WeaponAreaProperties,
	ncth: WeaponAreaNCTH,
	temp: WeaponAreaTemperature,
	modifiers: WeaponAreaModifiers,
	dirtDamageChance: Listener<IntInput>,
	dirtIncreaseFactor: Listener<FloatInput>,
	bloodyItem: Listener<Choice>
}
impl WeaponArea
{
	fn initialize(x: i32, y: i32) -> WeaponArea
	{
		let mainWidth = 300; let mainHeight = 700;
		// Main framed box. Everything else is located relative to this
		let (main, _) = createBox(
			x, y,
			mainWidth, mainHeight - 10,
			110, 80, "General"
		);

		//-------------------------------------------------
		// General
		let width = 150; let height = 20;
		let mut flex = Pack::new(main.x() + 40, main.y() + 10, 120, main.h() - 10, None);
		flex.set_spacing(5);
		let class = Choice::default().with_size(width, height).with_label("Class").into();
		let guntype = Choice::default().with_size(width, height).with_label("Type").into();
		flex.end();
		
		let mut flex = Pack::new(main.x() + main.w() - 80, main.y() + 10, 70, main.h() - 10, None);
		flex.set_spacing(5);
		let caliber = Choice::default().with_size(width, height).with_label("Caliber").into();
		let magsize = IntInput::default().with_size(width, height).with_label("Capacity").into();
		flex.end();


		let w = 100;
		//-------------------------------------------------
		// Range & Damage
		let (frame, _) = createBox(
			main.x(), y + 70,
			mainWidth, 160,
			(mainWidth-w)/2, 50, "Stats"
		);

		let mut flex = Pack::new(frame.x() + 70, frame.y() + 10, 70, frame.h() - 10, None);
		flex.set_spacing(5);
		let range = IntInput::default().with_size(width, height).with_label("Range").into();
		let accuracy = IntInput::default().with_size(width, height).with_label("Accuracy").into();
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 80, frame.y() + 10, 70, frame.h() - 10, None);
		flex.set_spacing(5);
		let damage = IntInput::default().with_size(width, height).with_label("Damage").into();
		let deadliness = IntInput::default().with_size(width, height).with_label("Deadliness").into();
		let messydeath =  IntInput::default().with_size(width, height).with_label("Messy Death Dist.").into();
		flex.end();

		//-------------------------------------------------
		// Melee weapons
		let (mut frame2, _) = createBox(
			frame.x() + 5,
			frame.y() + frame.h() - 65,
			frame.w()-10, 55,
			20, 120, "Melee Weapons"
		);
		frame2.set_frame(FrameType::BorderBox);

		let mut flex = Pack::new(frame2.x() + 5, frame2.y() + 5, 70, frame2.h() - 10, None);
		flex.set_spacing(5);
		let brassknuckles = CheckButton::default().with_size(width, height).with_label("Brass Knuckles").into();
		let crowbar = CheckButton::default().with_size(width, height).with_label("Crowbar").into();
		flex.end();
		let mut flex = Pack::new(frame2.x() + frame2.w() - 80, frame2.y() + 5, 70, frame2.h() - 10, None);
		flex.set_spacing(5);
		let meleeDamage = IntInput::default().with_size(width, height).with_label("Dmg bonus").into();
		let crowbarBonus = IntInput::default().with_size(width, height).with_label("Crowbar bonus").into();
		flex.end();


		//-------------------------------------------------
		// Auto / Burst Fire
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h(),
			frame.w(), 95,
			(frame.w()-w)/2, 120, "Auto / Burst Fire"
		);

		let mut flex = Pack::new(frame.x() + 100, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let autofirespeed = IntInput::default().with_size(width, height).with_label("Shots / 5 APs").into();
		let autofirepenalty = IntInput::default().with_size(width, height).with_label("To-Hit Penalty").into();
		let fullauto = CheckButton::default().with_size(width, height).with_label("Full Auto only").with_align(Align::Left).into();
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 55, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let burstshots = IntInput::default().with_size(width, height).with_label("Shots / Burst").into();
		let burstpenalty = IntInput::default().with_size(width, height).with_label("To-Hit Penalty").into();
		let burstAPcost = IntInput::default().with_size(width, height).with_label("AP Cost").into();
		flex.end();


		//-------------------------------------------------
		// AP Costs
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h(),
			frame.w(), 95,
			(frame.w()-w)/2, 100, "AP Costs"
		);

		let mut flex = Pack::new(frame.x() + 100, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let reloadAP = IntInput::default().with_size(width, height).with_label("Reload").into();
		let manualreloadAP = IntInput::default().with_size(width, height).with_label("Manual Reload").into();
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 55, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let readyAP = IntInput::default().with_size(width, height).with_label("Ready Weapon").into();
		let shotsper4turns = IntInput::default().with_size(width, height).with_label("Shots / 4 turns").into();
		let brRateOfFire = IntInput::default().with_size(width, height).with_label("BR ROF").into();
		flex.end();


		//-------------------------------------------------
		// Animation
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h(),
			frame.w(), 100,
			(frame.w()-w)/2, 100, "Animation"
		);

		let reloadAnimDelay = IntInput::new(x + mainWidth - width - 10, frame.y() + 10, width, height, "Reload Delay").into();
		let burstfireAnimDelay = IntInput::new(x + mainWidth - width - 10, frame.y() + 40, width, height, "Burst Fire Delay").into();
		let bulletspeed = IntInput::new(x + mainWidth - width - 10, frame.y() + 70, width, height, "Bullet Speed").into();


		//-------------------------------------------------
		// Properties
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h(),
			frame.w(), 170,
			(frame.w()-w)/2, 100, "Properties"
		);

		let mut flex = Pack::new(frame.x() + 10, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let rocketrifle = CheckButton::default().with_size(width, height).with_label("Rocket Rifle").into();
		let fingerprintid = CheckButton::default().with_size(width, height).with_label("Fingerprint ID").into();
		let easyunjam = CheckButton::default().with_size(width, height).with_label("Easy Unjam").into();
		let heavyweapon = CheckButton::default().with_size(width, height).with_label("Heavy Weapon").into();
		let hidemuzzleflash = CheckButton::default().with_size(width, height).with_label("Hide Muzzleflash").into();
		let barrel = CheckButton::default().with_size(width, height).with_label("Barrel").into();
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w() + 80, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w() + 50, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("");//.into();
		flex.end();



		//-------------------------------------------------
		// NCTH
		let mainWidth = 665; let mainHeight = 350;
		let (main, _) = createBox(
			980 - mainWidth - 10,
			y,
			mainWidth, mainHeight,
			100, 60, "NCTH"
		);

		let mut flatbase: Vec<Listener<IntInput>> = Vec::new();
		let mut flataim: Vec<Listener<IntInput>> = Vec::new();
		let mut base: Vec<Listener<IntInput>> = Vec::new();
		let mut cap: Vec<Listener<IntInput>> = Vec::new();
		let mut handling: Vec<Listener<IntInput>> = Vec::new();
		let mut tracking: Vec<Listener<IntInput>> = Vec::new();
		let mut dropCompensation: Vec<Listener<IntInput>> = Vec::new();
		let mut maxCounterforce: Vec<Listener<IntInput>> = Vec::new();
		let mut CFaccuracy: Vec<Listener<IntInput>> = Vec::new();
		let mut CFfrequency: Vec<Listener<IntInput>> = Vec::new();
		let mut aimlevel: Vec<Listener<IntInput>> = Vec::new();
	

		let width = 75; let height = 20;
		let mut flex = Pack::new(main.x() + 150, main.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Standing");
		flatbase.push( IntInput::default().with_size(width, height).with_label("Flat Base").into() );
		flataim.push( IntInput::default().with_size(width, height).with_label("Flat Aim").into() );
		base.push( IntInput::default().with_size(width, height).with_label("Base %").into() );
		cap.push( IntInput::default().with_size(width, height).with_label("Cap %").into() );
		handling.push( IntInput::default().with_size(width, height).with_label("Handling % ").into() );
		tracking.push( IntInput::default().with_size(width, height).with_label("Tracking %").into() );
		dropCompensation.push( IntInput::default().with_size(width, height).with_label("Drop Compensation %").into() );
		maxCounterforce.push( IntInput::default().with_size(width, height).with_label("Max Counterforce %").into() );
		CFaccuracy.push( IntInput::default().with_size(width, height).with_label("CF Accuracy %").into() );
		CFfrequency.push( IntInput::default().with_size(width, height).with_label("CF Frequency %").into() );
		aimlevel.push( IntInput::default().with_size(width, height).with_label("Aimlevel Modifier").into() );
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w(), flex.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Crouching");
		flatbase.push( IntInput::default().with_size(width, height).into() );
		flataim.push( IntInput::default().with_size(width, height).into() );
		base.push( IntInput::default().with_size(width, height).into() );
		cap.push( IntInput::default().with_size(width, height).into() );
		handling.push( IntInput::default().with_size(width, height).into() );
		tracking.push( IntInput::default().with_size(width, height).into() );
		dropCompensation.push( IntInput::default().with_size(width, height).into() );
		maxCounterforce.push( IntInput::default().with_size(width, height).into() );
		CFaccuracy.push( IntInput::default().with_size(width, height).into() );
		CFfrequency.push( IntInput::default().with_size(width, height).into() );
		aimlevel.push( IntInput::default().with_size(width, height).into() );
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w(), flex.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Prone");
		flatbase.push( IntInput::default().with_size(width, height).into() );
		flataim.push( IntInput::default().with_size(width, height).into() );
		base.push( IntInput::default().with_size(width, height).into() );
		cap.push( IntInput::default().with_size(width, height).into() );
		handling.push( IntInput::default().with_size(width, height).into() );
		tracking.push( IntInput::default().with_size(width, height).into() );
		dropCompensation.push( IntInput::default().with_size(width, height).into() );
		maxCounterforce.push( IntInput::default().with_size(width, height).into() );
		CFaccuracy.push( IntInput::default().with_size(width, height).into() );
		CFfrequency.push( IntInput::default().with_size(width, height).into() );
		aimlevel.push( IntInput::default().with_size(width, height).into() );
		flex.end();


		//-------------------------------------------------
		// NCTH Items.xml
		let (frame, _) = createBox(
			main.x()+420,
			main.y()+12,
			240, 160,
			(240-w)/2, w, "Items.xml"
		);

		let width = 75; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let scopeMagFactor: Listener<FloatInput> = ( FloatInput::default().with_size(width, height).with_label("Scope Mag Factor").into() );
		let laserProjFactor: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Laser Proj. Factor").into() );
		let recoilXmodifier: Listener<FloatInput> = ( FloatInput::default().with_size(width, height).with_label("Recoil X Modifier").into() );
		let recoilYmodifier: Listener<FloatInput> = ( FloatInput::default().with_size(width, height).with_label("Recoil Y Modifier").into() );
		let recoilModifier: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Recoil Modifier %").into() );
		let accuracyModifier: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Accuracy Modifier %").into() );
		flex.end();


		//-------------------------------------------------
		// NCTH Weapons.xml
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h()+10,
			frame.w(), 160,
			(240-w)/2, w, "Weapons.xml"
		);

		let width = 75; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let NCTHaccuracy: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("NCTH Accuracy").into() );
		let recoilX: Listener<FloatInput> = ( FloatInput::default().with_size(width, height).with_label("Recoil X").into() );
		let recoilY: Listener<FloatInput> = ( FloatInput::default().with_size(width, height).with_label("Recoil Y").into() );
		let recoilDelay: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Recoil Delay").into() );
		let defaultAimLevels: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Default Aim Levels").into() );
		let weaponHandling: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Weapon Handling").into() );
		flex.end();


		//-------------------------------------------------
		// Modifiers
		let (frame, _) = createBox(
			main.x() + main.w() - mainWidth/2-15,
			main.y() + main.h(),
			mainWidth/2+15, 350, 
			(mainWidth/2+15-w)/2, 100, "Modifiers"
		);

		let (frame, _) = createBox(
			frame.x() + 5,
			frame.y() + 25,
			165, 185,
			30, 120, "Ranged Weapons"
		);

		let width = 45; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let modifierdamage: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Damage").into() );
		let modifierrange: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Range").into() );
		let modifiermagSize: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Mag Size").into() );
		let modifierburstSize: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Burst Size").into() );
		let modifiershotsper4turns: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Shots / 4 turns").into() );
		let modifierbulletspeed: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Bullet Speed").into() );
		let modifiernoiseReduction: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Noise Reduction").into() );
		flex.end();


		let (frame, _) = createBox(
			frame.x() + frame.w() + 5,
			frame.y(),
			165, 185,
			30, 60, "To-Hit"
		);

		let width = 45; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let modifiergeneral: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("General %").into() );
		let modifieraimedShot: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Aimed Shot %").into() );
		let modifierbipodProne: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Bipod/Prone %").into() );
		let modifierburst: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Burst %").into() );
		let modifierautofire: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Autofire %").into() );
		let modifierlaserRange: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Laser Range").into() );
		let modifierminRange: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Min. Range").into() );
		flex.end();


		let (frame, _) = createBox(
			frame.x(),
			frame.y() + frame.h() + 5,
			165, 185,
			30, 120, "AP Reductions"
		);

		let width = 45; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let modifiergeneralAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("General %").into() );
		let modifierreadyAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Ready %").into() );
		let modifierreloadAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Reload %").into() );
		let modifierburstAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Burst %").into() );
		let modifierautofireAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Autofire %").into() );
		flex.end();


		let (frame, _) = createBox(
			frame.x() - 165 - 5,
			frame.y(),
			165, 185,
			30, 120, "Bonuses"
		);

		let width = 45; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - width - 10, frame.y() + 10, width, 300, None);
		flex.set_spacing(5);
		let bonusAP: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Action points").into() );
		let bonusHearing: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Hearing Range").into() );
		let bonusKitStatus: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Kit Status %").into() );
		let bonusSize: Listener<IntInput> = ( IntInput::default().with_size(width, height).with_label("Size Adjustment").into() );
		flex.end();

		//-------------------------------------------------
		// Temperature properties
		let (frame, _) = createBox(
			main.x(),
			main.y() + main.h(),
			mainWidth/2 - 15, 160,
			(mainWidth/2 - 15 - w )/2 +50, 120, "Temperature"
		);

		let mut flex = Pack::new(frame.x() + 100, frame.y() + 10, 50, frame.h() - 10, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Weapon");
		let jamThreshold = FloatInput::default().with_size(width, height).with_label("Jam Threshold").into();
		let dmgThreshold = FloatInput::default().with_size(width, height).with_label("Dmg Threshold").into();
		let increasePerShot = FloatInput::default().with_size(width, height).with_label("Increase / Shot").into();
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 50, frame.y() + 10, 40, frame.h() - 10, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Item");
		let cooldownFactor = FloatInput::default().with_size(width, height).with_label("Cooldown Factor").into();
		let cooldownModifier = FloatInput::default().with_size(width, height).with_label("Cooldown Modifier").into();
		let tempModifier = FloatInput::default().with_size(width, height).with_label("Temp. Modifier").into();
		let jamThresholdModifier = FloatInput::default().with_size(width, height).with_label("Jam Threshold Modifier").into();
		let damageThresholdModifier = FloatInput::default().with_size(width, height).with_label("Damage Threshold Modifier").into();
		flex.end();


		//-------------------------------------------------
		// Dirt
		let (frame, _) = createBox(
			main.x(),
			frame.y() + frame.h(),
			mainWidth/2 - 15, 70,
			(mainWidth/2 - 15 - w )/2, 50, "Dirt"
		);

		let mut flex = Pack::new(frame.x() + frame.w() - 55, frame.y() + 10, 50, frame.h() - 10, None);
		flex.set_spacing(5);
		let dirtDamageChance: Listener<IntInput> = IntInput::default().with_size(width, height).with_label("Damage Chance").into();
		let dirtIncreaseFactor = FloatInput::default().with_size(width, height).with_label("Increase Factor").into();
		flex.end();


		let (frame, _) = createBox(
			main.x(),
			frame.y() + frame.h(),
			mainWidth/2 - 15, 40,
			(mainWidth/2 - 15 - w )/2, 120, "Throwing Knives"
		);

		let mut flex = Pack::new(frame.x() + frame.w() - 155, frame.y() + 10, 150, frame.h() - 10, None);
		flex.set_spacing(5);
		let bloodyItem: Listener<Choice> = Choice::default().with_size(width, height).with_label("Bloody Item").into();
		flex.end();




		let general = WeaponAreaGeneral{ caliber, class, guntype, magsize };
		let stats = WeaponAreaStats { 
			range, accuracy, damage, deadliness, messydeath, meleeDamage, crowbarBonus, autofirespeed, autofirepenalty, burstshots, burstpenalty,
			burstAPcost, reloadAP, manualreloadAP, readyAP, shotsper4turns, brRateOfFire, reloadAnimDelay, burstfireAnimDelay, bulletspeed 
		};
		let properties = WeaponAreaProperties { 
			crowbar, brassknuckles, fullauto, rocketrifle, fingerprintid, easyunjam, heavyweapon, hidemuzzleflash, barrel 
		};
		let ncth = WeaponAreaNCTH { 
			flatbase, flataim, base, cap, handling, tracking, dropCompensation, maxCounterforce, CFaccuracy, CFfrequency, aimlevel, scopeMagFactor,
			laserProjFactor, recoilXmodifier, recoilYmodifier, recoilModifier, accuracyModifier, NCTHaccuracy, recoilX, recoilY, recoilDelay,
			defaultAimLevels, weaponHandling 
		};
		let temp = WeaponAreaTemperature { 
			jamThreshold, dmgThreshold, increasePerShot, cooldownFactor, cooldownModifier, tempModifier, jamThresholdModifier, damageThresholdModifier 
		};
		let modifiers = WeaponAreaModifiers{
			// ranged
			damage: modifierdamage,
			range: modifierrange,
			magSize: modifiermagSize,
			burstSize: modifierburstSize,
			shotsper4turns: modifiershotsper4turns,
			bulletspeed: modifierbulletspeed,
			noiseReduction: modifiernoiseReduction,
			// to hit
			general: modifiergeneral,
			aimedShot: modifieraimedShot,
			bipodProne: modifierbipodProne,
			burst: modifierburst,
			autofire: modifierautofire,
			laserRange: modifierlaserRange,
			minRange: modifierminRange,
			// AP reductions
			generalAP: modifiergeneralAP,
			readyAP: modifierreadyAP,
			reloadAP: modifierreloadAP,
			burstAP: modifierburstAP,
			autofireAP: modifierautofireAP,
			// bonuses
			bonusAP,
			bonusHearing,
			bonusKitStatus,
			bonusSize,
		};
		
		return WeaponArea { general, stats, properties, ncth, temp, modifiers, dirtDamageChance, dirtIncreaseFactor, bloodyItem }
	}

	fn addChoices(&mut self, xmldata: &JAxml::JAxmlState)
	{
		self.general.class.clear();
		self.general.guntype.clear();
		self.general.caliber.clear();


		self.general.class.add_choice("None|Handgun|Submachinegun|Rifle|Machinegun|Shotgun|Knife|Monster");
		self.general.guntype.add_choice("Not gun|Pistol|Machine Pistol|Submachinegun|Rifle|Sniper rifle|Assault rifle|Light machinegun|Shotgun");
		for caliber in &xmldata.calibers.items
		{
			self.general.caliber.add_choice(&caliber.AmmoCaliber);
		}
	}

	fn update(&mut self, xmldata: &JAxml::JAxmlState, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		
		// Update weapon related widgets only if we find a match
		if let Some(weapon) = xmldata.getWeapon(uiIndex as u32)
		{
			self.general.class.activate();
			self.general.guntype.activate();
			self.general.caliber.activate();
			self.general.magsize.activate();
			self.stats.range.activate();
			self.stats.accuracy.activate();
			self.stats.damage.activate();
			self.stats.deadliness.activate();
			self.stats.messydeath.activate();
			self.stats.meleeDamage.activate();
			self.stats.crowbarBonus.activate();
			self.stats.autofirespeed.activate();
			self.stats.autofirepenalty.activate();
			self.stats.burstshots.activate();
			self.stats.burstpenalty.activate();
			self.stats.burstAPcost.activate();
			self.stats.reloadAP.activate();
			self.stats.manualreloadAP.activate();
			self.stats.readyAP.activate();
			self.stats.shotsper4turns.activate();
			self.stats.brRateOfFire.activate();
			self.stats.reloadAnimDelay.activate();
			self.stats.burstfireAnimDelay.activate();
			self.stats.bulletspeed.activate();
			self.properties.fullauto.activate();
			self.properties.easyunjam.activate();
			self.properties.heavyweapon.activate();
			self.ncth.NCTHaccuracy.activate();
			self.ncth.recoilX.activate();
			self.ncth.recoilY.activate();
			self.ncth.recoilDelay.activate();
			self.ncth.defaultAimLevels.activate();
			self.ncth.weaponHandling.activate();
			self.temp.jamThreshold.activate();
			self.temp.dmgThreshold.activate();
			self.temp.increasePerShot.activate();

			self.general.class.set_value(weapon.ubWeaponClass as i32);
			self.general.guntype.set_value(weapon.ubWeaponType as i32);
			self.general.caliber.set_value(weapon.ubCalibre as i32);
			self.general.magsize.set_value(&format!("{}", weapon.ubMagSize));

			self.stats.range.set_value( &format!("{}", weapon.usRange) );
			self.stats.accuracy.set_value( &format!("{}", weapon.bAccuracy) );
			self.stats.damage.set_value( &format!("{}", weapon.ubImpact) );
			self.stats.deadliness.set_value( &format!("{}", weapon.ubDeadliness) );
			self.stats.messydeath.set_value( &format!("{}", weapon.maxdistformessydeath) );
			self.stats.meleeDamage.set_value( &format!("{}", item.meleedamagebonus) );
			self.stats.crowbarBonus.set_value( &format!("{}", item.CrowbarModifier) );
			self.stats.autofirespeed.set_value( &format!("{}", weapon.bAutofireShotsPerFiveAP) );
			self.stats.autofirepenalty.set_value( &format!("{}", weapon.AutoPenalty) );
			self.stats.burstshots.set_value( &format!("{}", weapon.ubShotsPerBurst) );
			self.stats.burstpenalty.set_value( &format!("{}", weapon.ubBurstPenalty) );
			self.stats.burstAPcost.set_value( &format!("{}", weapon.bBurstAP) );
			self.stats.reloadAP.set_value( &format!("{}", weapon.APsToReload) );
			self.stats.manualreloadAP.set_value( &format!("{}", weapon.APsToReloadManually) );
			self.stats.readyAP.set_value( &format!("{}", weapon.ubReadyTime) );
			self.stats.shotsper4turns.set_value( &format!("{}", weapon.ubShotsPer4Turns) );
			self.stats.brRateOfFire.set_value( &format!("{}", item.BR_ROF) );
			self.stats.reloadAnimDelay.set_value( &format!("{}", weapon.usReloadDelay) );
			self.stats.burstfireAnimDelay.set_value( &format!("{}", weapon.sAniDelay) );
			self.stats.bulletspeed.set_value( &format!("{}", weapon.ubBulletSpeed) );

			self.properties.fullauto.set_value(weapon.NoSemiAuto);
			self.properties.easyunjam.set_value(weapon.EasyUnjam);
			self.properties.heavyweapon.set_value(weapon.HeavyGun);

			self.ncth.NCTHaccuracy.set_value( &format!("{}", weapon.nAccuracy) );
			self.ncth.recoilX.set_value( &format!("{}", weapon.bRecoilX) );
			self.ncth.recoilY.set_value( &format!("{}", weapon.bRecoilY) );
			self.ncth.recoilDelay.set_value( &format!("{}", weapon.ubRecoilDelay) );
			self.ncth.defaultAimLevels.set_value( &format!("{}", weapon.ubAimLevels) );
			self.ncth.weaponHandling.set_value( &format!("{}", weapon.ubHandling) );
	
			self.temp.jamThreshold.set_value( &format!("{}", weapon.usOverheatingJamThreshold) );
			self.temp.dmgThreshold.set_value( &format!("{}", weapon.usOverheatingDamageThreshold) );
			self.temp.increasePerShot.set_value( &format!("{}", weapon.usOverheatingSingleShotTemperature) );
		}
		else
		{
			self.general.class.deactivate();
			self.general.guntype.deactivate();
			self.general.caliber.deactivate();
			self.general.magsize.deactivate();
			self.stats.range.deactivate();
			self.stats.accuracy.deactivate();
			self.stats.damage.deactivate();
			self.stats.deadliness.deactivate();
			self.stats.messydeath.deactivate();
			self.stats.meleeDamage.deactivate();
			self.stats.crowbarBonus.deactivate();
			self.stats.autofirespeed.deactivate();
			self.stats.autofirepenalty.deactivate();
			self.stats.burstshots.deactivate();
			self.stats.burstpenalty.deactivate();
			self.stats.burstAPcost.deactivate();
			self.stats.reloadAP.deactivate();
			self.stats.manualreloadAP.deactivate();
			self.stats.readyAP.deactivate();
			self.stats.shotsper4turns.deactivate();
			self.stats.brRateOfFire.deactivate();
			self.stats.reloadAnimDelay.deactivate();
			self.stats.burstfireAnimDelay.deactivate();
			self.stats.bulletspeed.deactivate();
			self.properties.fullauto.deactivate();
			self.properties.easyunjam.deactivate();
			self.properties.heavyweapon.deactivate();
			self.ncth.NCTHaccuracy.deactivate();
			self.ncth.recoilX.deactivate();
			self.ncth.recoilY.deactivate();
			self.ncth.recoilDelay.deactivate();
			self.ncth.defaultAimLevels.deactivate();
			self.ncth.weaponHandling.deactivate();
			self.temp.jamThreshold.deactivate();
			self.temp.dmgThreshold.deactivate();
			self.temp.increasePerShot.deactivate();



			self.general.class.set_value(-1);
			self.general.guntype.set_value(-1);
			self.general.caliber.set_value(-1);

			self.properties.fullauto.set_value(false);
			self.properties.easyunjam.set_value(false);
			self.properties.heavyweapon.set_value(false);

			self.general.magsize.set_value("");
			self.stats.range.set_value( "" );
			self.stats.accuracy.set_value( "" );
			self.stats.damage.set_value( "" );
			self.stats.deadliness.set_value( "" );
			self.stats.messydeath.set_value( "" );
			self.stats.meleeDamage.set_value( "" );
			self.stats.crowbarBonus.set_value( "" );
			self.stats.autofirespeed.set_value( "" );
			self.stats.autofirepenalty.set_value( "" );
			self.stats.burstshots.set_value( "" );
			self.stats.burstpenalty.set_value( "" );
			self.stats.burstAPcost.set_value( "" );
			self.stats.reloadAP.set_value( "" );
			self.stats.manualreloadAP.set_value( "" );
			self.stats.readyAP.set_value( "" );
			self.stats.shotsper4turns.set_value( "" );
			self.stats.brRateOfFire.set_value( "" );
			self.stats.reloadAnimDelay.set_value( "" );
			self.stats.burstfireAnimDelay.set_value( "" );
			self.stats.bulletspeed.set_value( "" );
			self.ncth.NCTHaccuracy.set_value( "" );
			self.ncth.recoilX.set_value( "" );
			self.ncth.recoilY.set_value( "" );
			self.ncth.recoilDelay.set_value( "" );
			self.ncth.defaultAimLevels.set_value( "" );
			self.ncth.weaponHandling.set_value( "" );
			self.temp.jamThreshold.set_value( "" );
			self.temp.dmgThreshold.set_value( "" );
			self.temp.increasePerShot.set_value( "" );

		}




		self.properties.crowbar.set_value(item.crowbar);
		self.properties.brassknuckles.set_value(item.brassknuckles);
		self.properties.rocketrifle.set_value(item.rocketrifle);
		self.properties.fingerprintid.set_value(item.fingerprintid);
		self.properties.hidemuzzleflash.set_value(item.hidemuzzleflash);
		self.properties.barrel.set_value(item.barrel);


		for i in 0..3
		{
			self.ncth.flatbase[i].set_value( &format!("{}", item.flatbasemodifier[i]) );
			self.ncth.flataim[i].set_value( &format!("{}", item.flataimmodifier[i]) );
			self.ncth.base[i].set_value( &format!("{}", item.percentbasemodifier[i]) );
			self.ncth.cap[i].set_value( &format!("{}", item.percentcapmodifier[i]) );
			self.ncth.handling[i].set_value( &format!("{}", item.percenthandlingmodifier[i]) );
			self.ncth.tracking[i].set_value( &format!("{}", item.targettrackingmodifier[i]) );
			self.ncth.dropCompensation[i].set_value( &format!("{}", item.percentdropcompensationmodifier[i]) );
			self.ncth.maxCounterforce[i].set_value( &format!("{}", item.maxcounterforcemodifier[i]) );
			self.ncth.CFaccuracy[i].set_value( &format!("{}", item.counterforceaccuracymodifier[i]) );
			self.ncth.CFfrequency[i].set_value( &format!("{}", item.counterforcefrequency[i]) );
			self.ncth.aimlevel[i].set_value( &format!("{}", item.aimlevelsmodifier[i]) );
		}

		self.ncth.scopeMagFactor.set_value( &format!("{}", item.scopemagfactor) );
		self.ncth.laserProjFactor.set_value( &format!("{}", item.bestlaserrange) );
		self.ncth.recoilXmodifier.set_value( &format!("{}", item.RecoilModifierX) );
		self.ncth.recoilYmodifier.set_value( &format!("{}", item.RecoilModifierY) );
		self.ncth.recoilModifier.set_value( &format!("{}", item.PercentRecoilModifier) );
		self.ncth.accuracyModifier.set_value( &format!("{}", item.percentaccuracymodifier) );


		self.temp.cooldownFactor.set_value( &format!("{}", item.usOverheatingCooldownFactor) );
		self.temp.cooldownModifier.set_value( &format!("{}", item.overheatCooldownModificator) );
		self.temp.tempModifier.set_value( &format!("{}", item.overheatTemperatureModificator) );
		self.temp.jamThresholdModifier.set_value( &format!("{}", item.overheatJamThresholdModificator) );
		self.temp.damageThresholdModifier.set_value( &format!("{}", item.overheatDamageThresholdModificator) );


		// ranged
		self.modifiers.damage.set_value( &format!("{}", item.damagebonus) );
		self.modifiers.range.set_value( &format!("{}", item.rangebonus) );
		self.modifiers.magSize.set_value( &format!("{}", item.magsizebonus) );
		self.modifiers.burstSize.set_value( &format!("{}", item.burstsizebonus) );
		self.modifiers.shotsper4turns.set_value( &format!("{}", item.rateoffirebonus) );
		self.modifiers.bulletspeed.set_value( &format!("{}", item.bulletspeedbonus) );
		self.modifiers.noiseReduction.set_value( &format!("{}", item.stealthbonus) );
		// to hit
		self.modifiers.general.set_value( &format!("{}", item.tohitbonus) );
		self.modifiers.aimedShot.set_value( &format!("{}", item.aimbonus) );
		self.modifiers.bipodProne.set_value( &format!("{}", item.bipod) );
		self.modifiers.burst.set_value( &format!("{}", item.bursttohitbonus) );
		self.modifiers.autofire.set_value( &format!("{}", item.autofiretohitbonus) );
		self.modifiers.laserRange.set_value( &format!("{}", item.bestlaserrange) );
		self.modifiers.minRange.set_value( &format!("{}", item.minrangeforaimbonus) );
		// AP reductions
		self.modifiers.generalAP.set_value( &format!("{}", item.percentapreduction) );
		self.modifiers.readyAP.set_value( &format!("{}", item.percentreadytimeapreduction) );
		self.modifiers.reloadAP.set_value( &format!("{}", item.percentreloadtimeapreduction) );
		self.modifiers.burstAP.set_value( &format!("{}", item.percentburstfireapreduction) );
		self.modifiers.autofireAP.set_value( &format!("{}", item.percentautofireapreduction) );
		// bonuses
		self.modifiers.bonusAP.set_value( &format!("{}", item.APBonus) );
		self.modifiers.bonusHearing.set_value( &format!("{}", item.hearingrangebonus) );
		self.modifiers.bonusKitStatus.set_value( &format!("{}", item.percentstatusdrainreduction) );
		self.modifiers.bonusSize.set_value( &format!("{}", item.ItemSizeBonus) );

		self.dirtDamageChance.set_value( &format!("{}", item.usDamageChance) );
		self.dirtIncreaseFactor.set_value( &format!("{}", item.dirtIncreaseFactor) );
	}
}


fn createBox(x: i32, y: i32, w: i32, h: i32, xtitle: i32, widthtitle: i32, label: &str) -> (Frame, Frame)
{
	let mut main = Frame::default().with_size(w, h).with_pos(x, y);
	main.set_frame(FrameType::EngravedBox);

	let mut title = Frame::default().with_size(widthtitle, 20).with_pos(x + xtitle, y - 10).with_label(label);
	title.set_frame(FrameType::FlatBox);
	title.set_label_font(enums::Font::HelveticaBold);

	return (main, title);
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
	ItemClass,
	Tab1,
	Tab2,
	Tab3,
	Tab4
}

    
