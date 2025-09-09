#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use gdfastformat::{GdscriptParser, format_code};

struct TestCase<'a> {
    name: &'a str,
    source: &'a str,
}

fn run_test(test: &TestCase) {
    println!("--- Test: {} ---", test.name);
    let mut parser = GdscriptParser::new();
    
    match parser.parse(test.source) {
        Ok(tree) => {
            println!("Parsed successfully! Root node kind: {}", tree.root_node().kind());
            let formatted = format_code(test.source);
            println!("Formatted output:\n{formatted}");
        }
        Err(err) => {
            println!("Parse error: {err}");
        }
    }

    println!("---------------------------\n");
}

fn main() {
    let tests = [
        TestCase {
            name: "Simple function",
            source: r#"func _ready():
    print("Hello World")"#,
        },
        TestCase {
            name: "If statement",
            source: r#"func _process(delta):
    if delta > 1.0:
        print("Too slow!")"#,
        },
        TestCase {
            name: "Nested blocks",
            source: r#"class Player:
    func move():
        if is_moving:
            print("Moving")"#,
        },
        TestCase {
            name: "Comments",
            source: r#"# Top-level comment
func _ready():
    print("Hello World")  # inline comment"#,
        },
        TestCase {
            name: "Multi-line string",
            source: r#"func _ready():
    var s = """
Hello
World
"""
    print(s)"#,
        },
        // Add more tests here as needed
    ];

    for test in &tests {
        run_test(test);
    }
}
