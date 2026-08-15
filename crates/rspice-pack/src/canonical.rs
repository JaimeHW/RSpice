use serde_json::Value;

use crate::error::PackError;

/// Encodes a JSON value as RFC 8785 (JCS) bytes, restricted to the value
/// domain the pack schemas use.
///
/// The restriction is what makes the encoding provable rather than assumed:
/// object keys must be ASCII, so ordering by UTF-8 bytes is identical to
/// JCS's UTF-16 code-unit ordering; numbers must be integers, so the
/// ECMAScript double formatting JCS mandates degenerates to plain digits.
/// Keys are sorted explicitly here rather than inherited from
/// `serde_json::Map`'s ordering, which a `preserve_order` feature elsewhere in
/// a workspace could silently change. String escaping is delegated to
/// `serde_json`, whose escape set (short forms for the five named controls,
/// lowercase `\u00xx` for the rest, no escaping of non-ASCII) is exactly JCS's.
pub(crate) fn canonical_bytes(value: &Value) -> Result<Vec<u8>, PackError> {
    let mut encoded = Vec::new();
    let mut location = String::from("$");
    write_value(value, &mut location, &mut encoded)?;
    Ok(encoded)
}

/// Recursion is bounded by `serde_json`'s own 128-level parse limit, so a
/// hostile document cannot drive this past a shallow, fixed depth.
fn write_value(value: &Value, location: &mut String, out: &mut Vec<u8>) -> Result<(), PackError> {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(flag) => out.extend_from_slice(if *flag { b"true" } else { b"false" }),
        Value::Number(number) => {
            let integer = if let Some(unsigned) = number.as_u64() {
                unsigned.to_string()
            } else if let Some(signed) = number.as_i64() {
                signed.to_string()
            } else {
                return Err(PackError::FloatInManifest(location.clone()));
            };
            out.extend_from_slice(integer.as_bytes());
        }
        Value::String(text) => write_string(text, out)?,
        Value::Array(items) => {
            out.push(b'[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(b',');
                }
                let mark = location.len();
                location.push('[');
                location.push_str(&index.to_string());
                location.push(']');
                write_value(item, location, out)?;
                location.truncate(mark);
            }
            out.push(b']');
        }
        Value::Object(object) => {
            let mut members: Vec<(&String, &Value)> = object.iter().collect();
            for (key, _) in &members {
                if !key.is_ascii() {
                    return Err(PackError::NonAsciiKey((*key).clone()));
                }
            }
            members.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            out.push(b'{');
            for (index, (key, member)) in members.iter().enumerate() {
                if index > 0 {
                    out.push(b',');
                }
                write_string(key, out)?;
                out.push(b':');
                let mark = location.len();
                location.push('.');
                location.push_str(key);
                write_value(member, location, out)?;
                location.truncate(mark);
            }
            out.push(b'}');
        }
    }
    Ok(())
}

fn write_string(text: &str, out: &mut Vec<u8>) -> Result<(), PackError> {
    serde_json::to_writer(&mut *out, text).map_err(|error| PackError::Encoding(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(text: &str) -> String {
        let value: Value = serde_json::from_str(text).expect("test fixture parses");
        let bytes = canonical_bytes(&value).expect("test fixture canonicalizes");
        String::from_utf8(bytes).expect("canonical output is UTF-8")
    }

    #[test]
    fn object_keys_are_sorted_and_whitespace_is_removed() {
        assert_eq!(
            encode("{ \"b\": 1, \"a\": { \"z\": [1, 2], \"A\": true } }"),
            "{\"a\":{\"A\":true,\"z\":[1,2]},\"b\":1}"
        );
    }

    #[test]
    fn escaping_matches_the_jcs_set() {
        assert_eq!(
            encode("{\"k\":\"a\\u0007b\\\"c\\\\d\\ne\\u00e9\"}"),
            "{\"k\":\"a\\u0007b\\\"c\\\\d\\ne\u{e9}\"}"
        );
    }

    #[test]
    fn floats_are_refused_with_their_location() {
        let value: Value =
            serde_json::from_str("{\"files\":[{\"length\":1.5}]}").expect("test fixture parses");
        match canonical_bytes(&value) {
            Err(PackError::FloatInManifest(location)) => {
                assert_eq!(location, "$.files[0].length");
            }
            other => panic!("expected a float rejection, got {other:?}"),
        }
    }

    #[test]
    fn exponent_and_oversized_integer_literals_are_floats() {
        for literal in ["{\"a\":1e2}", "{\"a\":99999999999999999999}"] {
            let value: Value = serde_json::from_str(literal).expect("test fixture parses");
            assert!(matches!(
                canonical_bytes(&value),
                Err(PackError::FloatInManifest(_))
            ));
        }
    }

    #[test]
    fn non_ascii_keys_leave_the_canonical_domain() {
        let value: Value = serde_json::from_str("{\"\u{e9}\":1}").expect("test fixture parses");
        assert!(matches!(
            canonical_bytes(&value),
            Err(PackError::NonAsciiKey(_))
        ));
    }
}
