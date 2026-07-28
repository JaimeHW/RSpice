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

    /// Yield analysis results from Monte Carlo runs
    pub yield_results: Vec<YieldResult>,

    /// Stable identity of the run and dataset from which `yield_results` were
    /// calculated. `None` means no yield evidence is currently retained.
    pub yield_provenance: Option<YieldAnalysisProvenance>,

    /// Current optimizer state (if running)
    pub optimizer_state: Option<OptimizerState>,

    // =========================================================================
    // Multi-Run Results History (Cadence Spectre PSF-style)
    // =========================================================================
    /// Simulation run history (newest first, limited to MAX_RUN_HISTORY)
    pub runs: Vec<SimulationRun>,

    /// Next run ID to assign (monotonically increasing)
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
    /// Newest complete OP state from the unchanged project revision. The
    /// sealed source digest is carried into the task and checked again during
    /// preflight, so this early selection cannot authorize stale topology.
    pub(crate) fn newest_retained_op_state(
        &self,
        project_revision: crate::product::ObjectRevision,
    ) -> Option<crate::simulation::dialog::OpPreviousState> {
        self.runs.iter().find_map(|run| {
            let receipt = run.prepared_receipt()?;
            if receipt.project_revision() != project_revision {
                return None;
            }
            run.analyses.iter().rev().find_map(|analysis| {
                if !analysis.success || analysis.analysis_type != AnalysisType::DcOp {
                    return None;
                }
                let provenance = analysis.provenance()?;
                let AnalysisResultPayload::OperatingPoint {
                    mna_node_names,
                    mna_branch_names,
                    mna_solution,
                    effective_source_content_digest: Some(effective_source_content_digest),
                    ..
                } = analysis.result_payload.as_ref()?
                else {
                    return None;
                };
                if mna_solution.is_empty()
                    || mna_node_names.len().saturating_add(mna_branch_names.len())
                        != mna_solution.len()
                {
                    return None;
                }
                Some(crate::simulation::dialog::OpPreviousState {
                    source_content_digest: *effective_source_content_digest,
                    producer_snapshot_digest: provenance.prepared_snapshot_digest(),
                    producer_result_digest: analysis.result_data_digest(),
                    node_names: mna_node_names.clone(),
                    branch_names: mna_branch_names.clone(),
                    solution: mna_solution.clone(),
                })
            })
        })
    }

    /// Canonical devices with retained SOA warning/violation evidence in the
    /// active run. This is the sole authority for OP `Violations only`.
    pub(crate) fn active_soa_violation_context(
        &self,
        project_revision: crate::product::ObjectRevision,
    ) -> Option<(crate::product::ContentDigest, Vec<String>)> {
        let Some(run) = self.active_run() else {
            return None;
        };
        let receipt = run.prepared_receipt()?;
        if receipt.project_revision() != project_revision {
            return None;
        }
        let mut devices = run
            .analyses
            .iter()
            .flat_map(|analysis| match analysis.result_payload.as_ref() {
                Some(AnalysisResultPayload::Soa { violations, .. }) => violations
                    .iter()
                    .map(|violation| violation.device_id.clone())
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect::<Vec<_>>();
        devices.sort();
        devices.dedup();
        (!devices.is_empty()).then(|| (receipt.source_content_digest(), devices))
    }
}
