// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
pub mod error;
mod mods;
pub mod prelude;
pub mod settings;
mod state;

use commands::register_command_handlers;
use log::error;
use state::load_state;
use tauri::{CustomMenuItem, Menu, Submenu, Manager};
use tauri_plugin_log::LogTarget;

const QUIT_MENU_EVENT: &str = "quit";
const ADD_MOD_MENU_EVENT: &str = "add_mod";

fn main() {
    let (send_event_tx, mut emit_event_rx) = 
        tokio::sync::mpsc::channel::<crate::state::Event>(1);

    let app = tauri::Builder::default();
    let app = register_command_handlers(app);
    let app = load_state(app, send_event_tx);

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
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(event) = emit_event_rx.recv().await {
                        if let Err(e) = app_handle.emit_all(event.event_name(), &event) {
                            error!("emit event {event:?} failed, error: {e}");
                        }
                    }
                }
            });

            Ok(())
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
