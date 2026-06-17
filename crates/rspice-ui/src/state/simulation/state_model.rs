use super::*;

//=============================================================================
// Simulation State
//=============================================================================

/// Requested simulation source for the next controller start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimulationRunIntent {
    /// Run the analyses configured in the Simulate workspace.
    #[default]
    RunSet,
    /// Run the text deck currently owned by the netlist editor.
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

    /// Source selected for the next simulation trigger.
    pub run_intent: SimulationRunIntent,

    /// Flag to trigger simulation abort from stop button
    /// When set to true, SimulationController will call runner.abort() and reset to false
    pub trigger_abort: bool,

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

    /// Current optimizer state (if running)
    pub optimizer_state: Option<OptimizerState>,

    /// Safety Operating Area violations
    pub soa_violations: Vec<SoAViolation>,

    /// Long-term reliability results
    pub reliability_results: Vec<ReliabilityResult>,

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

    /// Run IDs overlaid onto the active run in the results viewers.
    ///
    /// Overlay grammar: *signal owns hue, run owns weight* — a signal keeps
    /// one trace color across every run; the active run draws at full
    /// strength and overlaid runs at reduced alpha/stroke. IDs that leave
    /// the history are pruned automatically.
    pub overlay_run_ids: Vec<u64>,
}
