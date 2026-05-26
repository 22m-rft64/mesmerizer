pub mod extract;
pub mod format;
pub mod parse;

use crate::error::MesError;
use std::fs;

pub fn pack(spec_str: &str) -> Result<String, MesError> {
    let mut spec = parse::parse_spec(spec_str)?;
    let source = fs::read_to_string(&spec.file)?;
    let total = source.lines().count();
    if spec.end == usize::MAX {
        spec.end = total;
    }
    let ctx = extract::extract(&spec.file, &source, spec.start.max(1), spec.end)?;
    Ok(format::format_markdown(&spec, &ctx))
}
