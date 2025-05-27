use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::error::Error),
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Base64 Decode Error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Element is not present in the provided set!")]
    NotPresent
}