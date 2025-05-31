use serde::Deserialize;

/// Response struct for querying a player's UUID, username, skin and cape.
///
/// This struct corresponds to the data returned by the following Mojang API endpoints:
/// - `https://api.minecraftservices.com/minecraft/profile/lookup/name/<player name>`
/// - `https://api.minecraftservices.com/minecraft/profile/lookup/<UUID>`
/// - `https://sessionserver.mojang.com/session/minecraft/profile/<UUID>`
/// 
/// Fields:
/// - `id`: The UUID of the player.
/// - `name`: The username of the player.
/// - `legacy`: Optional. Present if the account has **not** migrated to a Mojang account.
/// - `demo`: Optional. Present if the account does **not** own the game.
/// - `properties`: Optional. Present when querying skin and cape. A list of player properties.  
#[derive(Debug, Deserialize)]
pub struct PlayerInfo {
    pub id: String,
    pub name: String,
    pub legacy: Option<bool>,
    pub demo: Option<bool>,
    pub properties: Option<Vec<PlayerProperty>>
}

/// Part of response struct, which is needed when querying player's skin and cape.
/// Fields:
/// - `name`: Name of the property. For now, the only property that exists is textures.
/// - `value`:  Base64 string with all player textures (skin and cape).
#[derive(Debug, Deserialize)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct DecodedPlayerProperty {
    pub value: Textures
}

/// Fields:
/// - `skin`: Skin texture. This does not exist if the player does not have a custom skin. 
/// - `cape`: Cape texture. If the player does not have a cape, this does not exist. 
#[derive(Debug, Deserialize)]
pub struct Textures {
    pub skin: Option<Texture>,
    pub cape: Option<Texture>
}

/// Field:
/// - `url`: URL to the texture.
#[derive(Debug, Deserialize)]
pub struct Texture {
    pub url: String
}