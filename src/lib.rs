pub mod ast;
mod core;
pub mod scan;

pub use core::{
    CliResult, Config, DuplicatePair, FunctionInfo, Lang, OutputFormat, PairEndpoint, RunResult,
    execute_cli, find_duplicate_pairs, format_json, format_text, jaccard, parse_output_format,
    source_lines, validate_lang,
};

use ast::parse_source_tree;
use scan::{build_glob_set, collect_all_functions};

pub fn parse_rust_source(source: &str) -> bool {
    parse_source_tree(source).is_ok()
}

pub fn run(paths: &[String], config: &Config) -> RunResult {
    let exclude_set = match build_glob_set(&config.excludes) {
        Ok(gs) => gs,
        Err(e) => return RunResult::Error(format!("invalid exclude glob: {}", e)),
    };

    let (functions, errors) = collect_all_functions(paths, &exclude_set, config.lang);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{JS_CONFIG, extract_functions, extract_functions_with_config};
    use crate::core::{Lang, jaccard, source_lines};

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
        let source = "fn tiny(x: i32) -> i32 { x + 1 }\n";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut functions = Vec::new();
        extract_functions(tree.root_node(), source, "test.rs", &mut functions);
        for f in &functions {
            assert!(
                source_lines(f) < 4,
                "expected tiny function to be < 4 lines"
            );
        }
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
        assert_eq!(
            functions.len(),
            0,
            "static with non-closure value, no functions expected"
        );
    }

    #[test]
    fn run_js_duplicate_files_returns_duplicates() {
        let dir = tempfile::tempdir().unwrap();
        let alpha_src = "function accumulate_sum(a, b) {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let beta_src = "function accumulate_sum(x, y) {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        std::fs::write(dir.path().join("alpha.js"), alpha_src).unwrap();
        std::fs::write(dir.path().join("beta.js"), beta_src).unwrap();
        let config = Config::default();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        assert!(
            matches!(result, RunResult::Duplicates(_)),
            "expected duplicates in js files"
        );
    }

    #[test]
    fn run_ts_duplicate_files_returns_duplicates() {
        let dir = tempfile::tempdir().unwrap();
        let alpha_src = "function accumulate_sum(a: number, b: number): number {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let beta_src = "function accumulate_sum(x: number, y: number): number {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        std::fs::write(dir.path().join("alpha.ts"), alpha_src).unwrap();
        std::fs::write(dir.path().join("beta.ts"), beta_src).unwrap();
        let config = Config::default();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        assert!(
            matches!(result, RunResult::Duplicates(_)),
            "expected duplicates in ts files"
        );
    }

    #[test]
    fn run_with_force_lang_js_scans_non_standard_ext() {
        let dir = tempfile::tempdir().unwrap();
        let alpha_src = "function accumulate_sum(a, b) {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let beta_src = "function accumulate_sum(x, y) {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        std::fs::write(dir.path().join("alpha.inc"), alpha_src).unwrap();
        std::fs::write(dir.path().join("beta.inc"), beta_src).unwrap();
        let config = Config {
            lang: Some(Lang::JavaScript),
            ..Config::default()
        };
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        assert!(
            matches!(result, RunResult::Duplicates(_)),
            "expected duplicates with forced js lang"
        );
    }

    #[test]
    fn run_mixed_language_dir_reports_both_pairs() {
        let dir = tempfile::tempdir().unwrap();
        let rs_alpha = r#"pub fn accumulate_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    let extra = sum * 2;
    let more = extra + a;
    let result = more + b;
    result
}
"#;
        let rs_beta = r#"pub fn accumulate_sum(x: i32, y: i32) -> i32 {
    let total = x + y;
    let extra = total * 2;
    let more = extra + x;
    let result = more + y;
    result
}
"#;
        let ts_alpha = "function accumulate_sum(a: number, b: number): number {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        let ts_beta = "function accumulate_sum(x: number, y: number): number {\n  let total = x + y;\n  let extra = total * 2;\n  let more = extra + x;\n  let result = more + y;\n  return result;\n}\n";
        std::fs::write(dir.path().join("a.rs"), rs_alpha).unwrap();
        std::fs::write(dir.path().join("b.rs"), rs_beta).unwrap();
        std::fs::write(dir.path().join("c.ts"), ts_alpha).unwrap();
        std::fs::write(dir.path().join("d.ts"), ts_beta).unwrap();
        let config = Config::default();
        let paths = vec![dir.path().to_string_lossy().to_string()];
        let result = run(&paths, &config);
        match result {
            RunResult::Duplicates(pairs) => {
                assert!(
                    pairs.len() >= 2,
                    "expected at least 2 pairs, got {}",
                    pairs.len()
                );
            }
            _ => panic!("expected duplicates"),
        }
    }

    #[test]
    fn js_function_declaration_extracts_one_form() {
        let src = "function compute(a, b) {\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  let f = e + b;\n  return f;\n}\n";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(src, None).unwrap();
        let mut funcs = Vec::new();
        extract_functions_with_config(tree.root_node(), src, "f.js", &JS_CONFIG, &mut funcs);
        assert_eq!(funcs.len(), 1, "function-declaration form");
    }
}
