#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

mod parser;
mod formatter;

use parser::GdscriptParser;
use formatter::format_tree;

fn main() {
    let mut parser = GdscriptParser::new();

    let source = r#"
func _ready():
    print("Hello World")
"#;

    match parser.parse(source) {
        Ok(tree) => {
            println!("Parsed successfully! Root node kind: {}", tree.root_node().kind());

            // Format the parsed tree
            let formatted = format_tree(source, &tree);
            println!("Formatted output:\n{}", formatted);
        }
        Err(err) => eprintln!("Parse error: {}", err),
    }
}
