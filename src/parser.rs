use tree_sitter::{Parser, Tree, Node};
use tree_sitter_gdscript::LANGUAGE;

pub struct GdscriptParser {
    parser: Parser,
}

impl GdscriptParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        // Convert LANGUAGE (LanguageFn) into Language correctly
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading GDScript grammar");
        Self { parser }
    }

    pub fn parse(&mut self, source: &str) -> Result<Tree, String> {
        match self.parser.parse(source, None) {
            Some(tree) => {
                if tree.root_node().has_error() {
                    Err("Syntax errors detected in GDScript source".to_string())
                } else {
                    Ok(tree)
                }
            }
            None => Err("Failed to parse GDScript source".to_string()),
        }
    }

    pub fn walk_tree(&mut self, source: &str) {
        if let Ok(tree) = self.parse(source) {
            let root = tree.root_node();
            self.visit_node(root, source);
        }
    }

    fn visit_node(&self, node: Node, source: &str) {
        println!(
            "{}: {}",
            node.kind(),
            &source[node.start_byte()..node.end_byte()]
        );
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_node(child, source);
            }
        }
    }
}
