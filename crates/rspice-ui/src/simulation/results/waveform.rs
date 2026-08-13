//! Sampled waveform data.

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

    /// Whether this is a complex result (for AC analysis)
    pub is_complex: bool,

    /// Imaginary part (for AC analysis)
    pub y_imag: Option<Vec<f64>>,
}

impl WaveformData {
    /// Create a new time-domain node-voltage waveform.
    pub fn new_time_domain(name: impl Into<String>, time: Vec<f64>, values: Vec<f64>) -> Self {
        Self::new_time_domain_in_unit(name, time, values, "V")
    }

    /// Create a new time-domain waveform of a stated quantity.
    ///
    /// A retained unit is read as fact downstream — the results browser and
    /// the plot axes both prefer it over guessing from the signal name — so a
    /// series that is not a node voltage must name its own unit here rather
    /// than inherit volts from [`Self::new_time_domain`].
    pub fn new_time_domain_in_unit(
        name: impl Into<String>,
        time: Vec<f64>,
        values: Vec<f64>,
        y_unit: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: time,
            y_values: values,
            y_unit: y_unit.into(),
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
            is_complex: true,
            y_imag: Some(imag),
        }
    }

    /// Create a complex-valued time-domain waveform, such as an I/Q carrier
    /// envelope. Existing viewers can derive magnitude and phase without
    /// discarding either component.
    pub fn new_complex_time_domain(
        name: impl Into<String>,
        time: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: time,
            y_values: real,
            y_unit: "V".to_string(),
            is_complex: true,
            y_imag: Some(imag),
        }
    }
}
