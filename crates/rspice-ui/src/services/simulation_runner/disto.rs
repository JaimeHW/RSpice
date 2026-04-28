use super::{
    build_disto_two_tone_harmonic_plan, build_engine_config, generate_freq_points,
    parse_runner_netlist, run_ac_analysis_with_source_path,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::engine::Engine;
use std::fmt;
use std::path::Path;

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
    fn validate(&self) -> Result<(), DistoRunError> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(DistoRunError::Validation(
                "DISTO start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(DistoRunError::Validation(
                "DISTO stop frequency must be greater than start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(DistoRunError::Validation(
                "DISTO points per unit must be greater than zero".to_string(),
            ));
        }
        if let Some(ratio) = self.f2_over_f1
            && (!ratio.is_finite() || ratio <= 1.0)
        {
            return Err(DistoRunError::Validation(
                "DISTO f2_over_f1 must be finite and > 1".to_string(),
            ));
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum DistoRunError {
    Validation(String),
    Parse(String),
    Execution(String),
    Data(String),
}

impl fmt::Display for DistoRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message)
            | Self::Parse(message)
            | Self::Execution(message)
            | Self::Data(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for DistoRunError {}

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
    run_disto_analysis_with_source_path(netlist_text, config, None)
}

/// Run DISTO analysis with a source path used to resolve relative includes and
/// model file references.
pub fn run_disto_analysis_with_source_path(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
) -> Result<DistoData, String> {
    run_disto_analysis_typed(netlist_text, config, source_path).map_err(|error| error.to_string())
}

fn run_disto_analysis_typed(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
) -> Result<DistoData, DistoRunError> {
    config.validate()?;

    match run_disto_analysis_nonlinear_hb(netlist_text, config, source_path) {
        Ok(data) => Ok(data),
        Err(nonlinear_error) => {
            if !config.allow_linearized_fallback {
                return Err(DistoRunError::Execution(format!(
                    "DISTO nonlinear HB path failed ({}). Set allow_linearized_fallback=true to use the lower-fidelity linearized approximation.",
                    nonlinear_error
                )));
            }
            let mut linearized = run_disto_analysis_linearized(netlist_text, config, source_path)?;
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
    source_path: Option<&Path>,
) -> Result<DistoData, DistoRunError> {
    use rspice_core::analysis::{HbConfig, HbTone};

    let netlist = parse_runner_netlist(netlist_text, source_path).map_err(DistoRunError::Parse)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let two_tone_plan = config
        .f2_over_f1
        .map(build_disto_two_tone_harmonic_plan)
        .transpose()
        .map_err(DistoRunError::Execution)?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err(DistoRunError::Data(
            "DISTO sweep generated no frequency points".to_string(),
        ));
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

        let hb = engine.run_hb(&netlist, hb_config).map_err(|e| {
            DistoRunError::Execution(format!("HB DISTO solve failed at {:.6e} Hz: {}", freq, e))
        })?;

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
                    DistoRunError::Data(format!(
                        "HB DISTO solve at {:.6e} Hz is missing spectral voltage for {}",
                        freq, trace_name
                    ))
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
        return Err(DistoRunError::Data(
            "DISTO produced no output traces".to_string(),
        ));
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

fn run_disto_analysis_linearized(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
) -> Result<DistoData, DistoRunError> {
    let f2_over_f1 = config.f2_over_f1.unwrap_or(2.0);
    let max_factor = 3.0_f64
        .max(f2_over_f1 + 1.0)
        .max((2.0 * f2_over_f1 - 1.0).abs())
        .max((2.0 - f2_over_f1).abs())
        .max((f2_over_f1 - 1.0).abs());
    let extended_stop = config.stop_freq * max_factor;

    let ac = run_ac_analysis_with_source_path(
        netlist_text,
        config.start_freq,
        extended_stop,
        config.points_per_unit,
        config.sweep.keyword(),
        source_path,
    )
    .map_err(DistoRunError::Execution)?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err(DistoRunError::Data(
            "DISTO sweep generated no frequency points".to_string(),
        ));
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
        return Err(DistoRunError::Data(
            "DISTO produced no output traces".to_string(),
        ));
    }

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


