//! Keep authored report text from becoming protocol syntax.

/// XML 1.0 cannot represent every Unicode scalar, even through references.
/// Preserve allowed whitespace with references so attribute normalization does
/// not change names; render forbidden characters as visible Unicode escapes.
pub(super) fn xml_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            '\t' => escaped.push_str("&#x9;"),
            '\n' => escaped.push_str("&#xA;"),
            '\r' => escaped.push_str("&#xD;"),
            '\u{20}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..='\u{10ffff}' => {
                escaped.push(ch)
            }
            _ => escaped.extend(ch.escape_unicode()),
        }
    }
    escaped
}

/// TAP descriptions are single lines; any hash begins a directive. Use visible
/// escapes, including for backslashes, so literal escape text stays distinct.
pub(super) fn tap_description(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '#' => escaped.push_str("\\x23"),
            '\\' => escaped.push_str("\\\\"),
            ch if ch.is_control() || matches!(ch, '\u{2028}' | '\u{2029}') => {
                escaped.extend(ch.escape_default());
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// JSON quoting is a YAML double-quoted scalar. Also escape the characters
/// that YAML excludes from printable text or normalizes as legacy line breaks.
pub(super) fn tap_yaml_scalar(value: &str) -> String {
    use std::fmt::Write as _;
    let json = serde_json::to_string(value).expect("serializing a string cannot fail");
    let mut escaped = String::with_capacity(json.len());
    for ch in json.chars() {
        if ch.is_control() || matches!(ch, '\u{2028}' | '\u{2029}' | '\u{fffe}' | '\u{ffff}') {
            write!(escaped, "\\u{:04x}", ch as u32).expect("writing to String cannot fail");
        } else {
            escaped.push(ch);
        }
    }
    escaped
}
