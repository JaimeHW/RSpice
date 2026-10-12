//! C6 — digitally controlled buck converter (design §3 C6).
//!
//! 5 V in; 22 µH; a 2 µs switching period of 64 ticks with two dead ticks on
//! each side of the low-side pulse; 2 Ω load. The controller regulates the
//! mean sampled code to 922, i.e. v(vout) = 922/512 V.

use std::collections::BTreeMap;

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::design::PerVariant;
use crate::suites::mixed::events::Trace;
use crate::suites::mixed::measure::{Measured, evaluate};

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

/// The design's lossless peak-to-peak ripple, (Vin − Vout)·D·T/L with
/// D = Vout/Vin.
fn design_ripple(l: f64) -> f64 {
    let v = vout();
    (VIN - v) * (v / VIN) * PERIOD / l
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

/// The quantity measures min(−i) = −max(i) over its window; the expectation
/// is −(min(i) over the same window of the same run + the design ripple), so
/// the comparison is the peak-to-peak ripple against the design figure.
fn ripple(context: &Context<'_>) -> Result<Expected, String> {
    let quantity = context.quantity;
    let mut spec = quantity
        .of
        .clone()
        .unwrap_or_else(|| quantity.measure.clone());
    spec.kind = "min".to_owned();
    spec.signal = Some(PerVariant::All("i(lbuck)".to_owned()));
    let floor = match evaluate(&spec, context.variant, context.rspice) {
        Ok(Measured::Values(values)) if !values.is_empty() => values[0],
        Ok(_) => return Err("c06::ripple: no inductor-current minimum".into()),
        Err(error) => return Err(format!("c06::ripple: {error}")),
    };
    Ok(values(vec![-(floor + design_ripple(inductance(context)))]))
}

/// Charge balance: the settled mean inductor current is the load current.
fn ilmean(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![vout() / R_LOAD]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ripple_and_regulation_match_the_design() {
        // (Vin − Vout)·D·T/L = 0.1047·(22u/L) A; vout = 922/512 = 1.80078 V.
        assert!((design_ripple(L_NOMINAL) - 0.1047).abs() < 1e-4);
        assert!((design_ripple(2.0 * L_NOMINAL) - 0.1047 / 2.0).abs() < 1e-4);
        assert!((vout() - 1.80078).abs() < 1e-5);
    }
}
