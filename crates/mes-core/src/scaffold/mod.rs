pub mod templates;

use crate::error::MesError;
use std::fs;
use std::path::Path;

pub fn scaffold(category: &str, target_dir: &Path) -> Result<String, MesError> {
    let (filename, content) = templates::resolve(category)?;
    let target = target_dir.join(filename);
    if target.exists() {
        return Err(MesError::Parse(format!(
            "{} already exists at target dir, refusing to overwrite",
            filename
        )));
    }
    fs::write(&target, content)?;
    Ok(target.to_string_lossy().into_owned())
}
