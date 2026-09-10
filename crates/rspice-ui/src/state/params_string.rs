//! Shared lexical handling for authored SPICE parameter text.
//!
//! Lookups decode quoted strings; structural edits retain raw values. Both
//! use the same token boundaries, including escaped quotes and nested groups.

use std::collections::HashMap;

/// One authored parameter or bare flag, excluding its surrounding separators.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParameterEntry<'a> {
    pub(crate) raw: &'a str,
    pub(crate) key: &'a str,
    pub(crate) value: Option<&'a str>,
}

pub(crate) struct ParameterEntries<'a> {
    input: &'a str,
    offset: usize,
}

/// Iterate complete entries, stopping after a lexical error. Mutating callers
/// must handle that error before publishing any changes.
pub(crate) fn parameter_entries(input: &str) -> ParameterEntries<'_> {
    ParameterEntries { input, offset: 0 }
}

impl<'a> Iterator for ParameterEntries<'a> {
    type Item = Result<ParameterEntry<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.offset
            + self.input[self.offset..]
                .chars()
                .take_while(|ch| separator(*ch))
                .map(char::len_utf8)
                .sum::<usize>();
        if start == self.input.len() {
            self.offset = start;
            return None;
        }
        match parse_entry(self.input, start) {
            Ok((entry, end)) => {
                self.offset = end;
                Some(Ok(entry))
            }
            Err(error) => {
                self.offset = self.input.len();
                Some(Err(error))
            }
        }
    }
}

fn separator(ch: char) -> bool {
    ch.is_whitespace() || ch == ','
}

fn parse_entry(input: &str, start: usize) -> Result<(ParameterEntry<'_>, usize), String> {
    let key_end = start
        + input[start..]
            .chars()
            .take_while(|ch| !separator(*ch) && *ch != '=')
            .map(char::len_utf8)
            .sum::<usize>();
    let key = &input[start..key_end];
    if key.is_empty() {
        return Err("Expected a parameter name before '='.".to_owned());
    }
    let equals = key_end
        + input[key_end..]
            .chars()
            .take_while(|ch| ch.is_whitespace())
            .map(char::len_utf8)
            .sum::<usize>();
    if !input[equals..].starts_with('=') {
        return Ok((
            ParameterEntry {
                raw: &input[start..key_end],
                key,
                value: None,
            },
            key_end,
        ));
    }
    let value_start = equals
        + 1
        + input[equals + 1..]
            .chars()
            .take_while(|ch| ch.is_whitespace())
            .map(char::len_utf8)
            .sum::<usize>();
    if value_start == input.len() || input[value_start..].starts_with(',') {
        return Err(format!("Parameter '{key}' has no value."));
    }
    let end =
        value_end(input, value_start).map_err(|error| format!("Parameter '{key}': {error}"))?;
    if input[end..].chars().next().is_some_and(|ch| !separator(ch)) {
        return Err(format!("Parameter '{key}' is not followed by a separator."));
    }
    Ok((
        ParameterEntry {
            raw: &input[start..end],
            key,
            value: Some(&input[value_start..end]),
        },
        end,
    ))
}

/// Locate a complete value without interpreting expression or string content.
fn value_end(input: &str, start: usize) -> Result<usize, &'static str> {
    let delimited = input[start..].starts_with(['\'', '"', '{', '[']);
    let mut groups = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    for (relative, ch) in input[start..].char_indices() {
        let end = start + relative + ch.len_utf8();
        if let Some(active) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active {
                quote = None;
                if groups.is_empty() && delimited {
                    return Ok(end);
                }
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '{' => groups.push('}'),
            '[' => groups.push(']'),
            '(' => groups.push(')'),
            '}' | ']' | ')' => {
                if groups.pop() != Some(ch) {
                    return Err("mismatched closing delimiter.");
                }
                if groups.is_empty() && delimited {
                    return Ok(end);
                }
            }
            ch if separator(ch) && groups.is_empty() => return Ok(start + relative),
            _ => {}
        }
    }
    if quote.is_some() {
        Err("unterminated quoted value.")
    } else if !groups.is_empty() {
        Err("unterminated grouped expression.")
    } else {
        Ok(input.len())
    }
}

pub(crate) fn valid_parameter_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Lookup view of complete assignments. Keys are case-insensitive and the
/// last assignment wins. Bare flags are represented as enabled (`1`). For
/// malformed imported text this view stops at the error; mutations must use
/// the checked entries rather than reconstructing text from this lookup.
pub fn parse_params_string(params: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for entry in parameter_entries(params) {
        let Ok(entry) = entry else {
            break;
        };
        let value = entry
            .value
            .map_or_else(|| "1".to_owned(), decode_parameter_value);
        if !value.is_empty() {
            result.insert(entry.key.to_lowercase(), value);
        }
    }
    result
}

fn decode_parameter_value(raw: &str) -> String {
    let Some(quote @ ('\'' | '"')) = raw.chars().next() else {
        return raw.to_owned();
    };
    let mut chars = raw[1..raw.len() - 1].chars().peekable();
    let mut decoded = String::with_capacity(raw.len() - 2);
    while let Some(ch) = chars.next() {
        if ch == '\\'
            && chars
                .peek()
                .is_some_and(|next| *next == quote || *next == '\\')
        {
            decoded.push(chars.next().expect("peeked escape"));
        } else {
            decoded.push(ch);
        }
    }
    decoded
}

fn encode_parameter_value(value: &str) -> String {
    let needs_quotes = value
        .chars()
        .any(|ch| ch.is_whitespace() || matches!(ch, ',' | '=' | '\'' | '"'))
        || value_end(value, 0) != Ok(value.len());
    if needs_quotes {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        value.to_owned()
    }
}

/// Deterministic encoding of decoded values. Empty optional overrides are
/// omitted; quoted spaces, backslashes and quote characters round-trip.
pub fn format_params_string(params: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| format!("{key}={}", encode_parameter_value(value)))
        .collect();
    pairs.sort();
    pairs.join(" ")
}

/// Authoring validation accepts flags and requires unambiguous assignments.
/// A single-field update may repair duplicates of its own key; a whole-text
/// or property-map publication must not silently choose between duplicates.
pub(crate) fn validate_parameter_text(params: &str) -> Result<(), String> {
    let mut names = std::collections::HashSet::new();
    for entry in parameter_entries(params) {
        let entry = entry?;
        if !valid_parameter_name(entry.key) {
            return Err(format!("'{}' is not a valid parameter name.", entry.key));
        }
        if !names.insert(entry.key.to_ascii_lowercase()) {
            return Err(format!(
                "Parameter '{}' is assigned more than once.",
                entry.key
            ));
        }
    }
    Ok(())
}

/// Replace one decoded field, preserving every other raw entry and its order.
/// Collapse only that key's duplicates. A blank field removes all overrides;
/// malformed existing text is refused before any candidate is returned.
pub(crate) fn set_parameter_value(params: &str, key: &str, value: &str) -> Result<String, String> {
    if !valid_parameter_name(key) {
        return Err(format!("'{key}' is not a valid parameter name."));
    }
    let replacement =
        (!value.trim().is_empty()).then(|| format!("{key}={}", encode_parameter_value(value)));
    let mut parts = Vec::new();
    let mut replaced = false;
    for entry in parameter_entries(params) {
        let entry = entry?;
        if entry.key.eq_ignore_ascii_case(key) {
            if !replaced && let Some(replacement) = replacement.as_deref() {
                parts.push(replacement);
            }
            replaced = true;
        } else {
            parts.push(entry.raw);
        }
    }
    if !replaced && let Some(replacement) = replacement.as_deref() {
        parts.push(replacement);
    }
    Ok(parts.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatter_escapes_nested_quotes_and_backslashes_losslessly() {
        let parameters = HashMap::from([(
            "argv".to_owned(),
            r#"["first arg" "C:\Program Files\device"]"#.to_owned(),
        )]);
        let formatted = format_params_string(&parameters);

        assert_eq!(
            formatted,
            r#"argv="[\"first arg\" \"C:\\Program Files\\device\"]""#
        );
        assert_eq!(parse_params_string(&formatted), parameters);
    }

    #[test]
    fn parameter_values_round_trip_all_quoting_and_separator_forms() {
        for value in [
            "a  b",
            "\ta\nb\r\nc",
            "O'Brien",
            "a\"b",
            "µΩ λ",
            r"C:\Program Files\device",
            r"C:\temp\new",
            "{a + f(b, c)}",
            "[1, 2, (3 + 4)]",
            "{",
            ")",
            "a=b",
            "x,y",
            "'",
            "\"",
        ] {
            let expected = HashMap::from([("note".to_owned(), value.to_owned())]);
            assert_eq!(
                parse_params_string(&format_params_string(&expected)),
                expected,
                "{value:?}"
            );
        }
    }

    #[test]
    fn parameter_lookup_respects_flags_spacing_and_balanced_values() {
        let parsed = parse_params_string(
            "off W = 2u, expr={a + nested({b, c})}, points=[0 1 (2 + 3)], gain=V(a,b)\r\nTEMP=27 temp=85",
        );
        assert_eq!(parsed["w"], "2u");
        assert_eq!(parsed["expr"], "{a + nested({b, c})}");
        assert_eq!(parsed["points"], "[0 1 (2 + 3)]");
        assert_eq!(parsed["gain"], "V(a,b)");
        assert_eq!(parsed["temp"], "85");
        assert_eq!(parsed["off"], "1");
        assert!(!parsed.contains_key("offw"));
    }

    #[test]
    fn quoted_paths_preserve_literal_backslashes_and_only_decode_quoted_escapes() {
        let parsed =
            parse_params_string(r#"file="C:\temp\new data" label='a\'b' expr={V("a,b") + 1}"#);
        assert_eq!(parsed["file"], r"C:\temp\new data");
        assert_eq!(parsed["label"], "a'b");
        assert_eq!(parsed["expr"], r#"{V("a,b") + 1}"#);
    }

    #[test]
    fn single_field_writes_preserve_entries_and_replace_all_case_variants_once() {
        assert_eq!(
            set_parameter_value("w=2u l=180n", "l", "220n").unwrap(),
            "w=2u l=220n"
        );
        assert_eq!(
            set_parameter_value("w=2u l=180n", "m", "4").unwrap(),
            "w=2u l=180n m=4"
        );
        assert_eq!(set_parameter_value("", "temp", "85").unwrap(), "temp=85");
        assert_eq!(
            set_parameter_value("TEMP=85", "temp", "27").unwrap(),
            "temp=27"
        );
        assert_eq!(set_parameter_value("W=2u w=3u", "w", "4u").unwrap(), "w=4u");
        let text = r#"off label="a  b" W = 2u, expr={a + f(b, c)} w=3u path='C:\a b'"#;
        assert_eq!(
            set_parameter_value(text, "w", "4u").unwrap(),
            r#"off label="a  b" w=4u expr={a + f(b, c)} path='C:\a b'"#
        );
    }

    #[test]
    fn clearing_one_field_removes_every_override_and_retains_flags_and_quoted_neighbors() {
        assert_eq!(
            set_parameter_value("w=2u temp=85 l=180n", "temp", "").unwrap(),
            "w=2u l=180n"
        );
        assert_eq!(set_parameter_value("temp=85", "temp", "   ").unwrap(), "");
        assert_eq!(
            set_parameter_value("off w=2u", "w", "3u").unwrap(),
            "off w=3u"
        );
        assert_eq!(
            set_parameter_value(r#"off W=2u note="a  b" w=3u"#, "w", "").unwrap(),
            r#"off note="a  b""#
        );
    }

    #[test]
    fn field_text_is_encoded_as_one_value_without_injecting_another_parameter() {
        for value in [
            r#"a" w=0 note="b"#,
            " leading and trailing ",
            r#"["a b" "C:\my data"]"#,
            "a\tb\nc",
        ] {
            let updated = set_parameter_value("w=2u note=old", "note", value).unwrap();
            assert_eq!(
                parse_params_string(&updated),
                HashMap::from([
                    ("w".to_owned(), "2u".to_owned()),
                    ("note".to_owned(), value.to_owned()),
                ])
            );
            validate_parameter_text(&updated).unwrap();
        }
    }

    #[test]
    fn malformed_text_cannot_be_reconstructed_into_a_partial_edit() {
        for text in [
            "good=1 note='",
            r#"good=1 note="a\""#,
            "good=1 expr={a + b",
            "good=1 list=[1 2)",
            "good=1 note=",
            "good=1 note=, next=2",
            "good=1 note='a'b",
            "good=1 =2",
        ] {
            assert!(validate_parameter_text(text).is_err(), "{text}");
            assert!(set_parameter_value(text, "good", "3").is_err(), "{text}");
            let parsed = parse_params_string(text);
            assert_eq!(parsed, HashMap::from([("good".to_owned(), "1".to_owned())]));
        }
        assert!(validate_parameter_text("w=1 W=2").is_err());
        assert!(validate_parameter_text("w=1 bad-key=2").is_err());
        assert!(set_parameter_value("w=1", "w other", "2").is_err());
        validate_parameter_text("off note='' expr={a + b}").unwrap();
    }

    #[test]
    fn comparison_operators_remain_part_of_the_expression_value() {
        assert_eq!(parse_params_string("expr=a==b w=2u")["expr"], "a==b");
    }

    #[test]
    fn arithmetic_after_parentheses_remains_part_of_the_expression_value() {
        assert_eq!(parse_params_string("expr=(a+b)*2 w=2u")["expr"], "(a+b)*2");
    }
}
