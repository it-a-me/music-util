use std::path::Path;

use symphonia::{
    core::{
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::get_probe,
};

pub fn hash_metadata(path: &Path) -> Result<Option<blake3::Hash>, crate::Error> {
    let song = std::fs::File::open(path)?;
    let mut probed = get_probe().format(
        &Hint::new(),
        MediaSourceStream::new(Box::new(song), MediaSourceStreamOptions::default()),
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let Some(title) = crate::metadata::get_title(&mut probed) else {
        return Ok(None);
    };
    Ok(Some(blake3::hash(title.as_bytes())))
}
