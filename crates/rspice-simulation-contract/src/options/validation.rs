//! Option validation.
//!
//! Rejects a value the engine would refuse, naming the option and the bound
//! it violated rather than failing at run time.

use std::fmt;

/// Validation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTolerance(&'static str, f64),
    InvalidNonNegative(&'static str, f64),
    InvalidPivotRelative(f64),
    InvalidIteration(&'static str, usize),
    InvalidTimestep(&'static str, f64),
    TimestepOrder(f64, f64),
    InvalidTemperature(&'static str, f64),
    /// A connect-rule block name that is not a simple identifier.
    InvalidConnectRules(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidTolerance(name, val) => {
                write!(f, "{} must be finite and positive, got {}", name, val)
            }
            ValidationError::InvalidNonNegative(name, val) => {
                write!(f, "{} must be finite and non-negative, got {}", name, val)
            }
            ValidationError::InvalidPivotRelative(val) => {
                write!(f, "pivrel must be finite and in (0, 1], got {}", val)
            }
            ValidationError::InvalidIteration(name, val) => {
                write!(f, "{} must be > 0, got {}", name, val)
            }
            ValidationError::InvalidTimestep(name, val) => {
                write!(f, "{} must be finite and positive, got {}", name, val)
            }
            ValidationError::TimestepOrder(min, max) => {
                write!(f, "min_timestep ({}) must be < max_timestep ({})", min, max)
            }
            ValidationError::InvalidTemperature(name, val) => {
                write!(f, "{} must be finite and > -273.15C, got {}", name, val)
            }
            ValidationError::InvalidConnectRules(block) => write!(
                f,
                "connectrules must name a connect-rule block by a simple identifier, got '{block}'"
            ),
        }
    }
}

impl std::error::Error for ValidationError {}
