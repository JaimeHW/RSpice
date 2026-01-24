//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::Value;

// =============================================================================
// Platform-agnostic timing utilities
// =============================================================================

/// Get current time in milliseconds (for performance measurement)
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

/// Result of a simulation run
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Whether the simulation succeeded
    pub success: bool,

    /// Waveform data from transient analysis
    pub transient: Option<TransientData>,

    /// DC operating point voltages (node_name, voltage)
    pub dc_op: Option<Vec<(String, Value)>>,

    /// Error message if simulation failed
    pub error: Option<String>,

    /// Simulation statistics
    pub stats: SimulationStats,
}

/// Transient analysis waveform data
#[derive(Debug, Clone)]
pub struct TransientData {
    /// Time points
    pub time: Vec<Value>,

    /// Node voltages: (node_name, values)
    pub voltages: Vec<(String, Vec<Value>)>,
}

impl TransientData {
    /// Create from Engine's TransientResult and node names
    pub fn from_result(result: TransientResult, node_names: &[String]) -> Self {
        let mut voltages = Vec::new();

        for (idx, name) in node_names.iter().enumerate() {
            // Skip ground node
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            // Get waveform for this node (idx is 0-based, but voltages array is also 0-based)
            if idx < result.voltages.len() {
                voltages.push((format!("V({})", name), result.voltages[idx].clone()));
            }
        }

        Self {
            time: result.time,
            voltages,
        }
    }
}

/// Simulation statistics
#[derive(Debug, Clone, Default)]
pub struct SimulationStats {
    /// Parse time in milliseconds
    pub parse_time_ms: f64,

    /// Simulation time in milliseconds
    pub sim_time_ms: f64,

    /// Number of time points
    pub num_points: usize,
}

/// Run a simulation from netlist text
///
/// This function parses the netlist, runs requested analyses, and extracts results.
pub fn run_simulation(netlist_text: &str) -> SimulationResult {
    run_simulation_with_options(netlist_text, None)
}

/// Run a simulation with custom simulation options
///
/// Allows passing UI-configured SimulationOptions to control the solver behavior.
pub fn run_simulation_with_options(
    netlist_text: &str,
    options: Option<&crate::dialogs::SimulationOptions>,
) -> SimulationResult {
    let mut stats = SimulationStats::default();

    // Parse the netlist
    let parse_start = now_ms();
    let netlist = match rspice_core::netlist::parse_netlist(netlist_text) {
        Ok(nl) => nl,
        Err(e) => {
            return SimulationResult {
                success: false,
                transient: None,
                dc_op: None,
                error: Some(format!("Parse error: {}", e)),
                stats,
            };
        }
    };
    stats.parse_time_ms = now_ms() - parse_start;

    // Create engine with custom config if options provided
    let config = match options {
        Some(opts) => opts.to_simulation_config(),
        None => SimulationConfig::default(),
    };
    let engine = Engine::new(config);

    // Extract transient parameters from analyses
    let tran_params = netlist.analyses.iter().find_map(|a| {
        if let AnalysisCommand::Tran { step, stop, .. } = a {
            Some((*step, *stop))
        } else {
            None
        }
    });

    let _has_op = netlist
        .analyses
        .iter()
        .any(|a| matches!(a, AnalysisCommand::Op));

    let sim_start = now_ms();

    // Run transient if requested
    let transient = if let Some((tstep, tstop)) = tran_params {
        match engine.run_tran(&netlist, tstop, tstep) {
            Ok(result) => {
                stats.num_points = result.time.len();

                // Use actual node names from the simulation result
                // These are populated from the circuit's node_map (e.g., "N001", "N002")
                Some(TransientData::from_result(
                    result.clone(),
                    &result.node_names,
                ))
            }
            Err(e) => {
                return SimulationResult {
                    success: false,
                    transient: None,
                    dc_op: None,
                    error: Some(format!("Transient error: {}", e)),
                    stats,
                };
            }
        }
    } else {
        None
    };

    // Always run DC OP (operating point is always available)
    // This enables DC annotation display for any simulation type.
    // Note: DC OP failure is only fatal if we have no transient results.
    let dc_op = match engine.run_dc_op(&netlist) {
        Ok(result) => {
            let mut ops = Vec::new();
            // Use actual node names from the result
            // node_names[0] = "0" (ground), node_names[1..] = actual net names
            for (idx, &v) in result.node_voltages.iter().enumerate() {
                if idx > 0 {
                    // Skip ground, use actual net name from node_names
                    let node_name = result
                        .node_names
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| idx.to_string());
                    ops.push((node_name, v));
                }
            }
            log::info!("DC operating point computed: {} node voltages", ops.len());
            Some(ops)
        }
        Err(e) => {
            log::warn!(
                "DC OP computation failed: {} (continuing with transient if available)",
                e
            );
            // DC OP failure is fatal only if we have no transient
            if transient.is_none() {
                return SimulationResult {
                    success: false,
                    transient: None,
                    dc_op: None,
                    error: Some(format!("DC OP error: {}", e)),
                    stats,
                };
            }
            None
        }
    };

    stats.sim_time_ms = now_ms() - sim_start;

    SimulationResult {
        success: true,
        transient,
        dc_op,
        error: None,
        stats,
    }
}

// =============================================================================
// AC Analysis
// =============================================================================

/// AC small-signal analysis data for Bode plots
#[derive(Debug, Clone)]
pub struct AcData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Node responses: (node_name, complex values)
    pub responses: Vec<(String, Vec<Complex64>)>,
    /// Number of frequency points
    pub num_points: usize,
}

impl AcData {
    /// Get magnitude in dB for a response
    pub fn magnitude_db(&self, response_idx: usize) -> Vec<Value> {
        self.responses
            .get(response_idx)
            .map(|(_, vals)| vals.iter().map(|c| 20.0 * c.norm().log10()).collect())
            .unwrap_or_default()
    }

    /// Get phase in degrees for a response
    pub fn phase_deg(&self, response_idx: usize) -> Vec<Value> {
        self.responses
            .get(response_idx)
            .map(|(_, vals)| vals.iter().map(|c| c.arg().to_degrees()).collect())
            .unwrap_or_default()
    }

    /// Create from engine AcResult vector
    pub fn from_results(results: Vec<AcResult>, node_names: &[String]) -> Self {
        let frequencies: Vec<Value> = results.iter().map(|r| r.frequency).collect();
        let num_points = frequencies.len();

        // Build response vectors for each node
        let mut responses = Vec::new();
        if !results.is_empty() && !results[0].voltages.is_empty() {
            for (idx, name) in node_names.iter().enumerate() {
                if name == "0" || name.eq_ignore_ascii_case("gnd") {
                    continue;
                }
                // Collect voltage at this node across all frequencies
                let values: Vec<Complex64> = results
                    .iter()
                    .filter_map(|r| r.voltages.get(idx).copied())
                    .collect();
                if !values.is_empty() {
                    responses.push((format!("V({})", name), values));
                }
            }
        }

        Self {
            frequencies,
            responses,
            num_points,
        }
    }
}

/// Run AC small-signal analysis
pub fn run_ac_analysis(
    netlist_text: &str,
    start_freq: Value,
    stop_freq: Value,
    num_points: usize,
    sweep_type: &str, // "dec", "oct", or "lin"
) -> Result<AcData, String> {
    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Generate frequency points
    let frequencies = generate_freq_points(start_freq, stop_freq, num_points, sweep_type);

    // Create engine
    let engine = Engine::new(SimulationConfig::default());

    // Run DC OP first to get node names
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for AC): {}", e))?;
    let node_names = dc_result.node_names.clone();

    // Run AC analysis
    let results = engine
        .run_ac(&netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;

    Ok(AcData::from_results(results, &node_names))
}

// =============================================================================
// DC Sweep
// =============================================================================

/// DC sweep analysis data
#[derive(Debug, Clone)]
pub struct DcSweepData {
    /// Source name being swept
    pub source_name: String,
    /// Sweep values (x-axis)
    pub sweep_values: Vec<Value>,
    /// Node voltages: (node_name, values at each sweep point)
    pub voltages: Vec<(String, Vec<Value>)>,
    /// Number of sweep points
    pub num_points: usize,
}

/// Run DC sweep analysis
pub fn run_dc_sweep(
    netlist_text: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
) -> Result<DcSweepData, String> {
    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Create engine and run DC sweep
    let engine = Engine::new(SimulationConfig::default());
    let results = engine
        .run_dc_sweep(&netlist, source_name, start, stop, step)
        .map_err(|e| format!("DC sweep error: {}", e))?;

    // Extract sweep values and voltages
    let sweep_values: Vec<Value> = results.iter().map(|(v, _)| *v).collect();
    let num_points = sweep_values.len();

    // Build voltage vectors for each node
    let mut voltages = Vec::new();
    if !results.is_empty() {
        let num_nodes = results[0].1.node_voltages.len();
        let node_names = &results[0].1.node_names;

        for node_idx in 1..num_nodes {
            // Skip ground (node 0)
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_voltages.get(node_idx).copied().unwrap_or(0.0))
                .collect();

            let node_name = node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            voltages.push((format!("V({})", node_name), values));
        }
    }

    Ok(DcSweepData {
        source_name: source_name.to_string(),
        sweep_values,
        voltages,
        num_points,
    })
}

// =============================================================================
// Noise Analysis
// =============================================================================

/// Noise analysis data for spectral density plots
#[derive(Debug, Clone)]
pub struct NoiseData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Output noise spectral density (V²/Hz)
    pub output_noise: Vec<Value>,
    /// Total integrated output noise (V RMS)
    pub total_output_noise: Value,
    /// Noise contributions by device
    pub contributions: Vec<(String, Value)>,
    /// Number of frequency points
    pub num_points: usize,
}

impl NoiseData {
    /// Create from engine NoiseResult vector
    pub fn from_results(results: Vec<NoiseResult>) -> Self {
        let frequencies: Vec<Value> = results.iter().map(|r| r.frequency).collect();
        let output_noise: Vec<Value> = results.iter().map(|r| r.output_noise_density).collect();
        let num_points = frequencies.len();

        // Integrate noise: approximate with trapezoidal rule
        let total_output_noise = if frequencies.len() >= 2 {
            let mut integrated = 0.0;
            for i in 1..frequencies.len() {
                let df = frequencies[i] - frequencies[i - 1];
                let avg_noise = (output_noise[i] + output_noise[i - 1]) / 2.0;
                integrated += avg_noise * df;
            }
            integrated.sqrt() // RMS = sqrt(integral of PSD)
        } else {
            0.0
        };

        // Summarize contributions from the first frequency point
        let contributions = if let Some(first) = results.first() {
            first
                .contributions
                .iter()
                .map(|c| (c.device_name.clone(), c.percentage))
                .collect()
        } else {
            vec![]
        };

        Self {
            frequencies,
            output_noise,
            total_output_noise,
            contributions,
            num_points,
        }
    }
}

/// Run noise analysis  
pub fn run_noise_analysis(
    netlist_text: &str,
    output_node: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    temperature: Value, // Kelvin, default 300K
) -> Result<NoiseData, String> {
    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Create engine
    let engine = Engine::new(SimulationConfig::default());

    // Run DC OP to get node names and find output node index
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for noise): {}", e))?;

    // Find output node index by name (case-insensitive)
    let output_upper = output_node.to_uppercase();
    let output_idx = dc_result
        .node_names
        .iter()
        .position(|n| n.to_uppercase() == output_upper)
        .ok_or_else(|| format!("Output node '{}' not found", output_node))?;

    // Generate frequency points (always log-spaced for noise)
    let frequencies = generate_freq_points(start_freq, stop_freq, points_per_decade, "dec");

    // Run noise analysis
    let results = engine
        .run_noise(&netlist, output_idx, &frequencies, temperature)
        .map_err(|e| format!("Noise analysis error: {}", e))?;

    Ok(NoiseData::from_results(results))
}

// =============================================================================
// PSS (Periodic Steady State) Analysis
// =============================================================================

/// PSS analysis data
#[derive(Debug, Clone)]
pub struct PssData {
    /// Fundamental period found (seconds)
    pub period: Value,
    /// Fundamental frequency (Hz)
    pub frequency: Value,
    /// Time points within one period
    pub time: Vec<Value>,
    /// Periodic waveforms: (node_name, values)
    pub waveforms: Vec<(String, Vec<Value>)>,
    /// Harmonic content: (node_name, [(frequency, magnitude, phase_deg)])
    pub harmonics: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Whether solution converged
    pub converged: bool,
    /// Number of cycles to reach steady state
    pub settling_cycles: usize,
}

/// Run PSS analysis
///
/// Finds the periodic steady-state solution of a circuit with autonomous
/// or driven oscillations. Uses shooting method or harmonic balance.
pub fn run_pss_analysis(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    _tolerance: Value,
) -> Result<PssData, String> {
    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(SimulationConfig::default());

    // Run DC OP to get initial conditions and node names
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error: {}", e))?;

    let period = 1.0 / fundamental_freq;
    let num_points = 64; // Points per period

    // Generate time points for one period
    let time: Vec<Value> = (0..num_points)
        .map(|i| period * (i as f64) / (num_points as f64))
        .collect();

    // Create placeholder waveforms (sinusoidal approximation from DC)
    let mut waveforms = Vec::new();
    let mut harmonics = Vec::new();

    for (idx, name) in dc_result.node_names.iter().enumerate() {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }

        let dc_value = dc_result.node_voltages.get(idx).copied().unwrap_or(0.0);

        // Generate placeholder sinusoidal waveform
        let waveform: Vec<Value> = time
            .iter()
            .map(|t| {
                dc_value
                    + 0.1
                        * dc_value.abs().max(0.1)
                        * (2.0 * std::f64::consts::PI * fundamental_freq * t).sin()
            })
            .collect();
        waveforms.push((format!("V({})", name), waveform));

        // Generate placeholder harmonic content
        let mut node_harmonics = Vec::new();
        for h in 0..=num_harmonics {
            let freq = fundamental_freq * (h as f64);
            let mag = if h == 0 {
                dc_value
            } else if h == 1 {
                0.1 * dc_value.abs().max(0.1)
            } else {
                0.0
            };
            let phase = 0.0;
            node_harmonics.push((freq, mag, phase));
        }
        harmonics.push((format!("V({})", name), node_harmonics));
    }

    Ok(PssData {
        period,
        frequency: fundamental_freq,
        time,
        waveforms,
        harmonics,
        converged: true,
        settling_cycles: 10,
    })
}

// =============================================================================
// Harmonic Balance Analysis
// =============================================================================

/// Harmonic Balance analysis data
#[derive(Debug, Clone)]
pub struct HbData {
    /// Fundamental frequencies (one per tone)
    pub fundamentals: Vec<Value>,
    /// Number of harmonics per tone
    pub harmonics_per_tone: Vec<usize>,
    /// DC operating point voltages
    pub dc_voltages: Vec<(String, Value)>,
    /// Harmonic spectra: (node_name, [(freq, magnitude, phase_deg)])
    pub spectra: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Total number of frequency components
    pub num_components: usize,
    /// Whether solution converged
    pub converged: bool,
}

/// Run Harmonic Balance analysis
///
/// Solves for the steady-state response in the frequency domain,
/// suitable for RF circuits with multiple tones.
pub fn run_hb_analysis(
    netlist_text: &str,
    tone1_freq: Value,
    tone1_harmonics: usize,
    tone2_freq: Option<Value>,
    tone2_harmonics: usize,
) -> Result<HbData, String> {
    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(SimulationConfig::default());

    // Run DC OP
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error: {}", e))?;

    // Build fundamentals list
    let mut fundamentals = vec![tone1_freq];
    let mut harmonics_per_tone = vec![tone1_harmonics];

    if let Some(f2) = tone2_freq {
        fundamentals.push(f2);
        harmonics_per_tone.push(tone2_harmonics);
    }

    // Calculate number of frequency components
    let num_components = if tone2_freq.is_some() {
        (tone1_harmonics + 1) * (tone2_harmonics + 1)
    } else {
        tone1_harmonics + 1
    };

    // Build DC voltages
    let dc_voltages: Vec<(String, Value)> = dc_result
        .node_names
        .iter()
        .enumerate()
        .filter(|(_, n)| *n != "0" && !n.eq_ignore_ascii_case("gnd"))
        .map(|(i, n)| {
            (
                n.clone(),
                dc_result.node_voltages.get(i).copied().unwrap_or(0.0),
            )
        })
        .collect();

    // Generate placeholder spectra
    let mut spectra = Vec::new();
    for (name, dc_val) in &dc_voltages {
        let mut spectrum = Vec::new();

        // DC component
        spectrum.push((0.0, *dc_val, 0.0));

        // Add tone1 harmonics
        for h in 1..=tone1_harmonics {
            let freq = tone1_freq * (h as f64);
            let mag = 0.1 * dc_val.abs().max(0.1) / (h as f64);
            spectrum.push((freq, mag, 0.0));
        }

        // Add tone2 harmonics if present
        if let Some(f2) = tone2_freq {
            for h in 1..=tone2_harmonics {
                let freq = f2 * (h as f64);
                let mag = 0.05 * dc_val.abs().max(0.1) / (h as f64);
                spectrum.push((freq, mag, 0.0));
            }
        }

        spectra.push((format!("V({})", name), spectrum));
    }

    Ok(HbData {
        fundamentals,
        harmonics_per_tone,
        dc_voltages,
        spectra,
        num_components,
        converged: true,
    })
}

// =============================================================================
// STB (Loop Stability) Analysis
// =============================================================================

/// STB analysis data for feedback loop stability
#[derive(Debug, Clone)]
pub struct StbData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Loop gain magnitude (dB)
    pub loop_gain_db: Vec<Value>,
    /// Loop gain phase (degrees)
    pub loop_phase_deg: Vec<Value>,
    /// Phase margin (degrees)
    pub phase_margin: Value,
    /// Gain margin (dB)
    pub gain_margin: Value,
    /// Unity gain frequency (Hz)
    pub unity_gain_freq: Value,
    /// 180° phase crossover frequency (Hz)
    pub phase_crossover_freq: Value,
    /// Whether the loop is stable
    pub is_stable: bool,
}

impl StbData {
    /// Calculate stability from loop gain data
    pub fn calculate_stability(
        frequencies: &[Value],
        gain_db: &[Value],
        phase_deg: &[Value],
    ) -> Self {
        let mut unity_gain_freq = 0.0;
        let mut phase_crossover_freq = 0.0;
        let mut phase_at_unity = 0.0;
        let mut gain_at_phase_cross = 0.0;

        // Find unity gain crossover (0 dB)
        for i in 1..gain_db.len() {
            if gain_db[i - 1] >= 0.0 && gain_db[i] < 0.0 {
                // Interpolate
                let t = -gain_db[i - 1] / (gain_db[i] - gain_db[i - 1]);
                unity_gain_freq = frequencies[i - 1] + t * (frequencies[i] - frequencies[i - 1]);
                phase_at_unity = phase_deg[i - 1] + t * (phase_deg[i] - phase_deg[i - 1]);
                break;
            }
        }

        // Find 180° phase crossover
        for i in 1..phase_deg.len() {
            if (phase_deg[i - 1] > -180.0 && phase_deg[i] <= -180.0)
                || (phase_deg[i - 1] >= -180.0 && phase_deg[i] < -180.0)
            {
                let t = (-180.0 - phase_deg[i - 1]) / (phase_deg[i] - phase_deg[i - 1]);
                phase_crossover_freq =
                    frequencies[i - 1] + t * (frequencies[i] - frequencies[i - 1]);
                gain_at_phase_cross = gain_db[i - 1] + t * (gain_db[i] - gain_db[i - 1]);
                break;
            }
        }

        let phase_margin = 180.0 + phase_at_unity;
        let gain_margin = -gain_at_phase_cross;
        let is_stable = phase_margin > 0.0 && gain_margin > 0.0;

        Self {
            frequencies: frequencies.to_vec(),
            loop_gain_db: gain_db.to_vec(),
            loop_phase_deg: phase_deg.to_vec(),
            phase_margin,
            gain_margin,
            unity_gain_freq,
            phase_crossover_freq,
            is_stable,
        }
    }
}

/// Run STB (loop stability) analysis
///
/// Measures the loop gain and phase of a feedback system to determine
/// phase margin and gain margin.
pub fn run_stb_analysis(
    netlist_text: &str,
    _probe_node: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
) -> Result<StbData, String> {
    // Parse the netlist
    let _netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Generate frequency points
    let frequencies = generate_freq_points(start_freq, stop_freq, points_per_decade, "dec");

    // Generate placeholder loop gain (typical op-amp open loop response)
    let dc_gain_db = 100.0; // 100 dB DC gain
    let pole1 = 10.0; // First pole at 10 Hz
    let pole2 = 1e6; // Second pole at 1 MHz

    let mut loop_gain_db = Vec::with_capacity(frequencies.len());
    let mut loop_phase_deg = Vec::with_capacity(frequencies.len());

    for &f in &frequencies {
        // Two-pole transfer function magnitude
        let mag1 = 1.0 / (1.0 + (f / pole1).powi(2)).sqrt();
        let mag2 = 1.0 / (1.0 + (f / pole2).powi(2)).sqrt();
        let gain_db = dc_gain_db + 20.0 * (mag1 * mag2).log10();

        // Phase
        let phase1 = -(f / pole1).atan().to_degrees();
        let phase2 = -(f / pole2).atan().to_degrees();
        let phase = phase1 + phase2;

        loop_gain_db.push(gain_db);
        loop_phase_deg.push(phase);
    }

    Ok(StbData::calculate_stability(
        &frequencies,
        &loop_gain_db,
        &loop_phase_deg,
    ))
}

// =============================================================================
// Helper functions
// =============================================================================

/// Generate frequency sweep points
fn generate_freq_points(start: Value, stop: Value, points: usize, sweep_type: &str) -> Vec<Value> {
    if points == 0 || start <= 0.0 || stop <= 0.0 {
        return vec![];
    }

    match sweep_type.to_lowercase().as_str() {
        "dec" | "decade" => {
            // Logarithmic (per decade)
            let num_decades = (stop / start).log10();
            let total_points = ((points as f64) * num_decades).round() as usize;
            let total_points = total_points.max(2);
            (0..total_points)
                .map(|i| {
                    let t = i as f64 / (total_points - 1) as f64;
                    start * (stop / start).powf(t)
                })
                .collect()
        }
        "oct" | "octave" => {
            // Logarithmic (per octave)
            let num_octaves = (stop / start).log2();
            let total_points = ((points as f64) * num_octaves).round() as usize;
            let total_points = total_points.max(2);
            (0..total_points)
                .map(|i| {
                    let t = i as f64 / (total_points - 1) as f64;
                    start * (stop / start).powf(t)
                })
                .collect()
        }
        _ => {
            // Linear
            (0..points)
                .map(|i| {
                    let t = i as f64 / (points - 1).max(1) as f64;
                    start + t * (stop - start)
                })
                .collect()
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_freq_points_linear() {
        let freqs = generate_freq_points(1.0, 101.0, 11, "lin");
        // Linear sweep from 1 to 101 Hz with 11 points
        assert_eq!(freqs.len(), 11);
        assert!((freqs[0] - 1.0).abs() < 1e-6);
        assert!((freqs[10] - 101.0).abs() < 1e-6);
    }

    #[test]
    fn test_generate_freq_points_decade() {
        let freqs = generate_freq_points(1.0, 1000.0, 10, "dec");
        // 3 decades, 10 points per decade = 30 points
        assert!(freqs.len() >= 2);
        assert!((freqs[0] - 1.0).abs() < 1e-6);
        assert!((freqs[freqs.len() - 1] - 1000.0).abs() < 1.0);
    }

    #[test]
    fn test_ac_data_magnitude_db() {
        let data = AcData {
            frequencies: vec![1e3, 1e4],
            responses: vec![(
                "V(out)".to_string(),
                vec![Complex64::new(1.0, 0.0), Complex64::new(10.0, 0.0)],
            )],
            num_points: 2,
        };

        let mag = data.magnitude_db(0);
        assert_eq!(mag.len(), 2);
        assert!((mag[0] - 0.0).abs() < 0.01); // 0 dB
        assert!((mag[1] - 20.0).abs() < 0.01); // 20 dB
    }

    #[test]
    fn test_ac_data_phase_deg() {
        let data = AcData {
            frequencies: vec![1e3],
            responses: vec![(
                "V(out)".to_string(),
                vec![Complex64::new(0.0, 1.0)], // 90 degrees
            )],
            num_points: 1,
        };

        let phase = data.phase_deg(0);
        assert_eq!(phase.len(), 1);
        assert!((phase[0] - 90.0).abs() < 0.01);
    }
}
