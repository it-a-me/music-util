use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::Serialize;
use symphonia::{core::io::MediaSourceStream, default::get_probe};

use crate::{
    metadata::{get_artist, get_title},
    sort::target_location,
};

#[derive(Debug, Clone)]
pub struct Stats<'a> {
    pub total: Vec<&'a Path>,
    pub tagged: Vec<&'a Path>,
    pub untagged: Vec<&'a Path>,
    pub sorted: Vec<&'a Path>,
    pub unsorted: Vec<&'a Path>,
}

impl Stats<'_> {
    pub(crate) fn numbers(&self) -> StatNumbers {
        StatNumbers {
            total: self.total.len(),
            tagged: self.tagged.len(),
            untagged: self.untagged.len(),
            sorted: self.sorted.len(),
            unsorted: self.unsorted.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StatNumbers {
    pub total: usize,
    pub tagged: usize,
    pub untagged: usize,
    pub sorted: usize,
    pub unsorted: usize,
}

pub fn get_stats<'a>(prefix: &Path, songs: &'a [PathBuf]) -> crate::Result<Stats<'a>> {
    let songs = songs
        .iter()
        .map(|s| -> crate::Result<_> {
            let mut probed = get_probe().format(
                &Default::default(),
                MediaSourceStream::new(Box::new(File::open(s)?), Default::default()),
                &Default::default(),
                &Default::default(),
            )?;
            Ok((s.as_path(), get_artist(&mut probed), get_title(&mut probed)))
        })
        .collect::<crate::Result<Vec<_>>>()?;
    let total = songs.iter().map(|(p, _, _)| *p).collect();
    let tagged: Vec<&Path> = songs
        .iter()
        .filter(|(_, a, t)| a.is_some() && t.is_some())
        .map(|(p, _, _)| *p)
        .collect();
    let untagged = songs
        .iter()
        .filter(|(_, a, t)| a.is_none() || t.is_none())
        .map(|(p, _, _)| *p)
        .collect();

    let sorted = songs
        .iter()
        .filter(|(p, a, t)| {
            if let (Some(a), Some(t)) = (a, t) {
                return *p
                    == target_location(
                        prefix,
                        a,
                        t,
                        &p.extension()
                            .expect("All songs have an extention")
                            .to_string_lossy(),
                    );
            }
            false
        })
        .map(|(p, _, _)| *p)
        .collect();
    let unsorted = songs
        .iter()
        .filter(|(p, a, t)| {
            if let (Some(a), Some(t)) = (a, t) {
                return *p
                    != target_location(
                        prefix,
                        a,
                        t,
                        &p.extension()
                            .expect("All songs have an extention")
                            .to_string_lossy(),
                    );
            }
            false
        })
        .map(|(p, _, _)| *p)
        .collect();

    Ok(Stats {
        total,
        tagged,
        untagged,
        sorted,
        unsorted,
    })
}
