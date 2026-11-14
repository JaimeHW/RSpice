//! C11 — switched-capacitor low-pass with a Verilog non-overlapping clock
//! (design §3 C11).
//!
//! T = 1 µs; ph1 high over [20, 480] ns and ph2 over [520, 980] ns of each
//! period. C1 = 1 pF samples the input on ph1 and shares its charge with
//! C2 = 9 pF on ph2, so the value held after period k is
//! y[k] = a·x[k] + (1 − a)·y[k−1], a = C1/(C1 + C2) = 0.1, with x[k] the input
//! at the ph1 fall of the same period. The operating point holds both
//! capacitors at the input's 0.5 V (both switches open, ROFF to the input).

use std::collections::BTreeMap;

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::events::Trace;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[
    ("rec", rec),
    ("pac", pac),
    ("fft", fft),
    ("pn", pn),
    ("proc", proc),
    ("held", held),
    ("track", track),
    ("pac_lsb", pac_lsb),
    ("pn_band", pn_band),
    ("pn_psd", pn_psd),
];

const PERIOD: f64 = 1e-6;
const PH1_RISE: f64 = 20e-9;
const PH1_FALL: f64 = 480e-9;
const PH2_RISE: f64 = 520e-9;
const PH2_FALL: f64 = 980e-9;
const SHARE: f64 = 1e-12 / (1e-12 + 9e-12);
const INPUT_OFFSET: f64 = 0.5;
const INPUT_AMPLITUDE: f64 = 0.2;
const INPUT_FREQUENCY: f64 = 10e3;

// The analog-clock twin (variant `pulse`, net/sc_pulse.cir). Each PULSE edge
// takes 1 ns between 0 and vcc = 1.8 V; a switch opens below 0.8 V and closes
// above 1.0 V, so S1 opens (the sample) 1/1.8 ns after the ph1 fall edge and
// S2 closes (the update) 1/1.8 ns after the ph2 rise edge.
const TWIN_SAMPLE: f64 = PH1_FALL + 1e-9 / 1.8;
const TWIN_UPDATE: f64 = PH2_RISE + 1e-9 / 1.8;
const TWIN_PH1_CLOSE: f64 = PH1_RISE + 1e-9 / 1.8;
/// Ron·C1: C1 follows the input through the closed S1.
const TWIN_TRACK_TAU: f64 = 1e3 * 1e-12;
/// The periodic decks' input: 0.5 + 0.2·sin(ω₀t − 83°) at the 1 MHz clock,
/// whose peak falls at the sample.
const TWIN_CARRIER_PHASE_DEG: f64 = -83.0;
const BOLTZMANN: f64 = 1.380_649e-23;
/// The corpus runs at 27 °C.
const KELVIN: f64 = 300.15;
const C2: f64 = 9e-12;

/// Variant `proc`: the `d_process` toggle peer, clocked by ph1, answers every
/// ph1 rise with the complement of its last answer (1 first), and `d` takes
/// each answer `clk_delay` = 1 ns later. Before the first rise `d` is the
/// unknown, high-impedance value the code model starts its outputs at.
fn proc(context: &Context<'_>) -> Result<Expected, String> {
    const CLK_DELAY: f64 = 1e-9;
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or("c11::proc needs the quantity's window")?;
    let mut trace: Trace = Vec::new();
    if start <= 0.0 {
        trace.push((0.0, "z".to_owned()));
    }
    for edge in 0u32.. {
        let time = f64::from(edge) * PERIOD + PH1_RISE + CLK_DELAY;
        if time > stop {
            break;
        }
        if time >= start {
            trace.push((time, if edge % 2 == 0 { "1" } else { "0" }.to_owned()));
        }
    }
    Ok(Expected::Measured(Measured::Events(BTreeMap::from([(
        "d".to_owned(),
        trace,
    )]))))
}

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
    Ok(values(vec![BOLTZMANN * KELVIN / C2]))
}

/// C1's voltage while S1 is closed, once the switch's 1 ns transient has
/// gone: the carrier through the first-order Ron·C1 low-pass.
fn twin_tracking(time: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI / PERIOD;
    let lag = (omega * TWIN_TRACK_TAU).atan();
    let gain = 1.0 / (1.0 + (omega * TWIN_TRACK_TAU).powi(2)).sqrt();
    INPUT_OFFSET
        + INPUT_AMPLITUDE * gain * (omega * time + TWIN_CARRIER_PHASE_DEG.to_radians() - lag).sin()
}

/// The value the twin's periodic steady state holds: the sample C1 takes
/// when S1 opens. The carrier peaks there, so C1, lagging it by
/// atan(ω₀·Ron·C1), holds 0.5 + 0.2/(1 + (ω₀·Ron·C1)²); every period samples
/// it, and y = 0.1·x + 0.9·y holds it on C2.
fn twin_held() -> f64 {
    twin_tracking(TWIN_SAMPLE)
}

/// Variant `pulse` pss: the held output, constant over the period.
fn held(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![twin_held()]))
}

/// Variant `pulse` pss: v(a) at times in absolute time mod T. While S1 is
/// closed (and its closing transient has gone, 10 Ron·C1) C1 follows the
/// carrier; from the sample to S1's next closing it holds the sample, which
/// C2 already holds, so sharing it on ph2 changes neither.
fn track(context: &Context<'_>) -> Result<Expected, String> {
    let times = context
        .quantity
        .measure
        .at
        .clone()
        .ok_or("c11::track needs the quantity's sample times")?;
    let expected = times
        .into_iter()
        .map(|time| {
            let phase = time.rem_euclid(PERIOD);
            if phase > TWIN_PH1_CLOSE + 10.0 * TWIN_TRACK_TAU && phase < TWIN_SAMPLE {
                Ok(twin_tracking(phase))
            } else if !(TWIN_PH1_CLOSE..=TWIN_SAMPLE).contains(&phase) {
                Ok(twin_held())
            } else {
                Err(format!(
                    "c11::track: {time:e} s is inside S1's closing transient"
                ))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(values(expected))
}

/// Variant `pulse` pac, sideband −1 (the output at f − f₀ per unit input at
/// f), design-p7 §5.1: with the sample at t_s, the hold from the update t_u
/// for one period and H_d(ω) = a/(1 − (1 − a)e^{−jωT}),
/// Y_m = H_d(ω)·e^{jω(t_s − t_u)}·e^{−jmω₀t_u}·(1 − e^{−jωT})/(j(ω + mω₀)T).
/// Ron·C1 and Ron·C1C2/(C1 + C2) (about 1 ns each) are neglected: ≤ 0.04°
/// at 100 kHz.
fn pac_lsb(context: &Context<'_>) -> Result<Expected, String> {
    let sideband = -1.0;
    let omega_0 = 2.0 * std::f64::consts::PI / PERIOD;
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| {
            let omega = 2.0 * std::f64::consts::PI * frequency;
            let z_inverse = Complex64::from_polar(1.0, -omega * PERIOD);
            let recursion = SHARE / (1.0 - (1.0 - SHARE) * z_inverse);
            let delay = Complex64::from_polar(1.0, omega * (TWIN_SAMPLE - TWIN_UPDATE));
            let update = Complex64::from_polar(1.0, -sideband * omega_0 * TWIN_UPDATE);
            let hold = (1.0 - z_inverse) / (Complex64::i() * (omega + sideband * omega_0) * PERIOD);
            recursion * delay * update * hold
        })
        .collect();
    Ok(complex(expected))
}

/// The one-sided spectrum of the twin's output sample sequence on
/// [0, f₀/2], normalized to integrate to the sample variance (design-p7
/// §5.1): the samples are AR(1), y_k = 0.9·y_{k−1} + e_k, with innovation
/// variance 0.19·kT/C2, so S_y(f) = (2/f₀)·0.19·(kT/C2)/|1 − 0.9e^{−j2πf/f₀}|².
fn twin_sample_psd(frequency: f64) -> f64 {
    let pole = 1.0 - SHARE;
    let innovation = (1.0 - pole * pole) * BOLTZMANN * KELVIN / C2;
    let denominator = (1.0
        - pole * Complex64::from_polar(1.0, -2.0 * std::f64::consts::PI * frequency * PERIOD))
    .norm_sqr();
    2.0 * PERIOD * innovation / denominator
}

/// Variant `pulse` pnoise: S_y integrated over the quantity's window, in
/// closed form: ∫ S_y df from f₁ to f₂ ≤ f₀/2 is
/// (kT/C2)·(2/π)·[atan(((1 + b)/(1 − b))·tan(πfT))] with b = 0.9, so the
/// whole of [0, f₀/2] is kT/C2 (4.6045e-10 V²) and [100 Hz, f₀/2] is
/// 4.5870e-10 V².
fn pn_band(context: &Context<'_>) -> Result<Expected, String> {
    let [low, high] = context
        .quantity
        .measure
        .window
        .ok_or("c11::pn_band needs the quantity's window")?;
    if !(0.0..=0.5 / PERIOD).contains(&low) || !(low..=0.5 / PERIOD).contains(&high) {
        return Err(format!(
            "c11::pn_band: [{low:e}, {high:e}] Hz is not inside [0, f0/2]"
        ));
    }
    let pole = 1.0 - SHARE;
    let primitive = |frequency: f64| {
        let half_angle = std::f64::consts::PI * frequency * PERIOD;
        if (half_angle - std::f64::consts::FRAC_PI_2).abs() < 1e-15 {
            std::f64::consts::FRAC_PI_2
        } else {
            ((1.0 + pole) / (1.0 - pole) * half_angle.tan()).atan()
        }
    };
    let variance = BOLTZMANN * KELVIN / C2;
    Ok(values(vec![
        variance * std::f64::consts::FRAC_2_PI * (primitive(high) - primitive(low)),
    ]))
}

/// Variant `pulse` pnoise: S_y at the quantity's points.
fn pn_psd(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(twin_sample_psd)
            .collect(),
    ))
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

    #[test]
    fn the_twin_constants_match_the_design() {
        // design-p7 §5.1 at 10 kHz: |Y₋₁| = 0.0086755.
        let lower = {
            let omega = 2.0 * std::f64::consts::PI * 10e3;
            let omega_0 = 2.0 * std::f64::consts::PI / PERIOD;
            let z_inverse = Complex64::from_polar(1.0, -omega * PERIOD);
            SHARE / (1.0 - (1.0 - SHARE) * z_inverse)
                * Complex64::from_polar(1.0, omega * (TWIN_SAMPLE - TWIN_UPDATE))
                * (1.0 - z_inverse)
                / (Complex64::i() * (omega - omega_0) * PERIOD)
        };
        assert!((lower.norm() - 0.0086755).abs() < 5e-8, "{}", lower.norm());
        // S_y(100 Hz) = 1.7496e-14 V²/Hz; [100 Hz, f₀/2] holds 4.587e-10 V².
        assert!((twin_sample_psd(100.0) / 1.7496e-14 - 1.0).abs() < 1e-4);
        // The closed-form band integral equals a fine quadrature of S_y.
        let (low, high) = (100.0, 0.5 / PERIOD);
        let steps = 200_000;
        let width = (high - low) / f64::from(steps);
        let quadrature: f64 = (0..steps)
            .map(|step| twin_sample_psd(low + (f64::from(step) + 0.5) * width) * width)
            .sum();
        let pole: f64 = 1.0 - SHARE;
        let closed = BOLTZMANN * KELVIN / C2
            * std::f64::consts::FRAC_2_PI
            * (std::f64::consts::FRAC_PI_2
                - ((1.0 + pole) / (1.0 - pole) * (std::f64::consts::PI * low * PERIOD).tan())
                    .atan());
        assert!(
            (quadrature / closed - 1.0).abs() < 1e-6,
            "{quadrature} vs {closed}"
        );
        assert!((closed / 4.587e-10 - 1.0).abs() < 1e-3, "{closed}");
        // The held value: 0.5 + 0.2/(1 + (2π·1e6·1e-9)²).
        assert!(
            (twin_held() - 0.699_992_104_6).abs() < 1e-9,
            "{}",
            twin_held()
        );
    }
}
