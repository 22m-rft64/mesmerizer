use mes_core::scaffold::scaffold;
use tempfile::TempDir;

#[test]
fn scaffold_pwn_creates_solve_py() {
    let dir = TempDir::new().unwrap();
    let result = scaffold("pwn", dir.path()).unwrap();
    assert!(result.ends_with("solve.py"));
    let content = std::fs::read_to_string(dir.path().join("solve.py")).unwrap();
    assert!(content.contains("from pwn import *"));
}

#[test]
fn scaffold_crypto_sage() {
    let dir = TempDir::new().unwrap();
    let result = scaffold("crypto", dir.path()).unwrap();
    assert!(result.ends_with("solve.sage"));
}

#[test]
fn scaffold_pwn_kernel_emits_c() {
    let dir = TempDir::new().unwrap();
    let result = scaffold("pwn/kernel", dir.path()).unwrap();
    assert!(result.ends_with("solve.c"));
}

#[test]
fn scaffold_unknown_category_errors() {
    let dir = TempDir::new().unwrap();
    let result = scaffold("foobar", dir.path());
    assert!(result.is_err());
}

#[test]
fn scaffold_refuses_overwrite() {
    let dir = TempDir::new().unwrap();
    scaffold("pwn", dir.path()).unwrap();
    let second = scaffold("pwn", dir.path());
    assert!(second.is_err());
}
