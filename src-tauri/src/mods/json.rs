use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Mod {
    Author: String,
    Name: String,
    Folder: String,
    Version: Option<String>,
    Description: String,
    UUID: String,
    Created: String,
    Dependencies: Vec<String>,
    Group: String,
}

#[derive(Debug, Deserialize)]
struct ModsResponse {
    Mods: Vec<Mod>,
    MD5: String,
}

#[cfg(test)]
mod test {
    use super::ModsResponse;

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
        let mods_response: ModsResponse =
            serde_json::from_str(json_data).expect("deserialize json");
    }
}
