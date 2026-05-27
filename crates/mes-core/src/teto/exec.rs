use crate::teto::error::TetoError;
use std::process::Command;

pub enum ShellOutcome {
    Ok { stdout: String, stderr: String },
    Fail { code: Option<i32>, stdout: String, stderr: String },
}

/// Run a command via `sh -c <cmd>`. Captures stdout/stderr. Returns OS-level errors
/// as TetoError::Io; non-zero exit codes go in ShellOutcome::Fail.
pub fn run_shell(cmd: &str) -> Result<ShellOutcome, TetoError> {
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    if output.status.success() {
        Ok(ShellOutcome::Ok { stdout, stderr })
    } else {
        Ok(ShellOutcome::Fail {
            code: output.status.code(),
            stdout,
            stderr,
        })
    }
}

/// Expand `$VAR`, `${VAR}`, and leading `~/` in a string. NOT a full shell-expansion —
/// just the patterns we use in env_ref path-typed fields.
pub fn expand_path(s: &str) -> String {
    let s = if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            format!("{home}/{rest}")
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    };

    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' && i + 1 < bytes.len() {
            if bytes[i + 1] == b'{' {
                if let Some(end) = s[i + 2..].find('}') {
                    let name = &s[i + 2..i + 2 + end];
                    let val = std::env::var(name).unwrap_or_default();
                    out.push_str(&val);
                    i += 2 + end + 1;
                    continue;
                }
            } else if bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == b'_' {
                let start = i + 1;
                let mut end = start;
                while end < bytes.len()
                    && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_')
                {
                    end += 1;
                }
                let name = &s[start..end];
                let val = std::env::var(name).unwrap_or_default();
                out.push_str(&val);
                i = end;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}
