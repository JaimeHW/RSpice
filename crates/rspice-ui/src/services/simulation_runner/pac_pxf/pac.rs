//! Periodic AC analysis.
//!
//! Small-signal response about a periodic steady state, where a stimulus at
//! one frequency produces a response at every sideband.

use std::collections::HashSet;
use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;

use super::super::error::{ensure_not_aborted, poll_periodically};
use super::super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
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
    fn to_core(self) -> rspice_core::analysis::pac::PacSweepType {
        match self {
            Self::Decade => rspice_core::analysis::pac::PacSweepType::Decade,
            Self::Octave => rspice_core::analysis::pac::PacSweepType::Octave,
            Self::Linear => rspice_core::analysis::pac::PacSweepType::Linear,
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
    /// Exact complex PAC traces aligned with `frequencies`.
    pub traces: Vec<PacTrace>,
}

/// One exact complex-valued PAC trace.
///
/// The service keeps rectangular components rather than converting through
/// magnitude and phase. That preserves the engine's complex values for worker
/// transport, persistence, and export; presentation can derive polar views.
#[derive(Debug, Clone)]
pub struct PacTrace {
    pub name: String,
    pub unit: &'static str,
    pub values: Vec<Complex64>,
}

pub(crate) struct PacInternalResult {
    pub(crate) pac_result: rspice_core::analysis::pac::PacResult,
    pub(super) output_node_name: String,
}

pub(super) fn run_pac_internal_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    run_pac_internal_impl(netlist, config, None, abort)
}

pub(crate) fn run_pac_internal_from_pss_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    run_pac_internal_impl(netlist, config, Some(operating_point), abort)
}

pub(crate) fn run_pac_internal_from_hb_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    operating_point: &rspice_core::engine::HbOperatingPoint,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let engine = build_resolved_periodic_engine(
        netlist,
        config.pss_tolerance,
        "PAC resolved producer configuration is invalid",
    )?;
    let pac_config = build_core_pac_config(config)?;
    let pac_result = engine
        .run_pac_from_hb_with_abort(netlist, pac_config, operating_point, abort)
        .map_err(|error| ServiceRunError::from_core("PAC error", error))?
        .result;
    // The engine builds the conversion basis on the carrier's fundamental, not
    // on this configuration's; see `carrier_fundamental` below.
    let carrier_fundamental = operating_point.config().fundamental_freq;
    finish_pac_internal(pac_result, config, carrier_fundamental, abort)
}

fn run_pac_internal_impl(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let engine = build_resolved_periodic_engine(
        netlist,
        config.pss_tolerance,
        "PAC resolved producer configuration is invalid",
    )?;

    let pac_config = build_core_pac_config(config)?;

    ensure_not_aborted(abort)?;

    // The engine solves the periodic operating point with harmonic balance
    // and the sideband-coupled small-signal system around it.
    let pac_result = match operating_point {
        Some(operating_point) => {
            engine.run_pac_from_pss_with_abort(netlist, pac_config, operating_point, abort)
        }
        None => engine.run_pac_with_abort(netlist, pac_config, abort),
    }
    .map_err(|error| ServiceRunError::from_core("PAC error", error))?
    .result;

    finish_pac_internal(
        pac_result,
        config,
        carrier_fundamental(config, operating_point),
        abort,
    )
}

/// The fundamental the engine builds this PAC's conversion basis on.
///
/// `Engine::run_pac_*_from_*` replaces the authored fundamental with the
/// carrier's own before it solves anything
/// (`rspice-core/src/engine/hb/pac.rs`), and the result is constructed from
/// that value. So the result's fundamental is a property of the *carrier*, and
/// the only honest check here is that the two are the same number.
///
/// For a driven carrier the drive sets the period, so the carrier's
/// fundamental and the authored one are the same bits and the difference never
/// showed. For an **autonomous** carrier the shooting solver holds the period
/// as an unknown and moves it (`rspice-core/src/engine/pss.rs`), so the
/// converged fundamental is not the authored guess -- and comparing the result
/// against the guess refused every oscillator, which is the whole of
/// oscillator conversion analysis.
///
/// Whether the *carrier itself* is the one the deck asked for is a different
/// question, asked earlier and elsewhere: `PeriodicStateArtifact::
/// validate_consumer_basis` compares the authored basis against the producer's
/// authored basis before the run is dispatched.
fn carrier_fundamental(
    config: &PacRunConfig,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
) -> Value {
    // With no retained carrier the engine keeps the authored fundamental, so
    // that is what its result is built on.
    operating_point.map_or(config.pss_fundamental_freq, |point| {
        point.analysis().result.frequency
    })
}

fn build_core_pac_config(
    config: &PacRunConfig,
) -> ServiceRunResult<rspice_core::analysis::pac::PacConfig> {
    use rspice_core::analysis::pac::PacConfig;

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
    Ok(pac_config)
}

fn finish_pac_internal(
    pac_result: rspice_core::analysis::pac::PacResult,
    config: &PacRunConfig,
    carrier_fundamental: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacInternalResult> {
    validate_pac_result(&pac_result, config, carrier_fundamental)?;
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
    let output_node_name = pac_result.node_names[output_node_idx].clone();

    Ok(PacInternalResult {
        pac_result,
        output_node_name,
    })
}

fn validate_pac_result(
    result: &rspice_core::analysis::pac::PacResult,
    config: &PacRunConfig,
    carrier_fundamental: Value,
) -> ServiceRunResult<()> {
    if !result.fundamental_frequency.is_finite()
        || result.fundamental_frequency <= 0.0
        || !result.residual.is_finite()
        || result.residual < 0.0
    {
        return Err(ServiceRunError::Failure(
            "PAC engine returned an invalid solved basis or residual".to_owned(),
        ));
    }
    // Identity, not approximation: the engine copies the carrier's fundamental
    // into the result without arithmetic, so any difference at all means the
    // conversion matrix was built on a periodic solution other than the one
    // this run was handed, and every sideband frequency below is then a
    // different measurement than the one reported.
    if result.fundamental_frequency.to_bits() != carrier_fundamental.to_bits() {
        return Err(ServiceRunError::Failure(format!(
            "PAC conversion basis is at {:.17e} Hz but its periodic carrier is at {carrier_fundamental:.17e} Hz",
            result.fundamental_frequency
        )));
    }
    if result.sideband_min != -config.max_sideband
        || result.sideband_max != config.max_sideband
        || result.conversion_matrix.sideband_indices() != result.sideband_indices()
        || result.conversion_matrix.num_frequencies() != result.frequencies.len()
        || result.conversion_matrix.fundamental().to_bits()
            != result.fundamental_frequency.to_bits()
        || result.conversion_matrix.frequencies().len() != result.frequencies.len()
        || result
            .conversion_matrix
            .frequencies()
            .iter()
            .zip(&result.frequencies)
            .any(|(matrix, result)| matrix.to_bits() != result.to_bits())
    {
        return Err(ServiceRunError::Failure(
            "PAC engine returned an inconsistent conversion-matrix basis".to_owned(),
        ));
    }
    if result.frequencies.is_empty()
        || result
            .frequencies
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency <= 0.0)
        || result.frequencies.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(ServiceRunError::Failure(
            "PAC engine returned an invalid frequency grid".to_owned(),
        ));
    }
    let mut node_names = HashSet::with_capacity(result.node_names.len());
    if result.node_names.is_empty()
        || result.node_names.iter().any(|name| {
            let normalized = name.trim().to_ascii_lowercase();
            normalized.is_empty() || !node_names.insert(normalized)
        })
    {
        return Err(ServiceRunError::Failure(
            "PAC engine returned an empty or duplicate node identity".to_owned(),
        ));
    }
    let mut branch_names = HashSet::with_capacity(result.branch_names.len());
    if result.branch_names.iter().any(|name| {
        let normalized = name.trim().to_ascii_lowercase();
        normalized.is_empty() || !branch_names.insert(normalized)
    }) {
        return Err(ServiceRunError::Failure(
            "PAC engine returned an empty or duplicate branch identity".to_owned(),
        ));
    }
    for frequency_index in 0..result.frequencies.len() {
        let frequency = result.frequencies[frequency_index];
        for sideband in result.sideband_indices() {
            let data = result
                .get_sideband_data(frequency_index, sideband)
                .ok_or_else(|| {
                    ServiceRunError::Failure(format!(
                        "PAC sideband result is unavailable at frequency point {}, sideband {}",
                        frequency_index + 1,
                        sideband
                    ))
                })?;
            let expected_absolute =
                (sideband as Value).mul_add(result.fundamental_frequency, frequency);
            if data.sideband != sideband
                || data.frequency_offset.to_bits() != frequency.to_bits()
                || data.absolute_frequency.to_bits() != expected_absolute.to_bits()
                || data.node_voltages.len() != result.node_names.len()
                || data.branch_currents.len() != result.branch_names.len()
            {
                return Err(ServiceRunError::Failure(format!(
                    "PAC sideband result has inconsistent metadata or cardinality at frequency point {}, sideband {}",
                    frequency_index + 1,
                    sideband
                )));
            }
            if data
                .node_voltages
                .iter()
                .chain(&data.branch_currents)
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(ServiceRunError::Failure(format!(
                    "PAC sideband result contains a non-finite node voltage or branch current at frequency point {}, sideband {}",
                    frequency_index + 1,
                    sideband
                )));
            }
        }
        for output_sideband in result.sideband_indices() {
            for input_sideband in result.sideband_indices() {
                let value = result
                    .conversion_matrix
                    .get(frequency_index, output_sideband, input_sideband)
                    .map_err(|error| {
                        ServiceRunError::Failure(format!(
                            "PAC conversion result is unavailable: {error}"
                        ))
                    })?;
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(ServiceRunError::Failure(format!(
                        "PAC conversion matrix contains a non-finite value at frequency point {}, output sideband {}, input sideband {}",
                        frequency_index + 1,
                        output_sideband,
                        input_sideband
                    )));
                }
            }
        }
    }
    Ok(())
}

fn checked_scale_pac_value(
    value: Complex64,
    scale: Value,
    trace: &str,
    point_index: usize,
) -> ServiceRunResult<Complex64> {
    let scale_component = |component: Value, component_name: &str| {
        let scaled = component * scale;
        if !scaled.is_finite() || (component != 0.0 && scale != 0.0 && scaled == 0.0) {
            return Err(ServiceRunError::Failure(format!(
                "PAC trace '{trace}' {component_name} component at point {} cannot be represented after applying PAC magnitude {scale}",
                point_index + 1
            )));
        }
        Ok(scaled)
    };
    Ok(Complex64::new(
        scale_component(value.re, "real")?,
        scale_component(value.im, "imaginary")?,
    ))
}

/// Run PAC standalone -- solving its own PSS and then linearizing around that
/// periodic solution -- with cooperative cancellation.
///
/// Test-only. PAC ships as a dependent task through
/// [`run_pac_analysis_from_pss_with_source_path_and_abort`].
#[cfg(test)]
pub fn run_pac_analysis_with_abort(
    netlist_text: &str,
    config: &PacRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    run_pac_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PAC from an exact retained PSS state with direct-call source-relative
/// include and model resolution.
pub fn run_pac_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PacRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pac_analysis_for_netlist_with_operating_point_abort(
        &netlist,
        config,
        Some(operating_point),
        abort,
    )
}

/// Run PAC analysis with source-path resolution and cooperative cancellation,
/// solving its own PSS.
///
/// Test-only; see [`run_pac_analysis_with_abort`].
#[cfg(test)]
pub fn run_pac_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &PacRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pac_analysis_for_netlist_with_operating_point_abort(&netlist, config, None, abort)
}

fn run_pac_analysis_for_netlist_with_operating_point_abort(
    netlist: &rspice_core::Netlist,
    config: &PacRunConfig,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PacData> {
    let pac_internal = match operating_point {
        Some(operating_point) => {
            run_pac_internal_from_pss_with_abort(netlist, config, operating_point, abort)?
        }
        None => run_pac_internal_with_abort(netlist, config, abort)?,
    };
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

    let traces_per_sideband = pac_result
        .branch_names
        .len()
        .checked_add(1)
        .ok_or_else(|| ServiceRunError::Failure("PAC trace count overflows usize".to_owned()))?;
    let trace_count = sidebands
        .len()
        .checked_mul(traces_per_sideband)
        .ok_or_else(|| ServiceRunError::Failure("PAC trace count overflows usize".to_owned()))?;
    let mut traces = Vec::new();
    traces.try_reserve_exact(trace_count).map_err(|error| {
        ServiceRunError::Failure(format!("PAC trace allocation failed: {error}"))
    })?;
    for (sideband_index, sideband) in sidebands.iter().enumerate() {
        poll_periodically(abort, sideband_index)?;
        let voltage_name = format!("V({})[sb={:+}]", output_node_name, sideband);
        let mut voltage_values = Vec::new();
        voltage_values
            .try_reserve_exact(frequencies.len())
            .map_err(|error| {
                ServiceRunError::Failure(format!(
                    "PAC voltage-trace allocation failed for '{voltage_name}': {error}"
                ))
            })?;
        for freq_idx in 0..frequencies.len() {
            poll_periodically(abort, freq_idx)?;
            // The configured conversion matrix is the authoritative output
            // channel: unlike the per-node spectra, it includes output_ref
            // for a differential PAC measurement.
            let voltage = pac_result
                .conversion_matrix
                .get(freq_idx, *sideband, 0)
                .map_err(|error| {
                    ServiceRunError::Failure(format!(
                        "PAC conversion result is unavailable: {error}"
                    ))
                })?;
            voltage_values.push(checked_scale_pac_value(
                voltage,
                config.pac_magnitude,
                &voltage_name,
                freq_idx,
            )?);
        }
        traces.push(PacTrace {
            name: voltage_name,
            unit: "V",
            values: voltage_values,
        });

        for (branch_index, branch_name) in pac_result.branch_names.iter().enumerate() {
            poll_periodically(abort, branch_index)?;
            let trace_name = format!("I({branch_name})[sb={sideband:+}]");
            let mut current_values = Vec::new();
            current_values
                .try_reserve_exact(frequencies.len())
                .map_err(|error| {
                    ServiceRunError::Failure(format!(
                        "PAC current-trace allocation failed for '{trace_name}': {error}"
                    ))
                })?;
            for freq_idx in 0..frequencies.len() {
                poll_periodically(abort, freq_idx)?;
                let current = pac_result
                    .get_sideband_data(freq_idx, *sideband)
                    .and_then(|data| data.branch_currents.get(branch_index))
                    .copied()
                    .ok_or_else(|| {
                        ServiceRunError::Failure(format!(
                            "PAC branch current '{branch_name}' is unavailable at frequency point {}, sideband {}",
                            freq_idx + 1,
                            sideband
                        ))
                    })?;
                current_values.push(checked_scale_pac_value(
                    current,
                    config.pac_magnitude,
                    &trace_name,
                    freq_idx,
                )?);
            }
            traces.push(PacTrace {
                name: trace_name,
                unit: "A",
                values: current_values,
            });
        }
    }

    ensure_not_aborted(abort)?;
    Ok(PacData {
        frequencies,
        traces,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    #[test]
    fn pac_service_preserves_typed_entry_abort() {
        let result =
            run_pac_analysis_with_abort("not a netlist", &PacRunConfig::default(), &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    fn one_point_config() -> PacRunConfig {
        PacRunConfig {
            pss_fundamental_freq: 1.0e6,
            pss_num_harmonics: 3,
            pss_tolerance: 1.0e-9,
            start_freq: 1.0e4,
            stop_freq: 1.0e4,
            points_per_unit: 1,
            sweep: PacFrequencySweep::Linear,
            max_sideband: 0,
            input_source: "V1".to_owned(),
            output_node: "out".to_owned(),
            output_ref: None,
            pac_magnitude: 2.0,
            include_dc: true,
            reltol: 1.0e-9,
            abstol: 1.0e-15,
        }
    }

    #[test]
    fn pac_service_emits_exact_unit_bearing_voltage_and_branch_current_traces() {
        let data = run_pac_analysis_with_abort(
            "* exact PAC branch-current fixture\nV1 out 0 DC 0 AC 17 31\nR1 out 0 1k\n.end\n",
            &one_point_config(),
            &NoAbort,
        )
        .expect("PAC service completes");

        assert_eq!(data.frequencies, vec![1.0e4]);
        let voltage = data
            .traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case("V(out)[sb=+0]"))
            .expect("output-voltage trace is retained");
        assert_eq!(voltage.unit, "V");
        assert!((voltage.values[0] - Complex64::new(2.0, 0.0)).norm() <= 1.0e-12);

        let current = data
            .traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case("I(V1)[sb=+0]"))
            .expect("voltage-source branch-current trace is retained");
        assert_eq!(current.unit, "A");
        assert!(
            (current.values[0] - Complex64::new(-2.0e-3, 0.0)).norm() <= 1.0e-12,
            "positive-to-negative source current is {}",
            current.values[0]
        );
    }

    #[test]
    fn pac_result_validation_rejects_branch_cardinality_identity_and_finiteness_drift() {
        let config = one_point_config();
        let valid = || {
            rspice_core::analysis::pac::PacResult::new(
                config.pss_fundamental_freq,
                vec![config.start_freq],
                0,
                0,
                vec!["out".to_owned()],
                vec!["V1".to_owned()],
            )
            .expect("fixture result is valid")
        };

        let basis = config.pss_fundamental_freq;

        let mut truncated = valid();
        truncated
            .get_sideband_data_mut(0, 0)
            .expect("sideband exists")
            .branch_currents
            .clear();
        assert!(
            validate_pac_result(&truncated, &config, basis)
                .expect_err("truncated branch data must fail")
                .to_string()
                .contains("cardinality")
        );

        let mut duplicate = valid();
        duplicate.branch_names.push("v1".to_owned());
        duplicate
            .get_sideband_data_mut(0, 0)
            .expect("sideband exists")
            .branch_currents
            .push(Complex64::new(0.0, 0.0));
        assert!(
            validate_pac_result(&duplicate, &config, basis)
                .expect_err("duplicate identity must fail")
                .to_string()
                .contains("duplicate branch identity")
        );

        let mut nonfinite = valid();
        nonfinite
            .get_sideband_data_mut(0, 0)
            .expect("sideband exists")
            .branch_currents[0] = Complex64::new(Value::NAN, 0.0);
        assert!(
            validate_pac_result(&nonfinite, &config, basis)
                .expect_err("non-finite current must fail")
                .to_string()
                .contains("non-finite")
        );
    }

    /// A result built on a periodic solution other than the carrier this run
    /// was handed is still refused, and now says which two frequencies
    /// disagree instead of naming a "basis or residual" that is neither.
    #[test]
    fn a_conversion_basis_that_is_not_its_carriers_is_refused_by_name() {
        let config = one_point_config();
        let result = rspice_core::analysis::pac::PacResult::new(
            config.pss_fundamental_freq,
            vec![config.start_freq],
            0,
            0,
            vec!["out".to_owned()],
            vec!["V1".to_owned()],
        )
        .expect("fixture result is valid");

        let message = validate_pac_result(&result, &config, config.pss_fundamental_freq * 2.0)
            .expect_err("a basis that is not the carrier's must be refused")
            .to_string();
        assert!(
            message.contains("conversion basis") && message.contains("carrier"),
            "the refusal must name both frequencies: {message}"
        );

        validate_pac_result(&result, &config, config.pss_fundamental_freq)
            .expect("the carrier's own basis is admitted");
    }

    /// The carrier is what the conversion matrix is built on, so the run must
    /// publish the carrier's fundamental even when it is not the number this
    /// configuration authored.
    ///
    /// That gap is exactly an autonomous carrier: the shooting solver holds
    /// the period as an unknown and moves it off the authored guess, so a
    /// comparison against the guess refused every oscillator. It is reproduced
    /// here on a driven carrier because the engine takes the same branch for
    /// both, and because `.PAC`'s exact periodic MNA cannot yet linearize the
    /// behavioural-source oscillator that the autonomous solver converges on.
    #[test]
    fn the_published_basis_is_the_carriers_fundamental_not_the_authored_one() {
        const DECK: &str = "* PAC carrier basis fixture\n\
             v1 in 0 dc 0 ac 1\n\
             r1 in out 1k\n\
             c1 out 0 1n\n\
             .end\n";
        const CARRIER_FUNDAMENTAL: Value = 1.0e6;

        let netlist =
            parse_runner_netlist_with_abort(DECK, None, &NoAbort).expect("the deck parses");
        let engine = build_resolved_periodic_engine(&netlist, 1.0e-9, "fixture producer")
            .expect("the fixture engine resolves");
        let carrier = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(CARRIER_FUNDAMENTAL)
                    .with_harmonics(9)
                    .with_points_per_period(64)
                    .with_tstab_periods(2)
                    .with_tolerance(1.0e-9),
                &NoAbort,
            )
            .expect("the driven RC carrier converges");

        let mut config = one_point_config();
        config.pss_tolerance = 1.0e-9;
        config.output_node = "out".to_owned();
        // Authored one place away from the carrier, which is what an
        // autonomous run produces once the solver has moved the period.
        config.pss_fundamental_freq = CARRIER_FUNDAMENTAL * 1.000_001;
        assert_ne!(
            config.pss_fundamental_freq.to_bits(),
            carrier.analysis().result.frequency.to_bits()
        );

        let internal = run_pac_internal_from_pss_with_abort(&netlist, &config, &carrier, &NoAbort)
            .expect("a run against its own carrier is admitted");

        assert_eq!(
            internal.pac_result.fundamental_frequency.to_bits(),
            carrier.analysis().result.frequency.to_bits(),
            "the published basis is the carrier's, by identity"
        );
    }

    #[test]
    fn pac_magnitude_scaling_rejects_overflow_and_nonzero_erasure() {
        assert!(
            checked_scale_pac_value(Complex64::new(Value::MAX, 0.0), 2.0, "V(out)", 0).is_err()
        );
        assert!(
            checked_scale_pac_value(Complex64::new(Value::from_bits(1), 0.0), 0.5, "I(V1)", 0,)
                .is_err()
        );
    }
}
