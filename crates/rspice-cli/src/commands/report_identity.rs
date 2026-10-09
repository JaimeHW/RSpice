//! Unambiguous field identities shared by report exports and comparisons.

/// An unambiguous tuple component; ordinary SPICE identifiers stay readable.
pub(super) fn encode_part(value: &str) -> String {
    use std::fmt::Write;
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.') {
            encoded.push(char::from(byte));
        } else {
            // Writing into a String cannot fail.
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

/// Decode a case-insensitive identity field, including escaped ASCII letters.
pub(super) fn decode_folded_part(value: &str) -> Option<String> {
    let mut bytes = value.bytes();
    let mut decoded = Vec::with_capacity(value.len());
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            let high = char::from(bytes.next()?).to_digit(16)?;
            let low = char::from(bytes.next()?).to_digit(16)?;
            decoded.push((high * 16 + low) as u8);
        } else if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.') {
            decoded.push(byte);
        } else {
            return None;
        }
    }
    String::from_utf8(decoded)
        .ok()
        .map(|value| value.to_ascii_lowercase())
}
