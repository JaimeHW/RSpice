//! Result viewer interaction state and retained presentation mutations.

use super::*;
use rspice_results_ui::derived::DerivedSeries;

impl ResultsState {
    pub fn clear_project_scoped_state(&mut self) {
        self.session.clear_project_scoped_state();
        *self = Self {
            session: std::mem::take(&mut self.session),
            ..Self::default()
        };
    }

    pub(crate) fn clear_design_scoped_state(&mut self) {
        self.session.clear_design_scoped_state();
        *self = Self {
            session: std::mem::take(&mut self.session),
            ..Self::default()
        };
    }

    /// Return the selection only while it resolves to the exact active
    /// retained dataset and source waveform that created it.
    pub fn valid_selected_trace<'a>(
        &'a self,
        simulation: &crate::state::SimulationState,
    ) -> Option<&'a SelectedResultTrace> {
        let selected = self.session.selected_trace.as_ref()?;
        let run = simulation.active_run()?;
        selected.resolve(run).map(|_| selected)
    }

    /// Take the schematic cross-probe's trace: drop the one it selected
    /// before while that is still the selection, so a trace chosen by hand
    /// since stays, then select `trace` when there is one.
    pub fn project_cross_probe_trace(
        &mut self,
        released: Option<SelectedResultTrace>,
        trace: Option<SelectedResultTrace>,
    ) {
        if released.is_some() && self.session.selected_trace == released {
            self.session.selected_trace = None;
        }
        if let Some(trace) = trace {
            self.session.selected_trace = Some(trace);
        }
    }

    pub(super) fn set_sample_selection(&mut self, selection: Option<SourceSampleSelection>) {
        let current = self
            .session
            .sample_selection
            .as_ref()
            .map(SourceSampleSelection::fingerprint);
        let next = selection.as_ref().map(SourceSampleSelection::fingerprint);
        if current != next {
            self.models.invalidate();
            self.analysis_expr_cache.clear();
            self.session.cache.invalidate();
            self.session.derived = DerivedSeries::default();
            self.session.clear_cursors();
            self.session.hidden_family_traces.clear();
        }
        self.session.sample_selection = selection;
    }

    pub(super) fn toggle_family_trace_visibility(&mut self, key: waves::FamilyTraceVisibilityKey) {
        if !self.session.hidden_family_traces.insert(key) {
            self.session.hidden_family_traces.remove(&key);
        }
        self.models.invalidate();
        self.session.cache.invalidate();
        self.session.derived = DerivedSeries::default();
        self.session.clear_cursors();
    }

    /// Reveal a stored output's members through presentation state only.
    pub fn reveal_waveforms(
        &mut self,
        waveforms: impl IntoIterator<Item = (SourceWaveformPresentationKey, bool)>,
    ) {
        let mut changed = false;
        for (key, dataset_default) in waveforms {
            if !self.session.waveform_visibility(&key, dataset_default) {
                if dataset_default {
                    self.session.waveform_visibility.remove(&key);
                } else {
                    self.session.waveform_visibility.insert(key.clone(), true);
                }
                changed = true;
            }
            self.session.note_recent_signal(key);
        }
        if changed {
            self.models.invalidate();
            self.session.cache.invalidate();
            self.session.derived = DerivedSeries::default();
            self.session.clear_cursors();
        }
    }

    pub(super) fn toggle_waveform_visibility(
        &mut self,
        key: SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        let visible = !self.session.waveform_visibility(&key, dataset_default);
        // A hide is kept even when it restores the dataset's default: a pane
        // whose signal the reader plotted and then hid keeps its header, where
        // the signal waits to be shown again.
        if visible && visible == dataset_default {
            self.session.waveform_visibility.remove(&key);
        } else {
            self.session.waveform_visibility.insert(key, visible);
        }
        self.models.invalidate();
        self.session.cache.invalidate();
        self.session.derived = DerivedSeries::default();
        self.session.clear_cursors();
        visible
    }

    /// Forget the reader's hide of a source the dataset hides by default, so
    /// it is unplotted as it was before the reader plotted it: a signal taken
    /// off the plot from the browser leaves no pane behind for itself.
    pub(super) fn forget_waveform_hide(
        &mut self,
        key: &SourceWaveformPresentationKey,
        dataset_default: bool,
    ) {
        if !dataset_default && self.session.waveform_visibility.get(key) == Some(&false) {
            self.session.waveform_visibility.remove(key);
            self.models.invalidate();
            self.session.cache.invalidate();
        }
    }

    /// Replace quick-view visibility for one analysis with a document-owned
    /// projection. Reconciliation is idempotent so rendering the same document
    /// on another frame does not invalidate waveform caches.
    pub(crate) fn project_waveform_visibility(
        &mut self,
        analysis: AnalysisPresentationKey,
        traces: impl IntoIterator<Item = (String, bool, bool)>,
    ) {
        let desired = traces
            .into_iter()
            .filter_map(|(source_name, dataset_default, visible)| {
                (visible != dataset_default).then(|| {
                    (
                        SourceWaveformPresentationKey::new(analysis, source_name),
                        visible,
                    )
                })
            })
            .collect::<HashMap<_, _>>();
        let unchanged = self
            .session
            .waveform_visibility
            .iter()
            .filter(|(key, _)| key.analysis() == analysis)
            .count()
            == desired.len()
            && desired
                .iter()
                .all(|(key, visible)| self.session.waveform_visibility.get(key) == Some(visible));
        if unchanged {
            return;
        }
        self.session
            .waveform_visibility
            .retain(|key, _| key.analysis() != analysis);
        self.session.waveform_visibility.extend(desired);
        self.models.invalidate();
        self.session.cache.invalidate();
        self.session.derived = DerivedSeries::default();
        self.session.clear_cursors();
    }

    /// Both frame preparation and direct plot/readout actions cross this
    /// boundary. A repeated display version cannot authorize cached data from
    /// a restored history, and nested source edits cannot bypass invalidation.
    pub(super) fn synchronize_wave_caches(&mut self, simulation: &crate::state::SimulationState) {
        let source = (
            simulation.retained.runs.revision(),
            simulation.view.data_version,
        );
        if self.wave_cache_source.as_ref() == Some(&source) {
            return;
        }
        self.wave_cache_source = Some(source);
        self.models.invalidate();
        self.session.cache.invalidate();
        self.session.derived = DerivedSeries::default();
        self.session
            .derived
            .ensure_version(simulation.view.data_version);
        self.analysis_expr_cache.clear();
    }

    pub(super) fn reconcile_expression_projection(
        &mut self,
        simulation: &crate::state::SimulationState,
    ) {
        let Some(run) = simulation.active_run() else {
            self.session.exprs.clear();
            self.session.expr_projection_keys.clear();
            return;
        };
        let current = run
            .analyses
            .iter()
            .enumerate()
            .map(|(index, analysis)| {
                (
                    index,
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                )
            })
            .collect::<Vec<_>>();

        // An ordinal row may be imported only while it still names the
        // identity it named when this projection was emitted. A reorder
        // therefore cannot relabel an expression as belonging to the new
        // occupant of the same slot.
        for (index, key) in &current {
            let may_import = self
                .session
                .expr_projection_keys
                .get(index)
                .is_none_or(|projected| projected == key);
            if may_import && let Some(projected) = self.session.exprs.get(index) {
                self.session.analysis_exprs.insert(*key, projected.clone());
            }
        }
        self.session.exprs = current
            .iter()
            .filter_map(|(index, key)| {
                self.session
                    .analysis_exprs
                    .get(key)
                    .cloned()
                    .map(|exprs| (*index, exprs))
            })
            .collect();
        self.session.expr_projection_keys = current.into_iter().collect();
    }

    pub(crate) fn expression_entries_for_analysis(
        &mut self,
        simulation: &crate::state::SimulationState,
        analysis: AnalysisPresentationKey,
    ) -> Vec<(ResultExpressionPresentationKey, ExprTrace)> {
        self.reconcile_expression_projection(simulation);
        self.session
            .analysis_exprs
            .get(&analysis)
            .map(|traces| {
                traces
                    .iter()
                    .cloned()
                    .map(|trace| {
                        (
                            ResultExpressionPresentationKey::new(analysis, trace.text.clone()),
                            trace,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn add_expression_trace(
        &mut self,
        simulation: &crate::state::SimulationState,
        key: AnalysisPresentationKey,
        text: String,
    ) -> Result<bool, String> {
        self.reconcile_expression_projection(simulation);
        let run = simulation.active_run().ok_or_else(|| {
            "Select a retained result dataset before plotting an expression.".to_owned()
        })?;
        let (analysis_index, _) = key
            .resolve(run)
            .ok_or_else(|| "The selected result analysis is no longer retained.".to_owned())?;
        let traces = self.session.analysis_exprs.entry(key).or_default();
        if let Some(trace) = traces.iter_mut().find(|trace| trace.text == text) {
            // Explicitly re-evaluating a historical expression adopts the
            // current interpretation. Merely opening the document does not.
            let upgraded = trace.complex_policy.is_legacy();
            trace.complex_policy = crate::state::ComplexExpressionPolicy::Rectangular;
            self.session.sync_expression_projection(key, analysis_index);
            return Ok(upgraded);
        }
        traces.push(ExprTrace {
            text,
            complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
            visible: true,
        });
        self.session.sync_expression_projection(key, analysis_index);
        Ok(true)
    }

    pub(crate) fn toggle_expression_visibility_by_key(
        &mut self,
        simulation: &crate::state::SimulationState,
        key: &ResultExpressionPresentationKey,
    ) -> Result<(), String> {
        self.reconcile_expression_projection(simulation);
        let run = simulation.active_run().ok_or_else(|| {
            "The expression's retained dataset is no longer available.".to_owned()
        })?;
        let (analysis_index, _) = key.analysis().resolve(run).ok_or_else(|| {
            "The expression's retained analysis is no longer available.".to_owned()
        })?;
        let traces = self
            .session
            .analysis_exprs
            .get_mut(&key.analysis())
            .ok_or_else(|| "The selected expression no longer exists.".to_owned())?;
        let mut matches = traces.iter_mut().filter(|trace| trace.text == key.text());
        let trace = matches
            .next()
            .filter(|_| matches.next().is_none())
            .ok_or_else(|| "The selected expression no longer resolves uniquely.".to_owned())?;
        trace.visible = !trace.visible;
        self.session
            .sync_expression_projection(key.analysis(), analysis_index);
        Ok(())
    }

    pub(super) fn dataset_is_retained(
        simulation: &crate::state::SimulationState,
        analysis: AnalysisPresentationKey,
    ) -> bool {
        simulation
            .retained
            .runs
            .iter()
            .any(|run| run.dataset_id == analysis.dataset_id())
    }

    /// Capture every authored quick-view field about retained datasets.
    pub(crate) fn project_presentation(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> ResultPresentation {
        ResultPresentation {
            markers: self.project_markers(simulation),
            marker_id_high_water: Some(self.session.marker_id_high_water()),
            log_y_panes: self.project_log_y_panes(simulation),
            expression_groups: self.project_expression_groups(simulation),
            signal_sets: self.signal_sets.clone(),
            x_scales: {
                // While a document pane shows its own X scale, the project
                // keeps the quick view's, set aside until it is left.
                let mut scales: Vec<_> = self
                    .session
                    .quick_view_presentation
                    .as_ref()
                    .map_or(&self.session.x_scales, |quick| &quick.x_scales)
                    .iter()
                    .filter(|(analysis, _)| Self::dataset_is_retained(simulation, **analysis))
                    .map(
                        |(analysis, scale)| rspice_results::result_presentation::StripXScale {
                            analysis: *analysis,
                            logarithmic: *scale == crate::ui::plot::XScale::Log10,
                        },
                    )
                    .collect();
                scales.sort_by_key(|scale| scale.analysis.order_key());
                scales
            },
            xy_plots: {
                // Likewise the quick view's strips, not a document's, in the
                // order the reader made them.
                self.session
                    .quick_view_presentation
                    .as_ref()
                    .map_or(&self.session.xy_strips, |quick| &quick.xy_strips)
                    .iter()
                    .filter(|strip| Self::dataset_is_retained(simulation, strip.analysis))
                    .map(|strip| rspice_results::result_presentation::StripXyPlot {
                        analysis: strip.analysis,
                        unit: strip.unit.clone(),
                        abscissa: strip.abscissa.clone(),
                        sweep_range: strip.sweep_range.and_then(|(minimum, maximum)| {
                            rspice_results::visualization_document::AxisRange::new(minimum, maximum)
                                .ok()
                        }),
                        follow_sweep_view: strip.follow_view,
                        log_abscissa: strip.log_x,
                    })
                    .collect()
            },
        }
    }

    /// The markers this project saves: the reader's own, about datasets the
    /// project still holds.
    pub(crate) fn project_markers(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> Vec<ResultMarker> {
        self.session
            .markers
            .iter()
            .filter(|marker| Self::dataset_is_retained(simulation, marker.analysis))
            .cloned()
            .collect()
    }

    /// The logarithmic-axis choices this project saves.
    pub(crate) fn project_log_y_panes(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> Vec<WavePanePresentationKey> {
        let mut panes = self
            .session
            .log_y_panes
            .iter()
            .filter(|pane| Self::dataset_is_retained(simulation, pane.analysis))
            .cloned()
            .collect();
        crate::state::result_presentation::canonicalize_log_y_panes(&mut panes);
        panes
    }

    /// Deterministic project projection of every stable expression trace.
    pub fn project_expression_groups(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> Vec<ResultExpressionGroup> {
        // While a document pane shows its own expressions, the project
        // keeps the quick view's, which are set aside until it is left.
        let quick = self.session.quick_view_presentation.as_ref();
        let units = quick.map_or(&self.session.expression_units, |quick| {
            &quick.expression_units
        });
        let mut stable = quick.map_or_else(
            || self.session.analysis_exprs.clone(),
            |quick| quick.analysis_exprs.clone(),
        );
        if let Some(run) = simulation.active_run().filter(|_| quick.is_none()) {
            for (analysis_index, analysis) in run.analyses.iter().enumerate() {
                let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
                let may_import = self
                    .session
                    .expr_projection_keys
                    .get(&analysis_index)
                    .is_none_or(|projected| *projected == key);
                if may_import && let Some(projected) = self.session.exprs.get(&analysis_index) {
                    stable.insert(key, projected.clone());
                }
            }
        }
        // Retention discards datasets; a group naming one the project no
        // longer holds would be written out only to fail its own resolvability
        // check on the next load.
        let mut groups = stable
            .iter()
            .filter(|(analysis, traces)| {
                !traces.is_empty() && Self::dataset_is_retained(simulation, **analysis)
            })
            .map(|(analysis, traces)| ResultExpressionGroup {
                analysis: *analysis,
                traces: traces.clone(),
                units: traces
                    .iter()
                    .filter_map(|trace| {
                        let unit = units.get(&(*analysis, trace.text.clone()))?;
                        Some(
                            rspice_results::result_presentation::DeclaredExpressionUnit {
                                text: trace.text.clone(),
                                unit: unit.clone(),
                            },
                        )
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| left.analysis.order_key().cmp(&right.analysis.order_key()));
        groups
    }

    /// Release discarded local state and source-derived caches together.
    pub(crate) fn retain_datasets(&mut self, retained: &HashSet<DatasetId>) {
        self.retained_history_revision = None;
        self.session.retain_datasets(retained);
        let live = |analysis: AnalysisPresentationKey| retained.contains(&analysis.dataset_id());
        if self
            .event_order_cache
            .as_ref()
            .is_some_and(|cache| !live(cache.analysis))
        {
            self.event_order_cache = None;
        }
        self.network_matrix.retain_datasets(retained);
        self.analysis_expr_cache
            .retain(|(analysis, _), _| live(*analysis));
        self.retained_evidence_validity
            .retain(|analysis, _| live(*analysis));
        self.dataset_digests
            .retain(|dataset, _| retained.contains(dataset));
        self.noise_spectrum_shapes
            .retain(|analysis, _| live(*analysis));
        self.structural_gates
            .retain(|(analysis, _), _| live(*analysis));
        self.plans = view_plans::ViewPlans::default();
    }

    /// Bring dataset-scoped presentation state in step with the retained
    /// history, at whatever moment the workspace next looks at it.
    ///
    /// Pruning happens in several places inside the simulation state, which
    /// has no reach into presentation state at all. Reconciling against the
    /// retained set here gives that one owner. Unchanged history is checked
    /// through its mutation revision without allocating another dataset set.
    pub fn reconcile_retained_datasets(&mut self, simulation: &crate::state::SimulationState) {
        let revision = simulation.retained.runs.revision();
        if self.retained_history_revision.as_ref() == Some(&revision) {
            return;
        }
        frame_work::note(frame_work::DatasetWalk::RetainedHistoryScan);
        let retained: HashSet<DatasetId> = simulation
            .retained
            .runs
            .iter()
            .map(|run| run.dataset_id)
            .collect();
        if retained == self.retained_datasets {
            self.retained_history_revision = Some(revision);
            return;
        }
        self.retain_datasets(&retained);
        self.retained_datasets = retained;
        self.retained_history_revision = Some(revision);
    }

    /// The standing reason one document's Latest retarget onto this exact
    /// candidate was refused, if it already was.
    pub fn latest_retarget_failure(
        &self,
        document_id: crate::product::ResultDocumentId,
        candidate: DatasetId,
    ) -> Option<&str> {
        self.latest_retarget_failures
            .get(&document_id)
            .filter(|(failed, _)| *failed == candidate)
            .map(|(_, reason)| reason.as_str())
    }

    pub(crate) fn record_latest_retarget_failure(
        &mut self,
        document_id: crate::product::ResultDocumentId,
        candidate: DatasetId,
        reason: String,
    ) {
        self.latest_retarget_failures
            .insert(document_id, (candidate, reason));
    }

    /// Why the archived run behind `binding` could not be restored, if it
    /// already could not.
    pub(crate) fn archive_restore_failure(
        &self,
        binding: crate::product::DatasetBinding,
    ) -> Option<&str> {
        self.archive_restore_failures
            .get(&binding)
            .map(String::as_str)
    }

    pub(crate) fn record_archive_restore_failure(
        &mut self,
        binding: crate::product::DatasetBinding,
        reason: String,
    ) {
        self.archive_restore_failures.insert(binding, reason);
    }

    pub(crate) fn clear_latest_retarget_failure(
        &mut self,
        document_id: crate::product::ResultDocumentId,
    ) {
        self.latest_retarget_failures.remove(&document_id);
    }

    /// Restore a strip addressed by the current run's transient ordinal.
    /// Command/search integrations may still produce an ordinal, but the
    /// retained presentation state is removed by its stable identity.
    pub fn restore_analysis_strip(
        &mut self,
        simulation: &crate::state::SimulationState,
        analysis_index: usize,
    ) {
        let Some(run) = simulation.active_run() else {
            return;
        };
        let Some(analysis) = run.analyses.get(analysis_index) else {
            return;
        };
        self.session
            .hidden_strips
            .remove(&AnalysisPresentationKey::new(run.dataset_id, analysis));
    }
}

/// The project lifecycle captures the Results presentation it saves through
/// this.
impl rspice_app_services::workbench::lifecycle::project_lifecycle::ResultPresentationSource
    for ResultsState
{
    fn project_presentation(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> ResultPresentation {
        Self::project_presentation(self, simulation)
    }
}
