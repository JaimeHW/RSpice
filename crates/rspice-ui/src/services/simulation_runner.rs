//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use crate::output_spec::{
    ac_output_value, collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_node_or_ground_index,
    resolve_sensitivity_ac_frequency, run_ac_output_at_frequency, run_dc_output_sensitivity,
    validate_sensitivity_output_spec, OutputSpec, OutputVoltageSpec,
};
use crate::services::safety::{SoADefinition, SoALimit, SoAManager, SoAParameter, SoAViolation};
use crate::simulation::optimizer::{
    DesignVar, OptimizationGoal, OptimizerAlgo, OptimizerConfig, OptimizerEngine,
};
use crate::simulation::reliability_engine::{
    ParamShift, ReliabilityEngine, ReliabilityResult, StressMetrics,
};
use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{
    AnalysisCommand, Element, ElementKind, MonteCarloDistribution, SourceSpec, StepCommand,
    StepSweep, StepTarget,
};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use rspice_core::{resolve_simulation_config, SimulationConfigOverrides, Value};

#[path = "simulation_runner/harmonic_basis.rs"]
mod harmonic_basis;
use harmonic_basis::{build_disto_two_tone_harmonic_plan, build_multi_tone_hb_layout};

#[path = "simulation_runner/pnoise.rs"]
mod pnoise;
#[path = "simulation_runner/pnoise_sideband.rs"]
mod pnoise_sideband;
pub use pnoise::{
    run_pnoise_analysis, run_pnoise_analysis_with_config, PnoiseData, PnoiseFrequencySweep,
    PnoiseReference, PnoiseRunConfig,
};
#[cfg(test)]
use pnoise_sideband::{build_pnoise_sideband_translated_frequencies, fold_sideband_samples};

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

/// Sweep type for DISTO analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistoFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl DistoFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Explicit configuration for DISTO execution.
#[derive(Debug, Clone)]
pub struct DistoRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: DistoFrequencySweep,
    /// Optional secondary tone ratio for IMD estimates.
    pub f2_over_f1: Option<Value>,
    /// Whether to allow linearized AC fallback when nonlinear HB DISTO is unavailable.
    pub allow_linearized_fallback: bool,
}

impl DistoRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("DISTO start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err("DISTO stop frequency must be greater than start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("DISTO points per unit must be greater than zero".to_string());
        }
        if let Some(ratio) = self.f2_over_f1 {
            if !ratio.is_finite() || ratio <= 1.0 {
                return Err("DISTO f2_over_f1 must be finite and > 1".to_string());
            }
        }
        Ok(())
    }
}

/// Per-trace DISTO output.
#[derive(Debug, Clone)]
pub struct DistoTrace {
    pub name: String,
    /// Fundamental transfer magnitude in dB.
    pub fundamental_gain_db: Vec<Value>,
    /// 2nd-harmonic estimate in dBc.
    pub hd2_db: Vec<Value>,
    /// 3rd-harmonic estimate in dBc.
    pub hd3_db: Vec<Value>,
    /// THD estimate in percent (from HD2/HD3).
    pub thd_percent: Vec<Value>,
    /// Optional IMD2 estimate in dBc when f2/f1 is configured.
    pub imd2_db: Option<Vec<Value>>,
    /// Optional IMD3 estimate in dBc when f2/f1 is configured.
    pub imd3_db: Option<Vec<Value>>,
}

/// DISTO analysis output.
#[derive(Debug, Clone)]
pub struct DistoData {
    pub frequencies: Vec<Value>,
    pub traces: Vec<DistoTrace>,
    pub warnings: Vec<String>,
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

/// Run DISTO analysis using nonlinear HB harmonic extraction.
///
/// Primary execution solves HB per sweep point and extracts HD2/HD3/THD from
/// harmonic spectra. When `f2_over_f1` is configured, it additionally performs
/// commensurate two-tone HB and derives IMD2/IMD3 from nonlinear sidebands.
/// Linearized AC fallback is only used when explicitly enabled.
pub fn run_disto_analysis(
    netlist_text: &str,
    config: &DistoRunConfig,
) -> Result<DistoData, String> {
    config.validate()?;

    match run_disto_analysis_nonlinear_hb(netlist_text, config) {
        Ok(data) => Ok(data),
        Err(nonlinear_error) => {
            if !config.allow_linearized_fallback {
                return Err(format!(
                    "DISTO nonlinear HB path failed ({}). Set allow_linearized_fallback=true to use the lower-fidelity linearized approximation.",
                    nonlinear_error
                ));
            }
            let mut linearized = run_disto_analysis_linearized(netlist_text, config)?;
            linearized.warnings.push(format!(
                "DISTO nonlinear HB path was unavailable ({}); used linearized transfer-based fallback.",
                nonlinear_error
            ));
            Ok(linearized)
        }
    }
}

fn run_disto_analysis_nonlinear_hb(
    netlist_text: &str,
    config: &DistoRunConfig,
) -> Result<DistoData, String> {
    use rspice_core::analysis::{HbConfig, HbTone};

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let two_tone_plan = config
        .f2_over_f1
        .map(build_disto_two_tone_harmonic_plan)
        .transpose()?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err("DISTO sweep generated no frequency points".to_string());
    }

    struct DistoAccum {
        fundamental_gain_db: Vec<Value>,
        hd2_db: Vec<Value>,
        hd3_db: Vec<Value>,
        thd_percent: Vec<Value>,
        imd2_db: Option<Vec<Value>>,
        imd3_db: Option<Vec<Value>>,
    }

    let mut accumulators: Vec<(String, DistoAccum)> = Vec::new();
    for (point_idx, &freq) in frequencies.iter().enumerate() {
        let (hb_config, fundamental_harmonic) = if let Some(plan) = two_tone_plan {
            let base_freq = freq / plan.tone1_harmonic as Value;
            let mut hb_config = HbConfig::new(base_freq)
                .with_harmonics(plan.max_harmonic)
                .with_tolerance(1e-6);
            hb_config.tones = vec![
                HbTone::new(freq, 1).with_name("f1"),
                HbTone::new(freq * plan.f2_over_f1, 1).with_name("f2"),
            ];
            (hb_config, plan.tone1_harmonic)
        } else {
            (
                HbConfig::new(freq).with_harmonics(3).with_tolerance(1e-6),
                1,
            )
        };

        let hb = engine
            .run_hb(&netlist, hb_config)
            .map_err(|e| format!("HB DISTO solve failed at {:.6e} Hz: {}", freq, e))?;

        if point_idx == 0 {
            accumulators = hb
                .result
                .spectral_voltages
                .iter()
                .map(|sv| {
                    (
                        format!("V({})", sv.node_name),
                        DistoAccum {
                            fundamental_gain_db: Vec::with_capacity(frequencies.len()),
                            hd2_db: Vec::with_capacity(frequencies.len()),
                            hd3_db: Vec::with_capacity(frequencies.len()),
                            thd_percent: Vec::with_capacity(frequencies.len()),
                            imd2_db: two_tone_plan.map(|_| Vec::with_capacity(frequencies.len())),
                            imd3_db: two_tone_plan.map(|_| Vec::with_capacity(frequencies.len())),
                        },
                    )
                })
                .collect();
        }

        for (trace_name, acc) in &mut accumulators {
            let node_name = trace_name
                .strip_prefix("V(")
                .and_then(|s| s.strip_suffix(')'))
                .unwrap_or(trace_name.as_str());
            let spectrum = hb
                .result
                .spectral_voltages
                .iter()
                .find(|sv| sv.node_name.eq_ignore_ascii_case(node_name))
                .ok_or_else(|| {
                    format!(
                        "HB DISTO solve at {:.6e} Hz is missing spectral voltage for {}",
                        freq, trace_name
                    )
                })?;
            let fund =
                hb_magnitude_at_harmonic(&spectrum.coefficients, fundamental_harmonic).max(1e-30);
            let h2 =
                hb_magnitude_at_harmonic(&spectrum.coefficients, 2 * fundamental_harmonic).max(0.0);
            let h3 =
                hb_magnitude_at_harmonic(&spectrum.coefficients, 3 * fundamental_harmonic).max(0.0);

            let r2 = h2 / fund;
            let r3 = h3 / fund;
            acc.fundamental_gain_db.push(magnitude_to_db(fund));
            acc.hd2_db.push(ratio_to_dbc(r2));
            acc.hd3_db.push(ratio_to_dbc(r3));
            acc.thd_percent.push((r2 * r2 + r3 * r3).sqrt() * 100.0);

            if let Some(plan) = two_tone_plan {
                let imd2_harmonics = [
                    plan.tone2_harmonic.abs_diff(plan.tone1_harmonic),
                    plan.tone1_harmonic + plan.tone2_harmonic,
                ];
                let imd3_harmonics = [
                    (2 * plan.tone1_harmonic).abs_diff(plan.tone2_harmonic),
                    (2 * plan.tone2_harmonic).abs_diff(plan.tone1_harmonic),
                ];
                let imd2_ratio =
                    max_spectral_sideband_ratio(&spectrum.coefficients, &imd2_harmonics, fund);
                let imd3_ratio =
                    max_spectral_sideband_ratio(&spectrum.coefficients, &imd3_harmonics, fund);
                if let Some(series) = acc.imd2_db.as_mut() {
                    series.push(ratio_to_dbc(imd2_ratio));
                }
                if let Some(series) = acc.imd3_db.as_mut() {
                    series.push(ratio_to_dbc(imd3_ratio));
                }
            }
        }
    }

    if accumulators.is_empty() {
        return Err("DISTO produced no output traces".to_string());
    }

    let traces: Vec<DistoTrace> = accumulators
        .into_iter()
        .map(|(name, acc)| DistoTrace {
            name,
            fundamental_gain_db: acc.fundamental_gain_db,
            hd2_db: acc.hd2_db,
            hd3_db: acc.hd3_db,
            thd_percent: acc.thd_percent,
            imd2_db: acc.imd2_db,
            imd3_db: acc.imd3_db,
        })
        .collect();

    Ok(DistoData {
        frequencies,
        traces,
        warnings: Vec::new(),
    })
}

fn hb_magnitude_at_harmonic(coefficients: &[Complex64], harmonic: usize) -> Value {
    coefficients
        .get(harmonic)
        .copied()
        .unwrap_or_else(|| Complex64::new(0.0, 0.0))
        .norm()
}

fn max_spectral_sideband_ratio(
    coefficients: &[Complex64],
    sideband_harmonics: &[usize],
    fundamental: Value,
) -> Value {
    let mut best: Value = 0.0;
    for &harmonic in sideband_harmonics {
        if harmonic == 0 {
            continue;
        }
        let magnitude = hb_magnitude_at_harmonic(coefficients, harmonic).max(0.0);
        best = best.max(magnitude / fundamental.max(1e-30));
    }
    best
}

fn run_disto_analysis_linearized(
    netlist_text: &str,
    config: &DistoRunConfig,
) -> Result<DistoData, String> {
    let f2_over_f1 = config.f2_over_f1.unwrap_or(2.0);
    let max_factor = 3.0_f64
        .max(f2_over_f1 + 1.0)
        .max((2.0 * f2_over_f1 - 1.0).abs())
        .max((2.0 - f2_over_f1).abs())
        .max((f2_over_f1 - 1.0).abs());
    let extended_stop = config.stop_freq * max_factor;

    let ac = run_ac_analysis(
        netlist_text,
        config.start_freq,
        extended_stop,
        config.points_per_unit,
        config.sweep.keyword(),
    )?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err("DISTO sweep generated no frequency points".to_string());
    }

    let mut traces = Vec::with_capacity(ac.responses.len());
    for (name, response) in &ac.responses {
        let magnitudes: Vec<Value> = response.iter().map(|value| value.norm()).collect();
        let mut fundamental_gain_db = Vec::with_capacity(frequencies.len());
        let mut hd2_db = Vec::with_capacity(frequencies.len());
        let mut hd3_db = Vec::with_capacity(frequencies.len());
        let mut thd_percent = Vec::with_capacity(frequencies.len());
        let mut imd2_db = config
            .f2_over_f1
            .map(|_| Vec::with_capacity(frequencies.len()));
        let mut imd3_db = config
            .f2_over_f1
            .map(|_| Vec::with_capacity(frequencies.len()));

        for &f1 in &frequencies {
            let fund = interpolate_magnitude_at(&ac.frequencies, &magnitudes, f1)
                .unwrap_or(0.0)
                .max(1e-30);
            let h2 = interpolate_magnitude_at(&ac.frequencies, &magnitudes, 2.0 * f1)
                .unwrap_or(0.0)
                .max(0.0);
            let h3 = interpolate_magnitude_at(&ac.frequencies, &magnitudes, 3.0 * f1)
                .unwrap_or(0.0)
                .max(0.0);

            let r2 = h2 / fund;
            let r3 = h3 / fund;

            fundamental_gain_db.push(magnitude_to_db(fund));
            hd2_db.push(ratio_to_dbc(r2));
            hd3_db.push(ratio_to_dbc(r3));
            thd_percent.push((r2 * r2 + r3 * r3).sqrt() * 100.0);

            if let Some(series) = imd2_db.as_mut() {
                let sidebands = [((f2_over_f1 - 1.0).abs() * f1), ((f2_over_f1 + 1.0) * f1)];
                let ratio = max_sideband_ratio(&ac.frequencies, &magnitudes, &sidebands, fund);
                series.push(ratio_to_dbc(ratio.unwrap_or(0.0)));
            }
            if let Some(series) = imd3_db.as_mut() {
                let sidebands = [
                    ((2.0 - f2_over_f1).abs() * f1),
                    ((2.0 * f2_over_f1 - 1.0).abs() * f1),
                ];
                let ratio = max_sideband_ratio(&ac.frequencies, &magnitudes, &sidebands, fund);
                series.push(ratio_to_dbc(ratio.unwrap_or(0.0)));
            }
        }

        traces.push(DistoTrace {
            name: name.clone(),
            fundamental_gain_db,
            hd2_db,
            hd3_db,
            thd_percent,
            imd2_db,
            imd3_db,
        });
    }

    if traces.is_empty() {
        return Err("DISTO produced no output traces".to_string());
    }

    Ok(DistoData {
        frequencies,
        traces,
        warnings: Vec::new(),
    })
}

fn magnitude_to_db(value: Value) -> Value {
    20.0 * value.max(1e-30).log10()
}

fn ratio_to_dbc(ratio: Value) -> Value {
    20.0 * ratio.max(1e-30).log10()
}

fn max_sideband_ratio(
    frequencies: &[Value],
    magnitudes: &[Value],
    sidebands: &[Value],
    fundamental: Value,
) -> Option<Value> {
    let mut best: Option<Value> = None;
    for &freq in sidebands {
        if freq <= 0.0 {
            continue;
        }
        let Some(mag) = interpolate_magnitude_at(frequencies, magnitudes, freq) else {
            continue;
        };
        let ratio = mag.max(0.0) / fundamental.max(1e-30);
        best = Some(match best {
            Some(existing) => existing.max(ratio),
            None => ratio,
        });
    }
    best
}

fn interpolate_magnitude_at(
    frequencies: &[Value],
    magnitudes: &[Value],
    target: Value,
) -> Option<Value> {
    if frequencies.len() != magnitudes.len() || frequencies.is_empty() || !target.is_finite() {
        return None;
    }
    let first = *frequencies.first()?;
    let last = *frequencies.last()?;
    if target < first || target > last {
        return None;
    }
    if frequencies.len() == 1 {
        return Some(magnitudes[0]);
    }

    match frequencies.binary_search_by(|value| {
        value
            .partial_cmp(&target)
            .unwrap_or(std::cmp::Ordering::Less)
    }) {
        Ok(idx) => magnitudes.get(idx).copied(),
        Err(upper) => {
            if upper == 0 || upper >= frequencies.len() {
                return None;
            }
            let lower = upper - 1;
            let f0 = frequencies[lower];
            let f1 = frequencies[upper];
            let y0 = magnitudes[lower];
            let y1 = magnitudes[upper];
            if (f1 - f0).abs() <= f64::EPSILON {
                return Some(y0);
            }

            let t = if f0 > 0.0 && f1 > 0.0 && target > 0.0 {
                let l0 = f0.log10();
                let l1 = f1.log10();
                if (l1 - l0).abs() <= f64::EPSILON {
                    0.0
                } else {
                    (target.log10() - l0) / (l1 - l0)
                }
            } else {
                (target - f0) / (f1 - f0)
            };
            let t = t.clamp(0.0, 1.0);
            if y0 > 0.0 && y1 > 0.0 {
                let ly0 = y0.log10();
                let ly1 = y1.log10();
                if ly0.is_finite() && ly1.is_finite() {
                    return Some(10.0_f64.powf(ly0 + (ly1 - ly0) * t));
                }
            }
            Some(y0 + (y1 - y0) * t)
        }
    }
}

// =============================================================================
// S-Parameter Analysis
// =============================================================================

/// Sweep type for S-parameter analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SParameterSweep {
    Decade,
    Octave,
    Linear,
}

impl SParameterSweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Port definition for S-parameter analysis.
#[derive(Debug, Clone)]
pub struct SParameterPort {
    pub node_pos: String,
    pub node_neg: String,
    pub z0: Option<Value>,
}

impl SParameterPort {
    pub fn single_ended(node_pos: impl Into<String>) -> Self {
        Self {
            node_pos: node_pos.into(),
            node_neg: "0".to_string(),
            z0: None,
        }
    }
}

/// Explicit configuration for S-parameter execution.
#[derive(Debug, Clone)]
pub struct SParameterRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: SParameterSweep,
    pub z0: Value,
    pub ports: Vec<SParameterPort>,
}

impl SParameterRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("S-parameter start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(
                "S-parameter stop frequency must be greater than start frequency".to_string(),
            );
        }
        if self.points_per_unit == 0 {
            return Err("S-parameter points per unit must be greater than zero".to_string());
        }
        if !self.z0.is_finite() || self.z0 <= 0.0 {
            return Err("S-parameter reference impedance must be positive".to_string());
        }
        if self.ports.len() < 2 {
            return Err("S-parameter analysis requires at least 2 ports".to_string());
        }
        for (idx, port) in self.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} positive node is required",
                    idx + 1
                ));
            }
            if port.node_neg.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} negative node is required",
                    idx + 1
                ));
            }
            if let Some(port_z0) = port.z0 {
                if !port_z0.is_finite() || port_z0 <= 0.0 {
                    return Err(format!("S-parameter port{} z0 must be positive", idx + 1));
                }
            }
        }
        Ok(())
    }
}

/// N-port S-parameter analysis output.
#[derive(Debug, Clone)]
pub struct SParameterData {
    pub frequencies: Vec<Value>,
    /// Number of ports in the solved network.
    pub num_ports: usize,
    /// S-parameter matrix traces indexed as [row][col][frequency_index], 0-based.
    pub s: Vec<Vec<Vec<Complex64>>>,
    /// Per-port reference impedances (ohms).
    pub z0: Vec<Value>,
}

/// Run N-port S-parameter analysis by solving Y-parameters from AC source injections.
pub fn run_sparameter_analysis(
    netlist_text: &str,
    config: &SParameterRunConfig,
) -> Result<SParameterData, String> {
    config.validate()?;

    let parsed_netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err("S-parameter sweep generated no frequency points".to_string());
    }

    let num_ports = config.ports.len();
    let num_freqs = frequencies.len();
    let z0_by_port: Vec<Value> = config
        .ports
        .iter()
        .map(|port| port.z0.unwrap_or(config.z0))
        .collect();
    let mut y = vec![vec![vec![Complex64::new(0.0, 0.0); num_freqs]; num_ports]; num_ports];

    for excite_port in 0..num_ports {
        let mut excited_netlist = parsed_netlist.clone();
        let port_sources =
            inject_sparameter_port_sources(&mut excited_netlist, config, excite_port)?;
        let engine = Engine::new(build_engine_config(&excited_netlist, None));
        let circuit = engine
            .build_circuit(&excited_netlist)
            .map_err(|e| format!("S-parameter circuit build error: {}", e))?;
        let mut port_branches = Vec::with_capacity(num_ports);
        for port_src in &port_sources {
            let branch = circuit
                .get_branch_by_name(port_src)
                .ok_or_else(|| format!("S-parameter source '{}' branch not found", port_src))?
                as usize;
            port_branches.push(branch);
        }

        let ac_points = engine
            .run_ac(&excited_netlist, &frequencies)
            .map_err(|e| format!("S-parameter AC analysis error: {}", e))?;
        if ac_points.len() != frequencies.len() {
            return Err(format!(
                "S-parameter AC returned {} points for {} requested frequencies",
                ac_points.len(),
                frequencies.len()
            ));
        }

        for (freq_idx, point) in ac_points.iter().enumerate() {
            // AC source branch current sign is opposite to port-current-into-network.
            for (row_port, (branch, port_src)) in
                port_branches.iter().zip(port_sources.iter()).enumerate()
            {
                let current = -branch_current_from_ac(point, *branch).ok_or_else(|| {
                    format!(
                        "S-parameter missing branch current for {} at point {}",
                        port_src, freq_idx
                    )
                })?;
                y[row_port][excite_port][freq_idx] = current;
            }
        }
    }

    let mut s = vec![vec![vec![Complex64::new(0.0, 0.0); num_freqs]; num_ports]; num_ports];
    for freq_idx in 0..num_freqs {
        let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); num_ports]; num_ports];
        for row in 0..num_ports {
            for col in 0..num_ports {
                y_matrix[row][col] = y[row][col][freq_idx];
            }
        }
        let s_matrix = compute_s_from_y_matrix(&y_matrix, &z0_by_port);
        for row in 0..num_ports {
            for col in 0..num_ports {
                s[row][col][freq_idx] = s_matrix[row][col];
            }
        }
    }

    Ok(SParameterData {
        frequencies,
        num_ports,
        s,
        z0: z0_by_port,
    })
}

fn inject_sparameter_port_sources(
    netlist: &mut rspice_core::Netlist,
    config: &SParameterRunConfig,
    excite_port: usize,
) -> Result<Vec<String>, String> {
    if excite_port >= config.ports.len() {
        return Err(format!(
            "S-parameter excite_port {} out of range for {} ports",
            excite_port,
            config.ports.len()
        ));
    }

    let mut port_sources = Vec::with_capacity(config.ports.len());
    for (idx, port) in config.ports.iter().enumerate() {
        let name = unique_aux_element_name(netlist, &format!("__RSPICE_SP_PORT{}", idx + 1));
        let magnitude = if idx == excite_port { 1.0 } else { 0.0 };
        netlist.elements.push(Element {
            name: name.clone(),
            nodes: vec![port.node_pos.clone(), port.node_neg.clone()],
            kind: ElementKind::VoltageSource(SourceSpec::DcAc {
                dc_value: 0.0,
                ac_magnitude: magnitude,
                ac_phase: 0.0,
            }),
        });
        port_sources.push(name);
    }

    Ok(port_sources)
}

fn unique_aux_element_name(netlist: &rspice_core::Netlist, base: &str) -> String {
    let name_exists = |candidate: &str| {
        netlist
            .elements
            .iter()
            .any(|elem| elem.name.eq_ignore_ascii_case(candidate))
    };

    if !name_exists(base) {
        return base.to_string();
    }

    for idx in 1.. {
        let candidate = format!("{}_{}", base, idx);
        if !name_exists(&candidate) {
            return candidate;
        }
    }
    unreachable!("unbounded iterator should always find a unique name");
}

fn branch_current_from_ac(point: &AcResult, branch_ordinal: usize) -> Option<Complex64> {
    let branch_index = branch_ordinal.checked_sub(1)?;
    point.currents.get(branch_index).copied()
}

fn compute_s_from_y_matrix(y: &[Vec<Complex64>], z0_by_port: &[Value]) -> Vec<Vec<Complex64>> {
    let n = y.len();
    if n == 0 || y.iter().any(|row| row.len() != n) {
        return Vec::new();
    }
    if z0_by_port.len() != n {
        return vec![vec![Complex64::new(0.0, 0.0); n]; n];
    }

    let mut a = identity_complex_matrix(n);
    let mut b = identity_complex_matrix(n);
    for row in 0..n {
        let z0 = Complex64::new(z0_by_port[row], 0.0);
        for col in 0..n {
            let zy = z0 * y[row][col];
            a[row][col] += zy;
            b[row][col] -= zy;
        }
    }

    let Some(inv_a) = invert_complex_matrix(&a) else {
        return vec![vec![Complex64::new(0.0, 0.0); n]; n];
    };
    let mut s = multiply_complex_matrix(&b, &inv_a);
    // General multi-port normalization for non-uniform real reference impedances:
    // S = D^(-1) * (I - ZY) * (I + ZY)^(-1) * D, where D = diag(sqrt(Z0_i)).
    for row in 0..n {
        for col in 0..n {
            let scale = (z0_by_port[col] / z0_by_port[row]).sqrt();
            s[row][col] *= Complex64::new(scale, 0.0);
        }
    }
    s
}

fn identity_complex_matrix(size: usize) -> Vec<Vec<Complex64>> {
    let mut matrix = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    for (idx, row) in matrix.iter_mut().enumerate() {
        row[idx] = Complex64::new(1.0, 0.0);
    }
    matrix
}

fn multiply_complex_matrix(lhs: &[Vec<Complex64>], rhs: &[Vec<Complex64>]) -> Vec<Vec<Complex64>> {
    let rows = lhs.len();
    let cols = rhs.first().map_or(0, |row| row.len());
    let inner = rhs.len();
    let mut out = vec![vec![Complex64::new(0.0, 0.0); cols]; rows];
    for row in 0..rows {
        for k in 0..inner {
            let lhs_value = lhs[row][k];
            if lhs_value.norm() <= 1e-30 {
                continue;
            }
            for col in 0..cols {
                out[row][col] += lhs_value * rhs[k][col];
            }
        }
    }
    out
}

fn invert_complex_matrix(matrix: &[Vec<Complex64>]) -> Option<Vec<Vec<Complex64>>> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut augmented = vec![vec![Complex64::new(0.0, 0.0); 2 * n]; n];
    for row in 0..n {
        for col in 0..n {
            augmented[row][col] = matrix[row][col];
        }
        augmented[row][n + row] = Complex64::new(1.0, 0.0);
    }

    for col in 0..n {
        let mut pivot_row = col;
        let mut pivot_norm = augmented[col][col].norm();
        for row in (col + 1)..n {
            let candidate_norm = augmented[row][col].norm();
            if candidate_norm > pivot_norm {
                pivot_norm = candidate_norm;
                pivot_row = row;
            }
        }
        if pivot_norm <= 1e-30 {
            return None;
        }
        if pivot_row != col {
            augmented.swap(pivot_row, col);
        }

        let pivot = augmented[col][col];
        for idx in col..(2 * n) {
            augmented[col][idx] /= pivot;
        }

        let pivot_snapshot = augmented[col].clone();
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = augmented[row][col];
            if factor.norm() <= 1e-30 {
                continue;
            }
            for idx in col..(2 * n) {
                augmented[row][idx] -= factor * pivot_snapshot[idx];
            }
        }
    }

    let mut inverse = vec![vec![Complex64::new(0.0, 0.0); n]; n];
    for row in 0..n {
        for col in 0..n {
            inverse[row][col] = augmented[row][n + col];
        }
    }
    Some(inverse)
}

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

// =============================================================================
// Transfer Function (XF) Analysis
// =============================================================================

/// Frequency sweep type for transfer function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TfFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl TfFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Explicit configuration for transfer-function execution.
#[derive(Debug, Clone)]
pub struct TfRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: TfFrequencySweep,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub group_delay: bool,
    pub input_impedance: bool,
    pub output_impedance: bool,
}

impl Default for TfRunConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: TfFrequencySweep::Decade,
            input_source: "VIN".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: None,
            group_delay: false,
            input_impedance: false,
            output_impedance: false,
        }
    }
}

impl TfRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("TF start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("TF stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("TF points per unit must be greater than zero".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("TF input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("TF output node must be specified".to_string());
        }
        Ok(())
    }
}

/// Transfer-function analysis data.
#[derive(Debug, Clone)]
pub struct TfData {
    /// Frequency points (Hz).
    pub frequencies: Vec<Value>,
    /// Complex transfer function H(jw).
    pub transfer: Vec<Complex64>,
    /// Magnitude response in dB.
    pub magnitude_db: Vec<Value>,
    /// Phase response in degrees.
    pub phase_deg: Vec<Value>,
    /// Group delay curve: (frequency_hz, delay_s).
    pub group_delay: Option<Vec<(Value, Value)>>,
    /// Input impedance vs frequency (Ohms), if requested.
    pub input_impedance: Option<Vec<Complex64>>,
    /// Output impedance vs frequency (Ohms), if requested.
    pub output_impedance: Option<Vec<Complex64>>,
    /// Output trace label (for display).
    pub output_label: String,
    /// Input source name (for display).
    pub input_source: String,
    /// Low-frequency gain magnitude (linear).
    pub dc_gain: Option<Value>,
}

/// Run transfer-function analysis with explicit configuration.
pub fn run_tf_analysis_with_config(
    netlist_text: &str,
    config: &TfRunConfig,
) -> Result<TfData, String> {
    config.validate()?;

    let parsed_netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Build a baseline netlist with all AC source magnitudes forced to zero.
    // We then explicitly excite only the requested input source to keep the
    // transfer denominator deterministic and independent of unrelated sources.
    let mut tf_netlist = parsed_netlist.clone();
    zero_all_source_ac(&mut tf_netlist);
    set_source_ac_excitation(&mut tf_netlist, &config.input_source, 1.0, 0.0)?;

    let engine = Engine::new(build_engine_config(&tf_netlist, None));
    let dc_result = engine
        .run_dc_op(&tf_netlist)
        .map_err(|e| format!("DC OP error (required for TF): {}", e))?;
    let circuit = engine
        .build_circuit(&tf_netlist)
        .map_err(|e| format!("Circuit build error (required for TF): {}", e))?;

    let output_expr =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());
    let output_spec = parse_output_spec(&output_expr, &dc_result.node_names, &circuit)
        .ok_or_else(|| format!("TF output '{}' could not be resolved", output_expr))?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err("TF frequency sweep produced no points".to_string());
    }

    let ac_results = engine
        .run_ac(&tf_netlist, &frequencies)
        .map_err(|e| format!("TF AC analysis error: {}", e))?;
    if ac_results.len() != frequencies.len() {
        return Err(format!(
            "TF AC analysis returned {} points for {} requested frequencies",
            ac_results.len(),
            frequencies.len()
        ));
    }

    let transfer: Vec<Complex64> = ac_results
        .iter()
        .map(|point| ac_output_value(point, &output_spec))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("TF output extraction error: {}", e))?;
    let magnitude_db: Vec<Value> = transfer
        .iter()
        .map(|h| 20.0 * h.norm().max(1e-30).log10())
        .collect();
    let phase_deg: Vec<Value> = transfer.iter().map(|h| h.arg().to_degrees()).collect();

    let input_impedance = if config.input_impedance {
        let branch_ordinal = circuit
            .get_branch_by_name(config.input_source.trim())
            .ok_or_else(|| {
                format!(
                    "TF input source '{}' does not expose an AC branch current; cannot compute Zin",
                    config.input_source
                )
            })? as usize;
        let branch_idx = branch_ordinal.saturating_sub(1);
        Some(
            ac_results
                .iter()
                .map(|point| {
                    let iin = point.currents.get(branch_idx).copied().ok_or_else(|| {
                        format!(
                            "TF input source '{}' branch index {} is unavailable in AC result",
                            config.input_source, branch_idx
                        )
                    })?;
                    if iin.norm() <= 1e-30 {
                        Ok(Complex64::new(f64::INFINITY, 0.0))
                    } else {
                        Ok(Complex64::new(1.0, 0.0) / iin)
                    }
                })
                .collect::<Result<Vec<_>, String>>()?,
        )
    } else {
        None
    };

    let output_impedance = if config.output_impedance {
        let mut zout_netlist = parsed_netlist.clone();
        zero_all_source_ac(&mut zout_netlist);
        inject_tf_output_test_source(
            &mut zout_netlist,
            config.output_node.trim(),
            config.output_ref.as_deref(),
        )?;

        let zout_engine = Engine::new(build_engine_config(&zout_netlist, None));
        let zout_dc = zout_engine
            .run_dc_op(&zout_netlist)
            .map_err(|e| format!("DC OP error (required for TF Zout): {}", e))?;
        let zout_circuit = zout_engine
            .build_circuit(&zout_netlist)
            .map_err(|e| format!("Circuit build error (required for TF Zout): {}", e))?;
        let zout_spec = parse_output_spec(&output_expr, &zout_dc.node_names, &zout_circuit)
            .ok_or_else(|| format!("TF output '{}' could not be resolved for Zout", output_expr))?;

        let zout_points = zout_engine
            .run_ac(&zout_netlist, &frequencies)
            .map_err(|e| format!("TF output-impedance AC analysis error: {}", e))?;
        Some(
            zout_points
                .iter()
                .map(|point| {
                    ac_output_value(point, &zout_spec)
                        .map_err(|e| format!("TF output-impedance extraction error: {}", e))
                })
                .collect::<Result<Vec<_>, String>>()?,
        )
    } else {
        None
    };

    let group_delay = if config.group_delay && frequencies.len() >= 2 {
        use std::f64::consts::PI;
        let mut points = Vec::with_capacity(frequencies.len().saturating_sub(1));
        let mut prev_phase = transfer[0].arg();
        for idx in 1..frequencies.len() {
            let df = frequencies[idx] - frequencies[idx - 1];
            if df <= 0.0 {
                prev_phase = transfer[idx].arg();
                continue;
            }
            let mut phase = transfer[idx].arg();
            while phase - prev_phase > PI {
                phase -= 2.0 * PI;
            }
            while phase - prev_phase < -PI {
                phase += 2.0 * PI;
            }
            let delay = -(phase - prev_phase) / (2.0 * PI * df);
            let mid = (frequencies[idx - 1] + frequencies[idx]) * 0.5;
            points.push((mid, delay));
            prev_phase = phase;
        }
        Some(points)
    } else {
        None
    };

    Ok(TfData {
        dc_gain: transfer.first().map(|h| h.norm()),
        frequencies,
        transfer,
        magnitude_db,
        phase_deg,
        group_delay,
        input_impedance,
        output_impedance,
        output_label: output_expr,
        input_source: config.input_source.clone(),
    })
}

/// Run transfer-function analysis using inferred/default settings.
pub fn run_tf_analysis(netlist_text: &str) -> Result<TfData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for TF defaults): {}", e))?;

    let cfg = infer_tf_run_config(&netlist, &dc_result.node_names)?;
    run_tf_analysis_with_config(netlist_text, &cfg)
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

// =============================================================================
// PAC (Periodic AC) Analysis
// =============================================================================

/// Frequency sweep type for PAC analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PacFrequencySweep {
    fn to_core(self) -> rspice_core::analysis::advanced::pac::PacSweepType {
        match self {
            Self::Decade => rspice_core::analysis::advanced::pac::PacSweepType::Decade,
            Self::Octave => rspice_core::analysis::advanced::pac::PacSweepType::Octave,
            Self::Linear => rspice_core::analysis::advanced::pac::PacSweepType::Linear,
        }
    }
}

/// Explicit configuration for PAC execution.
#[derive(Debug, Clone)]
pub struct PacRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PacFrequencySweep,
    pub max_sideband: i32,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: Value,
    pub include_dc: bool,
    pub reltol: Value,
    pub abstol: Value,
}

impl Default for PacRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: PacFrequencySweep::Decade,
            max_sideband: 5,
            input_source: "VRF".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: None,
            pac_magnitude: 1.0,
            include_dc: true,
            reltol: 1e-3,
            abstol: 1e-12,
        }
    }
}

impl PacRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PAC requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PAC requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PAC requires a positive PSS tolerance".to_string());
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("PAC start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("PAC stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("PAC points per unit must be greater than zero".to_string());
        }
        if self.max_sideband < 0 {
            return Err("PAC max sideband must be non-negative".to_string());
        }
        if self.max_sideband == 0 && !self.include_dc {
            return Err("PAC configuration must include at least one sideband".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("PAC input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("PAC output node must be specified".to_string());
        }
        if !self.pac_magnitude.is_finite() || self.pac_magnitude <= 0.0 {
            return Err("PAC magnitude must be positive".to_string());
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("PAC relative tolerance must be positive".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("PAC absolute tolerance must be positive".to_string());
        }
        Ok(())
    }
}

/// PAC analysis data.
#[derive(Debug, Clone)]
pub struct PacData {
    /// Frequency offsets from carrier in Hz.
    pub frequencies: Vec<Value>,
    /// Sidebands included in spectra.
    pub sidebands: Vec<i32>,
    /// Spectra: (trace_name, [(frequency_offset_hz, magnitude, phase_deg)])
    pub spectra: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Whether solution converged.
    pub converged: bool,
}

struct PacInternalResult {
    pac_result: rspice_core::analysis::advanced::pac::PacResult,
    output_node_idx: usize,
    output_node_name: String,
}

fn run_pac_internal(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
) -> Result<PacInternalResult, String> {
    use rspice_core::analysis::advanced::pac::{PacAnalyzer, PacConfig};
    use rspice_core::analysis::PssConfig;

    config.validate()?;

    let mut sim_config = build_engine_config(netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let engine = Engine::new(sim_config);

    let pss_config = PssConfig::new(config.pss_fundamental_freq)
        .with_harmonics(config.pss_num_harmonics)
        .with_tolerance(config.pss_tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);
    let pss_result = engine
        .run_pss(netlist, pss_config)
        .map_err(|e| format!("PAC prerequisite PSS error: {}", e))?;

    let fundamental = if pss_result.period > 0.0 {
        1.0 / pss_result.period
    } else {
        return Err("PAC prerequisite PSS returned non-positive period".to_string());
    };

    let mut pac_config = PacConfig::new()
        .with_sweep(config.start_freq, config.stop_freq, config.points_per_unit)
        .with_sweep_type(config.sweep.to_core())
        .with_sidebands(-config.max_sideband, config.max_sideband)
        .with_input_source(config.input_source.trim())
        .with_output_node(&normalize_pac_node_name(&config.output_node))
        .with_tolerances(config.reltol, config.abstol)
        .with_dc(config.include_dc)
        .with_fundamental(fundamental);

    if let Some(output_ref) = &config.output_ref {
        let trimmed = output_ref.trim();
        if !trimmed.is_empty() {
            pac_config = pac_config.with_output_ref(trimmed);
        }
    }

    pac_config
        .validate()
        .map_err(|e| format!("PAC configuration error: {}", e))?;

    let mut analyzer = PacAnalyzer::new(pac_config);
    let pac_result = analyzer
        .analyze(
            &pss_result.result,
            pss_result.result.num_nodes(),
            pss_result.result.node_names.clone(),
            Vec::new(),
        )
        .map_err(|e| format!("PAC error: {}", e))?;

    let output_node_idx =
        resolve_pac_output_node(&pac_result, &config.output_node).ok_or_else(|| {
            format!(
                "PAC output node '{}' was not found in PSS result nodes {:?}",
                config.output_node, pac_result.node_names
            )
        })?;
    let output_node_name = pac_result
        .node_names
        .get(output_node_idx)
        .cloned()
        .unwrap_or_else(|| normalize_pac_node_name(&config.output_node));

    Ok(PacInternalResult {
        pac_result,
        output_node_idx,
        output_node_name,
    })
}

/// Run PAC analysis by first solving PSS and then linearizing around the periodic solution.
pub fn run_pac_analysis(netlist_text: &str, config: &PacRunConfig) -> Result<PacData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let pac_internal = run_pac_internal(&netlist, config)?;
    let output_node_idx = pac_internal.output_node_idx;
    let pac_result = pac_internal.pac_result;

    let sidebands: Vec<i32> = pac_result
        .sideband_indices()
        .into_iter()
        .filter(|sb| config.include_dc || *sb != 0)
        .collect();
    if sidebands.is_empty() {
        return Err("PAC produced no sidebands with current configuration".to_string());
    }

    let frequencies = pac_result.frequencies.clone();
    let output_node_name = pac_internal.output_node_name;

    let mut spectra: Vec<(String, Vec<(Value, Value, Value)>)> =
        Vec::with_capacity(sidebands.len());
    for sideband in &sidebands {
        let mut spectrum = Vec::with_capacity(frequencies.len());
        for (freq_idx, freq_offset) in frequencies.iter().copied().enumerate() {
            let voltage = pac_result.voltage(output_node_idx, freq_idx, *sideband)
                * Complex64::new(config.pac_magnitude, 0.0);
            spectrum.push((freq_offset, voltage.norm(), voltage.arg().to_degrees()));
        }
        spectra.push((
            format!("V({})[sb={:+}]", output_node_name, sideband),
            spectrum,
        ));
    }

    Ok(PacData {
        frequencies,
        sidebands,
        spectra,
        converged: true,
    })
}

/// Run PAC analysis using inferred/default settings.
pub fn run_pac_analysis_auto(netlist_text: &str) -> Result<PacData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for PAC defaults): {}", e))?;
    let input_source = infer_primary_source_name(&netlist)
        .ok_or_else(|| "PAC requires at least one independent source in the netlist".to_string())?;
    let output_node = infer_primary_output_node(&dc_result.node_names).ok_or_else(|| {
        "PAC could not infer an output node; ensure at least one non-ground node exists".to_string()
    })?;

    let cfg = PacRunConfig {
        input_source,
        output_node,
        ..PacRunConfig::default()
    };
    run_pac_analysis(netlist_text, &cfg)
}

// =============================================================================
// PXF (Periodic Transfer Function) Analysis
// =============================================================================

/// Frequency sweep type for periodic transfer-function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PxfFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PxfFrequencySweep {
    fn to_core(self) -> rspice_core::analysis::advanced::pxf::PxfSweepType {
        match self {
            Self::Decade => rspice_core::analysis::advanced::pxf::PxfSweepType::Decade,
            Self::Octave => rspice_core::analysis::advanced::pxf::PxfSweepType::Octave,
            Self::Linear => rspice_core::analysis::advanced::pxf::PxfSweepType::Linear,
        }
    }
}

/// Explicit configuration for PXF execution.
#[derive(Debug, Clone)]
pub struct PxfRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PxfFrequencySweep,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: Value,
    pub abstol: Value,
}

impl Default for PxfRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: PxfFrequencySweep::Decade,
            input_source: "VIN".to_string(),
            input_sideband: 1,
            output_node: "VOUT".to_string(),
            output_ref: None,
            output_sideband: 1,
            max_sideband: 5,
            reltol: 1e-3,
            abstol: 1e-12,
        }
    }
}

impl PxfRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PXF requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PXF requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PXF requires a positive PSS tolerance".to_string());
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("PXF start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("PXF stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("PXF points per unit must be greater than zero".to_string());
        }
        if self.max_sideband < 0 {
            return Err("PXF max sideband must be non-negative".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("PXF input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("PXF output node must be specified".to_string());
        }
        if self.input_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF input sideband {} exceeds configured max sideband {}",
                self.input_sideband, self.max_sideband
            ));
        }
        if self.output_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF output sideband {} exceeds configured max sideband {}",
                self.output_sideband, self.max_sideband
            ));
        }
        if let Some(reference) = self
            .output_ref
            .as_deref()
            .map(str::trim)
            .filter(|node| !node.is_empty() && !is_ground_like(node))
        {
            if reference.eq_ignore_ascii_case(self.output_node.trim()) {
                return Err(
                    "PXF output node and output reference cannot be the same node".to_string(),
                );
            }
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("PXF relative tolerance must be positive".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("PXF absolute tolerance must be positive".to_string());
        }
        Ok(())
    }
}

/// PXF analysis data.
#[derive(Debug, Clone)]
pub struct PxfData {
    /// Input frequency sweep points (Hz).
    pub frequencies: Vec<Value>,
    /// Output frequencies for each point (Hz).
    pub output_frequencies: Vec<Value>,
    /// Complex transfer H(f_in -> f_out).
    pub transfer: Vec<Complex64>,
    /// Magnitude in dB.
    pub magnitude_db: Vec<Value>,
    /// Phase in degrees.
    pub phase_deg: Vec<Value>,
    /// Optional group delay curve [(Hz, s)].
    pub group_delay: Option<Vec<(Value, Value)>>,
    /// Input sideband index.
    pub input_sideband: i32,
    /// Output sideband index.
    pub output_sideband: i32,
    /// Fundamental carrier frequency from PSS (Hz).
    pub fundamental_frequency: Value,
    /// Output label.
    pub output_label: String,
    /// Optional DC transfer gain.
    pub dc_gain: Option<Complex64>,
    /// Optional peak gain metric (frequency, gain_db).
    pub peak_gain: Option<(Value, Value)>,
    /// Optional 3 dB bandwidth.
    pub bandwidth_3db: Option<Value>,
    /// Optional unity-gain frequency.
    pub unity_gain_freq: Option<Value>,
    /// Non-fatal caveats for approximations/fallbacks.
    pub warnings: Vec<String>,
}

/// Run PXF analysis with explicit configuration.
pub fn run_pxf_analysis_with_config(
    netlist_text: &str,
    config: &PxfRunConfig,
) -> Result<PxfData, String> {
    use rspice_core::analysis::advanced::pxf::{PxfAnalyzer, PxfConfig};

    config.validate()?;

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let required_sideband = config
        .input_sideband
        .abs()
        .max(config.output_sideband.abs())
        .max(config.max_sideband);
    let pac_cfg = PacRunConfig {
        pss_fundamental_freq: config.pss_fundamental_freq,
        pss_num_harmonics: config.pss_num_harmonics,
        pss_tolerance: config.pss_tolerance,
        start_freq: config.start_freq,
        stop_freq: config.stop_freq,
        points_per_unit: config.points_per_unit,
        sweep: match config.sweep {
            PxfFrequencySweep::Decade => PacFrequencySweep::Decade,
            PxfFrequencySweep::Octave => PacFrequencySweep::Octave,
            PxfFrequencySweep::Linear => PacFrequencySweep::Linear,
        },
        max_sideband: required_sideband,
        input_source: config.input_source.clone(),
        output_node: config.output_node.clone(),
        output_ref: config.output_ref.clone(),
        pac_magnitude: 1.0,
        include_dc: true,
        reltol: config.reltol,
        abstol: config.abstol,
    };

    let pac_internal = run_pac_internal(&netlist, &pac_cfg)?;
    let pac_result = pac_internal.pac_result;

    let sideband_indices = pac_result.conversion_matrix.sideband_indices();
    if !sideband_indices.contains(&config.input_sideband) {
        return Err(format!(
            "PXF input sideband {} is outside analyzed PAC sideband range {:?}",
            config.input_sideband, sideband_indices
        ));
    }
    if !sideband_indices.contains(&config.output_sideband) {
        return Err(format!(
            "PXF output sideband {} is outside analyzed PAC sideband range {:?}",
            config.output_sideband, sideband_indices
        ));
    }

    let frequencies = pac_result.conversion_matrix.frequencies().to_vec();
    if frequencies.is_empty() {
        return Err("PXF conversion matrix has no frequency points".to_string());
    }

    let mut matrix_cube: Vec<Vec<Vec<Complex64>>> = Vec::with_capacity(frequencies.len());
    for freq_idx in 0..frequencies.len() {
        let mut out_rows = Vec::with_capacity(sideband_indices.len());
        for out_sb in &sideband_indices {
            let mut row = Vec::with_capacity(sideband_indices.len());
            for in_sb in &sideband_indices {
                row.push(pac_result.conversion_matrix.get(freq_idx, *out_sb, *in_sb));
            }
            out_rows.push(row);
        }
        matrix_cube.push(out_rows);
    }

    let mut pxf_cfg = PxfConfig::new()
        .with_sweep(config.start_freq, config.stop_freq, config.points_per_unit)
        .with_sweep_type(config.sweep.to_core())
        .with_sidebands(config.input_sideband, config.output_sideband)
        .with_input(config.input_source.trim())
        .with_output(&normalize_pac_node_name(&config.output_node))
        .with_fundamental(pac_result.fundamental_frequency);
    pxf_cfg.max_sidebands = required_sideband as usize;
    if let Some(reference) = config.output_ref.as_deref() {
        let trimmed = reference.trim();
        if !trimmed.is_empty() {
            pxf_cfg.ref_node = trimmed.to_string();
        }
    }
    pxf_cfg
        .validate()
        .map_err(|e| format!("PXF configuration error: {}", e))?;

    let analyzer = PxfAnalyzer::new(pxf_cfg);
    let pxf_result = analyzer
        .analyze_from_conversion_matrix(
            &frequencies,
            &matrix_cube,
            pac_result.fundamental_frequency,
        )
        .map_err(|e| format!("PXF error: {}", e))?;

    if pxf_result.points.is_empty() {
        return Err("PXF produced no transfer points".to_string());
    }

    let sweep_freqs: Vec<Value> = pxf_result
        .points
        .iter()
        .map(|point| point.freq_in)
        .collect();
    let output_freqs: Vec<Value> = pxf_result
        .points
        .iter()
        .map(|point| point.freq_out)
        .collect();
    let transfer: Vec<Complex64> = pxf_result
        .points
        .iter()
        .map(|point| point.transfer)
        .collect();
    let magnitude_db: Vec<Value> = pxf_result
        .points
        .iter()
        .map(|point| point.magnitude_db())
        .collect();
    let phase_deg: Vec<Value> = pxf_result
        .points
        .iter()
        .map(|point| point.phase_degrees())
        .collect();

    let group_delay_curve = pxf_result.group_delay_curve();
    let group_delay = (!group_delay_curve.is_empty()).then_some(group_delay_curve);

    let output_label =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());

    Ok(PxfData {
        frequencies: sweep_freqs,
        output_frequencies: output_freqs,
        transfer,
        magnitude_db,
        phase_deg,
        group_delay,
        input_sideband: pxf_result.input_sideband,
        output_sideband: pxf_result.output_sideband,
        fundamental_frequency: pxf_result.fundamental_freq,
        output_label,
        dc_gain: pxf_result.dc_gain,
        peak_gain: pxf_result.peak_gain,
        bandwidth_3db: pxf_result.bandwidth_3db,
        unity_gain_freq: pxf_result.unity_gain_freq,
        warnings: Vec::new(),
    })
}

/// Run PXF analysis using inferred/default settings.
pub fn run_pxf_analysis(netlist_text: &str) -> Result<PxfData, String> {
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for PXF defaults): {}", e))?;
    let input_source = infer_primary_source_name(&netlist)
        .ok_or_else(|| "PXF requires at least one independent source in the netlist".to_string())?;
    let output_node = infer_primary_output_node(&dc_result.node_names).ok_or_else(|| {
        "PXF could not infer an output node; ensure at least one non-ground node exists".to_string()
    })?;

    let cfg = PxfRunConfig {
        input_source,
        output_node,
        ..PxfRunConfig::default()
    };
    run_pxf_analysis_with_config(netlist_text, &cfg)
}

fn normalize_pac_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && trimmed
            .get(0..2)
            .map(|prefix| prefix.eq_ignore_ascii_case("V("))
            .unwrap_or(false)
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_string();
    }
    trimmed.to_string()
}

fn resolve_pac_output_node(
    result: &rspice_core::analysis::advanced::pac::PacResult,
    requested: &str,
) -> Option<usize> {
    let trimmed = requested.trim();
    if trimmed.is_empty() {
        return None;
    }
    result
        .node_index(trimmed)
        .or_else(|| result.node_index(&normalize_pac_node_name(trimmed)))
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
// Optimization Analysis
// =============================================================================

/// Optimization objective strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationGoalMode {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach target value.
    Target,
}

/// Optimization algorithm mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationAlgorithmMode {
    /// Gradient-descent algorithm.
    GradientDescent,
    /// Pattern-search algorithm.
    PatternSearch,
    /// Simulated-annealing algorithm.
    SimulatedAnnealing,
}

/// Single optimization variable.
#[derive(Debug, Clone)]
pub struct OptimizationVariable {
    /// Parameter name (`.param <name>=...`).
    pub name: String,
    /// Lower bound.
    pub min: Value,
    /// Upper bound.
    pub max: Value,
    /// Initial value.
    pub initial: Value,
}

/// Optimization run configuration.
#[derive(Debug, Clone)]
pub struct OptimizationRunConfig {
    /// Optimization variables.
    pub variables: Vec<OptimizationVariable>,
    /// Objective node.
    pub objective_node: String,
    /// Objective reference node.
    pub objective_ref: String,
    /// Goal mode.
    pub goal: OptimizationGoalMode,
    /// Optional goal target value.
    pub target: Option<Value>,
    /// Algorithm selection.
    pub algorithm: OptimizationAlgorithmMode,
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Cost tolerance.
    pub cost_tolerance: Value,
    /// Finite difference relative step.
    pub fd_step: Value,
    /// Initial step size.
    pub initial_step: Value,
    /// Minimum step size.
    pub min_step: Value,
}

impl Default for OptimizationRunConfig {
    fn default() -> Self {
        Self {
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 500.0,
                max: 5000.0,
                initial: 1000.0,
            }],
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: OptimizationGoalMode::Target,
            target: Some(1.2),
            algorithm: OptimizationAlgorithmMode::PatternSearch,
            max_iterations: 120,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.1,
            min_step: 1e-8,
        }
    }
}

impl OptimizationRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.variables.is_empty() {
            return Err("Optimization requires at least one variable".to_string());
        }
        if self.objective_node.trim().is_empty() {
            return Err("Optimization objective_node must not be empty".to_string());
        }
        if self.objective_ref.trim().is_empty() {
            return Err("Optimization objective_ref must not be empty".to_string());
        }
        if self
            .objective_node
            .eq_ignore_ascii_case(&self.objective_ref)
        {
            return Err("Optimization objective_node and objective_ref must differ".to_string());
        }
        if self.max_iterations == 0 {
            return Err("Optimization max_iterations must be > 0".to_string());
        }
        if !self.cost_tolerance.is_finite() || self.cost_tolerance <= 0.0 {
            return Err("Optimization cost_tolerance must be finite and > 0".to_string());
        }
        if !self.fd_step.is_finite() || self.fd_step <= 0.0 {
            return Err("Optimization fd_step must be finite and > 0".to_string());
        }
        if !self.initial_step.is_finite() || self.initial_step <= 0.0 {
            return Err("Optimization initial_step must be finite and > 0".to_string());
        }
        if !self.min_step.is_finite() || self.min_step <= 0.0 {
            return Err("Optimization min_step must be finite and > 0".to_string());
        }
        if self.min_step > self.initial_step {
            return Err("Optimization min_step must be <= initial_step".to_string());
        }
        if self.goal == OptimizationGoalMode::Target {
            if self.target.is_none() || self.target.is_some_and(|v| !v.is_finite()) {
                return Err("Optimization target goal requires a finite target value".to_string());
            }
        } else if self.target.is_some_and(|v| !v.is_finite()) {
            return Err("Optimization target must be finite when provided".to_string());
        }

        let mut seen = std::collections::HashSet::new();
        for var in &self.variables {
            if !is_valid_param_identifier(&var.name) {
                return Err(format!(
                    "Invalid optimization variable name '{}': expected [A-Za-z_][A-Za-z0-9_]*",
                    var.name
                ));
            }
            if !var.min.is_finite() || !var.max.is_finite() || !var.initial.is_finite() {
                return Err(format!(
                    "Optimization variable '{}' bounds/initial must be finite",
                    var.name
                ));
            }
            if var.max <= var.min {
                return Err(format!(
                    "Optimization variable '{}' requires max > min",
                    var.name
                ));
            }
            if var.initial < var.min || var.initial > var.max {
                return Err(format!(
                    "Optimization variable '{}' initial must be within [{}, {}]",
                    var.name, var.min, var.max
                ));
            }
            if !seen.insert(var.name.to_ascii_uppercase()) {
                return Err(format!(
                    "Optimization variable '{}' is defined more than once",
                    var.name
                ));
            }
        }
        Ok(())
    }
}

/// Optimization output data.
#[derive(Debug, Clone)]
pub struct OptimizationData {
    /// Iteration axis points.
    pub iterations: Vec<Value>,
    /// Cost history.
    pub costs: Vec<Value>,
    /// Variable traces by name.
    pub variable_traces: std::collections::HashMap<String, Vec<Value>>,
    /// Best cost reached.
    pub best_cost: Value,
    /// Best variable values.
    pub best_variables: std::collections::HashMap<String, Value>,
    /// Whether convergence criterion was met.
    pub converged: bool,
}

/// Run optimization analysis with default configuration.
pub fn run_optimization_analysis(netlist_text: &str) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_config(netlist_text, &OptimizationRunConfig::default())
}

/// Run optimization analysis with explicit configuration.
pub fn run_optimization_analysis_with_config(
    netlist_text: &str,
    config: &OptimizationRunConfig,
) -> Result<OptimizationData, String> {
    config.validate()?;

    let mut optimizer_config = OptimizerConfig::default();
    optimizer_config.algorithm = match config.algorithm {
        OptimizationAlgorithmMode::GradientDescent => OptimizerAlgo::GradientDescent,
        OptimizationAlgorithmMode::PatternSearch => OptimizerAlgo::PatternSearch,
        OptimizationAlgorithmMode::SimulatedAnnealing => OptimizerAlgo::SimulatedAnnealing,
    };
    optimizer_config.max_iterations = config.max_iterations;
    optimizer_config.cost_tolerance = config.cost_tolerance;
    optimizer_config.fd_step = config.fd_step;
    optimizer_config.initial_step = config.initial_step;
    optimizer_config.min_step = config.min_step;

    let mut optimizer = OptimizerEngine::with_config(optimizer_config);
    for var in &config.variables {
        optimizer.add_var(DesignVar::new(
            var.name.clone(),
            var.initial,
            var.min,
            var.max,
        ));
    }
    let mut synthetic_goal = match config.goal {
        OptimizationGoalMode::Minimize => OptimizationGoal::minimize("__objective"),
        OptimizationGoalMode::Maximize => OptimizationGoal::maximize("__objective"),
        OptimizationGoalMode::Target => {
            OptimizationGoal::hit_target("__objective", config.target.unwrap_or_default())
        }
    };
    synthetic_goal.weight = 1.0;
    optimizer.add_goal(synthetic_goal);

    let mut variable_traces: std::collections::HashMap<String, Vec<Value>> =
        std::collections::HashMap::new();
    for var in &config.variables {
        variable_traces.insert(
            var.name.clone(),
            Vec::with_capacity(config.max_iterations + 1),
        );
    }
    let mut iterations = Vec::with_capacity(config.max_iterations + 1);
    let mut costs = Vec::with_capacity(config.max_iterations + 1);

    let mut eval_error: Option<String> = None;
    let mut successful_evals: usize = 0;
    let mut cost_fn = |vars: &std::collections::HashMap<String, Value>| -> Value {
        match evaluate_optimization_objective(netlist_text, vars, config) {
            Ok(value) => {
                successful_evals += 1;
                objective_to_cost(value, config.goal, config.target)
            }
            Err(err) => {
                if eval_error.is_none() {
                    eval_error = Some(err);
                }
                1e30
            }
        }
    };

    let mut record_state =
        |iter: Value, vars: &std::collections::HashMap<String, Value>, cost: Value| {
            iterations.push(iter);
            costs.push(cost);
            for (name, trace) in &mut variable_traces {
                trace.push(vars.get(name).copied().unwrap_or(0.0));
            }
        };

    let initial_vars = optimizer.current_vars();
    let initial_cost = cost_fn(&initial_vars);
    record_state(0.0, &initial_vars, initial_cost);

    while optimizer.current_iteration() < config.max_iterations {
        optimizer.step(&mut cost_fn);
        let vars = optimizer.current_vars();
        let cost = cost_fn(&vars);
        record_state(optimizer.current_iteration() as Value, &vars, cost);
        if optimizer.is_converged() {
            break;
        }
    }

    if successful_evals == 0 {
        return Err(eval_error.unwrap_or_else(|| {
            "Optimization failed: objective evaluation returned no valid samples".to_string()
        }));
    }

    let (best_vars, best_cost) = optimizer.best_result();
    Ok(OptimizationData {
        iterations,
        costs,
        variable_traces,
        best_cost,
        best_variables: best_vars.clone(),
        converged: optimizer.is_converged(),
    })
}

fn objective_to_cost(objective: Value, goal: OptimizationGoalMode, target: Option<Value>) -> Value {
    match goal {
        OptimizationGoalMode::Minimize => objective.abs(),
        OptimizationGoalMode::Maximize => {
            if objective > 0.0 {
                1.0 / objective
            } else {
                1e30
            }
        }
        OptimizationGoalMode::Target => {
            let t = target.unwrap_or_default();
            (objective - t).powi(2)
        }
    }
}

fn evaluate_optimization_objective(
    netlist_text: &str,
    vars: &std::collections::HashMap<String, Value>,
    config: &OptimizationRunConfig,
) -> Result<Value, String> {
    let overridden = inject_param_overrides(netlist_text, vars);
    let netlist = rspice_core::netlist::parse_netlist(&overridden)
        .map_err(|e| format!("Parse error during optimization: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC operating point failed during optimization: {}", e))?;

    let node_idx = resolve_node_index_case_insensitive(&dc.node_names, &config.objective_node)
        .ok_or_else(|| {
            format!(
                "Optimization objective node '{}' not found",
                config.objective_node
            )
        })?;
    let ref_idx = if is_ground_like(&config.objective_ref) {
        Some(0usize)
    } else {
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_ref)
    }
    .ok_or_else(|| {
        format!(
            "Optimization objective reference node '{}' not found",
            config.objective_ref
        )
    })?;

    let node_v = *dc
        .node_voltages
        .get(node_idx)
        .ok_or_else(|| "Optimization node voltage index out of bounds".to_string())?;
    let ref_v = *dc
        .node_voltages
        .get(ref_idx)
        .ok_or_else(|| "Optimization reference voltage index out of bounds".to_string())?;
    Ok(node_v - ref_v)
}

fn inject_param_overrides(
    netlist_text: &str,
    vars: &std::collections::HashMap<String, Value>,
) -> String {
    if vars.is_empty() {
        return netlist_text.to_string();
    }

    let mut entries: Vec<(String, String, Value)> = vars
        .iter()
        .filter_map(|(name, value)| {
            if is_valid_param_identifier(name) {
                Some((name.to_ascii_uppercase(), name.clone(), *value))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    if entries.is_empty() {
        return netlist_text.to_string();
    }

    let mut lines: Vec<String> = netlist_text.lines().map(str::to_string).collect();
    if lines.is_empty() {
        let mut line = ".param".to_string();
        for (_, name, value) in &entries {
            line.push(' ');
            line.push_str(name);
            line.push('=');
            line.push_str(&format_param_override_value(*value));
        }
        return format!("{}\n", line);
    }

    let mut overrides_found: std::collections::HashSet<String> = std::collections::HashSet::new();

    for line in lines.iter_mut().skip(1) {
        if !is_param_directive_line(line) {
            continue;
        }

        let assigned = collect_param_assignment_names(line);
        let mut append_parts = Vec::new();
        for (upper, name, value) in &entries {
            if assigned.contains(upper) {
                overrides_found.insert(upper.clone());
                append_parts.push(format!("{}={}", name, format_param_override_value(*value)));
            }
        }

        if !append_parts.is_empty() {
            let suffix = append_parts.join(" ");
            if let Some(comment_idx) = line.find(';') {
                let (head, comment) = line.split_at(comment_idx);
                let mut rebuilt = head.trim_end().to_string();
                rebuilt.push(' ');
                rebuilt.push_str(&suffix);
                rebuilt.push(' ');
                rebuilt.push_str(comment.trim_start());
                *line = rebuilt;
            } else {
                line.push(' ');
                line.push_str(&suffix);
            }
        }
    }

    let missing: Vec<(String, Value)> = entries
        .iter()
        .filter(|(upper, _, _)| !overrides_found.contains(upper))
        .map(|(_, name, value)| (name.clone(), *value))
        .collect();

    if !missing.is_empty() {
        let mut line = ".param".to_string();
        for (name, value) in &missing {
            line.push(' ');
            line.push_str(name);
            line.push('=');
            line.push_str(&format_param_override_value(*value));
        }
        lines.insert(1, line);
    }

    let mut out = lines.join("\n");
    if netlist_text.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn format_param_override_value(value: Value) -> String {
    let raw = format!("{:.16e}", value);
    let Some(exp_pos) = raw.find('e') else {
        return raw;
    };
    let mantissa = &raw[..exp_pos];
    let exponent = &raw[exp_pos + 1..];
    match exponent.parse::<i32>() {
        Ok(exp) => format!("{}e{:+03}", mantissa, exp),
        Err(_) => raw,
    }
}

fn is_param_directive_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".param"))
    {
        return false;
    }
    trimmed
        .as_bytes()
        .get(6)
        .is_none_or(|ch| ch.is_ascii_whitespace())
}

fn collect_param_assignment_names(line: &str) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    let trimmed = line.trim_start();
    if !trimmed
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".param"))
    {
        return names;
    }
    let rest = trimmed[6..].split(';').next().unwrap_or("").trim();
    let bytes = rest.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;

    while i < len {
        while i < len && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
        if i >= len {
            break;
        }

        let start = i;
        let first = bytes[i];
        if !(first.is_ascii_alphabetic() || first == b'_') {
            i += 1;
            continue;
        }
        i += 1;
        while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        let name = &rest[start..i];

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < len && bytes[i] == b'=' {
            names.insert(name.to_ascii_uppercase());
        }
    }

    names
}

fn resolve_node_index_case_insensitive(node_names: &[String], target: &str) -> Option<usize> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return None;
    }
    node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(trimmed))
}

fn is_valid_param_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

// =============================================================================
// Safety / SOA Analysis
// =============================================================================

/// Configuration for SOA analysis.
#[derive(Debug, Clone)]
pub struct SoaRunConfig {
    /// Transient stop time.
    pub stop_time: Value,
    /// Transient step time.
    pub step_time: Value,
    /// Enable Vgs limit checks.
    pub check_vgs_max: bool,
    /// Maximum allowed Vgs magnitude.
    pub max_vgs: Value,
    /// Enable Vds limit checks.
    pub check_vds_max: bool,
    /// Maximum allowed Vds magnitude.
    pub max_vds: Value,
    /// Enable Vbe limit checks.
    pub check_vbe_max: bool,
    /// Maximum allowed Vbe magnitude.
    pub max_vbe: Value,
    /// Enable Vce limit checks.
    pub check_vce_max: bool,
    /// Maximum allowed Vce magnitude.
    pub max_vce: Value,
}

impl Default for SoaRunConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            step_time: 1e-9,
            check_vgs_max: true,
            max_vgs: 1.8,
            check_vds_max: true,
            max_vds: 3.3,
            check_vbe_max: true,
            max_vbe: 0.9,
            check_vce_max: true,
            max_vce: 5.0,
        }
    }
}

impl SoaRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("SOA stop_time must be finite and > 0".to_string());
        }
        if !self.step_time.is_finite() || self.step_time <= 0.0 {
            return Err("SOA step_time must be finite and > 0".to_string());
        }
        if self.step_time > self.stop_time {
            return Err("SOA step_time must be <= stop_time".to_string());
        }
        if !self.check_vgs_max && !self.check_vds_max && !self.check_vbe_max && !self.check_vce_max
        {
            return Err("SOA requires at least one enabled check".to_string());
        }
        if self.check_vgs_max && (!self.max_vgs.is_finite() || self.max_vgs <= 0.0) {
            return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
        }
        if self.check_vds_max && (!self.max_vds.is_finite() || self.max_vds <= 0.0) {
            return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
        }
        if self.check_vbe_max && (!self.max_vbe.is_finite() || self.max_vbe <= 0.0) {
            return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
        }
        if self.check_vce_max && (!self.max_vce.is_finite() || self.max_vce <= 0.0) {
            return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
        }
        Ok(())
    }
}

/// SOA analysis output.
#[derive(Debug, Clone)]
pub struct SoaData {
    /// Transient time vector.
    pub time: Vec<Value>,
    /// Cumulative violation count over time.
    pub violation_count: Vec<Value>,
    /// Collected violations.
    pub violations: Vec<SoAViolation>,
}

/// Run SOA analysis using default configuration.
pub fn run_soa_analysis(netlist_text: &str) -> Result<SoaData, String> {
    run_soa_analysis_with_config(netlist_text, &SoaRunConfig::default())
}

/// Run SOA analysis using explicit configuration.
pub fn run_soa_analysis_with_config(
    netlist_text: &str,
    config: &SoaRunConfig,
) -> Result<SoaData, String> {
    config.validate()?;

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let transient = run_transient_analysis(netlist_text, config.stop_time, config.step_time)?;

    let mut manager = SoAManager::new();
    register_soa_limits_for_netlist(&mut manager, &netlist.elements, config);
    if manager.violations().is_empty() && netlist.elements.is_empty() {
        return Err("SOA analysis received an empty netlist".to_string());
    }

    let mut active_devices = 0usize;
    for element in &netlist.elements {
        if is_soa_supported_element(&element.kind) {
            active_devices += 1;
        }
    }
    if active_devices == 0 {
        return Err("SOA analysis found no supported semiconductor devices".to_string());
    }

    let node_waveforms = build_transient_node_lookup(&transient.voltages);
    let mut violation_count = Vec::with_capacity(transient.time.len());

    for (idx, &time) in transient.time.iter().enumerate() {
        let mut values: std::collections::HashMap<
            String,
            std::collections::HashMap<SoAParameter, Value>,
        > = std::collections::HashMap::new();

        for element in &netlist.elements {
            match &element.kind {
                ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. } => {
                    if element.nodes.len() < 3 {
                        continue;
                    }
                    let vd = sample_node_waveform(&node_waveforms, &element.nodes[0], idx);
                    let vg = sample_node_waveform(&node_waveforms, &element.nodes[1], idx);
                    let vs = sample_node_waveform(&node_waveforms, &element.nodes[2], idx);
                    let mut device_values = std::collections::HashMap::new();
                    if config.check_vgs_max {
                        device_values.insert(SoAParameter::Vgs, (vg - vs).abs());
                    }
                    if config.check_vds_max {
                        device_values.insert(SoAParameter::Vds, (vd - vs).abs());
                    }
                    if !device_values.is_empty() {
                        values.insert(element.name.clone(), device_values);
                    }
                }
                ElementKind::Bjt { .. } => {
                    if element.nodes.len() < 3 {
                        continue;
                    }
                    let vc = sample_node_waveform(&node_waveforms, &element.nodes[0], idx);
                    let vb = sample_node_waveform(&node_waveforms, &element.nodes[1], idx);
                    let ve = sample_node_waveform(&node_waveforms, &element.nodes[2], idx);
                    let mut device_values = std::collections::HashMap::new();
                    if config.check_vbe_max {
                        device_values.insert(SoAParameter::Vbe, (vb - ve).abs());
                    }
                    if config.check_vce_max {
                        device_values.insert(SoAParameter::Vce, (vc - ve).abs());
                    }
                    if !device_values.is_empty() {
                        values.insert(element.name.clone(), device_values);
                    }
                }
                _ => {}
            }
        }

        manager.check_point(time, &values);
        violation_count.push(manager.violations().len() as Value);
    }

    Ok(SoaData {
        time: transient.time,
        violation_count,
        violations: manager.violations().to_vec(),
    })
}

fn is_soa_supported_element(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::Mosfet { .. }
            | ElementKind::Jfet { .. }
            | ElementKind::Mesfet { .. }
            | ElementKind::Bjt { .. }
    )
}

fn register_soa_limits_for_netlist(
    manager: &mut SoAManager,
    elements: &[Element],
    config: &SoaRunConfig,
) {
    for element in elements {
        let mut def = SoADefinition::new();
        match &element.kind {
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                if config.check_vgs_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vgs,
                        max_value: config.max_vgs,
                        min_value: None,
                        max_duration: None,
                        unit: "V".to_string(),
                        description: "Maximum gate-source voltage".to_string(),
                    });
                }
                if config.check_vds_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vds,
                        max_value: config.max_vds,
                        min_value: None,
                        max_duration: None,
                        unit: "V".to_string(),
                        description: "Maximum drain-source voltage".to_string(),
                    });
                }
            }
            ElementKind::Bjt { .. } => {
                if config.check_vbe_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vbe,
                        max_value: config.max_vbe,
                        min_value: None,
                        max_duration: None,
                        unit: "V".to_string(),
                        description: "Maximum base-emitter voltage".to_string(),
                    });
                }
                if config.check_vce_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vce,
                        max_value: config.max_vce,
                        min_value: None,
                        max_duration: None,
                        unit: "V".to_string(),
                        description: "Maximum collector-emitter voltage".to_string(),
                    });
                }
            }
            _ => continue,
        }
        if !def.limits.is_empty() {
            manager.register_device(element.name.clone(), def);
        }
    }
}

fn build_transient_node_lookup(
    voltages: &[(String, Vec<Value>)],
) -> std::collections::HashMap<String, Vec<Value>> {
    let mut map = std::collections::HashMap::with_capacity(voltages.len() + 2);
    for (name, values) in voltages {
        map.insert(normalize_voltage_signal_name(name), values.clone());
    }
    map.insert("0".to_string(), Vec::new());
    map.insert("GND".to_string(), Vec::new());
    map
}

fn sample_node_waveform(
    waveforms: &std::collections::HashMap<String, Vec<Value>>,
    node_name: &str,
    idx: usize,
) -> Value {
    if is_ground_like(node_name) {
        return 0.0;
    }
    let key = node_name.trim().to_ascii_uppercase();
    waveforms
        .get(&key)
        .and_then(|values| values.get(idx).copied())
        .unwrap_or(0.0)
}

// =============================================================================
// Reliability (Aging) Analysis
// =============================================================================

/// Explicit configuration for reliability analysis.
#[derive(Debug, Clone)]
pub struct ReliabilityRunConfig {
    /// Lifetime checkpoints to evaluate (years).
    pub target_years: Vec<Value>,
    /// Enable HCI contribution.
    pub enable_hci: bool,
    /// Enable NBTI contribution.
    pub enable_nbti: bool,
    /// Enable electromigration contribution.
    pub enable_em: bool,
    /// Minimum stress magnitude to include a device.
    pub min_stress_voltage: Value,
}

impl Default for ReliabilityRunConfig {
    fn default() -> Self {
        Self {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.1,
        }
    }
}

impl ReliabilityRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.target_years.is_empty() {
            return Err("Reliability target years must not be empty".to_string());
        }
        if self
            .target_years
            .iter()
            .any(|years| !years.is_finite() || *years <= 0.0)
        {
            return Err("Reliability target years must be finite and > 0".to_string());
        }
        if !self.enable_hci && !self.enable_nbti && !self.enable_em {
            return Err("Reliability requires at least one enabled mechanism".to_string());
        }
        if !self.min_stress_voltage.is_finite() || self.min_stress_voltage < 0.0 {
            return Err("Reliability min stress voltage must be finite and >= 0".to_string());
        }
        Ok(())
    }
}

/// Reliability analysis output.
#[derive(Debug, Clone)]
pub struct ReliabilityData {
    /// Evaluated lifetime checkpoints (years).
    pub years: Vec<Value>,
    /// Per-device reliability results.
    pub device_results: Vec<ReliabilityResult>,
}

/// Run reliability analysis with default configuration.
pub fn run_reliability_analysis(netlist_text: &str) -> Result<ReliabilityData, String> {
    run_reliability_analysis_with_config(netlist_text, &ReliabilityRunConfig::default())
}

/// Run reliability analysis using explicit configuration.
pub fn run_reliability_analysis_with_config(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
) -> Result<ReliabilityData, String> {
    config.validate()?;

    let mut years = config.target_years.clone();
    years.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    years.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let sim_config = build_engine_config(&netlist, None);
    let temperature_k = sim_config.temperature;
    let engine = Engine::new(sim_config);
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC operating point error: {}", e))?;

    let node_voltages = build_node_voltage_lookup(&dc_result);
    let stress_data = extract_reliability_stress_data(
        &netlist.elements,
        &node_voltages,
        temperature_k,
        config.min_stress_voltage,
    );
    if stress_data.is_empty() {
        return Err(
            "Reliability analysis found no stressed semiconductor devices in the circuit"
                .to_string(),
        );
    }

    let reliability_engine = ReliabilityEngine::new();
    let mut device_results = reliability_engine.analyze_circuit(&stress_data, &years);
    apply_reliability_mechanism_scaling(&mut device_results, config);
    device_results.sort_by_cached_key(|result| result.device_id.to_ascii_uppercase());

    Ok(ReliabilityData {
        years,
        device_results,
    })
}

fn build_node_voltage_lookup(
    dc_result: &CoreSimulationResult,
) -> std::collections::HashMap<String, Value> {
    let mut lookup = std::collections::HashMap::new();
    for (idx, node_name) in dc_result.node_names.iter().enumerate() {
        if let Some(voltage) = dc_result.node_voltages.get(idx) {
            lookup.insert(node_name.clone(), *voltage);
            lookup.insert(node_name.to_ascii_uppercase(), *voltage);
        }
    }
    lookup.insert("0".to_string(), 0.0);
    lookup.insert("GND".to_string(), 0.0);
    lookup
}

fn resolve_node_voltage(
    node_voltages: &std::collections::HashMap<String, Value>,
    node_name: &str,
) -> Value {
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return 0.0;
    }
    if trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
        return 0.0;
    }
    node_voltages
        .get(trimmed)
        .copied()
        .or_else(|| node_voltages.get(&trimmed.to_ascii_uppercase()).copied())
        .unwrap_or(0.0)
}

fn extract_reliability_stress_data(
    elements: &[Element],
    node_voltages: &std::collections::HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
) -> std::collections::HashMap<String, StressMetrics> {
    let mut stress_data = std::collections::HashMap::new();
    let min_stress = min_stress_voltage.max(0.0);

    for element in elements {
        let stress_pair = match &element.kind {
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vd = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vg = resolve_node_voltage(node_voltages, &element.nodes[1]);
                    let vs = resolve_node_voltage(node_voltages, &element.nodes[2]);
                    Some(((vg - vs).abs(), (vd - vs).abs()))
                }
            }
            ElementKind::Bjt { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vc = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vb = resolve_node_voltage(node_voltages, &element.nodes[1]);
                    let ve = resolve_node_voltage(node_voltages, &element.nodes[2]);
                    Some(((vb - ve).abs(), (vc - ve).abs()))
                }
            }
            ElementKind::Diode { .. } => {
                if element.nodes.len() < 2 {
                    None
                } else {
                    let va = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vk = resolve_node_voltage(node_voltages, &element.nodes[1]);
                    let vak = (va - vk).abs();
                    Some((vak, vak))
                }
            }
            _ => None,
        };

        let Some((avg_vgs_stress, avg_vds_stress)) = stress_pair else {
            continue;
        };
        if avg_vgs_stress.max(avg_vds_stress) < min_stress {
            continue;
        }

        stress_data.insert(
            element.name.clone(),
            StressMetrics {
                avg_vgs_stress,
                avg_vds_stress,
                avg_temp: temperature_k,
                duration: 3600.0,
            },
        );
    }

    stress_data
}

fn apply_reliability_mechanism_scaling(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
) {
    let mut vth_factor = 0.0;
    let mut mobility_factor = 0.0;
    let mut rds_factor = 0.0;

    if config.enable_hci {
        vth_factor += 1.0;
        mobility_factor += 1.0;
        rds_factor += 0.8;
    }
    if config.enable_nbti {
        vth_factor += 0.85;
        mobility_factor += 0.65;
        rds_factor += 0.4;
    }
    if config.enable_em {
        rds_factor += 2.2;
    }

    for result in results {
        for shift in result.shifts.values_mut() {
            apply_shift_factors(shift, vth_factor, mobility_factor, rds_factor);
        }
    }
}

fn apply_shift_factors(
    shift: &mut ParamShift,
    vth_factor: Value,
    mobility_factor: Value,
    rds_factor: Value,
) {
    shift.vth_shift *= vth_factor;
    shift.mobility_shift *= mobility_factor;
    shift.rds_shift *= rds_factor;
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

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbToneRunConfig {
    pub frequency: Value,
    pub harmonics: usize,
    pub source: Option<String>,
    pub name: Option<String>,
}

impl HbToneRunConfig {
    pub fn new(frequency: Value, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbRunConfig {
    pub tones: Vec<HbToneRunConfig>,
    pub reltol: Value,
    pub abstol: Value,
    pub max_iterations: usize,
    pub damping: Value,
    pub oversample: usize,
    pub max_mixing_order: usize,
    pub use_krylov: bool,
    pub gmres_restart: usize,
    pub source_stepping: bool,
    pub verbose: bool,
}

impl Default for HbRunConfig {
    fn default() -> Self {
        Self {
            tones: vec![HbToneRunConfig::new(1e9, 9)],
            reltol: 1e-6,
            abstol: 1e-12,
            max_iterations: 100,
            damping: 1.0,
            oversample: 2,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            verbose: false,
        }
    }
}

impl HbRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.tones.is_empty() {
            return Err("HB requires at least one tone".to_string());
        }
        for (idx, tone) in self.tones.iter().enumerate() {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(format!("HB tone {} frequency must be positive", idx + 1));
            }
            if tone.harmonics == 0 {
                return Err(format!("HB tone {} harmonics must be > 0", idx + 1));
            }
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("HB reltol must be > 0".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("HB abstol must be > 0".to_string());
        }
        if self.max_iterations == 0 {
            return Err("HB max_iterations must be > 0".to_string());
        }
        if !self.damping.is_finite() || self.damping <= 0.0 || self.damping > 1.0 {
            return Err("HB damping must be in (0, 1]".to_string());
        }
        if self.oversample == 0 {
            return Err("HB oversample must be > 0".to_string());
        }
        if self.max_mixing_order == 0 {
            return Err("HB max_mixing_order must be > 0".to_string());
        }
        if self.gmres_restart == 0 {
            return Err("HB gmres_restart must be > 0".to_string());
        }
        Ok(())
    }
}

/// Run Harmonic Balance analysis
///
/// Solves for the steady-state response in the frequency domain,
/// suitable for RF circuits with multiple tones.
pub fn run_hb_analysis(netlist_text: &str, config: &HbRunConfig) -> Result<HbData, String> {
    use rspice_core::analysis::{HbConfig, HbTone};
    config.validate()?;

    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(build_engine_config(&netlist, None));

    let layout = build_multi_tone_hb_layout(&config.tones, config.max_mixing_order)?;
    let mut hb_config = HbConfig::new(layout.base_frequency).with_harmonics(layout.max_harmonic);
    hb_config.tones = config
        .tones
        .iter()
        .enumerate()
        .map(|(idx, tone)| {
            let mut hb_tone = HbTone::new(tone.frequency, tone.harmonics.max(1));
            if let Some(name) = tone
                .name
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                hb_tone = hb_tone.with_name(name.to_string());
            } else {
                hb_tone = hb_tone.with_name(format!("tone{}", idx + 1));
            }
            if let Some(source) = tone
                .source
                .as_deref()
                .map(str::trim)
                .filter(|source| !source.is_empty())
            {
                hb_tone = hb_tone.with_source(source.to_string());
            }
            hb_tone
        })
        .collect();
    hb_config = hb_config
        .with_tolerance(config.reltol)
        .with_max_iterations(config.max_iterations)
        .with_damping(config.damping)
        .with_oversample(config.oversample);
    hb_config.abstol = config.abstol;
    hb_config.max_mixing_order = config.max_mixing_order;
    hb_config.use_krylov = config.use_krylov;
    hb_config.gmres_restart = config.gmres_restart;
    hb_config.source_stepping = config.source_stepping;
    hb_config.verbose = config.verbose;

    // Run actual HB analysis
    let hb_result = engine
        .run_hb(&netlist, hb_config)
        .map_err(|e| format!("HB error: {}", e))?;

    // Build fundamentals list
    let fundamentals = config.tones.iter().map(|tone| tone.frequency).collect();
    let harmonics_per_tone = config.tones.iter().map(|tone| tone.harmonics).collect();

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
            let freq = hb_result.fundamental_freq * h as Value;
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
// PSTB (Periodic Stability) Analysis
// =============================================================================

/// Explicit configuration for PSTB execution.
#[derive(Debug, Clone)]
pub struct PstbRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: Value,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: Value,
}

impl Default for PstbRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 10,
            num_multipliers: 10,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }
    }
}

impl PstbRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PSTB requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PSTB requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PSTB requires a positive PSS tolerance".to_string());
        }
        if self.probe_instance.trim().is_empty() {
            return Err("PSTB probe instance must be specified".to_string());
        }
        if self.max_harmonics == 0 {
            return Err("PSTB max harmonics must be greater than zero".to_string());
        }
        if self.num_multipliers == 0 {
            return Err("PSTB number of multipliers must be greater than zero".to_string());
        }
        if !self.stability_threshold.is_finite() || self.stability_threshold <= 0.0 {
            return Err("PSTB stability threshold must be positive".to_string());
        }
        if !self.eigenvalue_tolerance.is_finite() || self.eigenvalue_tolerance <= 0.0 {
            return Err("PSTB eigenvalue tolerance must be positive".to_string());
        }
        Ok(())
    }
}

/// PSTB analysis data.
#[derive(Debug, Clone)]
pub struct PstbData {
    /// Fundamental period in seconds.
    pub period: Value,
    /// Fundamental frequency in Hz.
    pub fundamental_frequency: Value,
    /// Probe instance used for metadata correlation.
    pub probe_instance: String,
    /// Probe branch ordinal in MNA ordering (1-indexed branch numbering).
    pub probe_branch_ordinal: usize,
    /// Reactive state index in the PSS/PSTB monodromy matrix (0-indexed).
    pub probe_state_index: usize,
    /// Probe-state self-transition term M[i,i] over one period.
    pub probe_state_self_transition: Value,
    /// Euclidean norm of the probe-state monodromy column ||M[:,i]||2.
    pub probe_state_column_norm: Value,
    /// Euclidean norm of the probe-state monodromy row ||M[i,:]||2.
    pub probe_state_row_norm: Value,
    /// Probe-state persistence in dB, computed from |M[i,i]|.
    pub probe_state_persistence_db: Value,
    /// Mode indices (1-based) for plotting.
    pub mode_indices: Vec<Value>,
    /// Probe-local mode participation (normalized |v_i| contribution per mode).
    pub probe_mode_participation: Vec<Value>,
    /// Floquet multiplier magnitudes.
    pub multiplier_magnitude: Vec<Value>,
    /// Floquet multiplier phases in degrees.
    pub multiplier_phase_deg: Vec<Value>,
    /// Mode damping factors in 1/s.
    pub mode_damping: Vec<Value>,
    /// Mode natural frequencies in Hz.
    pub mode_frequency_hz: Vec<Value>,
    /// Per-mode stability margin in dB.
    pub stability_margin_db: Vec<Value>,
    /// Dominant multiplier magnitude.
    pub dominant_multiplier_magnitude: Value,
    /// Global minimum stability margin in dB.
    pub min_stability_margin_db: Value,
    /// Mode index (1-based) with largest probe participation.
    pub dominant_probe_mode: usize,
    /// Largest probe participation value across retained modes.
    pub dominant_probe_mode_participation: Value,
    /// Number of unstable modes.
    pub num_unstable: usize,
    /// Stability classification string.
    pub stability_classification: String,
    /// Whether the periodic orbit is stable.
    pub is_stable: bool,
}

fn stability_type_label(
    stability: rspice_core::analysis::advanced::pstb::StabilityType,
) -> &'static str {
    use rspice_core::analysis::advanced::pstb::StabilityType;
    match stability {
        StabilityType::Stable => "Stable",
        StabilityType::UnstableReal => "UnstableReal",
        StabilityType::UnstableComplex => "UnstableComplex",
        StabilityType::PeriodDoubling => "PeriodDoubling",
        StabilityType::NeimarkSacker => "NeimarkSacker",
        StabilityType::SaddleNode => "SaddleNode",
        StabilityType::Marginal => "Marginal",
    }
}

fn sanitize_db(value: Value) -> Value {
    if value.is_finite() {
        value
    } else if value.is_sign_positive() {
        300.0
    } else {
        -300.0
    }
}

#[derive(Debug, Clone)]
struct ResolvedPstbProbe {
    canonical_name: String,
    branch_ordinal: usize,
    state_index: usize,
}

fn normalize_branch_name_list(mut names: Vec<String>) -> Vec<String> {
    names.sort_by_cached_key(|name| name.to_ascii_uppercase());
    names.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    names
}

fn format_branch_name_list(names: &[String]) -> String {
    if names.is_empty() {
        return "<none>".to_string();
    }
    const DISPLAY_LIMIT: usize = 12;
    if names.len() <= DISPLAY_LIMIT {
        return names.join(", ");
    }
    let shown = names[..DISPLAY_LIMIT].join(", ");
    format!("{shown}, ... (+{} more)", names.len() - DISPLAY_LIMIT)
}

fn available_branch_names(circuit: &rspice_core::circuit::CircuitData) -> Vec<String> {
    let mut names = Vec::new();
    names.extend(circuit.inductors.names.iter().cloned());
    names.extend(circuit.voltage_sources.names.iter().cloned());
    names.extend(circuit.ccvs.names.iter().cloned());
    names.extend(
        circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .map(|src| src.name.clone()),
    );
    normalize_branch_name_list(names)
}

fn available_inductor_probe_names(circuit: &rspice_core::circuit::CircuitData) -> Vec<String> {
    normalize_branch_name_list(circuit.inductors.names.clone())
}

fn resolve_pstb_probe(
    circuit: &rspice_core::circuit::CircuitData,
    probe_instance: &str,
) -> Result<ResolvedPstbProbe, String> {
    let probe_name = probe_instance.trim();
    let branch_ordinal = circuit.get_branch_by_name(probe_name).ok_or_else(|| {
        let available = format_branch_name_list(&available_branch_names(circuit));
        format!(
            "PSTB probe '{}' was not found in branch-capable elements. Available branches: {}",
            probe_name, available
        )
    })? as usize;

    let inductor_index = circuit
        .inductors
        .branch_indices
        .iter()
        .position(|branch| *branch == branch_ordinal)
        .ok_or_else(|| {
            let available = format_branch_name_list(&available_inductor_probe_names(circuit));
            format!(
                "PSTB probe '{}' resolved to branch ordinal {} but is not an inductor probe. \
PSTB currently supports dynamic inductor-current probes only. Available inductor probes: {}",
                probe_name, branch_ordinal, available
            )
        })?;

    let state_index = circuit.capacitors.len() + inductor_index;
    let canonical_name = circuit.inductors.names[inductor_index].clone();

    Ok(ResolvedPstbProbe {
        canonical_name,
        branch_ordinal,
        state_index,
    })
}

fn finite_l2_norm(values: impl Iterator<Item = Value>) -> Value {
    let mut sum_sq = 0.0;
    for value in values {
        if !value.is_finite() {
            return f64::INFINITY;
        }
        sum_sq += value * value;
    }
    sum_sq.sqrt()
}

fn sanitize_nonnegative(value: Value) -> Value {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        0.0
    }
}

fn sanitize_finite(value: Value) -> Value {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

fn normalized_probe_participation(
    eigenvector: Option<&Vec<num_complex::Complex64>>,
    state_index: usize,
) -> Value {
    let Some(vector) = eigenvector else {
        return 0.0;
    };
    let Some(component) = vector.get(state_index) else {
        return 0.0;
    };
    let denom = vector.iter().map(|v| v.norm_sqr()).sum::<Value>().sqrt();
    if !denom.is_finite() || denom <= 1e-30 {
        return 0.0;
    }
    let ratio = component.norm() / denom;
    if ratio.is_finite() {
        ratio.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Run PSTB analysis using a PSS operating point and monodromy-based Floquet analysis.
pub fn run_pstb_analysis_with_config(
    netlist_text: &str,
    config: &PstbRunConfig,
) -> Result<PstbData, String> {
    use rspice_core::analysis::advanced::pstb::{PstbAnalyzer, PstbConfig};
    use rspice_core::analysis::PssConfig;

    config.validate()?;

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let engine = Engine::new(sim_config);
    let circuit = engine
        .build_circuit(&netlist)
        .map_err(|e| format!("PSTB prerequisite circuit-build error: {}", e))?;
    let probe = resolve_pstb_probe(&circuit, &config.probe_instance)?;

    let pss_harmonics = config.pss_num_harmonics.max(config.max_harmonics);
    let pss_config = PssConfig::new(config.pss_fundamental_freq)
        .with_harmonics(pss_harmonics)
        .with_tolerance(config.pss_tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);
    let pss_result = engine
        .run_pss(&netlist, pss_config)
        .map_err(|e| format!("PSTB prerequisite PSS error: {}", e))?;

    if pss_result.monodromy.is_empty() {
        return Err("PSTB prerequisite PSS returned an empty monodromy matrix".to_string());
    }
    let monodromy_dim = pss_result.monodromy.len();
    if pss_result
        .monodromy
        .iter()
        .any(|row| row.len() != monodromy_dim)
    {
        return Err("PSTB prerequisite PSS returned a non-square monodromy matrix".to_string());
    }
    if probe.state_index >= monodromy_dim {
        return Err(format!(
            "PSTB probe '{}' maps to reactive state {} but monodromy dimension is {}",
            probe.canonical_name, probe.state_index, monodromy_dim
        ));
    }

    let probe_row = &pss_result.monodromy[probe.state_index];
    let probe_self_transition = sanitize_finite(probe_row[probe.state_index]);
    let probe_column_norm = sanitize_nonnegative(finite_l2_norm(
        pss_result
            .monodromy
            .iter()
            .map(|row| row[probe.state_index]),
    ));
    let probe_row_norm = sanitize_nonnegative(finite_l2_norm(probe_row.iter().copied()));
    let probe_persistence_db = sanitize_db(20.0 * probe_self_transition.abs().max(1e-30).log10());

    let pstb_config = PstbConfig::new()
        .with_num_eigenvalues(config.num_multipliers)
        .with_eigenvectors(true)
        .with_tolerance(config.eigenvalue_tolerance)
        .with_stability_threshold(config.stability_threshold)
        .with_subharmonic_detection(config.detect_subharmonics);
    let mut analyzer = PstbAnalyzer::new(pstb_config);
    let pstb_result = analyzer.analyze_monodromy(&pss_result.monodromy, pss_result.period);

    let mut mode_indices = Vec::new();
    let mut probe_mode_participation = Vec::new();
    let mut multiplier_magnitude = Vec::new();
    let mut multiplier_phase_deg = Vec::new();
    let mut mode_damping = Vec::new();
    let mut mode_frequency_hz = Vec::new();
    let mut stability_margin_db = Vec::new();

    for (idx, multiplier) in pstb_result
        .multipliers
        .iter()
        .take(config.num_multipliers)
        .enumerate()
    {
        mode_indices.push((idx + 1) as Value);
        probe_mode_participation.push(sanitize_nonnegative(normalized_probe_participation(
            multiplier.eigenvector.as_ref(),
            probe.state_index,
        )));
        multiplier_magnitude.push(multiplier.magnitude());
        multiplier_phase_deg.push(multiplier.phase_degrees());
        mode_damping.push(multiplier.damping());
        mode_frequency_hz.push(multiplier.natural_frequency());
        stability_margin_db.push(sanitize_db(multiplier.stability_margin_db()));
    }

    if mode_indices.is_empty() {
        return Err("PSTB produced no Floquet multipliers".to_string());
    }
    let (dominant_probe_mode, dominant_probe_mode_participation) = probe_mode_participation
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(idx, value)| (idx + 1, value))
        .unwrap_or((0, 0.0));

    Ok(PstbData {
        period: pstb_result.period,
        fundamental_frequency: pstb_result.fundamental_frequency,
        probe_instance: probe.canonical_name,
        probe_branch_ordinal: probe.branch_ordinal,
        probe_state_index: probe.state_index,
        probe_state_self_transition: probe_self_transition,
        probe_state_column_norm: probe_column_norm,
        probe_state_row_norm: probe_row_norm,
        probe_state_persistence_db: probe_persistence_db,
        mode_indices,
        probe_mode_participation,
        multiplier_magnitude,
        multiplier_phase_deg,
        mode_damping,
        mode_frequency_hz,
        stability_margin_db,
        dominant_multiplier_magnitude: pstb_result.max_multiplier_magnitude,
        min_stability_margin_db: sanitize_db(pstb_result.min_stability_margin_db),
        dominant_probe_mode,
        dominant_probe_mode_participation,
        num_unstable: pstb_result.num_unstable,
        stability_classification: stability_type_label(pstb_result.stability).to_string(),
        is_stable: pstb_result.is_stable(),
    })
}

/// Run PSTB analysis with default configuration.
pub fn run_pstb_analysis(netlist_text: &str) -> Result<PstbData, String> {
    let cfg = PstbRunConfig::default();
    run_pstb_analysis_with_config(netlist_text, &cfg)
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
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
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

fn infer_tf_run_config(
    netlist: &rspice_core::Netlist,
    node_names: &[String],
) -> Result<TfRunConfig, String> {
    let input_source = infer_primary_source_name(netlist)
        .ok_or_else(|| "TF requires at least one independent source in the netlist".to_string())?;
    let output_node = infer_primary_output_node(node_names).ok_or_else(|| {
        "TF could not infer an output node; ensure at least one non-ground node exists".to_string()
    })?;
    Ok(TfRunConfig {
        input_source,
        output_node,
        ..TfRunConfig::default()
    })
}

fn source_dc_bias(spec: &SourceSpec) -> Value {
    match spec {
        SourceSpec::Dc(v) => *v,
        SourceSpec::Ac { .. } => 0.0,
        SourceSpec::DcAc { dc_value, .. } => *dc_value,
        SourceSpec::Pulse { v1, .. } => *v1,
        SourceSpec::Sin { offset, .. } => *offset,
        SourceSpec::Pwl { points } => points.first().map(|(_, value)| *value).unwrap_or(0.0),
        SourceSpec::PwlFile { .. } => 0.0,
        SourceSpec::Exp { v1, .. } => *v1,
    }
}

fn source_with_ac_excitation(spec: &SourceSpec, magnitude: Value, phase_deg: Value) -> SourceSpec {
    SourceSpec::DcAc {
        dc_value: source_dc_bias(spec),
        ac_magnitude: magnitude,
        ac_phase: phase_deg,
    }
}

fn source_without_ac(spec: &SourceSpec) -> SourceSpec {
    source_with_ac_excitation(spec, 0.0, 0.0)
}

fn zero_all_source_ac(netlist: &mut rspice_core::Netlist) {
    for element in &mut netlist.elements {
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_without_ac(spec);
            }
            _ => {}
        }
    }
}

fn set_source_ac_excitation(
    netlist: &mut rspice_core::Netlist,
    source_name: &str,
    magnitude: Value,
    phase_deg: Value,
) -> Result<(), String> {
    let source_name = source_name.trim();
    if source_name.is_empty() {
        return Err("source name cannot be empty".to_string());
    }

    let mut matched = false;
    for element in &mut netlist.elements {
        if !element.name.eq_ignore_ascii_case(source_name) {
            continue;
        }
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_with_ac_excitation(spec, magnitude, phase_deg);
                matched = true;
            }
            _ => {
                return Err(format!(
                    "Element '{}' exists but is not an independent source",
                    source_name
                ));
            }
        }
    }

    if !matched {
        return Err(format!(
            "Independent source '{}' was not found in the netlist",
            source_name
        ));
    }

    Ok(())
}

fn unique_element_name(netlist: &rspice_core::Netlist, prefix: &str) -> String {
    if netlist
        .elements
        .iter()
        .all(|element| !element.name.eq_ignore_ascii_case(prefix))
    {
        return prefix.to_string();
    }

    for idx in 1.. {
        let candidate = format!("{}{}", prefix, idx);
        if netlist
            .elements
            .iter()
            .all(|element| !element.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }

    unreachable!("monotonic suffix search should always find a free element name")
}

fn inject_tf_output_test_source(
    netlist: &mut rspice_core::Netlist,
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<(), String> {
    let output_node = output_node.trim();
    if output_node.is_empty() {
        return Err("TF output node must be non-empty".to_string());
    }
    let output_ref = output_ref
        .map(str::trim)
        .filter(|node| !node.is_empty())
        .unwrap_or("0");
    let test_name = unique_element_name(netlist, "__TF_ZOUT_TEST");
    netlist.elements.push(rspice_core::netlist::Element {
        name: test_name,
        kind: ElementKind::CurrentSource(SourceSpec::Ac {
            magnitude: 1.0,
            phase: 0.0,
        }),
        nodes: vec![output_node.to_string(), output_ref.to_string()],
    });
    Ok(())
}

fn normalize_voltage_signal_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.len() > 3 && trimmed[..2].eq_ignore_ascii_case("V(") && trimmed.ends_with(')') {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
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
    fn test_interpolate_magnitude_at_log_frequency() {
        let frequencies = vec![1.0, 10.0, 100.0];
        let magnitudes = vec![1.0, 10.0, 100.0];
        let mid = interpolate_magnitude_at(&frequencies, &magnitudes, 31.622776601683793)
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
        assert!(result
            .voltages
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("V(out)")));
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
        assert!(result
            .voltages
            .iter()
            .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
                && values.len() == 3
                && values.iter().all(|v| v.is_finite() && *v >= 0.0)));
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
        assert!(result
            .voltages
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("V(out)")));
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
        assert!(result
            .corner_labels
            .iter()
            .any(|label| label.contains("FF_1.100000V_125.000000C")));
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
        assert!(result
            .voltages
            .iter()
            .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
                && values.len() == 3
                && values.iter().all(|v| v.is_finite() && *v >= 0.0)));
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
        assert!(result
            .transfer
            .iter()
            .all(|value| value.re.is_finite() && value.im.is_finite()));
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
        assert!(result
            .transfer
            .iter()
            .all(|value| value.re.is_finite() && value.im.is_finite()));
        assert!(result
            .group_delay
            .as_ref()
            .is_some_and(|curve| !curve.is_empty()));
        assert!(result
            .input_impedance
            .as_ref()
            .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite())));
        assert!(result
            .output_impedance
            .as_ref()
            .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite())));
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
        assert!(result
            .total_output_noise
            .is_some_and(|value| value.is_finite() && value >= 0.0));
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
        assert!(!input_result
            .warnings
            .iter()
            .any(|warning| warning.contains("TF fallback")));
        assert!(!input_result
            .warnings
            .iter()
            .any(|warning| warning.contains("unity gain")));

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
        assert!(!result
            .warnings
            .iter()
            .any(|warning| warning.contains("TF fallback")));

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
        assert!(result
            .contributors
            .iter()
            .all(|(_, percentage)| percentage.is_finite() && *percentage >= 0.0));
        assert!(!result
            .warnings
            .iter()
            .any(|warning| warning.contains("uncorrelated PSD summation")));
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
        assert!(result
            .probe_mode_participation
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0 && *value <= 1.0));
        assert!(result
            .multiplier_magnitude
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0));
        assert!(result
            .multiplier_phase_deg
            .iter()
            .all(|value| value.is_finite()));
        assert!(result.mode_damping.iter().all(|value| value.is_finite()));
        assert!(result
            .mode_frequency_hz
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0));
        assert!(result
            .stability_margin_db
            .iter()
            .all(|value| value.is_finite()));
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
    fn test_compute_s_from_y_matrix_applies_nonuniform_reference_normalization() {
        let y = vec![
            vec![Complex64::new(0.0, 0.0), Complex64::new(5e-3, 0.0)],
            vec![Complex64::new(5e-3, 0.0), Complex64::new(0.0, 0.0)],
        ];
        let z0 = vec![50.0, 200.0];

        let mut a = identity_complex_matrix(2);
        let mut b = identity_complex_matrix(2);
        for row in 0..2 {
            let zref = Complex64::new(z0[row], 0.0);
            for col in 0..2 {
                let zy = zref * y[row][col];
                a[row][col] += zy;
                b[row][col] -= zy;
            }
        }
        let inv_a = invert_complex_matrix(&a).expect("matrix should be invertible");
        let raw = multiply_complex_matrix(&b, &inv_a);
        let mut expected = raw.clone();
        for row in 0..2 {
            for col in 0..2 {
                let scale = (z0[col] / z0[row]).sqrt();
                expected[row][col] *= Complex64::new(scale, 0.0);
            }
        }

        let actual = compute_s_from_y_matrix(&y, &z0);
        for row in 0..2 {
            for col in 0..2 {
                assert!(
                    (actual[row][col] - expected[row][col]).norm() < 1e-12,
                    "S{}{} mismatch: actual={}, expected={}",
                    row + 1,
                    col + 1,
                    actual[row][col],
                    expected[row][col]
                );
            }
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
