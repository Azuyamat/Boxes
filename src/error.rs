#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config error: {0}")]
    Config(#[from] confy::ConfyError),
    #[error("Inquire error: {0}")]
    Inquire(#[from] inquire::InquireError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Toml error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,

    #[error("🚨 Resource not found: {0}")]
    ResourceNotFound(String),
}