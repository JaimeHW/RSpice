//! Unit-interval (bit period) recovery from a sampled waveform.
//!
//! The period an eye folds at is the number every other eye measurement is
//! quoted against. Fold at the wrong period and height, width, jitter, the
//! mask verdict and the reported data rate are all wrong together — and none
//! of them look wrong, because a wrongly folded eye is still a picture of an
//! eye.
//!
//! The obvious construction — the median gap between consecutive *rising*
//! edges — measures the pattern's repetition period, not its bit period. On a
//! 1010 clock every rising gap is two unit intervals, so it reports 2 UI and
//! the viewer folds two bits into one window. The estimator here reads
//! crossings of both polarities, clusters the intervals between them, and
//! resolves which small multiple of the true unit interval each cluster is.
//!
//! # What it refuses
//!
//! Data with no swing, too few transitions, or no single period consistent
//! with the observed intervals is rejected rather than folded at a guess:
//! [`UiEstimateRejection`] is what the viewer surfaces so the reader can set
//! the rate instead of reading a fabricated eye. Data whose transitions are
//! all commensurate with a longer interval (a pure 1010 clock is not, but a
//! pattern with a minimum run length of two is) estimates the greatest common
//! period it can see; that ambiguity is irreducible from the waveform alone,
//! which is why every commercial eye tool takes the data rate as an input and
//! why the estimate is always displayed with its provenance.

use super::eye_data::find_edges;
use std::f64::consts::TAU;

/// Relative gap that separates two interval clusters: a jump of more than
/// this fraction of the running cluster median starts a new cluster.
const REL_CLUSTER_GAP: f64 = 0.25;
/// Largest residual, in unit intervals, that still counts as a cluster
/// sitting on an integer multiple of the candidate period.
const RESIDUAL_TOL_UI: f64 = 0.25;
/// Deepest subdivision of the smallest cluster considered as a candidate.
const MAX_SUBDIVISION: u32 = 4;
/// Fewest crossings that can produce an estimate at all.
const MIN_CROSSINGS: usize = 4;
/// Below this many crossings the estimate is labelled low-confidence.
const LOW_CONFIDENCE_BELOW: usize = 8;
/// Iterations of the weighted least-squares refit.
const MAX_REFIT_ITERS: usize = 3;
/// Relative slack on the pair-mean ratio gate. Duty-cycle distortion of
/// exactly 60/40 puts the two clusters at a ratio of exactly 1.5, so the gate
/// has to be inclusive of its own boundary in floating point.
const PAIR_RATIO_EPS: f64 = 1e-6;
/// Resultant length below which the crossings do not agree on a phase at the
/// period they were folded against. A record whose crossings all sit at one
/// phase scores 1; crossings spread evenly over the period average to 0. Half
/// is far from both: an ideal clock at its own rate scores above 0.99, and
/// the same clock at a rate 37 % away scores below 0.2.
const MIN_CROSSING_COHERENCE: f64 = 0.5;

/// A recovered unit interval and what it was recovered from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiEstimate {
    /// Recovered unit interval in seconds.
    pub unit_interval: f64,
    /// Threshold crossings the estimate was built from.
    pub crossing_count: usize,
    /// RMS residual of the crossings against the fitted grid, in unit
    /// intervals.
    pub rms_residual_ui: f64,
    /// Too few crossings to be trusted without the reader's confirmation.
    pub low_confidence: bool,
    /// Mean crossing time modulo the unit interval, in seconds in `[0, T)`.
    pub mean_crossing_phase: f64,
}

/// Why no unit interval could be recovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEstimateRejection {
    /// The record has no finite amplitude swing to threshold.
    NoSignalSwing,
    /// Fewer threshold crossings than an interval fit needs.
    TooFewTransitions { crossings: usize },
    /// The intervals between crossings are not multiples of any one period.
    NoConsistentFundamental { clusters: usize },
}

/// Mean crossing phase of a record at a period the caller already knows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossingPhaseFit {
    /// Circular mean crossing time modulo the period, in seconds in `[0, T)`.
    pub phase: f64,
    /// Resultant length of the circular mean, in `[0, 1]`. Near 1 the
    /// crossings agree on a phase; near 0 they are spread over the period and
    /// the requested rate does not describe the data.
    pub coherence: f64,
    /// Crossings the fit was built from.
    pub crossing_count: usize,
}

impl CrossingPhaseFit {
    /// Do the crossings agree on a phase at this period?
    ///
    /// The answer does not decide whether to fold — a rate the reader stated
    /// is the rate the eye folds at, because they asked for it. It decides
    /// what the viewer must say about the result: an eye folded at a period
    /// the crossings do not share is a picture of the fold, and a reader who
    /// is not told that will read jitter, width and mask margin off it.
    pub fn is_coherent(&self) -> bool {
        self.coherence >= MIN_CROSSING_COHERENCE
    }
}

/// Recover the unit interval of a sampled digital waveform.
pub fn estimate_unit_interval(
    time: &[f64],
    signal: &[f64],
) -> Result<UiEstimate, UiEstimateRejection> {
    let crossings = threshold_crossings(time, signal).ok_or(UiEstimateRejection::NoSignalSwing)?;
    if crossings.len() < MIN_CROSSINGS {
        return Err(UiEstimateRejection::TooFewTransitions {
            crossings: crossings.len(),
        });
    }

    let mut intervals: Vec<f64> = crossings
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|gap| gap.is_finite() && *gap > 0.0)
        .collect();
    if intervals.is_empty() {
        return Err(UiEstimateRejection::NoConsistentFundamental { clusters: 0 });
    }
    intervals.sort_by(f64::total_cmp);

    let clusters = cluster_intervals(&intervals);
    let rejected = UiEstimateRejection::NoConsistentFundamental {
        clusters: clusters.len(),
    };
    let base = candidate_periods(&clusters)
        .into_iter()
        .find(|period| is_consistent(&clusters, *period))
        .ok_or(rejected)?;

    let refined = refit_period(&clusters, base);
    let fit = grid_fit(&crossings, refined).ok_or(rejected)?;
    if fit.rms_residual_ui > RESIDUAL_TOL_UI {
        return Err(rejected);
    }

    Ok(UiEstimate {
        unit_interval: fit.period,
        crossing_count: crossings.len(),
        rms_residual_ui: fit.rms_residual_ui,
        low_confidence: crossings.len() < LOW_CONFIDENCE_BELOW,
        mean_crossing_phase: fit.phase,
    })
}

/// Circular mean crossing phase at a caller-supplied period.
///
/// This is the explicit-rate counterpart of the estimator's own grid fit: the
/// reader has stated the data rate, so there is nothing to recover except
/// where in the period the crossings sit.
pub fn crossing_phase_at(
    time: &[f64],
    signal: &[f64],
    unit_interval: f64,
) -> Option<CrossingPhaseFit> {
    if !(unit_interval.is_finite() && unit_interval > 0.0) {
        return None;
    }
    let crossings = threshold_crossings(time, signal)?;
    if crossings.is_empty() {
        return None;
    }

    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    for &crossing in &crossings {
        let angle = TAU * (crossing / unit_interval).rem_euclid(1.0);
        cos_sum += angle.cos();
        sin_sum += angle.sin();
    }
    let count = crossings.len() as f64;
    let (cos_mean, sin_mean) = (cos_sum / count, sin_sum / count);
    let phase = (unit_interval / TAU) * sin_mean.atan2(cos_mean);

    Some(CrossingPhaseFit {
        phase: phase.rem_euclid(unit_interval),
        coherence: cos_mean.hypot(sin_mean),
        crossing_count: crossings.len(),
    })
}

/// Window start offset that puts the mean crossing at a half-integer phase.
///
/// The eye opening — not the crossing — belongs at the centre of the folded
/// window: that is where the height, the noise statistics and every
/// compliance mask are measured. Anchoring the window half a unit interval
/// before the mean crossing puts crossings at 0.5 and 1.5 of a two-UI window
/// and the opening at 1.0.
pub fn fold_anchor(mean_crossing_phase: f64, unit_interval: f64) -> Option<f64> {
    if !(unit_interval.is_finite() && unit_interval > 0.0) || !mean_crossing_phase.is_finite() {
        return None;
    }
    Some((mean_crossing_phase - 0.5 * unit_interval).rem_euclid(unit_interval))
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

/// Mid-swing crossings of both polarities, ascending. `None` when the record
/// has no finite swing to threshold.
fn threshold_crossings(time: &[f64], signal: &[f64]) -> Option<Vec<f64>> {
    let n = time.len().min(signal.len());
    let mut v_min = f64::INFINITY;
    let mut v_max = f64::NEG_INFINITY;
    for &value in signal.iter().take(n) {
        if value.is_finite() {
            v_min = v_min.min(value);
            v_max = v_max.max(value);
        }
    }
    if !v_min.is_finite() || !v_max.is_finite() || v_max - v_min <= 0.0 {
        return None;
    }

    let threshold = 0.5 * (v_min + v_max);
    Some(
        find_edges(&time[..n], &signal[..n], threshold)
            .into_iter()
            .map(|edge| edge.time)
            .filter(|t| t.is_finite())
            .collect(),
    )
}

#[derive(Debug, Clone, Copy)]
struct IntervalCluster {
    centroid: f64,
    weight: f64,
}

/// Single-linkage clustering of sorted intervals with a relative gap rule.
fn cluster_intervals(sorted: &[f64]) -> Vec<IntervalCluster> {
    let mut clusters = Vec::new();
    let mut start = 0usize;
    for index in 1..sorted.len() {
        let current = &sorted[start..index];
        let gap = sorted[index] - sorted[index - 1];
        if gap > REL_CLUSTER_GAP * sorted_median(current) {
            clusters.push(cluster_of(current));
            start = index;
        }
    }
    if start < sorted.len() {
        clusters.push(cluster_of(&sorted[start..]));
    }
    clusters
}

fn cluster_of(values: &[f64]) -> IntervalCluster {
    IntervalCluster {
        centroid: sorted_median(values),
        weight: values.len() as f64,
    }
}

fn sorted_median(sorted: &[f64]) -> f64 {
    match sorted.len() {
        0 => 0.0,
        len if len.is_multiple_of(2) => 0.5 * (sorted[len / 2 - 1] + sorted[len / 2]),
        len => sorted[len / 2],
    }
}

/// Candidate periods, largest first.
///
/// Two families are needed and neither suffices alone. The smallest cluster
/// divided by a small integer covers patterns whose shortest run is longer
/// than one bit. The weighted mean of the two smallest clusters covers
/// duty-cycle distortion, where the true period is the *average* of a short
/// and a long interval and the smallest cluster alone is a trap — a 60/40
/// clock's 0.4 UI cluster would otherwise be taken as the period.
fn candidate_periods(clusters: &[IntervalCluster]) -> Vec<f64> {
    let Some(smallest) = clusters.first() else {
        return Vec::new();
    };

    let mut candidates = Vec::with_capacity(MAX_SUBDIVISION as usize + 1);
    if let Some(second) = clusters.get(1)
        && second.centroid <= 1.5 * smallest.centroid * (1.0 + PAIR_RATIO_EPS)
    {
        let weight = smallest.weight + second.weight;
        if weight > 0.0 {
            candidates.push(
                (smallest.weight * smallest.centroid + second.weight * second.centroid) / weight,
            );
        }
    }
    for divisor in 1..=MAX_SUBDIVISION {
        candidates.push(smallest.centroid / f64::from(divisor));
    }

    candidates.retain(|period| period.is_finite() && *period > 0.0);
    candidates.sort_by(|a, b| b.total_cmp(a));
    candidates.dedup_by(|a, b| (*a - *b).abs() <= 1e-12 * b.abs());
    candidates
}

/// Every cluster sits within tolerance of an integer multiple of `period`.
fn is_consistent(clusters: &[IntervalCluster], period: f64) -> bool {
    clusters.iter().all(|cluster| {
        let multiple = (cluster.centroid / period).round();
        multiple >= 1.0 && (cluster.centroid - multiple * period).abs() <= RESIDUAL_TOL_UI * period
    })
}

/// Weighted least-squares refit through the origin over the resolved cluster
/// multiples, so a period picked from one cluster is corrected by all of them.
fn refit_period(clusters: &[IntervalCluster], mut period: f64) -> f64 {
    let mut previous: Option<Vec<f64>> = None;
    for _ in 0..MAX_REFIT_ITERS {
        let multiples: Vec<f64> = clusters
            .iter()
            .map(|cluster| (cluster.centroid / period).round().max(1.0))
            .collect();
        if previous.as_deref() == Some(multiples.as_slice()) {
            break;
        }
        let numerator: f64 = clusters
            .iter()
            .zip(&multiples)
            .map(|(cluster, &k)| cluster.weight * k * cluster.centroid)
            .sum();
        let denominator: f64 = clusters
            .iter()
            .zip(&multiples)
            .map(|(cluster, &k)| cluster.weight * k * k)
            .sum();
        if denominator <= 0.0 {
            break;
        }
        let next = numerator / denominator;
        if !(next.is_finite() && next > 0.0) {
            break;
        }
        period = next;
        previous = Some(multiples);
    }
    period
}

struct GridFit {
    period: f64,
    phase: f64,
    rms_residual_ui: f64,
}

/// Drift-free fit of the absolute crossing times to a uniform grid.
///
/// Assigning each crossing an integer grid index and regressing time against
/// index uses the whole record's span, so the period does not inherit the
/// accumulated error of a chain of local differences.
fn grid_fit(crossings: &[f64], period: f64) -> Option<GridFit> {
    if crossings.len() < 2 || !(period.is_finite() && period > 0.0) {
        return None;
    }

    let mut indices = Vec::with_capacity(crossings.len());
    indices.push(0.0f64);
    for pair in crossings.windows(2) {
        let step = ((pair[1] - pair[0]) / period).round();
        let previous = indices[indices.len() - 1];
        indices.push(previous + step);
    }

    let count = crossings.len() as f64;
    let index_mean = indices.iter().sum::<f64>() / count;
    let time_mean = crossings.iter().sum::<f64>() / count;
    let mut covariance = 0.0;
    let mut variance = 0.0;
    for (index, crossing) in indices.iter().zip(crossings) {
        covariance += (index - index_mean) * (crossing - time_mean);
        variance += (index - index_mean).powi(2);
    }
    if variance <= 0.0 {
        return None;
    }
    let fitted = covariance / variance;
    if !(fitted.is_finite() && fitted > 0.0) {
        return None;
    }

    let origin = time_mean - fitted * index_mean;
    let residual_sum: f64 = indices
        .iter()
        .zip(crossings)
        .map(|(index, crossing)| (crossing - (origin + fitted * index)).powi(2))
        .sum();

    Some(GridFit {
        period: fitted,
        phase: origin.rem_euclid(fitted),
        rms_residual_ui: (residual_sum / count).sqrt() / fitted,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::signal_integrity::eye_test_signals::{
        DT, RISE_2080, UI, beating_toggle_events, clock_events, dcd_clock_events, nrz_events,
        prbs7_bits, trapezoid,
    };

    /// The shipping estimator this module replaces: the median gap between
    /// consecutive *rising* edges. Kept as a test fixture so the defect it
    /// carries stays pinned rather than remembered.
    fn legacy_rising_median_ui(time: &[f64], signal: &[f64]) -> Option<f64> {
        let n = time.len().min(signal.len());
        let mut v_min = f64::INFINITY;
        let mut v_max = f64::NEG_INFINITY;
        for &value in signal.iter().take(n) {
            if value.is_finite() {
                v_min = v_min.min(value);
                v_max = v_max.max(value);
            }
        }
        let threshold = 0.5 * (v_min + v_max);
        let edges = find_edges(&time[..n], &signal[..n], threshold);
        let mut rising: Vec<f64> = edges
            .iter()
            .filter(|edge| edge.rising)
            .map(|edge| edge.time)
            .collect();
        rising.sort_by(f64::total_cmp);
        if rising.len() < 3 {
            return None;
        }
        let mut gaps: Vec<f64> = rising.windows(2).map(|pair| pair[1] - pair[0]).collect();
        gaps.sort_by(f64::total_cmp);
        Some(gaps[gaps.len() / 2])
    }

    fn ideal_clock(bits: usize, t_start: f64) -> (Vec<f64>, Vec<f64>) {
        let (initial, events) = clock_events(bits, t_start, UI, 0.0, 1.0, |_| 0.0);
        trapezoid(
            initial,
            &events,
            t_start + (bits as f64 + 1.0) * UI,
            DT,
            RISE_2080,
        )
    }

    /// A 1010 clock is exactly the case the rising-edge median cannot see:
    /// every rising gap is two bits wide, so it reports twice the unit
    /// interval and the viewer folds two bits into one eye.
    #[test]
    fn ideal_clock_recovers_one_unit_interval_where_rising_median_reports_two() {
        let (time, signal) = ideal_clock(40, 0.137e-9);

        let estimate = estimate_unit_interval(&time, &signal).expect("clock has a bit period");
        assert!(
            (estimate.unit_interval - UI).abs() <= 1e-3 * UI,
            "unit interval {:e} is not 1 ns",
            estimate.unit_interval
        );
        assert!(!estimate.low_confidence);
        assert_eq!(estimate.crossing_count, 40);
        assert!(estimate.rms_residual_ui < 1e-6);

        let legacy = legacy_rising_median_ui(&time, &signal).expect("legacy estimate");
        assert!(
            (legacy - 2.0 * UI).abs() <= 1e-3 * UI,
            "legacy rising-edge median returned {legacy:e}, expected 2 ns"
        );
    }

    /// The mean crossing phase is the crossing's own position in the period,
    /// so the anchor derived from it lands the crossing at half a UI.
    #[test]
    fn ideal_clock_mean_phase_tracks_the_pattern_start() {
        let t_start = 0.137e-9;
        let (time, signal) = ideal_clock(40, t_start);
        let estimate = estimate_unit_interval(&time, &signal).expect("clock has a bit period");

        let expected = t_start.rem_euclid(estimate.unit_interval);
        let error = (estimate.mean_crossing_phase - expected).abs();
        assert!(
            error <= 1e-3 * UI,
            "phase {error:e} s from the pattern start"
        );

        let anchor = fold_anchor(estimate.mean_crossing_phase, estimate.unit_interval)
            .expect("anchor for a positive period");
        let crossing_phase = ((t_start - anchor) / estimate.unit_interval).rem_euclid(1.0);
        assert!(
            (crossing_phase - 0.5).abs() < 1e-6,
            "crossings land at {crossing_phase} UI, expected 0.5"
        );
    }

    #[test]
    fn prbs7_recovers_the_bit_period_from_run_lengths() {
        let bits = prbs7_bits(300);
        let (initial, events) = nrz_events(&bits, 0.137e-9, UI, 0.0, 1.0);
        let (time, signal) = trapezoid(initial, &events, 302.0 * UI, DT, RISE_2080);

        let estimate = estimate_unit_interval(&time, &signal).expect("PRBS has a bit period");
        assert!(
            (estimate.unit_interval - UI).abs() <= 2e-3 * UI,
            "unit interval {:e} is not 1 ns",
            estimate.unit_interval
        );
        assert!(!estimate.low_confidence);
    }

    /// 60/40 duty-cycle distortion puts the interval clusters at 0.8 and
    /// 1.2 UI. The smallest cluster is a trap; the pair mean is the period.
    #[test]
    fn duty_cycle_distorted_clock_recovers_the_average_period() {
        let (initial, events) = dcd_clock_events(60, 0.137e-9, UI, 0.6, 0.0, 1.0);
        let (time, signal) = trapezoid(initial, &events, 124.0 * UI, DT, RISE_2080);

        let estimate = estimate_unit_interval(&time, &signal).expect("DCD clock has a bit period");
        assert!(
            (estimate.unit_interval - UI).abs() <= 5e-3 * UI,
            "unit interval {:e} is not 1 ns",
            estimate.unit_interval
        );
        assert!(estimate.rms_residual_ui <= RESIDUAL_TOL_UI);
    }

    #[test]
    fn sinusoidal_jitter_leaves_the_period_intact() {
        let amplitude = 0.05 * UI;
        let jitter_frequency = 3.7e6;
        let (initial, events) = clock_events(1000, 0.137e-9, UI, 0.0, 1.0, |n| {
            amplitude * (TAU * jitter_frequency * n as f64 * UI).sin()
        });
        let (time, signal) = trapezoid(initial, &events, 1002.0 * UI, DT, RISE_2080);

        let estimate = estimate_unit_interval(&time, &signal).expect("jittered clock has a period");
        assert!(
            (estimate.unit_interval - UI).abs() <= 2e-3 * UI,
            "unit interval {:e} is not 1 ns",
            estimate.unit_interval
        );
    }

    #[test]
    fn direct_current_is_rejected_for_having_no_swing() {
        let time: Vec<f64> = (0..1000).map(|i| i as f64 * DT).collect();
        let signal = vec![0.5; time.len()];
        assert_eq!(
            estimate_unit_interval(&time, &signal),
            Err(UiEstimateRejection::NoSignalSwing)
        );
    }

    #[test]
    fn three_crossings_are_too_few_to_fit() {
        let (time, signal) = ideal_clock(3, 0.137e-9);
        assert_eq!(
            estimate_unit_interval(&time, &signal),
            Err(UiEstimateRejection::TooFewTransitions { crossings: 3 })
        );
    }

    #[test]
    fn five_crossings_estimate_but_are_labelled_low_confidence() {
        let (time, signal) = ideal_clock(5, 0.137e-9);
        let estimate = estimate_unit_interval(&time, &signal).expect("five crossings still fit");
        assert_eq!(estimate.crossing_count, 5);
        assert!(estimate.low_confidence);
        assert!((estimate.unit_interval - UI).abs() <= 1e-3 * UI);
    }

    /// Two incommensurate clocks beating against each other have no bit
    /// period. Reporting one would fold an eye out of an accident.
    #[test]
    fn incommensurate_clocks_are_rejected_rather_than_folded() {
        let (initial, events) = beating_toggle_events(1.0e-9, 1.618e-9, 200.0 * UI, 0.0, 1.0);
        let (time, signal) = trapezoid(initial, &events, 200.0 * UI, DT, RISE_2080);

        assert!(
            matches!(
                estimate_unit_interval(&time, &signal),
                Err(UiEstimateRejection::NoConsistentFundamental { .. })
            ),
            "beating clocks produced {:?}",
            estimate_unit_interval(&time, &signal)
        );
    }

    /// The explicit-rate path reads the same crossings and reports how well
    /// they agree on a phase, so a rate the data does not have is visible.
    #[test]
    fn explicit_rate_phase_is_coherent_only_at_the_real_period() {
        let (time, signal) = ideal_clock(200, 0.137e-9);

        let matched = crossing_phase_at(&time, &signal, UI).expect("crossings exist");
        assert!(matched.coherence > 0.99, "coherence {}", matched.coherence);
        assert!(matched.is_coherent());
        assert!((matched.phase - 0.137e-9).abs() <= 1e-3 * UI);

        let mismatched = crossing_phase_at(&time, &signal, 1.37e-9).expect("crossings exist");
        assert!(
            mismatched.coherence < 0.5,
            "coherence {} at a wrong rate",
            mismatched.coherence
        );
        assert!(!mismatched.is_coherent());
    }
}
