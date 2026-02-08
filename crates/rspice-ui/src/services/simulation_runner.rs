//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{resolve_simulation_config, SimulationConfigOverrides, Value};
use crate::output_spec::{
    dc_output_value, parse_output_spec, resolve_node_or_ground_index, run_ac_output_at_frequency,
    run_dc_output_sensitivity, OutputSpec, OutputVoltageSpec,
};

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

fn build_engine_config(
    netlist: &rspice_core::Netlist,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
) -> SimulationConfig {
    match options {
        Some(opts) => opts.resolve_simulation_config(Some(&netlist.options)),
        None => resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        ),
    }
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
    options: Option<&crate::simulation::dialog::SimulationOptions>,
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

    // Create engine config with proper precedence:
    // default/UI base < netlist .OPTIONS < explicit UI overrides.
    let config = build_engine_config(&netlist, options);
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

/// Run transient analysis with explicit parameters.
///
/// Unlike `run_simulation`, this path does not depend on `.tran` directives
/// embedded in the netlist text.
pub fn run_transient_analysis(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
) -> Result<TransientData, String> {
    if stop_time <= 0.0 {
        return Err("Transient stop_time must be > 0".to_string());
    }
    if step_time <= 0.0 {
        return Err("Transient step_time must be > 0".to_string());
    }
    if step_time > stop_time {
        return Err("Transient step_time must be <= stop_time".to_string());
    }

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .run_tran(&netlist, stop_time, step_time)
        .map_err(|e| format!("Transient analysis error: {}", e))?;

    let node_names = result.node_names.clone();
    Ok(TransientData::from_result(result, &node_names))
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
                // AcResult voltages are node-indexed without ground, so map
                // node_names index (with ground at 0) to AC vector index.
                let ac_idx = idx.saturating_sub(1);
                // Collect voltage at this node across all frequencies
                let values: Vec<Complex64> = results
                    .iter()
                    .filter_map(|r| r.voltages.get(ac_idx).copied())
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
    let engine = Engine::new(build_engine_config(&netlist, None));

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
    let engine = Engine::new(build_engine_config(&netlist, None));
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
    let engine = Engine::new(build_engine_config(&netlist, None));

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
// Pole-Zero Analysis
// =============================================================================

/// Pole-zero analysis data
#[derive(Debug, Clone)]
pub struct PoleZeroData {
    /// Poles in the s-plane (real, imag)
    pub poles: Vec<(Value, Value)>,
    /// Zeros in the s-plane (real, imag)
    pub zeros: Vec<(Value, Value)>,
    /// DC transfer gain
    pub gain: Value,
}

/// Run pole-zero analysis.
pub fn run_pole_zero_analysis(
    netlist_text: &str,
    input_node: &str,
    input_ref: &str,
    output_node: &str,
    output_ref: &str,
    transfer_type: &str,
    analysis_type: &str,
) -> Result<PoleZeroData, String> {
    let input_node = input_node.trim();
    let output_node = output_node.trim();
    if input_node.is_empty() {
        return Err("Pole-zero input_node is required".to_string());
    }
    if output_node.is_empty() {
        return Err("Pole-zero output_node is required".to_string());
    }

    let transfer_type = transfer_type.trim().to_ascii_uppercase();
    if transfer_type != "VOL" && transfer_type != "CUR" {
        return Err("Pole-zero transfer_type must be VOL or CUR".to_string());
    }

    let analysis_type = analysis_type.trim().to_ascii_uppercase();
    if analysis_type != "PZ" && analysis_type != "POL" && analysis_type != "ZER" {
        return Err("Pole-zero analysis_type must be PZ, POL, or ZER".to_string());
    }

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for pole-zero): {}", e))?;

    let input_idx = resolve_node_or_ground_index(input_node, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero input node '{}' not found", input_node))?;
    let input_ref_idx = resolve_node_or_ground_index(input_ref, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero input reference '{}' not found", input_ref))?;
    let output_idx = resolve_node_or_ground_index(output_node, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero output node '{}' not found", output_node))?;
    let output_ref_idx = resolve_node_or_ground_index(output_ref, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero output reference '{}' not found", output_ref))?;

    if input_idx == input_ref_idx {
        return Err("Pole-zero input_node and input_ref cannot be the same node".to_string());
    }
    if output_idx == output_ref_idx {
        return Err("Pole-zero output_node and output_ref cannot be the same node".to_string());
    }

    let (input_pos, input_neg, input_sign) = canonicalize_pz_port(input_idx, input_ref_idx)
        .map_err(|e| format!("Invalid pole-zero input port: {}", e))?;
    let (output_pos, output_neg, output_sign) = canonicalize_pz_port(output_idx, output_ref_idx)
        .map_err(|e| format!("Invalid pole-zero output port: {}", e))?;

    let input_is_current = transfer_type == "CUR";
    let (compute_poles, compute_zeros) = match analysis_type.as_str() {
        "POL" => (true, false),
        "ZER" => (false, true),
        _ => (true, true),
    };

    let pz_result = engine
        .run_pz_ports(
            &netlist,
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            input_is_current,
            compute_poles,
            compute_zeros,
        )
        .map_err(|e| {
            format!(
                "Pole-zero analysis error (input={:?}->{:?}, output={:?}->{:?}): {}",
                input_idx, input_ref_idx, output_idx, output_ref_idx, e
            )
        })?;

    let mut poles: Vec<(Value, Value)> = pz_result.poles.iter().map(|p| (p.re, p.im)).collect();
    let mut zeros: Vec<(Value, Value)> = pz_result.zeros.iter().map(|z| (z.re, z.im)).collect();

    match analysis_type.as_str() {
        "POL" => zeros.clear(),
        "ZER" => poles.clear(),
        _ => {}
    }

    Ok(PoleZeroData {
        poles,
        zeros,
        gain: input_sign * output_sign * pz_result.dc_gain,
    })
}

// =============================================================================
// Sensitivity Analysis
// =============================================================================

/// Sensitivity analysis data
#[derive(Debug, Clone)]
pub struct SensitivityData {
    /// Output variable that sensitivities are computed for
    pub output_var: String,
    /// (parameter_name, raw_sensitivity, normalized_sensitivity)
    pub sensitivities: Vec<(String, Value, Value)>,
}

/// Run sensitivity analysis against all global netlist parameters.
pub fn run_sensitivity_analysis(
    netlist_text: &str,
    output_var: &str,
    ac_mode: bool,
    frequency: Option<Value>,
) -> Result<SensitivityData, String> {
    let output_var = output_var.trim();
    if output_var.is_empty() {
        return Err("Sensitivity output_var is required".to_string());
    }
    let ac_frequency = if ac_mode {
        let freq = frequency.unwrap_or(1.0);
        if freq <= 0.0 {
            return Err("Sensitivity AC frequency must be > 0".to_string());
        }
        Some(freq)
    } else if frequency.is_some() {
        return Err("Sensitivity frequency is only valid when AC mode is enabled".to_string());
    } else {
        None
    };

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let circuit = engine.build_circuit(&netlist).map_err(|e| {
        format!(
            "Circuit build error (required for sensitivity output resolution): {}",
            e
        )
    })?;
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for sensitivity): {}", e))?;

    let output_spec =
        parse_output_spec(output_var, &dc_result.node_names, &circuit).ok_or_else(|| {
            format!(
                "Sensitivity output '{}' could not be resolved to a node or branch",
                output_var
            )
        })?;
    if let OutputSpec::Voltage(vspec) = &output_spec {
        if vspec.pos == 0 && vspec.neg.is_none() {
            return Err("Sensitivity output node cannot be ground".to_string());
        }
    }

    let nominal_output = if let Some(freq) = ac_frequency {
        run_ac_output_at_frequency(&engine, &netlist, &output_spec, freq)
            .map(|value| value.norm())?
    } else {
        dc_output_value(&dc_result, &output_spec)?
    };

    let params = collect_sensitivity_parameters(&netlist);

    if params.is_empty() {
        return Ok(SensitivityData {
            output_var: output_var.to_string(),
            sensitivities: Vec::new(),
        });
    }

    let mut sensitivities = Vec::new();
    for (name, value) in params {
        if !value.is_finite() || value == 0.0 {
            continue;
        }

        let raw = if let Some(freq) = ac_frequency {
            run_ac_output_sensitivity_finite_difference(
                &engine,
                &netlist,
                &output_spec,
                &name,
                value,
                freq,
            )
            .map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else if let OutputSpec::Voltage(vspec) = &output_spec {
            run_dc_output_sensitivity(&engine, &netlist, *vspec, &name, value)
                .map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else {
            run_dc_output_sensitivity_finite_difference(
                &engine,
                &netlist,
                &output_spec,
                &name,
                value,
            )
            .map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        };

        let normalized = if nominal_output.abs() > 1e-18 {
            (value / nominal_output) * raw
        } else {
            0.0
        };
        sensitivities.push((name, raw, normalized));
    }

    sensitivities.sort_by(|a, b| {
        b.2.abs()
            .partial_cmp(&a.2.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(SensitivityData {
        output_var: output_var.to_string(),
        sensitivities,
    })
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
/// or driven oscillations. Uses the shooting method with Newton iteration.
pub fn run_pss_analysis(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
) -> Result<PssData, String> {
    use rspice_core::analysis::PssConfig;

    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Create engine with appropriate tolerance
    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = tolerance;
    let engine = Engine::new(sim_config);

    // Build PSS configuration
    let pss_config = PssConfig::new(fundamental_freq)
        .with_harmonics(num_harmonics)
        .with_tolerance(tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);

    // Run actual PSS analysis
    let pss_result = engine
        .run_pss(&netlist, pss_config)
        .map_err(|e| format!("PSS error: {}", e))?;

    // Extract results from engine output
    let period = pss_result.period;
    let frequency = 1.0 / period;

    // Get time points from the result (PssAnalysisResult wraps PssResult)
    let time = pss_result.result.time.clone();

    // Build waveforms from PSS result
    let mut waveforms: Vec<(String, Vec<Value>)> = Vec::new();
    let node_names = &pss_result.result.node_names;

    for (i, waveform) in pss_result.result.waveforms.iter().enumerate() {
        let name = node_names
            .get(i)
            .cloned()
            .unwrap_or_else(|| format!("n{}", i + 1));

        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }

        // Extract per-period values from the PeriodicWaveform (values is a pub field)
        let values: Vec<Value> = waveform.values.clone();
        waveforms.push((format!("V({})", name), values));
    }

    // Extract harmonic content via FFT from the waveforms
    let mut harmonics: Vec<(String, Vec<(Value, Value, Value)>)> = Vec::new();
    for (name, waveform_values) in &waveforms {
        let mut node_harmonics: Vec<(Value, Value, Value)> = Vec::new();

        if !waveform_values.is_empty() {
            // Compute FFT to get harmonic content
            let fft_result = compute_fft_harmonics(waveform_values, frequency, num_harmonics);
            node_harmonics = fft_result;
        }

        harmonics.push((name.clone(), node_harmonics));
    }

    Ok(PssData {
        period,
        frequency,
        time,
        waveforms,
        harmonics,
        converged: true, // Engine would have errored if not converged
        settling_cycles: 10,
    })
}

/// Compute FFT harmonics from time-domain waveform
fn compute_fft_harmonics(
    waveform: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
) -> Vec<(Value, Value, Value)> {
    use std::f64::consts::PI;

    let n = waveform.len();
    if n == 0 {
        return vec![];
    }

    let mut harmonics = Vec::with_capacity(num_harmonics + 1);

    // DC component (harmonic 0)
    let dc = waveform.iter().sum::<Value>() / n as Value;
    harmonics.push((0.0, dc, 0.0));

    // Compute harmonics using DFT
    for h in 1..=num_harmonics {
        let freq = fundamental_freq * h as Value;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (i, &sample) in waveform.iter().enumerate() {
            let phase = 2.0 * PI * h as Value * i as Value / n as Value;
            real += sample * phase.cos();
            imag -= sample * phase.sin();
        }

        real *= 2.0 / n as Value;
        imag *= 2.0 / n as Value;

        let magnitude = (real * real + imag * imag).sqrt();
        let phase_deg = imag.atan2(real).to_degrees();

        harmonics.push((freq, magnitude, phase_deg));
    }

    harmonics
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
    _tone2_harmonics: usize,
) -> Result<HbData, String> {
    use rspice_core::analysis::HbConfig;

    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(build_engine_config(&netlist, None));

    // Build HB configuration
    // Note: Multi-tone HB requires additional configuration - for now we focus on single tone
    let hb_config = HbConfig::new(tone1_freq)
        .with_harmonics(tone1_harmonics)
        .with_tolerance(1e-6);

    // Run actual HB analysis
    let hb_result = engine
        .run_hb(&netlist, hb_config)
        .map_err(|e| format!("HB error: {}", e))?;

    // Build fundamentals list
    let mut fundamentals = vec![tone1_freq];
    let mut harmonics_per_tone = vec![tone1_harmonics];

    if let Some(f2) = tone2_freq {
        fundamentals.push(f2);
        harmonics_per_tone.push(_tone2_harmonics);
    }

    // Extract DC operating point from spectral data
    let dc_voltages: Vec<(String, Value)> = hb_result
        .result
        .spectral_voltages
        .iter()
        .map(|sv| {
            // DC is the zeroth harmonic (real part only for DC)
            let dc_val = sv.coefficients.first().map(|c| c.re).unwrap_or(0.0);
            (sv.node_name.clone(), dc_val)
        })
        .collect();

    // Build spectra from HB result's spectral voltages
    let mut spectra = Vec::new();
    for sv in &hb_result.result.spectral_voltages {
        let mut spectrum = Vec::new();

        // For each harmonic coefficient
        for (h, coeff) in sv.coefficients.iter().enumerate() {
            let freq = tone1_freq * h as Value;
            let magnitude = coeff.norm();
            let phase_deg = coeff.arg().to_degrees();
            spectrum.push((freq, magnitude, phase_deg));
        }

        spectra.push((format!("V({})", sv.node_name), spectrum));
    }

    // Number of frequency components
    let num_components = hb_result.num_harmonics + 1;

    Ok(HbData {
        fundamentals,
        harmonics_per_tone,
        dc_voltages,
        spectra,
        num_components,
        converged: hb_result.converged,
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
/// phase margin and gain margin using AC analysis data.
pub fn run_stb_analysis(
    netlist_text: &str,
    probe_node: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
) -> Result<StbData, String> {
    use rspice_core::analysis::advanced::stb::{StbAnalyzer, StbConfig};

    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(build_engine_config(&netlist, None));

    // Create STB configuration
    let stb_config = StbConfig::new()
        .with_sweep(start_freq, stop_freq, points_per_decade)
        .with_probe(probe_node)
        .with_nyquist(true);

    // Get frequency points from config
    let frequencies = stb_config.frequency_points();

    // Run AC analysis at all frequency points
    // run_ac takes netlist and &[Value] frequencies, returns Vec<AcResult>
    let ac_results = engine
        .run_ac(&netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;

    // Find the probe node index from the first result
    // Node names are derived from the circuit, probe_node is 1-indexed
    let probe_idx = probe_node.parse::<usize>().unwrap_or(1).saturating_sub(1);

    // Extract loop gain at each frequency from AC results
    // AcResult contains: frequency, voltages (Vec<Complex64>), currents
    let loop_gains: Vec<Complex64> = ac_results
        .iter()
        .map(|result| {
            // Get complex voltage at probe node
            result
                .voltages
                .get(probe_idx)
                .copied()
                .unwrap_or(Complex64::new(1.0, 0.0))
        })
        .collect();

    // Extract frequency points from results
    let result_frequencies: Vec<Value> = ac_results.iter().map(|r| r.frequency).collect();

    // Use StbAnalyzer to extract stability margins
    let analyzer = StbAnalyzer::new(stb_config);
    let stb_result = analyzer.analyze(&result_frequencies, &loop_gains);

    // Convert Bode data to our format
    let loop_gain_db: Vec<Value> = stb_result
        .bode_points
        .iter()
        .map(|p| p.magnitude_db)
        .collect();

    let loop_phase_deg: Vec<Value> = stb_result.bode_points.iter().map(|p| p.phase_deg).collect();

    // Build StbData from the analysis result
    Ok(StbData {
        frequencies: result_frequencies,
        loop_gain_db,
        loop_phase_deg,
        phase_margin: stb_result.margins.phase_margin_deg,
        gain_margin: stb_result.margins.gain_margin_db,
        unity_gain_freq: stb_result.margins.unity_gain_bandwidth,
        phase_crossover_freq: stb_result.margins.gain_margin_freq,
        is_stable: stb_result.margins.is_stable(),
    })
}

// =============================================================================
// Helper functions
// =============================================================================

fn canonicalize_pz_port(pos: usize, neg: usize) -> Result<(usize, Option<usize>, Value), String> {
    if pos == neg {
        return Err("positive and reference nodes cannot be the same".to_string());
    }

    if pos != 0 {
        return Ok((pos, if neg == 0 { None } else { Some(neg) }, 1.0));
    }

    if neg == 0 {
        return Err("port cannot be ground-ground".to_string());
    }

    // Canonicalize V(0, n) or I(0, n) to -(V(n,0) / I(n,0)).
    Ok((neg, None, -1.0))
}

fn collect_sensitivity_parameters(netlist: &rspice_core::Netlist) -> Vec<(String, Value)> {
    let mut params: Vec<(String, Value)> = netlist
        .params
        .all_params()
        .into_iter()
        .filter(|(name, value)| {
            value.is_finite() && !name.starts_with("IC_") && !name.starts_with("NODESET_")
        })
        .collect();
    params.sort_by(|a, b| a.0.cmp(&b.0));
    params
}

fn run_ac_output_sensitivity_finite_difference(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: &OutputSpec,
    param_name: &str,
    param_value: Value,
    frequency: Value,
) -> Result<Value, String> {
    let delta = (param_value.abs() * 0.01).max(1e-12);

    let mut netlist_plus = netlist.clone();
    netlist_plus.params.set(param_name, param_value + delta);
    let plus = run_ac_output_at_frequency(engine, &netlist_plus, output_spec, frequency)?;

    let mut netlist_minus = netlist.clone();
    netlist_minus.params.set(param_name, param_value - delta);
    let minus = run_ac_output_at_frequency(engine, &netlist_minus, output_spec, frequency)?;

    Ok((plus.norm() - minus.norm()) / (2.0 * delta))
}

fn run_dc_output_sensitivity_finite_difference(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: &OutputSpec,
    param_name: &str,
    param_value: Value,
) -> Result<Value, String> {
    let delta = (param_value.abs() * 0.01).max(1e-12);

    let mut netlist_plus = netlist.clone();
    netlist_plus.params.set(param_name, param_value + delta);
    let plus_result = engine
        .run_dc_op(&netlist_plus)
        .map_err(|e| format!("DC OP error (plus perturbation): {}", e))?;
    let plus = dc_output_value(&plus_result, output_spec)?;

    let mut netlist_minus = netlist.clone();
    netlist_minus.params.set(param_name, param_value - delta);
    let minus_result = engine
        .run_dc_op(&netlist_minus)
        .map_err(|e| format!("DC OP error (minus perturbation): {}", e))?;
    let minus = dc_output_value(&minus_result, output_spec)?;

    Ok((plus - minus) / (2.0 * delta))
}

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

    #[test]
    fn test_ac_data_from_results_node_mapping() {
        let results = vec![
            AcResult {
                frequency: 1e3,
                voltages: vec![Complex64::new(1.0, 0.0), Complex64::new(0.5, 0.0)],
                currents: vec![],
            },
            AcResult {
                frequency: 1e6,
                voltages: vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
                currents: vec![],
            },
        ];
        let node_names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];

        let data = AcData::from_results(results, &node_names);
        assert_eq!(data.responses.len(), 2);
        assert_eq!(data.responses[0].0, "V(IN)");
        assert_eq!(data.responses[0].1[0], Complex64::new(1.0, 0.0));
        assert_eq!(data.responses[1].0, "V(OUT)");
        assert_eq!(data.responses[1].1[0], Complex64::new(0.5, 0.0));
    }

    #[test]
    fn test_build_engine_config_applies_netlist_options_when_ui_is_none() {
        let netlist = rspice_core::netlist::parse_netlist(
            r#"Test
.OPTIONS TEMP=85 ITL1=120 METHOD=GEAR RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 GMIN=1e-11
.END
"#,
        )
        .expect("netlist should parse");

        let config = build_engine_config(&netlist, None);

        assert!((config.temperature - 358.15).abs() < 1e-12);
        assert_eq!(config.max_iterations, 120);
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::Gear2
        );
        assert!((config.tolerance - 2e-4).abs() < 1e-15);
        assert!((config.convergence_config.voltage_reltol - 2e-4).abs() < 1e-15);
        assert!((config.convergence_config.residual_reltol - 2e-4).abs() < 1e-15);
        assert!((config.convergence_config.voltage_abstol - 3e-6).abs() < 1e-18);
        assert!((config.convergence_config.current_abstol - 4e-12).abs() < 1e-24);
        assert!((config.convergence_config.gmin_initial - 1e-11).abs() < 1e-24);
    }

    #[test]
    fn test_build_engine_config_ui_options_override_netlist_options() {
        let netlist = rspice_core::netlist::parse_netlist(
            r#"Test
.OPTIONS TEMP=125 ITL1=200 METHOD=GEAR RELTOL=2e-4 VNTOL=2e-6 IABSTOL=2e-12 GMIN=1e-11
.END
"#,
        )
        .expect("netlist should parse");

        let mut ui = crate::simulation::dialog::SimulationOptions::default();
        ui.temp = 27.0;
        ui.itl1 = 90;
        ui.method = crate::simulation::dialog::IntegrationMethod::Trap;
        ui.reltol = 7e-4;
        ui.residual_reltol = 4e-4;
        ui.vntol = 9e-6;
        ui.iabstol = 8e-12;
        ui.gmin = 3e-10;

        let config = build_engine_config(&netlist, Some(&ui));

        assert!((config.temperature - 300.15).abs() < 1e-12);
        assert_eq!(config.max_iterations, 90);
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::Trapezoidal
        );
        assert!((config.tolerance - 7e-4).abs() < 1e-15);
        assert!((config.convergence_config.voltage_reltol - 7e-4).abs() < 1e-15);
        assert!((config.convergence_config.residual_reltol - 4e-4).abs() < 1e-15);
        assert!((config.convergence_config.voltage_abstol - 9e-6).abs() < 1e-18);
        assert!((config.convergence_config.current_abstol - 8e-12).abs() < 1e-24);
        assert!((config.convergence_config.gmin_initial - 3e-10).abs() < 1e-22);
    }

    #[test]
    fn test_transient_analysis_validation() {
        let netlist = "* test\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.end\n";

        let err =
            run_transient_analysis(netlist, 0.0, 1e-9).expect_err("expected validation error");
        assert!(err.contains("stop_time"));

        let err =
            run_transient_analysis(netlist, 1e-6, 0.0).expect_err("expected validation error");
        assert!(err.contains("step_time"));
    }

    #[test]
    fn test_parse_output_node_helper() {
        let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        assert_eq!(crate::output_spec::parse_output_node("V(OUT)", &names), Some(2));
        assert_eq!(crate::output_spec::parse_output_node("out", &names), Some(2));
        assert_eq!(crate::output_spec::parse_output_node("2", &names), Some(2));
        assert_eq!(crate::output_spec::parse_output_node("V(OUT,IN)", &names), None);
        assert_eq!(crate::output_spec::parse_output_node("I(R1)", &names), None);
    }

    #[test]
    fn test_parse_output_voltage_spec_helper() {
        let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        assert_eq!(
            crate::output_spec::parse_output_voltage_spec("V(OUT)", &names),
            Some(OutputVoltageSpec { pos: 2, neg: None })
        );
        assert_eq!(
            crate::output_spec::parse_output_voltage_spec("V(OUT,IN)", &names),
            Some(OutputVoltageSpec {
                pos: 2,
                neg: Some(1)
            })
        );
        assert_eq!(
            crate::output_spec::parse_output_voltage_spec("V(OUT,GND)", &names),
            Some(OutputVoltageSpec {
                pos: 2,
                neg: Some(0)
            })
        );
        assert_eq!(crate::output_spec::parse_output_voltage_spec("I(R1)", &names), None);
    }

    #[test]
    fn test_parse_output_spec_current_helper() {
        let netlist = rspice_core::netlist::parse_netlist("* t\nV1 in 0 1\nR1 in 0 1k\n")
            .expect("netlist should parse");
        let engine = Engine::new(SimulationConfig::default());
        let circuit = engine
            .build_circuit(&netlist)
            .expect("circuit build should succeed");
        let node_names = vec!["0".to_string(), "IN".to_string()];

        let spec = crate::output_spec::parse_output_spec("I(V1)", &node_names, &circuit);
        assert!(matches!(
            spec,
            Some(OutputSpec::BranchCurrent {
                branch_ordinal: 1,
                ..
            })
        ));
    }

    #[test]
    fn test_run_pole_zero_analysis_validation() {
        let netlist = "* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n";

        let err = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "BAD", "PZ")
            .expect_err("expected transfer_type validation");
        assert!(err.contains("transfer_type"));

        let err = run_pole_zero_analysis(netlist, "in", "nref", "out", "0", "VOL", "PZ")
            .expect_err("expected reference validation");
        assert!(err.contains("not found"));

        let err = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "BAD")
            .expect_err("expected analysis_type validation");
        assert!(err.contains("analysis_type"));

        run_pole_zero_analysis(netlist, "in", "0", "out", "0", "CUR", "PZ")
            .expect("CUR transfer_type should be accepted");
    }

    #[test]
    fn test_run_pole_zero_analysis_filters_analysis_type() {
        let netlist = "* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n";

        let pol = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "POL")
            .expect("POL run should succeed");
        assert!(pol.zeros.is_empty());

        let zer = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "ZER")
            .expect("ZER run should succeed");
        assert!(zer.poles.is_empty());
    }

    #[test]
    fn test_run_pole_zero_analysis_supports_non_ground_references() {
        let netlist =
            "* pz diff\nV1 in 0 1\nR1 in out 1k\nR2 out ref 500\nC1 out ref 1n\nR3 ref 0 1k\n";

        let diff = run_pole_zero_analysis(netlist, "in", "ref", "out", "ref", "VOL", "PZ")
            .expect("differential pole-zero should succeed");

        let h11 = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "PZ")
            .expect("h11 should succeed")
            .gain;
        let h12 = run_pole_zero_analysis(netlist, "ref", "0", "out", "0", "VOL", "PZ")
            .expect("h12 should succeed")
            .gain;
        let h21 = run_pole_zero_analysis(netlist, "in", "0", "ref", "0", "VOL", "PZ")
            .expect("h21 should succeed")
            .gain;
        let h22 = run_pole_zero_analysis(netlist, "ref", "0", "ref", "0", "VOL", "PZ")
            .expect("h22 should succeed")
            .gain;
        let expected = h11 - h12 - h21 + h22;

        assert!((diff.gain - expected).abs() < 1e-9);
    }

    #[test]
    fn test_run_pole_zero_analysis_voltage_mode_reports_highpass_zero() {
        let netlist = "* hp\nC1 in out 1n\nR1 out 0 1k\n";

        let result = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "ZER")
            .expect("voltage-mode zero analysis should succeed");

        assert!(
            result
                .zeros
                .iter()
                .any(|(re, im)| (re * re + im * im).sqrt() < 1e-2),
            "expected zero near origin, got {:?}",
            result.zeros
        );
    }

    #[test]
    fn test_run_sensitivity_analysis_validation() {
        let netlist = "* sens\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n";

        let err = run_sensitivity_analysis(netlist, "", false, None)
            .expect_err("expected output variable validation");
        assert!(err.contains("output_var"));

        let err = run_sensitivity_analysis(netlist, "V(out)", true, Some(0.0))
            .expect_err("expected AC frequency validation");
        assert!(err.contains("frequency"));

        let err = run_sensitivity_analysis(netlist, "V(out)", false, Some(1e6))
            .expect_err("expected mode/frequency validation");
        assert!(err.contains("only valid"));

        let err = run_sensitivity_analysis(netlist, "I(NO_SUCH_BRANCH)", false, None)
            .expect_err("expected branch output resolution failure");
        assert!(err.contains("resolved"));
    }

    #[test]
    fn test_run_sensitivity_analysis_no_parameters_returns_empty() {
        let netlist = "* sens\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
            .expect("sensitivity run should succeed");
        assert_eq!(result.output_var, "V(out)");
        assert!(result.sensitivities.is_empty());
    }

    #[test]
    fn test_run_sensitivity_analysis_with_parameter() {
        let netlist = "* sens\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
            .expect("sensitivity run should succeed");
        assert_eq!(result.output_var, "V(out)");
        assert!(!result.sensitivities.is_empty());
        assert!(result
            .sensitivities
            .iter()
            .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL")));
    }

    #[test]
    fn test_run_sensitivity_analysis_filters_internal_side_channel_parameters() {
        let netlist = "* sens params\n.param RVAL=1k\n.param IC_START=0.1\n.param NODESET_OUT=0.2\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
            .expect("sensitivity run should succeed");

        assert!(result
            .sensitivities
            .iter()
            .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL")));
        assert!(result
            .sensitivities
            .iter()
            .all(|(name, _, _)| !name.starts_with("IC_") && !name.starts_with("NODESET_")));
    }

    #[test]
    fn test_run_sensitivity_analysis_ac_mode_with_parameter() {
        let netlist = "* sens ac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in out RVAL\nC1 out 0 1n\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", true, Some(1e6))
            .expect("ac sensitivity run should succeed");
        assert_eq!(result.output_var, "V(out)");
        let (_name, raw, normalized) = result
            .sensitivities
            .iter()
            .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
            .expect("expected RVAL sensitivity");
        assert!(raw.is_finite());
        assert!(normalized.is_finite());
    }

    #[test]
    fn test_run_sensitivity_analysis_supports_differential_output() {
        let netlist = "* sens diff\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out,in)", false, None)
            .expect("differential sensitivity run should succeed");
        assert!(!result.sensitivities.is_empty());
        assert!(result
            .sensitivities
            .iter()
            .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite()));
    }

    #[test]
    fn test_run_sensitivity_analysis_supports_current_output_dc() {
        let netlist = "* sens i\n.param RVAL=1k\nV1 in 0 1\nR1 in 0 {RVAL}\n";

        let result = run_sensitivity_analysis(netlist, "I(V1)", false, None)
            .expect("current-output dc sensitivity should succeed");
        assert_eq!(result.output_var, "I(V1)");
        assert!(!result.sensitivities.is_empty());
        let (_name, raw, normalized) = result
            .sensitivities
            .iter()
            .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
            .expect("expected RVAL sensitivity");
        assert!(raw.is_finite());
        assert!(normalized.is_finite());
    }

    #[test]
    fn test_run_sensitivity_analysis_supports_current_output_ac() {
        let netlist = "* sens iac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in 0 {RVAL}\n";

        let result = run_sensitivity_analysis(netlist, "I(V1)", true, Some(1e3))
            .expect("current-output ac sensitivity should succeed");
        assert_eq!(result.output_var, "I(V1)");
        assert!(!result.sensitivities.is_empty());
        assert!(result
            .sensitivities
            .iter()
            .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite()));
    }
}
