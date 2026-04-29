//=============================================================================
// Waveform Data
//=============================================================================

/// Time-domain waveform data for a single signal
#[derive(Debug, Clone)]
pub struct WaveformData {
    /// Signal name (e.g., "V(out)", "I(R1)")
    pub name: String,

    /// X-axis values (time for transient, frequency for AC)
    pub x_values: Vec<f64>,

    /// Y-axis values (voltage, current, etc.)
    pub y_values: Vec<f64>,

    /// Unit for Y-axis (V, A, dB, degrees, etc.)
    pub y_unit: String,

    /// Unit for X-axis (s, Hz, etc.)
    pub x_unit: String,

    /// Whether this is a complex result (for AC analysis)
    pub is_complex: bool,

    /// Imaginary part (for AC analysis)
    pub y_imag: Option<Vec<f64>>,
}

impl WaveformData {
    /// Create new time-domain waveform
    pub fn new_time_domain(name: impl Into<String>, time: Vec<f64>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            x_values: time,
            y_values: values,
            y_unit: "V".to_string(),
            x_unit: "s".to_string(),
            is_complex: false,
            y_imag: None,
        }
    }

    /// Create new frequency-domain waveform (magnitude only)
    pub fn new_freq_domain(name: impl Into<String>, freq: Vec<f64>, magnitude: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            x_values: freq,
            y_values: magnitude,
            y_unit: "dB".to_string(),
            x_unit: "Hz".to_string(),
            is_complex: false,
            y_imag: None,
        }
    }

    /// Create new complex frequency-domain waveform
    pub fn new_complex(
        name: impl Into<String>,
        freq: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: freq,
            y_values: real,
            y_unit: "".to_string(),
            x_unit: "Hz".to_string(),
            is_complex: true,
            y_imag: Some(imag),
        }
    }

    /// Get number of data points
    pub fn len(&self) -> usize {
        self.x_values.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.x_values.is_empty()
    }

    /// Get magnitude (for complex data)
    pub fn magnitude(&self) -> Vec<f64> {
        if let Some(ref imag) = self.y_imag {
            self.y_values
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| (r * r + i * i).sqrt())
                .collect()
        } else {
            self.y_values.clone()
        }
    }

    /// Get magnitude in dB (for complex data)
    pub fn magnitude_db(&self) -> Vec<f64> {
        self.magnitude().iter().map(|m| 20.0 * m.log10()).collect()
    }

    /// Get phase in degrees (for complex data)
    pub fn phase_deg(&self) -> Option<Vec<f64>> {
        self.y_imag.as_ref().map(|imag| {
            self.y_values
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| i.atan2(*r).to_degrees())
                .collect()
        })
    }

    /// Get X-axis range
    pub fn x_range(&self) -> Option<(f64, f64)> {
        if self.x_values.is_empty() {
            return None;
        }
        let min = self.x_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self
            .x_values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Get Y-axis range
    pub fn y_range(&self) -> Option<(f64, f64)> {
        if self.y_values.is_empty() {
            return None;
        }
        let min = self.y_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self
            .y_values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Set Y unit
    pub fn with_y_unit(mut self, unit: impl Into<String>) -> Self {
        self.y_unit = unit.into();
        self
    }
}
