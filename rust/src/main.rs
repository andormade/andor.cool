mod file_readers;
mod front_matter;
mod handlebars;
mod layout;
mod liquid;
mod load_includes;
mod markdown;
mod markdown_with_front_matter;
mod write;
mod file_copier;

use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::handlebars::replace_template_variables;
use crate::layout::insert_body_into_layout;
use crate::layout::load_layout;
use crate::liquid::process_liquid_includes;
use crate::load_includes::load_liquid_includes;
use crate::markdown::markdown_to_html;
use crate::write::write_html_to_file;
use crate::file_copier::copy_file_with_versioning;
use std::collections::HashMap;
use std::io::Result;

fn render_page(
    page: &HashMap<String, String>,
    directory: &str,
    layout: &str,
    includes: &HashMap<String, String>,
) -> Result<()> {
    let content = page.get("content").cloned().unwrap_or_else(String::new);
    let mut html = markdown_to_html(&content);
    let slug = page.get("slug").cloned().unwrap_or_else(String::new);
    let file_name = directory.to_string() + &slug + ".html";
    html = process_liquid_includes(&html, &includes);
    html = insert_body_into_layout(&layout, &html);
    write_html_to_file(&file_name, &html)?;

    Ok(())
}

fn main() -> Result<()> {
    let css_file_name = copy_file_with_versioning("../style.css", "./out/")?;
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory("../_posts")?;
    let pages = load_and_parse_markdown_files_with_front_matter_in_directory("../_pages")?;
    let includes = load_liquid_includes("../_includes");
    
    let mut main_layout = load_layout("../_layouts/main.html")?;
    let mut main_layout_variables = HashMap::new();
    main_layout_variables.insert("css_file_name".to_string(), css_file_name);
    main_layout = replace_template_variables(&main_layout, &main_layout_variables);

    // Generate index.html
    let list_item_template = includes
        .get("list_item.liquid")
        .cloned()
        .unwrap_or_else(String::new);
    let mut html_list = String::new();
    html_list.push_str("<ul>\n");
    for post in &posts {
        html_list.push_str(&replace_template_variables(&list_item_template, &post));
    }
    html_list.push_str("</ul>");
    let html = insert_body_into_layout(&main_layout, &html_list);
    write_html_to_file(&"out/index.html", &html)?;

    // Generate posts
    for post in &posts {
        render_page(&post, &"out/posts/", &main_layout, &includes)?;
    }

    // Generate pages
    for page in &pages {
        render_page(&page, &"out/", &main_layout, &includes)?;
    }

    Ok(())
}
