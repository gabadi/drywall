use crate::FunctionInfo;

pub fn make_parser() -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to load Rust grammar");
    parser
}

pub fn parse_source_tree(source: &str) -> Result<tree_sitter::Tree, &'static str> {
    let tree = make_parser().parse(source, None).ok_or("parse returned None")?;
    if tree.root_node().has_error() {
        return Err("parse error");
    }
    Ok(tree)
}

pub fn children_of(node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut cursor = node.walk();
    node.children(&mut cursor).collect()
}

pub fn extract_functions(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    functions: &mut Vec<FunctionInfo>,
) {
    match node.kind() {
        "function_item" => {
            functions.push(make_function_info(node, source, file));
        }
        "let_declaration" => {
            extract_let_declaration(node, source, file, functions);
        }
        _ => {
            for child in children_of(node) {
                extract_functions(child, source, file, functions);
            }
        }
    }
}

pub fn extract_let_declaration(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    functions: &mut Vec<FunctionInfo>,
) {
    if let Some(val) = find_child_by_field(node, "value") && val.kind() == "closure_expression" {
        functions.push(make_function_info(node, source, file));
    }
    for child in children_of(node) {
        if child.kind() != "closure_expression" {
            extract_functions(child, source, file, functions);
        }
    }
}

pub fn find_child_by_field<'a>(
    node: tree_sitter::Node<'a>,
    field: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == field {
            return Some(child);
        }
    }
    node.child_by_field_name(field)
}

pub fn make_function_info(node: tree_sitter::Node, source: &str, file: &str) -> FunctionInfo {
    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;
    let hashes = collect_subtree_hashes(node, source);
    FunctionInfo {
        file: file.to_string(),
        start_line,
        end_line,
        node_hashes: hashes,
    }
}

fn normalize_node_text(node: tree_sitter::Node, source: &str) -> String {
    match node.kind() {
        "identifier" | "field_identifier" | "type_identifier" | "lifetime" => "_ID".to_string(),
        "integer_literal" | "float_literal" | "string_literal" | "raw_string_literal"
        | "char_literal" | "boolean_literal" => "_LIT".to_string(),
        _ => {
            let start = node.start_byte();
            let end = node.end_byte();
            source[start..end].to_string()
        }
    }
}

pub fn build_normalized_subtree(node: tree_sitter::Node, source: &str) -> String {
    if node.child_count() == 0 {
        return normalize_node_text(node, source);
    }
    let mut parts = vec![format!("({}:", node.kind())];
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        parts.push(build_normalized_subtree(child, source));
    }
    parts.push(")".to_string());
    parts.join("")
}

fn hash_str(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

fn collect_subtree_hashes(node: tree_sitter::Node, source: &str) -> Vec<u64> {
    let mut hashes = Vec::new();
    collect_hashes_recursive(node, source, &mut hashes);
    hashes.sort_unstable();
    hashes
}

fn collect_hashes_recursive(node: tree_sitter::Node, source: &str, hashes: &mut Vec<u64>) {
    let subtree = build_normalized_subtree(node, source);
    hashes.push(hash_str(&subtree));
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_hashes_recursive(child, source, hashes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_source_tree_valid_returns_ok() {
        assert!(parse_source_tree("fn main() {}").is_ok());
    }

    #[test]
    fn parse_source_tree_invalid_returns_err() {
        assert!(parse_source_tree("this @@ is not valid rust {{{{").is_err());
    }

    #[test]
    fn normalize_replaces_identifier() {
        let source = "fn foo() {}";
        let tree = make_parser().parse(source, None).unwrap();
        let subtree = build_normalized_subtree(tree.root_node(), source);
        assert!(subtree.contains("_ID"), "expected _ID in: {}", subtree);
    }

    fn run_extract_let_declaration(source: &str) -> Vec<FunctionInfo> {
        let tree = make_parser().parse(source, None).unwrap();
        let mut functions = Vec::new();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let children: Vec<_> = root.children(&mut cursor).collect();
        if let Some(let_node) = children.iter().find(|n| n.kind() == "let_declaration") {
            extract_let_declaration(*let_node, source, "test.rs", &mut functions);
        }
        functions
    }

    #[test]
    fn extract_let_declaration_with_closure_adds_function() {
        let functions = run_extract_let_declaration("let f = |x: i32| -> i32 { x + 1 };\n");
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn extract_let_declaration_without_closure_adds_no_function() {
        let functions = run_extract_let_declaration("let x = 42;\n");
        assert_eq!(functions.len(), 0);
    }

    #[test]
    fn find_child_by_field_finds_body() {
        let source = "fn foo() {}";
        let tree = make_parser().parse(source, None).unwrap();
        let func = tree.root_node().child(0).unwrap();
        assert!(find_child_by_field(func, "body").is_some());
    }

    #[test]
    fn find_child_by_field_returns_none_for_missing() {
        let source = "fn foo() {}";
        let tree = make_parser().parse(source, None).unwrap();
        assert!(find_child_by_field(tree.root_node(), "nonexistent").is_none());
    }
}
