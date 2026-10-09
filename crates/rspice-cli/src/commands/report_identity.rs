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

/// Preserve case-sensitive metadata even in formats with case-insensitive names.
pub(super) fn encode_exact(value: &str) -> String {
    use std::fmt::Write;
    let mut encoded = String::with_capacity(value.len().saturating_mul(2));
    for byte in value.bytes() {
        let _ = write!(encoded, "{byte:02x}");
    }
    encoded
}

pub(super) fn decode_exact(value: &str) -> Option<String> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    let bytes = value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            Some((char::from(pair[0]).to_digit(16)? * 16 + char::from(pair[1]).to_digit(16)?) as u8)
        })
        .collect::<Option<Vec<_>>>()?;
    String::from_utf8(bytes).ok()
}
