use std::path::Path;

pub fn hash_filename(path: &Path) -> Result<Option<blake3::Hash>, crate::Error> {
    let filename = path.file_name().expect("No file name");
    Ok(Some(blake3::hash(filename.as_encoded_bytes())))
}
