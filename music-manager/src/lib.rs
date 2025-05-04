pub mod duplicates;
mod error;
pub mod info;
pub mod metadata;
pub mod sort;
pub mod stats;
mod walksongs;

pub use error::Error;
pub use error::Result;
pub use walksongs::get_songs;
