use super::*;

impl SimulationState {
    /// Add a console message
    pub fn log(&mut self, message: ConsoleMessage) {
        self.console_messages.push(message);
    }

    /// Clear console messages
    pub fn clear_console(&mut self) {
        self.console_messages.clear();
    }

    /// Clear waveforms and increment version
    pub fn clear_waveforms(&mut self) {
        self.waveforms.clear();
        self.data_version = self.data_version.wrapping_add(1);
    }

    /// Add a waveform trace and increment version
    pub fn add_waveform(&mut self, waveform: WaveformData) {
        self.waveforms.push(waveform);
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

        if let Some(ref ground) = self.ground_node {
            if ground.eq_ignore_ascii_case(net_name_check) {
                log::info!(
                    "Probe '{}' is the ground reference (0V) - no waveform displayed",
                    probe_name
                );
                return false;
            }
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
        self.next_run_id += 1;
        let run = SimulationRun::new(self.next_run_id);

        // Insert at front (newest first)
        self.runs.insert(0, run);

        // Set as active run
        self.active_run_idx = Some(0);
        self.active_analysis_idx = None;

        // Prune history if needed
        self.prune_runs_history();

        // Return mutable reference to the new run
        &mut self.runs[0]
    }

    /// Complete the current run and update legacy waveforms for compatibility
    ///
    /// This syncs the new run-based results with the legacy flat waveforms list
    /// so existing waveform viewer code continues to work.
    pub fn complete_run(&mut self) {
        if let Some(run_idx) = self.active_run_idx {
            if let Some(run) = self.runs.get(run_idx) {
                // Auto-select first analysis if available - this will sync only that analysis's waveforms
                if !run.analyses.is_empty() {
                    // Use select_analysis to properly load only the selected analysis's waveforms
                    self.select_analysis(0);
                }
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

            // Auto-select first analysis in this run (this will sync only that analysis's waveforms)
            if let Some(run) = self.runs.get(run_idx) {
                if !run.analyses.is_empty() {
                    self.select_analysis(0);
                }
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
        if let Some(run_idx) = self.active_run_idx {
            if let Some(run) = self.runs.get(run_idx) {
                if analysis_idx < run.analyses.len() {
                    self.active_analysis_idx = Some(analysis_idx);

                    // Sync waveforms from selected analysis only
                    self.waveforms.clear();
                    if let Some(analysis) = run.analyses.get(analysis_idx) {
                        for wf in &analysis.waveforms {
                            self.waveforms.push(wf.clone());
                        }
                    }
                    self.data_version = self.data_version.wrapping_add(1);
                    return true;
                }
            }
        }
        false
    }

    /// Get the currently active run (if any)
    pub fn active_run(&self) -> Option<&SimulationRun> {
        self.active_run_idx.and_then(|idx| self.runs.get(idx))
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
        // Don't reset next_run_id to preserve uniqueness
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

            // Adjust active indices
            if let Some(active) = self.active_run_idx {
                if active == run_idx {
                    // Deleted the active run, select newest if available
                    self.active_run_idx = if self.runs.is_empty() { None } else { Some(0) };
                    self.active_analysis_idx = None;
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
}
