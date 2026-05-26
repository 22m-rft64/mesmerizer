use crate::error::MesError;
use std::io::Write;
use std::process::{Command, Stdio};

use super::{Provider, SendOpts};

pub struct ClaudeCode;

impl ClaudeCode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClaudeCode {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for ClaudeCode {
    fn name(&self) -> &str {
        "claude_code"
    }

    fn send(&self, prompt: &str, _opts: &SendOpts) -> Result<String, MesError> {
        let mut child = Command::new("claude")
            .arg("-p")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| MesError::Provider(format!("claude spawn: {e}")))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(prompt.as_bytes())?;
        }
        let output = child
            .wait_with_output()
            .map_err(|e| MesError::Provider(format!("claude wait: {e}")))?;
        if !output.status.success() {
            return Err(MesError::Provider(format!(
                "claude exit code: {:?}",
                output.status.code()
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
