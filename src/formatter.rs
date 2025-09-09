use tree_sitter::{Node, Tree};
use std::fmt::Write;

pub fn format_tree(source: &str, tree: &Tree) -> String {
    let mut output = String::new();
    format_node(source, tree.root_node(), 0, &mut output);
    output
}

const BLOCK_KINDS: &[&str] = &[
    "function_definition",
    "if_statement",
    "for_statement",
    "while_statement",
    "match_statement",
    "class_definition",
];

fn format_node(source: &str, node: Node, indent: usize, output: &mut String) {
    let kind = node.kind();

    // Handle comments directly
    if kind == "comment" {
        let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();
        if !text.is_empty() {
            let _ = writeln!(output, "{}{}", " ".repeat(indent * 4), text);
        }
        return;
    }

    // Handle blocks (functions, classes, ifs, loops, etc.)
    if BLOCK_KINDS.contains(&kind) {
        // Print only header line (first line)
        let header_line = node.utf8_text(source.as_bytes()).unwrap_or("").lines().next().unwrap_or("");
        let _ = writeln!(output, "{}{}", " ".repeat(indent * 4), header_line);

        // Recurse only into the block body
        for child in node.children(&mut node.walk()) {
            if child.start_byte() > node.start_byte() {
                format_node(source, child, indent + 1, output);
            }
        }
        return;
    }

    // Treat any leaf node or "expression" node as a single line
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();
    if !text.is_empty() {
        let _ = writeln!(output, "{}{}", " ".repeat(indent * 4), text);
    }
}
