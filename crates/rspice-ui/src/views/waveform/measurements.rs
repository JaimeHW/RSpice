//! Waveform measurement types.
//!
//! Contains types for cursor values and waveform measurements like
//! peak-to-peak voltage, RMS, etc.

/// Cursor value at a specific time for a single trace.
#[derive(Debug, Clone, Default)]
pub struct CursorValue {
    /// Trace name
    pub name: String,
    /// Trace color for display
    pub color: String,
    /// Interpolated Y value at cursor X
    pub value: f64,
}

/// Cursor values for all traces at a cursor position.
#[derive(Debug, Clone, Default)]
pub struct CursorValues {
    /// X position (time)
    pub x: f64,
    /// Values per trace
    pub traces: Vec<CursorValue>,
}

/// Waveform measurements computed from data.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct WaveformMeasurements {
    /// Peak-to-peak voltage
    pub vpp: f64,
    /// Maximum voltage
    pub vmax: f64,
    /// Minimum voltage
    pub vmin: f64,
    /// Average voltage
    pub vavg: f64,
    /// RMS voltage
    pub vrms: f64,
}

impl WaveformMeasurements {
    /// Calculate measurements from Y data.
    #[allow(dead_code)]
    pub fn from_data(y: &[f64]) -> Self {
        if y.is_empty() {
            return Self::default();
        }

        let vmax = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let vmin = y.iter().cloned().fold(f64::INFINITY, f64::min);
        let vpp = vmax - vmin;
        let vavg = y.iter().sum::<f64>() / y.len() as f64;
        let vrms = (y.iter().map(|v| v * v).sum::<f64>() / y.len() as f64).sqrt();

        Self {
            vpp,
            vmax,
            vmin,
            vavg,
            vrms,
        }
    }
}
