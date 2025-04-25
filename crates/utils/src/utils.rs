use swc_common::{SourceMap, Span};

pub fn format_location_message(span: Span, cm: &SourceMap, message: &str) -> String {
    let loc = cm.lookup_char_pos(span.lo());
    let file = loc.file.name.to_string();
    let line = loc.line;
    let col = loc.col_display; // already 1-based for VSCode

    format!("[{}:{}:{}] {}", file, line, col, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_common::{SourceMap, Span, BytePos, FileName, sync::Lrc};
    use std::sync::Arc;

    #[test]
    fn test_format_location_message() {
        // Mock SourceMap and Span
        let cm = Arc::new(SourceMap::default());
        let _sf = cm.new_source_file(Lrc::new(FileName::Custom("test.js".into())), "let x = 1;".into());
        let span = Span::new(BytePos(4), BytePos(5)); // Span for 'x'

        let message = "Test message";

        let actual = format_location_message(span, &cm, message);

        let loc = cm.lookup_char_pos(span.lo());
        let expected = format!("[{}:{}:{}] {}", "test.js", loc.line, loc.col_display, message);

        assert_eq!(actual, expected);
    }
}
