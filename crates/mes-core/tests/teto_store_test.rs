use mes_core::teto::store::load_env_refs;
use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/teto_env_refs")
}

#[test]
fn load_all_env_refs() {
    let refs = load_env_refs(&fixtures_dir()).unwrap();
    assert_eq!(refs.len(), 2);
    let names: Vec<_> = refs.iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"crypto-toolkit"));
    assert!(names.contains(&"pwn-toolkit"));
}

#[test]
fn load_empty_dir_returns_empty() {
    let tmp = tempfile::tempdir().unwrap();
    let refs = load_env_refs(tmp.path()).unwrap();
    assert!(refs.is_empty());
}

#[test]
fn load_missing_dir_returns_empty() {
    let refs = load_env_refs(std::path::Path::new("/nonexistent/whatever")).unwrap();
    assert!(refs.is_empty());
}

#[test]
fn category_filter() {
    let refs = load_env_refs(&fixtures_dir()).unwrap();
    let crypto: Vec<_> = refs
        .iter()
        .filter(|r| mes_core::teto::store::category_matches(&r.category, "crypto"))
        .collect();
    assert_eq!(crypto.len(), 1);
    assert!(mes_core::teto::store::category_matches("pwn/userland", "pwn"));
    assert!(mes_core::teto::store::category_matches("pwn", "pwn"));
    assert!(!mes_core::teto::store::category_matches("crypto", "pwn"));
}
