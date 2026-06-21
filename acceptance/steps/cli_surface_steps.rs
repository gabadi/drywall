use crate::runtime::{Example, StepResult, World};
use shared::{
    accumulate_sum_source, ensure_fixture_dir, resolve_arg_paths, resolve_params,
    step_then_exit_code, step_then_no_duplicate_pair, step_then_stdout_reports_pair_for,
    write_fixture,
};
use std::cell::RefCell;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

mod shared {
    include!("shared.rs");
}

thread_local! {
    static HAS_GIT_WORK_TREE: RefCell<bool> = const { RefCell::new(false) };
    static GIT_ABSENT: RefCell<bool> = const { RefCell::new(false) };
    // True when using the real project dir (dogfood mode), not a temp fixture
    static DOGFOOD_MODE: RefCell<bool> = const { RefCell::new(false) };
}

#[allow(dead_code)]
pub fn reset_test_state() {
    shared::FIXTURE_DIR.with(|d| *d.borrow_mut() = None);
    shared::FIXTURE_ROOT.with(|r| *r.borrow_mut() = None);
    HAS_GIT_WORK_TREE.with(|f| *f.borrow_mut() = false);
    GIT_ABSENT.with(|f| *f.borrow_mut() = false);
    DOGFOOD_MODE.with(|f| *f.borrow_mut() = false);
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
    try_step!(step_when_run_drywall_twice_with_args);
    try_step!(step_then_exit_code);
    try_step!(step_then_both_runs_share_exit_code);
    try_step!(step_then_both_runs_byte_identical_stdout);
    try_step!(step_then_stdout_reports_pair_for);
    try_step!(step_then_no_duplicate_pair);
    try_step!(step_then_stderr_empty);

    StepResult::fail(format!("unsupported step: {}", s))
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
    shared::FIXTURE_DIR.with(|d| *d.borrow_mut() = None);
    shared::FIXTURE_ROOT.with(|r| *r.borrow_mut() = None);
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

fn step_when_run_drywall_twice_with_args(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^I run drywall twice with the arguments "([^"]*)"$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let raw_args = caps.get(1).map_or("", |m| m.as_str());

    let binary = std::env::var("DRYWALL_BINARY")
        .unwrap_or_else(|_| "./target/release/drywall".to_string());
    let resolved = resolve_arg_paths(raw_args);

    let run = |b: &str, args: &[String]| -> Result<(i32, String), String> {
        Command::new(b)
            .args(args)
            .output()
            .map_err(|e| format!("failed to run binary: {}", e))
            .map(|o| (
                o.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&o.stdout).to_string(),
            ))
    };

    let (code1, out1) = match run(&binary, &resolved) {
        Ok(r) => r,
        Err(e) => return Some(StepResult::fail(e)),
    };
    let (code2, out2) = match run(&binary, &resolved) {
        Ok(r) => r,
        Err(e) => return Some(StepResult::fail(e)),
    };

    world.exit_code = Some(code1);
    world.stdout = Some(out1);
    world.exit_code2 = Some(code2);
    world.stdout2 = Some(out2);
    Some(StepResult::ok())
}

fn step_then_both_runs_share_exit_code(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^both runs share the exit code "?(\d+)"?$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let expected: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let code1 = match world.exit_code {
        Some(c) => c,
        None => return Some(StepResult::fail("first run exit code not recorded")),
    };
    let code2 = match world.exit_code2 {
        Some(c) => c,
        None => return Some(StepResult::fail("second run exit code not recorded")),
    };
    Some(if code1 == expected && code2 == expected {
        StepResult::ok()
    } else {
        StepResult::fail(format!(
            "expected both runs to have exit code {}, got {} and {}",
            expected, code1, code2
        ))
    })
}

fn step_then_both_runs_byte_identical_stdout(step: &str, world: &mut World) -> Option<StepResult> {
    if step != "both runs produce byte-identical stdout" {
        return None;
    }
    let out1 = match &world.stdout {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("first run stdout not recorded")),
    };
    let out2 = match &world.stdout2 {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("second run stdout not recorded")),
    };
    Some(if out1 == out2 {
        StepResult::ok()
    } else {
        StepResult::fail(format!(
            "stdout differs between runs:\nrun1: {}\nrun2: {}",
            out1, out2
        ))
    })
}
