use crate::runtime::{Example, StepResult, World};
use std::process::Command;
use std::sync::OnceLock;

pub fn dispatch(step_text: &str, world: &mut World, example: &Example) -> StepResult {
    if try_step_given_binary_built(step_text, world, example).is_some() {
        return StepResult::ok();
    }
    if let Some(result) = try_step_minimal_rust_dir(step_text, world, example) {
        return result;
    }
    if let Some(result) = try_step_run_with_args(step_text, world, example) {
        return result;
    }
    if let Some(result) = try_step_exit_code(step_text, world, example) {
        return result;
    }
    if let Some(result) = try_step_stdout_empty(step_text, world, example) {
        return result;
    }
    if let Some(result) = try_step_stderr_empty(step_text, world, example) {
        return result;
    }
    if let Some(result) = try_step_stderr_no_panic(step_text, world, example) {
        return result;
    }
    StepResult::fail(format!("unsupported step: {}", step_text))
}

fn try_step_given_binary_built(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<()> {
    static PATTERN: &str = "the drywall release binary is built";
    if step_text != PATTERN {
        return None;
    }
    let path = std::env::var("DRYWALL_BINARY")
        .unwrap_or_else(|_| "./target/release/drywall".to_string());
    world.binary_path = Some(path);
    Some(())
}

fn step_run_with_args_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r#"^the binary is run with the arguments "([^"]*)"$"#).unwrap())
}

fn example_param_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"^<([A-Za-z0-9_]+)>$").unwrap())
}

fn resolve_args_string(raw: &str, example: &Example) -> Result<String, StepResult> {
    if let Some(c) = example_param_re().captures(raw) {
        let key = c.get(1).unwrap().as_str();
        example
            .get(key)
            .cloned()
            .ok_or_else(|| StepResult::fail(format!("missing example key: {}", key)))
    } else {
        Ok(raw.to_string())
    }
}

fn run_binary(binary: &str, args_str: &str, world: &mut World) -> StepResult {
    let args: Vec<&str> = if args_str.is_empty() {
        vec![]
    } else {
        args_str.split_whitespace().collect()
    };
    match Command::new(binary).args(&args).output() {
        Ok(o) => {
            world.exit_code = Some(o.status.code().unwrap_or(-1));
            world.stdout = Some(String::from_utf8_lossy(&o.stdout).to_string());
            world.stderr = Some(String::from_utf8_lossy(&o.stderr).to_string());
            StepResult::ok()
        }
        Err(e) => StepResult::fail(format!("failed to run binary: {}", e)),
    }
}

fn try_step_run_with_args(
    step_text: &str,
    world: &mut World,
    example: &Example,
) -> Option<StepResult> {
    let caps = step_run_with_args_re().captures(step_text)?;
    let raw_args = caps.get(1).map_or("", |m| m.as_str());

    let args_str = match resolve_args_string(raw_args, example) {
        Ok(s) => s,
        Err(e) => return Some(e),
    };

    let binary = match &world.binary_path {
        Some(p) => p.clone(),
        None => return Some(StepResult::fail("binary path not set")),
    };

    Some(run_binary(&binary, &args_str, world))
}

fn step_exit_code_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"^the exit code is (\d+)$").unwrap())
}

fn try_step_exit_code(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    let caps = step_exit_code_re().captures(step_text)?;
    let expected: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let actual = match world.exit_code {
        Some(c) => c,
        None => return Some(StepResult::fail("exit code not yet recorded")),
    };
    Some(if actual == expected {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected exit code {}, got {}", expected, actual))
    })
}

fn try_step_stdout_empty(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    if step_text != "stdout is empty" {
        return None;
    }
    let stdout = match &world.stdout {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stdout not yet recorded")),
    };
    Some(if stdout.is_empty() {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected empty stdout, got: {:?}", stdout))
    })
}

fn try_step_stderr_empty(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    if step_text != "stderr is empty" {
        return None;
    }
    let stderr = match &world.stderr {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stderr not yet recorded")),
    };
    Some(if stderr.is_empty() {
        StepResult::ok()
    } else {
        StepResult::fail(format!("expected empty stderr, got: {:?}", stderr))
    })
}

fn try_step_stderr_no_panic(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    if step_text != "stderr contains no panic text" {
        return None;
    }
    let stderr = match &world.stderr {
        Some(s) => s.clone(),
        None => return Some(StepResult::fail("stderr not yet recorded")),
    };
    Some(if stderr.contains("panicked at") || stderr.contains("stack backtrace") {
        StepResult::fail(format!("stderr contains panic text: {:?}", stderr))
    } else {
        StepResult::ok()
    })
}

fn try_step_minimal_rust_dir(
    step_text: &str,
    _world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    static PATTERN: &str = "a minimal Rust source directory exists at \"./tmp/qa-minimal\"";
    if step_text != PATTERN {
        return None;
    }
    let dir = std::path::Path::new("./tmp/qa-minimal");
    if let Err(e) = std::fs::create_dir_all(dir) {
        return Some(StepResult::fail(format!("failed to create tmp/qa-minimal: {}", e)));
    }
    let lib_path = dir.join("lib.rs");
    if let Err(e) = std::fs::write(&lib_path, "pub fn answer() -> i32 { 42 }\n") {
        return Some(StepResult::fail(format!("failed to write lib.rs: {}", e)));
    }
    Some(StepResult::ok())
}
