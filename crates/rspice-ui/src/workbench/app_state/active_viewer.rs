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

}
