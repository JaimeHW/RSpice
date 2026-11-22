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
use super::jitter_fit::{Bathtub, bathtub, gaussian_tail_departure};
use super::normal::q_function;
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

/// Fewest levels an eye is measured with: NRZ.
pub const MIN_EYE_LEVELS: u8 = 2;
/// Most levels an eye is measured with: PAM4.
pub const MAX_EYE_LEVELS: u8 = 4;

/// What an eye is measured with beyond its folded acquisitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeMeasureOptions {
    /// Signal levels, from [`MIN_EYE_LEVELS`] to [`MAX_EYE_LEVELS`].
    pub levels: u8,
    /// Where in the folded window, in unit intervals, the height is read:
    /// the window's centre when `None`.
    pub height_at: Option<f64>,
    /// The level, in volts, the width of a two-level eye is read at: the
    /// crossing threshold when `None`.
    pub width_at: Option<f64>,
}

impl Default for EyeMeasureOptions {
    fn default() -> Self {
        Self {
            levels: MIN_EYE_LEVELS,
            height_at: None,
            width_at: None,
        }
    }
}

/// One signal level of the eye at the sampling phase.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeLevel {
    /// Mean of the level's samples, in volts.
    pub mean: f64,
    /// Their standard deviation, in volts.
    pub sigma: f64,
}

/// One opening of the eye, between two neighbouring levels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeOpening {
    /// The decision threshold between the two levels, in volts: midway
    /// between their means.
    pub threshold: f64,
    /// Vertical opening at the sampling phase, in volts.
    pub height: f64,
    /// Horizontal opening at the threshold, as a fraction of the unit
    /// interval.
    pub width: f64,
    /// Q factor of the two levels; `None` when either has no samples.
    pub q_factor: Option<f64>,
}

/// The bit-error rate the eye's levels support, and why there is none when
/// there is none.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BerEstimate {
    /// The levels could not be measured, or are too few to judge.
    #[default]
    Unmeasured,
    /// The Gaussian tail of the Q factor at the sampling phase: Q(Q).
    FromQ(f64),
    /// The spread at the sampling phase is not random noise — none at all,
    /// or the bounded, stepped spread of intersymbol interference — so a
    /// Gaussian tail extrapolated from it is not an error rate.
    NoRandomNoise,
}

impl BerEstimate {
    /// The estimated rate, when there is one.
    #[must_use]
    pub fn value(self) -> Option<f64> {
        match self {
            Self::FromQ(ber) => Some(ber),
            Self::Unmeasured | Self::NoRandomNoise => None,
        }
    }
}

/// Complete eye diagram measurements
#[derive(Debug, Clone, Default)]
pub struct EyeMeasurements {
    /// Eye height (vertical opening) in volts: the narrowest opening of a
    /// multi-level eye.
    pub eye_height: f64,
    /// Eye width (horizontal opening) as fraction of UI: the narrowest
    /// opening of a multi-level eye.
    pub eye_width: f64,
    /// Eye area: height times width, in volt unit intervals.
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
    /// Peak-to-peak spread of the crossing times in seconds, each edge
    /// polarity about its own: random and deterministic jitter together.
    pub jitter_pp: f64,
    /// RMS spread of the crossing times in seconds, on the same terms.
    pub jitter_rms: f64,
    /// The crossings' bathtub, and the dual-Dirac fit that separates their
    /// random jitter from their deterministic jitter when there are enough
    /// of them.
    pub bathtub: Bathtub,
    /// Measured crossing level in volts — where the rising and falling edge
    /// families intersect. `None` when they do not intersect within the eye,
    /// which is what duty-cycle distortion large enough to split the crossing
    /// looks like.
    pub crossing_level: Option<f64>,
    /// Crossing level as a fraction of the eye amplitude (one level minus
    /// zero level, both measured at the eye centre).
    pub crossing_percentage: Option<f64>,
    /// Highest level's mean less the lowest's, in volts.
    pub amplitude: Option<f64>,
    /// Signal-to-noise ratio in dB. `Some(f64::INFINITY)` for a noiseless
    /// eye; `None` when the levels could not be measured.
    pub snr_db: Option<f64>,
    /// Q-factor (quality factor). `Some(f64::INFINITY)` for a noiseless eye;
    /// `None` when the levels could not be measured. The lowest opening's of
    /// a multi-level eye.
    pub q_factor: Option<f64>,
    /// Bit-error rate estimated from [`Self::q_factor`], when the spread at
    /// the sampling phase is random noise.
    pub ber: BerEstimate,
    /// The levels at the sampling phase, lowest first.
    pub levels: Vec<EyeLevel>,
    /// The openings between them, lowest first.
    pub openings: Vec<EyeOpening>,
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

    /// The bit-error rate of a Q factor: the Gaussian tail Q(Q) = ½·erfc(Q/√2),
    /// on the complementary error function at every Q.
    pub fn estimate_ber_from_q(q: f64) -> f64 {
        if q <= 0.0 {
            return 1.0;
        }
        if q.is_infinite() {
            return 0.0;
        }
        q_function(q)
    }
}

// =============================================================================
// Measurement Calculator
// =============================================================================

/// Calculate the measurements of a two-level eye.
pub fn calculate_eye_measurements(data: &EyeData) -> EyeMeasurements {
    calculate_eye_measurements_with(data, &EyeMeasureOptions::default())
}

/// Calculate an eye's measurements with `options`.
pub fn calculate_eye_measurements_with(
    data: &EyeData,
    options: &EyeMeasureOptions,
) -> EyeMeasurements {
    let mut m = EyeMeasurements::new();

    if data.traces.is_empty() {
        return m;
    }

    m.unit_interval = data.bit_period;
    m.data_rate = data.data_rate;
    let levels = options.levels.clamp(MIN_EYE_LEVELS, MAX_EYE_LEVELS);
    let sampling_phase = options
        .height_at
        .filter(|phase| phase.is_finite())
        .unwrap_or(data.ui_count as f64 / 2.0);

    // Calculate eye opening at center of UI
    let center_opening = calculate_eye_opening_at(data, sampling_phase);
    m.eye_height = center_opening.height;
    m.vertical_margin = center_opening.margin;

    // Calculate horizontal eye opening
    let width_level = options
        .width_at
        .filter(|level| level.is_finite())
        .unwrap_or(data.v_cross);
    let horizontal_opening = calculate_horizontal_opening(data, width_level);
    m.eye_width = horizontal_opening.width_fraction;
    m.horizontal_margin = horizontal_opening.margin_fraction;

    // Calculate jitter from edge crossings
    let jitter_stats = calculate_jitter_stats(data);
    m.jitter_pp = jitter_stats.peak_to_peak;
    m.jitter_rms = jitter_stats.rms;

    // The one and zero levels of the eye come from the amplitude histograms
    // at the eye centre, not from the record's extremes: overshoot and
    // ringing are excursions, not levels, and quoting a Q or a crossing
    // percentage against them understates both.
    let noise_stats = calculate_noise_stats(data, sampling_phase);

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

        if amplitude > 0.0 {
            let noise_power = noise_stats.sigma_high.powi(2) + noise_stats.sigma_low.powi(2);
            m.snr_db = Some(if noise_power > 0.0 {
                10.0 * (amplitude.powi(2) / noise_power).log10()
            } else {
                f64::INFINITY
            });
        }
    }

    if levels == MIN_EYE_LEVELS {
        let populations = split_two_levels(data, sampling_phase, NOISE_PHASE_TOLERANCE_UI);
        m.levels = populations
            .iter()
            .filter_map(|samples| level_of(samples))
            .collect();
        if m.levels.len() == 2 {
            m.amplitude = Some(m.levels[1].mean - m.levels[0].mean);
            m.openings = vec![EyeOpening {
                threshold: data.v_cross,
                height: m.eye_height,
                width: m.eye_width,
                q_factor: m.q_factor,
            }];
        }
        m.ber = ber_estimate(m.q_factor, &populations);
    } else {
        measure_multilevel(data, sampling_phase, levels, &mut m);
    }

    m.eye_area = m.eye_height * m.eye_width;
    m.bathtub = crossing_bathtub(data, jitter_threshold(&m, data));

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

/// Vertical opening at `phase` unit intervals into the folded window.
fn calculate_eye_opening_at(data: &EyeData, phase: f64) -> CenterOpening {
    if data.traces.is_empty() || data.ui_count == 0 {
        return CenterOpening::default();
    }
    let center_ui = phase;

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
fn calculate_horizontal_opening(data: &EyeData, level: f64) -> HorizontalOpening {
    let crossings = collect_folded_crossings(data, level);
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

/// Every crossing of `level` in every acquisition, folded to `[0, 1)`.
fn collect_folded_crossings(data: &EyeData, level: f64) -> Vec<FoldedCrossing> {
    let mut crossings = Vec::new();
    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            if let Some(crossing) = interpolate_crossing(
                trace.time[i],
                trace.time[i + 1],
                trace.amplitude[i],
                trace.amplitude[i + 1],
                level,
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
    let folded = collect_folded_crossings(data, data.v_cross);
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

    JitterStats { peak_to_peak, rms }
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
/// through the eye at `phase`.
fn calculate_noise_stats(data: &EyeData, phase: f64) -> NoiseStats {
    let center_ui = phase;

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
// Levels, openings and the error rate
// =============================================================================

/// How far a level's inner tail may depart from a Gaussian's, in standard
/// deviations of the Q scale, and still be read as random noise. Random
/// noise sits near 0.03 on a record's few thousand centre samples; the
/// stepped spread of intersymbol interference with no noise, above 0.1.
const GAUSSIAN_TAIL_LIMIT: f64 = 0.07;
/// Most rounds of assigning samples to levels and re-centring the levels.
const LEVEL_ROUNDS: usize = 100;

/// Samples within `tolerance` unit intervals of `phase` in every
/// acquisition.
fn samples_near(data: &EyeData, phase: f64, tolerance: f64) -> Vec<f64> {
    let mut samples = Vec::new();
    for trace in &data.traces {
        for (i, &t) in trace.time.iter().enumerate() {
            if (t - phase).abs() < tolerance
                && let Some(&v) = trace.amplitude.get(i)
                && v.is_finite()
            {
                samples.push(v);
            }
        }
    }
    samples
}

/// The zero and one populations near `phase`, split at the crossing
/// threshold, each ascending.
fn split_two_levels(data: &EyeData, phase: f64, tolerance: f64) -> Vec<Vec<f64>> {
    let mut samples = samples_near(data, phase, tolerance);
    samples.sort_by(f64::total_cmp);
    let split = samples.partition_point(|&v| v < data.v_cross);
    let high = samples.split_off(split);
    vec![samples, high]
}

/// Mean and standard deviation of one level's samples.
fn level_of(samples: &[f64]) -> Option<EyeLevel> {
    let (mean, sigma) = level_and_sigma(samples);
    mean.map(|mean| EyeLevel { mean, sigma })
}

/// The error rate the levels support: the Gaussian tail of `q` when every
/// tail facing a threshold is random noise.
///
/// Q is quoted as it is conventionally defined, from the levels' standard
/// deviations, but the tail it implies is an error rate only where those
/// deviations are random. A simulation with no noise source spreads its
/// levels by intersymbol interference alone: a bounded spread whose Gaussian
/// extrapolation reads 1.4e-7 for an eye open by 0.74 V that never errs.
fn ber_estimate(q: Option<f64>, populations: &[Vec<f64>]) -> BerEstimate {
    let Some(q) = q else {
        return BerEstimate::Unmeasured;
    };
    if q.is_infinite() {
        return BerEstimate::NoRandomNoise;
    }
    let last = populations.len().saturating_sub(1);
    for (index, samples) in populations.iter().enumerate() {
        // The lowest level faces a threshold above it only, the highest one
        // below it only, the levels between face both.
        let mut tails: Vec<Vec<f64>> = Vec::with_capacity(2);
        if index > 0 {
            tails.push(samples.clone());
        }
        if index < last {
            tails.push(samples.iter().rev().map(|v| -v).collect());
        }
        for inner_first in tails {
            match gaussian_tail_departure(&inner_first) {
                None => return BerEstimate::Unmeasured,
                Some(departure) if departure > GAUSSIAN_TAIL_LIMIT => {
                    return BerEstimate::NoRandomNoise;
                }
                Some(_) => {}
            }
        }
    }
    BerEstimate::FromQ(EyeMeasurements::estimate_ber_from_q(q))
}

/// Levels, openings, Q and the error rate of an eye of `levels` levels.
///
/// The levels are found by one-dimensional k-means on the samples around
/// the sampling phase, started at evenly spaced levels across the swing;
/// each opening's threshold is midway between its two levels' means.
fn measure_multilevel(data: &EyeData, phase: f64, levels: u8, m: &mut EyeMeasurements) {
    let mut samples = samples_near(data, phase, NOISE_PHASE_TOLERANCE_UI);
    samples.sort_by(f64::total_cmp);
    let count = usize::from(levels);
    if samples.len() < count || !(data.swing.is_finite() && data.swing > 0.0) {
        return;
    }
    let mut means: Vec<f64> = (0..count)
        .map(|level| data.v_low + data.swing * level as f64 / (count - 1) as f64)
        .collect();
    let thresholds_of = |means: &[f64]| -> Vec<f64> {
        means
            .windows(2)
            .map(|pair| 0.5 * (pair[0] + pair[1]))
            .collect()
    };
    let split = |sorted: &[f64], thresholds: &[f64]| -> Vec<std::ops::Range<usize>> {
        let mut bounds = vec![0usize];
        bounds.extend(
            thresholds
                .iter()
                .map(|&threshold| sorted.partition_point(|&v| v < threshold)),
        );
        bounds.push(sorted.len());
        bounds.windows(2).map(|pair| pair[0]..pair[1]).collect()
    };
    for _ in 0..LEVEL_ROUNDS {
        let ranges = split(&samples, &thresholds_of(&means));
        let next: Vec<f64> = ranges
            .iter()
            .zip(&means)
            .map(|(range, &held)| mean_of(&samples[range.clone()]).unwrap_or(held))
            .collect();
        let settled = next == means;
        means = next;
        if settled {
            break;
        }
    }

    let thresholds = thresholds_of(&means);
    let populations: Vec<Vec<f64>> = split(&samples, &thresholds)
        .into_iter()
        .map(|range| samples[range].to_vec())
        .collect();
    m.levels = populations
        .iter()
        .filter_map(|samples| level_of(samples))
        .collect();
    if m.levels.len() != count {
        m.levels.clear();
        return;
    }

    let mut narrow = samples_near(data, phase, HEIGHT_PHASE_TOLERANCE_UI);
    narrow.sort_by(f64::total_cmp);
    let slices = split(&narrow, &thresholds);
    m.openings = thresholds
        .iter()
        .enumerate()
        .map(|(index, &threshold)| {
            let below = &narrow[slices[index].clone()];
            let above = &narrow[slices[index + 1].clone()];
            let height = match (below.last(), above.first()) {
                (Some(&top), Some(&bottom)) => (bottom - top).max(0.0),
                _ => 0.0,
            };
            let (lower, upper) = (m.levels[index], m.levels[index + 1]);
            EyeOpening {
                threshold,
                height,
                width: calculate_horizontal_opening(data, threshold).width_fraction,
                q_factor: EyeMeasurements::calculate_q(
                    upper.mean,
                    lower.mean,
                    upper.sigma,
                    lower.sigma,
                ),
            }
        })
        .collect();

    let narrowest =
        |value: fn(&EyeOpening) -> f64| m.openings.iter().map(value).fold(f64::INFINITY, f64::min);
    m.eye_height = narrowest(|opening| opening.height);
    m.eye_width = narrowest(|opening| opening.width);
    m.vertical_margin = m.eye_height / 2.0;
    m.horizontal_margin = m.eye_width / 2.0;
    m.q_factor = m
        .openings
        .iter()
        .filter_map(|opening| opening.q_factor)
        .reduce(f64::min);
    m.amplitude = Some(m.levels[count - 1].mean - m.levels[0].mean);
    // The crossing level and S/N of a two-level eye name no one threshold
    // of a multi-level one.
    m.crossing_level = None;
    m.crossing_percentage = None;
    m.snr_db = None;
    m.ber = ber_estimate(m.q_factor, &populations);
}

/// The threshold the crossings are fitted at: the crossing threshold of a
/// two-level eye, the middle opening's (the lower middle one of three
/// levels) of a multi-level one.
fn jitter_threshold(m: &EyeMeasurements, data: &EyeData) -> f64 {
    if m.openings.len() < 2 {
        return data.v_cross;
    }
    m.openings[(m.openings.len() - 1) / 2].threshold
}

/// The bathtub of the crossings of `level`: each acquisition contributes
/// the crossings of its first unit interval, so every crossing of the
/// record is counted once over one unit interval per acquisition.
fn crossing_bathtub(data: &EyeData, level: f64) -> Bathtub {
    let mut phases = Vec::new();
    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            if let Some((time, _)) = interpolate_crossing(
                trace.time[i],
                trace.time[i + 1],
                trace.amplitude[i],
                trace.amplitude[i + 1],
                level,
            ) && (0.0..1.0).contains(&time)
            {
                phases.push(time);
            }
        }
    }
    bathtub(&phases, data.traces.len(), data.bit_period)
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
        assert_eq!(m.ber, BerEstimate::NoRandomNoise, "noiseless eye");
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

    /// Start time shared by every oracle in this module.
    const T_START: f64 = 0.137e-9;
    /// Peak displacement of the sinusoidal-jitter oracle's crossings.
    const JITTER_AMPLITUDE: f64 = 0.05 * UI;

    /// The shift that parks the crossings on the centre of the folded window
    /// when the fold is left unanchored.
    ///
    /// Every oracle starts at 0.137 UI and the settling skip is a whole
    /// number of unit intervals from t = 0, so an unanchored window opens at
    /// the skip and its crossings sit 0.137 UI in. Adding 0.85 UI moves them
    /// to 0.987 UI — 13 ps from the centre of a two-UI window, inside the
    /// edge's own ramp — which is precisely where the height, the noise
    /// statistics, the Q and every compliance mask are read. Anchored, it is
    /// one more record phase like any other; unanchored, it reads the eye off
    /// the transition.
    const UNANCHORED_CENTRE_SHIFT: f64 = 0.85 * UI;

    /// The headline invariant: where the record happens to start is not a
    /// property of the signal, so no measurement may depend on it.
    ///
    /// Reproducing the baseline is necessary but not sufficient — a metric
    /// broken to a constant reproduces its own constant perfectly — so each
    /// oracle is first pinned to the closed form it exists to check, and that
    /// pin is then required to hold at every record phase. Three oracles are
    /// needed rather than one: the ideal clock's opening is insensitive to
    /// where the crossings land in the window, while duty-cycle distortion
    /// pins the width against a number the fold cannot fake and sinusoidal
    /// jitter pins the peak-to-peak.
    #[test]
    fn eye_metrics_are_invariant_to_the_record_phase() {
        /// An ideal clock opens the whole unit interval, in both axes.
        fn assert_ideal_clock_closed_form(m: &EyeMeasurements) {
            assert!(m.eye_width >= 0.98, "eye width {} UI", m.eye_width);
            assert!(m.eye_height >= 0.98, "eye height {} V", m.eye_height);
        }

        /// 60/40 duty-cycle distortion puts the two crossing families 0.2 UI
        /// apart, so the eye is 0.80 UI wide and vertically wide open.
        fn assert_dcd_closed_form(m: &EyeMeasurements) {
            assert!(
                (m.eye_width - 0.80).abs() <= 0.02,
                "eye width {} UI, expected 0.80",
                m.eye_width
            );
            assert!(m.eye_height >= 0.98, "eye height {} V", m.eye_height);
        }

        /// Sinusoidal jitter of amplitude A shows 2A peak-to-peak and closes
        /// the eye by the same 2A.
        fn assert_sinusoidal_jitter_closed_form(m: &EyeMeasurements) {
            let expected_pp = 2.0 * JITTER_AMPLITUDE;
            assert!(
                (m.jitter_pp - expected_pp).abs() <= 0.1 * expected_pp,
                "jitter p-p {:e} s, expected {expected_pp:e}",
                m.jitter_pp
            );
            let expected_width = 1.0 - 2.0 * JITTER_AMPLITUDE / UI;
            assert!(
                (m.eye_width - expected_width).abs() <= 0.02,
                "eye width {} UI, expected {expected_width}",
                m.eye_width
            );
        }

        fn ideal_clock_at(t_start: f64) -> EyeData {
            ideal_clock_eye(200, t_start)
        }

        fn dcd_clock_at(t_start: f64) -> EyeData {
            dcd_clock_eye(60, t_start)
        }

        fn jittered_clock_at(t_start: f64) -> EyeData {
            jittered_clock_eye(400, t_start, JITTER_AMPLITUDE, 3.7e6)
        }

        /// Every figure that must survive a change of record phase, with the
        /// relative budget it has to reproduce within. `None` is itself a
        /// claim — a crossing that splits has no single level — so it has to
        /// reproduce as exactly as a number does.
        ///
        /// A part in a thousand for everything the fold pins. `jitter_rms` is
        /// the one exception, and the reason is the record's ends rather than
        /// its phase: the recovered period carries a residual of a few parts
        /// in 100 000, which walks each polarity's crossings a few parts in a
        /// thousand of a unit interval across the record, and whether one
        /// polarity's walk reaches the cycle at the boundary depends on where
        /// the first anchored window opens. One cycle in 116 is 0.9 % of that
        /// family's spread, and the pooled RMS moves with it. Two per cent
        /// leaves that alone while still catching an unanchored fold, which
        /// moves the same figure by orders of magnitude.
        fn phase_invariant_metrics(m: &EyeMeasurements) -> [(&'static str, Option<f64>, f64); 11] {
            [
                ("unit_interval", Some(m.unit_interval), 1e-3),
                ("eye_width", Some(m.eye_width), 1e-3),
                ("eye_height", Some(m.eye_height), 1e-3),
                ("eye_area", Some(m.eye_area), 1e-3),
                ("crossing_percentage", m.crossing_percentage, 1e-3),
                ("rise_time", m.rise_time, 1e-3),
                ("fall_time", m.fall_time, 1e-3),
                ("jitter_pp", Some(m.jitter_pp), 1e-3),
                ("jitter_rms", Some(m.jitter_rms), 2e-2),
                ("q_factor", m.q_factor, 1e-3),
                ("snr_db", m.snr_db, 1e-3),
            ]
        }

        /// A noiseless eye's Q and SNR are unbounded, which is an answer and
        /// not a number to subtract; it has to reproduce as itself. Anything
        /// finite compares relatively.
        fn assert_reproduces(
            name: &str,
            expected: Option<f64>,
            actual: Option<f64>,
            budget: f64,
            context: &str,
        ) {
            match (expected, actual) {
                (None, None) => {}
                (Some(expected), Some(actual))
                    if expected.is_infinite() || actual.is_infinite() =>
                {
                    assert_eq!(expected, actual, "{name} moved {context}");
                }
                (Some(expected), Some(actual)) => {
                    let scale = expected.abs().max(1e-12);
                    assert!(
                        (actual - expected).abs() <= budget * scale,
                        "{name} moved from {expected} to {actual} {context}"
                    );
                }
                _ => panic!("{name} moved from {expected:?} to {actual:?} {context}"),
            }
        }

        type Oracle = (&'static str, fn(f64) -> EyeData, fn(&EyeMeasurements));
        let oracles: [Oracle; 3] = [
            (
                "ideal clock",
                ideal_clock_at,
                assert_ideal_clock_closed_form,
            ),
            (
                "60/40 duty-cycle distortion",
                dcd_clock_at,
                assert_dcd_closed_form,
            ),
            (
                "sinusoidal jitter",
                jittered_clock_at,
                assert_sinusoidal_jitter_closed_form,
            ),
        ];

        for (oracle, build, assert_closed_form) in oracles {
            let baseline = calculate_eye_measurements(&build(T_START));
            assert_closed_form(&baseline);

            for shift in [0.37 * UI, 0.5 * UI, UNANCHORED_CENTRE_SHIFT] {
                let m = calculate_eye_measurements(&build(T_START + shift));
                let context = format!("on the {oracle} oracle under a {shift:e} s shift");
                assert_closed_form(&m);
                for ((name, expected, budget), (_, actual, _)) in phase_invariant_metrics(&baseline)
                    .into_iter()
                    .zip(phase_invariant_metrics(&m))
                {
                    assert_reproduces(name, expected, actual, budget, &context);
                }
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

    /// Q to bit-error rate stays on the complementary error function past
    /// Q = 8, where the one-term asymptote it switched to read 1.5 % high
    /// and jumped. The rates are scipy's `norm.sf`.
    #[test]
    fn the_error_rate_of_a_q_stays_on_erfc_past_eight() {
        for (q, expected) in [
            (7.9, 1.394_517_146_659_261e-15),
            (8.0, 6.220_960_574_271_74e-16),
            (8.01, 5.735_422_180_258_049e-16),
            (10.0, 7.619_853_024_160_47e-24),
        ] {
            let ber = EyeMeasurements::estimate_ber_from_q(q);
            assert!(
                ((ber - expected) / expected).abs() < 1e-12,
                "Q = {q}: {ber:e}, not {expected:e}"
            );
        }
    }

    /// PRBS7 through a first-order lag of time constant `tau_ui` unit
    /// intervals, 100 samples a bit, with Gaussian noise of `noise` volts rms,
    /// folded at the bit period it was generated at.
    fn lagged_prbs7_eye(tau_ui: f64, noise: f64) -> EyeData {
        let bits = super::super::eye_test_signals::prbs_bits(7, (7, 6), 600);
        let dt = UI / 100.0;
        let decay = (-(dt / UI) / tau_ui).exp();
        let mut draw = super::super::eye_test_signals::gaussian_draws(0x0e1e_7e5e);
        let mut held = 0.0;
        let mut time = Vec::with_capacity(bits.len() * 100);
        let mut signal = Vec::with_capacity(bits.len() * 100);
        for (index, &bit) in bits.iter().enumerate() {
            for step in 0..100 {
                held = decay * held + (1.0 - decay) * if bit { 1.0 } else { 0.0 };
                time.push((index * 100 + step) as f64 * dt);
                signal.push(held + noise * draw());
            }
        }
        let phase =
            crate::analysis::signal_integrity::unit_interval::crossing_phase_at(&time, &signal, UI)
                .expect("the lagged record crosses");
        EyeDataBuilder::new()
            .bit_period(UI)
            .ui_count(2)
            .skip_initial(4)
            .fold_anchor(fold_anchor(phase.phase, UI))
            .build(&time, &signal)
    }

    /// With no noise source the levels at the eye centre are spread by
    /// intersymbol interference alone. The audit's eye, open by 0.74 V, read
    /// an estimated error rate of 1.4e-7; it has no random noise to estimate
    /// one from and says so, while its Q, conventionally defined, stays.
    #[test]
    fn a_noise_free_open_eye_reports_no_estimated_error_rate() {
        let m = calculate_eye_measurements(&lagged_prbs7_eye(0.45, 0.0));
        assert!(m.eye_height > 0.5, "eye height {} V", m.eye_height);
        let q = m.q_factor.expect("the levels are measured");
        assert!(q.is_finite() && q > 1.0, "Q {q}");
        assert_eq!(m.ber, BerEstimate::NoRandomNoise);
    }

    /// The same eye with 20 mV of random noise has an error rate to estimate,
    /// the Gaussian tail of its Q.
    #[test]
    fn a_noisy_eye_estimates_its_error_rate_from_its_q() {
        let m = calculate_eye_measurements(&lagged_prbs7_eye(0.45, 0.02));
        let q = m.q_factor.expect("the levels are measured");
        assert_eq!(m.ber, BerEstimate::FromQ(q_function(q)));
    }

    /// Four levels at 0, 0.3, 0.7 and 1 V, symbols drawn from a seeded
    /// generator: three openings, as tall as the gaps between the levels.
    #[test]
    fn a_four_level_eye_reports_three_openings_at_the_generated_heights() {
        let levels = [0.0, 0.3, 0.7, 1.0];
        let mut draw = super::super::eye_test_signals::uniform_draws(0x4a_4a4a);
        let symbols: Vec<f64> = (0..1_500)
            .map(|_| levels[((draw() * 4.0) as usize).min(3)])
            .collect();
        let t_start = 0.137e-9;
        let events: Vec<LevelEvent> = symbols
            .windows(2)
            .enumerate()
            .filter(|(_, pair)| pair[0] != pair[1])
            .map(|(index, pair)| (t_start + (index + 1) as f64 * UI, pair[1]))
            .collect();
        let (time, signal) = trapezoid(
            symbols[0],
            &events,
            t_start + (symbols.len() as f64 + 1.0) * UI,
            DT,
            RISE_2080,
        );
        let phase =
            crate::analysis::signal_integrity::unit_interval::crossing_phase_at(&time, &signal, UI)
                .expect("the record crosses mid-swing");
        let data = EyeDataBuilder::new()
            .bit_period(UI)
            .ui_count(2)
            .skip_initial(2)
            .fold_anchor(fold_anchor(phase.phase, UI))
            .build(&time, &signal);
        let m = calculate_eye_measurements_with(
            &data,
            &EyeMeasureOptions {
                levels: 4,
                ..EyeMeasureOptions::default()
            },
        );

        assert_eq!(m.levels.len(), 4);
        for (level, expected) in m.levels.iter().zip(levels) {
            assert!((level.mean - expected).abs() < 1e-9, "level {level:?}");
        }
        assert_eq!(m.openings.len(), 3);
        for (opening, expected) in m.openings.iter().zip([0.3, 0.4, 0.3]) {
            assert!(
                (opening.height - expected).abs() < 1e-9,
                "opening {opening:?}, not {expected} V tall"
            );
            assert!(opening.width > 0.8, "opening {opening:?}");
        }
        assert!((m.eye_height - 0.3).abs() < 1e-9);
        assert!(
            m.amplitude
                .is_some_and(|amplitude| (amplitude - 1.0).abs() < 1e-9)
        );
    }

    /// A clock whose edges carry 0.05 UI of sinusoidal jitter: the fit finds
    /// it deterministic, a dual-Dirac figure just inside its 0.1 UI
    /// peak-to-peak with a sliver of random jitter.
    ///
    /// The sinusoid runs some sixty periods over the record. One of a few
    /// periods is partly a slow wander, which the recovered timebase follows
    /// as a clock recovery would, and the eye then shows less of it.
    #[test]
    fn an_eye_with_sinusoidal_jitter_separates_it_as_deterministic() {
        let data = jittered_clock_eye(1_000, 0.137e-9, 0.05 * UI, 61.7e6);
        let m = calculate_eye_measurements(&data);
        let fit = m.bathtub.fit.expect("a thousand crossings are fitted");
        let dj = fit.deterministic() / UI;
        let rj = fit.random_rms() / UI;
        assert!((0.085..=0.1).contains(&dj), "DJ {dj} UI");
        assert!(rj <= 0.1 * dj, "RJ {rj} UI beside DJ {dj} UI");
        // A clock crosses at every boundary.
        assert!((fit.transition_density - 1.0).abs() < 0.01);
    }

    /// A clock whose edges carry 0.02 UI rms of Gaussian jitter: the fit finds
    /// it random, its sigma within 15 % over half the crossings the fit's
    /// 10 % is quoted at.
    #[test]
    fn an_eye_with_gaussian_jitter_separates_it_as_random() {
        let mut draw = super::super::eye_test_signals::gaussian_draws(0x00ab_cdef);
        let jitter: Vec<f64> = (0..2_000).map(|_| 0.02 * UI * draw()).collect();
        let t_start = 0.137e-9;
        let (initial, events) = clock_events(2_000, t_start, UI, 0.0, 1.0, |n| jitter[n]);
        let data = eye_of(initial, &events, t_start + 2_001.0 * UI);
        let m = calculate_eye_measurements(&data);
        let fit = m.bathtub.fit.expect("two thousand crossings are fitted");
        let rj = fit.random_rms() / UI;
        let dj = fit.deterministic() / UI;
        assert!((rj - 0.02).abs() <= 0.15 * 0.02, "RJ {rj} UI");
        assert!(dj <= 0.5 * rj, "DJ {dj} UI beside RJ {rj} UI");
    }
}
