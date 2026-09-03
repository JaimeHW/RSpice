//! Typed `.FFT` post-processing over accepted transient histories.

use super::super::{
    Engine, SimulationError, TransientFftBin, TransientFftHarmonic, TransientFftMetrics,
    TransientFftResult, TransientResult,
};
use crate::abort_signal::AbortSignal;
use crate::netlist::{FftAnalysis, FftFormat, FftWindow, Netlist, XyceFftMode};
use crate::numerics::rustfft_qualification::qualify_rustfft_forward_length;
use crate::{ResourceKind, ResourceLimitError, Value};
use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::PI;

pub(super) const FFT_RETAINED_VALUES_PER_BIN: usize = 5;
pub(super) const FFT_RETAINED_VALUES_PER_HARMONIC: usize = 4;
pub(super) const FFT_RETAINED_METRIC_VALUES: usize = 8;
pub(super) const FFT_MAX_RANKED_HARMONICS: usize = 30;
const FFT_DB_NOISE_FLOOR: Value = 1.0e-10;

/// Xyce enables exact FFT sample-time stops unless explicitly disabled. Its
/// output interval schedule is interpolation-only and makes the modes
/// mutually exclusive.
pub(super) fn uses_accurate_sampling(netlist: &Netlist) -> bool {
    netlist.options.fft_accurate.unwrap_or(true)
        && netlist.options.output_interval_schedule.is_none()
}

pub(super) fn sample_time(analysis: &FftAnalysis, transient_stop: Value, sample: usize) -> Value {
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    let interval = (stop - start) / analysis.points as Value;
    start + sample as Value * interval
}

pub(super) fn preflight(
    engine: &Engine,
    netlist: &Netlist,
    transient_stop: Value,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let mut retained_values = 0usize;
    for (index, analysis) in netlist.fft_analyses.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_request(index, analysis, transient_stop)?;
        engine.ensure_analysis_points(analysis.points)?;
        qualify_rustfft_forward_length(analysis.points).map_err(|error| {
            SimulationError::Circuit(format!(
                ".FFT request {} cannot create a qualified transform plan: {error}",
                index + 1
            ))
        })?;
        let mut analysis_values =
            (analysis.points / 2 + 1).saturating_mul(FFT_RETAINED_VALUES_PER_BIN);
        if netlist.options.fft_output_metrics.unwrap_or(false) {
            analysis_values = analysis_values
                .saturating_add(FFT_RETAINED_METRIC_VALUES)
                .saturating_add(
                    (analysis.points / 2)
                        .min(FFT_MAX_RANKED_HARMONICS)
                        .saturating_mul(FFT_RETAINED_VALUES_PER_HARMONIC),
                );
        }
        retained_values = retained_values.saturating_add(analysis_values);
    }
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained_values,
        engine.config.resource_limits.max_result_values,
    )?;
    Ok(())
}

pub(super) fn evaluate(
    engine: &Engine,
    netlist: &Netlist,
    result: &TransientResult,
    transient_stop: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<TransientFftResult>, SimulationError> {
    if netlist.fft_analyses.is_empty() {
        return Ok(Vec::new());
    }
    validate_history(result)?;
    let mut planner = FftPlanner::<Value>::new();
    let mode = netlist.options.fft_mode.unwrap_or_default();
    let accurate_sampling = uses_accurate_sampling(netlist);
    let emit_metrics = netlist.options.fft_output_metrics.unwrap_or(false);
    let mut spectra = Vec::new();
    spectra
        .try_reserve_exact(netlist.fft_analyses.len())
        .map_err(|_| allocation_error("result list"))?;
    for (index, analysis) in netlist.fft_analyses.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let (output_name, physical_type, values) =
            crate::analysis::measure_signals::evaluate_tran_fft_output_with_abort(
                netlist,
                result,
                index,
                engine.config.resource_limits,
                abort,
            )?;
        spectra.push(evaluate_one(
            index,
            analysis,
            mode,
            accurate_sampling,
            emit_metrics,
            output_name,
            physical_type,
            &result.time,
            &values,
            transient_stop,
            &mut planner,
            abort,
        )?);
    }
    Ok(spectra)
}

fn validate_request(
    index: usize,
    analysis: &FftAnalysis,
    transient_stop: Value,
) -> Result<(), SimulationError> {
    if analysis.points < 4 || !analysis.points.is_power_of_two() {
        return Err(request_error(
            index,
            format!(
                "NP must be a power of two no smaller than 4, found {}",
                analysis.points
            ),
        ));
    }
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    if !start.is_finite() || start < 0.0 {
        return Err(request_error(
            index,
            format!("START must be finite and non-negative, found {start}"),
        ));
    }
    if !stop.is_finite() || stop <= start {
        return Err(request_error(
            index,
            format!("STOP must be finite and greater than START ({start}), found {stop}"),
        ));
    }
    if stop > transient_stop {
        return Err(request_error(
            index,
            format!("STOP {stop} exceeds transient stop time {transient_stop}"),
        ));
    }
    for (name, value, allow_zero) in [
        ("FREQ", analysis.fundamental_frequency, false),
        ("FMIN", analysis.minimum_frequency, true),
        ("FMAX", analysis.maximum_frequency, false),
    ] {
        if let Some(value) = value
            && (!value.is_finite() || value < 0.0 || (!allow_zero && value == 0.0))
        {
            return Err(request_error(
                index,
                format!(
                    "{name} must be a {}finite frequency, found {value}",
                    if allow_zero {
                        "non-negative "
                    } else {
                        "positive "
                    }
                ),
            ));
        }
    }
    if let (Some(minimum), Some(maximum)) = (analysis.minimum_frequency, analysis.maximum_frequency)
        && minimum > maximum
    {
        return Err(request_error(
            index,
            format!("FMIN {minimum} exceeds FMAX {maximum}"),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn evaluate_one(
    index: usize,
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    accurate_sampling: bool,
    emit_metrics: bool,
    output_name: String,
    physical_type: &'static str,
    time: &[Value],
    values: &[Value],
    transient_stop: Value,
    planner: &mut FftPlanner<Value>,
    abort: &dyn AbortSignal,
) -> Result<TransientFftResult, SimulationError> {
    validate_request(index, analysis, transient_stop)?;
    if values.len() != time.len() {
        return Err(request_error(
            index,
            format!(
                "resolved output has {} values for {} transient samples",
                values.len(),
                time.len()
            ),
        ));
    }
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    let point_count = analysis.points;
    let sample_interval = (stop - start) / point_count as Value;
    let frequency_resolution = 1.0 / (stop - start);
    let first_target = start;
    let last_target = sample_time(analysis, transient_stop, point_count - 1);
    let history_start = time[0];
    let history_stop = *time.last().expect("validated non-empty history");
    if first_target < history_start || last_target > history_stop {
        return Err(request_error(
            index,
            format!(
                "sample record [{first_target}, {last_target}] lies outside retained transient history [{history_start}, {history_stop}]"
            ),
        ));
    }

    let mut input = Vec::new();
    input
        .try_reserve_exact(point_count)
        .map_err(|_| allocation_error("uniform input record"))?;
    let mut interval = 0usize;
    let denominator = if mode.uses_periodic_windows() {
        point_count as Value
    } else {
        (point_count - 1) as Value
    };
    let mut window_sum = 0.0;
    for sample in 0..point_count {
        if sample.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let target = sample_time(analysis, transient_stop, sample);
        while interval + 1 < time.len() && time[interval + 1] < target {
            interval += 1;
        }
        let value = interpolate_at(time, values, interval, target).ok_or_else(|| {
            request_error(
                index,
                format!("cannot interpolate the resolved output at time {target}"),
            )
        })?;
        if !value.is_finite() {
            return Err(request_error(
                index,
                format!("resolved output is non-finite at uniform sample {sample} (time {target})"),
            ));
        }
        let window = window_coefficient(analysis.window, sample, point_count, denominator);
        window_sum += window;
        input.push(Complex::new(value * window, 0.0));
    }
    let coherent_gain = window_sum / point_count as Value;
    if !coherent_gain.is_finite() || coherent_gain <= 0.0 {
        return Err(request_error(
            index,
            format!("window coherent gain is invalid ({coherent_gain})"),
        ));
    }
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    qualify_rustfft_forward_length(point_count).map_err(|error| {
        request_error(
            index,
            format!("transform planning is not qualified: {error}"),
        )
    })?;
    let fft = planner.plan_fft_forward(point_count);
    let scratch_len = fft.get_inplace_scratch_len();
    let mut scratch = Vec::new();
    scratch
        .try_reserve_exact(scratch_len)
        .map_err(|_| allocation_error("transform scratch"))?;
    scratch.resize(scratch_len, Complex::new(0.0, 0.0));
    fft.process_with_scratch(&mut input, &mut scratch);
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }

    let bin_count = point_count / 2 + 1;
    input.truncate(bin_count);
    let base_scale = 1.0 / (point_count as Value * coherent_gain);
    for (bin, coefficient) in input.iter_mut().enumerate() {
        let one_sided_scale = if bin == 0 || bin == point_count / 2 {
            base_scale
        } else {
            2.0 * base_scale
        };
        *coefficient *= one_sided_scale;
    }
    let format = analysis.format.unwrap_or_else(|| mode.default_format());
    if format == FftFormat::Normalized {
        let largest = input
            .iter()
            .map(|coefficient| coefficient.norm())
            .fold(0.0, Value::max);
        if largest > 0.0 {
            for coefficient in &mut input {
                *coefficient /= largest;
            }
        }
    }

    let nyquist_bin = point_count / 2;
    let fundamental_bin = rounded_frequency_bin(
        index,
        "FREQ",
        analysis.fundamental_frequency,
        1,
        frequency_resolution,
        nyquist_bin,
    )?;
    let maximum_metric_bin = rounded_frequency_bin(
        index,
        "FMAX",
        analysis.maximum_frequency,
        nyquist_bin,
        frequency_resolution,
        nyquist_bin,
    )?;
    let minimum_metric_bin = rounded_frequency_bin(
        index,
        "FMIN",
        analysis.minimum_frequency,
        1,
        frequency_resolution,
        nyquist_bin,
    )?;
    if maximum_metric_bin < minimum_metric_bin
        || (fundamental_bin == 1 && maximum_metric_bin < 2)
        || (fundamental_bin > 1 && maximum_metric_bin < 1)
    {
        return Err(request_error(
            index,
            format!(
                "rounded FREQ/FMIN/FMAX bins are inconsistent ({fundamental_bin}/{minimum_metric_bin}/{maximum_metric_bin})"
            ),
        ));
    }

    let mut bins = Vec::new();
    bins.try_reserve_exact(bin_count)
        .map_err(|_| allocation_error("one-sided spectrum"))?;
    for (bin, coefficient) in input.into_iter().enumerate() {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let magnitude = coefficient.norm();
        let phase_degrees = coefficient.im.atan2(coefficient.re).to_degrees();
        if !coefficient.re.is_finite()
            || !coefficient.im.is_finite()
            || !magnitude.is_finite()
            || !phase_degrees.is_finite()
        {
            return Err(request_error(
                index,
                format!("transform produced a non-finite coefficient at bin {bin}"),
            ));
        }
        bins.push(TransientFftBin {
            index: bin,
            frequency: bin as Value * frequency_resolution,
            real: coefficient.re,
            imaginary: coefficient.im,
            magnitude,
            phase_degrees,
        });
    }
    let metrics = emit_metrics
        .then(|| {
            calculate_metrics(
                index,
                &bins,
                fundamental_bin,
                minimum_metric_bin,
                maximum_metric_bin,
                analysis.minimum_frequency.is_some(),
                abort,
            )
        })
        .transpose()?;
    Ok(TransientFftResult {
        output: analysis.output.clone(),
        output_name,
        physical_type,
        start_time: start,
        stop_time: stop,
        sample_interval,
        point_count,
        accurate_sampling,
        format,
        mode,
        window: analysis.window,
        window_name: analysis.window_name.clone(),
        alpha: analysis.alpha,
        coherent_gain,
        frequency_resolution,
        fundamental_bin,
        minimum_metric_bin,
        maximum_metric_bin,
        bins,
        metrics,
    })
}

fn calculate_metrics(
    request_index: usize,
    bins: &[TransientFftBin],
    fundamental_bin: usize,
    minimum_metric_bin: usize,
    maximum_metric_bin: usize,
    minimum_was_authored: bool,
    abort: &dyn AbortSignal,
) -> Result<TransientFftMetrics, SimulationError> {
    let fundamental = bins.get(fundamental_bin).ok_or_else(|| {
        SimulationError::Circuit(".FFT fundamental bin is unavailable".to_string())
    })?;
    let fundamental_magnitude = fundamental.magnitude;
    if !fundamental_magnitude.is_finite() || fundamental_magnitude <= FFT_DB_NOISE_FLOOR {
        return Err(request_error(
            request_index,
            format!(
                "FFTOUT metrics require a finite first-harmonic magnitude above {FFT_DB_NOISE_FLOOR:e}; bin {fundamental_bin} has {fundamental_magnitude}"
            ),
        ));
    }

    let mut distortion_power = 0.0;
    for bin in
        (fundamental_bin.saturating_mul(2)..=maximum_metric_bin).step_by(fundamental_bin.max(1))
    {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        distortion_power += bins[bin].magnitude.powi(2);
    }
    let thd_ratio = distortion_power.sqrt() / fundamental_magnitude;
    let thd_db = 20.0 * thd_ratio.max(FFT_DB_NOISE_FLOOR).log10();

    let mut noise_and_distortion_power = 0.0;
    for (bin, value) in bins.iter().enumerate().skip(1) {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if bin != fundamental_bin {
            noise_and_distortion_power += value.magnitude.powi(2);
        }
    }
    let sndr_denominator = noise_and_distortion_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let sndr_db = 20.0 * (fundamental_magnitude / sndr_denominator).log10();
    let enob_bits = (sndr_db - 1.76) / 6.02;

    let signal_frequency_limit = maximum_metric_bin.max(fundamental_bin);
    let mut noise_power = 0.0;
    for (bin, value) in bins.iter().enumerate().skip(1) {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if bin % fundamental_bin != 0 || bin > signal_frequency_limit {
            noise_power += value.magnitude.powi(2);
        }
    }
    let snr_ratio = fundamental_magnitude / noise_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let snr_db = 20.0 * snr_ratio.log10();

    let sfdr_lower = if !minimum_was_authored && maximum_metric_bin >= fundamental_bin {
        fundamental_bin
    } else {
        minimum_metric_bin
    };
    let mut largest_spur: Option<&TransientFftBin> = None;
    for (bin, candidate) in bins
        .iter()
        .enumerate()
        .take(maximum_metric_bin + 1)
        .skip(sfdr_lower)
    {
        if bin != fundamental_bin
            && candidate.magnitude > largest_spur.map_or(0.0, |spur| spur.magnitude)
        {
            largest_spur = Some(candidate);
        }
    }
    let sfdr_spur_magnitude = largest_spur.map_or(0.0, |spur| spur.magnitude);
    let sfdr_db =
        20.0 * (fundamental_magnitude / sfdr_spur_magnitude.max(FFT_DB_NOISE_FLOOR)).log10();

    let ranked_len = bins.len().saturating_sub(1).min(FFT_MAX_RANKED_HARMONICS);
    let mut largest_harmonics: Vec<TransientFftHarmonic> = Vec::new();
    largest_harmonics
        .try_reserve_exact(ranked_len)
        .map_err(|_| allocation_error("ranked harmonic list"))?;
    for bin in bins.iter().skip(1) {
        if bin.index.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let harmonic = TransientFftHarmonic {
            rank: 0,
            bin: bin.index,
            frequency: bin.frequency,
            magnitude: bin.magnitude,
            magnitude_db: 20.0 * bin.magnitude.max(FFT_DB_NOISE_FLOOR).log10(),
            phase_degrees: bin.phase_degrees,
        };
        let position = largest_harmonics
            .iter()
            .position(|retained| {
                harmonic.magnitude > retained.magnitude
                    || (harmonic.magnitude == retained.magnitude && harmonic.bin < retained.bin)
            })
            .unwrap_or(largest_harmonics.len());
        if largest_harmonics.len() < ranked_len {
            largest_harmonics.insert(position, harmonic);
        } else if position < ranked_len {
            largest_harmonics.pop();
            largest_harmonics.insert(position, harmonic);
        }
    }
    for (rank, harmonic) in largest_harmonics.iter_mut().enumerate() {
        harmonic.rank = rank + 1;
    }

    for (name, value) in [
        ("fundamental magnitude", fundamental_magnitude),
        ("THD ratio", thd_ratio),
        ("THD dB", thd_db),
        ("SNDR", sndr_db),
        ("ENOB", enob_bits),
        ("SNR", snr_db),
        ("SFDR", sfdr_db),
    ] {
        if !value.is_finite() {
            return Err(request_error(
                request_index,
                format!("FFTOUT {name} is non-finite for the selected spectrum"),
            ));
        }
    }

    Ok(TransientFftMetrics {
        fundamental_magnitude,
        thd_ratio,
        thd_db,
        sndr_db,
        enob_bits,
        snr_db,
        sfdr_db,
        sfdr_spur_bin: largest_spur.map(|spur| spur.index),
        sfdr_spur_frequency: largest_spur.map(|spur| spur.frequency),
        largest_harmonics,
    })
}

fn validate_history(result: &TransientResult) -> Result<(), SimulationError> {
    if result.time.is_empty() {
        return Err(SimulationError::Circuit(
            ".FFT requires at least one retained transient sample".to_string(),
        ));
    }
    if !result.time[0].is_finite()
        || result
            .time
            .windows(2)
            .any(|pair| !pair[1].is_finite() || pair[1] <= pair[0])
    {
        return Err(SimulationError::Circuit(
            ".FFT requires finite, strictly increasing transient times".to_string(),
        ));
    }
    Ok(())
}

fn interpolate_at(
    time: &[Value],
    values: &[Value],
    interval: usize,
    target: Value,
) -> Option<Value> {
    if target == time[interval] {
        return values.get(interval).copied();
    }
    let next = interval.checked_add(1)?;
    if target == *time.get(next)? {
        return values.get(next).copied();
    }
    let left_time = *time.get(interval)?;
    let right_time = *time.get(next)?;
    if target < left_time || target > right_time {
        return None;
    }
    let fraction = (target - left_time) / (right_time - left_time);
    Some(values[interval] + fraction * (values[next] - values[interval]))
}

fn window_coefficient(window: FftWindow, index: usize, points: usize, denominator: Value) -> Value {
    if window == FftWindow::Rectangular {
        return 1.0;
    }
    let x = index as Value / denominator;
    let cosine = |multiple: Value| (multiple * 2.0 * PI * x).cos();
    match window {
        FftWindow::Rectangular => 1.0,
        FftWindow::Bartlett => {
            if (index as Value) < 0.5 * (points - 1) as Value {
                2.0 * x
            } else {
                2.0 - 2.0 * x
            }
        }
        FftWindow::BartlettHann => {
            0.62 - 0.48 * (x - 0.5).abs() + 0.38 * (2.0 * PI * (x - 0.5)).cos()
        }
        FftWindow::Hamming => 0.54 - 0.46 * cosine(1.0),
        FftWindow::Hann | FftWindow::Cosine2 => 0.5 - 0.5 * cosine(1.0),
        FftWindow::Blackman67Db => 0.42323 - 0.49755 * cosine(1.0) + 0.07922 * cosine(2.0),
        FftWindow::Blackman => 0.42 - 0.5 * cosine(1.0) + 0.08 * cosine(2.0),
        FftWindow::BlackmanHarris => {
            0.35875 - 0.48829 * cosine(1.0) + 0.14128 * cosine(2.0) - 0.01168 * cosine(3.0)
        }
        FftWindow::Nuttall => {
            0.3635819 - 0.4891775 * cosine(1.0) + 0.1365995 * cosine(2.0) - 0.0106411 * cosine(3.0)
        }
        FftWindow::HalfCycleSine => (PI * x).sin(),
        FftWindow::HalfCycleSine3 => (PI * x).sin().powi(3),
        FftWindow::HalfCycleSine6 => (PI * x).sin().powi(6),
        FftWindow::Cosine4 => 0.375 - 0.5 * cosine(1.0) + 0.125 * cosine(2.0),
    }
}

/// Compute the coherent gain of the exact FFT window convention used by the
/// transient engine. Public result adapters use this to validate that
/// serialized window metadata describes the coefficients the engine applied.
pub fn transient_fft_window_coherent_gain(
    window: FftWindow,
    mode: XyceFftMode,
    points: usize,
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    if points < 4 || !points.is_power_of_two() {
        return Err(SimulationError::Circuit(
            "FFT coherent gain requires a power-of-two point count of at least four".to_owned(),
        ));
    }
    let denominator = if mode.uses_periodic_windows() {
        points as Value
    } else {
        (points - 1) as Value
    };
    let mut window_sum = 0.0;
    for index in 0..points {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        window_sum += window_coefficient(window, index, points, denominator);
    }
    let coherent_gain = window_sum / points as Value;
    if !coherent_gain.is_finite() || coherent_gain <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "FFT window coherent gain is invalid ({coherent_gain})"
        )));
    }
    Ok(coherent_gain)
}

fn rounded_frequency_bin(
    request_index: usize,
    name: &str,
    requested: Option<Value>,
    default: usize,
    resolution: Value,
    nyquist_bin: usize,
) -> Result<usize, SimulationError> {
    let Some(requested) = requested else {
        return Ok(default);
    };
    let rounded = (requested / resolution).round();
    if !rounded.is_finite() || rounded < 0.0 || rounded > usize::MAX as Value {
        return Err(request_error(
            request_index,
            format!("{name} cannot be represented as a transform bin"),
        ));
    }
    let bin = rounded as usize;
    if name == "FREQ" && bin == 0 {
        return Err(request_error(
            request_index,
            format!("FREQ {requested} Hz rounds below the first transform bin"),
        ));
    }
    if bin > nyquist_bin {
        return Err(request_error(
            request_index,
            format!("{name} {requested} Hz rounds to bin {bin}, above Nyquist bin {nyquist_bin}"),
        ));
    }
    Ok(bin)
}

fn request_error(index: usize, detail: String) -> SimulationError {
    SimulationError::Circuit(format!(".FFT request {}: {detail}", index + 1))
}

fn allocation_error(allocation: &str) -> SimulationError {
    SimulationError::Circuit(format!(
        ".FFT could not reserve memory for its {allocation}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;
    use crate::netlist::FftOutput;

    #[test]
    fn all_symmetric_windows_are_finite_and_have_positive_gain() {
        let windows = [
            FftWindow::Rectangular,
            FftWindow::Bartlett,
            FftWindow::BartlettHann,
            FftWindow::Hamming,
            FftWindow::Hann,
            FftWindow::Blackman67Db,
            FftWindow::Blackman,
            FftWindow::BlackmanHarris,
            FftWindow::Nuttall,
            FftWindow::HalfCycleSine,
            FftWindow::HalfCycleSine3,
            FftWindow::HalfCycleSine6,
            FftWindow::Cosine2,
            FftWindow::Cosine4,
        ];
        for window in windows {
            let coefficients = (0..64)
                .map(|index| window_coefficient(window, index, 64, 63.0))
                .collect::<Vec<_>>();
            assert!(
                coefficients
                    .iter()
                    .all(|coefficient| coefficient.is_finite())
            );
            assert!(coefficients.iter().sum::<Value>() > 0.0);
            for pair in coefficients.iter().zip(coefficients.iter().rev()) {
                assert!((pair.0 - pair.1).abs() < 1.0e-14, "{window:?}");
            }
        }
    }

    #[test]
    fn public_window_gain_preserves_periodic_and_symmetric_conventions() {
        let symmetric = transient_fft_window_coherent_gain(
            FftWindow::Hann,
            XyceFftMode::HspiceCompatible,
            8,
            &NoAbort,
        )
        .expect("symmetric Hann gain");
        let periodic = transient_fft_window_coherent_gain(
            FftWindow::Hann,
            XyceFftMode::SpectreCompatible,
            8,
            &NoAbort,
        )
        .expect("periodic Hann gain");
        assert!((symmetric - 7.0 / 16.0).abs() < 1.0e-14);
        assert!((periodic - 0.5).abs() < 1.0e-14);
        assert!(
            transient_fft_window_coherent_gain(
                FftWindow::Rectangular,
                XyceFftMode::HspiceCompatible,
                6,
                &NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn uniform_interpolation_uses_the_current_segment() {
        let time = [0.0, 0.1, 0.4, 1.0];
        let values = [0.0, 0.2, 0.8, 2.0];
        assert_eq!(interpolate_at(&time, &values, 1, 0.25), Some(0.5));
        assert_eq!(interpolate_at(&time, &values, 2, 1.0), Some(2.0));
    }

    #[test]
    fn output_interval_schedule_selects_interpolation_even_when_accurate_is_authored() {
        let netlist = Netlist::parse(
            "fft output schedule compatibility\nV1 out 0 0\nR1 out 0 1k\n.options output initial_interval=100u\n.options fft fft_accurate=1\n.tran 1u 1m\n.fft v(out) np=8\n.end\n",
        )
        .expect("compatible interpolation schedule parses");

        assert!(!uses_accurate_sampling(&netlist));
    }

    #[test]
    fn one_sided_calibration_does_not_double_the_nyquist_bin() {
        let analysis = FftAnalysis {
            output: FftOutput::Probe("V(OUT)".to_string()),
            start: Some(0.0),
            stop: Some(1.0),
            points: 8,
            format: Some(FftFormat::Unnormalized),
            window: FftWindow::Rectangular,
            window_name: "RECT".to_string(),
            alpha: FftAnalysis::DEFAULT_ALPHA,
            fundamental_frequency: None,
            minimum_frequency: None,
            maximum_frequency: None,
        };
        let time = (0..=8)
            .map(|index| index as Value / 8.0)
            .collect::<Vec<_>>();
        let values = (0..=8)
            .map(|index| if index % 2 == 0 { 1.0 } else { -1.0 })
            .collect::<Vec<_>>();
        let mut planner = FftPlanner::new();
        let result = evaluate_one(
            0,
            &analysis,
            XyceFftMode::HspiceCompatible,
            true,
            false,
            "V(OUT)".to_string(),
            "voltage",
            &time,
            &values,
            1.0,
            &mut planner,
            &NoAbort,
        )
        .expect("exact Nyquist record transforms");

        assert_eq!(result.bins.len(), 5);
        assert!((result.bins[4].magnitude - 1.0).abs() < 1.0e-14);
        assert!(result.bins[..4].iter().all(|bin| bin.magnitude < 1.0e-14));
    }

    #[test]
    fn no_spur_metrics_are_finite_and_use_the_reporting_floor() {
        let bins = (0..=4)
            .map(|index| TransientFftBin {
                index,
                frequency: index as Value,
                real: if index == 1 { 1.0 } else { 0.0 },
                imaginary: 0.0,
                magnitude: if index == 1 { 1.0 } else { 0.0 },
                phase_degrees: 0.0,
            })
            .collect::<Vec<_>>();
        let metrics = calculate_metrics(0, &bins, 1, 1, 4, false, &NoAbort)
            .expect("a resolved pure fundamental has finite floor-limited metrics");

        assert_eq!(metrics.thd_ratio, 0.0);
        assert_eq!(metrics.thd_db, -200.0);
        assert_eq!(metrics.sndr_db, 200.0);
        assert_eq!(metrics.snr_db, 200.0);
        assert_eq!(metrics.sfdr_db, 200.0);
        assert_eq!(metrics.sfdr_spur_bin, None);
        assert_eq!(metrics.sfdr_spur_frequency, None);
        assert!(
            [
                metrics.enob_bits,
                metrics.thd_db,
                metrics.sndr_db,
                metrics.snr_db,
                metrics.sfdr_db,
            ]
            .into_iter()
            .all(Value::is_finite)
        );
    }
}
