use crate::error::Result;
use crate::layout::insert_body_into_layout;
use crate::template_processors::handlebars::{
    remove_handlebars_variables, replace_template_variable,
};
use crate::template_processors::process_template_tags;
use crate::types::{ContentCollection, PostsByYear, TemplateIncludes, Variables};
use crate::write::write_html_to_file;

pub fn generate_index_page(
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    global_variables: &Variables,
) -> Result<()> {
    // Group posts by year
    let mut posts_by_year: PostsByYear = PostsByYear::new();
    for post in posts {
        if let Some(date_str) = post.get("date") {
            let year = &date_str[0..4]; // Extract the first 4 characters as the year
            posts_by_year
                .entry(year.to_string())
                .or_default()
                .push(post.clone());
        }
    }

    // Collect and sort the years in descending order
    let mut years: Vec<String> = posts_by_year.keys().cloned().collect();
    years.sort_by(|a, b| b.cmp(a));

    let list_item_template = includes
        .get("list_item.liquid")
        .cloned()
        .unwrap_or_default();
    let year_section_template = includes
        .get("year_section.liquid")
        .cloned()
        .unwrap_or_default();
    let mut html_list = String::new();

    for year in years {
        if let Some(posts) = posts_by_year.get(&year) {
            let mut year_content = String::new();
            for post in posts {
                year_content.push_str(&process_template_tags(&list_item_template, post)?);
            }

            let mut year_variables = Variables::new();
            year_variables.insert("content".to_string(), year_content);
            year_variables.insert(
                "year_include".to_string(),
                includes
                    .get(&format!("{}.liquid", year))
                    .cloned()
                    .unwrap_or_default(),
            );

            html_list.push_str(&process_template_tags(
                &year_section_template,
                &year_variables,
            )?);
        }
    }

    let mut variables = global_variables.clone();
    variables.insert("content".to_string(), html_list);

    let index_intro_template = includes
        .get("index_intro.liquid")
        .cloned()
        .unwrap_or_default();
    let processed_content = process_template_tags(&index_intro_template, &variables)?;

    let mut html = insert_body_into_layout(main_layout, &processed_content)?;
    html = replace_template_variable(
        &html,
        "title",
        global_variables.get("title").map_or("", String::as_str),
    )?;
    html = remove_handlebars_variables(&html)?;

    let index_filename = global_variables
        .get("index_filename")
        .map_or("index.html", String::as_str);
    let output_path = format!("out/{}", index_filename);
    write_html_to_file(&output_path, &html)?;

    Ok(())
}
