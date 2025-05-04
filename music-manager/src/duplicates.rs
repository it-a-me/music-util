use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

mod filename;
mod metadata;
mod stream;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
pub use stream::hash_stream;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Duplicates<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Vec<&'a Path>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub filename: Vec<Vec<&'a Path>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stream: Vec<Vec<&'a Path>>,
}

pub fn detect_duplicates(
    songs: &Vec<PathBuf>,
    metadata: bool,
    filename: bool,
    stream: bool,
) -> crate::Result<Duplicates> {
    let mut duplicates = Duplicates::default();
    if metadata {
        duplicates
            .metadata
            .append(&mut find_duplicates(songs, metadata::hash_metadata)?);
    }
    if filename {
        duplicates
            .filename
            .append(&mut find_duplicates(songs, filename::hash_filename)?);
    }
    if stream {
        duplicates
            .stream
            .append(&mut find_duplicates(songs, stream::hash_stream)?);
    }
    Ok(duplicates)
}

fn find_duplicates<F>(songs: &Vec<PathBuf>, hasher: F) -> crate::Result<Vec<Vec<&Path>>>
where
    F: Fn(&Path) -> crate::Result<Option<blake3::Hash>> + Send + Sync,
{
    let song_hashes = songs
        .par_iter()
        .map(|p| p.as_path())
        .filter_map(|p| -> Option<crate::Result<(&Path, blake3::Hash)>> {
            let hash = hasher(p).transpose()?;
            Some(hash.map(|h| (p, h)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let song_hashes = song_hashes.into_iter().fold(
        HashMap::<blake3::Hash, Vec<&Path>>::new(),
        |mut map, (song, hash)| {
            if let Some(paths) = map.get_mut(&hash) {
                paths.push(song);
            } else {
                map.insert(hash, vec![song]);
            }
            map
        },
    );
    let duplicates = song_hashes
        .into_values()
        .filter(|paths| paths.len() > 1)
        .collect();
    Ok(duplicates)
}
