#![allow(clippy::too_many_arguments)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::pnoise_sideband::{
    build_pnoise_sideband_translated_frequencies_with_abort, fold_sideband_contributors_with_abort,
    fold_sideband_noise_results_with_abort, resolve_pnoise_sideband_stride_with_abort,
};
use super::{
    PssData, ServiceRunError, ServiceRunResult, build_engine_config,
    generate_freq_points_with_abort, infer_primary_output_node_with_abort,
    infer_primary_source_name_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, normalize_voltage_signal_name,
    parse_runner_netlist_with_abort, run_pss_analysis_with_source_path_and_abort,
};
use crate::output_spec::resolve_node_or_ground_index;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::Engine;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PnoiseRunError {
    Validation(String),
    Resolution(String),
    Data(String),
}

impl fmt::Display for PnoiseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) | Self::Resolution(message) | Self::Data(message) => {
                f.write_str(message)
            }
        }
    }
}

impl std::error::Error for PnoiseRunError {}

impl From<PnoiseRunError> for ServiceRunError {
    fn from(error: PnoiseRunError) -> Self {
        Self::Failure(error.to_string())
    }
}

/// Frequency sweep type for periodic-noise analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnoiseFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PnoiseFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// PNoise noise-reference mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnoiseReference {
    Output,
    Input,
    Phase,
}

/// Explicit configuration for PNoise execution.
#[derive(Debug, Clone)]
pub struct PnoiseRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PnoiseFrequencySweep,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: PnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: Value,
    pub abstol: Value,
}

impl Default for PnoiseRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 5,
            output_node: "VOUT".to_string(),
            output_ref: None,
            input_source: "VIN".to_string(),
            noise_ref: PnoiseReference::Output,
            integrated_noise: false,
            noise_summary: true,
            reltol: 1e-3,
            abstol: 1e-18,
        }
    }
}

impl PnoiseRunConfig {
    fn validate(&self) -> Result<(), PnoiseRunError> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS fundamental frequency".to_string(),
            ));
        }
        if self.pss_num_harmonics == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires at least one PSS harmonic".to_string(),
            ));
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS tolerance".to_string(),
            ));
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err(PnoiseRunError::Validation(
                "PNOISE stop frequency must be >= start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE points per unit must be greater than zero".to_string(),
            ));
        }
        if self.max_sideband < 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE max sideband must be non-negative".to_string(),
            ));
        }
        if self.output_node.trim().is_empty() {
            return Err(PnoiseRunError::Validation(
                "PNOISE output node must be specified".to_string(),
            ));
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE relative tolerance must be positive".to_string(),
            ));
        }
        if !self.abstol.is_finite() || self.abstol < 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE absolute tolerance must be non-negative".to_string(),
            ));
        }
        Ok(())
    }
}

/// PNoise analysis data.
#[derive(Debug, Clone)]
pub struct PnoiseData {
    /// Offset frequencies (Hz).
    pub frequencies: Vec<Value>,
    /// Noise values. Units depend on `reference`:
    /// - Output/Input: V^2/Hz
    /// - Phase: dBc/Hz
    pub output_noise: Vec<Value>,
    /// Optional input-referred noise vector (V^2/Hz), when available.
    pub input_noise: Option<Vec<Value>>,
    /// Optional total integrated output noise (RMS).
    pub total_output_noise: Option<Value>,
    /// Device contributors (name, percentage) at the measured output port.
    pub contributors: Vec<(String, Value)>,
    /// Carrier frequency used for the analysis (Hz).
    pub carrier_frequency: Value,
    /// Effective sideband folding multiplier.
    pub sideband_factor: usize,
    /// Requested noise reference.
    pub reference: PnoiseReference,
    /// Non-fatal caveats for approximations/fallbacks.
    pub warnings: Vec<String>,
}

/// Run PNoise analysis with explicit configuration.
pub fn run_pnoise_analysis_with_config(
    netlist_text: &str,
    config: &PnoiseRunConfig,
) -> Result<PnoiseData, String> {
    run_pnoise_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run PNoise analysis with explicit configuration and cancellation.
pub fn run_pnoise_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    run_pnoise_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PNoise analysis with explicit configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_pnoise_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
) -> Result<PnoiseData, String> {
    run_pnoise_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run PNoise analysis with source-path resolution and cooperative
/// cancellation through periodic solve, sideband folding, and conversion.
pub fn run_pnoise_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    ensure_not_aborted(abort)?;
    config.validate()?;
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    if config.noise_ref == PnoiseReference::Input {
        let source_name = config.input_source.trim();
        if !source_name.is_empty()
            && !netlist_has_independent_source_named_with_abort(&netlist, source_name, abort)?
        {
            return Err(PnoiseRunError::Resolution(format!(
                "PNOISE input source '{}' is not an independent voltage/current source in the netlist",
                source_name
            )).into());
        }
    }

    // PNOISE requires a periodic operating point. We run PSS first and reuse
    // its carrier for phase-noise normalization.
    let pss_data = run_pss_analysis_with_source_path_and_abort(
        netlist_text,
        config.pss_fundamental_freq,
        config.pss_num_harmonics,
        config.pss_tolerance,
        source_path,
        abort,
    )?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let noise_temperature = sim_config.temperature;
    let engine = Engine::new(sim_config);

    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| ServiceRunError::from_core("DC OP error (required for PNOISE)", error))?;
    let output_idx = resolve_node_or_ground_index(config.output_node.trim(), &dc_result.node_names)
        .ok_or_else(|| {
            PnoiseRunError::Resolution(format!(
                "PNOISE output node '{}' not found in node list {:?}",
                config.output_node, dc_result.node_names
            ))
        })?;
    if output_idx == 0 {
        return Err(
            PnoiseRunError::Resolution("PNOISE output node cannot be ground".to_string()).into(),
        );
    }
    let output_ref_idx = config
        .output_ref
        .as_deref()
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node))
        .map(|node| {
            resolve_node_or_ground_index(node, &dc_result.node_names).ok_or_else(|| {
                PnoiseRunError::Resolution(format!(
                    "PNOISE output reference node '{}' not found",
                    node
                ))
            })
        })
        .transpose()?;
    if let Some(ref_idx) = output_ref_idx
        && ref_idx == output_idx
    {
        return Err(PnoiseRunError::Resolution(
            "PNOISE output node and output reference cannot be the same node".to_string(),
        )
        .into());
    }

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    let sideband_stride = resolve_pnoise_sideband_stride_with_abort(config.max_sideband, abort)?;
    let sideband_factor = sideband_stride;
    let translated_frequencies = build_pnoise_sideband_translated_frequencies_with_abort(
        &frequencies,
        pss_data.frequency,
        config.max_sideband,
        abort,
    )?;
    let translated_noise_results = engine
        .run_noise_ports_with_abort(
            &netlist,
            output_idx,
            output_ref_idx,
            &translated_frequencies,
            noise_temperature,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("PNOISE noise analysis error", error))?;
    let folded_output_noise = fold_sideband_noise_results_with_abort(
        &translated_noise_results,
        frequencies.len(),
        sideband_stride,
        "output-referred",
        abort,
        |point| point.output_noise_density.max(0.0),
    )?;

    let mut warnings = Vec::new();

    let mut output_noise = folded_output_noise.clone();
    let mut exact_contributors: Option<Vec<(String, f64)>> = None;
    // The primary output curve and the contributor breakdown come from the
    // engine's cyclostationary conversion-matrix pnoise: it carries the
    // LO-modulated transfers and modulated shot/thermal intensities the
    // stationary sideband fold cannot represent. The fold remains only as
    // the failure fallback and behind the input-referred estimate.
    let pnoise_ref = config
        .output_ref
        .as_deref()
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));
    let pnoise_input = (config.noise_ref == PnoiseReference::Input)
        .then(|| config.input_source.trim())
        .filter(|name| !name.is_empty());
    let mut exact_input_noise: Option<Vec<Value>> = None;
    match engine.run_pnoise_with_abort(
        &netlist,
        pss_data.frequency,
        &frequencies,
        config.output_node.trim(),
        pnoise_ref,
        pnoise_input,
        config.max_sideband.max(1),
        abort,
    ) {
        Ok(exact) => {
            if config.noise_summary {
                let mut total = 0.0;
                for (index, value) in exact.output_noise.iter().enumerate() {
                    poll_periodically(abort, index)?;
                    total += value;
                }
                let mut contributors = Vec::with_capacity(exact.contributors.len());
                for (contributor_index, (name, psds)) in exact.contributors.iter().enumerate() {
                    poll_periodically(abort, contributor_index)?;
                    let mut share = 0.0;
                    for (sample_index, value) in psds.iter().enumerate() {
                        poll_periodically(abort, sample_index)?;
                        share += value;
                    }
                    contributors.push((
                        name.clone(),
                        if total > 0.0 {
                            100.0 * share / total
                        } else {
                            0.0
                        },
                    ));
                }
                exact_contributors = Some(contributors);
            }
            exact_input_noise = exact.input_noise.clone();
            output_noise = exact.output_noise;
        }
        Err(rspice_core::SimulationError::Aborted) => return Err(ServiceRunError::Aborted),
        Err(error) => warnings.push(format!(
            "PNOISE conversion-matrix solve unavailable ({error}); using the stationary sideband approximation"
        )),
    }
    if config.noise_ref == PnoiseReference::Input && exact_input_noise.is_none() {
        warnings.push(
            "PNOISE input-referred estimate uses the stationary sideband approximation".to_string(),
        );
    }
    let mut input_noise = None;
    let total_output_noise = if config.integrated_noise {
        Some(integrate_noise_rms_with_abort(
            &frequencies,
            &output_noise,
            abort,
        )?)
    } else {
        None
    };

    match config.noise_ref {
        PnoiseReference::Output => {}
        PnoiseReference::Input => {
            if let Some(exact) = exact_input_noise.take() {
                input_noise = Some(exact);
            } else {
                let estimate = compute_input_referred_pnoise(
                    &engine,
                    &netlist,
                    output_idx,
                    output_ref_idx,
                    config,
                    frequencies.len(),
                    &translated_frequencies,
                    sideband_stride,
                    noise_temperature,
                    abort,
                )?;
                input_noise = Some(estimate);
            }
        }
        PnoiseReference::Phase => {
            // True oscillator phase noise first: the Demir PPV path solves
            // the autonomous orbit and the phase diffusion constant, which
            // the driven carrier-normalization below cannot represent (the
            // conversion matrix degenerates at the carrier). Falls back to
            // the carrier-normalized driven conversion for forced circuits.
            let osc_config = rspice_core::analysis::PssConfig::autonomous()
                .with_period_guess(1.0 / config.pss_fundamental_freq)
                .with_tstab_periods(30)
                .with_tolerance(config.pss_tolerance)
                .with_max_iterations(60);
            match engine.run_pnoise_oscillator_with_abort(&netlist, osc_config, &frequencies, abort)
            {
                Ok(osc) => {
                    return Ok(PnoiseData {
                        frequencies,
                        output_noise: osc.phase_noise_dbc,
                        input_noise,
                        // The integral of the driven-fold PSD does not
                        // describe the PPV spectrum; omit rather than mix
                        // models.
                        total_output_noise: None,
                        contributors: Vec::new(),
                        carrier_frequency: 1.0 / osc.period,
                        sideband_factor,
                        reference: config.noise_ref,
                        warnings,
                    });
                }
                Err(rspice_core::SimulationError::Aborted) => {
                    return Err(ServiceRunError::Aborted);
                }
                Err(error) => warnings.push(format!(
                    "PNOISE oscillator (PPV) analysis unavailable ({error}); reporting \
                     carrier-normalized driven noise instead"
                )),
            }
            let carrier_rms = estimate_carrier_rms_for_output_with_abort(
                &pss_data,
                config.output_node.trim(),
                config.output_ref.as_deref(),
                abort,
            )?
            .ok_or_else(|| {
                PnoiseRunError::Data(format!(
                    "PNOISE phase-noise conversion could not determine carrier amplitude at '{}'{}",
                    config.output_node,
                    config
                        .output_ref
                        .as_deref()
                        .map(str::trim)
                        .filter(|node| !node.is_empty() && !is_ground_like(node))
                        .map(|node| format!(" referenced to '{}'", node))
                        .unwrap_or_default()
                ))
            })?;
            let carrier_power = (carrier_rms * carrier_rms).max(1e-30);
            for (index, psd) in output_noise.iter_mut().enumerate() {
                poll_periodically(abort, index)?;
                *psd = 10.0 * (psd.max(1e-30) / carrier_power).log10();
            }
        }
    }

    let contributors = if config.noise_summary {
        match exact_contributors {
            Some(exact) => exact,
            None => {
                warnings.push(
                    "PNOISE contributor breakdown uses the stationary sideband approximation"
                        .to_string(),
                );
                fold_sideband_contributors_with_abort(
                    &translated_noise_results,
                    sideband_stride,
                    abort,
                )?
            }
        }
    } else {
        Vec::new()
    };

    ensure_not_aborted(abort)?;
    Ok(PnoiseData {
        frequencies,
        output_noise,
        input_noise,
        total_output_noise,
        contributors,
        carrier_frequency: pss_data.frequency,
        sideband_factor,
        reference: config.noise_ref,
        warnings,
    })
}

fn compute_input_referred_pnoise(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_idx: usize,
    output_ref_idx: Option<usize>,
    config: &PnoiseRunConfig,
    num_offsets: usize,
    translated_frequencies: &[Value],
    sideband_stride: usize,
    temperature: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let fold_input_density = |core_results: &[NoiseResult]| -> ServiceRunResult<Vec<Value>> {
        fold_sideband_noise_results_with_abort(
            core_results,
            num_offsets,
            sideband_stride,
            "input-referred",
            abort,
            |point| point.input_referred_density.max(0.0),
        )
    };

    let source_name = if config.input_source.trim().is_empty() {
        infer_primary_source_name_with_abort(netlist, abort)?.ok_or_else(|| {
            ServiceRunError::Failure(
                "PNOISE input-referred conversion requires an explicit input source or at least one inferable independent source".to_string(),
            )
        })?
    } else {
        config.input_source.trim().to_string()
    };

    let core_results = engine
        .run_noise_with_input_source_and_abort(
            netlist,
            output_idx,
            output_ref_idx,
            &source_name,
            translated_frequencies,
            temperature,
            abort,
        )
        .map_err(|error| {
            ServiceRunError::from_core(
                &format!(
                    "PNOISE input source '{}' failed during source-referred noise evaluation",
                    source_name
                ),
                error,
            )
        })?;

    if core_results.len() != translated_frequencies.len() {
        return Err(ServiceRunError::Failure(format!(
            "PNOISE input source '{}' returned {} translated points, expected {}",
            source_name,
            core_results.len(),
            translated_frequencies.len()
        )));
    }

    fold_input_density(&core_results)
}

/// Run PNoise analysis using inferred/default settings.
pub fn run_pnoise_analysis(netlist_text: &str) -> Result<PnoiseData, String> {
    run_pnoise_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run inferred/default PNoise analysis with cancellation.
pub fn run_pnoise_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    run_pnoise_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run PNoise analysis using inferred/default settings and a source path used
/// to resolve relative includes and model file references.
pub fn run_pnoise_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PnoiseData, String> {
    run_pnoise_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run inferred/default PNoise analysis with source-path resolution and
/// cancellation.
pub fn run_pnoise_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for PNOISE defaults)", error)
        })?;
    let output_node = infer_primary_output_node_with_abort(&dc_result.node_names, abort)?
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "PNOISE could not infer an output node; ensure at least one non-ground node exists"
                    .to_string(),
            )
        })?;
    let input_source =
        infer_primary_source_name_with_abort(&netlist, abort)?.unwrap_or_else(|| "VIN".to_string());

    let cfg = PnoiseRunConfig {
        output_node,
        input_source,
        ..PnoiseRunConfig::default()
    };
    run_pnoise_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &cfg,
        source_path,
        abort,
    )
}

fn find_pss_waveform_by_node_with_abort<'a>(
    pss_data: &'a PssData,
    node: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<&'a [Value]>> {
    let target = normalize_voltage_signal_name(node);
    for (index, (name, values)) in pss_data.waveforms.iter().enumerate() {
        poll_periodically(abort, index)?;
        if normalize_voltage_signal_name(name) == target {
            return Ok(Some(values.as_slice()));
        }
    }
    Ok(None)
}

fn estimate_carrier_rms_for_node_with_abort(
    pss_data: &PssData,
    output_node: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Value>> {
    let target = normalize_voltage_signal_name(output_node);

    for (trace_index, (name, harmonics)) in pss_data.harmonics.iter().enumerate() {
        poll_periodically(abort, trace_index)?;
        if normalize_voltage_signal_name(name) != target {
            continue;
        }
        let mut strongest = None;
        for (harmonic_index, (frequency, magnitude, _)) in harmonics.iter().enumerate() {
            poll_periodically(abort, harmonic_index)?;
            if frequency.is_finite()
                && *frequency > 0.0
                && magnitude.is_finite()
                && *magnitude > 0.0
                && strongest.is_none_or(|current: Value| *magnitude > current)
            {
                strongest = Some(*magnitude);
            }
        }
        if let Some(magnitude) = strongest {
            return Ok(Some(magnitude / 2.0_f64.sqrt()));
        }
    }

    for (trace_index, (name, values)) in pss_data.waveforms.iter().enumerate() {
        poll_periodically(abort, trace_index)?;
        if normalize_voltage_signal_name(name) != target || values.is_empty() {
            continue;
        }
        let mut sum = 0.0;
        for (sample_index, value) in values.iter().enumerate() {
            poll_periodically(abort, sample_index)?;
            sum += value * value;
        }
        let mean_sq = sum / values.len() as Value;
        return Ok((mean_sq > 0.0).then_some(mean_sq.sqrt()));
    }
    Ok(None)
}

fn estimate_carrier_rms_for_output_with_abort(
    pss_data: &PssData,
    output_node: &str,
    output_ref: Option<&str>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Value>> {
    ensure_not_aborted(abort)?;
    let reference = output_ref
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));
    if let Some(reference_node) = reference {
        let Some(out_values) = find_pss_waveform_by_node_with_abort(pss_data, output_node, abort)?
        else {
            return Ok(None);
        };
        let Some(ref_values) =
            find_pss_waveform_by_node_with_abort(pss_data, reference_node, abort)?
        else {
            return Ok(None);
        };
        let count = out_values.len().min(ref_values.len());
        if count == 0 {
            return Ok(None);
        }
        let mut sum = 0.0;
        for (index, (out, reference)) in out_values
            .iter()
            .zip(ref_values.iter())
            .take(count)
            .enumerate()
        {
            poll_periodically(abort, index)?;
            let difference = *out - *reference;
            sum += difference * difference;
        }
        let mean_sq = sum / count as Value;
        return Ok((mean_sq > 0.0).then_some(mean_sq.sqrt()));
    }
    estimate_carrier_rms_for_node_with_abort(pss_data, output_node, abort)
}

fn integrate_noise_rms_with_abort(
    frequencies: &[Value],
    psd: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    if frequencies.len() < 2 || psd.len() < 2 {
        return Ok(0.0);
    }
    let n = frequencies.len().min(psd.len());
    let mut integrated = 0.0;
    for idx in 1..n {
        poll_periodically(abort, idx)?;
        let df = frequencies[idx] - frequencies[idx - 1];
        if df <= 0.0 {
            continue;
        }
        let avg = (psd[idx].max(0.0) + psd[idx - 1].max(0.0)) * 0.5;
        integrated += avg * df;
    }
    ensure_not_aborted(abort)?;
    Ok(integrated.max(0.0).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};

    #[test]
    fn pnoise_service_preserves_typed_entry_abort() {
        let result = run_pnoise_analysis_with_config_and_abort(
            "not a netlist",
            &PnoiseRunConfig::default(),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn pnoise_integration_honors_in_loop_abort() {
        let frequencies = (0..256).map(|index| index as Value).collect::<Vec<_>>();
        let psd = vec![1e-18; frequencies.len()];
        let abort = CountingAbort::new(1);

        let result = integrate_noise_rms_with_abort(&frequencies, &psd, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 1);
    }
}
