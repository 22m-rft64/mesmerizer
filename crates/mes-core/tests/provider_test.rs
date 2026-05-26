use mes_core::provider::{claude_code::ClaudeCode, codex::Codex, registry, Provider};

#[test]
fn claude_code_name() {
    let p = ClaudeCode::new();
    assert_eq!(p.name(), "claude_code");
}

#[test]
fn codex_name() {
    let p = Codex::new();
    assert_eq!(p.name(), "codex");
}

#[test]
fn registry_has_both() {
    let reg = registry();
    let names: Vec<_> = reg.iter().map(|p| p.name()).collect();
    assert!(names.contains(&"claude_code"));
    assert!(names.contains(&"codex"));
}
