use mes_core::teto::check::{run_checks, CheckStatus};
use mes_core::teto::env_ref::EnvRef;

fn fixture(name: &str, cmd: &str) -> EnvRef {
    let body = format!(
        r#"---
name: {name}
description: test
category: misc
check:
  - id: only
    desc: "the check"
    cmd: '{cmd}'
---
"#
    );
    EnvRef::from_skill_md(&body).unwrap()
}

#[test]
fn check_pass() {
    let refs = vec![fixture("ok", "true")];
    let results = run_checks(&refs);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].env_ref, "ok");
    assert_eq!(results[0].id, "only");
    assert!(matches!(results[0].status, CheckStatus::Pass));
}

#[test]
fn check_fail() {
    let refs = vec![fixture("bad", "false")];
    let results = run_checks(&refs);
    assert!(matches!(results[0].status, CheckStatus::Fail { .. }));
}

#[test]
fn check_multiple_env_refs() {
    let refs = vec![fixture("a", "true"), fixture("b", "false")];
    let results = run_checks(&refs);
    assert_eq!(results.len(), 2);
    assert!(matches!(results[0].status, CheckStatus::Pass));
    assert!(matches!(results[1].status, CheckStatus::Fail { .. }));
}
