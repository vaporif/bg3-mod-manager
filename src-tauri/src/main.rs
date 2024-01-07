// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
pub mod error;
mod mods;
pub mod prelude;
pub mod settings;
mod state;

use commands::register_command_handlers;
use state::load_state;
use tauri::{CustomMenuItem, Menu, Submenu};
use tauri_plugin_log::LogTarget;

const QUIT_MENU_EVENT: &str = "quit";
const ADD_MOD_MENU_EVENT: &str = "add_mod";

fn main() {
    let app = tauri::Builder::default();
    let app = register_command_handlers(app);
    let app = load_state(app);

    app.menu(menu())
        .on_menu_event(|event| match event.menu_item_id() {
            QUIT_MENU_EVENT => {
                std::process::exit(0);
            }
            ADD_MOD_MENU_EVENT => {
                todo!()
            }
            _ => {}
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn menu() -> Menu {
    let quit = CustomMenuItem::new(QUIT_MENU_EVENT.to_string(), "Quit");
    let add_mod = CustomMenuItem::new(ADD_MOD_MENU_EVENT.to_string(), "Add Mod");
    let submenu = Submenu::new("File", Menu::new().add_item(add_mod).add_item(quit));
    Menu::new().add_submenu(submenu)
}
