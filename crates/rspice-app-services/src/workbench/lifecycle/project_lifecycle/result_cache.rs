//! Result snapshots keyed by every input to their persisted projection.
//!
//! Retained run and deck owners provide mutation identities. Selection,
//! overlays, retention, and the sequence counter are compared by exact value.
//! An unchanged lookup only clones the immutable result snapshot handle.
//!
//! Two snapshots are held. A save, a session and every decision read the
//! whole one, which records every run's signals. The dirty check reads the
//! one of the runs' headers: a run is immutable once written, so its
//! identity is what tells one retained history from another, and a run that
//! ends, however many signals it wrote, is compared without copying or
//! digesting any of them.

use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::io::ProjectSimulationResults;
use crate::product::DatasetId;
use crate::state::{ExecutedDeckArchive, RunHistoryRevision, SimulationState};

#[derive(Debug, Clone, Default)]
pub struct ResultCache {
    /// The results as a save writes them.
    whole: RefCell<Option<CachedResults>>,
    /// The results as the dirty check reads them: each run by its header.
    headers: RefCell<Option<CachedHeaders>>,
}

/// Issues the stamp of each captured snapshot, unique for the process.
static NEXT_SNAPSHOT_STAMP: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
struct CachedResults {
    key: ResultKey,
    snapshot: ProjectSimulationResults,
}

#[derive(Debug, Clone)]
struct CachedHeaders {
    key: ResultKey,
    /// Names the results' identity: two caches holding the same stamp hold
    /// results that compare equal. Kept while the runs' headers do not
    /// change, so a run streaming its samples moves no stamp.
    stamp: u64,
    identity: Option<rspice_vocabulary::product::ContentDigest>,
    snapshot: ProjectSimulationResults,
}

/// Every input to the persisted projection, by identity or by value.
#[derive(Debug, Clone)]
struct ResultKey {
    history: RunHistoryRevision,
    decks: ExecutedDeckArchive,
    imported_checkpoints: crate::state::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    next_run_id: u64,
    retained_dataset_limit: Option<usize>,
    active_run_idx: Option<usize>,
    active_analysis_idx: Option<usize>,
    overlay_dataset_ids: Vec<DatasetId>,
}

impl ResultKey {
    fn of(state: &SimulationState) -> Self {
        Self {
            history: state.retained.runs.revision(),
            decks: state.retained.executed_decks.clone(),
            imported_checkpoints: state.retained.imported_monte_carlo_checkpoints.clone(),
            imported_transient_checkpoints: state.retained.imported_transient_checkpoints.clone(),
            next_run_id: state.retained.next_run_id,
            retained_dataset_limit: state.retained.retained_dataset_limit,
            active_run_idx: state.view.active_run_idx,
            active_analysis_idx: state.view.active_analysis_idx,
            overlay_dataset_ids: state.view.overlay_dataset_ids.clone(),
        }
    }

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
                    // Runtime: Preferences supplies it at each dispatch.
                    preferred_dataset_limit: _,
                    // Derived: each frame judges it from the runs and the
                    // plan's limits, both persisted, so it never is.
                    yield_evidence: _,
                    verification: _,
                    // Runtime: the model restates it from the saved
                    // documents before anything that may prune.
                    held_by_documents: _,
                    // Runtime: restorable until the project closes, never saved.
                    deleted_runs: _,
                    deleted_decks: _,
                },
            view:
                crate::state::SimulationViewState {
                    active_run_idx,
                    active_analysis_idx,
                    overlay_dataset_ids,
                    // Runtime: the reader's overlay setting for the session.
                    overlay_previous: _,
                    // Runtime: the analysis each run was last left on.
                    remembered_analyses: _,
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
                    reported_progress: _,
                    phase: _,
                    live_window: _,
                    live_preview_note: _,
                },
            source:
                crate::state::SimulationSourceState {
                    netlist_content: _,
                    cross_probe: _,
                    manual_deck_path: _,
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
    pub fn capture(&self, state: &SimulationState) -> ProjectSimulationResults {
        let mut cached = self.whole.borrow_mut();
        if let Some(held) = cached.as_ref().filter(|held| held.key.matches(state)) {
            return held.snapshot.clone();
        }
        cached
            .insert(CachedResults {
                key: ResultKey::of(state),
                snapshot: crate::io::capture_simulation_results(state),
            })
            .snapshot
            .clone()
    }

    /// The results with each run by its header, as the dirty check compares
    /// them ([`crate::io::capture_simulation_result_headers`]).
    pub fn headers(&self, state: &SimulationState) -> ProjectSimulationResults {
        self.with_headers(state, |held| held.snapshot.clone())
    }

    /// The stamp of the results' identity as `state` holds them: equal
    /// stamps, results that compare equal. Moves only when the runs' headers,
    /// the selection or another persisted input changed, never because a
    /// run's samples did.
    pub fn stamp(&self, state: &SimulationState) -> u64 {
        self.with_headers(state, |held| held.stamp)
    }

    fn with_headers<R>(
        &self,
        state: &SimulationState,
        read: impl FnOnce(&CachedHeaders) -> R,
    ) -> R {
        let mut cached = self.headers.borrow_mut();
        if let Some(held) = cached.as_ref().filter(|held| held.key.matches(state)) {
            return read(held);
        }
        let snapshot = crate::io::capture_simulation_result_headers(state);
        let identity = rspice_project::registry::digest(&snapshot.identity()).ok();
        let stamp = cached
            .as_ref()
            .filter(|held| identity.is_some() && held.identity == identity)
            .map_or_else(
                || NEXT_SNAPSHOT_STAMP.fetch_add(1, Ordering::Relaxed),
                |held| held.stamp,
            );
        read(cached.insert(CachedHeaders {
            key: ResultKey::of(state),
            stamp,
            identity,
            snapshot,
        }))
    }
}
