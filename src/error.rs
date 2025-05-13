use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum RilotError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("WASM error: {0}")]
    WasmError(String),

    #[error("Proxy error: {0}")]
    ProxyError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("Invalid URI: {0}")]
    InvalidUri(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, RilotError>;

impl From<anyhow::Error> for RilotError {
    fn from(err: anyhow::Error) -> Self {
        RilotError::Unknown(err.to_string())
    }
}

impl From<hyper::Error> for RilotError {
    fn from(err: hyper::Error) -> Self {
        RilotError::HttpError(err.to_string())
    }
}

impl From<serde_json::Error> for RilotError {
    fn from(err: serde_json::Error) -> Self {
        RilotError::SerializationError(err.to_string())
    }
}