use mes_core::template::{builtin, render::render};

#[test]
fn builtin_explain_exists() {
    let t = builtin("explain").unwrap();
    assert!(t.contains("{context}"));
}

#[test]
fn builtin_debug_exists() {
    let t = builtin("debug").unwrap();
    assert!(t.contains("{context}"));
}

#[test]
fn render_substitutes_context() {
    let t = "ask about: {context}";
    let r = render(t, "the code");
    assert_eq!(r, "ask about: the code");
}

#[test]
fn unknown_template_returns_none() {
    assert!(builtin("nonexistent").is_none());
}
