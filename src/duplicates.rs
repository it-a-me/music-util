use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::walksongs::get_songs;

mod filename;
mod metadata;
mod stream;

pub use stream::hash_stream;

pub(crate) fn detect_duplicates(
    root: PathBuf,
    metadata: bool,
    filename: bool,
    stream: bool,
) -> crate::Result<()> {
    let songs = get_songs(root)?;
    let mut duplicates = Vec::new();
    if metadata {
        duplicates.append(&mut find_duplicates(&songs, metadata::hash_metadata)?);
    }
    if filename {
        duplicates.append(&mut find_duplicates(&songs, filename::hash_filename)?);
    }
    if stream {
        duplicates.append(&mut find_duplicates(&songs, stream::hash_stream)?);
    }
    dbg!(duplicates);
    Ok(())
}

fn find_duplicates<F: Fn(&Path) -> crate::Result<Option<blake3::Hash>>>(
    songs: &Vec<PathBuf>,
    hasher: F,
) -> crate::Result<Vec<Vec<&Path>>> {
    let mut song_map: HashMap<blake3::Hash, Vec<&Path>> = HashMap::new();
    for song in songs {
        let Some(hash) = hasher(song)? else { continue };
        if let Some(entry) = song_map.get_mut(&hash) {
            entry.push(song.as_path());
        } else {
            song_map.insert(hash, vec![song.as_path()]);
        }
    }
    let duplicates = song_map.into_values().filter(|v| v.len() > 1).collect();
    Ok(duplicates)
}
