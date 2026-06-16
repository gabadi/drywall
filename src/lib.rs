pub fn parse_rust_source(source: &str) -> bool {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to load Rust grammar");
    let tree = parser.parse(source, None).expect("parse returned None");
    !tree.root_node().has_error()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_rust_source_parses_without_error() {
        let source = "fn main() {}";
        assert!(parse_rust_source(source));
    }
}
