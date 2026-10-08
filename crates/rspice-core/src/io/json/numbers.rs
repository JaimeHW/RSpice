//! Borrow original number spellings alongside serde's authoritative parser.

pub use crate::io::decimal_underflowed;

/// JSON integer literals promise an exact value, unlike decimal floating-point
/// input which is rounded to binary64. serde retains i64/u64 values but already
/// rounds integers outside that range to f64, so check the original spelling.
pub fn integer_rounded(spelling: &str, value: f64) -> bool {
    // Include the boundary: 2^53 + 1 rounds down to 2^53. Smaller integers are
    // all exact and need no formatting/allocation. Above it, many integers are
    // still representable; a blanket 2^53 ceiling would reject valid samples.
    value.abs() >= (1_u64 << 53) as f64
        && !spelling.contains(['.', 'e', 'E'])
        && spelling != format!("{value:.0}")
}

/// Advances only when serde visits a number, without retaining tokens or
/// looking ahead through a container. JSON syntax remains serde's concern.
/// In particular, a sample budget refusal must precede scanning its payload.
pub struct Numbers<'a> {
    remaining: &'a str,
}

impl<'a> Numbers<'a> {
    /// Borrow the source being decoded by serde, without scanning ahead.
    pub fn new(content: &'a str) -> Self {
        Self { remaining: content }
    }
}

impl<'a> Iterator for Numbers<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.remaining.as_bytes();
        let mut position = 0;
        while position < bytes.len() {
            match bytes[position] {
                b'"' => {
                    position += 1;
                    while position < bytes.len() {
                        match bytes[position] {
                            b'\\' => position += 2,
                            b'"' => {
                                position += 1;
                                break;
                            }
                            _ => position += 1,
                        }
                    }
                }
                b'-' | b'0'..=b'9' => {
                    let start = position;
                    position += 1;
                    while position < bytes.len()
                        && matches!(
                            bytes[position],
                            b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-'
                        )
                    {
                        position += 1;
                    }
                    let number = &self.remaining[start..position];
                    self.remaining = &self.remaining[position..];
                    return Some(number);
                }
                _ => position += 1,
            }
        }
        self.remaining = "";
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_integer_literals_include_large_powers_and_exclude_rounded_neighbors() {
        for exponent in 0..=1023 {
            let value = 2.0_f64.powi(exponent);
            for value in [value, -value] {
                let spelling = format!("{value:.0}");
                let parsed = serde_json::from_str::<serde_json::Value>(&spelling)
                    .unwrap()
                    .as_f64()
                    .unwrap();
                assert_eq!(parsed.to_bits(), value.to_bits());
                assert!(!integer_rounded(&spelling, parsed), "{spelling}");
                if exponent >= 54 {
                    // Every power of two >= 2^54 ends in 2, 4, 6 or 8. Adding
                    // one to its magnitude changes only the final digit and
                    // must round back to the same binary64 power of two.
                    let mut neighbor = spelling.into_bytes();
                    *neighbor.last_mut().unwrap() += 1;
                    let neighbor = String::from_utf8(neighbor).unwrap();
                    let parsed = serde_json::from_str::<serde_json::Value>(&neighbor)
                        .unwrap()
                        .as_f64()
                        .unwrap();
                    assert_eq!(parsed.to_bits(), value.to_bits());
                    assert!(integer_rounded(&neighbor, parsed), "{neighbor}");
                }
            }
        }
        // Floating-point spellings keep ordinary correctly rounded semantics.
        for spelling in ["9007199254740993.0", "9007199254740993e0"] {
            assert!(!integer_rounded(spelling, 9007199254740992.0));
        }
    }

    #[test]
    fn number_spellings_skip_strings_keys_and_escaped_quotes() {
        let text = "µ𝄞 \"1e-999\" \\123 \n-0\t42";
        // Construct valid escaping with serde so digits in every escaped string
        // representation are exercised without depending on handwritten JSON.
        let content = format!(
            r#"{{"123":{},"\u0031":"\u0032","values":[-0,0.00e+20,42,-3,1e-999]}}"#,
            serde_json::to_string(text).unwrap()
        );
        assert!(serde_json::from_str::<serde_json::Value>(&content).is_ok());
        let mut numbers = Numbers::new(&content);
        for expected in ["-0", "0.00e+20", "42", "-3", "1e-999"] {
            assert_eq!(numbers.next(), Some(expected));
        }
        assert_eq!(numbers.next(), None);
    }
}
