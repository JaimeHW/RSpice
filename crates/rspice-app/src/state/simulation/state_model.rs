//! The simulation state model.
//!
//! Everything a session holds about simulation: the active run, its
//! intent (generated schematic or manual deck), the results retained per
//! dataset, and the run history.

use super::*;
use crate::product::DatasetId;

//=============================================================================
// Simulation State
//=============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimulationRunIntent {
    #[default]
    SimulateRunSet,
    ManualDeck,
}

/// Simulation execution state
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
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

    /// Waveform data for display
    pub waveforms: Vec<WaveformData>,

    /// Data version counter - incremented whenever waveforms change.
    /// Used by the waveform viewer to detect when to reload traces.
    pub data_version: u64,

    /// Current netlist content (from editor)
    pub netlist_content: String,

    /// Mapping from netlist node names (N001, N002) to waveform indices
    /// Populated after simulation to enable accurate probing
    pub node_to_waveform: HashMap<String, usize>,

    /// The node selected as ground reference (0V)
    /// When probing this node, we show a message that it's the ground reference
    pub ground_node: Option<String>,

    /// Cross-probing mapping between schematic grid points and SPICE net names
    /// Populated during netlist generation, used for probe mode
    pub cross_probe: CrossProbeMapping,

    /// Atomically replaced yield population and the run that produced it.
    pub yield_evidence: YieldEvidence,

    // =========================================================================
    // Multi-Run Results History (Cadence Spectre PSF-style)
    // =========================================================================
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

    /// Canonical devices with retained SOA warning/violation evidence in the
    /// active run. This is the sole authority for OP `Violations only`.
    pub(crate) fn active_soa_violation_context(
        &self,
        project_revision: crate::product::ObjectRevision,
    ) -> Option<(crate::product::ContentDigest, Vec<String>)> {
        self.active_run()?.soa_violation_context(project_revision)
    }
}
