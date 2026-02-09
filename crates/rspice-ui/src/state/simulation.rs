//! Simulation State
//!
//! Manages simulation execution state and results.

use super::dc_annotation::{AnnotationMode, DcAnnotationState};
use super::schematic::Point;
use crate::services::safety::SoAViolation;
use crate::services::yield_manager::YieldResult;
use crate::simulation::optimizer::OptimizerState;
use crate::simulation::reliability_engine::ReliabilityResult;
use rspice_core::Value;
use std::collections::HashMap;
use std::path::PathBuf;

//=============================================================================
// Cross-Probing Infrastructure
//=============================================================================

/// Bidirectional mapping between schematic grid points and SPICE net names.
///
/// This enables professional-grade cross-probing between schematic and waveform viewer:
/// - Click on schematic wire → find corresponding waveform
/// - Click on waveform → highlight corresponding wire on schematic
///
/// The mapping is populated during netlist generation and persists until the next
/// simulation run or schematic modification.
#[derive(Debug, Clone, Default)]
pub struct CrossProbeMapping {
    /// Grid point to net name lookup (e.g., Point(280, 200) → "NET3")
    /// Enables: click on wire → find net name → find waveform
    pub point_to_net: HashMap<Point, String>,

    /// Net name to grid points lookup (e.g., "NET3" → [Point(280, 200), ...])
    /// Enables: select waveform → highlight all connected wire segments
    pub net_to_points: HashMap<String, Vec<Point>>,

    /// Version counter - incremented when mapping is updated
    /// Used to detect when probe cache needs refresh
    pub version: u64,
}

impl CrossProbeMapping {
    /// Create a new empty mapping
    pub fn new() -> Self {
        Self::default()
    }

    /// Update mapping from netlist generation result
    pub fn update(
        &mut self,
        point_to_net: HashMap<Point, String>,
        net_to_points: HashMap<String, Vec<Point>>,
    ) {
        self.point_to_net = point_to_net;
        self.net_to_points = net_to_points;
        self.version += 1;
    }

    /// Clear the mapping
    pub fn clear(&mut self) {
        self.point_to_net.clear();
        self.net_to_points.clear();
        self.version += 1;
    }

    /// Look up net name for a grid point
    ///
    /// Returns None if the point is not on a net (e.g., empty space)
    pub fn net_at(&self, point: Point) -> Option<&String> {
        self.point_to_net.get(&point)
    }

    /// Look up all grid points for a net name
    ///
    /// Returns empty slice if net not found
    pub fn points_for_net(&self, net_name: &str) -> &[Point] {
        self.net_to_points
            .get(net_name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Check if mapping is populated
    pub fn is_populated(&self) -> bool {
        !self.point_to_net.is_empty()
    }
}

//=============================================================================
// Simulation Run Database (Cadence Spectre PSF-style)
//=============================================================================
//
// This implements a multi-run simulation history following commercial EDA patterns:
// - Each simulation run is timestamped and contains multiple analyses
// - Runs are stored in reverse chronological order (newest first)
// - Maximum history size prevents unbounded memory growth
// - Analysis-aware metadata enables type-specific viewers

/// Maximum number of simulation runs to retain in history
pub const MAX_RUN_HISTORY: usize = 20;

/// Analysis type identifier for viewer selection and result organization.
///
/// Each analysis type maps to a specialized viewer:
/// - DcOp → Operating Point Table
/// - DcSweep → DC Sweep Plot
/// - Ac → Bode Plot (magnitude/phase)
/// - Transient → Time-domain Waveform Viewer
/// - Noise → Noise Spectrum Plot
/// - PoleZero → S-plane Pole-Zero Diagram
/// - Sensitivity → Parameter Sensitivity Bar Chart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisType {
    /// DC operating point analysis - node voltages and branch currents
    DcOp,
    /// DC sweep analysis - parameter sweep with DC solution at each point
    DcSweep,
    /// AC small-signal frequency response analysis
    Ac,
    /// Time-domain transient analysis
    Transient,
    /// Noise analysis - noise spectral density vs frequency
    Noise,
    /// Pole-zero analysis - transfer function poles and zeros
    PoleZero,
    /// Transfer function analysis
    Tf,
    /// Sensitivity analysis - output sensitivity to parameters
    Sensitivity,
    /// Periodic AC analysis
    Pac,
    /// Periodic noise analysis
    Pnoise,
    /// Periodic transfer function analysis
    Pxf,
    /// Periodic stability analysis
    Pstb,
    /// Stability analysis
    Stb,
    /// Monte Carlo statistical analysis
    MonteCarlo,
    /// Parametric sweep analysis
    Parametric,
    /// Corner sweep analysis
    Corner,
    /// Reliability aging analysis
    Reliability,
    /// Optimization analysis
    Optimization,
    /// Safety/SOA analysis
    Soa,
    /// S-parameter analysis
    SParameter,
    /// Envelope analysis
    Envelope,
    /// Fourier analysis
    Fourier,
    /// Harmonic Balance analysis for RF circuits
    HarmonicBalance,
    /// Periodic Steady State analysis
    Pss,
}

impl AnalysisType {
    /// Get human-readable display name for this analysis type
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => "DC Operating Point",
            AnalysisType::DcSweep => "DC Sweep",
            AnalysisType::Ac => "AC Analysis",
            AnalysisType::Transient => "Transient",
            AnalysisType::Noise => "Noise",
            AnalysisType::PoleZero => "Pole-Zero",
            AnalysisType::Tf => "Transfer Function",
            AnalysisType::Sensitivity => "Sensitivity",
            AnalysisType::Pac => "PAC",
            AnalysisType::Pnoise => "PNoise",
            AnalysisType::Pxf => "PXF",
            AnalysisType::Pstb => "PSTB",
            AnalysisType::Stb => "STB",
            AnalysisType::MonteCarlo => "Monte Carlo",
            AnalysisType::Parametric => "Parametric Sweep",
            AnalysisType::Corner => "Corner Sweep",
            AnalysisType::Reliability => "Reliability",
            AnalysisType::Optimization => "Optimization",
            AnalysisType::Soa => "Safety (SOA)",
            AnalysisType::SParameter => "S-Parameter",
            AnalysisType::Envelope => "Envelope",
            AnalysisType::Fourier => "Fourier",
            AnalysisType::HarmonicBalance => "Harmonic Balance",
            AnalysisType::Pss => "PSS",
        }
    }

    /// Get short icon-friendly label
    pub fn short_label(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => "DC",
            AnalysisType::DcSweep => "DCS",
            AnalysisType::Ac => "AC",
            AnalysisType::Transient => "TR",
            AnalysisType::Noise => "NS",
            AnalysisType::PoleZero => "PZ",
            AnalysisType::Tf => "TF",
            AnalysisType::Sensitivity => "SN",
            AnalysisType::Pac => "PAC",
            AnalysisType::Pnoise => "PN",
            AnalysisType::Pxf => "PXF",
            AnalysisType::Pstb => "PSTB",
            AnalysisType::Stb => "STB",
            AnalysisType::MonteCarlo => "MC",
            AnalysisType::Parametric => "PAR",
            AnalysisType::Corner => "CRN",
            AnalysisType::Reliability => "REL",
            AnalysisType::Optimization => "OPT",
            AnalysisType::Soa => "SOA",
            AnalysisType::SParameter => "SP",
            AnalysisType::Envelope => "ENV",
            AnalysisType::Fourier => "FOU",
            AnalysisType::HarmonicBalance => "HB",
            AnalysisType::Pss => "PSS",
        }
    }

    /// Get axis labels and units for this analysis type
    ///
    /// Returns (x_axis_label, x_axis_unit, y_axis_label, y_axis_unit)
    pub fn axis_info(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            AnalysisType::Transient
            | AnalysisType::Pss
            | AnalysisType::Envelope
            | AnalysisType::Soa => ("Time", "s", "Voltage", "V"),
            AnalysisType::Ac
            | AnalysisType::Noise
            | AnalysisType::Tf
            | AnalysisType::Pac
            | AnalysisType::Pxf
            | AnalysisType::Pstb
            | AnalysisType::Stb
            | AnalysisType::SParameter
            | AnalysisType::HarmonicBalance
            | AnalysisType::Fourier => ("Frequency", "Hz", "Magnitude", "V"),
            AnalysisType::Pnoise => ("Frequency", "Hz", "Noise", "V^2/Hz"),
            AnalysisType::DcSweep => ("Voltage", "V", "Voltage", "V"),
            AnalysisType::DcOp => ("", "", "Voltage", "V"),
            AnalysisType::PoleZero => ("Real", "", "Imaginary", ""),
            AnalysisType::Sensitivity => ("Parameter", "", "Sensitivity", ""),
            AnalysisType::MonteCarlo => ("Value", "", "Count", "count"),
            AnalysisType::Parametric => ("Sweep", "", "Voltage", "V"),
            AnalysisType::Corner => ("Temperature", "C", "Voltage", "V"),
            AnalysisType::Reliability => ("Lifetime", "year", "Shift", ""),
            AnalysisType::Optimization => ("Iteration", "iter", "Cost", "cost"),
        }
    }
}

impl std::fmt::Display for AnalysisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Operating point data for a single node or device terminal
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointValue {
    /// Node or terminal name (e.g., "V(out)", "I(R1)")
    pub name: String,
    /// Value in base units (volts, amps, etc.)
    pub value: f64,
    /// Unit string for display (e.g., "V", "A", "W")
    pub unit: String,
}

/// DC operating point results - node voltages and branch currents
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Node voltages
    pub node_voltages: Vec<OperatingPointValue>,
    /// Branch currents
    pub branch_currents: Vec<OperatingPointValue>,
    /// Power dissipation by device
    pub power_dissipation: Vec<OperatingPointValue>,
}

/// Single analysis result with metadata and waveforms.
///
/// This represents one analysis within a simulation run, containing
/// all the data needed to display results in the appropriate viewer.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Unique ID within the simulation run
    pub id: u64,
    /// Analysis type for viewer selection
    pub analysis_type: AnalysisType,
    /// Human-readable label with parameters (e.g., "AC (1Hz-1GHz)")
    pub label: String,
    /// Unix timestamp when analysis completed
    pub timestamp: f64,
    /// Time-domain or frequency-domain waveforms (for sweep analyses)
    pub waveforms: Vec<WaveformData>,
    /// DC operating point data (for DC Op analysis)
    pub dc_op: Option<DcOpResult>,
    /// Whether this analysis completed successfully
    pub success: bool,
    /// Error message if analysis failed
    pub error_message: Option<String>,
}

impl AnalysisResult {
    /// Create a new successful analysis result
    pub fn new(id: u64, analysis_type: AnalysisType, label: impl Into<String>) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            success: true,
            error_message: None,
        }
    }

    /// Create a failed analysis result
    pub fn failed(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            success: false,
            error_message: Some(error.into()),
        }
    }

    /// Add waveform data to this analysis
    pub fn with_waveforms(mut self, waveforms: Vec<WaveformData>) -> Self {
        self.waveforms = waveforms;
        self
    }

    /// Add DC operating point data
    pub fn with_dc_op(mut self, dc_op: DcOpResult) -> Self {
        self.dc_op = Some(dc_op);
        self
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Check if this analysis has any viewable data
    pub fn has_data(&self) -> bool {
        !self.waveforms.is_empty() || self.dc_op.is_some()
    }
}

/// A simulation run containing multiple analysis results.
///
/// This represents a single invocation of the simulator, which may contain
/// multiple analyses (e.g., DC Op + Transient + AC in one run).
///
/// Follows Cadence Spectre PSF database conventions:
/// - Timestamped runs for identification
/// - Multiple analyses per run
/// - Retained in history for comparison
#[derive(Debug, Clone)]
pub struct SimulationRun {
    /// Unique run identifier (monotonically increasing)
    pub id: u64,
    /// Human-readable label with timestamp (e.g., "Run 3 (1:04:01 PM)")
    pub label: String,
    /// Unix timestamp when run started
    pub timestamp: f64,
    /// All analysis results from this run
    pub analyses: Vec<AnalysisResult>,
    /// Total simulation time in seconds
    pub elapsed_time: f64,
    /// Whether all analyses in this run succeeded
    pub success: bool,
}

impl SimulationRun {
    /// Create a new simulation run with auto-generated ID and timestamp
    pub fn new(run_number: u64) -> Self {
        let timestamp = Self::current_timestamp();
        let time_str = Self::format_time(timestamp);
        Self {
            id: run_number,
            label: format!("Run {} ({})", run_number, time_str),
            timestamp,
            analyses: Vec::new(),
            elapsed_time: 0.0,
            success: true,
        }
    }

    /// Add an analysis result to this run
    pub fn add_analysis(&mut self, analysis: AnalysisResult) {
        if !analysis.success {
            self.success = false;
        }
        self.analyses.push(analysis);
    }

    /// Set the total elapsed time for this run
    pub fn set_elapsed_time(&mut self, elapsed: f64) {
        self.elapsed_time = elapsed;
    }

    /// Get count of successful analyses
    pub fn successful_analyses(&self) -> usize {
        self.analyses.iter().filter(|a| a.success).count()
    }

    /// Find analysis by type (returns first match)
    pub fn find_analysis(&self, analysis_type: AnalysisType) -> Option<&AnalysisResult> {
        self.analyses
            .iter()
            .find(|a| a.analysis_type == analysis_type)
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Format timestamp as human-readable time string (e.g., "1:04:01 PM")
    fn format_time(timestamp: f64) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let _datetime = UNIX_EPOCH + Duration::from_secs_f64(timestamp);
        // Use chrono-free approach for portability
        let secs = timestamp as u64;
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        let (hour_12, am_pm) = if hours == 0 {
            (12, "AM")
        } else if hours < 12 {
            (hours, "AM")
        } else if hours == 12 {
            (12, "PM")
        } else {
            (hours - 12, "PM")
        };
        format!("{}:{:02}:{:02} {}", hour_12, minutes, seconds, am_pm)
    }
}

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

// SimulationState implementation continues at line 210

/// Waveform trace data
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
    /// Trace name (e.g., "V(out)")
    pub name: String,

    /// X-axis values (time or frequency)
    pub x: Vec<Value>,

    /// Y-axis values
    pub y: Vec<Value>,

    /// Trace color (hex string)
    pub color: String,

    /// Whether this trace is visible
    pub visible: bool,
}

impl WaveformData {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: Vec<Value>,
        y: Vec<Value>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x,
            y,
            color: color.into(),
            visible: true,
        }
    }

    /// Get the X range (min, max)
    pub fn x_range(&self) -> (Value, Value) {
        let min = self.x.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.x.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }

    /// Get the Y range (min, max)
    pub fn y_range(&self) -> (Value, Value) {
        let min = self.y.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.y.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }
}

/// Console message severity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Error,
    Success,
}

/// Console log message
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleMessage {
    /// Message severity
    pub severity: MessageSeverity,

    /// Message content
    pub message: String,

    /// Timestamp (seconds since simulation start)
    pub timestamp: Option<f64>,
}

impl ConsoleMessage {
    /// Get current timestamp as epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Info,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Warning,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Error,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create a success message
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Success,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }
}

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

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AnalysisType Tests
    // =========================================================================

    #[test]
    fn test_analysis_type_display_names() {
        assert_eq!(AnalysisType::DcOp.display_name(), "DC Operating Point");
        assert_eq!(AnalysisType::Ac.display_name(), "AC Analysis");
        assert_eq!(AnalysisType::Transient.display_name(), "Transient");
        assert_eq!(AnalysisType::Noise.display_name(), "Noise");
        assert_eq!(AnalysisType::PoleZero.display_name(), "Pole-Zero");
        assert_eq!(AnalysisType::Tf.display_name(), "Transfer Function");
        assert_eq!(AnalysisType::Pac.display_name(), "PAC");
        assert_eq!(AnalysisType::Pnoise.display_name(), "PNoise");
        assert_eq!(AnalysisType::Pxf.display_name(), "PXF");
        assert_eq!(AnalysisType::Pstb.display_name(), "PSTB");
        assert_eq!(AnalysisType::Stb.display_name(), "STB");
        assert_eq!(AnalysisType::MonteCarlo.display_name(), "Monte Carlo");
        assert_eq!(AnalysisType::Parametric.display_name(), "Parametric Sweep");
        assert_eq!(AnalysisType::Corner.display_name(), "Corner Sweep");
        assert_eq!(AnalysisType::Reliability.display_name(), "Reliability");
        assert_eq!(AnalysisType::Optimization.display_name(), "Optimization");
        assert_eq!(AnalysisType::Soa.display_name(), "Safety (SOA)");
        assert_eq!(AnalysisType::SParameter.display_name(), "S-Parameter");
        assert_eq!(AnalysisType::Envelope.display_name(), "Envelope");
        assert_eq!(AnalysisType::Fourier.display_name(), "Fourier");
    }

    #[test]
    fn test_analysis_type_short_labels() {
        assert_eq!(AnalysisType::DcOp.short_label(), "DC");
        assert_eq!(AnalysisType::Ac.short_label(), "AC");
        assert_eq!(AnalysisType::Transient.short_label(), "TR");
        assert_eq!(AnalysisType::Tf.short_label(), "TF");
        assert_eq!(AnalysisType::Pac.short_label(), "PAC");
        assert_eq!(AnalysisType::Pnoise.short_label(), "PN");
        assert_eq!(AnalysisType::Pxf.short_label(), "PXF");
        assert_eq!(AnalysisType::Pstb.short_label(), "PSTB");
        assert_eq!(AnalysisType::Stb.short_label(), "STB");
        assert_eq!(AnalysisType::MonteCarlo.short_label(), "MC");
        assert_eq!(AnalysisType::Parametric.short_label(), "PAR");
        assert_eq!(AnalysisType::Corner.short_label(), "CRN");
        assert_eq!(AnalysisType::Reliability.short_label(), "REL");
        assert_eq!(AnalysisType::Optimization.short_label(), "OPT");
        assert_eq!(AnalysisType::Soa.short_label(), "SOA");
        assert_eq!(AnalysisType::SParameter.short_label(), "SP");
        assert_eq!(AnalysisType::Envelope.short_label(), "ENV");
        assert_eq!(AnalysisType::Fourier.short_label(), "FOU");
        assert_eq!(AnalysisType::HarmonicBalance.short_label(), "HB");
        assert_eq!(AnalysisType::Pss.short_label(), "PSS");
    }

    #[test]
    fn test_analysis_type_display_trait() {
        assert_eq!(format!("{}", AnalysisType::DcOp), "DC Operating Point");
        assert_eq!(format!("{}", AnalysisType::Transient), "Transient");
    }

    #[test]
    fn test_analysis_type_equality() {
        assert_eq!(AnalysisType::Ac, AnalysisType::Ac);
        assert_ne!(AnalysisType::Ac, AnalysisType::Transient);
    }

    #[test]
    fn test_analysis_type_axis_info_for_hb_and_pss() {
        assert_eq!(
            AnalysisType::HarmonicBalance.axis_info(),
            ("Frequency", "Hz", "Magnitude", "V")
        );
        assert_eq!(AnalysisType::Pss.axis_info(), ("Time", "s", "Voltage", "V"));
        assert_eq!(
            AnalysisType::Pnoise.axis_info(),
            ("Frequency", "Hz", "Noise", "V^2/Hz")
        );
        assert_eq!(
            AnalysisType::Optimization.axis_info(),
            ("Iteration", "iter", "Cost", "cost")
        );
        assert_eq!(
            AnalysisType::Reliability.axis_info(),
            ("Lifetime", "year", "Shift", "")
        );
    }

    // =========================================================================
    // AnalysisResult Tests
    // =========================================================================

    #[test]
    fn test_analysis_result_creation() {
        let result = AnalysisResult::new(1, AnalysisType::Transient, "Transient (0-1µs)");

        assert_eq!(result.id, 1);
        assert_eq!(result.analysis_type, AnalysisType::Transient);
        assert_eq!(result.label, "Transient (0-1µs)");
        assert!(result.success);
        assert!(result.error_message.is_none());
        assert!(result.waveforms.is_empty());
        assert!(result.dc_op.is_none());
    }

    #[test]
    fn test_analysis_result_failed() {
        let result = AnalysisResult::failed(
            2,
            AnalysisType::Ac,
            "AC Analysis",
            "Singular matrix at DC operating point",
        );

        assert_eq!(result.id, 2);
        assert!(!result.success);
        assert_eq!(
            result.error_message.as_deref(),
            Some("Singular matrix at DC operating point")
        );
    }

    #[test]
    fn test_analysis_result_with_waveforms() {
        let wf = WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.5, 1.5], "#ff0000");
        let result =
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![wf]);

        assert!(result.has_data());
        assert_eq!(result.waveforms.len(), 1);
        assert_eq!(result.waveforms[0].name, "V(out)");
    }

    #[test]
    fn test_analysis_result_with_dc_op() {
        let dc_op = DcOpResult {
            node_voltages: vec![OperatingPointValue {
                name: "V(out)".to_string(),
                value: 2.5,
                unit: "V".to_string(),
            }],
            branch_currents: vec![],
            power_dissipation: vec![],
        };

        let result = AnalysisResult::new(1, AnalysisType::DcOp, "DC Op").with_dc_op(dc_op);

        assert!(result.has_data());
        assert!(result.dc_op.is_some());
        assert_eq!(result.dc_op.as_ref().unwrap().node_voltages.len(), 1);
    }

    // =========================================================================
    // SimulationRun Tests
    // =========================================================================

    #[test]
    fn test_simulation_run_creation() {
        let run = SimulationRun::new(1);

        assert_eq!(run.id, 1);
        assert!(run.label.starts_with("Run 1"));
        assert!(run.analyses.is_empty());
        assert!(run.success);
        assert_eq!(run.elapsed_time, 0.0);
    }

    #[test]
    fn test_simulation_run_add_analysis() {
        let mut run = SimulationRun::new(1);

        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "Transient"));

        assert_eq!(run.analyses.len(), 2);
        assert_eq!(run.successful_analyses(), 2);
        assert!(run.success);
    }

    #[test]
    fn test_simulation_run_failed_analysis_marks_run_failed() {
        let mut run = SimulationRun::new(1);

        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
        run.add_analysis(AnalysisResult::failed(
            2,
            AnalysisType::Ac,
            "AC",
            "Matrix singular",
        ));

        assert!(!run.success);
        assert_eq!(run.successful_analyses(), 1);
    }

    #[test]
    fn test_simulation_run_find_analysis() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "Transient"));

        let dc = run.find_analysis(AnalysisType::DcOp);
        assert!(dc.is_some());
        assert_eq!(dc.unwrap().id, 1);

        let ac = run.find_analysis(AnalysisType::Ac);
        assert!(ac.is_none());
    }

    // =========================================================================
    // SimulationState Run Management Tests
    // =========================================================================

    #[test]
    fn test_simulation_state_start_run() {
        let mut state = SimulationState::default();

        assert!(!state.has_results());
        assert_eq!(state.run_count(), 0);

        let run = state.start_run();
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "Transient"));

        assert!(state.has_results());
        assert_eq!(state.run_count(), 1);
        assert_eq!(state.active_run_idx, Some(0));
    }

    #[test]
    fn test_simulation_state_multiple_runs() {
        let mut state = SimulationState::default();

        // Start first run
        state.start_run();
        assert_eq!(state.next_run_id, 1);

        // Start second run
        state.start_run();
        assert_eq!(state.next_run_id, 2);
        assert_eq!(state.run_count(), 2);

        // Newest run is at index 0
        assert_eq!(state.runs[0].id, 2);
        assert_eq!(state.runs[1].id, 1);
    }

    #[test]
    fn test_simulation_state_select_run() {
        let mut state = SimulationState::default();

        state.start_run();
        state.start_run();

        // Active is newest (index 0)
        assert_eq!(state.active_run_idx, Some(0));

        // Select older run
        assert!(state.select_run(1));
        assert_eq!(state.active_run_idx, Some(1));

        // Invalid index
        assert!(!state.select_run(10));
    }

    #[test]
    fn test_simulation_state_select_analysis() {
        let mut state = SimulationState::default();

        let run = state.start_run();
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC"));
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "TR"));

        assert!(state.select_analysis(0));
        assert_eq!(state.active_analysis_idx, Some(0));

        assert!(state.select_analysis(1));
        assert_eq!(state.active_analysis_idx, Some(1));

        assert!(!state.select_analysis(10));
    }

    #[test]
    fn test_simulation_state_complete_run_syncs_waveforms() {
        let mut state = SimulationState::default();

        let run = state.start_run();
        let wf1 = WaveformData::new("V(1)", vec![0.0, 1.0], vec![0.0, 1.0], "#ff0000");
        let wf2 = WaveformData::new("V(2)", vec![0.0, 1.0], vec![0.5, 1.5], "#00ff00");

        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_waveforms(vec![wf1, wf2]),
        );

        state.complete_run();

        // Legacy waveforms should be synced
        assert_eq!(state.waveforms.len(), 2);
        assert_eq!(state.waveforms[0].name, "V(1)");
        assert_eq!(state.waveforms[1].name, "V(2)");
    }

    #[test]
    fn test_simulation_state_delete_run() {
        let mut state = SimulationState::default();

        state.start_run();
        state.start_run();
        state.start_run();

        assert_eq!(state.run_count(), 3);

        // Delete middle run
        assert!(state.delete_run(1));
        assert_eq!(state.run_count(), 2);

        // Active was 0, should still be 0
        assert_eq!(state.active_run_idx, Some(0));

        // Delete invalid
        assert!(!state.delete_run(10));
    }

    #[test]
    fn test_simulation_state_delete_active_run() {
        let mut state = SimulationState::default();

        state.start_run();
        state.start_run();

        // Active is run at index 0
        assert_eq!(state.active_run_idx, Some(0));

        // Delete active run
        state.delete_run(0);

        // Should select newest remaining (index 0)
        assert_eq!(state.active_run_idx, Some(0));
        assert_eq!(state.run_count(), 1);
    }

    #[test]
    fn test_simulation_state_prune_history() {
        let mut state = SimulationState::default();

        // Create MAX_RUN_HISTORY + 5 runs
        for _ in 0..(MAX_RUN_HISTORY + 5) {
            state.start_run();
        }

        // Should be pruned to MAX_RUN_HISTORY
        assert_eq!(state.run_count(), MAX_RUN_HISTORY);

        // Newest run should still be at index 0
        assert_eq!(state.runs[0].id, (MAX_RUN_HISTORY + 5) as u64);
    }

    #[test]
    fn test_simulation_state_clear_runs() {
        let mut state = SimulationState::default();

        state.start_run();
        state.start_run();
        let first_id = state.next_run_id;

        state.clear_runs();

        assert!(!state.has_results());
        assert_eq!(state.active_run_idx, None);
        assert_eq!(state.active_analysis_idx, None);

        // next_run_id should be preserved
        assert_eq!(state.next_run_id, first_id);
    }

    #[test]
    fn test_simulation_state_active_getters() {
        let mut state = SimulationState::default();

        // No active run initially
        assert!(state.active_run().is_none());
        assert!(state.active_analysis().is_none());

        let run = state.start_run();
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC"));

        // Now we have an active run
        assert!(state.active_run().is_some());
        assert_eq!(state.active_run().unwrap().id, 1);

        // No active analysis yet
        assert!(state.active_analysis().is_none());

        state.select_analysis(0);
        assert!(state.active_analysis().is_some());
        assert_eq!(
            state.active_analysis().unwrap().analysis_type,
            AnalysisType::DcOp
        );
    }
}
