//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, collect_sensitivity_parameters, dc_output_value,
    finite_difference_derivative, normalized_sensitivity, parse_output_spec,
    resolve_node_or_ground_index, resolve_sensitivity_ac_frequency, run_ac_output_at_frequency,
    run_dc_output_sensitivity, validate_sensitivity_output_spec,
};
use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{
    AnalysisCommand, ElementKind, MonteCarloDistribution, SourceSpec, StepCommand, StepSweep,
    StepTarget,
};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use rspice_core::{SimulationConfigOverrides, Value, resolve_simulation_config};

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

const DEFAULT_MONTE_CARLO_SEED: u64 = 0x5EED_5EED;

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
    let ac_frequency = resolve_sensitivity_ac_frequency(ac_mode, frequency)?;

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
    validate_sensitivity_output_spec(&output_spec)?;

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
    let mut perturbed_netlist = netlist.clone();
    for (name, value) in params {
        if !value.is_finite() || value == 0.0 {
            continue;
        }

        let raw = if let Some(freq) = ac_frequency {
            let result = finite_difference_derivative(value, |candidate| {
                perturbed_netlist.params.set(&name, candidate);
                run_ac_output_at_frequency(&engine, &perturbed_netlist, &output_spec, freq)
                    .map(|value| value.norm())
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else if let OutputSpec::Voltage(vspec) = &output_spec {
            run_dc_output_sensitivity(&engine, &netlist, *vspec, &name, value)
                .map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else {
            let result = finite_difference_derivative(value, |candidate| {
                perturbed_netlist.params.set(&name, candidate);
                let dc_result = engine
                    .run_dc_op(&perturbed_netlist)
                    .map_err(|e| format!("DC OP error (perturbation): {}", e))?;
                dc_output_value(&dc_result, &output_spec)
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        };

        let normalized = normalized_sensitivity(raw, value, nominal_output);
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
// Monte Carlo Analysis
// =============================================================================

/// Monte Carlo variable summary statistics.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableData {
    pub name: String,
    pub mean: Value,
    pub std_dev: Value,
    pub min: Value,
    pub max: Value,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<Value>,
}

/// Monte Carlo analysis data.
#[derive(Debug, Clone)]
pub struct MonteCarloData {
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub num_failures: usize,
    pub all_converged: bool,
    pub variables: Vec<MonteCarloVariableData>,
}

/// Run Monte Carlo analysis by executing the first `.MC` command in the netlist.
pub fn run_monte_carlo_analysis(netlist_text: &str) -> Result<MonteCarloData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mc_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(cmd) => Some(cmd),
            _ => None,
        })
        .ok_or_else(|| "Monte Carlo analysis requires a .MC command in the netlist".to_string())?;

    let distribution = match mc_cmd.distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: mc_cmd.relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: mc_cmd.relative_spread,
        },
    };

    let seed = mc_cmd.seed.unwrap_or(DEFAULT_MONTE_CARLO_SEED);
    let parameter_filter = (!mc_cmd.params.is_empty()).then_some(mc_cmd.params.as_slice());

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .run_monte_carlo_with_options(&netlist, mc_cmd.runs, seed, distribution, parameter_filter)
        .map_err(|e| format!("Monte Carlo analysis error: {}", e))?;

    let mut variables: Vec<MonteCarloVariableData> = result
        .variables
        .into_values()
        .map(|stats| MonteCarloVariableData {
            name: stats.name,
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            histogram: stats.histogram,
            bin_edges: stats.bin_edges,
        })
        .collect();
    variables.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(MonteCarloData {
        runs_requested: mc_cmd.runs,
        runs_completed: result.num_runs,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
    })
}

// =============================================================================
// Parametric (.STEP) Analysis
// =============================================================================

/// Parametric sweep data.
#[derive(Debug, Clone)]
pub struct ParametricData {
    pub target: String,
    pub sweep_values: Vec<Value>,
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_points: usize,
    pub num_failures: usize,
}

/// Explicit configuration for temperature sweep execution.
#[derive(Debug, Clone)]
pub struct TempRunConfig {
    pub temperatures_c: Vec<Value>,
    pub base_mode: CornerBaseMode,
}

impl Default for TempRunConfig {
    fn default() -> Self {
        Self {
            temperatures_c: vec![25.0],
            base_mode: CornerBaseMode::Op,
        }
    }
}

impl TempRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.temperatures_c.is_empty() {
            return Err("Temperature sweep requires at least one temperature point".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Temperature sweep points must be finite values".to_string());
        }
        validate_base_mode("Temperature sweep", &self.base_mode)
    }
}

/// Run parametric analysis by executing the first `.STEP` command in the netlist.
pub fn run_parametric_analysis(netlist_text: &str) -> Result<ParametricData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let step_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .ok_or_else(|| "Parametric analysis requires a .STEP command in the netlist".to_string())?;

    let values = expand_step_sweep_values(&step_cmd.sweep)?;
    if values.is_empty() {
        return Err("Parametric analysis has no sweep points to execute".to_string());
    }

    let results = if step_cmd.target == StepTarget::Temp {
        let cfg = TempRunConfig {
            temperatures_c: values.clone(),
            base_mode: CornerBaseMode::Op,
        };
        return run_parametric_analysis_with_netlist_and_config(&netlist, &cfg, "TEMP");
    } else {
        let engine = Engine::new(build_engine_config(&netlist, None));
        engine
            .run_step_command(&netlist, step_cmd, &values)
            .map_err(|e| format!("Parametric analysis error: {}", e))?
    };

    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = values.len().saturating_sub(results.len());
    let (sweep_values, voltages) = map_dc_sweep_results(&results);

    Ok(ParametricData {
        target: describe_step_target(step_cmd),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}

/// Run temperature sweep analysis with explicit base-mode configuration.
pub fn run_parametric_analysis_with_config(
    netlist_text: &str,
    config: &TempRunConfig,
) -> Result<ParametricData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    run_parametric_analysis_with_netlist_and_config(&netlist, config, "TEMP")
}

fn run_parametric_analysis_with_netlist_and_config(
    netlist: &rspice_core::Netlist,
    config: &TempRunConfig,
    target: &str,
) -> Result<ParametricData, String> {
    config.validate()?;

    let results = run_temperature_sweep(netlist, &config.temperatures_c, &config.base_mode)?;
    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = config.temperatures_c.len().saturating_sub(results.len());
    let metric_label = config.base_mode.metric_label();
    let (sweep_values, voltages) = map_temperature_results(&results, metric_label);

    Ok(ParametricData {
        target: target.to_string(),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}

// =============================================================================
// Corner Analysis
// =============================================================================

/// Process-corner designation for UI corner sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

impl CornerProcess {
    fn as_keyword(self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
        }
    }

    fn nmos_factor(self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::SF => 0.9,
            Self::FF | Self::FS => 1.1,
        }
    }

    fn pmos_factor(self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::FS => 0.9,
            Self::FF | Self::SF => 1.1,
        }
    }
}

/// Explicit configuration for corner sweep execution.
#[derive(Debug, Clone)]
pub struct CornerRunConfig {
    pub process_corners: Vec<CornerProcess>,
    pub voltages: Vec<Value>,
    pub temperatures_c: Vec<Value>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<Value>,
    pub base_mode: CornerBaseMode,
}

/// Frequency sweep type used by corner AC base analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl CornerFrequencySweep {
    fn as_keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Base analysis executed at each corner point.
#[derive(Debug, Clone)]
pub enum CornerBaseMode {
    /// Run DC operating point directly at each corner.
    Op,
    /// Run DC sweep and record the final converged point at each corner.
    DcSweep {
        source_name: String,
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Run transient analysis and record the terminal sample at each corner.
    Transient { stop_time: Value, step_time: Value },
    /// Run AC analysis and record terminal-frequency magnitude at each corner.
    Ac {
        start_freq: Value,
        stop_freq: Value,
        points_per_unit: usize,
        sweep: CornerFrequencySweep,
    },
}

impl CornerBaseMode {
    fn metric_label(&self) -> CornerMetricLabel {
        match self {
            Self::Ac { .. } => CornerMetricLabel::AcMagnitude,
            _ => CornerMetricLabel::Voltage,
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::Op => "OP",
            Self::DcSweep { .. } => "DC",
            Self::Transient { .. } => "TRAN",
            Self::Ac { .. } => "AC",
        }
    }
}

impl Default for CornerBaseMode {
    fn default() -> Self {
        Self::Op
    }
}

impl Default for CornerRunConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::default(),
        }
    }
}

impl CornerRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("Corner analysis requires at least one process corner".to_string());
        }
        if self.voltages.is_empty() {
            return Err("Corner analysis requires at least one voltage corner".to_string());
        }
        if self.voltages.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err(
                "Corner analysis voltage corners must be positive finite values".to_string(),
            );
        }
        if self.temperatures_c.is_empty() {
            return Err("Corner analysis requires at least one temperature corner".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Corner analysis temperature corners must be finite values".to_string());
        }
        if let Some(vnom) = self.nominal_voltage {
            if !vnom.is_finite() || vnom <= 0.0 {
                return Err(
                    "Corner analysis nominal voltage must be a positive finite value".to_string(),
                );
            }
        }
        validate_base_mode("Corner", &self.base_mode)?;
        Ok(())
    }
}

fn validate_base_mode(context: &str, base_mode: &CornerBaseMode) -> Result<(), String> {
    match base_mode {
        CornerBaseMode::Op => {}
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            if source_name.trim().is_empty() {
                return Err(format!(
                    "{} DC sweep base mode requires a non-empty source name",
                    context
                ));
            }
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(format!(
                    "{} DC sweep base mode requires finite start/stop/step values",
                    context
                ));
            }
            if *step == 0.0 {
                return Err(format!(
                    "{} DC sweep base mode step cannot be zero",
                    context
                ));
            }
            if (stop - start).abs() > 0.0 && (stop - start).signum() != step.signum() {
                return Err(format!(
                    "{} DC sweep base mode step direction must match start/stop range",
                    context
                ));
            }
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode stop_time must be a positive finite value",
                    context
                ));
            }
            if !step_time.is_finite() || *step_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode step_time must be a positive finite value",
                    context
                ));
            }
            if step_time > stop_time {
                return Err(format!(
                    "{} transient base mode step_time must be <= stop_time",
                    context
                ));
            }
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            ..
        } => {
            if !start_freq.is_finite() || !stop_freq.is_finite() {
                return Err(format!(
                    "{} AC base mode requires finite start/stop frequencies",
                    context
                ));
            }
            if *start_freq <= 0.0 || *stop_freq <= 0.0 {
                return Err(format!(
                    "{} AC base mode frequencies must be positive values",
                    context
                ));
            }
            if stop_freq < start_freq {
                return Err(format!(
                    "{} AC base mode stop frequency must be >= start frequency",
                    context
                ));
            }
            if *points_per_unit == 0 {
                return Err(format!(
                    "{} AC base mode points_per_unit must be greater than zero",
                    context
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct CornerPoint {
    process: CornerProcess,
    voltage: Value,
    temperature_c: Value,
}

impl CornerPoint {
    fn label(&self) -> String {
        format!(
            "{}_{:.6}V_{:.6}C",
            self.process.as_keyword(),
            self.voltage,
            self.temperature_c
        )
    }
}

#[derive(Debug, Clone)]
struct SweepPointResult {
    node_names: Vec<String>,
    node_values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CornerMetricLabel {
    Voltage,
    AcMagnitude,
}

impl CornerMetricLabel {
    fn format_trace_name(self, node_name: &str) -> String {
        match self {
            Self::Voltage => format!("V({})", node_name),
            Self::AcMagnitude => format!("|V({})|", node_name),
        }
    }
}

/// Temperature/process/voltage corner sweep data.
#[derive(Debug, Clone)]
pub struct CornerData {
    /// X-axis values for each executed corner point.
    pub x_values: Vec<Value>,
    /// X-axis label for corner traces.
    pub x_label: String,
    /// X-axis unit for corner traces.
    pub x_unit: String,
    /// Temperature for each executed corner point.
    pub temperatures_c: Vec<Value>,
    /// Human-readable corner labels in execution order.
    pub corner_labels: Vec<String>,
    /// Per-node values for each corner point.
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_points: usize,
    pub num_failures: usize,
}

/// Run corner analysis from `.TEMP` commands in the netlist.
///
/// This compatibility entry point executes temperature-only TT/nominal sweeps.
pub fn run_corner_analysis(netlist_text: &str) -> Result<CornerData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let temperatures = extract_temp_points(&netlist);

    if temperatures.is_empty() {
        return Err("Corner analysis requires at least one .TEMP command".to_string());
    }

    let config = CornerRunConfig {
        temperatures_c: temperatures,
        ..Default::default()
    };
    run_corner_analysis_with_netlist(&netlist, &config)
}

/// Run corner analysis with explicit process/voltage/temperature configuration.
pub fn run_corner_analysis_with_config(
    netlist_text: &str,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    run_corner_analysis_with_netlist(&netlist, config)
}

fn run_corner_analysis_with_netlist(
    netlist: &rspice_core::Netlist,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    config.validate()?;
    let points = expand_corner_points(config);
    if points.is_empty() {
        return Err("Corner analysis produced no corner points".to_string());
    }

    let nominal_voltage = config
        .nominal_voltage
        .or_else(|| infer_nominal_supply_voltage(netlist))
        .unwrap_or(1.0);
    let results = run_corner_sweep(netlist, &points, config, nominal_voltage)?;
    if results.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_string());
    }

    let num_failures = points.len().saturating_sub(results.len());
    let metric = config.base_mode.metric_label();
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&results, metric);

    Ok(CornerData {
        x_values,
        x_label,
        x_unit,
        num_points: temperatures_c.len(),
        temperatures_c,
        corner_labels,
        voltages,
        num_failures,
    })
}

// =============================================================================
// Helper functions
// =============================================================================

fn describe_step_target(step_cmd: &StepCommand) -> String {
    match step_cmd.target {
        StepTarget::Param => format!("PARAM {}", step_cmd.name),
        StepTarget::Device => match step_cmd.param_name.as_deref() {
            Some(param) => format!("DEVICE {}.{}", step_cmd.name, param),
            None => format!("DEVICE {}", step_cmd.name),
        },
        StepTarget::Model => {
            let param = step_cmd.param_name.as_deref().unwrap_or("PARAM");
            format!("MODEL {}.{}", step_cmd.name, param)
        }
        StepTarget::Temp => "TEMP".to_string(),
    }
}

fn map_dc_sweep_results(
    results: &[(Value, CoreSimulationResult)],
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(value, _)| *value).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_voltages.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_voltages.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((format!("V({})", node_name), values));
        }
    }

    (sweep_values, voltages)
}

fn map_temperature_results(
    results: &[(Value, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(temp_c, _)| *temp_c).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (sweep_values, voltages)
}

fn map_corner_results(
    results: &[(CornerPoint, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (
    Vec<Value>,
    String,
    String,
    Vec<Value>,
    Vec<String>,
    Vec<(String, Vec<Value>)>,
) {
    let temperatures_c: Vec<Value> = results
        .iter()
        .map(|(point, _)| point.temperature_c)
        .collect();
    let (x_values, x_label, x_unit) = corner_axis_from_points(results, &temperatures_c);
    let corner_labels: Vec<String> = results.iter().map(|(point, _)| point.label()).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        voltages,
    )
}

fn corner_axis_from_points(
    results: &[(CornerPoint, SweepPointResult)],
    temperatures_c: &[Value],
) -> (Vec<Value>, String, String) {
    if results.is_empty() {
        return (Vec::new(), "Corner Index".to_string(), String::new());
    }

    let first_point = &results[0].0;
    let single_process = results
        .iter()
        .all(|(point, _)| point.process == first_point.process);
    let single_voltage = results
        .iter()
        .all(|(point, _)| (point.voltage - first_point.voltage).abs() < 1e-15);

    let mut seen_temps = std::collections::HashSet::with_capacity(temperatures_c.len());
    let has_duplicate_temp = temperatures_c
        .iter()
        .any(|temperature| !seen_temps.insert(temperature.to_bits()));

    if single_process && single_voltage && !has_duplicate_temp {
        return (
            temperatures_c.to_vec(),
            "Temperature".to_string(),
            "C".to_string(),
        );
    }

    (
        (0..results.len()).map(|index| index as Value).collect(),
        "Corner Index".to_string(),
        String::new(),
    )
}

fn expand_corner_points(config: &CornerRunConfig) -> Vec<CornerPoint> {
    if config.full_matrix {
        let mut points = Vec::with_capacity(
            config.process_corners.len() * config.voltages.len() * config.temperatures_c.len(),
        );
        for process in &config.process_corners {
            for &voltage in &config.voltages {
                for &temperature_c in &config.temperatures_c {
                    points.push(CornerPoint {
                        process: *process,
                        voltage,
                        temperature_c,
                    });
                }
            }
        }
        return points;
    }

    let n = config
        .process_corners
        .len()
        .max(config.voltages.len())
        .max(config.temperatures_c.len());
    let mut points = Vec::with_capacity(n);
    for idx in 0..n {
        points.push(CornerPoint {
            process: config.process_corners[idx % config.process_corners.len()],
            voltage: config.voltages[idx % config.voltages.len()],
            temperature_c: config.temperatures_c[idx % config.temperatures_c.len()],
        });
    }
    points
}

fn run_corner_sweep(
    netlist: &rspice_core::Netlist,
    points: &[CornerPoint],
    config: &CornerRunConfig,
    nominal_voltage: Value,
) -> Result<Vec<(CornerPoint, SweepPointResult)>, String> {
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner analysis nominal voltage must be a positive finite value".to_string());
    }

    let mut results = Vec::with_capacity(points.len());

    for point in points {
        if !point.voltage.is_finite() || point.voltage <= 0.0 {
            return Err(format!(
                "Corner voltage must be positive and finite (got {})",
                point.voltage
            ));
        }
        if !point.temperature_c.is_finite() {
            return Err(format!(
                "Corner temperature must be finite (got {})",
                point.temperature_c
            ));
        }

        let mut corner_netlist = netlist.clone();
        apply_process_corner(&mut corner_netlist, point.process);
        apply_voltage_corner(&mut corner_netlist, point.voltage, nominal_voltage)?;

        let mut sim_config = build_engine_config(&corner_netlist, None);
        sim_config.temperature = point.temperature_c + 273.15;
        let engine = Engine::new(sim_config);

        match run_base_mode_point(&engine, &corner_netlist, &config.base_mode) {
            Ok(result) => results.push((point.clone(), result)),
            Err(e) => {
                log::warn!(
                    "Corner {} ({}) failed: {}",
                    point.label(),
                    config.base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}

fn run_base_mode_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    base_mode: &CornerBaseMode,
) -> Result<SweepPointResult, String> {
    match base_mode {
        CornerBaseMode::Op => engine
            .run_dc_op(netlist)
            .map(|dc| sweep_point_result_from_dc(&dc))
            .map_err(|e| format!("DC operating point error: {}", e)),
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            let results = engine
                .run_dc_sweep(netlist, source_name, *start, *stop, *step)
                .map_err(|e| format!("DC sweep error: {}", e))?;
            let (_, terminal) = results
                .last()
                .ok_or_else(|| "DC sweep produced no points".to_string())?;
            Ok(sweep_point_result_from_dc(terminal))
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            let result = engine
                .run_tran(netlist, *stop_time, *step_time)
                .map_err(|e| format!("Transient analysis error: {}", e))?;
            sweep_point_result_from_transient(result)
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => run_base_mode_ac_point(
            engine,
            netlist,
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
        ),
    }
}

fn sweep_point_result_from_dc(result: &CoreSimulationResult) -> SweepPointResult {
    SweepPointResult {
        node_names: result.node_names.clone(),
        node_values: result.node_voltages.clone(),
    }
}

fn sweep_point_result_from_transient(result: TransientResult) -> Result<SweepPointResult, String> {
    if result.time.is_empty() {
        return Err("Transient analysis produced no time points".to_string());
    }
    if result.node_names.is_empty() {
        return Err("Transient analysis returned no node names".to_string());
    }

    let mut node_values = Vec::with_capacity(result.node_names.len());
    for (idx, node_name) in result.node_names.iter().enumerate() {
        let Some(waveform) = result.voltages.get(idx) else {
            return Err(format!(
                "Transient result missing waveform for node '{}'",
                node_name
            ));
        };
        let Some(value) = waveform.last().copied() else {
            return Err(format!(
                "Transient waveform for node '{}' contains no samples",
                node_name
            ));
        };
        node_values.push(value);
    }

    Ok(SweepPointResult {
        node_names: result.node_names,
        node_values,
    })
}

fn run_base_mode_ac_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    start_freq: Value,
    stop_freq: Value,
    points_per_unit: usize,
    sweep: CornerFrequencySweep,
) -> Result<SweepPointResult, String> {
    let frequencies =
        generate_freq_points(start_freq, stop_freq, points_per_unit, sweep.as_keyword());
    if frequencies.is_empty() {
        return Err("Corner AC base mode generated no frequency points".to_string());
    }

    let dc_result = engine
        .run_dc_op(netlist)
        .map_err(|e| format!("DC OP error (required for AC): {}", e))?;
    let node_names = dc_result.node_names;

    let ac_results = engine
        .run_ac(netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;
    let terminal = ac_results
        .last()
        .ok_or_else(|| "AC analysis produced no points".to_string())?;

    let mut node_values = vec![0.0; node_names.len()];
    for node_idx in 1..node_names.len() {
        let ac_idx = node_idx.saturating_sub(1);
        node_values[node_idx] = terminal
            .voltages
            .get(ac_idx)
            .map(|value| value.norm())
            .unwrap_or(0.0);
    }

    Ok(SweepPointResult {
        node_names,
        node_values,
    })
}

fn apply_process_corner(netlist: &mut rspice_core::Netlist, process: CornerProcess) {
    let nmos_factor = process.nmos_factor();
    let pmos_factor = process.pmos_factor();

    for model in &mut netlist.models {
        let factor = process_factor_for_model_type(&model.model_type, nmos_factor, pmos_factor);
        if (factor - 1.0).abs() < 1e-15 {
            continue;
        }
        for (param_name, param_value) in &mut model.params {
            if is_mobility_like_model_param(param_name) {
                *param_value *= factor;
            }
        }
    }
}

fn process_factor_for_model_type(
    model_type: &str,
    nmos_factor: Value,
    pmos_factor: Value,
) -> Value {
    let ty = model_type.trim().to_ascii_uppercase();
    if ty.contains("PMOS") || ty.contains("PJF") || ty.contains("PMF") || ty.contains("PNP") {
        pmos_factor
    } else if ty.contains("NMOS") || ty.contains("NJF") || ty.contains("NMF") || ty.contains("NPN")
    {
        nmos_factor
    } else {
        (nmos_factor + pmos_factor) * 0.5
    }
}

fn is_mobility_like_model_param(param_name: &str) -> bool {
    let upper = param_name.trim().to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "KP" | "BETA" | "U0" | "UO" | "MU" | "MOBILITY" | "KP0" | "KP1"
    )
}

fn apply_voltage_corner(
    netlist: &mut rspice_core::Netlist,
    corner_voltage: Value,
    nominal_voltage: Value,
) -> Result<(), String> {
    if !corner_voltage.is_finite() || corner_voltage <= 0.0 {
        return Err("Corner voltage must be a positive finite value".to_string());
    }
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner nominal voltage must be a positive finite value".to_string());
    }
    let scale = corner_voltage / nominal_voltage;

    let mut candidate_indices = Vec::new();
    for (idx, element) in netlist.elements.iter().enumerate() {
        let Some(neg) = element.nodes.get(1) else {
            continue;
        };
        if !is_ground_node(neg) {
            continue;
        }
        if let ElementKind::VoltageSource(spec) = &element.kind {
            if dc_value_from_source(spec).is_some() {
                candidate_indices.push(idx);
            }
        }
    }

    if candidate_indices.is_empty() {
        for (idx, element) in netlist.elements.iter().enumerate() {
            if let ElementKind::VoltageSource(spec) = &element.kind {
                if dc_value_from_source(spec).is_some() {
                    candidate_indices.push(idx);
                }
            }
        }
    }

    for idx in candidate_indices {
        let Some(element) = netlist.elements.get_mut(idx) else {
            continue;
        };
        if let ElementKind::VoltageSource(spec) = &mut element.kind {
            if let Some(dc) = dc_value_from_source(spec) {
                let _ = set_dc_value_for_source(spec, dc * scale);
            }
        }
    }

    Ok(())
}

fn infer_nominal_supply_voltage(netlist: &rspice_core::Netlist) -> Option<Value> {
    let mut ground_referenced = Vec::new();
    let mut all_sources = Vec::new();

    for element in &netlist.elements {
        if let ElementKind::VoltageSource(spec) = &element.kind {
            if let Some(dc) = dc_value_from_source(spec) {
                let abs_dc = dc.abs();
                if abs_dc <= 1e-15 {
                    continue;
                }
                all_sources.push(abs_dc);
                if element
                    .nodes
                    .get(1)
                    .map(|name| is_ground_node(name))
                    .unwrap_or(false)
                {
                    ground_referenced.push(abs_dc);
                }
            }
        }
    }

    if !ground_referenced.is_empty() {
        return ground_referenced
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }
    all_sources
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

fn is_ground_node(node: &str) -> bool {
    let n = node.trim();
    n == "0" || n.eq_ignore_ascii_case("gnd") || n.eq_ignore_ascii_case("ground")
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

fn set_dc_value_for_source(spec: &mut SourceSpec, value: Value) -> bool {
    match spec {
        SourceSpec::Dc(v) => {
            *v = value;
            true
        }
        SourceSpec::DcAc { dc_value, .. } => {
            *dc_value = value;
            true
        }
        _ => false,
    }
}

fn run_temperature_sweep(
    netlist: &rspice_core::Netlist,
    temperatures_c: &[Value],
    base_mode: &CornerBaseMode,
) -> Result<Vec<(Value, SweepPointResult)>, String> {
    let mut results = Vec::with_capacity(temperatures_c.len());

    for &temp_c in temperatures_c {
        if !temp_c.is_finite() {
            return Err("Temperature sweep contains non-finite value".to_string());
        }

        let mut config = build_engine_config(netlist, None);
        config.temperature = temp_c + 273.15;
        let engine = Engine::new(config);

        match run_base_mode_point(&engine, netlist, base_mode) {
            Ok(point_result) => results.push((temp_c, point_result)),
            Err(e) => {
                log::warn!(
                    "Temperature corner {}C ({}) failed: {}",
                    temp_c,
                    base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}

fn expand_step_sweep_values(sweep: &StepSweep) -> Result<Vec<Value>, String> {
    const MAX_SWEEP_POINTS: usize = 1_000_000;

    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err("Parametric linear sweep requires finite start/stop/step".to_string());
            }
            if *step == 0.0 {
                return Err("Parametric linear sweep step cannot be zero".to_string());
            }
            if (stop - start).signum() != step.signum() && (stop - start).abs() > 0.0 {
                return Err(
                    "Parametric linear sweep step direction must match start/stop".to_string(),
                );
            }

            if (stop - start).abs() == 0.0 {
                return Ok(vec![*start]);
            }

            let mut values = Vec::new();
            let mut current = *start;
            let tolerance = (step.abs() * 1e-12).max((start.abs().max(stop.abs())) * 1e-12);

            if *step > 0.0 {
                while current <= *stop + tolerance {
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(
                            "Parametric sweep exceeds maximum supported point count".to_string()
                        );
                    }
                    current += *step;
                }
            } else {
                while current >= *stop - tolerance {
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(
                            "Parametric sweep exceeds maximum supported point count".to_string()
                        );
                    }
                    current += *step;
                }
            }

            if values.is_empty() {
                return Err("Parametric linear sweep produced no points".to_string());
            }

            Ok(values)
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            if *points_per_decade == 0 {
                return Err("Parametric decade sweep points_per_decade must be > 0".to_string());
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(
                    "Parametric decade sweep requires positive finite start/stop".to_string(),
                );
            }
            let start_log = start.log10();
            let stop_log = stop.log10();
            let span = (stop_log - start_log).abs();
            let total_points = (span * (*points_per_decade as f64)).ceil() as usize + 1;
            if total_points > MAX_SWEEP_POINTS {
                return Err("Parametric sweep exceeds maximum supported point count".to_string());
            }
            let denom = (total_points - 1).max(1) as f64;
            Ok((0..total_points)
                .map(|i| {
                    let t = i as f64 / denom;
                    let log_value = start_log + (stop_log - start_log) * t;
                    10.0_f64.powf(log_value)
                })
                .collect())
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            if *points_per_octave == 0 {
                return Err("Parametric octave sweep points_per_octave must be > 0".to_string());
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(
                    "Parametric octave sweep requires positive finite start/stop".to_string(),
                );
            }
            let start_log = start.log2();
            let stop_log = stop.log2();
            let span = (stop_log - start_log).abs();
            let total_points = (span * (*points_per_octave as f64)).ceil() as usize + 1;
            if total_points > MAX_SWEEP_POINTS {
                return Err("Parametric sweep exceeds maximum supported point count".to_string());
            }
            let denom = (total_points - 1).max(1) as f64;
            Ok((0..total_points)
                .map(|i| {
                    let t = i as f64 / denom;
                    let log_value = start_log + (stop_log - start_log) * t;
                    2.0_f64.powf(log_value)
                })
                .collect())
        }
        StepSweep::List(values) => {
            if values.is_empty() {
                return Err("Parametric LIST sweep requires at least one value".to_string());
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err("Parametric LIST sweep requires finite values".to_string());
            }
            Ok(values.clone())
        }
    }
}

fn extract_temp_points(netlist: &rspice_core::Netlist) -> Vec<Value> {
    let mut temperatures: Vec<Value> = Vec::new();
    for analysis in &netlist.analyses {
        if let AnalysisCommand::Temp {
            temperatures: temps,
        } = analysis
        {
            for &temp in temps {
                if !temperatures
                    .iter()
                    .any(|existing| (*existing - temp).abs() < 1e-15)
                {
                    temperatures.push(temp);
                }
            }
        }
    }
    temperatures
}

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
        assert_eq!(
            crate::output_spec::parse_output_node("V(OUT)", &names),
            Some(2)
        );
        assert_eq!(
            crate::output_spec::parse_output_node("out", &names),
            Some(2)
        );
        assert_eq!(crate::output_spec::parse_output_node("2", &names), Some(2));
        assert_eq!(
            crate::output_spec::parse_output_node("V(OUT,IN)", &names),
            None
        );
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
        assert_eq!(
            crate::output_spec::parse_output_voltage_spec("I(R1)", &names),
            None
        );
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
        assert!(
            result
                .sensitivities
                .iter()
                .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
        );
    }

    #[test]
    fn test_run_sensitivity_analysis_filters_internal_side_channel_parameters() {
        let netlist = "* sens params\n.param RVAL=1k\n.param IC_START=0.1\n.param NODESET_OUT=0.2\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
            .expect("sensitivity run should succeed");

        assert!(
            result
                .sensitivities
                .iter()
                .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
        );
        assert!(
            result
                .sensitivities
                .iter()
                .all(|(name, _, _)| !name.starts_with("IC_") && !name.starts_with("NODESET_"))
        );
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
        assert!(
            result
                .sensitivities
                .iter()
                .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite())
        );
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
        assert!(
            result
                .sensitivities
                .iter()
                .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite())
        );
    }

    #[test]
    fn test_run_sensitivity_analysis_current_output_handles_multiple_parameters() {
        let netlist =
            "* sens i2\n.param RA=1k\n.param RB=2k\nV1 in 0 1\nR1 in mid {RA}\nR2 mid 0 {RB}\n";

        let result = run_sensitivity_analysis(netlist, "I(V1)", false, None)
            .expect("multi-parameter current-output sensitivity should succeed");
        let ra = result
            .sensitivities
            .iter()
            .find(|(name, _, _)| name.eq_ignore_ascii_case("RA"))
            .expect("expected RA sensitivity");
        let rb = result
            .sensitivities
            .iter()
            .find(|(name, _, _)| name.eq_ignore_ascii_case("RB"))
            .expect("expected RB sensitivity");

        assert!(ra.1.is_finite() && ra.2.is_finite());
        assert!(rb.1.is_finite() && rb.2.is_finite());
        assert!((ra.1 - rb.1).abs() < 1e-12);
    }

    #[test]
    fn test_run_sensitivity_analysis_normalized_reports_zero_when_nominal_is_near_zero() {
        let netlist = "* sens tiny\n.param RVAL=1k\nV1 in 0 1e-16\nR1 in out {RVAL}\nR2 out 0 1k\n";

        let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
            .expect("sensitivity run should succeed");
        let (_name, raw, normalized) = result
            .sensitivities
            .iter()
            .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
            .expect("expected RVAL sensitivity");
        assert!(raw.is_finite());
        assert_eq!(*normalized, 0.0);
    }

    #[test]
    fn test_expand_step_sweep_values_linear_descending() {
        let values = expand_step_sweep_values(&StepSweep::Linear {
            start: 5.0,
            stop: 1.0,
            step: -2.0,
        })
        .expect("descending linear sweep should expand");
        assert_eq!(values, vec![5.0, 3.0, 1.0]);
    }

    #[test]
    fn test_expand_step_sweep_values_rejects_wrong_direction() {
        let err = expand_step_sweep_values(&StepSweep::Linear {
            start: 1.0,
            stop: 5.0,
            step: -1.0,
        })
        .expect_err("mismatched direction should fail");
        assert!(err.contains("direction"));
    }

    #[test]
    fn test_run_monte_carlo_analysis_executes_mc_command() {
        let netlist = "* mc\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 6 SEED 3 DIST GAUSS SPREAD 0.01 PARAMS RVAL\n.end\n";
        let result =
            run_monte_carlo_analysis(netlist).expect("Monte Carlo analysis should execute");
        assert_eq!(result.runs_requested, 6);
        assert!(result.runs_completed > 0);
        assert!(!result.variables.is_empty());
        assert!(result.variables.iter().all(|var| var.mean.is_finite()));
    }

    #[test]
    fn test_run_monte_carlo_analysis_requires_command() {
        let err = run_monte_carlo_analysis("* no mc\nV1 in 0 1\nR1 in 0 1k\n")
            .expect_err("missing .MC command should fail");
        assert!(err.contains(".MC command"));
    }

    #[test]
    fn test_run_parametric_analysis_executes_step_param_command() {
        let netlist = "* step param\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.STEP PARAM RVAL 1k 4k 1k\n.end\n";
        let result =
            run_parametric_analysis(netlist).expect("parametric .STEP PARAM should execute");
        assert_eq!(result.target, "PARAM RVAL");
        assert_eq!(result.sweep_values.len(), 4);
        assert_eq!(result.num_points, 4);
        assert!(
            result
                .voltages
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        );
    }

    #[test]
    fn test_run_parametric_analysis_executes_step_temp_command() {
        let netlist =
            "* step temp\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.STEP TEMP LIST -40 27 125\n.end\n";
        let result =
            run_parametric_analysis(netlist).expect("parametric .STEP TEMP should execute");
        assert_eq!(result.target, "TEMP");
        assert_eq!(result.sweep_values, vec![-40.0, 27.0, 125.0]);
        assert_eq!(result.num_points, 3);
    }

    #[test]
    fn test_run_parametric_analysis_with_config_dc_base_mode() {
        let netlist = "* step temp dc\nVDD in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
        let cfg = TempRunConfig {
            temperatures_c: vec![-40.0, 25.0, 125.0],
            base_mode: CornerBaseMode::DcSweep {
                source_name: "VDD".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.25,
            },
        };

        let result = run_parametric_analysis_with_config(netlist, &cfg)
            .expect("temperature sweep DC base mode should execute");
        assert_eq!(result.target, "TEMP");
        assert_eq!(result.sweep_values, cfg.temperatures_c);
        assert_eq!(result.num_points, 3);
        let trace = result
            .voltages
            .iter()
            .find(|(_, values)| values.len() == 3 && values.iter().all(|value| value.is_finite()))
            .expect("expected finite temperature trace");
        assert!(trace.0.starts_with("V("));
    }

    #[test]
    fn test_run_parametric_analysis_with_config_transient_base_mode() {
        let netlist = "* step temp tran\nVDD vdd 0 1.0\nR1 vdd out 1k\nC1 out 0 1n\n.end\n";
        let cfg = TempRunConfig {
            temperatures_c: vec![-40.0, 25.0, 125.0],
            base_mode: CornerBaseMode::Transient {
                stop_time: 2e-6,
                step_time: 2e-8,
            },
        };

        let result = run_parametric_analysis_with_config(netlist, &cfg)
            .expect("temperature sweep transient base mode should execute");
        assert_eq!(result.target, "TEMP");
        assert_eq!(result.num_points, 3);
        let out = result
            .voltages
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("expected V(out) waveform");
        assert_eq!(out.1.len(), 3);
        assert!(out.1.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn test_run_parametric_analysis_with_config_ac_base_mode() {
        let netlist = "* step temp ac\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = TempRunConfig {
            temperatures_c: vec![-40.0, 25.0, 125.0],
            base_mode: CornerBaseMode::Ac {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: CornerFrequencySweep::Decade,
            },
        };

        let result = run_parametric_analysis_with_config(netlist, &cfg)
            .expect("temperature sweep AC base mode should execute");
        assert_eq!(result.target, "TEMP");
        assert_eq!(result.num_points, 3);
        assert!(
            result
                .voltages
                .iter()
                .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
                    && values.len() == 3
                    && values.iter().all(|v| v.is_finite() && *v >= 0.0))
        );
    }

    #[test]
    fn test_run_parametric_analysis_requires_step_command() {
        let err = run_parametric_analysis("* no step\nV1 in 0 1\nR1 in 0 1k\n")
            .expect_err("missing .STEP command should fail");
        assert!(err.contains(".STEP command"));
    }

    #[test]
    fn test_run_corner_analysis_executes_temp_directives() {
        let netlist = "* corners\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.TEMP -40 27 125\n.end\n";
        let result = run_corner_analysis(netlist).expect("corner analysis should execute");
        assert_eq!(result.x_label, "Temperature");
        assert_eq!(result.x_unit, "C");
        assert_eq!(result.x_values, vec![-40.0, 27.0, 125.0]);
        assert_eq!(result.temperatures_c, vec![-40.0, 27.0, 125.0]);
        assert_eq!(result.corner_labels.len(), 3);
        assert!(result.corner_labels[0].starts_with("TT_1.000000V_"));
        assert_eq!(result.num_points, 3);
        assert!(
            result
                .voltages
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        );
    }

    #[test]
    fn test_run_corner_analysis_requires_temp_command() {
        let err = run_corner_analysis("* no corners\nV1 in 0 1\nR1 in 0 1k\n")
            .expect_err("missing .TEMP command should fail");
        assert!(err.contains(".TEMP"));
    }

    #[test]
    fn test_run_corner_analysis_with_config_executes_full_matrix() {
        let netlist = "* corners cfg\nVDD vdd 0 1.0\nR1 vdd out 1k\nR2 out 0 1k\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::FF],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
        };

        let result = run_corner_analysis_with_config(netlist, &cfg)
            .expect("corner analysis with explicit config should execute");
        assert_eq!(result.num_points, 8);
        assert_eq!(result.x_label, "Corner Index");
        assert_eq!(result.x_unit, "");
        assert_eq!(
            result.x_values,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]
        );
        assert_eq!(result.temperatures_c.len(), 8);
        assert_eq!(result.corner_labels.len(), 8);
        assert_eq!(result.num_failures, 0);
        assert!(
            result
                .corner_labels
                .iter()
                .any(|label| label.contains("FF_1.100000V_125.000000C"))
        );
    }

    #[test]
    fn test_run_corner_analysis_with_config_executes_diagonal_mode() {
        let netlist = "* corners diag\nVDD vdd 0 1.0\nR1 vdd out 1k\nR2 out 0 1k\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::SS, CornerProcess::TT, CornerProcess::FF],
            voltages: vec![0.95, 1.0],
            temperatures_c: vec![25.0],
            full_matrix: false,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
        };

        let result = run_corner_analysis_with_config(netlist, &cfg)
            .expect("diagonal corner analysis should execute");
        assert_eq!(result.num_points, 3);
        assert_eq!(result.corner_labels.len(), 3);
        assert!(result.corner_labels[0].starts_with("SS_0.950000V_25.000000C"));
        assert!(result.corner_labels[1].starts_with("TT_1.000000V_25.000000C"));
        assert!(result.corner_labels[2].starts_with("FF_0.950000V_25.000000C"));
    }

    #[test]
    fn test_run_corner_analysis_with_config_rejects_invalid_voltage() {
        let netlist = "* corners invalid\nV1 in 0 1\nR1 in 0 1k\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.0],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
        };
        let err = run_corner_analysis_with_config(netlist, &cfg)
            .expect_err("invalid voltage corner must be rejected");
        assert!(err.contains("voltage corners"));
    }

    #[test]
    fn test_run_corner_analysis_with_config_transient_base_mode() {
        let netlist = "* corners tran\nVDD vdd 0 1.0\nR1 vdd out 1k\nC1 out 0 1n\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Transient {
                stop_time: 2e-6,
                step_time: 2e-8,
            },
        };

        let result = run_corner_analysis_with_config(netlist, &cfg)
            .expect("corner transient base mode should execute");
        assert_eq!(result.num_points, 2);
        let out = result
            .voltages
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("expected V(out) waveform");
        assert_eq!(out.1.len(), 2);
        assert!(out.1.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn test_run_corner_analysis_with_config_ac_base_mode() {
        let netlist = "* corners ac\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![-40.0, 25.0, 125.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Ac {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 10,
                sweep: CornerFrequencySweep::Decade,
            },
        };

        let result =
            run_corner_analysis_with_config(netlist, &cfg).expect("corner AC base mode should run");
        assert_eq!(result.num_points, 3);
        assert_eq!(result.x_label, "Temperature");
        assert_eq!(result.x_unit, "C");
        assert_eq!(result.x_values, vec![-40.0, 25.0, 125.0]);
        assert!(
            result
                .voltages
                .iter()
                .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
                    && values.len() == 3
                    && values.iter().all(|v| v.is_finite() && *v >= 0.0))
        );
    }

    #[test]
    fn test_run_corner_analysis_with_config_dc_sweep_base_mode() {
        let netlist = "* corners dc\nVDD in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::FF],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::DcSweep {
                source_name: "VDD".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.2,
            },
        };

        let result = run_corner_analysis_with_config(netlist, &cfg)
            .expect("corner DC sweep base mode should execute");
        assert_eq!(result.num_points, 2);
        assert_eq!(result.x_label, "Corner Index");
        assert_eq!(result.x_unit, "");
        assert_eq!(result.x_values, vec![0.0, 1.0]);
        let trace = result
            .voltages
            .iter()
            .find(|(_, values)| values.len() == 2 && values.iter().all(|value| value.is_finite()))
            .expect("expected at least one finite corner trace");
        assert!(trace.0.starts_with("V("));
    }

    #[test]
    fn test_run_corner_analysis_with_config_rejects_invalid_dc_base_mode_step() {
        let netlist = "* corners invalid dc\nV1 in 0 1\nR1 in 0 1k\n.end\n";
        let cfg = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::DcSweep {
                source_name: "V1".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.0,
            },
        };
        let err = run_corner_analysis_with_config(netlist, &cfg)
            .expect_err("invalid corner DC step must be rejected");
        assert!(err.contains("step cannot be zero"));
    }

    #[test]
    fn test_extract_temp_points_deduplicates_values() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* dedupe\nV1 in 0 1\nR1 in 0 1k\n.TEMP -40 27\n.TEMP 27 125\n.end\n",
        )
        .expect("netlist should parse");

        let temps = extract_temp_points(&netlist);
        assert_eq!(temps, vec![-40.0, 27.0, 125.0]);
    }
}
