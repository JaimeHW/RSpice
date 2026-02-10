//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, ac_output_value, parse_output_spec, resolve_node_or_ground_index,
};
#[cfg(test)]
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{AnalysisCommand, Element, ElementKind, StepSweep};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use rspice_core::{SimulationConfigOverrides, Value, resolve_simulation_config};

#[path = "simulation_runner/harmonic_basis.rs"]
mod harmonic_basis;
use harmonic_basis::{build_disto_two_tone_harmonic_plan, build_multi_tone_hb_layout};

#[path = "simulation_runner/ac.rs"]
mod ac;
#[path = "simulation_runner/dc_sweep.rs"]
mod dc_sweep;
#[path = "simulation_runner/disto.rs"]
mod disto;
#[path = "simulation_runner/hb.rs"]
mod hb;
#[path = "simulation_runner/monte_carlo.rs"]
mod monte_carlo;
#[path = "simulation_runner/noise.rs"]
mod noise;
#[path = "simulation_runner/optimization.rs"]
mod optimization;
#[path = "simulation_runner/pac_pxf.rs"]
mod pac_pxf;
#[path = "simulation_runner/pnoise.rs"]
mod pnoise;
#[path = "simulation_runner/pnoise_sideband.rs"]
mod pnoise_sideband;
#[path = "simulation_runner/pole_zero.rs"]
mod pole_zero;
#[path = "simulation_runner/pss.rs"]
mod pss;
#[path = "simulation_runner/pstb.rs"]
mod pstb;
#[path = "simulation_runner/reliability.rs"]
mod reliability;
#[path = "simulation_runner/sensitivity.rs"]
mod sensitivity;
#[path = "simulation_runner/soa.rs"]
mod soa;
#[path = "simulation_runner/sparameter.rs"]
mod sparameter;
#[path = "simulation_runner/stb.rs"]
mod stb;
#[path = "simulation_runner/sweeps.rs"]
mod sweeps;
#[path = "simulation_runner/tf.rs"]
mod tf;
pub use ac::{AcData, run_ac_analysis};
pub use dc_sweep::{DcSweepData, run_dc_sweep};
#[cfg(test)]
use disto::interpolate_magnitude_at_for_tests;
pub use disto::{DistoData, DistoFrequencySweep, DistoRunConfig, DistoTrace, run_disto_analysis};
pub use hb::{HbData, HbRunConfig, HbToneRunConfig, run_hb_analysis};
pub use monte_carlo::{MonteCarloData, MonteCarloVariableData, run_monte_carlo_analysis};
pub use noise::{NoiseData, run_noise_analysis};
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationData, OptimizationGoalMode, OptimizationRunConfig,
    OptimizationVariable, run_optimization_analysis, run_optimization_analysis_with_config,
};
pub use pac_pxf::{
    PacData, PacFrequencySweep, PacRunConfig, PxfData, PxfFrequencySweep, PxfRunConfig,
    run_pac_analysis, run_pac_analysis_auto, run_pxf_analysis, run_pxf_analysis_with_config,
};
pub use pnoise::{
    PnoiseData, PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig, run_pnoise_analysis,
    run_pnoise_analysis_with_config,
};
#[cfg(test)]
use pnoise_sideband::{build_pnoise_sideband_translated_frequencies, fold_sideband_samples};
pub use pole_zero::{PoleZeroData, run_pole_zero_analysis};
pub use pss::{PssData, run_pss_analysis};
pub use pstb::{PstbData, PstbRunConfig, run_pstb_analysis, run_pstb_analysis_with_config};
pub use reliability::{
    ReliabilityData, ReliabilityRunConfig, run_reliability_analysis,
    run_reliability_analysis_with_config,
};
pub use sensitivity::{SensitivityData, run_sensitivity_analysis};
pub use soa::{SoaData, SoaRunConfig, run_soa_analysis, run_soa_analysis_with_config};
pub use sparameter::{
    SParameterData, SParameterPort, SParameterRunConfig, SParameterSweep, run_sparameter_analysis,
};
pub use stb::{StbData, run_stb_analysis};
pub use sweeps::{
    CornerBaseMode, CornerData, CornerFrequencySweep, CornerProcess, CornerRunConfig,
    ParametricData, TempRunConfig, run_corner_analysis, run_corner_analysis_with_config,
    run_parametric_analysis, run_parametric_analysis_with_config,
};
pub use tf::{TfData, TfFrequencySweep, TfRunConfig, run_tf_analysis, run_tf_analysis_with_config};

#[cfg(test)]
fn inject_param_overrides(
    netlist_text: &str,
    vars: &std::collections::HashMap<String, Value>,
) -> String {
    optimization::inject_param_overrides_for_tests(netlist_text, vars)
}

#[cfg(test)]
fn format_param_override_value(value: Value) -> String {
    optimization::format_param_override_value_for_tests(value)
}

#[cfg(test)]
fn extract_reliability_stress_data(
    elements: &[Element],
    node_voltages: &std::collections::HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
) -> std::collections::HashMap<String, StressMetrics> {
    reliability::extract_reliability_stress_data_for_tests(
        elements,
        node_voltages,
        temperature_k,
        min_stress_voltage,
    )
}

#[cfg(test)]
fn apply_reliability_mechanism_scaling(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
) {
    reliability::apply_reliability_mechanism_scaling_for_tests(results, config)
}

#[cfg(test)]
fn expand_step_sweep_values(sweep: &StepSweep) -> Result<Vec<Value>, String> {
    sweeps::expand_step_sweep_values_for_tests(sweep)
}

#[cfg(test)]
fn extract_temp_points(netlist: &rspice_core::Netlist) -> Vec<Value> {
    sweeps::extract_temp_points_for_tests(netlist)
}

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
// S-Parameter Analysis
// =============================================================================

// =============================================================================
// Envelope/Fourier Analysis
// =============================================================================

/// Configuration for envelope analysis.
#[derive(Debug, Clone)]
pub struct EnvelopeRunConfig {
    pub fundamental_freq: Value,
    pub stop_time: Value,
    pub num_harmonics: usize,
    pub max_step: Option<Value>,
}

impl EnvelopeRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Envelope fundamental frequency must be positive".to_string());
        }
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("Envelope stop_time must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Envelope num_harmonics must be > 0".to_string());
        }
        if let Some(step) = self.max_step {
            if !step.is_finite() || step <= 0.0 {
                return Err("Envelope max_step must be positive when provided".to_string());
            }
        }
        Ok(())
    }
}

/// Envelope analysis output.
#[derive(Debug, Clone)]
pub struct EnvelopeData {
    pub time: Vec<Value>,
    pub waveforms: Vec<(String, Vec<Value>)>,
}

/// Run envelope analysis by post-processing transient data with a sliding RMS demodulation.
pub fn run_envelope_analysis(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
) -> Result<EnvelopeData, String> {
    config.validate()?;

    let samples_per_cycle = (config.num_harmonics.max(1) as f64 * 16.0).max(32.0);
    let carrier_step = 1.0 / (config.fundamental_freq * samples_per_cycle);
    let coarse_step = config.stop_time / 1200.0;
    let fine_floor = config.stop_time / 100_000.0;
    let mut step_time = config
        .max_step
        .unwrap_or_else(|| carrier_step.min(coarse_step).max(fine_floor));
    step_time = step_time.clamp(fine_floor, config.stop_time);

    let transient = run_transient_analysis(netlist_text, config.stop_time, step_time)?;
    if transient.time.is_empty() {
        return Err("Envelope analysis produced no transient samples".to_string());
    }
    if transient.voltages.is_empty() {
        return Err("Envelope analysis found no non-ground node waveforms".to_string());
    }

    let cycle_window = (1.0 / config.fundamental_freq / step_time).round().max(3.0) as usize;
    let mut waveforms = Vec::with_capacity(transient.voltages.len());
    for (name, values) in transient.voltages {
        if values.is_empty() {
            continue;
        }
        let env = compute_envelope_rms(&values, cycle_window);
        waveforms.push((format!("ENV({})", name), env));
    }
    if waveforms.is_empty() {
        return Err("Envelope analysis produced no envelope traces".to_string());
    }

    Ok(EnvelopeData {
        time: transient.time,
        waveforms,
    })
}

fn compute_envelope_rms(values: &[Value], window: usize) -> Vec<Value> {
    if values.is_empty() {
        return Vec::new();
    }
    let window = window.max(3).min(values.len());
    let half = window / 2;
    let mut prefix_sq = Vec::with_capacity(values.len() + 1);
    prefix_sq.push(0.0);
    for &sample in values {
        let next = prefix_sq.last().copied().unwrap_or(0.0) + sample * sample;
        prefix_sq.push(next);
    }

    let mut envelope = Vec::with_capacity(values.len());
    for idx in 0..values.len() {
        let start = idx.saturating_sub(half);
        let end = (idx + half + 1).min(values.len());
        let denom = (end - start).max(1) as Value;
        let mean_sq = (prefix_sq[end] - prefix_sq[start]) / denom;
        envelope.push((2.0 * mean_sq).sqrt());
    }
    envelope
}

/// Configuration for Fourier analysis.
#[derive(Debug, Clone)]
pub struct FourierRunConfig {
    pub fundamental_freq: Value,
    pub num_harmonics: usize,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub start_time: Value,
    pub stop_time: Value,
}

impl FourierRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Fourier fundamental frequency must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Fourier num_harmonics must be greater than zero".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("Fourier output node must be specified".to_string());
        }
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            return Err("Fourier start_time must be >= 0".to_string());
        }
        if !self.stop_time.is_finite() || self.stop_time <= self.start_time {
            return Err("Fourier stop_time must be greater than start_time".to_string());
        }
        Ok(())
    }
}

/// Fourier analysis output.
#[derive(Debug, Clone)]
pub struct FourierData {
    pub frequencies: Vec<Value>,
    pub response: Vec<Complex64>,
    pub thd_percent: Value,
    pub dc_component: Value,
    pub output_label: String,
}

/// Run Fourier analysis by executing transient and computing harmonic decomposition.
pub fn run_fourier_analysis(
    netlist_text: &str,
    config: &FourierRunConfig,
) -> Result<FourierData, String> {
    config.validate()?;

    let window = config.stop_time - config.start_time;
    let max_harmonic_freq = config.fundamental_freq * (config.num_harmonics as f64 + 1.0);
    let nyquist_oversample = 8.0;
    let fine_step = 1.0 / (max_harmonic_freq * nyquist_oversample);
    let coarse_step = window / 1500.0;
    let floor_step = window / 200_000.0;
    let step_time = fine_step
        .min(coarse_step)
        .max(floor_step)
        .min(config.stop_time);

    let transient = run_transient_analysis(netlist_text, config.stop_time, step_time)?;
    if transient.time.len() < 3 {
        return Err("Fourier analysis requires at least 3 transient samples".to_string());
    }

    let signal = extract_transient_signal(
        &transient,
        &config.output_node,
        config.output_ref.as_deref(),
    )?;
    let mut window_time = Vec::new();
    let mut window_values = Vec::new();
    for (&time, &value) in transient.time.iter().zip(signal.iter()) {
        if time >= config.start_time && time <= config.stop_time {
            window_time.push(time);
            window_values.push(value);
        }
    }
    if window_time.len() < 3 {
        return Err("Fourier analysis window has insufficient samples".to_string());
    }

    let analysis = FourierAnalysis::new(
        FourierConfig::new(config.fundamental_freq).with_harmonics(config.num_harmonics),
    );
    let result = analysis.analyze(&window_time, &window_values);

    let frequencies: Vec<Value> = result.harmonics.iter().map(|h| h.frequency).collect();
    let response: Vec<Complex64> = result
        .harmonics
        .iter()
        .map(|h| Complex64::from_polar(h.magnitude, h.phase.to_radians()))
        .collect();
    let output_label = if let Some(ref_node) = config.output_ref.as_deref() {
        if ref_node.trim().is_empty() || ref_node.eq_ignore_ascii_case("0") {
            format!("V({})", config.output_node.trim())
        } else {
            format!("V({}, {})", config.output_node.trim(), ref_node.trim())
        }
    } else {
        format!("V({})", config.output_node.trim())
    };

    Ok(FourierData {
        frequencies,
        response,
        thd_percent: result.thd,
        dc_component: result.dc_component,
        output_label,
    })
}

fn extract_transient_signal(
    transient: &TransientData,
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<Vec<Value>, String> {
    let node = output_node.trim();
    if node.is_empty() {
        return Err("Fourier output node is empty".to_string());
    }

    let node_waveform = find_transient_waveform(transient, node)
        .ok_or_else(|| format!("Fourier output node '{}' not found in transient data", node))?;

    if let Some(ref_name) = output_ref {
        let ref_name = ref_name.trim();
        if ref_name.is_empty() || ref_name.eq_ignore_ascii_case("0") {
            return Ok(node_waveform.to_vec());
        }
        let ref_waveform = find_transient_waveform(transient, ref_name).ok_or_else(|| {
            format!(
                "Fourier output reference node '{}' not found in transient data",
                ref_name
            )
        })?;
        if ref_waveform.len() != node_waveform.len() {
            return Err("Fourier node/reference waveform length mismatch".to_string());
        }
        return Ok(node_waveform
            .iter()
            .zip(ref_waveform.iter())
            .map(|(v, r)| v - r)
            .collect());
    }

    Ok(node_waveform.to_vec())
}

fn find_transient_waveform<'a>(
    transient: &'a TransientData,
    node_name: &str,
) -> Option<&'a [Value]> {
    let target = normalize_waveform_node_name(node_name);
    transient.voltages.iter().find_map(|(name, values)| {
        let wf_node = normalize_waveform_node_name(name);
        (wf_node == target).then_some(values.as_slice())
    })
}

fn normalize_waveform_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && (trimmed.starts_with("V(") || trimmed.starts_with("v("))
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

fn build_voltage_output_expr(output_node: &str, output_ref: Option<&str>) -> String {
    let output_node = output_node.trim();
    let output_ref = output_ref
        .map(str::trim)
        .filter(|name| !name.is_empty() && !is_ground_like(name));
    match output_ref {
        Some(reference) => format!("V({},{})", output_node, reference),
        None => format!("V({})", output_node),
    }
}

fn is_ground_like(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    )
}

fn infer_primary_source_name(netlist: &rspice_core::Netlist) -> Option<String> {
    netlist
        .elements
        .iter()
        .find_map(|element| match &element.kind {
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {
                Some(element.name.clone())
            }
            _ => None,
        })
}

fn netlist_has_independent_source_named(netlist: &rspice_core::Netlist, source_name: &str) -> bool {
    netlist.elements.iter().any(|element| {
        (matches!(
            &element.kind,
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
        )) && element.name.eq_ignore_ascii_case(source_name)
    })
}

fn infer_primary_output_node(node_names: &[String]) -> Option<String> {
    node_names
        .iter()
        .rev()
        .find(|name| !is_ground_like(name))
        .cloned()
}

fn normalize_voltage_signal_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.len() > 3 && trimmed[..2].eq_ignore_ascii_case("V(") && trimmed.ends_with(')') {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
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
    fn test_interpolate_magnitude_at_log_frequency() {
        let frequencies = vec![1.0, 10.0, 100.0];
        let magnitudes = vec![1.0, 10.0, 100.0];
        let mid = interpolate_magnitude_at_for_tests(&frequencies, &magnitudes, 31.622776601683793)
            .expect("interpolation should succeed");
        assert!((mid - 31.622776601683793).abs() < 1e-9);
    }

    #[test]
    fn test_run_disto_analysis_generates_harmonic_metrics() {
        let netlist = r#"
* disto transfer-estimation test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;

        let cfg = DistoRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: DistoFrequencySweep::Decade,
            f2_over_f1: Some(1.5),
            allow_linearized_fallback: false,
        };

        let data = run_disto_analysis(netlist, &cfg).expect("DISTO should execute");
        assert!(!data.frequencies.is_empty());
        assert!(!data.traces.is_empty());
        assert!(
            data.warnings.is_empty(),
            "nonlinear HB DISTO path should not emit fallback warnings"
        );

        let trace = &data.traces[0];
        assert_eq!(trace.fundamental_gain_db.len(), data.frequencies.len());
        assert_eq!(trace.hd2_db.len(), data.frequencies.len());
        assert_eq!(trace.hd3_db.len(), data.frequencies.len());
        assert_eq!(trace.thd_percent.len(), data.frequencies.len());
        assert!(trace.imd2_db.is_some());
        assert!(trace.imd3_db.is_some());
        assert_eq!(
            trace.imd2_db.as_ref().expect("imd2 should exist").len(),
            data.frequencies.len()
        );
        assert_eq!(
            trace.imd3_db.as_ref().expect("imd3 should exist").len(),
            data.frequencies.len()
        );
    }

    #[test]
    fn test_run_disto_analysis_rejects_invalid_f2_ratio() {
        let cfg = DistoRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: DistoFrequencySweep::Decade,
            f2_over_f1: Some(1.0),
            allow_linearized_fallback: false,
        };
        let err = run_disto_analysis("* invalid\nV1 in 0 AC 1\nR1 in 0 1k\n.end\n", &cfg)
            .expect_err("f2 ratio <= 1 should fail validation");
        assert!(err.contains("f2_over_f1"));
    }

    #[test]
    fn test_build_disto_two_tone_harmonic_plan_rational_ratio() {
        let plan = build_disto_two_tone_harmonic_plan(1.5).expect("1.5 should map to 3/2");
        assert_eq!(plan.tone1_harmonic, 2);
        assert_eq!(plan.tone2_harmonic, 3);
        assert!(plan.max_harmonic >= 9);
    }

    #[test]
    fn test_build_multi_tone_hb_layout_reports_harmonic_mapping() {
        let tones = vec![HbToneRunConfig::new(2e6, 2), HbToneRunConfig::new(3e6, 1)];
        let layout = build_multi_tone_hb_layout(&tones, 3)
            .expect("2 MHz and 3 MHz should map to a commensurate 1 MHz basis");
        assert!((layout.base_frequency - 1e6).abs() < 1e-9);
        assert_eq!(layout.tone_harmonics, vec![2, 3]);
        assert!(layout.max_harmonic >= 9);
    }

    #[test]
    fn test_build_disto_two_tone_plan_uses_shared_hb_layout_mapping() {
        let plan = build_disto_two_tone_harmonic_plan(2.5).expect("2.5 should map to 5/2");
        assert_eq!(plan.tone1_harmonic, 2);
        assert_eq!(plan.tone2_harmonic, 5);
        assert!(plan.max_harmonic >= 15);
    }

    #[test]
    fn test_build_disto_two_tone_harmonic_plan_rejects_unstable_ratio() {
        let err = build_disto_two_tone_harmonic_plan(2f64.sqrt())
            .expect_err("irrational ratio should not map to low-order harmonic basis");
        assert!(err.contains("low-order rational"));
    }

    #[test]
    fn test_run_disto_analysis_fallbacks_for_unstable_two_tone_ratio() {
        let netlist = r#"
* disto fallback ratio test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;
        let cfg = DistoRunConfig {
            start_freq: 1e3,
            stop_freq: 1e5,
            points_per_unit: 5,
            sweep: DistoFrequencySweep::Decade,
            f2_over_f1: Some(2f64.sqrt()),
            allow_linearized_fallback: true,
        };

        let data = run_disto_analysis(netlist, &cfg).expect("DISTO should fallback when needed");
        assert!(
            data.warnings
                .iter()
                .any(|warning| warning.contains("linearized transfer-based fallback")),
            "expected explicit nonlinear fallback warning"
        );
        assert!(
            data.traces
                .iter()
                .all(|trace| trace.imd2_db.is_some() && trace.imd3_db.is_some()),
            "linearized fallback should still produce IMD traces"
        );
    }

    #[test]
    fn test_run_disto_analysis_unstable_two_tone_ratio_errors_without_fallback() {
        let netlist = r#"
* disto strict ratio test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;
        let cfg = DistoRunConfig {
            start_freq: 1e3,
            stop_freq: 1e5,
            points_per_unit: 5,
            sweep: DistoFrequencySweep::Decade,
            f2_over_f1: Some(2f64.sqrt()),
            allow_linearized_fallback: false,
        };

        let err = run_disto_analysis(netlist, &cfg)
            .expect_err("DISTO should fail in strict mode when HB ratio mapping is unstable");
        assert!(err.contains("allow_linearized_fallback=true"));
        assert!(err.contains("nonlinear HB path failed"));
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
    fn test_run_monte_carlo_analysis_supports_worst_case_distribution() {
        let netlist = "* mc worst\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 8 DIST WORSTCASE SPREAD 0.03 PARAMS RVAL\n.end\n";
        let result = run_monte_carlo_analysis(netlist)
            .expect("Monte Carlo WORSTCASE analysis should execute");
        assert_eq!(result.runs_requested, 8);
        assert!(result.runs_completed > 0);
        assert!(!result.variables.is_empty());
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

    #[test]
    fn test_run_pss_analysis_executes_for_driven_rc() {
        let netlist = "* pss\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let result = run_pss_analysis(netlist, 1e6, 8, 1e-4)
            .expect("PSS analysis should execute for driven RC");
        assert!(result.period > 0.0);
        assert!(result.frequency > 0.0);
        assert!(!result.time.is_empty());
        assert!(!result.waveforms.is_empty());
        assert!(
            result
                .harmonics
                .iter()
                .any(|(_, harmonics)| !harmonics.is_empty()),
            "expected harmonic content for at least one waveform"
        );
    }

    #[test]
    fn test_run_hb_analysis_executes_for_driven_rc() {
        let netlist = "* hb\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let hb_cfg = HbRunConfig {
            tones: vec![HbToneRunConfig::new(1e6, 5)],
            ..HbRunConfig::default()
        };
        let result =
            run_hb_analysis(netlist, &hb_cfg).expect("HB analysis should execute for driven RC");
        assert_eq!(result.fundamentals, vec![1e6]);
        assert_eq!(result.harmonics_per_tone, vec![5]);
        assert!(result.converged);
        assert!(result.num_components >= 2);
        assert!(!result.dc_voltages.is_empty());
        assert!(
            result
                .spectra
                .iter()
                .any(|(_, spectrum)| !spectrum.is_empty()),
            "expected at least one non-empty HB spectrum"
        );
    }

    #[test]
    fn test_run_hb_analysis_executes_with_two_tone_layout() {
        let netlist = "* hb two-tone\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let hb_cfg = HbRunConfig {
            tones: vec![HbToneRunConfig::new(2e6, 4), HbToneRunConfig::new(3e6, 3)],
            ..HbRunConfig::default()
        };
        let result = run_hb_analysis(netlist, &hb_cfg)
            .expect("HB two-tone analysis should execute for commensurate tones");
        assert_eq!(result.fundamentals, vec![2e6, 3e6]);
        assert_eq!(result.harmonics_per_tone, vec![4, 3]);
        assert!(result.converged);
        assert!(result.num_components >= 10);
        let first_spectrum = result
            .spectra
            .first()
            .expect("expected at least one HB spectrum");
        assert!(
            first_spectrum
                .1
                .iter()
                .any(|(freq, _, _)| (*freq - 1e6).abs() < 1e-6),
            "two-tone HB should use the derived 1 MHz basis frequency"
        );
    }

    #[test]
    fn test_run_hb_analysis_executes_with_three_tone_layout() {
        let netlist = "* hb three-tone\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let hb_cfg = HbRunConfig {
            tones: vec![
                HbToneRunConfig::new(2e6, 2),
                HbToneRunConfig::new(3e6, 2),
                HbToneRunConfig::new(5e6, 1),
            ],
            max_mixing_order: 4,
            ..HbRunConfig::default()
        };
        let result = run_hb_analysis(netlist, &hb_cfg)
            .expect("HB three-tone analysis should execute for commensurate tones");
        assert_eq!(result.fundamentals, vec![2e6, 3e6, 5e6]);
        assert_eq!(result.harmonics_per_tone, vec![2, 2, 1]);
        assert!(result.converged);
        assert!(
            result.num_components >= 21,
            "max_mixing_order should increase harmonic budget for three-tone solve"
        );
        let first_spectrum = result
            .spectra
            .first()
            .expect("expected at least one HB spectrum");
        assert!(
            first_spectrum
                .1
                .iter()
                .any(|(freq, _, _)| (*freq - 1e6).abs() < 1e-6),
            "three-tone HB should use the derived 1 MHz basis frequency"
        );
    }

    #[test]
    fn test_run_hb_analysis_routes_source_filtered_tones() {
        let netlist = "* hb source-routed\nVRF 1 0 DC 0 AC 1\nVLO 2 0 DC 0 AC 1\nR1 1 0 1k\nR2 2 0 1k\nC1 1 0 1n\nC2 2 0 1n\n.end\n";
        let hb_cfg = HbRunConfig {
            tones: vec![
                HbToneRunConfig::new(2e6, 1)
                    .with_name("rf")
                    .with_source("VRF"),
                HbToneRunConfig::new(3e6, 1)
                    .with_name("lo")
                    .with_source("VLO"),
            ],
            ..HbRunConfig::default()
        };

        let data = run_hb_analysis(netlist, &hb_cfg)
            .expect("HB should route source-filtered tones to matching sources");
        assert!(data.converged);

        let magnitude_at = |spectrum: &[(Value, Value, Value)], target_freq: Value| -> Value {
            spectrum
                .iter()
                .find(|(freq, _, _)| (*freq - target_freq).abs() < 1e-6)
                .map(|(_, magnitude, _)| *magnitude)
                .unwrap_or(0.0)
        };

        let vrf = data
            .spectra
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(1)"))
            .expect("expected V(1) spectrum")
            .1
            .as_slice();
        let vlo = data
            .spectra
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(2)"))
            .expect("expected V(2) spectrum")
            .1
            .as_slice();

        assert!(
            magnitude_at(vrf, 2e6) > 0.9,
            "VRF should be driven at 2 MHz"
        );
        assert!(
            magnitude_at(vrf, 3e6) < 1e-9,
            "VRF should not be driven at 3 MHz"
        );
        assert!(
            magnitude_at(vlo, 3e6) > 0.9,
            "VLO should be driven at 3 MHz"
        );
        assert!(
            magnitude_at(vlo, 2e6) < 1e-9,
            "VLO should not be driven at 2 MHz"
        );
    }

    #[test]
    fn test_run_hb_analysis_rejects_invalid_runtime_controls() {
        let netlist = "* hb invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let hb_cfg = HbRunConfig {
            tones: vec![HbToneRunConfig::new(1e6, 5)],
            reltol: 0.0,
            ..HbRunConfig::default()
        };
        let err = run_hb_analysis(netlist, &hb_cfg)
            .expect_err("invalid HB runtime controls should be rejected");
        assert!(err.contains("reltol"));
    }

    #[test]
    fn test_run_pac_analysis_executes_for_driven_rc() {
        let netlist = "* pac\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PacRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: PacFrequencySweep::Decade,
            max_sideband: 2,
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: None,
            pac_magnitude: 0.5,
            include_dc: true,
            reltol: 1e-3,
            abstol: 1e-12,
        };

        let result = run_pac_analysis(netlist, &cfg).expect("PAC analysis should execute");
        assert!(result.converged);
        assert!(!result.frequencies.is_empty());
        assert!(result.sidebands.contains(&0));
        assert_eq!(result.sidebands, vec![-2, -1, 0, 1, 2]);
        assert_eq!(result.spectra.len(), result.sidebands.len());
        assert!(
            result.spectra.iter().all(|(_, spectrum)| {
                spectrum.len() == result.frequencies.len()
                    && spectrum.iter().all(|(f, mag, phase)| {
                        f.is_finite() && mag.is_finite() && phase.is_finite()
                    })
            }),
            "expected finite PAC spectra at all sweep points"
        );
    }

    #[test]
    fn test_run_pac_analysis_rejects_empty_sideband_configuration() {
        let netlist = "* pac invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PacRunConfig {
            max_sideband: 0,
            include_dc: false,
            output_node: "out".to_string(),
            input_source: "V1".to_string(),
            ..PacRunConfig::default()
        };

        let err = run_pac_analysis(netlist, &cfg)
            .expect_err("PAC without any enabled sidebands should be rejected");
        assert!(err.contains("at least one sideband"));
    }

    #[test]
    fn test_run_pac_analysis_auto_infers_source_and_output() {
        let netlist = "* pac auto\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let result =
            run_pac_analysis_auto(netlist).expect("PAC auto mode should infer IO for simple RC");
        assert!(result.converged);
        assert!(!result.frequencies.is_empty());
        assert!(!result.spectra.is_empty());
    }

    #[test]
    fn test_run_pxf_analysis_with_config_executes_for_driven_rc() {
        let netlist = "* pxf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PxfRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: PxfFrequencySweep::Decade,
            input_source: "V1".to_string(),
            input_sideband: 1,
            output_node: "out".to_string(),
            output_ref: None,
            output_sideband: 1,
            max_sideband: 3,
            reltol: 1e-3,
            abstol: 1e-12,
        };

        let result =
            run_pxf_analysis_with_config(netlist, &cfg).expect("PXF analysis should execute");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.frequencies.len(), result.transfer.len());
        assert_eq!(result.frequencies.len(), result.magnitude_db.len());
        assert_eq!(result.frequencies.len(), result.phase_deg.len());
        assert_eq!(result.frequencies.len(), result.output_frequencies.len());
        assert_eq!(result.input_sideband, 1);
        assert_eq!(result.output_sideband, 1);
        assert!(result.output_label.starts_with("V("));
        assert!(
            result
                .transfer
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
        assert!(result.magnitude_db.iter().all(|value| value.is_finite()));
        assert!(result.phase_deg.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn test_run_pxf_analysis_with_config_rejects_sideband_out_of_range() {
        let netlist = "* pxf invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PxfRunConfig {
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            max_sideband: 0,
            input_sideband: 1,
            output_sideband: 1,
            ..PxfRunConfig::default()
        };

        let err = run_pxf_analysis_with_config(netlist, &cfg)
            .expect_err("sideband outside configured range should fail");
        assert!(err.contains("sideband"));
    }

    #[test]
    fn test_run_pxf_analysis_with_config_supports_differential_reference_node() {
        let netlist = "* pxf ref\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PxfRunConfig {
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            ..PxfRunConfig::default()
        };

        let result = run_pxf_analysis_with_config(netlist, &cfg)
            .expect("differential output reference should execute");
        assert_eq!(result.frequencies.len(), result.transfer.len());
        assert!(result.output_label.contains("out"));
        assert!(result.output_label.contains("in"));
    }

    #[test]
    fn test_run_pxf_analysis_with_config_rejects_identical_output_and_reference_nodes() {
        let netlist = "* pxf invalid ref\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PxfRunConfig {
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: Some("out".to_string()),
            ..PxfRunConfig::default()
        };

        let err = run_pxf_analysis_with_config(netlist, &cfg)
            .expect_err("matching output and reference nodes should fail");
        assert!(err.contains("cannot be the same"));
    }

    #[test]
    fn test_run_pxf_analysis_auto_infers_output_node() {
        let netlist = "* pxf auto\nVDRIVE in 0 1\nR1 in out 2k\nC1 out 0 2n\n.end\n";
        let result = run_pxf_analysis(netlist).expect("PXF auto mode should infer IO");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.frequencies.len(), result.transfer.len());
        assert!(result.output_label.to_ascii_uppercase().contains("OUT"));
    }

    #[test]
    fn test_run_tf_analysis_with_config_executes_for_driven_rc() {
        let netlist = "* tf\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = TfRunConfig {
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_unit: 6,
            sweep: TfFrequencySweep::Decade,
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: None,
            group_delay: true,
            input_impedance: true,
            output_impedance: true,
        };

        let result = run_tf_analysis_with_config(netlist, &cfg)
            .expect("TF analysis should execute for driven RC");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.transfer.len(), result.frequencies.len());
        assert_eq!(result.magnitude_db.len(), result.frequencies.len());
        assert_eq!(result.phase_deg.len(), result.frequencies.len());
        assert!(
            result
                .transfer
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
        assert!(
            result
                .group_delay
                .as_ref()
                .is_some_and(|curve| !curve.is_empty())
        );
        assert!(
            result
                .input_impedance
                .as_ref()
                .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite()))
        );
        assert!(
            result
                .output_impedance
                .as_ref()
                .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite()))
        );
    }

    #[test]
    fn test_run_tf_analysis_with_config_rejects_unknown_input_source() {
        let netlist = "* tf invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = TfRunConfig {
            input_source: "V_NOT_PRESENT".to_string(),
            output_node: "out".to_string(),
            ..TfRunConfig::default()
        };
        let err =
            run_tf_analysis_with_config(netlist, &cfg).expect_err("missing source must fail TF");
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_run_tf_analysis_auto_infers_configuration() {
        let netlist = "* tf auto\nVDRIVE in 0 1\nR1 in out 2k\nC1 out 0 2n\n.end\n";
        let result = run_tf_analysis(netlist).expect("TF auto mode should infer source and output");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.transfer.len(), result.frequencies.len());
        assert_eq!(result.input_source, "VDRIVE");
        assert!(result.output_label.contains("out") || result.output_label.contains("OUT"));
    }

    #[test]
    fn test_netlist_has_independent_source_named_matches_case_insensitive() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* source lookup\nV1 in 0 1\nI_BIAS out 0 1m\nE1 x 0 in out 10\n.end\n",
        )
        .expect("netlist should parse");
        assert!(netlist_has_independent_source_named(&netlist, "v1"));
        assert!(netlist_has_independent_source_named(&netlist, "I_BIAS"));
        assert!(!netlist_has_independent_source_named(&netlist, "E1"));
        assert!(!netlist_has_independent_source_named(
            &netlist,
            "NOT_PRESENT"
        ));
    }

    #[test]
    fn test_run_pnoise_analysis_with_config_executes_output_referred() {
        let netlist = "* pnoise\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_unit: 6,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 3,
            output_node: "out".to_string(),
            output_ref: None,
            input_source: "V1".to_string(),
            noise_ref: PnoiseReference::Output,
            integrated_noise: true,
            noise_summary: true,
            reltol: 1e-3,
            abstol: 1e-18,
        };

        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("PNOISE output-referred analysis should execute");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.output_noise.len(), result.frequencies.len());
        assert_eq!(result.reference, PnoiseReference::Output);
        assert_eq!(result.sideband_factor, 7);
        assert!(
            result
                .total_output_noise
                .is_some_and(|value| value.is_finite() && value >= 0.0)
        );
    }

    #[test]
    fn test_run_pnoise_analysis_with_phase_reference_produces_dbc() {
        let netlist = "* pnoise phase\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            noise_ref: PnoiseReference::Phase,
            max_sideband: 2,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("PNOISE phase-noise mode should execute");
        assert_eq!(result.reference, PnoiseReference::Phase);
        assert_eq!(result.output_noise.len(), result.frequencies.len());
        assert!(result.output_noise.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn test_run_pnoise_analysis_respects_netlist_temperature_options() {
        let netlist_cold = "* pnoise cold\n.OPTIONS TEMP=-40\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1n\n.end\n";
        let netlist_hot = "* pnoise hot\n.OPTIONS TEMP=125\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            noise_ref: PnoiseReference::Output,
            start_freq: 1e3,
            stop_freq: 1e3,
            points_per_unit: 1,
            sweep: PnoiseFrequencySweep::Linear,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };

        let cold = run_pnoise_analysis_with_config(netlist_cold, &cfg)
            .expect("cold-temperature PNOISE should execute");
        let hot = run_pnoise_analysis_with_config(netlist_hot, &cfg)
            .expect("hot-temperature PNOISE should execute");

        let cold_psd = *cold
            .output_noise
            .first()
            .expect("cold PNOISE run should contain one noise point");
        let hot_psd = *hot
            .output_noise
            .first()
            .expect("hot PNOISE run should contain one noise point");
        assert!(cold_psd.is_finite() && cold_psd > 0.0);
        assert!(hot_psd.is_finite() && hot_psd > cold_psd);
        let ratio = hot_psd / cold_psd;
        assert!(
            ratio > 1.4,
            "expected hot/cold output-noise ratio to reflect temperature scaling, got {}",
            ratio
        );
    }

    #[test]
    fn test_run_pnoise_analysis_input_reference_matches_core_input_referred_density() {
        let netlist = "* pnoise input\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let output_cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            input_source: "V1".to_string(),
            noise_ref: PnoiseReference::Output,
            start_freq: 10.0,
            stop_freq: 1e7,
            points_per_unit: 8,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };
        let input_cfg = PnoiseRunConfig {
            noise_ref: PnoiseReference::Input,
            ..output_cfg.clone()
        };

        let output_result = run_pnoise_analysis_with_config(netlist, &output_cfg)
            .expect("output-referred PNOISE should execute");
        let input_result = run_pnoise_analysis_with_config(netlist, &input_cfg)
            .expect("input-referred PNOISE should execute");

        let input_curve = input_result
            .input_noise
            .as_ref()
            .expect("input-referred mode should return an input-noise vector");
        assert_eq!(input_curve.len(), output_result.output_noise.len());
        assert!(
            !input_result
                .warnings
                .iter()
                .any(|warning| warning.contains("TF fallback"))
        );
        assert!(
            !input_result
                .warnings
                .iter()
                .any(|warning| warning.contains("unity gain"))
        );

        let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
        let mut sim_config = build_engine_config(&parsed, None);
        sim_config.tolerance = input_cfg.pss_tolerance;
        let engine = Engine::new(sim_config);
        let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
        let out_idx =
            resolve_node_or_ground_index("out", &dc.node_names).expect("out must resolve");
        let ref_idx = resolve_node_or_ground_index("in", &dc.node_names).expect("in must resolve");
        let core_input = engine
            .run_noise_with_input_source(
                &parsed,
                out_idx,
                Some(ref_idx),
                "V1",
                &input_result.frequencies,
                300.0,
            )
            .expect("core input-referred noise run should execute");
        assert_eq!(core_input.len(), input_curve.len());
        for (idx, (core_point, ui_point)) in core_input.iter().zip(input_curve.iter()).enumerate() {
            let tol = 1e-24 + core_point.input_referred_density.abs() * 1e-9;
            assert!(
                (core_point.input_referred_density - *ui_point).abs() <= tol,
                "expected UI input-referred density to match core at idx {} (f={} Hz): core={}, ui={}",
                idx,
                core_point.frequency,
                core_point.input_referred_density,
                ui_point
            );
        }
    }

    #[test]
    fn test_run_pnoise_analysis_sideband_output_matches_translated_core_sum() {
        let netlist = "* pnoise sideband output\nV1 in 0 1\nR1 in out 1k\nC1 out 0 2n\n.end\n";
        let cfg = PnoiseRunConfig {
            pss_fundamental_freq: 20e3,
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            noise_ref: PnoiseReference::Output,
            start_freq: 1e3,
            stop_freq: 9e3,
            points_per_unit: 7,
            sweep: PnoiseFrequencySweep::Linear,
            max_sideband: 2,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("sideband PNOISE output run should execute");

        let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
        let mut sim_config = build_engine_config(&parsed, None);
        sim_config.tolerance = cfg.pss_tolerance;
        let noise_temperature = sim_config.temperature;
        let engine = Engine::new(sim_config);
        let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
        let out_idx =
            resolve_node_or_ground_index("out", &dc.node_names).expect("out node should resolve");
        let ref_idx =
            resolve_node_or_ground_index("in", &dc.node_names).expect("ref node should resolve");
        let translated = build_pnoise_sideband_translated_frequencies(
            &result.frequencies,
            result.carrier_frequency,
            cfg.max_sideband,
        )
        .expect("translated sideband frequencies should be generated");
        let core = engine
            .run_noise_ports(
                &parsed,
                out_idx,
                Some(ref_idx),
                &translated,
                noise_temperature,
            )
            .expect("core sideband-translated noise run should execute");
        let expected = fold_sideband_samples(
            &core
                .iter()
                .map(|point| point.output_noise_density.max(0.0))
                .collect::<Vec<_>>(),
            result.frequencies.len(),
            result.sideband_factor,
            "output-referred",
        )
        .expect("folded output sideband sum should compute");

        assert_eq!(expected.len(), result.output_noise.len());
        for (idx, (expected_psd, actual_psd)) in
            expected.iter().zip(result.output_noise.iter()).enumerate()
        {
            let tol = 1e-24 + expected_psd.abs() * 1e-9;
            assert!(
                (expected_psd - actual_psd).abs() <= tol,
                "expected folded output PSD to match translated core sum at idx {}: expected={}, actual={}",
                idx,
                expected_psd,
                actual_psd
            );
        }
    }

    #[test]
    fn test_run_pnoise_analysis_sideband_input_matches_translated_core_sum() {
        let netlist = "* pnoise sideband input\nV1 in 0 1\nR1 in out 1k\nC1 out 0 2n\n.end\n";
        let cfg = PnoiseRunConfig {
            pss_fundamental_freq: 20e3,
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            input_source: "V1".to_string(),
            noise_ref: PnoiseReference::Input,
            start_freq: 1e3,
            stop_freq: 9e3,
            points_per_unit: 7,
            sweep: PnoiseFrequencySweep::Linear,
            max_sideband: 2,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("sideband PNOISE input run should execute");
        assert!(
            !result
                .warnings
                .iter()
                .any(|warning| warning.contains("TF fallback"))
        );

        let input_curve = result
            .input_noise
            .as_ref()
            .expect("input-referred run should produce input curve");
        let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
        let mut sim_config = build_engine_config(&parsed, None);
        sim_config.tolerance = cfg.pss_tolerance;
        let noise_temperature = sim_config.temperature;
        let engine = Engine::new(sim_config);
        let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
        let out_idx =
            resolve_node_or_ground_index("out", &dc.node_names).expect("out node should resolve");
        let ref_idx =
            resolve_node_or_ground_index("in", &dc.node_names).expect("ref node should resolve");
        let translated = build_pnoise_sideband_translated_frequencies(
            &result.frequencies,
            result.carrier_frequency,
            cfg.max_sideband,
        )
        .expect("translated sideband frequencies should be generated");
        let core = engine
            .run_noise_with_input_source(
                &parsed,
                out_idx,
                Some(ref_idx),
                "V1",
                &translated,
                noise_temperature,
            )
            .expect("core input-referred translated noise run should execute");
        let expected = fold_sideband_samples(
            &core
                .iter()
                .map(|point| point.input_referred_density.max(0.0))
                .collect::<Vec<_>>(),
            result.frequencies.len(),
            result.sideband_factor,
            "input-referred",
        )
        .expect("folded input sideband sum should compute");

        assert_eq!(expected.len(), input_curve.len());
        for (idx, (expected_psd, actual_psd)) in expected.iter().zip(input_curve.iter()).enumerate()
        {
            let tol = 1e-24 + expected_psd.abs() * 1e-9;
            assert!(
                (expected_psd - actual_psd).abs() <= tol,
                "expected folded input PSD to match translated core sum at idx {}: expected={}, actual={}",
                idx,
                expected_psd,
                actual_psd
            );
        }
    }

    #[test]
    fn test_run_pnoise_analysis_sideband_contributor_percentages_are_normalized() {
        let netlist = "* pnoise sideband contributors\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            max_sideband: 2,
            noise_summary: true,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("sideband PNOISE contributor run should execute");
        assert!(!result.contributors.is_empty());
        let total_percentage: Value = result
            .contributors
            .iter()
            .map(|(_, percentage)| *percentage)
            .sum();
        assert!(
            (total_percentage - 100.0).abs() <= 1e-6,
            "expected sideband-folded contributor percentages to normalize to 100, got {}",
            total_percentage
        );
    }

    #[test]
    fn test_run_pnoise_analysis_input_reference_rejects_unknown_input_source() {
        let netlist = "* pnoise input unknown source\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            noise_ref: PnoiseReference::Input,
            input_source: "V_NOT_PRESENT".to_string(),
            start_freq: 1e3,
            stop_freq: 1e5,
            points_per_unit: 3,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };

        let err = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect_err("input-referred PNOISE should reject unknown explicit input source");
        assert!(err.contains("V_NOT_PRESENT"));
        assert!(err.contains("independent"));
    }

    #[test]
    fn test_run_pnoise_analysis_input_reference_requires_inferable_source_when_unspecified() {
        let netlist = "* pnoise missing source\nR1 out 0 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            noise_ref: PnoiseReference::Input,
            input_source: String::new(),
            start_freq: 1e3,
            stop_freq: 1e5,
            points_per_unit: 3,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };

        let err = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect_err("input-referred PNOISE should require explicit or inferable source");
        assert!(err.contains("requires an explicit input source"));
    }

    #[test]
    fn test_run_pnoise_analysis_supports_differential_reference_node() {
        let netlist = "* pnoise differential\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("differential PNOISE output should execute");
        assert_eq!(result.output_noise.len(), result.frequencies.len());
        assert!(!result.contributors.is_empty());
        assert!(
            result
                .contributors
                .iter()
                .all(|(_, percentage)| percentage.is_finite() && *percentage >= 0.0)
        );
        assert!(
            !result
                .warnings
                .iter()
                .any(|warning| warning.contains("uncorrelated PSD summation"))
        );
    }

    #[test]
    fn test_run_pnoise_analysis_differential_noise_is_not_less_than_single_ended() {
        let netlist = "* pnoise differential compare\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let single_cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: None,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };
        let differential_cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };

        let single = run_pnoise_analysis_with_config(netlist, &single_cfg)
            .expect("single-ended PNOISE should execute");
        let differential = run_pnoise_analysis_with_config(netlist, &differential_cfg)
            .expect("differential PNOISE should execute");
        assert_eq!(single.output_noise.len(), differential.output_noise.len());
        for (single_value, diff_value) in single
            .output_noise
            .iter()
            .zip(differential.output_noise.iter())
        {
            assert!(
                *diff_value + 1e-30 >= *single_value,
                "differential PSD {} should be >= single-ended PSD {}",
                diff_value,
                single_value
            );
        }
    }

    #[test]
    fn test_run_pnoise_analysis_differential_output_matches_core_noise_port_density() {
        let netlist = "* pnoise differential parity\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: Some("in".to_string()),
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 0,
            ..PnoiseRunConfig::default()
        };
        let result = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect("differential PNOISE output should execute");

        let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
        let mut sim_config = build_engine_config(&parsed, None);
        sim_config.tolerance = cfg.pss_tolerance;
        let engine = Engine::new(sim_config);
        let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
        let out_idx =
            resolve_node_or_ground_index("out", &dc.node_names).expect("out must resolve");
        let ref_idx = resolve_node_or_ground_index("in", &dc.node_names).expect("in must resolve");
        let core = engine
            .run_noise_ports(&parsed, out_idx, Some(ref_idx), &result.frequencies, 300.0)
            .expect("core differential noise port run should execute");

        assert_eq!(core.len(), result.output_noise.len());
        for (idx, (core_point, ui_point)) in core.iter().zip(result.output_noise.iter()).enumerate()
        {
            let tol = 1e-24 + core_point.output_noise_density.abs() * 1e-9;
            assert!(
                (core_point.output_noise_density - *ui_point).abs() <= tol,
                "expected UI differential output density to match core at idx {} (f={} Hz): core={}, ui={}",
                idx,
                core_point.frequency,
                core_point.output_noise_density,
                ui_point
            );
        }
    }

    #[test]
    fn test_run_pnoise_analysis_rejects_identical_output_and_reference_nodes() {
        let netlist = "* pnoise invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PnoiseRunConfig {
            output_node: "out".to_string(),
            output_ref: Some("out".to_string()),
            ..PnoiseRunConfig::default()
        };
        let err = run_pnoise_analysis_with_config(netlist, &cfg)
            .expect_err("PNOISE output/reference node collision should fail");
        assert!(err.contains("cannot be the same"));
    }

    #[test]
    fn test_run_pnoise_analysis_auto_infers_output_node() {
        let netlist = "* pnoise auto\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let result =
            run_pnoise_analysis(netlist).expect("PNOISE auto mode should infer output node");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.output_noise.len(), result.frequencies.len());
    }

    #[test]
    fn test_run_pstb_analysis_with_config_executes_for_driven_rlc_probe() {
        let netlist = "* pstb\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
        let cfg = PstbRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 8,
            num_multipliers: 4,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        };

        let result = run_pstb_analysis_with_config(netlist, &cfg)
            .expect("PSTB analysis should execute with explicit config");
        assert!(result.period.is_finite() && result.period > 0.0);
        assert!(result.fundamental_frequency.is_finite() && result.fundamental_frequency > 0.0);
        assert_eq!(result.probe_instance, "LPROBE");
        assert!(result.probe_branch_ordinal > 0);
        assert!(result.probe_state_self_transition.is_finite());
        assert!(
            result.probe_state_column_norm.is_finite() && result.probe_state_column_norm >= 0.0
        );
        assert!(result.probe_state_row_norm.is_finite() && result.probe_state_row_norm >= 0.0);
        assert!(result.probe_state_persistence_db.is_finite());
        assert!(!result.mode_indices.is_empty());
        assert_eq!(
            result.mode_indices.len(),
            result.probe_mode_participation.len()
        );
        assert_eq!(result.mode_indices.len(), result.multiplier_magnitude.len());
        assert_eq!(result.mode_indices.len(), result.multiplier_phase_deg.len());
        assert_eq!(result.mode_indices.len(), result.mode_damping.len());
        assert_eq!(result.mode_indices.len(), result.mode_frequency_hz.len());
        assert_eq!(result.mode_indices.len(), result.stability_margin_db.len());
        assert!(
            result
                .probe_mode_participation
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0 && *value <= 1.0)
        );
        assert!(
            result
                .multiplier_magnitude
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0)
        );
        assert!(
            result
                .multiplier_phase_deg
                .iter()
                .all(|value| value.is_finite())
        );
        assert!(result.mode_damping.iter().all(|value| value.is_finite()));
        assert!(
            result
                .mode_frequency_hz
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0)
        );
        assert!(
            result
                .stability_margin_db
                .iter()
                .all(|value| value.is_finite())
        );
        assert!(result.dominant_multiplier_magnitude.is_finite());
        assert!(result.min_stability_margin_db.is_finite());
        assert!(
            result.dominant_probe_mode >= 1
                && result.dominant_probe_mode <= result.mode_indices.len()
        );
        assert!(result.dominant_probe_mode_participation.is_finite());
        let max_probe_participation = result
            .probe_mode_participation
            .iter()
            .copied()
            .fold(0.0, Value::max);
        assert!((result.dominant_probe_mode_participation - max_probe_participation).abs() < 1e-12);
        assert!(result.stability_classification.len() >= 4);
        assert_eq!(
            result.num_unstable,
            result
                .multiplier_magnitude
                .iter()
                .filter(|value| **value > cfg.stability_threshold)
                .count()
        );
    }

    #[test]
    fn test_run_pstb_analysis_with_config_rejects_unknown_probe() {
        let netlist =
            "* pstb missing probe\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
        let cfg = PstbRunConfig {
            probe_instance: "LDOES_NOT_EXIST".to_string(),
            ..PstbRunConfig::default()
        };

        let err = run_pstb_analysis_with_config(netlist, &cfg)
            .expect_err("PSTB should reject unknown probe instance names");
        assert!(err.contains("not found"));
        assert!(err.contains("Available branches"));
        assert!(err.contains("LPROBE"));
    }

    #[test]
    fn test_run_pstb_analysis_with_config_rejects_non_dynamic_probe_branch() {
        let netlist = "* pstb non-dynamic probe\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
        let cfg = PstbRunConfig {
            probe_instance: "V1".to_string(),
            ..PstbRunConfig::default()
        };

        let err = run_pstb_analysis_with_config(netlist, &cfg)
            .expect_err("PSTB should reject voltage-source probes in probe-aware mode");
        assert!(err.contains("inductor"));
        assert!(err.contains("Available inductor probes"));
    }

    #[test]
    fn test_run_pstb_analysis_with_config_maps_probe_to_expected_state_index() {
        let netlist =
            "* pstb state index\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
        let cfg = PstbRunConfig {
            probe_instance: "lprobe".to_string(),
            num_multipliers: 3,
            ..PstbRunConfig::default()
        };

        let result = run_pstb_analysis_with_config(netlist, &cfg)
            .expect("PSTB should resolve case-insensitive inductor probe names");

        let parsed = rspice_core::netlist::parse_netlist(netlist).expect("test netlist must parse");
        let engine = Engine::new(build_engine_config(&parsed, None));
        let circuit = engine
            .build_circuit(&parsed)
            .expect("test circuit should build");
        let expected_branch = circuit
            .get_branch_by_name("LPROBE")
            .expect("LPROBE branch must exist");
        let expected_inductor_index = circuit
            .inductors
            .branch_indices
            .iter()
            .position(|branch| *branch == expected_branch)
            .expect("LPROBE should map to inductor branch");
        let expected_state_index = circuit.capacitors.len() + expected_inductor_index;

        assert_eq!(result.probe_branch_ordinal, expected_branch);
        assert_eq!(result.probe_state_index, expected_state_index);
    }

    #[test]
    fn test_run_pstb_analysis_with_config_rejects_invalid_multiplier_count() {
        let netlist = "* pstb invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        let cfg = PstbRunConfig {
            num_multipliers: 0,
            ..PstbRunConfig::default()
        };

        let err = run_pstb_analysis_with_config(netlist, &cfg)
            .expect_err("PSTB should reject zero requested multipliers");
        assert!(err.contains("multipliers"));
    }

    #[test]
    fn test_run_pstb_analysis_default_executes() {
        let netlist =
            "* pstb auto\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
        let result = run_pstb_analysis(netlist).expect("PSTB default mode should execute");
        assert!(!result.mode_indices.is_empty());
        assert_eq!(
            result.mode_indices.len(),
            result.probe_mode_participation.len()
        );
        assert_eq!(result.mode_indices.len(), result.multiplier_magnitude.len());
        assert!(result.stability_classification.len() >= 4);
        assert_eq!(result.probe_instance, "LPROBE");
    }

    #[test]
    fn test_run_sparameter_analysis_returns_expected_shapes_for_decoupled_matched_ports() {
        let netlist = "* S-parameter decoupled matched ports\nR1 in 0 50\nR2 out 0 50\n.end\n";
        let cfg = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 5,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![
                SParameterPort::single_ended("in"),
                SParameterPort::single_ended("out"),
            ],
        };

        let result = run_sparameter_analysis(netlist, &cfg)
            .expect("S-parameter analysis should execute for simple two-port");
        assert!(!result.frequencies.is_empty());
        assert_eq!(result.num_ports, 2);
        assert_eq!(result.s.len(), 2);
        assert_eq!(result.s[0].len(), 2);
        assert_eq!(result.s[1].len(), 2);
        assert_eq!(result.frequencies.len(), result.s[0][0].len());
        for idx in 0..result.frequencies.len() {
            assert!(
                result.s[1][0][idx].norm() < 1e-8,
                "S21 should be near 0 for decoupled ports"
            );
            assert!(
                result.s[0][1][idx].norm() < 1e-8,
                "S12 should be near 0 for decoupled ports"
            );
            assert!(
                (result.s[0][0][idx] - result.s[1][1][idx]).norm() < 1e-9,
                "symmetric ports should have matching reflections"
            );
            assert!(
                result.s[0][0][idx].norm().is_finite() && result.s[0][0][idx].norm() <= 1.0 + 1e-6
            );
            assert!(
                result.s[1][1][idx].norm().is_finite() && result.s[1][1][idx].norm() <= 1.0 + 1e-6
            );
        }
    }

    #[test]
    fn test_run_sparameter_analysis_preserves_per_port_reference_impedance_overrides() {
        let netlist = "* S-parameter decoupled matched ports\nR1 in 0 50\nR2 out 0 50\n.end\n";
        let cfg = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 5,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![
                SParameterPort::single_ended("in"),
                SParameterPort {
                    node_pos: "out".to_string(),
                    node_neg: "0".to_string(),
                    z0: Some(75.0),
                },
            ],
        };

        let result = run_sparameter_analysis(netlist, &cfg)
            .expect("S-parameter analysis should execute with per-port z0");
        assert_eq!(result.z0, vec![50.0, 75.0]);
    }

    #[test]
    fn test_run_sparameter_analysis_supports_three_port_matrices() {
        let netlist = "* S-parameter 3-port matched\nR1 p1 0 50\nR2 p2 0 50\nR3 p3 0 50\n.end\n";
        let cfg = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e5,
            points_per_unit: 3,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![
                SParameterPort::single_ended("p1"),
                SParameterPort::single_ended("p2"),
                SParameterPort::single_ended("p3"),
            ],
        };

        let result = run_sparameter_analysis(netlist, &cfg)
            .expect("S-parameter analysis should execute for simple three-port");
        assert_eq!(result.num_ports, 3);
        assert_eq!(result.s.len(), 3);
        for row in &result.s {
            assert_eq!(row.len(), 3);
            for trace in row {
                assert_eq!(trace.len(), result.frequencies.len());
            }
        }
        for idx in 0..result.frequencies.len() {
            for row in 0..3 {
                for col in 0..3 {
                    assert!(
                        result.s[row][col][idx].norm().is_finite(),
                        "S{}{} should be finite",
                        row + 1,
                        col + 1
                    );
                }
            }
            for row in 0..3 {
                for col in 0..3 {
                    if row == col {
                        continue;
                    }
                    assert!(
                        result.s[row][col][idx].norm() <= 1e-8,
                        "S{}{} should be near 0 for decoupled ports",
                        row + 1,
                        col + 1
                    );
                }
            }
            for row in 0..3 {
                assert!(
                    result.s[row][row][idx].norm() <= 1.0 + 1e-6,
                    "S{}{} should stay passive and bounded",
                    row + 1,
                    row + 1
                );
            }
            let s11 = result.s[0][0][idx];
            assert!((s11 - result.s[1][1][idx]).norm() < 1e-9);
            assert!((s11 - result.s[2][2][idx]).norm() < 1e-9);
        }
    }

    #[test]
    fn test_run_sparameter_analysis_rejects_single_port_config() {
        let netlist = "* S-parameter invalid\nR1 p1 0 50\n.end\n";
        let cfg = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 5,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![SParameterPort::single_ended("p1")],
        };

        let err = run_sparameter_analysis(netlist, &cfg)
            .expect_err("single-port S-parameter config should be rejected");
        assert!(err.contains("at least 2 ports"));
    }

    #[test]
    fn test_run_envelope_analysis_produces_envelope_traces() {
        let netlist = "* Envelope sine\nV1 out 0 SIN(0 1 1Meg 0 0 0)\nR1 out 0 1k\n.end\n";
        let cfg = EnvelopeRunConfig {
            fundamental_freq: 1e6,
            stop_time: 10e-6,
            num_harmonics: 9,
            max_step: None,
        };

        let result = run_envelope_analysis(netlist, &cfg)
            .expect("Envelope analysis should run for simple sinusoid");
        assert!(!result.time.is_empty());
        assert!(!result.waveforms.is_empty());
        for (name, values) in &result.waveforms {
            assert!(name.starts_with("ENV("));
            assert_eq!(values.len(), result.time.len());
            assert!(values.iter().all(|v| v.is_finite() && *v >= 0.0));
            let max_env = values.iter().copied().fold(0.0, Value::max);
            assert!(
                max_env > 1e-4,
                "envelope should contain non-trivial amplitude"
            );
        }
    }

    #[test]
    fn test_run_fourier_analysis_detects_fundamental_and_low_thd_for_sine() {
        let netlist = "* Fourier sine\nV1 out 0 SIN(0 1 1k 0 0 0)\nR1 out 0 1k\n.end\n";
        let cfg = FourierRunConfig {
            fundamental_freq: 1e3,
            num_harmonics: 8,
            output_node: "out".to_string(),
            output_ref: None,
            start_time: 0.0,
            stop_time: 20e-3,
        };

        let result =
            run_fourier_analysis(netlist, &cfg).expect("Fourier analysis should run for sine");
        assert_eq!(result.frequencies.len(), result.response.len());
        assert_eq!(result.frequencies.len(), cfg.num_harmonics + 1);
        assert!(!result.response.is_empty());
        let fundamental = result
            .response
            .get(1)
            .expect("fundamental component should exist");
        assert!(
            fundamental.norm() > 0.7 && fundamental.norm() < 1.3,
            "fundamental magnitude should be near 1V, got {}",
            fundamental.norm()
        );
        assert!(
            result.thd_percent < 1.0,
            "pure sine THD should be low, got {}%",
            result.thd_percent
        );
    }

    #[test]
    fn test_extract_reliability_stress_data_maps_transistor_biases() {
        let elements = vec![
            Element {
                name: "M1".to_string(),
                kind: ElementKind::Mosfet {
                    model: "NM".to_string(),
                    mos_type: rspice_core::netlist::MosType::Nmos,
                },
                nodes: vec![
                    "d".to_string(),
                    "g".to_string(),
                    "s".to_string(),
                    "0".to_string(),
                ],
            },
            Element {
                name: "Q1".to_string(),
                kind: ElementKind::Bjt {
                    model: "NPN".to_string(),
                    bjt_type: rspice_core::netlist::BjtType::Npn,
                },
                nodes: vec!["c".to_string(), "b".to_string(), "e".to_string()],
            },
        ];
        let node_voltages = std::collections::HashMap::from([
            ("D".to_string(), 1.8),
            ("G".to_string(), 1.1),
            ("S".to_string(), 0.2),
            ("C".to_string(), 2.5),
            ("B".to_string(), 0.9),
            ("E".to_string(), 0.0),
        ]);

        let stress = extract_reliability_stress_data(&elements, &node_voltages, 300.0, 0.05);
        let m1 = stress.get("M1").expect("M1 stress should be extracted");
        assert!((m1.avg_vgs_stress - 0.9).abs() < 1e-12);
        assert!((m1.avg_vds_stress - 1.6).abs() < 1e-12);

        let q1 = stress.get("Q1").expect("Q1 stress should be extracted");
        assert!((q1.avg_vgs_stress - 0.9).abs() < 1e-12);
        assert!((q1.avg_vds_stress - 2.5).abs() < 1e-12);
    }

    #[test]
    fn test_run_reliability_analysis_with_config_produces_device_results() {
        let netlist = r#"
* Reliability smoke
VDD vdd 0 1.8
VG g 0 1.2
R1 vdd d 1k
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#;
        let cfg = ReliabilityRunConfig {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.05,
        };

        let data = run_reliability_analysis_with_config(netlist, &cfg)
            .expect("reliability analysis should execute for stressed MOSFET");
        assert_eq!(data.years, vec![1.0, 5.0, 10.0]);
        assert!(!data.device_results.is_empty());
        let m1 = data
            .device_results
            .iter()
            .find(|result| result.device_id.eq_ignore_ascii_case("M1"))
            .expect("M1 should be present in reliability results");
        assert!(m1.shifts.contains_key("1y"));
        assert!(m1.shifts.contains_key("10y"));
        assert!(m1.shifts["10y"].vth_shift > m1.shifts["1y"].vth_shift);
    }

    #[test]
    fn test_run_reliability_analysis_with_config_rejects_if_no_stressed_devices() {
        let netlist = r#"
* No stressed semiconductor devices
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.end
"#;
        let cfg = ReliabilityRunConfig {
            target_years: vec![1.0, 5.0],
            enable_hci: true,
            enable_nbti: false,
            enable_em: false,
            min_stress_voltage: 0.1,
        };
        let err = run_reliability_analysis_with_config(netlist, &cfg)
            .expect_err("reliability should fail when no qualifying devices exist");
        assert!(err.contains("no stressed semiconductor devices"));
    }

    #[test]
    fn test_apply_reliability_mechanism_scaling_em_only_suppresses_vth_mobility_shift() {
        let mut results = vec![ReliabilityResult {
            device_id: "M1".to_string(),
            stress: StressMetrics {
                avg_vgs_stress: 1.0,
                avg_vds_stress: 1.0,
                avg_temp: 300.0,
                duration: 3600.0,
            },
            shifts: std::collections::HashMap::from([(
                "1y".to_string(),
                ParamShift {
                    vth_shift: 1.0,
                    mobility_shift: -1.0,
                    rds_shift: 0.5,
                },
            )]),
        }];
        let cfg = ReliabilityRunConfig {
            target_years: vec![1.0],
            enable_hci: false,
            enable_nbti: false,
            enable_em: true,
            min_stress_voltage: 0.0,
        };

        apply_reliability_mechanism_scaling(&mut results, &cfg);
        let shift = results[0]
            .shifts
            .get("1y")
            .expect("scaled shift should still be present");
        assert!((shift.vth_shift - 0.0).abs() < 1e-12);
        assert!((shift.mobility_shift - 0.0).abs() < 1e-12);
        assert!(shift.rds_shift > 1.0, "EM scaling should amplify Rds shift");
    }

    #[test]
    fn test_run_optimization_analysis_with_config_targets_resistive_divider_voltage() {
        let netlist = r#"
* optimization smoke
.param RTOP=1k
.param RBOT=1k
V1 in 0 2
R1 in out {RTOP}
R2 out 0 {RBOT}
.end
"#;
        let cfg = OptimizationRunConfig {
            variables: vec![OptimizationVariable {
                name: "RBOT".to_string(),
                min: 500.0,
                max: 3000.0,
                initial: 1000.0,
            }],
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: OptimizationGoalMode::Target,
            target: Some(1.2),
            algorithm: OptimizationAlgorithmMode::PatternSearch,
            max_iterations: 80,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.2,
            min_step: 1e-8,
        };

        let data = run_optimization_analysis_with_config(netlist, &cfg)
            .expect("optimization should run for divider netlist");
        assert!(!data.iterations.is_empty());
        assert_eq!(data.iterations.len(), data.costs.len());
        let rbot_trace = data
            .variable_traces
            .get("RBOT")
            .expect("RBOT trace should exist");
        assert_eq!(rbot_trace.len(), data.iterations.len());
        assert!(data.best_cost.is_finite());
        let best_rbot = data
            .best_variables
            .get("RBOT")
            .copied()
            .expect("best RBOT should be present");
        assert!(
            (best_rbot - 1500.0).abs() < 250.0,
            "expected optimizer to approach ideal RBOT ~1500 ohm, got {}",
            best_rbot
        );
    }

    #[test]
    fn test_run_optimization_analysis_rejects_invalid_variable_name() {
        let cfg = OptimizationRunConfig {
            variables: vec![OptimizationVariable {
                name: "1BAD".to_string(),
                min: 1.0,
                max: 2.0,
                initial: 1.5,
            }],
            ..OptimizationRunConfig::default()
        };
        let err = run_optimization_analysis_with_config("* invalid\nR1 in 0 1k\n.end\n", &cfg)
            .expect_err("invalid variable name must be rejected");
        assert!(err.contains("Invalid optimization variable name"));
    }

    #[test]
    fn test_inject_param_overrides_inserts_before_last_end() {
        let netlist = "* test\n.param A=1\nR1 in 0 {A}\n.end\n";
        let vars = std::collections::HashMap::from([("A".to_string(), 2.5)]);
        let overridden = inject_param_overrides(netlist, &vars);
        let lowered = overridden.to_ascii_lowercase();
        let end_pos = lowered
            .rfind(".end")
            .expect("overridden netlist should include .end");
        let param_pos = lowered
            .rfind("a=2.5000000000000000e+00")
            .expect("override assignment should be present");
        assert!(param_pos < end_pos);
    }

    #[test]
    fn test_inject_param_overrides_rewrites_existing_param_lines_in_place() {
        let netlist = "* opt\n.param A=1 B=2\n.param A=3\nR1 out 0 {A}\n.end\n";
        let vars = std::collections::HashMap::from([("A".to_string(), 4.25)]);
        let overridden = inject_param_overrides(netlist, &vars);
        let lowered = overridden.to_ascii_lowercase();
        let expected = ".param a=1 b=2 a=4.2500000000000000e+00";
        assert!(
            lowered.contains(expected),
            "first .param line should include override assignment: {}",
            overridden
        );
        assert!(
            lowered.contains(".param a=3 a=4.2500000000000000e+00"),
            "every matching .param line should include override assignment"
        );
    }

    #[test]
    fn test_inject_param_overrides_inserts_missing_params_after_title_line() {
        let netlist = "optimization deck\nR1 out 0 {A}\n.end\n";
        let vars = std::collections::HashMap::from([("A".to_string(), 1.75)]);
        let overridden = inject_param_overrides(netlist, &vars);
        let lines: Vec<&str> = overridden.lines().collect();
        assert_eq!(lines[0], "optimization deck");
        assert!(
            lines[1]
                .to_ascii_lowercase()
                .contains(".param a=1.7500000000000000e+00"),
            "missing overrides should be inserted directly after title line"
        );
    }

    #[test]
    fn test_format_param_override_value_uses_explicit_signed_exponent() {
        assert_eq!(
            format_param_override_value(2.5),
            "2.5000000000000000e+00".to_string()
        );
        assert_eq!(
            format_param_override_value(1e-9),
            "1.0000000000000001e-09".to_string()
        );
    }

    #[test]
    fn test_run_soa_analysis_with_config_detects_mos_voltage_violation() {
        let netlist = r#"
* soa smoke
VDD d 0 3.3
VG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#;
        let cfg = SoaRunConfig {
            stop_time: 32e-9,
            step_time: 1e-9,
            check_vgs_max: true,
            max_vgs: 1.2,
            check_vds_max: true,
            max_vds: 3.0,
            check_vbe_max: false,
            max_vbe: 1.0,
            check_vce_max: false,
            max_vce: 5.0,
        };
        let data = run_soa_analysis_with_config(netlist, &cfg)
            .expect("SOA should execute for MOS transient netlist");
        assert!(!data.time.is_empty());
        assert_eq!(data.time.len(), data.violation_count.len());
        assert!(
            !data.violations.is_empty(),
            "expected SOA voltage violations with aggressive limits"
        );
        let last = *data
            .violation_count
            .last()
            .expect("violation count trace should have data");
        assert!(last >= 1.0);
    }

    #[test]
    fn test_run_soa_analysis_rejects_netlist_without_supported_devices() {
        let netlist = "* soa none\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
        let err = run_soa_analysis(netlist)
            .expect_err("SOA should fail when no supported semiconductor devices exist");
        assert!(err.contains("supported semiconductor devices"));
    }
}
