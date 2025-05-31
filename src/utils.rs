
use base64::{engine::general_purpose, Engine};
use image::{imageops::{overlay, FilterType}, ImageBuffer, RgbaImage};
use crate::{errors::Errors, models::player_info::DecodedPlayerProperty};


use reqwest::Client;

pub fn decode_property_value(value: &str) -> Result<DecodedPlayerProperty, Errors> {
    let decoded = general_purpose::STANDARD.decode(value)?;
    let decoded_str = String::from_utf8_lossy(&decoded).to_string();
    let value: DecodedPlayerProperty = serde_json::from_str(&decoded_str)?;
    Ok(value)
}

pub async fn extract_minecraft_head_from_url(
    client: &Client,
    url: &str,
) -> Result<RgbaImage, Errors> {
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
