use super::{AppState, BottomPanelTab};
use crate::common::analysis_navigation;
use crate::state::AnalysisType;
use crate::viewers::ActiveViewer;

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

    /// Open/focus a specialized viewer and route the bottom panel to it.
    ///
    /// If the requested viewer lacks required data, this falls back to `Waveform`.
    pub fn open_viewer(&mut self, viewer: ActiveViewer) -> ActiveViewer {
        self.open_viewer_in_tab(viewer, BottomPanelTab::Waveform)
    }

    /// Open/focus a specialized viewer and route to a specific bottom tab.
    ///
    /// Returns the viewer that was actually activated after capability fallback.
    pub fn open_viewer_in_tab(
        &mut self,
        viewer: ActiveViewer,
        tab: BottomPanelTab,
    ) -> ActiveViewer {
        let selected = self.resolve_openable_viewer(viewer);
        self.viewer_workspace.open_or_focus(selected);
        self.panels.bottom_panel = true;
        self.panels.active_bottom_tab = tab;
        selected
    }

    /// Open the highest-priority available viewer for an analysis type.
    pub fn open_preferred_viewer_for_analysis(
        &mut self,
        analysis_type: AnalysisType,
    ) -> ActiveViewer {
        let tab = analysis_navigation::preferred_bottom_tab(analysis_type);
        let selected = analysis_navigation::preferred_viewers(analysis_type)
            .iter()
            .copied()
            .find(|viewer| self.viewer_is_available(*viewer))
            .unwrap_or(ActiveViewer::Waveform);
        self.open_viewer_in_tab(selected, tab)
    }

    fn resolve_openable_viewer(&self, viewer: ActiveViewer) -> ActiveViewer {
        if self.viewer_is_available(viewer) {
            viewer
        } else {
            ActiveViewer::Waveform
        }
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

    /// Close the active viewer tab.
    pub fn close_active_viewer(&mut self) {
        self.viewer_workspace.close_active();
    }

    /// Active specialized viewer currently displayed in the workspace.
    pub fn active_viewer(&self) -> ActiveViewer {
        self.viewer_workspace.active_viewer()
    }
}

