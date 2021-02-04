use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error")]
    ConfigFile(#[from] std::io::Error),
    #[error("Serialization error")]
    Serialization(#[from] ron::Error),
}
