use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Toml error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,
}