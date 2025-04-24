use swc_common::{SourceMap, Span};

pub fn format_location_message(span: Span, cm: &SourceMap, message: &str) -> String {
    let loc = cm.lookup_char_pos(span.lo());
    let file = loc.file.name.to_string();
    let line = loc.line;
    let col = loc.col_display; // already 1-based for VSCode

    format!("[{}:{}:{}] {}", file, line, col, message)
}
