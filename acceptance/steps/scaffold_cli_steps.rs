use crate::runtime::{Example, StepResult, World};
use std::process::Command;

pub fn dispatch(step_text: &str, world: &mut World, example: &Example) -> StepResult {
    if let Some(_) = try_step_given_binary_built(step_text, world, example) {
        return StepResult::ok();
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

fn try_step_run_with_args(
    step_text: &str,
    world: &mut World,
    example: &Example,
) -> Option<StepResult> {
    let re = regex::Regex::new(r#"^the binary is run with the arguments "([^"]*)"$"#).unwrap();
    let caps = re.captures(step_text)?;
    let raw_args = caps.get(1).map_or("", |m| m.as_str());

    let param_re = regex::Regex::new(r"^<([A-Za-z0-9_]+)>$").unwrap();
    let args_str = if let Some(c) = param_re.captures(raw_args) {
        let key = c.get(1).unwrap().as_str();
        match example.get(key) {
            Some(v) => v.clone(),
            None => return Some(StepResult::fail(format!("missing example key: {}", key))),
        }
    } else {
        raw_args.to_string()
    };

    let binary = match &world.binary_path {
        Some(p) => p.clone(),
        None => return Some(StepResult::fail("binary path not set")),
    };

    let args: Vec<&str> = if args_str.is_empty() {
        vec![]
    } else {
        args_str.split_whitespace().collect()
    };

    let output = match Command::new(&binary).args(&args).output() {
        Ok(o) => o,
        Err(e) => return Some(StepResult::fail(format!("failed to run binary: {}", e))),
    };

    world.exit_code = Some(output.status.code().unwrap_or(-1));
    world.stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
    Some(StepResult::ok())
}

fn try_step_exit_code(
    step_text: &str,
    world: &mut World,
    _example: &Example,
) -> Option<StepResult> {
    let re = regex::Regex::new(r"^the exit code is (\d+)$").unwrap();
    let caps = re.captures(step_text)?;
    let expected: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let actual = match world.exit_code {
        Some(c) => c,
        None => return Some(StepResult::fail("exit code not yet recorded")),
    };
    if actual == expected {
        Some(StepResult::ok())
    } else {
        Some(StepResult::fail(format!(
            "expected exit code {}, got {}",
            expected, actual
        )))
    }
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
    if stdout.is_empty() {
        Some(StepResult::ok())
    } else {
        Some(StepResult::fail(format!("expected empty stdout, got: {:?}", stdout)))
    }
}
