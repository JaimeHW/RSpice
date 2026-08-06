//! Canonical JSON exactly as the cloud contract tools compute it: object
//! keys sorted, compact separators, and — because signed corpus bytes must
//! never depend on a serializer's number formatting — no floating-point
//! numbers at all. Every fractional quantity in the corpus travels as a
//! canonical decimal string.

use serde_json::Value;
use sha2::{Digest, Sha256};

/// Serializes a value canonically. `None` if any number is not an integer:
/// admission's `canonicalJson` delegates number rendering to the JavaScript
/// runtime, and integers are the only values both runtimes render
/// identically by construction.
pub fn canonical_json(value: &Value) -> Option<String> {
    let mut output = String::new();
    write_canonical(value, &mut output)?;
    Some(output)
}

fn write_canonical(value: &Value, output: &mut String) -> Option<()> {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(true) => output.push_str("true"),
        Value::Bool(false) => output.push_str("false"),
        Value::Number(number) => {
            // Safe integers render identically in serde_json and
            // JSON.stringify; floats do not reliably, and integers beyond
            // 2^53-1 lose precision the moment a JavaScript admission tool
            // parses them. Both are refused outright.
            const MAX_SAFE_INTEGER: i64 = (1 << 53) - 1;
            if number
                .as_i64()
                .is_none_or(|value| value.abs() > MAX_SAFE_INTEGER)
            {
                return None;
            }
            output.push_str(&number.to_string());
        }
        Value::String(text) => {
            output.push_str(&serde_json::to_string(text).ok()?);
        }
        Value::Array(entries) => {
            output.push('[');
            for (index, entry) in entries.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                write_canonical(entry, output)?;
            }
            output.push(']');
        }
        Value::Object(members) => {
            // Sorted explicitly rather than trusting map order, so the
            // output is canonical even if a dependency switches serde_json
            // to insertion-ordered maps.
            let mut keys: Vec<&String> = members.keys().collect();
            keys.sort_unstable();
            output.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(key).ok()?);
                output.push(':');
                write_canonical(&members[key], output)?;
            }
            output.push('}');
        }
    }
    Some(())
}

/// Lowercase hex SHA-256 of the canonical serialization.
pub fn canonical_sha256(value: &Value) -> Option<String> {
    let digest: [u8; 32] = Sha256::digest(canonical_json(value)?.as_bytes()).into();
    Some(rspice_engine_adapter::wire::digest_hex(&digest))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn keys_sort_and_output_is_compact() {
        let value = json!({"b": [1, {"z": "x", "a": null}], "a": true});
        assert_eq!(
            canonical_json(&value).expect("canonical form"),
            r#"{"a":true,"b":[1,{"a":null,"z":"x"}]}"#
        );
    }

    #[test]
    fn string_escaping_matches_json_stringify() {
        // JSON.stringify escapes the mandatory set with short escapes,
        // other control characters as lowercase \u00xx, and nothing else.
        let value = json!({"k": "a\"b\\c\nd\te\u{1f}f\u{e9}"});
        assert_eq!(
            canonical_json(&value).expect("canonical form"),
            "{\"k\":\"a\\\"b\\\\c\\nd\\te\\u001ff\u{e9}\"}"
        );
    }

    #[test]
    fn unsafe_numbers_are_refused() {
        assert!(canonical_json(&json!({"x": 1.5})).is_none());
        assert!(canonical_json(&json!([0.1])).is_none());
        assert!(canonical_json(&json!({"n": 250, "big": 9_007_199_254_740_991_u64})).is_some());
        // Beyond Number.MAX_SAFE_INTEGER a JavaScript reader corrupts the
        // value silently, so the canonical form refuses to exist.
        assert!(canonical_json(&json!({"big": 9_007_199_254_740_992_u64})).is_none());
        assert!(canonical_json(&json!({"big": -9_007_199_254_740_992_i64})).is_none());
    }

    #[test]
    fn digests_are_stable_and_lowercase() {
        let digest = canonical_sha256(&json!({"format_version": 1})).expect("digest");
        assert_eq!(
            digest,
            {
                use sha2::{Digest, Sha256};
                let raw: [u8; 32] = Sha256::digest(b"{\"format_version\":1}").into();
                rspice_engine_adapter::wire::digest_hex(&raw)
            },
            "digest must be the SHA-256 of the canonical bytes"
        );
    }
}
