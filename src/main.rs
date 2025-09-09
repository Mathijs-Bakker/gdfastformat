 #![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     // clippy::cargo,
 )]

use anyhow::Result;
use std::fs;
use std::env;

mod parser;
mod formatter;

use parser::parse_code;
use formatter::format_tree;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gdscript_file>", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];
    let source = fs::read_to_string(filepath)?;

    if let Some(tree) = parse_code(&source) {
        let formatted = format_tree(&source, &tree);
        println!("{formatted}");
    } else {
        eprintln!("Failed to parse the code.");
    }

    Ok(())
}
