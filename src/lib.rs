use base64::{Engine, engine::general_purpose};
use errors::Errors;
use image::{imageops::{overlay, FilterType}, ImageBuffer, RgbaImage};
use reqwest::Client;
use structs::{
    PlayerInfo, PlayerSkinAndCape, PlayerSkinAndCapeQuery, PropertyDecoded, TextureObject,
};

pub mod errors;
pub mod structs;

const TEXTURES_PROPERTY_NAME: &str = "textures";
const SKIN_OBJECT_NAME: &str = "SKIN";

pub async fn query_player_by_username(client: &Client, name: &str) -> Result<PlayerInfo, Errors> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let response = client.get(url).send().await?;
    let playerinfo: PlayerInfo = response.json().await?;
    Ok(playerinfo)
}

pub async fn query_player_by_uuid(client: &Client, uuid: &str) -> Result<PlayerInfo, Errors> {
    let url = format!(
        "https://api.minecraftservices.com/minecraft/profile/lookup/{}",
        uuid
    );
    let response = client.get(url).send().await?;
    let playerinfo: PlayerInfo = response.json().await?;
    Ok(playerinfo)
}

pub async fn query_player_skin_and_cape(
    client: &Client,
    uuid: &str,
) -> Result<PlayerSkinAndCapeQuery, Errors> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
    let response = client.get(url).send().await?;
    let player_skin_and_cape: PlayerSkinAndCapeQuery = response.json().await?;
    Ok(player_skin_and_cape)
}

pub async fn get_player_skin_and_cape(
    client: &Client,
    uuid: &str,
) -> Result<PlayerSkinAndCape, Errors> {
    let data = query_player_skin_and_cape(&client, uuid).await?;
    let property = data.find_property(TEXTURES_PROPERTY_NAME).unwrap();
    let value = decode_value(&property.value)?;
    let decoded_property = PropertyDecoded {
        name: property.name.to_owned(),
        value,
    };

    let player_skin_and_cape = PlayerSkinAndCape {
        id: data.id,
        name: data.name,
        legacy: data.legacy,
        properties: vec![decoded_property],
    };

    Ok(player_skin_and_cape)
}

fn decode_value(value: &str) -> Result<TextureObject, Errors> {
    let decoded = general_purpose::STANDARD.decode(value)?;
    let decoded_str = String::from_utf8_lossy(&decoded).to_string();
    let value: TextureObject = serde_json::from_str(&decoded_str)?;
    Ok(value)
}

pub async fn extract_minecraft_head_from_url(
    client: &Client,
    url: &str,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let png_bytes = response.bytes().await?;

    let img = image::load_from_memory(&png_bytes)?;

    let base_head = img.crop_imm(8, 8, 8, 8);
    let hat = img.crop_imm(40, 8, 8, 8);

    let mut combined: RgbaImage = ImageBuffer::new(128, 128);

    let scaled_base = image::imageops::resize(&base_head, 128, 128, FilterType::Nearest);
    let scaled_hat = image::imageops::resize(&hat, 128, 128, FilterType::Nearest);

    overlay(&mut combined, &scaled_base, 0, 0);
    overlay(&mut combined, &scaled_hat, 0, 0);

    Ok(combined)
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::{
        SKIN_OBJECT_NAME, TEXTURES_PROPERTY_NAME, errors::Errors, extract_minecraft_head_from_url,
        get_player_skin_and_cape, query_player_by_username,
    };

    #[tokio::test]
    async fn test_minecraft_head_extraction() -> Result<(), Errors> {
        let client = Client::new();
        let uuid = query_player_by_username(&client, "STierProgrammer")
            .await?
            .id;
        let player_skin_and_cape = &get_player_skin_and_cape(&client, &uuid).await?;
        let textures = &player_skin_and_cape
            .find_property(TEXTURES_PROPERTY_NAME)
            .ok_or(Errors::NotPresent)?
            .value
            .textures;
        let skin = textures
            .entries
            .get(SKIN_OBJECT_NAME)
            .ok_or(Errors::NotPresent)?;
        let head = extract_minecraft_head_from_url(&client, &skin.url)
            .await
            .unwrap();
        head.save("head.png").unwrap();

        Ok(())
    }
}
