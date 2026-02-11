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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_viewer_default_is_waveform() {
        assert_eq!(ActiveViewer::default(), ActiveViewer::Waveform);
    }

    #[test]
    fn active_viewer_all_contains_every_variant() {
        let all = ActiveViewer::all();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&ActiveViewer::Waveform));
        assert!(all.contains(&ActiveViewer::SmithChart));
        assert!(all.contains(&ActiveViewer::EyeDiagram));
        assert!(all.contains(&ActiveViewer::Histogram));
        assert!(all.contains(&ActiveViewer::BodePlot));
        assert!(all.contains(&ActiveViewer::Nyquist));
        assert!(all.contains(&ActiveViewer::Fft));
        assert!(all.contains(&ActiveViewer::PoleZero));
    }

    #[test]
    fn active_viewer_display_names_are_stable() {
        assert_eq!(ActiveViewer::Waveform.name(), "Waveform");
        assert_eq!(ActiveViewer::SmithChart.name(), "Smith Chart");
        assert_eq!(ActiveViewer::EyeDiagram.name(), "Eye Diagram");
        assert_eq!(ActiveViewer::Histogram.name(), "Histogram");
        assert_eq!(ActiveViewer::BodePlot.name(), "Bode Plot");
        assert_eq!(ActiveViewer::Nyquist.name(), "Nyquist");
        assert_eq!(ActiveViewer::Fft.name(), "FFT Spectrum");
        assert_eq!(ActiveViewer::PoleZero.name(), "Pole-Zero");
    }

    #[test]
    fn active_viewer_data_domain_flags_are_correct() {
        assert!(ActiveViewer::SmithChart.needs_ac_data());
        assert!(ActiveViewer::BodePlot.needs_ac_data());
        assert!(ActiveViewer::Nyquist.needs_ac_data());
        assert!(ActiveViewer::PoleZero.needs_ac_data());
        assert!(!ActiveViewer::Waveform.needs_ac_data());

        assert!(ActiveViewer::Waveform.needs_transient_data());
        assert!(ActiveViewer::EyeDiagram.needs_transient_data());
        assert!(ActiveViewer::Histogram.needs_transient_data());
        assert!(ActiveViewer::Fft.needs_transient_data());
        assert!(!ActiveViewer::SmithChart.needs_transient_data());
    }
}
