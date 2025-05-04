use std::{
    io::{ErrorKind, Write},
    path::Path,
};

use symphonia::{
    core::{
        errors::Error as SymphoniaError,
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::get_probe,
};

pub fn hash_stream(path: &Path) -> Result<Option<blake3::Hash>, crate::Error> {
    let song = std::fs::File::open(path)?;
    let format = get_probe().format(
        &Hint::new(),
        MediaSourceStream::new(Box::new(song), MediaSourceStreamOptions::default()),
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let mut reader = format.format;
    let mut hasher = blake3::Hasher::new();
    loop {
        let packet = match reader.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(err)) if err.kind() == ErrorKind::UnexpectedEof => break,
            Err(err) => Err(err)?,
        };
        hasher.write_all(&packet.data)?;
    }
    Ok(Some(hasher.finalize()))
}
