use std::fmt;

/// Validation error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTolerance(&'static str, f64),
    InvalidIteration(&'static str, usize),
    InvalidTimestep(&'static str, f64),
    TimestepOrder(f64, f64),
    InvalidTemperature(&'static str, f64),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidTolerance(name, val) => {
                write!(f, "{} must be positive, got {}", name, val)
            }
            ValidationError::InvalidIteration(name, val) => {
                write!(f, "{} must be > 0, got {}", name, val)
            }
            ValidationError::InvalidTimestep(name, val) => {
                write!(f, "{} must be positive, got {}", name, val)
            }
            ValidationError::TimestepOrder(min, max) => {
                write!(f, "min_timestep ({}) must be < max_timestep ({})", min, max)
            }
            ValidationError::InvalidTemperature(name, val) => {
                write!(f, "{} must be > -273.15C, got {}", name, val)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
