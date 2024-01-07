use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ModInfo {
    pub author: String,
    pub name: String,
    pub folder: String,
    pub version: Option<String>,
    pub description: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub created: String,
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub order: u8,
    pub group: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Mods {
    pub mods: Vec<ModInfo>,
    #[serde(rename = "MD5")]
    pub md5: String,
}

#[cfg(test)]
mod test {
    use crate::mods::json::{ModInfo, Mods};

    #[test]
    fn deserialize() {
        let json_data = r#"{
            "Mods": [
                {
                    "Author": "Random Author",
                    "Name": "SomeName",
                    "Folder": "SomeFolder",
                    "Version": null,
                    "Description": "Description",
                    "UUID": "3fecde04-2f5d-4c6a-bb20-4ebd336472c2",
                    "Created": "2023-09-11T05:01:15.0784029-06:00",
                    "Dependencies": [],
                    "Group": "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df"
                }
            ],
            "MD5": "463bda86406dc324991fa3981ff63b72"
        }"#;
        let parsed: Mods = serde_json::from_str(json_data).expect("deserialize json");
        assert_eq!(
            Mods {
                mods: vec![ModInfo {
                    author: "Random Author".to_string(),
                    name: "SomeName".to_string(),
                    folder: "SomeFolder".to_string(),
                    version: None,
                    description: "Description".to_string(),
                    uuid: "3fecde04-2f5d-4c6a-bb20-4ebd336472c2".to_string(),
                    created: "2023-09-11T05:01:15.0784029-06:00".to_string(),
                    is_disabled: true,
                    order: 0,
                    dependencies: vec![],
                    group: "57bacf0b-9ab7-4cd9-b7ed-40050ffa41df".to_string()
                }],
                md5: "463bda86406dc324991fa3981ff63b72".to_string()
            },
            parsed
        )
    }
}
