use super::*;
use std::sync::Arc;

pub type SharedWaveformValues = Arc<Vec<Value>>;

/// Original complex samples associated with a display trace.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexWaveformComponents {
    /// Source signal name before display transformations such as magnitude.
    pub source_name: String,

    /// Real component samples.
    pub real: SharedWaveformValues,

    /// Imaginary component samples.
    pub imag: SharedWaveformValues,
}

/// Waveform trace data
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
    /// Trace name (e.g., "V(out)")
    pub name: String,

    /// X-axis values (time or frequency)
    pub x: SharedWaveformValues,

    /// Y-axis values
    pub y: SharedWaveformValues,

    /// Trace color (hex string)
    pub color: String,

    /// Optional original complex samples for export and downstream analysis.
    pub complex: Option<ComplexWaveformComponents>,

    /// Whether this trace is visible
    pub visible: bool,
}

impl WaveformData {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: impl Into<SharedWaveformValues>,
        y: impl Into<SharedWaveformValues>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x: x.into(),
            y: y.into(),
            color: color.into(),
            complex: None,
            visible: true,
        }
    }

    /// Attach original complex samples to a derived display trace.
    pub fn with_complex_components(
        mut self,
        source_name: impl Into<String>,
        real: impl Into<SharedWaveformValues>,
        imag: impl Into<SharedWaveformValues>,
    ) -> Self {
        self.complex = Some(ComplexWaveformComponents {
            source_name: source_name.into(),
            real: real.into(),
            imag: imag.into(),
        });
        self
    }

    /// Get the X range (min, max)
    pub fn x_range(&self) -> (Value, Value) {
        let min = self.x.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.x.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }

    /// Get the Y range (min, max)
    pub fn y_range(&self) -> (Value, Value) {
        let min = self.y.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.y.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }
}
