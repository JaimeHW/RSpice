//! C12 — Verilog-AMS testbench over the C5 LDO (design §3 C12).
//!
//! The testbench loads trim codes 0..15 every 100 µs and raises `ok` at
//! 1600 µs when every step of its readings lies in [5 mV, 20 mV]. The ladder
//! sets vout ≈ vref·(1 + Rtop/Rbot), Rbot = 60k + (15 − k)·1k, so the steps
//! grow from 9.0 to 13.7 mV and the check passes.

use std::collections::BTreeMap;

use super::{Context, Registry};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[("ok", ok)];

/// The verdict: 0 from the initial block, 1 at 1600 µs.
const VERDICT_TIME: f64 = 1600e-6;

fn ok(context: &Context<'_>) -> Result<Expected, String> {
    let nets = context
        .quantity
        .measure
        .nets
        .as_ref()
        .and_then(|nets| nets.for_variant(context.variant))
        .ok_or("c12::ok: the quantity names no net for this variant")?;
    let mut traces = BTreeMap::new();
    for net in nets {
        traces.insert(
            net.to_ascii_lowercase(),
            vec![(0.0, "0".to_owned()), (VERDICT_TIME, "1".to_owned())],
        );
    }
    Ok(Expected::Measured(Measured::Events(traces)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn ideal_trim_steps_lie_inside_the_check() {
        let vout = |code: f64| 0.5 * (1.0 + 100e3 / (60e3 + (15.0 - code) * 1e3));
        for code in 1..16 {
            let step = vout(f64::from(code)) - vout(f64::from(code - 1));
            assert!((5e-3..=20e-3).contains(&step), "{code}: {step}");
        }
    }
}
