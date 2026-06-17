mod ast;

use ast::{extract_functions, parse_source_tree};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn parse_rust_source(source: &str) -> bool {
    parse_source_tree(source).is_ok()
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

pub fn parse_output_format(s: &str) -> Result<OutputFormat, String> {
    match s {
        "json" => Ok(OutputFormat::Json),
        "text" => Ok(OutputFormat::Text),
        other => Err(format!(
            "unknown format '{}'; use 'text' or 'json'",
            other
        )),
    }
}

pub fn validate_lang(lang: &str) -> Result<(), String> {
    if lang == "rust" {
        Ok(())
    } else {
        Err(format!(
            "unsupported language '{}'; only 'rust' is supported",
            lang
        ))
    }
}

pub fn run(paths: &[String], config: &Config) -> RunResult {
    let exclude_set = match build_glob_set(&config.excludes) {
        Ok(gs) => gs,
        Err(e) => return RunResult::Error(format!("invalid exclude glob: {}", e)),
    };

    let (functions, errors) = collect_all_functions(paths, &exclude_set);

    if !errors.is_empty() {
        return RunResult::Error(errors.join("\n"));
    }

    let pairs = find_duplicate_pairs(
        &functions,
        config.threshold,
        config.min_lines,
        config.min_nodes,
    );

    if pairs.is_empty() {
        RunResult::Clean
    } else {
        RunResult::Duplicates(pairs)
    }
}

fn collect_all_functions(
    paths: &[String],
    exclude_set: &GlobSet,
) -> (Vec<FunctionInfo>, Vec<String>) {
    let mut functions: Vec<FunctionInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            errors.push(format!("path does not exist: {}", path_str));
        } else {
            collect_functions_from_path(path, exclude_set, &mut functions, &mut errors);
        }
    }
    (functions, errors)
}

fn build_glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat)?);
    }
    builder.build()
}

fn is_rust_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("rs")
}

fn collect_functions_from_path(
    path: &Path,
    exclude_set: &GlobSet,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    if path.is_file() {
        collect_from_single_file(path, exclude_set, functions, errors);
    } else {
        collect_from_directory(path, exclude_set, functions, errors);
    }
}

fn collect_from_single_file(
    path: &Path,
    exclude_set: &GlobSet,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    if !should_skip(path, exclude_set) && is_rust_file(path) {
        process_file(path, functions, errors);
    }
}

fn should_process_entry(e: &walkdir::DirEntry, exclude_set: &GlobSet) -> bool {
    let ep = e.path();
    !should_skip(ep, exclude_set) && e.file_type().is_file() && is_rust_file(ep)
}

fn collect_from_directory(
    path: &Path,
    exclude_set: &GlobSet,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    for entry in WalkDir::new(path).sort_by_file_name() {
        match entry {
            Ok(e) if should_process_entry(&e, exclude_set) => {
                process_file(e.path(), functions, errors);
            }
            Ok(_) => {}
            Err(err) => {
                errors.push(format!("walk error: {}", err));
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

    match parse_source_tree(&source) {
        Ok(tree) => {
            let file_str = path.to_string_lossy().to_string();
            extract_functions(tree.root_node(), &source, &file_str, functions);
        }
        Err(msg) => {
            errors.push(format!("{} in {}", msg, path.display()));
        }
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

fn make_pair(a: &FunctionInfo, b: &FunctionInfo, score: f64) -> DuplicatePair {
    let (left, right) = order_endpoints(a, b);
    DuplicatePair {
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
    }
}

fn is_same_location(a: &FunctionInfo, b: &FunctionInfo) -> bool {
    a.file == b.file && a.start_line == b.start_line
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
            if is_same_location(a, b) {
                continue;
            }
            let score = jaccard(&a.node_hashes, &b.node_hashes);
            if score >= threshold {
                pairs.push(make_pair(a, b, score));
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
            p.right.file, p.right.start_line, p.right.end_line, p.left.nodes, p.right.nodes
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
            for entry in WalkDir::new(path).sort_by_file_name().into_iter().flatten() {
                if entry.file_type().is_file()
                    && entry.path().extension().and_then(|x| x.to_str()) == Some("rs")
                {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::extract_functions;

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

    #[test]
    fn jaccard_both_empty_returns_one() {
        let score = jaccard(&[], &[]);
        assert!((score - 1.0).abs() < 1e-9);
    }

    #[test]
    fn build_glob_set_empty_patterns_builds_empty_set() {
        let gs = build_glob_set(&[]).unwrap();
        assert!(gs.is_empty());
    }

    #[test]
    fn build_glob_set_valid_pattern_matches() {
        let patterns = vec!["acceptance/**".to_string()];
        let gs = build_glob_set(&patterns).unwrap();
        assert!(gs.is_match("acceptance/steps/foo.rs"));
        assert!(!gs.is_match("src/lib.rs"));
    }

    #[test]
    fn build_glob_set_invalid_pattern_returns_error() {
        let patterns = vec!["[invalid".to_string()];
        assert!(build_glob_set(&patterns).is_err());
    }

    #[test]
    fn should_skip_empty_set_never_skips() {
        let gs = build_glob_set(&[]).unwrap();
        assert!(!should_skip(std::path::Path::new("anything.rs"), &gs));
    }

    #[test]
    fn should_skip_matching_path_skips() {
        let patterns = vec!["acceptance/**".to_string()];
        let gs = build_glob_set(&patterns).unwrap();
        assert!(should_skip(
            std::path::Path::new("acceptance/steps/foo.rs"),
            &gs
        ));
    }

    #[test]
    fn should_skip_non_matching_path_does_not_skip() {
        let patterns = vec!["acceptance/**".to_string()];
        let gs = build_glob_set(&patterns).unwrap();
        assert!(!should_skip(std::path::Path::new("src/lib.rs"), &gs));
    }

    #[test]
    fn process_file_valid_rust_adds_functions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rs");
        std::fs::write(
            &path,
            r#"fn alpha(a: i32) -> i32 {
    let x = a + 1;
    x * 2
}

fn beta(b: i32) -> i32 {
    let y = b + 1;
    y * 2
}
"#,
        )
        .unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        process_file(&path, &mut functions, &mut errors);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn process_file_unreadable_path_pushes_error() {
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        process_file(
            std::path::Path::new("/nonexistent/path/file.rs"),
            &mut functions,
            &mut errors,
        );
        assert_eq!(functions.len(), 0);
        assert!(!errors.is_empty());
    }

    #[test]
    fn process_file_invalid_rust_pushes_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.rs");
        std::fs::write(&path, "this @@ is not valid rust {{{{").unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        process_file(&path, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(!errors.is_empty());
    }

    #[test]
    fn collect_rust_files_from_directory_finds_rs_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.rs"), "fn a() {}").unwrap();
        std::fs::write(dir.path().join("b.rs"), "fn b() {}").unwrap();
        std::fs::write(dir.path().join("c.txt"), "not rust").unwrap();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let files = collect_rust_files(&paths);
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.extension().unwrap() == "rs"));
    }

    #[test]
    fn collect_rust_files_from_file_path_returns_that_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("single.rs");
        std::fs::write(&path, "fn x() {}").unwrap();
        let paths = vec![path.to_string_lossy().to_string()];
        let files = collect_rust_files(&paths);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], path);
    }

    #[test]
    fn collect_functions_from_path_single_file_no_exclude() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("funcs.rs");
        std::fs::write(
            &path,
            r#"fn one(x: i32) -> i32 {
    let a = x + 1;
    a * 2
}

fn two(y: i32) -> i32 {
    let b = y + 1;
    b * 2
}
"#,
        )
        .unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_functions_from_path(&path, &gs, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn collect_functions_from_path_skips_excluded_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("skip_me.rs");
        std::fs::write(&path, "fn foo() {}").unwrap();
        let pattern = path.to_string_lossy().to_string();
        let gs = build_glob_set(&[pattern]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_functions_from_path(&path, &gs, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_functions_from_path_directory_finds_rs_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("code.rs"),
            r#"fn alpha(a: i32) -> i32 {
    let x = a + 1;
    x * 2
}
"#,
        )
        .unwrap();
        std::fs::write(dir.path().join("readme.txt"), "not rust").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_functions_from_path(dir.path(), &gs, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn run_nonexistent_path_returns_error() {
        let config = Config::default();
        let paths = vec!["/nonexistent/does/not/exist".to_string()];
        let result = run(&paths, &config);
        assert!(matches!(result, RunResult::Error(_)));
    }

    #[test]
    fn run_clean_directory_returns_clean() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("unique.rs"),
            r#"fn solo(x: i32) -> i32 {
    let a = x + 1;
    a * 2
}
"#,
        )
        .unwrap();
        let config = Config::default();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        assert!(matches!(result, RunResult::Clean));
    }

    #[test]
    fn run_duplicate_files_returns_duplicates() {
        let dir = tempfile::tempdir().unwrap();
        let body = r#"fn alpha(a: i32, b: i32) -> i32 {
    let result = a + b;
    result + a
}

fn beta(x: i32, y: i32) -> i32 {
    let result = x + y;
    result + x
}
"#;
        std::fs::write(dir.path().join("dup.rs"), body).unwrap();
        let config = Config::default();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        assert!(matches!(result, RunResult::Duplicates(_)));
    }

    #[test]
    fn run_invalid_glob_returns_error() {
        let config = Config {
            excludes: vec!["[invalid".to_string()],
            ..Config::default()
        };
        let paths = vec![".".to_string()];
        let result = run(&paths, &config);
        assert!(matches!(result, RunResult::Error(_)));
    }

    #[test]
    fn extract_functions_finds_closure_in_top_level_let() {
        let source = "static HANDLER: fn(i32) -> i32 = |x| x + 1;\n";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut functions = Vec::new();
        extract_functions(tree.root_node(), source, "test.rs", &mut functions);
        assert_eq!(functions.len(), 0, "static with non-closure value, no functions expected");
    }

    #[test]
    fn collect_functions_from_path_non_rs_file_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("readme.txt");
        std::fs::write(&path, "not rust").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_functions_from_path(&path, &gs, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_functions_from_path_directory_excluded_file_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let rs_file = dir.path().join("skip.rs");
        std::fs::write(&rs_file, "fn foo() {}").unwrap();
        let gs = build_glob_set(&[rs_file.to_string_lossy().to_string()]).unwrap();
        let (functions, errors) = {
            let mut f = Vec::new();
            let mut e = Vec::new();
            collect_functions_from_path(dir.path(), &gs, &mut f, &mut e);
            (f, e)
        };
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_all_functions_nonexistent_path_returns_error() {
        let gs = build_glob_set(&[]).unwrap();
        let paths = vec!["/nonexistent/path".to_string()];
        let (functions, errors) = collect_all_functions(&paths, &gs);
        assert_eq!(functions.len(), 0);
        assert!(!errors.is_empty());
    }

    #[test]
    fn collect_all_functions_multiple_paths_collects_all() {
        let dir = tempfile::tempdir().unwrap();
        let f1 = dir.path().join("a.rs");
        let f2 = dir.path().join("b.rs");
        let body = r#"fn alpha(a: i32) -> i32 {
    let x = a + 1;
    x * 2
}
"#;
        std::fs::write(&f1, body).unwrap();
        std::fs::write(&f2, body).unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let paths = vec![
            f1.to_string_lossy().to_string(),
            f2.to_string_lossy().to_string(),
        ];
        let (functions, errors) = collect_all_functions(&paths, &gs);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn parse_output_format_json_returns_json() {
        assert!(matches!(parse_output_format("json"), Ok(OutputFormat::Json)));
    }

    #[test]
    fn parse_output_format_text_returns_text() {
        assert!(matches!(parse_output_format("text"), Ok(OutputFormat::Text)));
    }

    #[test]
    fn parse_output_format_unknown_returns_err() {
        assert!(parse_output_format("xml").is_err());
    }

    #[test]
    fn validate_lang_rust_returns_ok() {
        assert!(validate_lang("rust").is_ok());
    }

    #[test]
    fn validate_lang_other_returns_err() {
        assert!(validate_lang("python").is_err());
    }

    #[test]
    fn collect_from_single_file_excludes_non_rs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("notes.md");
        std::fs::write(&path, "# notes").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_single_file(&path, &gs, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_from_single_file_skips_excluded_rs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("skip.rs");
        std::fs::write(&path, "fn foo() {}").unwrap();
        let gs = build_glob_set(&[path.to_string_lossy().to_string()]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_single_file(&path, &gs, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_from_directory_finds_rs_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lib.rs"),
            r#"fn one(a: i32) -> i32 {
    let x = a + 1;
    x * 2
}
"#,
        )
        .unwrap();
        std::fs::write(dir.path().join("readme.txt"), "text").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_directory(dir.path(), &gs, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn is_rust_file_returns_true_for_rs() {
        assert!(is_rust_file(std::path::Path::new("lib.rs")));
    }

    #[test]
    fn is_rust_file_returns_false_for_non_rs() {
        assert!(!is_rust_file(std::path::Path::new("lib.txt")));
        assert!(!is_rust_file(std::path::Path::new("noext")));
    }

    #[test]
    fn is_same_location_same_file_same_line_returns_true() {
        let a = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 5,
            end_line: 10,
            node_hashes: vec![],
        };
        let b = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 5,
            end_line: 10,
            node_hashes: vec![],
        };
        assert!(is_same_location(&a, &b));
    }

    #[test]
    fn is_same_location_different_file_returns_false() {
        let a = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 5,
            end_line: 10,
            node_hashes: vec![],
        };
        let b = FunctionInfo {
            file: "b.rs".to_string(),
            start_line: 5,
            end_line: 10,
            node_hashes: vec![],
        };
        assert!(!is_same_location(&a, &b));
    }

    #[test]
    fn find_duplicate_pairs_skips_same_location() {
        let f1 = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 4, 5],
        };
        let f2 = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 1,
            end_line: 10,
            node_hashes: vec![1, 2, 3, 4, 5],
        };
        let pairs = find_duplicate_pairs(&[f1, f2], 0.5, 1, 1);
        assert_eq!(pairs.len(), 0, "same-location pair should be skipped");
    }

    #[test]
    fn make_pair_produces_correct_endpoints() {
        let a = FunctionInfo {
            file: "a.rs".to_string(),
            start_line: 1,
            end_line: 5,
            node_hashes: vec![1, 2],
        };
        let b = FunctionInfo {
            file: "b.rs".to_string(),
            start_line: 1,
            end_line: 5,
            node_hashes: vec![1, 2, 3],
        };
        let pair = make_pair(&a, &b, 0.9);
        assert_eq!(pair.left.file, "a.rs");
        assert_eq!(pair.right.file, "b.rs");
        assert!((pair.score - 0.9).abs() < 1e-9);
    }

    #[test]
    fn should_process_entry_returns_false_for_non_rs() {
        let dir = tempfile::tempdir().unwrap();
        let txt_path = dir.path().join("readme.txt");
        std::fs::write(&txt_path, "text").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        for entry in walkdir::WalkDir::new(dir.path()) {
            let e = entry.unwrap();
            if e.path() == txt_path {
                assert!(!should_process_entry(&e, &gs));
            }
        }
    }

    #[test]
    fn should_process_entry_returns_true_for_rs() {
        let dir = tempfile::tempdir().unwrap();
        let rs_path = dir.path().join("lib.rs");
        std::fs::write(&rs_path, "fn foo() {}").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        for entry in walkdir::WalkDir::new(dir.path()) {
            let e = entry.unwrap();
            if e.path() == rs_path {
                assert!(should_process_entry(&e, &gs));
            }
        }
    }
}
