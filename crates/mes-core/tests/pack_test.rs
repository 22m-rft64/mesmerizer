use mes_core::pack::{pack, parse::parse_spec};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn parse_simple_range() {
    let s = parse_spec("src/foo.py:10-20").unwrap();
    assert_eq!(s.file, PathBuf::from("src/foo.py"));
    assert_eq!(s.start, 10);
    assert_eq!(s.end, 20);
}

#[test]
fn parse_single_line() {
    let s = parse_spec("foo.py:5").unwrap();
    assert_eq!(s.start, 5);
    assert_eq!(s.end, 5);
}

#[test]
fn parse_no_range() {
    let s = parse_spec("foo.py").unwrap();
    assert_eq!(s.start, 0);
    assert_eq!(s.end, usize::MAX);
}

#[test]
fn parse_inverted_errors() {
    assert!(parse_spec("foo.py:20-10").is_err());
}

#[test]
fn pack_python_function() {
    let mut f = NamedTempFile::with_suffix(".py").unwrap();
    write!(
        f,
        "import os\nfrom pathlib import Path\n\ndef hello(name):\n    print(f'hello {{name}}')\n    return 42\n"
    )
    .unwrap();
    let path = f.path().to_string_lossy().to_string();
    let result = pack(&format!("{path}:4-6")).unwrap();
    assert!(result.contains("hello(name)"));
    assert!(result.contains("import os"));
    assert!(result.contains("```python"));
}

#[test]
fn pack_rust_function() {
    let mut f = NamedTempFile::with_suffix(".rs").unwrap();
    write!(
        f,
        "use std::io;\n\nfn hello(name: &str) {{\n    println!(\"hi {{name}}\");\n}}\n"
    )
    .unwrap();
    let path = f.path().to_string_lossy().to_string();
    let result = pack(&format!("{path}:3-5")).unwrap();
    assert!(result.contains("fn hello"));
    assert!(result.contains("use std::io"));
    assert!(result.contains("```rust"));
}
