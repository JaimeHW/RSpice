//! Exact conversion of integer sample values to the waveform `f64` representation.

pub const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;

/// An integer refused by the existing exact-sample conversion policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactIntegerError {
    Signed { identity: String, value: i64 },
    Unsigned { identity: String, value: u64 },
}

impl std::fmt::Display for ExactIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed { identity, value } => write!(
                f,
                "'{identity}' integer {value} cannot be represented exactly as f64"
            ),
            Self::Unsigned { identity, value } => write!(
                f,
                "'{identity}' integer {value} cannot be represented exactly as f64"
            ),
        }
    }
}

impl std::error::Error for ExactIntegerError {}

pub fn exact_signed_integer(identity: &str, value: i64) -> Result<f64, ExactIntegerError> {
    if value.unsigned_abs() > MAX_EXACT_F64_INTEGER {
        Err(ExactIntegerError::Signed {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(value as f64)
    }
}

pub fn exact_unsigned_integer(identity: &str, value: u64) -> Result<f64, ExactIntegerError> {
    if value > MAX_EXACT_F64_INTEGER {
        Err(ExactIntegerError::Unsigned {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(value as f64)
    }
}
