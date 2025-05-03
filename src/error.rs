use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Symphonia(#[from] symphonia::core::errors::Error),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("{}", .0)]
    Cli(String),
    #[error("Missing Metadata")]
    MissingMetadata,
    #[error("File already exists '{}'", .0)]
    AlreadyExists(String),
}
