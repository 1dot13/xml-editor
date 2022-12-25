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
// Add/Delete/Duplicate items
// Change item class & uiIndex
// Prompt to save work upon quitting if needed
// Inventories
// Merchants?
// Error checking
// Only allow saving of valid data
// Compatible launchers list for explosives
// Launchables list for launchers
// Bloody item selection
// Attachments
// bloodbag & splint flags
// Filtering for armor items


fn main() 
{
	let dataPath = PathBuf::from("H:\\JA2 Dev\\Data-1.13"); // <-- Temporary start path while developing
	// let mut dataPath = current_dir().unwrap();
	// dataPath.push("Data-1.13");
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
	let tabLabels = vec!["Item\t\t", "Item / Weapon\t", "Ammo / Explosives / Sounds", "Armor\t\t"];
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
	let mut armorArea = ArmorArea::initialize(x, y, &s);
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
		armorArea, state: State::Item 
	};
	//-----------------------------------------------------------------------------
	// Main loop
	//-----------------------------------------------------------------------------    
    let mut uiIndex = u32::MAX;
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
                uiIndex = unsafe{item.user_data::<u32>()}.unwrap();
                println!("uiIndex {}", uiIndex);
                
				uidata.update(&xmldata, uiIndex, &s);
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
				ShowDrugs | ShowAmmoTypeData | ShowCaliberData | ShowSoundData | ShowBurstSoundData | ShowClothesData
				| ShowArmorData => 
				{
					fillTree(&mut tree, &xmldata, msg);
					uidata.changeState(msg);
				}
				// Item Window
				Redraw => 
				{
					itemWindow.redraw();
				}
				Update =>
				{
					uidata.update(&xmldata, uiIndex, &s);
				}
				GraphicScroll =>
				{
					uidata.itemGraphics.redrawScrollAreaImages(&uidata.images, &s);
				}
				GraphicType =>
				{
					uidata.itemGraphics.updateScrollBarBounds(&uidata.images);
					uidata.itemGraphics.redrawScrollAreaImages(&uidata.images, &s);
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
				_ => {}
	        }
        }

		if uiIndex != u32::MAX { uidata.poll( &mut xmldata, uiIndex, &s); }
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
		ShowArmorData =>
		{
			for item in &xmldata.armors.items
			{
				let i = item.uiIndex;
				let name = "Armor Entry";

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
		Shortcut::Alt | 'q',
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
	menu.add_emit(
	    "&Data/Armor Data\t",
		Shortcut::None,
		MenuFlag::Normal,
		*s,
		Message::ShowArmorData
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
	armorArea: ArmorArea,
	state: State,
}
impl UIdata
{
	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		use State::*;
		match self.state
		{
			Item => 
			{
				self.updateItem(&xmldata, uiIndex, s);
				self.weaponArea.update(&xmldata, uiIndex);
				self.magArea.update(&xmldata, uiIndex);
				self.expArea.update(&xmldata, uiIndex);
				self.soundArea.update(&xmldata, uiIndex);
				self.armorArea.update(&xmldata, uiIndex);
			}
			AmmoCalibers => 
			{
				self.magArea.updateCaliber(&xmldata, uiIndex);
			}
			AmmoTypes => 
			{
				self.magArea.updateAmmoType(&xmldata, uiIndex);
			}
			Armors => { self.armorArea.updateFromArmorData(&xmldata, uiIndex); }
			Sounds => {}
			_ => {}
		}

		s.send(Message::Redraw);
	}

	fn updateItem(&mut self, xmldata: &JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		self.itemGraphics.update(&xmldata, &self.images, uiIndex, s);
		self.itemDescription.update(&xmldata, uiIndex);
		self.itemProperties.update(&xmldata, uiIndex);
		self.itemStats.update(&xmldata, uiIndex);
		self.itemKit.update(&xmldata, uiIndex);
		self.itemVision.update(&xmldata, uiIndex);
	}

	fn changeState(&mut self, msg: Message)
	{
		use Message::*;
		match msg
		{
			ShowAmmoTypeData => { self.state = State::AmmoTypes; }
			ShowCaliberData => { self.state = State::AmmoCalibers; }
			ShowSoundData | ShowBurstSoundData => { self.state = State::Sounds; }
			ShowClothesData => { self.state = State::Clothes; }
			ShowArmorData => { self.state = State::Armors; }
			_ => { self.state = State::Item }
		}
	}

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		use State::*;
		match self.state
		{
			Item => 
			{
				self.itemGraphics.poll(xmldata, uiIndex, &self.images, s);
				self.itemStats.poll(xmldata, uiIndex, s);
				self.itemDescription.poll(xmldata, uiIndex, s);
				self.itemProperties.poll(xmldata, uiIndex, s);
				self.itemKit.poll(xmldata, uiIndex, s);
				self.itemVision.poll(xmldata, uiIndex, s);
				self.weaponArea.poll(xmldata, uiIndex, s);
				self.magArea.poll(xmldata, uiIndex, s);
				self.expArea.poll(xmldata, uiIndex, s);
				self.soundArea.poll(xmldata, uiIndex, s);
				self.armorArea.poll(xmldata, uiIndex, s);
			}
			AmmoCalibers => 
			{
				self.magArea.pollcaliber(xmldata, uiIndex, s);
			}
			AmmoTypes => 
			{
				self.magArea.pollAmmoType(xmldata, uiIndex, s);
			}
			Armors => { self.armorArea.pollFromArmorData(xmldata, uiIndex, s); }
			Sounds => {}
			_ => {}
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
	itemType: Listener<Choice>,
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


		let itemType = itemType.into();
		let itemClass = itemClass.into();
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
		let max: usize;
		if sti.big[i].len() > self.images.len()
		{
			max = sti.big[i].len() - self.images.len();
		}
		else
		{
			max = sti.big[i].len();
		}

		// HACK
		// Scrollbar step is 16 by default and I can't find a function in fltk-rs documentation that could actually change it
		// so to get mousewheel scrolling to function properly we multiply the max value by 16 and then divide by same value when checking
		// scrollbar slider position
		self.scrollbar.set_maximum( (16*max) as f64 );
		self.scrollbar.set_minimum(0.0);
    	self.scrollbar.set_value(0.0);
	}

	fn redrawScrollAreaImages(&mut self, sti: &STI::Images, s: &app::Sender<Message>)
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

		s.send(Message::Redraw);
	}

	fn updateItemGraphics(&mut self, images: &STI::Images, stiType: usize, stiIndex: usize)
	{
		let margin = 4;
		
		if let Some(mut image) = images.getbig(stiType, stiIndex)
		{
			let width = self.big.w() - margin;
			let height = self.big.h() - margin;
			image.scale(width, height, true, true);
			self.big.set_image(Some(image));
		} else {
			self.big.set_image(None::<RgbImage>);
		}
		
		if let Some(mut image) = images.getmed(stiType, stiIndex)
		{
			let width = self.med.w() - margin;
			let height = self.med.h() - margin;
			image.scale(width, height, true, true);
			self.med.set_image(Some(image));
		} else {
			self.med.set_image(None::<RgbImage>);
		}
		
		if let Some(mut image) = images.getsmall(stiType, stiIndex)
		{
			let width = self.small.w() - margin;
			let height = self.small.h() - margin;
			image.scale(width, height, true, true);
			self.small.set_image(Some(image));
		} else {
			self.small.set_image(None::<RgbImage>);
		}
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

	fn update(&mut self, xmldata: &JAxml::Data, images: &STI::Images, uiIndex: u32, s: &app::Sender<Message>)
	{
		let item = &xmldata.items.items[uiIndex as usize];

		let stiType = item.ubGraphicType as usize;
		let stiIndex = item.ubGraphicNum as usize;
		// Only print type and item index if they change
		// if let Some(value) = self.itemIndex.value().parse::<i32>().ok()
		// {
		// 	if value != stiIndex as i32 { 
		// 		println!("Graphic Type {}", stiType);
		// 		println!("Graphic index {}", stiIndex); 
		// 	}
		// }

		if stiType < images.big.len() && stiIndex < images.big[stiType].len()
		{
			self.updateItemGraphics(&images, stiType, stiIndex);

			if stiType != self.itemType.value() as usize
			{
				self.itemType.set_value(stiType as i32);
				self.updateScrollBarBounds(&images);
				self.redrawScrollAreaImages(&images, s);
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, sti: &STI::Images, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			let start = (self.scrollbar.value() as usize / 16);
			let j = self.itemType.value() as usize;
			for i in 0..self.images.len()
			{
				let image = &self.images[i];

				if image.triggered()
				{
					let index = start + i;

					if index < sti.big[j].len()
					{
						item.ubGraphicNum = index as u16;
						s.send(Message::Update);
					}
					else 
					{
						println!("!!! Tried to access image [{}][{}] !!!", j, index);
					}
				}
			}

			if self.itemIndex.changed()
			{
				let value = self.itemIndex.value().parse::<u32>();
				match value
				{
					Ok(value) => 
					{
						if value < sti.big[j].len() as u32
						{
							item.ubGraphicNum = value as u16;
							self.itemIndex.set_text_color(Color::Black);
							s.send(Message::Update);
						}
						else { 
							self.itemIndex.set_text_color(Color::Red); 
							s.send(Message::Redraw);
						}
					}
					_ => { self.itemIndex.set_text_color(Color::Red); }
				}
			}

			if self.itemType.triggered() && item.ubGraphicType != j as u8
			{
				item.ubGraphicType = self.itemType.value() as u8;
				item.ubGraphicNum = 0;

				self.updateScrollBarBounds(sti);
				self.redrawScrollAreaImages(sti, s);
				s.send(Message::Update);
			}
		}
	}
}


struct ItemStatsArea
{
	price: IntInput,
	weight: IntInput,
	nperpocket: IntInput,
	size: IntInput,
	reliability: IntInput,
	repairease: IntInput,
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

		let mut weight = IntInput::default();
		flex.set_size(&mut weight, 20);

		let mut nperpocket = IntInput::default();
		flex.set_size(&mut nperpocket, 20);

		let mut size = IntInput::default();
		flex.set_size(&mut size, 20);

		let mut reliability = IntInput::default();
		flex.set_size(&mut reliability, 20);

		let mut repairease = IntInput::default();
		flex.set_size(&mut repairease, 20);

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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
			self.price.set_value(&format!("{}", item.usPrice));
			self.weight.set_value(&format!("{}", item.ubWeight));
			self.nperpocket.set_value(&format!("{}", item.ubPerPocket));
			self.size.set_value(&format!("{}", item.ItemSize));
			self.reliability.set_value(&format!("{}", item.bReliability));
			self.repairease.set_value(&format!("{}", item.bRepairEase));
			self.cursor.set_value(item.ubCursor as i32);
		}
	}

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			if let Some(value) = u16IntInput(&mut self.price, s)
			{
				item.usPrice = value;
			}

			if let Some(value) = u16IntInput(&mut self.weight, s)
			{
				item.ubWeight = value;
			}

			if let Some(value) = u8IntInput(&mut self.nperpocket, s)
			{
				item.ubPerPocket = value;
			}

			if let Some(value) = u16IntInput(&mut self.size, s)
			{
				item.ItemSize = value;
			}

			if let Some(value) = i8IntInput(&mut self.reliability, s)
			{
				item.bReliability = value;
			}

			if let Some(value) = i8IntInput(&mut self.repairease, s)
			{
				item.bRepairEase = value;
			}
			// cursor
			if self.cursor.triggered()
			{
				use JAxml::Cursor::*;

				let value = self.cursor.value();
				match value
				{
					x if x == Invalid as i32 => { item.ubCursor = Invalid as u8; }
					x if x == Quest as i32 => { item.ubCursor = Quest as u8; }
					x if x == Punch as i32 => { item.ubCursor = Punch as u8; }
					x if x == Target as i32 => { item.ubCursor = Target as u8; }
					x if x == Knife as i32 => { item.ubCursor = Knife as u8; }
					x if x == Aid as i32 => { item.ubCursor = Aid as u8; }
					x if x == Throw as i32 => { item.ubCursor = Throw as u8; }
					x if x == Mine as i32 => { item.ubCursor = Mine as u8; }
					x if x == Lockpick as i32 => { item.ubCursor = Lockpick as u8; }
					x if x == MineDetector as i32 => { item.ubCursor = MineDetector as u8; }
					x if x == Crowbar as i32 => { item.ubCursor = Crowbar as u8; }
					x if x == CCTV as i32 => { item.ubCursor = CCTV as u8; }
					x if x == Camera as i32 => { item.ubCursor = Camera as u8; }
					x if x == Key as i32 => { item.ubCursor = Key as u8; }
					x if x == Saw as i32 => { item.ubCursor = Saw as u8; }
					x if x == WireCutters as i32 => { item.ubCursor = WireCutters as u8; }
					x if x == Remote as i32 => { item.ubCursor = Remote as u8; }
					x if x == Bomb as i32 => { item.ubCursor = Bomb as u8; }
					x if x == Repair as i32 => { item.ubCursor = Repair as u8; }
					x if x == Trajectory as i32 => { item.ubCursor = Trajectory as u8; }
					x if x == Jar as i32 => { item.ubCursor = Jar as u8; }
					x if x == Tincan as i32 => { item.ubCursor = Tincan as u8; }
					x if x == Refuel as i32 => { item.ubCursor = Refuel as u8; }
					x if x == Fortification as i32 => { item.ubCursor = Fortification as u8; }
					x if x == Handcuffs as i32 => { item.ubCursor = Handcuffs as u8; }
					x if x == ApplyItem as i32 => { item.ubCursor = ApplyItem as u8; }
					x if x == InteractiveAction as i32 => { item.ubCursor = InteractiveAction as u8; }
					x if x == Bloodbag as i32 => { item.ubCursor = Bloodbag as u8; }
					x if x == Splint as i32 => { item.ubCursor = Splint as u8; }
					_ => { println!("!!! Tried to set item cursor to value not in enum::Cursor !!! "); }
				}
				s.send(Message::Update);
			}
		}
	}
}

struct ItemDescriptionArea
{
	name: Input,
	longname: Input,
	BRname: Input,
	description: MultilineInput,
	BRdescription: MultilineInput,
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
		let name = Input::default().with_size(0, h1).with_label("Name\n[80]");
		let longname = Input::default().with_size(0, h1).with_label("Long Name\n[80]");
		let mut description = MultilineInput::default().with_size(0, h2).with_label("Description\n[400]");
		flex.end();
		
		
		let mut flex = Pack::new(flex.x()+flex.w() + 80, y + 10, w, 180, None);
		flex.set_spacing(10);
		let _ = Frame::default().with_size(0, h1).with_label("Bobby Ray's");
		let BRname = Input::default().with_size(0, h1).with_label("Name\n[80]");
		let mut BRdescription = MultilineInput::default().with_size(0, h2).with_label("Description\n[400]");
		flex.end();
		
		
		description.set_wrap(true);
		BRdescription.set_wrap(true);

		return ItemDescriptionArea { name, longname, BRname, description, BRdescription };
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			let widget = &mut self.name;
			if let Some(text) = stringFromInput(widget, s, 80)
			{
				item.szItemName = text;
			}

			let widget = &mut self.longname;
			if let Some(text) = stringFromInput(widget, s, 80)
			{
				item.szLongItemName = text;
			}

			let widget = &mut self.BRname;
			if let Some(text) = stringFromInput(widget, s, 80)
			{
				item.szBRName = text;
			}


			let widget = &mut self.description;
			if let Some(text) = stringFromMultiLineInput(widget, s, 400)
			{
				item.szItemDesc = text;
			}

			let widget = &mut self.BRdescription;
			if let Some(text) = stringFromMultiLineInput(widget, s, 400)
			{
				item.szBRDesc = text;
			}
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
		inputs.push(CheckButton::default().with_size(w, h1).with_label("Cigarette").into());
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
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
			self.inputs[21].set_value(item.cigarette);
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			for i in 0..self.inputs.len()
			{
				let widget = &mut self.inputs[i];
				if widget.triggered()
				{
					match i
					{
						0 => { item.showstatus = widget.value(); }	
						1 => { item.Damageable = widget.value(); }	
						2 => { item.Repairable = widget.value(); }	
						3 => { item.WaterDamages = widget.value(); }	
						4 => { item.Sinks = widget.value(); }	
						5 => { item.unaerodynamic = widget.value(); }	
						6 => { item.electronic = widget.value(); }	
						7 => { item.Metal = widget.value(); }	
						8 => { item.twohanded = widget.value(); }	

						9 => { item.biggunlist = widget.value(); }	
						10 => { item.scifi = widget.value(); }	
						11 => { item.notbuyable = widget.value(); }	
						12 => { item.defaultundroppable = widget.value(); }	
						13 => { item.notineditor = widget.value(); }	
						14 => { item.newinv = widget.value(); }	
						15 => { item.tripwire = widget.value(); }	
						16 => { item.tripwireactivation = widget.value(); }	
						17 => { item.remotetrigger = widget.value(); }	

						18 => { item.containsliquid = widget.value(); }	
						19 => { item.canteen = widget.value(); }	
						20 => { item.gascan = widget.value(); }	
						21 => { item.cigarette = widget.value(); }
						22 => { item.jar = widget.value(); }	
						23 => { item.medical = widget.value(); }	
						24 => { item.gasmask = widget.value(); }	
						25 => { item.robotremotecontrol = widget.value(); }	
						26 => { item.walkman = widget.value(); }	

						27 => { item.rock = widget.value(); }	
						28 => { item.canandstring = widget.value(); }	
						29 => { item.marbles = widget.value(); }	
						30 => { item.duckbill = widget.value(); }	
						31 => { item.wirecutters = widget.value(); }	
						32 => { item.xray = widget.value(); }	
						33 => { item.metaldetector = widget.value(); }	
						34 => { item.batteries = widget.value(); }	
						35 => { item.needsbatteries = widget.value(); }	
						_ => {}
					}
				}
			}
		}
	}
}


struct ItemKitArea
{
	inputs: Vec<Listener<CheckButton>>,
	ints: Vec<IntInput>
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
		ints.push( IntInput::default().with_size(w, h1) );
		ints.push( IntInput::default().with_size(w, h1) );
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		let _ = Frame::default().with_size(w, h1);
		ints.push( IntInput::default().with_size(w, h1).with_label("Defusal Kit Bonus") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Sleep Modifier") );
		flex.end();


		return ItemKitArea { inputs, ints };
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			for i in 0..self.inputs.len()
			{
				let widget = &mut self.inputs[i];
				if widget.triggered()
				{
					match i
					{
						0 => { item.hardware = widget.value(); }
						1 => { item.toolkit = widget.value(); }	
						2 => { item.locksmithkit = widget.value(); }	
						3 => { item.camouflagekit = widget.value(); }	
						4 => { item.medicalkit = widget.value(); }	
						5 => { item.firstaidkit = widget.value(); }	
						_ => {}
					}
				}
			}

			for i in 0..self.ints.len()
			{
				let widget = &mut self.ints[i];

				if widget.changed()
				{
					match i
					{
						0 => 
						{ 
							if let Some(value) = i8IntInput(widget, s) 
							{
								item.RepairModifier = value; 
							}
						}
						1 => 
						{ 
							if let Some(value) = i8IntInput(widget, s) 
							{
								item.LockPickModifier = value; 
							}
						}
						2 => 
						{ 
							if let Some(value) = u8IntInput(widget, s) 
							{
								item.DisarmModifier = value; 
							}
						}
						3 => 
						{ 
							if let Some(value) = u8IntInput(widget, s) 
							{
								item.ubSleepModifier = value; 
							}
						}
						_ => {}
					}
				}
			}
		}
	}
}

struct ItemVisionArea
{
	ints: Vec<IntInput>,
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
		ints.push( IntInput::default().with_size(w, h1).with_label("General") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Nighttime") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Daytime") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Cave") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Bright Light") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Tunnelvision") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Flashlight Range") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Spotting Modifier") );
		let thermal = CheckButton::default().with_size(w, h1).with_label("Thermal Optics").with_align(Align::Left).into();
		flex.end();


		let mut flex = Pack::new(flex.x() + flex.w() + 100, y + 10, w, mainHeight - 20, None);
		flex.set_spacing(5);
		ints.push( IntInput::default().with_size(w, h1).with_label("Woodland") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Urban") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Desert") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Snow") );
		ints.push( IntInput::default().with_size(w, h1).with_label("Stealth") );
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			let widget = &mut self.thermal;
			if widget.triggered() { item.thermaloptics = widget.value(); }

			let widget = &mut self.clothesType;
			if widget.triggered() { item.clothestype = widget.value() as u32; }


			for i in 0..self.ints.len()
			{
				let widget = &mut self.ints[i];

				if widget.changed()
				{
					match i
					{
						0 => { if let Some(value) = i16IntInput(widget, s) {
								item.visionrangebonus = value; 
							}
						}
						1 => { if let Some(value) = i16IntInput(widget, s) {
								item.nightvisionrangebonus = value; 
							}
						}
						2 => { if let Some(value) = i16IntInput(widget, s) {
								item.dayvisionrangebonus = value; 
							}
						}
						3 => { if let Some(value) = i16IntInput(widget, s) {
								item.cavevisionrangebonus = value; 
							}
						}
						4 => { if let Some(value) = i16IntInput(widget, s) {
								item.brightlightvisionrangebonus = value; 
							}
						}
						5 => { if let Some(value) = u8IntInput(widget, s) {
								item.percenttunnelvision = value; 
							}
						}
						6 => { if let Some(value) = u8IntInput(widget, s) {
								item.usFlashLightRange = value; 
							}
						}
						7 => { if let Some(value) = i16IntInput(widget, s) {
								item.usSpotting = value; 
							}
						}
						8 => { if let Some(value) = i16IntInput(widget, s) {
								item.camobonus = value; 
							}
						}
						9 => { if let Some(value) = i16IntInput(widget, s) {
								item.urbanCamobonus = value; 
							}
						}
						10 => { if let Some(value) = i16IntInput(widget, s) {
								item.desertCamobonus = value; 
							}
						}
						11 => { if let Some(value) = i16IntInput(widget, s) {
								item.snowCamobonus = value; 
							}
						}
						12 => { if let Some(value) = i16IntInput(widget, s) {
								item.stealthbonus = value; 
							}
						}
						_ => {}
					}
				}
			}
		}
	}
}


struct WeaponAreaGeneral
{
	class: Listener<Choice>,
	guntype: Listener<Choice>,
	caliber: Listener<Choice>,
	magsize: IntInput
}

struct WeaponAreaStats
{
	range: IntInput,
	accuracy: IntInput,
	damage: IntInput,
	deadliness: IntInput,
	messydeath: IntInput,
	meleeDamage: IntInput,
	crowbarBonus: IntInput,
	autofirespeed: IntInput,
	autofirepenalty: IntInput,
	burstshots: IntInput,
	burstpenalty: IntInput,
	burstAPcost: IntInput,
	reloadAP: IntInput,
	manualreloadAP: IntInput,
	readyAP: IntInput,
	shotsper4turns: FloatInput,
	brRateOfFire: IntInput,
	reloadAnimDelay: IntInput,
	burstfireAnimDelay: IntInput,
	bulletspeed: IntInput,
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
	flatbase: Vec<IntInput>,
	flataim: Vec<IntInput>,
	base: Vec<IntInput>,
	cap: Vec<IntInput>,
	handling: Vec<IntInput>,
	tracking: Vec<IntInput>,
	dropCompensation: Vec<IntInput>,
	maxCounterforce: Vec<IntInput>,
	CFaccuracy: Vec<IntInput>,
	CFfrequency: Vec<IntInput>,
	aimlevel: Vec<IntInput>,
	// Items.xml
	scopeMagFactor: FloatInput,
	laserProjFactor: IntInput,
	recoilXmodifier: FloatInput,
	recoilYmodifier: FloatInput,
	recoilModifier: IntInput,
	accuracyModifier: IntInput,
	// Weapons.xml
	NCTHaccuracy: IntInput,
	recoilX: FloatInput,
	recoilY: FloatInput,
	recoilDelay: IntInput,
	defaultAimLevels: IntInput,
	weaponHandling: IntInput,
}
struct WeaponAreaTemperature
{
	jamThreshold: FloatInput,
	dmgThreshold: FloatInput,
	increasePerShot: FloatInput,
	cooldownFactor: FloatInput,
	cooldownModifier: FloatInput,
	tempModifier: FloatInput,
	jamThresholdModifier: FloatInput,
	damageThresholdModifier: FloatInput,
}
struct WeaponAreaModifiers
{
	// ranged
	damage: IntInput,
	range: IntInput,
	magSize: IntInput,
	burstSize: IntInput,
	shotsper4turns: IntInput,
	bulletspeed: IntInput,
	noiseReduction: IntInput,
	// to hit
	general: IntInput,
	aimedShot: IntInput,
	bipodProne: IntInput,
	burst: IntInput,
	autofire: IntInput,
	laserRange: IntInput,
	minRange: IntInput,
	// AP reductions
	generalAP: IntInput,
	readyAP: IntInput,
	reloadAP: IntInput,
	burstAP: IntInput,
	autofireAP: IntInput,
	// bonuses
	bonusAP: IntInput,
	bonusHearing: IntInput,
	bonusKitStatus: IntInput,
	bonusSize: IntInput,
}
struct WeaponArea
{
	general: WeaponAreaGeneral,
	stats: WeaponAreaStats,
	properties: WeaponAreaProperties,
	ncth: WeaponAreaNCTH,
	temp: WeaponAreaTemperature,
	modifiers: WeaponAreaModifiers,
	dirtDamageChance: IntInput,
	dirtIncreaseFactor: FloatInput,
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
		let magsize = IntInput::default().with_size(width, height).with_label("Capacity");
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
		let range = IntInput::default().with_size(width, height).with_label("Range");
		let accuracy = IntInput::default().with_size(width, height).with_label("Accuracy");
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 80, frame.y() + 10, 70, frame.h() - 10, None);
		flex.set_spacing(5);
		let damage = IntInput::default().with_size(width, height).with_label("Damage");
		let deadliness = IntInput::default().with_size(width, height).with_label("Deadliness");
		let messydeath =  IntInput::default().with_size(width, height).with_label("Messy Death Dist.");
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
		let meleeDamage = IntInput::default().with_size(width, height).with_label("Dmg bonus");
		let crowbarBonus = IntInput::default().with_size(width, height).with_label("Crowbar bonus");
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
		let autofirespeed = IntInput::default().with_size(width, height).with_label("Shots / 5 APs");
		let autofirepenalty = IntInput::default().with_size(width, height).with_label("To-Hit Penalty");
		let fullauto = CheckButton::default().with_size(width, height).with_label("Full Auto only").with_align(Align::Left).into();
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 55, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let burstshots = IntInput::default().with_size(width, height).with_label("Shots / Burst");
		let burstpenalty = IntInput::default().with_size(width, height).with_label("To-Hit Penalty");
		let burstAPcost = IntInput::default().with_size(width, height).with_label("AP Cost");
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
		let reloadAP = IntInput::default().with_size(width, height).with_label("Reload");
		let manualreloadAP = IntInput::default().with_size(width, height).with_label("Manual Reload");
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 55, frame.y() + 10, 45, frame.h() - 10, None);
		flex.set_spacing(5);
		let readyAP = IntInput::default().with_size(width, height).with_label("Ready Weapon");
		let shotsper4turns = FloatInput::default().with_size(width, height).with_label("Shots / 4 turns");
		let brRateOfFire = IntInput::default().with_size(width, height).with_label("BR ROF");
		flex.end();


		//-------------------------------------------------
		// Animation
		let (frame, _) = createBox(
			frame.x(),
			frame.y()+frame.h(),
			frame.w(), 100,
			(frame.w()-w)/2, 100, "Animation"
		);

		let reloadAnimDelay = IntInput::new(x + mainWidth - width - 10, frame.y() + 10, width, height, "Reload Delay");
		let burstfireAnimDelay = IntInput::new(x + mainWidth - width - 10, frame.y() + 40, width, height, "Burst Fire Delay");
		let bulletspeed = IntInput::new(x + mainWidth - width - 10, frame.y() + 70, width, height, "Bullet Speed");


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

		let mut flatbase: Vec<IntInput> = Vec::new();
		let mut flataim: Vec<IntInput> = Vec::new();
		let mut base: Vec<IntInput> = Vec::new();
		let mut cap: Vec<IntInput> = Vec::new();
		let mut handling: Vec<IntInput> = Vec::new();
		let mut tracking: Vec<IntInput> = Vec::new();
		let mut dropCompensation: Vec<IntInput> = Vec::new();
		let mut maxCounterforce: Vec<IntInput> = Vec::new();
		let mut CFaccuracy: Vec<IntInput> = Vec::new();
		let mut CFfrequency: Vec<IntInput> = Vec::new();
		let mut aimlevel: Vec<IntInput> = Vec::new();
	

		let width = 75; let height = 20;
		let mut flex = Pack::new(main.x() + 150, main.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Standing");
		flatbase.push( IntInput::default().with_size(width, height).with_label("Flat Base") );
		flataim.push( IntInput::default().with_size(width, height).with_label("Flat Aim") );
		base.push( IntInput::default().with_size(width, height).with_label("Base %") );
		cap.push( IntInput::default().with_size(width, height).with_label("Cap %") );
		handling.push( IntInput::default().with_size(width, height).with_label("Handling % ") );
		tracking.push( IntInput::default().with_size(width, height).with_label("Tracking %") );
		dropCompensation.push( IntInput::default().with_size(width, height).with_label("Drop Compensation %") );
		maxCounterforce.push( IntInput::default().with_size(width, height).with_label("Max Counterforce %") );
		CFaccuracy.push( IntInput::default().with_size(width, height).with_label("CF Accuracy %") );
		CFfrequency.push( IntInput::default().with_size(width, height).with_label("CF Frequency %") );
		aimlevel.push( IntInput::default().with_size(width, height).with_label("Aimlevel Modifier") );
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w(), flex.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Crouching");
		flatbase.push( IntInput::default().with_size(width, height) );
		flataim.push( IntInput::default().with_size(width, height) );
		base.push( IntInput::default().with_size(width, height) );
		cap.push( IntInput::default().with_size(width, height) );
		handling.push( IntInput::default().with_size(width, height) );
		tracking.push( IntInput::default().with_size(width, height) );
		dropCompensation.push( IntInput::default().with_size(width, height) );
		maxCounterforce.push( IntInput::default().with_size(width, height) );
		CFaccuracy.push( IntInput::default().with_size(width, height) );
		CFfrequency.push( IntInput::default().with_size(width, height) );
		aimlevel.push( IntInput::default().with_size(width, height) );
		flex.end();
		let mut flex = Pack::new(flex.x() + flex.w(), flex.y(), width, 300, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Prone");
		flatbase.push( IntInput::default().with_size(width, height) );
		flataim.push( IntInput::default().with_size(width, height) );
		base.push( IntInput::default().with_size(width, height) );
		cap.push( IntInput::default().with_size(width, height) );
		handling.push( IntInput::default().with_size(width, height) );
		tracking.push( IntInput::default().with_size(width, height) );
		dropCompensation.push( IntInput::default().with_size(width, height) );
		maxCounterforce.push( IntInput::default().with_size(width, height) );
		CFaccuracy.push( IntInput::default().with_size(width, height) );
		CFfrequency.push( IntInput::default().with_size(width, height) );
		aimlevel.push( IntInput::default().with_size(width, height) );
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
		let scopeMagFactor: FloatInput = ( FloatInput::default().with_size(width, height).with_label("Scope Mag Factor") );
		let laserProjFactor: IntInput = ( IntInput::default().with_size(width, height).with_label("Laser Proj. Factor") );
		let recoilXmodifier: FloatInput = ( FloatInput::default().with_size(width, height).with_label("Recoil X Modifier") );
		let recoilYmodifier: FloatInput = ( FloatInput::default().with_size(width, height).with_label("Recoil Y Modifier") );
		let recoilModifier: IntInput = ( IntInput::default().with_size(width, height).with_label("Recoil Modifier %") );
		let accuracyModifier: IntInput = ( IntInput::default().with_size(width, height).with_label("Accuracy Modifier %") );
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
		let NCTHaccuracy: IntInput = ( IntInput::default().with_size(width, height).with_label("NCTH Accuracy") );
		let recoilX: FloatInput = ( FloatInput::default().with_size(width, height).with_label("Recoil X") );
		let recoilY: FloatInput = ( FloatInput::default().with_size(width, height).with_label("Recoil Y") );
		let recoilDelay: IntInput = ( IntInput::default().with_size(width, height).with_label("Recoil Delay") );
		let defaultAimLevels: IntInput = ( IntInput::default().with_size(width, height).with_label("Default Aim Levels") );
		let weaponHandling: IntInput = ( IntInput::default().with_size(width, height).with_label("Weapon Handling") );
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
		let modifierdamage: IntInput = ( IntInput::default().with_size(width, height).with_label("Damage") );
		let modifierrange: IntInput = ( IntInput::default().with_size(width, height).with_label("Range") );
		let modifiermagSize: IntInput = ( IntInput::default().with_size(width, height).with_label("Mag Size") );
		let modifierburstSize: IntInput = ( IntInput::default().with_size(width, height).with_label("Burst Size") );
		let modifiershotsper4turns: IntInput = ( IntInput::default().with_size(width, height).with_label("Shots / 4 turns") );
		let modifierbulletspeed: IntInput = ( IntInput::default().with_size(width, height).with_label("Bullet Speed") );
		let modifiernoiseReduction: IntInput = ( IntInput::default().with_size(width, height).with_label("Noise Reduction") );
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
		let modifiergeneral: IntInput = ( IntInput::default().with_size(width, height).with_label("General %") );
		let modifieraimedShot: IntInput = ( IntInput::default().with_size(width, height).with_label("Aimed Shot %") );
		let modifierbipodProne: IntInput = ( IntInput::default().with_size(width, height).with_label("Bipod/Prone %") );
		let modifierburst: IntInput = ( IntInput::default().with_size(width, height).with_label("Burst %") );
		let modifierautofire: IntInput = ( IntInput::default().with_size(width, height).with_label("Autofire %") );
		let modifierlaserRange: IntInput = ( IntInput::default().with_size(width, height).with_label("Laser Range") );
		let modifierminRange: IntInput = ( IntInput::default().with_size(width, height).with_label("Min. Range") );
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
		let modifiergeneralAP: IntInput = ( IntInput::default().with_size(width, height).with_label("General %") );
		let modifierreadyAP: IntInput = ( IntInput::default().with_size(width, height).with_label("Ready %") );
		let modifierreloadAP: IntInput = ( IntInput::default().with_size(width, height).with_label("Reload %") );
		let modifierburstAP: IntInput = ( IntInput::default().with_size(width, height).with_label("Burst %") );
		let modifierautofireAP: IntInput = ( IntInput::default().with_size(width, height).with_label("Autofire %") );
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
		let bonusAP: IntInput = ( IntInput::default().with_size(width, height).with_label("Action points") );
		let bonusHearing: IntInput = ( IntInput::default().with_size(width, height).with_label("Hearing Range") );
		let bonusKitStatus: IntInput = ( IntInput::default().with_size(width, height).with_label("Kit Status %") );
		let bonusSize: IntInput = ( IntInput::default().with_size(width, height).with_label("Size Adjustment") );
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
		let jamThreshold = FloatInput::default().with_size(width, height).with_label("Jam Threshold");
		let dmgThreshold = FloatInput::default().with_size(width, height).with_label("Dmg Threshold");
		let increasePerShot = FloatInput::default().with_size(width, height).with_label("Increase / Shot");
		flex.end();
		let mut flex = Pack::new(frame.x() + frame.w() - 50, frame.y() + 10, 40, frame.h() - 10, None);
		flex.set_spacing(5);
		let _ = Frame::default().with_size(width, height).with_label("Item");
		let cooldownFactor = FloatInput::default().with_size(width, height).with_label("Cooldown Factor");
		let cooldownModifier = FloatInput::default().with_size(width, height).with_label("Cooldown Modifier");
		let tempModifier = FloatInput::default().with_size(width, height).with_label("Temp. Modifier");
		let jamThresholdModifier = FloatInput::default().with_size(width, height).with_label("Jam Threshold Modifier");
		let damageThresholdModifier = FloatInput::default().with_size(width, height).with_label("Damage Threshold Modifier");
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
		let dirtDamageChance: IntInput = IntInput::default().with_size(width, height).with_label("Damage Chance");
		let dirtIncreaseFactor = FloatInput::default().with_size(width, height).with_label("Increase Factor");
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
			// Update weapon related widgets only if we find a match
			if let Some(weapon) = xmldata.getWeapon(uiIndex)
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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(weapon) = xmldata.getWeapon_mut(uiIndex)
		{
			let widget = &mut self.general.class;
			if widget.triggered()
			{
				weapon.ubWeaponClass = widget.value() as u8;
				s.send(Message::Update);
			}
			let widget = &mut self.general.guntype;
			if widget.triggered()
			{
				weapon.ubWeaponType = widget.value() as u8;
				s.send(Message::Update);
			}
			let widget = &mut self.general.caliber;
			if widget.triggered()
			{
				weapon.ubCalibre = widget.value() as u8;
				s.send(Message::Update);
			}

			if let Some(value) = u16IntInput(&mut self.general.magsize, s) { weapon.ubMagSize = value; }

			if let Some(value) = u16IntInput(&mut self.stats.range, s) { weapon.usRange = value; }
			if let Some(value) = i8IntInput(&mut self.stats.accuracy, s) { weapon.bAccuracy = value; }
			if let Some(value) = u8IntInput(&mut self.stats.damage, s) { weapon.ubImpact = value; }
			if let Some(value) = u8IntInput(&mut self.stats.deadliness, s) { weapon.ubDeadliness = value; }
			if let Some(value) = u8IntInput(&mut self.stats.messydeath, s) { weapon.maxdistformessydeath = value; }
			if let Some(value) = u8IntInput(&mut self.stats.autofirespeed, s) { weapon.bAutofireShotsPerFiveAP = value; }
			if let Some(value) = u8IntInput(&mut self.stats.autofirepenalty, s) { weapon.AutoPenalty = value; }
			if let Some(value) = u8IntInput(&mut self.stats.burstshots, s) { weapon.ubShotsPerBurst = value; }
			if let Some(value) = u8IntInput(&mut self.stats.burstpenalty, s) { weapon.ubBurstPenalty = value; }
			if let Some(value) = u8IntInput(&mut self.stats.burstAPcost, s) { weapon.bBurstAP = value; }
			if let Some(value) = u8IntInput(&mut self.stats.reloadAP, s) { weapon.APsToReload = value; }
			if let Some(value) = u8IntInput(&mut self.stats.manualreloadAP, s) { weapon.APsToReloadManually = value; }
			if let Some(value) = u8IntInput(&mut self.stats.readyAP, s) { weapon.ubReadyTime = value; }
			if let Some(value) = f32FloatInput(&mut self.stats.shotsper4turns, s) { weapon.ubShotsPer4Turns = value; }
			if let Some(value) = u16IntInput(&mut self.stats.reloadAnimDelay, s) { weapon.usReloadDelay = value; }
			if let Some(value) = i16IntInput(&mut self.stats.burstfireAnimDelay, s) { weapon.sAniDelay = value; }
			if let Some(value) = u8IntInput(&mut self.stats.bulletspeed, s) { weapon.ubBulletSpeed = value; }

			let widget = &mut self.properties.fullauto;
			if widget.triggered() { weapon.NoSemiAuto = widget.value(); }
			let widget = &mut self.properties.easyunjam;
			if widget.triggered() { weapon.EasyUnjam = widget.value(); }
			let widget = &mut self.properties.heavyweapon;
			if widget.triggered() { weapon.HeavyGun = widget.value(); }
			let widget = &mut self.properties.magazinefed;
			if widget.triggered() { weapon.swapClips = widget.value(); }

			if let Some(value) = i8IntInput(&mut self.ncth.NCTHaccuracy, s) { weapon.nAccuracy = value; }
			if let Some(value) = f32FloatInput(&mut self.ncth.recoilX, s) { weapon.bRecoilX = value; }
			if let Some(value) = f32FloatInput(&mut self.ncth.recoilY, s) { weapon.bRecoilY = value; }
			if let Some(value) = u8IntInput(&mut self.ncth.recoilDelay, s) { weapon.ubRecoilDelay = value; }
			if let Some(value) = u8IntInput(&mut self.ncth.defaultAimLevels, s) { weapon.ubAimLevels = value; }
			if let Some(value) = u8IntInput(&mut self.ncth.weaponHandling, s) { weapon.ubHandling = value; }
	
			if let Some(value) = f32FloatInput(&mut self.temp.jamThreshold, s) { weapon.usOverheatingJamThreshold = value; }
			if let Some(value) = f32FloatInput(&mut self.temp.dmgThreshold, s) { weapon.usOverheatingDamageThreshold = value; }
			if let Some(value) = f32FloatInput(&mut self.temp.increasePerShot, s) { weapon.usOverheatingSingleShotTemperature = value; }
		}


		// Items.xml related data
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			if let Some(value) = i16IntInput(&mut self.stats.meleeDamage, s) { item.meleedamagebonus = value; }
			if let Some(value) = u8IntInput(&mut self.stats.crowbarBonus, s) { item.CrowbarModifier = value; }
			if let Some(value) = i16IntInput(&mut self.stats.brRateOfFire, s) { item.BR_ROF = value; }

			let widget = &mut self.properties.crowbar;
			if widget.triggered() { item.crowbar = widget.value(); }
			let widget = &mut self.properties.brassknuckles;
			if widget.triggered() { item.brassknuckles = widget.value(); }
			let widget = &mut self.properties.rocketrifle;
			if widget.triggered() { item.rocketrifle = widget.value(); }
			let widget = &mut self.properties.fingerprintid;
			if widget.triggered() { item.fingerprintid = widget.value(); }
			let widget = &mut self.properties.hidemuzzleflash;
			if widget.triggered() { item.hidemuzzleflash = widget.value(); }
			let widget = &mut self.properties.barrel;
			if widget.triggered() { item.barrel = widget.value(); }


			for i in 0..3
			{
				if let Some(value) = i16IntInput(&mut self.ncth.flatbase[i], s) {
					item.flatbasemodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.flataim[i], s) {
					item.flataimmodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.base[i], s) {
					item.percentbasemodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.cap[i], s) {
					item.percentcapmodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.handling[i], s) {
					item.percenthandlingmodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.tracking[i], s) {
					item.targettrackingmodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.dropCompensation[i], s) {
					item.percentdropcompensationmodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.maxCounterforce[i], s) {
					item.maxcounterforcemodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.CFaccuracy[i], s) {
					item.counterforceaccuracymodifier[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.CFfrequency[i], s) {
					item.counterforcefrequency[i] = value;
				}
				if let Some(value) = i16IntInput(&mut self.ncth.aimlevel[i], s) {
					item.aimlevelsmodifier[i] = value;
				}
			}


			if let Some(value) = f32FloatInput(&mut self.ncth.scopeMagFactor, s) {
				item.scopemagfactor = value;
			}
			if let Some(value) = i16IntInput(&mut self.ncth.laserProjFactor, s) {
				item.bestlaserrange = value;
			}
			if let Some(value) = f32FloatInput(&mut self.ncth.recoilXmodifier, s) {
				item.RecoilModifierX = value;
			}
			if let Some(value) = f32FloatInput(&mut self.ncth.recoilYmodifier, s) {
				item.RecoilModifierY = value;
			}
			if let Some(value) = i16IntInput(&mut self.ncth.recoilModifier, s) {
				item.PercentRecoilModifier = value;
			}
			if let Some(value) = i16IntInput(&mut self.ncth.accuracyModifier, s) {
				item.percentaccuracymodifier = value;
			}


			if let Some(value) = f32FloatInput(&mut self.temp.cooldownFactor, s) {
				item.usOverheatingCooldownFactor = value;
			}
			if let Some(value) = f32FloatInput(&mut self.temp.cooldownModifier, s) {
				item.overheatCooldownModificator = value;
			}
			if let Some(value) = f32FloatInput(&mut self.temp.tempModifier, s) {
				item.overheatTemperatureModificator = value;
			}
			if let Some(value) = f32FloatInput(&mut self.temp.jamThresholdModifier, s) {
				item.overheatJamThresholdModificator = value;
			}
			if let Some(value) = f32FloatInput(&mut self.temp.damageThresholdModifier, s) {
				item.overheatDamageThresholdModificator = value;
			}


			// // ranged
			if let Some(value) = i16IntInput(&mut self.modifiers.damage, s) { item.damagebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.range, s) { item.rangebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.magSize, s) { item.magsizebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.burstSize, s) { item.burstsizebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.shotsper4turns, s) { item.rateoffirebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.bulletspeed, s) { item.bulletspeedbonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.noiseReduction, s) { item.stealthbonus = value; }
			// // to hit
			if let Some(value) = i16IntInput(&mut self.modifiers.general, s) { item.tohitbonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.aimedShot, s) { item.aimbonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.bipodProne, s) { item.bipod = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.burst, s) { item.bursttohitbonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.autofire, s) { item.autofiretohitbonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.laserRange, s) { item.bestlaserrange = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.minRange, s) { item.minrangeforaimbonus = value; }
			// // AP reductions
			if let Some(value) = i16IntInput(&mut self.modifiers.generalAP, s) { item.percentapreduction = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.readyAP, s) { item.percentreadytimeapreduction = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.reloadAP, s) { item.percentreloadtimeapreduction = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.burstAP, s) { item.percentburstfireapreduction = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.autofireAP, s) { item.percentautofireapreduction = value; }
			// // bonuses
			if let Some(value) = i16IntInput(&mut self.modifiers.bonusAP, s) { item.APBonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.bonusHearing, s) { item.hearingrangebonus = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.bonusKitStatus, s) { item.percentstatusdrainreduction = value; }
			if let Some(value) = i16IntInput(&mut self.modifiers.bonusSize, s) { item.ItemSizeBonus = value; }


			if let Some(value) = u8IntInput(&mut self.dirtDamageChance, s) { item.usDamageChance = value; }
			if let Some(value) = f32FloatInput(&mut self.dirtIncreaseFactor, s) { item.dirtIncreaseFactor = value; }
		}
	}
}


struct AmmoTypesArea
{
	index: IntInput,
	name: Input,
	nbullets: IntInput,
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
	structImpactMultiplier: IntInput,
	armorImpactMultiplier: IntInput,
	beforeArmorMultpilier: IntInput,
	afterArmorMultiplier: IntInput,
	bulletsMultiplier	: IntInput,
	structImpactDivisor: IntInput,
	armorImpactDivisor: IntInput,
	beforeArmorDivisor: IntInput,
	afterArmorDivisor: IntInput,
	bulletsDivisor: IntInput,
	healthModifier: FloatInput,
	breathModifier: FloatInput,
	tankModifier: FloatInput,
	armoredVehicleModifier: FloatInput,
	civilianVehicleModifier: FloatInput,
	zombieModifier: FloatInput,
	lockModifier: IntInput,
	pierceModifier: IntInput,
	temperatureModifier: FloatInput,
	dirtModifier: FloatInput,
	freezingFlag: Listener<CheckButton>,
	blindingFlag: Listener<CheckButton>,
	antimaterialFlag: Listener<CheckButton>,
	smoketrailFlag: Listener<CheckButton>,
	firetrailFlag: Listener<CheckButton>,
	shotAnimation: Input,
	spreadpattern: Listener<Choice>,
}
struct AmmoStringsArea
{
	index: IntInput,
	caliber: Input,
	brcaliber: Input,
	nwsscaliber: Input,
}
struct MagazineArea
{
	caliber: Listener<Choice>,
	ammotype: Listener<Choice>,
	magsize: IntInput,
	magtype: Listener<Choice>,
	ammostrings: AmmoStringsArea,
	ammotypes: AmmoTypesArea,
	color: Listener<Button>,
	colorbox: Frame,
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
		let magsize = IntInput::default().with_size(width, height).with_label("Magazine size");
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
		let index = IntInput::default().with_size(width, height).with_label("Index");
		let ammocaliber = Input::default().with_size(width, height).with_label("Caliber");
		let brcaliber = Input::default().with_size(width, height).with_label("Bobby Ray's");
		let nwsscaliber = Input::default().with_size(width, height).with_label("NWSS");
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
		let index = IntInput::default().with_size(40, height).with_label("Index");
		let name = Input::default().with_size(80, height).with_label("Name");
		flex.end();

		let mut flex = Pack::new(frame.x()+155, flex.y()+flex.h()+10, 60, 20, None);
		flex.set_spacing(5);
		let nbullets = IntInput::default().with_size(40, height).with_label("Bullets / shot");
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
		let structImpactMultiplier = IntInput::default().with_size(width, height).with_label("Struct. Impact Red.");
		let armorImpactMultiplier = IntInput::default().with_size(width, height).with_label("Armor Impact Red.");
		let beforeArmorMultpilier = IntInput::default().with_size(width, height).with_label("Before Armor Dmg");
		let afterArmorMultiplier = IntInput::default().with_size(width, height).with_label("After Armor Dmg");
		let bulletsMultiplier = IntInput::default().with_size(width, height).with_label("Multiple Bullet Dmg");
		flex.end();

		let mut flex = Pack::new(flex.x() + flex.w() + 25, flex.y(), 30, 100, None);
		flex.set_spacing(5);
		let mut title = Frame::default().with_size(width, height).with_label("Divisor");
		title.set_label_font(Font::HelveticaBold);
		let structImpactDivisor = IntInput::default().with_size(width, height);
		let armorImpactDivisor = IntInput::default().with_size(width, height);
		let beforeArmorDivisor = IntInput::default().with_size(width, height);
		let afterArmorDivisor = IntInput::default().with_size(width, height);
		let bulletsDivisor = IntInput::default().with_size(width, height);
		flex.end();


		let mut title = Frame::default().with_size(width, height).with_pos(frame.x()+170, flex.y()+flex.h()+50).with_label("Modifiers");
		title.set_label_font(Font::HelveticaBold);

		let mut flex = Pack::new(frame.x()+155, flex.y()+flex.h()+70, 35, 100, None);
		flex.set_spacing(5);
		let healthModifier = FloatInput::default().with_size(width, height).with_label("Life Dmg");
		let breathModifier = FloatInput::default().with_size(width, height).with_label("Breath Dmg");
		let tankModifier = FloatInput::default().with_size(width, height).with_label("Tank Dmg");
		let armoredVehicleModifier = FloatInput::default().with_size(width, height).with_label("Armoured Vehicle Dmg");
		let civilianVehicleModifier = FloatInput::default().with_size(width, height).with_label("Civilian Vehicle Dmg");
		flex.end();

		let mut flex = Pack::new(flex.x() + 200, flex.y(), 35, 100, None);
		flex.set_spacing(5);
		let zombieModifier = FloatInput::default().with_size(width, height).with_label("Zombie Dmg");
		let lockModifier = IntInput::default().with_size(width, height).with_label("Lock Bonus Dmg");
		let pierceModifier = IntInput::default().with_size(width, height).with_label("Pierce person chance");
		let temperatureModifier = FloatInput::default().with_size(width, height).with_label("Temperature");
		let dirtModifier = FloatInput::default().with_size(width, height).with_label("Dirt");
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
		
		
		let shotAnimation = Input::default().with_size(180, height).with_pos(frame.x()+frame.w()-190, frame.y()+frame.h()-30).with_label("Shot Animation");
		let spreadpattern = Choice::default().with_size(180, height).with_pos(frame.x()+frame.w()-190, frame.y()+frame.h()-60).with_label("Spread Pattern").into();
		let mut color: Listener<_> = Button::new(frame.x()+10, frame.y() + frame.h() - 40, 80, 30, "Ammo color").into();

		let mut colorbox = Frame::default().with_size(20, 20).with_pos(color.x() + color.w() + 5, color.y());
		colorbox.set_frame(FrameType::EmbossedBox);
		colorbox.set_color(Color::White);

		let ammotypes = AmmoTypesArea{ 
			index, name, nbullets, rgb: (255, 255, 255), standardissue, zeromindamage, acidic, afterArmorDivisor, afterArmorMultiplier,
			antimaterialFlag, armorImpactDivisor, armorImpactMultiplier, armoredVehicleModifier, beforeArmorDivisor, beforeArmorMultpilier,
			blindingFlag, breathModifier, bulletsDivisor, bulletsMultiplier, civilianVehicleModifier, dart, dirtModifier, explosionid,
			explosionsize, firetrailFlag, freezingFlag, healthModifier, ignorearmor, knife, lockModifier, monsterspit, pierceModifier,
			smoketrailFlag, structImpactDivisor, structImpactMultiplier, tankModifier, temperatureModifier, tracer, zombieModifier,
			shotAnimation, spreadpattern
		};

		return MagazineArea{ammotype, caliber, magsize, magtype, ammostrings, color, ammotypes, colorbox};
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
			self.colorbox.set_color(Color::from_rgb(color.0, color.1, color.2));
		}
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
			let itemclass = item.usItemClass;
			let classIndex = item.ubClassIndex;

			self.caliber.deactivate();
			self.ammotype.deactivate();
			self.magsize.deactivate();
			self.magtype.deactivate();
			self.color.deactivate();
			self.colorbox.deactivate();
			self.ammotypes.index.deactivate();
			self.ammotypes.name.deactivate();
			self.ammotypes.nbullets.deactivate();
			self.ammotypes.explosionid.deactivate();
			self.ammotypes.explosionsize.deactivate();
			self.ammotypes.standardissue.deactivate();
			self.ammotypes.dart.deactivate();
			self.ammotypes.knife.deactivate();
			self.ammotypes.acidic.deactivate();
			self.ammotypes.ignorearmor.deactivate();
			self.ammotypes.tracer.deactivate();
			self.ammotypes.zeromindamage.deactivate();
			self.ammotypes.monsterspit.deactivate();
			self.ammotypes.structImpactMultiplier.deactivate();
			self.ammotypes.armorImpactMultiplier.deactivate();
			self.ammotypes.beforeArmorMultpilier.deactivate();
			self.ammotypes.afterArmorMultiplier.deactivate();
			self.ammotypes.bulletsMultiplier.deactivate();
			self.ammotypes.structImpactDivisor.deactivate();
			self.ammotypes.armorImpactDivisor.deactivate();
			self.ammotypes.beforeArmorDivisor.deactivate();
			self.ammotypes.afterArmorDivisor.deactivate();
			self.ammotypes.bulletsDivisor.deactivate();
			self.ammotypes.healthModifier.deactivate();
			self.ammotypes.breathModifier.deactivate();
			self.ammotypes.tankModifier.deactivate();
			self.ammotypes.armoredVehicleModifier.deactivate();
			self.ammotypes.civilianVehicleModifier.deactivate();
			self.ammotypes.zombieModifier.deactivate();
			self.ammotypes.lockModifier.deactivate();
			self.ammotypes.pierceModifier.deactivate();
			self.ammotypes.temperatureModifier.deactivate();
			self.ammotypes.dirtModifier.deactivate();
			self.ammotypes.freezingFlag.deactivate();
			self.ammotypes.blindingFlag.deactivate();
			self.ammotypes.antimaterialFlag.deactivate();
			self.ammotypes.smoketrailFlag.deactivate();
			self.ammotypes.firetrailFlag.deactivate();
			self.ammotypes.shotAnimation.deactivate();
			self.ammotypes.spreadpattern.deactivate();
			self.ammostrings.index.deactivate();
			self.ammostrings.caliber.deactivate();
			self.ammostrings.brcaliber.deactivate();
			self.ammostrings.nwsscaliber.deactivate();
		
			if itemclass == JAxml::ItemClass::Ammo as u32
			{
				self.caliber.activate();
				self.ammotype.activate();
				self.magsize.activate();
				self.magtype.activate();
				self.color.activate();
				self.colorbox.activate();
			
				if let Some(mag) = xmldata.getMagazine(classIndex as u32)
				{
					self.caliber.set_value(mag.ubCalibre as i32);
					self.ammotype.set_value(mag.ubAmmoType as i32);
					self.magtype.set_value(mag.ubMagType as i32);
					self.magsize.set_value(&format!("{}", mag.ubMagSize));

					self.updateAmmoType(xmldata, mag.ubAmmoType as u32);
					self.updateCaliber(xmldata, mag.ubCalibre as u32);
				}
			}
		}
	}

	fn updateAmmoType(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		self.ammotypes.index.deactivate();
		self.ammotypes.name.deactivate();
		self.ammotypes.nbullets.deactivate();
		self.ammotypes.explosionid.deactivate();
		self.ammotypes.explosionsize.deactivate();
		self.ammotypes.standardissue.deactivate();
		self.ammotypes.dart.deactivate();
		self.ammotypes.knife.deactivate();
		self.ammotypes.acidic.deactivate();
		self.ammotypes.ignorearmor.deactivate();
		self.ammotypes.tracer.deactivate();
		self.ammotypes.zeromindamage.deactivate();
		self.ammotypes.monsterspit.deactivate();
		self.ammotypes.structImpactMultiplier.deactivate();
		self.ammotypes.armorImpactMultiplier.deactivate();
		self.ammotypes.beforeArmorMultpilier.deactivate();
		self.ammotypes.afterArmorMultiplier.deactivate();
		self.ammotypes.bulletsMultiplier.deactivate();
		self.ammotypes.structImpactDivisor.deactivate();
		self.ammotypes.armorImpactDivisor.deactivate();
		self.ammotypes.beforeArmorDivisor.deactivate();
		self.ammotypes.afterArmorDivisor.deactivate();
		self.ammotypes.bulletsDivisor.deactivate();
		self.ammotypes.healthModifier.deactivate();
		self.ammotypes.breathModifier.deactivate();
		self.ammotypes.tankModifier.deactivate();
		self.ammotypes.armoredVehicleModifier.deactivate();
		self.ammotypes.civilianVehicleModifier.deactivate();
		self.ammotypes.zombieModifier.deactivate();
		self.ammotypes.lockModifier.deactivate();
		self.ammotypes.pierceModifier.deactivate();
		self.ammotypes.temperatureModifier.deactivate();
		self.ammotypes.dirtModifier.deactivate();
		self.ammotypes.freezingFlag.deactivate();
		self.ammotypes.blindingFlag.deactivate();
		self.ammotypes.antimaterialFlag.deactivate();
		self.ammotypes.smoketrailFlag.deactivate();
		self.ammotypes.firetrailFlag.deactivate();
		self.ammotypes.shotAnimation.deactivate();
		self.ammotypes.spreadpattern.deactivate();
	
		if let Some(item) = xmldata.getAmmoType(uiIndex)
		{
			self.ammotypes.index.activate();
			self.ammotypes.name.activate();
			self.ammotypes.nbullets.activate();
			self.ammotypes.explosionid.activate();
			self.ammotypes.explosionsize.activate();
			self.ammotypes.standardissue.activate();
			self.ammotypes.dart.activate();
			self.ammotypes.knife.activate();
			self.ammotypes.acidic.activate();
			self.ammotypes.ignorearmor.activate();
			self.ammotypes.tracer.activate();
			self.ammotypes.zeromindamage.activate();
			self.ammotypes.monsterspit.activate();
			self.ammotypes.structImpactMultiplier.activate();
			self.ammotypes.armorImpactMultiplier.activate();
			self.ammotypes.beforeArmorMultpilier.activate();
			self.ammotypes.afterArmorMultiplier.activate();
			self.ammotypes.bulletsMultiplier.activate();
			self.ammotypes.structImpactDivisor.activate();
			self.ammotypes.armorImpactDivisor.activate();
			self.ammotypes.beforeArmorDivisor.activate();
			self.ammotypes.afterArmorDivisor.activate();
			self.ammotypes.bulletsDivisor.activate();
			self.ammotypes.healthModifier.activate();
			self.ammotypes.breathModifier.activate();
			self.ammotypes.tankModifier.activate();
			self.ammotypes.armoredVehicleModifier.activate();
			self.ammotypes.civilianVehicleModifier.activate();
			self.ammotypes.zombieModifier.activate();
			self.ammotypes.lockModifier.activate();
			self.ammotypes.pierceModifier.activate();
			self.ammotypes.temperatureModifier.activate();
			self.ammotypes.dirtModifier.activate();
			self.ammotypes.freezingFlag.activate();
			self.ammotypes.blindingFlag.activate();
			self.ammotypes.antimaterialFlag.activate();
			self.ammotypes.smoketrailFlag.activate();
			self.ammotypes.firetrailFlag.activate();
			self.ammotypes.shotAnimation.activate();
			self.ammotypes.spreadpattern.activate();
	
			self.ammotypes.index.set_value(&format!("{}", item.uiIndex));
			self.ammotypes.name.set_value(&format!("{}", item.name));
			self.ammotypes.rgb = (item.red, item.green, item.blue);
			self.colorbox.set_color(Color::from_rgb(item.red, item.green, item.blue));
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
	}

	fn updateCaliber(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		self.ammostrings.index.deactivate();
		self.ammostrings.caliber.deactivate();
		self.ammostrings.brcaliber.deactivate();
		self.ammostrings.nwsscaliber.deactivate();

		if let Some(item) = xmldata.getAmmoString(uiIndex)
		{
			self.ammostrings.index.activate();
			self.ammostrings.caliber.activate();
			self.ammostrings.brcaliber.activate();
			self.ammostrings.nwsscaliber.activate();
				
			self.ammostrings.index.set_value(&format!("{}", item.uiIndex));
			self.ammostrings.caliber.set_value(&format!("{}", item.AmmoCaliber));
			self.ammostrings.brcaliber.set_value(&format!("{}", item.BRCaliber));
			self.ammostrings.nwsscaliber.set_value(&format!("{}", item.NWSSCaliber));
		}
	}

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
			let itemclass = item.usItemClass;
			let classIndex = item.ubClassIndex;

			if itemclass == JAxml::ItemClass::Ammo as u32
			{
				if let Some(mag) = xmldata.getMagazine_mut(classIndex as u32)
				{
					let widget = &mut self.caliber;
					if widget.triggered()
					{
						mag.ubCalibre = widget.value() as u8;
						s.send(Message::Update);
					}
					let widget = &mut self.ammotype;
					if widget.triggered()
					{
						mag.ubAmmoType = widget.value() as u8;
						s.send(Message::Update);
					}
					let widget = &mut self.magtype;
					if widget.triggered()
					{
						mag.ubMagType = widget.value() as u8;
						s.send(Message::Update);
					}

					if let Some(value) = u16IntInput(&mut self.magsize, s) { mag.ubMagSize = value; }
				
					
					let calibreIdx = mag.ubCalibre as u32;
					let ammoTypeIdx = mag.ubAmmoType as u32;
					
					self.pollcaliber(xmldata, calibreIdx, s);
					self.pollAmmoType(xmldata, ammoTypeIdx, s);
				}
			}
		}
	}

	fn pollcaliber(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getAmmoString_mut(uiIndex)
		{
			if let Some(value) = u32IntInput(&mut self.ammostrings.index, s) 
			{
				// TODO
				// This requires special handling to keep references between ammotypes, magazines, items & calibers intact.
				// item.uiIndex = value; 
			}

			if let Some(text) = stringFromInput(&mut self.ammostrings.caliber, s, 20) { item.AmmoCaliber = text; }
			if let Some(text) = stringFromInput(&mut self.ammostrings.brcaliber, s, 20) { item.BRCaliber = text; }
			if let Some(text) = stringFromInput(&mut self.ammostrings.nwsscaliber, s, 20) { item.NWSSCaliber = text; }
		}
	}

	fn pollAmmoType(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getAmmoType_mut(uiIndex)
		{
			if let Some(value) = u32IntInput(&mut self.ammotypes.index, s) 
			{
				// TODO
				// This requires special handling to keep references between ammotypes, magazines, items & calibers intact.
				// item.uiIndex = value; 
			}
			if let Some(text) = stringFromInput(&mut self.ammotypes.name, s, 80) { item.name = text; }

			if self.color.triggered()
			{
				self.changeColor();
				(item.red, item.green, item.blue) = self.ammotypes.rgb;
				s.send(Message::Redraw);
			}
			if let Some(value) = u16IntInput(&mut self.ammotypes.nbullets, s) { item.numberOfBullets = value; }
			if let Some(text) = stringFromInput(&mut self.ammotypes.shotAnimation, s, 100) { item.shotAnimation = text; }
			let widget = &mut self.ammotypes.explosionsize;
			if widget.triggered()
			{
				item.explosionSize = widget.value() as u8;
				s.send(Message::Update);
			}

			if let Some(value) = u8IntInput(&mut self.ammotypes.structImpactMultiplier, s) { item.structureImpactReductionMultiplier = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.structImpactDivisor, s) { item.structureImpactReductionDivisor = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.armorImpactMultiplier, s) { item.armourImpactReductionMultiplier = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.armorImpactDivisor, s) { item.armourImpactReductionDivisor = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.beforeArmorMultpilier, s) { item.beforeArmourDamageMultiplier = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.beforeArmorDivisor, s) { item.beforeArmourDamageDivisor = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.afterArmorMultiplier, s) { item.afterArmourDamageMultiplier = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.afterArmorDivisor, s) { item.afterArmourDamageDivisor = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.bulletsMultiplier, s) { item.multipleBulletDamageMultiplier = value; }
			if let Some(value) = u8IntInput(&mut self.ammotypes.bulletsDivisor, s) { item.multipleBulletDamageDivisor = value; }
			
			let widget = &mut self.ammotypes.acidic;
			if widget.triggered() { item.acidic = widget.value(); }
			let widget = &mut self.ammotypes.dart;
			if widget.triggered() { item.dart = widget.value(); }
			let widget = &mut self.ammotypes.standardissue;
			if widget.triggered() { item.standardIssue = widget.value(); }
			let widget = &mut self.ammotypes.knife;
			if widget.triggered() { item.knife = widget.value(); }
			let widget = &mut self.ammotypes.ignorearmor;
			if widget.triggered() { item.ignoreArmour = widget.value(); }
			let widget = &mut self.ammotypes.tracer;
			if widget.triggered() { item.tracerEffect = widget.value(); }
			let widget = &mut self.ammotypes.zeromindamage;
			if widget.triggered() { item.zeroMinimumDamage = widget.value(); }
			let widget = &mut self.ammotypes.monsterspit;
			if widget.triggered() { item.monsterSpit = widget.value(); }

			if let Some(value) = f32FloatInput(&mut self.ammotypes.healthModifier, s) { item.dDamageModifierLife = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.breathModifier, s) { item.dDamageModifierBreath = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.tankModifier, s) { item.dDamageModifierTank = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.armoredVehicleModifier, s) { item.dDamageModifierArmouredVehicle = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.civilianVehicleModifier, s) { item.dDamageModifierCivilianVehicle = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.zombieModifier, s) { item.dDamageModifierZombie = value; }
			if let Some(value) = u16IntInput(&mut self.ammotypes.lockModifier, s) { item.lockBustingPower = value; }
			if let Some(value) = u16IntInput(&mut self.ammotypes.pierceModifier, s) { item.usPiercePersonChanceModifier = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.temperatureModifier, s) { item.temperatureModificator = value; }
			if let Some(value) = f32FloatInput(&mut self.ammotypes.dirtModifier, s) { item.dirtModificator = value; }

			let flags = item.ammoflag;
			let widget = &mut self.ammotypes.freezingFlag;
			if widget.triggered() 
			{ 
				let value =  widget.value() as u8;
				if let Some(value) =  set_bit_at(flags, 0, value ) { item.ammoflag = value; } 
			}
			let widget = &mut self.ammotypes.blindingFlag;
			if widget.triggered() 
			{ 
				let value =  widget.value() as u8;
				if let Some(value) =  set_bit_at(flags, 1, value ) { item.ammoflag = value; } 
			}
			let widget = &mut self.ammotypes.antimaterialFlag;
			if widget.triggered() 
			{ 
				let value =  widget.value() as u8;
				if let Some(value) =  set_bit_at(flags, 2, value ) { item.ammoflag = value; } 
			}
			let widget = &mut self.ammotypes.smoketrailFlag;
			if widget.triggered() 
			{ 
				let value =  widget.value() as u8;
				if let Some(value) =  set_bit_at(flags, 3, value ) { item.ammoflag = value; } 
			}
			let widget = &mut self.ammotypes.firetrailFlag;
			if widget.triggered() 
			{ 
				let value =  widget.value() as u8;
				if let Some(value) =  set_bit_at(flags,4, value ) { item.ammoflag = value; } 
			}
		}


		// Does the job, but should probably be written more clearly
		let widget = &mut self.ammotypes.explosionid;
		if widget.triggered()
		{
			let idx = widget.value();
			if idx == 0
			{
				let item = xmldata.getAmmoType_mut(uiIndex as u32).unwrap();
				item.highExplosive = 0; 
			}
			else if let Some(menuitem) = widget.at(idx)
			{
				let label = menuitem.label().unwrap();
				let expIdx = xmldata.findIndexbyName(&label);
				match expIdx
				{
					Some(expIdx) => 
					{
						let item = xmldata.getAmmoType_mut(uiIndex as u32).unwrap();
						item.highExplosive = expIdx;
					}
					None => {}
				}
			}
		}

		let widget = &mut self.ammotypes.spreadpattern;
		if widget.triggered()
		{
			let idx = widget.value();
			if idx <= 0
			{
				let item = xmldata.getAmmoType_mut(uiIndex as u32).unwrap();
				item.spreadPattern.clear();
			}
			else if let Some(menuitem) = widget.at( widget.value() )
			{
				let label = menuitem.label().unwrap();
				if !label.is_empty()
				{
					let item = xmldata.getAmmoType_mut(uiIndex as u32).unwrap();
					item.spreadPattern = label;
				}
			}
		}
	}
}


struct ExplosivesArea
{
	// Bomb/Grenade
	explosionType: Listener<Choice>,
	animID: Listener<Choice>,
	damage: IntInput,
	startRadius: IntInput,
	endRadius: IntInput,
	duration: IntInput,
	volatility: IntInput,
	stundamage: IntInput,
	volume: IntInput,
	magsize: IntInput,
	fragmentType: Listener<Choice>,
	fragments: IntInput,
	fragrange: IntInput,
	fragdamage: IntInput,
	indoormodifier: FloatInput,
	horizontaldegrees: IntInput,
	verticaldegrees: IntInput,
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
		let damage = IntInput::default().with_size(width, height).with_label("Damage");
		let startRadius = IntInput::default().with_size(width, height).with_label("Start Radius");
		let duration = IntInput::default().with_size(width, height).with_label("Duration");
		let volatility = IntInput::default().with_size(width, height).with_label("Volatility");
		flex.end();

		let mut flex = Pack::new(flex.x()+150, flex.y(), 35, 100, None);
		flex.set_spacing(5);
		let stundamage = IntInput::default().with_size(width, height).with_label("Stun Damage");
		let endRadius = IntInput::default().with_size(width, height).with_label("End Radius");
		let volume = IntInput::default().with_size(width, height).with_label("Volume");
		let magsize = IntInput::default().with_size(width, height).with_label("Mag Size");
		flex.end();


		let fragmentType: Listener<_> = Choice::default().with_size(width, height).with_pos(x+100, flex.y()+flex.h()).with_label("Frag Type").into();

		let mut flex = Pack::new(fragmentType.x(), fragmentType.y()+fragmentType.h()+10, 35, 80, None);
		flex.set_spacing(5);
		let fragments = IntInput::default().with_size(width, height).with_label("# of Fragments");
		let fragrange = IntInput::default().with_size(width, height).with_label("Frag Range");
		let horizontaldegrees = IntInput::default().with_size(width, height).with_label("Horiz. Degrees");
		flex.end();

		let mut flex = Pack::new(flex.x()+150, flex.y(), 35, 80, None);
		flex.set_spacing(5);
		let fragdamage = IntInput::default().with_size(width, height).with_label("Frag Damage");
		let indoormodifier = FloatInput::default().with_size(width, height).with_label("Indoor Mod.");
		let verticaldegrees = IntInput::default().with_size(width, height).with_label("Vert. Degrees");
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

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		let item = &xmldata.items.items[uiIndex as usize];
		let itemclass = item.usItemClass;
		let classIndex = item.ubClassIndex;

		self.explosionType.deactivate();
		self.animID.deactivate();
		self.damage.deactivate();
		self.startRadius.deactivate();
		self.endRadius.deactivate();
		self.duration.deactivate();
		self.volatility.deactivate();
		self.stundamage.deactivate();
		self.volume.deactivate();
		self.magsize.deactivate();
		self.fragmentType.deactivate();
		self.fragments.deactivate();
		self.fragrange.deactivate();
		self.fragdamage.deactivate();
		self.indoormodifier.deactivate();
		self.horizontaldegrees.deactivate();
		self.verticaldegrees.deactivate();
		self.explodeOnImpact.deactivate();
		self.launcherType.deactivate();
		self.discardeditem.deactivate();

		use JAxml::ItemClass::*;
		match itemclass
		{
			x if x == Grenade as u32 || x == Bomb as u32 =>
			{
				if let Some(explosive) = xmldata.getExplosive(classIndex as u32)
				{
					self.explosionType.activate();
					self.animID.activate();
					self.damage.activate();
					self.startRadius.activate();
					self.endRadius.activate();
					self.duration.activate();
					self.volatility.activate();
					self.stundamage.activate();
					self.volume.activate();
					self.magsize.activate();
					self.fragmentType.activate();
					self.fragments.activate();
					self.fragrange.activate();
					self.fragdamage.activate();
					self.indoormodifier.activate();
					self.horizontaldegrees.activate();
					self.verticaldegrees.activate();
					self.explodeOnImpact.activate();
			
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
			}
			x if x == Launcher as u32 =>
			{
				self.launcherType.activate();

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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem(uiIndex)
		{
			let itemclass = item.usItemClass;
			let classIndex = item.ubClassIndex as u32;

			use JAxml::ItemClass::*;
			match itemclass
			{
				x if x == Grenade as u32 || x == Bomb as u32 =>
				{
					if let Some(explosive) = xmldata.getExplosive_mut(classIndex)
					{
						let widget = &mut self.explosionType;
						if widget.triggered() { explosive.ubType = widget.value() as u32; }

						let widget = &mut self.animID;
						if widget.triggered() { explosive.ubAnimationID = widget.value() as u32; }

						let widget = &mut self.fragmentType;
						if widget.triggered() { explosive.ubFragType = widget.value() as u32; }

						let widget = &mut self.explodeOnImpact;
						if widget.triggered() { explosive.fExplodeOnImpact = widget.value(); }

						if let Some(value) = u32IntInput(&mut self.damage, s) { explosive.ubDamage = value; }
						if let Some(value) = u32IntInput(&mut self.startRadius, s) { explosive.ubStartRadius = value; }
						if let Some(value) = u32IntInput(&mut self.endRadius, s) { explosive.ubRadius = value; }

						if let Some(value) = u32IntInput(&mut self.duration, s) { explosive.ubDuration = value; }
						if let Some(value) = u32IntInput(&mut self.volatility, s) { explosive.ubVolatility = value; }
						if let Some(value) = u32IntInput(&mut self.stundamage, s) { explosive.ubStunDamage = value; }
						if let Some(value) = u32IntInput(&mut self.volume, s) { explosive.ubVolume = value; }
						if let Some(value) = u32IntInput(&mut self.magsize, s) { explosive.ubMagSize = value; }
						if let Some(value) = u32IntInput(&mut self.fragments, s) { explosive.usNumFragments = value; }
						if let Some(value) = u32IntInput(&mut self.fragrange, s) { explosive.ubFragRange = value; }
						if let Some(value) = u32IntInput(&mut self.fragdamage, s) { explosive.ubFragDamage = value; }
						if let Some(value) = f32FloatInput(&mut self.indoormodifier, s) { explosive.bIndoorModifier = value; }
						if let Some(value) = u32IntInput(&mut self.horizontaldegrees, s) { explosive.ubHorizontalDegree = value; }
						if let Some(value) = u32IntInput(&mut self.verticaldegrees, s) { explosive.ubVerticalDegree = value; }
					}
				}
				x if x == Launcher as u32 =>
				{
					let widget = &mut self.launcherType;
					if widget.triggered()
					{
						// This is working on the assumption that only one of these should be active
						// If so, these really need to be an enum in the 1.13 source code instead of bunch bools
						if let Some(item) = xmldata.getItem_mut(uiIndex)
						{
							item.grenadelauncher = false;
							item.rocketlauncher = false;
							item.singleshotrocketlauncher = false;
							item.mortar = false;
							item.cannon = false;

							let value = widget.value();
							match value
							{
								0 =>
								{
									// N/A option
									// for now leave all to false.
									// Not sure if it's the correct approach. Might have to fix this in the future
								}
								1 => { item.grenadelauncher = true; }
								2 => { item.rocketlauncher = true; }
								3 => { item.singleshotrocketlauncher = true; }
								4 => { item.mortar = true; }
								5 => { item.cannon = true; }
								_ => {}
							}
							s.send(Message::Update);
						}
					}

					let widget = &mut self.discardeditem;
					if widget.triggered()
					{
						let idx = widget.value();
						if idx <= 0
						{
							if let Some(item) = xmldata.getItem_mut(uiIndex)
							{
								item.discardedlauncheritem = 0;
							}
						}
						else if let Some(menuitem) = widget.at( widget.value() )
						{
							let label = menuitem.label().unwrap();
							if !label.is_empty()
							{
								if let Some(itemIndex) = xmldata.findIndexbyName(&label)
								{
									if let Some(item) = xmldata.getItem_mut(uiIndex)
									{
										item.discardedlauncheritem = itemIndex as u16;
									}
								}
							}
						}
					}
				}
				_ => {}
			}
		}
	}
}


struct SoundsArea
{
	attackVolume: IntInput,
	hitVolume: IntInput,
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
		let attackVolume = IntInput::default().with_size(width, height).with_label("Attack Volume");
		let hitVolume = IntInput::default().with_size(width, height).with_label("Hit Volume");
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


	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		self.attackVolume.deactivate();
		self.hitVolume.deactivate();
		self.attack.deactivate();
		self.burst.deactivate();
		self.silenced.deactivate();
		self.silencedBurst.deactivate();
		self.reload.deactivate();
		self.locknload.deactivate();
		self.manualreload.deactivate();
	

		let item = &xmldata.items.items[uiIndex as usize];
		let itemclass = item.usItemClass;

		use JAxml::ItemClass::*;
		match itemclass
		{
			x if x == Gun as u32 || x == Launcher as u32 || x == Punch as u32 =>
			{
				if let Some(weapon) = &xmldata.getWeapon(uiIndex)
				{
					self.attackVolume.activate();
					self.hitVolume.activate();
					self.attack.activate();
					self.burst.activate();
					self.silenced.activate();
					self.silencedBurst.activate();
					self.reload.activate();
					self.locknload.activate();
					self.manualreload.activate();

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

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			let itemclass = item.usItemClass;
			let classIndex = item.ubClassIndex;

			use JAxml::ItemClass::*;
			match itemclass
			{
				x if x == Gun as u32 || x == Launcher as u32 || x == Punch as u32 =>
				{
					if let Some(weapon) = xmldata.getWeapon_mut(uiIndex)
					{
						if let Some(value) = u8IntInput(&mut self.attackVolume, s) { weapon.ubAttackVolume = value; }
						if let Some(value) = u8IntInput(&mut self.hitVolume, s) { weapon.ubHitVolume = value; }

						let widget = &mut self.attack;
						if widget.triggered() { weapon.sSound = widget.value() as u16; }
						let widget = &mut self.silenced;
						if widget.triggered() { weapon.silencedSound = widget.value() as u16; }
						let widget = &mut self.reload;
						if widget.triggered() { weapon.sReloadSound = widget.value() as u16; }
						let widget = &mut self.locknload;
						if widget.triggered() { weapon.sLocknLoadSound = widget.value() as u16; }
						let widget = &mut self.manualreload;
						if widget.triggered() { weapon.ManualReloadSound = widget.value() as u16; }
						let widget = &mut self.burst;
						if widget.triggered() { weapon.sBurstSound = widget.value() as u16; }
						let widget = &mut self.silencedBurst;
						if widget.triggered() { weapon.sSilencedBurstSound = widget.value() as u16; }
					}	
				}
				_ => {}
			}
		}
	}
}


struct ArmorArea
{
	index: Input,
	class: Listener<Choice>,
	protection: IntInput,
	coverage: IntInput,
	degrade: IntInput,
	flakjacket: Listener<CheckButton>,
	leatherjacket: Listener<CheckButton>,
}
impl ArmorArea
{
	fn initialize(x: i32, y: i32, s: &app::Sender<Message>) -> ArmorArea
	{
		let mainWidth = 210; let mainHeight = 185;

		// Main framed box. Everything else is located relative to this
		let (frame, _) = createBox(
			x, y,
			mainWidth, mainHeight,
			120, 80, "Armor"
		);

		let mut flex = Pack::new(x+80, y+10, 120, 100, None);
		flex.set_spacing(5);
		let index = Input::default().with_size(40, 20).with_label("Index");
		let mut class: Listener<_> = Choice::default().with_size(100, 20).with_label("Class").into();
		let protection = IntInput::default().with_size(40, 20).with_label("Protection");
		let coverage = IntInput::default().with_size(40, 20).with_label("Coverage");
		let degrade = IntInput::default().with_size(40, 20).with_label("Degrade %");
		let flakjacket: Listener<_>= CheckButton::default().with_size(40, 20).with_label("Flakjacket").into();
		let leatherjacket: Listener<_>= CheckButton::default().with_size(40, 20).with_label("Leatherjacket").into();
		flex.end();

		// Needs to match JAxml enum ArmorClass
		class.add_choice("Helmet|Vest|Leggings|Plate|Monster|Vehicle");

		return ArmorArea{ class, coverage, degrade, index, protection, flakjacket, leatherjacket };
	}

	fn update(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		self.index.deactivate();
		self.class.deactivate();
		self.protection.deactivate();
		self.coverage.deactivate();
		self.degrade.deactivate();

		let item = &xmldata.items.items[uiIndex as usize];
		let itemclass = item.usItemClass;
		let classIndex = item.ubClassIndex;

		self.flakjacket.set_value(item.flakjacket);
		self.leatherjacket.set_value(item.leatherjacket);

		use JAxml::ItemClass::*;
		match itemclass
		{
			x if x == Armor as u32 =>
			{
				if let Some(armor) =  &xmldata.getArmor(classIndex as u32)
				{
					self.index.activate();
					self.class.activate();
					self.protection.activate();
					self.coverage.activate();
					self.degrade.activate();
					
					self.index.set_value(&format!("{}", armor.uiIndex));
					self.class.set_value(armor.ubArmourClass as i32);
					self.protection.set_value(&format!("{}", armor.ubProtection));
					self.coverage.set_value(&format!("{}", armor.ubCoverage));
					self.degrade.set_value(&format!("{}", armor.ubDegradePercent));
				}
			}
			_ => {}
		}
	}

	fn updateFromArmorData(&mut self, xmldata: &JAxml::Data, uiIndex: u32)
	{
		self.index.deactivate();
		self.class.deactivate();
		self.protection.deactivate();
		self.coverage.deactivate();
		self.degrade.deactivate();

		if let Some(armor) =  &xmldata.getArmor(uiIndex)
		{
			self.index.activate();
			self.class.activate();
			self.protection.activate();
			self.coverage.activate();
			self.degrade.activate();

			self.index.set_value(&format!("{}", armor.uiIndex));
			self.class.set_value(armor.ubArmourClass as i32);
			self.protection.set_value(&format!("{}", armor.ubProtection));
			self.coverage.set_value(&format!("{}", armor.ubCoverage));
			self.degrade.set_value(&format!("{}", armor.ubDegradePercent));
		}
	}

	fn poll(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(item) = xmldata.getItem_mut(uiIndex)
		{
			let itemclass = item.usItemClass;
			let classIndex = item.ubClassIndex;

			if self.flakjacket.triggered() { item.flakjacket = self.flakjacket.value(); }
			if self.leatherjacket.triggered() { item.leatherjacket = self.leatherjacket.value(); }

			use JAxml::ItemClass::*;
			match itemclass
			{
				x if x == Armor as u32 =>
				{
					if let Some(armor) =  xmldata.getArmor_mut(classIndex as u32)
					{
						// self.index.set_value(&format!("{}", armor.uiIndex));
						if self.class.triggered() { armor.ubArmourClass = self.class.value() as u8; }

						if let Some(value) = u8IntInput(&mut self.protection, s) { armor.ubProtection = value; }
						if let Some(value) = u8IntInput(&mut self.coverage, s) { armor.ubCoverage = value; }
						if let Some(value) = u8IntInput(&mut self.degrade, s) { armor.ubDegradePercent = value; }
					}
				}
				_ => {}
			}
		}
	}

	fn pollFromArmorData(&mut self, xmldata: &mut JAxml::Data, uiIndex: u32, s: &app::Sender<Message>)
	{
		if let Some(armor) =  xmldata.getArmor_mut(uiIndex)
		{
			// self.index.set_value(&format!("{}", armor.uiIndex));
			if self.class.triggered() { armor.ubArmourClass = self.class.value() as u8; }

			if let Some(value) = u8IntInput(&mut self.protection, s) { armor.ubProtection = value; }
			if let Some(value) = u8IntInput(&mut self.coverage, s) { armor.ubCoverage = value; }
			if let Some(value) = u8IntInput(&mut self.degrade, s) { armor.ubDegradePercent = value; }
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


fn stringFromInput(widget: &mut Input, s: &app::Sender<Message>, maxLength: usize) -> Option<String>
{
	if widget.changed()
	{
		let text = &widget.value();

		if text.len() <= maxLength
		{
			widget.set_text_color(Color::Black);
			s.send(Message::Update);
			return Some(text.clone());
		}
		else
		{
			widget.set_text_color(Color::Red);
			s.send(Message::Update);
		}
	}

	return None;
}

fn stringFromMultiLineInput(widget: &mut MultilineInput, s: &app::Sender<Message>, maxLength: usize) -> Option<String>
{
	if widget.changed()
	{
		let text = &widget.value();

		if text.len() <= maxLength
		{
			widget.set_text_color(Color::Black);
			s.send(Message::Update);
			return Some(text.clone());
		}
		else
		{
			widget.set_text_color(Color::Red);
			s.send(Message::Update);
		}
	}

	return None;
}

fn f32FloatInput(widget: &mut FloatInput, s: &app::Sender<Message>) -> Option<f32>
{
	if widget.changed()
	{
		let value = widget.value().parse::<f32>();
		match value
		{
			Ok(value) => 
			{
				widget.set_text_color(Color::Black);
				s.send(Message::Update);
				return Some(value);
			}
			_ => 
			{ 
				widget.set_text_color(Color::Red); 
				s.send(Message::Redraw);
				return None;
			}
		}
	}
	else { return None; }
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
	Update,
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
	ShowArmorData,
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
	Clothes,
	Armors,
}


//---------------------------------------------------------------------------------------------------------------------
// Macros
//---------------------------------------------------------------------------------------------------------------------
macro_rules! IntInputs {
	($($name:ident, $type:ty),*) => {
		
		$(fn $name(widget: &mut IntInput, s: &app::Sender<Message>) -> Option<$type>
		{
			if widget.changed()
			{
				let value = widget.value().parse::<$type>();
				match value
				{
					Ok(value) => 
					{
						widget.set_text_color(Color::Black);
						s.send(Message::Update);
						return Some(value);
					}
					_ => 
					{ 
						widget.set_text_color(Color::Red); 
						s.send(Message::Redraw);
						return None;
					}
				}
			}
			else { return None; }
		})*
	};
}

IntInputs!(u16IntInput, u16, u8IntInput, u8, i8IntInput, i8, i16IntInput, i16, u32IntInput, u32);