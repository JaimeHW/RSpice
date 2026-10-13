//! C11 — switched-capacitor low-pass with a Verilog non-overlapping clock
//! (design §3 C11).
//!
//! T = 1 µs; ph1 high over [20, 480] ns and ph2 over [520, 980] ns of each
//! period. C1 = 1 pF samples the input on ph1 and shares its charge with
//! C2 = 9 pF on ph2, so the value held after period k is
//! y[k] = a·x[k] + (1 − a)·y[k−1], a = C1/(C1 + C2) = 0.1, with x[k] the input
//! at the ph1 fall of the same period. The operating point holds both
//! capacitors at the input's 0.5 V (both switches open, ROFF to the input).

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[("rec", rec), ("pac", pac), ("fft", fft), ("pn", pn)];

const PERIOD: f64 = 1e-6;
const PH1_FALL: f64 = 480e-9;
const PH2_RISE: f64 = 520e-9;
const PH2_FALL: f64 = 980e-9;
const SHARE: f64 = 1e-12 / (1e-12 + 9e-12);
const INPUT_OFFSET: f64 = 0.5;
const INPUT_AMPLITUDE: f64 = 0.2;
const INPUT_FREQUENCY: f64 = 10e3;

fn input(time: f64) -> f64 {
    INPUT_OFFSET + INPUT_AMPLITUDE * (2.0 * std::f64::consts::PI * INPUT_FREQUENCY * time).sin()
}

/// y[k] after the ph2 of period k, from y[−1] = 0.5 V; each grid time reads
/// the value of the last ph2 that ended at or before it.
fn rec(context: &Context<'_>) -> Result<Expected, String> {
    let grid = context
        .quantity
        .measure
        .times
        .ok_or("c11::rec needs the sample time grid")?;
    let times = grid.times();
    let last = times
        .iter()
        .map(|time| ((time - PH2_FALL) / PERIOD).floor())
        .fold(-1.0f64, f64::max);
    let mut held = Vec::with_capacity(last.max(0.0) as usize + 1);
    let mut y = INPUT_OFFSET;
    for period in 0..=(last.max(-1.0) as i64) {
        let x = input(period as f64 * PERIOD + PH1_FALL);
        y = SHARE * x + (1.0 - SHARE) * y;
        held.push(y);
    }
    let expected = times
        .into_iter()
        .map(|time| {
            let period = ((time - PH2_FALL) / PERIOD).floor();
            if period < 0.0 {
                INPUT_OFFSET
            } else {
                held[period as usize]
            }
        })
        .collect();
    Ok(values(expected))
}

/// Sideband 0 of v(out) per unit input phasor: the recursion's transfer at
/// z = e^{jωT}, the one-period hold of the output (sinc(πfT)·e^{−jωT/2}), and
/// the 40 ns from the input sample (ph1 fall, 0.48 T) to the output update
/// (ph2 rise, 0.52 T): a/(1 − (1−a)e^{−jωT}) · sinc(πfT) · e^{−jω·0.54 T}.
fn pac(context: &Context<'_>) -> Result<Expected, String> {
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| {
            let omega = 2.0 * std::f64::consts::PI * frequency;
            let z_inverse = Complex64::from_polar(1.0, -omega * PERIOD);
            let recursion = SHARE / (1.0 - (1.0 - SHARE) * z_inverse);
            let x = std::f64::consts::PI * frequency * PERIOD;
            let sinc = if x == 0.0 { 1.0 } else { x.sin() / x };
            let delay = PERIOD / 2.0 + (PH2_RISE - PH1_FALL);
            recursion * sinc * Complex64::from_polar(1.0, -omega * delay)
        })
        .collect();
    Ok(complex(expected))
}

/// The amplitude of each frequency in the held output once the recursion's
/// start has died away: the input amplitude times |sideband 0| (the design's
/// 0.2·0.85903·0.999836 = 0.17178 V at 10 kHz).
fn fft(context: &Context<'_>) -> Result<Expected, String> {
    let Expected::Measured(Measured::Complex(sideband)) = pac(context)? else {
        return Err("c11::fft: sideband 0 is complex".into());
    };
    Ok(values(
        sideband
            .iter()
            .map(|value| INPUT_AMPLITUDE * value.norm())
            .collect(),
    ))
}

/// The sampled output's noise power. Each ph1 leaves kT/C1 on C1, of which
/// the share hands C2 a²; the ph2 switch's own noise across C1 and C2 in
/// series leaves a²·kT/(C1C2/(C1 + C2)) on C2; (1 − a)² of C2's previous
/// noise survives. The stationary variance a²kT(1/C1 + (C1 + C2)/(C1C2)) /
/// (1 − (1 − a)²) is exactly kT/C2 (design: 4.60e-10 V² at 27 °C).
fn pn(_: &Context<'_>) -> Result<Expected, String> {
    const BOLTZMANN: f64 = 1.380_649e-23;
    const KELVIN: f64 = 300.15;
    const C2: f64 = 9e-12;
    Ok(values(vec![BOLTZMANN * KELVIN / C2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_domain_constants_match_the_design() {
        // |H(10 kHz)| = 0.85903 for a = 0.1 (design §3 C11).
        let omega_t = 2.0 * std::f64::consts::PI * 10e3 * PERIOD;
        let h = SHARE / (1.0 - (1.0 - SHARE) * Complex64::from_polar(1.0, -omega_t));
        assert!((h.norm() - 0.85903).abs() < 2e-5, "{}", h.norm());
        // The recursion settles to the input's mean for a constant input.
        let mut y = 0.0;
        for _ in 0..400 {
            y = SHARE * 0.5 + (1.0 - SHARE) * y;
        }
        assert!((y - 0.5).abs() < 1e-12);
    }
}
