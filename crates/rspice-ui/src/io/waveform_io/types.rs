//! Waveform file formats, and how each is detected.

use super::*;

// =============================================================================
// Waveform Format
// =============================================================================

/// Supported waveform file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaveformFormat {
    /// Cadence PSF (Parameter Storage Format)
    Psf,
    /// SPICE3/ngspice raw file (NUTMEG)
    Nutmeg,
    /// ASCII raw format
    AsciiRaw,
    /// Binary raw format
    BinaryRaw,
    /// CSV (comma-separated values)
    Csv,
    /// Tab-separated values
    Tsv,
    /// Touchstone S-parameter format
    Touchstone,
}

impl WaveformFormat {
    /// Whether this format currently supports writing.
    #[cfg(test)]
    pub fn can_write(&self) -> bool {
        matches!(
            self,
            WaveformFormat::Csv | WaveformFormat::Tsv | WaveformFormat::Touchstone
        )
    }
}

// =============================================================================
// Waveform Signal
// =============================================================================

/// A single signal/variable in waveform data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformSignal {
    /// Signal name
    pub name: String,
    /// Signal type (voltage, current, time, etc.)
    pub signal_type: SignalType,
    /// Unit (V, A, s, Hz, etc.)
    pub unit: String,
    /// Data values
    pub data: Vec<f64>,
}

/// Type of signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SignalType {
    /// Time axis
    Time,
    /// Frequency axis
    Frequency,
    /// Voltage signal
    #[default]
    Voltage,
    /// Current signal
    Current,
    /// Power signal
    Power,
    /// Complex voltage (real part)
    VoltageReal,
    /// Complex voltage (imag part)
    VoltageImag,
    /// Complex current
    CurrentReal,
    /// Complex current (imag part)
    CurrentImag,
    /// S-parameter
    SParameter,
    /// Unknown
    Unknown,
}

impl WaveformSignal {
    /// Create a new signal
    pub fn new(name: impl Into<String>, signal_type: SignalType) -> Self {
        Self {
            name: name.into(),
            signal_type,
            unit: signal_type.default_unit().to_string(),
            data: Vec::new(),
        }
    }

    /// Get number of points
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get value at index
    pub fn get(&self, idx: usize) -> Option<f64> {
        self.data.get(idx).copied()
    }
}

impl SignalType {
    /// Default unit for signal type
    pub fn default_unit(&self) -> &'static str {
        match self {
            SignalType::Time => "s",
            SignalType::Frequency => "Hz",
            SignalType::Voltage | SignalType::VoltageReal | SignalType::VoltageImag => "V",
            SignalType::Current | SignalType::CurrentReal | SignalType::CurrentImag => "A",
            SignalType::Power => "W",
            SignalType::SParameter => "",
            SignalType::Unknown => "",
        }
    }

    fn from_label(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "time" => SignalType::Time,
            "frequency" | "freq" => SignalType::Frequency,
            "voltage" | "v" => SignalType::Voltage,
            "current" | "i" => SignalType::Current,
            "power" | "p" => SignalType::Power,
            _ => SignalType::Unknown,
        }
    }
}

impl std::str::FromStr for SignalType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_label(s))
    }
}

impl From<&str> for SignalType {
    fn from(s: &str) -> Self {
        Self::from_label(s)
    }
}

// =============================================================================
// Waveform Dataset
// =============================================================================

/// A complete waveform dataset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaveformDataset {
    /// Dataset title
    pub title: String,
    /// Analysis type (tran, ac, dc, etc.)
    pub analysis: String,
    /// Independent variable (usually time or frequency)
    pub x_signal: Option<WaveformSignal>,
    /// Dependent signals
    pub signals: Vec<WaveformSignal>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl WaveformDataset {
    /// Create a new dataset
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set X axis signal
    pub fn set_x(&mut self, signal: WaveformSignal) {
        self.x_signal = Some(signal);
    }

    /// Add a Y signal
    pub fn add_signal(&mut self, signal: WaveformSignal) {
        self.signals.push(signal);
    }

    /// Get signal by name
    #[cfg(test)]
    pub fn get_signal(&self, name: &str) -> Option<&WaveformSignal> {
        self.signals.iter().find(|s| s.name == name)
    }

    /// Signal count
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Point count (from X signal)
    pub fn point_count(&self) -> usize {
        self.x_signal.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    /// Get signal names
    #[cfg(test)]
    pub fn signal_names(&self) -> Vec<&str> {
        self.signals.iter().map(|s| s.name.as_str()).collect()
    }
}
