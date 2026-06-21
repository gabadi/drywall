use crate::runtime::{Example, StepResult};
use std::cell::RefCell;
use std::path::Path;

thread_local! {
    pub static FIXTURE_DIR: RefCell<Option<tempfile::TempDir>> = const { RefCell::new(None) };
    pub static FIXTURE_ROOT: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn fixture_root() -> Option<String> {
    FIXTURE_ROOT.with(|r| r.borrow().clone())
}

pub fn ensure_fixture_dir() -> Result<String, StepResult> {
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

pub fn write_fixture(root: &str, relative: &str, content: &str) -> Result<(), StepResult> {
    let full = Path::new(root).join(relative);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| StepResult::fail(format!("mkdir {}: {}", parent.display(), e)))?;
    }
    std::fs::write(&full, content)
        .map_err(|e| StepResult::fail(format!("write {}: {}", full.display(), e)))?;
    Ok(())
}

#[allow(dead_code)]
pub fn accumulate_sum_source(params: &str) -> String {
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

pub fn resolve_params(step_text: &str, example: &Example) -> String {
    let mut result = step_text.to_string();
    for (k, v) in example {
        result = result.replace(&format!("<{}>", k), v);
    }
    result
}

pub fn step_then_exit_code(step: &str, world: &mut crate::runtime::World) -> Option<StepResult> {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
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
        let stderr = world.require_stderr().unwrap_or("");
        let stdout = world.require_stdout().unwrap_or("");
        StepResult::fail(format!(
            "expected exit code {}, got {}\nstdout: {}\nstderr: {}",
            expected, actual, stdout, stderr
        ))
    })
}

pub fn step_then_stdout_reports_pair_for(step: &str, world: &mut crate::runtime::World) -> Option<StepResult> {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^stdout reports a duplicate pair for "([^"]+)" and "([^"]+)"$"#)
            .unwrap()
    });
    let caps = re.captures(step)?;
    let left_file = caps.get(1).unwrap().as_str();
    let right_file = caps.get(2).unwrap().as_str();

    let stdout = match world.require_stdout() {
        Ok(s) => s.to_owned(),
        Err(e) => return Some(e),
    };

    let left_name = Path::new(left_file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(left_file);
    let right_name = Path::new(right_file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(right_file);

    Some(
        if stdout.contains(left_name) && stdout.contains(right_name) && stdout.contains("DUPLICATE")
        {
            StepResult::ok()
        } else {
            StepResult::fail(format!(
                "expected stdout to report pair for {} and {}, got: {}",
                left_file, right_file, stdout
            ))
        },
    )
}

pub fn step_then_no_duplicate_pair(step: &str, world: &mut crate::runtime::World) -> Option<StepResult> {
    if step != "no duplicate pair is reported" {
        return None;
    }
    let stdout = match world.require_stdout() {
        Ok(s) => s.to_owned(),
        Err(e) => return Some(e),
    };
    Some(if !stdout.contains("DUPLICATE") {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected no duplicate pair, stdout: {}", stdout))
    })
}

// Options that take a value argument (not a path)
const VALUE_OPTIONS: &[&str] = &[
    "--threshold",
    "--min-lines",
    "--min-nodes",
    "--format",
    "--lang",
    "--exclude",
];

pub fn resolve_arg_paths(args_str: &str) -> Vec<String> {
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
