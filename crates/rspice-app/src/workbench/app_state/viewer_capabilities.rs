//! What each analysis viewer can do.
//!
//! Drives which affordances a viewer offers — cursors, markers, export — so
//! the toolbar for a viewer is derived from the viewer rather than
//! maintained alongside it.

#[cfg(test)]
mod histogram_publication;
mod run_events;

use crate::state::{
    ActiveViewer, SpecializedViewerAnalysisIdentity, SpecializedViewerCacheProvenance,
};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::documents::result_document::{
    ResultsParts, ResultsReads, histogram_is_available,
};
use rspice_results_ui::eye_diagram::{EyeTimebase, EyeTimebaseKey};

/// Provenance for the mutable specialized result caches.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SpecializedViewerCacheAuthority {
    pub(crate) bode: Option<SpecializedViewerCacheProvenance>,
    pub(crate) nyquist: Option<SpecializedViewerCacheProvenance>,
    pub(crate) smith: Option<SpecializedViewerCacheProvenance>,
    pub(crate) fft: Option<SpecializedViewerCacheProvenance>,
    pub(crate) eye: Option<SpecializedViewerCacheProvenance>,
}

/// Analysis viewer state grouped behind a dedicated workspace surface.
#[derive(Debug, Clone, Default)]
pub struct AnalysisWorkspaceState {
    /// Bode viewer state
    pub(crate) bode_plot_state: rspice_results_ui::bode::BodePlotState,
    /// Nyquist viewer state
    pub(crate) nyquist_state: rspice_results_ui::nyquist::NyquistState,
    /// Eye diagram viewer state
    pub(crate) eye_diagram_state: rspice_results_ui::eye_diagram::EyeDiagramState,
    /// FFT viewer state
    pub(crate) fft_state: rspice_results_ui::fft::FftState,
    /// Smith chart viewer state
    pub(crate) smith_chart_state: rspice_results_ui::smith_chart::SmithChartState,
    /// Histogram viewer state
    pub(crate) histogram_state: rspice_results_ui::histogram::HistogramState,
    /// Exact immutable result that owns each mutable viewer cache.
    pub(crate) cache_authority: SpecializedViewerCacheAuthority,
}

/// Availability metadata for a specialized viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerCapability {
    /// Whether the viewer currently has sufficient data to be useful.
    pub available: bool,
    /// Static user-facing reason describing required data.
    pub reason: &'static str,
}

impl AnalysisWorkspaceState {
    /// Clear transient-derived viewer caches without disturbing AC/RF state.
    pub(crate) fn clear_transient_specialized_viewer_data(&mut self) {
        self.fft_state.clear();
        self.eye_diagram_state
            .load_data(rspice_results_ui::eye_diagram::EyeData::default());
        self.cache_authority.fft = None;
        self.cache_authority.eye = None;
    }

    /// Clear all specialized (non-waveform) viewer data caches.
    ///
    /// Use when result selection changes in a way that can invalidate
    /// cached analysis-specific visualizations.
    pub(crate) fn clear_specialized_viewer_data(&mut self) {
        self.clear_transient_specialized_viewer_data();
        self.histogram_state.clear_selection();
        self.bode_plot_state
            .load_data(rspice_results_ui::bode::BodeData::new());
        self.nyquist_state.clear();
        self.smith_chart_state.clear_traces();
        self.cache_authority = Default::default();
    }

    /// The exact result that owns a viewer's mutable cache, for the viewers
    /// that keep one.
    fn cache_provenance(&self, viewer: ActiveViewer) -> Option<SpecializedViewerCacheProvenance> {
        match viewer {
            ActiveViewer::BodePlot => self.cache_authority.bode,
            ActiveViewer::Nyquist => self.cache_authority.nyquist,
            ActiveViewer::SmithChart => self.cache_authority.smith,
            ActiveViewer::Fft => self.cache_authority.fft,
            ActiveViewer::EyeDiagram => self.cache_authority.eye,
            ActiveViewer::Waveform | ActiveViewer::PoleZero | ActiveViewer::Histogram => None,
        }
    }
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
    /// Start the session's waveform viewer from Results ▸ Waveform defaults:
    /// wrapped or unwrapped phase, and the minor grid. Its own toggles change
    /// both afterwards.
    pub(crate) fn start_waveform_viewer_session(&mut self) {
        self.results.results.session.phase_continuous = self.model.preferences.unwrapped_phase();
        self.results
            .start_waveform_view_from_preferences(&self.model.preferences);
    }

    /// The bit period the eye folds this result at.
    ///
    /// Lives on `AppState` rather than in the simulation controller because
    /// the choice belongs to the Results document that offers the control,
    /// and the controller must not reach up into `workbench::documents` to
    /// read it.
    pub(crate) fn eye_timebase_for(
        &self,
        provenance: SpecializedViewerCacheProvenance,
    ) -> EyeTimebase {
        self.results
            .results
            .session
            .eye_timebase
            .get(&eye_timebase_key(provenance))
            .copied()
            .unwrap_or_default()
    }

    /// Record the reader's bit period for this result.
    ///
    /// Setting it back to [`EyeTimebase::Auto`] forgets the entry rather than
    /// storing a default, so a session file carries only the rates a reader
    /// actually stated.
    pub(crate) fn set_eye_timebase(
        &mut self,
        provenance: SpecializedViewerCacheProvenance,
        timebase: EyeTimebase,
    ) {
        let key = eye_timebase_key(provenance);
        match timebase {
            EyeTimebase::Auto => {
                self.results.results.session.eye_timebase.remove(&key);
            }
            explicit => {
                self.results
                    .results
                    .session
                    .eye_timebase
                    .insert(key, explicit);
            }
        }
    }

    /// Bind a freshly populated mutable viewer cache to its immutable owner.
    pub(crate) fn bind_specialized_viewer_cache(
        &mut self,
        viewer: ActiveViewer,
        provenance: SpecializedViewerCacheProvenance,
    ) {
        *self.specialized_viewer_cache_authority_mut(viewer) = Some(provenance);
    }

    /// Forget cache ownership while a controller replaces or clears its data.
    pub(crate) fn clear_specialized_viewer_cache_authority(&mut self, viewer: ActiveViewer) {
        *self.specialized_viewer_cache_authority_mut(viewer) = None;
    }

    /// Forget ownership of the FFT cache after a transform-setting failure.
    ///
    /// The FFT document owns the setting controls but does not own the
    /// application-level viewer vocabulary. Keeping that detail here lets the
    /// document request the one transition it needs without reaching upward
    /// for [`ActiveViewer`]. The cached samples remain intact so the reader can
    /// correct the setting and recompute them.
    pub(crate) fn clear_fft_viewer_cache_authority(&mut self) {
        self.clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
    }

    /// Remove mutable viewer data that does not belong to the active result.
    ///
    /// Capability checks reject a mismatch synchronously, so a selection
    /// change cannot expose stale data even before this cleanup runs. The
    /// controller calls this after selection/polling to release the stale
    /// payload and keep every direct cache consumer safe as well.
    pub(crate) fn synchronize_specialized_viewer_cache_authority(&mut self) {
        let active = self.model.active_specialized_viewer_cache_provenance();
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Fft,
            ActiveViewer::EyeDiagram,
        ] {
            let retained = self.results.analysis.cache_provenance(viewer);
            let unbound_data = retained.is_none() && self.specialized_viewer_cache_has_data(viewer);
            let belongs_to_another_result = retained.is_some() && retained != active;
            if unbound_data || belongs_to_another_result {
                self.clear_specialized_viewer_cache(viewer);
            }
        }
    }

    fn clear_specialized_viewer_cache(&mut self, viewer: ActiveViewer) {
        match viewer {
            ActiveViewer::BodePlot => self
                .results
                .analysis
                .bode_plot_state
                .load_data(rspice_results_ui::bode::BodeData::new()),
            ActiveViewer::Nyquist => self.results.analysis.nyquist_state.clear(),
            ActiveViewer::SmithChart => self.results.analysis.smith_chart_state.clear_traces(),
            ActiveViewer::Fft => self.results.analysis.fft_state.clear(),
            ActiveViewer::EyeDiagram => self
                .results
                .analysis
                .eye_diagram_state
                .load_data(rspice_results_ui::eye_diagram::EyeData::default()),
            ActiveViewer::Waveform | ActiveViewer::PoleZero | ActiveViewer::Histogram => return,
        }
        self.clear_specialized_viewer_cache_authority(viewer);
    }

    fn specialized_viewer_cache_authority_mut(
        &mut self,
        viewer: ActiveViewer,
    ) -> &mut Option<SpecializedViewerCacheProvenance> {
        match viewer {
            ActiveViewer::BodePlot => &mut self.results.analysis.cache_authority.bode,
            ActiveViewer::Nyquist => &mut self.results.analysis.cache_authority.nyquist,
            ActiveViewer::SmithChart => &mut self.results.analysis.cache_authority.smith,
            ActiveViewer::Fft => &mut self.results.analysis.cache_authority.fft,
            ActiveViewer::EyeDiagram => &mut self.results.analysis.cache_authority.eye,
            ActiveViewer::Waveform | ActiveViewer::PoleZero | ActiveViewer::Histogram => {
                panic!("this viewer does not own mutable cache provenance")
            }
        }
    }

    fn specialized_viewer_cache_has_data(&self, viewer: ActiveViewer) -> bool {
        match viewer {
            ActiveViewer::BodePlot => !self.results.analysis.bode_plot_state.is_empty(),
            ActiveViewer::Nyquist => !self.results.analysis.nyquist_state.is_empty(),
            ActiveViewer::SmithChart => !self.results.analysis.smith_chart_state.traces.is_empty(),
            ActiveViewer::Fft => self.results.analysis.fft_state.has_data(),
            ActiveViewer::EyeDiagram => self.results.analysis.eye_diagram_state.trace_count() > 0,
            ActiveViewer::Waveform | ActiveViewer::PoleZero | ActiveViewer::Histogram => false,
        }
    }

    /// Whether a viewer is currently available for activation.
    #[cfg(test)]
    pub fn viewer_is_available(&self, viewer: ActiveViewer) -> bool {
        self.results_reads().viewer_capability(viewer).available
    }
}

impl ResultsParts<'_> {
    /// Clear user-visible simulation result history and derived result viewers.
    pub(crate) fn clear_simulation_results(&mut self) {
        if self.model.simulation.execution.has_active_execution() {
            return;
        }
        self.model.simulation.clear_runs();
        self.analysis.clear_specialized_viewer_data();
    }
}

impl ResultsReads<'_> {
    fn specialized_viewer_cache_matches_active(self, viewer: ActiveViewer) -> bool {
        self.model
            .active_specialized_viewer_cache_provenance()
            .is_some_and(|active| self.analysis.cache_provenance(viewer) == Some(active))
    }

    /// Resolve whether a viewer can currently be opened with meaningful data.
    pub(crate) fn viewer_capability(self, viewer: ActiveViewer) -> ViewerCapability {
        match viewer {
            ActiveViewer::Waveform => ViewerCapability::available("Always available"),
            ActiveViewer::SmithChart => {
                if !self.specialized_viewer_cache_matches_active(viewer)
                    || self.analysis.smith_chart_state.traces.is_empty()
                {
                    ViewerCapability::unavailable(
                        "Requires retained S-parameter traces and per-port reference impedances",
                    )
                } else {
                    ViewerCapability::available(
                        "Active S-parameter traces loaded from retained impedance authority",
                    )
                }
            }
            ActiveViewer::EyeDiagram => {
                if self.specialized_viewer_cache_matches_active(viewer)
                    && self.analysis.eye_diagram_state.trace_count() > 0
                {
                    ViewerCapability::available("Active analysis eye traces loaded")
                } else if active_analysis_supports_eye_diagram(self.model) {
                    ViewerCapability::available(
                        "Can derive eye data from active transient waveforms",
                    )
                } else {
                    ViewerCapability::unavailable("Requires transient eye traces")
                }
            }
            ActiveViewer::Histogram => {
                if histogram_is_available(self) {
                    ViewerCapability::available("Retained Monte Carlo samples available")
                } else {
                    ViewerCapability::unavailable("Requires retained Monte Carlo samples")
                }
            }
            ActiveViewer::BodePlot => {
                if !self.specialized_viewer_cache_matches_active(viewer)
                    || self.analysis.bode_plot_state.is_empty()
                {
                    ViewerCapability::unavailable("Requires AC/transfer-function response data")
                } else {
                    ViewerCapability::available("Active analysis frequency-response data loaded")
                }
            }
            ActiveViewer::Nyquist => {
                if !self.specialized_viewer_cache_matches_active(viewer)
                    || self.analysis.nyquist_state.is_empty()
                {
                    // The sheet takes a retained loop-gain contour and nothing
                    // else — an AC node response is not a loop gain, and the
                    // stability numbers beside the locus have no meaning on
                    // one. Offering an AC sweep sent the reader to run an
                    // analysis that can never fill this viewer.
                    ViewerCapability::unavailable(
                        "Requires a retained Nyquist contour from a stability (.STB) run",
                    )
                } else {
                    ViewerCapability::available("Active analysis loop-gain locus loaded")
                }
            }
            ActiveViewer::Fft => {
                let has_spectrum = self.analysis.fft_state.has_data()
                    && self
                        .analysis
                        .fft_state
                        .data
                        .as_ref()
                        .is_some_and(|data| !data.is_empty());
                if self.specialized_viewer_cache_matches_active(viewer) && has_spectrum {
                    ViewerCapability::available("Active analysis FFT spectrum data loaded")
                } else if active_analysis_supports_fft(self.model) {
                    ViewerCapability::available("Can derive FFT from active transient waveforms")
                } else {
                    ViewerCapability::unavailable("Requires sampled time-domain data for FFT")
                }
            }
            ActiveViewer::PoleZero => {
                let retained = self
                    .model
                    .simulation
                    .active_analysis()
                    .is_some_and(|analysis| {
                        analysis.success
                            && analysis.analysis_type == crate::state::AnalysisType::PoleZero
                            && analysis.result_payload.as_ref().is_some_and(|payload| {
                                matches!(
                                    payload,
                                    crate::state::AnalysisResultPayload::PoleZero { .. }
                                ) && payload.validate_for(analysis.analysis_type).is_ok()
                            })
                    });
                if retained {
                    ViewerCapability::available("Pole-zero result retained by the active analysis")
                } else {
                    ViewerCapability::unavailable(
                        "Requires a retained pole-zero payload in the active analysis",
                    )
                }
            }
        }
    }
}

fn active_analysis_supports_eye_diagram(model: &Model) -> bool {
    active_time_domain_waveform_len(model)
        .map(|len| len >= 8)
        .unwrap_or(false)
}

fn active_analysis_supports_fft(model: &Model) -> bool {
    active_time_domain_waveform_len(model)
        .map(|len| len >= rspice_results_ui::fft::MIN_FFT_SAMPLES)
        .unwrap_or(false)
}

fn active_time_domain_waveform_len(model: &Model) -> Option<usize> {
    let analysis = model.simulation.active_analysis()?;
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

/// The timebase map's key for one result.
fn eye_timebase_key(provenance: SpecializedViewerCacheProvenance) -> EyeTimebaseKey {
    match provenance.analysis_identity {
        SpecializedViewerAnalysisIdentity::Prepared(instance) => EyeTimebaseKey::Prepared(instance),
        SpecializedViewerAnalysisIdentity::LegacyResultId(id) => {
            EyeTimebaseKey::Legacy(provenance.dataset_id, id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::{LogSeverity, LogSource};
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
    use rspice_results_ui::{
        bode::BodeData,
        eye_diagram::{EyeData, EyeTrace},
        fft::{FftData, WindowFunction},
        nyquist::NyquistData,
    };

    fn retained_analysis(
        id: u64,
        analysis_type: crate::state::AnalysisType,
        digest_byte: u8,
    ) -> crate::state::AnalysisResult {
        let provenance = crate::state::AnalysisResultProvenance::new(
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([digest_byte; 32]),
            Vec::new(),
        )
        .expect("test analysis provenance");
        crate::state::AnalysisResult::new(id, analysis_type, format!("analysis {id}"))
            .with_provenance(provenance)
    }

    fn seed_result_viewers(state: &mut AppState) {
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(retained_analysis(1, crate::state::AnalysisType::Ac, 0x11));
        state.model.simulation.retained.runs.push(run);
        assert!(state.model.simulation.select_run(0));

        let mut bode = BodeData::new();
        bode.add_response();
        state.results.analysis.bode_plot_state.load_data(bode);

        state
            .results
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays(
                "old nyquist",
                &[1.0, 10.0],
                &[1.0, -0.5],
                &[0.0, 0.25],
            ));

        state
            .results
            .analysis
            .smith_chart_state
            .load_sparam_data("S11", &[1.0], &[0.25], &[0.0], Some(50.0))
            .expect("valid Smith fixture");

        let mut eye = EyeData::new(1e-9, 2);
        eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]));
        state.results.analysis.eye_diagram_state.load_data(eye);

        state.results.analysis.fft_state.load_data(
            FftData::from_time_domain_with_normalization(
                "old fft",
                &[
                    0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,
                    -1.0,
                ],
                16.0,
                WindowFunction::Rectangular,
                rspice_results_ui::fft::data::SpectrumNormalization::Peak,
            )
            .expect("finite qualified prior FFT fixture"),
        );

        let provenance = state
            .model
            .active_specialized_viewer_cache_provenance()
            .expect("seeded active result");
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Fft,
            ActiveViewer::EyeDiagram,
        ] {
            state.bind_specialized_viewer_cache(viewer, provenance);
        }
    }

    fn seed_blocking_drc_result(state: &mut AppState) {
        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MissingGround,
            "missing ground",
            DrcLocation::Global,
        ));
        result.completed = true;
        state
            .publish_active_design_check_result(result)
            .expect("test DRC receipt must publish");
        state
            .model
            .log_buffer
            .log(LogSeverity::Error, LogSource::Studio, "old DRC anchor");
    }

    #[test]
    fn clearing_simulation_results_clears_specialized_result_viewers() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);
        assert!(state.model.simulation.retained.has_results());
        // Arbitrary cached bins on an AC analysis are not MC evidence.
        assert!(!state.viewer_is_available(ActiveViewer::Histogram));
        assert!(state.viewer_is_available(ActiveViewer::SmithChart));
        assert!(state.viewer_is_available(ActiveViewer::Fft));

        state.results_parts().clear_simulation_results();

        assert!(!state.model.simulation.retained.has_results());
        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
            ActiveViewer::PoleZero,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} should be unavailable after clearing results",
                viewer.name()
            );
        }
    }

    /// Run 3 announced in one design is not run 3 of the next: once run
    /// history is replaced the notice keeps its words and loses the offer, and
    /// following nothing cannot open the wrong run.
    #[test]
    fn replacing_the_design_retires_run_offers_and_keeps_the_notices() {
        use crate::ui::widgets::{NotificationAction, NotificationCategory, ToastKind};

        let ctx = egui::Context::default();
        let mut state = AppState::default();
        state.workbench.toasts.notify_with_action(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Success,
            "Run 3 complete",
            "2 analyses saved with this run.",
            NotificationAction::OpenRunInResults { run_sequence: 3 },
        );
        state.workbench.toasts.notify_with_action(
            &ctx,
            NotificationCategory::System,
            ToastKind::Error,
            "Export failed",
            "The folder is read-only.",
            NotificationAction::ShowInConsole { log_id: 7 },
        );

        state.clear_design_execution_context();

        let records = state.workbench.toasts.activity();
        assert_eq!(records.len(), 2, "what happened is still true");
        assert_eq!(records[1].title(), "Run 3 complete");
        assert_eq!(records[1].message(), "2 analyses saved with this run.");
        assert_eq!(
            records[1].action(),
            None,
            "the offer named a run the new history does not have"
        );
        assert_eq!(
            records[0].action(),
            Some(NotificationAction::ShowInConsole { log_id: 7 }),
            "the session log survives the switch, and so does the way to it"
        );
    }

    #[test]
    fn clearing_fft_authority_preserves_samples_and_other_viewer_owners() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);
        let bode_authority = state.results.analysis.cache_authority.bode;

        state.clear_fft_viewer_cache_authority();

        assert_eq!(state.results.analysis.cache_authority.fft, None);
        assert_eq!(state.results.analysis.cache_authority.bode, bode_authority);
        assert!(state.results.analysis.fft_state.has_data());
    }

    #[test]
    fn compatible_run_and_analysis_switches_never_inherit_another_result_cache() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);

        state.model.simulation.retained.runs[0].add_analysis(retained_analysis(
            2,
            crate::state::AnalysisType::Ac,
            0x12,
        ));
        let mut second_run = crate::state::SimulationRun::new(2);
        second_run.add_analysis(retained_analysis(1, crate::state::AnalysisType::Ac, 0x21));
        state.model.simulation.retained.runs.push(second_run);

        assert!(state.model.simulation.select_analysis(1));
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} must reject cache data from another analysis identity",
                viewer.name()
            );
        }

        assert!(state.model.simulation.select_run(1));
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} must reject cache data from another dataset",
                viewer.name()
            );
        }
    }

    #[test]
    fn incompatible_run_switch_rejects_and_releases_all_specialized_caches() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);

        let mut incompatible = crate::state::SimulationRun::new(2);
        incompatible.add_analysis(retained_analysis(1, crate::state::AnalysisType::DcOp, 0x31));
        state.model.simulation.retained.runs.push(incompatible);
        assert!(state.model.simulation.select_run(1));

        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
            ActiveViewer::Fft,
            ActiveViewer::EyeDiagram,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} must reject cache data from an incompatible dataset",
                viewer.name()
            );
        }

        state.synchronize_specialized_viewer_cache_authority();
        assert!(state.results.analysis.bode_plot_state.is_empty());
        assert!(state.results.analysis.nyquist_state.is_empty());
        assert!(state.results.analysis.smith_chart_state.traces.is_empty());
        assert!(!state.results.analysis.fft_state.has_data());
        assert_eq!(state.results.analysis.eye_diagram_state.trace_count(), 0);
        assert_eq!(
            state.results.analysis.cache_authority,
            SpecializedViewerCacheAuthority::default()
        );
    }

    /// The gate has to name the analysis that can open it.
    ///
    /// The sheet takes a retained loop-gain contour and nothing else, so
    /// "complex loop-gain/AC response data" sends a reader to run an AC
    /// sweep that can never fill it. And a Nyquist result is one locus, not
    /// a set of curves.
    #[test]
    fn the_nyquist_gate_names_the_stability_analysis_and_its_single_locus() {
        let mut state = AppState::default();
        let closed = state
            .results_reads()
            .viewer_capability(ActiveViewer::Nyquist);
        assert!(!closed.available);
        assert!(
            closed.reason.contains(".STB"),
            "the closed gate does not name the analysis that fills it: {}",
            closed.reason
        );
        assert!(
            !closed.reason.contains("AC response"),
            "the closed gate still offers an AC sweep: {}",
            closed.reason
        );

        seed_result_viewers(&mut state);
        let open = state
            .results_reads()
            .viewer_capability(ActiveViewer::Nyquist);
        assert!(open.available, "{}", open.reason);
        assert!(
            !open.reason.contains("curves"),
            "one locus is not a set of curves: {}",
            open.reason
        );
    }

    #[test]
    fn pole_zero_capability_uses_only_the_active_retained_payload() {
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::PoleZero,
            "PZ without retained evidence",
        ));
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        assert!(
            !state.viewer_is_available(ActiveViewer::PoleZero),
            "an analysis without retained pole-zero evidence must not enable the viewer"
        );

        state.model.simulation.retained.runs[0].analyses[0] = crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::PoleZero,
            "Retained PZ",
        )
        .with_result_payload(crate::state::AnalysisResultPayload::PoleZero {
            poles: vec![crate::state::ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: Vec::new(),
            pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(3.5),
        });

        assert!(
            state.viewer_is_available(ActiveViewer::PoleZero),
            "retained root evidence must enable the viewer without a mutable cache"
        );
    }

    #[test]
    fn clearing_design_execution_context_clears_stale_drc_results() {
        let mut state = AppState::default();
        seed_blocking_drc_result(&mut state);
        assert!(state.current_blocking_drc_result().is_some());
        let logged = state.model.log_buffer.entries().count();

        state.clear_design_execution_context();

        assert!(state.model.design_checks.drc_results.is_none());
        assert!(state.model.diagnostics.current_finding().is_none());
        assert!(state.current_blocking_drc_result().is_none());
        assert_eq!(
            state.model.log_buffer.entries().count(),
            logged,
            "the findings go; the Console's record of the check stays"
        );
    }

    #[test]
    fn clearing_design_execution_context_clears_source_runtime_and_keeps_the_console() {
        let mut state = AppState::default();
        state.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
        state.code.code_workspace.automation.debug.watches.push(
            crate::workbench::documents::code_workspace::AutomationWatch {
                expression: "project.old_value".to_owned(),
                value: "17".to_owned(),
                error: None,
            },
        );
        state.code.netlist.diagnostics = std::sync::Arc::new(
            crate::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(
                vec![crate::state::netlist_diagnostics::Diagnostic::error(
                    "old project diagnostic",
                )],
                "",
            )
            .unwrap(),
        );
        state
            .model
            .log_buffer
            .warning(LogSource::Run, "old project warning");
        state.clear_design_execution_context();

        assert_eq!(
            state.code.code_workspace.page,
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        );
        assert!(
            state
                .code
                .code_workspace
                .automation
                .debug
                .watches
                .is_empty()
        );
        assert_eq!(state.code.netlist.diagnostics.summary().total(), 0);
        assert!(
            state
                .model
                .log_buffer
                .entries()
                .any(|entry| entry.message == "old project warning"),
            "the Console is the session's record and keeps its history"
        );
    }

    #[test]
    fn clearing_design_execution_context_clears_project_scoped_results_state() {
        use crate::workbench::documents::result_document::{
            AnalysisPresentationKey, ExprSeries, ExprTrace, ResultViewer,
        };
        use rspice_results_ui::session::ExprEditor;

        let mut state = AppState::default();
        let analysis = crate::state::AnalysisResult::new(
            71,
            crate::state::AnalysisType::Transient,
            "Transient",
        );
        let analysis_key =
            AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        state.results.results_seen_version = 99;
        let results = &mut state.results.results;
        results.session.viewer = ResultViewer::Fft;
        results.session.phase_continuous = true;
        results.session.cursors.place(1.0);
        results.session.cursor_strip = Some(0);
        results.session.hidden_strips.insert(analysis_key);
        results.session.maximized_strip = Some(analysis_key);
        *results.session.plot_view_mut(ResultViewer::Waves, 0) =
            rspice_results_ui::presentation::PlotView {
                x: Some((0.0, 1.0)),
                y: Some((-1.0, 1.0)),
            };
        results.session.analysis_exprs.insert(
            analysis_key,
            vec![ExprTrace {
                text: "V(out)/V(in)".to_string(),
                complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
                visible: true,
            }],
        );
        results.session.expr_editor = Some(ExprEditor {
            analysis: analysis_key,
            text: "V(out)".to_string(),
            error: Some("stale".to_string()),
            want_focus: false,
        });
        results.analysis_expr_cache.insert(
            (analysis_key, "V(out)".to_string()),
            ExprSeries {
                version: 10,
                series: Err("stale".to_string()),
            },
        );
        results.session.rf_pin.insert(ResultViewer::Smith, (0, 1));
        results.session.op_filter = "M1".to_string();
        results.session.op_sort = Some(("gm".to_string(), true));
        results.session.spec_drafts = Some(Vec::new());
        state.results.specification_editor_route_pending = true;

        state.clear_design_execution_context();
        let results = &state.results.results;

        assert_eq!(state.results.results_seen_version, 0);
        assert_eq!(results.session.viewer, ResultViewer::Fft);
        assert!(results.session.phase_continuous);
        assert!(!results.session.cursors.any());
        assert_eq!(results.session.cursor_strip, None);
        assert!(results.session.hidden_strips.is_empty());
        assert_eq!(results.session.maximized_strip, None);
        assert!(results.session.views.is_empty());
        assert!(results.session.analysis_exprs.is_empty());
        assert!(results.session.expr_editor.is_none());
        assert!(results.analysis_expr_cache.is_empty());
        assert!(results.session.rf_pin.is_empty());
        assert!(results.session.op_filter.is_empty());
        assert_eq!(results.session.op_sort, None);
        assert!(results.session.spec_drafts.is_none());
        assert!(!state.results.specification_editor_route_pending);
    }
}
