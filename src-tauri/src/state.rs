use parking_lot::RwLock;
use tauri::{Builder, Wry};

use crate::{mods::Mod, settings::Settings};

pub fn load_state(builder: Builder<Wry>) -> Builder<Wry> {
    let store = Store::load();

    builder.manage(store)
}

pub struct Store {
    pub settings: RwLock<Settings>,
    pub mods: RwLock<Vec<Mod>>,
}

impl Store {
    fn load() -> Self {
        Self {
            settings: RwLock::new(Settings::default()),
            mods: RwLock::new(Vec::new()),
        }
    }
    pub fn update_settings(&self, new: Settings) {
        self.settings.write().game_data_path = new.game_data_path;
    }
}
