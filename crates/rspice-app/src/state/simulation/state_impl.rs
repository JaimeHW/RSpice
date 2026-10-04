//! Operations over [`SimulationState`].

use super::*;
use crate::product::{DatasetId, RunId};

impl SimulationState {
    /// Lifecycle of the exact execution currently owned by the controller.
    #[must_use]
    pub fn active_execution_lifecycle(&self) -> Option<SimulationRunLifecycle> {
        let identity = self.execution.active_execution?;
        self.retained
            .runs
            .iter()
            .find(|run| run.execution_identity() == Some(identity))
            .map(|run| run.lifecycle)
    }

    /// Whether a new identity-bound cancellation request can be accepted.
    #[must_use]
    pub fn can_request_abort_active_run(&self) -> bool {
        self.execution.active_execution.is_some()
            && self.execution.abort_request.is_none()
            && matches!(
                self.active_execution_lifecycle(),
                Some(SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running)
            )
    }

    /// Whether the active execution is already waiting for cancellation
    /// acknowledgement from the runner.
    #[must_use]
    pub fn cancellation_is_pending(&self) -> bool {
        self.execution
            .active_execution
            .is_some_and(|identity| self.execution.abort_request == Some(identity))
            || matches!(
                self.active_execution_lifecycle(),
                Some(SimulationRunLifecycle::Cancelling)
            )
    }

    /// Materialize one authenticated deferred output from the immutable
    /// retained source analysis. Stable identities are used so a UI action
    /// cannot target a different row after history reorder or pruning.
    pub fn materialize_deferred_saved_output(
        &mut self,
        run_id: RunId,
        analysis_id: u64,
        receipt_index: usize,
    ) -> Result<(), String> {
        let run_index = self
            .retained
            .runs
            .iter()
            .position(|run| run.run_id == run_id)
            .ok_or_else(|| "saved-output dataset is no longer retained".to_owned())?;
        let analysis_index = self.retained.runs[run_index]
            .analyses
            .iter()
            .position(|analysis| analysis.id == analysis_id)
            .ok_or_else(|| "saved-output analysis is no longer retained".to_owned())?;
        crate::simulation::output_contract::materialize_deferred_saved_output(
            &mut self.retained.runs[run_index].analyses[analysis_index],
            receipt_index,
        )?;
        self.retained.runs[run_index].validate_provenance()?;

        if self.view.active_run_idx == Some(run_index)
            && self.view.active_analysis_idx == Some(analysis_index)
        {
            self.sync_selected_analysis_waveforms();
        } else {
            self.view.data_version = self.view.data_version.wrapping_add(1);
        }
        Ok(())
    }

    /// Atomically replace yield evidence and its stable dataset authority.
    /// Empty result sets cannot retain stale provenance from a prior run.
    ///
    /// This is a new generation of the retained evidence. The distribution
    /// sheet resolves its spec limits, its yield verdict and the consistency
    /// check behind them once per data version, so replacing the evidence
    /// underneath that memo without moving the version left the sheet showing
    /// the previous run's limits against the new population.
    pub fn replace_yield_evidence(
        &mut self,
        results: Vec<YieldResult>,
        provenance: Option<YieldAnalysisProvenance>,
    ) {
        self.retained.yield_evidence.replace(results, provenance);
        self.view.data_version = self.view.data_version.wrapping_add(1);
    }

    /// Yield evidence for the currently selected result dataset. Callers use
    /// this instead of displaying evidence from whichever run happened to
    /// complete most recently.
    #[must_use]
    pub fn yield_results_for_active_dataset(&self) -> Option<&[YieldResult]> {
        self.active_run().and_then(|run| {
            self.retained
                .yield_evidence
                .for_run_ids(run.run_id, run.dataset_id)
        })
    }

    /// Request cancellation of the exact execution active at this instant.
    /// Returns the bound identity on success; no unbound abort request is
    /// emitted when the controller has no authoritative active execution.
    pub fn request_abort_active_run(&mut self) -> Result<SimulationExecutionIdentity, String> {
        let identity = self
            .execution
            .active_execution
            .ok_or_else(|| "there is no active simulation execution to cancel".to_owned())?;
        if let Some(requested) = self.execution.abort_request {
            return Err(if requested == identity {
                "simulation cancellation is already pending".to_owned()
            } else {
                "a cancellation request for another simulation execution is still pending"
                    .to_owned()
            });
        }
        let run = self
            .retained
            .runs
            .iter()
            .find(|run| run.execution_identity() == Some(identity))
            .ok_or_else(|| {
                format!(
                    "active simulation execution {} / {} has no retained run",
                    identity.job_id, identity.run_id
                )
            })?;
        if run.lifecycle == SimulationRunLifecycle::Cancelling {
            return Err(format!(
                "simulation run {} cancellation is already in progress",
                run.id
            ));
        }
        if !matches!(
            run.lifecycle,
            SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running
        ) {
            return Err(format!(
                "simulation run {} is {:?} and cannot be cancelled",
                run.id, run.lifecycle
            ));
        }
        self.execution.abort_request = Some(identity);
        self.execution.trigger_abort = true;
        Ok(identity)
    }

    /// Toggle visibility of a waveform by name, returns true if found
    /// Handles multiple naming conventions:
    /// - Exact match (e.g., "V(N001)" == "V(N001)")
    /// - Net name matching (e.g., "V(N001)" matches "N001")
    /// - N00X to numeric mapping (e.g., "V(N001)" matches "V(1)")
    pub fn toggle_waveform_visibility(&mut self, probe_name: &str) -> bool {
        if let Some(index) = self.waveform_index_for_probe(probe_name) {
            let waveform = &mut self.view.waveforms[index];
            waveform.visible = !waveform.visible;
            log::info!(
                "Toggled waveform '{}' (matched '{}') visibility to {}",
                waveform.name,
                probe_name,
                waveform.visible
            );
            return true;
        }

        // Check if this is the ground reference node
        let net_name_check = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');

        if let Some(ref ground) = self.view.ground_node
            && ground.eq_ignore_ascii_case(net_name_check)
        {
            log::info!(
                "Probe '{}' is the ground reference (0V) - no waveform displayed",
                probe_name
            );
            return false;
        }

        log::warn!(
            "Probe '{}' not found in {} waveforms",
            probe_name,
            self.view.waveforms.len()
        );
        false
    }

    /// Ensure an already-materialized waveform is visible without turning a
    /// visible trace off. `Some(true)` means visibility changed, `Some(false)`
    /// means it was already visible, and `None` means no waveform matched.
    pub fn ensure_waveform_visible(&mut self, probe_name: &str) -> Option<bool> {
        let index = self.waveform_index_for_probe(probe_name)?;
        let changed = !self.view.waveforms[index].visible;
        self.view.waveforms[index].visible = true;
        Some(changed)
    }

    fn waveform_index_for_probe(&self, probe_name: &str) -> Option<usize> {
        if let Some(index) = self
            .view
            .waveforms
            .iter()
            .position(|waveform| waveform.name.eq_ignore_ascii_case(probe_name))
        {
            return Some(index);
        }

        let net_name = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');
        if let Some(index) = self.view.waveforms.iter().position(|waveform| {
            waveform
                .name
                .trim_start_matches("V(")
                .trim_start_matches("I(")
                .trim_end_matches(')')
                .eq_ignore_ascii_case(net_name)
        }) {
            return Some(index);
        }

        let numeric_index = Self::extract_n00x_numeric(net_name)?;
        self.view.waveforms.iter().position(|waveform| {
            waveform
                .name
                .trim_start_matches("V(")
                .trim_start_matches("I(")
                .trim_end_matches(')')
                == numeric_index
        })
    }

    /// Extract numeric index from N00X format (e.g., "N001" -> "1", "N002" -> "2")
    fn extract_n00x_numeric(name: &str) -> Option<String> {
        let name_upper = name.to_uppercase();
        if name_upper.starts_with('N') {
            let rest = &name[1..];
            // Try to parse as a number and strip leading zeros
            if let Ok(num) = rest.parse::<u32>() {
                return Some(num.to_string());
            }
        }
        None
    }

    // =========================================================================
    // Multi-Run Results Management (Cadence Spectre PSF-style)
    // =========================================================================

    /// Allocate a fixture run without preparing a simulation.
    #[cfg(test)]
    pub fn start_run(&mut self) -> &mut SimulationRun {
        self.start_run_with_receipt(None)
            .expect("fixture run sequence is available")
    }

    /// Allocate a run for external evidence; the importer seals its provenance
    /// and payload before publishing it. Exhaustion does not mutate history.
    pub(crate) fn start_imported_run(&mut self) -> Result<&mut SimulationRun, String> {
        self.start_run_with_receipt(None)
    }

    /// Start a run already sealed by the exact consumed prepared snapshot.
    /// Sequence exhaustion leaves history, selection, and retention unchanged.
    pub(crate) fn start_prepared_run(
        &mut self,
        receipt: PreparedRunReceipt,
    ) -> Result<&mut SimulationRun, String> {
        self.start_run_with_receipt(Some(receipt))
    }

    fn start_run_with_receipt(
        &mut self,
        receipt: Option<PreparedRunReceipt>,
    ) -> Result<&mut SimulationRun, String> {
        let next_run_id = self.retained.next_run_id.checked_add(1).ok_or_else(|| {
            "The project run sequence is exhausted; start a new project to add another run."
                .to_owned()
        })?;
        let plan_scoped = receipt
            .as_ref()
            .and_then(PreparedRunReceipt::simulation_plan_id)
            .is_some();
        self.retained.next_run_id = next_run_id;
        let run = match receipt {
            Some(receipt) => SimulationRun::new_prepared(self.retained.next_run_id, receipt),
            None => SimulationRun::new(self.retained.next_run_id),
        };

        // Insert at front (newest first)
        self.retained.runs.insert(0, run);

        // Set as active run
        self.view.active_run_idx = Some(0);
        self.view.active_analysis_idx = None;

        // Prune history if needed
        if !plan_scoped {
            self.prune_runs_history();
        }
        self.prune_overlay_dataset_ids();

        // Return mutable reference to the new run
        Ok(&mut self.retained.runs[0])
    }

    /// Complete the current run and update legacy waveforms for compatibility
    ///
    /// This syncs the new run-based results with the legacy flat waveforms list
    /// so existing waveform viewer code continues to work.
    pub fn complete_run(&mut self) {
        if let Some(run_idx) = self.view.active_run_idx
            && let Some(run) = self.retained.runs.get(run_idx)
        {
            // Auto-select first analysis if available - this will sync only that analysis's waveforms
            if !run.analyses.is_empty() {
                // Use select_analysis to properly load only the selected analysis's waveforms
                self.select_analysis(0);
            }
        }
    }

    /// Select a run by index
    ///
    /// Returns true if the run exists and was selected.
    pub fn select_run(&mut self, run_idx: usize) -> bool {
        if run_idx < self.retained.runs.len() {
            self.view.active_run_idx = Some(run_idx);
            self.view.active_analysis_idx = None;

            let has_analyses = self
                .retained
                .runs
                .get(run_idx)
                .map(|run| !run.analyses.is_empty())
                .unwrap_or(false);

            // Auto-select first analysis in this run (this will sync only that analysis's waveforms).
            // If the run has no analyses, clear displayed waveform data so the viewer cannot show stale traces.
            if has_analyses {
                self.select_analysis(0);
            } else {
                self.sync_selected_analysis_waveforms();
            }
            true
        } else {
            false
        }
    }

    /// Select an analysis within the current run
    ///
    /// Returns true if the analysis exists and was selected.
    pub fn select_analysis(&mut self, analysis_idx: usize) -> bool {
        if let Some(run_idx) = self.view.active_run_idx
            && let Some(run) = self.retained.runs.get(run_idx)
            && analysis_idx < run.analyses.len()
        {
            self.view.active_analysis_idx = Some(analysis_idx);
            self.sync_selected_analysis_waveforms();
            return true;
        }
        false
    }

    #[cfg(test)]
    pub fn select_latest_analysis(&mut self) -> bool {
        let Some(run_idx) = self.view.active_run_idx else {
            return false;
        };
        let Some(last_idx) = self
            .retained
            .runs
            .get(run_idx)
            .and_then(|run| run.analyses.len().checked_sub(1))
        else {
            return false;
        };
        self.view.active_analysis_idx = Some(last_idx);
        self.sync_selected_analysis_waveforms();
        true
    }

    /// Get the currently active run (if any)
    pub fn active_run(&self) -> Option<&SimulationRun> {
        self.view
            .active_run_idx
            .and_then(|idx| self.retained.runs.get(idx))
    }

    // =========================================================================
    // Run overlay (signal owns hue, run owns weight)
    // =========================================================================

    /// Select a run by its legacy display sequence.
    pub fn select_run_by_sequence(&mut self, run_sequence: u64) -> bool {
        let Some(run_idx) = self
            .retained
            .runs
            .iter()
            .position(|run| run.id == run_sequence)
        else {
            return false;
        };
        self.select_run(run_idx)
    }

    /// Select the latest analysis in a run addressed by display sequence.
    pub fn select_latest_analysis_in_run_sequence(&mut self, run_sequence: u64) -> bool {
        let Some(run_idx) = self
            .retained
            .runs
            .iter()
            .position(|run| run.id == run_sequence)
        else {
            return false;
        };
        let Some(last_idx) = self
            .retained
            .runs
            .get(run_idx)
            .and_then(|run| run.analyses.len().checked_sub(1))
        else {
            return false;
        };
        self.view.active_run_idx = Some(run_idx);
        self.view.active_analysis_idx = Some(last_idx);
        self.sync_selected_analysis_waveforms();
        true
    }

    /// Whether a dataset is currently overlaid onto the active dataset.
    pub fn is_dataset_overlaid(&self, dataset_id: DatasetId) -> bool {
        self.view.overlay_dataset_ids.contains(&dataset_id)
    }

    /// Toggle a dataset in or out of the overlay set. The active dataset is
    /// always drawn and cannot be overlaid onto itself; toggling it is a no-op.
    /// Returns the new membership state.
    pub fn toggle_dataset_overlay(&mut self, dataset_id: DatasetId) -> bool {
        if self
            .active_run()
            .is_some_and(|run| run.dataset_id == dataset_id)
        {
            return false;
        }
        if let Some(pos) = self
            .view
            .overlay_dataset_ids
            .iter()
            .position(|id| *id == dataset_id)
        {
            self.view.overlay_dataset_ids.remove(pos);
            self.view.data_version = self.view.data_version.wrapping_add(1);
            false
        } else if self.retained.run_by_dataset_id(dataset_id).is_some() {
            self.view.overlay_dataset_ids.push(dataset_id);
            self.view.data_version = self.view.data_version.wrapping_add(1);
            true
        } else {
            false
        }
    }

    /// The runs to draw: the active run first (full weight), then every
    /// overlaid run in history order (reduced weight). The active run never
    /// repeats even when its ID is also in the overlay set.
    pub fn display_runs(&self) -> Vec<&SimulationRun> {
        let mut out = Vec::new();
        let active_id = self.active_run().map(|run| run.dataset_id);
        if let Some(run) = self.active_run() {
            out.push(run);
        }
        for run in &self.retained.runs {
            if Some(run.dataset_id) != active_id
                && self.view.overlay_dataset_ids.contains(&run.dataset_id)
            {
                out.push(run);
            }
        }
        out
    }

    /// Drop overlay IDs whose runs have left the history.
    fn prune_overlay_dataset_ids(&mut self) {
        self.view
            .overlay_dataset_ids
            .retain(|id| self.retained.runs.iter().any(|run| run.dataset_id == *id));
    }

    /// Get the currently active analysis (if any)
    pub fn active_analysis(&self) -> Option<&AnalysisResult> {
        self.active_run().and_then(|run| {
            self.view
                .active_analysis_idx
                .and_then(|idx| run.analyses.get(idx))
        })
    }

    /// Get mutable reference to the currently active run
    #[cfg(test)]
    pub fn active_run_mut(&mut self) -> Option<&mut SimulationRun> {
        self.view
            .active_run_idx
            .and_then(|idx| self.retained.runs.get_mut(idx))
    }

    /// The selected immutable dataset only when it belongs to `plan_id`.
    /// Legacy datasets predate prepared receipts and remain readable; every
    /// current prepared dataset is fail-closed against its frozen plan owner.
    pub fn active_run_for_plan(
        &self,
        plan_id: crate::product::SimulationPlanId,
    ) -> Option<&SimulationRun> {
        self.active_run()
            .filter(|run| run.evidence_domain(Some(plan_id)).answers_a_plan_limit())
    }

    /// How the selected dataset relates to the plan whose limits are being read.
    ///
    /// The one owner of that question. Two surfaces answer a limit against the
    /// selected dataset — the studio's requirements page and Verify's cockpit —
    /// and they used to decide independently whether the dataset was theirs to
    /// judge, so the same run could be evidence on one and silently absent from
    /// the other. Both now classify it here and say which case they are in.
    #[must_use]
    pub fn evidence_domain(
        &self,
        plan_id: Option<crate::product::SimulationPlanId>,
    ) -> EvidenceDomain {
        self.active_run().map_or(EvidenceDomain::NoDataset, |run| {
            run.evidence_domain(plan_id)
        })
    }

    /// Clear all runs history
    pub fn clear_runs(&mut self) {
        if self.execution.has_active_execution() {
            return;
        }
        self.retained.runs.clear();
        self.retained.executed_decks = ExecutedDeckArchive::default();
        self.view.active_run_idx = None;
        self.view.active_analysis_idx = None;
        self.view.overlay_dataset_ids.clear();
        self.replace_yield_evidence(Vec::new(), None);
        self.sync_selected_analysis_waveforms();
        // Don't reset next_run_id to preserve uniqueness
    }

    /// Replace persisted run history and rebuild every derived selection cache.
    ///
    /// Project files persist stable run and dataset IDs plus a run-local
    /// analysis sequence rather than fragile vector indices. On restore, this
    /// method maps that composite reference back to the current history layout
    /// and falls back to the first available analysis when no selection exists.
    pub fn restore_run_history(
        &mut self,
        runs: Vec<SimulationRun>,
        next_run_id: u64,
        active_run_id: Option<RunId>,
        active_dataset_id: Option<DatasetId>,
        active_analysis_sequence: Option<u64>,
        overlay_dataset_ids: Vec<DatasetId>,
    ) {
        self.retained.runs = runs.into();
        // Loading a project must not apply the retired project-global limit to
        // a history now owned by multiple simulation plans. Each plan applies
        // its own policy when it next creates or explicitly edits retention.

        let max_run_id = self
            .retained
            .runs
            .iter()
            .map(|run| run.id)
            .max()
            .unwrap_or(0);
        self.retained.next_run_id = next_run_id.max(max_run_id);

        self.view.active_run_idx = match (active_run_id, active_dataset_id) {
            (Some(run_id), Some(dataset_id)) => self
                .retained
                .runs
                .iter()
                .position(|run| run.run_id == run_id && run.dataset_id == dataset_id),
            (Some(run_id), None) => self
                .retained
                .runs
                .iter()
                .position(|run| run.run_id == run_id),
            (None, Some(dataset_id)) => self
                .retained
                .runs
                .iter()
                .position(|run| run.dataset_id == dataset_id),
            (None, None) => None,
        }
        .or_else(|| (!self.retained.runs.is_empty()).then_some(0));

        self.view.active_analysis_idx = self.view.active_run_idx.and_then(|run_idx| {
            let run = &self.retained.runs[run_idx];
            active_analysis_sequence
                .and_then(|id| run.analyses.iter().position(|analysis| analysis.id == id))
                .or_else(|| (!run.analyses.is_empty()).then_some(0))
        });

        let active_id = self.active_run().map(|run| run.dataset_id);
        self.view.overlay_dataset_ids.clear();
        for id in overlay_dataset_ids {
            if Some(id) != active_id
                && self.retained.runs.iter().any(|run| run.dataset_id == id)
                && !self.view.overlay_dataset_ids.contains(&id)
            {
                self.view.overlay_dataset_ids.push(id);
            }
        }

        // Whatever this session had run is not this history. Its decks are
        // dropped here rather than left to be addressed by a run sequence the
        // opened project gave to a different run; the project's own retained
        // decks are installed afterwards.
        self.prune_executed_decks();
        self.sync_selected_analysis_waveforms();
    }

    /// Prune runs history to stay within the project's retention limit.
    ///
    /// Two runs are never discarded: a golden baseline, and the newest run in
    /// the history. When those alone exceed the limit the history stays over
    /// it — reported by [`Self::retention_limit_is_unenforceable`] — because
    /// the alternative is destroying a signed-off baseline or the dataset a
    /// run has just produced, and neither is what "retain fewer" asks for.
    fn prune_runs_history(&mut self) {
        let limit = self.retained.effective_retained_dataset_limit();
        let selected_run_id = self.active_run().map(|run| run.run_id);
        self.retained.runs.prune_runs(limit);
        // Pinning makes pruning remove from the middle, so the index-based
        // selection is re-resolved from the identity it pointed at rather than
        // left to land on whichever run shifted into that slot.
        if let Some(run_id) = selected_run_id {
            self.view.active_run_idx = self
                .retained
                .runs
                .iter()
                .position(|run| run.run_id == run_id);
            if self.view.active_run_idx.is_none() {
                self.view.active_analysis_idx = None;
            }
        }
        self.prune_executed_decks();
        self.prune_yield_evidence_provenance();
    }

    pub(crate) fn prune_plan_runs(
        &mut self,
        plan_id: crate::product::SimulationPlanId,
        limit: usize,
    ) {
        let selected_run_id = self.active_run().map(|run| run.run_id);
        self.retained
            .runs
            .prune_plan_runs(plan_id, limit, selected_run_id);
        if let Some(run_id) = selected_run_id {
            self.view.active_run_idx = self
                .retained
                .runs
                .iter()
                .position(|run| run.run_id == run_id);
            if self.view.active_run_idx.is_none() {
                self.view.active_analysis_idx = None;
            }
        }
        self.prune_overlay_dataset_ids();
        self.prune_executed_decks();
        self.prune_yield_evidence_provenance();
    }

    /// Whether the retention limit can still be honoured at all.
    ///
    /// Baselines are exempt from pruning, so once they fill the limit the
    /// history grows past it instead of holding at it. Surfaces report that
    /// rather than a count that implies the limit is being enforced.
    #[must_use]
    #[cfg(test)]
    pub fn retention_limit_is_unenforceable(&self) -> bool {
        self.retained.pinned_run_count() >= self.retained.effective_retained_dataset_limit()
    }

    /// Classify a retained run for retention.
    ///
    /// Returns whether the run is in the history. Deliberately does **not**
    /// prune: releasing a baseline says the dataset is no longer special, not
    /// that it should be discarded now, and the oldest pruneable dataset is
    /// usually the one just released — so pruning here would destroy it on the
    /// same click, with no undo and no confirmation. It becomes eligible at the
    /// next run, which is when retention discards anything else.
    ///
    /// This is not the rule [`Self::set_retained_dataset_limit`] follows, and
    /// the difference is deliberate: lowering the limit is the project stating
    /// how much it keeps, so it must take effect at once. Releasing one pin is
    /// curation of a single dataset. Until the next run the project may hold
    /// more than its limit; [`Self::retention_limit_is_unenforceable`] is how
    /// the page says so rather than implying the limit is being enforced.
    pub fn set_run_retention(&mut self, run_id: RunId, retention: RunRetention) -> bool {
        self.retained.runs.set_run_retention(run_id, retention)
    }

    /// Set the retention limit and apply it immediately.
    ///
    /// Applying at once is deliberate: a limit that only takes effect on the
    /// next run would report a policy the project is not actually under.
    #[cfg(test)]
    pub fn set_retained_dataset_limit(&mut self, limit: usize) {
        self.retained.retained_dataset_limit = Some(limit.max(1));
        self.prune_runs_history();
    }

    /// Delete a specific run by index
    ///
    /// Returns true if the run was deleted.
    #[cfg(test)]
    pub fn delete_run(&mut self, run_idx: usize) -> bool {
        if run_idx < self.retained.runs.len() {
            self.retained.runs.remove(run_idx);
            self.prune_overlay_dataset_ids();
            self.prune_executed_decks();
            self.prune_yield_evidence_provenance();
            self.view.data_version = self.view.data_version.wrapping_add(1);

            // Adjust active indices
            if let Some(active) = self.view.active_run_idx {
                if active == run_idx {
                    if self.retained.runs.is_empty() {
                        // Deleted final run: clear active selection and displayed waveform data.
                        self.view.active_run_idx = None;
                        self.view.active_analysis_idx = None;
                        self.sync_selected_analysis_waveforms();
                    } else {
                        // Deleted active run: select the new head run and synchronize displayed data.
                        let _ = self.select_run(0);
                    }
                } else if active > run_idx {
                    // Shift active index down
                    self.view.active_run_idx = Some(active - 1);
                }
            }
            true
        } else {
            false
        }
    }

    fn sync_selected_analysis_waveforms(&mut self) {
        let selected_waveforms = self
            .view
            .active_run_idx
            .and_then(|run_idx| self.retained.runs.get(run_idx))
            .and_then(|run| {
                self.view
                    .active_analysis_idx
                    .and_then(|analysis_idx| run.analyses.get(analysis_idx))
            })
            .map(|analysis| analysis.waveforms.clone())
            .unwrap_or_default();

        self.view.replace_waveforms(selected_waveforms);
    }

    /// Drop the executed decks of runs that are no longer in the history.
    ///
    /// Retention discards a dataset; the deck that dataset's engine read is
    /// part of what it cost and part of what it explained, so it goes at the
    /// same moment. Keeping it would leave a deck addressable only by a run
    /// sequence nothing resolves — and would spend the archive's ceiling, and
    /// the project's storage, on it.
    fn prune_executed_decks(&mut self) {
        let runs = &self.retained.runs;
        self.retained
            .executed_decks
            .retain_runs(|run_id| runs.iter().any(|run| run.id == run_id));
    }

    fn prune_yield_evidence_provenance(&mut self) {
        let is_retained = self.retained.yield_provenance().is_some_and(|provenance| {
            self.retained.runs.iter().any(|run| {
                run.run_id == provenance.source_run_id
                    && run.dataset_id == provenance.source_dataset_id
            })
        });
        if self.retained.yield_provenance().is_some() && !is_retained {
            self.replace_yield_evidence(Vec::new(), None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan_receipt(plan_id: crate::product::SimulationPlanId, byte: u8) -> PreparedRunReceipt {
        PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(plan_id),
            project_revision: crate::product::ObjectRevision::INITIAL,
            prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([byte; 32]),
            source_content_digest: crate::product::ContentDigest::from_bytes(
                [byte.wrapping_add(1); 32],
            ),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(
                crate::product::ContentDigest::from_bytes([byte.wrapping_add(2); 32]),
            ),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    crate::product::AnalysisInstanceId::new(),
                    crate::product::ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    crate::product::ContentDigest::from_bytes([byte.wrapping_add(3); 32]),
                )
                .expect("task receipt"),
            ],
        })
        .expect("plan receipt")
    }

    #[test]
    fn run_sequence_allocates_the_last_value_once_and_rejects_exhaustion_atomically() {
        let plan = crate::product::SimulationPlanId::new();
        let mut state = SimulationState::default();
        state.retained.next_run_id = u64::MAX - 2;
        let previous = state
            .start_prepared_run(plan_receipt(plan, 1))
            .unwrap()
            .run_id;
        let last = state.start_prepared_run(plan_receipt(plan, 11)).unwrap();
        assert_eq!(last.id, u64::MAX);
        assert_ne!(last.run_id, previous);
        state.select_run(1);
        state
            .view
            .overlay_dataset_ids
            .push(state.retained.runs[0].dataset_id);
        state.retained.retained_dataset_limit = Some(1);
        let revision = state.retained.runs.revision();
        let overlays = state.view.overlay_dataset_ids.clone();
        let data_version = state.view.data_version;

        for _ in 0..2 {
            let error = state
                .start_prepared_run(plan_receipt(plan, 21))
                .unwrap_err();
            assert!(error.contains("run sequence is exhausted"));
            assert_eq!(state.retained.next_run_id, u64::MAX);
            assert_eq!(state.retained.runs.revision(), revision);
            assert_eq!(state.retained.runs.len(), 2);
            assert_eq!(state.view.active_run_idx, Some(1));
            assert_eq!(state.active_run().unwrap().run_id, previous);
            assert_eq!(state.view.overlay_dataset_ids, overlays);
            assert_eq!(state.retained.retained_dataset_limit, Some(1));
            assert_eq!(state.view.data_version, data_version);
        }
        state.clear_runs();
        assert!(state.start_prepared_run(plan_receipt(plan, 31)).is_err());
        assert!(state.retained.runs.is_empty());
        assert_eq!(state.retained.next_run_id, u64::MAX);
    }

    #[test]
    fn plan_retention_prunes_only_the_owning_plans_datasets() {
        let plan_a = crate::product::SimulationPlanId::new();
        let plan_b = crate::product::SimulationPlanId::new();
        let mut state = SimulationState::default();
        state.start_prepared_run(plan_receipt(plan_a, 1)).unwrap();
        state.start_prepared_run(plan_receipt(plan_b, 11)).unwrap();
        state.start_prepared_run(plan_receipt(plan_a, 21)).unwrap();
        state.start_prepared_run(plan_receipt(plan_b, 31)).unwrap();
        state.start_prepared_run(plan_receipt(plan_a, 41)).unwrap();

        state.prune_plan_runs(plan_a, 2);

        assert_eq!(state.retained.retained_plan_dataset_count(plan_a), 2);
        assert_eq!(state.retained.retained_plan_dataset_count(plan_b), 2);
        assert_eq!(state.retained.runs.len(), 4);
    }

    #[test]
    fn clearing_run_history_releases_its_executed_decks_without_mutating_snapshots() {
        let mut state = SimulationState::default();
        let run = state.start_run();
        let sequence = run.id;
        let execution = run.execution_identity().unwrap();
        let deck: std::sync::Arc<str> = std::sync::Arc::from("retained deck\n.end\n");
        let storage = std::sync::Arc::downgrade(&deck);
        state
            .retained
            .executed_decks
            .retain(crate::state::ExecutedDeck {
                run_id: sequence,
                points: vec![crate::state::ExecutedDeckPoint {
                    label: "TT".to_owned(),
                    model_sources: Vec::new(),
                    deck,
                }],
            });
        let snapshot = state.clone();
        for worker_active in [true, false] {
            state.execution.is_running = worker_active;
            state.execution.active_execution = (!worker_active).then_some(execution);
            state.clear_runs();
            assert_eq!(state.retained.runs.len(), 1);
            assert!(
                state
                    .retained
                    .executed_decks
                    .shares_content_with(&snapshot.retained.executed_decks)
            );
        }
        state.execution.active_execution = None;
        state.clear_runs();
        assert!(state.retained.runs.is_empty());
        assert_eq!(state.retained.next_run_id, sequence);
        assert_eq!(state.retained.executed_decks.iter().count(), 0);
        assert!(state.retained.executed_decks.get(sequence).is_none());
        assert!(snapshot.retained.executed_decks.get(sequence).is_some());
        drop(snapshot);
        assert!(
            storage.upgrade().is_none(),
            "no orphaned deck storage remains"
        );
        state.clear_runs();
        assert_eq!(state.start_run().id, sequence + 1);
        assert_eq!(state.retained.executed_decks.iter().count(), 0);
    }

    #[test]
    fn discarding_a_dataset_discards_the_deck_its_engine_read() {
        let plan = crate::product::SimulationPlanId::new();
        let mut state = SimulationState::default();
        let mut sequences = Vec::new();
        for byte in [1_u8, 21, 41] {
            let run = state.start_prepared_run(plan_receipt(plan, byte)).unwrap();
            let sequence = run.id;
            sequences.push(sequence);
            let deck: std::sync::Arc<str> = std::sync::Arc::from(format!("run {sequence}\n.end\n"));
            state
                .retained
                .executed_decks
                .retain(crate::state::ExecutedDeck {
                    run_id: sequence,
                    points: vec![crate::state::ExecutedDeckPoint {
                        label: "TT 27C".to_owned(),
                        model_sources: Vec::new(),
                        deck,
                    }],
                });
        }
        assert!(
            sequences
                .iter()
                .all(|id| state.retained.executed_decks.get(*id).is_some())
        );

        state.prune_plan_runs(plan, 1);

        assert_eq!(state.retained.retained_plan_dataset_count(plan), 1);
        let kept = state
            .retained
            .runs
            .first()
            .expect("one dataset survives")
            .id;
        assert!(
            state.retained.executed_decks.get(kept).is_some(),
            "the surviving dataset keeps the deck that explains it"
        );
        assert!(
            sequences.iter().filter(|id| **id != kept).all(|id| state
                .retained
                .executed_decks
                .get(*id)
                .is_none()),
            "and a discarded dataset leaves no deck nothing can open"
        );
    }

    #[test]
    fn deferred_saved_output_materializes_by_stable_identity_and_refreshes_selection() {
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);
        analysis.saved_output_receipts.push(SavedOutputReceipt {
            source_bindings: Some(crate::state::SavedOutputSourceBindings {
                axis: crate::state::SavedOutputAxis::Waveform {
                    name: "out".to_owned(),
                },
                references: [(
                    "v(out)".to_owned(),
                    crate::state::SavedOutputBoundSource::Waveform {
                        name: "out".to_owned(),
                    },
                )]
                .into(),
            }),
            output_id: crate::product::SavedOutputId::new(),
            output_revision: crate::product::ObjectRevision::INITIAL,
            analysis_id: crate::product::AnalysisInstanceId::new(),
            contract_digest: crate::product::ContentDigest::from_bytes([0x7a; 32]),
            name: "output_voltage".to_owned(),
            source_expression: "V(out)".to_owned(),
            complex_policy: crate::state::ComplexExpressionPolicy::LegacyMagnitude,
            output_kind: crate::state::SavedOutputKind::RawVoltageOrCurrent,
            save_policy: crate::state::SavedOutputPolicy::OnDemandFromRetainedState,
            stored_precision: crate::state::SavedOutputPrecision::FullSourcePrecision,
            streaming: crate::state::SavedOutputStreaming::StoreOnly,
            display_intent: crate::state::SavedOutputDisplayIntent::Plot,
            status: SavedOutputMaterializationStatus::Deferred,
        });
        let mut run = SimulationRun::new(1);
        let run_id = run.run_id;
        run.add_analysis(analysis);
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture is explicitly classified");
        let mut state = SimulationState {
            retained: crate::state::RetainedSimulationState {
                runs: vec![run].into(),
                ..Default::default()
            },
            view: crate::state::SimulationViewState {
                active_run_idx: Some(0),
                active_analysis_idx: Some(0),
                ..Default::default()
            },
            ..SimulationState::default()
        };
        let version = state.view.data_version;

        state
            .materialize_deferred_saved_output(run_id, 1, 0)
            .expect("retained source materializes");

        assert_eq!(state.retained.runs[0].analyses[0].waveforms.len(), 2);
        assert_eq!(state.view.waveforms.len(), 2);
        assert!(state.view.data_version > version);
        assert!(matches!(
            state.retained.runs[0].analyses[0].saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 2,
                ..
            }
        ));
    }

    #[test]
    fn the_built_in_retention_limit_applies_until_the_project_states_one() {
        let state = SimulationState::default();

        assert_eq!(state.retained.retained_dataset_limit, None);
        assert_eq!(
            state.retained.effective_retained_dataset_limit(),
            MAX_RUN_HISTORY
        );
    }

    #[test]
    fn lowering_the_retention_limit_discards_the_oldest_datasets_at_once() {
        let mut state = SimulationState::default();
        for _ in 0..5 {
            state.start_run();
        }
        assert_eq!(state.retained.runs.len(), 5);
        let newest = state.retained.runs.first().expect("newest run").run_id;

        state.set_retained_dataset_limit(2);

        assert_eq!(
            state.retained.runs.len(),
            2,
            "the limit applies immediately"
        );
        assert_eq!(
            state.retained.runs.first().expect("newest run").run_id,
            newest,
            "pruning discards the oldest, never the newest"
        );
    }

    #[test]
    fn a_retention_limit_of_zero_still_keeps_the_run_just_produced() {
        let mut state = SimulationState::default();
        state.start_run();

        state.set_retained_dataset_limit(0);

        assert_eq!(state.retained.effective_retained_dataset_limit(), 1);
        assert_eq!(
            state.retained.runs.len(),
            1,
            "retaining nothing is never the intent"
        );
    }

    #[test]
    fn a_raised_limit_retains_more_without_recovering_what_was_discarded() {
        let mut state = SimulationState::default();
        for _ in 0..3 {
            state.start_run();
        }
        state.set_retained_dataset_limit(1);
        assert_eq!(state.retained.runs.len(), 1);

        state.set_retained_dataset_limit(10);

        assert_eq!(
            state.retained.runs.len(),
            1,
            "raising the limit cannot bring back a discarded dataset"
        );
    }

    #[test]
    fn a_golden_baseline_survives_a_limit_lowered_beneath_it() {
        let mut state = SimulationState::default();
        let baseline = state.start_run().run_id;
        for _ in 0..3 {
            state.start_run();
        }
        assert!(state.set_run_retention(baseline, RunRetention::GoldenBaseline));
        let newest = state.retained.runs.first().expect("newest run").run_id;

        state.set_retained_dataset_limit(1);

        assert_eq!(
            state
                .retained
                .runs
                .iter()
                .map(|run| run.run_id)
                .collect::<Vec<_>>(),
            vec![newest, baseline],
            "pruning stops at the baseline and at the run just produced"
        );
        assert_eq!(state.retained.pinned_run_count(), 1);
        assert!(state.retention_limit_is_unenforceable());
    }

    #[test]
    fn pruning_discards_the_oldest_pruneable_dataset_not_the_oldest_dataset() {
        let mut state = SimulationState::default();
        let baseline = state.start_run().run_id;
        let oldest_pruneable = state.start_run().run_id;
        let newest = state.start_run().run_id;
        assert!(state.set_run_retention(baseline, RunRetention::GoldenBaseline));

        state.set_retained_dataset_limit(2);

        assert_eq!(
            state
                .retained
                .runs
                .iter()
                .map(|run| run.run_id)
                .collect::<Vec<_>>(),
            vec![newest, baseline],
            "a baseline at the tail is skipped for the oldest dataset that may be discarded"
        );
        assert!(
            !state
                .retained
                .runs
                .iter()
                .any(|run| run.run_id == oldest_pruneable)
        );
    }

    #[test]
    fn pinning_every_retained_slot_still_keeps_the_run_just_produced() {
        let mut state = SimulationState::default();
        let first = state.start_run().run_id;
        let second = state.start_run().run_id;
        state.set_retained_dataset_limit(2);
        for baseline in [first, second] {
            assert!(state.set_run_retention(baseline, RunRetention::GoldenBaseline));
        }

        let produced = state.start_run().run_id;

        assert_eq!(
            state
                .retained
                .runs
                .iter()
                .map(|run| run.run_id)
                .collect::<Vec<_>>(),
            vec![produced, second, first],
            "the dataset a run just produced is never the one discarded"
        );
        assert!(state.retention_limit_is_unenforceable());
    }

    #[test]
    fn releasing_a_baseline_does_not_discard_it_on_the_same_click() {
        let mut state = SimulationState::default();
        let baseline = state.start_run().run_id;
        state.start_run();
        assert!(state.set_run_retention(baseline, RunRetention::GoldenBaseline));
        state.set_retained_dataset_limit(1);
        assert_eq!(
            state.retained.runs.len(),
            2,
            "the baseline blocks the limit"
        );

        assert!(state.set_run_retention(baseline, RunRetention::Pruneable));

        // Releasing says the dataset is no longer special, not that it should
        // be destroyed now. The released baseline is usually the oldest
        // pruneable run, so pruning here would delete exactly the dataset the
        // reader just touched, with no undo.
        assert_eq!(
            state.retained.runs.len(),
            2,
            "releasing a pin must not destroy the dataset it released"
        );
        assert_eq!(state.retained.pinned_run_count(), 0);
        assert!(!state.retention_limit_is_unenforceable());

        // It is merely eligible now: the next run collects it, which is when
        // retention discards anything else.
        state.start_run();
        assert_eq!(state.retained.runs.len(), 1);
    }

    #[test]
    fn retention_cannot_be_classified_for_a_run_outside_the_history() {
        let mut state = SimulationState::default();
        state.start_run();

        assert!(!state.set_run_retention(RunId::new(), RunRetention::GoldenBaseline));
        assert_eq!(state.retained.pinned_run_count(), 0);
    }

    #[test]
    fn the_selected_dataset_follows_pruning_that_discards_a_newer_one() {
        let mut state = SimulationState::default();
        let oldest = state.start_run().run_id;
        let selected = state.start_run().run_id;
        state.start_run();
        state.start_run();
        for baseline in [oldest, selected] {
            assert!(state.set_run_retention(baseline, RunRetention::GoldenBaseline));
        }
        assert!(
            state.select_run_by_sequence(2),
            "the baseline is selectable"
        );

        state.set_retained_dataset_limit(3);

        assert_eq!(
            state.active_run().map(|run| run.run_id),
            Some(selected),
            "selection is identity, not the index a middle removal shifted"
        );
    }

    #[test]
    fn abort_request_is_bound_to_exact_active_execution_identity() {
        let mut state = SimulationState::default();
        let run = state.start_run();
        run.mark_running().expect("fixture enters running state");
        let identity = run
            .execution_identity()
            .expect("current run has job identity");
        state.execution.active_execution = Some(identity);

        let requested = state
            .request_abort_active_run()
            .expect("active execution can be cancelled");

        assert_eq!(requested, identity);
        assert_eq!(state.execution.abort_request, Some(identity));
        assert!(state.execution.trigger_abort);
        assert_eq!(
            state
                .retained
                .run_by_stable_id(identity.run_id)
                .unwrap()
                .lifecycle,
            SimulationRunLifecycle::Running,
            "the controller owns the authoritative transition to Cancelling"
        );
    }

    #[test]
    fn stable_execution_identity_outlives_instantaneous_runner_activity() {
        let mut state = SimulationState::default();
        let run = state.start_run();
        run.mark_running().unwrap();
        let identity = run.execution_identity().unwrap();
        state.execution.active_execution = Some(identity);
        state.execution.is_running = false;

        assert!(state.execution.has_active_execution());
        assert_eq!(
            state.active_execution_lifecycle(),
            Some(SimulationRunLifecycle::Running)
        );
        assert!(state.can_request_abort_active_run());
    }

    #[test]
    fn duplicate_cancellation_requests_are_rejected_until_acknowledged() {
        let mut state = SimulationState::default();
        let run = state.start_run();
        run.mark_running().unwrap();
        let identity = run.execution_identity().unwrap();
        state.execution.active_execution = Some(identity);

        state.request_abort_active_run().unwrap();

        assert!(state.cancellation_is_pending());
        assert!(!state.can_request_abort_active_run());
        assert_eq!(
            state.request_abort_active_run().unwrap_err(),
            "simulation cancellation is already pending"
        );
    }

    #[test]
    fn abort_request_fails_closed_without_active_execution_identity() {
        let mut state = SimulationState::default();
        state.start_run().mark_running().unwrap();

        assert!(state.request_abort_active_run().is_err());
        assert!(!state.execution.trigger_abort);
        assert!(state.execution.abort_request.is_none());
    }

    #[test]
    fn active_execution_history_cannot_be_cleared() {
        let mut state = SimulationState::default();
        let run = state.start_run();
        run.mark_running().unwrap();
        let identity = run.execution_identity().unwrap();
        state.execution.active_execution = Some(identity);
        state.execution.is_running = true;

        state.clear_runs();

        assert_eq!(state.retained.runs.len(), 1);
        assert_eq!(state.execution.active_execution, Some(identity));
    }

    #[test]
    fn newest_retained_result_skips_empty_in_progress_run_records() {
        let mut retained = SimulationRun::new(1);
        retained.add_analysis(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "retained TRAN",
        ));
        let empty_newer = SimulationRun::new(2);
        let mut state = SimulationState::default();
        state.retained.runs = vec![empty_newer, retained].into();

        assert!(
            state.retained.has_results(),
            "run history itself is not empty"
        );
        assert!(state.retained.has_retained_result_dataset());
        assert_eq!(state.retained.newest_retained_result_run_index(), Some(1));
    }

    #[test]
    fn empty_run_history_does_not_advertise_a_result_dataset() {
        let mut state = SimulationState::default();
        state.retained.runs.push(SimulationRun::new(1));

        assert!(state.retained.has_results());
        assert!(!state.retained.has_retained_result_dataset());
        assert_eq!(state.retained.newest_retained_result_run_index(), None);
    }

    #[test]
    fn stable_restore_rebuilds_selection_overlays_and_waveform_cache() {
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN one").with_waveforms(vec![
                WaveformData::new("V(one)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        let overlay_dataset_id = run_one.dataset_id;

        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC two").with_waveforms(vec![
                WaveformData::new("V(two)", vec![1.0, 10.0], vec![2.0, 3.0], "#ffaa00"),
            ]),
        );
        let active_run_id = run_two.run_id;
        let active_dataset_id = run_two.dataset_id;
        let active_analysis_sequence = run_two.analyses[0].id;

        let mut state = SimulationState::default();
        state.restore_run_history(
            vec![run_one, run_two],
            2,
            Some(active_run_id),
            Some(active_dataset_id),
            Some(active_analysis_sequence),
            vec![
                overlay_dataset_id,
                overlay_dataset_id,
                active_dataset_id,
                DatasetId::new(),
            ],
        );

        assert_eq!(
            state.active_run().map(|run| run.run_id),
            Some(active_run_id)
        );
        assert_eq!(
            state.active_analysis().map(|analysis| analysis.id),
            Some(active_analysis_sequence)
        );
        assert_eq!(state.view.overlay_dataset_ids, vec![overlay_dataset_id]);
        assert_eq!(state.view.waveforms[0].name, "V(two)");
        assert_eq!(
            state
                .display_runs()
                .into_iter()
                .map(|run| run.dataset_id)
                .collect::<Vec<_>>(),
            vec![active_dataset_id, overlay_dataset_id]
        );
    }

    #[test]
    fn overlay_commands_accept_only_existing_non_active_stable_ids() {
        let active = SimulationRun::new(1);
        let active_id = active.dataset_id;
        let overlay = SimulationRun::new(2);
        let overlay_id = overlay.dataset_id;
        let mut state = SimulationState::default();
        state.retained.runs = vec![active, overlay].into();
        state.view.active_run_idx = Some(0);

        assert!(!state.toggle_dataset_overlay(active_id));
        assert!(!state.toggle_dataset_overlay(DatasetId::new()));
        assert!(state.toggle_dataset_overlay(overlay_id));
        assert!(state.is_dataset_overlaid(overlay_id));
        assert!(!state.toggle_dataset_overlay(overlay_id));
        assert!(!state.is_dataset_overlaid(overlay_id));
    }

    #[test]
    fn replacing_yield_evidence_keeps_dataset_authority_in_sync() {
        let run = SimulationRun::new(1);
        let monte_carlo = crate::simulation::SimulationResult::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 19,
            runs_requested: 3,
            runs_completed: 2,
            num_failures: 1,
            all_converged: false,
            variables: Vec::new(),
        };
        let provenance = rspice_simulation::results::yield_provenance_from_monte_carlo_result(
            run.run_id,
            run.dataset_id,
            &monte_carlo,
        )
        .expect("Monte Carlo result creates yield provenance");
        let source_dataset_id = run.dataset_id;
        let result = YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower("V(out)", 0.9, "V"),
            total_runs: 2,
            pass_count: 1,
            fail_count: 1,
            yield_percent: 50.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            trail: vec![true, false],
            samples: vec![1.0, 0.8],
        };
        let mut state = SimulationState::default();
        state.retained.runs.push(run);
        state.view.active_run_idx = Some(0);

        // Replacing the evidence is a new generation of it. The distribution
        // sheet resolves its spec limits, its yield verdict and the
        // consistency check behind them once per data version, so replacing
        // the population underneath that memo without moving the version left
        // the sheet showing the previous run's limits against the new one.
        let version = state.view.data_version;
        state.replace_yield_evidence(vec![result], Some(provenance));
        assert_ne!(
            state.view.data_version, version,
            "the yield evidence was replaced at the generation its memo already describes"
        );
        assert_eq!(state.retained.yield_provenance(), Some(provenance));
        assert_eq!(provenance.seed, 19);
        assert_eq!(provenance.runs_requested, 3);
        assert_eq!(provenance.runs_completed, 2);
        assert_eq!(
            provenance.sampling_mode,
            rspice_results::yield_analysis::MonteCarloSamplingMode::PseudoRandom
        );
        assert_eq!(
            state
                .yield_results_for_active_dataset()
                .expect("active dataset owns evidence")
                .len(),
            1
        );
        assert!(
            state
                .retained
                .yield_results_for_dataset(DatasetId::new())
                .is_none()
        );

        state.replace_yield_evidence(Vec::new(), Some(provenance));
        assert!(state.retained.yield_evidence.results().is_empty());
        assert_eq!(state.retained.yield_provenance(), None);

        state.replace_yield_evidence(
            vec![YieldResult {
                spec: rspice_results::yield_analysis::YieldSpec::lower("V(out)", 0.9, "V"),
                total_runs: 1,
                pass_count: 1,
                fail_count: 0,
                yield_percent: 100.0,
                stats: rspice_results::yield_analysis::DistributionStats::default(),
                trail: vec![true],
                samples: vec![1.0],
            }],
            Some(provenance),
        );
        assert!(
            state
                .retained
                .yield_results_for_dataset(source_dataset_id)
                .is_some()
        );
        assert!(state.delete_run(0));
        assert!(state.retained.yield_evidence.results().is_empty());
        assert_eq!(state.retained.yield_provenance(), None);
    }
}
