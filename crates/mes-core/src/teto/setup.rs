use crate::teto::env_ref::{DeploySource, EnvRef};
use crate::teto::error::TetoError;
use crate::teto::exec::{expand_path, run_shell, ShellOutcome};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum SetupAction {
    GitClone {
        repo: String,
        branch: String,
        sparse_path: Option<String>,
        install_dir: PathBuf,
    },
    PostInstall {
        cmd: String,
    },
    McpConfigPatch {
        client: String,
        config_path: PathBuf,
        description: String,
    },
}

pub struct SetupPlan {
    pub env_ref_name: String,
    pub actions: Vec<SetupAction>,
}

/// Build the action list for `setup <name>`. Does NOT execute.
pub fn plan_setup(er: &EnvRef) -> SetupPlan {
    let mut actions = Vec::new();
    if let Some(deploy) = &er.deploy {
        match &deploy.source {
            DeploySource::Git {
                repo,
                branch,
                sparse_path,
            } => {
                actions.push(SetupAction::GitClone {
                    repo: repo.clone(),
                    branch: branch.clone(),
                    sparse_path: sparse_path.clone(),
                    install_dir: PathBuf::from(expand_path(&deploy.install_dir)),
                });
            }
        }
        for cmd in &deploy.post_install {
            actions.push(SetupAction::PostInstall { cmd: cmd.clone() });
        }
    }
    for m in &er.mcp {
        for (client, install) in &m.install_for {
            actions.push(SetupAction::McpConfigPatch {
                client: client.clone(),
                config_path: PathBuf::from(expand_path(&install.config_path)),
                description: format!("patch {} ({})", install.config_path, m.id),
            });
        }
    }
    SetupPlan {
        env_ref_name: er.name.clone(),
        actions,
    }
}

/// Format a plan as human-readable lines (used by --dry-run output).
pub fn render_plan(plan: &SetupPlan) -> String {
    let mut out = format!("setup plan for `{}`:\n", plan.env_ref_name);
    for (i, a) in plan.actions.iter().enumerate() {
        let line = match a {
            SetupAction::GitClone {
                repo,
                branch,
                sparse_path,
                install_dir,
            } => format!(
                "  [{i:>2}] git clone {} (branch={}{}) → {}",
                repo,
                branch,
                sparse_path
                    .as_ref()
                    .map(|p| format!(", sparse={p}"))
                    .unwrap_or_default(),
                install_dir.display()
            ),
            SetupAction::PostInstall { cmd } => {
                format!("  [{i:>2}] sh -c `{}`", cmd)
            }
            SetupAction::McpConfigPatch {
                description, ..
            } => format!("  [{i:>2}] mcp config: {}", description),
        };
        out.push_str(&line);
        out.push('\n');
    }
    if plan.actions.is_empty() {
        out.push_str("  (no actions — env_ref has no deploy section nor MCPs)\n");
    }
    out
}

/// Execute the plan. MCP patching is delegated to `mcp::apply_mcp_patch`.
pub fn apply(plan: &SetupPlan, er: &EnvRef) -> Result<(), TetoError> {
    for action in &plan.actions {
        match action {
            SetupAction::GitClone {
                repo,
                branch,
                sparse_path,
                install_dir,
            } => apply_git_clone(repo, branch, sparse_path.as_deref(), install_dir)?,
            SetupAction::PostInstall { cmd } => apply_shell(cmd)?,
            SetupAction::McpConfigPatch {
                client,
                config_path,
                ..
            } => {
                let mcp_server = er
                    .mcp
                    .iter()
                    .find(|m| m.install_for.contains_key(client))
                    .ok_or_else(|| TetoError::Config(format!("mcp `{client}` not found")))?;
                let install = mcp_server.install_for.get(client).unwrap();
                crate::teto::mcp::apply_mcp_patch(client, config_path, &install.block)?;
            }
        }
    }
    Ok(())
}

fn apply_git_clone(
    repo: &str,
    branch: &str,
    sparse_path: Option<&str>,
    install_dir: &std::path::Path,
) -> Result<(), TetoError> {
    if install_dir.exists() {
        return Err(TetoError::Config(format!(
            "{} already exists — refusing to clone over it",
            install_dir.display()
        )));
    }
    if let Some(parent) = install_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut cmd = format!("git clone --branch {} ", shell_quote(branch));
    if sparse_path.is_some() {
        cmd.push_str("--filter=blob:none --no-checkout ");
    }
    cmd.push_str(&shell_quote(repo));
    cmd.push(' ');
    cmd.push_str(&shell_quote(install_dir.to_string_lossy().as_ref()));
    apply_shell(&cmd)?;

    if let Some(sub) = sparse_path {
        let dir = install_dir.to_string_lossy().to_string();
        let cmds = [
            format!(
                "git -C {dir} sparse-checkout init --cone",
                dir = shell_quote(&dir)
            ),
            format!(
                "git -C {dir} sparse-checkout set {sub}",
                dir = shell_quote(&dir),
                sub = shell_quote(sub),
            ),
            format!(
                "git -C {dir} checkout {branch}",
                dir = shell_quote(&dir),
                branch = shell_quote(branch),
            ),
        ];
        for c in cmds {
            apply_shell(&c)?;
        }
    }
    Ok(())
}

fn apply_shell(cmd: &str) -> Result<(), TetoError> {
    match run_shell(cmd)? {
        ShellOutcome::Ok { .. } => Ok(()),
        ShellOutcome::Fail { code, stderr, .. } => {
            eprintln!("stderr: {stderr}");
            Err(TetoError::Shell {
                cmd: cmd.into(),
                code,
            })
        }
    }
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
