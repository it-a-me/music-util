use std::{
    cmp::Ordering,
    fs::File,
    path::{Path, PathBuf},
};

use serde::Serialize;
use symphonia::{core::io::MediaSourceStream, default::get_probe};
use tracing::info;

use crate::{
    Error,
    metadata::{get_artist, get_title},
};
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Transaction {
    Mkdir(PathBuf),
    Move { src: PathBuf, dest: PathBuf },
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mkdir(path) => {
                write!(f, "Create directory '{}'", path.to_string_lossy())
            }
            Self::Move { src, dest } => {
                write!(
                    f,
                    "Rename '{}' to '{}'",
                    src.to_string_lossy(),
                    dest.to_string_lossy()
                )
            }
        }
    }
}
impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Move { .. }, Self::Move { .. }) => Ordering::Equal,
            (Self::Mkdir(_), Self::Mkdir(_)) => Ordering::Equal,
            (Self::Move { .. }, Self::Mkdir(_)) => Ordering::Greater,
            (Self::Mkdir(_), Self::Move { .. }) => Ordering::Less,
        }
    }
}

impl Transaction {
    pub fn apply(&self) -> Result<(), std::io::Error> {
        info!("{self}");
        match self {
            Self::Mkdir(path) => std::fs::create_dir(path)?,
            Self::Move { src, dest } => std::fs::rename(src, dest)?,
        }
        Ok(())
    }
}

pub fn sort_songs_transactions(
    prefix: &Path,
    songs: &[impl AsRef<Path>],
) -> crate::Result<Vec<Transaction>> {
    let mut transactions: Vec<Transaction> = songs
        .iter()
        .map(|s| sort_song(prefix, s.as_ref()))
        .try_fold(Vec::new(), |mut v, r| -> crate::Result<_> {
            v.append(&mut r?);
            Ok(v)
        })?;
    // ensure that Mkdirs come before Moves
    transactions.sort_unstable();
    Ok(transactions)
}

fn sort_song(prefix: &Path, song: &Path) -> crate::Result<Vec<Transaction>> {
    let mut probed = get_probe().format(
        &Default::default(),
        MediaSourceStream::new(Box::new(File::open(song)?), Default::default()),
        &Default::default(),
        &Default::default(),
    )?;
    let artist = get_artist(&mut probed).ok_or(crate::Error::MissingMetadata)?;
    let title = get_title(&mut probed).ok_or(crate::Error::MissingMetadata)?;

    let dest = target_location(
        prefix,
        &artist,
        &title,
        &song
            .extension()
            .expect("All songs have an extension")
            .to_string_lossy(),
    );

    if song == dest.as_path() {
        return Ok(Vec::new());
    } else if dest.exists() {
        return Err(Error::AlreadyExists {
            src: song.to_path_buf(),
            dest,
        });
    }

    let mut transactions = Vec::new();
    if !prefix.join(&artist).exists() {
        transactions.push(Transaction::Mkdir(prefix.join(&artist)));
    }
    transactions.push(Transaction::Move {
        src: song.to_path_buf(),
        dest,
    });
    Ok(transactions)
}

pub fn target_location(prefix: &Path, artist: &str, title: &str, extention: &str) -> PathBuf {
    prefix
        .join(sanitize(artist))
        .join(sanitize(&format!("{title}.{extention}")))
}

fn sanitize(s: &str) -> String {
    const VALID_CHARACTERS: &[char] = &[
        '.', ',', '!', '(', ')', ':', '?', ' ', '\'', '"', '-', '_', '=', '&',
    ];
    s.chars()
        .filter_map(|c| match c {
            c if c.is_ascii_alphanumeric() || VALID_CHARACTERS.contains(&c) => Some(c),
            '’' => Some('\''),
            _ => None,
        })
        .collect()
}
