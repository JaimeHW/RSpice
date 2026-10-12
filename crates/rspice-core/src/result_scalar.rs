//! Typed per-analysis scalar vocabulary.
//!
//! A scalar is data, like the [`SignalUnit`] it carries: the analysis result
//! that determines one (a stability margin, a DC mismatch sigma), the
//! control-script host that prints one and the result document that publishes
//! one all hold the same value, so none of them owns the type.
//! `execution::result_document` re-exports every public name here under its
//! existing path.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::Value;
use crate::signal_unit::SignalUnit;

/// One complex sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ComplexSample {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexSample {
    /// Build a sample from a core complex value.
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl From<num_complex::Complex64> for ComplexSample {
    fn from(value: num_complex::Complex64) -> Self {
        Self {
            real: value.re,
            imaginary: value.im,
        }
    }
}

impl From<ComplexSample> for num_complex::Complex64 {
    fn from(sample: ComplexSample) -> Self {
        Self::new(sample.real, sample.imaginary)
    }
}

/// One typed per-analysis scalar, such as a stability margin or a residual.
///
/// [`Self::new`] is the only public constructor and stores the name trimmed
/// and lower-cased. The fields are visible to the crate so the result
/// document can validate, look up and migrate its scalars; a writer inside
/// the crate keeps that canonical form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultScalar {
    pub(crate) name: String,
    pub(crate) display_name: String,
    #[serde(with = "crate::signal_unit::wire::optional")]
    pub(crate) unit: Option<SignalUnit>,
    pub(crate) value: ScalarValue,
}

impl ResultScalar {
    /// Build one scalar, rejecting empty names and non-finite values.
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        unit: Option<SignalUnit>,
        value: ScalarValue,
    ) -> Result<Self, ResultScalarError> {
        let name = name.into();
        let display_name = display_name.into();
        require_name("scalar name", &name)?;
        require_name("scalar display name", &display_name)?;
        let scalar = Self {
            name: name.trim().to_ascii_lowercase(),
            display_name: display_name.trim().to_owned(),
            unit,
            value,
        };
        scalar.validate()?;
        Ok(scalar)
    }

    /// Canonical scalar name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Display spelling of the scalar name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Unit, or `None` when the producing result declares none.
    pub const fn unit(&self) -> Option<&SignalUnit> {
        self.unit.as_ref()
    }

    /// The value itself.
    pub const fn value(&self) -> &ScalarValue {
        &self.value
    }

    pub(crate) fn validate(&self) -> Result<(), ResultScalarError> {
        require_name("scalar name", &self.name)?;
        require_name("scalar display name", &self.display_name)?;
        match &self.value {
            ScalarValue::Real { value } => finite_optional("scalar", *value),
            ScalarValue::Complex { value } => {
                if let Some(sample) = value {
                    finite("scalar real part", sample.real)?;
                    finite("scalar imaginary part", sample.imaginary)?;
                }
                Ok(())
            }
            ScalarValue::Integer { .. }
            | ScalarValue::Count { .. }
            | ScalarValue::Boolean { .. }
            | ScalarValue::Unavailable { .. } => Ok(()),
            ScalarValue::Text { value } => require_name("scalar text value", value),
        }
    }
}

/// A typed scalar value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum ScalarValue {
    Real {
        value: Option<f64>,
    },
    Complex {
        value: Option<ComplexSample>,
    },
    Integer {
        value: i64,
    },
    Count {
        value: u64,
    },
    Boolean {
        value: bool,
    },
    Text {
        value: String,
    },
    /// A real quantity the analysis proved has no finite value, with the
    /// reason. A finite `ResultScalar` cannot hold `±inf`, and rounding one
    /// down to a large number would report a margin the loop does not have.
    Unavailable {
        reason: ScalarUnavailability,
    },
}

/// Why a real scalar has no finite value.
///
/// This is stronger than `Real { value: None }`, which says only that the
/// producing analysis did not compute the quantity. Each variant here is a
/// determination: the analysis ran, and the answer is not a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ScalarUnavailability {
    /// The quantity diverges upward, such as ideal-source input impedance.
    PositiveInfinity,
    /// The quantity diverges downward, such as the logarithm of measured zero gain.
    NegativeInfinity,
    /// The quantity is only defined at a crossover not resolved in the sampled band,
    /// so it has no value at all rather than an infinite one.
    NoCrossover,
    /// The quantity aggregates over a set — a maximum, a minimum, an extremum
    /// across a sweep or a spectrum — and that set is empty, so there is
    /// nothing to aggregate rather than a crossing that failed to happen.
    ///
    /// Distinct from [`NoCrossover`](Self::NoCrossover) because the two say
    /// different things to the reader. A missing crossover is a statement
    /// about the shape of a response that was measured: it went somewhere,
    /// just never through the level asked about. An empty domain says the
    /// measurement had no applicable members at all — an autonomous Floquet
    /// spectrum holding only its own phase mode, or a transfer with no finite
    /// magnitude anywhere to take a peak over. Reporting the second as the
    /// first sends the reader looking for a crossing in a set that has no
    /// elements.
    EmptyDomain,
}

impl ScalarUnavailability {
    /// Classify one non-finite real value, or `None` when it is finite.
    ///
    /// `NaN` is deliberately not classified: it is a defect in the producing
    /// computation, not a determination about the circuit, and the projection
    /// must keep rejecting it.
    pub(crate) const fn classify(value: f64) -> Option<Self> {
        if value == f64::INFINITY {
            Some(Self::PositiveInfinity)
        } else if value == f64::NEG_INFINITY {
            Some(Self::NegativeInfinity)
        } else {
            None
        }
    }

    /// Stable tag used by diagnostics and frontends.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::PositiveInfinity => "positive_infinity",
            Self::NegativeInfinity => "negative_infinity",
            Self::NoCrossover => "no_crossover",
            Self::EmptyDomain => "empty_domain",
        }
    }
}

/// Why a scalar could not be built or projected.
///
/// Each variant is the same finding as the result-document error of the same
/// name, which it converts into with its message unchanged.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ResultScalarError {
    /// A required name was empty.
    EmptyName { location: &'static str },
    /// A value that must be finite was not.
    NonFinite { location: String, value: f64 },
    /// A source result could not be projected into a scalar.
    SourceResult {
        location: &'static str,
        detail: String,
    },
}

impl fmt::Display for ResultScalarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName { location } => write!(formatter, "{location} is empty"),
            Self::NonFinite { location, value } => {
                write!(formatter, "{location} has non-finite value {value}")
            }
            Self::SourceResult { location, detail } => {
                write!(formatter, "{location} cannot be projected: {detail}")
            }
        }
    }
}

impl std::error::Error for ResultScalarError {}

fn require_name(location: &'static str, name: &str) -> Result<(), ResultScalarError> {
    if name.trim().is_empty() {
        Err(ResultScalarError::EmptyName { location })
    } else {
        Ok(())
    }
}

fn finite(location: &'static str, value: f64) -> Result<(), ResultScalarError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ResultScalarError::NonFinite {
            location: location.to_owned(),
            value,
        })
    }
}

fn finite_optional(location: &'static str, value: Option<f64>) -> Result<(), ResultScalarError> {
    match value {
        Some(value) => finite(location, value),
        None => Ok(()),
    }
}

fn finite_scalar(
    location: &'static str,
    name: &str,
    value: Value,
) -> Result<ScalarValue, ResultScalarError> {
    if value.is_finite() {
        Ok(ScalarValue::Real { value: Some(value) })
    } else {
        Err(ResultScalarError::SourceResult {
            location,
            detail: format!("scalar '{name}' is {value}, which JSON cannot represent"),
        })
    }
}

pub(crate) fn real_scalar(
    location: &'static str,
    name: &str,
    display: &str,
    unit: SignalUnit,
    value: Value,
) -> Result<ResultScalar, ResultScalarError> {
    ResultScalar::new(
        name,
        display,
        Some(unit),
        finite_scalar(location, name, value)?,
    )
}

/// One real scalar whose analysis may legitimately produce no finite value.
///
/// An ideal voltage source can have unbounded input impedance.
/// Those are determinations, so they are recorded as such. `NaN` stays a hard
/// projection failure: it is a defect in the producing computation, not a
/// statement about the circuit.
pub(crate) fn real_or_unbounded_scalar(
    location: &'static str,
    name: &str,
    display: &str,
    unit: SignalUnit,
    value: Value,
) -> Result<ResultScalar, ResultScalarError> {
    match ScalarUnavailability::classify(value) {
        Some(reason) => ResultScalar::new(
            name,
            display,
            Some(unit),
            ScalarValue::Unavailable { reason },
        ),
        None => real_scalar(location, name, display, unit, value),
    }
}

/// One derived curve metric, which exists only where the curve has the feature
/// it measures — and the two ways it can be missing are different findings.
///
/// A transfer that never falls 3 dB below its peak has no -3 dB bandwidth, and
/// one that never reaches unity has no 0 dB crossing: both are crossings a
/// measured response did not make. A transfer with no finite magnitude
/// anywhere has nothing to take a peak *over*, which is an empty domain and
/// not a crossing at all. The caller states which of the two it found, because
/// only the caller knows: the `None` arriving here is the same `None` either
/// way. Both are determinations about the response, so both are recorded as
/// such rather than reported as a zero the sweep would read as DC.
pub(crate) fn derived_metric_scalar(
    location: &'static str,
    name: &str,
    display: &str,
    unit: SignalUnit,
    absent: ScalarUnavailability,
    value: Option<Value>,
) -> Result<ResultScalar, ResultScalarError> {
    match value {
        Some(value) => real_or_unbounded_scalar(location, name, display, unit, value),
        None => ResultScalar::new(
            name,
            display,
            Some(unit),
            ScalarValue::Unavailable { reason: absent },
        ),
    }
}

pub(crate) fn count_scalar(
    name: &str,
    display: &str,
    value: usize,
) -> Result<ResultScalar, ResultScalarError> {
    ResultScalar::new(
        name,
        display,
        Some(SignalUnit::Dimensionless),
        ScalarValue::Count {
            value: value as u64,
        },
    )
}

pub(crate) fn boolean_scalar(
    name: &str,
    display: &str,
    value: bool,
) -> Result<ResultScalar, ResultScalarError> {
    ResultScalar::new(name, display, None, ScalarValue::Boolean { value })
}
