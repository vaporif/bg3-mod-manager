use serde::{Deserialize, Serialize};
use serde_json::Value; // Import Value for the Version field that can be null

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
