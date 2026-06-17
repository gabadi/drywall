use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn parse_rust_source(source: &str) -> bool {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to load Rust grammar");
    let tree = parser.parse(source, None).expect("parse returned None");
    !tree.root_node().has_error()
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub node_hashes: Vec<u64>,
}

#[derive(Debug, Serialize)]
pub struct PairEndpoint {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub nodes: usize,
}

#[derive(Debug, Serialize)]
pub struct DuplicatePair {
    pub score: f64,
    pub left: PairEndpoint,
    pub right: PairEndpoint,
}

pub struct Config {
    pub threshold: f64,
    pub min_lines: usize,
    pub min_nodes: usize,
    pub format: OutputFormat,
    pub excludes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threshold: 0.82,
            min_lines: 4,
            min_nodes: 20,
            format: OutputFormat::Text,
            excludes: vec![],
        }
    }
}

pub enum OutputFormat {
    Text,
    Json,
}

pub enum RunResult {
    Clean,
    Duplicates(Vec<DuplicatePair>),
    Error(String),
}

pub fn run(paths: &[String], config: &Config) -> RunResult {
    let exclude_set = match build_glob_set(&config.excludes) {
        Ok(gs) => gs,
        Err(e) => return RunResult::Error(format!("invalid exclude glob: {}", e)),
    };

    let mut functions: Vec<FunctionInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            errors.push(format!("path does not exist: {}", path_str));
            continue;
        }
        collect_functions_from_path(path, &exclude_set, &mut functions, &mut errors);
    }

    if !errors.is_empty() {
        return RunResult::Error(errors.join("\n"));
    }

    let pairs = find_duplicate_pairs(&functions, config.threshold, config.min_lines, config.min_nodes);

    if pairs.is_empty() {
        RunResult::Clean
    } else {
        RunResult::Duplicates(pairs)
    }
}

fn build_glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat)?);
    }
    builder.build()
}

fn collect_functions_from_path(
    path: &Path,
    exclude_set: &GlobSet,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    if path.is_file() {
        if should_skip(path, exclude_set) {
            return;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            process_file(path, functions, errors);
        }
    } else {
        for entry in WalkDir::new(path).sort_by_file_name() {
            match entry {
                Ok(e) => {
                    let ep = e.path();
                    if should_skip(ep, exclude_set) {
                        continue;
                    }
                    if e.file_type().is_file()
                        && ep.extension().and_then(|x| x.to_str()) == Some("rs")
                    {
                        process_file(ep, functions, errors);
                    }
                }
                Err(err) => {
                    errors.push(format!("walk error: {}", err));
                }
            }
        }
    }
}

fn should_skip(path: &Path, exclude_set: &GlobSet) -> bool {
    if exclude_set.is_empty() {
        return false;
    }
    exclude_set.is_match(path)
}

fn process_file(path: &Path, functions: &mut Vec<FunctionInfo>, errors: &mut Vec<String>) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("cannot read {}: {}", path.display(), e));
            return;
        }
    };

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to load Rust grammar");

    let tree = match parser.parse(&source, None) {
        Some(t) => t,
        None => {
            errors.push(format!("parse returned None for {}", path.display()));
            return;
        }
    };

    if tree.root_node().has_error() {
        errors.push(format!("parse error in {}", path.display()));
        return;
    }

    let file_str = path.to_string_lossy().to_string();
    extract_functions(tree.root_node(), &source, &file_str, functions);
}

fn extract_functions(
    node: tree_sitter::Node,
    source: &str,
    file: &str,
    functions: &mut Vec<FunctionInfo>,
) {
    match node.kind() {
        "function_item" => {
            let info = make_function_info(node, source, file);
            functions.push(info);
        }
        "let_declaration" => {
            if let Some(val) = find_child_by_field(node, "value") {
                if val.kind() == "closure_expression" {
                    let info = make_function_info(node, source, file);
                    functions.push(info);
                }
            }
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();
            for child in children {
                if child.kind() != "closure_expression" {
                    extract_functions(child, source, file, functions);
                }
            }
        }
        _ => {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();
            for child in children {
                extract_functions(child, source, file, functions);
            }
        }
    }
}

fn find_child_by_field<'a>(
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

fn make_function_info(node: tree_sitter::Node, source: &str, file: &str) -> FunctionInfo {
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
        "identifier" | "field_identifier" | "type_identifier" | "lifetime" => {
            "_ID".to_string()
        }
        "integer_literal"
        | "float_literal"
        | "string_literal"
        | "raw_string_literal"
        | "char_literal"
        | "boolean_literal" => "_LIT".to_string(),
        _ => {
            let start = node.start_byte();
            let end = node.end_byte();
            source[start..end].to_string()
        }
    }
}

fn build_normalized_subtree(node: tree_sitter::Node, source: &str) -> String {
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

fn jaccard(a: &[u64], b: &[u64]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let set_a: HashSet<u64> = a.iter().copied().collect();
    let set_b: HashSet<u64> = b.iter().copied().collect();
    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();
    if union == 0 {
        return 0.0;
    }
    intersection as f64 / union as f64
}

fn source_lines(fi: &FunctionInfo) -> usize {
    fi.end_line.saturating_sub(fi.start_line) + 1
}

fn find_duplicate_pairs(
    functions: &[FunctionInfo],
    threshold: f64,
    min_lines: usize,
    min_nodes: usize,
) -> Vec<DuplicatePair> {
    let candidates: Vec<&FunctionInfo> = functions
        .iter()
        .filter(|f| source_lines(f) >= min_lines && f.node_hashes.len() >= min_nodes)
        .collect();

    let mut pairs: Vec<DuplicatePair> = Vec::new();

    for i in 0..candidates.len() {
        for j in (i + 1)..candidates.len() {
            let a = candidates[i];
            let b = candidates[j];
            if a.file == b.file && a.start_line == b.start_line {
                continue;
            }
            let score = jaccard(&a.node_hashes, &b.node_hashes);
            if score >= threshold {
                let (left, right) = order_endpoints(a, b);
                pairs.push(DuplicatePair {
                    score,
                    left: PairEndpoint {
                        file: left.file.clone(),
                        start_line: left.start_line,
                        end_line: left.end_line,
                        nodes: left.node_hashes.len(),
                    },
                    right: PairEndpoint {
                        file: right.file.clone(),
                        start_line: right.start_line,
                        end_line: right.end_line,
                        nodes: right.node_hashes.len(),
                    },
                });
            }
        }
    }

    pairs.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.left.file.cmp(&b.left.file))
            .then_with(|| a.left.start_line.cmp(&b.left.start_line))
            .then_with(|| a.right.file.cmp(&b.right.file))
            .then_with(|| a.right.start_line.cmp(&b.right.start_line))
    });

    pairs
}

fn order_endpoints<'a>(
    a: &'a FunctionInfo,
    b: &'a FunctionInfo,
) -> (&'a FunctionInfo, &'a FunctionInfo) {
    let cmp = a
        .file
        .cmp(&b.file)
        .then_with(|| a.start_line.cmp(&b.start_line));
    if cmp == std::cmp::Ordering::Greater {
        (b, a)
    } else {
        (a, b)
    }
}

pub fn format_text(pairs: &[DuplicatePair]) -> String {
    let mut out = String::new();
    for p in pairs {
        out.push_str(&format!("DUPLICATE score={:.4}\n", p.score));
        out.push_str(&format!(
            "  {}:{}-{}\n",
            p.left.file, p.left.start_line, p.left.end_line
        ));
        out.push_str(&format!(
            "  {}:{}-{}  ({} nodes / {} nodes)\n",
            p.right.file,
            p.right.start_line,
            p.right.end_line,
            p.left.nodes,
            p.right.nodes
        ));
    }
    out
}

pub fn format_json(pairs: &[DuplicatePair]) -> String {
    serde_json::to_string_pretty(pairs).unwrap_or_else(|_| "[]".to_string())
}

pub fn collect_rust_files(paths: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for p in paths {
        let path = Path::new(p);
        if path.is_file() {
            files.push(path.to_path_buf());
        } else {
            for entry in WalkDir::new(path).sort_by_file_name() {
                if let Ok(e) = entry {
                    if e.file_type().is_file()
                        && e.path().extension().and_then(|x| x.to_str()) == Some("rs")
                    {
                        files.push(e.path().to_path_buf());
                    }
                }
            }
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_rust_source_parses_without_error() {
        let source = "fn main() {}";
        assert!(parse_rust_source(source));
    }

    #[test]
    fn invalid_rust_source_returns_false() {
        let source = "this is not valid rust @@@@";
        assert!(!parse_rust_source(source));
    }

    #[test]
    fn jaccard_identical_sets_returns_one() {
        let a = vec![1u64, 2, 3];
        let b = vec![1u64, 2, 3];
        let score = jaccard(&a, &b);
        assert!((score - 1.0).abs() < 1e-9);
    }

    #[test]
    fn jaccard_disjoint_sets_returns_zero() {
        let a = vec![1u64, 2, 3];
        let b = vec![4u64, 5, 6];
        let score = jaccard(&a, &b);
        assert!(score < 1e-9);
    }

    #[test]
    fn jaccard_partial_overlap() {
        let a = vec![1u64, 2, 3, 4];
        let b = vec![3u64, 4, 5, 6];
        let score = jaccard(&a, &b);
        // intersection={3,4}, union={1,2,3,4,5,6} => 2/6
        assert!((score - 2.0 / 6.0).abs() < 1e-9);
    }

    #[test]
    fn normalize_replaces_identifier() {
        let source = "fn foo() {}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        let subtree = build_normalized_subtree(root, source);
        assert!(subtree.contains("_ID"), "expected _ID in: {}", subtree);
    }

    #[test]
    fn duplicate_functions_detected() {
        let source = r#"
fn alpha(a: i32, b: i32) -> i32 {
    let result = a + b;
    result + a
}

fn beta(x: i32, y: i32) -> i32 {
    let result = x + y;
    result + x
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut functions = Vec::new();
        extract_functions(tree.root_node(), source, "test.rs", &mut functions);
        assert_eq!(functions.len(), 2);
        let score = jaccard(&functions[0].node_hashes, &functions[1].node_hashes);
        assert!(score >= 0.82, "expected score >= 0.82, got {}", score);
    }

    #[test]
    fn unrelated_functions_not_duplicates() {
        let source = r#"
fn alpha(a: i32, b: i32) -> i32 {
    let result = a + b;
    result + a
}

fn parse_cfg(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('=').collect();
    parts.len() == 2
}
"#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut functions = Vec::new();
        extract_functions(tree.root_node(), source, "test.rs", &mut functions);
        assert_eq!(functions.len(), 2);
        let score = jaccard(&functions[0].node_hashes, &functions[1].node_hashes);
        assert!(score < 0.82, "expected score < 0.82, got {}", score);
    }

    #[test]
    fn function_below_min_lines_not_candidate() {
        // 3-line function (start+body+end on same few lines)
        let source = "fn tiny(x: i32) -> i32 { x + 1 }\n";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut functions = Vec::new();
        extract_functions(tree.root_node(), source, "test.rs", &mut functions);
        // line count = 1 line; well below min_lines=4
        for f in &functions {
            assert!(
                source_lines(f) < 4,
                "expected tiny function to be < 4 lines"
            );
        }
    }

    #[test]
    fn pairs_sorted_score_descending() {
        let f1 = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 4, 5],
        };
        let f2 = FunctionInfo {
            file: "b.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 4, 5],
        };
        let f3 = FunctionInfo {
            file: "c.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 6, 7],
        };
        let f4 = FunctionInfo {
            file: "d.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 6, 7],
        };
        let functions = vec![f1, f2, f3, f4];
        let pairs = find_duplicate_pairs(&functions, 0.5, 1, 1);
        assert!(pairs.len() >= 2);
        assert!(pairs[0].score >= pairs[1].score);
    }

    #[test]
    fn json_output_empty_array_when_no_pairs() {
        let output = format_json(&[]);
        assert_eq!(output.trim(), "[]");
    }

    #[test]
    fn text_output_contains_duplicate_header() {
        let pair = DuplicatePair {
            score: 0.95,
            left: PairEndpoint {
                file: "a.rs".to_string(),
                start_line: 1,
                end_line: 5,
                nodes: 10,
            },
            right: PairEndpoint {
                file: "b.rs".to_string(),
                start_line: 1,
                end_line: 5,
                nodes: 10,
            },
        };
        let output = format_text(&[pair]);
        assert!(output.contains("DUPLICATE score="));
        assert!(output.contains("a.rs"));
        assert!(output.contains("b.rs"));
    }
}
