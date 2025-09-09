use tree_sitter::{Node, Tree};

/// Format ``GDScript`` source code based on Tree-sitter AST.
pub fn format_tree(source: &str, tree: &Tree) -> String {
    let root = tree.root_node();
    let mut output = String::new();
    format_node(source, root, 0, &mut output);
    output
}

fn format_node(source: &str, node: Node, indent_level: usize, output: &mut String) {
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").trim();

    // List of node kinds that introduce blocks
    let block_kinds = ["function_definition", "if_statement", "for_statement",
                       "while_statement", "match_statement", "class_definition"];

    if block_kinds.contains(&node.kind()) {
        // Print header line
        output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), text.split('\n').next().unwrap_or("")));
        
        // Recurse children with +1 indent
        for child in node.children(&mut node.walk()) {
            format_node(source, child, indent_level + 1, output);
        }
    } else if node.child_count() == 0 {
        // Leaf node
        if !text.is_empty() {
            output.push_str(&format!("{}{}\n", " ".repeat(indent_level * 4), text));
        }
    } else {
        // Non-leaf, non-block
        for child in node.children(&mut node.walk()) {
            format_node(source, child, indent_level, output);
        }
    }
}
