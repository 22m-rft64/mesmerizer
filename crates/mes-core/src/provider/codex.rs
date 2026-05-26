use crate::error::MesError;
use std::process::Command;

use super::{Provider, SendOpts};

pub struct Codex;

impl Codex {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Codex {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for Codex {
    fn name(&self) -> &str {
        "codex"
    }

    fn send(&self, prompt: &str, _opts: &SendOpts) -> Result<String, MesError> {
        let output = Command::new("codex")
            .arg("exec")
            .arg(prompt)
            .output()
            .map_err(|e| MesError::Provider(format!("codex spawn: {e}")))?;
        if !output.status.success() {
            return Err(MesError::Provider(format!(
                "codex exit code: {:?}",
                output.status.code()
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
