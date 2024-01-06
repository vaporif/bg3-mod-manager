use std::path::PathBuf;

use crate::mods::ZippedMod;
use crate::prelude::*;

#[instrument]
#[tauri::command]
pub fn files_dropped(file_paths: Vec<String>) -> String {
    for file in file_paths {
        let path: PathBuf = file.into();
        let file = ZippedMod::from_file(path);
        info!("{:?}", file);
    }

    "r".into()
}
