use crate::markdown_with_front_matter::parse_markdown_with_front_matter;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn load_and_parse_markdown_file_with_front_matter(
    file_path: &Path,
) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(file_path)?;
    let mut parsed_content = parse_markdown_with_front_matter(&content);

    if let Some(file_stem) = file_path.file_stem().and_then(|s| s.to_str()) {
        parsed_content.insert("slug".to_string(), file_stem.to_string());
    }

    Ok(parsed_content)
}

pub fn load_and_parse_markdown_files_with_front_matter_in_directory(
    dir_path: &str,
) -> Result<Vec<HashMap<String, String>>> {
    let path = Path::new(dir_path);
    let mut results = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let parsed_content = load_and_parse_markdown_file_with_front_matter(&path)?;
            results.push(parsed_content);
        }
    }

    results.sort_by(|a: &HashMap<String, String>, b| b["slug"].cmp(&a["slug"]));

    Ok(results)
}
