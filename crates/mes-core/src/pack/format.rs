use super::extract::ExtractedContext;
use super::parse::PackSpec;

pub fn format_markdown(spec: &PackSpec, ctx: &ExtractedContext) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "`{}:{}-{}`\n\n",
        spec.file.display(),
        spec.start,
        spec.end
    ));
    if !ctx.imports.is_empty() {
        out.push_str("Top imports:\n```");
        out.push_str(ctx.language);
        out.push('\n');
        for imp in &ctx.imports {
            out.push_str(imp.trim());
            out.push('\n');
        }
        out.push_str("```\n\n");
    }
    if !ctx.enclosing_signatures.is_empty() {
        out.push_str("Enclosing function(s):\n");
        for sig in &ctx.enclosing_signatures {
            out.push_str("- `");
            out.push_str(sig.trim());
            out.push_str("`\n");
        }
        out.push('\n');
    }
    out.push_str("Selection:\n```");
    out.push_str(ctx.language);
    out.push('\n');
    out.push_str(&ctx.selection);
    if !ctx.selection.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("```\n");
    out
}
