//! C1 — first-order continuous-time sigma-delta modulator.
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

/// The clock period.
const CLOCK: f64 = 0.2e-6;

/// Decisions a window's code counts. The divider latches the counter at the
/// clock that ends a window (0.1 µs + m·100 µs) and holds it in reset for the
/// next clock period (d_fdiv's one high cycle, delayed 50 ns), so that clock
/// is not counted: 499 of the window's 500 clocks are.
const COUNTED: f64 = 499.0;

/// `code = (2·499 − 1)·(1/2 + v̄/4)` in expectation. Counter bit 1 (J = dv,
/// K = dvb) holds the decision its clock reads, and bits 2–10 count the
/// clocks at which bit 1 held a one, so the latched code is twice the ones
/// among the window's first 498 counted decisions plus the last one, which
/// counts once. The ones fraction is `1/2 + v̄/4` (±2 V differential
/// feedback against the input mean v̄), so the 499 decisions give
/// `997·(1/2 + v̄/4)`. ngspice-46 (`oracle/xs.tran.ngspice.cir`, its control
/// nets saved) gives this count exactly in all 19 windows. Each counted clock
/// reads the decision of the clock before it (dv changes 2 ns after the
/// counter's clock), so the decisions run from the clock the reset swallows
/// to the clock before the latch, and their feedback spans
/// `[latch + 1 clock, next latch]`. The code read at `t` (150 µs + k·100 µs)
/// was latched at `t − 50 µs + 0.1 µs`, so v̄ is the input mean over
/// `[t − 150 µs + 0.1 µs + 1 clock, t − 50 µs + 0.1 µs]`.
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
            let latch = time - 0.5 * WINDOW + FIRST_CLOCK;
            let start = latch - COUNTED * CLOCK;
            (2.0 * COUNTED - 1.0) * (0.5 + window_mean(start, latch) / 4.0)
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
        // The counted decisions span 99.8 µs, from one clock after a latch
        // to the next latch; a zero input mean gives 997/2.
        assert!((COUNTED * CLOCK - 99.8e-6).abs() < 1e-15);
        assert!(((2.0 * COUNTED - 1.0) * 0.5 - 498.5).abs() < 1e-12);
    }
}
