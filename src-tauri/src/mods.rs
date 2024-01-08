use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Context};

use quick_xml::de;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::prelude::*;

mod json;
mod xml;
pub use json::ModInfo;

#[derive(Debug)]
pub struct ZippedMod {
    pub path: PathBuf,
    pub info: ModInfo,
    pub md5: String
}

#[derive(Serialize, Deserialize, Type)]
pub struct InstalledMod {
    pub author: String,
    pub name: String,
    pub folder: String,
    pub version: Option<String>,
    pub description: String,
    pub uuid: String,
    pub created: String,
    pub dependencies: Vec<String>,
    pub group: String,
    pub md5: String,
    pub order: u8,
    pub is_enabled: bool
}

impl From<ZippedMod> for InstalledMod {
    fn from(value: ZippedMod) -> Self {
        InstalledMod {
            author: value.info.author,
            name: value.info.name,
            folder: value.info.folder,
            version: value.info.version,
            description: value.info.description,
            uuid: value.info.uuid,
            created: value.info.created,
            dependencies: value.info.dependencies,
            group: value.info.group,
            md5: value.md5,
            order: 0, 
            is_enabled: false
        }
    }
}

impl ZippedMod {
    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let file = std::fs::File::open(&path)?;

        let mut zip = zip::read::ZipArchive::new(file).context("reading mod zip file")?;

        let info_json = zip
            .by_name("info.json")
            .context("looking for info.json file in zip archive")?;

        let mut mods_list: crate::mods::json::Mods =
            serde_json::from_reader(info_json).context("parsing info_json")?;

        if mods_list.mods.len() != 1 {
            bail!("Only single file per zip supported");
        }

        let info = mods_list
            .mods
            .drain(..)
            .next()
            .ok_or_else(|| anyhow::anyhow!("No elements in mods list"))?;

        Ok(Self { path, info, md5: mods_list.md5 })
    }

    pub fn unzip(self, path: PathBuf) -> anyhow::Result<()> {
        let file = std::fs::File::open(&path).context("open zip file")?;
        let mut zip = zip::read::ZipArchive::new(file).context("should be zip archive")?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {i} comment: {comment}");
                }
            }

            if (*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                info!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath)
                    .with_context(|| format!("creating file from zip {}", outpath.display()))?;
                std::io::copy(&mut file, &mut outfile)
                    .with_context(|| format!("copying mod zip file {}", outpath.display()))?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
        Ok(())
    }
}

pub struct ModSettingFile {
    path: PathBuf,
    xml: xml::Save,
    mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Type)]
pub struct Mod {
    uuid: String,
    folder: String,
    md5: String,
    name: String,
    // TODO: is it u64??
    version64: u32,
    is_disabled: bool,
    order: u16,
}

impl ModSettingFile {
    pub async fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let xml_string = fs::read_to_string(&path).context("file read")?;
        let xml = de::from_str(&xml_string).context("xml deserialized")?;
        let mods = Self::parse_mods(&xml);
        Ok(Self { path, xml, mods })
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

    pub fn mods(&self) -> &Vec<Mod> {
        &self.mods
    }
}

#[cfg(test)]
mod tests {}
