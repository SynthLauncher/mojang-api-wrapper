use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::error::Error),
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error)
}
