//! What each analysis viewer can do.
//!
//! Drives which affordances a viewer offers — cursors, markers, export — so
//! the toolbar for a viewer is derived from the viewer rather than
//! maintained alongside it.

use crate::workbench::app_state::{ActiveViewer, AppState, SpecializedViewerCacheProvenance};

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
    /// Clear execution and viewer state tied to the previous design document.
    ///
    /// Use when replacing the active schematic/project with unrelated design
    /// content. File identity and dirty state are intentionally left to the
    /// caller because open/import/new workflows each own those semantics.
    pub(crate) fn clear_design_execution_context(&mut self) {
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.workspace.netlist_source = None;
        self.workspace.netlist_document = None;
        self.workspace.netlist_descriptor = None;
        self.workspace.netlist_source_path = None;
        self.workspace.netlist_source_dirty = false;
        self.simulation = crate::state::SimulationState::default();
        self.ui.netlist = Default::default();
        self.ui.results_seen_version = 0;
        self.ui.results.clear_project_scoped_state();
        self.dialogs.drc_results = None;
        self.dialogs.drc_checked_version = 0;
        self.dialogs.drc_cycle = None;
        self.clear_all_design_checks();
        self.log_buffer
            .clear_source(crate::diagnostics::LogSource::Drc);
        self.clear_specialized_viewer_data();
    }

    /// Clear user-visible simulation result history and derived result viewers.
    pub(crate) fn clear_simulation_results(&mut self) {
        if self.simulation.has_active_execution() {
            return;
        }
        self.simulation.clear_runs();
        self.clear_specialized_viewer_data();
    }

    /// Clear transient-derived viewer caches without disturbing AC/RF state.
    pub fn clear_transient_specialized_viewer_data(&mut self) {
        self.analysis.fft_state.clear();
        self.analysis
            .eye_diagram_state
            .load_data(crate::analysis::eye_diagram::EyeData::default());
        self.analysis.cache_authority.fft = None;
        self.analysis.cache_authority.eye = None;
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
        self.analysis.cache_authority = Default::default();
    }

    /// Exact immutable result currently selected in the Results workspace.
    pub(crate) fn active_specialized_viewer_cache_provenance(
        &self,
    ) -> Option<SpecializedViewerCacheProvenance> {
        let run = self.simulation.active_run()?;
        let analysis = self.simulation.active_analysis()?;
        Some(SpecializedViewerCacheProvenance::for_analysis(
            run.dataset_id,
            analysis,
        ))
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

    /// Remove mutable viewer data that does not belong to the active result.
    ///
    /// Capability checks reject a mismatch synchronously, so a selection
    /// change cannot expose stale data even before this cleanup runs. The
    /// controller calls this after selection/polling to release the stale
    /// payload and keep every direct cache consumer safe as well.
    pub(crate) fn synchronize_specialized_viewer_cache_authority(&mut self) {
        let active = self.active_specialized_viewer_cache_provenance();
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
            ActiveViewer::Fft,
            ActiveViewer::EyeDiagram,
        ] {
            let retained = self.specialized_viewer_cache_authority(viewer);
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
                .analysis
                .bode_plot_state
                .load_data(crate::analysis::bode::BodeData::new()),
            ActiveViewer::Nyquist => self.analysis.nyquist_state.clear(),
            ActiveViewer::SmithChart => self.analysis.smith_chart_state.clear_traces(),
            ActiveViewer::Histogram => self.analysis.histogram_state.clear(),
            ActiveViewer::Fft => self.analysis.fft_state.clear(),
            ActiveViewer::EyeDiagram => self
                .analysis
                .eye_diagram_state
                .load_data(crate::analysis::eye_diagram::EyeData::default()),
            ActiveViewer::Waveform | ActiveViewer::PoleZero => return,
        }
        self.clear_specialized_viewer_cache_authority(viewer);
    }

    fn specialized_viewer_cache_authority(
        &self,
        viewer: ActiveViewer,
    ) -> Option<SpecializedViewerCacheProvenance> {
        match viewer {
            ActiveViewer::BodePlot => self.analysis.cache_authority.bode,
            ActiveViewer::Nyquist => self.analysis.cache_authority.nyquist,
            ActiveViewer::SmithChart => self.analysis.cache_authority.smith,
            ActiveViewer::Histogram => self.analysis.cache_authority.histogram,
            ActiveViewer::Fft => self.analysis.cache_authority.fft,
            ActiveViewer::EyeDiagram => self.analysis.cache_authority.eye,
            ActiveViewer::Waveform | ActiveViewer::PoleZero => None,
        }
    }

    fn specialized_viewer_cache_authority_mut(
        &mut self,
        viewer: ActiveViewer,
    ) -> &mut Option<SpecializedViewerCacheProvenance> {
        match viewer {
            ActiveViewer::BodePlot => &mut self.analysis.cache_authority.bode,
            ActiveViewer::Nyquist => &mut self.analysis.cache_authority.nyquist,
            ActiveViewer::SmithChart => &mut self.analysis.cache_authority.smith,
            ActiveViewer::Histogram => &mut self.analysis.cache_authority.histogram,
            ActiveViewer::Fft => &mut self.analysis.cache_authority.fft,
            ActiveViewer::EyeDiagram => &mut self.analysis.cache_authority.eye,
            ActiveViewer::Waveform | ActiveViewer::PoleZero => {
                panic!("waveform and pole-zero viewers do not own mutable cache provenance")
            }
        }
    }

    fn specialized_viewer_cache_matches_active(&self, viewer: ActiveViewer) -> bool {
        self.active_specialized_viewer_cache_provenance()
            .is_some_and(|active| self.specialized_viewer_cache_authority(viewer) == Some(active))
    }

    fn specialized_viewer_cache_has_data(&self, viewer: ActiveViewer) -> bool {
        match viewer {
            ActiveViewer::BodePlot => !self.analysis.bode_plot_state.is_empty(),
            ActiveViewer::Nyquist => !self.analysis.nyquist_state.is_empty(),
            ActiveViewer::SmithChart => !self.analysis.smith_chart_state.traces.is_empty(),
            ActiveViewer::Histogram => !self.analysis.histogram_state.is_empty(),
            ActiveViewer::Fft => self.analysis.fft_state.has_data(),
            ActiveViewer::EyeDiagram => self.analysis.eye_diagram_state.trace_count() > 0,
            ActiveViewer::Waveform | ActiveViewer::PoleZero => false,
        }
    }

    /// Resolve whether a viewer can currently be opened with meaningful data.
    pub fn viewer_capability(&self, viewer: ActiveViewer) -> ViewerCapability {
        match viewer {
            ActiveViewer::Waveform => ViewerCapability::available("Always available"),
            ActiveViewer::SmithChart => {
                if !self.specialized_viewer_cache_matches_active(viewer)
                    || self.analysis.smith_chart_state.traces.is_empty()
                {
                    ViewerCapability::unavailable("Requires S-parameter complex traces")
                } else {
                    ViewerCapability::available("Active analysis S-parameter traces loaded")
                }
            }
            ActiveViewer::EyeDiagram => {
                if self.specialized_viewer_cache_matches_active(viewer)
                    && self.analysis.eye_diagram_state.trace_count() > 0
                {
                    ViewerCapability::available("Active analysis eye traces loaded")
                } else if self.active_analysis_supports_eye_diagram() {
                    ViewerCapability::available(
                        "Can derive eye data from active transient waveforms",
                    )
                } else {
                    ViewerCapability::unavailable("Requires transient eye traces")
                }
            }
            ActiveViewer::Histogram => {
                if !self.specialized_viewer_cache_matches_active(viewer)
                    || self.analysis.histogram_state.is_empty()
                {
                    ViewerCapability::unavailable("Requires histogram bins from sweep/MC data")
                } else {
                    ViewerCapability::available("Active analysis histogram data loaded")
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
                    ViewerCapability::unavailable("Requires complex loop-gain/AC response data")
                } else {
                    ViewerCapability::available("Active analysis Nyquist curves loaded")
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
                if self.specialized_viewer_cache_matches_active(viewer) && has_spectrum {
                    ViewerCapability::available("Active analysis FFT spectrum data loaded")
                } else if self.active_analysis_supports_fft() {
                    ViewerCapability::available("Can derive FFT from active transient waveforms")
                } else {
                    ViewerCapability::unavailable("Requires sampled time-domain data for FFT")
                }
            }
            ActiveViewer::PoleZero => {
                let retained = self.simulation.active_analysis().is_some_and(|analysis| {
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

    /// Whether a viewer is currently available for activation.
    #[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{
        BodeData, EyeData, EyeTrace, FftData, HistogramBuilder, NyquistData, PoleZeroData,
        WindowFunction,
    };
    use crate::diagnostics::{LogSeverity, LogSource};
    use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
    use crate::workbench::app_state::SpecializedViewerCacheAuthority;

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
        state.simulation.runs.push(run);
        assert!(state.simulation.select_run(0));

        state
            .analysis
            .histogram_state
            .load_histogram(HistogramBuilder::new().build(&[1.0, 2.0, 3.0]));

        let mut bode = BodeData::new();
        bode.add_response();
        state.analysis.bode_plot_state.load_data(bode);

        state
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays(
                "old nyquist",
                &[1.0, 10.0],
                &[1.0, -0.5],
                &[0.0, 0.25],
            ));

        state
            .analysis
            .smith_chart_state
            .load_sparam_data("S11", &[1.0], &[0.25], &[0.0]);

        let mut pz = PoleZeroData::new("old pz");
        pz.add_real_pole(-1.0);
        state.analysis.pole_zero_state.load_data(pz);

        let mut eye = EyeData::new(1e-9, 2);
        eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]));
        state.analysis.eye_diagram_state.load_data(eye);

        state
            .analysis
            .fft_state
            .load_data(FftData::from_time_domain(
                "old fft",
                &[0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0],
                8.0,
                WindowFunction::Rectangular,
            ));

        let provenance = state
            .active_specialized_viewer_cache_provenance()
            .expect("seeded active result");
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
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
            .log_buffer
            .log(LogSeverity::Error, LogSource::Drc, "old DRC anchor", None);
    }

    #[test]
    fn clearing_simulation_results_clears_specialized_result_viewers() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);
        assert!(state.simulation.has_results());
        assert!(state.viewer_is_available(ActiveViewer::Histogram));
        assert!(state.viewer_is_available(ActiveViewer::SmithChart));
        assert!(state.viewer_is_available(ActiveViewer::Fft));

        state.clear_simulation_results();

        assert!(!state.simulation.has_results());
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

    #[test]
    fn compatible_run_and_analysis_switches_never_inherit_another_result_cache() {
        let mut state = AppState::default();
        seed_result_viewers(&mut state);

        state.simulation.runs[0].add_analysis(retained_analysis(
            2,
            crate::state::AnalysisType::Ac,
            0x12,
        ));
        let mut second_run = crate::state::SimulationRun::new(2);
        second_run.add_analysis(retained_analysis(1, crate::state::AnalysisType::Ac, 0x21));
        state.simulation.runs.push(second_run);

        assert!(state.simulation.select_analysis(1));
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

        assert!(state.simulation.select_run(1));
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
        state.simulation.runs.push(incompatible);
        assert!(state.simulation.select_run(1));

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
        assert!(state.analysis.bode_plot_state.is_empty());
        assert!(state.analysis.nyquist_state.is_empty());
        assert!(state.analysis.smith_chart_state.traces.is_empty());
        assert!(state.analysis.histogram_state.is_empty());
        assert!(!state.analysis.fft_state.has_data());
        assert_eq!(state.analysis.eye_diagram_state.trace_count(), 0);
        assert_eq!(
            state.analysis.cache_authority,
            SpecializedViewerCacheAuthority::default()
        );
    }

    #[test]
    fn pole_zero_capability_uses_only_the_active_retained_payload() {
        let mut state = AppState::default();
        let mut stale = PoleZeroData::new("stale cache");
        stale.add_real_pole(-99.0);
        state.analysis.pole_zero_state.load_data(stale);

        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::PoleZero,
            "PZ without retained evidence",
        ));
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(
            !state.viewer_is_available(ActiveViewer::PoleZero),
            "an unbound viewer cache must never enable the pole-zero viewer"
        );

        state.simulation.runs[0].analyses[0] = crate::state::AnalysisResult::new(
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
            gain: 3.5,
        });
        state.analysis.pole_zero_state.clear();

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

        state.clear_design_execution_context();

        assert!(state.dialogs.drc_results.is_none());
        assert!(state.dialogs.drc_cycle.is_none());
        assert!(state.current_blocking_drc_result().is_none());
        assert_eq!(
            state.log_buffer.entries_by_source(LogSource::Drc).count(),
            0
        );
    }

    #[test]
    fn clearing_design_execution_context_clears_project_scoped_results_state() {
        use crate::workbench::documents::result_document::{
            AnalysisPresentationKey, ExprEditor, ExprSeries, ExprTrace, PlotView, ResultViewer,
        };

        let mut state = AppState::default();
        let analysis = crate::state::AnalysisResult::new(
            71,
            crate::state::AnalysisType::Transient,
            "Transient",
        );
        let analysis_key =
            AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        state.ui.results_seen_version = 99;
        let results = &mut state.ui.results;
        results.viewer = ResultViewer::Fft;
        results.phase_continuous = true;
        results.cursors.place(1.0);
        results.cursor_strip = Some(0);
        results.hidden_strips.insert(analysis_key);
        results.maximized_strip = Some(analysis_key);
        *results.plot_view_mut(ResultViewer::Waves, 0) = PlotView {
            x: Some((0.0, 1.0)),
            y: Some((-1.0, 1.0)),
        };
        results.analysis_exprs.insert(
            analysis_key,
            vec![ExprTrace {
                text: "V(out)/V(in)".to_string(),
                visible: true,
            }],
        );
        results.expr_editor = Some(ExprEditor {
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
        results.rf_pin.insert(ResultViewer::Smith, (0, 1));
        results.op_filter = "M1".to_string();
        results.op_sort = Some(("gm".to_string(), true));
        results.spec_drafts = Some(Vec::new());

        state.clear_design_execution_context();
        let results = &state.ui.results;

        assert_eq!(state.ui.results_seen_version, 0);
        assert_eq!(results.viewer, ResultViewer::Fft);
        assert!(results.phase_continuous);
        assert!(!results.cursors.any());
        assert_eq!(results.cursor_strip, None);
        assert!(results.hidden_strips.is_empty());
        assert_eq!(results.maximized_strip, None);
        assert!(results.views.is_empty());
        assert!(results.analysis_exprs.is_empty());
        assert!(results.expr_editor.is_none());
        assert!(results.expr_cache.is_empty());
        assert!(results.analysis_expr_cache.is_empty());
        assert!(results.rf_pin.is_empty());
        assert!(results.op_filter.is_empty());
        assert_eq!(results.op_sort, None);
        assert!(results.spec_drafts.is_none());
    }
}
