use crate::teto::env_ref::{EnvRef, McpInstall};
use crate::teto::error::TetoError;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct McpEntry {
    pub env_ref: String,
    pub id: String,
    pub transport: String,
    pub server_cmd: Vec<String>,
    pub available_for: Vec<String>,
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

pub fn install_block<'a>(
    m: &'a crate::teto::env_ref::McpServer,
    client: &str,
) -> Option<&'a McpInstall> {
    m.install_for.get(client)
}

pub fn apply_mcp_patch(
    client: &str,
    config_path: &Path,
    block: &serde_yaml::Value,
) -> Result<(), TetoError> {
    match client {
        "claude" => patch_claude(config_path, block),
        "codex" => patch_codex(config_path, block),
        other => Err(TetoError::Config(format!(
            "unknown MCP client: {other}"
        ))),
    }
}

fn patch_claude(config_path: &Path, block: &serde_yaml::Value) -> Result<(), TetoError> {
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut root: serde_json::Value = if config_path.exists() {
        let text = std::fs::read_to_string(config_path)?;
        if text.trim().is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&text).map_err(TetoError::Json)?
        }
    } else {
        serde_json::json!({})
    };
    let block_json = yaml_to_json(block)?;
    let mcp_servers = root
        .as_object_mut()
        .ok_or_else(|| TetoError::Config("claude config root not an object".into()))?
        .entry("mcpServers".to_string())
        .or_insert_with(|| serde_json::json!({}));
    let map = mcp_servers
        .as_object_mut()
        .ok_or_else(|| TetoError::Config("mcpServers is not an object".into()))?;
    let block_obj = block_json
        .as_object()
        .ok_or_else(|| TetoError::Config("env_ref MCP block must be a mapping".into()))?;
    for (k, v) in block_obj {
        map.insert(k.clone(), v.clone());
    }
    let pretty = serde_json::to_string_pretty(&root).map_err(TetoError::Json)?;
    std::fs::write(config_path, pretty + "\n")?;
    Ok(())
}

fn patch_codex(config_path: &Path, block: &serde_yaml::Value) -> Result<(), TetoError> {
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let block_text = match block {
        serde_yaml::Value::String(s) => s.clone(),
        other => serde_yaml::to_string(other).map_err(TetoError::Yaml)?,
    };
    let fragment: toml_edit::DocumentMut = block_text
        .parse()
        .map_err(|e: toml_edit::TomlError| TetoError::Config(format!("codex block parse: {e}")))?;
    let mut existing: toml_edit::DocumentMut = if config_path.exists() {
        let text = std::fs::read_to_string(config_path)?;
        if text.trim().is_empty() {
            toml_edit::DocumentMut::new()
        } else {
            text.parse().map_err(|e: toml_edit::TomlError| {
                TetoError::Config(format!("codex existing parse: {e}"))
            })?
        }
    } else {
        toml_edit::DocumentMut::new()
    };
    for (key, item) in fragment.iter() {
        existing.insert(key, item.clone());
    }
    std::fs::write(config_path, existing.to_string())?;
    Ok(())
}

fn yaml_to_json(v: &serde_yaml::Value) -> Result<serde_json::Value, TetoError> {
    let s = serde_yaml::to_string(v).map_err(TetoError::Yaml)?;
    let mut de = serde_yaml::Deserializer::from_str(&s);
    serde_json::Value::deserialize(de.next().unwrap())
        .map_err(|e| TetoError::Config(format!("yaml->json: {e}")))
}
