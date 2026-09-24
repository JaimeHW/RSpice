//! Numeric shapes shared by real and complex expression evaluation.
//! A complex signal remains rectangular until an explicit display projection.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use super::evaluator::EvaluationError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumericValue<T> {
    Scalar(T),
    Waveform(Vec<f64>, Vec<T>),
}

pub type RealValue = NumericValue<f64>;
pub type ComplexValue = NumericValue<Complex64>;

impl<T> NumericValue<T> {
    pub fn create_waveform(x: Vec<f64>, y: Vec<T>) -> Self {
        debug_assert_eq!(x.len(), y.len());
        Self::Waveform(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcValue {
    Real(RealValue),
    Complex(ComplexValue),
}

impl From<RealValue> for CalcValue {
    fn from(value: RealValue) -> Self {
        Self::Real(value)
    }
}

impl From<ComplexValue> for CalcValue {
    fn from(value: ComplexValue) -> Self {
        Self::Complex(value)
    }
}

impl CalcValue {
    pub fn create_waveform(x: Vec<f64>, y: Vec<f64>) -> Self {
        RealValue::create_waveform(x, y).into()
    }

    pub fn into_complex(self) -> ComplexValue {
        match self {
            Self::Complex(value) => value,
            Self::Real(RealValue::Scalar(value)) => ComplexValue::Scalar(value.into()),
            Self::Real(RealValue::Waveform(x, y)) => {
                ComplexValue::Waveform(x, y.into_iter().map(Complex64::from).collect())
            }
        }
    }

    pub fn into_real(self) -> Result<RealValue, EvaluationError> {
        match self {
            Self::Real(value) => Ok(value),
            Self::Complex(_) => Err(EvaluationError::TypeMismatch(
                "complex values require an explicit real(), imag(), mag(), or phase() projection"
                    .to_owned(),
            )),
        }
    }

    /// Reject a non-finite scalar rather than presenting a successful reading.
    /// Series retain explicit holes at undefined samples.
    pub fn checked(self) -> Result<Self, EvaluationError> {
        match &self {
            Self::Real(RealValue::Scalar(value)) if !value.is_finite() => {
                Err(EvaluationError::MathError(
                    "scalar result is undefined or outside the finite range".to_owned(),
                ))
            }
            Self::Complex(ComplexValue::Scalar(value)) if !finite(*value) => {
                Err(EvaluationError::MathError(
                    "complex scalar result is undefined or outside the finite range".to_owned(),
                ))
            }
            Self::Real(RealValue::Waveform(x, y)) => {
                super::interpolation::validate_samples(x, y)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
                Ok(self)
            }
            Self::Complex(ComplexValue::Waveform(x, y)) => {
                super::interpolation::validate_samples(x, y)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
                Ok(self)
            }
            _ => Ok(self),
        }
    }
}

pub(super) fn finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

pub(super) fn hole(value: Complex64) -> Complex64 {
    if finite(value) {
        value
    } else {
        Complex64::new(f64::NAN, f64::NAN)
    }
}
