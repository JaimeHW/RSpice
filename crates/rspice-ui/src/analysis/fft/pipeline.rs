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
pub const DEFAULT_MAX_FFT_POINTS: usize = 65_536;

const NONUNIFORM_OVERSAMPLE_FACTOR: usize = 4;
const MAX_RESAMPLE_POINTS: usize = 262_144;
const UNIFORMITY_REL_TOL: f64 = 1e-6;

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

    let mut samples = uniform.samples;
    remove_dc_offset(&mut samples);

    Some(PreparedFftInput {
        name: name.to_string(),
        samples,
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
    let tangents = pchip_tangents(data);

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
                pchip_eval(t0, y0, tangents[src_idx], t1, y1, tangents[src_idx + 1], t)
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

fn pchip_tangents(data: &[(f64, f64)]) -> Vec<f64> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0.0];
    }
    if n == 2 {
        let h = data[1].0 - data[0].0;
        let slope = if h > 0.0 {
            (data[1].1 - data[0].1) / h
        } else {
            0.0
        };
        return vec![slope, slope];
    }

    let mut h = vec![0.0; n - 1];
    let mut delta = vec![0.0; n - 1];
    for i in 0..(n - 1) {
        h[i] = data[i + 1].0 - data[i].0;
        if h[i] > 0.0 {
            delta[i] = (data[i + 1].1 - data[i].1) / h[i];
        }
    }

    let mut m = vec![0.0; n];
    m[0] = pchip_endpoint_tangent(h[0], h[1], delta[0], delta[1]);
    m[n - 1] = pchip_endpoint_tangent(h[n - 2], h[n - 3], delta[n - 2], delta[n - 3]);

    for i in 1..(n - 1) {
        if delta[i - 1] == 0.0 || delta[i] == 0.0 || delta[i - 1].signum() != delta[i].signum() {
            m[i] = 0.0;
            continue;
        }
        let w1 = 2.0 * h[i] + h[i - 1];
        let w2 = h[i] + 2.0 * h[i - 1];
        let denom = w1 / delta[i - 1] + w2 / delta[i];
        m[i] = if denom.abs() > 0.0 {
            (w1 + w2) / denom
        } else {
            0.0
        };
    }

    m
}

fn pchip_endpoint_tangent(h0: f64, h1: f64, d0: f64, d1: f64) -> f64 {
    if !(h0.is_finite() && h1.is_finite() && d0.is_finite() && d1.is_finite()) {
        return 0.0;
    }
    if h0 <= 0.0 || h1 <= 0.0 {
        return d0;
    }

    let mut m0 = ((2.0 * h0 + h1) * d0 - h0 * d1) / (h0 + h1);
    if m0.signum() != d0.signum() {
        m0 = 0.0;
    } else if d0.signum() != d1.signum() && m0.abs() > 3.0 * d0.abs() {
        m0 = 3.0 * d0;
    }
    m0
}

fn pchip_eval(t0: f64, y0: f64, m0: f64, t1: f64, y1: f64, m1: f64, t: f64) -> f64 {
    let h = t1 - t0;
    if !h.is_finite() || h <= 0.0 {
        return y0;
    }
    let u = ((t - t0) / h).clamp(0.0, 1.0);
    let u2 = u * u;
    let u3 = u2 * u;

    let h00 = 2.0 * u3 - 3.0 * u2 + 1.0;
    let h10 = u3 - 2.0 * u2 + u;
    let h01 = -2.0 * u3 + 3.0 * u2;
    let h11 = u3 - u2;

    h00 * y0 + h10 * h * m0 + h01 * y1 + h11 * h * m1
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

fn remove_dc_offset(samples: &mut [f64]) {
    if samples.is_empty() {
        return;
    }
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    if !mean.is_finite() {
        return;
    }
    for sample in samples {
        *sample -= mean;
    }
}

#[cfg(test)]
mod tests {
    use super::super::data::FftData;
    use super::super::window::WindowFunction;
    use super::*;

    fn generate_sine(freq: f64, sample_rate: f64, n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| {
                let t = i as f64 / sample_rate;
                (2.0 * PI * freq * t).sin()
            })
            .collect()
    }

    fn magnitude_db_at_nearest_bin(fft: &FftData, freq: f64) -> f64 {
        let (_, point) = fft
            .points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                (a.frequency - freq)
                    .abs()
                    .total_cmp(&(b.frequency - freq).abs())
            })
            .expect("fft has bins");
        point.magnitude_db()
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
    fn test_prepare_fft_input_treats_subpercent_jitter_as_nonuniform() {
        let fs = 100_000.0;
        let n = 4096usize;
        let dt = 1.0 / fs;

        let mut time = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        for i in 0..n {
            let base_t = i as f64 * dt;
            // ~0.01% deterministic jitter (larger than strict uniformity tolerance).
            let jitter = if i % 2 == 0 {
                1.0e-4 * dt
            } else {
                -1.0e-4 * dt
            };
            let t = base_t + jitter;
            time.push(t);
            values.push((2.0 * PI * 5_000.0 * t).sin());
        }

        let prepared = prepare_fft_input("jittered", &time, &values, n).expect("prepared");
        // Resampling creates exactly target_count samples and derives sample rate from duration.
        assert_eq!(prepared.samples.len(), n);
        assert!(prepared.sample_rate.is_finite());
        assert!(prepared.sample_rate > 0.0);
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

    #[test]
    fn test_prepare_fft_input_removes_dc_offset() {
        let fs = 10_000.0;
        let n = 4096usize;
        let dc = 3.3;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                dc + (2.0 * PI * 1000.0 * t).sin()
            })
            .collect();

        let prepared =
            prepare_fft_input("biased", &time, &values, DEFAULT_MAX_FFT_POINTS).expect("prepared");
        let mean = prepared.samples.iter().sum::<f64>() / prepared.samples.len() as f64;
        assert!(mean.abs() < 1e-9);
    }

    #[test]
    fn test_higher_fft_cap_preserves_high_frequency_tone() {
        let fs = 5_000_000.0;
        let n = 50_000usize;
        let tone = 300_000.0;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values = generate_sine(tone, fs, n);

        let low_cap = 4096usize;
        let prepared_low = prepare_fft_input("tone", &time, &values, low_cap).expect("low cap");
        let fft_low = FftData::from_time_domain(
            "low",
            &prepared_low.samples,
            prepared_low.sample_rate,
            WindowFunction::Hanning,
        );
        let low_peak = fft_low
            .find_peak()
            .map(|(_, p)| p.frequency)
            .expect("low-cap peak");

        let high_cap = DEFAULT_MAX_FFT_POINTS;
        let prepared_high = prepare_fft_input("tone", &time, &values, high_cap).expect("high cap");
        let fft_high = FftData::from_time_domain(
            "high",
            &prepared_high.samples,
            prepared_high.sample_rate,
            WindowFunction::Hanning,
        );
        let high_peak = fft_high
            .find_peak()
            .map(|(_, p)| p.frequency)
            .expect("high-cap peak");

        assert!((high_peak - tone).abs() < fft_high.frequency_resolution() * 2.0);
        assert!((low_peak - tone).abs() > 100_000.0);
    }

    #[test]
    fn test_nonuniform_resampling_preserves_harmonic_levels() {
        let n = 5000usize;
        let duration = 0.02; // 50 Hz bin spacing after uniform resample.
        let f1 = 1000.0;
        let f2 = 2000.0;
        let a2 = 0.1;

        let dt = duration / (n - 1) as f64;
        let mut time = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        for i in 0..n {
            let base_t = i as f64 * dt;
            let jitter = 0.15 * dt * (2.0 * PI * i as f64 / 97.0).sin();
            let t = base_t + jitter;
            time.push(t);
            values.push((2.0 * PI * f1 * t).sin() + a2 * (2.0 * PI * f2 * t + 0.37).sin());
        }

        let prepared =
            prepare_fft_input("harmonic", &time, &values, DEFAULT_MAX_FFT_POINTS).expect("input");
        let fft = FftData::from_time_domain(
            "harmonic",
            &prepared.samples,
            prepared.sample_rate,
            WindowFunction::Hanning,
        );

        let fundamental_db = magnitude_db_at_nearest_bin(&fft, f1);
        let second_db = magnitude_db_at_nearest_bin(&fft, f2);
        let observed_delta = second_db - fundamental_db;
        let expected_delta = 20.0 * a2.log10(); // -20 dB

        assert!(fundamental_db.is_finite());
        assert!(second_db.is_finite());
        assert!((observed_delta - expected_delta).abs() < 1.5);
    }

    #[test]
    fn test_pchip_eval_matches_interval_endpoints() {
        let y0 = pchip_eval(0.0, 2.0, 0.5, 1.0, -1.0, 1.25, 0.0);
        let y1 = pchip_eval(0.0, 2.0, 0.5, 1.0, -1.0, 1.25, 1.0);
        assert!((y0 - 2.0).abs() < 1e-12);
        assert!((y1 + 1.0).abs() < 1e-12);
    }
}
