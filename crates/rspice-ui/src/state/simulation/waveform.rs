use super::*;
use std::sync::Arc;

pub type SharedWaveformValues = Arc<[Value]>;

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
            visible: true,
        }
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
