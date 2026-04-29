// Measurement Types
// =============================================================================

/// Waveform measurement result
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    /// Measurement name
    pub name: &'static str,
    /// Measurement value
    pub value: f64,
    /// Unit string
    pub unit: &'static str,
}

impl Measurement {
    /// Create a new measurement
    pub fn new(name: &'static str, value: f64, unit: &'static str) -> Self {
        Self { name, value, unit }
    }
}

/// Complete measurement set for a trace
#[derive(Debug, Clone, Default)]
pub struct TraceMeasurements {
    /// Trace name
    pub trace_name: String,
    /// Minimum value
    pub min: Option<f64>,
    /// Maximum value
    pub max: Option<f64>,
    /// Peak-to-peak
    pub pk_pk: Option<f64>,
    /// Mean (average)
    pub mean: Option<f64>,
    /// RMS value
    pub rms: Option<f64>,
    /// Standard deviation
    pub std_dev: Option<f64>,
    /// Rise time (10%-90%)
    pub rise_time: Option<f64>,
    /// Fall time (90%-10%)
    pub fall_time: Option<f64>,
    /// Period (for periodic signals)
    pub period: Option<f64>,
    /// Frequency (1/period)
    pub frequency: Option<f64>,
    /// Duty cycle (%) for digital-like signals
    pub duty_cycle: Option<f64>,
    /// Integral (area under curve)
    pub integral: Option<f64>,
}
