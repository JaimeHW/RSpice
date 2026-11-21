//! Closed-form oracles for the jitter readings, through the parser and
//! evaluator: a trapezoid clock whose every pulse is shifted by
//! a·(-1)^k, so its periods alternate T - 2a and T + 2a and every reading
//! has a closed form computed here.

use crate::calculator::evaluator::evaluate;
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::{CalcValue, RealValue};
use crate::waveform::RetainedWaveform;

fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn evaluated(waveforms: &[RetainedWaveform], expression: &str) -> Result<RealValue, String> {
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, &WaveformsContext::new(waveforms)).map_err(|error| error.to_string())? {
        CalcValue::Real(value) => Ok(value),
        other => Err(format!("{expression} is complex: {other:?}")),
    }
}

/// The trace a "plot" reading gives, as (x, y) pairs.
#[track_caller]
fn trace(waveforms: &[RetainedWaveform], expression: &str) -> Vec<(f64, f64)> {
    match evaluated(waveforms, expression) {
        Ok(RealValue::Waveform(x, y)) => x.into_iter().zip(y).collect(),
        other => panic!("{expression} is not a trace: {other:?}"),
    }
}

#[track_caller]
fn scalar(waveforms: &[RetainedWaveform], expression: &str) -> f64 {
    match evaluated(waveforms, expression) {
        Ok(RealValue::Scalar(value)) => value,
        other => panic!("{expression} is not a scalar: {other:?}"),
    }
}

const PERIOD: f64 = 1e-6;
const SHIFT: f64 = 20e-9;
const EDGE: f64 = 50e-9;
const PULSES: usize = 41;
/// Rounding in the edge times, which are sums of a few products of PERIOD.
const TIME_TOLERANCE: f64 = 1e-18;

/// Pulse k's rising mid-level crossing: k·T + T/4 + a·(-1)^k.
fn rising(k: usize) -> f64 {
    let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
    k as f64 * PERIOD + PERIOD / 4.0 + sign * SHIFT
}

/// Pulse k's falling crossing, half a period after its rising one.
fn falling(k: usize) -> f64 {
    rising(k) + PERIOD / 2.0
}

/// The clock from 0 to PULSES·T: each pulse rises through 0.5 at
/// rising(k) and falls through it at falling(k), its edges straight and
/// EDGE long. Each pulse's area is T/2 whatever its shift, so the trace's
/// mean level is exactly 0.5 and the default threshold finds the same
/// crossings.
fn clock() -> RetainedWaveform {
    let mut x = vec![0.0];
    let mut y = vec![0.0];
    for k in 0..PULSES {
        for (at, level) in [
            (rising(k) - EDGE / 2.0, 0.0),
            (rising(k) + EDGE / 2.0, 1.0),
            (falling(k) - EDGE / 2.0, 1.0),
            (falling(k) + EDGE / 2.0, 0.0),
        ] {
            x.push(at);
            y.push(level);
        }
    }
    x.push(PULSES as f64 * PERIOD);
    y.push(0.0);
    RetainedWaveform::new("V(clk)", x, y)
}

#[track_caller]
fn assert_trace(actual: &[(f64, f64)], expected: &[(f64, f64)]) {
    assert_eq!(actual.len(), expected.len(), "{actual:?}");
    for (&(x, y), &(at, value)) in actual.iter().zip(expected) {
        assert!(
            (x - at).abs() <= TIME_TOLERANCE * 100.0 && (y - value).abs() <= TIME_TOLERANCE,
            "({x}, {y}) against ({at}, {value})"
        );
    }
}

fn population_deviation(values: &[f64]) -> f64 {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    (values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / values.len() as f64).sqrt()
}

/// Period jitter is each period less the mean period. The first and last
/// pulses share their shift, so the mean period is T, and the periods
/// alternate T - 2a and T + 2a: each reading stands at the edge that ends
/// its period.
#[test]
fn a_jittered_clocks_period_jitter_is_its_alternating_shift() {
    let waveforms = [clock()];
    let mean = (rising(PULSES - 1) - rising(0)) / (PULSES - 1) as f64;
    let expected: Vec<(f64, f64)> = (0..PULSES - 1)
        .map(|k| (rising(k + 1), rising(k + 1) - rising(k) - mean))
        .collect();
    assert_trace(&trace(&waveforms, "period_jitter(V(clk))"), &expected);
    assert_trace(
        &trace(&waveforms, "period_jitter(V(clk), \"rising\", 0.5)"),
        &expected,
    );
    let values: Vec<f64> = expected.iter().map(|&(_, value)| value).collect();
    let deviation = scalar(&waveforms, "period_jitter(V(clk), \"sd\")");
    assert!(
        (deviation - population_deviation(&values)).abs() <= TIME_TOLERANCE,
        "{deviation}"
    );
    assert!((deviation - 2.0 * SHIFT).abs() <= 1e-15, "{deviation}");
    // The falling edges carry the same shifts.
    let expected: Vec<(f64, f64)> = (0..PULSES - 1)
        .map(|k| (falling(k + 1), falling(k + 1) - falling(k) - mean))
        .collect();
    assert_trace(
        &trace(&waveforms, "period_jitter(V(clk), \"falling\")"),
        &expected,
    );
}

/// Cycle-to-cycle jitter is each period less the one before: ±4a, by
/// turns, twenty of one and nineteen of the other. Long-term jitter over n periods is the time they take less n mean
/// periods: an even span's shifts cancel, an odd one's read ±2a.
#[test]
fn cycle_to_cycle_and_long_term_jitter_follow_the_edges() {
    let waveforms = [clock()];
    let periods: Vec<f64> = (0..PULSES - 1).map(|k| rising(k + 1) - rising(k)).collect();
    let expected: Vec<(f64, f64)> = (0..PULSES - 2)
        .map(|k| (rising(k + 2), periods[k + 1] - periods[k]))
        .collect();
    assert_trace(&trace(&waveforms, "cc_jitter(V(clk))"), &expected);
    let deviation = scalar(&waveforms, "cc_jitter(V(clk), \"sd\")");
    let (high, low): (f64, f64) = (20.0, 19.0);
    let share = high * low / ((high + low) * (high + low));
    let closed = 2.0 * 4.0 * SHIFT * share.sqrt();
    assert!(
        (deviation - closed).abs() <= 1e-15,
        "{deviation} against {closed}"
    );

    let mean = (rising(PULSES - 1) - rising(0)) / (PULSES - 1) as f64;
    for span in [2, 3] {
        let expected: Vec<(f64, f64)> = (0..PULSES - span)
            .map(|k| {
                let at = rising(k + span);
                (at, at - rising(k) - span as f64 * mean)
            })
            .collect();
        assert_trace(
            &trace(&waveforms, &format!("longterm_jitter(V(clk), {span})")),
            &expected,
        );
    }
    let even = scalar(&waveforms, "longterm_jitter(V(clk), 2, \"sd\")");
    assert!(even.abs() <= 1e-15, "{even}");
}

/// Absolute jitter, the time interval error, is each edge less the same
/// edge of an ideal clock started at the first: by default of the mean
/// period, 0 and -2a by turns (twenty-one zeros, twenty of -2a); with a
/// stated nominal period, the drift of the difference as well.
#[test]
fn absolute_jitter_is_each_edge_against_an_ideal_clock() {
    let waveforms = [clock()];
    let mean = (rising(PULSES - 1) - rising(0)) / (PULSES - 1) as f64;
    let ideal = |nominal: f64| -> Vec<(f64, f64)> {
        (0..PULSES)
            .map(|k| (rising(k), rising(k) - (rising(0) + k as f64 * nominal)))
            .collect()
    };
    assert_trace(&trace(&waveforms, "abs_jitter(V(clk))"), &ideal(mean));
    let nominal = 1.001e-6;
    assert_trace(
        &trace(
            &waveforms,
            &format!("abs_jitter(V(clk), \"rising\", \"plot\", 0.5, {nominal})"),
        ),
        &ideal(nominal),
    );
    let deviation = scalar(&waveforms, "abs_jitter(V(clk), \"sd\")");
    let (zeros, shifted): (f64, f64) = (21.0, 20.0);
    let share = zeros * shifted / ((zeros + shifted) * (zeros + shifted));
    let closed = 2.0 * SHIFT * share.sqrt();
    assert!(
        (deviation - closed).abs() <= 1e-15,
        "{deviation} against {closed}"
    );
}

/// An edge that rattles across the threshold makes one crossing: the
/// signal must pass a tenth of its samples' deviation beyond the threshold
/// before the next crossing counts.
#[test]
fn an_edge_that_rattles_across_the_threshold_counts_once() {
    let mut x = vec![0.0];
    let mut y = vec![0.0];
    for k in 0..5 {
        let at = rising(k);
        for (offset, level) in [
            (-EDGE / 2.0, 0.0),
            (-EDGE / 8.0, 0.52),
            (0.0, 0.47),
            (EDGE / 8.0, 0.53),
            (EDGE / 2.0, 1.0),
            (PERIOD / 2.0 - EDGE / 2.0, 1.0),
            (PERIOD / 2.0 + EDGE / 2.0, 0.0),
        ] {
            x.push(at + offset);
            y.push(level);
        }
    }
    x.push(5.0 * PERIOD);
    y.push(0.0);
    let waveforms = [RetainedWaveform::new("V(r)", x, y)];
    let periods = trace(&waveforms, "period_jitter(V(r), 0.5)");
    assert_eq!(periods.len(), 4, "{periods:?}");
}

/// What cannot be read is refused with the reason.
#[test]
fn a_jitter_reading_that_cannot_be_made_is_refused() {
    let waveforms = [clock()];
    for (expression, reason) in [
        ("period_jitter(V(clk), 5)", "needs at least 2 edges"),
        ("cc_jitter(V(clk), \"high\")", "not \"high\""),
        ("longterm_jitter(V(clk))", "expected 2"),
        ("longterm_jitter(V(clk), \"sd\")", "whole number of periods"),
        ("longterm_jitter(V(clk), 1.5)", "whole number of periods"),
        ("longterm_jitter(V(clk), 99)", "needs at least 100 edges"),
        (
            "abs_jitter(V(clk), \"rising\", \"plot\", 0.5, 0)",
            "nominal period must be positive",
        ),
        ("period_jitter(2)", "measures a waveform, not a scalar"),
    ] {
        let error = evaluated(&waveforms, expression).unwrap_err();
        assert!(error.contains(reason), "{expression}: {error}");
    }
}
