//! `.param` assignment scanning for the netlist editor.
//!
//! Completion and the document's parameter list need the name and the byte
//! span of every assigned value, without parsing the deck. The spans are
//! line-local: callers use them to locate a value inside the line they already
//! hold, not to index the whole buffer. The character-run helpers here are
//! shared with the highlighter.

/// Parse a `.param` line into `(name, value_start, value_end)` triples
/// (byte offsets into `line`). Returns `None` for non-`.param` lines.
pub(super) fn scan_assignments(line: &str) -> Option<Vec<(String, usize, usize)>> {
    let trimmed = line.trim_start();
    let prefix_len = line.len() - trimmed.len();
    let lower = trimmed.to_ascii_lowercase();
    if !(lower.starts_with(".param") || lower.starts_with(".parameter")) {
        return None;
    }
    // Every offset below comes from a character search, so each slice falls
    // on a character boundary whatever the line holds.
    let mut i = trimmed.find(char::is_whitespace)?;
    let mut out = Vec::new();

    loop {
        i = run_end(trimmed, i, char::is_whitespace);
        if trimmed[i..].is_empty() || trimmed[i..].starts_with([';', '$']) {
            break;
        }
        // Name token.
        let name_start = i;
        i = run_end(trimmed, i, |ch| !ch.is_whitespace() && ch != '=');
        let name = trimmed[name_start..i].to_owned();
        // Skip to '='.
        i = run_end(trimmed, i, char::is_whitespace);
        if !trimmed[i..].starts_with('=') {
            break; // malformed tail; the parser diagnostics own this
        }
        i = run_end(trimmed, i + 1, char::is_whitespace);
        if i == trimmed.len() {
            break;
        }
        // Value: `{balanced expression}` or a bare token.
        let value_start = i;
        i = if trimmed[i..].starts_with('{') {
            expression_end(trimmed, i)
        } else {
            run_end(trimmed, i, |ch| !ch.is_whitespace())
        };
        out.push((name, prefix_len + value_start, prefix_len + i));
    }

    Some(out)
}

/// End of the run of characters from byte `from` that `keep` accepts: the
/// byte offset of the first character it refuses, or the end of the text.
pub(super) fn run_end(text: &str, from: usize, keep: impl Fn(char) -> bool) -> usize {
    text[from..]
        .find(|ch: char| !keep(ch))
        .map_or(text.len(), |offset| from + offset)
}

/// End of the `{expression}` that opens at byte `start`: just past its
/// matching brace, or the end of the text when it never closes.
pub(super) fn expression_end(text: &str, start: usize) -> usize {
    let mut depth = 0i32;
    for (offset, ch) in text[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return start + offset + 1;
                }
            }
            _ => {}
        }
    }
    text.len()
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

    #[test]
    fn non_ascii_assignments_scan_on_character_boundaries() {
        let line = ".param\u{a0}gain=2\u{a0}tà=1Å vbias={gain*入} e\u{85}=\u{85}😀";
        let rows = scan_assignments(line).expect("a .param line");
        let read: Vec<(&str, &str)> = rows
            .iter()
            .map(|(name, start, end)| (name.as_str(), &line[*start..*end]))
            .collect();

        assert_eq!(
            read,
            vec![
                ("gain", "2"),
                ("tà", "1Å"),
                ("vbias", "{gain*入}"),
                ("e", "😀")
            ]
        );
    }

    #[test]
    fn a_param_line_with_any_character_anywhere_scans_without_a_split_character() {
        let row = ".param gain=2 vbias={gain*1.5} x = y";
        let boundaries = row.char_indices().map(|(at, _)| at).chain([row.len()]);
        for at in boundaries {
            for character in ["\u{a0}", "\u{85}", "à", "Å", "Р", "入", "😀"] {
                let line = format!("{}{character}{}", &row[..at], &row[at..]);
                for (_, start, end) in scan_assignments(&line).into_iter().flatten() {
                    assert!(line.is_char_boundary(start) && line.is_char_boundary(end));
                }
            }
        }
    }
}
