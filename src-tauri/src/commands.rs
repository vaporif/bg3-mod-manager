use std::path::PathBuf;

use tauri::{Builder, State, Wry};

use crate::mods::{Mod, ZippedMod};
use crate::prelude::*;
use crate::settings::Settings;
use crate::state::{Store, Event};

pub fn register_command_handlers(builder: Builder<Wry>) -> Builder<Wry> {
    // Specta generating typed binding interfaces
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![
            add_mod_files,
            update_mods,
            save_settings,
            get_default_game_data_path,
        ],
        "../src/lib/ipc.ts",
    )
    .expect("unable to generate ts typings for commands");

    // tauri_specta::ts::export(
    //     specta::ts::export::<Event>(&ExportConfiguration::default()).unwrap(), 
    //     "../src/lib/events.ts")
    // .expect("unable to generate ts typings for events");

    builder.invoke_handler(tauri::generate_handler![
        add_mod_files,
        update_mods,
        save_settings,
        get_default_game_data_path
    ])
}

#[tauri::command]
#[specta::specta]
fn add_mod_files(file_paths: Vec<PathBuf>) {
    for file in file_paths {
        let file = ZippedMod::from_file(file);
        info!("{:?}", &file);
    }
}

#[tauri::command]
#[specta::specta]
fn update_mods(mods: Vec<Mod>) {
    todo!()
}

#[tauri::command]
#[specta::specta]
async fn save_settings<'s>(settings: Settings, state: State<'s, Store>) -> Result<()> {
    state.test_notify().await;
    state.update_settings(settings).await;

    Ok(())

    // if !new_path.exists() {
    //     bail!("Path not found");
    // }

    // if !new_path.is_dir() {
    //     bail!("should be directory");
    // }

    // let modsettings_file = crate::settings::modsettings_path(&new_path);

    // settings.game_data_path = Some(new_path);
}

#[tauri::command]
#[specta::specta]
fn get_default_game_data_path() -> String {
    let res = &crate::settings::DEFAULT_GAME_PATH;

    let res = res.to_string_lossy().into_owned();

    info!("{}", res);
    res
}
