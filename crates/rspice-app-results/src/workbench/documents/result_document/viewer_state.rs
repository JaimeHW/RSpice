//! Result viewer interaction state and retained presentation mutations.

use super::*;
use crate::results::visualization_document::TraceColor;
use rspice_results::result_presentation::SignalSet;
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

    /// The shown window's own Results state, kept for it while another
    /// window is drawn and saved with the session.
    pub fn window_view(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> rspice_results_ui::session::WindowResultsView {
        let run = simulation.active_run().map(|run| run.dataset_id);
        let mut view = self
            .session
            .window_view(run, simulation.view.active_analysis_idx);
        view.quick_view_viewer = self.quick_view_viewer;
        view
    }

    /// Put a window's own Results state back on screen: the analysis it
    /// shows of the run its document shows, its sheet, cursors, zoom and
    /// selection. Going back to a window is not a change of what the reader
    /// looks at in it, so nothing placed there is retired for it.
    pub fn show_window_view(
        &mut self,
        simulation: &mut crate::state::SimulationState,
        view: &rspice_results_ui::session::WindowResultsView,
    ) {
        let run = view
            .run
            .and_then(|dataset| {
                simulation
                    .retained
                    .runs
                    .iter()
                    .position(|run| run.dataset_id == dataset)
            })
            .or(simulation.view.active_run_idx);
        if let Some(run) = run {
            let shown = view
                .analysis
                .is_some_and(|analysis| simulation.select_run_analysis(run, analysis));
            if !shown {
                simulation.select_run(run);
            }
        }
        self.session.show_window_view(view);
        self.quick_view_viewer = view.quick_view_viewer;
        self.seen_selection = shown_selection(simulation);
        if let Some(kind) = self.seen_selection.and_then(|shown| shown.kind) {
            self.cursor_kind = Some(kind);
        }
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
        // Hiding a member changes what is drawn, not any trace's samples: the
        // cursors stay, and every cached series stays good for its trace.
        self.models.invalidate();
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
        // Hiding or showing a trace keeps the A/B cursors, and the cached
        // series of every other trace: each is keyed by its own trace, so the
        // one shown again is the only one built.
        self.models.invalidate();
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
    }

    /// Draw `analysis`'s traces in the colours the view being drawn gives
    /// them, by signal, and every other in its own. Idempotent, like
    /// [`Self::project_waveform_visibility`].
    pub(crate) fn project_waveform_colors(
        &mut self,
        analysis: AnalysisPresentationKey,
        colors: impl IntoIterator<Item = (String, TraceColor)>,
    ) {
        let desired = colors
            .into_iter()
            .map(|(signal, color)| (SourceWaveformPresentationKey::new(analysis, signal), color))
            .collect::<HashMap<_, _>>();
        let colors = &mut self.session.waveform_colors;
        let unchanged = colors
            .iter()
            .filter(|(key, _)| key.analysis() == analysis)
            .count()
            == desired.len()
            && desired
                .iter()
                .all(|(key, color)| colors.get(key) == Some(color));
        if unchanged {
            return;
        }
        colors.retain(|key, _| key.analysis() != analysis);
        colors.extend(desired);
        self.models.invalidate();
    }

    /// Draw `analysis`' traces in the styles and names the view being drawn
    /// gives them, by signal, and every other as its palette place draws
    /// it. Idempotent, like [`Self::project_waveform_colors`].
    pub(crate) fn project_waveform_looks(
        &mut self,
        analysis: AnalysisPresentationKey,
        looks: impl IntoIterator<Item = (String, rspice_results_ui::session::WaveformLook)>,
    ) {
        let desired = looks
            .into_iter()
            .filter(|(_, look)| *look != rspice_results_ui::session::WaveformLook::default())
            .map(|(signal, look)| (SourceWaveformPresentationKey::new(analysis, signal), look))
            .collect::<HashMap<_, _>>();
        let looks = &mut self.session.waveform_looks;
        let unchanged = looks
            .iter()
            .filter(|(key, _)| key.analysis() == analysis)
            .count()
            == desired.len()
            && desired
                .iter()
                .all(|(key, look)| looks.get(key) == Some(look));
        if unchanged {
            return;
        }
        looks.retain(|key, _| key.analysis() != analysis);
        looks.extend(desired);
        self.models.invalidate();
    }

    /// Draw `key`'s signal with `look` from this frame on: the view that
    /// keeps it was just given the style or the name.
    pub(super) fn draw_waveform_look(
        &mut self,
        key: SourceWaveformPresentationKey,
        look: rspice_results_ui::session::WaveformLook,
    ) {
        let changed = if look == rspice_results_ui::session::WaveformLook::default() {
            self.session.waveform_looks.remove(&key).is_some()
        } else {
            self.session.waveform_looks.insert(key, look.clone()) != Some(look)
        };
        if changed {
            self.models.invalidate();
        }
    }

    /// Draw `key`'s signal in `color` from this frame on, or with `None` in
    /// its own colour: the view that keeps it was just given the colour.
    pub(super) fn draw_waveform_color(
        &mut self,
        key: SourceWaveformPresentationKey,
        color: Option<TraceColor>,
    ) {
        let changed = match color {
            Some(color) => self.session.waveform_colors.insert(key, color) != Some(color),
            None => self.session.waveform_colors.remove(&key).is_some(),
        };
        if changed {
            self.models.invalidate();
        }
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
        // Only a change to the history changes samples. A new selection or
        // overlay moves the data version alone, and the decimated and derived
        // series, each keyed by its run, analysis and trace, still hold.
        let samples_changed = self
            .wave_cache_source
            .as_ref()
            .is_none_or(|(revision, _)| *revision != source.0);
        self.wave_cache_source = Some(source);
        self.models.invalidate();
        if samples_changed {
            self.samples_generation = self.samples_generation.wrapping_add(1);
            self.session.cache.invalidate();
            self.session.derived = DerivedSeries::default();
        }
        self.session.derived.ensure_version(self.samples_generation);
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

    /// What the project keeps of Results beside its visualization
    /// documents: the reader's signal sets. What the reader made of a run is
    /// its view's.
    pub(crate) fn saved_signal_sets(&self) -> Vec<SignalSet> {
        self.signal_sets.clone()
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

/// The project lifecycle captures the signal sets it saves through this.
impl rspice_app_services::workbench::lifecycle::project_lifecycle::SignalSetSource
    for ResultsState
{
    fn saved_signal_sets(&self) -> Vec<SignalSet> {
        Self::saved_signal_sets(self)
    }
}

#[cfg(test)]
mod tests;
