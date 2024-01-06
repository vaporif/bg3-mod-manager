use std::path::PathBuf;

use crate::mods::ZippedMod;
use crate::prelude::*;

#[instrument]
#[tauri::command]
pub fn files_dropped(file_paths: Vec<PathBuf>) -> String {
    for file in file_paths {
        let file = ZippedMod::from_file(file);
        info!("{:?}", file);
    }

    "r".into()
}

#[instrument]
#[tauri::command]
pub fn save_settings(game_data_path: String) {}
