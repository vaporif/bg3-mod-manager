use std::fs;
use std::io::Write;
use std::path::{self, PathBuf};

use quick_xml::{de, se};
use serde;

mod json;
mod xml;

pub struct ModSettingFile {
    path: PathBuf,
    xml: xml::Save,
    mods: Vec<Mod>,
}

pub struct Mod {
    uuid: String,
    folder: String,
    md5: String,
    name: String,
    version64: u64,
    is_disabled: bool,
    order: u16,
}

impl ModSettingFile {
    pub fn mods(&self) -> &Vec<Mod> {
        &self.mods
    }

    pub async fn from_path(path: PathBuf) -> Self {
        let xml_string = fs::read_to_string(&path).expect("file read");
        let xml = de::from_str(&xml_string).expect("xml deserialized");
        let mods = Self::parse_mods(&xml);
        Self { path, xml, mods }
    }

    pub async fn save(&self) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("file opened");
        let xml = Self::mods_to_xml(&self.mods);
        let xml = quick_xml::se::to_string(&xml).expect("serialized");
        file.write_all(xml.as_bytes()).expect("saved");
    }

    pub fn upsert(&mut self, mod_to_update: Mod) {
        match self.mods.iter_mut().find(|m| m.uuid == mod_to_update.uuid) {
            Some(mod_info) => {
                if mod_info.version64 != mod_to_update.version64 {
                    panic!("updates not possible yet");
                }

                mod_info.is_disabled = mod_to_update.is_disabled;
                mod_info.order = mod_to_update.order;
            }
            None => {
                self.mods.push(mod_to_update);
            }
        }

        // TODO: emit event
    }

    // TODO: validate
    fn parse_mods(xml: &xml::Save) -> Vec<Mod> {
        let node = &xml.region.node;

        let iter = node.children.nodes.iter().take(2);
        // let mod_order = iter.find(|m| m.id == No)
        todo!()
    }

    fn mods_to_xml(mods: &Vec<Mod>) -> xml::Save {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
