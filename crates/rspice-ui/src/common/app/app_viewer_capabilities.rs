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
    /// Clear all specialized (non-waveform) viewer data caches.
    ///
    /// Use when result selection changes in a way that can invalidate
    /// cached analysis-specific visualizations.
    pub fn clear_specialized_viewer_data(&mut self) {
        self.fft_state.clear();
        self.eye_diagram_state
            .load_data(crate::analysis::eye_diagram::data::EyeData::default());
        self.histogram_state.clear();
        self.bode_plot_state
            .load_data(crate::analysis::bode::BodeData::new());
        self.nyquist_state.clear();
        self.smith_chart_state.clear_traces();
        self.pole_zero_state.clear();
    }

    /// Resolve whether a viewer can currently be opened with meaningful data.
    pub fn viewer_capability(&self, viewer: ActiveViewer) -> ViewerCapability {
        match viewer {
            ActiveViewer::Waveform => ViewerCapability::available("Always available"),
            ActiveViewer::SmithChart => {
                if self.smith_chart_state.traces.is_empty() {
                    ViewerCapability::unavailable("Requires S-parameter complex traces")
                } else {
                    ViewerCapability::available("S-parameter traces loaded")
                }
            }
            ActiveViewer::EyeDiagram => {
                if self.eye_diagram_state.trace_count() == 0 {
                    ViewerCapability::unavailable("Requires transient eye traces")
                } else {
                    ViewerCapability::available("Eye traces loaded")
                }
            }
            ActiveViewer::Histogram => {
                if self.histogram_state.is_empty() {
                    ViewerCapability::unavailable("Requires histogram bins from sweep/MC data")
                } else {
                    ViewerCapability::available("Histogram data loaded")
                }
            }
            ActiveViewer::BodePlot => {
                if self.bode_plot_state.is_empty() {
                    ViewerCapability::unavailable("Requires AC/transfer-function response data")
                } else {
                    ViewerCapability::available("Frequency-response data loaded")
                }
            }
            ActiveViewer::Nyquist => {
                if self.nyquist_state.is_empty() {
                    ViewerCapability::unavailable("Requires complex loop-gain/AC response data")
                } else {
                    ViewerCapability::available("Nyquist curves loaded")
                }
            }
            ActiveViewer::Fft => {
                let has_spectrum = self
                    .fft_state
                    .data
                    .as_ref()
                    .map(|data| !data.is_empty())
                    .unwrap_or(false);
                if has_spectrum {
                    ViewerCapability::available("FFT spectrum data loaded")
                } else {
                    ViewerCapability::unavailable("Requires sampled time-domain data for FFT")
                }
            }
            ActiveViewer::PoleZero => {
                if self.pole_zero_state.is_empty() {
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

    /// Close the active viewer tab.
    pub fn close_active_viewer(&mut self) {
        self.viewer_workspace.close_active();
    }

    /// Active specialized viewer currently displayed in the workspace.
    pub fn active_viewer(&self) -> ActiveViewer {
        self.viewer_workspace.active_viewer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_bode(state: &mut AppState) {
        let mut response = crate::analysis::bode::data::FrequencyResponse::new("tf");
        response.add_point(crate::analysis::bode::data::FrequencyPoint::new(
            1.0, 1.0, 0.0,
        ));
        let mut data = crate::analysis::bode::BodeData::new();
        data.add_response(response);
        state.bode_plot_state.load_data(data);
    }

    fn seed_nyquist(state: &mut AppState) {
        let curve = crate::analysis::nyquist::NyquistData::from_arrays(
            "loop",
            &[1.0, 10.0],
            &[0.5, 0.25],
            &[0.0, -0.2],
        );
        state.nyquist_state.load_data(curve);
    }

    fn seed_fft(state: &mut AppState) {
        let mut fft = crate::analysis::fft::FftData::new("spec");
        fft.points
            .push(crate::analysis::fft::FftPoint::new(1.0, 1.0, 0.0));
        state.fft_state.load_data(fft);
    }

    fn seed_eye(state: &mut AppState) {
        let mut eye = crate::analysis::eye_diagram::data::EyeData::default();
        eye.add_trace(crate::analysis::eye_diagram::data::EyeTrace::new(
            vec![0.0, 1.0],
            vec![0.0, 1.0],
        ));
        state.eye_diagram_state.load_data(eye);
    }

    fn seed_histogram(state: &mut AppState) {
        let histogram = crate::analysis::histogram::data::HistogramBuilder::new()
            .name("mc")
            .build(&[0.9, 1.0, 1.1]);
        state.histogram_state.load_histogram(histogram);
    }

    fn seed_smith(state: &mut AppState) {
        state
            .smith_chart_state
            .load_sparam_data("S11", &[1.0, 2.0], &[0.2, 0.1], &[0.0, -0.1]);
    }

    fn seed_pole_zero(state: &mut AppState) {
        let mut data = crate::analysis::pole_zero::PoleZeroData::new("pz");
        data.add_real_pole(-1.0);
        state.pole_zero_state.load_data(data);
    }

    #[test]
    fn viewer_capabilities_default_to_waveform_only() {
        let state = AppState::default();
        assert!(state.viewer_is_available(ActiveViewer::Waveform));
        for viewer in ActiveViewer::all()
            .iter()
            .copied()
            .filter(|viewer| *viewer != ActiveViewer::Waveform)
        {
            assert!(
                !state.viewer_is_available(viewer),
                "{:?} should require analysis data",
                viewer
            );
        }
    }

    #[test]
    fn viewer_capabilities_enable_each_specialized_viewer_when_seeded() {
        let mut state = AppState::default();
        seed_bode(&mut state);
        seed_nyquist(&mut state);
        seed_fft(&mut state);
        seed_eye(&mut state);
        seed_histogram(&mut state);
        seed_smith(&mut state);
        seed_pole_zero(&mut state);

        assert!(state.viewer_is_available(ActiveViewer::BodePlot));
        assert!(state.viewer_is_available(ActiveViewer::Nyquist));
        assert!(state.viewer_is_available(ActiveViewer::Fft));
        assert!(state.viewer_is_available(ActiveViewer::EyeDiagram));
        assert!(state.viewer_is_available(ActiveViewer::Histogram));
        assert!(state.viewer_is_available(ActiveViewer::SmithChart));
        assert!(state.viewer_is_available(ActiveViewer::PoleZero));
    }

    #[test]
    fn open_viewer_in_tab_falls_back_to_waveform_when_unavailable() {
        let mut state = AppState::default();
        let opened = state.open_viewer_in_tab(ActiveViewer::BodePlot, BottomPanelTab::Automation);

        assert_eq!(opened, ActiveViewer::Waveform);
        assert_eq!(state.active_viewer(), ActiveViewer::Waveform);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Automation);
    }

    #[test]
    fn open_preferred_viewer_uses_highest_priority_available_option() {
        let mut state = AppState::default();
        seed_bode(&mut state);
        seed_nyquist(&mut state);
        seed_smith(&mut state);

        let opened = state.open_preferred_viewer_for_analysis(AnalysisType::SParameter);
        assert_eq!(opened, ActiveViewer::SmithChart);

        state.smith_chart_state.clear_traces();
        let opened = state.open_preferred_viewer_for_analysis(AnalysisType::SParameter);
        assert_eq!(opened, ActiveViewer::BodePlot);
    }

    #[test]
    fn open_preferred_viewer_falls_back_to_waveform_when_no_specialized_data() {
        let mut state = AppState::default();
        let opened = state.open_preferred_viewer_for_analysis(AnalysisType::Ac);
        assert_eq!(opened, ActiveViewer::Waveform);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
    }

    #[test]
    fn clear_specialized_viewer_data_resets_all_non_waveform_capabilities() {
        let mut state = AppState::default();
        seed_bode(&mut state);
        seed_nyquist(&mut state);
        seed_fft(&mut state);
        seed_eye(&mut state);
        seed_histogram(&mut state);
        seed_smith(&mut state);
        seed_pole_zero(&mut state);

        state.clear_specialized_viewer_data();

        assert!(state.viewer_is_available(ActiveViewer::Waveform));
        for viewer in ActiveViewer::all()
            .iter()
            .copied()
            .filter(|viewer| *viewer != ActiveViewer::Waveform)
        {
            assert!(
                !state.viewer_is_available(viewer),
                "{:?} should be unavailable after cache clear",
                viewer
            );
        }
    }
}
