//! C8 — abstraction ladder (design §3 C8).
//!
//! One unity-gain low-pass, f0 = 10 kHz, Q = 1/√2, driven by `net/tb_c8.vams`:
//! 0 V, a 0.6 V step at 20 µs, then from 200 µs one period of a 2 kHz sine
//! (0.5 ± 0.3 V) updated every microsecond, then 0.5 V from 700 µs. The
//! quantizer is clamp(⌊16·v⌋, 0, 15), sampled at each rise of a 100 kHz clock
//! (5 µs + k·10 µs) and registered, so the code read at a rise is the one
//! sampled at the rise before.
//!
//! Rungs (a) `tr` and (b) `va` are the continuous filter; rung (c) `wr` is
//! its bilinear transform at 1 MHz, updated at 0.5 µs + n·1 µs.

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::measure::{Measured, evaluate};

pub const REGISTRY: Registry = &[("ovs", ovs), ("codes", codes), ("ach", ach)];

const F0: f64 = 1e4;
const STEP_AT: f64 = 20e-6;
const STEP_LEVEL: f64 = 0.6;
const SINE_START: f64 = 200e-6;
const SINE_SAMPLES: usize = 500;
const UPDATE: f64 = 1e-6;
const SINE_COS: f64 = 0.999_921_044_203_816_1;
const SINE_SIN: f64 = 0.012_566_039_883_352_606;
const CLOCK_PERIOD: f64 = 10e-6;
const BQ_B0: f64 = 0.000_944_084_114_395_548_9;
const BQ_A1: f64 = -1.911_226_230_340_913_5;
const BQ_A2: f64 = 0.915_002_566_798_495_7;
const BQ_OFFSET: f64 = 0.5e-6;

fn omega0() -> f64 {
    2.0 * std::f64::consts::PI * F0
}

/// The testbench's staircase: (start time, level) for every segment.
fn stimulus() -> Vec<(f64, f64)> {
    let mut segments = vec![(0.0, 0.0), (STEP_AT, STEP_LEVEL)];
    let (mut s, mut c) = (0.0f64, 1.0f64);
    for n in 0..SINE_SAMPLES {
        segments.push((SINE_START + n as f64 * UPDATE, 0.5 + 0.3 * s));
        let next = s * SINE_COS + c * SINE_SIN;
        c = c * SINE_COS - s * SINE_SIN;
        s = next;
    }
    segments.push((SINE_START + SINE_SAMPLES as f64 * UPDATE, 0.5));
    segments
}

fn level_at(segments: &[(f64, f64)], time: f64) -> f64 {
    segments
        .iter()
        .take_while(|(start, _)| *start <= time)
        .last()
        .map_or(0.0, |(_, level)| *level)
}

/// The continuous filter's output at `time`, propagated exactly through the
/// piecewise-constant stimulus (state x = [v, v']; ζ = 1/√2).
fn continuous_output(segments: &[(f64, f64)], time: f64) -> f64 {
    let w0 = omega0();
    let sigma = w0 / std::f64::consts::SQRT_2;
    let wd = w0 / std::f64::consts::SQRT_2;
    let propagate = |state: [f64; 2], input: f64, dt: f64| -> [f64; 2] {
        let (e, cs, sn) = ((-sigma * dt).exp(), (wd * dt).cos(), (wd * dt).sin());
        let (x0, x1) = (state[0] - input, state[1]);
        [
            input + e * ((cs + sigma / wd * sn) * x0 + sn / wd * x1),
            e * (-(w0 * w0) / wd * sn * x0 + (cs - sigma / wd * sn) * x1),
        ]
    };
    let mut state = [0.0, 0.0];
    let mut at = 0.0;
    let mut input = 0.0;
    for (start, level) in segments {
        if *start > time {
            break;
        }
        state = propagate(state, input, start - at);
        at = *start;
        input = *level;
    }
    propagate(state, input, time - at)[0]
}

/// The bilinear biquad's held output at `time`.
fn discrete_output(segments: &[(f64, f64)], time: f64) -> f64 {
    let (mut x1, mut x2, mut y1, mut y2, mut y0) = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut n = 0usize;
    loop {
        let update = BQ_OFFSET + n as f64 * UPDATE;
        if update > time {
            return y0;
        }
        let x = level_at(segments, update);
        y0 = BQ_B0 * (x + 2.0 * x1 + x2) - BQ_A1 * y1 - BQ_A2 * y2;
        x2 = x1;
        x1 = x;
        y2 = y1;
        y1 = y0;
        n += 1;
    }
}

fn output(variant: &str, segments: &[(f64, f64)], time: f64) -> f64 {
    if variant == "wr" {
        discrete_output(segments, time)
    } else {
        continuous_output(segments, time)
    }
}

/// −(peak after the step): 0.6·(1 + e^−π) for the continuous filter; the
/// biquad's largest held sample for `wr`.
fn ovs(context: &Context<'_>) -> Result<Expected, String> {
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or("c08::ovs needs the quantity's window")?;
    let segments = stimulus();
    let peak = if context.variant == "wr" {
        let first = ((start - BQ_OFFSET) / UPDATE).ceil().max(0.0) as usize;
        let last = ((stop - BQ_OFFSET) / UPDATE).floor() as usize;
        (first..=last)
            .map(|n| discrete_output(&segments, BQ_OFFSET + n as f64 * UPDATE))
            .fold(f64::NEG_INFINITY, f64::max)
    } else {
        STEP_LEVEL * (1.0 + (-std::f64::consts::PI).exp())
    };
    Ok(values(vec![-peak]))
}

/// Decision guard per rung, in LSB: a sample whose ideal value lies this
/// close to a threshold may take either neighbouring code.
fn guard(variant: &str) -> f64 {
    match variant {
        // Op-amp follower error (≈ 3 mV at 0.6 V, gate leakage through
        // R1 + R2) and comparator offsets.
        "tr" => 1.0 / 8.0,
        _ => 1.0 / 32.0,
    }
}

/// One code per strobe rise RSpice delivered: the code of the sample taken
/// at the rise before (0 before the first). Within the rung's guard of a
/// threshold, the code RSpice reports is accepted when it is one of the two
/// codes that meet there.
fn codes(context: &Context<'_>) -> Result<Expected, String> {
    let spec = &context.quantity.measure;
    let strobe = spec
        .strobe
        .as_ref()
        .ok_or("c08::codes needs the quantity's strobe")?
        .to_ascii_lowercase();
    let trace = context
        .rspice
        .events
        .get(&strobe)
        .ok_or_else(|| format!("c08::codes: strobe {strobe} is absent"))?;
    let rises: Vec<f64> = trace
        .windows(2)
        .filter(|pair| pair[0].1 != "1" && pair[1].1 == "1")
        .map(|pair| pair[1].0)
        .collect();
    let reported = match evaluate(spec, context.variant, context.rspice) {
        Ok(Measured::Values(values)) => values,
        _ => Vec::new(),
    };
    let segments = stimulus();
    let band = guard(context.variant);
    let expected = rises
        .iter()
        .enumerate()
        .map(|(index, rise)| {
            if index == 0 || rise - rises[index - 1] > 1.5 * CLOCK_PERIOD {
                return 0.0;
            }
            let scaled = 16.0 * output(context.variant, &segments, rises[index - 1]);
            let code = scaled.floor().clamp(0.0, 15.0);
            let nearest = scaled.round();
            let ambiguous = (scaled - nearest).abs() < band && (1.0..=15.0).contains(&nearest);
            match reported.get(index) {
                Some(seen) if ambiguous && (*seen == nearest || *seen == nearest - 1.0) => *seen,
                _ => code,
            }
        })
        .collect();
    Ok(values(expected))
}

/// H(jω) = 1/(1 − (f/f0)² + j·√2·f/f0).
fn ach(context: &Context<'_>) -> Result<Expected, String> {
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| {
            let ratio = frequency / F0;
            Complex64::new(1.0, 0.0)
                / Complex64::new(1.0 - ratio * ratio, std::f64::consts::SQRT_2 * ratio)
        })
        .collect();
    Ok(complex(expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_overshoot_and_settling_match_the_design() {
        let segments = stimulus();
        // Peak 4.32 % over 0.6 V at 70.7 µs after the step.
        let peak = continuous_output(&segments, STEP_AT + 70.71e-6);
        assert!(
            (peak - STEP_LEVEL * (1.0 + (-std::f64::consts::PI).exp())).abs() < 1e-6,
            "{peak}"
        );
        // 170 µs after the step the ringing envelope is √2·e^(−ω0·170µs/√2)
        // of the step (σ = ωd at ζ = 1/√2).
        let residual = std::f64::consts::SQRT_2
            * STEP_LEVEL
            * (-std::f64::consts::FRAC_1_SQRT_2 * omega0() * 170e-6).exp();
        let settled = continuous_output(&segments, 190e-6);
        assert!((settled - 0.6).abs() < residual, "{settled}");
        // The bilinear biquad has unity DC gain and the same overshoot to 0.1 %.
        let held = discrete_output(&segments, 190e-6);
        assert!((held - 0.6).abs() < 1.1 * residual, "{held}");
        let discrete_peak = (40..150)
            .map(|n| discrete_output(&segments, BQ_OFFSET + n as f64 * UPDATE))
            .fold(f64::NEG_INFINITY, f64::max);
        assert!((discrete_peak - peak).abs() < 1e-3, "{discrete_peak}");
        // |H(2 kHz)| = 0.99920, −16.42°.
        let h =
            Complex64::new(1.0, 0.0) / Complex64::new(1.0 - 0.04, std::f64::consts::SQRT_2 * 0.2);
        assert!((h.norm() - 0.99920).abs() < 1e-5);
        assert!((h.arg().to_degrees() + 16.42).abs() < 0.01);
    }
}
