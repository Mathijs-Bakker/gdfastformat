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

    // Test cases
    let tests = vec![
        (
            "Simple function",
            r#"
func _ready():
    print("Hello World")
"#,
        ),
        (
            "If statement",
            r#"
func _process(delta):
    if delta > 1.0:
        print("Too slow!")
"#,
        ),
        (
            "Nested blocks",
            r#"
class Player:
    func move():
        if is_moving:
            print("Moving")
"#,
        ),
    ];

    for (name, source) in tests {
        println!("--- Test: {} ---", name);
        match parser.parse(source) {
            Ok(tree) => {
                println!("Parsed successfully! Root node kind: {}", tree.root_node().kind());
                let formatted = format_tree(source, &tree);
                println!("Formatted output:\n{}", formatted);
            }
            Err(err) => eprintln!("Parse error: {}", err),
        }
        println!("---------------------------\n");
    }
}
