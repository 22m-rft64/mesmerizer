use mes_core::teto::env_ref::EnvRef;

#[test]
fn parse_minimal_env_ref() {
    let body = r#"---
name: crypto-toolkit
description: test
category: crypto
---

# body
"#;
    let er = EnvRef::from_skill_md(body).unwrap();
    assert_eq!(er.name, "crypto-toolkit");
    assert_eq!(er.description, "test");
    assert_eq!(er.category, "crypto");
    assert!(er.check.is_empty());
    assert!(er.mcp.is_empty());
}

#[test]
fn parse_full_env_ref() {
    let body = r#"---
name: crypto-toolkit
description: full
category: crypto/math
path: $HOME/ctf-tools/crypto
activation:
  - 'export FOO=1'
check:
  - id: env-set
    desc: "var set"
    cmd: 'test -n "$CRYPTO_ROOT"'
deploy:
  source:
    type: git
    repo: "ssh://git@host/r.git"
    branch: main
    sparse_path: ctf-tools/crypto
  install_dir: $HOME/ctf-tools/crypto
  post_install:
    - 'cd "$HOME/ctf-tools/crypto" && make'
repair:
  - id: bad-link
    desc: "link broken"
    detect: 'test -L $TARGET'
    fix: 'ln -sf $SRC $TARGET'
invocations:
  - id: scout
    cmd: 'scout {chal_dir}'
    desc: "triage"
    args:
      - name: chal_dir
        desc: "path"
mcp:
  - id: teto
    server_cmd: ["teto-mcp"]
    transport: stdio
    install_for:
      claude:
        config_path: "~/.claude/mcp.json"
        block:
          teto.local:
            command: "teto-mcp"
      codex:
        config_path: "~/.codex/config.toml"
        block: |
          [mcp_servers."teto.local"]
          command = "teto-mcp"
---

# body
"#;
    let er = EnvRef::from_skill_md(body).unwrap();
    assert_eq!(er.name, "crypto-toolkit");
    assert_eq!(er.category, "crypto/math");
    assert_eq!(er.check.len(), 1);
    assert_eq!(er.check[0].id, "env-set");
    assert!(er.deploy.is_some());
    assert_eq!(er.repair.len(), 1);
    assert_eq!(er.invocations.len(), 1);
    assert_eq!(er.mcp.len(), 1);
    assert_eq!(er.mcp[0].id, "teto");
}

#[test]
fn parse_no_frontmatter_errors() {
    let body = "# just markdown\nno frontmatter\n";
    assert!(EnvRef::from_skill_md(body).is_err());
}

#[test]
fn parse_unknown_field_ignored() {
    let body = r#"---
name: foo
description: bar
category: misc
extra_field: ignored
---
"#;
    let er = EnvRef::from_skill_md(body).unwrap();
    assert_eq!(er.name, "foo");
}
