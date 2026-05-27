use mes_core::teto::exec::{expand_path, run_shell, ShellOutcome};

#[test]
fn run_shell_zero_exit() {
    let out = run_shell("true").unwrap();
    assert!(matches!(out, ShellOutcome::Ok { .. }));
}

#[test]
fn run_shell_nonzero_exit() {
    let out = run_shell("false").unwrap();
    match out {
        ShellOutcome::Fail { code, .. } => assert_ne!(code, Some(0)),
        _ => panic!("expected Fail"),
    }
}

#[test]
fn run_shell_captures_stdout() {
    let out = run_shell("echo hello").unwrap();
    match out {
        ShellOutcome::Ok { stdout, .. } => assert_eq!(stdout.trim(), "hello"),
        _ => panic!(),
    }
}

#[test]
fn expand_path_home() {
    let expanded = expand_path("~/foo");
    let home = std::env::var("HOME").unwrap();
    assert_eq!(expanded, format!("{home}/foo"));
}

#[test]
fn expand_path_env_var() {
    std::env::set_var("TETO_TEST_VAR", "/etc");
    let expanded = expand_path("$TETO_TEST_VAR/passwd");
    assert_eq!(expanded, "/etc/passwd");
}

#[test]
fn expand_path_braced_env_var() {
    std::env::set_var("TETO_TEST_BRACE", "/usr/local");
    let expanded = expand_path("${TETO_TEST_BRACE}/bin");
    assert_eq!(expanded, "/usr/local/bin");
}

#[test]
fn expand_path_no_var_unchanged() {
    let expanded = expand_path("/plain/path");
    assert_eq!(expanded, "/plain/path");
}
