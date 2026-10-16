//! P0.6 probes (`tests/mixed-corpus/probes/`, design §4): three questions the
//! plan could not answer by reading code, each a small design with the same
//! schema and a decisive assertion.
//!
//! - (a) `p06a-trnoise-module`: does transient noise include a Verilog-A noise
//!   source in a mixed module, in its analog-only twin, and in a plain
//!   resistor (the control)? Sample standard deviations against χ² intervals.
//! - (b) `p06b-mc-step-mixed`: do MC and `.step` of a transient rebuild the
//!   mixed design per point (initial blocks re-run, no state leak)? First `q`
//!   rise at 30 ns and `i(vp) = −(1 mA + 1/r_k)` per point and trial.
//! - (c) `p06c-dd-mixed`: is a read-before-assign variable retained across
//!   accepted points on both routes? `V(o)[k] = V(a)[k−1]`.
//! - (d) `p06d-shaped-noise`: does transient noise render noise a module
//!   shapes through `laplace_np` at `|H|²·S`? A sample standard deviation
//!   against an interval from the shaped process's own correlation.
//!
//! Interval and per-trial assertions use the ordinary quantity kinds; the
//! one-step delay is a probe assertion (`kind = "assert"`) evaluated here.

use super::analytic::{Context, Registry, values};
use super::case::Observation;
use super::compare::{Expected, Verdict};
use super::design::Quantity;
use super::measure::real_series;

pub const REGISTRY: Registry = &[("first_rise", first_rise), ("ivp", ivp)];

/// Probe assertions evaluated by [`assert`] (`kind = "assert"`).
pub const ASSERTIONS: [&str; 1] = ["p06c::one_step"];

/// `cnt` counts on a 10 ns process and raises `q` when the count is 3.
fn first_rise(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![30e-9]))
}

/// `i(vp) = −(V/1 kΩ + V/r_k)` with V = 1 V: the module's 1 kΩ branch and the
/// trial's own `rp`. The trial records the `r` it drew as scalar `param:r`.
fn ivp(context: &Context<'_>) -> Result<Expected, String> {
    let r = context
        .rspice
        .scalars
        .get("param:r")
        .copied()
        .ok_or("the trial did not record its drawn parameter r")?;
    Ok(values(vec![-(1e-3 + 1.0 / r)]))
}

/// Evaluate a probe assertion (`kind = "assert"`).
pub fn assert(quantity: &Quantity, observation: &Observation) -> Result<Verdict, String> {
    match quantity.expect.as_deref() {
        Some("p06c::one_step") => one_step(quantity, observation),
        other => Err(format!("unknown probe assertion {other:?}")),
    }
}

/// For every accepted point `k ≥ 1`, `signals[0][k] = signals[1][k−1]` within
/// `abs`: the analog block contributes the value it held from the previous
/// accepted point.
fn one_step(quantity: &Quantity, observation: &Observation) -> Result<Verdict, String> {
    let signals = quantity
        .measure
        .signals
        .as_ref()
        .filter(|signals| signals.len() == 2)
        .ok_or("p06c::one_step needs signals = [output, input]")?;
    let output = real_series(observation, &signals[0]).map_err(|error| error.to_string())?;
    let input = real_series(observation, &signals[1]).map_err(|error| error.to_string())?;
    if output.len() != input.len() || output.len() < 2 {
        return Err("one_step needs two aligned series of at least two points".into());
    }
    let tolerance = quantity.abs.unwrap_or(0.0);
    let mut worst = (0usize, 0.0f64);
    for k in 1..output.len() {
        let diff = (output[k] - input[k - 1]).abs();
        if diff > worst.1 {
            worst = (k, diff);
        }
    }
    let (k, diff) = worst;
    let time = observation.axis.get(k).copied().unwrap_or(f64::NAN);
    let detail = format!(
        "worst k={k} (t={time:.4e}): |{}[k] − {}[k−1]|={diff:.3e} allowed {tolerance:.1e} over {} points",
        signals[0],
        signals[1],
        output.len()
    );
    Ok(Verdict {
        pass: diff <= tolerance,
        severity: if tolerance > 0.0 {
            diff / tolerance
        } else {
            f64::INFINITY
        },
        detail,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::mixed::design::MeasureSpec;

    fn quantity(abs: f64) -> Quantity {
        Quantity {
            id: "delay".into(),
            analysis: "tran".into(),
            variants: vec!["mixed".into()],
            oracle: "O6".into(),
            measure: MeasureSpec {
                kind: "assert".into(),
                signals: Some(vec!["v(o)".into(), "v(a)".into()]),
                ..Default::default()
            },
            of: None,
            stat: None,
            over: None,
            interval: None,
            ratio: None,
            trials: None,
            expect: Some("p06c::one_step".into()),
            reference: None,
            abs: Some(abs),
            rel: None,
            abs_db: None,
            abs_deg: None,
            edge_abs: None,
            why: "w".into(),
        }
    }

    #[test]
    fn one_step_delay_passes_a_delayed_copy_and_fails_a_live_one() {
        let mut observation = Observation {
            axis: vec![0.0, 1.0, 2.0, 3.0],
            ..Default::default()
        };
        observation
            .real
            .insert("v(a)".into(), vec![0.0, 0.1, 0.2, 0.3]);
        observation
            .real
            .insert("v(o)".into(), vec![0.0, 0.0, 0.1, 0.2]);
        assert!(assert(&quantity(1e-9), &observation).unwrap().pass);
        observation
            .real
            .insert("v(o)".into(), vec![0.0, 0.1, 0.2, 0.3]);
        let verdict = assert(&quantity(1e-9), &observation).unwrap();
        assert!(!verdict.pass, "{verdict:?}");
    }
}
