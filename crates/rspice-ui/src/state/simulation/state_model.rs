use super::*;

//=============================================================================
// Simulation State
//=============================================================================

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

    /// Current simulation progress (0.0 to 1.0)
    pub progress: f64,

    /// Status message
    pub status: String,

    /// Waveform data for display
    pub waveforms: Vec<WaveformData>,

    /// Data version counter - incremented whenever waveforms change.
    /// Used by the waveform viewer to detect when to reload traces.
    pub data_version: u64,

    /// Console log messages
    pub console_messages: Vec<ConsoleMessage>,

    /// Current netlist content (from editor)
    pub netlist_content: String,

    /// Current file path (if opened/saved)
    pub current_file: Option<PathBuf>,

    /// Whether the editor content has unsaved changes
    pub is_dirty: bool,

    /// Mapping from netlist node names (N001, N002) to waveform indices
    /// Populated after simulation to enable accurate probing
    pub node_to_waveform: HashMap<String, usize>,

    /// The node selected as ground reference (0V)
    /// When probing this node, we show a message that it's the ground reference
    pub ground_node: Option<String>,

    /// Cross-probing mapping between schematic grid points and SPICE net names
    /// Populated during netlist generation, used for probe mode
    pub cross_probe: CrossProbeMapping,

    /// DC operating point annotations for display on schematic
    /// Populated after DC OP simulation, shows node voltages and branch currents
    pub dc_annotations: DcAnnotationState,

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
}
