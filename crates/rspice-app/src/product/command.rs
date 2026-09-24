//! Command identity.
//!
//! A stable identifier for the commands the GUI dispatches, so a recorded
//! interaction — a shortcut binding, a replayed script — still names the same
//! command in a later build.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Stable command identity. Commands use lowercase alphanumeric segments
/// separated by a single hyphen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandId(String);

impl CommandId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IdentifierError::Empty);
        }
        let mut previous_separator = true;
        for byte in value.bytes() {
            let separator = byte == b'-';
            if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() && !separator {
                return Err(IdentifierError::Character(byte as char));
            }
            if separator && previous_separator {
                return Err(IdentifierError::Separator);
            }
            previous_separator = separator;
        }
        if previous_separator {
            return Err(IdentifierError::Separator);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommandId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for CommandId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for CommandId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CommandId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdentifierError {
    #[error("identifier must not be empty")]
    Empty,
    #[error("identifier contains unsupported character {0:?}")]
    Character(char),
    #[error("identifier separators must occur singly between alphanumeric segments")]
    Separator,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_ids_reject_ambiguous_or_unportable_forms() {
        for invalid in [
            "",
            "Create-Result",
            "-create",
            "create-",
            "create--result",
            "create result",
            "create.result",
        ] {
            assert!(
                CommandId::new(invalid).is_err(),
                "accepted invalid command ID {invalid}"
            );
        }
        assert_eq!(
            CommandId::new("create-result-document")
                .expect("valid command ID")
                .as_str(),
            "create-result-document"
        );
    }

    #[test]
    fn identifier_deserialization_is_validated() {
        assert!(serde_json::from_str::<CommandId>("\"bad command\"").is_err());
        assert!(serde_json::from_str::<CommandId>("\"create-result-document\"").is_ok());
    }
}
