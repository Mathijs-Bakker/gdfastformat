#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use std::{env, fs};

use gdfastformat::{GdscriptParser, format_source};

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.gd>", args[0]);
        return;
    }
    let file_path = &args[1];

    // Read the source code from the file
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read {file_path}: {err}");
            return;
        }
    };

    // Parse the source code
    let mut parser = GdscriptParser::new();
    match parser.parse(&source_code) {
        Ok(tree) => {
            println!("Parsed successfully! Root node kind: {}", tree.root_node().kind());

            let formatted = format_source(&source_code);

            println!("Formatted output:\n{formatted}");
        }
        Err(err) => {
            println!("Parse error: {err}");
        }
    }
}
