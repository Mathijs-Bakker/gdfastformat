use tree_sitter::{Parser, Tree};
use tree_sitter_gdscript::LANGUAGE; 

pub fn parse_code(source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&LANGUAGE.into())
        .expect("Error loading GDScript grammar");
    parser.parse(source, None)
}
