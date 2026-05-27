use crate::teto::env_ref::{EnvRef, RepairItem};
use crate::teto::error::TetoError;
use crate::teto::exec::{run_shell, ShellOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairStatus {
    /// `detect` returned 0 → system already correct, fix unnecessary.
    Skip,
    /// `detect` returned non-zero → fix is needed.
    Needed,
}

pub struct RepairPlan {
    pub env_ref_name: String,
    pub item: RepairItem,
    pub status: RepairStatus,
}

pub enum RepairOutcome {
    Skipped,
    Applied { stdout: String, stderr: String },
}

pub fn plan_repair(refs: &[EnvRef], issue_id: &str) -> Result<RepairPlan, TetoError> {
    for er in refs {
        if let Some(item) = er.repair.iter().find(|r| r.id == issue_id) {
            let status = match run_shell(&item.detect)? {
                ShellOutcome::Ok { .. } => RepairStatus::Skip,
                ShellOutcome::Fail { .. } => RepairStatus::Needed,
            };
            return Ok(RepairPlan {
                env_ref_name: er.name.clone(),
                item: item.clone(),
                status,
            });
        }
    }
    Err(TetoError::NotFound(format!("repair id `{issue_id}`")))
}

pub fn apply_repair(plan: &RepairPlan) -> Result<RepairOutcome, TetoError> {
    if plan.status == RepairStatus::Skip {
        return Ok(RepairOutcome::Skipped);
    }
    match run_shell(&plan.item.fix)? {
        ShellOutcome::Ok { stdout, stderr } => Ok(RepairOutcome::Applied { stdout, stderr }),
        ShellOutcome::Fail { code, stderr, .. } => {
            eprintln!("stderr: {stderr}");
            Err(TetoError::Shell {
                cmd: plan.item.fix.clone(),
                code,
            })
        }
    }
}

pub fn render_plan(plan: &RepairPlan) -> String {
    let status = match plan.status {
        RepairStatus::Skip => "skip (detect ok)",
        RepairStatus::Needed => "would run fix",
    };
    format!(
        "repair `{}` (from env_ref `{}`):\n  desc: {}\n  detect: {}\n  fix: {}\n  status: {}\n",
        plan.item.id,
        plan.env_ref_name,
        plan.item.desc,
        plan.item.detect,
        plan.item.fix,
        status,
    )
}
