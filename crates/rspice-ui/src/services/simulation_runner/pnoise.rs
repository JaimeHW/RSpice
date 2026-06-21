#![allow(clippy::too_many_arguments)]

use super::pnoise_sideband::{
    build_pnoise_sideband_translated_frequencies, fold_sideband_contributors,
    fold_sideband_noise_results, resolve_pnoise_sideband_stride,
};
use super::{
    PssData, build_engine_config, generate_freq_points, infer_primary_output_node,
    infer_primary_source_name, is_ground_like, netlist_has_independent_source_named,
    normalize_voltage_signal_name, parse_runner_netlist, run_pss_analysis_with_source_path,
};
use crate::output_spec::resolve_node_or_ground_index;
use rspice_core::Value;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::Engine;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PnoiseRunError {
    Validation(String),
    Parse(String),
    Resolution(String),
    Execution(String),
    Data(String),
}

impl fmt::Display for PnoiseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message)
            | Self::Parse(message)
            | Self::Resolution(message)
            | Self::Execution(message)
            | Self::Data(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for PnoiseRunError {}

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
    run_pnoise_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run PNoise analysis with explicit configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_pnoise_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
) -> Result<PnoiseData, String> {
    run_pnoise_analysis_with_config_typed(netlist_text, config, source_path)
        .map_err(|error| error.to_string())
}

fn run_pnoise_analysis_with_config_typed(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
) -> Result<PnoiseData, PnoiseRunError> {
    config.validate()?;

    let netlist = parse_runner_netlist(netlist_text, source_path).map_err(PnoiseRunError::Parse)?;
    if config.noise_ref == PnoiseReference::Input {
        let source_name = config.input_source.trim();
        if !source_name.is_empty() && !netlist_has_independent_source_named(&netlist, source_name) {
            return Err(PnoiseRunError::Resolution(format!(
                "PNOISE input source '{}' is not an independent voltage/current source in the netlist",
                source_name
            )));
        }
    }

    // PNOISE requires a periodic operating point. We run PSS first and reuse
    // its carrier for phase-noise normalization.
    let pss_data = run_pss_analysis_with_source_path(
        netlist_text,
        config.pss_fundamental_freq,
        config.pss_num_harmonics,
        config.pss_tolerance,
        source_path,
    )
    .map_err(PnoiseRunError::Execution)?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let noise_temperature = sim_config.temperature;
    let engine = Engine::new(sim_config);

    let dc_result = engine.run_dc_op(&netlist).map_err(|e| {
        PnoiseRunError::Execution(format!("DC OP error (required for PNOISE): {}", e))
    })?;
    let output_idx = resolve_node_or_ground_index(config.output_node.trim(), &dc_result.node_names)
        .ok_or_else(|| {
            PnoiseRunError::Resolution(format!(
                "PNOISE output node '{}' not found in node list {:?}",
                config.output_node, dc_result.node_names
            ))
        })?;
    if output_idx == 0 {
        return Err(PnoiseRunError::Resolution(
            "PNOISE output node cannot be ground".to_string(),
        ));
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
        ));
    }

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    )
    .map_err(PnoiseRunError::Data)?;

    let sideband_stride =
        resolve_pnoise_sideband_stride(config.max_sideband).map_err(PnoiseRunError::Data)?;
    let sideband_factor = sideband_stride;
    let translated_frequencies = build_pnoise_sideband_translated_frequencies(
        &frequencies,
        pss_data.frequency,
        config.max_sideband,
    )
    .map_err(PnoiseRunError::Data)?;
    let translated_noise_results = engine
        .run_noise_ports(
            &netlist,
            output_idx,
            output_ref_idx,
            &translated_frequencies,
            noise_temperature,
        )
        .map_err(|e| PnoiseRunError::Execution(format!("PNOISE noise analysis error: {}", e)))?;
    let folded_output_noise = fold_sideband_noise_results(
        &translated_noise_results,
        frequencies.len(),
        sideband_stride,
        "output-referred",
        |point| point.output_noise_density.max(0.0),
    )
    .map_err(PnoiseRunError::Data)?;

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
    match engine.run_pnoise(
        &netlist,
        pss_data.frequency,
        &frequencies,
        config.output_node.trim(),
        pnoise_ref,
        pnoise_input,
        config.max_sideband.max(1),
    ) {
        Ok(exact) => {
            if config.noise_summary {
                let total: f64 = exact.output_noise.iter().sum();
                exact_contributors = Some(
                    exact
                        .contributors
                        .iter()
                        .map(|(name, psds)| {
                            let share: f64 = psds.iter().sum();
                            let pct = if total > 0.0 {
                                100.0 * share / total
                            } else {
                                0.0
                            };
                            (name.clone(), pct)
                        })
                        .collect(),
                );
            }
            exact_input_noise = exact.input_noise.clone();
            output_noise = exact.output_noise;
        }
        Err(e) => warnings.push(format!(
            "PNOISE conversion-matrix solve unavailable ({e}); using the stationary sideband approximation"
        )),
    }
    if config.noise_ref == PnoiseReference::Input && exact_input_noise.is_none() {
        warnings.push(
            "PNOISE input-referred estimate uses the stationary sideband approximation".to_string(),
        );
    }
    let mut input_noise = None;
    let total_output_noise = if config.integrated_noise {
        Some(integrate_noise_rms(&frequencies, &output_noise))
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
            match engine.run_pnoise_oscillator(&netlist, osc_config, &frequencies) {
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
                Err(e) => warnings.push(format!(
                    "PNOISE oscillator (PPV) analysis unavailable ({e}); reporting \
                     carrier-normalized driven noise instead"
                )),
            }
            let carrier_rms = estimate_carrier_rms_for_output(
                &pss_data,
                config.output_node.trim(),
                config.output_ref.as_deref(),
            )
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
            output_noise = output_noise
                .iter()
                .map(|psd| 10.0 * (psd.max(1e-30) / carrier_power).log10())
                .collect();
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
                fold_sideband_contributors(&translated_noise_results, sideband_stride)
                    .map_err(PnoiseRunError::Data)?
            }
        }
    } else {
        Vec::new()
    };

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
) -> Result<Vec<Value>, PnoiseRunError> {
    let fold_input_density = |core_results: &[NoiseResult]| -> Result<Vec<Value>, PnoiseRunError> {
        fold_sideband_noise_results(
            core_results,
            num_offsets,
            sideband_stride,
            "input-referred",
            |point| point.input_referred_density.max(0.0),
        )
        .map_err(PnoiseRunError::Data)
    };

    let source_name = if config.input_source.trim().is_empty() {
        infer_primary_source_name(netlist).ok_or_else(|| {
            PnoiseRunError::Resolution(
                "PNOISE input-referred conversion requires an explicit input source or at least one inferable independent source".to_string(),
            )
        })?
    } else {
        config.input_source.trim().to_string()
    };

    let core_results = engine
        .run_noise_with_input_source(
            netlist,
            output_idx,
            output_ref_idx,
            &source_name,
            translated_frequencies,
            temperature,
        )
        .map_err(|error| {
            PnoiseRunError::Execution(format!(
                "PNOISE input source '{}' failed during source-referred noise evaluation: {}",
                source_name, error
            ))
        })?;

    if core_results.len() != translated_frequencies.len() {
        return Err(PnoiseRunError::Data(format!(
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
    run_pnoise_analysis_with_source_path(netlist_text, None)
}

/// Run PNoise analysis using inferred/default settings and a source path used
/// to resolve relative includes and model file references.
pub fn run_pnoise_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PnoiseData, String> {
    run_pnoise_analysis_typed(netlist_text, source_path).map_err(|error| error.to_string())
}

fn run_pnoise_analysis_typed(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PnoiseData, PnoiseRunError> {
    let netlist = parse_runner_netlist(netlist_text, source_path).map_err(PnoiseRunError::Parse)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine.run_dc_op(&netlist).map_err(|e| {
        PnoiseRunError::Execution(format!("DC OP error (required for PNOISE defaults): {}", e))
    })?;
    let output_node = infer_primary_output_node(&dc_result.node_names).ok_or_else(|| {
        PnoiseRunError::Resolution(
            "PNOISE could not infer an output node; ensure at least one non-ground node exists"
                .to_string(),
        )
    })?;
    let input_source = infer_primary_source_name(&netlist).unwrap_or_else(|| "VIN".to_string());

    let cfg = PnoiseRunConfig {
        output_node,
        input_source,
        ..PnoiseRunConfig::default()
    };
    run_pnoise_analysis_with_config_typed(netlist_text, &cfg, source_path)
}

fn find_pss_waveform_by_node<'a>(pss_data: &'a PssData, node: &str) -> Option<&'a [Value]> {
    let target = normalize_voltage_signal_name(node);
    pss_data
        .waveforms
        .iter()
        .find(|(name, _)| normalize_voltage_signal_name(name) == target)
        .map(|(_, values)| values.as_slice())
}

fn estimate_carrier_rms_for_node(pss_data: &PssData, output_node: &str) -> Option<Value> {
    let target = normalize_voltage_signal_name(output_node);

    if let Some((_, harmonics)) = pss_data
        .harmonics
        .iter()
        .find(|(name, _)| normalize_voltage_signal_name(name) == target)
        && let Some((_, magnitude, _)) = harmonics
            .iter()
            .filter(|(frequency, magnitude, _)| {
                frequency.is_finite()
                    && *frequency > 0.0
                    && magnitude.is_finite()
                    && *magnitude > 0.0
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
    {
        return Some(*magnitude / 2.0_f64.sqrt());
    }

    pss_data
        .waveforms
        .iter()
        .find(|(name, _)| normalize_voltage_signal_name(name) == target)
        .and_then(|(_, values)| {
            if values.is_empty() {
                return None;
            }
            let mean_sq =
                values.iter().map(|value| value * value).sum::<Value>() / values.len() as Value;
            (mean_sq > 0.0).then_some(mean_sq.sqrt())
        })
}

fn estimate_carrier_rms_for_output(
    pss_data: &PssData,
    output_node: &str,
    output_ref: Option<&str>,
) -> Option<Value> {
    let reference = output_ref
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));
    if let Some(reference_node) = reference {
        let out_values = find_pss_waveform_by_node(pss_data, output_node)?;
        let ref_values = find_pss_waveform_by_node(pss_data, reference_node)?;
        let count = out_values.len().min(ref_values.len());
        if count == 0 {
            return None;
        }
        let mean_sq = out_values
            .iter()
            .zip(ref_values.iter())
            .take(count)
            .map(|(out, reference)| {
                let vdiff = *out - *reference;
                vdiff * vdiff
            })
            .sum::<Value>()
            / count as Value;
        return (mean_sq > 0.0).then_some(mean_sq.sqrt());
    }
    estimate_carrier_rms_for_node(pss_data, output_node)
}

fn integrate_noise_rms(frequencies: &[Value], psd: &[Value]) -> Value {
    if frequencies.len() < 2 || psd.len() < 2 {
        return 0.0;
    }
    let n = frequencies.len().min(psd.len());
    let mut integrated = 0.0;
    for idx in 1..n {
        let df = frequencies[idx] - frequencies[idx - 1];
        if df <= 0.0 {
            continue;
        }
        let avg = (psd[idx].max(0.0) + psd[idx - 1].max(0.0)) * 0.5;
        integrated += avg * df;
    }
    integrated.max(0.0).sqrt()
}
