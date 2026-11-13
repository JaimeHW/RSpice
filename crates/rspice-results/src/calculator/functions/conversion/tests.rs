//! Analytic oracles for `digital()` and `analog()`: threshold crossings of a
//! sine on a non-uniform grid, the hysteresis band and the time to X, bus
//! words read back from the staircase, and the round trip between the two.

use super::*;
use crate::calculator::evaluator::{CalcValue, EvaluationContext, evaluate};
use crate::calculator::functions::FunctionRegistry;
use std::f64::consts::PI;

const PERIOD: f64 = 1e-6;
const AMPLITUDE: f64 = 0.9;
const OFFSET: f64 = 0.9;

/// Two periods of `OFFSET + AMPLITUDE·sin(2πt/T)` on a grid whose step
/// alternates between T/600 and T/300.
fn sine() -> RealValue {
    let mut x = vec![0.0];
    let mut step = 0;
    while *x.last().unwrap() < 2.0 * PERIOD {
        let h = if step % 3 == 0 {
            PERIOD / 300.0
        } else {
            PERIOD / 600.0
        };
        x.push((x.last().unwrap() + h).min(2.0 * PERIOD));
        step += 1;
    }
    let y = x
        .iter()
        .map(|t| AMPLITUDE.mul_add((2.0 * PI * t / PERIOD).sin(), OFFSET))
        .collect();
    RealValue::create_waveform(x, y)
}

fn digital_of(args: Vec<RealValue>) -> (Vec<f64>, Vec<f64>) {
    match FunctionRegistry::dispatch("digital", args) {
        Ok(RealValue::Waveform(x, y)) => (x, y),
        other => panic!("digital did not return a waveform: {other:?}"),
    }
}

/// The instants at which the step waveform changes level, with the level
/// from each (NaN for X), read the way a logic lane reads it.
fn edges(x: &[f64], y: &[f64]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (&t, &v) in x.iter().zip(y) {
        if out.last().is_some_and(|last| last.0 == t) {
            out.pop();
        }
        let same = out
            .last()
            .is_some_and(|last| last.1 == v || (last.1.is_nan() && v.is_nan()));
        if !same {
            out.push((t, v));
        }
    }
    out
}

fn level(v: f64) -> &'static str {
    if v.is_nan() {
        "x"
    } else if v == 0.0 {
        "0"
    } else if v == 1.0 {
        "1"
    } else {
        "?"
    }
}

fn scalar(v: f64) -> RealValue {
    RealValue::Scalar(v)
}

/// The analytic crossing instants of the sine through `level`, rising and
/// falling, over two periods.
fn crossings(level: f64) -> (Vec<f64>, Vec<f64>) {
    let phase = ((level - OFFSET) / AMPLITUDE).asin();
    let rising = (0..2)
        .map(|k| (phase + 2.0 * PI * f64::from(k)) / (2.0 * PI) * PERIOD)
        .collect();
    let falling = (0..2)
        .map(|k| (PI - phase + 2.0 * PI * f64::from(k)) / (2.0 * PI) * PERIOD)
        .collect();
    (rising, falling)
}

/// Linear interpolation over steps of at most T/300 misplaces a crossing by
/// at most h²ω/(8·cos φ) ≈ 9e-6 T at these levels.
const TOLERANCE: f64 = 3e-5 * PERIOD;

#[test]
fn single_threshold_edges_sit_on_the_analytic_crossings() {
    let vth = OFFSET + 0.3 * AMPLITUDE;
    let (x, y) = digital_of(vec![sine(), scalar(vth)]);
    let found = edges(&x, &y);
    let (rising, falling) = crossings(vth);
    // The sine starts below the threshold, so the lane starts low.
    let levels: String = found.iter().map(|(_, v)| level(*v)).collect();
    assert_eq!(levels, "01010");
    let expected = [rising[0], falling[0], rising[1], falling[1]];
    for ((t, _), want) in found[1..].iter().zip(expected) {
        assert!(
            (t - want).abs() <= TOLERANCE,
            "edge at {t}, analytic {want}"
        );
    }
    // Edges are interpolated, not moved to a retained sample.
    let RealValue::Waveform(grid, _) = sine() else {
        unreachable!()
    };
    assert!(found[1..].iter().all(|(t, _)| !grid.contains(t)));
}

#[test]
fn hysteresis_rises_at_the_high_threshold_and_falls_at_the_low_one() {
    let high = OFFSET + 0.6 * AMPLITUDE;
    let low = OFFSET - 0.4 * AMPLITUDE;
    let (x, y) = digital_of(vec![sine(), scalar(high), scalar(low)]);
    let found = edges(&x, &y);
    let levels: String = found.iter().map(|(_, v)| level(*v)).collect();
    // The sine starts at the offset, inside the band: X until it reaches
    // the high threshold, since there is no level to hold yet.
    assert_eq!(levels, "x1010");
    let (rising, _) = crossings(high);
    let (_, falling) = crossings(low);
    let expected = [rising[0], falling[0], rising[1], falling[1]];
    for ((t, _), want) in found[1..].iter().zip(expected) {
        assert!(
            (t - want).abs() <= TOLERANCE,
            "edge at {t}, analytic {want}"
        );
    }
}

#[test]
fn hysteresis_ignores_ripple_smaller_than_its_band() {
    // A slow ramp through 0.5 with ripple of ±0.05 crosses 0.5 several times;
    // a band of ±0.1 around it switches once.
    let x: Vec<f64> = (0..=1000).map(|i| f64::from(i) * 1e-9).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|t| 0.05f64.mul_add((2.0 * PI * t / 20e-9).sin(), t / 1e-6))
        .collect();
    let wave = RealValue::create_waveform(x, y);
    let (sx, sy) = digital_of(vec![wave.clone(), scalar(0.5)]);
    assert!(
        edges(&sx, &sy).len() > 3,
        "the ripple chatters on one threshold"
    );
    let (hx, hy) = digital_of(vec![wave, scalar(0.6), scalar(0.4)]);
    let levels: String = edges(&hx, &hy).iter().map(|(_, v)| level(*v)).collect();
    assert_eq!(levels, "01");
}

/// 0 V until 0, up to 0.5 V at 10 ns, held to 20 ns, up to 1 V at 30 ns: it
/// enters the 0.3–0.7 V band at 6 ns and leaves it at 24 ns, 18 ns later.
fn parked() -> RealValue {
    RealValue::create_waveform(vec![0.0, 10e-9, 20e-9, 30e-9], vec![0.0, 0.5, 0.5, 1.0])
}

fn parked_levels(time_to_x: Option<f64>) -> Vec<(f64, &'static str)> {
    let mut args = vec![parked(), scalar(0.7), scalar(0.3)];
    args.extend(time_to_x.map(scalar));
    let (x, y) = digital_of(args);
    edges(&x, &y)
        .into_iter()
        .map(|(t, v)| ((t * 1e12).round() / 1e3, level(v)))
        .collect()
}

#[test]
fn time_to_x_turns_a_held_level_to_x_once_the_band_outlasts_it() {
    // Unlimited: the band only holds.
    assert_eq!(parked_levels(None), [(0.0, "0"), (24.0, "1")]);
    // 18 ns in the band is within 20 ns.
    assert_eq!(parked_levels(Some(20e-9)), [(0.0, "0"), (24.0, "1")]);
    // Over 5 ns it is X from 6 + 5 ns until it reaches the high threshold.
    assert_eq!(
        parked_levels(Some(5e-9)),
        [(0.0, "0"), (11.0, "x"), (24.0, "1")]
    );
    // Zero: the whole band is X.
    assert_eq!(
        parked_levels(Some(0.0)),
        [(0.0, "0"), (6.0, "x"), (24.0, "1")]
    );
}

#[test]
fn a_band_still_occupied_at_the_end_runs_out_there() {
    let wave = RealValue::create_waveform(vec![0.0, 10e-9, 40e-9], vec![0.0, 0.5, 0.5]);
    let (x, y) = digital_of(vec![wave, scalar(0.7), scalar(0.3), scalar(10e-9)]);
    let found: Vec<_> = edges(&x, &y)
        .into_iter()
        .map(|(t, v)| ((t * 1e12).round() / 1e3, level(v)))
        .collect();
    assert_eq!(found, [(0.0, "0"), (16.0, "x")]);
    assert_eq!(
        *x.last().unwrap(),
        40e-9,
        "the lane runs to the last sample"
    );
}

#[test]
fn a_hole_is_x_and_the_next_value_starts_afresh() {
    let wave = RealValue::create_waveform(
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![1.0, 1.0, f64::NAN, 0.0, 0.0],
    );
    let (x, y) = digital_of(vec![wave, scalar(0.5)]);
    let found: Vec<_> = edges(&x, &y)
        .into_iter()
        .map(|(t, v)| (t, level(v)))
        .collect();
    assert_eq!(found, [(0.0, "1"), (2.0, "x"), (3.0, "0")]);
}

#[test]
fn a_sample_on_the_threshold_is_high_and_changes_where_the_signal_leaves_it() {
    // Down to the threshold and back up: equal to it is still 1.
    let touch = RealValue::create_waveform(vec![0.0, 1.0, 2.0], vec![1.0, 0.5, 1.0]);
    let (x, y) = digital_of(vec![touch, scalar(0.5)]);
    assert_eq!(edges(&x, &y).len(), 1);
    // Down through it: 0 from the instant it leaves the threshold, and a
    // last-bit nudge of that sample moves the edge by no more than that.
    let through = |v: f64| {
        let wave = RealValue::create_waveform(vec![0.0, 1.0, 2.0], vec![1.0, v, 0.0]);
        let (x, y) = digital_of(vec![wave, scalar(0.5)]);
        edges(&x, &y)
    };
    let exact = through(0.5);
    assert_eq!(exact, [(0.0, 1.0), (1.0, 0.0)]);
    for nudged in [0.5_f64.next_down(), 0.5_f64.next_up()] {
        let moved = through(nudged);
        assert_eq!(moved.len(), 2);
        assert!((moved[1].0 - 1.0).abs() < 1e-12, "{moved:?}");
    }
}

#[test]
fn digital_refuses_what_has_no_reading() {
    let refuse = |args: Vec<RealValue>| FunctionRegistry::dispatch("digital", args).unwrap_err();
    assert!(
        refuse(vec![scalar(1.0), scalar(0.5)])
            .to_string()
            .contains("not a scalar")
    );
    assert!(
        refuse(vec![sine(), scalar(0.3), scalar(0.7)])
            .to_string()
            .contains("low threshold below its high one")
    );
    assert!(
        refuse(vec![sine(), scalar(0.7), scalar(0.3), scalar(-1.0)])
            .to_string()
            .contains("time to X")
    );
    let reversed = RealValue::create_waveform(vec![2.0, 1.0, 0.0], vec![0.0, 1.0, 0.0]);
    assert!(
        refuse(vec![reversed, scalar(0.5)])
            .to_string()
            .contains("backwards")
    );
}

fn bits(text: &str) -> Vec<Option<bool>> {
    text.chars()
        .map(|c| match c {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
        })
        .collect()
}

fn word(times: &[f64], words: &[&str], signed: bool, end: f64) -> EventWord {
    EventWord {
        times: times.to_vec(),
        words: words.iter().map(|text| bits(text)).collect(),
        signed,
        end,
    }
}

/// The staircase's value held from each event instant.
fn held_values(value: &RealValue) -> Vec<(f64, f64)> {
    let RealValue::Waveform(x, y) = value else {
        panic!("analog did not return a waveform")
    };
    edges(x, y)
}

#[test]
fn analog_staircase_equals_the_bus_word_at_every_event() {
    let times = [0.0, 1e-9, 2.5e-9, 4e-9, 7e-9];
    let words = ["0000", "0101", "1111", "10x1", "1000"];
    let unsigned = analog_staircase(&word(&times, &words, false, 10e-9)).unwrap();
    let held = held_values(&unsigned);
    let expected = [0.0, 5.0, 15.0, f64::NAN, 8.0];
    assert_eq!(held.len(), expected.len());
    for ((t, v), (want_t, want)) in held.iter().zip(times.iter().zip(expected)) {
        assert_eq!(t, want_t);
        assert!(
            *v == want || (v.is_nan() && want.is_nan()),
            "{t}: {v} vs {want}"
        );
    }
    // Horizontal between events, vertical at them, through the end.
    let RealValue::Waveform(x, _) = &unsigned else {
        unreachable!()
    };
    assert_eq!(x.first(), Some(&0.0));
    assert_eq!(x.last(), Some(&10e-9));
    assert_eq!(x.len(), 2 * times.len());

    let signed = analog_staircase(&word(&times, &words, true, 10e-9)).unwrap();
    let values: Vec<f64> = held_values(&signed).into_iter().map(|(_, v)| v).collect();
    assert_eq!(values[..3], [0.0, 5.0, -1.0]);
    assert!(values[3].is_nan());
    assert_eq!(values[4], -8.0);
}

#[test]
fn analog_refuses_a_net_without_events() {
    let empty = word(&[], &[], false, 1.0);
    assert!(analog_staircase(&empty).is_err());
    assert!(FunctionRegistry::dispatch("analog", vec![scalar(1.0)]).is_err());
}

#[test]
fn digital_of_analog_reproduces_the_bits() {
    let times = [0.0, 3e-9, 3e-9, 5e-9, 9e-9, 12e-9];
    let words = ["0110", "1001", "1011", "x011", "0100", "1110"];
    let staircase = analog_staircase(&word(&times, &words, false, 15e-9)).unwrap();
    // Each member, as a one-bit word, comes back bit for bit, X included.
    for member in 0..4 {
        let bit_words: Vec<String> = words
            .iter()
            .map(|word| word[member..=member].to_owned())
            .collect();
        let bit_refs: Vec<&str> = bit_words.iter().map(String::as_str).collect();
        let analog = analog_staircase(&word(&times, &bit_refs, false, 15e-9)).unwrap();
        let (x, y) = digital_of(vec![analog.clone(), scalar(0.5)]);
        let spell = |changes: Vec<(f64, f64)>| -> Vec<(f64, &str)> {
            changes.into_iter().map(|(t, v)| (t, level(v))).collect()
        };
        assert_eq!(
            spell(edges(&x, &y)),
            spell(held_values(&analog)),
            "member {member}"
        );
    }
    // The whole word at its mid-scale threshold is its MSB.
    let (x, y) = digital_of(vec![staircase, scalar(7.5)]);
    let msb: Vec<(f64, &str)> = edges(&x, &y)
        .into_iter()
        .map(|(t, v)| (t * 1e9, level(v)))
        .collect();
    let msb: Vec<(f64, &str)> = msb
        .into_iter()
        .map(|(t, v)| ((t * 1e3).round() / 1e3, v))
        .collect();
    assert_eq!(
        msb,
        [(0.0, "0"), (3.0, "1"), (5.0, "x"), (9.0, "0"), (12.0, "1")]
    );
}

fn try_parse(text: &str) -> Result<crate::calculator::ast::CalculatorExpr, String> {
    crate::calculator::parser::try_parse(text, |number| {
        number.parse::<f64>().map_err(|error| error.to_string())
    })
    .map_err(|error| error.to_string())
}

/// A context that holds one bus.
struct Bus(EventWord);

impl EvaluationContext for Bus {
    fn get_waveform(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<CalcValue, crate::calculator::evaluator::EvaluationError> {
        Err(crate::calculator::evaluator::EvaluationError::IdentifierNotFound(signal.to_owned()))
    }

    fn get_event_word(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<EventWord, crate::calculator::evaluator::EvaluationError> {
        if signal == "D(q[3:0])" {
            Ok(self.0.clone())
        } else {
            Err(
                crate::calculator::evaluator::EvaluationError::IdentifierNotFound(
                    signal.to_owned(),
                ),
            )
        }
    }
}

#[test]
fn an_expression_reads_a_bus_by_its_d_name() {
    let bus = Bus(word(&[0.0, 1e-9], &["0011", "1100"], false, 2e-9));
    let analog = evaluate(&try_parse("analog(D(q[3:0]))").unwrap(), &bus).unwrap();
    let CalcValue::Real(analog) = analog else {
        panic!("analog is real")
    };
    assert_eq!(held_values(&analog), [(0.0, 3.0), (1e-9, 12.0)]);
    let round = evaluate(&try_parse("digital(analog(D(q[3:0])), 7.5)").unwrap(), &bus).unwrap();
    let CalcValue::Real(RealValue::Waveform(x, y)) = round else {
        panic!("digital is a waveform")
    };
    assert_eq!(edges(&x, &y), [(0.0, 0.0), (1e-9, 1.0)]);
    // A name the context has no history for is no signal at all.
    let missing = evaluate(&try_parse("analog(D(r[3:0]))").unwrap(), &bus).unwrap_err();
    assert!(missing.to_string().contains("D(r[3:0])"), "{missing}");
}
