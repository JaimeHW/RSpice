//! Pickle state encoding shared by every result type.
//!
//! A result pickles as a plain tuple of primitives rather than through PyO3's
//! derive, because the core types it wraps are not themselves Python objects.
//! Two conventions here are load-bearing across versions:
//!
//! - Complex values travel as `(real, imaginary)` pairs. `Complex64` has no
//!   PyO3 scalar conversion, and a pair is the lossless encoding a NumPy-free
//!   consumer can read too.
//! - Enums travel as stable string labels, never ordinals, so state pickled by
//!   one build still reads correctly under a later one that reorders a variant.

use super::*;

/// Rebuild a core `SimulationResult` from its Python-visible state.
///
/// `SimulationResult::new` leaves its private observable index empty, and
/// core's observable lookup falls back to a linear scan in exactly that
/// case, so DC observables still resolve after a round-trip.
pub(super) fn rebuild_simulation_result(state: SimulationResultState) -> SimulationResult {
    let (node_voltages, node_names, branch_currents, branch_names, dc_observables) = state;
    let mut result =
        SimulationResult::new(node_voltages.len().saturating_sub(1), branch_currents.len());
    result.node_voltages = node_voltages;
    result.node_names = node_names;
    result.branch_currents = branch_currents;
    result.branch_names = branch_names;
    result.dc_observables = dc_observables;
    result
}

/// Complete Python-visible state of a core `SimulationResult`.
pub(super) type SimulationResultState = (
    Vec<f64>,
    Vec<String>,
    Vec<f64>,
    Vec<String>,
    Vec<(String, f64)>,
);

pub(super) fn simulation_result_state(result: &SimulationResult) -> SimulationResultState {
    (
        result.node_voltages.clone(),
        result.node_names.clone(),
        result.branch_currents.clone(),
        result.branch_names.clone(),
        result.dc_observables.clone(),
    )
}

/// Complex values in pickled state travel as `(real, imaginary)` pairs.
///
/// `num_complex::Complex64` has no PyO3 scalar conversion, and a pair is the
/// lossless encoding NumPy-free consumers can read too.
pub(super) fn complex_state(values: &[Complex64]) -> Vec<(f64, f64)> {
    values.iter().map(|value| (value.re, value.im)).collect()
}

pub(super) fn complex_from_state(values: Vec<(f64, f64)>) -> Vec<Complex64> {
    values
        .into_iter()
        .map(|(re, im)| Complex64::new(re, im))
        .collect()
}

/// Pickle schema version shared by full and compressed transient FFT state.
///
/// Version zero never carried FFT evidence.  It cannot be migrated honestly:
/// an empty list could mean either that no `.FFT` was authored or that an old
/// Python binding silently discarded computed spectra.  The transient
/// unpicklers therefore reject an absent contract instead of fabricating an
/// empty result set.
pub(super) const TRANSIENT_FFT_STATE_VERSION: usize = 1;

/// Stable tagged source descriptor: `(probe|expression, authored spelling)`.
pub(super) type TransientFftSourceState = (String, String);

/// One calibrated FFT bin in primitive pickle form.
pub(super) type TransientFftBinState = (usize, f64, f64, f64, f64, f64);

/// One magnitude-ranked harmonic in primitive pickle form.
pub(super) type TransientFftHarmonicState = (usize, usize, f64, f64, f64, f64);

/// Xyce-compatible FFT metrics plus their ranked harmonic evidence.
pub(super) type TransientFftMetricsState = (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    Option<usize>,
    Option<f64>,
    Vec<TransientFftHarmonicState>,
);

/// Complete typed state of one source-authored transient `.FFT` product.
///
/// The groups keep the state readable while avoiding enum ordinals and Python
/// extension objects in the persisted representation.
pub(super) type TransientFftResultState = (
    (TransientFftSourceState, String, String),
    (f64, f64, f64, usize, bool),
    (String, String, String, String, f64, f64),
    (f64, usize, usize, usize),
    Vec<TransientFftBinState>,
    Option<TransientFftMetricsState>,
);

/// Version and ordered spectra attached to either transient container.
pub(super) type TransientFftPersistenceState = (usize, Vec<TransientFftResultState>);

/// Pickle schema version of the full transient result's structural contract.
///
/// Version zero carried neither a version tag nor the XSPICE event histories a
/// mixed-signal transient computes. It cannot be migrated honestly: an empty
/// digital-trace list would equally mean "this deck has no event nodes" and
/// "an older binding dropped them", so a version-zero state is rejected rather
/// than restored as an analog-only result.
pub(super) const TRANSIENT_STRUCTURE_STATE_VERSION: usize = 1;

/// One digital event as `(time, state label, strength label)`.
pub(super) type DigitalEventState = (f64, String, String);

/// One XSPICE digital node's committed event history.
pub(super) type DigitalTraceState = (String, Vec<DigitalEventState>);

/// One XSPICE real-valued event node's committed history.
pub(super) type RealTraceState = (String, Vec<(f64, f64)>);

/// Version plus the event histories a full transient result carries.
pub(super) type TransientEventPersistenceState =
    (usize, Vec<DigitalTraceState>, Vec<RealTraceState>);

/// Stable wire spelling of one digital logic state.
///
/// The spellings are core's own `DigitalStateTag`, so the pickle and the
/// shared result document name the same states.
fn digital_state_label(state: rspice_core::xspice::DigitalState) -> &'static str {
    use rspice_core::execution::result_document::DigitalStateTag as Tag;
    match Tag::from(state) {
        Tag::Zero => "zero",
        Tag::One => "one",
        Tag::Unknown => "unknown",
        Tag::ZeroResistive => "zero_resistive",
        Tag::OneResistive => "one_resistive",
        Tag::UnknownResistive => "unknown_resistive",
        Tag::ZeroHighZ => "zero_high_z",
        Tag::OneHighZ => "one_high_z",
        Tag::UnknownHighZ => "unknown_high_z",
        Tag::HighZ => "high_z",
    }
}

fn digital_state_from_label(label: &str) -> Result<rspice_core::xspice::DigitalState, String> {
    use rspice_core::execution::result_document::DigitalStateTag as Tag;
    let tag = match label {
        "zero" => Tag::Zero,
        "one" => Tag::One,
        "unknown" => Tag::Unknown,
        "zero_resistive" => Tag::ZeroResistive,
        "one_resistive" => Tag::OneResistive,
        "unknown_resistive" => Tag::UnknownResistive,
        "zero_high_z" => Tag::ZeroHighZ,
        "one_high_z" => Tag::OneHighZ,
        "unknown_high_z" => Tag::UnknownHighZ,
        "high_z" => Tag::HighZ,
        other => {
            return Err(format!(
                "unknown digital logic state '{other}' in pickled transient state"
            ));
        }
    };
    Ok(tag.into())
}

/// Stable wire spelling of one digital drive strength.
fn digital_strength_label(strength: rspice_core::xspice::DigitalStrength) -> &'static str {
    use rspice_core::execution::result_document::DigitalStrengthTag as Tag;
    match Tag::from(strength) {
        Tag::Undetermined => "undetermined",
        Tag::HighZ => "high_z",
        Tag::Resistive => "resistive",
        Tag::Strong => "strong",
    }
}

fn digital_strength_from_label(
    label: &str,
) -> Result<rspice_core::xspice::DigitalStrength, String> {
    use rspice_core::execution::result_document::DigitalStrengthTag as Tag;
    let tag = match label {
        "undetermined" => Tag::Undetermined,
        "high_z" => Tag::HighZ,
        "resistive" => Tag::Resistive,
        "strong" => Tag::Strong,
        other => {
            return Err(format!(
                "unknown digital drive strength '{other}' in pickled transient state"
            ));
        }
    };
    Ok(tag.into())
}

/// Persist the XSPICE event histories a transient result carries.
pub(super) fn transient_event_persistence_state(
    digital_traces: &[rspice_core::engine::DigitalTrace],
    real_traces: &[rspice_core::engine::RealTrace],
) -> TransientEventPersistenceState {
    (
        TRANSIENT_STRUCTURE_STATE_VERSION,
        digital_traces
            .iter()
            .map(|trace| {
                (
                    trace.node_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| {
                            (
                                point.time,
                                digital_state_label(point.value.state).to_string(),
                                digital_strength_label(point.value.strength).to_string(),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
        real_traces
            .iter()
            .map(|trace| {
                (
                    trace.node_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| (point.time, point.value))
                        .collect(),
                )
            })
            .collect(),
    )
}

/// Rebuild the event histories, rejecting a state this build cannot read.
#[allow(clippy::type_complexity)]
pub(super) fn rebuild_transient_event_traces(
    state: Option<TransientEventPersistenceState>,
) -> Result<
    (
        Vec<rspice_core::engine::DigitalTrace>,
        Vec<rspice_core::engine::RealTrace>,
    ),
    String,
> {
    let Some((version, digital, real)) = state else {
        return Err(
            "legacy transient pickle predates the versioned result contract and carries no XSPICE \
             event histories; rerun the analysis"
                .to_string(),
        );
    };
    if version != TRANSIENT_STRUCTURE_STATE_VERSION {
        return Err(format!(
            "unsupported transient pickle state version {version}; this build reads version {TRANSIENT_STRUCTURE_STATE_VERSION}"
        ));
    }
    let digital_traces = digital
        .into_iter()
        .map(|(node_name, points)| {
            Ok(rspice_core::engine::DigitalTrace {
                node_name,
                points: points
                    .into_iter()
                    .map(|(time, state, strength)| {
                        Ok(rspice_core::engine::DigitalTracePoint {
                            time,
                            value: rspice_core::xspice::DigitalValue {
                                state: digital_state_from_label(&state)?,
                                strength: digital_strength_from_label(&strength)?,
                            },
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let real_traces = real
        .into_iter()
        .map(|(node_name, points)| rspice_core::engine::RealTrace {
            node_name,
            points: points
                .into_iter()
                .map(|(time, value)| rspice_core::engine::RealTracePoint { time, value })
                .collect(),
        })
        .collect();
    Ok((digital_traces, real_traces))
}

fn fft_output_state(output: &rspice_core::netlist::FftOutput) -> TransientFftSourceState {
    match output {
        rspice_core::netlist::FftOutput::Probe(value) => ("probe".to_string(), value.clone()),
        rspice_core::netlist::FftOutput::Expression(value) => {
            ("expression".to_string(), value.clone())
        }
    }
}

fn fft_output_from_state(
    state: TransientFftSourceState,
) -> PyResult<rspice_core::netlist::FftOutput> {
    let (kind, value) = state;
    if value.trim().is_empty() {
        return Err(crate::errors::value_error(
            "pickled transient FFT source descriptor cannot be empty",
        ));
    }
    match kind.as_str() {
        "probe" => Ok(rspice_core::netlist::FftOutput::Probe(value)),
        "expression" => Ok(rspice_core::netlist::FftOutput::Expression(value)),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT source kind '{other}' in pickled state"
        ))),
    }
}

fn fft_format_label(format: rspice_core::netlist::FftFormat) -> &'static str {
    match format {
        rspice_core::netlist::FftFormat::Normalized => "normalized",
        rspice_core::netlist::FftFormat::Unnormalized => "unnormalized",
    }
}

fn fft_format_from_label(label: &str) -> PyResult<rspice_core::netlist::FftFormat> {
    match label {
        "normalized" => Ok(rspice_core::netlist::FftFormat::Normalized),
        "unnormalized" => Ok(rspice_core::netlist::FftFormat::Unnormalized),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT format '{other}' in pickled state"
        ))),
    }
}

fn fft_mode_label(mode: rspice_core::netlist::XyceFftMode) -> &'static str {
    match mode {
        rspice_core::netlist::XyceFftMode::HspiceCompatible => "hspice_compatible",
        rspice_core::netlist::XyceFftMode::SpectreCompatible => "spectre_compatible",
    }
}

fn fft_mode_from_label(label: &str) -> PyResult<rspice_core::netlist::XyceFftMode> {
    match label {
        "hspice_compatible" => Ok(rspice_core::netlist::XyceFftMode::HspiceCompatible),
        "spectre_compatible" => Ok(rspice_core::netlist::XyceFftMode::SpectreCompatible),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT mode '{other}' in pickled state"
        ))),
    }
}

fn fft_window_label(window: rspice_core::netlist::FftWindow) -> &'static str {
    use rspice_core::netlist::FftWindow;
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

fn fft_window_from_label(label: &str) -> PyResult<rspice_core::netlist::FftWindow> {
    use rspice_core::netlist::FftWindow;
    match label {
        "rectangular" => Ok(FftWindow::Rectangular),
        "bartlett" => Ok(FftWindow::Bartlett),
        "bartlett_hann" => Ok(FftWindow::BartlettHann),
        "hamming" => Ok(FftWindow::Hamming),
        "hann" => Ok(FftWindow::Hann),
        "blackman_67db" => Ok(FftWindow::Blackman67Db),
        "blackman" => Ok(FftWindow::Blackman),
        "blackman_harris" => Ok(FftWindow::BlackmanHarris),
        "nuttall" => Ok(FftWindow::Nuttall),
        "half_cycle_sine" => Ok(FftWindow::HalfCycleSine),
        "half_cycle_sine_3" => Ok(FftWindow::HalfCycleSine3),
        "half_cycle_sine_6" => Ok(FftWindow::HalfCycleSine6),
        "cosine_2" => Ok(FftWindow::Cosine2),
        "cosine_4" => Ok(FftWindow::Cosine4),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT window '{other}' in pickled state"
        ))),
    }
}

fn fft_physical_type_from_label(label: &str) -> PyResult<&'static str> {
    match label {
        "voltage" => Ok("voltage"),
        "current" => Ok("current"),
        "parameter" => Ok("parameter"),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT physical type '{other}' in pickled state"
        ))),
    }
}

fn fft_bin_state(bin: &rspice_core::engine::TransientFftBin) -> TransientFftBinState {
    (
        bin.index,
        bin.frequency,
        bin.real,
        bin.imaginary,
        bin.magnitude,
        bin.phase_degrees,
    )
}

fn fft_harmonic_state(
    harmonic: &rspice_core::engine::TransientFftHarmonic,
) -> TransientFftHarmonicState {
    (
        harmonic.rank,
        harmonic.bin,
        harmonic.frequency,
        harmonic.magnitude,
        harmonic.magnitude_db,
        harmonic.phase_degrees,
    )
}

fn fft_metrics_state(
    metrics: &rspice_core::engine::TransientFftMetrics,
) -> TransientFftMetricsState {
    (
        metrics.fundamental_magnitude,
        metrics.thd_ratio,
        metrics.thd_db,
        metrics.sndr_db,
        metrics.enob_bits,
        metrics.snr_db,
        metrics.sfdr_db,
        metrics.sfdr_spur_bin,
        metrics.sfdr_spur_frequency,
        metrics
            .largest_harmonics
            .iter()
            .map(fft_harmonic_state)
            .collect(),
    )
}

pub(super) fn transient_fft_result_state(
    result: &rspice_core::engine::TransientFftResult,
) -> PyResult<TransientFftResultState> {
    // `physical_type` is a core string for forward-extensibility.  Fail closed
    // if core grows a quantity Python has not classified yet.
    fft_physical_type_from_label(result.physical_type)?;
    Ok((
        (
            fft_output_state(&result.output),
            result.output_name.clone(),
            result.physical_type.to_string(),
        ),
        (
            result.start_time,
            result.stop_time,
            result.sample_interval,
            result.point_count,
            result.accurate_sampling,
        ),
        (
            fft_format_label(result.format).to_string(),
            fft_mode_label(result.mode).to_string(),
            fft_window_label(result.window).to_string(),
            result.window_name.clone(),
            result.alpha,
            result.coherent_gain,
        ),
        (
            result.frequency_resolution,
            result.fundamental_bin,
            result.minimum_metric_bin,
            result.maximum_metric_bin,
        ),
        result.bins.iter().map(fft_bin_state).collect(),
        result.metrics.as_ref().map(fft_metrics_state),
    ))
}

pub(super) fn transient_fft_persistence_state(
    results: &[rspice_core::engine::TransientFftResult],
) -> PyResult<TransientFftPersistenceState> {
    Ok((
        TRANSIENT_FFT_STATE_VERSION,
        results
            .iter()
            .map(transient_fft_result_state)
            .collect::<PyResult<_>>()?,
    ))
}

fn rebuild_fft_metrics(
    state: TransientFftMetricsState,
    bin_count: usize,
) -> PyResult<rspice_core::engine::TransientFftMetrics> {
    let (
        fundamental_magnitude,
        thd_ratio,
        thd_db,
        sndr_db,
        enob_bits,
        snr_db,
        sfdr_db,
        sfdr_spur_bin,
        sfdr_spur_frequency,
        largest_harmonics,
    ) = state;
    if sfdr_spur_bin.is_some() != sfdr_spur_frequency.is_some() {
        return Err(crate::errors::value_error(
            "pickled transient FFT SFDR spur bin and frequency must be present together",
        ));
    }
    if sfdr_spur_bin.is_some_and(|bin| bin >= bin_count) {
        return Err(crate::errors::value_error(
            "pickled transient FFT SFDR spur bin is outside the spectrum",
        ));
    }
    let mut rebuilt_harmonics = Vec::with_capacity(largest_harmonics.len());
    for (index, state) in largest_harmonics.into_iter().enumerate() {
        let (rank, bin, frequency, magnitude, magnitude_db, phase_degrees) = state;
        if rank != index + 1 {
            return Err(crate::errors::value_error(format!(
                "pickled transient FFT harmonic rank {rank} is out of sequence at position {}",
                index + 1
            )));
        }
        if bin == 0 || bin >= bin_count {
            return Err(crate::errors::value_error(format!(
                "pickled transient FFT ranked harmonic bin {bin} is outside the non-DC spectrum"
            )));
        }
        rebuilt_harmonics.push(rspice_core::engine::TransientFftHarmonic {
            rank,
            bin,
            frequency,
            magnitude,
            magnitude_db,
            phase_degrees,
        });
    }
    Ok(rspice_core::engine::TransientFftMetrics {
        fundamental_magnitude,
        thd_ratio,
        thd_db,
        sndr_db,
        enob_bits,
        snr_db,
        sfdr_db,
        sfdr_spur_bin,
        sfdr_spur_frequency,
        largest_harmonics: rebuilt_harmonics,
    })
}

pub(super) fn rebuild_transient_fft_result(
    state: TransientFftResultState,
) -> PyResult<rspice_core::engine::TransientFftResult> {
    let (source, sampling, configuration, axes, bins, metrics) = state;
    let (output, output_name, physical_type) = source;
    let (start_time, stop_time, sample_interval, point_count, accurate_sampling) = sampling;
    let (format, mode, window, window_name, alpha, coherent_gain) = configuration;
    let (frequency_resolution, fundamental_bin, minimum_metric_bin, maximum_metric_bin) = axes;
    if output_name.trim().is_empty() {
        return Err(crate::errors::value_error(
            "pickled transient FFT output name cannot be empty",
        ));
    }
    if point_count < 4 || !point_count.is_power_of_two() {
        return Err(crate::errors::value_error(format!(
            "pickled transient FFT point count must be a power of two of at least 4, got {point_count}"
        )));
    }
    let expected_bins = point_count / 2 + 1;
    if bins.len() != expected_bins {
        return Err(crate::errors::value_error(format!(
            "pickled transient FFT has {} bins, expected {expected_bins} for {point_count} points",
            bins.len()
        )));
    }
    if !start_time.is_finite()
        || !stop_time.is_finite()
        || stop_time <= start_time
        || !sample_interval.is_finite()
        || sample_interval <= 0.0
        || !frequency_resolution.is_finite()
        || frequency_resolution <= 0.0
    {
        return Err(crate::errors::value_error(
            "pickled transient FFT has an invalid sampling interval or frequency resolution",
        ));
    }
    if [fundamental_bin, minimum_metric_bin, maximum_metric_bin]
        .into_iter()
        .any(|bin| bin >= expected_bins)
        || minimum_metric_bin > maximum_metric_bin
    {
        return Err(crate::errors::value_error(
            "pickled transient FFT metric bounds are outside the spectrum",
        ));
    }

    let mut rebuilt_bins = Vec::with_capacity(bins.len());
    for (expected_index, state) in bins.into_iter().enumerate() {
        let (index, frequency, real, imaginary, magnitude, phase_degrees) = state;
        if index != expected_index {
            return Err(crate::errors::value_error(format!(
                "pickled transient FFT bin index {index} is out of sequence at position {expected_index}"
            )));
        }
        if !frequency.is_finite() || frequency < 0.0 {
            return Err(crate::errors::value_error(format!(
                "pickled transient FFT bin {index} has an invalid frequency"
            )));
        }
        rebuilt_bins.push(rspice_core::engine::TransientFftBin {
            index,
            frequency,
            real,
            imaginary,
            magnitude,
            phase_degrees,
        });
    }
    let metrics = metrics
        .map(|state| rebuild_fft_metrics(state, expected_bins))
        .transpose()?;
    Ok(rspice_core::engine::TransientFftResult {
        output: fft_output_from_state(output)?,
        output_name,
        physical_type: fft_physical_type_from_label(&physical_type)?,
        start_time,
        stop_time,
        sample_interval,
        point_count,
        accurate_sampling,
        format: fft_format_from_label(&format)?,
        mode: fft_mode_from_label(&mode)?,
        window: fft_window_from_label(&window)?,
        window_name,
        alpha,
        coherent_gain,
        frequency_resolution,
        fundamental_bin,
        minimum_metric_bin,
        maximum_metric_bin,
        bins: rebuilt_bins,
        metrics,
    })
}

pub(super) fn rebuild_transient_fft_results(
    state: Option<TransientFftPersistenceState>,
) -> PyResult<Vec<rspice_core::engine::TransientFftResult>> {
    let Some((version, results)) = state else {
        return Err(crate::errors::value_error(
            "legacy transient pickle predates lossless FFT persistence; rerun the analysis because the old state cannot prove whether FFT results were discarded",
        ));
    };
    if version != TRANSIENT_FFT_STATE_VERSION {
        return Err(crate::errors::value_error(format!(
            "unsupported transient FFT pickle state version {version}"
        )));
    }
    results
        .into_iter()
        .map(rebuild_transient_fft_result)
        .collect()
}

/// Stable primitive pickle state for a qualified Floquet eigenspectrum.
pub(super) type FloquetSpectrumCertificateState = (usize, f64, f64);

/// Stable tagged pickle state for the provenance of a Floquet spectrum.
pub(super) type FloquetSpectrumEvidenceState = (String, Option<FloquetSpectrumCertificateState>);

/// Versioned Floquet contract appended to the legacy six-argument PSS state.
pub(super) type PssFloquetContractState =
    (usize, FloquetSpectrumEvidenceState, String, Option<usize>);

pub(super) const PSS_FLOQUET_CONTRACT_STATE_VERSION: usize = 1;

pub(super) fn floquet_spectrum_certificate_state(
    certificate: &rspice_core::analysis::FloquetSpectrumCertificate,
) -> FloquetSpectrumCertificateState {
    (
        certificate.problem_order,
        certificate.max_backward_error,
        certificate.qualification_tolerance,
    )
}

pub(super) fn floquet_spectrum_certificate_from_state(
    state: FloquetSpectrumCertificateState,
) -> PyResult<rspice_core::analysis::FloquetSpectrumCertificate> {
    let (problem_order, max_backward_error, qualification_tolerance) = state;
    rspice_core::analysis::FloquetSpectrumCertificate::new(
        problem_order,
        max_backward_error,
        qualification_tolerance,
    )
    .ok_or_else(|| {
        crate::errors::value_error(
            "invalid Floquet spectrum certificate in pickled PSS result".to_string(),
        )
    })
}

pub(super) fn floquet_spectrum_evidence_state(
    evidence: &rspice_core::analysis::FloquetSpectrumEvidence,
) -> PyResult<FloquetSpectrumEvidenceState> {
    use rspice_core::analysis::FloquetSpectrumEvidence;

    match evidence {
        FloquetSpectrumEvidence::NotComputed => Ok(("not_computed".to_string(), None)),
        FloquetSpectrumEvidence::NoDynamicModes => Ok(("no_dynamic_modes".to_string(), None)),
        FloquetSpectrumEvidence::Qualified { certificate } => Ok((
            "qualified".to_string(),
            Some(floquet_spectrum_certificate_state(certificate)),
        )),
        FloquetSpectrumEvidence::LegacyUnknown => Ok(("legacy_unknown".to_string(), None)),
        _ => Err(crate::errors::value_error(
            "unsupported Floquet spectrum evidence variant".to_string(),
        )),
    }
}

pub(super) fn floquet_spectrum_evidence_from_state(
    state: FloquetSpectrumEvidenceState,
) -> PyResult<rspice_core::analysis::FloquetSpectrumEvidence> {
    use rspice_core::analysis::FloquetSpectrumEvidence;

    let (kind, certificate) = state;
    match (kind.as_str(), certificate) {
        ("not_computed", None) => Ok(FloquetSpectrumEvidence::NotComputed),
        ("no_dynamic_modes", None) => Ok(FloquetSpectrumEvidence::NoDynamicModes),
        ("legacy_unknown", None) => Ok(FloquetSpectrumEvidence::LegacyUnknown),
        ("qualified", Some(certificate)) => {
            let certificate = floquet_spectrum_certificate_from_state(certificate)?;
            FloquetSpectrumEvidence::qualified(certificate).ok_or_else(|| {
                crate::errors::value_error(
                    "inconsistent qualified Floquet evidence in pickled PSS result".to_string(),
                )
            })
        }
        ("not_computed" | "no_dynamic_modes" | "legacy_unknown", Some(_)) => {
            Err(crate::errors::value_error(format!(
                "Floquet evidence tag '{kind}' cannot carry a certificate"
            )))
        }
        ("qualified", None) => Err(crate::errors::value_error(
            "Floquet evidence tag 'qualified' requires a certificate".to_string(),
        )),
        (other, _) => Err(crate::errors::value_error(format!(
            "unknown Floquet evidence tag '{other}' in pickled PSS state"
        ))),
    }
}

pub(super) fn floquet_orbit_kind_state(
    orbit_kind: rspice_core::analysis::FloquetOrbitKind,
) -> PyResult<String> {
    use rspice_core::analysis::FloquetOrbitKind;

    match orbit_kind {
        FloquetOrbitKind::Driven => Ok("driven".to_string()),
        FloquetOrbitKind::Autonomous => Ok("autonomous".to_string()),
        _ => Err(crate::errors::value_error(
            "unsupported Floquet orbit kind".to_string(),
        )),
    }
}

pub(super) fn floquet_orbit_kind_from_state(
    orbit_kind: &str,
) -> PyResult<rspice_core::analysis::FloquetOrbitKind> {
    match orbit_kind {
        "driven" => Ok(rspice_core::analysis::FloquetOrbitKind::Driven),
        "autonomous" => Ok(rspice_core::analysis::FloquetOrbitKind::Autonomous),
        other => Err(crate::errors::value_error(format!(
            "unknown Floquet orbit kind '{other}' in pickled PSS state"
        ))),
    }
}

/// Stable primitive pickle state for a pole-zero spectrum certificate.
pub(super) type SpectrumCertificateState = (usize, usize, f64, f64);

/// Stable tagged pickle state for pole/zero root-set evidence.
pub(super) type RootSetEvidenceState = (String, Option<SpectrumCertificateState>);

pub(super) fn spectrum_certificate_state(
    certificate: &rspice_core::analysis::SpectrumCertificate,
) -> SpectrumCertificateState {
    (
        certificate.problem_order,
        certificate.infinite_count,
        certificate.max_backward_error,
        certificate.qualification_tolerance,
    )
}

pub(super) fn spectrum_certificate_from_state(
    state: SpectrumCertificateState,
) -> PyResult<rspice_core::analysis::SpectrumCertificate> {
    let (problem_order, infinite_count, max_backward_error, qualification_tolerance) = state;
    rspice_core::analysis::SpectrumCertificate::new(
        problem_order,
        infinite_count,
        max_backward_error,
        qualification_tolerance,
    )
    .ok_or_else(|| {
        crate::errors::value_error(
            "invalid spectrum certificate in pickled pole-zero result".to_string(),
        )
    })
}

pub(super) fn root_set_evidence_state(
    evidence: &rspice_core::analysis::RootSetEvidence,
) -> PyResult<RootSetEvidenceState> {
    use rspice_core::analysis::RootSetEvidence;

    match evidence {
        RootSetEvidence::NotRequested => Ok(("not_requested".to_string(), None)),
        RootSetEvidence::QualifiedEmpty { certificate } => Ok((
            "qualified_empty".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::Qualified { certificate } => Ok((
            "qualified".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::Approximate { certificate } => Ok((
            "approximate".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::LegacyUnknown => Ok(("legacy_unknown".to_string(), None)),
        _ => Err(crate::errors::value_error(
            "unsupported root-set evidence variant".to_string(),
        )),
    }
}

pub(super) fn root_set_evidence_from_state(
    state: RootSetEvidenceState,
) -> PyResult<rspice_core::analysis::RootSetEvidence> {
    use rspice_core::analysis::RootSetEvidence;

    let (kind, certificate) = state;
    match (kind.as_str(), certificate) {
        ("not_requested", None) => Ok(RootSetEvidence::NotRequested),
        ("legacy_unknown", None) => Ok(RootSetEvidence::LegacyUnknown),
        ("qualified_empty" | "qualified" | "approximate", Some(certificate)) => {
            let certificate = spectrum_certificate_from_state(certificate)?;
            let derived =
                RootSetEvidence::from_certificate(certificate.finite_count(), certificate)
                    .ok_or_else(|| {
                        crate::errors::value_error(
                            "inconsistent root-set certificate in pickled pole-zero result"
                                .to_string(),
                        )
                    })?;
            let tag_matches = matches!(
                (kind.as_str(), &derived),
                ("qualified_empty", RootSetEvidence::QualifiedEmpty { .. })
                    | ("qualified", RootSetEvidence::Qualified { .. })
                    | ("approximate", RootSetEvidence::Approximate { .. })
            );
            if tag_matches {
                Ok(derived)
            } else {
                Err(crate::errors::value_error(format!(
                    "root-set evidence tag '{kind}' disagrees with its certificate"
                )))
            }
        }
        ("not_requested" | "legacy_unknown", Some(_)) => Err(crate::errors::value_error(format!(
            "root-set evidence tag '{kind}' cannot carry a certificate"
        ))),
        ("qualified_empty" | "qualified" | "approximate", None) => Err(crate::errors::value_error(
            format!("root-set evidence tag '{kind}' requires a certificate"),
        )),
        (other, _) => Err(crate::errors::value_error(format!(
            "unknown root-set evidence tag '{other}' in pickled state"
        ))),
    }
}

/// The `(absolute, normalized)` complex derivative pair of an AC sensitivity
/// trace, in pickled `(real, imaginary)` form.
pub(super) type ComplexSeriesPair = (Vec<(f64, f64)>, Vec<(f64, f64)>);

/// Complete Python-visible state of one core `AcResult` row.
pub(super) type AcRowState = (
    f64,
    Vec<String>,
    Vec<String>,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
);

pub(super) fn ac_row_state(row: &AcResult) -> AcRowState {
    (
        row.frequency,
        row.node_names.clone(),
        row.branch_names.clone(),
        complex_state(&row.voltages),
        complex_state(&row.currents),
    )
}

pub(super) fn rebuild_ac_row(state: AcRowState) -> AcResult {
    let (frequency, node_names, branch_names, voltages, currents) = state;
    AcResult {
        frequency,
        node_names,
        branch_names,
        voltages: complex_from_state(voltages),
        currents: complex_from_state(currents),
    }
}

/// One named complex spectrum: `(name, coefficients, harmonic frequencies)`.
///
/// `SpectralVoltage` and `SpectralBranchCurrent` share this shape.
pub(super) type SpectralSeriesState = (String, Vec<(f64, f64)>, Vec<f64>);

/// One reactive element's retained HB spectra:
/// `(device, kind, voltage phasors, current phasors, DC current is exact)`.
pub(super) type HbReactiveState = (String, String, Vec<(f64, f64)>, Vec<(f64, f64)>, bool);

/// Complex values indexed `[frequency][sideband][node]` or
/// `[frequency][output sideband][input sideband]`.
pub(super) type ComplexGridState = Vec<Vec<Vec<(f64, f64)>>>;

/// One distortion product and its per-F1-point rows, keyed by stable label.
pub(super) type DistortionProductState = (String, Vec<AcRowState>);

pub(super) fn spectral_series_state(
    name: &str,
    coefficients: &[Complex64],
    frequencies: &[f64],
) -> SpectralSeriesState {
    (
        name.to_string(),
        complex_state(coefficients),
        frequencies.to_vec(),
    )
}

/// `HbReactiveKind` travels as its lowercase element name.
pub(super) fn hb_reactive_kind_label(kind: HbReactiveKind) -> &'static str {
    match kind {
        HbReactiveKind::Capacitor => "capacitor",
        HbReactiveKind::Inductor => "inductor",
    }
}

pub(super) fn rebuild_hb_reactive(
    state: HbReactiveState,
) -> PyResult<rspice_core::analysis::HbReactiveSpectrum> {
    let (device_name, kind, voltage, current, dc_current_is_exact) = state;
    Ok(rspice_core::analysis::HbReactiveSpectrum {
        device_name,
        kind: hb_reactive_kind_from_label(&kind)?,
        voltage_coefficients: complex_from_state(voltage),
        current_coefficients: complex_from_state(current),
        dc_current_is_exact,
    })
}

fn hb_reactive_kind_from_label(label: &str) -> PyResult<HbReactiveKind> {
    match label {
        "capacitor" => Ok(HbReactiveKind::Capacitor),
        "inductor" => Ok(HbReactiveKind::Inductor),
        other => Err(crate::errors::value_error(format!(
            "unknown reactive element kind '{other}' in pickled state"
        ))),
    }
}

/// Continuation limitations travel as stable snake-case tags rather than
/// ordinals, so a state pickled by one build still reads correctly under a
/// later one that reorders the enum.
pub(super) fn hb_limitation_label(limitation: &HbContinuationLimitation) -> &'static str {
    match limitation {
        HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent => {
            "nonlinear_voltage_sources_use_norton_equivalent"
        }
        HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate => {
            "inductor_dc_current_uses_short_surrogate"
        }
        HbContinuationLimitation::VerilogAInternalStateNotRetained => {
            "veriloga_internal_state_not_retained"
        }
    }
}

pub(super) fn hb_limitation_from_label(label: &str) -> PyResult<HbContinuationLimitation> {
    match label {
        "nonlinear_voltage_sources_use_norton_equivalent" => {
            Ok(HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent)
        }
        "inductor_dc_current_uses_short_surrogate" => {
            Ok(HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate)
        }
        "veriloga_internal_state_not_retained" => {
            Ok(HbContinuationLimitation::VerilogAInternalStateNotRetained)
        }
        other => Err(crate::errors::value_error(format!(
            "unknown continuation limitation '{other}' in pickled state"
        ))),
    }
}

/// Distortion products travel as the same stable labels the accessors accept.
pub(super) fn distortion_product_from_label(label: &str) -> PyResult<DistortionProduct> {
    match label {
        "2f1" => Ok(DistortionProduct::SecondHarmonic),
        "3f1" => Ok(DistortionProduct::ThirdHarmonic),
        "f1+f2" => Ok(DistortionProduct::Sum),
        "f1-f2" => Ok(DistortionProduct::Difference),
        "2f1-f2" => Ok(DistortionProduct::ThirdOrderDifference),
        other => Err(crate::errors::value_error(format!(
            "unknown distortion product '{other}' in pickled state"
        ))),
    }
}

/// The `_unpickle` callable bound to a pyclass, referenced by `__reduce__`.
///
/// Pickling a bound staticmethod resolves by qualified name, which keeps each
/// helper namespaced on its own class instead of adding private names to the
/// module surface.
pub(super) fn unpickler<'py, T: pyo3::PyTypeInfo>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    py.get_type::<T>().getattr("_unpickle")
}

#[cfg(test)]
mod event_pickle_tests {
    use super::*;
    use rspice_core::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    const EVERY_STATE: [DigitalState; 10] = [
        DigitalState::Zero,
        DigitalState::One,
        DigitalState::Unknown,
        DigitalState::ZeroR,
        DigitalState::OneR,
        DigitalState::UnknownR,
        DigitalState::ZeroZ,
        DigitalState::OneZ,
        DigitalState::UnknownZ,
        DigitalState::HighZ,
    ];

    const EVERY_STRENGTH: [DigitalStrength; 4] = [
        DigitalStrength::Undetermined,
        DigitalStrength::HighZ,
        DigitalStrength::Resistive,
        DigitalStrength::Strong,
    ];

    #[test]
    fn every_digital_state_and_strength_round_trips_through_its_label() {
        let mut labels = std::collections::BTreeSet::new();
        for state in EVERY_STATE {
            let label = digital_state_label(state);
            assert!(labels.insert(label), "duplicate state label '{label}'");
            assert_eq!(digital_state_from_label(label).unwrap(), state);
        }
        let mut strength_labels = std::collections::BTreeSet::new();
        for strength in EVERY_STRENGTH {
            let label = digital_strength_label(strength);
            assert!(
                strength_labels.insert(label),
                "duplicate strength label '{label}'"
            );
            assert_eq!(digital_strength_from_label(label).unwrap(), strength);
        }
    }

    #[test]
    fn event_histories_survive_the_persistence_round_trip() {
        let digital = vec![DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![
                DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue {
                        state: DigitalState::ZeroZ,
                        strength: DigitalStrength::HighZ,
                    },
                },
                DigitalTracePoint {
                    time: 1.0e-9,
                    value: DigitalValue {
                        state: DigitalState::One,
                        strength: DigitalStrength::Strong,
                    },
                },
            ],
        }];
        let real = vec![RealTrace {
            node_name: "ctrl".to_string(),
            points: vec![RealTracePoint {
                time: 2.0e-9,
                value: -0.5,
            }],
        }];

        let state = transient_event_persistence_state(&digital, &real);
        assert_eq!(state.0, TRANSIENT_STRUCTURE_STATE_VERSION);
        let (restored_digital, restored_real) =
            rebuild_transient_event_traces(Some(state)).unwrap();
        assert_eq!(restored_digital, digital);
        assert_eq!(restored_real, real);
    }

    #[test]
    fn a_legacy_state_is_refused_with_an_actionable_message() {
        let error = rebuild_transient_event_traces(None)
            .expect_err("a state without a version tag cannot be restored");
        assert!(error.contains("legacy transient pickle"), "{error}");
        assert!(error.contains("rerun the analysis"), "{error}");
    }

    #[test]
    fn a_future_state_version_is_refused_by_number() {
        let future = TRANSIENT_STRUCTURE_STATE_VERSION + 1;
        let error = rebuild_transient_event_traces(Some((future, Vec::new(), Vec::new())))
            .expect_err("a newer contract cannot be read by this build");
        assert!(
            error.contains(&format!("state version {future}")),
            "{error}"
        );
    }

    #[test]
    fn an_unknown_logic_label_is_refused_rather_than_defaulted() {
        let error = rebuild_transient_event_traces(Some((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![(
                "clk".to_string(),
                vec![(0.0, "floating".to_string(), "strong".to_string())],
            )],
            Vec::new(),
        )))
        .expect_err("an unknown logic state must not become Unknown");
        assert!(error.contains("'floating'"), "{error}");
    }
}

#[cfg(test)]
mod fft_pickle_tests {
    use super::*;
    use rspice_core::engine::{
        TransientFftBin, TransientFftHarmonic, TransientFftMetrics, TransientFftResult,
    };
    use rspice_core::netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode};

    fn spectrum() -> TransientFftResult {
        TransientFftResult {
            output: FftOutput::Expression("2*V(out)".to_string()),
            output_name: "{2*V(out)}".to_string(),
            physical_type: "parameter",
            start_time: 0.0,
            stop_time: 0.01,
            sample_interval: 0.0025,
            point_count: 4,
            accurate_sampling: false,
            format: FftFormat::Unnormalized,
            mode: XyceFftMode::SpectreCompatible,
            window: FftWindow::BlackmanHarris,
            window_name: "BLACKMANHARRIS".to_string(),
            alpha: 3.5,
            coherent_gain: 0.42,
            frequency_resolution: 100.0,
            fundamental_bin: 1,
            minimum_metric_bin: 1,
            maximum_metric_bin: 2,
            bins: vec![
                TransientFftBin {
                    index: 0,
                    frequency: 0.0,
                    real: 0.25,
                    imaginary: 0.0,
                    magnitude: 0.25,
                    phase_degrees: 0.0,
                },
                TransientFftBin {
                    index: 1,
                    frequency: 100.0,
                    real: 0.0,
                    imaginary: -2.0,
                    magnitude: 2.0,
                    phase_degrees: -90.0,
                },
                TransientFftBin {
                    index: 2,
                    frequency: 200.0,
                    real: 0.5,
                    imaginary: 0.25,
                    magnitude: 0.559_016_994_374_947_5,
                    phase_degrees: 26.565_051_177_077_99,
                },
            ],
            metrics: Some(TransientFftMetrics {
                fundamental_magnitude: 2.0,
                thd_ratio: 0.25,
                thd_db: -12.041_199_826_559_248,
                sndr_db: 20.0,
                enob_bits: 3.029_900_332_225_913_4,
                snr_db: f64::INFINITY,
                sfdr_db: 12.041_199_826_559_248,
                sfdr_spur_bin: Some(2),
                sfdr_spur_frequency: Some(200.0),
                largest_harmonics: vec![
                    TransientFftHarmonic {
                        rank: 1,
                        bin: 1,
                        frequency: 100.0,
                        magnitude: 2.0,
                        magnitude_db: 6.020_599_913_279_624,
                        phase_degrees: -90.0,
                    },
                    TransientFftHarmonic {
                        rank: 2,
                        bin: 2,
                        frequency: 200.0,
                        magnitude: 0.559_016_994_374_947_5,
                        magnitude_db: -5.051_499_783_199_059,
                        phase_degrees: 26.565_051_177_077_99,
                    },
                ],
            }),
        }
    }

    #[test]
    fn complete_fft_state_round_trips_exactly() {
        let original = spectrum();
        let state = transient_fft_persistence_state(std::slice::from_ref(&original))
            .expect("supported FFT result serializes");
        assert_eq!(state.0, TRANSIENT_FFT_STATE_VERSION);
        let rebuilt = rebuild_transient_fft_results(Some(state))
            .expect("same-version FFT state reconstructs");
        assert_eq!(rebuilt, [original]);
    }

    #[test]
    fn fft_state_uses_stable_tags_and_preserves_source_order() {
        let expression = spectrum();
        let mut probe = spectrum();
        probe.output = FftOutput::Probe("V(OUT)".to_string());
        probe.output_name = "V(OUT)".to_string();
        probe.physical_type = "voltage";
        probe.format = FftFormat::Normalized;
        probe.mode = XyceFftMode::HspiceCompatible;
        probe.window = FftWindow::Hann;

        let state = transient_fft_persistence_state(&[expression, probe])
            .expect("supported FFT results serialize");
        assert_eq!(state.0, TRANSIENT_FFT_STATE_VERSION);
        assert_eq!(state.1[0].0.0.0, "expression");
        assert_eq!(state.1[0].2.0, "unnormalized");
        assert_eq!(state.1[0].2.1, "spectre_compatible");
        assert_eq!(state.1[0].2.2, "blackman_harris");
        assert_eq!(state.1[1].0.0.0, "probe");
        assert_eq!(state.1[1].2.0, "normalized");
        assert_eq!(state.1[1].2.1, "hspice_compatible");
        assert_eq!(state.1[1].2.2, "hann");
    }
}
