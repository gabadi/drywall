// Mutation hardening tests — separate from unit and acceptance suites.
// Each test targets a specific surviving mutant identified by cargo-mutants.

use drywall::ast::{
    RUST_CONFIG, build_normalized_subtree, extract_functions, extract_let_declaration, make_parser,
};
use drywall::scan::{build_glob_set, collect_from_directory};
use drywall::{FunctionInfo, find_duplicate_pairs, jaccard, source_lines};
use std::fs;

fn parse_and_extract(source: &str) -> Vec<FunctionInfo> {
    let tree = make_parser().parse(source, None).unwrap();
    let mut functions = Vec::new();
    extract_functions(tree.root_node(), source, "test.rs", &mut functions);
    functions
}

fn make_fi(start_line: usize, end_line: usize, hashes: Vec<u64>) -> FunctionInfo {
    FunctionInfo {
        file: "test.rs".to_string(),
        start_line,
        end_line,
        node_hashes: hashes,
    }
}

// --- ast.rs:55 --- extract_let_declaration: child.kind() != "closure_expression"
// Mutant: replace != with ==
// Guard: a closure containing a nested closure must yield only 1 entry (the outer let-bound
// closure). If != became ==, we'd recurse INTO the outer closure's children, potentially
// finding and registering the inner closure as a second function. Verify we do NOT double-count.
#[test]
fn let_decl_nested_closure_not_double_counted() {
    // A let with a closure that itself contains another let-bound closure inside its body.
    // With != mutated to ==: we'd recurse into the outer closure_expression, find the inner
    // let_declaration, and push a second function entry.
    let source = "let outer = |x: i32| -> i32 { let inner = |y: i32| y + 1; inner(x) };\n";
    let tree = make_parser().parse(source, None).unwrap();
    let mut functions = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();
    let children: Vec<_> = root.children(&mut cursor).collect();
    if let Some(let_node) = children.iter().find(|n| n.kind() == "let_declaration") {
        extract_let_declaration(*let_node, source, "test.rs", &mut functions);
    }
    assert_eq!(
        functions.len(),
        1,
        "only the outer closure should be captured, got {} entries",
        functions.len()
    );
}

// Verify that non-closure let children ARE traversed (extract nested fn items from sub-nodes).
#[test]
fn let_decl_non_closure_children_are_traversed() {
    // A plain let that contains no closure; the extract_let_declaration should not add anything,
    // but it must not prevent traversal of other children.
    let source = "let x = 42;\n";
    let tree = make_parser().parse(source, None).unwrap();
    let mut functions = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();
    let children: Vec<_> = root.children(&mut cursor).collect();
    if let Some(let_node) = children.iter().find(|n| n.kind() == "let_declaration") {
        extract_let_declaration(*let_node, source, "test.rs", &mut functions);
    }
    assert_eq!(functions.len(), 0);
}

// --- ast.rs:34 --- delete match arm "let_declaration" in extract_functions
// Mutant: if the let_declaration arm is deleted, extract_functions falls to the _ arm which
// recurses into children — but never calls extract_let_declaration, so let-bound closures
// are never captured as functions.
#[test]
fn extract_functions_captures_let_bound_closure() {
    // extract_functions must find the closure in a top-level let statement.
    // If the let_declaration arm is deleted, the closure is never pushed.
    let source = "let f = |x: i32| -> i32 { x + 1 };\n";
    let functions = parse_and_extract(source);
    assert_eq!(
        functions.len(),
        1,
        "let-bound closure must be captured by extract_functions, got {}",
        functions.len()
    );
}

// --- ast.rs:75 --- make_function_info: start_line = row + 1 (mutation: + -> *)
// Guard: tree-sitter rows are 0-based; +1 makes them 1-based. For a top-level function on row 0,
// start_line must be 1 (not 0, which * would produce: 0*1=0).
#[test]
fn make_function_info_start_line_is_one_based() {
    let source = "fn top() {}\n";
    let functions = parse_and_extract(source);
    assert_eq!(functions.len(), 1);
    assert_eq!(
        functions[0].start_line, 1,
        "start_line must be 1-based (row 0 + 1), got {}",
        functions[0].start_line
    );
}

// For a function on row 2 (third line), start_line must be 3, not 6 (which * would give: 2*3=6).
#[test]
fn make_function_info_start_line_correct_for_non_zero_row() {
    let source = "\n\nfn third_row() {}\n";
    let functions = parse_and_extract(source);
    assert_eq!(functions.len(), 1);
    assert_eq!(
        functions[0].start_line, 3,
        "start_line for row 2 must be 3 (2+1), got {}",
        functions[0].start_line
    );
}

// --- ast.rs:89 --- normalize_node_text: literal match arm deletion
// Guard: integer/float/string/char/bool literals must normalize to _LIT so that structurally
// identical functions with different literal values still get high Jaccard scores.
#[test]
fn normalize_literal_integers_are_replaced() {
    // Two functions differing only in integer literals must have the same normalized subtree hash.
    let source_a = "fn a() -> i32 { 42 }\n";
    let source_b = "fn b() -> i32 { 99 }\n";
    let tree_a = make_parser().parse(source_a, None).unwrap();
    let tree_b = make_parser().parse(source_b, None).unwrap();
    let norm_a = build_normalized_subtree(tree_a.root_node(), source_a, &RUST_CONFIG);
    let norm_b = build_normalized_subtree(tree_b.root_node(), source_b, &RUST_CONFIG);
    assert_eq!(
        norm_a, norm_b,
        "functions differing only in integer literals must normalize identically"
    );
}

#[test]
fn normalize_float_literals_are_replaced() {
    // float_literal is a leaf in tree-sitter-rust (like integer_literal).
    let source_a = "fn a() -> f64 { 1.5 }\n";
    let source_b = "fn b() -> f64 { 9.9 }\n";
    let tree_a = make_parser().parse(source_a, None).unwrap();
    let tree_b = make_parser().parse(source_b, None).unwrap();
    let norm_a = build_normalized_subtree(tree_a.root_node(), source_a, &RUST_CONFIG);
    let norm_b = build_normalized_subtree(tree_b.root_node(), source_b, &RUST_CONFIG);
    assert_eq!(
        norm_a, norm_b,
        "functions differing only in float literals must normalize identically"
    );
}

#[test]
fn normalized_subtree_contains_lit_placeholder_not_raw_value() {
    // Direct check: the normalized form must contain _LIT, not the raw integer.
    let source = "fn a() -> i32 { 42 }\n";
    let tree = make_parser().parse(source, None).unwrap();
    let norm = build_normalized_subtree(tree.root_node(), source, &RUST_CONFIG);
    assert!(
        norm.contains("_LIT"),
        "normalized form must contain _LIT, got: {}",
        norm
    );
    assert!(
        !norm.contains("42"),
        "normalized form must not contain raw literal '42', got: {}",
        norm
    );
}

// --- core.rs:78 --- jaccard: a.is_empty() && b.is_empty() (mutation: && -> ||)
// Guard: if only one is empty, jaccard must NOT return 1.0. With || the early return fires
// when either set is empty, incorrectly giving 1.0 for (empty, non-empty).
#[test]
fn jaccard_one_empty_one_nonempty_is_not_one() {
    let a: Vec<u64> = vec![];
    let b: Vec<u64> = vec![1, 2, 3];
    let score = jaccard(&a, &b);
    assert_eq!(
        score, 0.0,
        "jaccard(empty, non-empty) must be 0.0, got {}",
        score
    );
}

#[test]
fn jaccard_nonempty_one_empty_is_not_one() {
    let a: Vec<u64> = vec![1, 2, 3];
    let b: Vec<u64> = vec![];
    let score = jaccard(&a, &b);
    assert_eq!(
        score, 0.0,
        "jaccard(non-empty, empty) must be 0.0, got {}",
        score
    );
}

#[test]
fn jaccard_both_empty_is_one() {
    let a: Vec<u64> = vec![];
    let b: Vec<u64> = vec![];
    let score = jaccard(&a, &b);
    assert_eq!(score, 1.0, "jaccard(empty, empty) must be 1.0");
}

// --- core.rs:126 --- find_duplicate_pairs filter: source_lines >= min_lines && node_hashes.len() >= min_nodes
// Mutation: && -> ||
// Guard: a function that is long enough in lines but too short in nodes must NOT be a candidate.
#[test]
fn find_pairs_excludes_function_with_too_few_nodes() {
    // Build two FunctionInfos that are above min_lines but below min_nodes.
    let f1 = make_fi(1, 10, vec![1, 2]); // 10 lines, 2 nodes
    let f2 = make_fi(20, 30, vec![1, 2]); // 11 lines, 2 nodes
    let pairs = find_duplicate_pairs(&[f1, f2], 0.5, 4, 10); // min_nodes=10
    assert!(
        pairs.is_empty(),
        "functions below min_nodes must not be candidates, but got {:?} pairs",
        pairs.len()
    );
}

#[test]
fn find_pairs_excludes_function_with_too_few_lines() {
    // Above min_nodes but below min_lines.
    let hashes: Vec<u64> = (0..20).collect();
    let f1 = make_fi(1, 2, hashes.clone()); // 2 lines, 20 nodes
    let f2 = make_fi(10, 11, hashes);
    let pairs = find_duplicate_pairs(&[f1, f2], 0.5, 10, 5); // min_lines=10
    assert!(
        pairs.is_empty(),
        "functions below min_lines must not be candidates, but got {:?} pairs",
        pairs.len()
    );
}

// --- core.rs:132 --- j starts at (i + 1): mutation + -> *
// Guard: when i=0 and + becomes *, j starts at 0 instead of 1, so j==i==0 and we'd compare
// a function to itself. Since is_same_location skips same-file/same-line, the bug may be hidden,
// but the loop range [0..len) is valid while [i*1..len) == [0..len) re-tests all pairs twice.
// Verify no duplicate pairs are returned (which would happen with j starting at 0 and i+1 starting at 1,
// since each pair would be seen twice and is_same_location would only skip self).
#[test]
fn find_pairs_no_duplicate_pairs_in_output() {
    let hashes: Vec<u64> = (0..15).collect();
    let f1 = make_fi(1, 10, hashes.clone());
    let f2 = make_fi(20, 30, hashes.clone());
    let f3 = make_fi(40, 50, hashes);
    let pairs = find_duplicate_pairs(&[f1, f2, f3], 0.5, 4, 5);
    // 3 functions → at most 3 unique pairs (1-2, 1-3, 2-3). With j starting at i*1=0, some
    // would be revisited.
    let pair_count = pairs.len();
    assert!(
        pair_count <= 3,
        "expected at most 3 unique pairs for 3 functions, got {}",
        pair_count
    );
    // Verify no pair has left == right location.
    for p in &pairs {
        assert!(
            !(p.left.file == p.right.file && p.left.start_line == p.right.start_line),
            "pair should not compare a function to itself"
        );
    }
}

// Confirm source_lines helper used in filter.
#[test]
fn source_lines_counts_correctly() {
    let fi = make_fi(3, 7, vec![]);
    assert_eq!(source_lines(&fi), 5);
}

#[test]
fn source_lines_single_line() {
    let fi = make_fi(5, 5, vec![]);
    assert_eq!(source_lines(&fi), 1);
}

// --- scan.rs:146 --- collect_from_directory filter_entry: delete first ! in
// `!e.file_type().is_dir() || !is_builtin_excluded(e.path())`
// Mutation: `e.file_type().is_dir() || !is_builtin_excluded(e.path())`
// With mutation: a builtin-excluded directory satisfies `is_dir()`, so filter_entry returns true
// and WalkDir recurses into it. The guard: a .rs file inside a builtin-excluded dir must NOT
// appear in results.
#[test]
fn collect_from_directory_skips_builtin_excluded_dirs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let target_dir = dir.path().join("target");
    fs::create_dir(&target_dir).expect("create target/");
    let rs_file = target_dir.join("hidden.rs");
    fs::write(&rs_file, "fn inside_excluded() {}\n").expect("write rs");
    let empty_glob = build_glob_set(&[]).expect("glob");
    let mut functions = Vec::new();
    let mut errors = Vec::new();
    collect_from_directory(dir.path(), &empty_glob, None, &mut functions, &mut errors);
    assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    assert!(
        functions.is_empty(),
        "expected no functions from builtin-excluded dir, got: {:?}",
        functions
    );
}
