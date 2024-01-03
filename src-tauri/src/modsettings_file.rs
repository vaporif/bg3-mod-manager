use std::fs;
use std::path::{self, PathBuf};

use quick_xml::de;
use serde;

pub struct ModSettingFile {
    xml: xml::Save,
    mods: Vec<Mod>,
}

struct Mod {
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
        let xml_string = fs::read_to_string(path).expect("file read");
        let xml = de::from_str(&xml_string).expect("xml deserialized");
        let mods = Self::parse_mods(&xml);
        Self { xml, mods }
    }

    pub async fn save(&self) {
        todo!()
    }

    fn parse_mods(xml: &xml::Save) -> Vec<Mod> {
        todo!()
    }
}

#[allow(unused)]
mod xml {
    use serde::Deserialize;

    pub const ROOT_ID: &str = "root";
    pub const MODS_ID: &str = "Mods";

    pub enum NodeType {
        ModOrder,
        Module,
        Mods,
        ModuleShortDesc,
    }

    pub enum AttributeType {
        Folder,
        MD5,
        Name,
        UUID,
        Version64,
    }

    pub enum AttributeDataType {
        FixedString,
        LSString,
        int64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Save {
        version: Version,
        region: Region,
    }

    #[derive(Debug, Deserialize)]
    struct Version {
        #[serde(rename = "@major")]
        major: u8,
        #[serde(rename = "@minor")]
        minor: u8,
        #[serde(rename = "@revision")]
        revision: u8,
        #[serde(rename = "@build")]
        build: u64,
    }

    #[derive(Debug, Deserialize)]
    struct Region {
        #[serde(rename = "node")]
        node: Node,
    }

    #[derive(Debug, Deserialize)]
    struct Node {
        #[serde(rename = "children")]
        children: Children,
    }

    #[derive(Debug, Deserialize)]
    struct Children {
        #[serde(rename = "node")]
        nodes: Vec<XmlNode>,
    }

    #[derive(Debug, Deserialize)]
    struct XmlNode {
        #[serde(rename = "@id")]
        id: String,
        attributes: Option<Vec<XmlAttribute>>,
    }

    #[derive(Debug, Deserialize)]
    struct XmlAttribute {
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "@value")]
        value: String,
        #[serde(rename = "@type")]
        data_type: String,
    }

    #[cfg(test)]
    mod tests {
        use std::fs;

        use super::Save;

        #[test]
        fn deserialize() {
            let xml_string = fs::read_to_string("test-xml/test.xml").expect("file read");

            let xml: Save = quick_xml::de::from_str(&xml_string).expect("xml deserialized");
        }
    }
}

#[cfg(test)]
mod tests {}
