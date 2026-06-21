use crate::runtime::{Example, StepResult, World};
use shared::{
    ensure_fixture_dir, resolve_arg_paths, resolve_params, step_then_exit_code,
    step_then_no_duplicate_pair, step_then_stdout_reports_pair_for, write_fixture,
};
use std::process::Command;
use std::sync::OnceLock;

mod shared {
    include!("shared.rs");
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

    try_step!(step_given_ext_file_with_structure_and_ids);
    try_step!(step_given_ext_file_containing_source_fails_to_parse);
    try_step!(step_when_run_drywall_with_args);
    try_step!(step_then_exit_code);
    try_step!(step_then_stdout_reports_pair_for);
    try_step!(step_then_stderr_not_empty);
    try_step!(step_then_no_duplicate_pair);
    try_step!(step_then_reported_score_at_least);

    StepResult::fail(format!("unsupported step: {}", s))
}


fn accumulate_sum_js(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "function {fn_name}({p0}, {p1}) {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return result;\n}}\n"
    )
}

fn accumulate_sum_ts(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "function {fn_name}({p0}: number, {p1}: number): number {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return result;\n}}\n"
    )
}

fn accumulate_sum_rs(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "pub fn {fn_name}({p0}: i32, {p1}: i32) -> i32 {{\n    let {p2} = {p0} + {p1};\n    let extra = {p2} * 2;\n    let more = extra + {p0};\n    let result = more + {p1};\n    result\n}}\n"
    )
}

fn arrow_function_js(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "const {fn_name} = ({p0}, {p1}) => {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return result;\n}};\n"
    )
}

fn arrow_function_ts(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "const {fn_name} = ({p0}: number, {p1}: number): number => {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return result;\n}};\n"
    )
}

fn class_method_ts(ids: &str, class_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "class {class_name} {{\n  accumulate({p0}: number, {p1}: number): number {{\n    let {p2} = {p0} + {p1};\n    let extra = {p2} * 2;\n    let more = extra + {p0};\n    let result = more + {p1};\n    return result;\n  }}\n}}\n"
    )
}

fn exported_named_function_js(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "export function {fn_name}({p0}, {p1}) {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return result;\n}}\n"
    )
}

fn accumulate_sum_tsx(ids: &str, fn_name: &str) -> String {
    let parts: Vec<&str> = ids.split(',').collect();
    let (p0, p1, p2) = if parts.len() >= 3 {
        (parts[0].trim(), parts[1].trim(), parts[2].trim())
    } else {
        ("a", "b", "sum")
    };
    format!(
        "function {fn_name}({p0}: number, {p1}: number): JSX.Element {{\n  let {p2} = {p0} + {p1};\n  let extra = {p2} * 2;\n  let more = extra + {p0};\n  let result = more + {p1};\n  return <div>{{result}}</div>;\n}}\n"
    )
}

fn make_source(ext: &str, structure: &str, ids: &str, fn_name: &str) -> String {
    match (ext, structure) {
        (_, "accumulate_sum") | (_, "function-declaration") => match ext {
            "rs" => accumulate_sum_rs(ids, fn_name),
            "tsx" => accumulate_sum_tsx(ids, fn_name),
            "ts" => accumulate_sum_ts(ids, fn_name),
            _ => accumulate_sum_js(ids, fn_name),
        },
        (_, "arrow-function") => match ext {
            "ts" | "tsx" => arrow_function_ts(ids, fn_name),
            _ => arrow_function_js(ids, fn_name),
        },
        (_, "class-method") => match ext {
            "ts" | "tsx" => class_method_ts(ids, fn_name),
            _ => class_method_ts(ids, fn_name),
        },
        (_, "exported-named-function") => exported_named_function_js(ids, fn_name),
        _ => match ext {
            "rs" => accumulate_sum_rs(ids, fn_name),
            "ts" | "tsx" => accumulate_sum_ts(ids, fn_name),
            _ => accumulate_sum_js(ids, fn_name),
        },
    }
}

// Step: a "<ext>" file "<file>" containing a function with structure "<structure>" and identifiers "<ids>"
fn step_given_ext_file_with_structure_and_ids(step: &str, _world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(
            r#"^a "([^"]+)" file "([^"]+)" containing a function with structure "([^"]+)" and identifiers "([^"]+)"$"#,
        ).unwrap()
    });
    let caps = re.captures(step)?;
    let ext = caps.get(1).unwrap().as_str();
    let file = caps.get(2).unwrap().as_str();
    let structure = caps.get(3).unwrap().as_str();
    let ids = caps.get(4).unwrap().as_str();

    let root = match ensure_fixture_dir() {
        Ok(r) => r,
        Err(e) => return Some(e),
    };

    let fn_name = std::path::Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("fn_a");

    let content = make_source(ext, structure, ids, fn_name);

    match write_fixture(&root, file, &content) {
        Ok(()) => Some(StepResult::ok()),
        Err(e) => Some(e),
    }
}

// Step: a "<ext>" file "<file>" containing source that fails to parse
fn step_given_ext_file_containing_source_fails_to_parse(
    step: &str,
    _world: &mut World,
) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(
            r#"^a "([^"]+)" file "([^"]+)" containing source that fails to parse$"#,
        )
        .unwrap()
    });
    let caps = re.captures(step)?;
    let _ext = caps.get(1).unwrap().as_str();
    let file = caps.get(2).unwrap().as_str();

    let root = match ensure_fixture_dir() {
        Ok(r) => r,
        Err(e) => return Some(e),
    };

    // This source reliably trips tree-sitter's error detection for both JS and TS
    let bad_source = "function {{ INVALID SYNTAX (((( ;\n";

    match write_fixture(&root, file, bad_source) {
        Ok(()) => Some(StepResult::ok()),
        Err(e) => Some(e),
    }
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

    let output = match Command::new(&binary).args(&resolved).output() {
        Ok(o) => o,
        Err(e) => return Some(StepResult::fail(format!("failed to run binary: {}", e))),
    };

    world.exit_code = Some(output.status.code().unwrap_or(-1));
    world.stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
    Some(StepResult::ok())
}

fn step_then_reported_score_at_least(step: &str, world: &mut World) -> Option<StepResult> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r#"^the reported score is at least "([^"]+)"$"#).unwrap()
    });
    let caps = re.captures(step)?;
    let min_score: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(0.0);

    let stdout = match world.require_stdout() {
        Ok(s) => s.to_owned(),
        Err(e) => return Some(e),
    };

    static SCORE_RE: OnceLock<regex::Regex> = OnceLock::new();
    let score_re =
        SCORE_RE.get_or_init(|| regex::Regex::new(r"DUPLICATE score=(\d+\.\d+)").unwrap());

    let scores: Vec<f64> = score_re
        .captures_iter(&stdout)
        .filter_map(|c| c.get(1).unwrap().as_str().parse().ok())
        .collect();

    Some(if scores.iter().any(|&s| s >= min_score) {
        StepResult::ok()
    } else {
        StepResult::fail(format!(
            "expected at least one score >= {}, found: {:?}\nstdout: {}",
            min_score, scores, stdout
        ))
    })
}

fn step_then_stderr_not_empty(step: &str, world: &mut World) -> Option<StepResult> {
    if step != "stderr is not empty" {
        return None;
    }
    let stderr = match world.require_stderr() {
        Ok(s) => s.to_owned(),
        Err(e) => return Some(e),
    };
    Some(if !stderr.is_empty() {
        StepResult::ok()
    } else {
        StepResult::fail("expected non-empty stderr, got empty".to_string())
    })
}

