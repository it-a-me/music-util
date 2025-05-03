use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use symphonia::{
    core::{
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::{Metadata, MetadataOptions, Tag},
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    path: PathBuf,
    metadata: HashMap<String, String>,
    codec: &'static str,
}

pub fn get_info(path: &Path, nonstandard: bool) -> crate::Result<Info> {
    let song = std::fs::File::open(path)?;
    let mut probe = get_probe().format(
        &Hint::new(),
        MediaSourceStream::new(Box::new(song), MediaSourceStreamOptions::default()),
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let mut metadata = HashMap::new();
    if let Some(mut md) = probe.metadata.get() {
        add_metadata(&mut md, &mut metadata, nonstandard);
    }

    let mut format = probe.format;
    add_metadata(&mut format.metadata(), &mut metadata, nonstandard);

    let codec = format
        .default_track()
        .and_then(|t| get_codecs().get_codec(t.codec_params.codec))
        .map(|c| c.long_name)
        .unwrap_or("Unknown");

    Ok(Info {
        path: path.to_path_buf(),
        metadata,
        codec,
    })
}

fn add_metadata(metadata: &mut Metadata, map: &mut HashMap<String, String>, nonstandard: bool) {
    let Some(md) = metadata.skip_to_latest() else {
        return;
    };

    for tag in md.tags() {
        let Some((key, value)) = format_tag(tag, nonstandard) else {
            continue;
        };
        map.insert(key, value);
    }
}

fn format_tag(tag: &Tag, nonstandard: bool) -> Option<(String, String)> {
    if let Some(key) = tag.std_key {
        Some((format!("{key:?}"), tag.value.to_string()))
    } else if nonstandard {
        Some((tag.key.to_string(), tag.value.to_string()))
    } else {
        None
    }
}
