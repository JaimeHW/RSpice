//! C6 — digitally controlled buck converter (design §3 C6).
//!
//! 5 V in; 22 µH; a 2 µs switching period of 64 ticks with two dead ticks on
//! each side of the low-side pulse; 2 Ω load. The controller regulates the
//! mean sampled code to 922, i.e. v(vout) = 922/512 V.
//!
//! The `vsup` variant drives each gate through the built-in D/A converter
//! with its supply on the `vdrv` net, `rhi` = 10 Ω and `rlo` = 5 Ω, into
//! Cg = 1 nF; vdrv sags from 5 V to 3.5 V and recovers.

use std::collections::BTreeMap;

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::events::Trace;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[
    ("vss", vss),
    ("tper", tper),
    ("dead", dead),
    ("dead_time", dead_time),
    ("ripple", ripple),
    ("ilmean", ilmean),
    ("vgate", vgate),
    ("tau_r", tau_r),
    ("tau_r_ramp", tau_r_ramp),
    ("tau_f", tau_f),
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

/// hs's fall to ls's rise: two ticks, 62.5 ns.
fn dead_time(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![DEAD_TICKS * TICK]))
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

/// `(time, volts)` corners of the `vsup` variant's gate-drive supply, vdrv.
const DRIVE_SUPPLY: [(f64, f64); 5] = [
    (0.0, 5.0),
    (1.0e-3, 5.0),
    (1.1e-3, 3.5),
    (1.3e-3, 3.5),
    (1.4e-3, 5.0),
];
/// The gate drive's strong-1 and strong-0 resistances, and each gate's
/// capacitance (`deck/vsup.tran.cir`).
const R_HIGH: f64 = 10.0;
const R_LOW: f64 = 5.0;
const C_GATE: f64 = 1e-9;
/// hs rises this many ticks into each period; ls falls at the period's end.
const HS_RISE_TICKS: f64 = 2.0;
/// The `p3` variant's authored gate drive (`net/buck_ctrl_p3.vams`): its
/// resistance, and the duration of its `transition()` edge.
const R_AUTHORED: f64 = 20.0;
const AUTHORED_EDGE: f64 = 1e-9;

fn drive_supply_at(time: f64) -> f64 {
    for pair in DRIVE_SUPPLY.windows(2) {
        let ((t0, v0), (t1, v1)) = (pair[0], pair[1]);
        if time <= t1 {
            return v0 + (v1 - v0) * (time - t0) / (t1 - t0);
        }
    }
    DRIVE_SUPPLY[DRIVE_SUPPLY.len() - 1].1
}

/// A held 1 drives V(vdrv). The switch's control draws no current, so the
/// gate is the converter's Thevenin source and Cg alone, and settles at the
/// level exactly.
fn vgate(context: &Context<'_>) -> Result<Expected, String> {
    let times = context
        .quantity
        .measure
        .at
        .clone()
        .ok_or("c06::vgate needs its sample times")?;
    Ok(values(times.into_iter().map(drive_supply_at).collect()))
}

/// The period boundary a gate-edge quantity's window opens on.
fn window_period(context: &Context<'_>) -> Result<f64, String> {
    let [start, _] = context
        .quantity
        .measure
        .window
        .ok_or("a gate edge needs the quantity's window")?;
    let boundary = (start / PERIOD).round() * PERIOD;
    if (start - boundary).abs() > 1e-12 {
        return Err(format!(
            "a gate edge's window opens on a period boundary, not {start:e}"
        ));
    }
    Ok(boundary)
}

/// The gate is first order through the converter's resistance: hs rises two
/// ticks into the period, and v(ghs) passes 1 − 1/e of V(vdrv) one rhi·Cg
/// (10 ns) later.
fn tau_r(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![
        window_period(context)? + HS_RISE_TICKS * TICK + R_HIGH * C_GATE,
    ]))
}

/// The authored gate drive ramps its level over T = 1 ns into τ = 20 Ω·Cg =
/// 20 ns. After the ramp the gate is V(1 − (τ/T)(e^{T/τ} − 1)e^{−t/τ}), so it
/// passes 1 − 1/e of V(vdrv) at t = τ(1 + ln((τ/T)(e^{T/τ} − 1))) =
/// 20.502 ns after hs rises.
fn tau_r_ramp(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![
        window_period(context)? + HS_RISE_TICKS * TICK + ramp_crossing(),
    ]))
}

/// The authored gate's 1 − 1/e crossing after its edge starts.
fn ramp_crossing() -> f64 {
    let tau = R_AUTHORED * C_GATE;
    tau * (1.0 + ((tau / AUTHORED_EDGE) * ((AUTHORED_EDGE / tau).exp() - 1.0)).ln())
}

/// ls falls at the period's end, and v(gls) passes 1/e of V(vdrv) one rlo·Cg
/// (5 ns) later.
fn tau_f(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![window_period(context)? + R_LOW * C_GATE]))
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

    #[test]
    fn the_gate_drive_supply_is_flat_where_the_gates_are_sampled() {
        for (time, volts) in [(0.9e-3, 5.0), (1.05e-3, 4.25), (1.2e-3, 3.5), (1.5e-3, 5.0)] {
            assert!((drive_supply_at(time) - volts).abs() < 1e-12, "{time}");
        }
        assert!((R_HIGH * C_GATE - 10e-9).abs() < 1e-21);
        assert!((R_LOW * C_GATE - 5e-9).abs() < 1e-21);
        assert!((HS_RISE_TICKS * TICK - 62.5e-9).abs() < 1e-21);
        // 20(1 + ln(20(e^0.05 − 1))) ns.
        assert!(
            (ramp_crossing() - 20.502_083e-9).abs() < 1e-15,
            "{}",
            ramp_crossing()
        );
    }
}
