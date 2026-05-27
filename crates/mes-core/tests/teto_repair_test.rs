use mes_core::teto::env_ref::EnvRef;
use mes_core::teto::repair::{plan_repair, apply_repair, RepairOutcome};

fn er_with_repair(detect: &str, fix: &str) -> EnvRef {
    let body = format!(
        r#"---
name: r
description: test
category: misc
repair:
  - id: only
    desc: "the repair"
    detect: '{detect}'
    fix: '{fix}'
---
"#
    );
    EnvRef::from_skill_md(&body).unwrap()
}

#[test]
fn detect_ok_means_no_repair_needed() {
    let er = er_with_repair("true", "touch /tmp/should-not-run");
    let plan = plan_repair(&[er], "only").unwrap();
    assert_eq!(plan.status, mes_core::teto::repair::RepairStatus::Skip);
}

#[test]
fn detect_fail_means_fix_runs() {
    let tmp = tempfile::tempdir().unwrap();
    let marker = tmp.path().join("marker");
    let er = er_with_repair(
        &format!("test -e {}", marker.display()),
        &format!("touch {}", marker.display()),
    );
    let plan = plan_repair(&[er.clone()], "only").unwrap();
    assert_eq!(plan.status, mes_core::teto::repair::RepairStatus::Needed);
    let result = apply_repair(&plan).unwrap();
    assert!(matches!(result, RepairOutcome::Applied { .. }));
    assert!(marker.exists());
}

#[test]
fn unknown_repair_id() {
    let er = er_with_repair("true", "true");
    let res = plan_repair(&[er], "nope");
    assert!(res.is_err());
}
