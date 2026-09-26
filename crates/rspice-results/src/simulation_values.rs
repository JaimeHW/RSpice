//! Solver-independent values retained in simulation results.

/// Electrical quantity represented by a transfer-function input or output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferFunctionQuantity {
    Voltage,
    Current,
}

/// JSON-safe scalar result. Transfer-function resistance can be infinite for
/// an open circuit, so infinity is represented explicitly rather than placed
/// in a floating-point field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl TransferFunctionScalar {
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        if value == f64::INFINITY {
            Self::PositiveInfinity
        } else if value == f64::NEG_INFINITY {
            Self::NegativeInfinity
        } else {
            Self::Finite(value)
        }
    }
}

/// One retained Floquet mode from a periodic stability analysis.
///
/// The vector containing these records is the complete authenticated spectrum,
/// sorted by decreasing multiplier magnitude. Presentation limits are applied
/// only to the separate waveform vectors in the PSTB result.
#[derive(Debug, Clone, PartialEq)]
pub struct PstbFloquetMode {
    /// Complex Floquet multiplier `(real, imaginary)`.
    pub multiplier: (f64, f64),
    /// Complex Floquet exponent `(real, imaginary)` in `1/s`.
    pub exponent: (f64, f64),
    /// Normalized participation of the configured PSTB probe in this mode.
    pub probe_participation: f64,
    /// Whether the mode lies outside the configured outer stability boundary.
    pub is_unstable: bool,
    /// Whether this is the explicitly authenticated autonomous phase mode.
    pub is_trivial: bool,
    /// Detected root-of-unity order, when subharmonic detection was enabled.
    pub subharmonic_order: Option<usize>,
}

/// One committed event on an XSPICE digital node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitalEventPoint {
    /// Event time in seconds.
    pub time_s: f64,
    /// XSPICE 12-state code, produced by `DigitalValue::event_code`.
    pub value_code: u8,
}

/// One committed event on an XSPICE real-valued node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealEventPoint {
    /// Event time in seconds.
    pub time_s: f64,
    pub value: f64,
}

/// The committed event history of one node.
#[derive(Debug, Clone, PartialEq)]
pub struct EventNodeHistory<P> {
    pub node_name: String,
    pub points: Vec<P>,
}

/// One exact complex value retained from an analysis result.
///
/// Preserves the solver's ordered real/imaginary evidence independently of
/// viewer state. The containing analysis payload owns root classification.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplexResultValue {
    pub real: f64,
    pub imaginary: f64,
}

impl From<ComplexResultValue> for num_complex::Complex64 {
    fn from(value: ComplexResultValue) -> Self {
        Self::new(value.real, value.imaginary)
    }
}
