use crate::teto::env_ref::EnvRef;
use crate::teto::error::TetoError;
use std::fs;
use std::path::{Path, PathBuf};

/// Walk `<root>/*/SKILL.md` and load each as an EnvRef.
/// Missing root returns empty vec, not error.
pub fn load_env_refs(root: &Path) -> Result<Vec<EnvRef>, TetoError> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let skill = entry.path().join("SKILL.md");
        if !skill.exists() {
            continue;
        }
        let text = fs::read_to_string(&skill)?;
        let mut er = EnvRef::from_skill_md(&text).map_err(|e| match e {
            TetoError::Parse { msg, .. } => TetoError::Parse {
                file: skill.to_string_lossy().into_owned(),
                msg,
            },
            other => other,
        })?;
        er.source_path = Some(skill);
        out.push(er);
    }
    Ok(out)
}

pub fn default_root() -> PathBuf {
    directories::ProjectDirs::from("", "", "mes")
        .map(|p| p.data_local_dir().join("skills").join("env"))
        .unwrap_or_else(|| PathBuf::from("~/.local/share/mes/skills/env"))
}

/// Prefix-match a category filter. `filter="crypto"` matches "crypto" and "crypto/math".
pub fn category_matches(category: &str, filter: &str) -> bool {
    category == filter || category.starts_with(&format!("{filter}/"))
}
