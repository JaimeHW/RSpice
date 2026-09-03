//! Pickle state of the transient `.FFT` products.
//!
//! A `.FFT` product is evidence about a waveform the result no longer has to
//! keep, so the round trip re-proves its shape on the way back in: the bin
//! count has to match the point count, bin and harmonic indices have to be in
//! sequence, and every metric bound has to land inside the spectrum. A state
//! that cannot prove those is refused rather than restored as a spectrum with
//! quietly different axes.

use super::transient_fft_labels::*;
use super::*;

/// Pickle schema version shared by full and compressed transient FFT state.
///
/// Version zero never carried FFT evidence.  It cannot be migrated honestly:
/// an empty list could mean either that no `.FFT` was authored or that an old
/// Python binding silently discarded computed spectra.  The transient
/// unpicklers therefore reject an absent contract instead of fabricating an
/// empty result set.
pub(crate) const TRANSIENT_FFT_STATE_VERSION: usize = 1;

/// Stable tagged source descriptor: `(probe|expression, authored spelling)`.
pub(crate) type TransientFftSourceState = (String, String);

/// One calibrated FFT bin in primitive pickle form.
pub(crate) type TransientFftBinState = (usize, f64, f64, f64, f64, f64);

/// One magnitude-ranked harmonic in primitive pickle form.
pub(crate) type TransientFftHarmonicState = (usize, usize, f64, f64, f64, f64);

/// Xyce-compatible FFT metrics plus their ranked harmonic evidence.
pub(crate) type TransientFftMetricsState = (
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
pub(crate) type TransientFftResultState = (
    (TransientFftSourceState, String, String),
    (f64, f64, f64, usize, bool),
    (String, String, String, String, f64, f64),
    (f64, usize, usize, usize),
    Vec<TransientFftBinState>,
    Option<TransientFftMetricsState>,
);

/// Version and ordered spectra attached to either transient container.
pub(crate) type TransientFftPersistenceState = (usize, Vec<TransientFftResultState>);

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

pub(crate) fn transient_fft_result_state(
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

pub(crate) fn transient_fft_persistence_state(
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

pub(crate) fn rebuild_transient_fft_result(
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

pub(crate) fn rebuild_transient_fft_results(
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
