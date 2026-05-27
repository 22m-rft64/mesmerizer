use crate::teto::env_ref::EnvRef;
use crate::teto::exec::{run_shell, ShellOutcome};

#[derive(Debug, Clone)]
pub enum CheckStatus {
    Pass,
    Fail { code: Option<i32>, stderr: String },
    ExecError { msg: String },
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub env_ref: String,
    pub category: String,
    pub id: String,
    pub desc: String,
    pub status: CheckStatus,
}

pub fn run_checks(refs: &[EnvRef]) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for er in refs {
        for c in &er.check {
            let status = match run_shell(&c.cmd) {
                Ok(ShellOutcome::Ok { .. }) => CheckStatus::Pass,
                Ok(ShellOutcome::Fail { code, stderr, .. }) => {
                    CheckStatus::Fail { code, stderr }
                }
                Err(e) => CheckStatus::ExecError { msg: e.to_string() },
            };
            out.push(CheckResult {
                env_ref: er.name.clone(),
                category: er.category.clone(),
                id: c.id.clone(),
                desc: c.desc.clone(),
                status,
            });
        }
    }
    out
}
