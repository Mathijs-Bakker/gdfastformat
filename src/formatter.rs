use tree_sitter::{Node, Tree};
use std::fmt::Write;

/// Format GDScript source code based on Tree-sitter AST
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
        // Print block header
        let _ = writeln!(
            output,
            "{}{}",
            " ".repeat(indent_level * 4),
            text.split('\n').next().unwrap_or("")
        );

        // Recurse for body
        for child in node.children(&mut node.walk()) {
            if child.start_byte() != node.start_byte() {
                format_node(source, child, indent_level + 1, output);
            }
        }
    } else if node.child_count() == 0 {
        // Leaf nodes: print as-is
        if !text.is_empty() {
            let _ = writeln!(output, "{}{}", " ".repeat(indent_level * 4), text);
        }
    } else {
        // Non-leaf, non-block: flatten children into single line
        let mut line = String::new();
        for child in node.children(&mut node.walk()) {
            let child_text = child.utf8_text(source.as_bytes()).unwrap_or("").trim();
            if !child_text.is_empty() {
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(child_text);
            }
        }
        if !line.is_empty() {
            let _ = writeln!(output, "{}{}", " ".repeat(indent_level * 4), line);
        }
    }
}
