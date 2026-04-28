//! Active Viewer Selection
//!
//! Runtime selector for the specialized panel rendered in the bottom workspace.

/// Currently active viewer in the waveform panel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActiveViewer {
    /// Standard waveform viewer.
    #[default]
    Waveform,
    /// Smith chart (RF/S-parameters).
    SmithChart,
    /// Eye diagram (signal integrity).
    EyeDiagram,
    /// Histogram (statistical).
    Histogram,
    /// Bode plot (frequency response).
    BodePlot,
    /// Nyquist plot (stability).
    Nyquist,
    /// FFT spectrum.
    Fft,
    /// Pole-zero map.
    PoleZero,
}

impl ActiveViewer {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Waveform => "Waveform",
            Self::SmithChart => "Smith Chart",
            Self::EyeDiagram => "Eye Diagram",
            Self::Histogram => "Histogram",
            Self::BodePlot => "Bode Plot",
            Self::Nyquist => "Nyquist",
            Self::Fft => "FFT Spectrum",
            Self::PoleZero => "Pole-Zero",
        }
    }

    /// Full supported viewer set.
    pub fn all() -> &'static [ActiveViewer] {
        &[
            Self::Waveform,
            Self::SmithChart,
            Self::EyeDiagram,
            Self::Histogram,
            Self::BodePlot,
            Self::Nyquist,
            Self::Fft,
            Self::PoleZero,
        ]
    }

    /// Stable numeric identifier for persistence.
    pub fn id(self) -> u8 {
        match self {
            Self::Waveform => 0,
            Self::SmithChart => 1,
            Self::EyeDiagram => 2,
            Self::Histogram => 3,
            Self::BodePlot => 4,
            Self::Nyquist => 5,
            Self::Fft => 6,
            Self::PoleZero => 7,
        }
    }

    /// Parse a stable numeric identifier.
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Waveform),
            1 => Some(Self::SmithChart),
            2 => Some(Self::EyeDiagram),
            3 => Some(Self::Histogram),
            4 => Some(Self::BodePlot),
            5 => Some(Self::Nyquist),
            6 => Some(Self::Fft),
            7 => Some(Self::PoleZero),
            _ => None,
        }
    }

    /// Whether this viewer primarily requires frequency-domain data.
    pub fn needs_ac_data(&self) -> bool {
        matches!(
            self,
            Self::SmithChart | Self::BodePlot | Self::Nyquist | Self::PoleZero
        )
    }

    /// Whether this viewer primarily requires time-domain data.
    pub fn needs_transient_data(&self) -> bool {
        matches!(
            self,
            Self::Waveform | Self::EyeDiagram | Self::Histogram | Self::Fft
        )
    }
}
