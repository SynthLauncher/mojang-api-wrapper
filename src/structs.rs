use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub id: String,
    pub legacy: Option<bool>,
    pub demo: Option<bool>,
}



#[derive(Debug, Deserialize)]
pub struct TextureEntry {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Textures {
    #[serde(flatten)]
    pub entries: HashMap<String, TextureEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureObject {
    pub profile_id: String,
    pub profile_name: String,
    pub textures: Textures,
}

#[derive(Debug, Deserialize)]
pub struct PropertyEncoded {
    pub name: String,
    // Base 64 string
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct PropertyDecoded {
    pub name: String,
    pub value: TextureObject,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSkinAndCapeQuery {
    pub id: String,
    pub name: String,
    pub legacy: Option<bool>,
    pub properties: Vec<PropertyEncoded>,
}

impl PlayerSkinAndCapeQuery {
    pub fn find_property(&self, name: &str) -> Option<&PropertyEncoded> {
        self.properties.iter().find(|x| x.name == name)
    }
}

#[derive(Debug, Deserialize)]
pub struct PlayerSkinAndCape {
    pub id: String,
    pub name: String,
    pub legacy: Option<bool>,
    pub properties: Vec<PropertyDecoded>,
}

impl PlayerSkinAndCape {
    pub fn find_property(&self, name: &str) -> Option<&PropertyDecoded> {
        self.properties.iter().find(|x| x.name == name)
    }
}