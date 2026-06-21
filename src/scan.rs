use crate::ast::{LangConfig, extract_functions_with_config, lang_config, parse_source_tree_for};
use crate::core::{FunctionInfo, Lang};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Component, Path};
use std::process::Command;
use walkdir::WalkDir;

const BUILTIN_EXCLUDED_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "__pycache__",
    "vendor",
    "dist",
    ".next",
];

pub fn is_builtin_excluded(path: &Path) -> bool {
    path.components().any(|c| {
        if let Component::Normal(name) = c {
            BUILTIN_EXCLUDED_DIRS.contains(&name.to_str().unwrap_or(""))
        } else {
            false
        }
    })
}

fn git_check_ignore(path: &Path) -> Option<bool> {
    let abs = path.canonicalize().ok()?;
    let parent = abs.parent()?;
    let output = Command::new("git")
        .args([
            "-C",
            &parent.to_string_lossy(),
            "check-ignore",
            "--",
            &abs.to_string_lossy(),
        ])
        .output()
        .ok()?;
    Some(output.status.code() == Some(0))
}

pub fn is_git_ignored(path: &Path) -> bool {
    git_check_ignore(path).unwrap_or(false)
}

pub fn build_glob_set(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat)?);
    }
    builder.build()
}

pub fn detect_lang(path: &Path) -> Option<Lang> {
    let ext = path.extension().and_then(|e| e.to_str())?;
    match ext {
        "rs" => Some(Lang::Rust),
        "js" | "jsx" => Some(Lang::JavaScript),
        "ts" => Some(Lang::TypeScript),
        "tsx" => Some(Lang::Tsx),
        _ => None,
    }
}

pub fn is_rust_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("rs")
}

pub fn is_supported_file(path: &Path) -> bool {
    detect_lang(path).is_some()
}

pub fn should_skip(path: &Path, exclude_set: &GlobSet) -> bool {
    if exclude_set.is_empty() {
        return false;
    }
    exclude_set.is_match(path)
}

/// Returns whether a file should be scanned.
/// When force_lang is Some, every non-excluded, non-builtin-excluded file is scanned.
/// When force_lang is None, only files with recognized extensions are scanned.
pub fn should_scan_file(
    path: &Path,
    exclude_set: &GlobSet,
    git_ignored: &dyn Fn(&Path) -> bool,
    force_lang: Option<Lang>,
) -> bool {
    if is_builtin_excluded(path) || should_skip(path, exclude_set) || git_ignored(path) {
        return false;
    }
    match force_lang {
        Some(_) => true,
        None => is_supported_file(path),
    }
}

pub fn collect_all_functions(
    paths: &[String],
    exclude_set: &GlobSet,
    force_lang: Option<Lang>,
) -> (Vec<FunctionInfo>, Vec<String>) {
    let mut functions: Vec<FunctionInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            errors.push(format!("path does not exist: {}", path_str));
        } else {
            collect_functions_from_path(path, exclude_set, force_lang, &mut functions, &mut errors);
        }
    }
    (functions, errors)
}

pub fn collect_functions_from_path(
    path: &Path,
    exclude_set: &GlobSet,
    force_lang: Option<Lang>,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    if path.is_file() {
        collect_from_single_file(path, exclude_set, force_lang, functions, errors);
    } else {
        collect_from_directory(path, exclude_set, force_lang, functions, errors);
    }
}

pub fn collect_from_single_file(
    path: &Path,
    exclude_set: &GlobSet,
    force_lang: Option<Lang>,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    if should_scan_file(path, exclude_set, &is_git_ignored, force_lang) {
        let lang = force_lang.or_else(|| detect_lang(path)).unwrap();
        process_file(path, lang, functions, errors);
    }
}

fn process_entry(
    entry: Result<walkdir::DirEntry, walkdir::Error>,
    exclude_set: &GlobSet,
    force_lang: Option<Lang>,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    match entry {
        Ok(e)
            if e.file_type().is_file()
                && should_scan_file(e.path(), exclude_set, &is_git_ignored, force_lang) =>
        {
            let lang = force_lang.or_else(|| detect_lang(e.path())).unwrap();
            process_file(e.path(), lang, functions, errors);
        }
        Ok(_) => {}
        Err(err) => errors.push(format!("walk error: {}", err)),
    }
}

pub fn collect_from_directory(
    path: &Path,
    exclude_set: &GlobSet,
    force_lang: Option<Lang>,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    let walker = WalkDir::new(path)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !e.file_type().is_dir() || !is_builtin_excluded(e.path()));
    for entry in walker {
        process_entry(entry, exclude_set, force_lang, functions, errors);
    }
}

pub fn process_file(
    path: &Path,
    lang: Lang,
    functions: &mut Vec<FunctionInfo>,
    errors: &mut Vec<String>,
) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("cannot read {}: {}", path.display(), e));
            return;
        }
    };

    let config: &LangConfig = lang_config(lang);
    match parse_source_tree_for(&source, lang) {
        Ok(tree) => {
            let file_str = path.to_string_lossy().to_string();
            extract_functions_with_config(tree.root_node(), &source, &file_str, config, functions);
        }
        Err(msg) => {
            errors.push(format!("{} in {}", msg, path.display()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- built-in exclusion ---

    #[test]
    fn is_builtin_excluded_target_dir_excluded() {
        assert!(is_builtin_excluded(Path::new("target/release/drywall")));
        assert!(is_builtin_excluded(Path::new("proj/target/foo.rs")));
    }

    #[test]
    fn is_builtin_excluded_all_seven_names() {
        for name in &[
            ".git",
            "target",
            "node_modules",
            "__pycache__",
            "vendor",
            "dist",
            ".next",
        ] {
            let p = Path::new(name).join("file.rs");
            assert!(is_builtin_excluded(&p), "{} should be excluded", name);
        }
    }

    #[test]
    fn is_builtin_excluded_nested_at_any_depth() {
        assert!(is_builtin_excluded(Path::new("a/b/c/node_modules/d/e.rs")));
    }

    #[test]
    fn is_builtin_excluded_non_excluded_path_passes() {
        assert!(!is_builtin_excluded(Path::new("src/lib.rs")));
        assert!(!is_builtin_excluded(Path::new("src/core.rs")));
    }

    #[test]
    fn is_builtin_excluded_partial_name_does_not_match() {
        assert!(!is_builtin_excluded(Path::new("targets/foo.rs")));
        assert!(!is_builtin_excluded(Path::new("my_target/foo.rs")));
    }

    #[test]
    fn collect_from_directory_skips_builtin_excluded_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let target_dir = dir.path().join("target");
        std::fs::create_dir_all(&target_dir).unwrap();
        let body = r#"pub fn accumulate_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    let extra = sum * 2;
    let more = extra + a;
    let result = more + b;
    result
}
"#;
        std::fs::write(target_dir.join("excluded.rs"), body).unwrap();
        std::fs::write(dir.path().join("keep.rs"), body).unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_directory(dir.path(), &gs, None, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn collect_from_single_file_skips_builtin_excluded_path() {
        let dir = tempfile::tempdir().unwrap();
        let node_modules = dir.path().join("node_modules");
        std::fs::create_dir_all(&node_modules).unwrap();
        let path = node_modules.join("bad.rs");
        std::fs::write(&path, "fn foo() {}").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_single_file(&path, &gs, None, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    // --- glob ---

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
    fn should_skip_glob_star_suffix_does_not_match_different_suffix() {
        let gs = build_glob_set(&["*x/beta.rs".to_string()]).unwrap();
        assert!(!should_skip(std::path::Path::new("src/beta.rs"), &gs));
        assert!(should_skip(std::path::Path::new("srcx/beta.rs"), &gs));
    }

    #[test]
    fn should_skip_two_patterns_union() {
        let gs = build_glob_set(&["**/alpha.rs".to_string(), "**/beta.rs".to_string()]).unwrap();
        assert!(should_skip(std::path::Path::new("src/alpha.rs"), &gs));
        assert!(should_skip(std::path::Path::new("src/beta.rs"), &gs));
        let gs_one = build_glob_set(&["**/alpha.rs".to_string()]).unwrap();
        assert!(should_skip(std::path::Path::new("src/alpha.rs"), &gs_one));
        assert!(!should_skip(std::path::Path::new("src/beta.rs"), &gs_one));
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
        process_file(&path, Lang::Rust, &mut functions, &mut errors);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn process_file_unreadable_path_pushes_error() {
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        process_file(
            std::path::Path::new("/nonexistent/path/file.rs"),
            Lang::Rust,
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
        process_file(&path, Lang::Rust, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(!errors.is_empty());
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
        collect_functions_from_path(&path, &gs, None, &mut functions, &mut errors);
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
        collect_functions_from_path(&path, &gs, None, &mut functions, &mut errors);
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_functions_from_path_directory_finds_rs_only_no_force_lang() {
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
        collect_functions_from_path(dir.path(), &gs, None, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn collect_functions_from_path_non_rs_file_skipped_no_force_lang() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("readme.txt");
        std::fs::write(&path, "not rust").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_functions_from_path(&path, &gs, None, &mut functions, &mut errors);
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
            collect_functions_from_path(dir.path(), &gs, None, &mut f, &mut e);
            (f, e)
        };
        assert_eq!(functions.len(), 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_all_functions_nonexistent_path_returns_error() {
        let gs = build_glob_set(&[]).unwrap();
        let paths = vec!["/nonexistent/path".to_string()];
        let (functions, errors) = collect_all_functions(&paths, &gs, None);
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
        let (functions, errors) = collect_all_functions(&paths, &gs, None);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn collect_from_single_file_excludes_non_rs_no_force_lang() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("notes.md");
        std::fs::write(&path, "# notes").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_single_file(&path, &gs, None, &mut functions, &mut errors);
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
        collect_from_single_file(&path, &gs, None, &mut functions, &mut errors);
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
        collect_from_directory(dir.path(), &gs, None, &mut functions, &mut errors);
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
    fn collect_from_directory_skips_non_supported_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("readme.txt"), "text").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_directory(dir.path(), &gs, None, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 0);
    }

    #[test]
    fn collect_from_directory_includes_rs_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("lib.rs"), "fn foo(a: i32) -> i32 { let b = a + 1; let c = b * 2; let d = c + a; let e = d + 1; e }").unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_directory(dir.path(), &gs, None, &mut functions, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn collect_from_directory_skips_excluded_rs_file() {
        let dir = tempfile::tempdir().unwrap();
        let excluded = dir.path().join("excluded_by_glob.rs");
        let kept = dir.path().join("kept.rs");
        std::fs::write(&excluded, "fn foo() {}").unwrap();
        std::fs::write(&kept, "fn bar() {}").unwrap();
        let pattern = excluded.to_string_lossy().to_string();
        let gs = build_glob_set(&[pattern]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_directory(dir.path(), &gs, None, &mut functions, &mut errors);
        assert!(errors.is_empty());
        assert_eq!(functions.len(), 1);
    }

    // --- is_git_ignored ---

    #[test]
    fn is_git_ignored_nonexistent_path_returns_false() {
        assert!(!is_git_ignored(Path::new(
            "/nonexistent/totally/made/up/file.rs"
        )));
    }

    #[test]
    fn is_git_ignored_file_outside_git_repo_returns_false() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plain.rs");
        std::fs::write(&path, "fn x() {}").unwrap();
        assert!(!is_git_ignored(&path));
    }

    #[test]
    fn is_git_ignored_gitignored_file_returns_true() {
        let dir = tempfile::tempdir().unwrap();
        Command::new("git")
            .args(["-C", &dir.path().to_string_lossy(), "init"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        std::fs::write(dir.path().join(".gitignore"), "*.gen.rs\n").unwrap();
        let ignored = dir.path().join("foo.gen.rs");
        std::fs::write(&ignored, "fn x() {}").unwrap();
        assert!(is_git_ignored(&ignored));
    }

    #[test]
    fn is_git_ignored_non_ignored_file_in_git_repo_returns_false() {
        let dir = tempfile::tempdir().unwrap();
        Command::new("git")
            .args(["-C", &dir.path().to_string_lossy(), "init"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        std::fs::write(dir.path().join(".gitignore"), "*.gen.rs\n").unwrap();
        let normal = dir.path().join("lib.rs");
        std::fs::write(&normal, "fn x() {}").unwrap();
        assert!(!is_git_ignored(&normal));
    }

    // --- detect_lang ---

    #[test]
    fn detect_lang_rs_returns_rust() {
        assert_eq!(detect_lang(Path::new("foo.rs")), Some(Lang::Rust));
    }

    #[test]
    fn detect_lang_js_returns_javascript() {
        assert_eq!(detect_lang(Path::new("foo.js")), Some(Lang::JavaScript));
    }

    #[test]
    fn detect_lang_jsx_returns_javascript() {
        assert_eq!(detect_lang(Path::new("foo.jsx")), Some(Lang::JavaScript));
    }

    #[test]
    fn detect_lang_ts_returns_typescript() {
        assert_eq!(detect_lang(Path::new("foo.ts")), Some(Lang::TypeScript));
    }

    #[test]
    fn detect_lang_tsx_returns_tsx() {
        assert_eq!(detect_lang(Path::new("foo.tsx")), Some(Lang::Tsx));
    }

    #[test]
    fn detect_lang_txt_returns_none() {
        assert_eq!(detect_lang(Path::new("foo.txt")), None);
    }

    #[test]
    fn force_lang_scans_non_standard_extension() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("foo.txt");
        let js_src = "function accumulate_sum(a, b) {\n  let sum = a + b;\n  let extra = sum * 2;\n  let more = extra + a;\n  let result = more + b;\n  return result;\n}\n";
        std::fs::write(&path, js_src).unwrap();
        let gs = build_glob_set(&[]).unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        collect_from_single_file(
            &path,
            &gs,
            Some(Lang::JavaScript),
            &mut functions,
            &mut errors,
        );
        assert!(errors.is_empty(), "errors: {:?}", errors);
        assert_eq!(
            functions.len(),
            1,
            "forced lang should scan non-standard ext"
        );
    }

    #[test]
    fn process_file_valid_js_adds_functions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.js");
        std::fs::write(
            &path,
            "function alpha(a, b) {\n  let x = a + b;\n  let y = x * 2;\n  return y;\n}\n\nfunction beta(c, d) {\n  let e = c + d;\n  let f = e * 2;\n  return f;\n}\n",
        )
        .unwrap();
        let mut functions = Vec::new();
        let mut errors = Vec::new();
        process_file(&path, Lang::JavaScript, &mut functions, &mut errors);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        assert_eq!(functions.len(), 2);
    }
}
