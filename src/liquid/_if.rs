pub fn process_if_tags(input: &str, conditions: &[&str]) -> String {
    let mut result = input.to_string();
    let mut start = 0;
    let mut replacements = Vec::new();

    while let Some(start_index) = result[start..].find("{if ") {
        let tag_start = start + start_index;
        let end_index = match result[tag_start..].find("{/if}") {
            Some(index) => tag_start + index + 5,
            None => break,
        };

        let tag_content = &result[tag_start..end_index];
        if let Some(condition_end) = tag_content.find('}') {
            let condition = &tag_content[4..condition_end].trim();
            if conditions.contains(&condition) {
                // Collect the range to replace with just the content inside
                let content_start = tag_start + condition_end + 1;
                let content_end = end_index - 5;
                let content = &result[content_start..content_end];
                replacements.push((tag_start, end_index, content.to_string()));
            } else {
                // Collect the range to remove
                replacements.push((tag_start, end_index, "".to_string()));
            }
        }

        start = tag_start + 1;
    }

    // Perform the replacements
    for (start, end, replacement) in replacements.iter().rev() {
        result.replace_range(*start..*end, replacement);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_if_tags() {
        let input = "{if something}lorem ipsum dolor sit amet{/if} and some other text {if another}this should stay{/if}";
        let conditions = ["another"];
        let expected_output = " and some other text this should stay";
        let output = process_if_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_multiple_conditions() {
        let input = "{if something}lorem ipsum dolor sit amet{/if} and some other text {if another}this should stay{/if} {if yet_another}this should also stay{/if}";
        let conditions = ["another", "yet_another"];
        let expected_output = " and some other text this should stay this should also stay";
        let output = process_if_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_no_conditions() {
        let input = "{if something}lorem ipsum dolor sit amet{/if} and some other text {if another}this should stay{/if}";
        let conditions: [&str; 0] = [];
        let expected_output = " and some other text ";
        let output = process_if_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_process_if_tags_with_all_conditions() {
        let input = "{if something}lorem ipsum dolor sit amet{/if} and some other text {if another}this should stay{/if}";
        let conditions = ["something", "another"];
        let expected_output = "lorem ipsum dolor sit amet and some other text this should stay";
        let output = process_if_tags(input, &conditions);
        assert_eq!(output, expected_output);
    }
}
