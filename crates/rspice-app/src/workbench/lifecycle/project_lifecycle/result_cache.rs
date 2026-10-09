//! Result snapshots keyed by every input to their persisted projection.
//!
//! Retained run and deck owners provide mutation identities. Selection,
//! overlays, retention, and the sequence counter are compared by exact value.
//! An unchanged lookup only clones the immutable result snapshot handle.

use std::cell::RefCell;

use crate::io::ProjectSimulationResults;
use crate::product::DatasetId;
use crate::state::{ExecutedDeckArchive, RunHistoryRevision, SimulationState};

#[derive(Debug, Clone, Default)]
pub(super) struct ResultCache(RefCell<Option<CachedResults>>);

#[derive(Debug, Clone)]
struct CachedResults {
    history: RunHistoryRevision,
    decks: ExecutedDeckArchive,
    imported_checkpoints: crate::state::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints: rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    next_run_id: u64,
    retained_dataset_limit: Option<usize>,
    active_run_idx: Option<usize>,
    active_analysis_idx: Option<usize>,
    overlay_dataset_ids: Vec<DatasetId>,
    snapshot: ProjectSimulationResults,
}

impl CachedResults {
    fn matches(&self, state: &SimulationState) -> bool {
        // Adding state requires an explicit persisted-versus-runtime choice.
        let SimulationState {
            retained:
                crate::state::RetainedSimulationState {
                    runs,
                    executed_decks,
                    imported_monte_carlo_checkpoints,
                    imported_transient_checkpoints,
                    next_run_id,
                    retained_dataset_limit,
                    yield_evidence: _,
                },
            view:
                crate::state::SimulationViewState {
                    active_run_idx,
                    active_analysis_idx,
                    overlay_dataset_ids,
                    waveforms: _,
                    data_version: _,
                    node_to_waveform: _,
                    ground_node: _,
                },
            execution:
                crate::state::SimulationExecutionState {
                    is_running: _,
                    trigger_simulation: _,
                    trigger_abort: _,
                    active_execution: _,
                    abort_request: _,
                    run_intent: _,
                    progress: _,
                    status: _,
                },
            source:
                crate::state::SimulationSourceState {
                    netlist_content: _,
                    cross_probe: _,
                },
        } = state;
        self.history == runs.revision()
            && self.decks.shares_content_with(executed_decks)
            && self
                .imported_checkpoints
                .shares_content_with(imported_monte_carlo_checkpoints)
            && self
                .imported_transient_checkpoints
                .shares_content_with(imported_transient_checkpoints)
            && self.next_run_id == *next_run_id
            && self.retained_dataset_limit == *retained_dataset_limit
            && self.active_run_idx == *active_run_idx
            && self.active_analysis_idx == *active_analysis_idx
            && self.overlay_dataset_ids == *overlay_dataset_ids
    }
}

impl ResultCache {
    pub(super) fn capture(&self, state: &SimulationState) -> ProjectSimulationResults {
        let mut cached = self.0.borrow_mut();
        if let Some(held) = cached.as_ref().filter(|held| held.matches(state)) {
            return held.snapshot.clone();
        }
        let snapshot = crate::io::capture_simulation_results(state);
        *cached = Some(CachedResults {
            history: state.retained.runs.revision(),
            decks: state.retained.executed_decks.clone(),
            imported_checkpoints: state.retained.imported_monte_carlo_checkpoints.clone(),
            imported_transient_checkpoints: state.retained.imported_transient_checkpoints.clone(),
            next_run_id: state.retained.next_run_id,
            retained_dataset_limit: state.retained.retained_dataset_limit,
            active_run_idx: state.view.active_run_idx,
            active_analysis_idx: state.view.active_analysis_idx,
            overlay_dataset_ids: state.view.overlay_dataset_ids.clone(),
            snapshot: snapshot.clone(),
        });
        snapshot
    }
}
