use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::metadata::is_file_tagged;

#[derive(Debug, Clone)]
pub struct Stats<'a> {
    pub total: Vec<&'a Path>,
    pub tagged: Vec<&'a Path>,
    pub untagged: Vec<&'a Path>,
    pub sorted: Vec<&'a Path>,
}

impl<'a> Stats<'a> {
    pub(crate) fn numbers(&self) -> StatNumbers {
        StatNumbers {
            total: self.total.len(),
            tagged: self.tagged.len(),
            untagged: self.untagged.len(),
            sorted: self.sorted.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct StatNumbers {
    pub total: usize,
    pub tagged: usize,
    pub untagged: usize,
    pub sorted: usize,
}

pub fn get_stats(songs: &Vec<PathBuf>) -> crate::Result<Stats> {
    let songs = songs
        .iter()
        .map(|s| -> crate::Result<_> { Ok((s.as_path(), is_file_tagged(s)?)) })
        .collect::<crate::Result<Vec<_>>>()?;
    let total = songs.iter().map(|(p, _)| *p).collect();
    let tagged = songs.iter().filter(|(_, t)| *t).map(|(p, _)| *p).collect();
    let untagged = songs.iter().filter(|(_, t)| !*t).map(|(p, _)| *p).collect();
    Ok(Stats {
        total,
        tagged,
        untagged,
        sorted: Vec::new(),
    })
}
