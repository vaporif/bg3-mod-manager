use parking_lot::RwLock;
use serde::Serialize;
use tauri::{Builder, Wry};
use tokio::sync::mpsc::Sender;
use crate::mods::ModInfo;
use crate::prelude::*;

use crate::{mods::Mod, settings::Settings};

#[derive(Debug, Serialize, Clone, specta::Type)]
pub enum Event {
    SettingsUpdated(Settings),
    ModfilesParsed(Vec<ModInfo>),
    ModsUpdated,
}
impl Event {
    pub fn event_name(&self) -> &str {
        match self {
            Event::SettingsUpdated(_) => "SettingsUpdated",
            Event::ModfilesParsed(_) => "ModfilesParsed",
            Event::ModsUpdated => "ModsUpdated",
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
        self.settings.write().game_data_path = new.game_data_path;

        // TODO: Proper error
        self.send_events_tx
            .send(Event::SettingsUpdated(self.settings.read().clone())).await.map_err(|e|{
            Error::Other(e.to_string())
        })?;
        Ok(())
    }
}
