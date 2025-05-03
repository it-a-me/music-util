use std::path::Path;

use symphonia::{
    core::{
        io::MediaSourceStream,
        meta::{Metadata, StandardTagKey},
        probe::ProbeResult,
    },
    default::get_probe,
};

pub fn get_artist(probed: &mut ProbeResult) -> Option<String> {
    get_standard_metadata(probed, StandardTagKey::Artist)
}

pub fn get_title(probed: &mut ProbeResult) -> Option<String> {
    get_standard_metadata(probed, StandardTagKey::TrackTitle)
}

pub fn get_standard_metadata(probed: &mut ProbeResult, key: StandardTagKey) -> Option<String> {
    // try from metadata in container format
    if let Some(mut metadata) = probed.metadata.get() {
        if let Some(val) = try_get_key(&mut metadata, key) {
            return Some(val);
        }
    }

    // try other metadata
    if let Some(val) = try_get_key(&mut probed.format.metadata(), key) {
        return Some(val);
    }
    None
}

fn try_get_key(metadata: &mut Metadata, key: StandardTagKey) -> Option<String> {
    let Some(metadata) = metadata.skip_to_latest() else {
        return None;
    };
    for tag in metadata.tags() {
        if let Some(k) = tag.std_key {
            if k == key {
                return Some(tag.value.to_string());
            }
        }
    }
    None
}

pub fn is_file_tagged(song: &Path) -> crate::Result<bool> {
    let song = std::fs::File::open(song)?;
    let mut probed = get_probe().format(
        &Default::default(),
        MediaSourceStream::new(Box::new(song), Default::default()),
        &Default::default(),
        &Default::default(),
    )?;
    Ok(is_tagged(&mut probed))
}

pub fn is_tagged(probed: &mut ProbeResult) -> bool {
    get_title(probed).is_some() && get_artist(probed).is_some()
}
