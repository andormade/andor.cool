// Template processors module
// This module contains different template processing implementations

pub mod liquid;
pub mod markdown;
pub mod handlebars;
mod processor;

pub use processor::process_template_tags;

