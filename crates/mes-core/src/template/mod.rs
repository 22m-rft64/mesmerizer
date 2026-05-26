pub mod render;

use crate::error::MesError;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn templates_dir() -> Result<PathBuf, MesError> {
    let dirs = ProjectDirs::from("", "", "mes")
        .ok_or_else(|| MesError::NotFound("XDG project dirs".into()))?;
    Ok(dirs.config_dir().join("templates"))
}

pub fn load_template(name: &str) -> Result<String, MesError> {
    let path = templates_dir()?.join(format!("{name}.md"));
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        builtin(name).ok_or_else(|| MesError::NotFound(format!("template: {name}")))
    }
}

pub fn builtin(name: &str) -> Option<String> {
    let body = match name {
        "explain" => "Explain what this code does in one sentence.\n\n{context}\n",
        "debug" => "Find bugs in this code. For each bug, give line number and a fix.\n\n{context}\n",
        "shorten" => {
            "Rewrite this code preserving exact behavior. Make it shorter (aim 30-50%).\n\n{context}\n"
        }
        "harden" => "List the input assumptions of this code. For each assumption, describe what breaks if it's violated.\n\n{context}\n",
        _ => return None,
    };
    Some(body.to_string())
}
