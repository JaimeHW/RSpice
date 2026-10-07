//! Logical CSV/TSV records and quoted fields shared by every result importer.

/// Borrow complete records, retaining embedded line endings and physical lines.
/// Quote syntax is checked by `parse_record`; escaped pairs leave quote state
/// unchanged here, so their contents cannot split a logical record.
pub(crate) fn records(content: &str, separator: char) -> impl Iterator<Item = (usize, &str)> {
    // A UTF-8 signature is transport metadata only at the start of the file.
    // Keep U+FEFF inside quoted fields and in all subsequent records intact.
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut start = 0;
    let mut start_line = 1;
    let mut line = 1;
    let mut quoted = false;
    let mut offsets = content.char_indices().peekable();
    std::iter::from_fn(move || {
        while let Some((index, ch)) = offsets.next() {
            if ch == '"' {
                quoted = !quoted;
            }
            if ch == '\n' || ch == '\r' {
                let mut end = index + 1;
                if ch == '\r' && offsets.peek().is_some_and(|(_, next)| *next == '\n') {
                    offsets.next();
                    end += 1;
                }
                line += 1;
                if !quoted {
                    let record = (start_line, &content[start..index]);
                    start = end;
                    start_line = line;
                    return Some(record);
                }
            }
        }
        if start < content.len() {
            let record = (start_line, &content[start..]);
            start = content.len();
            Some(record)
        } else {
            None
        }
    })
    // In TSV a tab is both whitespace and a field separator. A record made
    // only of separators still contains fields; skipping it can discard a
    // header or conceal a row whose numeric samples are missing.
    .filter(move |(_, record)| !record.trim().is_empty() || record.contains(separator))
}

pub(crate) fn parse_record(line: &str, separator: char) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    let mut quoted = false;

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                    quoted = true;
                }
            } else {
                field.push(ch);
            }
            continue;
        }

        if ch == separator {
            fields.push(finish_delimited_field(&field, quoted));
            field.clear();
            quoted = false;
        } else if quoted {
            if !ch.is_whitespace() {
                return Err("unexpected characters after closing quote".into());
            }
        } else if ch == '"' {
            if !field.trim().is_empty() {
                return Err("unexpected quote in an unquoted field".into());
            }
            field.clear();
            in_quotes = true;
        } else {
            field.push(ch);
        }
    }

    if in_quotes {
        return Err("unterminated quoted field".to_string());
    }
    fields.push(finish_delimited_field(&field, quoted));
    Ok(fields)
}

fn finish_delimited_field(field: &str, quoted: bool) -> String {
    if quoted {
        field.to_string()
    } else {
        field.trim().to_string()
    }
}
