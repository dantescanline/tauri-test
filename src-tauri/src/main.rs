#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{SystemTray, SystemTrayMenu};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
	
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let quit2 = quit.clone();
	let close = CustomMenuItem::new("close".to_string(), "Close");
	let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
	let menu = Menu::new().add_submenu(submenu);

	
	let tray_menu = SystemTrayMenu::new()
	.add_item(quit2);
	let tray = SystemTray::new().with_menu(tray_menu);


    tauri::Builder::default()
        .menu(menu)
				.system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
