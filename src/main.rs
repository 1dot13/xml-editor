#![allow(non_snake_case)]
#![allow(unused)]
use std::env::current_dir;
// use std::io::{BufReader, Write, Read};
// use std::fs::{File, read};
// use std::fmt;
// use std::str;
use std::path::PathBuf;
use std::time::{Instant};
use fltk::enums::Color;
use fltk::group::{Tabs, Group};
use fltk::menu::{MenuFlag, SysMenuBar, Choice};
use fltk::valuator::Scrollbar;
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
// Allow editing item info
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
	let bigw = 104; let bigh = 74;
	let medw = 74; let medh = 74;
	let smallw = 34; let smallh = 34;
	
	let tab1 = Group::default().with_size(w, h).below_of(&tabs, 0).with_label("Tab1\t\t");
	// Item Graphics section
	let _ = Frame::default().with_size(300, 450).with_pos(5, 25).set_frame(FrameType::EngravedBox);
	let _ = Frame::default().with_size(60, 20).with_pos(130, 15).with_label("Graphics").set_frame(FrameType::FlatBox);
	
	let mut bigimage = Frame::default().with_size(bigw, bigh).with_pos(10, 50);
    bigimage.set_frame(FrameType::EngravedBox);
	let mut medimage = Frame::default().with_size(medw, medh).below_of(&bigimage, 20);
    medimage.set_frame(FrameType::EngravedBox);
	let mut smallimage = Frame::default().with_size(smallw, smallh).below_of(&medimage, 20);
    smallimage.set_frame(FrameType::EngravedBox);
    
	let _ = Frame::default().with_size(20, 20).with_pos(32, bigimage.y() - 20).with_label("Big Image");
	let _ = Frame::default().with_size(20, 20).with_pos(50, medimage.y() - 20).with_label("Inventory Image");
	let _ = Frame::default().with_size(20, 20).with_pos(42, smallimage.y() - 20).with_label("Ground Image");
	let _ = Frame::default().with_size(20, 20).with_pos(42, smallimage.y() + smallimage.h() + 5).with_label("Graphic Type");
	let _ = Frame::default().with_size(20, 20).with_pos(42, smallimage.y() + smallimage.h() + 50).with_label("Graphic Index");
    
    let mut graphType: Listener<_> = Choice::default().with_pos(10, smallimage.y() + smallimage.h() + 25).with_size(100, 20).into();
    let mut graphChooser = GraphicChooser::new(150, 40, 150, 420, &s, Message::GraphicScroll);
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
                
                let graphicType = xmldata.items.items[uiIndex as usize].ubGraphicType as usize;
                let stiIndex = xmldata.items.items[uiIndex as usize].ubGraphicNum as usize;
                println!("Graphic index {}", stiIndex);
                if graphicType < images.big.len() && stiIndex < images.big[graphicType].len()
                {
                		let mut image = images.big[graphicType][stiIndex].clone();
                		image.scale(bigw-4, bigh-4, true, true);
                		bigimage.set_image(Some(image));
                		
                		let mut image = images.med[graphicType][stiIndex].clone();
                		image.scale(medw-4, medh-4, true, true);
                		medimage.set_image(Some(image));
                		
                		let mut image = images.small[graphicType][stiIndex].clone();
                		image.scale(smallw-4, smallh-4, true, true);
                		smallimage.set_image(Some(image));
                		
                		itemWindow.redraw()
                	}
                	else 
                	{
    						        println!("Graphic index out of graphic vector bounds!");
    						        println!("Tried to access image [{}][{}]", graphicType, stiIndex);
																		    }
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
            	    		itemWindow.redraw();
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

struct GraphicChooser
{
	g: group::Group,
	i: Vec<Frame>,
	s: Scrollbar
}
impl GraphicChooser
{
	fn new(x: i32, y: i32, w: i32, h: i32, sender: &app::Sender<Message>, msg: Message) -> GraphicChooser
	{
		let mut g= group::Group::new(x, y, w, h, None);
		g.set_frame(FrameType::FlatBox);
		g.set_color(Color::White);
		
		let mut images = Vec::new();
		let w = 104; let h = 54;
		let padding = 5;
		for i in 0..7
		{
			let mut image = Frame::default().with_size(w, h).with_pos(g.x()+5, g.y()+5 + (h+5)*i).with_label("title");
			image.set_frame(FrameType::BorderBox);
			images.push(image);
		}
		
		let w = 20;
		let mut s = Scrollbar::default().with_pos(g.x() + g.w() - w, g.y()).with_size(w, g.h());
		s.emit(*sender, msg);
		
		g.end();
		GraphicChooser{g, i: images, s}
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
}

    
