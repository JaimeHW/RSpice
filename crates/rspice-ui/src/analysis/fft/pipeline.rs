//! FFT Input Preparation Pipeline
//!
//! Converts raw time-domain simulation output into a numerically robust,
//! uniformly-sampled sequence suitable for FFT processing.
//!
//! Key steps:
//! - Drop invalid/non-monotonic samples
//! - Resample variable-step data to a uniform grid
//! - Anti-alias low-pass filter before decimation
//! - Enforce strict point cap for responsive UI

use std::f64::consts::PI;

/// Minimum usable sample count for FFT processing.
pub const MIN_FFT_SAMPLES: usize = 16;

/// Default point cap for interactive FFT computation.
pub const DEFAULT_MAX_FFT_POINTS: usize = 4096;

const NONUNIFORM_OVERSAMPLE_FACTOR: usize = 4;
const MAX_RESAMPLE_POINTS: usize = 65_536;
const UNIFORMITY_REL_TOL: f64 = 1e-3;

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedFftInput {
    /// Source label (typically waveform name).
    pub name: String,
    /// Uniformly sampled signal values.
    pub samples: Vec<f64>,
    /// Effective sample rate (Hz) for `samples`.
    pub sample_rate: f64,
    /// Original valid sample count before processing.
    pub original_count: usize,
    /// Final decimation factor (1 means no decimation).
    pub decimation_factor: usize,
}

#[derive(Debug, Clone)]
struct UniformSeries {
    samples: Vec<f64>,
    sample_rate: f64,
}

/// Prepare a waveform for FFT analysis.
///
/// Returns `None` when there is insufficient valid data.
pub fn prepare_fft_input(
    name: &str,
    time: &[f64],
    values: &[f64],
    max_points: usize,
) -> Option<PreparedFftInput> {
    let max_points = max_points.max(MIN_FFT_SAMPLES);
    let cleaned = clean_time_series(time, values);
    if cleaned.len() < MIN_FFT_SAMPLES {
        return None;
    }

    let original_count = cleaned.len();
    let mut uniform = if is_uniform_timeline(&cleaned) {
        uniform_from_cleaned(&cleaned)?
    } else {
        let target = choose_resample_count(cleaned.len(), max_points);
        resample_to_uniform(&cleaned, target)?
    };

    let mut decimation_factor = 1usize;
    if uniform.samples.len() > max_points {
        decimation_factor = ceil_div(uniform.samples.len(), max_points);
        uniform = anti_alias_decimate(&uniform, decimation_factor)?;
    }

    // Safety clamp: decimation can still land slightly above cap depending on rounding.
    while uniform.samples.len() > max_points {
        decimation_factor = decimation_factor.saturating_mul(2);
        uniform = anti_alias_decimate(&uniform, 2)?;
    }

    if uniform.samples.len() < MIN_FFT_SAMPLES || !uniform.sample_rate.is_finite() {
        return None;
    }

    Some(PreparedFftInput {
        name: name.to_string(),
        samples: uniform.samples,
        sample_rate: uniform.sample_rate,
        original_count,
        decimation_factor,
    })
}

fn clean_time_series(time: &[f64], values: &[f64]) -> Vec<(f64, f64)> {
    let mut cleaned = Vec::with_capacity(time.len().min(values.len()));
    let mut last_t: Option<f64> = None;

    for (&t, &v) in time.iter().zip(values.iter()) {
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        if let Some(prev) = last_t {
            if t <= prev {
                continue;
            }
        }
        cleaned.push((t, v));
        last_t = Some(t);
    }

    cleaned
}

fn is_uniform_timeline(data: &[(f64, f64)]) -> bool {
    if data.len() < 3 {
        return false;
    }

    let dt_mean = mean_dt(data);
    if !dt_mean.is_finite() || dt_mean <= 0.0 {
        return false;
    }

    let mut max_rel_err: f64 = 0.0;
    for window in data.windows(2) {
        let dt = window[1].0 - window[0].0;
        if dt <= 0.0 || !dt.is_finite() {
            return false;
        }
        let rel_err = ((dt - dt_mean) / dt_mean).abs();
        max_rel_err = max_rel_err.max(rel_err);
    }

    max_rel_err <= UNIFORMITY_REL_TOL
}

fn uniform_from_cleaned(data: &[(f64, f64)]) -> Option<UniformSeries> {
    let dt = mean_dt(data);
    if !dt.is_finite() || dt <= 0.0 {
        return None;
    }
    let sample_rate = 1.0 / dt;
    if !sample_rate.is_finite() || sample_rate <= 0.0 {
        return None;
    }
    let samples = data.iter().map(|(_, y)| *y).collect();
    Some(UniformSeries {
        samples,
        sample_rate,
    })
}

fn resample_to_uniform(data: &[(f64, f64)], target_count: usize) -> Option<UniformSeries> {
    if data.len() < 2 || target_count < MIN_FFT_SAMPLES {
        return None;
    }

    let t_start = data.first()?.0;
    let t_end = data.last()?.0;
    let duration = t_end - t_start;
    if !duration.is_finite() || duration <= 0.0 {
        return None;
    }

    let n = target_count.max(MIN_FFT_SAMPLES);
    let dt = duration / (n.saturating_sub(1) as f64);
    if !dt.is_finite() || dt <= 0.0 {
        return None;
    }

    let sample_rate = 1.0 / dt;
    if !sample_rate.is_finite() || sample_rate <= 0.0 {
        return None;
    }

    let mut samples = Vec::with_capacity(n);
    let mut src_idx = 0usize;

    for i in 0..n {
        let t = t_start + (i as f64) * dt;

        while src_idx + 1 < data.len() && data[src_idx + 1].0 < t {
            src_idx += 1;
        }

        let y = if src_idx + 1 >= data.len() {
            data[data.len() - 1].1
        } else {
            let (t0, y0) = data[src_idx];
            let (t1, y1) = data[src_idx + 1];
            if t1 <= t0 {
                y0
            } else {
                let alpha = ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
                y0 + alpha * (y1 - y0)
            }
        };
        samples.push(y);
    }

    Some(UniformSeries {
        samples,
        sample_rate,
    })
}

fn anti_alias_decimate(input: &UniformSeries, factor: usize) -> Option<UniformSeries> {
    if factor <= 1 {
        return Some(input.clone());
    }
    if input.samples.len() < MIN_FFT_SAMPLES || !input.sample_rate.is_finite() {
        return None;
    }

    let taps = fir_tap_count(factor);
    let cutoff = 0.45 / factor as f64;
    let fir = design_lowpass_fir(taps, cutoff);
    let filtered = apply_fir(&input.samples, &fir);

    let decimated: Vec<f64> = filtered.iter().step_by(factor).copied().collect();
    if decimated.len() < MIN_FFT_SAMPLES {
        return None;
    }

    let sample_rate = input.sample_rate / factor as f64;
    if !sample_rate.is_finite() || sample_rate <= 0.0 {
        return None;
    }

    Some(UniformSeries {
        samples: decimated,
        sample_rate,
    })
}

fn choose_resample_count(cleaned_count: usize, max_points: usize) -> usize {
    let upper = max_points
        .saturating_mul(NONUNIFORM_OVERSAMPLE_FACTOR)
        .clamp(MIN_FFT_SAMPLES, MAX_RESAMPLE_POINTS);
    cleaned_count.clamp(MIN_FFT_SAMPLES, upper)
}

fn mean_dt(data: &[(f64, f64)]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let duration =
        data.last().map(|x| x.0).unwrap_or(0.0) - data.first().map(|x| x.0).unwrap_or(0.0);
    duration / (data.len().saturating_sub(1) as f64)
}

fn ceil_div(n: usize, d: usize) -> usize {
    n.saturating_add(d.saturating_sub(1)) / d.max(1)
}

fn fir_tap_count(factor: usize) -> usize {
    let mut taps = (32 * factor).clamp(63, 1023);
    if taps % 2 == 0 {
        taps += 1;
    }
    taps
}

fn design_lowpass_fir(taps: usize, cutoff: f64) -> Vec<f64> {
    let fc = cutoff.clamp(1e-6, 0.499_999);
    let m = (taps - 1) as f64;
    let center = m * 0.5;
    let mut coeffs = Vec::with_capacity(taps);

    for n in 0..taps {
        let k = n as f64 - center;
        let sinc = if k.abs() < 1e-12 {
            2.0 * fc
        } else {
            (2.0 * PI * fc * k).sin() / (PI * k)
        };

        // Blackman window for strong stopband suppression.
        let w =
            0.42 - 0.5 * (2.0 * PI * n as f64 / m).cos() + 0.08 * (4.0 * PI * n as f64 / m).cos();
        coeffs.push(sinc * w);
    }

    let sum: f64 = coeffs.iter().sum();
    if sum.abs() > 0.0 {
        for c in &mut coeffs {
            *c /= sum;
        }
    }
    coeffs
}

fn apply_fir(samples: &[f64], coeffs: &[f64]) -> Vec<f64> {
    if samples.is_empty() || coeffs.is_empty() {
        return Vec::new();
    }

    let half = (coeffs.len() / 2) as isize;
    let last = samples.len().saturating_sub(1) as isize;
    let mut out = vec![0.0; samples.len()];

    for (i, out_sample) in out.iter_mut().enumerate() {
        let mut acc = 0.0;
        for (tap_idx, &c) in coeffs.iter().enumerate() {
            let src = i as isize + tap_idx as isize - half;
            let src_idx = src.clamp(0, last) as usize;
            acc += c * samples[src_idx];
        }
        *out_sample = acc;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine(freq: f64, sample_rate: f64, n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| {
                let t = i as f64 / sample_rate;
                (2.0 * PI * freq * t).sin()
            })
            .collect()
    }

    fn rms(signal: &[f64]) -> f64 {
        if signal.is_empty() {
            return 0.0;
        }
        let mean_power = signal.iter().map(|x| x * x).sum::<f64>() / signal.len() as f64;
        mean_power.sqrt()
    }

    #[test]
    fn test_clean_time_series_filters_invalid_and_nonmonotonic() {
        let time = vec![0.0, 1.0, f64::NAN, 1.5, 1.4, 2.0];
        let values = vec![0.0, 1.0, 2.0, f64::INFINITY, 4.0, 5.0];
        let cleaned = clean_time_series(&time, &values);
        assert_eq!(
            cleaned,
            vec![(0.0, 0.0), (1.0, 1.0), (1.4, 4.0), (2.0, 5.0)]
        );
    }

    #[test]
    fn test_prepare_fft_input_uniform_no_decimation() {
        let fs = 10_000.0;
        let n = 1024usize;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values = generate_sine(1000.0, fs, n);

        let prepared = prepare_fft_input("V(out)", &time, &values, 4096).expect("prepared");
        assert_eq!(prepared.name, "V(out)");
        assert_eq!(prepared.original_count, n);
        assert_eq!(prepared.samples.len(), n);
        assert!((prepared.sample_rate - fs).abs() < 1e-6);
        assert_eq!(prepared.decimation_factor, 1);
    }

    #[test]
    fn test_prepare_fft_input_enforces_point_cap() {
        let fs = 50_000.0;
        let n = 20_000usize;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values = generate_sine(2000.0, fs, n);

        let prepared = prepare_fft_input("big", &time, &values, 2048).expect("prepared");
        assert!(prepared.samples.len() <= 2048);
        assert!(prepared.decimation_factor >= 1);
        assert!(prepared.sample_rate > 0.0);
    }

    #[test]
    fn test_prepare_fft_input_resamples_nonuniform_data() {
        let fs = 10_000.0;
        let n = 3000usize;
        let mut time = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        let mut t = 0.0;
        for i in 0..n {
            // Deterministic variable-step timeline.
            let jitter = if i % 3 == 0 { 0.85 } else { 1.15 };
            t += jitter / fs;
            time.push(t);
            values.push((2.0 * PI * 500.0 * t).sin());
        }

        let prepared = prepare_fft_input("nonuniform", &time, &values, 1024).expect("prepared");
        assert!(prepared.samples.len() <= 1024);
        assert!(prepared.sample_rate.is_finite());
        assert!(prepared.sample_rate > 0.0);
        assert!(prepared.decimation_factor >= 1);
    }

    #[test]
    fn test_anti_alias_decimate_reduces_out_of_band_component() {
        let fs = 20_000.0;
        let n = 8192usize;
        let low = generate_sine(300.0, fs, n);
        let high = generate_sine(4600.0, fs, n); // > new Nyquist after /4

        let mixed: Vec<f64> = low
            .iter()
            .zip(high.iter())
            .map(|(l, h)| l + 0.8 * h)
            .collect();

        let series = UniformSeries {
            samples: mixed,
            sample_rate: fs,
        };
        let decimated = anti_alias_decimate(&series, 4).expect("decimated");
        assert!(decimated.samples.len() >= MIN_FFT_SAMPLES);
        assert!((decimated.sample_rate - fs / 4.0).abs() < 1e-9);

        // Project on expected 300 Hz sine and aliased high-tone (~400 Hz alias) to
        // confirm low-pass filtering suppresses out-of-band content.
        let proj = |freq: f64| -> f64 {
            let omega = 2.0 * PI * freq / decimated.sample_rate;
            let mut acc = 0.0;
            for (i, &x) in decimated.samples.iter().enumerate() {
                acc += x * (omega * i as f64).sin();
            }
            acc.abs() / decimated.samples.len() as f64
        };

        let low_proj = proj(300.0);
        let alias_proj = proj(400.0); // 4600 Hz aliases to 400 Hz at fs=5000
        assert!(low_proj > alias_proj * 3.0);
    }

    #[test]
    fn test_prepare_fft_input_rejects_too_small() {
        let time = vec![0.0, 1.0, 2.0];
        let values = vec![0.0, 1.0, 0.0];
        assert!(prepare_fft_input("small", &time, &values, 64).is_none());
    }

    #[test]
    fn test_apply_fir_preserves_dc_gain() {
        let samples = vec![1.0; 2048];
        let coeffs = design_lowpass_fir(129, 0.2);
        let filtered = apply_fir(&samples, &coeffs);
        let out_rms = rms(&filtered);
        assert!((out_rms - 1.0).abs() < 1e-3);
    }
}
