//! C9 — power-on reset under a supply ramp (design §3 C9, amendments A4, A4b
//! and A6).
//!
//! Supply `pwl(0 0 100u 3.3 300u 3.3 310u 2.0 320u 2.0 330u 3.3)`; comparators
//! `hi = v·½(1 + tanh(200(v − 2.7)))` and `lo_n = v·½(1 + tanh(200(v − 2.4)))`
//! of v = v(vdd); the controller's clock rises at 0.5 µs + k µs and `rst_n`
//! rises on the 17th clock rise after `por_n` (the next one plus 16 µs).
//!
//! The converter model is the variant's: `base` states `.param vcc=3.3`, a
//! static supply (A/D threshold 1.65 V, D/A level 3.3 V, amendment A4); `p3`
//! (authored supply-port connect modules, P3.2) and `live` (inferred supply,
//! P3.2a) convert against v(vdd) itself (threshold v/2, level v).

use super::{Context, Registry, points, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("tpor", tpor),
    ("trst", trst),
    ("tbo", tbo),
    ("tpor2", tpor2),
    ("trst2", trst2),
    ("cnt", cnt),
    ("track", track),
    ("porn_up", porn_up),
    ("porn_down", porn_down),
    ("porn_restart", porn_restart),
    ("rstn", rstn),
    ("thr_hi", thr_hi),
    ("thr_lo", thr_lo),
];

const STATIC_SUPPLY: f64 = 3.3;
const HI_THRESHOLD: f64 = 2.7;
const LO_THRESHOLD: f64 = 2.4;
const COMPARATOR_GAIN: f64 = 200.0;
const CLOCK_PERIOD: f64 = 1e-6;
const FIRST_CLOCK_RISE: f64 = 0.5e-6;
/// `rst_n` rises this many clock periods after the first clock rise that sees
/// `por_n` high.
const RESET_PERIODS: f64 = 16.0;

/// `(time, volts)` corners of the supply's PWL.
const SUPPLY: [(f64, f64); 6] = [
    (0.0, 0.0),
    (100e-6, 3.3),
    (300e-6, 3.3),
    (310e-6, 2.0),
    (320e-6, 2.0),
    (330e-6, 3.3),
];

/// Whether the variant's converters track the supply (p3, live) or convert
/// against the stated static 3.3 V (base).
fn tracks_supply(variant: &str) -> bool {
    variant != "base"
}

fn comparator(vdd: f64, threshold: f64) -> f64 {
    vdd * 0.5 * (1.0 + (COMPARATOR_GAIN * (vdd - threshold)).tanh())
}

/// The A/D threshold a converter input compares against at supply `vdd`.
fn ad_threshold(variant: &str, vdd: f64) -> f64 {
    if tracks_supply(variant) {
        0.5 * vdd
    } else {
        0.5 * STATIC_SUPPLY
    }
}

/// The level a held 1 drives at supply `vdd`.
fn da_level(variant: &str, vdd: f64) -> f64 {
    if tracks_supply(variant) {
        vdd
    } else {
        STATIC_SUPPLY
    }
}

/// Whether the comparator output reads 1 at supply `vdd`.
fn reads_high(variant: &str, vdd: f64, comparator_threshold: f64) -> bool {
    comparator(vdd, comparator_threshold) >= ad_threshold(variant, vdd)
}

/// The supply at which the comparator output crosses the A/D threshold, by
/// bisection (the output minus the threshold rises monotonically through the
/// comparator's own threshold).
fn crossing_supply(variant: &str, comparator_threshold: f64) -> f64 {
    let difference = |vdd: f64| comparator(vdd, comparator_threshold) - ad_threshold(variant, vdd);
    let (mut low, mut high) = (comparator_threshold - 0.1, comparator_threshold + 0.1);
    for _ in 0..200 {
        let middle = 0.5 * (low + high);
        if difference(middle) < 0.0 {
            low = middle;
        } else {
            high = middle;
        }
    }
    0.5 * (low + high)
}

/// The time at which the supply's linear segment `[a, b]` passes `vdd`.
fn segment_time(segment: usize, vdd: f64) -> f64 {
    let (t0, v0) = SUPPLY[segment];
    let (t1, v1) = SUPPLY[segment + 1];
    t0 + (t1 - t0) * (vdd - v0) / (v1 - v0)
}

fn supply_at(time: f64) -> f64 {
    for pair in SUPPLY.windows(2) {
        let ((t0, v0), (t1, v1)) = (pair[0], pair[1]);
        if time <= t1 {
            return v0 + (v1 - v0) * (time - t0) / (t1 - t0);
        }
    }
    SUPPLY[SUPPLY.len() - 1].1
}

/// The first clock rise strictly after `time`.
fn next_clock_rise(time: f64) -> f64 {
    let k = ((time - FIRST_CLOCK_RISE) / CLOCK_PERIOD).floor() + 1.0;
    FIRST_CLOCK_RISE + k.max(0.0) * CLOCK_PERIOD
}

fn power_on(variant: &str) -> f64 {
    segment_time(0, crossing_supply(variant, HI_THRESHOLD))
}

fn power_on_again(variant: &str) -> f64 {
    segment_time(4, crossing_supply(variant, HI_THRESHOLD))
}

/// hi rises through the A/D threshold on the 0 → 3.3 V ramp.
fn tpor(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![power_on(context.variant)]))
}

/// The 17th clock rise after `por_n`: the next one plus 16 periods.
fn trst(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![
        next_clock_rise(power_on(context.variant)) + RESET_PERIODS * CLOCK_PERIOD,
    ]))
}

/// lo_n falls through the A/D threshold on the 3.3 → 2.0 V brown-out ramp.
fn tbo(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![segment_time(
        2,
        crossing_supply(context.variant, LO_THRESHOLD),
    )]))
}

/// hi rises through the A/D threshold again on the 2.0 → 3.3 V recovery.
fn tpor2(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![power_on_again(context.variant)]))
}

fn trst2(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![
        next_clock_rise(power_on_again(context.variant)) + RESET_PERIODS * CLOCK_PERIOD,
    ]))
}

/// The counter, sampled before each clock rise in the window, has counted the
/// clock rises since its release: one per rise from the first after `por_n`.
fn cnt(context: &Context<'_>) -> Result<Expected, String> {
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or("c09::cnt needs the sample window")?;
    let first = next_clock_rise(power_on(context.variant));
    let mut codes = Vec::new();
    let mut rise = next_clock_rise(start - 1e-12);
    while rise <= stop {
        let counted = ((rise - first) / CLOCK_PERIOD).round();
        codes.push(counted.rem_euclid(16.0));
        rise += CLOCK_PERIOD;
    }
    Ok(values(codes))
}

/// A held 1 at the supply's own level.
fn track(context: &Context<'_>) -> Result<Expected, String> {
    let times = context
        .quantity
        .measure
        .at
        .clone()
        .ok_or("c09::track needs its sample times")?;
    Ok(values(times.into_iter().map(supply_at).collect()))
}

/// por_n over the upward sweep: hi rising through the threshold sets it, and
/// nothing on the way up clears it.
fn porn_up(context: &Context<'_>) -> Result<Expected, String> {
    let variant = context.variant;
    let mut set = false;
    let mut out = Vec::new();
    for vdd in points(context.quantity)? {
        set |= reads_high(variant, vdd, HI_THRESHOLD);
        out.push(if set { da_level(variant, vdd) } else { 0.0 });
    }
    Ok(values(out))
}

/// por_n over the downward sweep: the first point's operating point sets it
/// (hi is high at time 0 there) and lo_n falling through the threshold clears
/// it. The sweep is monotonic, so each point's state follows from lo_n alone.
fn porn_down(context: &Context<'_>) -> Result<Expected, String> {
    let variant = context.variant;
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(|vdd| {
                if reads_high(variant, vdd, LO_THRESHOLD) {
                    da_level(variant, vdd)
                } else {
                    0.0
                }
            })
            .collect(),
    ))
}

/// Each point an independent operating point: por_n is set exactly where hi
/// reads high at time 0.
fn porn_restart(context: &Context<'_>) -> Result<Expected, String> {
    let variant = context.variant;
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(|vdd| {
                if reads_high(variant, vdd, HI_THRESHOLD) {
                    da_level(variant, vdd)
                } else {
                    0.0
                }
            })
            .collect(),
    ))
}

/// rst_n keeps its initial 0: no time passes in a sweep.
fn rstn(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![0.0; points(context.quantity)?.len()]))
}

fn thr_hi(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(|vdd| comparator(vdd, HI_THRESHOLD))
            .collect(),
    ))
}

fn thr_lo(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(|vdd| comparator(vdd, LO_THRESHOLD))
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_and_times_match_the_design() {
        // Supply-tracking converters cross exactly at the comparator
        // thresholds: the design's 81.82 µs and 306.92 µs.
        assert!((power_on("live") - 81.818_181_8e-6).abs() < 1e-12);
        let brown_out = segment_time(2, crossing_supply("live", LO_THRESHOLD));
        assert!((brown_out - 306.923_076_9e-6).abs() < 1e-12, "{brown_out}");
        // A static 1.65 V threshold sits 1.1 mV and 2.0 mV past them.
        assert!((crossing_supply("base", HI_THRESHOLD) - 2.701_13).abs() < 1e-5);
        assert!((crossing_supply("base", LO_THRESHOLD) - 2.401_97).abs() < 1e-5);
        assert!((power_on("base") - 81.852e-6).abs() < 1e-9);
        // The reset releases on the 17th clock rise: 82.5 µs + 16 µs.
        assert!((next_clock_rise(power_on("base")) - 82.5e-6).abs() < 1e-15);
        assert!((next_clock_rise(power_on_again("base")) - 325.5e-6).abs() < 1e-15);
        assert!((supply_at(50e-6) - 1.65).abs() < 1e-12);
        assert!((supply_at(315e-6) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn sweep_states_step_at_the_first_point_past_the_threshold() {
        assert!(!reads_high("base", 2.70, HI_THRESHOLD));
        assert!(reads_high("base", 2.71, HI_THRESHOLD));
        assert!(!reads_high("base", 2.40, LO_THRESHOLD));
        assert!(reads_high("base", 2.41, LO_THRESHOLD));
        assert!(!reads_high("live", 2.69, HI_THRESHOLD));
        assert!(reads_high("live", 2.71, HI_THRESHOLD));
        assert_eq!(da_level("base", 3.0), 3.3);
        assert_eq!(da_level("live", 3.0), 3.0);
    }
}
