//! Axis scales and tick generation.

use super::number_format::{
    axis_tick_labels, offset_anchor_label, offset_tick_labels, tick_label, wants_offset_notation,
};

/// X-axis scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum XScale {
    /// Linear data axis (time, sweep value, frequency bins).
    #[default]
    Linear,
    /// Logarithmic frequency axis; ticks fall on decades.
    Log10,
}

impl XScale {
    /// Map a data value to 0‥1 within `[min, max]`.
    pub fn normalize(self, x: f64, min: f64, max: f64) -> f64 {
        match self {
            XScale::Linear => (x - min) / (max - min),
            XScale::Log10 => {
                let (lx, lmin, lmax) = (x.max(f64::MIN_POSITIVE).log10(), min.log10(), max.log10());
                (lx - lmin) / (lmax - lmin)
            }
        }
    }

    /// Inverse of [`XScale::normalize`].
    pub fn denormalize(self, t: f64, min: f64, max: f64) -> f64 {
        match self {
            XScale::Linear => min + t * (max - min),
            XScale::Log10 => {
                let (lmin, lmax) = (min.log10(), max.log10());
                10f64.powf(lmin + t * (lmax - lmin))
            }
        }
    }
}

/// One axis' worth of ticks.
///
/// The anchor is what makes a deep zoom readable: past the point where every
/// label would share its leading digits, the labels become offsets and the
/// common part is stated once beside the axis.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TickSeries {
    /// Positions in canonical data space, with the label each one draws.
    pub ticks: Vec<(f64, String)>,
    /// Value the labels are offsets from, when they are offsets.
    pub anchor: Option<f64>,
    /// Even spacing in data space; zero when the ticks are not evenly spaced
    /// (log decades, or an explicit list).
    pub step: f64,
}

/// The share of its span a fit leaves free on each side: enough that no
/// curve runs along the frame, little enough that the curves fill the plot,
/// as ViVA's and EZwave's fits leave them. A sheet and its printed page fit
/// alike.
pub const FIT_HEADROOM: f64 = 0.04;

/// A linear fit of the values spanning `min..max`, padded by
/// [`FIT_HEADROOM`] each side.
///
/// A span that cannot survive its own padding is flat as far as any axis
/// can draw it: exactly constant, or constant to within a few ulps. Both
/// widen by one either side, because a range finer than the tick ladder can
/// subdivide leaves the plot with no labels at all.
#[must_use]
pub fn padded_fit((min, max): (f64, f64)) -> (f64, f64) {
    let pad = (max - min) * FIT_HEADROOM;
    if pad <= 0.0 {
        return (min - 1.0, max + 1.0);
    }
    (min - pad, max + pad)
}

/// However tall the pane, an axis past this many gridlines is a hatch, not a
/// scale — and every one of them costs a line and a label layout per frame.
pub const MAX_AXIS_TICKS: usize = 40;

/// The 45° lattice a phase in degrees is ruled on, as a Bode phase is
/// read: `(min, max)` widened out to whole 45° steps, and the values
/// ruled across it. Every step is ruled while they number at most
/// [`MAX_AXIS_TICKS`]; past that the lattice thins by whole multiples of
/// 45°, so what is left still falls on values a phase is read at, and
/// it halves again for each step of `thinning` a short pane gives up.
pub fn degree_lattice((min, max): (f64, f64), thinning: usize) -> (f64, f64, Vec<f64>) {
    let (low, high) = ((min / 45.0).floor() * 45.0, (max / 45.0).ceil() * 45.0);
    let steps = ((high - low) / 45.0).round().max(0.0) as usize;
    let stride = (steps + 1).div_ceil(MAX_AXIS_TICKS).max(1) << thinning;
    let values = (0..=steps)
        .step_by(stride)
        .map(|index| 45.0f64.mul_add(index as f64, low))
        .collect();
    (low, high, values)
}

/// The outer ring a polar chart rules a linear magnitude to: the first
/// 1-2-5 number at or past `peak`, so its rings land on numbers a reader
/// can hold. A peak that is not a positive number rules to 1.
pub fn polar_ring_ceiling(peak: f64) -> f64 {
    if !(peak.is_finite() && peak > 0.0) {
        return 1.0;
    }
    let decade = 10.0_f64.powf(peak.log10().floor());
    [1.0, 2.0, 5.0, 10.0]
        .iter()
        .map(|step| step * decade)
        .find(|candidate| *candidate >= peak)
        .unwrap_or(peak)
}

/// The next coarser spacing on the 1–2–5 ladder.
fn coarser(step: f64) -> f64 {
    let magnitude = 10f64.powf(step.log10().floor());
    let residual = step / magnitude;
    magnitude
        * if residual < 1.5 {
            2.0
        } else if residual < 3.5 {
            5.0
        } else {
            10.0
        }
}

/// "Nice" linear tick positions covering `[min, max]` with roughly `target`
/// steps, on a 1–2–5 ladder.
pub fn linear_ticks(min: f64, max: f64, target: usize) -> TickSeries {
    let span = max - min;
    if !(span.is_finite() && span > 0.0) {
        return TickSeries::default();
    }
    let raw_step = span / target.max(2) as f64;
    let magnitude = 10f64.powf(raw_step.log10().floor());
    let residual = raw_step / magnitude;
    let mut step = magnitude
        * if residual <= 1.5 {
            1.0
        } else if residual <= 3.5 {
            2.0
        } else if residual <= 7.5 {
            5.0
        } else {
            10.0
        };
    // A span finer than its own ladder can resolve — a trace flat to within
    // a few ulps — underflows `step` to zero. `min / 0.0` is then not a tick
    // index but an infinity, and the range it spans is every integer there
    // is: materialising that exhausts memory before it ever reaches a
    // painter.
    if !(step.is_finite() && step > 0.0) {
        return TickSeries::default();
    }
    // Past that guard the indices always fit an i64: two distinct doubles
    // differ by at least an ulp, so `step` is never smaller than about
    // 1e-17 of the range's own magnitude.
    let mut first = (min / step).ceil() as i64;
    let mut last = (max / step).floor() as i64;
    while last.saturating_sub(first) >= MAX_AXIS_TICKS as i64 {
        let coarsened = coarser(step);
        if !(coarsened.is_finite() && coarsened > step) {
            break;
        }
        step = coarsened;
        first = (min / step).ceil() as i64;
        last = (max / step).floor() as i64;
    }
    series_at(min, max, step, first..=last)
}

/// Linear tick positions covering `[min, max]` at exactly `step`: the ladder
/// [`linear_ticks`] chose, re-stepped. Empty when `step` would cut the window
/// into more than [`MAX_AXIS_TICKS`] steps.
pub fn linear_ticks_at(min: f64, max: f64, step: f64) -> TickSeries {
    if !(max > min && step.is_finite() && step > 0.0) {
        return TickSeries::default();
    }
    let first = (min / step).ceil() as i64;
    let last = (max / step).floor() as i64;
    if last.saturating_sub(first) >= MAX_AXIS_TICKS as i64 {
        return TickSeries::default();
    }
    series_at(min, max, step, first..=last)
}

/// The ticks at the ladder indices `indices` of `step`, labelled.
fn series_at(min: f64, max: f64, step: f64, indices: std::ops::RangeInclusive<i64>) -> TickSeries {
    let anchor = wants_offset_notation(min, max).then(|| {
        // An anchor on the tick grid keeps every delta an exact multiple of
        // the step, so the offsets read as round numbers rather than as the
        // rounding error between two of them.
        (((min + max) * 0.5) / step).round() * step
    });
    let values = indices
        .map(|index| {
            let value = index as f64 * step;
            // Snap float noise (e.g. 0.30000000000000004) to the step grid.
            (value / step).round() * step
        })
        // The ladder indices bound the window mathematically, but snapping
        // moves a value by an ulp — and an edge tick that lands an ulp
        // outside draws its gridline in the axis gutter.
        .filter(|value| (min..=max).contains(value))
        .collect::<Vec<_>>();
    let labels = match anchor {
        Some(anchor) => offset_tick_labels(
            &values
                .iter()
                .map(|value| value - anchor)
                .collect::<Vec<_>>(),
            step,
        ),
        None => axis_tick_labels(&values, step),
    };
    let ticks = values.into_iter().zip(labels).collect();
    TickSeries {
        ticks,
        anchor,
        step,
    }
}

/// How a column of tick labels stands along an axis `length` points long:
/// each label `label` points tall, kept at least `gap` points from the next.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelColumn {
    pub length: f64,
    pub label: f64,
    pub gap: f64,
}

impl LabelColumn {
    /// Where the label of a tick `fraction` of the way up the axis is
    /// centred, in points from its low end: on the tick, nudged inside at
    /// either end, as a framed plot keeps no margin to clip a label in.
    #[must_use]
    pub fn centre(&self, fraction: f64) -> f64 {
        let half = self.label * 0.5;
        (fraction * self.length).clamp(half, (self.length - half).max(half))
    }

    /// Whether the labels of ticks standing at `fractions` of the axis, low
    /// end first, all stand clear of each other.
    #[must_use]
    pub fn holds(&self, fractions: impl IntoIterator<Item = f64>) -> bool {
        let centres: Vec<f64> = fractions.into_iter().map(|f| self.centre(f)).collect();
        stands_clear(&centres, self.label + self.gap)
    }
}

/// Whether labels centred at `centres`, in points from the axis' low end and
/// in that order, stand at least `pitch` apart. A rounding hair under the
/// pitch still counts as clear.
fn stands_clear(centres: &[f64], pitch: f64) -> bool {
    centres
        .windows(2)
        .all(|pair| pair[1] - pair[0] >= pitch - 1.0e-6)
}

/// The ladder a linear axis over `[min, max]` is labelled with along
/// `column`: the densest 1–2–5 step, `step` or coarser, at which every label
/// stands clear of the next, so a short plot reads even steps with every
/// value named rather than a dense ladder with labels dropped from it. A
/// column too short even for two clear labels keeps the sparsest ladder that
/// still has two ticks, and [`even_labels`] chooses which of them it names.
pub fn fitted_linear_ticks(min: f64, max: f64, step: f64, column: LabelColumn) -> TickSeries {
    let fractions = |series: &TickSeries| {
        series
            .ticks
            .iter()
            .map(|(value, _)| (value - min) / (max - min))
            .collect::<Vec<_>>()
    };
    let mut series = linear_ticks_at(min, max, step);
    while !column.holds(fractions(&series)) {
        let next_step = coarser(series.step);
        if !(next_step.is_finite() && next_step > series.step) {
            break;
        }
        let next = linear_ticks_at(min, max, next_step);
        if next.ticks.len() < 2 {
            break;
        }
        series = next;
    }
    series
}

/// Which of the labels centred at `centres` (points from the axis' low end,
/// in that order) a column too short for all of them names: every `k`-th,
/// counted down from the top one, at the smallest `k` whose labels stand
/// `pitch` apart. The labels left keep even steps, and no hole opens between
/// two of them; the top one, which carries the unit, is always named.
#[must_use]
pub fn even_labels(centres: &[f64], pitch: f64) -> Vec<bool> {
    let count = centres.len();
    let named = |stride: usize, index: usize| (count - 1 - index).is_multiple_of(stride);
    let stride = (1..count)
        .find(|&stride| {
            let kept: Vec<f64> = (0..count)
                .filter(|&index| named(stride, index))
                .map(|index| centres[index])
                .collect();
            stands_clear(&kept, pitch)
        })
        .unwrap_or(count.max(1));
    (0..count).map(|index| named(stride, index)).collect()
}

/// Tick positions for a log axis covering `[min, max]`.
///
/// A decade ladder (1, 10, 100, 1k, …) is the right answer only while the
/// window holds decades. Zoomed inside one it holds none, and the axis used
/// to come back empty — a log plot with no scale on it at all, at exactly the
/// zoom a reader uses to measure something. So the ladder degrades: a window
/// narrower than two decades ticks the 1–2–5 mantissas, one narrower than a
/// decade ticks every mantissa, and a window too narrow even for that falls
/// back to a linear ladder whose labels still state true axis values.
pub fn decade_ticks(min: f64, max: f64) -> TickSeries {
    if !(min > 0.0 && max > min && max.is_finite()) {
        return TickSeries::default();
    }
    let (low, high) = (min.log10(), max.log10());
    let decades = high - low;
    let mut ticks: Vec<(f64, String)> = Vec::new();
    if decades >= 2.0 {
        let first = low.ceil() as i32;
        let last = high.floor() as i32;
        let count = (last - first + 1).max(1) as usize;
        let stride = count.div_ceil(MAX_AXIS_TICKS).max(1) as i32;
        let mut decade = first;
        while decade <= last {
            let value = 10f64.powi(decade);
            ticks.push((value, tick_label(value)));
            decade += stride;
        }
    } else {
        let mantissas: &[f64] = if decades >= 1.0 {
            &[1.0, 2.0, 5.0]
        } else {
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        };
        let first = low.floor() as i32;
        let last = high.floor() as i32;
        for decade in first..=last {
            let scale = 10f64.powi(decade);
            for mantissa in mantissas {
                let value = mantissa * scale;
                if (min..=max).contains(&value) {
                    ticks.push((value, tick_label(value)));
                }
            }
        }
    }
    if ticks.len() >= 2 {
        return TickSeries {
            ticks,
            anchor: None,
            step: 0.0,
        };
    }
    // Too narrow for any mantissa to land in it. The window is a hair wide,
    // so it is linear to well within a pixel — ladder it that way, and label
    // the true values.
    let linear = linear_ticks(min, max, 6);
    if !linear.ticks.is_empty() {
        return linear;
    }
    // Nothing resolves. One tick still tells the reader where they are, which
    // is the whole job of an axis.
    let midpoint = 10f64.powf((low + high) * 0.5);
    TickSeries {
        ticks: vec![(midpoint, tick_label(midpoint))],
        anchor: None,
        step: 0.0,
    }
}

/// Unlabeled subdivisions drawn between an axis' major ticks.
///
/// On a log axis these are the mantissa positions a reader interpolates
/// against — 2, 3, … 9 within each decade. Subdividing the screen distance
/// between decades instead put lines at 1.8, 3.2 and 5.6, which is a grid
/// that reads as a scale and is not one.
pub fn minor_grid_values(scale: XScale, ticks: &[(f64, String)], min: f64, max: f64) -> Vec<f64> {
    match scale {
        XScale::Log10 => {
            if !(min > 0.0 && max > min && max.is_finite()) {
                return Vec::new();
            }
            let (low, high) = (min.log10(), max.log10());
            // Every decade would draw eight lines; past a handful of decades
            // that is a wash rather than a grid.
            if high - low > 6.0 {
                return Vec::new();
            }
            let mut values = Vec::new();
            for decade in (low.floor() as i32)..=(high.floor() as i32) {
                let base = 10f64.powi(decade);
                for mantissa in 2..=9 {
                    let value = f64::from(mantissa) * base;
                    if value > min && value < max && !ticks.iter().any(|(tick, _)| *tick == value) {
                        values.push(value);
                    }
                }
            }
            values
        }
        XScale::Linear => ticks
            .windows(2)
            .flat_map(|pair| {
                let (a, b) = (pair[0].0, pair[1].0);
                (1..4).map(move |step| a + (b - a) * f64::from(step) / 4.0)
            })
            .collect(),
    }
}

/// Whether an axis is spaced by a constant ratio rather than a constant
/// step, as a decade or octave sweep is and as its log axis draws it,
/// judged from its first two and last two samples. An axis that does not
/// start positive, or does not rise, is not.
#[must_use]
pub fn spaced_by_ratio(first: [f64; 2], last: [f64; 2]) -> bool {
    if !(first[0].is_finite() && first[0] > 0.0 && last.iter().all(|x| x.is_finite())) {
        return false;
    }
    let (first_ratio, last_ratio) = (first[1] / first[0], last[1] / last[0]);
    let (first_step, last_step) = (first[1] - first[0], last[1] - last[0]);
    let geometric =
        (first_ratio - last_ratio).abs() <= 1e-6 * first_ratio && first_ratio > 1.0 + 1e-9;
    let arithmetic = (first_step - last_step).abs() <= 1e-6 * first_step.abs();
    geometric && !arithmetic
}

/// The chrome an offset axis states once: its anchor, with the unit.
pub fn anchor_label(series: &TickSeries, unit: &str) -> Option<String> {
    series
        .anchor
        .map(|anchor| offset_anchor_label(anchor, unit, series.step))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The next finer spacing on the 1–2–5 ladder.
    fn finer(step: f64) -> f64 {
        let magnitude = 10f64.powf(step.log10().floor());
        let residual = (step / magnitude).round();
        if residual == 5.0 {
            step * 0.4
        } else {
            step * 0.5
        }
    }

    /// A short plot reads even steps with every tick named. 3C's harmonic
    /// spectrum, −74 dBV to 10 dBV on a strip about 120 points tall, kept
    /// the tall plot's 10 dB ladder and dropped the −60 label beside a −70
    /// nudged inside the frame: "−50, [gap], −70" read as a missing value.
    #[test]
    fn a_short_axis_coarsens_its_ladder_until_every_label_is_named() {
        let column = |length| LabelColumn {
            length,
            label: 13.0,
            gap: 2.0,
        };
        for (min, max) in [
            (-74.08, 10.0),
            (-114.5, -38.2),
            (0.0, 1.0),
            (-1.3e-3, 2.2e-3),
            (-180.0, 180.0),
        ] {
            let tall = linear_ticks(min, max, 6);
            for length in [60.0, 90.0, 120.0, 140.0, 240.0, 600.0] {
                let series = fitted_linear_ticks(min, max, tall.step, column(length));
                let values: Vec<f64> = series.ticks.iter().map(|(value, _)| *value).collect();
                let at = format!("{min} … {max} over {length} pt: {values:?}");
                assert!(values.len() >= 2, "a scale needs two ticks, {at}");
                assert!(
                    values
                        .windows(2)
                        .all(|pair| ((pair[1] - pair[0]) - series.step).abs() <= series.step * 1e-9),
                    "the steps are uneven, {at}"
                );
                let fraction = |value: &f64| (value - min) / (max - min);
                assert!(
                    column(length).holds(values.iter().map(fraction)),
                    "a label is dropped, {at}"
                );
                let centres: Vec<f64> = values
                    .iter()
                    .map(|value| column(length).centre(fraction(value)))
                    .collect();
                assert!(
                    even_labels(&centres, 15.0).iter().all(|named| *named),
                    "a tick goes unnamed, {at}"
                );
                // The ladder is the densest that fits: the tall plot's where it
                // fits, else one finer step would drop a label.
                if series.step > tall.step {
                    let denser = linear_ticks_at(min, max, finer(series.step));
                    assert!(
                        !column(length)
                            .holds(denser.ticks.iter().map(|(value, _)| fraction(value))),
                        "a denser ladder fits, {at}"
                    );
                } else {
                    assert_eq!(series.step, tall.step, "{at}");
                }
            }
        }
    }

    /// The spectrum strip of the report, at the three heights 3C pictures.
    #[test]
    fn the_harmonic_spectrum_reads_twenty_db_steps_on_a_short_strip() {
        let column = |length| LabelColumn {
            length,
            label: 13.0,
            gap: 2.0,
        };
        let labels = |length| {
            fitted_linear_ticks(-74.08, 10.0, 10.0, column(length))
                .ticks
                .into_iter()
                .map(|(_, label)| label)
                .collect::<Vec<_>>()
        };
        // At 140 points the 10 dB ladder would fit but for its top tick,
        // 10 dBV on the frame, whose label is nudged down onto 0 dBV's.
        assert_eq!(labels(60.0), ["−50", "0"]);
        assert_eq!(labels(90.0), ["−60", "−40", "−20", "0"]);
        assert_eq!(labels(140.0), ["−60", "−40", "−20", "0"]);
        assert_eq!(
            labels(200.0),
            ["−70", "−60", "−50", "−40", "−30", "−20", "−10", "0", "10"]
        );
    }

    /// A column too short for even two clear labels names every k-th from
    /// the top, the top one always: never a hole between two named ticks.
    #[test]
    fn labels_too_tight_for_the_column_are_named_at_even_strides_from_the_top() {
        assert_eq!(even_labels(&[6.0, 30.0, 54.0], 15.0), [true, true, true]);
        assert_eq!(
            even_labels(&[6.0, 14.0, 22.0, 30.0, 38.0], 15.0),
            [true, false, true, false, true]
        );
        assert_eq!(
            even_labels(&[6.0, 12.0, 18.0, 24.0], 15.0),
            [true, false, false, true]
        );
        assert_eq!(even_labels(&[6.0, 9.0], 15.0), [false, true]);
        assert!(even_labels(&[], 15.0).is_empty());
    }

    #[test]
    fn linear_ticks_are_nice() {
        let series = linear_ticks(0.0, 10e-6, 6);
        assert!(series.ticks.len() >= 4 && series.ticks.len() <= 8);
        assert_eq!(series.ticks[0].1, "0");
        assert!(series.ticks.iter().any(|tick| tick.1 == "4µ"));
        assert!(series.anchor.is_none());
    }

    #[test]
    fn a_phase_is_ruled_on_whole_forty_five_degree_steps() {
        assert_eq!(
            degree_lattice((-170.0, -3.0), 0),
            (-180.0, 0.0, vec![-180.0, -135.0, -90.0, -45.0, 0.0])
        );
        // A span past the tick budget thins by whole steps, and each step
        // of thinning halves it again.
        let (_, _, wide) = degree_lattice((0.0, 45.0 * 60.0), 0);
        assert_eq!(wide[1] - wide[0], 90.0);
        let (_, _, thinned) = degree_lattice((0.0, 45.0 * 60.0), 1);
        assert_eq!(thinned[1] - thinned[0], 180.0);
    }

    #[test]
    fn a_span_finer_than_its_ladder_yields_no_ticks_rather_than_every_integer() {
        // The step underflows to zero here. Before the guard, `min / step`
        // was an infinity and the tick range covered the whole i64 grid,
        // which asked the allocator for well over a hundred gigabytes.
        assert!(linear_ticks(0.0, 5.0e-324, 6).ticks.is_empty());
        assert!(linear_ticks(-5.0e-324, 5.0e-324, 6).ticks.is_empty());
    }

    #[test]
    fn values_that_dwarf_their_own_span_still_ladder_across_the_window() {
        // 1e300 zoomed to a 1e290-wide window. The ladder indices are ~5e10,
        // far from zero, but they stay a handful of steps apart and inside
        // the window — the axis describes the window, not the magnitude.
        let (min, max) = (1.0e300, 1.0e300 + 1.0e290);
        let series = linear_ticks(min, max, 6);
        assert!(!series.ticks.is_empty());
        assert!(series.ticks.len() <= 12, "{} ticks", series.ticks.len());
        assert!(
            series
                .ticks
                .iter()
                .all(|(value, _)| (min..=max).contains(value))
        );
    }

    #[test]
    fn a_wide_but_ordinary_span_still_ladders() {
        // The guards must not disturb the ranges the instrument actually
        // draws, including a genuinely astronomical but well-formed one.
        let series = linear_ticks(-1.16e300, 1.16e300, 6);
        assert!(series.ticks.len() >= 3 && series.ticks.len() <= 9);
        assert!(series.ticks.iter().all(|(value, _)| value.is_finite()));
    }

    #[test]
    fn decade_ticks_span() {
        let series = decade_ticks(1.0, 1e9);
        assert_eq!(series.ticks.len(), 10);
        assert_eq!(series.ticks[3].1, "1k");
        assert_eq!(series.ticks[9].1, "1G");
    }

    /// The tick series as an axis would render it: labels in order.
    fn labels(min: f64, max: f64, target: usize) -> Vec<String> {
        linear_ticks(min, max, target)
            .ticks
            .into_iter()
            .map(|(_, label)| label)
            .collect()
    }

    fn log_labels(min: f64, max: f64) -> Vec<String> {
        decade_ticks(min, max)
            .ticks
            .into_iter()
            .map(|(_, label)| label)
            .collect()
    }

    /// An axis whose labels all read the same has stopped being an axis. This
    /// sweeps the zoom range the viewport floor admits, at magnitudes from
    /// picoseconds to gigahertz, and asks only that a reader can tell two
    /// neighbouring ticks apart.
    #[test]
    fn adjacent_tick_labels_never_render_identically_at_any_zoom() {
        for center in [0.0_f64, 1.0, -3.3, 1.0e-3, 5.0e6, 2.5e-9, 1.234e12] {
            let reference = if center == 0.0 { 1.0 } else { center.abs() };
            for exponent in -12..=3 {
                let span = reference * 10f64.powi(exponent);
                let (min, max) = (center - span * 0.5, center + span * 0.5);
                let rendered = labels(min, max, 6);
                for pair in rendered.windows(2) {
                    assert_ne!(pair[0], pair[1], "[{min:e}, {max:e}] rendered {rendered:?}");
                }
            }
        }
    }

    /// Neither does a log axis, whose labels come from a different ladder.
    #[test]
    fn adjacent_log_tick_labels_never_render_identically_either() {
        for decade in -9..9 {
            let base = 10f64.powi(decade) * 1.37;
            for ratio in [1.000_01, 1.2, 2.0, 7.0, 40.0, 1.0e4] {
                let rendered = log_labels(base, base * ratio);
                for pair in rendered.windows(2) {
                    assert_ne!(pair[0], pair[1], "[{base:e} ×{ratio}] {rendered:?}");
                }
            }
        }
    }

    /// A log axis zoomed inside one decade still has to say where it is.
    #[test]
    fn a_log_window_inside_a_single_decade_still_has_ticks() {
        assert_eq!(log_labels(1.2e3, 4.5e3), vec!["2k", "3k", "4k"]);
        assert!(!log_labels(1.0e6, 1.9e6).is_empty());
        assert_eq!(log_labels(2.0e3, 9.0e3).len(), 8);
        // Between one and two decades the mantissa ladder thins to 1–2–5.
        assert_eq!(
            log_labels(1.0e3, 2.0e4),
            vec!["1k", "2k", "5k", "10k", "20k"]
        );
    }

    /// The property behind it: a log axis is never tickless for a finite,
    /// non-empty window, at any zoom and any magnitude.
    #[test]
    fn a_log_axis_is_never_tickless() {
        for decade in -12..12 {
            let base = 10f64.powi(decade) * 1.37;
            for ratio in [
                1.000_001, 1.05, 1.4, 2.0, 5.0, 9.5, 12.0, 40.0, 1.0e3, 1.0e6,
            ] {
                let (min, max) = (base, base * ratio);
                let rendered = decade_ticks(min, max);
                assert!(
                    !rendered.ticks.is_empty(),
                    "[{min:e}, {max:e}] (ratio {ratio}) drew no ticks"
                );
                assert!(
                    rendered
                        .ticks
                        .iter()
                        .all(|(value, _)| (min..=max).contains(value)),
                    "[{min:e}, {max:e}] placed a tick outside the window: {rendered:?}"
                );
            }
        }
    }

    /// However deep the zoom, an axis stays a scale rather than a hatch.
    #[test]
    fn no_axis_ever_exceeds_the_gridline_clamp() {
        for target in [2usize, 6, 40, 400] {
            assert!(linear_ticks(0.0, 1.0, target).ticks.len() <= MAX_AXIS_TICKS);
        }
        assert!(linear_ticks(0.0, 1.0e18, 2_000).ticks.len() <= MAX_AXIS_TICKS);
        assert!(decade_ticks(1.0e-300, 1.0e300).ticks.len() <= MAX_AXIS_TICKS);
    }

    #[test]
    fn a_deep_zoom_states_its_anchor_and_labels_the_offsets() {
        let series = linear_ticks(1.0e-3 - 5.0e-8, 1.0e-3 + 5.0e-8, 6);
        assert!(series.anchor.is_some());
        assert!(
            series.ticks.iter().any(|(_, label)| label.starts_with('+')),
            "{series:?}"
        );
        assert_eq!(anchor_label(&series, "s").as_deref(), Some("+1 ms"));
        let plain = linear_ticks(0.0, 1.0e-3, 6);
        assert!(plain.anchor.is_none());
        assert_eq!(anchor_label(&plain, "s"), None);
    }

    /// A log grid's minor lines are the mantissas, not a screen-space
    /// quartering of the distance between decades.
    #[test]
    fn log_minor_grid_sits_on_mantissa_positions() {
        let ticks = decade_ticks(1.0, 100.0).ticks;
        let minors = minor_grid_values(XScale::Log10, &ticks, 1.0, 100.0);
        assert!(minors.contains(&2.0));
        assert!(minors.contains(&5.0));
        assert!(minors.contains(&50.0));
        assert!(!minors.contains(&10.0));
        assert_eq!(minors.len(), 16);
        // A linear axis keeps its even subdivisions.
        let linear = linear_ticks(0.0, 1.0, 4).ticks;
        let minors = minor_grid_values(XScale::Linear, &linear, 0.0, 1.0);
        assert_eq!(minors.len(), (linear.len() - 1) * 3);
    }

    #[test]
    fn log_normalize_roundtrip() {
        let x = 12_345.0;
        let t = XScale::Log10.normalize(x, 1.0, 1e9);
        let back = XScale::Log10.denormalize(t, 1.0, 1e9);
        assert!((back / x - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_polar_rim_is_the_first_one_two_five_number_past_the_peak() {
        assert_eq!(polar_ring_ceiling(7.3), 10.0);
        assert_eq!(polar_ring_ceiling(1.5), 2.0);
        assert!((polar_ring_ceiling(0.04) - 0.05).abs() < 1e-15);
        assert_eq!(polar_ring_ceiling(5.0), 5.0);
        assert_eq!(polar_ring_ceiling(0.0), 1.0);
    }
}
