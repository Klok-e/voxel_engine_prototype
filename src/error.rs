use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error")]
    ConfigFile(#[from] std::io::Error),
    #[error("Ron Serialization error")]
    SerializationRon(#[from] ron::Error),
    #[error("Toml Serialization error")]
    SerializationToml(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
