use crate::teto::check::{run_checks, CheckResult};
use crate::teto::env_ref::EnvRef;
use crate::teto::error::TetoError;
use crate::teto::exec::{run_shell, ShellOutcome};
use crate::teto::mcp::{collect_mcps, McpEntry};
use crate::teto::store::{category_matches, default_root, load_env_refs};
use std::env;

#[derive(Debug, Clone)]
pub struct SysInfo {
    pub os: String,
    pub arch: String,
    pub path: String,
    pub shell: String,
    pub key_tools: Vec<KeyTool>,
}

#[derive(Debug, Clone)]
pub struct KeyTool {
    pub name: String,
    pub version: Option<String>,
}

pub struct DoctorReport {
    pub category_filter: Option<String>,
    pub env_refs: Vec<EnvRef>,
    pub checks: Vec<CheckResult>,
    pub mcps: Vec<McpEntry>,
    pub sys: SysInfo,
}

pub fn collect(category: Option<&str>) -> Result<DoctorReport, TetoError> {
    let root = default_root();
    let mut refs = load_env_refs(&root)?;
    if let Some(filter) = category {
        refs.retain(|r| category_matches(&r.category, filter));
    }
    let checks = run_checks(&refs);
    let mcps = collect_mcps(&refs);
    let sys = sysinfo();
    Ok(DoctorReport {
        category_filter: category.map(str::to_owned),
        env_refs: refs,
        checks,
        mcps,
        sys,
    })
}

fn sysinfo() -> SysInfo {
    let key_tools = ["sage", "python3", "rustc", "cargo", "git", "claude", "codex"]
        .iter()
        .map(|name| KeyTool {
            name: (*name).into(),
            version: probe_version(name),
        })
        .collect();
    SysInfo {
        os: env::consts::OS.into(),
        arch: env::consts::ARCH.into(),
        path: env::var("PATH").unwrap_or_default(),
        shell: env::var("SHELL").unwrap_or_default(),
        key_tools,
    }
}

fn probe_version(tool: &str) -> Option<String> {
    let cmd = format!("command -v {tool} >/dev/null && {tool} --version 2>&1 | head -n 1");
    match run_shell(&cmd).ok()? {
        ShellOutcome::Ok { stdout, .. } => {
            let s = stdout.trim();
            if s.is_empty() { None } else { Some(s.into()) }
        }
        _ => None,
    }
}
