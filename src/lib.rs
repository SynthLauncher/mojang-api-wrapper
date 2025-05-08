use errors::Errors;
use serde::Deserialize;

pub mod errors;

#[derive(Debug, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub id: String,
    pub legacy: Option<bool>,
    pub demo: Option<bool>
}

pub async fn get_player_by_username(name: &str) -> Result<PlayerInfo, Errors> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let playerinfo: PlayerInfo = serde_json::from_slice(&bytes)?;

    Ok(playerinfo)
}

pub async fn get_player_by_uuid(uuid: &str) -> Result<PlayerInfo, Errors> {
    let url = format!("https://api.minecraftservices.com/minecraft/profile/lookup/{}", uuid);
    let response = reqwest::get(url).await?;
    let  bytes = response.bytes().await?;
    let playerinfo: PlayerInfo = serde_json::from_slice(&bytes)?;

    Ok(playerinfo)
}

#[cfg(test)]
mod tests {
    use crate::{get_player_by_username, get_player_by_uuid};
    
    #[tokio::test]
    async fn get_player_by_username_test() {
        let player = get_player_by_username("STierProgrammer").await.unwrap();
        println!("{:?}", player);
    }

    #[tokio::test]
    async fn get_player_by_uuid_test() {
        let player = get_player_by_uuid("94240269bb0f4570ab261e2a47dbc565").await.unwrap();
        println!("{:?}", player);
    }
}