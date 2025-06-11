mod error;
mod file_copier;
mod file_readers;
mod parsers;
mod generate;
mod generate_pagination_pages;
mod watch;
mod types;

mod index_page;
mod layout;
mod template_processors;
mod load_includes;

mod server;
mod render_page;
mod write;

use generate::generate;
use std::env;
use error::Result;
use server::listen;
use watch::watch;

fn print_usage() {
    eprintln!("Available commands:");
    eprintln!("  generate <site_name>  Generate the static site");
    eprintln!("  serve                 Start the development server");
    eprintln!("  watch <site_name>     Watch for changes and regenerate");
    eprintln!("  watch <site_name> --ramdisk  Watch with RAM-based output (Linux only)");
}

fn handle_command(args: &[&str]) -> Result<()> {
    match args {
        ["generate", site_name] => {
            generate(site_name)?;
        }
        ["generate"] => {
            eprintln!("Error: Site name is required for generate command.");
            eprintln!("Usage: {} generate <site_name>", args[0]);
            eprintln!("Example: {} generate lepkef.ing", args[0]);
            std::process::exit(1);
        }
        ["serve"] => {
            listen()?;
        }
        ["watch", site_name] => {
            watch(site_name, false)?;
        }
        ["watch", site_name, "--ramdisk"] | ["watch", "--ramdisk", site_name] => {
            watch(site_name, true)?;
        }
        [unknown_command] => {
            eprintln!("Error: Unknown command '{}'", unknown_command);
            print_usage();
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: No command specified");
            print_usage();
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    handle_command(&args[1..])
}
