//! C6 — digitally controlled buck converter (design §3 C6).
//!
//! 5 V in; 22 µH; a 2 µs switching period of 64 ticks with two dead ticks on
//! each side of the low-side pulse; 2 Ω load. The controller regulates the
//! mean sampled code to 922, i.e. v(vout) = 922/512 V.

use std::collections::BTreeMap;

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::events::Trace;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[
    ("vss", vss),
    ("tper", tper),
    ("dead", dead),
    ("ripple", ripple),
    ("ilmean", ilmean),
];

const VIN: f64 = 5.0;
const TARGET: f64 = 922.0;
const CODES_PER_VOLT: f64 = 512.0;
const PERIOD: f64 = 2e-6;
const TICK: f64 = PERIOD / 64.0;
const DEAD_TICKS: f64 = 2.0;
const R_LOAD: f64 = 2.0;
const L_NOMINAL: f64 = 22e-6;
const HS_NET: &str = "xctrl.hs__event_0";

fn vout() -> f64 {
    TARGET / CODES_PER_VOLT
}

fn vss(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![vout()]))
}

fn tper(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![PERIOD]))
}

/// ls as the controller's schedule derives it from hs: up two ticks after
/// each hs fall, down two ticks before each hs rise. Built from RSpice's own
/// hs trace, so it checks the dead-time rule, not the duty.
fn dead(context: &Context<'_>) -> Result<Expected, String> {
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or("c06::dead needs the quantity's window")?;
    let hs = context
        .rspice
        .events
        .get(HS_NET)
        .ok_or_else(|| format!("c06::dead: {HS_NET} is absent"))?;
    let mut edges: Trace = hs
        .windows(2)
        .filter_map(|pair| match (pair[0].1.as_str(), pair[1].1.as_str()) {
            ("1", "0") => Some((pair[1].0 + DEAD_TICKS * TICK, "1".to_owned())),
            ("0", "1") => Some((pair[1].0 - DEAD_TICKS * TICK, "0".to_owned())),
            _ => None,
        })
        .filter(|(time, _)| *time >= start && *time <= stop)
        .collect();
    edges.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut traces = BTreeMap::new();
    traces.insert("xctrl.ls__event_0".to_owned(), edges);
    Ok(Expected::Measured(Measured::Events(traces)))
}

/// Switch on-resistance, inductor DCR, and the low-side body diode
/// (`net/stage.cir`), with kT/q at 27 °C.
const R_ON: f64 = 20e-3;
const R_DCR: f64 = 20e-3;
const DIODE_IS: f64 = 1e-12;
const DIODE_RS: f64 = 10e-3;
const THERMAL_VOLTAGE: f64 = 0.025865;

/// The ticks per period the regulated stage needs on average:
/// 64(Vout + I·DCR) + 60·I·Ron + 4·Vf(I) = d*·Vin, with I = Vout/R and the
/// body diode conducting the four dead ticks. 24.07 for the 2 Ω load.
fn required_ticks() -> f64 {
    let current = vout() / R_LOAD;
    let diode = THERMAL_VOLTAGE * (current / DIODE_IS + 1.0).ln() + DIODE_RS * current;
    (64.0 * (vout() + current * R_DCR) + 60.0 * current * R_ON + 4.0 * diode) / VIN
}

/// The peak-to-peak inductor current the sampled controller produces (the
/// derivation in `tolerance.toml`): the sigma-delta dithers the duty between
/// ⌊d*⌋ and ⌊d*⌋ + 1 ticks, so the largest rise is s_on·(⌊d*⌋ + 1)·τ with
/// s_on = (Vin − Vout − I(Ron + DCR))/L, and the period-start current drifts
/// by at most one tick's step Vin·τ/L over a dither cycle. The expectation is
/// the centre of [rise, rise + Vin·τ/L]; the quantity's rel is the half-width.
/// The design's lossless (Vin − Vout)·D·T/L = 0.1047 A ignored the dither,
/// the losses and the dead time.
fn design_ripple(l: f64) -> f64 {
    let current = vout() / R_LOAD;
    let rise_rate = (VIN - vout() - current * (R_ON + R_DCR)) / l;
    let ticks = required_ticks().floor() + 1.0;
    rise_rate * ticks * TICK + 0.5 * VIN * TICK / l
}

/// The run's inductance: a Monte Carlo trial's drawn `lv` (recorded by the
/// engine fallback as `param:lv`), else the decks' nominal 22 µH.
fn inductance(context: &Context<'_>) -> f64 {
    context
        .rspice
        .scalars
        .get("param:lv")
        .copied()
        .unwrap_or(L_NOMINAL)
}

/// The peak-to-peak inductor current over the quantity's window, measured
/// by the `pp` kind, against the design's lossless ripple for the run's L.
fn ripple(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![design_ripple(inductance(context))]))
}

/// Charge balance: the settled mean inductor current is the load current.
fn ilmean(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![vout() / R_LOAD]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ripple_and_regulation_match_the_derivation() {
        // d* = 24.07 ticks, so the dither is 24/25; the 25-tick rise is
        // 0.11233 A and one tick's drift 7.10 mA at 22 µH: centre 0.11588 A.
        assert!(
            (required_ticks() - 24.07).abs() < 0.01,
            "{}",
            required_ticks()
        );
        assert!((design_ripple(L_NOMINAL) - 0.11588).abs() < 2e-5);
        assert!((design_ripple(2.0 * L_NOMINAL) - 0.11588 / 2.0).abs() < 1e-5);
        assert!((vout() - 1.80078).abs() < 1e-5);
    }
}
