use tree_sitter::{Parser, TreeCursor};
use tree_sitter_gdscript::LANGUAGE;

/// Format GDScript source code.
pub fn format_source(source: &str) -> String {
    let mut parser = Parser::new();
    parser.set_language(&LANGUAGE.into()).expect("Error loading GDScript grammar");

    let tree = parser.parse(source, None).expect("Error parsing source");
    let mut cursor = tree.walk();
    let mut result = String::new();

    format_node(source, &mut cursor, 0, &mut result);

    result
}

fn format_node(source: &str, cursor: &mut TreeCursor, indent: usize, result: &mut String) {
    loop {
        let node = cursor.node();

        // Grab the raw text for this node
        let text = &source[node.byte_range()];

        // Handle comments and newlines as-is
        if node.kind() == "comment" {
            result.push_str(&" ".repeat(indent * 4));
            result.push_str(text.trim_end());
            result.push('\n');
        }
        // Handle blocks
        else if node.child_count() > 0 {
            if node.kind() == "block" {
                // Increase indentation inside a block
                let mut child_cursor = node.walk();
                format_node(source, &mut child_cursor, indent + 1, result);
            } else {
                // Non-block nodes → keep current indentation
                if node.is_named() {
                    result.push_str(&" ".repeat(indent * 4));
                    result.push_str(text.trim());
                    result.push('\n');
                }
            }
        } else {
            // Leaf nodes (identifiers, literals, operators, etc.)
            if node.is_named() {
                result.push_str(&" ".repeat(indent * 4));
                result.push_str(text.trim());
                result.push('\n');
            }
        }

        // Traverse siblings
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}
