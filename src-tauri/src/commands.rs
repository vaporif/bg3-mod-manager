use std::path::PathBuf;

use tauri::{Builder, State, Wry};

use crate::mods::ZippedMod;
use crate::prelude::*;
use crate::settings::Settings;
use crate::state::Store;

pub fn register_command_handlers(builder: Builder<Wry>) -> Builder<Wry> {
    // Specta generating typed binding interfaces
    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![files_dropped, save_settings, get_default_game_data_path],
        "../src/lib/dto.ts",
    )
    .expect("unable to generate specta types");

    builder.invoke_handler(tauri::generate_handler![
        files_dropped,
        save_settings,
        get_default_game_data_path
    ])
}

#[tauri::command]
#[specta::specta]
fn files_dropped(file_paths: Vec<PathBuf>) {
    for file in file_paths {
        let file = ZippedMod::from_file(file);
        info!("{:?}", &file);
    }
}

#[tauri::command]
#[specta::specta]
fn save_settings(settings: Settings, state: State<Store>) {
    state.update_settings(settings);

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
