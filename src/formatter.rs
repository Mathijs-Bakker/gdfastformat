use tree_sitter::{Node, Tree, };

/// Format `GDScript` source code based on Tree-sitter AST
pub fn format_tree(source: &str, tree: &Tree) -> String {
    let root = tree.root_node();
    let mut output = String::new();
    format_node(source, root, 0, &mut output);
    output
}

fn format_node(source: &str, node: Node, indent_level: usize, output: &mut String) {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();

    // Node kinds that introduce blocks
    let block_kinds = [
        "function_definition",
        "if_statement",
        "for_statement",
        "while_statement",
        "match_statement",
        "class_definition",
    ];

    // If this node is a block, print header, then recurse into children
    if block_kinds.contains(&node.kind()) {
        // Print the header line
        if !text.is_empty() {
            let header_line = text.split('\n').next().unwrap_or("");
            output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), header_line));
        }

        // Recurse into children with +1 indent
        for child in node.children(&mut node.walk()) {
            format_node(source, child, indent_level + 1, output);
        }
    } else if node.child_count() == 0 {
        // Leaf node
        if !text.is_empty() {
            output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), text));
        }
    } else {
        // Non-leaf, non-block node
        let mut inline_text = String::new();
        for child in node.children(&mut node.walk()) {
            let child_text = child.utf8_text(source.as_bytes()).unwrap_or("").trim();
            if child.child_count() == 0 {
                inline_text.push_str(child_text);
            } else {
                // For nested blocks, print accumulated inline text first
                if !inline_text.is_empty() {
                    output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), inline_text));
                    inline_text.clear();
                }
                format_node(source, child, indent_level, output);
            }
        }
        if !inline_text.is_empty() {
            output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), inline_text));
        }
    }
}
