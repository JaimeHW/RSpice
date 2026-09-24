//! `.param` assignment scanning for the netlist editor.
//!
//! Completion and the document's parameter list need the name and the byte
//! span of every assigned value, without parsing the deck. The spans are
//! line-local: callers use them to locate a value inside the line they already
//! hold, not to index the whole buffer.

/// Parse a `.param` line into `(name, value_start, value_end)` triples
/// (byte offsets into `line`). Returns `None` for non-`.param` lines.
pub(super) fn scan_assignments(line: &str) -> Option<Vec<(String, usize, usize)>> {
    let trimmed = line.trim_start();
    let prefix_len = line.len() - trimmed.len();
    let lower = trimmed.to_ascii_lowercase();
    if !(lower.starts_with(".param") || lower.starts_with(".parameter")) {
        return None;
    }
    let after_cmd = trimmed.find(char::is_whitespace)?;
    let mut out = Vec::new();
    let bytes = trimmed.as_bytes();
    let mut i = after_cmd;

    while i < bytes.len() {
        // Skip whitespace.
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b';' || bytes[i] == b'$' {
            break;
        }
        // Name token.
        let name_start = i;
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch.is_whitespace() || ch == '=' {
                break;
            }
            i += 1;
        }
        let name = trimmed[name_start..i].to_owned();
        // Skip to '='.
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            break; // malformed tail; the parser diagnostics own this
        }
        i += 1;
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        // Value: `{balanced expression}` or a bare token.
        let value_start = i;
        if bytes[i] == b'{' {
            let mut depth = 0i32;
            while i < bytes.len() {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        } else {
            while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                i += 1;
            }
        }
        out.push((name, prefix_len + value_start, prefix_len + i));
    }

    Some(out)
}

/// Every `.param` assignment in the buffer, for completion: the spans are
/// line-local and only the names matter to callers.
pub(super) fn buffer_assignments(buffer: &str) -> Vec<(String, usize, usize)> {
    buffer
        .lines()
        .filter_map(scan_assignments)
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_multiple_assignments_with_spans() {
        let line = "  .param itail=20u  cl = 1.5p vdd={supply*2}";
        let rows = scan_assignments(line).expect("a .param line");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "itail");
        assert_eq!(&line[rows[0].1..rows[0].2], "20u");
        assert_eq!(rows[1].0, "cl");
        assert_eq!(&line[rows[1].1..rows[1].2], "1.5p");
        assert_eq!(rows[2].0, "vdd");
        assert_eq!(&line[rows[2].1..rows[2].2], "{supply*2}");
    }

    #[test]
    fn non_param_lines_are_ignored() {
        assert!(scan_assignments("R1 in out 4.7k").is_none());
        assert!(scan_assignments("* .param comment=1").is_none());
    }

    #[test]
    fn buffer_assignments_span_every_param_line() {
        let buffer = ".title deck\n.param a=1\nR1 in out 1k\n.param b=2 c={a*2}\n";
        let names: Vec<String> = buffer_assignments(buffer)
            .into_iter()
            .map(|(name, _, _)| name)
            .collect();

        assert_eq!(names, vec!["a", "b", "c"]);
    }
}
