use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_liquid_includes(dir_path: &str) -> HashMap<String, String> {
    let path = Path::new(dir_path);
    let mut templates = HashMap::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("liquid") {
                if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        templates.insert(filename.to_string(), contents);
                    }
                }
            }
        }
    }

    templates
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::collections::BTreeMap;
    use serde_json;

    #[test]
    fn test_load_liquid_includes() {
        let templates = load_liquid_includes("../_includes");
        let sorted_templates: BTreeMap<_, _> = templates.into_iter().collect();
        let templates_json = serde_json::to_string_pretty(&sorted_templates).unwrap();
        // Use assert_snapshot! to compare the output against the stored snapshot
        assert_snapshot!(templates_json);
    }
}
