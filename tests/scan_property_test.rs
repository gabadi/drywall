use drywall::scan::{
    build_glob_set, detect_lang, is_builtin_excluded, is_rust_file, should_scan_file, should_skip,
};
use proptest::prelude::*;
use std::path::Path;

const BUILTIN_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "__pycache__",
    "vendor",
    "dist",
    ".next",
];

fn builtin_dir() -> impl Strategy<Value = &'static str> {
    proptest::sample::select(BUILTIN_DIRS)
}

fn safe_segment() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,8}".prop_map(|s| s)
}

proptest! {
    // --- is_builtin_excluded invariants ---

    #[test]
    fn builtin_excluded_any_segment_position(
        prefix in prop::collection::vec(safe_segment(), 0..3),
        excluded in builtin_dir(),
        suffix in prop::collection::vec(safe_segment(), 0..3),
    ) {
        let mut parts = prefix;
        parts.push(excluded.to_string());
        parts.extend(suffix);
        parts.push("file.rs".to_string());
        let path = parts.join("/");
        prop_assert!(
            is_builtin_excluded(Path::new(&path)),
            "expected excluded for path: {}",
            path
        );
    }

    #[test]
    fn non_builtin_segments_not_excluded(
        segments in prop::collection::vec(safe_segment(), 1..4),
    ) {
        let mut path_parts = segments.clone();
        path_parts.push("file.rs".to_string());
        let path = path_parts.join("/");
        // Only paths with no builtin segment should pass
        let has_builtin = segments.iter().any(|s| BUILTIN_DIRS.contains(&s.as_str()));
        if !has_builtin {
            prop_assert!(
                !is_builtin_excluded(Path::new(&path)),
                "expected non-excluded for path: {}",
                path
            );
        }
    }

    #[test]
    fn partial_name_match_not_excluded(
        excluded in builtin_dir(),
        suffix in "[a-z]{1,4}",
    ) {
        let segment = format!("{}{}", excluded, suffix);
        let path = format!("{}/file.rs", segment);
        prop_assert!(
            !is_builtin_excluded(Path::new(&path)),
            "partial name '{}' must not be excluded",
            segment
        );
    }

    // --- is_rust_file invariant ---

    #[test]
    fn is_rust_file_iff_rs_extension(
        stem in "[a-z][a-z0-9_]{0,10}",
        ext in "[a-z]{1,4}",
    ) {
        let path = format!("{}.{}", stem, ext);
        let result = is_rust_file(Path::new(&path));
        if ext == "rs" {
            prop_assert!(result, "expected .rs file to be rust: {}", path);
        } else {
            prop_assert!(!result, "expected non-.rs file to not be rust: {}", path);
        }
    }

    // --- should_skip (glob) invariants ---

    #[test]
    fn should_skip_empty_set_never_skips(path in "[a-z/]{1,20}\\.rs") {
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(!should_skip(Path::new(&path), &gs));
    }

    #[test]
    fn should_skip_matching_pattern_skips(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("acceptance/{}.rs", stem);
        let gs = build_glob_set(&["acceptance/**".to_string()]).unwrap();
        prop_assert!(
            should_skip(Path::new(&path), &gs),
            "expected acceptance path to be skipped: {}",
            path
        );
    }

    #[test]
    fn should_skip_non_matching_path_not_skipped(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("src/{}.rs", stem);
        let gs = build_glob_set(&["acceptance/**".to_string()]).unwrap();
        prop_assert!(
            !should_skip(Path::new(&path), &gs),
            "expected src path not to be skipped: {}",
            path
        );
    }

    #[test]
    fn glob_union_order_independent(
        stem in "[a-z][a-z0-9_]{0,8}",
    ) {
        let p1 = "acceptance/**".to_string();
        let p2 = format!("tests/{}.rs", stem);
        let path = format!("tests/{}.rs", stem);

        let gs_ab = build_glob_set(&[p1.clone(), p2.clone()]).unwrap();
        let gs_ba = build_glob_set(&[p2.clone(), p1.clone()]).unwrap();

        let result_ab = should_skip(Path::new(&path), &gs_ab);
        let result_ba = should_skip(Path::new(&path), &gs_ba);

        prop_assert_eq!(
            result_ab, result_ba,
            "glob union must be order-independent for path: {}",
            path
        );
    }

    #[test]
    fn glob_union_idempotent(stem in "[a-z][a-z0-9_]{0,8}") {
        let pattern = format!("tests/{}.rs", stem);
        let path = format!("tests/{}.rs", stem);

        let gs_once = build_glob_set(&[pattern.clone()]).unwrap();
        let gs_twice = build_glob_set(&[pattern.clone(), pattern.clone()]).unwrap();

        let result_once = should_skip(Path::new(&path), &gs_once);
        let result_twice = should_skip(Path::new(&path), &gs_twice);

        prop_assert_eq!(
            result_once, result_twice,
            "duplicating a glob pattern must not change skip result for: {}",
            path
        );
    }

    // --- should_scan_file composition invariants ---

    #[test]
    fn should_scan_file_unsupported_ext_never_scanned(
        stem in "[a-z][a-z0-9_]{0,8}",
        ext in "[a-z]{2,4}",
    ) {
        // Extensions not in {rs, js, jsx, ts, tsx, py} must never be scanned with force_lang=None
        prop_assume!(!matches!(ext.as_str(), "rs" | "js" | "jsx" | "ts" | "tsx" | "py"));
        let path = format!("src/{}.{}", stem, ext);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(!should_scan_file(Path::new(&path), &gs, None));
    }

    #[test]
    fn should_scan_file_builtin_excluded_never_scanned(
        excluded in proptest::sample::select(BUILTIN_DIRS),
        stem in "[a-z][a-z0-9_]{0,8}",
    ) {
        let path = format!("{}/{}.rs", excluded, stem);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(
            !should_scan_file(Path::new(&path), &gs, None),
            "builtin-excluded path must not scan: {}",
            path
        );
    }

    #[test]
    fn should_scan_file_glob_excluded_never_scanned(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("acceptance/{}.rs", stem);
        let gs = build_glob_set(&["acceptance/**".to_string()]).unwrap();
        prop_assert!(
            !should_scan_file(Path::new(&path), &gs, None),
            "glob-excluded file must not scan: {}",
            path
        );
    }

    #[test]
    fn should_scan_file_clean_rs_path_scanned(stem in "[a-z][a-z0-9_]{0,8}") {
        // A .rs file in src/ with no exclusions must scan
        let path = format!("src/{}.rs", stem);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(
            should_scan_file(Path::new(&path), &gs, None),
            "clean .rs file must scan: {}",
            path
        );
    }

    // --- detect_lang extension table invariants ---

    #[test]
    fn detect_lang_known_extensions_return_some(
        stem in "[a-z][a-z0-9_]{0,8}",
        ext in proptest::sample::select(&["rs", "js", "jsx", "ts", "tsx", "py"][..]),
    ) {
        let path = format!("src/{}.{}", stem, ext);
        prop_assert!(
            detect_lang(Path::new(&path)).is_some(),
            "known extension '{}' must return Some lang",
            ext
        );
    }

    #[test]
    fn detect_lang_unsupported_ext_returns_none(
        stem in "[a-z][a-z0-9_]{0,8}",
        ext in "[a-z]{2,4}",
    ) {
        prop_assume!(!matches!(ext.as_str(), "rs" | "js" | "jsx" | "ts" | "tsx" | "py"));
        let path = format!("src/{}.{}", stem, ext);
        prop_assert!(
            detect_lang(Path::new(&path)).is_none(),
            "unsupported extension '{}' must return None",
            ext
        );
    }

    // --- should_scan_file: supported JS/TS extensions are scanned ---

    #[test]
    fn should_scan_file_clean_js_path_scanned(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("src/{}.js", stem);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(
            should_scan_file(Path::new(&path), &gs, None),
            "clean .js file must scan: {}",
            path
        );
    }

    #[test]
    fn should_scan_file_clean_ts_path_scanned(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("src/{}.ts", stem);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(
            should_scan_file(Path::new(&path), &gs, None),
            "clean .ts file must scan: {}",
            path
        );
    }

    #[test]
    fn should_scan_file_clean_py_path_scanned(stem in "[a-z][a-z0-9_]{0,8}") {
        let path = format!("src/{}.py", stem);
        let gs = build_glob_set(&[]).unwrap();
        prop_assert!(
            should_scan_file(Path::new(&path), &gs, None),
            "clean .py file must scan: {}",
            path
        );
    }
}
