use crate::error::MesError;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

pub struct ExtractedContext {
    pub imports: Vec<String>,
    pub enclosing_signatures: Vec<String>,
    pub selection: String,
    pub language: &'static str,
}

pub fn extract(
    file: &Path,
    source: &str,
    start_line: usize,
    end_line: usize,
) -> Result<ExtractedContext, MesError> {
    let lang = detect_language(file);
    let (language, import_query_src, function_query_src) = match lang {
        "python" => (
            tree_sitter_python::LANGUAGE.into(),
            "(import_statement) @i\n(import_from_statement) @i",
            "(function_definition) @f",
        ),
        "rust" => (
            tree_sitter_rust::LANGUAGE.into(),
            "(use_declaration) @i",
            "(function_item) @f",
        ),
        _ => {
            let total = source.lines().count();
            return Ok(ExtractedContext {
                imports: vec![],
                enclosing_signatures: vec![],
                selection: slice_lines(source, start_line, end_line.min(total)),
                language: "unknown",
            });
        }
    };
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .map_err(|e| MesError::Parse(format!("set_language: {e}")))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| MesError::Parse("parse returned None".into()))?;
    let root = tree.root_node();
    let import_q = Query::new(&language, import_query_src)
        .map_err(|e| MesError::Parse(format!("import query: {e}")))?;
    let function_q = Query::new(&language, function_query_src)
        .map_err(|e| MesError::Parse(format!("function query: {e}")))?;
    let mut cursor = QueryCursor::new();
    let mut imports = Vec::new();
    let mut matches = cursor.matches(&import_q, root, source.as_bytes());
    while let Some(m) = matches.next() {
        for cap in m.captures {
            let text = &source[cap.node.byte_range()];
            imports.push(text.to_string());
        }
    }
    let mut sigs = Vec::new();
    let mut cursor2 = QueryCursor::new();
    let mut fm = cursor2.matches(&function_q, root, source.as_bytes());
    while let Some(m) = fm.next() {
        for cap in m.captures {
            let node = cap.node;
            let start = node.start_position().row + 1;
            let end = node.end_position().row + 1;
            // Only report as "enclosing" if the function's header line is NOT
            // visible in the selection itself (otherwise it would duplicate).
            let header_in_selection = start >= start_line && start <= end_line;
            let overlaps = !(end < start_line || start > end_line);
            if overlaps && !header_in_selection {
                let node_text = &source[node.byte_range()];
                let first_line_end = node_text.find('\n').unwrap_or(node_text.len());
                let header = &node_text[..first_line_end];
                sigs.push(header.to_string());
            }
        }
    }
    let total = source.lines().count();
    let selection = slice_lines(source, start_line, end_line.min(total));
    Ok(ExtractedContext {
        imports,
        enclosing_signatures: sigs,
        selection,
        language: lang,
    })
}

fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("py") | Some("sage") => "python",
        Some("rs") => "rust",
        Some("js") | Some("ts") | Some("tsx") | Some("jsx") => "javascript",
        Some("c") | Some("h") | Some("cpp") | Some("cc") | Some("hpp") => "c",
        Some("lua") => "lua",
        _ => "unknown",
    }
}

fn slice_lines(source: &str, start: usize, end: usize) -> String {
    if start == 0 {
        return source.to_string();
    }
    let lines: Vec<&str> = source.lines().collect();
    if start > lines.len() {
        return String::new();
    }
    let s = start.saturating_sub(1);
    let e = end.min(lines.len());
    lines[s..e].join("\n")
}
