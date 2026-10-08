//! Borrow original number spellings alongside serde's authoritative parser.

/// Advances only when serde visits a number, without retaining tokens or
/// looking ahead through a container. JSON syntax remains serde's concern.
/// In particular, a sample budget refusal must precede scanning its payload.
pub(super) struct Numbers<'a> {
    remaining: &'a str,
}

impl<'a> Numbers<'a> {
    pub(super) fn new(content: &'a str) -> Self {
        Self { remaining: content }
    }

    pub(super) fn next(&mut self) -> Option<&'a str> {
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
