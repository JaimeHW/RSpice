use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;

use super::super::error::{ensure_not_aborted, poll_periodically};
use super::super::{
    ServiceRunError, ServiceRunResult, build_engine_config, infer_primary_output_node_with_abort,
    infer_primary_source_name_with_abort, parse_runner_netlist_with_abort,
};
use super::shared::{normalize_pac_node_name, resolve_pac_output_node_with_abort};
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

pub(super) struct PacInternalResult {
    pub(super) pac_result: rspice_core::analysis::advanced::pac::PacResult,
    pub(super) output_node_idx: usize,
    pub(super) output_node_name: String,
}

pub(super) fn run_pac_internal_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    use rspice_core::analysis::advanced::pac::PacConfig;

    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let mut sim_config = build_engine_config(netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let engine = Engine::new(sim_config);

    let mut pac_config = PacConfig::new()
        .with_sweep(config.start_freq, config.stop_freq, config.points_per_unit)
        .with_sweep_type(config.sweep.to_core())
        .with_sidebands(-config.max_sideband, config.max_sideband)
        .with_input_source(config.input_source.trim())
        .with_output_node(&normalize_pac_node_name(&config.output_node))
        .with_tolerances(config.reltol, config.abstol)
        .with_dc(config.include_dc)
        .with_fundamental(config.pss_fundamental_freq);

    if let Some(output_ref) = &config.output_ref {
        let trimmed = output_ref.trim();
        if !trimmed.is_empty() {
            pac_config = pac_config.with_output_ref(trimmed);
        }
    }

    pac_config
        .validate()
        .map_err(|error| ServiceRunError::Failure(format!("PAC configuration error: {error}")))?;
    ensure_not_aborted(abort)?;

    // The engine solves the periodic operating point with harmonic balance
    // and the sideband-coupled small-signal system around it.
    let pac_result = engine
        .run_pac_with_abort(netlist, pac_config, abort)
        .map_err(|error| ServiceRunError::from_core("PAC error", error))?
        .result;

    let output_node_idx =
        resolve_pac_output_node_with_abort(&pac_result, &config.output_node, abort)?.ok_or_else(
            || {
                ServiceRunError::Failure(format!(
                    "PAC output node '{}' was not found in PSS result nodes {:?}",
                    config.output_node, pac_result.node_names
                ))
            },
        )?;
    ensure_not_aborted(abort)?;
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
    run_pac_analysis_with_abort(netlist_text, config, &NoAbort).map_err(|error| error.to_string())
}

/// Run PAC analysis with cooperative cancellation.
pub fn run_pac_analysis_with_abort(
    netlist_text: &str,
    config: &PacRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    run_pac_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PAC analysis by first solving PSS and then linearizing around the
/// periodic solution, resolving relative includes from the source path when
/// provided.
pub fn run_pac_analysis_with_source_path(
    netlist_text: &str,
    config: &PacRunConfig,
    source_path: Option<&Path>,
) -> Result<PacData, String> {
    run_pac_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run PAC analysis with source-path resolution and cooperative cancellation.
pub fn run_pac_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &PacRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pac_analysis_for_netlist_with_abort(&netlist, config, abort)
}

fn run_pac_analysis_for_netlist_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let pac_internal = run_pac_internal_with_abort(netlist, config, abort)?;
    let output_node_idx = pac_internal.output_node_idx;
    let pac_result = pac_internal.pac_result;

    let mut sidebands = Vec::new();
    for (index, sideband) in pac_result.sideband_indices().into_iter().enumerate() {
        poll_periodically(abort, index)?;
        if config.include_dc || sideband != 0 {
            sidebands.push(sideband);
        }
    }
    if sidebands.is_empty() {
        return Err(ServiceRunError::Failure(
            "PAC produced no sidebands with current configuration".to_string(),
        ));
    }

    let frequencies = pac_result.frequencies.clone();
    let output_node_name = pac_internal.output_node_name;

    let mut spectra: Vec<(String, Vec<(Value, Value, Value)>)> =
        Vec::with_capacity(sidebands.len());
    for (sideband_index, sideband) in sidebands.iter().enumerate() {
        poll_periodically(abort, sideband_index)?;
        let mut spectrum = Vec::with_capacity(frequencies.len());
        for (freq_idx, freq_offset) in frequencies.iter().copied().enumerate() {
            poll_periodically(abort, freq_idx)?;
            let voltage = pac_result.voltage(output_node_idx, freq_idx, *sideband)
                * Complex64::new(config.pac_magnitude, 0.0);
            spectrum.push((freq_offset, voltage.norm(), voltage.arg().to_degrees()));
        }
        spectra.push((
            format!("V({})[sb={:+}]", output_node_name, sideband),
            spectrum,
        ));
    }

    ensure_not_aborted(abort)?;
    Ok(PacData {
        frequencies,
        sidebands,
        spectra,
        converged: true,
    })
}

/// Run PAC analysis using inferred/default settings.
pub fn run_pac_analysis_auto(netlist_text: &str) -> Result<PacData, String> {
    run_pac_analysis_auto_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run inferred/default PAC analysis with cancellation.
pub fn run_pac_analysis_auto_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    run_pac_analysis_auto_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run PAC analysis using inferred/default settings and a source path used to
/// resolve relative includes and model file references.
pub fn run_pac_analysis_auto_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PacData, String> {
    run_pac_analysis_auto_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run inferred/default PAC analysis with source-path resolution and
/// cancellation.
pub fn run_pac_analysis_auto_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for PAC defaults)", error)
        })?;
    let input_source = infer_primary_source_name_with_abort(&netlist, abort)?.ok_or_else(|| {
        ServiceRunError::Failure(
            "PAC requires at least one independent source in the netlist".to_string(),
        )
    })?;
    let output_node = infer_primary_output_node_with_abort(&dc_result.node_names, abort)?
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "PAC could not infer an output node; ensure at least one non-ground node exists"
                    .to_string(),
            )
        })?;

    let cfg = PacRunConfig {
        input_source,
        output_node,
        ..PacRunConfig::default()
    };
    run_pac_analysis_for_netlist_with_abort(&netlist, &cfg, abort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::ImmediateAbort;

    #[test]
    fn pac_service_preserves_typed_entry_abort() {
        let result =
            run_pac_analysis_with_abort("not a netlist", &PacRunConfig::default(), &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
