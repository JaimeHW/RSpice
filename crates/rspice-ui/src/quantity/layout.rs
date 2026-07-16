use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

/// Authoritative physical length of one layout database unit.
///
/// The value is deliberately owned by the PDK configuration, never by a
/// display preference. No default exists because inventing a DBU can corrupt
/// layout coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(transparent)]
pub struct LayoutDatabaseUnit(f64);

impl LayoutDatabaseUnit {
    /// Construct a positive, finite database-unit length in metres.
    pub fn from_metres(metres: f64) -> Result<Self, LayoutDatabaseUnitError> {
        if !metres.is_finite() {
            return Err(LayoutDatabaseUnitError::NonFinite);
        }
        if metres <= 0.0 {
            return Err(LayoutDatabaseUnitError::NotPositive);
        }
        Ok(Self(metres))
    }

    #[must_use]
    pub const fn metres(self) -> f64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for LayoutDatabaseUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let metres = f64::deserialize(deserializer)?;
        Self::from_metres(metres).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDatabaseUnitError {
    NonFinite,
    NotPositive,
}

impl fmt::Display for LayoutDatabaseUnitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonFinite => "layout database unit must be finite",
            Self::NotPositive => "layout database unit must be greater than zero",
        })
    }
}

impl std::error::Error for LayoutDatabaseUnitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_unit_has_no_fabricated_default_and_rejects_invalid_lengths() {
        assert!(LayoutDatabaseUnit::from_metres(1.0e-9).is_ok());
        assert_eq!(
            LayoutDatabaseUnit::from_metres(0.0),
            Err(LayoutDatabaseUnitError::NotPositive)
        );
        assert_eq!(
            LayoutDatabaseUnit::from_metres(f64::NAN),
            Err(LayoutDatabaseUnitError::NonFinite)
        );
    }
}
