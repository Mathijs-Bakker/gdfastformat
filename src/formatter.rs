use tree_sitter::{Node, TreeCursor, Language};

/// Recursively write a node and its children into the output string with correct indentation.
pub fn write_node(node: Node, source: &str, output: &mut String, indent: usize) {
    let kind = node.kind();

    // Handle leaf nodes
    if node.child_count() == 0 {
        let text = source[node.byte_range()].trim();
        if !text.is_empty() {
            output.push_str(text);
        }
        return;
    }

    match kind {
        // Structural nodes that introduce indentation
        "class" | "func" | "if" | "for" | "while" => {
            output.push_str(&"    ".repeat(indent));
            for child in node.named_children(&mut node.walk()) {
                write_node(child, source, output, indent);
            }
            output.push('\n');
        }

        // Blocks of code (body of function, if, etc.) — indent children
        "block" | "func_body" | "if_body" | "for_body" | "while_body" => {
            for child in node.named_children(&mut node.walk()) {
                output.push_str(&"    ".repeat(indent + 1));
                write_node(child, source, output, indent + 1);
                output.push('\n');
            }
        }

        // Comments — preserve exactly
        "comment" => {
            output.push_str(&"    ".repeat(indent));
            output.push_str(source[node.byte_range()].trim());
            output.push('\n');
        }

        // Multiline strings — preserve exactly
        "string" => {
            output.push_str(source[node.byte_range()].trim());
        }

        // Other nodes — default: recurse
        _ => {
            for child in node.named_children(&mut node.walk()) {
                write_node(child, source, output, indent);
            }
        }
    }
}

/// Top-level formatter function
pub fn format_code(root: Node, source: &str) -> String {
    let mut output = String::new();
    write_node(root, source, &mut output, 0);
    output
}
