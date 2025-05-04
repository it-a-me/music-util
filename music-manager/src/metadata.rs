use symphonia::core::{
    meta::{Metadata, StandardTagKey},
    probe::ProbeResult,
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
    let metadata = metadata.skip_to_latest()?;
    for tag in metadata.tags() {
        if let Some(k) = tag.std_key {
            if k == key {
                return Some(tag.value.to_string());
            }
        }
    }
    None
}
