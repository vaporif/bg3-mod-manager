use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use specta::Type;

#[cfg(target_os = "macos")]
lazy_static! {
    pub static ref DEFAULT_GAME_PATH: PathBuf = {
        home::home_dir()
            .expect("home_dir set")
            .join("/Documents/Larian Studios/Baldur's Gate 3")
    };
}
#[cfg(target_os = "linux")]
lazy_static! {
    pub static ref DEFAULT_GAME_PATH: PathBuf = { todo!() };
}
#[cfg(target_os = "windows")]
lazy_static! {
    pub static ref DEFAULT_GAME_PATH: PathBuf = { todo!() };
}

#[derive(Deserialize, Serialize, Type)]
pub struct Settings {
    pub game_data_path: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_data_path: Some(DEFAULT_GAME_PATH.to_path_buf()),
        }
    }
}

// pub fn modsettings_path(game_data_path: &Option<PathBuf>) -> Option<PathBuf> {
//     game_data_path.and_then(|p| Some(p.join("/PlayerProfiles/Public/modsettings.lsx")))
// }
