use std::{
    fs::File,
    path::{Path, PathBuf},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use symphonia::{core::io::MediaSourceStream, default::get_probe};

use crate::{
    metadata::{get_artist, get_title},
    sort::target_location,
};

#[derive(Debug, Clone, Serialize)]
pub struct Stats<'a> {
    pub stats: StatNumbers,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub total: Vec<&'a Path>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tagged: Vec<&'a Path>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub untagged: Vec<&'a Path>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sorted: Vec<&'a Path>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unsorted: Vec<&'a Path>,
}

impl<'a> Stats<'a> {
    pub fn new(
        total: Vec<&'a Path>,
        tagged: Vec<&'a Path>,
        untagged: Vec<&'a Path>,
        sorted: Vec<&'a Path>,
        unsorted: Vec<&'a Path>,
    ) -> Self {
        let mut s = Self {
            stats: Default::default(),
            total,
            tagged,
            untagged,
            sorted,
            unsorted,
        };
        s.update_numbers();
        s
    }

    fn update_numbers(&mut self) {
        self.stats = StatNumbers {
            total: self.total.len(),
            tagged: self.tagged.len(),
            untagged: self.untagged.len(),
            sorted: self.sorted.len(),
            unsorted: self.unsorted.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct StatNumbers {
    pub total: usize,
    pub tagged: usize,
    pub untagged: usize,
    pub sorted: usize,
    pub unsorted: usize,
}

pub fn get_stats<'a>(prefix: &Path, songs: &'a [PathBuf]) -> crate::Result<Stats<'a>> {
    let songs = songs
        .par_iter()
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
    let total = songs.par_iter().map(|(p, _, _)| *p).collect();
    let tagged: Vec<&Path> = songs
        .par_iter()
        .filter(|(_, a, t)| a.is_some() && t.is_some())
        .map(|(p, _, _)| *p)
        .collect();
    let untagged = songs
        .par_iter()
        .filter(|(_, a, t)| a.is_none() || t.is_none())
        .map(|(p, _, _)| *p)
        .collect();

    let sorted = songs
        .par_iter()
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
        .par_iter()
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

    Ok(Stats::new(total, tagged, untagged, sorted, unsorted))
}
