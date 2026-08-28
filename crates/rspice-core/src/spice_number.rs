//! Shared SPICE numeric-token semantics.
//!
//! This leaf owns engineering suffixes so the deck lexer, expression parser,
//! and typed-input surfaces cannot drift into incompatible numeric dialects.

use crate::Value;

/// Resolve the SPICE engineering suffix at the head of `text`.
///
/// Returns the multiplier the suffix names together with how many characters
/// of `text` it consumed. Unknown text consumes nothing and scales by one, so
/// callers that require a complete token can reject an unconsumed tail.
pub fn spice_suffix_scale(text: &str) -> (Value, usize) {
    let chars = text.chars().collect::<Vec<_>>();
    parse_spice_suffix(&chars)
}

pub(crate) fn parse_spice_suffix(chars: &[char]) -> (Value, usize) {
    if chars.is_empty() {
        return (1.0, 0);
    }

    if chars.len() >= 3 {
        let three = chars[..3].iter().collect::<String>().to_ascii_uppercase();
        match three.as_str() {
            "MEG" => return (1e6, 3),
            "MIL" => return (25.4e-6, 3),
            "GHZ" => return (1e9, 3),
            "MHZ" => return (1e6, 3),
            "KHZ" => return (1e3, 3),
            "UHZ" => return (1e-6, 3),
            "NHZ" => return (1e-9, 3),
            "PHZ" => return (1e-12, 3),
            "FHZ" => return (1e-15, 3),
            "THZ" => return (1e12, 3),
            _ => {}
        }
    }

    if chars.len() >= 2 {
        let prefix = chars[0].to_ascii_uppercase();
        let unit = chars[1].to_ascii_uppercase();
        let scale = match unit {
            'S' | 'F' | 'H' => match prefix {
                'N' => Some(1e-9),
                'P' => Some(1e-12),
                'U' => Some(1e-6),
                'M' => Some(1e-3),
                'F' if unit == 'S' => Some(1e-15),
                _ => None,
            },
            'V' | 'A' | 'M' => match prefix {
                'T' => Some(1e12),
                'G' => Some(1e9),
                'K' => Some(1e3),
                'M' => Some(1e-3),
                'U' => Some(1e-6),
                'N' => Some(1e-9),
                'P' => Some(1e-12),
                'F' => Some(1e-15),
                _ => None,
            },
            _ => None,
        };
        if let Some(scale) = scale {
            return (scale, 2);
        }
    }

    match chars[0].to_ascii_uppercase() {
        'T' => (1e12, 1),
        'G' => (1e9, 1),
        'K' => (1e3, 1),
        'M' => (1e-3, 1),
        'U' => (1e-6, 1),
        'N' => (1e-9, 1),
        'P' => (1e-12, 1),
        'F' => (1e-15, 1),
        'X' if chars.len() == 1 || !chars[1].is_ascii_alphabetic() => (1e6, 1),
        'V' | 'A' | 'S' => (1.0, 1),
        _ => (1.0, 0),
    }
}
