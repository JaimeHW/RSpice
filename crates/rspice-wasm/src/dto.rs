//! Serializable snapshots of core results and parser diagnostics.
//!
//! These are compatibility data-transfer objects: they mirror one core result
//! shape each, keep its ordering, and represent a channel that authored
//! output projection excluded as explicit absence rather than as a zero.

use rspice_core::engine::{
    TransientFftHarmonic, TransientFftMetrics, TransientFftResult, TransientResult,
    TransientResultCompressed,
};
use rspice_core::netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetlistSummary {
    pub title: String,
    pub element_count: usize,
    pub analysis_count: usize,
    pub model_count: usize,
    pub subcircuit_count: usize,
    pub parameter_count: usize,
    pub diagnostics: Vec<WasmDiagnostic>,
    #[serde(default)]
    pub startup_diagnostics: Vec<WasmStartupDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmDiagnostic {
    pub line: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmSourceLocation {
    pub source: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDirectiveScope {
    pub kind: String,
    pub qualified_definition: Option<String>,
    pub qualified_instances: Vec<String>,
}

/// Stable structured representation of a non-fatal `.IC`/`.NODESET`
/// semantic diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDiagnostic {
    pub code: String,
    pub stage: String,
    pub directive: String,
    pub origins: Vec<WasmSourceLocation>,
    pub scopes: Vec<WasmStartupDirectiveScope>,
    pub canonical_nodes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcOperatingPoint {
    pub node_names: Vec<String>,
    pub node_voltages: Vec<f64>,
    pub branch_names: Vec<String>,
    pub branch_currents: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSeries {
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcPointSnapshot {
    pub frequency: f64,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub voltages: ComplexSeries,
    pub currents: ComplexSeries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientSnapshot {
    pub time: Vec<f64>,
    /// Exact accepted integration intervals aligned with `time`.
    pub step_sizes: Vec<f64>,
    /// Core node count, retained explicitly so schema drift cannot hide an
    /// incomplete name or waveform inventory.
    pub num_nodes: usize,
    pub node_names: Vec<String>,
    /// Node waveforms in core node order. A projected-out waveform is `None`
    /// (`null` in JavaScript), while a retained zero-point waveform is an
    /// explicitly present empty typed array.
    pub voltages: Vec<Option<Vec<f64>>>,
    /// Branch identities in the same stable order as `branch_currents`.
    pub branch_names: Vec<String>,
    /// Branch-current waveforms in core branch order. `None` means the known
    /// branch was deliberately projected out of the result.
    pub branch_currents: Vec<Option<Vec<f64>>>,
    /// Requested device operating-point channels in core discovery order.
    pub device_op_traces: Vec<TransientDeviceOpSnapshot>,
    /// Typed non-solution device-store channels in core topology order.
    pub store_traces: Vec<TransientStoreSnapshot>,
    /// Source-authored transient FFT results in declaration order.
    pub fft_results: Vec<TransientFftSnapshot>,
    /// Compression provenance. Full accepted-grid results use `None`; a
    /// compressed result reports its original and retained point counts.
    pub compression: Option<TransientCompressionSnapshot>,
}

/// One requested device operating-point history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientDeviceOpSnapshot {
    pub device_name: String,
    pub parameter: String,
    pub values: Vec<f64>,
}

/// One typed, non-solution device-store history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientStoreSnapshot {
    pub name: String,
    pub values: Vec<f64>,
}

/// Provenance for a compressed transient result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientCompressionSnapshot {
    /// Version of the compression evidence contract.
    pub schema_version: u32,
    /// Stable compression-algorithm identifier.
    pub algorithm: String,
    /// Sample domain over which reconstruction error was measured.
    pub sample_domain: String,
    /// Whether decimation was enabled.
    pub enabled: bool,
    /// Applied absolute tolerance in each signal's native unit.
    pub absolute_tolerance: f64,
    /// Applied relative tolerance.
    pub relative_tolerance: f64,
    /// Applied maximum interval between retained samples.
    pub maximum_retained_interval: f64,
    pub input_points: usize,
    pub retained_points: usize,
    pub compression_ratio: f64,
    /// Worst final-grid reconstruction error, selected by tolerance use.
    pub worst_observed: Option<TransientCompressionErrorSnapshot>,
}

/// Browser-facing final-grid compression-error evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientCompressionErrorSnapshot {
    pub signal_kind: String,
    pub canonical_name: String,
    pub input_sample_index: usize,
    pub time: f64,
    pub actual_value: f64,
    pub absolute_error: f64,
    pub relative_error: Option<f64>,
    pub allowed_tolerance: f64,
    pub tolerance_utilization: f64,
}

/// Columnar FFT bins. The JavaScript export materializes every field as a
/// typed array while the Rust API retains ordinary owned vectors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftBinsSnapshot {
    pub indices: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

/// Columnar magnitude-ranked harmonic report. Ordering is the exact ordering
/// produced by the core (descending magnitude, then source bin).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftHarmonicsSnapshot {
    pub ranks: Vec<usize>,
    pub bins: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub magnitudes_db: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

/// Optional Xyce-compatible FFT figures emitted when `FFTOUT=1`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftMetricsSnapshot {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency: Option<f64>,
    pub largest_harmonics: TransientFftHarmonicsSnapshot,
}

/// Complete browser representation of one core `TransientFftResult`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftSnapshot {
    /// Stable source-order identity for this transient post-process result.
    pub analysis_id: String,
    /// Stable identity of the direct transient result consumed by this FFT.
    pub parent_analysis_id: String,
    /// One-based source-order ordinal among FFT directives.
    pub ordinal: usize,
    /// `probe` or `expression`, allowing consumers to interpret `source_text`
    /// without parsing it heuristically.
    pub source_kind: String,
    /// Canonical probe spelling or the expression body retained by the parser.
    pub source_text: String,
    /// Display spelling of the authored source; expression bodies include
    /// their braces here.
    pub authored_output: String,
    /// Resolved scalar result-column spelling.
    pub output_name: String,
    pub physical_type: String,
    /// Effective unit of Cartesian coefficients, magnitudes, and
    /// magnitude-like metrics. Normalized spectra use `1` while retaining
    /// `physical_type`; an unnormalized parameter has no known unit.
    pub value_unit: Option<String>,
    pub start_time: f64,
    pub stop_time: f64,
    pub sample_interval: f64,
    pub point_count: usize,
    pub accurate_sampling: bool,
    pub format: String,
    pub mode: String,
    pub window: String,
    pub window_name: String,
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    pub bins: TransientFftBinsSnapshot,
    /// `null` in JavaScript when `FFTOUT` was not requested.
    pub metrics: Option<TransientFftMetricsSnapshot>,
}

/// Browser-facing parser-to-solver readiness result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasmHealthReport {
    pub status: String,
    pub ready: bool,
    pub duration_seconds: f64,
    pub element_count: usize,
    pub node_count: usize,
    pub branch_count: usize,
    pub output_voltage: f64,
}

pub(crate) fn complex_series_from_slice(values: &[rspice_core::Complex64]) -> ComplexSeries {
    ComplexSeries {
        real: values.iter().map(|value| value.re).collect(),
        imag: values.iter().map(|value| value.im).collect(),
    }
}

pub(crate) fn fft_output_identity(output: &FftOutput) -> (&'static str, &str, String) {
    match output {
        FftOutput::Probe(probe) => ("probe", probe, probe.clone()),
        FftOutput::Expression(expression) => {
            ("expression", expression, format!("{{{expression}}}"))
        }
    }
}

pub(crate) const fn fft_format_name(format: FftFormat) -> &'static str {
    match format {
        FftFormat::Normalized => "normalized",
        FftFormat::Unnormalized => "unnormalized",
    }
}

pub(crate) const fn fft_mode_name(mode: XyceFftMode) -> &'static str {
    match mode {
        XyceFftMode::HspiceCompatible => "hspice_compatible",
        XyceFftMode::SpectreCompatible => "spectre_compatible",
    }
}

pub(crate) const fn fft_window_name(window: FftWindow) -> &'static str {
    match window {
        FftWindow::Rectangular => "rectangular",
        FftWindow::Bartlett => "bartlett",
        FftWindow::BartlettHann => "bartlett_hann",
        FftWindow::Hamming => "hamming",
        FftWindow::Hann => "hann",
        FftWindow::Blackman67Db => "blackman_67db",
        FftWindow::Blackman => "blackman",
        FftWindow::BlackmanHarris => "blackman_harris",
        FftWindow::Nuttall => "nuttall",
        FftWindow::HalfCycleSine => "half_cycle_sine",
        FftWindow::HalfCycleSine3 => "half_cycle_sine_3",
        FftWindow::HalfCycleSine6 => "half_cycle_sine_6",
        FftWindow::Cosine2 => "cosine_2",
        FftWindow::Cosine4 => "cosine_4",
    }
}

pub(crate) fn fft_harmonics_snapshot(
    harmonics: &[TransientFftHarmonic],
) -> TransientFftHarmonicsSnapshot {
    TransientFftHarmonicsSnapshot {
        ranks: harmonics.iter().map(|harmonic| harmonic.rank).collect(),
        bins: harmonics.iter().map(|harmonic| harmonic.bin).collect(),
        frequencies: harmonics
            .iter()
            .map(|harmonic| harmonic.frequency)
            .collect(),
        magnitudes: harmonics
            .iter()
            .map(|harmonic| harmonic.magnitude)
            .collect(),
        magnitudes_db: harmonics
            .iter()
            .map(|harmonic| harmonic.magnitude_db)
            .collect(),
        phase_degrees: harmonics
            .iter()
            .map(|harmonic| harmonic.phase_degrees)
            .collect(),
    }
}

pub(crate) fn fft_metrics_snapshot(metrics: &TransientFftMetrics) -> TransientFftMetricsSnapshot {
    TransientFftMetricsSnapshot {
        fundamental_magnitude: metrics.fundamental_magnitude,
        thd_ratio: metrics.thd_ratio,
        thd_db: metrics.thd_db,
        sndr_db: metrics.sndr_db,
        enob_bits: metrics.enob_bits,
        snr_db: metrics.snr_db,
        sfdr_db: metrics.sfdr_db,
        sfdr_spur_bin: metrics.sfdr_spur_bin,
        sfdr_spur_frequency: metrics.sfdr_spur_frequency,
        largest_harmonics: fft_harmonics_snapshot(&metrics.largest_harmonics),
    }
}

pub(crate) fn fft_value_unit(
    physical_type: &str,
    format: FftFormat,
) -> Result<Option<&'static str>, String> {
    let physical_unit = match physical_type {
        "voltage" => Some("V"),
        "current" => Some("A"),
        "parameter" => None,
        other => {
            return Err(format!("unsupported transient FFT physical type '{other}'"));
        }
    };
    Ok(match format {
        FftFormat::Normalized => Some("1"),
        FftFormat::Unnormalized => physical_unit,
    })
}

pub(crate) fn fft_snapshot(
    result: &TransientFftResult,
    ordinal: usize,
    parent_analysis_id: &str,
) -> Result<TransientFftSnapshot, String> {
    let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
    let value_unit = fft_value_unit(result.physical_type, result.format)?;
    Ok(TransientFftSnapshot {
        analysis_id: format!("fft-{ordinal:03}"),
        parent_analysis_id: parent_analysis_id.to_owned(),
        ordinal,
        source_kind: source_kind.to_string(),
        source_text: source_text.to_string(),
        authored_output,
        output_name: result.output_name.clone(),
        physical_type: result.physical_type.to_string(),
        value_unit: value_unit.map(str::to_string),
        start_time: result.start_time,
        stop_time: result.stop_time,
        sample_interval: result.sample_interval,
        point_count: result.point_count,
        accurate_sampling: result.accurate_sampling,
        format: fft_format_name(result.format).to_string(),
        mode: fft_mode_name(result.mode).to_string(),
        window: fft_window_name(result.window).to_string(),
        window_name: result.window_name.clone(),
        alpha: result.alpha,
        coherent_gain: result.coherent_gain,
        frequency_resolution: result.frequency_resolution,
        fundamental_bin: result.fundamental_bin,
        minimum_metric_bin: result.minimum_metric_bin,
        maximum_metric_bin: result.maximum_metric_bin,
        bins: TransientFftBinsSnapshot {
            indices: result.bins.iter().map(|bin| bin.index).collect(),
            frequencies: result.bins.iter().map(|bin| bin.frequency).collect(),
            real: result.bins.iter().map(|bin| bin.real).collect(),
            imaginary: result.bins.iter().map(|bin| bin.imaginary).collect(),
            magnitudes: result.bins.iter().map(|bin| bin.magnitude).collect(),
            phase_degrees: result.bins.iter().map(|bin| bin.phase_degrees).collect(),
        },
        metrics: result.metrics.as_ref().map(fft_metrics_snapshot),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_transient_analog_inventory(
    time: &[f64],
    step_sizes: &[f64],
    num_nodes: usize,
    node_names: &[String],
    voltages: &[Vec<f64>],
    branch_names: &[String],
    branch_currents: &[Vec<f64>],
    device_op_traces: &[rspice_core::engine::TransientDeviceOpTrace],
    store_traces: &[rspice_core::engine::TransientStoreTrace],
) -> Result<(), String> {
    let point_count = time.len();
    if step_sizes.len() != point_count {
        return Err(format!(
            "transient result has {} step sizes for {point_count} time points",
            step_sizes.len()
        ));
    }
    if num_nodes != node_names.len() || num_nodes != voltages.len() {
        return Err(format!(
            "transient result declares {num_nodes} nodes but has {} node names and {} voltage channels",
            node_names.len(),
            voltages.len()
        ));
    }
    if branch_names.len() != branch_currents.len() {
        return Err(format!(
            "transient result has {} branch names but {} branch-current channels",
            branch_names.len(),
            branch_currents.len()
        ));
    }
    if time
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
        || time.last().is_some_and(|value| !value.is_finite())
    {
        return Err(
            "transient result time points must be finite and strictly increasing".to_string(),
        );
    }
    if step_sizes
        .iter()
        .any(|step| !step.is_finite() || *step < 0.0)
    {
        return Err("transient result step sizes must be finite and non-negative".to_string());
    }

    for (kind, name, values, may_be_projected_out) in
        voltages
            .iter()
            .enumerate()
            .map(|(index, values)| ("voltage", node_names[index].as_str(), values, true))
            .chain(branch_currents.iter().enumerate().map(|(index, values)| {
                ("branch-current", branch_names[index].as_str(), values, true)
            }))
            .chain(device_op_traces.iter().map(|trace| {
                (
                    "device operating-point",
                    trace.parameter.as_str(),
                    &trace.values,
                    false,
                )
            }))
            .chain(
                store_traces
                    .iter()
                    .map(|trace| ("device store", trace.name.as_str(), &trace.values, false)),
            )
    {
        if values.len() != point_count && !(may_be_projected_out && values.is_empty()) {
            return Err(format!(
                "transient {kind} channel '{name}' has {} values for {point_count} time points",
                values.len()
            ));
        }
    }
    Ok(())
}

pub(crate) fn solution_waveforms(
    waveforms: Vec<Vec<f64>>,
    point_count: usize,
) -> Vec<Option<Vec<f64>>> {
    waveforms
        .into_iter()
        .map(|waveform| {
            if waveform.is_empty() && point_count != 0 {
                None
            } else {
                Some(waveform)
            }
        })
        .collect()
}

pub(crate) fn device_op_snapshots(
    traces: Vec<rspice_core::engine::TransientDeviceOpTrace>,
) -> Vec<TransientDeviceOpSnapshot> {
    traces
        .into_iter()
        .map(|trace| TransientDeviceOpSnapshot {
            device_name: trace.device_name,
            parameter: trace.parameter,
            values: trace.values,
        })
        .collect()
}

pub(crate) fn store_snapshots(
    traces: Vec<rspice_core::engine::TransientStoreTrace>,
) -> Vec<TransientStoreSnapshot> {
    traces
        .into_iter()
        .map(|trace| TransientStoreSnapshot {
            name: trace.name,
            values: trace.values,
        })
        .collect()
}

/// Convert a complete core transient result into the loss-aware browser DTO.
/// Solution-channel vector order is preserved exactly; an empty projected-out
/// voltage or branch-current channel becomes typed `None`/JavaScript `null`.
pub fn transient_snapshot_from_result(
    result: TransientResult,
) -> Result<TransientSnapshot, String> {
    validate_transient_analog_inventory(
        &result.time,
        &result.step_sizes,
        result.num_nodes,
        &result.node_names,
        &result.voltages,
        &result.branch_names,
        &result.branch_currents,
        &result.device_op_traces,
        &result.store_traces,
    )?;
    let point_count = result.time.len();
    let fft_results = result
        .fft_results
        .iter()
        .enumerate()
        .map(|(index, result)| fft_snapshot(result, index + 1, "tran-001"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransientSnapshot {
        time: result.time,
        step_sizes: result.step_sizes,
        num_nodes: result.num_nodes,
        node_names: result.node_names,
        voltages: solution_waveforms(result.voltages, point_count),
        branch_names: result.branch_names,
        branch_currents: solution_waveforms(result.branch_currents, point_count),
        device_op_traces: device_op_snapshots(result.device_op_traces),
        store_traces: store_snapshots(result.store_traces),
        fft_results,
        compression: None,
    })
}

/// Convert a validated compressed core transient into the same browser DTO.
/// Compression provenance is retained rather than inferred from the grid.
pub fn transient_snapshot_from_compressed_result(
    result: TransientResultCompressed,
) -> Result<TransientSnapshot, String> {
    result.validate()?;
    validate_transient_analog_inventory(
        &result.time,
        &result.step_sizes,
        result.num_nodes,
        &result.node_names,
        &result.voltages,
        &result.branch_names,
        &result.branch_currents,
        &result.device_op_traces,
        &result.store_traces,
    )?;
    let point_count = result.time.len();
    let fft_results = result
        .fft_results
        .iter()
        .enumerate()
        .map(|(index, result)| fft_snapshot(result, index + 1, "tran-001"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransientSnapshot {
        time: result.time,
        step_sizes: result.step_sizes,
        num_nodes: result.num_nodes,
        node_names: result.node_names,
        voltages: solution_waveforms(result.voltages, point_count),
        branch_names: result.branch_names,
        branch_currents: solution_waveforms(result.branch_currents, point_count),
        device_op_traces: device_op_snapshots(result.device_op_traces),
        store_traces: store_snapshots(result.store_traces),
        fft_results,
        compression: Some(TransientCompressionSnapshot {
            schema_version: result.compression_report.schema_version,
            algorithm: result.compression_report.algorithm.as_str().to_string(),
            sample_domain: result.compression_report.sample_domain.as_str().to_string(),
            enabled: result.compression_report.applied_policy.enabled,
            absolute_tolerance: result.compression_report.applied_policy.absolute_tolerance,
            relative_tolerance: result.compression_report.applied_policy.relative_tolerance,
            maximum_retained_interval: result
                .compression_report
                .applied_policy
                .maximum_retained_interval,
            input_points: result.input_points,
            retained_points: point_count,
            compression_ratio: result.compression_ratio,
            worst_observed: result.compression_report.worst_observed.map(|observation| {
                TransientCompressionErrorSnapshot {
                    signal_kind: observation.signal.kind.as_str().to_string(),
                    canonical_name: observation.signal.canonical_name,
                    input_sample_index: observation.input_sample_index,
                    time: observation.time,
                    actual_value: observation.actual_value,
                    absolute_error: observation.absolute_error,
                    relative_error: observation.relative_error,
                    allowed_tolerance: observation.allowed_tolerance,
                    tolerance_utilization: observation.tolerance_utilization,
                }
            }),
        }),
    })
}
