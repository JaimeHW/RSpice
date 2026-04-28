//! FFT Input Preparation Pipeline
//!
//! Converts raw time-domain simulation output into a numerically robust,
//! uniformly-sampled sequence suitable for FFT processing.
//!
//! Key steps:
//! - Drop invalid/non-monotonic samples
//! - Resample variable-step data to a uniform grid
//! - Anti-alias low-pass filter before decimation
//! - Optional strict point cap for responsive UI paths

use std::f64::consts::PI;

/// Minimum usable sample count for FFT processing.
pub const MIN_FFT_SAMPLES: usize = 16;

/// Default point cap for interactive FFT computation.
pub const DEFAULT_MAX_FFT_POINTS: usize = 65_536;

/// Maximum reference-quality nonuniform resample point count.
///
/// This keeps memory/time bounded while still preserving far more detail than
/// the interactive cap when users need analysis-grade fidelity.
pub const MAX_REFERENCE_RESAMPLE_POINTS: usize = 1_048_576;

const NONUNIFORM_OVERSAMPLE_FACTOR: usize = 4;
const UNIFORMITY_REL_TOL: f64 = 1e-6;

/// FFT input preparation policy.
///
/// - `Reference`: preserve available time-domain detail (no post-resample
///   decimation), bounded only by `MAX_REFERENCE_RESAMPLE_POINTS` for safety.
/// - `Interactive`: enforce a hard point cap for responsiveness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FftInputPolicy {
    Reference,
    Interactive { max_points: usize },
}

impl FftInputPolicy {
    pub const fn reference() -> Self {
        Self::Reference
    }

    pub const fn interactive_default() -> Self {
        Self::Interactive {
            max_points: DEFAULT_MAX_FFT_POINTS,
        }
    }

    pub const fn capped(max_points: usize) -> Self {
        Self::Interactive { max_points }
    }

    fn point_cap(self) -> Option<usize> {
        match self {
            Self::Reference => None,
            Self::Interactive { max_points } => Some(max_points.max(MIN_FFT_SAMPLES)),
        }
    }
}

/// Optional time-domain bounds for FFT input selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftTimeWindow {
    /// Inclusive lower bound (seconds).
    pub start: f64,
    /// Inclusive upper bound (seconds).
    pub end: f64,
}

impl FftTimeWindow {
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    fn normalized(self) -> Option<Self> {
        if self.start.is_finite() && self.end.is_finite() && self.end > self.start {
            Some(self)
        } else {
            None
        }
    }
}

/// End-to-end FFT input preparation options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftInputOptions {
    /// Data fidelity/performance policy.
    pub policy: FftInputPolicy,
    /// Optional selected time-domain window.
    pub time_window: Option<FftTimeWindow>,
    /// Optional target sample count for explicit resampling.
    pub target_samples: Option<usize>,
}

impl Default for FftInputOptions {
    fn default() -> Self {
        Self {
            policy: FftInputPolicy::interactive_default(),
            time_window: None,
            target_samples: None,
        }
    }
}

impl FftInputOptions {
    pub fn with_policy(policy: FftInputPolicy) -> Self {
        Self {
            policy,
            ..Self::default()
        }
    }

    pub fn with_time_window(mut self, time_window: Option<FftTimeWindow>) -> Self {
        self.time_window = time_window;
        self
    }

    pub fn with_target_samples(mut self, target_samples: Option<usize>) -> Self {
        self.target_samples = target_samples;
        self
    }
}

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
    prepare_fft_input_with_policy(name, time, values, FftInputPolicy::capped(max_points))
}

/// Prepare a waveform for FFT analysis using an explicit policy.
///
/// Returns `None` when there is insufficient valid data.
pub fn prepare_fft_input_with_policy(
    name: &str,
    time: &[f64],
    values: &[f64],
    policy: FftInputPolicy,
) -> Option<PreparedFftInput> {
    prepare_fft_input_with_options(name, time, values, FftInputOptions::with_policy(policy))
}

/// Prepare a waveform for FFT analysis using explicit options.
///
/// Returns `None` when there is insufficient valid data.
pub fn prepare_fft_input_with_options(
    name: &str,
    time: &[f64],
    values: &[f64],
    options: FftInputOptions,
) -> Option<PreparedFftInput> {
    let point_cap = options.policy.point_cap();
    let target_samples = options
        .target_samples
        .map(|n| n.clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS));
    let cleaned = clean_time_series(time, values);
    if cleaned.len() < MIN_FFT_SAMPLES {
        return None;
    }

    let windowed = apply_time_window(&cleaned, options.time_window);
    if windowed.len() < MIN_FFT_SAMPLES {
        return None;
    }

    let original_count = windowed.len();
    let mut uniform = if let Some(target) = target_samples {
        if target == windowed.len() && is_uniform_timeline(&windowed) {
            uniform_from_cleaned(&windowed)?
        } else {
            resample_to_uniform(&windowed, target)?
        }
    } else if is_uniform_timeline(&windowed) {
        uniform_from_cleaned(&windowed)?
    } else {
        let target = choose_resample_count(windowed.len(), point_cap);
        resample_to_uniform(&windowed, target)?
    };

    let mut decimation_factor = 1usize;
    if let Some(max_points) = point_cap {
        if uniform.samples.len() > max_points {
            decimation_factor = ceil_div(uniform.samples.len(), max_points);
            uniform = anti_alias_decimate(&uniform, decimation_factor)?;
        }

        // Safety clamp: decimation can still land slightly above cap depending on rounding.
        while uniform.samples.len() > max_points {
            decimation_factor = decimation_factor.saturating_mul(2);
            uniform = anti_alias_decimate(&uniform, 2)?;
        }
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
        if let Some(prev) = last_t
            && t <= prev
        {
            continue;
        }
        cleaned.push((t, v));
        last_t = Some(t);
    }

    cleaned
}

fn apply_time_window(data: &[(f64, f64)], time_window: Option<FftTimeWindow>) -> Vec<(f64, f64)> {
    let Some(window) = time_window.and_then(FftTimeWindow::normalized) else {
        return data.to_vec();
    };
    if data.is_empty() {
        return Vec::new();
    }

    let min_t = data.first().map(|(t, _)| *t).unwrap_or(0.0);
    let max_t = data.last().map(|(t, _)| *t).unwrap_or(0.0);
    let start = window.start.clamp(min_t, max_t);
    let end = window.end.clamp(min_t, max_t);
    if !start.is_finite() || !end.is_finite() || end <= start {
        return Vec::new();
    }

    let start_idx = data.partition_point(|(t, _)| *t < start);
    let end_idx = data.partition_point(|(t, _)| *t <= end);
    if end_idx <= start_idx {
        return Vec::new();
    }

    data[start_idx..end_idx].to_vec()
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

fn choose_resample_count(cleaned_count: usize, point_cap: Option<usize>) -> usize {
    let upper = match point_cap {
        Some(max_points) => max_points
            .saturating_mul(NONUNIFORM_OVERSAMPLE_FACTOR)
            .clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS),
        None => MAX_REFERENCE_RESAMPLE_POINTS,
    };
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
    if crate::utils::numeric::is_multiple_of(taps, 2) {
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

