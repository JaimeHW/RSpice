use super::*;
use crate::product::{DatasetId, RunId};

impl SimulationState {
    pub fn request_simulate_run_set(&mut self) {
        self.run_intent = SimulationRunIntent::SimulateRunSet;
        self.trigger_simulation = true;
    }

    pub fn request_manual_deck_run(&mut self) {
        self.run_intent = SimulationRunIntent::ManualDeck;
        self.trigger_simulation = true;
    }

    /// Clear waveforms and increment version
    pub fn clear_waveforms(&mut self) {
        self.replace_waveforms(Vec::new());
    }

    /// Add a waveform trace and increment version
    pub fn add_waveform(&mut self, waveform: WaveformData) {
        let index = self.waveforms.len();
        self.node_to_waveform.insert(waveform.name.clone(), index);
        self.waveforms.push(waveform);
        self.data_version = self.data_version.wrapping_add(1);
    }

    /// Replace the displayed waveform set and rebuild cross-probe mappings.
    pub fn replace_waveforms(&mut self, waveforms: Vec<WaveformData>) {
        self.node_to_waveform.clear();
        self.waveforms = waveforms;
        for (index, waveform) in self.waveforms.iter().enumerate() {
            self.node_to_waveform.insert(waveform.name.clone(), index);
        }
        self.data_version = self.data_version.wrapping_add(1);
    }

    /// Toggle visibility of a waveform by name, returns true if found
    /// Handles multiple naming conventions:
    /// - Exact match (e.g., "V(N001)" == "V(N001)")
    /// - Net name matching (e.g., "V(N001)" matches "N001")
    /// - N00X to numeric mapping (e.g., "V(N001)" matches "V(1)")
    pub fn toggle_waveform_visibility(&mut self, probe_name: &str) -> bool {
        // Try exact match first
        for wf in &mut self.waveforms {
            if wf.name.eq_ignore_ascii_case(probe_name) {
                wf.visible = !wf.visible;
                log::info!(
                    "Toggled waveform '{}' visibility to {}",
                    wf.name,
                    wf.visible
                );
                return true;
            }
        }

        // Extract net name from V()/I()
        let net_name = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');

        // Try matching net name inside V() or I()
        for wf in &mut self.waveforms {
            let wf_net = wf
                .name
                .trim_start_matches("V(")
                .trim_start_matches("I(")
                .trim_end_matches(')');

            if wf_net.eq_ignore_ascii_case(net_name) {
                wf.visible = !wf.visible;
                log::info!(
                    "Toggled waveform '{}' (matched '{}') visibility to {}",
                    wf.name,
                    probe_name,
                    wf.visible
                );
                return true;
            }
        }

        // Handle N00X -> numeric index mapping
        // The netlist generator creates N001, N002, etc. but the simulation
        // engine uses internal numeric indices like 1, 2, 3
        if let Some(numeric_index) = Self::extract_n00x_numeric(net_name) {
            for wf in &mut self.waveforms {
                let wf_net = wf
                    .name
                    .trim_start_matches("V(")
                    .trim_start_matches("I(")
                    .trim_end_matches(')');

                if wf_net == numeric_index {
                    wf.visible = !wf.visible;
                    log::info!(
                        "Toggled waveform '{}' (N00X matched '{}') visibility to {}",
                        wf.name,
                        probe_name,
                        wf.visible
                    );
                    return true;
                }
            }
        }

        // Check if this is the ground reference node
        let net_name_check = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');

        if let Some(ref ground) = self.ground_node
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
            self.waveforms.len()
        );
        false
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

    /// Find waveform names that match a node/net name
    pub fn find_waveforms_for_node(&self, node_name: &str) -> Vec<String> {
        self.waveforms
            .iter()
            .filter(|wf| {
                let wf_net = wf
                    .name
                    .trim_start_matches("V(")
                    .trim_start_matches("I(")
                    .trim_end_matches(')');
                wf_net.eq_ignore_ascii_case(node_name)
            })
            .map(|wf| wf.name.clone())
            .collect()
    }

    // =========================================================================
    // Multi-Run Results Management (Cadence Spectre PSF-style)
    // =========================================================================

    /// Start a new simulation run, returning the new run
    ///
    /// This creates a new SimulationRun with an auto-incremented ID and
    /// prepares it for receiving analysis results.
    pub fn start_run(&mut self) -> &mut SimulationRun {
        self.start_run_with_receipt(None)
    }

    /// Start a run already sealed by the exact consumed prepared snapshot.
    pub(crate) fn start_prepared_run(&mut self, receipt: PreparedRunReceipt) -> &mut SimulationRun {
        self.start_run_with_receipt(Some(receipt))
    }

    fn start_run_with_receipt(
        &mut self,
        receipt: Option<PreparedRunReceipt>,
    ) -> &mut SimulationRun {
        self.next_run_id += 1;
        let run = match receipt {
            Some(receipt) => SimulationRun::new_prepared(self.next_run_id, receipt),
            None => SimulationRun::new(self.next_run_id),
        };

        // Insert at front (newest first)
        self.runs.insert(0, run);

        // Set as active run
        self.active_run_idx = Some(0);
        self.active_analysis_idx = None;

        // Prune history if needed
        self.prune_runs_history();
        self.prune_overlay_dataset_ids();

        // Return mutable reference to the new run
        &mut self.runs[0]
    }

    /// Complete the current run and update legacy waveforms for compatibility
    ///
    /// This syncs the new run-based results with the legacy flat waveforms list
    /// so existing waveform viewer code continues to work.
    pub fn complete_run(&mut self) {
        if let Some(run_idx) = self.active_run_idx
            && let Some(run) = self.runs.get(run_idx)
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
        if run_idx < self.runs.len() {
            self.active_run_idx = Some(run_idx);
            self.active_analysis_idx = None;

            let has_analyses = self
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
        if let Some(run_idx) = self.active_run_idx
            && let Some(run) = self.runs.get(run_idx)
            && analysis_idx < run.analyses.len()
        {
            self.active_analysis_idx = Some(analysis_idx);
            self.sync_selected_analysis_waveforms();
            return true;
        }
        false
    }

    /// Select the most recently added analysis in the active run.
    pub fn select_latest_analysis(&mut self) -> bool {
        let Some(run_idx) = self.active_run_idx else {
            return false;
        };
        let Some(last_idx) = self
            .runs
            .get(run_idx)
            .and_then(|run| run.analyses.len().checked_sub(1))
        else {
            return false;
        };
        self.active_analysis_idx = Some(last_idx);
        self.sync_selected_analysis_waveforms();
        true
    }

    /// Get the currently active run (if any)
    pub fn active_run(&self) -> Option<&SimulationRun> {
        self.active_run_idx.and_then(|idx| self.runs.get(idx))
    }

    // =========================================================================
    // Run overlay (signal owns hue, run owns weight)
    // =========================================================================

    /// Look up a run by its legacy display sequence.
    ///
    /// New persistence and cross-object references must use
    /// [`Self::run_by_stable_id`]. This sequence lookup remains while runner
    /// internals migrate independently from customer-visible history.
    pub fn run_by_sequence(&self, run_sequence: u64) -> Option<&SimulationRun> {
        self.runs.iter().find(|run| run.id == run_sequence)
    }

    /// Look up a mutable run by its legacy display sequence.
    pub fn run_by_sequence_mut(&mut self, run_sequence: u64) -> Option<&mut SimulationRun> {
        self.runs.iter_mut().find(|run| run.id == run_sequence)
    }

    /// Look up a run by its stable product identity.
    pub fn run_by_stable_id(&self, run_id: RunId) -> Option<&SimulationRun> {
        self.runs.iter().find(|run| run.run_id == run_id)
    }

    /// Look up the run that owns an immutable result dataset.
    pub fn run_by_dataset_id(&self, dataset_id: DatasetId) -> Option<&SimulationRun> {
        self.runs.iter().find(|run| run.dataset_id == dataset_id)
    }

    /// Select a run by its legacy display sequence.
    pub fn select_run_by_sequence(&mut self, run_sequence: u64) -> bool {
        let Some(run_idx) = self.runs.iter().position(|run| run.id == run_sequence) else {
            return false;
        };
        self.select_run(run_idx)
    }

    /// Select the latest analysis in a run addressed by display sequence.
    pub fn select_latest_analysis_in_run_sequence(&mut self, run_sequence: u64) -> bool {
        let Some(run_idx) = self.runs.iter().position(|run| run.id == run_sequence) else {
            return false;
        };
        let Some(last_idx) = self
            .runs
            .get(run_idx)
            .and_then(|run| run.analyses.len().checked_sub(1))
        else {
            return false;
        };
        self.active_run_idx = Some(run_idx);
        self.active_analysis_idx = Some(last_idx);
        self.sync_selected_analysis_waveforms();
        true
    }

    /// Whether a dataset is currently overlaid onto the active dataset.
    pub fn is_dataset_overlaid(&self, dataset_id: DatasetId) -> bool {
        self.overlay_dataset_ids.contains(&dataset_id)
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
            .overlay_dataset_ids
            .iter()
            .position(|id| *id == dataset_id)
        {
            self.overlay_dataset_ids.remove(pos);
            self.data_version = self.data_version.wrapping_add(1);
            false
        } else if self.run_by_dataset_id(dataset_id).is_some() {
            self.overlay_dataset_ids.push(dataset_id);
            self.data_version = self.data_version.wrapping_add(1);
            true
        } else {
            false
        }
    }

    /// Remove every overlay toggle in one action.
    pub fn clear_dataset_overlays(&mut self) {
        if !self.overlay_dataset_ids.is_empty() {
            self.overlay_dataset_ids.clear();
            self.data_version = self.data_version.wrapping_add(1);
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
        for run in &self.runs {
            if Some(run.dataset_id) != active_id
                && self.overlay_dataset_ids.contains(&run.dataset_id)
            {
                out.push(run);
            }
        }
        out
    }

    /// Drop overlay IDs whose runs have left the history.
    fn prune_overlay_dataset_ids(&mut self) {
        self.overlay_dataset_ids
            .retain(|id| self.runs.iter().any(|run| run.dataset_id == *id));
    }

    /// Get the currently active analysis (if any)
    pub fn active_analysis(&self) -> Option<&AnalysisResult> {
        self.active_run().and_then(|run| {
            self.active_analysis_idx
                .and_then(|idx| run.analyses.get(idx))
        })
    }

    /// Get mutable reference to the currently active run
    pub fn active_run_mut(&mut self) -> Option<&mut SimulationRun> {
        self.active_run_idx.and_then(|idx| self.runs.get_mut(idx))
    }

    /// Check if there are any runs with results
    pub fn has_results(&self) -> bool {
        !self.runs.is_empty()
    }

    /// Get count of runs in history
    pub fn run_count(&self) -> usize {
        self.runs.len()
    }

    /// Clear all runs history
    pub fn clear_runs(&mut self) {
        self.runs.clear();
        self.active_run_idx = None;
        self.active_analysis_idx = None;
        self.overlay_dataset_ids.clear();
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
        self.runs = runs;
        self.prune_runs_history();

        let max_run_id = self.runs.iter().map(|run| run.id).max().unwrap_or(0);
        self.next_run_id = next_run_id.max(max_run_id);

        self.active_run_idx = match (active_run_id, active_dataset_id) {
            (Some(run_id), Some(dataset_id)) => self
                .runs
                .iter()
                .position(|run| run.run_id == run_id && run.dataset_id == dataset_id),
            (Some(run_id), None) => self.runs.iter().position(|run| run.run_id == run_id),
            (None, Some(dataset_id)) => self
                .runs
                .iter()
                .position(|run| run.dataset_id == dataset_id),
            (None, None) => None,
        }
        .or_else(|| (!self.runs.is_empty()).then_some(0));

        self.active_analysis_idx = self.active_run_idx.and_then(|run_idx| {
            let run = &self.runs[run_idx];
            active_analysis_sequence
                .and_then(|id| run.analyses.iter().position(|analysis| analysis.id == id))
                .or_else(|| (!run.analyses.is_empty()).then_some(0))
        });

        let active_id = self.active_run().map(|run| run.dataset_id);
        self.overlay_dataset_ids.clear();
        for id in overlay_dataset_ids {
            if Some(id) != active_id
                && self.runs.iter().any(|run| run.dataset_id == id)
                && !self.overlay_dataset_ids.contains(&id)
            {
                self.overlay_dataset_ids.push(id);
            }
        }

        self.sync_selected_analysis_waveforms();
    }

    /// Prune runs history to stay within MAX_RUN_HISTORY limit
    fn prune_runs_history(&mut self) {
        while self.runs.len() > MAX_RUN_HISTORY {
            self.runs.pop(); // Remove oldest (last in list)
        }
    }

    /// Delete a specific run by index
    ///
    /// Returns true if the run was deleted.
    pub fn delete_run(&mut self, run_idx: usize) -> bool {
        if run_idx < self.runs.len() {
            self.runs.remove(run_idx);
            self.prune_overlay_dataset_ids();
            self.data_version = self.data_version.wrapping_add(1);

            // Adjust active indices
            if let Some(active) = self.active_run_idx {
                if active == run_idx {
                    if self.runs.is_empty() {
                        // Deleted final run: clear active selection and displayed waveform data.
                        self.active_run_idx = None;
                        self.active_analysis_idx = None;
                        self.sync_selected_analysis_waveforms();
                    } else {
                        // Deleted active run: select the new head run and synchronize displayed data.
                        let _ = self.select_run(0);
                    }
                } else if active > run_idx {
                    // Shift active index down
                    self.active_run_idx = Some(active - 1);
                }
            }
            true
        } else {
            false
        }
    }

    fn sync_selected_analysis_waveforms(&mut self) {
        let selected_waveforms = self
            .active_run_idx
            .and_then(|run_idx| self.runs.get(run_idx))
            .and_then(|run| {
                self.active_analysis_idx
                    .and_then(|analysis_idx| run.analyses.get(analysis_idx))
            })
            .map(|analysis| analysis.waveforms.clone())
            .unwrap_or_default();

        self.replace_waveforms(selected_waveforms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(state.overlay_dataset_ids, vec![overlay_dataset_id]);
        assert_eq!(state.waveforms[0].name, "V(two)");
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
        state.runs = vec![active, overlay];
        state.active_run_idx = Some(0);

        assert!(!state.toggle_dataset_overlay(active_id));
        assert!(!state.toggle_dataset_overlay(DatasetId::new()));
        assert!(state.toggle_dataset_overlay(overlay_id));
        assert!(state.is_dataset_overlaid(overlay_id));
        assert!(!state.toggle_dataset_overlay(overlay_id));
        assert!(!state.is_dataset_overlaid(overlay_id));
    }
}
