use tree_sitter::{Node, Tree};

/// Format GDScript source code based on Tree-sitter AST.
pub fn format_tree(source: &str, tree: &Tree) -> String {
    let root = tree.root_node();
    let mut output = String::new();
    format_node(source, root, 0, &mut output);
    output
}

fn format_node(source: &str, node: Node, indent_level: usize, output: &mut String) {
    let block_kinds = [
        "function_definition", "if_statement", "for_statement",
        "while_statement", "match_statement", "class_definition"
    ];

    let atomic_kinds = [
        "call_expression", "string_literal", "identifier", "number_literal",
        "expression_statement"
    ];

    let indent = " ".repeat(indent_level * 4);

    if block_kinds.contains(&node.kind()) {
        // Print the first line (header)
        output.push_str(&format!(
            "{}{}\n",
            indent,
            node.utf8_text(source.as_bytes())
                .unwrap_or("")
                .lines()
                .next()
                .unwrap_or("")
        ));

        // Recurse only into the body
        if let Some(body) = node.child_by_field_name("body") {
            for child in body.children(&mut body.walk()) {
                format_node(source, child, indent_level + 1, output);
            }
        }
    } else if atomic_kinds.contains(&node.kind()) {
        // Print atomic node as one line
        output.push_str(&format!(
            "{}{}\n",
            indent,
            &source[node.start_byte()..node.end_byte()]
        ));
    } else {
        // Recursively format other nodes
        for child in node.children(&mut node.walk()) {
            format_node(source, child, indent_level, output);
        }
    }
}
