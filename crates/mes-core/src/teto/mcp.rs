use crate::teto::env_ref::{EnvRef, McpInstall};

#[derive(Debug, Clone)]
pub struct McpEntry {
    pub env_ref: String,
    pub id: String,
    pub transport: String,
    pub server_cmd: Vec<String>,
    pub available_for: Vec<String>, // claude / codex / etc
}

pub fn collect_mcps(refs: &[EnvRef]) -> Vec<McpEntry> {
    let mut out = Vec::new();
    for er in refs {
        for m in &er.mcp {
            let available_for: Vec<String> = m.install_for.keys().cloned().collect();
            out.push(McpEntry {
                env_ref: er.name.clone(),
                id: m.id.clone(),
                transport: m.transport.clone(),
                server_cmd: m.server_cmd.clone(),
                available_for,
            });
        }
    }
    out
}

/// Helper used later by setup: get the McpInstall block for a given client.
pub fn install_block<'a>(m: &'a crate::teto::env_ref::McpServer, client: &str) -> Option<&'a McpInstall> {
    m.install_for.get(client)
}

/// Stub for Task 13. Real implementation lands in that task.
pub fn apply_mcp_patch(
    _client: &str,
    _config_path: &std::path::Path,
    _block: &serde_yaml::Value,
) -> Result<(), crate::teto::error::TetoError> {
    Err(crate::teto::error::TetoError::Config(
        "apply_mcp_patch not yet implemented (Task 13)".into(),
    ))
}
