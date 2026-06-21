use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Rust,
    JavaScript,
    TypeScript,
    Tsx,
    Python,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub node_hashes: Vec<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PairEndpoint {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub nodes: usize,
}

#[derive(Debug, Clone, Serialize)]
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
    pub lang: Option<Lang>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threshold: 0.82,
            min_lines: 4,
            min_nodes: 20,
            format: OutputFormat::Text,
            excludes: vec![],
            lang: None,
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
        other => Err(format!("unknown format '{}'; use 'text' or 'json'", other)),
    }
}

pub fn validate_lang(lang: &str) -> Result<Lang, String> {
    match lang {
        "rust" => Ok(Lang::Rust),
        "js" => Ok(Lang::JavaScript),
        "ts" => Ok(Lang::TypeScript),
        "py" => Ok(Lang::Python),
        other => Err(format!(
            "unsupported language '{}'; use rust, js, ts, or py",
            other
        )),
    }
}

pub fn jaccard(a: &[u64], b: &[u64]) -> f64 {
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

pub fn source_lines(fi: &FunctionInfo) -> usize {
    fi.end_line.saturating_sub(fi.start_line) + 1
}

pub fn make_pair(a: &FunctionInfo, b: &FunctionInfo, score: f64) -> DuplicatePair {
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

pub fn is_same_location(a: &FunctionInfo, b: &FunctionInfo) -> bool {
    a.file == b.file && a.start_line == b.start_line
}

pub fn find_duplicate_pairs(
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

pub struct CliResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

fn cli_error(msg: String) -> CliResult {
    CliResult {
        exit_code: 2,
        stdout: String::new(),
        stderr: format!("error: {}", msg),
    }
}

fn format_run_output(format: &OutputFormat, pairs: Vec<DuplicatePair>) -> String {
    match format {
        OutputFormat::Text => format_text(&pairs),
        OutputFormat::Json => format_json(&pairs),
    }
}

fn clean_output(format: &OutputFormat) -> String {
    if matches!(format, OutputFormat::Json) {
        "[]".to_string()
    } else {
        String::new()
    }
}

fn run_result_to_cli(result: RunResult, format: &OutputFormat) -> CliResult {
    match result {
        RunResult::Clean => CliResult {
            exit_code: 0,
            stdout: clean_output(format),
            stderr: String::new(),
        },
        RunResult::Duplicates(pairs) => CliResult {
            exit_code: 1,
            stdout: format_run_output(format, pairs),
            stderr: String::new(),
        },
        RunResult::Error(msg) => cli_error(msg),
    }
}

pub fn execute_cli(
    paths: &[String],
    format_str: &str,
    lang: Option<&str>,
    config: Config,
    run_fn: impl Fn(&[String], &Config) -> RunResult,
) -> CliResult {
    let format = match parse_output_format(format_str) {
        Ok(f) => f,
        Err(e) => return cli_error(e),
    };

    let lang_val = if let Some(lang) = lang {
        match validate_lang(lang) {
            Ok(l) => Some(l),
            Err(e) => return cli_error(e),
        }
    } else {
        None
    };

    let config = Config {
        format,
        lang: lang_val,
        ..config
    };
    let result = run_fn(paths, &config);
    run_result_to_cli(result, &config.format)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn jaccard_both_empty_returns_one() {
        let score = jaccard(&[], &[]);
        assert!((score - 1.0).abs() < 1e-9);
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
    fn parse_output_format_json_returns_json() {
        assert!(matches!(
            parse_output_format("json"),
            Ok(OutputFormat::Json)
        ));
    }

    #[test]
    fn parse_output_format_text_returns_text() {
        assert!(matches!(
            parse_output_format("text"),
            Ok(OutputFormat::Text)
        ));
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
    fn validate_lang_py_returns_python() {
        assert!(matches!(validate_lang("py"), Ok(Lang::Python)));
    }

    #[test]
    fn validate_lang_other_returns_err() {
        assert!(validate_lang("python").is_err());
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
    fn execute_cli_invalid_format_returns_exit_2() {
        let result = execute_cli(
            &[".".to_string()],
            "xml",
            None,
            Config::default(),
            |_, _| RunResult::Clean,
        );
        assert_eq!(result.exit_code, 2);
        assert!(result.stderr.contains("error:"));
        assert!(result.stdout.is_empty());
    }

    #[test]
    fn execute_cli_invalid_lang_returns_exit_2() {
        let result = execute_cli(
            &[".".to_string()],
            "text",
            Some("python"),
            Config::default(),
            |_, _| RunResult::Clean,
        );
        assert_eq!(result.exit_code, 2);
        assert!(result.stderr.contains("error:"));
    }

    #[test]
    fn execute_cli_clean_text_returns_exit_0_empty_stdout() {
        let result = execute_cli(
            &[".".to_string()],
            "text",
            None,
            Config::default(),
            |_, _| RunResult::Clean,
        );
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn execute_cli_clean_json_returns_exit_0_empty_array() {
        let result = execute_cli(
            &[".".to_string()],
            "json",
            None,
            Config::default(),
            |_, _| RunResult::Clean,
        );
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout.trim(), "[]");
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn execute_cli_duplicates_returns_exit_1_with_text_output() {
        let pair = DuplicatePair {
            score: 0.9,
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
        let result = execute_cli(
            &[".".to_string()],
            "text",
            None,
            Config::default(),
            move |_, _| RunResult::Duplicates(vec![pair.clone()]),
        );
        assert_eq!(result.exit_code, 1);
        assert!(result.stdout.contains("DUPLICATE"));
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn execute_cli_duplicates_json_returns_exit_1_with_json_output() {
        let pair = DuplicatePair {
            score: 0.9,
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
        let result = execute_cli(
            &[".".to_string()],
            "json",
            None,
            Config::default(),
            move |_, _| RunResult::Duplicates(vec![pair.clone()]),
        );
        assert_eq!(result.exit_code, 1);
        assert!(result.stdout.contains("score"));
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn execute_cli_error_returns_exit_2_with_stderr() {
        let result = execute_cli(
            &[".".to_string()],
            "text",
            None,
            Config::default(),
            |_, _| RunResult::Error("something went wrong".to_string()),
        );
        assert_eq!(result.exit_code, 2);
        assert!(result.stderr.contains("error:"));
        assert!(result.stdout.is_empty());
    }

    #[test]
    fn execute_cli_valid_lang_rust_proceeds() {
        let result = execute_cli(
            &[".".to_string()],
            "text",
            Some("rust"),
            Config::default(),
            |_, _| RunResult::Clean,
        );
        assert_eq!(result.exit_code, 0);
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
}
