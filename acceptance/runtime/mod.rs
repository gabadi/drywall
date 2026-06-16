use std::collections::HashMap;

pub type Example = HashMap<String, String>;

#[derive(Default)]
pub struct World {
    pub binary_path: Option<String>,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct StepResult {
    pub success: bool,
    pub message: String,
}

impl StepResult {
    pub fn ok() -> Self {
        StepResult { success: true, message: String::new() }
    }
    pub fn fail(msg: impl Into<String>) -> Self {
        StepResult { success: false, message: msg.into() }
    }
}
