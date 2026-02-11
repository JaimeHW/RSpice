use super::*;

/// Waveform trace data
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
    /// Trace name (e.g., "V(out)")
    pub name: String,

    /// X-axis values (time or frequency)
    pub x: Vec<Value>,

    /// Y-axis values
    pub y: Vec<Value>,

    /// Trace color (hex string)
    pub color: String,

    /// Whether this trace is visible
    pub visible: bool,
}

impl WaveformData {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: Vec<Value>,
        y: Vec<Value>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x,
            y,
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
