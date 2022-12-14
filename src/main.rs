#![allow(non_snake_case)]
#![allow(unused)]
use std::env::current_dir;
// use std::io::{BufReader, Write, Read};
// use std::fs::{File, read};
// use std::fmt;
// use std::str;
use std::path::PathBuf;
use std::time::{Instant};
use fltk::app::{event_inside_widget, frame_border_radius_max};
use fltk::button::{RadioButton, ToggleButton, CheckButton, LightButton, RepeatButton, RadioLightButton, RadioRoundButton, ReturnButton};
use fltk::enums::{Color, Align, Font};
use fltk::group::{Tabs, Group, FlexType, Pack, ColorChooser};
use fltk::input::{IntInput, Input, FloatInput, MultilineInput};
use fltk::menu::{MenuFlag, SysMenuBar, Choice, MenuButton};
use fltk::output::Output;
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
// Update Caliber & Ammo Type sections if Caliber or ammotype is changed for the selected Magazine
// Compatible launchers list for explosives
// Launchables list for launchers
// Context aware (de)activation of widgets



fn main() 
{
	let dataPath = PathBuf::from("H:\\JA2 Dev\\Data-1.13"); // <-- Temporary start path while developing
	let mut xmldata = JAxml::Data::new();
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
	tree.set_connector_style(tree::TreeConnectorStyle::None);
	tree.set_connector_width(0);
	tree.set_margin_left(0);
	fillTree(&mut tree, &xmldata, Message::ShowAll);
	
	// Item info
	let mut itemWindow = Window::default()
		.with_size(980, 720)
		.with_pos(300, 0)
		.with_label("itemWindow"
	);


	//-----------------------------------------------------------------------------
	// Okay, so this is stupid, but I can't get the widgets to work without defining them outside of tabs when using app.wait()
	// With app.run() it works as expected, but I need app.wait() for better event handling via messages and fltk_evented crate
	let tabLabels = vec!["Item\t\t", "Item / Weapon\t", "Ammo / Explosives / Sounds", "Tab4\t\t"];
	let mut tabs = Tabs::new(0, 0, itemWindow.w(), 20, "tabs");
	
	let w = itemWindow.w(); let h = itemWindow.h() - tabs.h();
	
	let tab0 = Group::default().with_size(w, h).below_of(&tabs, 0).with_label(tabLabels[0]);
	tab0.end();
    let tab1 = Group::default().with_size(w, h).right_of(&tab0, 0).with_label(tabLabels[1]);
    tab1.end();
    let tab2 = Group::default().with_size(w, h).right_of(&tab1, 0).with_label(tabLabels[2]);
    tab2.end();
    let tab3 = Group::default().with_size(w, h).right_of(&tab2, 0).with_label(tabLabels[3]);
    tab3.end();
    tabs.end();
	tabs.emit(s, Message::Tabs);


	let mut tabGroups = Vec::new();
	let x = 0;
	let y = 30;
	
	let mut g = Group::default().with_size(itemWindow.w(), itemWindow.h()).with_pos(tabs.x(), tabs.y()+tabs.h());
	let mut itemGraphics = ItemGraphicsArea::initialize(x, y, &s, &images);
	let mut itemStats = ItemStatsArea::initialize(x, 485);
	let mut itemDescription = ItemDescriptionArea::initialize(310, y);
	let mut itemProperties = ItemPropertiesArea::initialize(310, y + 210);
	let mut itemKit = ItemKitArea::initialize(310, 485);
	let mut itemVision = ItemVisionArea::initialize(310+235+10, 485);
	g.end();
	tabGroups.push( g );

	let mut g = Group::default().with_size(itemWindow.w(), itemWindow.h()).with_pos(tabs.x(), tabs.y()+tabs.h());
	let mut weaponArea = WeaponArea::initialize(x, y);
	g.end();
	g.hide();
	tabGroups.push( g );

	let mut g = Group::default().with_size(itemWindow.w(), itemWindow.h()).with_pos(tabs.x(), tabs.y()+tabs.h());
	let mut magArea = MagazineArea::initialize(x, y, &s);
	let mut expArea = ExplosivesArea::initialize(980-490, y, &s);
	let mut soundArea = SoundsArea::initialize(980-490, y+360);
	g.end();
	g.hide();
	tabGroups.push( g );

	let mut g = Group::default().with_size(itemWindow.w(), itemWindow.h()).with_pos(tabs.x(), tabs.y()+tabs.h());
	g.end();
	g.hide();
	tabGroups.push( g );
	//-----------------------------------------------------------------------------

	
	itemWindow.end();
	mainWindow.end();
	// mainWindow.make_resizable(true);
	mainWindow.show();

	itemVision.addChoicesToClothesTypes(&xmldata);
	weaponArea.addChoices(&xmldata);
	magArea.addChoices(&xmldata);
	expArea.addChoices(&xmldata);
	soundArea.addChoices(&xmldata);

	let mut uidata = UIdata{ 
		images, itemDescription, itemGraphics, itemKit, itemProperties, itemStats, itemVision, magArea, weaponArea, expArea, soundArea,
		state: State::Item 
	};
	//-----------------------------------------------------------------------------
	// Main loop
	//-----------------------------------------------------------------------------    
    let mut index = 0;
    while a.wait() 
    {
		// if let Some(w) = app::belowmouse::<widget::Widget>()
		// {
		// 	println!("{}", w.label());
		// }

		if tree.triggered()
		{
 			if let Some(item) = tree.first_selected_item() 
 			{
                println!("{} selected", item.label().unwrap());
                let uiIndex = unsafe{item.user_data::<u32>()}.unwrap() as usize;
                println!("uiIndex {}", uiIndex);
                
				uidata.update(&xmldata, uiIndex);
				itemWindow.redraw()
			}
			else 
			{
				uidata.itemGraphics.clearImages();
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
					openFileDialog(&mut xmldata, &mut uidata.images, &mut tree);
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
				ShowDrugs | ShowAmmoTypeData | ShowCaliberData | ShowSoundData | ShowBurstSoundData => 
				{
					fillTree(&mut tree, &xmldata, msg);
					uidata.changeState(msg);
				}
				// Item Window
				Redraw => 
				{
					itemWindow.redraw();
				}
				GraphicScroll =>
				{
					uidata.itemGraphics.redrawScrollAreaImages(&uidata.images);
					itemWindow.redraw();
				}
				GraphicType =>
				{
					uidata.itemGraphics.updateScrollBarBounds(&uidata.images);
					uidata.itemGraphics.redrawScrollAreaImages(&uidata.images);

				}
				Tabs => 
				{
					for tab in &mut tabGroups { tab.hide(); }
					
					let tab = tabs.value().unwrap().label();
					match tab
					{
						x if x == tabLabels[0] => { tabGroups[0].show(); }
						x if x == tabLabels[1] => { tabGroups[1].show(); }
						x if x == tabLabels[2] => { tabGroups[2].show(); }
						x if x == tabLabels[3] => { tabGroups[3].show(); }
						_ => ()
					}

					itemWindow.redraw();
				}
				AmmoTypeFontColor =>
				{
					uidata.magArea.changeColor();
				}
				_ => {}
	        }
        }

		uidata.itemGraphics.poll(&uidata.images, &s);
    }
}


//---------------------------------------------------------------------------------------------------------------------
// Functions
//---------------------------------------------------------------------------------------------------------------------
fn openFileDialog(xmldata: &mut JAxml::Data, images: &mut STI::Images, tree: &mut Listener<tree::Tree>)
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

fn saveFileDialog(xmldata: &JAxml::Data)
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

fn loadData(xmldata: &mut JAxml::Data, images: &mut STI::Images, dataPath: &PathBuf)
{
	let t = Instant::now();
	xmldata.loadData(&dataPath);
	println!("Loading xml data took: {:?}", t.elapsed());
	let t = Instant::now();
	images.loadImages(&dataPath);
	println!("Loading sti files took: {:?}", t.elapsed());
}

fn saveData(dataPath: &PathBuf, xmldata: &JAxml::Data)
{
	let t = Instant::now();
	xmldata.saveData(&dataPath);
	println!("Saving xml data took: {:?}", t.elapsed());
}

fn fillTree(tree: &mut Listener<tree::Tree>, xmldata: &JAxml::Data, msg: Message)
{
  	tree.clear();
	use Message::*;
  	match msg
  	{
		ShowAll =>
		{
			for item in &xmldata.items.items
			{
				addItemToTree(tree, item);
		    }
		}
		ShowGuns =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Gun)
		}
		ShowAmmo =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Ammo)
		}
		ShowArmor =>
		{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Armor)
		}
    	ShowLaunchers =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Launcher)
    	}
    	ShowGrenades =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Grenade)
    	}
    	ShowExplosives =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Bomb)
    	}
    	ShowKnives =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Blade)
    	}
    	ShowOther =>
    	{
			for item in &xmldata.items.items
			{
				if item.usItemClass == JAxml::ItemClass::Thrown as u32 || item.usItemClass == JAxml::ItemClass::Punch as u32
				{
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowFaceGear =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Face)
    	}
    	ShowKits =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Kit)
    	}
    	ShowMedical =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Medkit)
    	}
    	ShowKeys =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Key)
    	}
    	ShowLBE =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::LBE)
    	}
    	ShowMisc =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Misc)
    	}
    	ShowNone =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::None)
    	}
    	ShowRandom =>
    	{
			matchItemClass(xmldata, tree, JAxml::ItemClass::Random)
    	}
    	ShowScifi =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.scifi
			    {
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowNonScifi =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.scifi == false
			    {
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowTonsOfGuns =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.biggunlist
			    {
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowReducedGuns =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.biggunlist == false
			    {
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowAttachments =>
    	{
		    for item in &xmldata.items.items
		    {
			    if item.attachment
			    {
					addItemToTree(tree, item);
				}
			}
    	}
    	ShowDrugs =>
    	{
			for item in &xmldata.items.items
		    {
			    if item.medical
			    {
					addItemToTree(tree, item);
				}
			}
    	}
		ShowAmmoTypeData => 
		{
			for item in &xmldata.ammotypes.items
			{
				let name: String;
				if item.name.contains("/")
				{
					name = item.name.replace("/", "\\/");
				}
				else
				{
					name = item.name.clone();
				}

				if item.uiIndex < 10
				{
					tree.add(&format!("[{}]      {}", item.uiIndex, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", item.uiIndex, name) );
				}

				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(item.uiIndex);
			}
		}
		ShowCaliberData => 
		{
			for item in &xmldata.calibers.items
			{
				let name: String;
				if item.AmmoCaliber.contains("/")
				{
					name = item.AmmoCaliber.replace("/", "\\/");
				}
				else
				{
					name = item.AmmoCaliber.clone();
				}

				if item.uiIndex < 10
				{
					tree.add(&format!("[{}]      {}", item.uiIndex, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", item.uiIndex, name) );
				}
					
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(item.uiIndex);
			}
		}
		ShowSoundData => 
		{
			let mut i = 0;
			for item in &xmldata.sounds.sounds
			{
				let name: String;
				if item.contains("\\")
				{
					name = item.replace("\\", "\\\\");
				}
				else
				{
					name = item.clone();
				}

				if i < 10
				{
					tree.add(&format!("[{}]      {}", i, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", i, name) );
				}
				
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(i);
				i += 1;
			}
		}
		ShowBurstSoundData => 
		{
			let mut i = 0;
			for item in &xmldata.burstsounds.sounds
			{
				let name: String;
				if item.contains("\\")
				{
					name = item.replace("\\", "\\\\");
				}
				else
				{
					name = item.clone();
				}

				if i < 10
				{
					tree.add(&format!("[{}]      {}", i, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", i, name) );
				}
				
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(i);
				i += 1;
			}
		}
		ShowExplosionData =>
		{
			for item in &xmldata.explosiondata.items
			{
				let i = item.uiIndex;
				let name = item.name.clone();

				if i < 10
				{
					tree.add(&format!("[{}]      {}", i, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", i, name) );
				}
				
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(i);
			}
		}
		ShowClothesData =>
		{
			for item in &xmldata.clothes.items
			{
				let i = item.uiIndex;
				let name = item.szName.clone();

				if i < 10
				{
					tree.add(&format!("[{}]      {}", i, name) );
				}
				else
				{
					tree.add(&format!("[{}]    {}", i, name) );
				}
				
				let mut treeitem = tree.last().unwrap();
				treeitem.set_user_data(i);
			}
		}
		_ => {}
	}

	tree.set_vposition(0);
	tree.redraw();
}

fn matchItemClass(xmldata: &JAxml::Data, tree: &mut Listener<tree::Tree>, itemClass: JAxml::ItemClass)
{
	for item in &xmldata.items.items
	{
		if item.usItemClass == itemClass as u32
		{
			addItemToTree(tree, item);
		}
	}
}

fn addItemToTree(tree: &mut Listener<tree::Tree>, item: &JAxml::ITEM)
{
	let name: String;
	if item.szLongItemName.contains("/")
	{
		name = item.szLongItemName.replace("/", "\\/");
	}
	else
	{
		name = item.szLongItemName.clone();
	}

	if item.uiIndex < 10
	{
		tree.add(&format!("[{}]      {}", item.uiIndex, name) );
	}
	else
	{
		tree.add(&format!("[{}]    {}", item.uiIndex, name) );
	}


	let mut treeitem = tree.last().unwrap();
	treeitem.set_user_data(item.uiIndex);
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
	menu.add_emit(
	    "&Data/Ammo Calibers\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowCaliberData
	);
	menu.add_emit(
	    "&Data/Ammo Types\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowAmmoTypeData
	);
	menu.add_emit(
	    "&Data/Explosion Data\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowExplosionData
	);
	menu.add_emit(
	    "&Data/Sounds\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowSoundData
	);
	menu.add_emit(
	    "&Data/Burst Sounds\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowBurstSoundData
	);	
	menu.add_emit(
	    "&Data/Clothes\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowClothesData
	);

	return menu;
}


fn set_bit_at(x: u32, index: u8, bit: u8) -> Option<u32> {
    if index >= 32 {
        println!("The new bit is out of u32 range.");
        println!("0b_11111111");
        println!("  ^ trying to set here");
        return None;
    }

    if bit == 0
    {
        let bitmask = 1 << index;
        let bitmask_flipped = !bitmask; // flip all bits
        let result = x & bitmask_flipped;
        // println!("x        = {},    binary = {:#010b} ", x, x);
        // println!("bitmask  = {},    binary = {:#010b} ", bitmask, bitmask);
        // println!("!bitmask = {},    binary = {:#010b} ", bitmask_flipped, bitmask_flipped);
        // println!("result   = {},    binary = {:#010b} ", result, result);
        Some(result)
    }
    else
    {
        let bitmask = 1 << index;
        let result = x | bitmask;
        // println!("x        = {},    binary = {:#010b} ", x, x);
        // println!("bitmask  = {},    binary = {:#010b} ", bitmask, bitmask);
        // println!("result   = {},    binary = {:#010b} ", result, result);
        Some(result)

    }
}


fn get_bit_at(x: u32, index: u8) -> Option<u32> {
    if index >= 32
    {
        println!("Index is out of u32 range.");
        println!("0b_11111111");
        println!("  ^ trying to read from here");
        return None;
    }
    if x & (1 << index) != 0 {return Some(1);}
    else {return Some(0);}
}
//---------------------------------------------------------------------------------------------------------------------
// Structs
//---------------------------------------------------------------------------------------------------------------------
struct UIdata
{
	images: STI::Images,
	itemGraphics: ItemGraphicsArea,
	itemDescription: ItemDescriptionArea,
	itemProperties: ItemPropertiesArea,
	itemStats: ItemStatsArea,
	itemKit: ItemKitArea,
	itemVision: ItemVisionArea,
	weaponArea: WeaponArea,
	magArea: MagazineArea,
	expArea: ExplosivesArea,
	soundArea: SoundsArea,
	state: State,
}
impl UIdata
{
	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		use State::*;
		match self.state
		{
			Item => 
			{
				self.updateItem(&xmldata, uiIndex);
				self.updateWeapon(&xmldata, uiIndex);
				self.updateMagazine(&xmldata, uiIndex);
				self.expArea.update(&xmldata, uiIndex);
				self.soundArea.update(&xmldata, uiIndex);
			}
			AmmoCalibers => 
			{
				self.magArea.updateCaliber(&xmldata, uiIndex);
			}
			AmmoTypes => 
			{
				self.magArea.updateAmmoType(&xmldata, uiIndex);
			}
			Sounds => {}
		}
	}

	fn updateItem(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		self.itemGraphics.update(&xmldata, &self.images, uiIndex);
		self.itemDescription.update(&xmldata, uiIndex);
		self.itemProperties.update(&xmldata, uiIndex);
		self.itemStats.update(&xmldata, uiIndex);
		self.itemKit.update(&xmldata, uiIndex);
		self.itemVision.update(&xmldata, uiIndex);
	}

	fn updateWeapon(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		self.weaponArea.update(&xmldata, uiIndex);
	}

	fn updateMagazine(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		self.magArea.update(&xmldata, uiIndex);
	}

	fn changeState(&mut self, msg: Message)
	{
		use Message::*;
		match msg
		{
			ShowAmmoTypeData => { self.state = State::AmmoTypes; }
			ShowCaliberData => { self.state = State::AmmoCalibers; }
			ShowSoundData | ShowBurstSoundData => { self.state = State::Sounds; }
			_ => { self.state = State::Item }
		}
	}
}

struct ItemGraphicsArea
{
	big: Frame,
	med: Frame,
	small: Frame,
	images: Vec<Listener<Button>>,
	scrollbar: Scrollbar,
	itemType: Choice,
	itemIndex: IntInput,
	itemClass: Listener<Choice>,
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
		// itemClass.emit(*s, Message::ItemClass);

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
			let mut image = Button::default().with_size(w, h).with_pos(scrollArea.x() + 5, scrollArea.y() + 5 + (h+5)*i);
			image.set_frame(FrameType::BorderBox);
			image.set_color(Color::White);

			images.push(image.into());
		}
		
		let w = 20;
		let mut scrollbar = Scrollbar::default().with_pos(scrollArea.x() + scrollArea.w() - w, scrollArea.y()).with_size(w, scrollArea.h());
		scrollbar.emit(*s, Message::GraphicScroll);

		return ItemGraphicsArea{big, med, small, images, scrollbar, itemType, itemIndex, uiIndex, itemClass: itemClass.into()};
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

		// HACK
		// Scrollbar step is 16 by default and I can't find a function in fltk-rs documentation that could actually change it
		// so to get mousewheel scrolling to function properly we multiply the max value by 16 and then divide by same value when checking
		// scrollbar slider position
		self.scrollbar.set_maximum( (16*max) as f64 );
		self.scrollbar.set_minimum(0.0);
    	self.scrollbar.set_value(0.0);

		println!("{}", self.scrollbar.step());
	}

	fn redrawScrollAreaImages(&mut self, sti: &STI::Images)
	{
		let mut graphType = self.itemType.value() as usize;
		if graphType >= sti.big.len()
		{
			println!("!!! In redrawScrollAreaImages !!!");
    		println!("Tried to access nonexistent graphtype! images[{}]", graphType);
    		println!("Defaulting to guns");
			graphType = 0;
		}


		let w = self.images[0].w(); let h = self.images[0].h();
		let start = (self.scrollbar.value() as usize / 16);
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

	fn update(&mut self, xmldata: &JAxml::Data, images: &STI::Images, uiIndex: usize)
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

	fn poll(&mut self, sti: &STI::Images, s: &app::Sender<Message>)
	{
		let j = self.itemType.value() as usize;
		let start = (self.scrollbar.value() as usize / 16);

		
		for i in 0..self.images.len()
		{
			let image = &self.images[i];

			if image.triggered()
			{
				let index = start + i;

				if index < sti.big[j].len()
				{
					self.updateItemGraphics(sti, j, index);
					s.send(Message::Redraw);
				}
				else 
				{
					println!("!!! Tried to access image [{}][{}] !!!", j, index);
				}
			}
		}
	}
}


struct ItemStatsArea
{
	price: Listener<IntInput>,
	weight: Listener<IntInput>,
	nperpocket: Listener<IntInput>,
	size: Listener<IntInput>,
	reliability: Listener<IntInput>,
	repairease: Listener<IntInput>,
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

		let mut flex = Flex::default().with_pos(x + xMargin + w, y + yMargin).with_size(w, h);
		flex.set_type(FlexType::Column);

		let mut price = IntInput::default();
		flex.set_size(&mut price, 20);
		let price = price.into();

		let mut weight = IntInput::default();
		flex.set_size(&mut weight, 20);
		let weight = weight.into();

		let mut nperpocket = IntInput::default();
		flex.set_size(&mut nperpocket, 20);
		let nperpocket = nperpocket.into();

		let mut size = IntInput::default();
		flex.set_size(&mut size, 20);
		let size = size.into();

		let mut reliability = IntInput::default();
		flex.set_size(&mut reliability, 20);
		let reliability = reliability.into();

		let mut repairease = IntInput::default();
		flex.set_size(&mut repairease, 20);
		let repairease = repairease.into();

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

		return ItemStatsArea { price, nperpocket, reliability, repairease, size, weight, cursor }
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		self.price.set_value(&format!("{}", item.usPrice));
		self.weight.set_value(&format!("{}", item.ubWeight));
		self.nperpocket.set_value(&format!("{}", item.ubPerPocket));
		self.size.set_value(&format!("{}", item.ItemSize));
		self.reliability.set_value(&format!("{}", item.bReliability));
		self.repairease.set_value(&format!("{}", item.bRepairEase));

		self.cursor.set_value(item.ubCursor as i32);
	}
}

struct ItemDescriptionArea
{
	name: Listener<Input>,
	longname: Listener<Input>,
	BRname: Listener<Input>,
	description: Listener<MultilineInput>,
	BRdescription: Listener<MultilineInput>
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

		let xOffset = 80;
		let h1 = 30; let h2 = 100;
		let w = 240;
		
		let mut flex = Pack::new(x + xOffset, y + 10, w, 180, None);
		flex.set_spacing(10);
		let name = Input::default().with_size(0, h1).with_label("Name\n[80]").into();
		let longname = Input::default().with_size(0, h1).with_label("Long Name\n[80]").into();
		let mut description: Listener<_> = MultilineInput::default().with_size(0, h2).with_label("Description\n[400]").into();
		flex.end();
		
		
		let mut flex = Pack::new(flex.x()+flex.w() + 80, y + 10, w, 180, None);
		flex.set_spacing(10);
		let _ = Frame::default().with_size(0, h1).with_label("Bobby Ray's");
		let BRname = Input::default().with_size(0, h1).with_label("Name\n[80]").into();
		let mut BRdescription: Listener<_> = MultilineInput::default().with_size(0, h2).with_label("Description\n[400]").into();
		flex.end();
		
		
		description.set_wrap(true);
		BRdescription.set_wrap(true);

		return ItemDescriptionArea { name, longname, BRname, description, BRdescription };
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		if uiIndex < xmldata.items.items.len()
		{
			let item = &xmldata.items.items[uiIndex];
			self.name.set_value(&item.szItemName);
			self.longname.set_value(&item.szLongItemName);
			self.description.set_value(&item.szItemDesc);
			self.BRname.set_value(&item.szBRName);
			self.BRdescription.set_value(&item.szBRDesc);

			let label = format!("Name\n[{}]", 80 - item.szItemName.len());
			self.name.set_label(&label);
			let label = format!("Long Name\n[{}]", 80 - item.szLongItemName.len());
			self.longname.set_label(&label);
			let label = format!("Description\n[{}]", 400 - item.szItemDesc.len());
			self.description.set_label(&label);
			let label = format!("Name\n[{}]", 80 - item.szBRName.len());
			self.BRname.set_label(&label);
			let label = format!("Description\n[{}]", 400 - item.szBRDesc.len());
			self.BRdescription.set_label(&label);
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
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

	fn addChoicesToClothesTypes(&mut self, xmldata: &JAxml::Data)
	{
		self.clothesType.clear();
		for cloth in &xmldata.clothes.items
		{
			self.clothesType.add_choice(&format!("{}", cloth.szName));
		}
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
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
	magazinefed: Listener<CheckButton>,
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
		let magazinefed = CheckButton::default().with_size(width, height).with_label("Magazine Fed").into();
		let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		flex.end();
		// let mut flex = Pack::new(flex.x() + flex.w() + 50, frame.y() + 10, 45, frame.h() - 10, None);
		// flex.set_spacing(5);
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// let _ = CheckButton::default().with_size(width, height).with_label("").deactivate();//.into();
		// flex.end();



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
			frame.y() + 20,
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
			crowbar, brassknuckles, fullauto, rocketrifle, fingerprintid, easyunjam, heavyweapon, hidemuzzleflash, barrel, magazinefed 
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

	fn addChoices(&mut self, xmldata: &JAxml::Data)
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
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
			self.properties.magazinefed.activate();
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
			self.properties.magazinefed.set_value(weapon.swapClips);

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
			self.properties.magazinefed.deactivate();
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


struct AmmoTypesArea
{
	index: Listener<IntInput>,
	name: Listener<Input>,
	nbullets: Listener<IntInput>,
	explosionid: Listener<Choice>,
	explosionsize: Listener<Choice>,
	rgb: (u8, u8, u8),
	standardissue: Listener<CheckButton>,
	dart: Listener<CheckButton>,
	knife: Listener<CheckButton>,
	acidic: Listener<CheckButton>,
	ignorearmor: Listener<CheckButton>,
	tracer: Listener<CheckButton>,
	zeromindamage: Listener<CheckButton>,
	monsterspit: Listener<CheckButton>,
	structImpactMultiplier: Listener<IntInput>,
	armorImpactMultiplier: Listener<IntInput>,
	beforeArmorMultpilier: Listener<IntInput>,
	afterArmorMultiplier: Listener<IntInput>,
	bulletsMultiplier	: Listener<IntInput>,
	structImpactDivisor: Listener<IntInput>,
	armorImpactDivisor: Listener<IntInput>,
	beforeArmorDivisor: Listener<IntInput>,
	afterArmorDivisor: Listener<IntInput>,
	bulletsDivisor: Listener<IntInput>,
	healthModifier: Listener<FloatInput>,
	breathModifier: Listener<FloatInput>,
	tankModifier: Listener<FloatInput>,
	armoredVehicleModifier: Listener<FloatInput>,
	civilianVehicleModifier: Listener<FloatInput>,
	zombieModifier: Listener<FloatInput>,
	lockModifier: Listener<IntInput>,
	pierceModifier: Listener<IntInput>,
	temperatureModifier: Listener<FloatInput>,
	dirtModifier: Listener<FloatInput>,
	freezingFlag: Listener<CheckButton>,
	blindingFlag: Listener<CheckButton>,
	antimaterialFlag: Listener<CheckButton>,
	smoketrailFlag: Listener<CheckButton>,
	firetrailFlag: Listener<CheckButton>,
	shotAnimation: Listener<Input>,
	spreadpattern: Listener<Choice>,
}
struct AmmoStringsArea
{
	index: Listener<IntInput>,
	caliber: Listener<Input>,
	brcaliber: Listener<Input>,
	nwsscaliber: Listener<Input>,
}
struct MagazineArea
{
	caliber: Listener<Choice>,
	ammotype: Listener<Choice>,
	magsize: Listener<IntInput>,
	magtype: Listener<Choice>,
	ammostrings: AmmoStringsArea,
	ammotypes: AmmoTypesArea,
	color: Listener<Button>
}
impl MagazineArea
{
	fn initialize(x: i32, y: i32, sender: &app::Sender<Message>) -> MagazineArea
	{
		let mainWidth = 240; let mainHeight = 115;

		// Main framed box. Everything else is located relative to this
		let (frame, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			20, 60, "Magazine"
		);

		let width = 100; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - (width+5), frame.y() + 10, width, frame.h() - 10, None);
		flex.set_spacing(5);
		let caliber: Listener<Choice> = Choice::default().with_size(width, height).with_label("Caliber").into();
		let ammotype = Choice::default().with_size(width, height).with_label("Ammo type").into();
		let magsize = IntInput::default().with_size(width, height).with_label("Magazine size").into();
		let magtype = Choice::default().with_size(width, height).with_label("Magazine type").into();
		flex.end();



		let (frame, _) = createBox(
			frame.x()+frame.w(), frame.y(),
			200, mainHeight,
			20, 60, "Caliber"
		);

		let width = 100; let height = 20;
		let mut flex = Pack::new(frame.x() + frame.w() - (width+5), frame.y() + 10, width, frame.h() - 10, None);
		flex.set_spacing(5);
		let index = IntInput::default().with_size(width, height).with_label("Index").into();
		let ammocaliber = Input::default().with_size(width, height).with_label("Caliber").into();
		let brcaliber = Input::default().with_size(width, height).with_label("Bobby Ray's").into();
		let nwsscaliber = Input::default().with_size(width, height).with_label("NWSS").into();
		flex.end();
		let ammostrings = AmmoStringsArea { index, caliber: ammocaliber, brcaliber, nwsscaliber };



		let (frame, _) = createBox(
			x, frame.y()+frame.h(),
			440, 470,
			10, 100, "Ammo Type"
		);

		let width = 100; let height = 20;
		let firstColumnX = frame.x() + frame.w() - 100;


		let mut flex = Pack::new(frame.x()+50, frame.y()+10, 30, 20, None);
		flex.set_type(group::PackType::Horizontal);
		flex.set_spacing(45);
		let index = IntInput::default().with_size(40, height).with_label("Index").into();
		let name = Input::default().with_size(80, height).with_label("Name").into();
		flex.end();

		let mut flex = Pack::new(frame.x()+155, flex.y()+flex.h()+10, 60, 20, None);
		flex.set_spacing(5);
		let nbullets = IntInput::default().with_size(40, height).with_label("Bullets / shot").into();
		let explosionid = Choice::default().with_size(40, height).with_label("Explosion Item Id").into();
		let explosionsize = Choice::default().with_size(40, height).with_label("Explosion Size").into();
		flex.end();


		let mut flex = Pack::new(frame.x()+220, frame.y()+10, 30, 80, None);
		flex.set_spacing(5);
		let standardissue = CheckButton::default().with_size(width, height).with_label("Std. Issue").into();
		let dart = CheckButton::default().with_size(width, height).with_label("Dart").into();
		let knife = CheckButton::default().with_size(width, height).with_label("Knife").into();
		let acidic = CheckButton::default().with_size(width, height).with_label("Acidic").into();
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 60, flex.y(), 30, 80, None);
		flex.set_spacing(5);
		let ignorearmor = CheckButton::default().with_size(width, height).with_label("Ignore Armor").into();
		let tracer = CheckButton::default().with_size(width, height).with_label("Tracer Effect").into();
		let zeromindamage = CheckButton::default().with_size(width, height).with_label("Zero Min. Dmg").into();
		let monsterspit = CheckButton::default().with_size(width, height).with_label("Monster Spit").into();
		flex.end();



		let mut flex = Pack::new(firstColumnX, flex.y()+flex.h()+20, 30, 100, None);
		flex.set_spacing(5);
		let mut title = Frame::default().with_size(width, height).with_label("Multiplier");
		title.set_label_font(Font::HelveticaBold);
		let structImpactMultiplier = IntInput::default().with_size(width, height).with_label("Struct. Impact Red.").into();
		let armorImpactMultiplier = IntInput::default().with_size(width, height).with_label("Armor Impact Red.").into();
		let beforeArmorMultpilier = IntInput::default().with_size(width, height).with_label("Before Armor Dmg").into();
		let afterArmorMultiplier = IntInput::default().with_size(width, height).with_label("After Armor Dmg").into();
		let bulletsMultiplier = IntInput::default().with_size(width, height).with_label("Multiple Bullet Dmg").into();
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w() + 25, flex.y(), 30, 100, None);
		flex.set_spacing(5);
		let mut title = Frame::default().with_size(width, height).with_label("Divisor");
		title.set_label_font(Font::HelveticaBold);
		let structImpactDivisor = IntInput::default().with_size(width, height).into();
		let armorImpactDivisor = IntInput::default().with_size(width, height).into();
		let beforeArmorDivisor = IntInput::default().with_size(width, height).into();
		let afterArmorDivisor = IntInput::default().with_size(width, height).into();
		let bulletsDivisor = IntInput::default().with_size(width, height).into();
		flex.end();


		let mut title = Frame::default().with_size(width, height).with_pos(frame.x()+170, flex.y()+flex.h()+50).with_label("Modifiers");
		title.set_label_font(Font::HelveticaBold);

		let mut flex = Pack::new(frame.x()+155, flex.y()+flex.h()+70, 35, 100, None);
		flex.set_spacing(5);
		let healthModifier = FloatInput::default().with_size(width, height).with_label("Life Dmg").into();
		let breathModifier = FloatInput::default().with_size(width, height).with_label("Breath Dmg").into();
		let tankModifier = FloatInput::default().with_size(width, height).with_label("Tank Dmg").into();
		let armoredVehicleModifier = FloatInput::default().with_size(width, height).with_label("Armoured Vehicle Dmg").into();
		let civilianVehicleModifier = FloatInput::default().with_size(width, height).with_label("Civilian Vehicle Dmg").into();
		flex.end();

		let mut flex = Pack::new(flex.x() + 200, flex.y(), 35, 100, None);
		flex.set_spacing(5);
		let zombieModifier = FloatInput::default().with_size(width, height).with_label("Zombie Dmg").into();
		let lockModifier = IntInput::default().with_size(width, height).with_label("Lock Bonus Dmg").into();
		let pierceModifier = IntInput::default().with_size(width, height).with_label("Pierce person chance").into();
		let temperatureModifier = FloatInput::default().with_size(width, height).with_label("Temperature").into();
		let dirtModifier = FloatInput::default().with_size(width, height).with_label("Dirt").into();
		flex.end();

		
		let mut title = Frame::default().with_size(width, height).with_pos(frame.x()+50, frame.y()+110).with_label("Ammo bitflags");
		title.set_label_font(Font::HelveticaBold);
		let mut flex = Pack::new(frame.x()+10, title.y()+15, 30, 100, None);
		flex.set_spacing(5);
		let freezingFlag = CheckButton::default().with_size(width, height).with_label("Freezing").into();
		let blindingFlag = CheckButton::default().with_size(width, height).with_label("Blinding").into();
		let antimaterialFlag = CheckButton::default().with_size(width, height).with_label("Anti-Material").into();
		let smoketrailFlag = CheckButton::default().with_size(width, height).with_label("White Smoketrail").into();
		let firetrailFlag = CheckButton::default().with_size(width, height).with_label("Fire trail").into();
		flex.end();
		
		
		let shotAnimation = Input::default().with_size(180, height).with_pos(frame.x()+frame.w()-190, frame.y()+frame.h()-30).with_label("Shot Animation").into();
		let spreadpattern = Choice::default().with_size(180, height).with_pos(frame.x()+frame.w()-190, frame.y()+frame.h()-60).with_label("Spread Pattern").into();
		let mut color: Listener<_> = Button::new(frame.x()+10, frame.y() + frame.h() - 40, 80, 30, "Ammo color").into();
		color.emit(*sender, Message::AmmoTypeFontColor);

		let ammotypes = AmmoTypesArea{ 
			index, name, nbullets, rgb: (255, 255, 255), standardissue, zeromindamage, acidic, afterArmorDivisor, afterArmorMultiplier,
			antimaterialFlag, armorImpactDivisor, armorImpactMultiplier, armoredVehicleModifier, beforeArmorDivisor, beforeArmorMultpilier,
			blindingFlag, breathModifier, bulletsDivisor, bulletsMultiplier, civilianVehicleModifier, dart, dirtModifier, explosionid,
			explosionsize, firetrailFlag, freezingFlag, healthModifier, ignorearmor, knife, lockModifier, monsterspit, pierceModifier,
			smoketrailFlag, structImpactDivisor, structImpactMultiplier, tankModifier, temperatureModifier, tracer, zombieModifier,
			shotAnimation, spreadpattern
		};

		return MagazineArea{ammotype, caliber, magsize, magtype, ammostrings, color, ammotypes};
	}


	fn addChoices(&mut self, xmldata: &JAxml::Data)
	{
		self.caliber.clear();
		self.ammotype.clear();
		self.magtype.clear();
		self.ammotypes.explosionid.clear();
		self.ammotypes.explosionsize.clear();
		self.ammotypes.spreadpattern.clear();


		for item in &xmldata.calibers.items
		{
			self.caliber.add_choice(&format!("{}", item.AmmoCaliber));
		}
		for item in &xmldata.ammotypes.items
		{
			if item.name.contains("/")
			{
				let name = item.name.replace("/", "\\/");
				self.ammotype.add_choice(&format!("{}", name));
			} 
			else
			{
				self.ammotype.add_choice(&format!("{}", item.name));
			}
		}
		self.magtype.add_choice("Magazine|Bullet(s)|Box|Crate");
		self.ammotypes.explosionsize.add_choice("None|Small|Medium|Large|Flame Retardant");

		self.ammotypes.explosionid.add_choice("-");
		for item in &xmldata.items.items
		{
			if item.usItemClass == JAxml::ItemClass::Bomb as u32 || item.usItemClass == JAxml::ItemClass::Grenade as u32
			{
				self.ammotypes.explosionid.add_choice(&format!("{}", item.szItemName));
			}
		}

		self.ammotypes.spreadpattern.add_choice("-");
		for item in &xmldata.spreadpatterns.items
		{
			self.ammotypes.spreadpattern.add_choice(&item.name);
		}
	}

	fn changeColor(&mut self)
	{
		if let Some(color) = dialog::color_chooser("", dialog::ColorMode::Byte)
		{
			self.ammotypes.rgb = color;
		}
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		let itemclass = item.usItemClass;
		let classIndex = item.ubClassIndex;

		if itemclass == JAxml::ItemClass::Ammo as u32
		{
			let mag = &xmldata.magazines.items[classIndex as usize];

			self.caliber.set_value(mag.ubCalibre as i32);
			self.ammotype.set_value(mag.ubAmmoType as i32);
			self.magtype.set_value(mag.ubMagType as i32);
			self.magsize.set_value(&format!("{}", mag.ubMagSize));

			self.updateAmmoType(xmldata, mag.ubAmmoType as usize);
			self.updateCaliber(xmldata, mag.ubCalibre as usize);
		}
	}

	fn updateAmmoType(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.ammotypes.items[uiIndex];
		self.ammotypes.index.set_value(&format!("{}", item.uiIndex));
		self.ammotypes.name.set_value(&format!("{}", item.name));
		self.ammotypes.rgb = (item.red, item.green, item.blue);
		self.ammotypes.nbullets.set_value(&format!("{}", item.numberOfBullets));
		self.ammotypes.shotAnimation.set_value(&format!("{}", item.shotAnimation));
		self.ammotypes.explosionsize.set_value(item.explosionSize as i32);

		self.ammotypes.structImpactMultiplier.set_value(&format!("{}", item.structureImpactReductionMultiplier));
		self.ammotypes.structImpactDivisor.set_value(&format!("{}", item.structureImpactReductionDivisor));
		self.ammotypes.armorImpactMultiplier.set_value(&format!("{}", item.armourImpactReductionMultiplier));
		self.ammotypes.armorImpactDivisor.set_value(&format!("{}", item.armourImpactReductionDivisor));
		self.ammotypes.beforeArmorMultpilier.set_value(&format!("{}", item.beforeArmourDamageMultiplier));
		self.ammotypes.beforeArmorDivisor.set_value(&format!("{}", item.beforeArmourDamageDivisor));
		self.ammotypes.afterArmorMultiplier.set_value(&format!("{}", item.afterArmourDamageMultiplier));
		self.ammotypes.afterArmorDivisor.set_value(&format!("{}", item.afterArmourDamageDivisor));
		self.ammotypes.bulletsMultiplier.set_value(&format!("{}", item.multipleBulletDamageMultiplier));
		self.ammotypes.bulletsDivisor.set_value(&format!("{}", item.multipleBulletDamageDivisor));
		
		self.ammotypes.acidic.set_value(item.acidic);
		self.ammotypes.dart.set_value(item.dart);
		self.ammotypes.standardissue.set_value(item.standardIssue);
		self.ammotypes.knife.set_value(item.knife);
		self.ammotypes.ignorearmor.set_value(item.ignoreArmour);
		self.ammotypes.tracer.set_value(item.tracerEffect);
		self.ammotypes.zeromindamage.set_value(item.zeroMinimumDamage);
		self.ammotypes.monsterspit.set_value(item.monsterSpit);

		self.ammotypes.healthModifier.set_value(&format!("{}", item.dDamageModifierLife));
		self.ammotypes.breathModifier.set_value(&format!("{}", item.dDamageModifierBreath));
		self.ammotypes.tankModifier.set_value(&format!("{}", item.dDamageModifierTank));
		self.ammotypes.armoredVehicleModifier.set_value(&format!("{}", item.dDamageModifierArmouredVehicle));
		self.ammotypes.civilianVehicleModifier.set_value(&format!("{}", item.dDamageModifierCivilianVehicle));
		self.ammotypes.zombieModifier.set_value(&format!("{}", item.dDamageModifierZombie));
		self.ammotypes.lockModifier.set_value(&format!("{}", item.lockBustingPower));
		self.ammotypes.pierceModifier.set_value(&format!("{}", item.usPiercePersonChanceModifier));
		self.ammotypes.temperatureModifier.set_value(&format!("{}", item.temperatureModificator));
		self.ammotypes.dirtModifier.set_value(&format!("{}", item.dirtModificator));

		let flags = item.ammoflag;
		self.ammotypes.freezingFlag.set_value(get_bit_at(flags, 0).unwrap() != 0);
		self.ammotypes.blindingFlag.set_value(get_bit_at(flags, 1).unwrap() != 0);
		self.ammotypes.antimaterialFlag.set_value(get_bit_at(flags, 2).unwrap() != 0);
		self.ammotypes.smoketrailFlag.set_value(get_bit_at(flags, 3).unwrap() != 0);
		self.ammotypes.firetrailFlag.set_value(get_bit_at(flags, 4).unwrap() != 0);

		if item.highExplosive != 0
		{
			let name = &xmldata.items.items[item.highExplosive as usize].szItemName;

			let widgetindex = self.ammotypes.explosionid.find_index(name);
			self.ammotypes.explosionid.set_value(widgetindex);
		} else { self.ammotypes.explosionid.set_value(-1); }


		if !item.spreadPattern.is_empty()
		{
			let name = &item.spreadPattern;

			let widgetindex = self.ammotypes.spreadpattern.find_index(name);
			self.ammotypes.spreadpattern.set_value(widgetindex);
		} else { self.ammotypes.spreadpattern.set_value(-1); }
	}

	fn updateCaliber(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.calibers.items[uiIndex];

		self.ammostrings.index.set_value(&format!("{}", item.uiIndex));
		self.ammostrings.caliber.set_value(&format!("{}", item.AmmoCaliber));
		self.ammostrings.brcaliber.set_value(&format!("{}", item.BRCaliber));
		self.ammostrings.nwsscaliber.set_value(&format!("{}", item.NWSSCaliber));
	}

}


struct ExplosivesArea
{
	// Bomb/Grenade
	explosionType: Listener<Choice>,
	animID: Listener<Choice>,
	damage: Listener<IntInput>,
	startRadius: Listener<IntInput>,
	endRadius: Listener<IntInput>,
	duration: Listener<IntInput>,
	volatility: Listener<IntInput>,
	stundamage: Listener<IntInput>,
	volume: Listener<IntInput>,
	magsize: Listener<IntInput>,
	fragmentType: Listener<Choice>,
	fragments: Listener<IntInput>,
	fragrange: Listener<IntInput>,
	fragdamage: Listener<IntInput>,
	indoormodifier: Listener<FloatInput>,
	horizontaldegrees: Listener<IntInput>,
	verticaldegrees: Listener<IntInput>,
	explodeOnImpact: Listener<CheckButton>,
	// Launcher
	launcherType: Listener<Choice>,
	discardeditem: Listener<Choice>,
}
impl ExplosivesArea
{
	fn initialize(x: i32, y: i32, sender: &app::Sender<Message>) -> ExplosivesArea
	{
		let mainWidth = 480; let mainHeight = 360;

		// Main framed box. Everything else is located relative to this
		let (frame, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			120, 80, "Explosives"
		);

		let width = 100; let height = 20;
		let explosionType: Listener<_> = Choice::default().with_size(width, height).with_pos(x+100, y+10).with_label("Type").into();
		let animID = Choice::default().with_size(width, height).with_pos(x+100, y+40).with_label("Animation ID").into();
		let explodeOnImpact = CheckButton::default().with_size(width, height).with_pos(explosionType.x()+explosionType.w(), y+10).with_label("Explode on impact").into();


		let mut flex = Pack::new(frame.x()+100, frame.y()+70, 35, 100, None);
		flex.set_spacing(5);
		let damage = IntInput::default().with_size(width, height).with_label("Damage").into();
		let startRadius = IntInput::default().with_size(width, height).with_label("Start Radius").into();
		let duration = IntInput::default().with_size(width, height).with_label("Duration").into();
		let volatility = IntInput::default().with_size(width, height).with_label("Volatility").into();
		flex.end();

		let mut flex = Pack::new(flex.x()+150, flex.y(), 35, 100, None);
		flex.set_spacing(5);
		let stundamage = IntInput::default().with_size(width, height).with_label("Stun Damage").into();
		let endRadius = IntInput::default().with_size(width, height).with_label("End Radius").into();
		let volume = IntInput::default().with_size(width, height).with_label("Volume").into();
		let magsize = IntInput::default().with_size(width, height).with_label("Mag Size").into();
		flex.end();


		let fragmentType: Listener<_> = Choice::default().with_size(width, height).with_pos(x+100, flex.y()+flex.h()).with_label("Frag Type").into();

		let mut flex = Pack::new(fragmentType.x(), fragmentType.y()+fragmentType.h()+10, 35, 80, None);
		flex.set_spacing(5);
		let fragments = IntInput::default().with_size(width, height).with_label("# of Fragments").into();
		let fragrange = IntInput::default().with_size(width, height).with_label("Frag Range").into();
		let horizontaldegrees = IntInput::default().with_size(width, height).with_label("Horiz. Degrees").into();
		flex.end();

		let mut flex = Pack::new(flex.x()+150, flex.y(), 35, 80, None);
		flex.set_spacing(5);
		let fragdamage = IntInput::default().with_size(width, height).with_label("Frag Damage").into();
		let indoormodifier = FloatInput::default().with_size(width, height).with_label("Indoor Mod.").into();
		let verticaldegrees = IntInput::default().with_size(width, height).with_label("Vert. Degrees").into();
		flex.end();


		let (frame, _) = createBox(
			x+10, flex.y()+flex.h(),
			240, 70,
			80, 80, "Launchers"
		);

		let width = 130; let height = 20;
		let launcherType = Choice::default().with_size(width, height).with_pos(frame.x()+100, frame.y()+10).with_label("Type").into();
		let discardeditem = Choice::default().with_size(width, height).with_pos(frame.x()+100, frame.y()+40).with_label("Discarded Item").into();


		return ExplosivesArea{ 
			animID, damage, duration, endRadius, explosionType, fragdamage, fragmentType, fragments, fragrange, horizontaldegrees, indoormodifier,
			magsize, startRadius, stundamage, verticaldegrees, volatility, volume, discardeditem, launcherType, explodeOnImpact 
		};
	}

	fn addChoices(&mut self, xmldata: &JAxml::Data)
	{
		self.explosionType.clear();
		self.animID.clear();
		self.fragmentType.clear();
		self.launcherType.clear();
		self.discardeditem.clear();

		self.explosionType.add_choice("Normal|Stun|Tear gas|Mustard gas|Flare|Noise|Smoke|Creature gas|Burnable gas|Flashbang|Signal Smoke|Smoke Debris|Smoke FireRetardant|Any Type");
		self.animID.add_choice("No Blast|Small|Medium|Large|Stun|Underwater|Tear gas|Smoke|Mustard|Fire|Thermobaric|Flashbang|Roof Collapse|Roof Collapse Smoke");

		for item in &xmldata.ammotypes.items
		{
			if item.name.contains("/")
			{
				let name = item.name.replace("/", "\\/");
				self.fragmentType.add_choice(&format!("{}", name));
			} 
			else
			{
				self.fragmentType.add_choice(&format!("{}", item.name));
			}
		}

		self.launcherType.add_choice("N\\/A|Grenade Launcher|Rocket Launcher|Single Shot Rocket|Mortar|Cannon");

		self.discardeditem.add_choice("-");
		for item in &xmldata.items.items
		{
			if item.usItemClass == JAxml::ItemClass::Misc as u32
			{
				if item.szItemName.contains("/")
				{
					let name = item.szItemName.replace("/", "\\/");
					self.discardeditem.add_choice(&format!("{}", name));
				} 
				else
				{
					self.discardeditem.add_choice(&format!("{}", item.szItemName));
				}
			}
		}
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		let itemclass = item.usItemClass;
		let classIndex = item.ubClassIndex;

		use JAxml::ItemClass::*;
		match itemclass
		{
			x if x == Grenade as u32 || x == Bomb as u32 =>
			{
				let explosive = &xmldata.explosives.items[classIndex as usize];

				self.explosionType.set_value(explosive.ubType as i32);
				self.animID.set_value(explosive.ubAnimationID as i32);
				self.fragmentType.set_value(explosive.ubFragType as i32);
				self.damage.set_value( &format!("{}", explosive.ubDamage) );
				self.startRadius.set_value( &format!("{}", explosive.ubStartRadius) );
				self.endRadius.set_value( &format!("{}", explosive.ubRadius) );
				self.duration.set_value( &format!("{}", explosive.ubDuration) );
				self.volatility.set_value( &format!("{}", explosive.ubVolatility) );
				self.stundamage.set_value( &format!("{}", explosive.ubStunDamage) );
				self.volume.set_value( &format!("{}", explosive.ubVolume) );
				self.magsize.set_value( &format!("{}", explosive.ubMagSize) );
				self.fragments.set_value( &format!("{}", explosive.usNumFragments) );
				self.fragrange.set_value( &format!("{}", explosive.ubFragRange) );
				self.fragdamage.set_value( &format!("{}", explosive.ubFragDamage) );
				self.indoormodifier.set_value( &format!("{}", explosive.bIndoorModifier) );
				self.horizontaldegrees.set_value( &format!("{}", explosive.ubHorizontalDegree) );
				self.verticaldegrees.set_value( &format!("{}", explosive.ubVerticalDegree) );
				self.explodeOnImpact.set_value(explosive.fExplodeOnImpact);
			}
			x if x == Launcher as u32 =>
			{
				let gl = item.grenadelauncher;
				let rl = item.rocketlauncher;
				let singleshot = item.singleshotrocketlauncher;
				let mortar = item.mortar;
				let cannon = item.cannon;

				if gl { self.launcherType.set_value(1); }
				else if rl { self.launcherType.set_value(2); }
				else if singleshot { self.launcherType.set_value(3); }
				else if mortar { self.launcherType.set_value(4); }
				else if cannon { self.launcherType.set_value(5); }
				else { self.launcherType.set_value(0); }

				if singleshot 
				{
					self.discardeditem.activate();
					let name = &xmldata.items.items[item.discardedlauncheritem as usize].szItemName;
		
					let widgetindex = self.discardeditem.find_index(name);
					self.discardeditem.set_value(widgetindex);
				} else 
				{ 
					self.discardeditem.set_value(-1);
					self.discardeditem.deactivate();
				}
			}
			_ => {}
		}
	}
}


struct SoundsArea
{
	attackVolume: Listener<IntInput>,
	hitVolume: Listener<IntInput>,
	attack: Listener<Choice>,
	burst: Listener<Choice>,
	silenced: Listener<Choice>,
	silencedBurst: Listener<Choice>,
	reload: Listener<Choice>,
	locknload: Listener<Choice>,
	manualreload: Listener<Choice>,
}
impl SoundsArea
{
	fn initialize(x: i32, y: i32) -> SoundsArea
	{
		let mainWidth = 480; let mainHeight = 325;

		// Main framed box. Everything else is located relative to this
		let (frame, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			120, 80, "Sounds"
		);

		let width = 80; let height = 20;
		let mut flex = Pack::new(frame.x()+150, frame.y()+20, width, 50, None);
		flex.set_spacing(5);
		let attackVolume = IntInput::default().with_size(width, height).with_label("Attack Volume").into();
		let hitVolume = IntInput::default().with_size(width, height).with_label("Hit Volume").into();
		flex.end();


		let width = 250;
		let mut flex = Pack::new(flex.x(), flex.y()+flex.h(), width, 100, None);
		flex.set_spacing(5);
		let attack = Choice::default().with_size(width, height).with_label("Attack Sound").into();
		let burst = Choice::default().with_size(width, height).with_label("Burst Sound").into();
		let silenced = Choice::default().with_size(width, height).with_label("Silenced Sound").into();
		let silencedBurst = Choice::default().with_size(width, height).with_label("Silenced Burst Sound").into();
		let reload = Choice::default().with_size(width, height).with_label("Reload Sound").into();
		let locknload = Choice::default().with_size(width, height).with_label("Lock'n'Load Sound").into();
		let manualreload = Choice::default().with_size(width, height).with_label("Manual Reload Sound").into();
		flex.end();


		return SoundsArea{ attackVolume, hitVolume, attack, burst, locknload, manualreload, reload, silenced, silencedBurst };
	}

	fn addChoices(&mut self, xmldata: &JAxml::Data)
	{
		self.attack.clear();
		self.burst.clear();
		self.silenced.clear();
		self.silencedBurst.clear();
		self.reload.clear();
		self.locknload.clear();
		self.manualreload.clear();

		let mut i = 0;
		for sound in &xmldata.sounds.sounds
		{
			let path: String;
			if sound.contains("\\")
			{
				path = sound.replace("\\", "\\\\");
			}
			else { path = sound.clone(); }
			let path = format!("[{}] {}", i, path);
			i += 1;

			self.attack.add_choice(&path);
			self.silenced.add_choice(&path);
			self.reload.add_choice(&path);
			self.locknload.add_choice(&path);
			self.manualreload.add_choice(&path);
		}

		let mut i = 0;
		for sound in &xmldata.burstsounds.sounds
		{
			let path: String;
			if sound.contains("\\")
			{
				path = sound.replace("\\", "\\\\");
			}
			else { path = sound.clone(); }
			let path = format!("[{}] {}", i, path);
			i += 1;

			self.burst.add_choice(&path);
			self.silencedBurst.add_choice(&path);
		}
	}


	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: usize)
	{
		let item = &xmldata.items.items[uiIndex];
		let itemclass = item.usItemClass;

		use JAxml::ItemClass::*;
		match itemclass
		{
			x if x == Gun as u32 || x == Launcher as u32 || x == Punch as u32 =>
			{
				if let Some(weapon) = &xmldata.getWeapon(uiIndex as u32)
				{
					
					self.attackVolume.set_value(&format!("{}", weapon.ubAttackVolume));
					self.hitVolume.set_value(&format!("{}", weapon.ubHitVolume));
					
					self.attack.set_value( weapon.sSound as i32 );
					self.silenced.set_value( weapon.silencedSound as i32 );
					self.reload.set_value( weapon.sReloadSound as i32 );
					self.locknload.set_value( weapon.sLocknLoadSound as i32 );
					self.manualreload.set_value( weapon.ManualReloadSound as i32 );
					self.burst.set_value( weapon.sBurstSound as i32 );
					self.silencedBurst.set_value( weapon.sSilencedBurstSound as i32 );
				} else
				{
					println!("!!! COULDN'T FIND WEAPON DATA TO UPDATE FOR uiIndex {}", uiIndex);
				}
			}
			_ => {}
		}
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
//---------------------------------------------------------------------------------------------------------------------
// Enums
//---------------------------------------------------------------------------------------------------------------------
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
	ShowCaliberData,
	ShowAmmoTypeData,
	ShowExplosionData,
	ShowSoundData,
	ShowBurstSoundData,
	ShowClothesData,
    Redraw,
    GraphicScroll,
	GraphicType,
	ItemClass,
	Tabs,
	AmmoTypeFontColor,
}

    
#[derive(Copy, Clone)]
pub enum State {
	Item,
	AmmoTypes,
	AmmoCalibers,
	Sounds,
}