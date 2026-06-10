use super::{ActiveViewer, AppState};

/// Availability metadata for a specialized viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerCapability {
    /// Whether the viewer currently has sufficient data to be useful.
    pub available: bool,
    /// Static user-facing reason describing required data.
    pub reason: &'static str,
}

impl ViewerCapability {
    const fn available(reason: &'static str) -> Self {
        Self {
            available: true,
            reason,
        }
    }

    const fn unavailable(reason: &'static str) -> Self {
        Self {
            available: false,
            reason,
        }
    }
}

impl AppState {
    /// Clear transient-derived viewer caches without disturbing AC/RF state.
    pub fn clear_transient_specialized_viewer_data(&mut self) {
        self.analysis.fft_state.clear();
        self.analysis
            .eye_diagram_state
            .load_data(crate::analysis::eye_diagram::data::EyeData::default());
    }

    /// Clear all specialized (non-waveform) viewer data caches.
    ///
    /// Use when result selection changes in a way that can invalidate
    /// cached analysis-specific visualizations.
    pub fn clear_specialized_viewer_data(&mut self) {
        self.clear_transient_specialized_viewer_data();
        self.analysis.histogram_state.clear();
        self.analysis
            .bode_plot_state
            .load_data(crate::analysis::bode::BodeData::new());
        self.analysis.nyquist_state.clear();
        self.analysis.smith_chart_state.clear_traces();
        self.analysis.pole_zero_state.clear();
    }

    /// Resolve whether a viewer can currently be opened with meaningful data.
    pub fn viewer_capability(&self, viewer: ActiveViewer) -> ViewerCapability {
        match viewer {
            ActiveViewer::Waveform => ViewerCapability::available("Always available"),
            ActiveViewer::SmithChart => {
                if self.analysis.smith_chart_state.traces.is_empty() {
                    ViewerCapability::unavailable("Requires S-parameter complex traces")
                } else {
                    ViewerCapability::available("S-parameter traces loaded")
                }
            }
            ActiveViewer::EyeDiagram => {
                if self.analysis.eye_diagram_state.trace_count() > 0 {
                    ViewerCapability::available("Eye traces loaded")
                } else if self.active_analysis_supports_eye_diagram() {
                    ViewerCapability::available(
                        "Can derive eye data from active transient waveforms",
                    )
                } else {
                    ViewerCapability::unavailable("Requires transient eye traces")
                }
            }
            ActiveViewer::Histogram => {
                if self.analysis.histogram_state.is_empty() {
                    ViewerCapability::unavailable("Requires histogram bins from sweep/MC data")
                } else {
                    ViewerCapability::available("Histogram data loaded")
                }
            }
            ActiveViewer::BodePlot => {
                if self.analysis.bode_plot_state.is_empty() {
                    ViewerCapability::unavailable("Requires AC/transfer-function response data")
                } else {
                    ViewerCapability::available("Frequency-response data loaded")
                }
            }
            ActiveViewer::Nyquist => {
                if self.analysis.nyquist_state.is_empty() {
                    ViewerCapability::unavailable("Requires complex loop-gain/AC response data")
                } else {
                    ViewerCapability::available("Nyquist curves loaded")
                }
            }
            ActiveViewer::Fft => {
                let has_spectrum = self
                    .analysis
                    .fft_state
                    .data
                    .as_ref()
                    .map(|data| !data.is_empty())
                    .unwrap_or(false);
                if has_spectrum {
                    ViewerCapability::available("FFT spectrum data loaded")
                } else if self.active_analysis_supports_fft() {
                    ViewerCapability::available("Can derive FFT from active transient waveforms")
                } else {
                    ViewerCapability::unavailable("Requires sampled time-domain data for FFT")
                }
            }
            ActiveViewer::PoleZero => {
                if self.analysis.pole_zero_state.is_empty() {
                    ViewerCapability::unavailable("Requires pole-zero root data")
                } else {
                    ViewerCapability::available("Pole-zero root data loaded")
                }
            }
        }
    }

    /// Whether a viewer is currently available for activation.
    pub fn viewer_is_available(&self, viewer: ActiveViewer) -> bool {
        self.viewer_capability(viewer).available
    }

    fn active_analysis_supports_eye_diagram(&self) -> bool {
        self.active_time_domain_waveform_len()
            .map(|len| len >= 8)
            .unwrap_or(false)
    }

    fn active_analysis_supports_fft(&self) -> bool {
        self.active_time_domain_waveform_len()
            .map(|len| len >= crate::analysis::fft::MIN_FFT_SAMPLES)
            .unwrap_or(false)
    }

    fn active_time_domain_waveform_len(&self) -> Option<usize> {
        let analysis = self.simulation.active_analysis()?;
        crate::simulation::SimulationController::analysis_supports_transient_derivation(
            analysis.analysis_type,
        )
        .then_some(())?;

        analysis
            .waveforms
            .iter()
            .map(|wf| wf.x.len().min(wf.y.len()))
            .max()
    }
}
