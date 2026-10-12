//! C1 — first-order continuous-time sigma-delta modulator (design §3 C1).
//!
//! Circuit constants from `net/mod1-ct.cir` and `net/converter.cir`: input
//! 0.5 V at 500 Hz, clock 5 MHz (first edge 0.1 µs), decimation 500 clocks
//! (100 µs windows), Ri = Rf = 500 Ω, Cint = 1 nF, integrator gain 1e5,
//! feedback DAC ±1 V per leg (±2 V differential).

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("code", code),
    ("lattice", lattice),
    ("opdiff", opdiff),
    ("acint", acint),
];

const INPUT_AMPLITUDE: f64 = 0.5;
const INPUT_FREQUENCY: f64 = 500.0;
const FIRST_CLOCK: f64 = 0.1e-6;
const WINDOW: f64 = 100e-6;
const GAIN: f64 = 1e5;
const RI: f64 = 500.0;
const RF: f64 = 500.0;
const CINT: f64 = 1e-9;

/// Mean of the differential input over `[a, b]`.
fn window_mean(a: f64, b: f64) -> f64 {
    let w = 2.0 * std::f64::consts::PI * INPUT_FREQUENCY;
    INPUT_AMPLITUDE / (w * (b - a)) * ((w * a).cos() - (w * b).cos())
}

/// `code_k = 500 + 250·v̄_{k−1}`: ones = 250 + 125·v̄ for ±2 V feedback over
/// 500 clocks, code = 2·ones. The code read at `t` was latched from the window
/// that closed at the latch before `t`; at the design's sample times
/// (150 µs + k·100 µs) that window is `[t − 150 µs + 0.1 µs, +100 µs]`.
fn code(context: &Context<'_>) -> Result<Expected, String> {
    let grid = context
        .quantity
        .measure
        .times
        .ok_or("c01::code needs the sample time grid")?;
    let expected = grid
        .times()
        .into_iter()
        .map(|time| {
            let start = time - 1.5 * WINDOW + FIRST_CLOCK;
            500.0 + 250.0 * window_mean(start, start + WINDOW)
        })
        .collect();
    Ok(values(expected))
}

/// The modulator output changes only on the clock lattice (asserted by the
/// quantity's `lattice`); there is no value expectation beyond it.
fn lattice(_: &Context<'_>) -> Result<Expected, String> {
    Ok(Expected::Structural)
}

/// At vin = 0 with the flip-flop at ic = 0 (adffq = −1, adffqb = +1) and the
/// capacitors open, the integrator sees half the ±2 V feedback difference.
fn opdiff(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![GAIN * (0.0 + 2.0) / 2.0]))
}

/// DAC held, open loop: `vo/vin = 1/((1+Ri/Rf)/A + s·Ri·Cint·(1+1/A))`.
fn acint(context: &Context<'_>) -> Result<Expected, String> {
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| {
            let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
            let denominator =
                Complex64::new((1.0 + RI / RF) / GAIN, 0.0) + s * RI * CINT * (1.0 + 1.0 / GAIN);
            Complex64::new(1.0, 0.0) / denominator
        })
        .collect();
    Ok(complex(expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_form_constants_match_the_design() {
        // DC gain 5e4, pole 6.366 Hz, unity 318.3 kHz.
        let dc = 1.0 / ((1.0 + RI / RF) / GAIN);
        assert!((dc - 5e4).abs() < 1e-6);
        let pole = 2.0 / (GAIN * RI * CINT * (1.0 + 1.0 / GAIN)) / (2.0 * std::f64::consts::PI);
        assert!((pole - 6.366).abs() < 1e-3, "{pole}");
        let unity = 1.0 / (2.0 * std::f64::consts::PI * RI * CINT);
        assert!((unity - 318.3e3).abs() < 100.0, "{unity}");
        // A full-scale positive window gives the top code.
        assert!((500.0_f64 + 250.0 * 0.5 - 625.0).abs() < 1e-12);
        assert!(window_mean(0.0, 2e-3).abs() < 1e-12);
    }
}
