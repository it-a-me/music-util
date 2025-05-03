use std::path::{Path, PathBuf};

use tracing::debug;

pub fn get_songs(root: PathBuf) -> crate::Result<Vec<PathBuf>> {
    let mut songs = Vec::new();
    add_dir(root, &mut songs)?;
    Ok(songs)
}

fn add_path(path: PathBuf, song_map: &mut Vec<PathBuf>) -> Result<(), crate::Error> {
    if path.is_file() {
        add_file(path, song_map)?;
    } else if path.is_dir() {
        add_dir(path, song_map)?;
    }
    Ok(())
}

fn add_dir(path: PathBuf, songs: &mut Vec<PathBuf>) -> crate::Result<()> {
    for child in path.read_dir()? {
        let child = child?;
        add_path(child.path(), songs)?
    }
    Ok(())
}

fn add_file(path: PathBuf, songs: &mut Vec<PathBuf>) -> Result<(), crate::Error> {
    if !is_song(&path) {
        return Ok(());
    }
    songs.push(path);
    Ok(())
}

const EXTENTIONS: &[&str] = &["mp1", "mp2", "mp3", "ogg", "opus", "flac", "aac"];
fn is_song(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let extension = extension.to_string_lossy();
        if !EXTENTIONS.contains(&extension.as_ref()) {
            debug!("Skipping unsupported extention {}", extension);
            return false;
        }
    } else {
        return false;
    };
    true
}
