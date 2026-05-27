use crate::teto::check::{CheckResult, CheckStatus};
use crate::teto::mcp::McpEntry;
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

pub fn mcp_list_text(entries: &[McpEntry], no_color: bool) -> String {
    if entries.is_empty() {
        return "(no MCP servers declared in any env_ref)\n".into();
    }
    let mut out = String::new();
    for e in entries {
        let id_styled = if no_color {
            e.id.clone()
        } else {
            e.id.bold().to_string()
        };
        out.push_str(&format!(
            "{}  ({})\n  from: {}\n  installable for: {}\n  cmd: {}\n\n",
            id_styled,
            e.transport,
            e.env_ref,
            if e.available_for.is_empty() {
                "(none)".into()
            } else {
                e.available_for.join(", ")
            },
            e.server_cmd.join(" "),
        ));
    }
    out
}

pub fn mcp_list_json(entries: &[McpEntry]) -> Result<String, serde_json::Error> {
    let arr: Vec<_> = entries
        .iter()
        .map(|e| {
            serde_json::json!({
                "id": e.id,
                "env_ref": e.env_ref,
                "transport": e.transport,
                "server_cmd": e.server_cmd,
                "available_for": e.available_for,
            })
        })
        .collect();
    serde_json::to_string_pretty(&arr)
}

use crate::teto::doctor::DoctorReport;

pub fn doctor_text(report: &DoctorReport, no_color: bool) -> String {
    let mut out = String::new();

    out.push_str("system\n");
    out.push_str(&format!("  os: {}\n", report.sys.os));
    out.push_str(&format!("  arch: {}\n", report.sys.arch));
    out.push_str(&format!("  shell: {}\n", report.sys.shell));
    out.push_str("  key tools:\n");
    for t in &report.sys.key_tools {
        let v = t.version.as_deref().unwrap_or("(not found)");
        out.push_str(&format!("    {:<10} {}\n", t.name, v));
    }
    out.push('\n');

    out.push_str(&format!("env_refs: {}\n", report.env_refs.len()));
    for r in &report.env_refs {
        out.push_str(&format!(
            "  - {}  [{}]  {}\n",
            r.name, r.category, r.description
        ));
    }
    out.push('\n');

    out.push_str("checks:\n");
    out.push_str(&check_text(&report.checks, no_color));
    out.push('\n');

    out.push_str("mcp servers:\n");
    out.push_str(&mcp_list_text(&report.mcps, no_color));

    out
}

pub fn doctor_json(report: &DoctorReport) -> Result<String, serde_json::Error> {
    let key_tools: Vec<_> = report
        .sys
        .key_tools
        .iter()
        .map(|t| serde_json::json!({ "name": t.name, "version": t.version }))
        .collect();
    let env_refs: Vec<_> = report
        .env_refs
        .iter()
        .map(|r| {
            serde_json::json!({
                "name": r.name,
                "category": r.category,
                "description": r.description,
            })
        })
        .collect();
    let check_status = |s: &crate::teto::check::CheckStatus| match s {
        crate::teto::check::CheckStatus::Pass => "pass",
        crate::teto::check::CheckStatus::Fail { .. } => "fail",
        crate::teto::check::CheckStatus::ExecError { .. } => "error",
    };
    let checks: Vec<_> = report
        .checks
        .iter()
        .map(|c| {
            serde_json::json!({
                "env_ref": c.env_ref,
                "category": c.category,
                "id": c.id,
                "desc": c.desc,
                "status": check_status(&c.status),
            })
        })
        .collect();
    let mcps: Vec<_> = report
        .mcps
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "env_ref": m.env_ref,
                "transport": m.transport,
                "server_cmd": m.server_cmd,
                "available_for": m.available_for,
            })
        })
        .collect();
    let payload = serde_json::json!({
        "system": {
            "os": report.sys.os,
            "arch": report.sys.arch,
            "shell": report.sys.shell,
            "path": report.sys.path,
            "key_tools": key_tools,
        },
        "env_refs": env_refs,
        "checks": checks,
        "mcps": mcps,
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

    #[test]
    fn render_mcp_list_text_empty() {
        let s = mcp_list_text(&[], true);
        assert!(s.contains("(no MCP servers"));
    }

    #[test]
    fn render_mcp_list_json_array() {
        let e = McpEntry {
            env_ref: "tk".into(),
            id: "teto".into(),
            transport: "stdio".into(),
            server_cmd: vec!["teto-mcp".into()],
            available_for: vec!["claude".into(), "codex".into()],
        };
        let j = mcp_list_json(&[e]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&j).unwrap();
        assert_eq!(v[0]["id"], "teto");
        assert_eq!(v[0]["available_for"].as_array().unwrap().len(), 2);
    }
}
