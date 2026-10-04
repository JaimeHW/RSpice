//! Application composition of execution, retained evidence, source and viewer state.

use super::*;
use crate::product::{DatasetId, RunId};

/// Coordinates operations that cross simulation state owners.
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    pub execution: SimulationExecutionState,
    pub retained: RetainedSimulationState,
    pub source: SimulationSourceState,
    pub view: SimulationViewState,
}

/// Runtime run requests, cancellation identity and observed controller progress.
#[derive(Debug, Clone, Default)]
pub struct SimulationExecutionState {
    /// Whether a simulation is currently running
    pub is_running: bool,

    /// Flag to trigger simulation from menu (toolbar watches this)
    /// When set to true, toolbar will start simulation and reset to false
    pub trigger_simulation: bool,

    /// Flag to trigger simulation abort from stop button
    /// When set to true, SimulationController will call runner.abort() and reset to false
    pub trigger_abort: bool,

    /// Stable identity of the run currently owned by the execution engine.
    /// Runtime-only; project history persists identity on the run itself.
    pub active_execution: Option<SimulationExecutionIdentity>,

    /// Identity-bound cancellation request awaiting controller processing.
    /// This prevents a delayed stop action from cancelling a replacement run.
    pub abort_request: Option<SimulationExecutionIdentity>,

    /// Which workflow requested the next simulation start.
    pub run_intent: SimulationRunIntent,

    /// Current simulation progress (0.0 to 1.0)
    pub progress: f64,

    /// Status message
    pub status: String,
}

/// Retained run evidence, imported checkpoints and retention policy.
#[derive(Debug, Clone, Default)]
pub struct RetainedSimulationState {
    /// Atomically replaced yield population and the run that produced it.
    pub yield_evidence: YieldEvidence,

    /// Simulation run history (newest first, limited to
    /// [`Self::retained_dataset_limit`])
    pub runs: crate::state::RunHistory,

    /// Imported resume inputs, preserved independently of dataset pruning.
    pub imported_monte_carlo_checkpoints: MonteCarloCheckpointLibrary,

    /// The exact deck every point of a recent run executed.
    ///
    /// Persisted alongside run history and bounded independently. Readers verify
    /// the retained bytes against the run's prepared source digest. See
    /// [`ExecutedDeckArchive`].
    pub executed_decks: ExecutedDeckArchive,

    /// How many datasets the project keeps before the oldest unpinned one is
    /// discarded.
    ///
    /// `None` means the built-in default. This is a real limit that silently
    /// destroyed work before it was surfaced: a project that ran twenty-one
    /// times lost its first dataset with no warning and no way to ask for
    /// more. Saying the number, and letting the reader raise it, is the
    /// difference between a retention policy and data loss.
    pub retained_dataset_limit: Option<usize>,

    /// Last allocated display sequence; zero before the first run. The legacy
    /// field name is retained, but allocation increments it before assigning.
    pub next_run_id: u64,
}

/// Working netlist buffer and its schematic cross-probe mapping.
#[derive(Debug, Clone, Default)]
pub struct SimulationSourceState {
    /// Current netlist content (from editor)
    pub netlist_content: String,

    /// Cross-probing mapping between schematic grid points and SPICE net names
    /// Populated during netlist generation, used for probe mode
    pub cross_probe: CrossProbeMapping,
}

/// Displayed waveforms, probe lookup and result selection.
/// Selection and overlay IDs retain their existing project persistence policy.
#[derive(Debug, Clone, Default)]
pub struct SimulationViewState {
    /// Waveform data for display
    pub waveforms: Vec<WaveformData>,

    /// Data version counter - incremented whenever waveforms change.
    /// Used by the waveform viewer to detect when to reload traces.
    pub data_version: u64,

    /// Mapping from netlist node names (N001, N002) to waveform indices
    /// Populated after simulation to enable accurate probing
    pub node_to_waveform: HashMap<String, usize>,

    /// The node selected as ground reference (0V)
    /// When probing this node, we show a message that it's the ground reference
    pub ground_node: Option<String>,

    /// Currently selected run index in the Results Browser
    pub active_run_idx: Option<usize>,

    /// Currently selected analysis index within the active run
    pub active_analysis_idx: Option<usize>,

    /// Stable dataset IDs overlaid onto the active dataset in result viewers.
    ///
    /// Overlay grammar: *signal owns hue, run owns weight* — a signal keeps
    /// one trace color across every run; the active run draws at full
    /// strength and overlaid runs at reduced alpha/stroke. IDs that leave
    /// the history are pruned automatically.
    pub overlay_dataset_ids: Vec<DatasetId>,
}

impl SimulationState {
    /// Canonical devices with retained SOA warning/violation evidence in the
    /// active run. This is the sole authority for OP `Violations only`.
    pub(crate) fn active_soa_violation_context(
        &self,
        project_revision: crate::product::ObjectRevision,
    ) -> Option<(crate::product::ContentDigest, Vec<String>)> {
        self.active_run()?.soa_violation_context(project_revision)
    }
}

impl SimulationExecutionState {
    /// Whether an execution still owns mutable simulation state.
    ///
    /// `is_running` is the runner's instantaneous worker activity and can
    /// become false before the controller has polled and sealed its result.
    /// Product controls must therefore treat the stable execution identity as
    /// authoritative and retain the legacy flag only as a fail-safe fallback.
    #[must_use]
    pub fn has_active_execution(&self) -> bool {
        self.active_execution.is_some() || self.is_running
    }

    pub fn request_simulate_run_set(&mut self) {
        self.run_intent = SimulationRunIntent::SimulateRunSet;
        self.trigger_simulation = true;
    }

    pub fn request_manual_deck_run(&mut self) {
        self.run_intent = SimulationRunIntent::ManualDeck;
        self.trigger_simulation = true;
    }
}

impl RetainedSimulationState {
    #[must_use]
    pub fn yield_provenance(&self) -> Option<YieldAnalysisProvenance> {
        self.yield_evidence.provenance()
    }

    /// Yield evidence for one exact immutable dataset, if that dataset is the
    /// authority recorded when the evidence was calculated.
    #[must_use]
    pub fn yield_results_for_dataset(&self, dataset_id: DatasetId) -> Option<&[YieldResult]> {
        self.runs
            .iter()
            .find(|run| run.dataset_id == dataset_id)
            .and_then(|run| self.yield_evidence.for_run_ids(run.run_id, run.dataset_id))
    }

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

    /// Mutably look up a run by its stable product identity.
    pub fn run_by_stable_id_mut(&mut self, run_id: RunId) -> Option<&mut SimulationRun> {
        self.runs.iter_mut().find(|run| run.run_id == run_id)
    }

    /// Look up the run that owns an immutable result dataset.
    pub fn run_by_dataset_id(&self, dataset_id: DatasetId) -> Option<&SimulationRun> {
        self.runs.iter().find(|run| run.dataset_id == dataset_id)
    }

    /// Check if there are any runs with results
    pub fn has_results(&self) -> bool {
        !self.runs.is_empty()
    }

    /// Whether at least one immutable run has materialized an analysis
    /// document. A newly allocated or still-empty execution record is not a
    /// result dataset and must not enable result-only workbench surfaces.
    pub fn has_retained_result_dataset(&self) -> bool {
        self.newest_retained_result_run_index().is_some()
    }

    /// Index of the newest run that owns at least one retained analysis.
    /// Run history is newest-first, so this is also the exact dataset the
    /// split-results stage should initially track.
    pub fn newest_retained_result_run_index(&self) -> Option<usize> {
        self.runs.newest_retained_result_run_index()
    }

    /// Get count of runs in history
    #[cfg(test)]
    pub fn run_count(&self) -> usize {
        self.runs.len()
    }

    /// How many retained datasets are pinned as golden baselines.
    #[must_use]
    #[cfg(test)]
    pub fn pinned_run_count(&self) -> usize {
        self.runs
            .iter()
            .filter(|run| run.retention().is_pinned())
            .count()
    }

    #[must_use]
    pub fn retained_plan_dataset_count(&self, plan_id: crate::product::SimulationPlanId) -> usize {
        self.runs.retained_plan_dataset_count(plan_id)
    }

    #[must_use]
    pub fn pinned_plan_run_count(&self, plan_id: crate::product::SimulationPlanId) -> usize {
        self.runs.pinned_plan_run_count(plan_id)
    }

    /// The retention limit in force, resolved from the project's own setting.
    ///
    /// Clamped to at least one: a limit of zero would discard the dataset the
    /// run just produced, which is never what "retain fewer" means.
    pub fn effective_retained_dataset_limit(&self) -> usize {
        self.retained_dataset_limit
            .unwrap_or(MAX_RUN_HISTORY)
            .max(1)
    }

    pub(crate) fn has_retained_op_state(
        &self,
        project_revision: crate::product::ObjectRevision,
        allow_changed_revision: bool,
    ) -> bool {
        rspice_results::run_history::has_retained_op_state(
            self.runs.iter().map(|run| &run.data),
            project_revision,
            allow_changed_revision,
        )
    }
}

impl SimulationViewState {
    /// Replace the displayed waveform set and rebuild cross-probe mappings.
    pub fn replace_waveforms(&mut self, waveforms: Vec<WaveformData>) {
        self.node_to_waveform.clear();
        self.waveforms = waveforms;
        for (index, waveform) in self.waveforms.iter().enumerate() {
            self.node_to_waveform.insert(waveform.name.clone(), index);
        }
        self.data_version = self.data_version.wrapping_add(1);
    }
}
