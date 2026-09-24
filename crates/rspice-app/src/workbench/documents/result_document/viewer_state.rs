//! Result viewer interaction state and retained presentation mutations.

use super::*;

impl ResultsState {
    /// Return the selection only while it resolves to the exact active
    /// retained dataset and source waveform that created it.
    pub(crate) fn valid_selected_trace<'a>(
        &'a self,
        simulation: &crate::state::SimulationState,
    ) -> Option<&'a SelectedResultTrace> {
        let selected = self.selected_trace.as_ref()?;
        let run = simulation.active_run()?;
        selected.resolve(run).map(|_| selected)
    }

    /// Clear cursors (Esc, Clear action).
    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.cursor_strip = None;
        self.cursor_a_anchor = None;
    }

    /// Arm or disarm the A│B tool. Disarming clears the pair, because a
    /// cursor nobody can move or read is not a cursor.
    pub fn toggle_cursor_tool(&mut self) {
        self.cursor_tool = CursorTool(!self.cursor_tool.is_armed());
        if !self.cursor_tool.is_armed() {
            self.clear_cursors();
        }
    }

    /// `true` when the cursor readout has something to say: the tool is
    /// armed and at least cursor A is placed.
    pub fn cursor_readout_active(&self) -> bool {
        self.cursor_tool.is_armed() && self.cursors.any()
    }

    /// Whether an ordinary plot click should place A/B. Box-zoom and pan own
    /// their primary drag/click gestures even while existing cursors remain
    /// visible in the readout strip.
    pub(in crate::workbench::documents) fn cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::Cursor && self.cursor_tool.is_armed()
    }

    pub(in crate::workbench::documents) fn horizontal_cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::HorizontalCursor
    }

    pub(in crate::workbench::documents) fn cursor_a_is_next(&self) -> bool {
        self.cursors.a.is_none() || self.cursors.b.is_some()
    }

    pub fn toggle_linked_cursors(&mut self) {
        self.linked_cursors = !self.linked_cursors;
    }

    /// Arm or disarm the marker tool. Disarming keeps the markers — they are
    /// document content, not a transient readout like the A/B pair.
    pub fn toggle_marker_tool(&mut self) {
        self.marker_tool = MarkerTool(!self.marker_tool.is_armed());
    }

    /// Place a valid marker without reusing an identity in this live project.
    pub fn add_marker(
        &mut self,
        analysis: AnalysisPresentationKey,
        anchor: WaveformPresentationKey,
        trace_name: String,
        x: f64,
    ) -> Result<u32, String> {
        ResultMarker::validate_placement(analysis, &anchor, x)?;
        let id = self.marker_id_high_water().max(self.next_marker_id).checked_add(1)
            .ok_or("result marker identity space is exhausted; existing markers can still be edited or removed")?;
        self.next_marker_id = id;
        self.marker_allocation_high_water = id;
        self.markers.push(ResultMarker {
            id,
            analysis,
            anchor,
            trace_name,
            x,
            kind: MarkerKind::default(),
            note: String::new(),
        });
        Ok(id)
    }

    pub fn marker_mut(&mut self, id: u32) -> Option<&mut ResultMarker> {
        self.markers.iter_mut().find(|marker| marker.id == id)
    }

    /// Remove one quick marker, and with it any open edit of it.
    pub fn remove_marker(&mut self, id: u32) {
        self.remember_marker_ids();
        self.markers.retain(|marker| marker.id != id);
        if self
            .marker_edit
            .as_ref()
            .is_some_and(|draft| draft.selector == MarkerSelector::Quick(id))
        {
            self.marker_edit = None;
        }
    }

    /// Adopt markers restored from a project, keeping the id allocator ahead
    /// of every label already in use.
    pub(crate) fn adopt_markers(&mut self, markers: Vec<ResultMarker>, high_water: u32) {
        let live_high_water = self.next_marker_id.max(self.marker_id_high_water());
        self.marker_allocation_high_water = markers
            .iter()
            .map(|marker| marker.id)
            .max()
            .unwrap_or(0)
            .max(high_water);
        self.next_marker_id = live_high_water.max(self.marker_allocation_high_water);
        self.markers = markers;
        self.marker_edit = None;
    }

    pub(super) fn marker_id_high_water(&self) -> u32 {
        self.marker_allocation_high_water.max(
            self.markers
                .iter()
                .map(|marker| marker.id)
                .max()
                .unwrap_or(0),
        )
    }

    /// Retain even IDs inserted through the legacy mutable marker collection
    /// before removing the last evidence of them.
    pub(super) fn remember_marker_ids(&mut self) {
        self.marker_allocation_high_water = self.marker_id_high_water();
        self.next_marker_id = self.next_marker_id.max(self.marker_allocation_high_water);
    }

    /// Replace the overlay with one persistent pane's retained markers.
    ///
    /// This writes nothing into the quick-view store and never touches the
    /// quick id allocator: a retained marker's serial belongs to its document,
    /// and letting it advance `next_marker_id` made the two stores contend for
    /// one integer space.
    pub(crate) fn project_document_markers(&mut self, markers: Vec<DocumentMarker>) {
        self.document_markers = markers;
        if let Some(draft) = &self.marker_edit
            && let MarkerSelector::Document { marker_id, .. } = draft.selector
            && !self
                .document_markers
                .iter()
                .any(|marker| marker.retained_id == marker_id)
        {
            // The marker vanished under the dialog: the draft has nothing to
            // apply to and must not linger.
            self.marker_edit = None;
        }
    }

    /// One retained overlay marker by its document serial.
    pub(crate) fn document_marker(&self, marker_id: MarkerId) -> Option<&DocumentMarker> {
        self.document_markers
            .iter()
            .find(|marker| marker.retained_id == marker_id)
    }

    /// How many markers the reader can currently act on, across both stores.
    pub(crate) fn context_marker_count(&self) -> usize {
        self.markers.len() + self.document_markers.len()
    }

    /// Markers on one strip, in placement order, from both stores.
    ///
    /// Retained document markers come last so they paint over the quick ones:
    /// on a persistent pane the document's own annotation is the authority.
    /// The overlay is additionally verified against the live pane context, so
    /// a stale projection cannot leak a document's markers onto a quick strip.
    pub(crate) fn strip_markers(
        &self,
        analysis: AnalysisPresentationKey,
    ) -> impl Iterator<Item = MarkerView<'_>> {
        let context = self.persistent_pane_context;
        self.markers
            .iter()
            .filter(move |marker| marker.analysis == analysis)
            .map(MarkerView::Quick)
            .chain(
                self.document_markers
                    .iter()
                    .filter(move |marker| {
                        marker.analysis == analysis
                            && context.is_some_and(|context| {
                                context.document_id == marker.document_id
                                    && context.pane_id == marker.pane_id
                            })
                    })
                    .map(MarkerView::Document),
            )
    }

    pub(super) fn set_sample_selection(&mut self, selection: Option<SourceSampleSelection>) {
        let current = self
            .sample_selection
            .as_ref()
            .map(SourceSampleSelection::fingerprint);
        let next = selection.as_ref().map(SourceSampleSelection::fingerprint);
        if current != next {
            self.models.invalidate();
            self.analysis_expr_cache.clear();
            self.cache.invalidate();
            self.derived = DerivedSeries::default();
            self.clear_cursors();
            self.hidden_family_traces.clear();
        }
        self.sample_selection = selection;
    }

    pub(super) fn toggle_family_trace_visibility(&mut self, key: waves::FamilyTraceVisibilityKey) {
        if !self.hidden_family_traces.insert(key) {
            self.hidden_family_traces.remove(&key);
        }
        self.models.invalidate();
        self.cache.invalidate();
        self.derived = DerivedSeries::default();
        self.clear_cursors();
    }

    pub(crate) fn waveform_visibility(
        &self,
        key: &SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        self.waveform_visibility
            .get(key)
            .copied()
            .unwrap_or(dataset_default)
    }

    /// Reveal a stored output's members through presentation state only.
    pub(crate) fn reveal_waveforms(
        &mut self,
        waveforms: impl IntoIterator<Item = (SourceWaveformPresentationKey, bool)>,
    ) {
        let mut changed = false;
        for (key, dataset_default) in waveforms {
            if !self.waveform_visibility(&key, dataset_default) {
                if dataset_default {
                    self.waveform_visibility.remove(&key);
                } else {
                    self.waveform_visibility.insert(key.clone(), true);
                }
                changed = true;
            }
            self.note_recent_signal(key);
        }
        if changed {
            self.models.invalidate();
            self.cache.invalidate();
            self.derived = DerivedSeries::default();
            self.clear_cursors();
        }
    }

    pub(super) fn toggle_waveform_visibility(
        &mut self,
        key: SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        let visible = !self.waveform_visibility(&key, dataset_default);
        if visible == dataset_default {
            self.waveform_visibility.remove(&key);
        } else {
            self.waveform_visibility.insert(key, visible);
        }
        self.models.invalidate();
        self.cache.invalidate();
        self.derived = DerivedSeries::default();
        self.clear_cursors();
        visible
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
            .waveform_visibility
            .iter()
            .filter(|(key, _)| key.analysis == analysis)
            .count()
            == desired.len()
            && desired
                .iter()
                .all(|(key, visible)| self.waveform_visibility.get(key) == Some(visible));
        if unchanged {
            return;
        }
        self.waveform_visibility
            .retain(|key, _| key.analysis != analysis);
        self.waveform_visibility.extend(desired);
        self.models.invalidate();
        self.cache.invalidate();
        self.derived = DerivedSeries::default();
        self.clear_cursors();
    }

    /// Both frame preparation and direct plot/readout actions cross this
    /// boundary. A repeated display version cannot authorize cached data from
    /// a restored history, and nested source edits cannot bypass invalidation.
    pub(super) fn synchronize_wave_caches(&mut self, simulation: &crate::state::SimulationState) {
        let source = (simulation.runs.revision(), simulation.data_version);
        if self.wave_cache_source.as_ref() == Some(&source) {
            return;
        }
        self.wave_cache_source = Some(source);
        self.models.invalidate();
        self.cache.invalidate();
        self.derived = DerivedSeries::default();
        self.derived.ensure_version(simulation.data_version);
        self.analysis_expr_cache.clear();
    }

    pub(super) fn reconcile_expression_projection(
        &mut self,
        simulation: &crate::state::SimulationState,
    ) {
        let Some(run) = simulation.active_run() else {
            self.exprs.clear();
            self.expr_projection_keys.clear();
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
                .expr_projection_keys
                .get(index)
                .is_none_or(|projected| projected == key);
            if may_import && let Some(projected) = self.exprs.get(index) {
                self.analysis_exprs.insert(*key, projected.clone());
            }
        }
        self.exprs = current
            .iter()
            .filter_map(|(index, key)| {
                self.analysis_exprs
                    .get(key)
                    .cloned()
                    .map(|exprs| (*index, exprs))
            })
            .collect();
        self.expr_projection_keys = current.into_iter().collect();
    }

    pub(super) fn sync_expression_projection(
        &mut self,
        analysis: AnalysisPresentationKey,
        analysis_index: usize,
    ) {
        match self.analysis_exprs.get(&analysis).cloned() {
            Some(exprs) => {
                self.exprs.insert(analysis_index, exprs);
            }
            None => {
                self.exprs.remove(&analysis_index);
            }
        }
        self.expr_projection_keys.insert(analysis_index, analysis);
    }

    pub(crate) fn expression_entries_for_analysis(
        &mut self,
        simulation: &crate::state::SimulationState,
        analysis: AnalysisPresentationKey,
    ) -> Vec<(ResultExpressionPresentationKey, ExprTrace)> {
        self.reconcile_expression_projection(simulation);
        self.analysis_exprs
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

    pub(crate) fn add_expression_trace(
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
        let traces = self.analysis_exprs.entry(key).or_default();
        if let Some(trace) = traces.iter_mut().find(|trace| trace.text == text) {
            // Explicitly re-evaluating a historical expression adopts the
            // current interpretation. Merely opening the document does not.
            let upgraded = trace.complex_policy.is_legacy();
            trace.complex_policy = crate::state::ComplexExpressionPolicy::Rectangular;
            self.sync_expression_projection(key, analysis_index);
            return Ok(upgraded);
        }
        traces.push(ExprTrace {
            text,
            complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
            visible: true,
        });
        self.sync_expression_projection(key, analysis_index);
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
            .analysis_exprs
            .get_mut(&key.analysis())
            .ok_or_else(|| "The selected expression no longer exists.".to_owned())?;
        let mut matches = traces.iter_mut().filter(|trace| trace.text == key.text());
        let trace = matches
            .next()
            .filter(|_| matches.next().is_none())
            .ok_or_else(|| "The selected expression no longer resolves uniquely.".to_owned())?;
        trace.visible = !trace.visible;
        self.sync_expression_projection(key.analysis(), analysis_index);
        Ok(())
    }

    pub(super) fn dataset_is_retained(
        simulation: &crate::state::SimulationState,
        analysis: AnalysisPresentationKey,
    ) -> bool {
        simulation
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
            marker_id_high_water: Some(self.marker_id_high_water()),
            log_y_panes: self.project_log_y_panes(simulation),
            expression_groups: self.project_expression_groups(simulation),
        }
    }

    /// The markers this project saves: the reader's own, about datasets the
    /// project still holds.
    pub(crate) fn project_markers(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> Vec<ResultMarker> {
        self.markers
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
            .log_y_panes
            .iter()
            .filter(|pane| Self::dataset_is_retained(simulation, pane.analysis))
            .cloned()
            .collect();
        crate::state::result_presentation::canonicalize_log_y_panes(&mut panes);
        panes
    }

    /// Deterministic project projection of every stable expression trace.
    pub(crate) fn project_expression_groups(
        &self,
        simulation: &crate::state::SimulationState,
    ) -> Vec<ResultExpressionGroup> {
        let mut stable = self.analysis_exprs.clone();
        if let Some(run) = simulation.active_run() {
            for (analysis_index, analysis) in run.analyses.iter().enumerate() {
                let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
                let may_import = self
                    .expr_projection_keys
                    .get(&analysis_index)
                    .is_none_or(|projected| *projected == key);
                if may_import && let Some(projected) = self.exprs.get(&analysis_index) {
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
            })
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| left.analysis.order_key().cmp(&right.analysis.order_key()));
        groups
    }

    /// Clear design results within the same project, retaining its marker IDs.
    pub(crate) fn clear_design_scoped_state(&mut self) {
        self.remember_marker_ids();
        let next_marker_id = self.next_marker_id;
        let marker_allocation_high_water = self.marker_allocation_high_water;
        self.clear_project_scoped_state();
        self.next_marker_id = next_marker_id;
        self.marker_allocation_high_water = marker_allocation_high_water;
    }

    /// Reset result UI state and identities when replacing the whole project.
    pub fn clear_project_scoped_state(&mut self) {
        let viewer = self.viewer;
        let phase_continuous = self.phase_continuous;
        *self = Self {
            viewer,
            phase_continuous,
            ..Self::default()
        };
    }

    pub(crate) fn persistent_document_page(
        &self,
        document_id: crate::product::ResultDocumentId,
    ) -> Option<crate::results::visualization_document::PageId> {
        self.persistent_document_pages.get(&document_id).copied()
    }

    pub(super) fn select_persistent_document_page(
        &mut self,
        document_id: crate::product::ResultDocumentId,
        page_id: crate::results::visualization_document::PageId,
    ) {
        self.persistent_document_pages.insert(document_id, page_id);
    }

    /// Drop presentation state naming datasets the project no longer retains.
    ///
    /// Retention discards a dataset. Every mark, filter, expression group and
    /// memo the reader made *about* that dataset is then a statement about
    /// something that no longer exists: left in place it accumulates for the
    /// life of the session, and the project file is written with expression
    /// groups and markers that resolve against nothing when it is reopened.
    ///
    /// Two stores are deliberately absent. `views` keys a strip's window by
    /// the authored analysis rather than by one dataset, precisely so the
    /// window survives a re-run; a document-keyed window belongs to its
    /// document. `eye_timebase` keeps its prepared keys for the same reason,
    /// and only its dataset-named legacy keys are pruned.
    ///
    /// The viewer projections are discarded wholesale rather than filtered.
    /// Every plan carries the version and identity it was built from, so a
    /// discarded dataset's plan can never be served — but it is still held,
    /// and `ArtifactTextPlan` holds the complete serialized text of a typed
    /// artifact. Pruning only runs when the retained set actually changed,
    /// and the next frame rebuilds what it needs, so dropping all of them
    /// here costs one rebuild and bounds the session.
    pub(crate) fn retain_datasets(&mut self, retained: &HashSet<DatasetId>) {
        self.retained_history_revision = None;
        self.remember_marker_ids();
        let live = |analysis: AnalysisPresentationKey| retained.contains(&analysis.dataset_id());
        self.markers.retain(|marker| live(marker.analysis));
        self.log_y_panes.retain(|pane| live(pane.analysis));
        self.analysis_exprs.retain(|analysis, _| live(*analysis));
        self.analysis_expr_cache
            .retain(|(analysis, _), _| live(*analysis));
        self.expr_projection_keys
            .retain(|_, analysis| live(*analysis));
        self.hidden_strips.retain(|analysis| live(*analysis));
        self.maximized_strip = self.maximized_strip.filter(|analysis| live(*analysis));
        self.retained_evidence_validity
            .retain(|analysis, _| live(*analysis));
        self.dataset_digests
            .retain(|dataset, _| retained.contains(dataset));
        self.noise_spectrum_shapes
            .retain(|analysis, _| live(*analysis));
        self.structural_gates
            .retain(|(analysis, _), _| live(*analysis));
        self.favorite_signals.retain(|key| live(key.analysis()));
        self.recent_signals.retain(|key| live(key.analysis()));
        self.favorite_result_artifacts
            .retain(|key| live(key.analysis));
        self.recent_result_artifacts
            .retain(|key| live(key.analysis));
        self.checked_result_quantities
            .retain(|key| retained.contains(&key.dataset_id()));
        self.browser_range_anchor = self
            .browser_range_anchor
            .take()
            .filter(|key| retained.contains(&key.dataset_id()));
        self.waveform_visibility
            .retain(|key, _| live(key.analysis()));
        self.hidden_family_traces.retain(|key| live(key.analysis()));
        self.eye_timebase.retain(|key, _| match key {
            crate::analysis::eye_diagram::EyeTimebaseKey::Prepared(_) => true,
            crate::analysis::eye_diagram::EyeTimebaseKey::Legacy(dataset, _) => {
                retained.contains(dataset)
            }
        });
        self.selected_trace = self
            .selected_trace
            .take()
            .filter(|selected| live(selected.analysis_key()));
        self.selected_result_artifact = self
            .selected_result_artifact
            .take()
            .filter(|key| live(key.analysis));
        self.plans = view_plans::ViewPlans::default();
    }

    /// Bring dataset-scoped presentation state in step with the retained
    /// history, at whatever moment the workspace next looks at it.
    ///
    /// Pruning happens in several places inside the simulation state, which
    /// has no reach into presentation state at all. Reconciling against the
    /// retained set here gives that one owner. Unchanged history is checked
    /// through its mutation revision without allocating another dataset set.
    pub(crate) fn reconcile_retained_datasets(
        &mut self,
        simulation: &crate::state::SimulationState,
    ) {
        let revision = simulation.runs.revision();
        if self.retained_history_revision.as_ref() == Some(&revision) {
            return;
        }
        frame_work::note(frame_work::DatasetWalk::RetainedHistoryScan);
        let retained: HashSet<DatasetId> =
            simulation.runs.iter().map(|run| run.dataset_id).collect();
        if retained == self.retained_datasets {
            self.retained_history_revision = Some(revision);
            return;
        }
        self.retain_datasets(&retained);
        self.retained_datasets = retained;
        self.retained_history_revision = Some(revision);
    }

    pub(crate) fn persistent_document_pane(
        &self,
        document_id: crate::product::ResultDocumentId,
    ) -> Option<PaneId> {
        self.persistent_document_panes.get(&document_id).copied()
    }

    pub(crate) fn select_persistent_document_pane(
        &mut self,
        document_id: crate::product::ResultDocumentId,
        pane_id: PaneId,
    ) {
        self.persistent_document_panes.insert(document_id, pane_id);
    }

    /// The standing reason one document's Latest retarget onto this exact
    /// candidate was refused, if it already was.
    pub(crate) fn latest_retarget_failure(
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

    pub(crate) fn clear_latest_retarget_failure(
        &mut self,
        document_id: crate::product::ResultDocumentId,
    ) {
        self.latest_retarget_failures.remove(&document_id);
    }

    pub(super) fn enter_persistent_pane(
        &mut self,
        document_id: ResultDocumentId,
        pane_id: PaneId,
        analysis: AnalysisPresentationKey,
    ) {
        self.persistent_pane_context = Some(PersistentPaneContext {
            document_id,
            pane_id,
            analysis,
        });
    }

    /// Leave the persistent projection: the pane context and the retained
    /// marker overlay it scopes both belong to the document that was open.
    pub(super) fn leave_persistent_document(&mut self) {
        self.persistent_pane_context = None;
        self.document_markers.clear();
    }

    pub(super) fn persistent_viewer_key(viewer: ResultViewer) -> ResultViewer {
        if viewer_uses_wave_stack(viewer) {
            ResultViewer::Waves
        } else {
            viewer
        }
    }

    /// The unit-pane ordinal a retained pane's axis ranges describe.
    ///
    /// A retained pane carries one horizontal and one vertical axis, while the
    /// waveform stack draws several unit panes inside it — volts and amps do
    /// not share a Y scale. The document can therefore only state one of those
    /// verticals, and it states the first, by construction. Every other unit
    /// pane's zoom is session state and stays out of the document rather than
    /// being written into it under whichever ordinal a hash map happened to
    /// yield first.
    const RETAINED_PANE_ORDINAL: usize = 0;

    pub(super) fn project_persistent_plot_view(
        &mut self,
        viewer: ResultViewer,
        x: Option<(f64, f64)>,
        y: Option<(f64, f64)>,
    ) {
        let Some(context) = self.persistent_pane_context else {
            return;
        };
        let viewer = Self::persistent_viewer_key(viewer);
        let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
        let key = (viewer, plot, Self::RETAINED_PANE_ORDINAL);
        // Only the ordinal the document actually states is replaced. Clearing
        // every ordinal here destroyed the zoom of every other unit pane on
        // every frame the document drew, so a multi-pane document could not
        // hold a zoom on anything but its first pane.
        if x.is_some() || y.is_some() {
            self.views.insert(key, PlotView { x, y });
        } else {
            self.views.remove(&key);
        }
    }

    pub(super) fn persistent_plot_view(&self, viewer: ResultViewer) -> PlotView {
        let Some(context) = self.persistent_pane_context else {
            return PlotView::default();
        };
        let viewer = Self::persistent_viewer_key(viewer);
        let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
        self.views
            .get(&(viewer, plot, Self::RETAINED_PANE_ORDINAL))
            .copied()
            .unwrap_or_default()
    }

    /// Flip one signal's membership in the browser's Favorites scope.
    pub(crate) fn toggle_favorite_signal(&mut self, key: SourceWaveformPresentationKey) {
        if !self.favorite_signals.remove(&key) {
            self.favorite_signals.insert(key);
        }
    }

    pub(crate) fn is_favorite_signal(&self, key: &SourceWaveformPresentationKey) -> bool {
        self.favorite_signals.contains(key)
    }

    /// Record a deliberate signal interaction for the Recent scope: front
    /// insertion, deduplicated, bounded so the scope stays a shortlist.
    pub(crate) fn note_recent_signal(&mut self, key: SourceWaveformPresentationKey) {
        const RECENT_SIGNAL_CAP: usize = 24;
        self.recent_signals.retain(|recent| recent != &key);
        self.recent_signals.insert(0, key);
        self.recent_signals.truncate(RECENT_SIGNAL_CAP);
    }

    /// Position in the Recent shortlist; `None` when never noted.
    pub(crate) fn recent_signal_rank(&self, key: &SourceWaveformPresentationKey) -> Option<usize> {
        self.recent_signals.iter().position(|recent| recent == key)
    }

    pub(crate) fn toggle_checked_result_quantity(&mut self, key: ResultBrowserSelectionKey) {
        if !self.checked_result_quantities.remove(&key) {
            self.checked_result_quantities.insert(key);
        }
    }

    pub(crate) fn is_checked_signal(&self, key: &SourceWaveformPresentationKey) -> bool {
        self.checked_result_quantities
            .contains(&ResultBrowserSelectionKey::Waveform(key.clone()))
    }

    pub(crate) fn is_checked_result_artifact(&self, key: &ResultArtifactPresentationKey) -> bool {
        self.checked_result_quantities
            .contains(&ResultBrowserSelectionKey::Artifact(key.clone()))
    }

    pub(crate) fn clear_checked_signals(&mut self) {
        self.checked_result_quantities.clear();
        self.browser_range_anchor = None;
    }

    pub(crate) fn set_browser_range_anchor(&mut self, key: ResultBrowserSelectionKey) {
        self.browser_range_anchor = Some(key);
    }

    pub(crate) fn select_checked_result_range(
        &mut self,
        target: &ResultBrowserSelectionKey,
        ordered_visible: &[ResultBrowserSelectionKey],
    ) {
        let anchor = self.browser_range_anchor.as_ref().unwrap_or(target);
        let Some(anchor_index) = ordered_visible.iter().position(|key| key == anchor) else {
            self.checked_result_quantities.insert(target.clone());
            self.browser_range_anchor = Some(target.clone());
            return;
        };
        let Some(target_index) = ordered_visible.iter().position(|key| key == target) else {
            return;
        };
        let (start, end) = if anchor_index <= target_index {
            (anchor_index, target_index)
        } else {
            (target_index, anchor_index)
        };
        self.checked_result_quantities
            .extend(ordered_visible[start..=end].iter().cloned());
    }

    pub(crate) fn select_visible_signals(&mut self, ordered_visible: &[ResultBrowserSelectionKey]) {
        self.checked_result_quantities
            .extend(ordered_visible.iter().cloned());
        self.browser_range_anchor = ordered_visible.last().cloned();
    }

    pub(crate) fn toggle_favorite_result_artifact(&mut self, key: ResultArtifactPresentationKey) {
        if !self.favorite_result_artifacts.remove(&key) {
            self.favorite_result_artifacts.insert(key);
        }
    }

    pub(crate) fn is_favorite_result_artifact(&self, key: &ResultArtifactPresentationKey) -> bool {
        self.favorite_result_artifacts.contains(key)
    }

    pub(crate) fn note_recent_result_artifact(&mut self, key: ResultArtifactPresentationKey) {
        const RECENT_ARTIFACT_CAP: usize = 24;
        self.recent_result_artifacts.retain(|recent| recent != &key);
        self.recent_result_artifacts.insert(0, key);
        self.recent_result_artifacts.truncate(RECENT_ARTIFACT_CAP);
    }

    pub(crate) fn recent_result_artifact_rank(
        &self,
        key: &ResultArtifactPresentationKey,
    ) -> Option<usize> {
        self.recent_result_artifacts
            .iter()
            .position(|recent| recent == key)
    }

    /// The zoom/pan override for a single-pane plot.
    pub fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), 0)
    }

    /// The zoom/pan override for one pane of one plot.
    ///
    /// Y is per pane because each pane carries its own unit — one zoom
    /// factor across volts and amps would mean nothing.
    #[cfg(test)]
    pub fn plot_view_pane(&self, viewer: ResultViewer, index: usize, pane: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), pane)
    }

    pub(super) fn plot_view_pane_for(
        &self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> PlotView {
        let plot = self.scoped_plot_key(plot);
        self.views
            .get(&(viewer, plot, pane))
            .copied()
            .unwrap_or_default()
    }

    pub(in crate::workbench::documents) fn analysis_plot_view_pane(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> PlotView {
        self.plot_view_pane_for(
            viewer,
            PlotPresentationKey::Analysis(analysis.authored()),
            pane,
        )
    }

    /// Mutable zoom/pan override for one pane of one plot.
    pub fn plot_view_pane_mut(
        &mut self,
        viewer: ResultViewer,
        index: usize,
        pane: usize,
    ) -> &mut PlotView {
        self.plot_view_pane_mut_for(viewer, PlotPresentationKey::Global(index), pane)
    }

    pub(super) fn plot_view_pane_mut_for(
        &mut self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        let plot = self.scoped_plot_key(plot);
        self.views.entry((viewer, plot, pane)).or_default()
    }

    pub(super) fn scoped_plot_key(&self, plot: PlotPresentationKey) -> PlotPresentationKey {
        let Some(context) = self.persistent_pane_context else {
            return plot;
        };
        match plot {
            PlotPresentationKey::Global(_) => {
                PlotPresentationKey::Document(context.document_id, context.pane_id)
            }
            PlotPresentationKey::Analysis(authored) if authored == context.analysis.authored() => {
                PlotPresentationKey::Document(context.document_id, context.pane_id)
            }
            PlotPresentationKey::Analysis(_) | PlotPresentationKey::Document(_, _) => plot,
        }
    }

    /// Crate-visible because the pinned window a waveform pane holds is read
    /// back by the Export/Print adapters, which have to be able to state the
    /// key production actually writes.
    pub(crate) fn analysis_plot_view_pane_mut(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        self.plot_view_pane_mut_for(
            viewer,
            PlotPresentationKey::Analysis(analysis.authored()),
            pane,
        )
    }

    /// Mutable zoom/pan override for a single-pane plot.
    pub fn plot_view_mut(&mut self, viewer: ResultViewer, index: usize) -> &mut PlotView {
        self.plot_view_pane_mut(viewer, index, 0)
    }

    /// Drop the zoom/pan override for one plot, every pane (FIT action).
    /// Fitting a strip fits all of it — leaving one pane zoomed would make
    /// the strip's panes disagree about the window they show.
    pub fn reset_plot_view(&mut self, viewer: ResultViewer, index: usize) {
        self.reset_plot_view_for(viewer, PlotPresentationKey::Global(index));
    }

    /// Drop every pinned viewport this sheet owns.
    ///
    /// A sheet can own several plots. Reset every ordinal so Fit does not
    /// leave any of its plots zoomed.
    pub(crate) fn reset_viewer_plot_views(&mut self, viewer: ResultViewer) {
        if let Some(context) = self.persistent_pane_context {
            let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
            self.views
                .retain(|(key_viewer, key_plot, _), _| (*key_viewer, *key_plot) != (viewer, plot));
            return;
        }
        self.views
            .retain(|(key_viewer, _, _), _| *key_viewer != viewer);
    }

    pub(super) fn reset_plot_view_for(&mut self, viewer: ResultViewer, plot: PlotPresentationKey) {
        let plot = self.scoped_plot_key(plot);
        self.views
            .retain(|(key_viewer, key_plot, _), _| (*key_viewer, *key_plot) != (viewer, plot));
    }

    pub(in crate::workbench::documents) fn reset_analysis_plot_view(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) {
        self.reset_plot_view_for(viewer, PlotPresentationKey::Analysis(analysis.authored()));
    }

    /// Drop every analysis-keyed viewport override of one viewer.
    pub(in crate::workbench::documents) fn reset_all_analysis_plot_views(
        &mut self,
        viewer: ResultViewer,
    ) {
        if let Some(context) = self.persistent_pane_context {
            self.reset_plot_view_for(
                viewer,
                PlotPresentationKey::Analysis(context.analysis.authored()),
            );
            return;
        }
        self.views.retain(|(key_viewer, key_plot, _), _| {
            *key_viewer != viewer || !matches!(key_plot, PlotPresentationKey::Analysis(_))
        });
    }

    /// Drop one axis' override on every pane of one strip, whatever ordinal it
    /// was stored under.
    ///
    /// The wave stack's panes share an abscissa, so an X window released on
    /// one ordinal and left on the rest is a strip whose panes disagree. Every
    /// stored ordinal is visited rather than the ones the strip currently
    /// draws, because a strip that has lost a pane keeps the departed
    /// ordinal's entry — and that entry goes on reporting the strip as zoomed
    /// with no pane left to fit it from. An entry with nothing left to say is
    /// removed, so the pinned-axis and zoomed predicates read the same answer.
    pub(in crate::workbench::documents) fn clear_analysis_plot_view_axis(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        axis: PaneAxis,
    ) {
        let plot = self.scoped_plot_key(PlotPresentationKey::Analysis(analysis.authored()));
        self.views.retain(|(key_viewer, key_plot, _), view| {
            if (*key_viewer, *key_plot) != (viewer, plot) {
                return true;
            }
            match axis {
                PaneAxis::X => view.x = None,
                PaneAxis::Y => view.y = None,
            }
            view.is_zoomed()
        });
    }

    /// Whether any pane of one plot is zoomed away from the automatic view.
    #[cfg(test)]
    pub fn strip_is_zoomed(&self, viewer: ResultViewer, index: usize) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Global(index))
    }

    pub(super) fn plot_is_zoomed(&self, viewer: ResultViewer, plot: PlotPresentationKey) -> bool {
        let plot = self.scoped_plot_key(plot);
        self.views.iter().any(|((key_viewer, key_index, _), view)| {
            (*key_viewer, *key_index) == (viewer, plot) && view.is_zoomed()
        })
    }

    pub(in crate::workbench::documents) fn analysis_strip_is_zoomed(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Analysis(analysis.authored()))
    }

    /// Whether any pane of one strip pins the given axis.
    pub(in crate::workbench::documents) fn analysis_strip_axis_is_pinned(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        axis: PaneAxis,
    ) -> bool {
        let plot = self.scoped_plot_key(PlotPresentationKey::Analysis(analysis.authored()));
        self.views.iter().any(|((key_viewer, key_plot, _), view)| {
            (*key_viewer, *key_plot) == (viewer, plot)
                && match axis {
                    PaneAxis::X => view.x.is_some(),
                    PaneAxis::Y => view.y.is_some(),
                }
        })
    }

    /// Restore a strip addressed by the current run's transient ordinal.
    /// Command/search integrations may still produce an ordinal, but the
    /// retained presentation state is removed by its stable identity.
    pub(crate) fn restore_analysis_strip(
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
        self.hidden_strips
            .remove(&AnalysisPresentationKey::new(run.dataset_id, analysis));
    }
}
