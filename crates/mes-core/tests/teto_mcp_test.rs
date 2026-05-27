use mes_core::teto::env_ref::EnvRef;
use mes_core::teto::mcp::{collect_mcps, McpEntry};

fn fixture_with_mcp() -> EnvRef {
    let body = r#"---
name: tk
description: test
category: misc
mcp:
  - id: teto
    server_cmd: ["teto-mcp"]
    transport: stdio
    install_for:
      claude:
        config_path: "~/.claude/mcp.json"
        block:
          teto.local:
            command: teto-mcp
      codex:
        config_path: "~/.codex/config.toml"
        block: |
          [mcp_servers."teto.local"]
          command = "teto-mcp"
---
"#;
    EnvRef::from_skill_md(body).unwrap()
}

#[test]
fn collect_mcps_returns_one_entry_per_server() {
    let refs = vec![fixture_with_mcp()];
    let entries = collect_mcps(&refs);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, "teto");
    assert_eq!(entries[0].transport, "stdio");
    assert_eq!(entries[0].available_for.len(), 2);
}

#[test]
fn collect_mcps_empty_when_no_mcp() {
    let body = r#"---
name: notk
description: test
category: misc
---
"#;
    let er = EnvRef::from_skill_md(body).unwrap();
    let entries = collect_mcps(&[er]);
    assert!(entries.is_empty());
}

use mes_core::teto::mcp::apply_mcp_patch;
use std::io::Write;

#[test]
fn patch_claude_creates_file_if_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = tmp.path().join("mcp.json");
    let block: serde_yaml::Value = serde_yaml::from_str(
        "teto.local:\n  command: teto-mcp\n",
    )
    .unwrap();
    apply_mcp_patch("claude", &cfg, &block).unwrap();
    let txt = std::fs::read_to_string(&cfg).unwrap();
    let v: serde_json::Value = serde_json::from_str(&txt).unwrap();
    assert!(v["mcpServers"]["teto.local"]["command"] == "teto-mcp");
}

#[test]
fn patch_claude_merges_existing() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = tmp.path().join("mcp.json");
    let mut f = std::fs::File::create(&cfg).unwrap();
    writeln!(
        f,
        r#"{{ "mcpServers": {{ "other": {{ "command": "other" }} }} }}"#
    )
    .unwrap();
    drop(f);
    let block: serde_yaml::Value =
        serde_yaml::from_str("teto.local:\n  command: teto-mcp\n").unwrap();
    apply_mcp_patch("claude", &cfg, &block).unwrap();
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&cfg).unwrap()).unwrap();
    assert!(v["mcpServers"]["other"]["command"] == "other");
    assert!(v["mcpServers"]["teto.local"]["command"] == "teto-mcp");
}

#[test]
fn patch_codex_creates_file() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = tmp.path().join("config.toml");
    // YAML block scalar needs indented content for serde_yaml 0.9
    let block: serde_yaml::Value = serde_yaml::from_str(
        "|\n  [mcp_servers.\"teto.local\"]\n  command = \"teto-mcp\"\n",
    )
    .unwrap();
    apply_mcp_patch("codex", &cfg, &block).unwrap();
    let txt = std::fs::read_to_string(&cfg).unwrap();
    assert!(txt.contains("teto.local"));
    assert!(txt.contains("teto-mcp"));
}

#[test]
fn patch_codex_merges_existing() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = tmp.path().join("config.toml");
    std::fs::write(&cfg, "[other]\nfoo = \"bar\"\n").unwrap();
    // YAML block scalar needs indented content for serde_yaml 0.9
    let block: serde_yaml::Value = serde_yaml::from_str(
        "|\n  [mcp_servers.\"teto.local\"]\n  command = \"teto-mcp\"\n",
    )
    .unwrap();
    apply_mcp_patch("codex", &cfg, &block).unwrap();
    let txt = std::fs::read_to_string(&cfg).unwrap();
    assert!(txt.contains("[other]"));
    assert!(txt.contains("teto.local"));
}

#[test]
fn patch_unknown_client_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = tmp.path().join("x.cfg");
    let block: serde_yaml::Value = serde_yaml::from_str("foo: bar").unwrap();
    assert!(apply_mcp_patch("nope", &cfg, &block).is_err());
}
