use std::{error::Error, fmt};

const MIN_KEY_BYTES: usize = 16;
const MAX_KEY_BYTES: usize = 200;

/// A validated, borrowed idempotency key for a retry-safe mutation.
///
/// Generate a high-entropy value once for each logical mutation, persist it
/// with the pending command, and reuse it only when retrying that command.
#[derive(Clone, Copy)]
pub struct IdempotencyKey<'a>(&'a str);

impl<'a> IdempotencyKey<'a> {
    /// Validates the RSpice Cloud `Idempotency-Key` wire grammar.
    pub fn new(value: &'a str) -> Result<Self, IdempotencyKeyError> {
        if value.len() < MIN_KEY_BYTES {
            return Err(IdempotencyKeyError::TooShort);
        }
        if value.len() > MAX_KEY_BYTES {
            return Err(IdempotencyKeyError::TooLong);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
        {
            return Err(IdempotencyKeyError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub(crate) const fn value(self) -> &'a str {
        self.0
    }
}

impl fmt::Debug for IdempotencyKey<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("IdempotencyKey([REDACTED])")
    }
}

/// Reason an idempotency key was rejected before a network request was made.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdempotencyKeyError {
    /// The key was shorter than the 16-byte minimum.
    TooShort,
    /// The key exceeded the 200-byte defensive bound.
    TooLong,
    /// The key contained a character outside the declared wire grammar.
    InvalidCharacter,
}

impl fmt::Display for IdempotencyKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TooShort => "idempotency key must contain at least 16 bytes",
            Self::TooLong => "idempotency key exceeds the supported size",
            Self::InvalidCharacter => "idempotency key contains an invalid character",
        })
    }
}

impl Error for IdempotencyKeyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_exact_wire_grammar_without_exposing_debug() {
        let raw = "019f76ae-0000-7000-8000-000000000001";
        let key = IdempotencyKey::new(raw).expect("valid idempotency key");
        assert_eq!(key.value(), raw);
        let debug = format!("{key:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(raw));

        assert_eq!(
            IdempotencyKey::new("short").expect_err("short key"),
            IdempotencyKeyError::TooShort
        );
        assert_eq!(
            IdempotencyKey::new(&"x".repeat(MAX_KEY_BYTES + 1)).expect_err("oversized key"),
            IdempotencyKeyError::TooLong
        );
        assert_eq!(
            IdempotencyKey::new("contains spaces 123").expect_err("invalid character"),
            IdempotencyKeyError::InvalidCharacter
        );
    }
}
