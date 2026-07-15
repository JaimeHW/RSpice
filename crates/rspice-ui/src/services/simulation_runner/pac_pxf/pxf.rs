use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;

use super::super::error::{ensure_not_aborted, poll_periodically};
use super::super::{
    ServiceRunError, ServiceRunResult, build_engine_config, build_voltage_output_expr,
    infer_primary_output_node_with_abort, infer_primary_source_name_with_abort, is_ground_like,
    parse_runner_netlist_with_abort,
};
use super::pac::{PacFrequencySweep, PacRunConfig, run_pac_internal_with_abort};
use super::shared::normalize_pac_node_name;
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
            && reference.eq_ignore_ascii_case(self.output_node.trim())
        {
            return Err("PXF output node and output reference cannot be the same node".to_string());
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
    run_pxf_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run PXF analysis with explicit configuration and cancellation.
pub fn run_pxf_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    run_pxf_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PXF analysis with explicit configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_pxf_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &PxfRunConfig,
    source_path: Option<&Path>,
) -> Result<PxfData, String> {
    run_pxf_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run PXF analysis with source-path resolution and cancellation.
pub fn run_pxf_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pxf_analysis_for_netlist_with_abort(&netlist, config, abort)
}

fn run_pxf_analysis_for_netlist_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PxfRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    use rspice_core::analysis::advanced::pxf::PxfConfig;

    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

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

    let pac_internal = run_pac_internal_with_abort(netlist, &pac_cfg, abort)?;
    let pac_result = pac_internal.pac_result;

    let sideband_indices = pac_result.conversion_matrix.sideband_indices();
    if !sideband_indices.contains(&config.input_sideband) {
        return Err(ServiceRunError::Failure(format!(
            "PXF input sideband {} is outside analyzed PAC sideband range {:?}",
            config.input_sideband, sideband_indices
        )));
    }
    if !sideband_indices.contains(&config.output_sideband) {
        return Err(ServiceRunError::Failure(format!(
            "PXF output sideband {} is outside analyzed PAC sideband range {:?}",
            config.output_sideband, sideband_indices
        )));
    }

    let frequencies = pac_result.conversion_matrix.frequencies().to_vec();
    if frequencies.is_empty() {
        return Err(ServiceRunError::Failure(
            "PXF conversion matrix has no frequency points".to_string(),
        ));
    }

    let mut matrix_cube: Vec<Vec<Vec<Complex64>>> = Vec::with_capacity(frequencies.len());
    for freq_idx in 0..frequencies.len() {
        poll_periodically(abort, freq_idx)?;
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
        .map_err(|error| ServiceRunError::Failure(format!("PXF configuration error: {error}")))?;

    let pxf_result = analyze_conversion_matrix_with_abort(
        &pxf_cfg,
        &frequencies,
        &matrix_cube,
        pac_result.fundamental_frequency,
        abort,
    )?;

    if pxf_result.points.is_empty() {
        return Err(ServiceRunError::Failure(
            "PXF produced no transfer points".to_string(),
        ));
    }

    let mut sweep_freqs = Vec::with_capacity(pxf_result.points.len());
    let mut output_freqs = Vec::with_capacity(pxf_result.points.len());
    let mut transfer = Vec::with_capacity(pxf_result.points.len());
    let mut magnitude_db = Vec::with_capacity(pxf_result.points.len());
    let mut phase_deg = Vec::with_capacity(pxf_result.points.len());
    for (index, point) in pxf_result.points.iter().enumerate() {
        poll_periodically(abort, index)?;
        sweep_freqs.push(point.freq_in);
        output_freqs.push(point.freq_out);
        transfer.push(point.transfer);
        magnitude_db.push(point.magnitude_db());
        phase_deg.push(point.phase_degrees());
    }

    let group_delay_curve = pxf_group_delay_with_abort(&pxf_result.points, abort)?;
    let group_delay = (!group_delay_curve.is_empty()).then_some(group_delay_curve);

    let output_label =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());

    ensure_not_aborted(abort)?;
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

fn analyze_conversion_matrix_with_abort(
    config: &rspice_core::analysis::advanced::pxf::PxfConfig,
    frequencies: &[Value],
    conversion_matrix: &[Vec<Vec<Complex64>>],
    fundamental_frequency: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::analysis::advanced::pxf::PxfResult> {
    use rspice_core::analysis::advanced::pxf::{PxfResult, TransferPoint};

    ensure_not_aborted(abort)?;
    if frequencies.is_empty() {
        return Err(ServiceRunError::Failure(
            "PXF conversion analysis has no frequency points".to_string(),
        ));
    }
    if conversion_matrix.len() != frequencies.len() {
        return Err(ServiceRunError::Failure(
            "PXF frequency/conversion-matrix size mismatch".to_string(),
        ));
    }

    let mut result = PxfResult::new(
        fundamental_frequency,
        config.input_sideband,
        config.output_sideband,
    );
    for (index, frequency) in frequencies.iter().copied().enumerate() {
        poll_periodically(abort, index)?;
        let matrix = &conversion_matrix[index];
        let Some(first_row) = matrix.first() else {
            continue;
        };
        let offset = (matrix.len() as i32 - 1) / 2;
        let output_index = (config.output_sideband + offset) as usize;
        let input_index = (config.input_sideband + offset) as usize;
        if output_index >= matrix.len() || input_index >= first_row.len() {
            continue;
        }
        let Some(transfer) = matrix
            .get(output_index)
            .and_then(|row| row.get(input_index))
            .copied()
        else {
            continue;
        };
        result.add_point(TransferPoint {
            freq_in: frequency,
            freq_out: frequency
                + (config.output_sideband - config.input_sideband) as Value * fundamental_frequency,
            transfer,
            sideband_in: config.input_sideband,
            sideband_out: config.output_sideband,
        });
    }
    compute_pxf_metrics_with_abort(&mut result, abort)?;
    Ok(result)
}

fn compute_pxf_metrics_with_abort(
    result: &mut rspice_core::analysis::advanced::pxf::PxfResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    let mut peak: Option<(Value, Value)> = None;
    for (index, point) in result.points.iter().enumerate() {
        poll_periodically(abort, index)?;
        let gain = point.magnitude_db();
        if gain.is_finite() && peak.is_none_or(|(_, current)| gain > current) {
            peak = Some((point.freq_in, gain));
        }
    }
    result.peak_gain = peak;

    if let Some((peak_frequency, peak_db)) = peak {
        let threshold = peak_db - 3.0;
        let mut lower = None;
        let mut upper = None;
        for (index, point) in result.points.iter().enumerate() {
            poll_periodically(abort, index)?;
            if point.magnitude_db() <= threshold {
                if point.freq_in < peak_frequency {
                    lower = Some(point.freq_in);
                } else if point.freq_in > peak_frequency && upper.is_none() {
                    upper = Some(point.freq_in);
                }
            }
        }
        result.bandwidth_3db = match (lower, upper, result.points.first(), result.points.last()) {
            (Some(lower), Some(upper), _, _) => Some(upper - lower),
            (None, Some(upper), Some(first), _) => Some(upper - first.freq_in),
            (Some(lower), None, _, Some(last)) => Some(last.freq_in - lower),
            _ => None,
        };
    }

    for (index, window) in result.points.windows(2).enumerate() {
        poll_periodically(abort, index)?;
        let first = window[0].magnitude_db();
        let second = window[1].magnitude_db();
        if (first >= 0.0 && second < 0.0) || (first < 0.0 && second >= 0.0) {
            let fraction = -first / (second - first);
            result.unity_gain_freq =
                Some(window[0].freq_in + fraction * (window[1].freq_in - window[0].freq_in));
            break;
        }
    }
    if let Some(first) = result.points.first()
        && first.freq_in < 100.0
    {
        result.dc_gain = Some(first.transfer);
    }
    ensure_not_aborted(abort)
}

fn pxf_group_delay_with_abort(
    points: &[rspice_core::analysis::advanced::pxf::TransferPoint],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(Value, Value)>> {
    let mut group_delay = Vec::with_capacity(points.len().saturating_sub(1));
    for (index, window) in points.windows(2).enumerate() {
        poll_periodically(abort, index)?;
        group_delay.push((
            (window[0].freq_in + window[1].freq_in) * 0.5,
            window[0].group_delay(&window[1]),
        ));
    }
    ensure_not_aborted(abort)?;
    Ok(group_delay)
}

/// Run PXF analysis using inferred/default settings.
pub fn run_pxf_analysis(netlist_text: &str) -> Result<PxfData, String> {
    run_pxf_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run inferred/default PXF analysis with cancellation.
pub fn run_pxf_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    run_pxf_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run PXF analysis using inferred/default settings and a source path used to
/// resolve relative includes and model file references.
pub fn run_pxf_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PxfData, String> {
    run_pxf_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run inferred/default PXF analysis with source-path resolution and
/// cancellation.
pub fn run_pxf_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for PXF defaults)", error)
        })?;
    let input_source = infer_primary_source_name_with_abort(&netlist, abort)?.ok_or_else(|| {
        ServiceRunError::Failure(
            "PXF requires at least one independent source in the netlist".to_string(),
        )
    })?;
    let output_node = infer_primary_output_node_with_abort(&dc_result.node_names, abort)?
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "PXF could not infer an output node; ensure at least one non-ground node exists"
                    .to_string(),
            )
        })?;

    let cfg = PxfRunConfig {
        input_source,
        output_node,
        ..PxfRunConfig::default()
    };
    run_pxf_analysis_for_netlist_with_abort(&netlist, &cfg, abort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};

    #[test]
    fn pxf_service_preserves_typed_entry_abort() {
        let result = run_pxf_analysis_with_config_and_abort(
            "not a netlist",
            &PxfRunConfig::default(),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn pxf_conversion_honors_in_loop_abort() {
        const POINTS: usize = 130;
        let mut config = rspice_core::analysis::advanced::pxf::PxfConfig::new();
        config.input_sideband = 0;
        config.output_sideband = 0;
        let frequencies = (1..=POINTS).map(|point| point as Value).collect::<Vec<_>>();
        let conversion_matrix = vec![vec![vec![Complex64::new(1.0, 0.0)]]; POINTS];
        let abort = CountingAbort::new(2);

        let result = analyze_conversion_matrix_with_abort(
            &config,
            &frequencies,
            &conversion_matrix,
            1e6,
            &abort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 2);
    }
}
