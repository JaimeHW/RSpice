//! Sampled waveform data.

mod output;
pub use output::OutputWaveform;

mod retained;
pub use retained::{
    ComplexWaveformComponents, RetainedWaveform, SharedWaveformValues, samples_equal,
};

mod trace;
mod values;
pub use trace::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, RetainedTrace};
#[cfg(feature = "engine-evidence")]
pub(crate) use values::report_column_read_failure;
pub use values::{
    ColumnReadFailure, ColumnSource, ColumnTile, RowsAround, StoredWindow, ValueExtent,
    WaveformValues, WindowLead, take_column_read_failures,
};

//=============================================================================
// Sampled Waveform
//=============================================================================

/// Time-domain waveform data for a single signal
#[derive(Debug, Clone)]
pub struct SampledWaveform {
    /// Signal name (e.g., "V(out)", "I(R1)")
    pub name: String,

    /// X-axis values (time for transient, frequency for AC), shared by every waveform of the
    /// run that holds the same axis.
    pub x_values: WaveformValues,

    /// Y-axis values (voltage, current, etc.)
    pub y_values: WaveformValues,

    /// Unit for Y-axis (V, A, dB, degrees, etc.)
    pub y_unit: String,

    /// Whether this is a complex result (for AC analysis)
    pub is_complex: bool,

    /// Imaginary part (for AC analysis)
    pub y_imag: Option<Vec<f64>>,
}

impl SampledWaveform {
    /// Create a new time-domain node-voltage waveform.
    pub fn new_time_domain(
        name: impl Into<String>,
        time: impl Into<WaveformValues>,
        values: impl Into<WaveformValues>,
    ) -> Self {
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
        time: impl Into<WaveformValues>,
        values: impl Into<WaveformValues>,
        y_unit: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: time.into(),
            y_values: values.into(),
            y_unit: y_unit.into(),
            is_complex: false,
            y_imag: None,
        }
    }

    /// Create new complex frequency-domain waveform
    pub fn new_complex(
        name: impl Into<String>,
        freq: impl Into<WaveformValues>,
        real: impl Into<WaveformValues>,
        imag: Vec<f64>,
    ) -> Self {
        Self::new_complex_in_unit(name, freq, real, imag, "")
    }

    /// Create a complex frequency-domain waveform with its producer's unit.
    pub fn new_complex_in_unit(
        name: impl Into<String>,
        freq: impl Into<WaveformValues>,
        real: impl Into<WaveformValues>,
        imag: Vec<f64>,
        unit: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: freq.into(),
            y_values: real.into(),
            y_unit: unit.into(),
            is_complex: true,
            y_imag: Some(imag),
        }
    }
}
