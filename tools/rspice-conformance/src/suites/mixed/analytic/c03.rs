//! C3 — ngspice `d_cosim` examples run natively (design §3 C3).
//!
//! Circuit constants from the upstream `555.shared`, `adc.shared`, `delay.v`
//! and `pwm.cir` (ngspice-46 `examples/xspice/verilator`).

use std::collections::BTreeMap;

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;
use crate::suites::mixed::events::Trace;
use crate::suites::mixed::measure::Measured;

pub const REGISTRY: Registry = &[
    ("t555", t555),
    ("johnson", johnson),
    ("adccode", adccode),
    ("adcop", adcop),
    ("delayop", delayop),
    ("pwmsine", pwmsine),
    ("pwmop", pwmop),
];

// 555: supply, timing network and the default-model discharge transistor.
const VCC_555: f64 = 12.0;
const R_CHARGE: f64 = 20e3;
const R_DISCHARGE: f64 = 10e3;
const C_TIMING: f64 = 1e-6;
/// Saturation voltage of `.model npn_transistor npn` (BF = 100, BR = 1) while
/// it discharges the timing capacitor: Vce,sat = Vt·ln[(1 + (β+1)/BR)/(1 − β/BF)]
/// with forced β = Ic/Ib ≈ (1.2 mA from r1 + 0.6 mA from the capacitor at
/// mid-swing)/(11.25 V / 10.02 kΩ) ≈ 1.6 and Vt = 25.865 mV at 27 °C. The
/// period moves 1.25 ms per volt of Vsat, so ±10 mV here is ±12.5 µs.
const VSAT_555: f64 = 0.0336;

// adc: supply, conversion timing and the input ramp.
const VCC_ADC: f64 = 3.3;
const ADC_BITS: i32 = 6;
/// Sample falls at the clock after Start; Done rises six clocks later (one
/// per bit of SR), and the observer delays Valid by 10 ns.
const SAMPLE_FALL_TO_STROBE: f64 = 6e-6 + 10e-9;

// delay: a 5-bit Johnson counter stepping on the microsecond.
const JOHNSON_STEP: f64 = 1e-6;
const JOHNSON_BITS: usize = 5;

// pwm: Cycles = Samples = 100 (the deck's X-card overrides), one tick per
// microsecond, into 20 Ω (D/A) + 100 kΩ and 1 µF.
const VCC_PWM: f64 = 3.3;
const PWM_RC: f64 = (100e3 + 20.0) * 1e-6;
const PWM_TICK: f64 = 1e-6;
const PWM_CYCLES: i64 = 100;
/// `M_TWO_PI` as pwm.v spells it.
const PWM_TWO_PI: f64 = 6.283_185_307_179_586;

/// Astable period away from a reset: charge from vcc/3 to 2vcc/3 through
/// r1 + r2, discharge from 2vcc/3 to vcc/3 through r2 into the saturated
/// discharge transistor.
fn t555(_: &Context<'_>) -> Result<Expected, String> {
    let (low, high) = (VCC_555 / 3.0, 2.0 * VCC_555 / 3.0);
    let charge = R_CHARGE * C_TIMING * ((VCC_555 - low) / (VCC_555 - high)).ln();
    let discharge = R_DISCHARGE * C_TIMING * ((high - VSAT_555) / (low - VSAT_555)).ln();
    Ok(values(vec![charge + discharge]))
}

/// Bit `bit` of the Johnson counter rises at (bit + 1) µs and falls at
/// (bit + 6) µs, repeating every 10 µs, from all-zero at t = 0. Nets are
/// d4..d0 for bits 4..0.
fn johnson(context: &Context<'_>) -> Result<Expected, String> {
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or("c03::johnson needs the quantity's window")?;
    let period = 2 * JOHNSON_BITS;
    let mut traces = BTreeMap::new();
    for bit in 0..JOHNSON_BITS {
        let mut trace: Trace = Vec::new();
        if start <= 0.0 {
            trace.push((0.0, "0".to_owned()));
        }
        for step in 1usize.. {
            let time = step as f64 * JOHNSON_STEP;
            if time > stop {
                break;
            }
            let phase = step % period;
            let value = if phase == (bit + 1) % period {
                "1"
            } else if phase == (bit + 1 + JOHNSON_BITS) % period {
                "0"
            } else {
                continue;
            };
            if time >= start {
                trace.push((time, value.to_owned()));
            }
        }
        traces.insert(format!("d{bit}"), trace);
    }
    Ok(Expected::Measured(Measured::Events(traces)))
}

/// The test input `PULSE(0 3 0 200u 200u 1u 401u)`.
fn adc_input(time: f64) -> f64 {
    let (rise, width, fall, period) = (200e-6, 1e-6, 200e-6, 401e-6);
    let t = time.rem_euclid(period);
    if t < rise {
        3.0 * t / rise
    } else if t < rise + width {
        3.0
    } else if t < rise + width + fall {
        3.0 * (1.0 - (t - rise - width) / fall)
    } else {
        0.0
    }
}

/// One code per rise of the strobe RSpice delivered, taken exactly as the
/// `codes` measure takes them: `floor(64·v(input)/vcc)` with the input read
/// at that conversion's Sample fall. The sampled charge is the input; each
/// bit adds vcc·2^k/64 through its capacitor, the comparator flips at vcc,
/// and the inverters turn the kept bits into N.
fn adccode(context: &Context<'_>) -> Result<Expected, String> {
    let spec = &context.quantity.measure;
    let strobe = spec
        .strobe
        .as_ref()
        .ok_or("c03::adccode needs the quantity's strobe")?
        .to_ascii_lowercase();
    let trace = context
        .rspice
        .events
        .get(&strobe)
        .ok_or_else(|| format!("c03::adccode: strobe {strobe} is absent"))?;
    let [start, stop] = spec.window.unwrap_or([f64::NEG_INFINITY, f64::INFINITY]);
    let excluded = |time: f64| {
        spec.exclude
            .iter()
            .any(|exclusion| time >= exclusion.window[0] && time <= exclusion.window[1])
    };
    let full_scale = f64::from(1 << ADC_BITS);
    let codes = trace
        .windows(2)
        .filter(|pair| pair[0].1 != "1" && pair[1].1 == "1")
        .map(|pair| pair[1].0)
        .filter(|time| *time >= start && *time <= stop && !excluded(*time))
        .map(|time| (full_scale * adc_input(time - SAMPLE_FALL_TO_STROBE) / VCC_ADC).floor())
        .collect();
    Ok(values(codes))
}

/// Result = 0 from the initial block; each inverter puts its d at vcc.
fn adcop(context: &Context<'_>) -> Result<Expected, String> {
    let count = context
        .quantity
        .measure
        .signals
        .as_ref()
        .map_or(0, Vec::len);
    Ok(values(vec![VCC_ADC; count]))
}

/// out = 0 from the initial block.
fn delayop(context: &Context<'_>) -> Result<Expected, String> {
    let count = context
        .quantity
        .measure
        .signals
        .as_ref()
        .map_or(0, Vec::len);
    Ok(values(vec![0.0; count]))
}

/// The output edges pwm.v produces, as (tick, level) from t = 0: its `always`
/// loop run tick by tick (i counts ticks in the cycle, the output falls when
/// i reaches the width and the next cycle's width is `$rtoi` of
/// Samples·(1 + sin(2π·j/Samples))/2, applied as the cycle starts).
fn pwm_edges(last_tick: i64) -> Vec<(i64, f64)> {
    let (mut i, mut j, mut width, mut out) = (0i64, 0i64, PWM_CYCLES / 2, false);
    let mut edges = vec![(0, 0.0)];
    for tick in 1..=last_tick {
        let before = out;
        i += 1;
        if i == width {
            out = false;
        }
        if i == PWM_CYCLES {
            i = 0;
            j += 1;
            if j == PWM_CYCLES {
                j = 0;
            }
            let sine = (j as f64 * PWM_TWO_PI / PWM_CYCLES as f64).sin();
            width = (PWM_CYCLES as f64 * (1.0 + sine) / 2.0).trunc() as i64;
            out = width != 0;
        }
        if out != before {
            edges.push((tick, if out { VCC_PWM } else { 0.0 }));
        }
    }
    edges
}

/// v(smooth) is the RC's exact response to those edges from the operating
/// point (out = 0, smooth = 0): between edges it relaxes exponentially toward
/// the D/A level with τ = (20 Ω + 100 kΩ)·1 µF.
fn pwmsine(context: &Context<'_>) -> Result<Expected, String> {
    let grid = context
        .quantity
        .measure
        .times
        .ok_or("c03::pwmsine needs the sample time grid")?;
    let times = grid.times();
    let last = times.iter().copied().fold(0.0f64, f64::max);
    let edges = pwm_edges((last / PWM_TICK).ceil() as i64 + 1);
    let expected = times
        .into_iter()
        .map(|time| {
            let mut v = 0.0;
            let mut level = 0.0;
            let mut at = 0.0;
            for (tick, next) in &edges {
                let edge = *tick as f64 * PWM_TICK;
                if edge > time {
                    break;
                }
                v = level + (v - level) * (-(edge - at) / PWM_RC).exp();
                level = *next;
                at = edge;
            }
            level + (v - level) * (-(time - at) / PWM_RC).exp()
        })
        .collect();
    Ok(values(expected))
}

/// out = 0 from the initial block, and the capacitor holds smooth at 0.
fn pwmop(context: &Context<'_>) -> Result<Expected, String> {
    let count = context
        .quantity
        .measure
        .signals
        .as_ref()
        .map_or(0, Vec::len);
    Ok(values(vec![0.0; count]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::mixed::case::Observation;
    use crate::suites::mixed::design::Quantity;

    fn quantity(window: &str) -> Quantity {
        toml::from_str(&format!(
            "id = \"q\"\nanalysis = \"tran\"\nvariants = [\"delay\"]\noracle = \"O6\"\nkind = \"events\"\nwindow = {window}\nwhy = \"w\"\n"
        ))
        .unwrap()
    }

    fn evaluate(function: super::super::AnalyticFn, quantity: &Quantity) -> Expected {
        let observation = Observation::default();
        function(&Context {
            quantity,
            variant: "delay",
            rspice: &observation,
            trial: None,
        })
        .unwrap()
    }

    #[test]
    fn closed_form_constants_match_the_design() {
        // 555: 13.863 ms charge and ≈ 6.97 ms discharge.
        let charge = R_CHARGE * C_TIMING * 2f64.ln();
        assert!((charge - 13.863e-3).abs() < 1e-6, "{charge}");
        let Expected::Measured(Measured::Values(period)) = evaluate(t555, &quantity("[0.0, 1.0]"))
        else {
            panic!()
        };
        assert!((period[0] - 20.836e-3).abs() < 2e-6, "{period:?}");
        // pwm: the first cycle stays low; cycle 1 is high for
        // trunc(50·(1 + sin(2π/100))) = 53 ticks; cycle 25 (sin = 1) is high
        // for the whole cycle and cycle 75 (sin = −1) not at all.
        let edges = pwm_edges(10_000);
        assert_eq!(&edges[..3], &[(0, 0.0), (100, VCC_PWM), (153, 0.0)]);
        assert!(!edges.iter().any(|(tick, _)| (2501..2600).contains(tick)));
        assert!(
            !edges
                .iter()
                .any(|(tick, level)| (7500..7600).contains(tick) && *level > 0.0)
        );
        // adc input: the ramp peaks at 3 V and falls back.
        assert!((adc_input(100e-6) - 1.5).abs() < 1e-12);
        assert!((adc_input(200.5e-6) - 3.0).abs() < 1e-12);
        assert!((adc_input(301e-6) - 1.5).abs() < 1e-12);
    }

    #[test]
    fn johnson_sequence_steps_every_microsecond() {
        let Expected::Measured(Measured::Events(traces)) =
            evaluate(johnson, &quantity("[0.0, 1.05e-5]"))
        else {
            panic!()
        };
        // d0 rises at 1 µs and falls at 6 µs; d4 rises at 5 µs and falls at 10 µs.
        let shape = |net: &str| -> Vec<(i64, String)> {
            traces[net]
                .iter()
                .map(|(time, value)| ((time / 1e-6).round() as i64, value.clone()))
                .collect()
        };
        assert_eq!(
            shape("d0"),
            vec![(0, "0".into()), (1, "1".into()), (6, "0".into())]
        );
        assert_eq!(
            shape("d4"),
            vec![(0, "0".into()), (5, "1".into()), (10, "0".into())]
        );
        assert_eq!(traces.len(), JOHNSON_BITS);
    }
}
