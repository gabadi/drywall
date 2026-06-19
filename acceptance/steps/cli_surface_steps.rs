use crate::runtime::{Example, StepResult, World};
use std::cell::RefCell;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

thread_local! {
    static FIXTURE_DIR: RefCell<Option<tempfile::TempDir>> = const { RefCell::new(None) };
    static FIXTURE_ROOT: RefCell<Option<String>> = const { RefCell::new(None) };
    static HAS_GIT_WORK_TREE: RefCell<bool> = const { RefCell::new(false) };
    static GIT_ABSENT: RefCell<bool> = const { RefCell::new(false) };
    // True when using the real project dir (dogfood mode), not a temp fixture
    static DOGFOOD_MODE: RefCell<bool> = const { RefCell::new(false) };
}

fn fixture_root() -> Option<String> {
    FIXTURE_ROOT.with(|r| r.borrow().clone())
}

fn ensure_fixture_dir() -> Result<String, StepResult> {
    FIXTURE_DIR.with(|dir| {
        let mut borrow = dir.borrow_mut();
        if borrow.is_none() {
            match tempfile::TempDir::new() {
                Ok(td) => {
                    let path = td.path().to_string_lossy().to_string();
                    *borrow = Some(td);
                    FIXTURE_ROOT.with(|r| *r.borrow_mut() = Some(path));
                }
                Err(e) => {
                    return Err(StepResult::fail(format!("failed to create temp dir: {}", e)));
                }
            }
        }
        Ok(FIXTURE_ROOT.with(|r| r.borrow().clone().unwrap()))
    })
}

fn write_fixture(root: &str, relative: &str, content: &str) -> Result<(), StepResult> {
    let full = Path::new(root).join(relative);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| StepResult::fail(format!("mkdir {}: {}", parent.display(), e)))?;
    }
    std::fs::write(&full, content)
        .map_err(|e| StepResult::fail(format!("write {}: {}", full.display(), e)))?;
    Ok(())
}

fn accumulate_sum_source(params: &str) -> String {
    let ids: Vec<&str> = params.split(',').collect();
    let (p0, p1, p2) = if ids.len() >= 3 {
        (ids[0].trim(), ids[1].trim(), ids[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "pub fn accumulate_sum({p0}: i32, {p1}: i32) -> i32 {{\n    let {p2} = {p0} + {p1};\n    let extra = {p2} * 2;\n    let more = extra + {p0};\n    let result = more + {p1};\n    result\n}}\n",
        p0 = p0, p1 = p1, p2 = p2
    )
}

// Options that take a value argument (not a path)
const VALUE_OPTIONS: &[&str] = &["--threshold", "--min-lines", "--min-nodes", "--format", "--lang", "--exclude"];

fn resolve_arg_paths(args_str: &str) -> Vec<String> {
    let root = fixture_root();
    let tokens: Vec<&str> = args_str.split_whitespace().collect();
    let mut result = Vec::new();
    let mut skip_next = false;
    for tok in tokens.iter() {
        if skip_next {
            skip_next = false;
            result.push(tok.to_string());
            continue;
        }
        if tok.starts_with("--") || tok.starts_with('-') {
            if VALUE_OPTIONS.contains(tok) {
                skip_next = true;
            }
            result.push(tok.to_string());
        } else if let Some(ref r) = root {
            let p = Path::new(tok);
            if p.is_absolute() {
                result.push(tok.to_string());
            } else {
                result.push(Path::new(r).join(tok).to_string_lossy().to_string());
            }
        } else {
            result.push(tok.to_string());
        }
    }
    result
}

pub fn dispatch(step_text: &str, world: &mut World, example: &Example) -> StepResult {
    let step = resolve_params(step_text, example);
    let s = step.as_str();

    macro_rules! try_step {
        ($fn:ident) => {
            if let Some(r) = $fn(s, world) {
                return r;
            }
        };
    }

    try_step!(step_given_rust_file_with_structure_and_ids);
    try_step!(step_given_git_work_tree);
    try_step!(step_given_gitignore_entry);
    try_step!(step_given_no_git_executable);
    try_step!(step_given_project_source_dir);
    try_step!(step_when_run_drywall_with_args);
    try_step!(step_then_exit_code);
    try_step!(step_then_stdout_reports_pair_for);
    try_step!(step_then_no_duplicate_pair);
    try_step!(step_then_stderr_empty);

    StepResult::fail(format!("unsupported step: {}", s))
}

fn resolve_params(step_text: &str, example: &Example) -> String {
    let mut result = step_text.to_string();
    for (k, v) in example {
        result = result.replace(&format!("<{}>", k), v);
    }
    result
}

fn step_given_rust_file_with_structure_and_ids(step: &str, _world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(
            r#"^a Rust file "([^"]+)" containing a function with structure "([^"]+)" and identifiers "([^"]+)"$"#,
        ).unwrap()
    });
    let caps = re.captures(step)?;
    let file = caps.get(1).unwrap().as_str();
    let _structure = caps.get(2).unwrap().as_str();
    let ids = caps.get(3).unwrap().as_str();

    let root = match ensure_fixture_dir() {
        Ok(r) => r,
        Err(e) => return Some(e),
    };

    let content = accumulate_sum_source(ids);
    match write_fixture(&root, file, &content) {
        Ok(()) => Some(StepResult::ok()),
        Err(e) => Some(e),
    }
}

fn step_given_git_work_tree(step: &str, _world: &mut World) -> Option<StepResult> {
    if step != "a git work tree with a git executable available" {
        return None;
    }
    // In dogfood mode the real project is already a git work tree
    if DOGFOOD_MODE.with(|f| *f.borrow()) {
        HAS_GIT_WORK_TREE.with(|f| *f.borrow_mut() = true);
        return Some(StepResult::ok());
    }
    let root = match ensure_fixture_dir() {
        Ok(r) => r,
        Err(e) => return Some(e),
    };
    let status = Command::new("git")
        .args(["-C", &root, "init"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    match status {
        Ok(s) if s.success() => {
            HAS_GIT_WORK_TREE.with(|f| *f.borrow_mut() = true);
            Some(StepResult::ok())
        }
        Ok(s) => Some(StepResult::fail(format!("git init failed: {:?}", s))),
        Err(e) => Some(StepResult::fail(format!("failed to run git init: {}", e))),
    }
}

fn step_given_gitignore_entry(step: &str, _world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^a gitignore entry "([^"]+)"$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let pattern = caps.get(1).unwrap().as_str();

    let root = match ensure_fixture_dir() {
        Ok(r) => r,
        Err(e) => return Some(e),
    };

    let gitignore = Path::new(&root).join(".gitignore");
    let existing = std::fs::read_to_string(&gitignore).unwrap_or_default();
    let new_content = format!("{}{}\n", existing, pattern);
    match std::fs::write(&gitignore, new_content) {
        Ok(()) => Some(StepResult::ok()),
        Err(e) => Some(StepResult::fail(format!("write .gitignore: {}", e))),
    }
}

fn step_given_no_git_executable(step: &str, _world: &mut World) -> Option<StepResult> {
    if step != "no git executable is available" {
        return None;
    }
    GIT_ABSENT.with(|f| *f.borrow_mut() = true);
    Some(StepResult::ok())
}

fn step_given_project_source_dir(step: &str, _world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^the drywall project source directory at "([^"]+)"$"#).unwrap()
    });
    let _caps = re.captures(step)?;
    // Dogfood: use real project directory, not a temp fixture
    FIXTURE_DIR.with(|d| *d.borrow_mut() = None);
    FIXTURE_ROOT.with(|r| *r.borrow_mut() = None);
    HAS_GIT_WORK_TREE.with(|f| *f.borrow_mut() = false);
    GIT_ABSENT.with(|f| *f.borrow_mut() = false);
    DOGFOOD_MODE.with(|f| *f.borrow_mut() = true);
    Some(StepResult::ok())
}

fn step_when_run_drywall_with_args(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^I run drywall with the arguments "([^"]*)"$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let raw_args = caps.get(1).map_or("", |m| m.as_str());

    let binary = std::env::var("DRYWALL_BINARY")
        .unwrap_or_else(|_| "./target/release/drywall".to_string());

    let resolved = resolve_arg_paths(raw_args);

    world.binary_path = Some(binary.clone());

    let git_absent = GIT_ABSENT.with(|f| *f.borrow());

    let output = if git_absent {
        // Strip git from PATH so drywall cannot find the git executable
        let path_without_git = std::env::var("PATH").unwrap_or_default();
        let filtered: Vec<&str> = path_without_git
            .split(':')
            .filter(|dir| {
                let git_path = Path::new(dir).join("git");
                !git_path.exists()
            })
            .collect();
        let new_path = filtered.join(":");
        Command::new(&binary)
            .args(&resolved)
            .env("PATH", &new_path)
            .output()
    } else {
        Command::new(&binary).args(&resolved).output()
    };

    let output = match output {
        Ok(o) => o,
        Err(e) => return Some(StepResult::fail(format!("failed to run binary: {}", e))),
    };

    world.exit_code = Some(output.status.code().unwrap_or(-1));
    world.stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
    Some(StepResult::ok())
}

fn step_then_exit_code(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r#"^the exit code is "?(\d+)"?$"#).unwrap());
    let caps = re.captures(step)?;
    let expected: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let actual = match world.exit_code {
        Some(c) => c,
        None => return Some(StepResult::fail("exit code not yet recorded")),
    };
    Some(if actual == expected {
        StepResult::ok()
    } else {
        let stderr = world.stderr.as_deref().unwrap_or("");
        let stdout = world.stdout.as_deref().unwrap_or("");
        StepResult::fail(format!(
            "expected exit code {}, got {}\nstdout: {}\nstderr: {}",
            expected, actual, stdout, stderr
        ))
    })
}

fn step_then_stdout_reports_pair_for(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^stdout reports a duplicate pair for "([^"]+)" and "([^"]+)"$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let left_file = caps.get(1).unwrap().as_str();
    let right_file = caps.get(2).unwrap().as_str();

    let stdout = match &world.stdout {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stdout not yet recorded")),
    };

    let left_name = Path::new(left_file).file_name().and_then(|n| n.to_str()).unwrap_or(left_file);
    let right_name = Path::new(right_file).file_name().and_then(|n| n.to_str()).unwrap_or(right_file);

    Some(if stdout.contains(left_name) && stdout.contains(right_name) && stdout.contains("DUPLICATE") {
        StepResult::ok()
    } else {
        StepResult::fail(format!(
            "expected stdout to report pair for {} and {}, got: {}",
            left_file, right_file, stdout
        ))
    })
}

fn step_then_no_duplicate_pair(step: &str, world: &mut World) -> Option<StepResult> {
    if step != "no duplicate pair is reported" {
        return None;
    }
    let stdout = match &world.stdout {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stdout not yet recorded")),
    };
    Some(if !stdout.contains("DUPLICATE") {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected no duplicate pair, stdout: {}", stdout))
    })
}

fn step_then_stderr_empty(step: &str, world: &mut World) -> Option<StepResult> {
    if step != "stderr is empty" {
        return None;
    }
    let stderr = match &world.stderr {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stderr not yet recorded")),
    };
    Some(if stderr.is_empty() {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected empty stderr, got: {}", stderr))
    })
}
