use std::{error::Error, fmt};

const MAX_BEARER_TOKEN_BYTES: usize = 16 * 1024;

/// A validated, borrowed OAuth 2.0 bearer token.
///
/// The client never stores this value. Construct it immediately before a
/// request from a platform credential provider or secure token store.
#[derive(Clone, Copy)]
pub struct BearerToken<'a>(&'a str);

impl<'a> BearerToken<'a> {
    /// Validates an RFC 6750 `b64token` without copying or retaining it.
    pub fn new(value: &'a str) -> Result<Self, CredentialError> {
        if value.is_empty() {
            return Err(CredentialError::Empty);
        }
        if value.len() > MAX_BEARER_TOKEN_BYTES {
            return Err(CredentialError::TooLong);
        }

        let mut saw_payload = false;
        let mut saw_padding = false;
        for byte in value.bytes() {
            if byte == b'=' {
                if !saw_payload {
                    return Err(CredentialError::InvalidPadding);
                }
                saw_padding = true;
                continue;
            }
            if saw_padding {
                return Err(CredentialError::InvalidPadding);
            }
            if !(byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'+' | b'/'))
            {
                return Err(CredentialError::InvalidCharacter);
            }
            saw_payload = true;
        }

        Ok(Self(value))
    }

    pub(crate) fn secret(self) -> &'a str {
        self.0
    }
}

impl fmt::Debug for BearerToken<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BearerToken([REDACTED])")
    }
}

/// Reason a bearer token was rejected before any network request was made.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialError {
    /// The token was empty.
    Empty,
    /// The token exceeded the defensive 16 KiB bound.
    TooLong,
    /// The token contained a character outside the RFC 6750 bearer grammar.
    InvalidCharacter,
    /// Base64 padding appeared before the end of the token or without payload.
    InvalidPadding,
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "bearer token must not be empty",
            Self::TooLong => "bearer token exceeds the supported size",
            Self::InvalidCharacter => "bearer token contains an invalid character",
            Self::InvalidPadding => "bearer token has invalid padding",
        })
    }
}

impl Error for CredentialError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_bearer_grammar_without_exposing_debug() {
        let secret = "header.payload_signature-~/+==";
        let token = BearerToken::new(secret).expect("valid bearer token");
        assert_eq!(token.secret(), secret);
        let debug = format!("{token:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(secret));

        assert_eq!(
            BearerToken::new("").expect_err("empty token"),
            CredentialError::Empty
        );
        assert_eq!(
            BearerToken::new("token with spaces").expect_err("space is invalid"),
            CredentialError::InvalidCharacter
        );
        assert_eq!(
            BearerToken::new("token\r\nheader").expect_err("newline is invalid"),
            CredentialError::InvalidCharacter
        );
        assert_eq!(
            BearerToken::new("token=tail").expect_err("interior padding is invalid"),
            CredentialError::InvalidPadding
        );
        assert_eq!(
            BearerToken::new(&"x".repeat(MAX_BEARER_TOKEN_BYTES + 1)).expect_err("oversized token"),
            CredentialError::TooLong
        );
    }
}
