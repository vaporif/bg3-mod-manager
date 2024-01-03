#[allow(unused)]
use serde::{Deserialize, Serialize};

pub const ROOT_ID: &str = "root";
pub const MODS_ID: &str = "Mods";

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeType {
    ModOrder,
    Module,
    Mods,
    ModuleShortDesc,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, Serialize)]
pub enum AttributeType {
    Folder,
    MD5,
    Name,
    UUID,
    Version64,
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
pub enum AttributeDataType {
    FixedString,
    LSString,
    int64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    version: Version,
    pub region: Region,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "@major")]
    major: u8,
    #[serde(rename = "@minor")]
    minor: u8,
    #[serde(rename = "@revision")]
    revision: u8,
    #[serde(rename = "@build")]
    build: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    #[serde(rename = "node")]
    pub node: Node,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "children")]
    pub children: Children,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Children {
    #[serde(rename = "node")]
    pub nodes: Vec<XmlNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmlNode {
    #[serde(rename = "@id")]
    pub id: NodeType,
    pub children: Option<Vec<Children>>,
    #[serde(rename = "attribute")]
    pub attributes: Option<Vec<XmlAttribute>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmlAttribute {
    #[serde(rename = "@id")]
    pub id: AttributeType,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@type")]
    pub data_type: AttributeDataType,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Save;

    #[test]
    fn deserialize() {
        let xml_string = fs::read_to_string("test-xml/test.xml").expect("file read");

        let xml: Save = quick_xml::de::from_str(&xml_string).expect("xml deserialized");
        dbg!(xml);
    }
}
