use reqwest::Client;

use crate::{errors::Errors, models::player_info::PlayerInfo};

pub async fn query_player_info_by_username(client: &Client, name: &str) -> Result<PlayerInfo, Errors> {
    let url = format!("https://api.minecraftservices.com/minecraft/profile/lookup/name/{}", name);
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

pub async fn query_player_info_by_uuid(client: &Client, uuid: &str) -> Result<PlayerInfo, Errors> {
    let url = format!(
        "https://api.minecraftservices.com/minecraft/profile/lookup/{}",
        uuid
    );
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

pub async fn query_player_info_with_properties(client: &Client, uuid: &str) -> Result<PlayerInfo, Errors> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

