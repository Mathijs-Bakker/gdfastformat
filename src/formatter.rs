use tree_sitter::{Node, Tree};
use std::fmt::Write;

/// Format `GDScript` source code based on Tree-sitter AST
pub fn format_tree(source: &str, tree: &Tree) -> String {
    let root = tree.root_node();
    let mut output = String::new();
    format_node(source, root, 0, &mut output);
    output
}

fn format_node(source: &str, node: Node, indent_level: usize, output: &mut String) {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();

    let block_kinds = [
        "function_definition",
        "if_statement",
        "for_statement",
        "while_statement",
        "match_statement",
        "class_definition",
    ];

    if block_kinds.contains(&node.kind()) {
        // Print the whole header line
        let _ = writeln!(output, "{}{}\n", " ".repeat(indent_level * 4), text.split('\n').next().unwrap_or(""));

        // Recurse into children for the block body
        for child in node.children(&mut node.walk()) {
            if child.start_byte() != node.start_byte() {
                format_node(source, child, indent_level + 1, output);
            }
        }
    } else if node.child_count() == 0 {
        // Leaf node, print as-is
        if !text.is_empty() {
            let _ = writeln!(output, "{}{}\n", " ".repeat(indent_level * 4), text);
        }
    } else {
        // Non-leaf, non-block node, print full text if it spans multiple tokens
        let full_text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();
        if !full_text.is_empty() {
            let _ = writeln!(output, "{}{}\n", " ".repeat(indent_level * 4), full_text);
        }
    }
}
