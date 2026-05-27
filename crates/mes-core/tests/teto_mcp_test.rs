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
