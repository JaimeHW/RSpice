//! Eye Diagram Measurements
//!
//! Commercial-grade signal integrity measurements from eye diagrams.
//! Includes eye opening, jitter, rise/fall times, and bit error rate estimation.
//!
//! # Measurements that could not be made
//!
//! Several figures here are `Option`. That is deliberate: a rise time of
//! `0 s`, a Q factor of `0`, or a crossing at exactly 50 % of the swing all
//! read as measurements when they are in fact the absence of one, and an eye
//! is exactly the surface where an engineer is entitled to assume that a
//! printed number came from the waveform. Where the data cannot support a
//! figure, this module reports that it cannot.

use super::eye_data::EyeData;
use std::f64::consts::PI;

/// Half-width, in unit intervals, of the phase slice sampled for the eye
/// height. The vertical opening is quoted at the centre of the eye, and ±5 %
/// of a unit interval is the window commercial eye tools use.
const HEIGHT_PHASE_TOLERANCE_UI: f64 = 0.05;
/// Half-width, in unit intervals, of the phase slice sampled for the noise
/// statistics. Wider than the height slice because σ needs a population.
const NOISE_PHASE_TOLERANCE_UI: f64 = 0.1;
/// Fraction of the rail-to-rail amplitude excluded at each rail when
/// bracketing the crossing-level search, so the bracket sits on the edges
/// rather than on the settled levels.
const CROSSING_SEARCH_INSET: f64 = 0.1;
/// Bisection steps for the crossing-level search. The bracket is the eye
/// amplitude, so this resolves the crossing to well under a part in 10^6.
const CROSSING_SEARCH_STEPS: usize = 24;

// =============================================================================
// Eye Measurements
// =============================================================================

/// Complete eye diagram measurements
#[derive(Debug, Clone, Default)]
pub struct EyeMeasurements {
    /// Eye height (vertical opening) in volts
    pub eye_height: f64,
    /// Eye width (horizontal opening) as fraction of UI
    pub eye_width: f64,
    /// Eye area (normalized, height × width)
    pub eye_area: f64,
    /// Vertical eye opening margin at center
    pub vertical_margin: f64,
    /// Horizontal eye opening margin at mid-level
    pub horizontal_margin: f64,
    /// Rise time (20%-80%) in seconds, from complete edges within single
    /// acquisitions. `None` when no acquisition contains a complete rising
    /// edge.
    pub rise_time: Option<f64>,
    /// Fall time (80%-20%) in seconds, on the same terms as [`Self::rise_time`].
    pub fall_time: Option<f64>,
    /// Total jitter (peak-to-peak) in seconds
    pub jitter_pp: f64,
    /// Random jitter (RMS) in seconds
    pub jitter_rms: f64,
    /// Deterministic jitter (peak-to-peak) in seconds
    pub jitter_dj: f64,
    /// Measured crossing level in volts — where the rising and falling edge
    /// families intersect. `None` when they do not intersect within the eye,
    /// which is what duty-cycle distortion large enough to split the crossing
    /// looks like.
    pub crossing_level: Option<f64>,
    /// Crossing level as a fraction of the eye amplitude (one level minus
    /// zero level, both measured at the eye centre).
    pub crossing_percentage: Option<f64>,
    /// Signal-to-noise ratio in dB. `Some(f64::INFINITY)` for a noiseless
    /// eye; `None` when the levels could not be measured.
    pub snr_db: Option<f64>,
    /// Q-factor (quality factor). `Some(f64::INFINITY)` for a noiseless eye;
    /// `None` when the levels could not be measured.
    pub q_factor: Option<f64>,
    /// Estimated BER (bit error rate), derived from [`Self::q_factor`].
    pub estimated_ber: Option<f64>,
    /// Data rate in bits per second
    pub data_rate: f64,
    /// Unit interval in seconds
    pub unit_interval: f64,
}

impl EyeMeasurements {
    /// Create new measurements with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Q-factor from the measured one/zero levels and their noise.
    ///
    /// A noiseless eye has an unbounded Q, which is a real answer and is
    /// reported as such; `None` means the levels themselves are unavailable.
    pub fn calculate_q(v_high: f64, v_low: f64, sigma_high: f64, sigma_low: f64) -> Option<f64> {
        let amplitude = v_high - v_low;
        if !(amplitude.is_finite() && amplitude > 0.0) {
            return None;
        }
        let noise = sigma_high.max(0.0) + sigma_low.max(0.0);
        Some(if noise > 0.0 {
            amplitude / noise
        } else {
            f64::INFINITY
        })
    }

    /// Estimate BER from Q-factor using Q-function approximation
    pub fn estimate_ber_from_q(q: f64) -> f64 {
        if q <= 0.0 {
            return 1.0;
        }
        if q.is_infinite() {
            return 0.0;
        }
        // BER ≈ erfc(Q/√2) / 2 ≈ exp(-Q²/2) / (Q·√(2π))
        let q_sq = q * q;
        if q > 8.0 {
            // Very low BER, use asymptotic
            (-q_sq / 2.0).exp() / (q * (2.0 * PI).sqrt())
        } else {
            // Use complementary error function approximation
            0.5 * erfc_approx(q / 2.0_f64.sqrt())
        }
    }
}

// =============================================================================
// Measurement Calculator
// =============================================================================

/// Calculate comprehensive eye measurements from eye data
pub fn calculate_eye_measurements(data: &EyeData) -> EyeMeasurements {
    let mut m = EyeMeasurements::new();

    if data.traces.is_empty() {
        return m;
    }

    m.unit_interval = data.bit_period;
    m.data_rate = data.data_rate;

    // Calculate eye opening at center of UI
    let center_opening = calculate_eye_opening_at_center(data);
    m.eye_height = center_opening.height;
    m.vertical_margin = center_opening.margin;

    // Calculate horizontal eye opening
    let horizontal_opening = calculate_horizontal_opening(data);
    m.eye_width = horizontal_opening.width_fraction;
    m.horizontal_margin = horizontal_opening.margin_fraction;

    // Calculate eye area
    m.eye_area = m.eye_height * m.eye_width;

    // Calculate jitter from edge crossings
    let jitter_stats = calculate_jitter_stats(data);
    m.jitter_pp = jitter_stats.peak_to_peak;
    m.jitter_rms = jitter_stats.rms;
    m.jitter_dj = jitter_stats.deterministic;

    // The one and zero levels of the eye come from the amplitude histograms
    // at the eye centre, not from the record's extremes: overshoot and
    // ringing are excursions, not levels, and quoting a Q or a crossing
    // percentage against them understates both.
    let noise_stats = calculate_noise_stats(data);

    // Calculate rise/fall times
    let edge_times = calculate_edge_times(data, &noise_stats);
    m.rise_time = edge_times.rise;
    m.fall_time = edge_times.fall;

    if let (Some(level_high), Some(level_low)) = (noise_stats.level_high, noise_stats.level_low) {
        let amplitude = level_high - level_low;
        m.crossing_level = calculate_crossing_level(data, level_low, level_high);
        if amplitude > 0.0 {
            m.crossing_percentage = m
                .crossing_level
                .map(|level| (level - level_low) / amplitude);
        }

        m.q_factor = EyeMeasurements::calculate_q(
            level_high,
            level_low,
            noise_stats.sigma_high,
            noise_stats.sigma_low,
        );
        m.estimated_ber = m.q_factor.map(EyeMeasurements::estimate_ber_from_q);

        if amplitude > 0.0 {
            let noise_power = noise_stats.sigma_high.powi(2) + noise_stats.sigma_low.powi(2);
            m.snr_db = Some(if noise_power > 0.0 {
                10.0 * (amplitude.powi(2) / noise_power).log10()
            } else {
                f64::INFINITY
            });
        }
    }

    m
}

// =============================================================================
// Internal Calculation Types
// =============================================================================

/// Eye opening at center
#[derive(Debug, Clone, Default)]
struct CenterOpening {
    height: f64,
    margin: f64,
}

/// Horizontal opening
#[derive(Debug, Clone, Default)]
struct HorizontalOpening {
    width_fraction: f64,
    margin_fraction: f64,
}

/// Jitter statistics
#[derive(Debug, Clone, Default)]
struct JitterStats {
    peak_to_peak: f64,
    rms: f64,
    deterministic: f64,
}

/// Edge timing statistics
#[derive(Debug, Clone, Default)]
struct EdgeTimes {
    rise: Option<f64>,
    fall: Option<f64>,
}

/// Level and noise statistics sampled at the eye centre.
#[derive(Debug, Clone, Default)]
struct NoiseStats {
    sigma_high: f64,
    sigma_low: f64,
    /// Mean of the one-level population, when it has one.
    level_high: Option<f64>,
    /// Mean of the zero-level population, when it has one.
    level_low: Option<f64>,
}

/// One threshold crossing of a folded acquisition.
#[derive(Debug, Clone, Copy)]
struct FoldedCrossing {
    /// Crossing time modulo one unit interval, in `[0, 1)`.
    phase: f64,
    rising: bool,
}

// =============================================================================
// Internal Calculation Functions
// =============================================================================

fn calculate_eye_opening_at_center(data: &EyeData) -> CenterOpening {
    if data.traces.is_empty() || data.ui_count == 0 {
        return CenterOpening::default();
    }

    // Sample at the centre of the folded window, which the fold anchor puts
    // at the centre of the eye opening.
    let center_ui = data.ui_count as f64 / 2.0;

    let mut high_samples = Vec::new();
    let mut low_samples = Vec::new();

    for trace in &data.traces {
        for (i, &t) in trace.time.iter().enumerate() {
            if (t - center_ui).abs() < HEIGHT_PHASE_TOLERANCE_UI && i < trace.amplitude.len() {
                let v = trace.amplitude[i];
                if v.is_finite() {
                    if v >= data.v_cross {
                        high_samples.push(v);
                    } else {
                        low_samples.push(v);
                    }
                }
            }
        }
    }

    if high_samples.is_empty() || low_samples.is_empty() {
        return CenterOpening::default();
    }

    let v_high_min = high_samples.iter().copied().fold(f64::MAX, f64::min);
    let v_low_max = low_samples.iter().copied().fold(f64::MIN, f64::max);

    let height = (v_high_min - v_low_max).max(0.0);

    CenterOpening {
        height,
        margin: height / 2.0,
    }
}

/// Horizontal eye opening: one unit interval less the spread of the crossings.
///
/// Every crossing of every acquisition, of either polarity, is folded to its
/// phase within one unit interval. The eye is open between the last crossing
/// of one bit boundary and the first crossing of the next, so the width is
/// what a unit interval has left after the crossing distribution takes its
/// share — the definition an oscilloscope reports and the one that yields a
/// full unit interval for a jitter-free signal.
///
/// The predecessor measured `earliest falling − latest rising` over the
/// *unfolded* two-unit-interval window. Those two families are a whole unit
/// interval apart there, so the difference was a negative number clamped to
/// zero: the eye width read 0 UI for every signal, open or closed.
fn calculate_horizontal_opening(data: &EyeData) -> HorizontalOpening {
    let crossings = collect_folded_crossings(data);
    if crossings.len() < 2 {
        return HorizontalOpening::default();
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for crossing in &crossings {
        min = min.min(crossing.phase);
        max = max.max(crossing.phase);
    }
    if !min.is_finite() || !max.is_finite() {
        return HorizontalOpening::default();
    }

    // A spread of a whole unit interval or more is a closed eye, and clamping
    // says so rather than wrapping into a fictitious opening.
    let width_fraction = (1.0 - (max - min)).clamp(0.0, 1.0);

    HorizontalOpening {
        width_fraction,
        margin_fraction: width_fraction / 2.0,
    }
}

/// Every threshold crossing of every acquisition, folded to `[0, 1)`.
fn collect_folded_crossings(data: &EyeData) -> Vec<FoldedCrossing> {
    let mut crossings = Vec::new();
    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            if let Some(crossing) = interpolate_crossing(
                trace.time[i],
                trace.time[i + 1],
                trace.amplitude[i],
                trace.amplitude[i + 1],
                data.v_cross,
            ) {
                crossings.push(FoldedCrossing {
                    phase: crossing.0.rem_euclid(1.0),
                    rising: crossing.1,
                });
            }
        }
    }
    crossings
}

/// Linear crossing of `level` between two samples, with its polarity.
fn interpolate_crossing(t0: f64, t1: f64, v0: f64, v1: f64, level: f64) -> Option<(f64, bool)> {
    if !v0.is_finite() || !v1.is_finite() {
        return None;
    }
    let dt = t1 - t0;
    if !(dt.is_finite() && dt > 0.0) {
        return None;
    }
    if v0 < level && v1 >= level {
        Some((t0 + (level - v0) / (v1 - v0) * dt, true))
    } else if v0 >= level && v1 < level {
        Some((t0 + (v0 - level) / (v0 - v1) * dt, false))
    } else {
        None
    }
}

/// Voltage at which the rising and falling edge families intersect.
///
/// This is the crossing point an eye tool marks, and it is a measurement: at
/// a level below it the rising family crosses earlier than the falling
/// family, above it later, and the level where the two mean phases agree is
/// the crossing. The predecessor reported `(v_cross − v_low) / swing`, where
/// `v_cross` is the midpoint of `v_low` and `v_high` — an identity that
/// returns exactly 50 % for every waveform ever measured.
///
/// `None` when the two families do not meet between the rails, which is what
/// duty-cycle distortion wide enough to split the crossing into two looks
/// like.
fn calculate_crossing_level(data: &EyeData, level_low: f64, level_high: f64) -> Option<f64> {
    let amplitude = level_high - level_low;
    if !(amplitude.is_finite() && amplitude > 0.0) {
        return None;
    }

    let mut low = level_low + CROSSING_SEARCH_INSET * amplitude;
    let mut high = level_high - CROSSING_SEARCH_INSET * amplitude;
    let gap_low = mean_phase_gap_at(data, low)?;
    let gap_high = mean_phase_gap_at(data, high)?;
    if !(gap_low > 0.0 && gap_high < 0.0) {
        return None;
    }

    for _ in 0..CROSSING_SEARCH_STEPS {
        let middle = 0.5 * (low + high);
        match mean_phase_gap_at(data, middle) {
            Some(gap) if gap > 0.0 => low = middle,
            Some(_) => high = middle,
            None => return None,
        }
    }
    Some(0.5 * (low + high))
}

/// Falling-family mean phase minus rising-family mean phase at one level,
/// signed and wrapped to ±half a unit interval.
fn mean_phase_gap_at(data: &EyeData, level: f64) -> Option<f64> {
    let mut rising = CircularPhaseMean::default();
    let mut falling = CircularPhaseMean::default();
    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            let Some((time, is_rising)) = interpolate_crossing(
                trace.time[i],
                trace.time[i + 1],
                trace.amplitude[i],
                trace.amplitude[i + 1],
                level,
            ) else {
                continue;
            };
            if is_rising {
                rising.add(time);
            } else {
                falling.add(time);
            }
        }
    }
    Some(wrap_phase_difference(falling.mean()? - rising.mean()?))
}

/// Circular mean of crossing phases, which cannot be taken arithmetically:
/// a family straddling the fold would average to the opposite phase.
#[derive(Debug, Default, Clone, Copy)]
struct CircularPhaseMean {
    cos_sum: f64,
    sin_sum: f64,
    count: usize,
}

impl CircularPhaseMean {
    fn add(&mut self, time_ui: f64) {
        let angle = 2.0 * PI * time_ui.rem_euclid(1.0);
        self.cos_sum += angle.cos();
        self.sin_sum += angle.sin();
        self.count += 1;
    }

    fn mean(&self) -> Option<f64> {
        (self.count > 0).then(|| self.sin_sum.atan2(self.cos_sum) / (2.0 * PI))
    }
}

/// Wrap a phase difference into `(-0.5, 0.5]` unit intervals.
fn wrap_phase_difference(difference: f64) -> f64 {
    let wrapped = difference.rem_euclid(1.0);
    if wrapped > 0.5 {
        wrapped - 1.0
    } else {
        wrapped
    }
}

fn calculate_jitter_stats(data: &EyeData) -> JitterStats {
    // Keep the two polarities apart: they are two different eye edges, and
    // pooling them would report their separation as jitter.
    let folded = collect_folded_crossings(data);
    let rising_crossings: Vec<f64> = folded
        .iter()
        .filter(|crossing| crossing.rising)
        .map(|crossing| crossing.phase)
        .collect();
    let falling_crossings: Vec<f64> = folded
        .iter()
        .filter(|crossing| !crossing.rising)
        .map(|crossing| crossing.phase)
        .collect();

    let rising = crossing_phase_stats(&rising_crossings);
    let falling = crossing_phase_stats(&falling_crossings);
    if rising.count == 0 && falling.count == 0 {
        return JitterStats::default();
    }

    let peak_to_peak_ui = rising.peak_to_peak_ui.max(falling.peak_to_peak_ui);
    let total_count = rising.count + falling.count;
    let rms_ui = if total_count > 0 {
        ((rising.rms_ui.powi(2) * rising.count as f64
            + falling.rms_ui.powi(2) * falling.count as f64)
            / total_count as f64)
            .sqrt()
    } else {
        0.0
    };

    let peak_to_peak = peak_to_peak_ui * data.bit_period;
    let rms = rms_ui * data.bit_period;
    // Approximate DDJ by removing RJ envelope from measured total jitter.
    let dj = (peak_to_peak - 14.0 * rms).max(0.0);

    JitterStats {
        peak_to_peak,
        rms,
        deterministic: dj,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CrossingPhaseStats {
    peak_to_peak_ui: f64,
    rms_ui: f64,
    count: usize,
}

fn crossing_phase_stats(phases_ui: &[f64]) -> CrossingPhaseStats {
    let mut phases: Vec<f64> = phases_ui
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .map(|v| v.rem_euclid(1.0))
        .collect();
    if phases.len() < 2 {
        return CrossingPhaseStats {
            count: phases.len(),
            ..CrossingPhaseStats::default()
        };
    }
    phases.sort_by(|a, b| a.total_cmp(b));
    let unwrapped = unwrap_circular_phases(&phases);
    if unwrapped.len() < 2 {
        return CrossingPhaseStats {
            count: unwrapped.len(),
            ..CrossingPhaseStats::default()
        };
    }

    let min = unwrapped[0];
    let max = *unwrapped.last().unwrap_or(&min);
    let peak_to_peak_ui = (max - min).max(0.0);
    let mean = unwrapped.iter().sum::<f64>() / unwrapped.len() as f64;
    let variance =
        unwrapped.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / unwrapped.len() as f64;
    CrossingPhaseStats {
        peak_to_peak_ui,
        rms_ui: variance.sqrt(),
        count: unwrapped.len(),
    }
}

fn unwrap_circular_phases(sorted_phases: &[f64]) -> Vec<f64> {
    if sorted_phases.len() < 2 {
        return sorted_phases.to_vec();
    }

    let mut max_gap = f64::MIN;
    let mut cut_after = 0usize;
    for idx in 0..sorted_phases.len() {
        let next_idx = (idx + 1) % sorted_phases.len();
        let current = sorted_phases[idx];
        let next = if next_idx == 0 {
            sorted_phases[next_idx] + 1.0
        } else {
            sorted_phases[next_idx]
        };
        let gap = next - current;
        if gap > max_gap {
            max_gap = gap;
            cut_after = idx;
        }
    }

    let start = (cut_after + 1) % sorted_phases.len();
    let mut out = Vec::with_capacity(sorted_phases.len());
    for i in 0..sorted_phases.len() {
        let idx = (start + i) % sorted_phases.len();
        let mut value = sorted_phases[idx];
        if idx < start {
            value += 1.0;
        }
        out.push(value);
    }
    out
}

/// 20–80 % rise and 80–20 % fall, measured inside single acquisitions.
///
/// The predecessor concatenated every acquisition into one array and ran a
/// look-ahead search across it, so a 20 % crossing at the end of one folded
/// window paired with an 80 % crossing at the start of the next — whose time
/// axis restarts at zero. That reported edges that no acquisition contains,
/// including negative ones, and reported `0 s` when it found none at all.
fn calculate_edge_times(data: &EyeData, levels: &NoiseStats) -> EdgeTimes {
    // Prefer the measured one/zero levels; fall back to the record extremes
    // when the eye centre had no population to measure them from.
    let (v_low, v_high) = match (levels.level_low, levels.level_high) {
        (Some(low), Some(high)) if high > low => (low, high),
        _ => (data.v_low, data.v_low + data.swing),
    };
    let amplitude = v_high - v_low;
    if !(amplitude.is_finite() && amplitude > 0.0) {
        return EdgeTimes::default();
    }
    let v_20 = v_low + 0.2 * amplitude;
    let v_80 = v_low + 0.8 * amplitude;

    let mut rising = Vec::new();
    let mut falling = Vec::new();
    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        let times: Vec<f64> = trace.time[..n]
            .iter()
            .map(|t| t * data.bit_period)
            .collect();
        collect_trace_edges(&times, &trace.amplitude[..n], v_20, v_80, true, &mut rising);
        collect_trace_edges(
            &times,
            &trace.amplitude[..n],
            v_20,
            v_80,
            false,
            &mut falling,
        );
    }

    EdgeTimes {
        rise: mean_of(&rising),
        fall: mean_of(&falling),
    }
}

/// Complete threshold-to-threshold transitions within one acquisition.
///
/// A transition counts only when the waveform reaches the far threshold
/// without first falling back through the near one, so a truncated edge at a
/// window boundary contributes nothing rather than pairing with whatever
/// follows it.
fn collect_trace_edges(
    times: &[f64],
    amplitudes: &[f64],
    v_20: f64,
    v_80: f64,
    rising: bool,
    out: &mut Vec<f64>,
) {
    let (near, far) = if rising { (v_20, v_80) } else { (v_80, v_20) };
    let n = times.len().min(amplitudes.len());
    let mut i = 0usize;
    while i + 1 < n {
        let Some((start, polarity)) = interpolate_crossing(
            times[i],
            times[i + 1],
            amplitudes[i],
            amplitudes[i + 1],
            near,
        ) else {
            i += 1;
            continue;
        };
        if polarity != rising {
            i += 1;
            continue;
        }

        // Walk the run that stays beyond the near threshold, starting with
        // the segment the near crossing itself sits in — a fast edge clears
        // both thresholds between one pair of samples.
        let mut j = i;
        while j + 1 < n {
            if j > i {
                let value = amplitudes[j];
                let beyond_near = if rising { value >= near } else { value <= near };
                if !beyond_near {
                    break;
                }
            }
            if let Some((end, far_polarity)) = interpolate_crossing(
                times[j],
                times[j + 1],
                amplitudes[j],
                amplitudes[j + 1],
                far,
            ) && far_polarity == rising
            {
                let duration = end - start;
                if duration.is_finite() && duration > 0.0 {
                    out.push(duration);
                }
                break;
            }
            j += 1;
        }
        i = j + 1;
    }
}

fn mean_of(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// One/zero levels and their noise, from the amplitude histograms in a slice
/// through the centre of the eye.
fn calculate_noise_stats(data: &EyeData) -> NoiseStats {
    let center_ui = data.ui_count as f64 / 2.0;

    let mut high_samples = Vec::new();
    let mut low_samples = Vec::new();

    for trace in &data.traces {
        for (i, &t) in trace.time.iter().enumerate() {
            if (t - center_ui).abs() < NOISE_PHASE_TOLERANCE_UI && i < trace.amplitude.len() {
                let v = trace.amplitude[i];
                if v.is_finite() {
                    if v >= data.v_cross {
                        high_samples.push(v);
                    } else {
                        low_samples.push(v);
                    }
                }
            }
        }
    }

    let (level_high, sigma_high) = level_and_sigma(&high_samples);
    let (level_low, sigma_low) = level_and_sigma(&low_samples);

    NoiseStats {
        sigma_high,
        sigma_low,
        level_high,
        level_low,
    }
}

fn level_and_sigma(samples: &[f64]) -> (Option<f64>, f64) {
    let Some(mean) = mean_of(samples) else {
        return (None, 0.0);
    };
    let sigma = if samples.len() > 1 {
        (samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / samples.len() as f64).sqrt()
    } else {
        0.0
    };
    (Some(mean), sigma)
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Complementary error function approximation
fn erfc_approx(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x.powi(2) - 1.26551223
            + 1.00002368 * t
            + 0.37409196 * t.powi(2)
            + 0.09678418 * t.powi(3)
            - 0.18628806 * t.powi(4)
            + 0.27886807 * t.powi(5)
            - 1.13520398 * t.powi(6)
            + 1.48851587 * t.powi(7)
            - 0.82215223 * t.powi(8)
            + 0.17087277 * t.powi(9))
        .exp();

    if x >= 0.0 { tau } else { 2.0 - tau }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::signal_integrity::eye_data::EyeDataBuilder;
    use crate::analysis::signal_integrity::eye_data::EyeTrace;
    use crate::analysis::signal_integrity::eye_test_signals::{
        DT, LevelEvent, RISE_2080, UI, clock_events, dcd_clock_events, nrz_events,
        overshooting_clock_events, prbs7_bits, trapezoid,
    };
    use crate::analysis::signal_integrity::unit_interval::{estimate_unit_interval, fold_anchor};
    use std::f64::consts::TAU;

    /// The production build path: recover the unit interval, then fold at it
    /// with the crossings anchored to half-integer phases.
    fn build_eye(time: &[f64], signal: &[f64]) -> EyeData {
        let estimate = estimate_unit_interval(time, signal).expect("stimulus has a bit period");
        EyeDataBuilder::new()
            .bit_period(estimate.unit_interval)
            .ui_count(2)
            .skip_initial(2)
            .fold_anchor(fold_anchor(
                estimate.mean_crossing_phase,
                estimate.unit_interval,
            ))
            .build(time, signal)
    }

    fn eye_of(initial: f64, events: &[LevelEvent], t_end: f64) -> EyeData {
        let (time, signal) = trapezoid(initial, events, t_end, DT, RISE_2080);
        build_eye(&time, &signal)
    }

    fn ideal_clock_eye(bits: usize, t_start: f64) -> EyeData {
        let (initial, events) = clock_events(bits, t_start, UI, 0.0, 1.0, |_| 0.0);
        eye_of(initial, &events, t_start + (bits as f64 + 1.0) * UI)
    }

    fn prbs7_eye(t_start: f64) -> EyeData {
        let bits = prbs7_bits(300);
        let (initial, events) = nrz_events(&bits, t_start, UI, 0.0, 1.0);
        eye_of(initial, &events, t_start + 302.0 * UI)
    }

    fn jittered_clock_eye(bits: usize, t_start: f64, amplitude: f64, frequency: f64) -> EyeData {
        let (initial, events) = clock_events(bits, t_start, UI, 0.0, 1.0, |n| {
            amplitude * (TAU * frequency * n as f64 * UI).sin()
        });
        eye_of(initial, &events, t_start + (bits as f64 + 1.0) * UI)
    }

    fn dcd_clock_eye(cycles: usize, t_start: f64) -> EyeData {
        let (initial, events) = dcd_clock_events(cycles, t_start, UI, 0.6, 0.0, 1.0);
        eye_of(initial, &events, t_start + (2 * cycles + 2) as f64 * UI)
    }

    /// An ideal clock has a full unit interval of horizontal opening and the
    /// whole swing of vertical opening; anything less is the fold, not the
    /// signal.
    #[test]
    fn ideal_clock_eye_opens_a_full_unit_interval() {
        let data = ideal_clock_eye(40, 0.137e-9);
        let m = calculate_eye_measurements(&data);

        assert!(m.eye_width >= 0.98, "eye width {} UI", m.eye_width);
        assert!(m.eye_height >= 0.98, "eye height {} V", m.eye_height);
        assert!(
            m.jitter_pp <= 1e-3 * UI,
            "jitter p-p {:e} s on a jitter-free clock",
            m.jitter_pp
        );
        let crossing = m.crossing_percentage.expect("symmetric edges cross");
        assert!(
            (crossing - 0.5).abs() <= 0.01,
            "crossing {crossing} of the eye amplitude"
        );
        assert_eq!(m.q_factor, Some(f64::INFINITY), "noiseless eye");
        assert_eq!(m.estimated_ber, Some(0.0), "noiseless eye");
    }

    #[test]
    fn prbs7_eye_opens_a_full_unit_interval() {
        let data = prbs7_eye(0.137e-9);
        let m = calculate_eye_measurements(&data);

        assert!(m.eye_width >= 0.98, "eye width {} UI", m.eye_width);
        assert!(m.eye_height >= 0.98, "eye height {} V", m.eye_height);
    }

    /// Sinusoidal jitter of amplitude A closes the eye by 2A and shows a
    /// peak-to-peak of 2A with an RMS of A/√2 — all three from the same
    /// closed form.
    #[test]
    fn sinusoidal_jitter_matches_its_closed_form() {
        let amplitude = 0.05 * UI;
        let data = jittered_clock_eye(1000, 0.137e-9, amplitude, 3.7e6);
        let m = calculate_eye_measurements(&data);

        let expected_pp = 2.0 * amplitude;
        assert!(
            (m.jitter_pp - expected_pp).abs() <= 0.1 * expected_pp,
            "jitter p-p {:e} s, expected {:e}",
            m.jitter_pp,
            expected_pp
        );
        let expected_rms = amplitude / 2.0_f64.sqrt();
        assert!(
            (m.jitter_rms - expected_rms).abs() <= 0.1 * expected_rms,
            "jitter rms {:e} s, expected {:e}",
            m.jitter_rms,
            expected_rms
        );
        let expected_width = 1.0 - 2.0 * amplitude / UI;
        assert!(
            (m.eye_width - expected_width).abs() <= 0.02,
            "eye width {} UI, expected {}",
            m.eye_width,
            expected_width
        );
    }

    /// A 60/40 clock puts its two crossing families 0.2 UI apart, so the eye
    /// is exactly 0.8 UI wide while staying vertically wide open.
    #[test]
    fn duty_cycle_distortion_closes_the_eye_by_its_skew() {
        let data = dcd_clock_eye(60, 0.137e-9);
        let m = calculate_eye_measurements(&data);

        assert!(
            (m.eye_width - 0.80).abs() <= 0.02,
            "eye width {} UI, expected 0.80",
            m.eye_width
        );
        assert!(m.eye_height >= 0.98, "eye height {} V", m.eye_height);
    }

    /// The headline invariant: where the record happens to start is not a
    /// property of the signal, so no measurement may depend on it.
    #[test]
    fn eye_metrics_are_invariant_to_the_record_phase() {
        let reference = ideal_clock_eye(200, 0.137e-9);
        let baseline = calculate_eye_measurements(&reference);

        for shift in [0.37 * UI, 0.5 * UI] {
            let data = ideal_clock_eye(200, 0.137e-9 + shift);
            let m = calculate_eye_measurements(&data);
            for (name, expected, actual) in [
                ("unit_interval", baseline.unit_interval, m.unit_interval),
                ("eye_width", baseline.eye_width, m.eye_width),
                ("eye_height", baseline.eye_height, m.eye_height),
                ("eye_area", baseline.eye_area, m.eye_area),
                (
                    "crossing_percentage",
                    baseline.crossing_percentage.expect("baseline crossing"),
                    m.crossing_percentage.expect("shifted crossing"),
                ),
                (
                    "rise_time",
                    baseline.rise_time.expect("baseline rise"),
                    m.rise_time.expect("shifted rise"),
                ),
                (
                    "fall_time",
                    baseline.fall_time.expect("baseline fall"),
                    m.fall_time.expect("shifted fall"),
                ),
                ("jitter_pp", baseline.jitter_pp, m.jitter_pp),
            ] {
                let scale = expected.abs().max(1e-12);
                assert!(
                    (actual - expected).abs() <= 1e-3 * scale,
                    "{name} moved from {expected} to {actual} under a {shift:e} s shift"
                );
            }
        }
    }

    /// The crossing point is where the rising and falling edge families
    /// intersect, which is a measurement. Reporting the midpoint of the
    /// record's extremes instead is an identity — it yields 50 % for every
    /// signal ever measured, including this one, whose edges provably meet
    /// at 1/1.625 of the eye amplitude.
    #[test]
    fn crossing_level_is_measured_rather_than_assumed_to_be_mid_swing() {
        let (initial, events) = overshooting_clock_events(60, 0.137e-9, UI, 0.0, 1.0, 1.6, 150e-12);
        let data = eye_of(initial, &events, 0.137e-9 + 62.0 * UI);
        let m = calculate_eye_measurements(&data);

        // Rising reaches v on a 0 → 1.6 ramp, falling leaves it on a
        // 1.0 → 0 ramp of the same width; they meet where
        // v / 1.6 - 0.5 = 0.5 - v, i.e. v = 1 / 1.625.
        let expected = 1.0 / 1.625;
        let crossing = m.crossing_percentage.expect("the edge families intersect");
        assert!(
            (crossing - expected).abs() <= 0.02,
            "crossing {crossing} of the eye amplitude, expected {expected}"
        );
        // The one level is the settled 1.0 V, not the 1.6 V overshoot peak.
        let level = m.crossing_level.expect("the edge families intersect");
        assert!((level - expected).abs() <= 0.02, "crossing level {level} V");
    }

    /// Duty-cycle distortion wide enough to split the crossing leaves no
    /// single level where the two edge families meet. Saying so is the
    /// measurement; 50 % would be a fabrication.
    #[test]
    fn a_split_crossing_reports_no_crossing_level() {
        let data = dcd_clock_eye(60, 0.137e-9);
        let m = calculate_eye_measurements(&data);
        assert_eq!(m.crossing_level, None);
        assert_eq!(m.crossing_percentage, None);
    }

    /// An acquisition set with no complete edge has no edge time. Reporting
    /// `0 s` claims an infinitely fast driver.
    #[test]
    fn unmeasurable_edges_report_nothing_rather_than_zero() {
        let mut data = EyeData::new(1e-9, 2);
        data.v_low = 0.0;
        data.v_high = 1.0;
        data.swing = 1.0;
        data.v_cross = 0.5;
        data.add_trace(EyeTrace::new(
            vec![0.0, 0.5, 1.0, 1.5, 2.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
        ));

        let m = calculate_eye_measurements(&data);
        assert_eq!(m.rise_time, None);
        assert_eq!(m.fall_time, None);
    }

    /// Rise and fall are properties of one acquisition. Pairing a 20 %
    /// crossing in one folded window with an 80 % crossing in the next
    /// measures the fold, not the driver.
    #[test]
    fn rise_time_never_pairs_thresholds_across_folded_acquisitions() {
        let mut data = EyeData::new(1e-9, 2);
        data.v_low = 0.0;
        data.v_high = 1.0;
        data.swing = 1.0;
        data.v_cross = 0.5;
        // Truncated: crosses the 20 % threshold and the window ends.
        data.add_trace(EyeTrace::new(
            vec![0.0, 0.5, 1.0, 1.5, 2.0],
            vec![0.0, 0.0, 0.0, 0.1, 0.3],
        ));
        // Complete: 20 % at 1.1 UI, 80 % at 1.4 UI — a 0.3 ns edge.
        data.add_trace(EyeTrace::new(
            vec![0.0, 0.5, 1.0, 1.5, 2.0],
            vec![0.0, 0.0, 0.0, 1.0, 1.0],
        ));

        let m = calculate_eye_measurements(&data);
        let rise = m.rise_time.expect("one acquisition has a complete edge");
        assert!(
            (rise - 0.3e-9).abs() <= 0.01e-9,
            "rise {rise:e} s, expected 3e-10 from the one complete edge"
        );
    }

    #[test]
    fn rise_and_fall_report_the_generated_edge() {
        let data = ideal_clock_eye(40, 0.137e-9);
        let m = calculate_eye_measurements(&data);

        let rise = m.rise_time.expect("the clock has rising edges");
        assert!(
            (rise - RISE_2080).abs() <= 0.05 * RISE_2080,
            "rise {rise:e} s, expected {RISE_2080:e}"
        );
        let fall = m.fall_time.expect("the clock has falling edges");
        assert!(
            (fall - RISE_2080).abs() <= 0.05 * RISE_2080,
            "fall {fall:e} s, expected {RISE_2080:e}"
        );
    }
}
