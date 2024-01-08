use std::path::PathBuf;

use parking_lot::RwLock;
use serde::Serialize;
use tauri::{Builder, Wry};
use tokio::sync::mpsc::Sender;
use crate::mods::{ModInfo, ModSettingFile};
use crate::prelude::*;

use crate::{mods::Mod, settings::Settings};

#[derive(Debug, Serialize, Clone, specta::Type)]
pub enum Event {
    SettingsUpdated(Settings),
    // ModfilesParsed(Vec<ModInfo>),
    ModsUpdated,
    Event
}
impl Event {
    pub fn event_name(&self) -> &str {
        match self {
            Event::SettingsUpdated(_) => "SettingsUpdated",
            // Event::ModfilesParsed(_) => "ModfilesParsed",
            Event::ModsUpdated => "ModsUpdated",
            Event::Event => "Event"
        }
    }
}

pub fn load_state(builder: Builder<Wry>, send_events_tx: Sender<Event>) -> Builder<Wry> {
    let store = Store::load(send_events_tx);

    builder.manage(store)
}

pub struct Store {
    pub settings: RwLock<Settings>,
    pub mods: RwLock<Vec<Mod>>,
    send_events_tx: Sender<Event>
}

impl Store {
    fn load(send_events_tx: Sender<Event>) -> Self {
        Self {
            settings: RwLock::new(Settings::default()),
            mods: RwLock::new(Vec::new()),
            send_events_tx
        }
    }
    pub async fn update_settings(&self, new: Settings) -> Result<()> {
        let Some(game_data_path) = new.game_data_path else {
            return Err(Error::Other("path not found".to_string()));
        };

        let modsetting_path = modsettings_path(&game_data_path);

        let modsettingFile = ModSettingFile::from_path(modsetting_path);

        self.settings.write().game_data_path = Some(game_data_path);
        let settings  = self.settings.read().clone(); 

        // TODO: Proper error
        self.send_events_tx
            .send(Event::SettingsUpdated(settings)).await;
        Ok(())
    }

    pub async fn test_notify(&self) {
        self.send_events_tx.send(Event::Event).await;
    }
}

pub fn modsettings_path(game_data_path: &PathBuf) -> PathBuf {
    game_data_path.join("/PlayerProfiles/Public/modsettings.lsx")
}
