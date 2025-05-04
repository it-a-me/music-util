use std::{io, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Symphonia(#[from] symphonia::core::errors::Error),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("Missing Metadata")]
    MissingMetadata,
    #[error("Unable to move {} to {}: File already exists", .src.to_string_lossy(), .dest.to_string_lossy())]
    AlreadyExists { src: PathBuf, dest: PathBuf },
}
