use crate::error::Result;
use crate::layout::{insert_body_into_layout, load_layout};
use crate::template_processors::liquid::process_liquid_tags;
use crate::template_processors::markdown::markdown_to_html;
use crate::template_processors::process_template_tags;
use crate::types::{TemplateIncludes, Variables};
use crate::write::write_html_to_file;

/// Processes a page through the template pipeline:
/// 1. Converts markdown to HTML (if content is markdown)
/// 2. Inserts into secondary layout (if specified)
/// 3. Inserts into main layout
/// 4. Processes liquid includes
/// 5. Processes template tags
/// 6. Writes to file
pub fn render_page(
    body: &str,
    directory: &str,
    slug: &str,
    layout: &str,
    includes: &TemplateIncludes,
    variables: &Variables,
) -> Result<()> {
    let file_name = directory.to_string() + slug + ".html";
    let keys: Vec<String> = variables.keys().cloned().collect();

    // Check if the content is markdown or HTML or handlebars
    let is_markdown = variables.get("file_type").map_or(true, |ft| ft == "md");
    let is_handlebars = variables.get("file_type").map_or(false, |ft| ft == "hbs");

    // Process the body content first
    let processed_body = if is_markdown {
        markdown_to_html(body)
    } else {
        // For handlebars files, process the template variables first
        if is_handlebars {
            process_template_tags(body, variables)?
        } else {
            body.to_string()
        }
    };

    // Apply secondary layout if specified in front matter
    let content_with_layout = if let Some(secondary_layout_name) = variables.get("layout") {
        let layout_path = format!(
            "./sites/{}/layouts/{}.html",
            variables.get("site_name").unwrap_or(&"".to_string()),
            secondary_layout_name
        );

        if let Ok(secondary_layout) = load_layout(&layout_path) {
            // Insert the content into the secondary layout
            let layout_with_content = insert_body_into_layout(&secondary_layout, &processed_body)?;
            // Process any template variables in the combined result
            process_template_tags(&layout_with_content, variables)?
        } else {
            eprintln!(
                "⚠️  Warning: Layout '{}' specified in '{}' was not found at '{}'",
                secondary_layout_name, file_name, layout_path
            );
            processed_body
        }
    } else {
        processed_body
    };

    // Insert content into main layout
    let combined_content = insert_body_into_layout(layout, &content_with_layout)?;

    // Process all liquid includes at once
    let content_with_includes = process_liquid_tags(&combined_content, &keys, includes)?;

    // Process all template tags
    let html = process_template_tags(&content_with_includes, variables)?;

    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // ... existing code ...
}
