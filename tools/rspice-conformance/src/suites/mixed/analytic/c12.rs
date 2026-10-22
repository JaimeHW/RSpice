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

pub const REGISTRY: Registry = &[("ok", ok), ("okt", okt), ("errors", errors)];

/// The verdict: 0 from the initial block, 1 at 1600 µs.
const VERDICT_TIME: f64 = 1600e-6;

/// Variant `fail`'s narrowed step window, in volts.
#[cfg(test)]
const FAIL_WINDOW: [f64; 2] = [5e-3, 7e-3];

/// When variant `fail` flags code k: its reading, 92.5 µs after the 6.5 µs
/// load that starts its 100 µs slot, at k·100 µs + 99 µs. Every step lies
/// above the narrowed window (the test below derives it from the O5-O1
/// references), so codes 1..15 are each flagged once.
fn errors(_: &Context<'_>) -> Result<Expected, String> {
    Ok(super::values(
        (1..16)
            .map(|code| f64::from(code) * 100e-6 + 99e-6)
            .collect(),
    ))
}

/// When `ok` rises: 16 trim steps of 100 µs, 1600 µs.
fn okt(_: &Context<'_>) -> Result<Expected, String> {
    Ok(super::values(vec![VERDICT_TIME]))
}

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

    /// Variant `fail` flags exactly the codes whose O5-O1 step lies outside
    /// its window, and every step is at least 1 mV from either limit, so the
    /// record set does not hang on a reading's last millivolt.
    #[test]
    fn the_fail_window_flags_every_code_by_the_o5_o1_references() {
        let refs = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/mixed-corpus/c12-ldo-tb/ref");
        let vout = |code: usize| -> f64 {
            let path = refs.join(format!("c{code}.op.O5-O1.out"));
            let text = std::fs::read_to_string(&path).expect("the reference is captured");
            let mut lines = text.lines().filter(|line| !line.starts_with('#'));
            let header: Vec<&str> = lines.next().expect("a header").split_whitespace().collect();
            let column = header
                .iter()
                .position(|name| *name == "v(vout)")
                .expect("v(vout)");
            let row: Vec<&str> = lines.next().expect("a row").split_whitespace().collect();
            row[column].parse().expect("a number")
        };
        let [low, high] = super::FAIL_WINDOW;
        let mut flagged = Vec::new();
        for code in 1..16 {
            let step = vout(code) - vout(code - 1);
            assert!(
                (step - low).abs() >= 1e-3 && (step - high).abs() >= 1e-3,
                "code {code}: step {step} is within 1 mV of a limit"
            );
            if !(low..=high).contains(&step) {
                flagged.push(code);
            }
        }
        assert_eq!(flagged, (1..16).collect::<Vec<_>>());
    }
}
