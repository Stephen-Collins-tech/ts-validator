use swc_common::{SourceMap, Spanned, FileName, SourceMapper};
use std::sync::Arc;
use std::fmt::Debug;

/// Logs the AST node, its location, and the source code snippet.
pub fn log_node<T: Spanned + Debug>(label: &str, cm: &Arc<SourceMap>, node: &T, level: usize) {
    let indent = "   ".repeat(level);
    let span = node.span();
    let loc = cm.lookup_char_pos(span.lo());
    let snippet = cm.span_to_snippet(span).unwrap_or_else(|_| "<could not get snippet>".into());

    let file = match &*loc.file.name {
        FileName::Real(real) => real.display().to_string(),
        other => format!("{:?}", other),
    };

    println!(
        "{}[{}] {}:{}:{}\n{}  └─ AST: {:?}\n{}  └─ Code: {}",
        indent,
        label,
        file,
        loc.line,
        loc.col_display + 1,
        indent,
        node,
        indent,
        snippet.trim().replace('\n', "\\n")
    );
}
