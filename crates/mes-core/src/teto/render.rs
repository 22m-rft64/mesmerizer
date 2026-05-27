use crate::teto::check::{CheckResult, CheckStatus};
use colored::Colorize;
use std::collections::BTreeMap;

/// Render check results as a CLI-style report grouped by category.
/// Color: ✓ green, ✗ red, category gray. Disable color when `no_color` is true.
pub fn check_text(results: &[CheckResult], no_color: bool) -> String {
    let mut by_cat: BTreeMap<&str, Vec<&CheckResult>> = BTreeMap::new();
    for r in results {
        by_cat.entry(r.category.as_str()).or_default().push(r);
    }

    let mut out = String::new();
    let (mut pass, mut fail) = (0usize, 0usize);

    for (cat, group) in &by_cat {
        let cat_label = format!("[{cat}]");
        let cat_styled = if no_color {
            cat_label
        } else {
            cat_label.bright_black().to_string()
        };
        out.push_str(&cat_styled);
        out.push('\n');

        let mut by_ref: BTreeMap<&str, Vec<&CheckResult>> = BTreeMap::new();
        for r in group {
            by_ref.entry(r.env_ref.as_str()).or_default().push(r);
        }
        for (env_ref, items) in &by_ref {
            out.push_str("  ");
            out.push_str(env_ref);
            out.push('\n');
            for r in items {
                let (mark, kind) = match &r.status {
                    CheckStatus::Pass => ("✓", "pass"),
                    CheckStatus::Fail { .. } => ("✗", "fail"),
                    CheckStatus::ExecError { .. } => ("!", "error"),
                };
                let mark_styled = if no_color {
                    mark.to_string()
                } else {
                    match kind {
                        "pass" => mark.green().to_string(),
                        "fail" => mark.red().to_string(),
                        _ => mark.yellow().to_string(),
                    }
                };
                out.push_str(&format!(
                    "    {}  {:<14} {}\n",
                    mark_styled, r.id, r.desc
                ));
                match &r.status {
                    CheckStatus::Pass => pass += 1,
                    _ => fail += 1,
                }
            }
        }
        out.push('\n');
    }

    let total = results.len();
    out.push_str(&format!("{total} checks, {pass} pass, {fail} fail\n"));
    out
}

pub fn check_json(results: &[CheckResult]) -> Result<String, serde_json::Error> {
    let entries: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            let status_str = match &r.status {
                CheckStatus::Pass => "pass",
                CheckStatus::Fail { .. } => "fail",
                CheckStatus::ExecError { .. } => "error",
            };
            serde_json::json!({
                "env_ref": r.env_ref,
                "category": r.category,
                "id": r.id,
                "desc": r.desc,
                "status": status_str,
            })
        })
        .collect();
    let pass = results
        .iter()
        .filter(|r| matches!(r.status, CheckStatus::Pass))
        .count();
    let fail = results.len() - pass;
    let payload = serde_json::json!({
        "checks": entries,
        "summary": { "pass": pass, "fail": fail, "total": results.len() }
    });
    serde_json::to_string_pretty(&payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teto::check::{CheckResult, CheckStatus};

    fn mk(env_ref: &str, cat: &str, id: &str, desc: &str, pass: bool) -> CheckResult {
        CheckResult {
            env_ref: env_ref.into(),
            category: cat.into(),
            id: id.into(),
            desc: desc.into(),
            status: if pass {
                CheckStatus::Pass
            } else {
                CheckStatus::Fail {
                    code: Some(1),
                    stderr: String::new(),
                }
            },
        }
    }

    #[test]
    fn render_groups_by_category() {
        let rs = vec![
            mk("a", "crypto", "x", "first", true),
            mk("a", "crypto", "y", "second", false),
            mk("b", "pwn", "z", "third", true),
        ];
        let text = check_text(&rs, true);
        assert!(text.contains("[crypto]"));
        assert!(text.contains("[pwn]"));
        assert!(text.contains("3 checks, 2 pass, 1 fail"));
    }

    #[test]
    fn render_no_color_strips_ansi() {
        let rs = vec![mk("a", "crypto", "x", "first", true)];
        let text = check_text(&rs, true);
        assert!(!text.contains("\x1b["));
    }

    #[test]
    fn render_json() {
        let rs = vec![
            mk("a", "crypto", "x", "first", true),
            mk("a", "crypto", "y", "second", false),
        ];
        let json = check_json(&rs).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["checks"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["summary"]["pass"], 1);
        assert_eq!(parsed["summary"]["fail"], 1);
        assert_eq!(parsed["summary"]["total"], 2);
    }
}
