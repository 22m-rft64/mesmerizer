use crate::error::MesError;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackSpec {
    pub file: PathBuf,
    pub start: usize,
    pub end: usize,
}

pub fn parse_spec(spec: &str) -> Result<PackSpec, MesError> {
    // Accept "path/file:start-end", "path/file:line", or just "path/file".
    let (file_part, range_part) = match spec.rsplit_once(':') {
        Some((f, r)) if r.chars().all(|c| c.is_ascii_digit() || c == '-') => (f, Some(r)),
        _ => (spec, None),
    };
    let (start, end) = match range_part {
        None => (0, usize::MAX),
        Some(r) => {
            if let Some((a, b)) = r.split_once('-') {
                let s: usize = a
                    .parse()
                    .map_err(|_| MesError::Parse(format!("bad start: {a}")))?;
                let e: usize = b
                    .parse()
                    .map_err(|_| MesError::Parse(format!("bad end: {b}")))?;
                if s > e {
                    return Err(MesError::Parse(format!("start > end: {s} > {e}")));
                }
                (s, e)
            } else {
                let n: usize = r
                    .parse()
                    .map_err(|_| MesError::Parse(format!("bad line: {r}")))?;
                (n, n)
            }
        }
    };
    Ok(PackSpec {
        file: PathBuf::from(file_part),
        start,
        end,
    })
}
