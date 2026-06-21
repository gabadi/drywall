use crate::FunctionInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Rust,
    JavaScript,
    TypeScript,
}

pub struct LangConfig {
    pub lang: Lang,
    pub identifier_kinds: &'static [&'static str],
    pub literal_kinds: &'static [&'static str],
}

pub static RUST_CONFIG: LangConfig = LangConfig {
    lang: Lang::Rust,
    identifier_kinds: &[
        "identifier",
        "field_identifier",
        "type_identifier",
        "lifetime",
    ],
    literal_kinds: &[
        "integer_literal",
        "float_literal",
        "string_literal",
        "raw_string_literal",
        "char_literal",
        "boolean_literal",
    ],
};

pub static JS_CONFIG: LangConfig = LangConfig {
    lang: Lang::JavaScript,
    identifier_kinds: &[
        "identifier",
        "property_identifier",
        "shorthand_property_identifier",
        "private_property_identifier",
    ],
    literal_kinds: &[
        "number",
        "string",
        "template_string",
        "true",
        "false",
        "null",
        "undefined",
        "regex",
    ],
};

pub static TS_CONFIG: LangConfig = LangConfig {
    lang: Lang::TypeScript,
    identifier_kinds: &[
        "identifier",
        "property_identifier",
        "shorthand_property_identifier",
        "private_property_identifier",
        "type_identifier",
    ],
    literal_kinds: &[
        "number",
        "string",
        "template_string",
        "true",
        "false",
        "null",
        "undefined",
        "regex",
        "predefined_type",
    ],
};

pub fn make_parser_for(lang: Lang) -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    let language: tree_sitter::Language = match lang {
        Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
        Lang::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    };
    parser
        .set_language(&language)
        .expect("failed to load grammar");
    parser
}

pub fn make_parser() -> tree_sitter::Parser {
    make_parser_for(Lang::Rust)
}

pub fn lang_config(lang: Lang) -> &'static LangConfig {
    match lang {
        Lang::Rust => &RUST_CONFIG,
        Lang::JavaScript => &JS_CONFIG,
        Lang::TypeScript => &TS_CONFIG,
    }
}

pub fn parse_source_tree_for(source: &str, lang: Lang) -> Result<tree_sitter::Tree, &'static str> {
    let tree = make_parser_for(lang)
        .parse(source, None)
        .ok_or("parse returned None")?;
    if tree.root_node().has_error() {
        return Err("parse error");
    }
    Ok(tree)
}

pub fn parse_source_tree(source: &str) -> Result<tree_sitter::Tree, &'static str> {
    parse_source_tree_for(source, Lang::Rust)
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
    extract_functions_with_config(node, source, file, &RUST_CONFIG, functions);
}

pub fn extract_functions_with_config(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    match config.lang {
        Lang::Rust => extract_rust_functions(node, source, file, functions),
        Lang::JavaScript | Lang::TypeScript => {
            extract_jsts_functions(node, source, file, config, functions)
        }
    }
}

fn extract_rust_functions(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    functions: &mut Vec<FunctionInfo>,
) {
    match node.kind() {
        "function_item" => {
            functions.push(make_function_info(node, source, file, &RUST_CONFIG));
        }
        "let_declaration" => {
            extract_rust_let_declaration(node, source, file, functions);
        }
        _ => {
            for child in children_of(node) {
                extract_rust_functions(child, source, file, functions);
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
    extract_rust_let_declaration(node, source, file, functions);
}

fn extract_rust_let_declaration(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    functions: &mut Vec<FunctionInfo>,
) {
    if let Some(val) = find_child_by_field(node, "value")
        && val.kind() == "closure_expression"
    {
        functions.push(make_function_info(node, source, file, &RUST_CONFIG));
    }
    for child in children_of(node) {
        if child.kind() != "closure_expression" {
            extract_rust_functions(child, source, file, functions);
        }
    }
}

fn extract_jsts_export_statement(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    let fn_decls: Vec<_> = children_of(node)
        .into_iter()
        .filter(|c| c.kind() == "function_declaration")
        .collect();
    if fn_decls.is_empty() {
        for child in children_of(node) {
            extract_jsts_functions(child, source, file, config, functions);
        }
    } else {
        for child in fn_decls {
            functions.push(make_function_info(child, source, file, config));
        }
    }
}

fn extract_jsts_lexical_declaration(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    for child in children_of(node) {
        if child.kind() == "variable_declarator" {
            extract_jsts_variable_declarator(child, source, file, config, functions);
        }
    }
}

fn extract_jsts_class_body(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    for child in children_of(node) {
        if child.kind() == "method_definition" {
            functions.push(make_function_info(child, source, file, config));
        }
    }
}

fn try_extract_jsts_node(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) -> bool {
    match node.kind() {
        "function_declaration" => functions.push(make_function_info(node, source, file, config)),
        "export_statement" => extract_jsts_export_statement(node, source, file, config, functions),
        "lexical_declaration" => {
            extract_jsts_lexical_declaration(node, source, file, config, functions)
        }
        "class_body" => extract_jsts_class_body(node, source, file, config, functions),
        _ => return false,
    }
    true
}

fn extract_jsts_functions(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    if !try_extract_jsts_node(node, source, file, config, functions) {
        for child in children_of(node) {
            extract_jsts_functions(child, source, file, config, functions);
        }
    }
}

fn extract_jsts_variable_declarator(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
    functions: &mut Vec<FunctionInfo>,
) {
    if let Some(val) = find_child_by_field(node, "value")
        && val.kind() == "arrow_function"
    {
        functions.push(make_function_info(node, source, file, config));
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

pub fn make_function_info(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    config: &LangConfig,
) -> FunctionInfo {
    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;
    let hashes = collect_subtree_hashes(node, source, config);
    FunctionInfo {
        file: file.to_string(),
        start_line,
        end_line,
        node_hashes: hashes,
    }
}

fn normalize_node_text(node: tree_sitter::Node, source: &str, config: &LangConfig) -> String {
    let kind = node.kind();
    if config.identifier_kinds.contains(&kind) {
        return "_ID".to_string();
    }
    if config.literal_kinds.contains(&kind) {
        return "_LIT".to_string();
    }
    let start = node.start_byte();
    let end = node.end_byte();
    source[start..end].to_string()
}

pub fn build_normalized_subtree(
    node: tree_sitter::Node,
    source: &str,
    config: &LangConfig,
) -> String {
    if node.child_count() == 0 {
        return normalize_node_text(node, source, config);
    }
    let mut parts = vec![format!("({}:", node.kind())];
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        parts.push(build_normalized_subtree(child, source, config));
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

fn collect_subtree_hashes(node: tree_sitter::Node, source: &str, config: &LangConfig) -> Vec<u64> {
    let mut hashes = Vec::new();
    collect_hashes_recursive(node, source, config, &mut hashes);
    hashes.sort_unstable();
    hashes
}

fn collect_hashes_recursive(
    node: tree_sitter::Node,
    source: &str,
    config: &LangConfig,
    hashes: &mut Vec<u64>,
) {
    let subtree = build_normalized_subtree(node, source, config);
    hashes.push(hash_str(&subtree));
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_hashes_recursive(child, source, config, hashes);
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
        let subtree = build_normalized_subtree(tree.root_node(), source, &RUST_CONFIG);
        assert!(subtree.contains("_ID"), "expected _ID in: {}", subtree);
    }

    fn run_extract_let_declaration(source: &str) -> Vec<FunctionInfo> {
        let tree = make_parser().parse(source, None).unwrap();
        let mut functions = Vec::new();
        let root = tree.root_node();
        let mut cursor = root.walk();
        let children: Vec<_> = root.children(&mut cursor).collect();
        if let Some(let_node) = children.iter().find(|n| n.kind() == "let_declaration") {
            extract_rust_let_declaration(*let_node, source, "test.rs", &mut functions);
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

    // --- JS/TS unit tests ---

    fn make_js_parser() -> tree_sitter::Parser {
        make_parser_for(Lang::JavaScript)
    }

    fn make_ts_parser() -> tree_sitter::Parser {
        make_parser_for(Lang::TypeScript)
    }

    #[test]
    fn js_grammar_loads_and_parses_function() {
        let mut parser = make_js_parser();
        let tree = parser.parse("function f() {}", None).unwrap();
        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn ts_grammar_loads_and_parses_function() {
        let mut parser = make_ts_parser();
        let tree = parser.parse("function f(): void {}", None).unwrap();
        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn js_twin_functions_same_structure_different_ids_score_high() {
        let src_a = "function accumulate_sum(a, b) {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let src_b = "function accumulate_sum(x, y) {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        let mut parser = make_js_parser();
        let tree_a = parser.parse(src_a, None).unwrap();
        let tree_b = parser.parse(src_b, None).unwrap();
        let mut funcs_a = Vec::new();
        let mut funcs_b = Vec::new();
        extract_functions_with_config(tree_a.root_node(), src_a, "a.js", &JS_CONFIG, &mut funcs_a);
        extract_functions_with_config(tree_b.root_node(), src_b, "b.js", &JS_CONFIG, &mut funcs_b);
        assert_eq!(funcs_a.len(), 1, "expected 1 function in a.js");
        assert_eq!(funcs_b.len(), 1, "expected 1 function in b.js");
        let score = crate::jaccard(&funcs_a[0].node_hashes, &funcs_b[0].node_hashes);
        assert!(score >= 0.82, "expected score >= 0.82, got {}", score);
    }

    #[test]
    fn ts_twin_functions_same_structure_different_ids_score_high() {
        let src_a = "function accumulate_sum(a: number, b: number): number {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let src_b = "function accumulate_sum(x: number, y: number): number {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        let mut parser = make_ts_parser();
        let tree_a = parser.parse(src_a, None).unwrap();
        let tree_b = parser.parse(src_b, None).unwrap();
        let mut funcs_a = Vec::new();
        let mut funcs_b = Vec::new();
        extract_functions_with_config(tree_a.root_node(), src_a, "a.ts", &TS_CONFIG, &mut funcs_a);
        extract_functions_with_config(tree_b.root_node(), src_b, "b.ts", &TS_CONFIG, &mut funcs_b);
        assert_eq!(funcs_a.len(), 1);
        assert_eq!(funcs_b.len(), 1);
        let score = crate::jaccard(&funcs_a[0].node_hashes, &funcs_b[0].node_hashes);
        assert!(score >= 0.82, "expected score >= 0.82, got {}", score);
    }

    #[test]
    fn js_arrow_function_const_is_extracted() {
        let src = "const f = (a, b) => {\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  return e;\n};\n";
        let mut parser = make_js_parser();
        let tree = parser.parse(src, None).unwrap();
        let mut funcs = Vec::new();
        extract_functions_with_config(tree.root_node(), src, "f.js", &JS_CONFIG, &mut funcs);
        assert_eq!(funcs.len(), 1, "const arrow function should be extracted");
    }

    #[test]
    fn ts_arrow_function_const_is_extracted() {
        let src = "const f = (a: number, b: number): number => {\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  return e;\n};\n";
        let mut parser = make_ts_parser();
        let tree = parser.parse(src, None).unwrap();
        let mut funcs = Vec::new();
        extract_functions_with_config(tree.root_node(), src, "f.ts", &TS_CONFIG, &mut funcs);
        assert_eq!(funcs.len(), 1, "const arrow function should be extracted");
    }

    #[test]
    fn js_class_method_is_extracted() {
        let src = "class Foo {\n  accumulate(a, b) {\n    let c = a + b;\n    let d = c * 2;\n    let e = d + a;\n    return e;\n  }\n}\n";
        let mut parser = make_js_parser();
        let tree = parser.parse(src, None).unwrap();
        let mut funcs = Vec::new();
        extract_functions_with_config(tree.root_node(), src, "f.js", &JS_CONFIG, &mut funcs);
        assert_eq!(funcs.len(), 1, "class method should be extracted");
    }

    #[test]
    fn js_exported_function_is_extracted() {
        let src = "export function compute(a, b) {\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  return e;\n}\n";
        let mut parser = make_js_parser();
        let tree = parser.parse(src, None).unwrap();
        let mut funcs = Vec::new();
        extract_functions_with_config(tree.root_node(), src, "f.js", &JS_CONFIG, &mut funcs);
        assert_eq!(
            funcs.len(),
            1,
            "exported named function should be extracted"
        );
    }

    #[test]
    fn rust_and_js_functions_with_same_structure_do_not_pair() {
        // Rust and JS use different grammar node kinds, so hashes differ.
        // The accumulate_sum pattern should not score >= threshold between grammars.
        let rust_src = "pub fn accumulate_sum(a: i32, b: i32) -> i32 {\n    let sum = a + b;\n    let extra = sum * 2;\n    let more = extra + a;\n    let result = more + b;\n    result\n}\n";
        let js_src = "function accumulate_sum(a, b) {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let mut rust_parser = make_parser();
        let mut js_parser = make_js_parser();
        let rust_tree = rust_parser.parse(rust_src, None).unwrap();
        let js_tree = js_parser.parse(js_src, None).unwrap();
        let mut rust_funcs = Vec::new();
        let mut js_funcs = Vec::new();
        extract_functions(rust_tree.root_node(), rust_src, "a.rs", &mut rust_funcs);
        extract_functions_with_config(
            js_tree.root_node(),
            js_src,
            "b.js",
            &JS_CONFIG,
            &mut js_funcs,
        );
        assert_eq!(rust_funcs.len(), 1);
        assert_eq!(js_funcs.len(), 1);
        let score = crate::jaccard(&rust_funcs[0].node_hashes, &js_funcs[0].node_hashes);
        // Cross-language pairs should not score >= 0.82
        assert!(
            score < 0.82,
            "cross-language pair should score < 0.82, got {}",
            score
        );
    }

    #[test]
    fn js_parse_error_detected() {
        let bad_js = "function {{ INVALID JS SYNTAX (((;\n";
        assert!(parse_source_tree_for(bad_js, Lang::JavaScript).is_err());
    }

    #[test]
    fn ts_parse_error_detected() {
        let bad_ts = "class {{ INVALID TS SYNTAX (((;\n";
        assert!(parse_source_tree_for(bad_ts, Lang::TypeScript).is_err());
    }
}
