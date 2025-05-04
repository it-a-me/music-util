use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::walksongs::get_songs;

mod filename;
mod metadata;
mod stream;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub use stream::hash_stream;

pub fn detect_duplicates(
    root: PathBuf,
    metadata: bool,
    filename: bool,
    stream: bool,
    json: bool,
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
    if json {
        println!("{}", serde_json::to_string_pretty(&duplicates).unwrap());
    } else {
        println!(
            "{}",
            toml::to_string_pretty(&HashMap::from([("Duplicates", &duplicates)])).unwrap()
        );
    }
    Ok(())
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
