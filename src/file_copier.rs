use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, Read};
use std::path::Path;

pub fn copy_file_with_versioning(source_path: &str, destination_dir: &str) -> io::Result<String> {
    let source_path = Path::new(source_path);
    let destination_dir = Path::new(destination_dir);

    // Ensure the destination directory exists
    fs::create_dir_all(destination_dir)?;

    // Read the contents of the source file for hashing
    let mut file = File::open(source_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Compute a simple hash of the contents
    let mut hasher = DefaultHasher::new();
    hasher.write(&contents);
    let hash = hasher.finish();

    // Split the file name and extension, then reassemble with the hash
    let file_stem = source_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    let new_file_name = format!("{file_stem}-{hash:x}.{extension}");

    let destination_path = destination_dir.join(&new_file_name);

    fs::copy(source_path, &destination_path)?;

    Ok(new_file_name)
}
