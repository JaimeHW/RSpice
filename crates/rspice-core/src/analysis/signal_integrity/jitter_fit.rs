//! Random and deterministic jitter, separated by a dual-Dirac tail fit, and
//! the bathtub curve it extrapolates.
//!
//! A crossing histogram is the deterministic jitter's bounded spread smeared
//! by Gaussian random jitter. Only the tails say which is which: far enough
//! out, each tail is the Gaussian of the outermost deterministic position,
//! so on a Q scale — the tail probability mapped through the inverse normal
//! tail — it is a straight line whose slope is the random jitter's sigma
//! and whose zero is that position. The two zeros are the dual-Dirac
//! deterministic jitter; the slopes are the random jitter; and the same two
//! lines extrapolate the bit-error rate across the unit interval into the
//! bathtub, past what any simulated record can count.
//!
//! The weight of each tail's Gaussian is fitted with it, one weight shared
//! by both tails: a purely random record straightens at weight one with its
//! zeros together, a two-position one at a half with its zeros a
//! deterministic jitter apart. A record with too few crossings to place a
//! tail is not fitted, and its bathtub keeps the points it measured.
//!
//! The fit's tolerance, over four thousand crossings: a purely random
//! jitter's sigma within 10 % with a deterministic figure under half of it;
//! a sinusoid's deterministic figure within 15 % under its peak-to-peak with
//! a random part under a tenth of it; a two-position jitter's deterministic
//! figure within 15 % and its random sigma within 20 %. How the record's
//! spread divides between the two moves more than the extrapolated opening
//! does, which lands within 4 % of the generator's.
//!
//! Every figure here is in unit intervals except where a field says
//! seconds.

use super::normal::{inverse_q_function, q_function};

/// Share of the crossings each tail fit reads, from the outermost in.
const TAIL_SHARE: f64 = 0.1;
/// The error rate total jitter and the eye's opening are read at unless
/// the reader asks for another: the rate serial links are specified at.
pub const DEFAULT_OPENING_BER: f64 = 1e-12;
/// Fewest crossings the fit extrapolates from: forty to a tail.
pub const MIN_FIT_CROSSINGS: usize = 400;
/// Least weight a tail's Gaussian is fitted at, and the steps to one.
const LEAST_TAIL_WEIGHT: f64 = 0.25;
const TAIL_WEIGHT_STEPS: usize = 61;
/// How much worse than the straightest a heavier tail weight's fit may be
/// and still be preferred. Over a few thousand crossings the residual of a
/// purely random record wanders by more than this between weights, which
/// left free would read a part of its random jitter as deterministic.
const WEIGHT_PARSIMONY: f64 = 0.1;
/// Sampling positions across the unit interval the measured bathtub is
/// counted at.
const MEASURED_POSITIONS: usize = 101;

/// One edge of the eye: where its outermost deterministic position sits,
/// relative to the centre of the crossings, and the sigma of the Gaussian
/// around it, both in unit intervals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeTail {
    pub mean: f64,
    pub sigma: f64,
}

/// The dual-Dirac fit of one eye's crossings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualDiracFit {
    /// The tail on the early side of the crossings, the eye's right edge.
    pub early: EdgeTail,
    /// The tail on the late side, the eye's left edge.
    pub late: EdgeTail,
    /// Probability mass of each tail's Gaussian, shared by both.
    pub tail_weight: f64,
    /// Crossings per unit interval: a half for random data.
    pub transition_density: f64,
    /// Crossings the fit read.
    pub crossing_count: usize,
    /// The unit interval in seconds, which the second-valued figures use.
    pub unit_interval: f64,
}

impl DualDiracFit {
    /// Random jitter, rms, in seconds: the mean of the two tails' sigmas.
    #[must_use]
    pub fn random_rms(&self) -> f64 {
        0.5 * (self.early.sigma + self.late.sigma) * self.unit_interval
    }

    /// Dual-Dirac deterministic jitter, in seconds: the distance between
    /// the two tails' zeros.
    #[must_use]
    pub fn deterministic(&self) -> f64 {
        (self.late.mean - self.early.mean).max(0.0) * self.unit_interval
    }

    /// Total jitter at `ber`, in seconds: a unit interval less the opening.
    #[must_use]
    pub fn total_jitter(&self, ber: f64) -> Option<f64> {
        let (left, right) = self.edges_at(ber)?;
        Some((1.0 - (right - left)).max(0.0) * self.unit_interval)
    }

    /// The eye's horizontal opening at `ber`, in unit intervals; zero when
    /// the two sides of the bathtub meet above it.
    #[must_use]
    pub fn opening(&self, ber: f64) -> Option<f64> {
        let (left, right) = self.edges_at(ber)?;
        Some((right - left).clamp(0.0, 1.0))
    }

    /// Where the bathtub crosses `ber` on each side, in unit intervals from
    /// the centre of the left crossings.
    #[must_use]
    pub fn edges_at(&self, ber: f64) -> Option<(f64, f64)> {
        let share = self.transition_density * self.tail_weight;
        if !(ber > 0.0 && share > 0.0 && ber < share) {
            return None;
        }
        let q = inverse_q_function(ber / share)?;
        let left = self.late.mean + q * self.late.sigma;
        let right = 1.0 + self.early.mean - q * self.early.sigma;
        Some((left, right))
    }

    /// Bit-error rate when sampling at `position` unit intervals from the
    /// centre of the left crossings: the late tail of the left crossings
    /// beyond it plus the early tail of the right crossings before it.
    #[must_use]
    pub fn ber_at(&self, position: f64) -> f64 {
        let tail = |offset: f64, sigma: f64| {
            if sigma > 0.0 {
                q_function(offset / sigma)
            } else if offset < 0.0 {
                1.0
            } else {
                0.0
            }
        };
        let left = tail(position - self.late.mean, self.late.sigma);
        let right = tail(1.0 + self.early.mean - position, self.early.sigma);
        (self.transition_density * self.tail_weight * (left + right)).min(0.5)
    }
}

/// A bathtub: the error rate counted at each sampling position across the
/// unit interval, and the fit that extrapolates it, when there is one.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bathtub {
    /// `(position, ber)` where the count is not zero, position in unit
    /// intervals from the centre of the left crossings.
    pub measured: Vec<(f64, f64)>,
    /// The dual-Dirac fit, or `None` when the record has too few crossings
    /// to extrapolate from.
    pub fit: Option<DualDiracFit>,
    /// Crossings the record has.
    pub crossing_count: usize,
    /// Unit intervals the crossings were counted over.
    pub unit_intervals: usize,
}

impl Bathtub {
    /// The lowest error rate the record can count: one error in all its
    /// unit intervals.
    #[must_use]
    pub fn counting_floor(&self) -> Option<f64> {
        #[allow(clippy::cast_precision_loss)]
        (self.unit_intervals > 0).then(|| 1.0 / self.unit_intervals as f64)
    }
}

/// The bathtub of crossings at `phases` (in unit intervals, at any offset:
/// they are folded and unwrapped here) counted over `unit_intervals` unit
/// intervals of `unit_interval` seconds.
///
/// Positions are measured from the centre of the crossings: the midpoint of
/// the two tails' deterministic positions when the record is fitted, the
/// middle of its spread when it is not.
#[must_use]
pub fn bathtub(phases: &[f64], unit_intervals: usize, unit_interval: f64) -> Bathtub {
    let mut crossings = unwrapped_crossings(phases);
    let mut out = Bathtub {
        crossing_count: crossings.len(),
        unit_intervals,
        ..Bathtub::default()
    };
    let (Some(&first), Some(&last)) = (crossings.first(), crossings.last()) else {
        return out;
    };
    if unit_intervals == 0 {
        return out;
    }
    let mut fit = fit_dual_dirac(&crossings, unit_intervals, unit_interval);
    let centre = fit.as_ref().map_or(0.5 * (first + last), |fit| {
        0.5 * (fit.early.mean + fit.late.mean)
    });
    for phase in &mut crossings {
        *phase -= centre;
    }
    if let Some(fit) = fit.as_mut() {
        fit.early.mean -= centre;
        fit.late.mean -= centre;
    }
    out.measured = measured_bathtub(&crossings, unit_intervals);
    out.fit = fit;
    out
}

/// Crossing phases folded into one unit interval and unwrapped across the
/// widest empty gap, so the histogram is contiguous. Ascending.
fn unwrapped_crossings(phases: &[f64]) -> Vec<f64> {
    let mut folded: Vec<f64> = phases
        .iter()
        .filter(|phase| phase.is_finite())
        .map(|phase| phase.rem_euclid(1.0))
        .collect();
    if folded.is_empty() {
        return folded;
    }
    folded.sort_by(f64::total_cmp);
    // Cut after the widest gap, the wrap-around gap included.
    let count = folded.len();
    let mut cut = count - 1;
    let mut widest = folded[0] + 1.0 - folded[count - 1];
    for index in 0..count - 1 {
        let gap = folded[index + 1] - folded[index];
        if gap > widest {
            widest = gap;
            cut = index;
        }
    }
    let start = (cut + 1) % count;
    (0..count)
        .map(|offset| {
            let index = (start + offset) % count;
            folded[index] + if index < start { 1.0 } else { 0.0 }
        })
        .collect()
}

/// The error rate counted at evenly spaced positions across the unit
/// interval: crossings of the left boundary later than the position plus
/// crossings of the right boundary earlier than it, per unit interval.
fn measured_bathtub(centred: &[f64], unit_intervals: usize) -> Vec<(f64, f64)> {
    #[allow(clippy::cast_precision_loss)]
    let per_interval = 1.0 / unit_intervals as f64;
    (0..MEASURED_POSITIONS)
        .filter_map(|step| {
            #[allow(clippy::cast_precision_loss)]
            let position = step as f64 / (MEASURED_POSITIONS - 1) as f64;
            let late = centred.len() - centred.partition_point(|&phase| phase <= position);
            let early = centred.partition_point(|&phase| phase + 1.0 < position);
            #[allow(clippy::cast_precision_loss)]
            let ber = (late + early) as f64 * per_interval;
            (ber > 0.0).then_some((position, ber))
        })
        .collect()
}

/// The dual-Dirac fit of ascending, unwrapped crossings, or `None` when there
/// are too few of them or their tails do not resolve a sigma.
fn fit_dual_dirac(
    crossings: &[f64],
    unit_intervals: usize,
    unit_interval: f64,
) -> Option<DualDiracFit> {
    let count = crossings.len();
    if count < MIN_FIT_CROSSINGS || !(unit_interval.is_finite() && unit_interval > 0.0) {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let tail_points = ((TAIL_SHARE * count as f64).floor() as usize).max(2);
    #[allow(clippy::cast_precision_loss)]
    let probability = |index: usize| (index as f64 + 0.5) / count as f64;
    let probabilities: Vec<f64> = (0..tail_points).map(probability).collect();
    // The early tail reads the earliest crossings; the late tail reads the
    // latest, mirrored so both are fitted as a lower tail.
    let early: Vec<f64> = crossings[..tail_points].to_vec();
    let late: Vec<f64> = crossings
        .iter()
        .rev()
        .take(tail_points)
        .map(|phase| -phase)
        .collect();

    let highest = probabilities[tail_points - 1];
    let mut fits: Vec<(f64, f64, Line, Line)> = Vec::with_capacity(TAIL_WEIGHT_STEPS);
    for step in 0..TAIL_WEIGHT_STEPS {
        #[allow(clippy::cast_precision_loss)]
        let weight = LEAST_TAIL_WEIGHT
            + (1.0 - LEAST_TAIL_WEIGHT) * step as f64 / (TAIL_WEIGHT_STEPS - 1) as f64;
        if highest >= weight {
            continue;
        }
        let quantiles: Option<Vec<f64>> = probabilities
            .iter()
            .map(|p| inverse_q_function(p / weight).map(|q| -q))
            .collect();
        let Some(quantiles) = quantiles else {
            continue;
        };
        // Each point weighted by the inverse variance of its position on
        // the Q scale: an order statistic's spread is √(F(1−F))/(N·f), and
        // the density f of a tail point is the normal density at its
        // quantile, so the few outermost crossings, which wander most, do
        // not steer the line the bulk of the tail sets.
        let weights: Vec<f64> = probabilities
            .iter()
            .zip(&quantiles)
            .map(|(p, q)| standard_normal_density(*q).powi(2) / (p * (1.0 - p)))
            .collect();
        let (Some(early_line), Some(late_line)) = (
            fit_line(&quantiles, &early, Some(&weights)),
            fit_line(&quantiles, &late, Some(&weights)),
        ) else {
            continue;
        };
        if !(early_line.slope > 0.0 && late_line.slope > 0.0) {
            continue;
        }
        // Straightness on the Q scale itself: each residual over its own
        // slope, so a tail is not judged straighter for being steeper.
        let cost = early_line.squared_residual / early_line.slope.powi(2)
            + late_line.squared_residual / late_line.slope.powi(2);
        fits.push((cost, weight, early_line, late_line));
    }
    // The heaviest weight whose lines are about as straight as the
    // straightest: a weight is lowered only for a tail that needs it.
    let straightest = fits.iter().map(|fit| fit.0).fold(f64::INFINITY, f64::min);
    let (_, weight, early_line, late_line) = fits
        .into_iter()
        .filter(|fit| fit.0 <= straightest * (1.0 + WEIGHT_PARSIMONY))
        .max_by(|a, b| a.1.total_cmp(&b.1))?;
    if !(early_line.slope >= 0.0 && late_line.slope >= 0.0) {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let transition_density = count as f64 / unit_intervals as f64;
    Some(DualDiracFit {
        early: EdgeTail {
            mean: early_line.intercept,
            sigma: early_line.slope,
        },
        late: EdgeTail {
            mean: -late_line.intercept,
            sigma: late_line.slope,
        },
        tail_weight: weight,
        transition_density,
        crossing_count: count,
        unit_interval,
    })
}

/// A least-squares line `y = intercept + slope · x` and its mean squared
/// residual.
#[derive(Debug, Clone, Copy)]
struct Line {
    intercept: f64,
    slope: f64,
    squared_residual: f64,
}

/// The normal density at `x`.
fn standard_normal_density(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

/// The least-squares line through `(x, y)`, each point weighted by
/// `weights` when given and equally otherwise; the residual is the
/// weighted mean.
fn fit_line(x: &[f64], y: &[f64], weights: Option<&[f64]>) -> Option<Line> {
    let count = x.len().min(y.len());
    if count < 2 {
        return None;
    }
    let weight = |index: usize| weights.map_or(1.0, |weights| weights[index]);
    let total: f64 = (0..count).map(weight).sum();
    if !(total.is_finite() && total > 0.0) {
        return None;
    }
    let x_mean = (0..count).map(|i| weight(i) * x[i]).sum::<f64>() / total;
    let y_mean = (0..count).map(|i| weight(i) * y[i]).sum::<f64>() / total;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..count {
        sxx += weight(i) * (x[i] - x_mean).powi(2);
        sxy += weight(i) * (x[i] - x_mean) * (y[i] - y_mean);
    }
    if sxx <= 0.0 {
        return None;
    }
    let slope = sxy / sxx;
    let intercept = y_mean - slope * x_mean;
    let squared_residual = (0..count)
        .map(|i| weight(i) * (y[i] - intercept - slope * x[i]).powi(2))
        .sum::<f64>()
        / total;
    (slope.is_finite() && intercept.is_finite()).then_some(Line {
        intercept,
        slope,
        squared_residual,
    })
}

/// How far the spread of `samples` (one population, its inner tail first
/// when sorted by `inner_first`) departs from a Gaussian's, in standard
/// deviations of the Q scale: near zero for random noise, an order larger
/// for the bounded, stepped spread of intersymbol interference. `None` with
/// too few samples to tell.
#[must_use]
pub(super) fn gaussian_tail_departure(inner_first: &[f64]) -> Option<f64> {
    let count = inner_first.len();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let tail_points = (TAIL_SHARE * count as f64).floor() as usize;
    if tail_points < 20 {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let quantiles: Option<Vec<f64>> = (0..tail_points)
        .map(|index| inverse_q_function((index as f64 + 0.5) / count as f64).map(|q| -q))
        .collect();
    let line = fit_line(&quantiles?, &inner_first[..tail_points], None)?;
    if line.slope <= 0.0 {
        return Some(f64::INFINITY);
    }
    Some(line.squared_residual.sqrt() / line.slope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::signal_integrity::eye_test_signals::{gaussian_draws, uniform_draws};

    const CROSSINGS: usize = 4_000;
    /// Random data crosses at half its bit boundaries.
    const INTERVALS: usize = 2 * CROSSINGS;
    const UI: f64 = 1e-9;

    fn fit_of(phases: &[f64]) -> DualDiracFit {
        bathtub(phases, INTERVALS, UI)
            .fit
            .expect("four thousand crossings are enough to fit")
    }

    /// Crossings displaced by Gaussian jitter alone read as random: the
    /// sigma is the generator's, and what deterministic jitter the fit
    /// finds is a fraction of it.
    #[test]
    fn purely_gaussian_jitter_reads_as_random() {
        let sigma = 0.02;
        let mut draw = gaussian_draws(0x51_7e57);
        let phases: Vec<f64> = (0..CROSSINGS).map(|_| 0.5 + sigma * draw()).collect();
        let fit = fit_of(&phases);
        let rj = fit.random_rms() / UI;
        let dj = fit.deterministic() / UI;
        assert!((rj - sigma).abs() <= 0.1 * sigma, "RJ {rj} UI, not {sigma}");
        assert!(dj <= 0.5 * rj, "DJ {dj} UI is not small beside RJ {rj} UI");
    }

    /// A sinusoidal jitter of 0.05 UI amplitude has a 0.1 UI peak-to-peak
    /// that is all deterministic. The dual-Dirac figure of a sinusoid sits
    /// a little inside its peak-to-peak, and its random part is a sliver.
    #[test]
    fn purely_sinusoidal_jitter_reads_as_deterministic() {
        let amplitude = 0.05;
        let phases: Vec<f64> = (0..CROSSINGS)
            .map(|edge| {
                let bit = edge as f64 * 2.3;
                0.5 + amplitude * (std::f64::consts::TAU * bit / 37.3).sin()
            })
            .collect();
        let fit = fit_of(&phases);
        let rj = fit.random_rms() / UI;
        let dj = fit.deterministic() / UI;
        assert!(
            (0.85 * 2.0 * amplitude..=2.0 * amplitude).contains(&dj),
            "DJ {dj} UI against a 0.1 UI peak-to-peak"
        );
        assert!(rj <= 0.1 * dj, "RJ {rj} UI is not a sliver of DJ {dj} UI");
    }

    /// Two deterministic positions 0.1 UI apart, each smeared by 0.01 UI rms
    /// of random jitter: the fit returns both.
    fn dual_dirac_mix() -> Vec<f64> {
        let mut draw = gaussian_draws(0x00d1_7ac5);
        let mut coin = uniform_draws(0x0c01_4f11);
        (0..CROSSINGS)
            .map(|_| {
                let side = if coin() < 0.5 { -0.05 } else { 0.05 };
                0.5 + side + 0.01 * draw()
            })
            .collect()
    }

    #[test]
    fn a_seeded_mix_returns_both_parts() {
        let fit = fit_of(&dual_dirac_mix());
        let rj = fit.random_rms() / UI;
        let dj = fit.deterministic() / UI;
        assert!((dj - 0.1).abs() <= 0.15 * 0.1, "DJ {dj} UI, not 0.1 UI");
        assert!((rj - 0.01).abs() <= 0.2 * 0.01, "RJ {rj} UI, not 0.01 UI");
        assert!((fit.transition_density - 0.5).abs() < 1e-12);
    }

    /// The opening at 1e-12 of the generator's own model: a half-and-half
    /// dual Dirac at ±0.05 UI with 0.01 UI rms, crossing at half the
    /// boundaries. Each side of the bathtub is ¼·Q((x − 0.05)/0.01) from its
    /// crossing, so the opening is 1 − 0.1 − 2·0.01·Q⁻¹(4e-12).
    #[test]
    fn the_bathtub_of_a_seeded_mix_opens_where_its_parameters_put_it() {
        let tub = bathtub(&dual_dirac_mix(), INTERVALS, UI);
        let fit = tub.fit.expect("fitted");
        let expected = 1.0 - 0.1 - 2.0 * 0.01 * inverse_q_function(4e-12).expect("inside");
        let opening = fit.opening(1e-12).expect("the eye opens at 1e-12");
        assert!(
            (opening - expected).abs() <= 0.04 * expected,
            "opening {opening} UI, not {expected} UI"
        );
        // The bathtub's floor is its count's: nothing is measured below one
        // error in its intervals, and the measured side walls stand where
        // the crossings are.
        let floor = tub.counting_floor().expect("intervals counted");
        assert!(tub.measured.iter().all(|&(_, ber)| ber >= floor));
        assert!(tub.measured.iter().any(|&(position, _)| position < 0.2));
        assert!(tub.measured.iter().any(|&(position, _)| position > 0.8));
        let centre = fit.ber_at(0.5);
        assert!(centre < 1e-12, "the centre of an open eye reads {centre:e}");
    }

    /// A record with too few crossings to place a tail keeps what it counted
    /// and extrapolates nothing.
    #[test]
    fn too_few_crossings_measure_and_do_not_extrapolate() {
        let phases: Vec<f64> = dual_dirac_mix().into_iter().take(60).collect();
        let tub = bathtub(&phases, 120, UI);
        assert_eq!(tub.fit, None);
        assert_eq!(tub.crossing_count, 60);
        assert!(!tub.measured.is_empty());
    }

    /// Crossings that straddle the fold are unwrapped before they are
    /// counted, so a histogram around phase zero is one histogram.
    #[test]
    fn crossings_straddling_the_fold_are_one_histogram() {
        let near_zero: Vec<f64> = dual_dirac_mix().iter().map(|phase| phase - 0.5).collect();
        let fit = fit_of(&near_zero);
        let reference = fit_of(&dual_dirac_mix());
        assert!((fit.deterministic() - reference.deterministic()).abs() < 1e-15);
        assert!((fit.random_rms() - reference.random_rms()).abs() < 1e-15);
    }
}
