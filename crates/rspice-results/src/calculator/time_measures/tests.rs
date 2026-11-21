//! Analytic oracles for the time-domain measurements, through the parser and
//! evaluator so the option words are read exactly as a reader types them.
//!
//! Piecewise-linear fixtures carry their breakpoints as samples, so linear
//! interpolation is exact on them and the expected values are exact. The
//! exponential, sinusoidal and second-order fixtures are sampled densely on a
//! non-uniform grid and compared against their closed forms.

use crate::calculator::evaluator::{EvaluationContext, EvaluationError, evaluate};
use crate::calculator::parser::try_parse;
use crate::calculator::value::CalcValue;
use crate::waveform::WaveformValues;
use std::f64::consts::PI;

struct Signals(Vec<(&'static str, CalcValue)>);

impl EvaluationContext for Signals {
    fn get_waveform(&self, signal: &str, _: Option<&str>) -> Result<CalcValue, EvaluationError> {
        self.0
            .iter()
            .find(|(name, _)| *name == signal)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| EvaluationError::IdentifierNotFound(signal.to_owned()))
    }
}

/// Plain decimal literals: these fixtures need no SPICE suffixes.
fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn measure(signals: &Signals, expression: &str) -> Result<f64, String> {
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, signals).map_err(|error| error.to_string())? {
        CalcValue::Real(crate::calculator::value::RealValue::Scalar(value)) => Ok(value),
        other => Err(format!("{expression} is not a scalar: {other:?}")),
    }
}

#[track_caller]
fn close(signals: &Signals, expression: &str, expected: f64, tolerance: f64) {
    let actual =
        measure(signals, expression).unwrap_or_else(|error| panic!("{expression} failed: {error}"));
    assert!(
        (actual - expected).abs() <= tolerance,
        "{expression}: expected {expected}, got {actual} (tolerance {tolerance})"
    );
}

#[track_caller]
fn refuses(signals: &Signals, expression: &str, expected: &str) {
    match measure(signals, expression) {
        Ok(value) => panic!("{expression} measured {value}"),
        Err(error) => assert!(
            error.contains(expected),
            "{expression} refused, but not with \"{expected}\": {error}"
        ),
    }
}

/// A piecewise-linear waveform with every breakpoint retained and `fill`
/// unevenly spaced samples inside each segment.
fn pwl(points: &[(f64, f64)], fill: usize) -> CalcValue {
    let (mut x, mut y) = (vec![points[0].0], vec![points[0].1]);
    for pair in points.windows(2) {
        let ((x0, y0), (x1, y1)) = (pair[0], pair[1]);
        let weights: Vec<f64> = (1..=fill + 1).map(|step| (step % 3 + 1) as f64).collect();
        let total: f64 = weights.iter().sum();
        let mut along = 0.0;
        for (index, weight) in weights.iter().enumerate() {
            along += weight;
            let t = if index == fill { 1.0 } else { along / total };
            x.push(x0 + t * (x1 - x0));
            y.push(y0 + t * (y1 - y0));
        }
    }
    CalcValue::create_waveform(x, y)
}

/// `f` sampled at `n` unevenly spaced points on `[t0, t1]`.
fn sampled(t0: f64, t1: f64, n: usize, f: impl Fn(f64) -> f64) -> CalcValue {
    let weights: Vec<f64> = (0..n - 1).map(|step| (step % 3 + 1) as f64).collect();
    let total: f64 = weights.iter().sum();
    let mut x = vec![t0];
    let mut along = 0.0;
    for weight in &weights {
        along += weight;
        x.push(t0 + (t1 - t0) * along / total);
    }
    *x.last_mut().unwrap() = t1;
    let y = x.iter().map(|&t| f(t)).collect();
    CalcValue::create_waveform(x, y)
}

/// A 0–5 V pulse train: delay 1, rise 1, high 3, fall 2, period 10.
fn pulse_train() -> CalcValue {
    let mut points = vec![(0.0, 0.0)];
    for cycle in 0..3 {
        let t = 10.0 * cycle as f64;
        points.extend([
            (t + 1.0, 0.0),
            (t + 2.0, 5.0),
            (t + 5.0, 5.0),
            (t + 7.0, 0.0),
        ]);
    }
    points.push((30.0, 0.0));
    pwl(&points, 4)
}

/// The pulse train inverted and half a unit late.
fn inverted_late_train() -> CalcValue {
    let CalcValue::Real(crate::calculator::value::RealValue::Waveform(x, y)) = pulse_train() else {
        unreachable!()
    };
    let x = std::iter::once(0.0)
        .chain(x.iter().map(|t| t + 0.5))
        .collect();
    let y = std::iter::once(5.0)
        .chain(y.iter().map(|v| 5.0 - v))
        .collect();
    CalcValue::create_waveform(x, y)
}

fn trains() -> Signals {
    Signals(vec![
        ("V(a)", pulse_train()),
        ("V(b)", inverted_late_train()),
    ])
}

// -- crossings and delays ---------------------------------------------------

#[test]
fn cross_counts_only_the_edge_type_it_is_given() {
    let s = trains();
    // Mid-level crossings of V(a): rising at 1.5, 11.5, 21.5; falling at 6,
    // 16, 26.
    close(&s, "cross(V(a), 2.5, 2)", 6.0, 1e-12);
    close(&s, "cross(V(a), 2.5, 2, \"either\")", 6.0, 1e-12);
    close(&s, "cross(V(a), 2.5, 2, \"rising\")", 11.5, 1e-12);
    close(&s, "cross(V(a), 2.5, 3, \"falling\")", 26.0, 1e-12);
    refuses(
        &s,
        "cross(V(a), 2.5, 4, \"falling\")",
        "crosses 2.5 falling 3 time(s), not 4",
    );
    refuses(&s, "cross(V(a), 2.5, 1, \"band\")", "edge type is one of");
}

#[test]
fn cross_reads_a_sinusoid_at_its_analytic_crossing() {
    let s = Signals(vec![(
        "V(s)",
        sampled(0.0, 3.0, 30_001, |t| (2.0 * PI * t).sin()),
    )]);
    // sin(2πt) = 0.5 rising at t = 1/12 + k, falling at t = 5/12 + k.
    close(
        &s,
        "cross(V(s), 0.5, 2, \"rising\")",
        1.0 + 1.0 / 12.0,
        1e-7,
    );
    close(
        &s,
        "cross(V(s), 0.5, 2, \"falling\")",
        1.0 + 5.0 / 12.0,
        1e-7,
    );
    close(&s, "cross(V(s), 0.5, 4)", 1.0 + 5.0 / 12.0, 1e-7);
}

#[test]
fn delay_takes_each_signals_level_edge_type_and_edge_number() {
    let s = trains();
    // The old forms: each signal at its own mid level, first crossing of
    // either kind; then one level for both.
    close(&s, "delay(V(a), V(b))", 0.5, 1e-12);
    close(&s, "delay(V(a), V(b), 2.5)", 0.5, 1e-12);
    // An inverting stage: V(a) rising to V(b) falling.
    close(
        &s,
        "delay(V(a), V(b), 2.5, \"rising\", 1, 2.5, \"falling\", 1)",
        0.5,
        1e-12,
    );
    // The second signal's edge type and number default to the first's:
    // V(a)'s third rising edge (21.5) to V(b)'s third rising edge (26.5).
    close(&s, "delay(V(a), V(b), 2.5, \"rising\", 3)", 5.0, 1e-12);
    // Different levels: V(a) rising through 1 V (1.2) to V(b) falling
    // through 4 V (1.5 + 0.2).
    close(
        &s,
        "delay(V(a), V(b), 1, \"rising\", 1, 4, \"falling\", 1)",
        0.5,
        1e-12,
    );
    // Edges are counted from the start of the window, so V(b)'s first
    // falling edge (2.0) precedes V(a)'s second rising edge (11.5).
    close(
        &s,
        "delay(V(a), V(b), 2.5, \"rising\", 2, 2.5, \"falling\", 1)",
        -9.5,
        1e-12,
    );
    refuses(
        &s,
        "delay(V(a), V(b), 2.5, \"rising\", 4)",
        "first waveform has no 4th rising crossing of 2.5",
    );
}

// -- edges ------------------------------------------------------------------

#[test]
fn rise_and_fall_take_thresholds_an_edge_number_and_absolute_levels() {
    let s = trains();
    // Rise 1 unit, fall 2 units, both linear, on a 0–5 V swing.
    close(&s, "rise(V(a), 10, 90, 1, 0, 5)", 0.8, 1e-12);
    close(&s, "rise(V(a), 20, 80, 1, 0, 5)", 0.6, 1e-12);
    close(&s, "rise(V(a), 10, 90, 3, 0, 5)", 0.8, 1e-12);
    close(&s, "fall(V(a), 10, 90, 1, 5, 0)", 1.6, 1e-12);
    close(&s, "fall(V(a), 10, 90, 2, 5, 0)", 1.6, 1e-12);
    // A 0–10 V swing stated absolutely: 10 % and 40 % of it are 1 V and
    // 4 V, three fifths of the 5 V edge.
    close(&s, "rise(V(a), 10, 40, 1, 0, 10)", 0.6, 1e-12);
    close(&s, "fall(V(a), 20, 80, 1, 5, 0)", 1.2, 1e-12);
    // A clock starts and ends at the same level, so its levels are given.
    refuses(
        &s,
        "rise(V(a))",
        "first and last samples set them unless the 0 % and 100 % levels are given",
    );
    refuses(
        &s,
        "rise(V(a), 10, 90, 4, 0, 5)",
        "finds 3 complete rising edge(s)",
    );
    refuses(&s, "rise(V(a), 90, 10)", "0 <= first < second <= 100");
    refuses(&s, "rise(V(a), 10)", "two thresholds together");
    refuses(
        &s,
        "rise(V(a), 10, 90, 1, 5, 0)",
        "final level must be above",
    );
}

/// The unit step response of a second-order system with damping `zeta` and
/// natural frequency `fn_hz`, settled at 1.
fn underdamped_step(zeta: f64, fn_hz: f64) -> impl Fn(f64) -> f64 {
    let wn = 2.0 * PI * fn_hz;
    let wd = wn * (1.0 - zeta * zeta).sqrt();
    move |t: f64| 1.0 - (-zeta * wn * t).exp() * ((wd * t).cos() + zeta * wn / wd * (wd * t).sin())
}

/// Where `f` first passes `level` going up on `[0, until]`, by scanning and
/// bisecting the closed form.
fn first_rising_crossing(f: &impl Fn(f64) -> f64, level: f64, until: f64) -> f64 {
    let steps = 100_000;
    for step in 0..steps {
        let (a, b) = (
            until * step as f64 / steps as f64,
            until * (step + 1) as f64 / steps as f64,
        );
        if f(a) < level && f(b) >= level {
            let (mut lo, mut hi) = (a, b);
            for _ in 0..100 {
                let mid = 0.5 * (lo + hi);
                if f(mid) < level {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return 0.5 * (lo + hi);
        }
    }
    panic!("{level} is never reached");
}

#[test]
fn an_underdamped_step_rises_between_its_first_and_last_values() {
    // ζ = 0.2 and fn = 1 kHz peak 52.7 % over the settled 1: read against
    // the excursion, the 90 % point would sit on the overshoot.
    let response = underdamped_step(0.2, 1.0e3);
    let s = Signals(vec![("V(a)", sampled(0.0, 0.02, 200_001, &response))]);
    let rise =
        first_rising_crossing(&response, 0.9, 0.02) - first_rising_crossing(&response, 0.1, 0.02);
    assert!((rise - 191.5e-6).abs() < 0.05e-6, "closed form {rise}");
    close(&s, "rise(V(a))", rise, 1e-8);
    close(&s, "riseTime(V(a))", rise, 1e-8);
    close(&s, "slewRate(V(a))", 0.8 / rise, 0.8 / rise * 1e-4);
}

#[test]
fn an_edge_is_a_complete_transition_not_a_runt_or_a_dip() {
    // A runt to 50 % at t = 1.5, then a real edge from t = 4 to t = 5.
    let runt = pwl(
        &[
            (0.0, 0.0),
            (1.0, 0.0),
            (1.5, 2.5),
            (2.0, 0.0),
            (4.0, 0.0),
            (5.0, 5.0),
            (8.0, 5.0),
        ],
        3,
    );
    // An edge, then a dip to 60 % that recovers: not a second edge.
    let dip = pwl(
        &[
            (0.0, 0.0),
            (1.0, 0.0),
            (2.0, 5.0),
            (3.0, 5.0),
            (3.5, 3.0),
            (4.0, 5.0),
            (6.0, 5.0),
        ],
        3,
    );
    let s = Signals(vec![("V(r)", runt), ("V(d)", dip)]);
    close(&s, "rise(V(r))", 0.8, 1e-12);
    close(&s, "rise(V(d))", 0.8, 1e-12);
    refuses(
        &s,
        "rise(V(d), 10, 90, 2)",
        "finds 1 complete rising edge(s)",
    );
}

#[test]
fn rise_and_slew_rate_of_an_exponential_match_their_closed_forms() {
    let tau = 2.0;
    let s = Signals(vec![(
        "V(e)",
        sampled(0.0, 20.0, 40_001, |t| 5.0 * (1.0 - (-t / tau).exp())),
    )]);
    // 10 % to 90 % of 5 V: t = τ ln 10 − τ ln (10/9) = τ ln 9.
    let rise = tau * 9f64.ln();
    close(&s, "rise(V(e), 10, 90, 1, 0, 5)", rise, 1e-6);
    close(&s, "slewRate(V(e), 10, 90, 1, 0, 5)", 4.0 / rise, 1e-6);
    // 20 % to 50 % (1 V to 2.5 V) takes τ ln(0.8 / 0.5).
    close(
        &s,
        "slewRate(V(e), 20, 50, 1, 0, 5)",
        1.5 / (tau * (0.8f64 / 0.5).ln()),
        1e-6,
    );
}

#[test]
fn slew_rate_is_signed_by_the_direction_of_its_edge() {
    let s = trains();
    close(&s, "slewRate(V(a), 10, 90, 1, 0, 5)", 5.0, 1e-12);
    close(&s, "slewRate(V(a), 10, 90, 2, 5, 0)", -2.5, 1e-12);
    let vertical = Signals(vec![(
        "V(v)",
        CalcValue::create_waveform(vec![0.0, 1.0, 1.0, 2.0], vec![0.0, 0.0, 5.0, 5.0]),
    )]);
    refuses(&vertical, "slewRate(V(v))", "vertical edge");
    close(&vertical, "rise(V(v))", 0.0, 0.0);
}

// -- pulses and periods -------------------------------------------------------

#[test]
fn pulse_width_measures_the_nth_high_or_low_pulse() {
    let s = trains();
    // High at 2.5 V from 1.5 to 6; low from 6 to 11.5.
    close(&s, "pulseWidth(V(a))", 4.5, 1e-12);
    close(&s, "pulseWidth(V(a), 2.5, \"high\", 3)", 4.5, 1e-12);
    close(&s, "pulseWidth(V(a), 2.5, \"low\")", 5.5, 1e-12);
    // Through 1 V: rising at 1.2, falling at 6.6.
    close(&s, "pulseWidth(V(a), 1)", 5.4, 1e-12);
    refuses(
        &s,
        "pulseWidth(V(a), 2.5, \"low\", 3)",
        "does not end in the window",
    );
    refuses(
        &s,
        "pulseWidth(V(a), 2.5, \"either\")",
        "polarity is one of",
    );
}

#[test]
fn duty_cycle_reads_a_level_and_keeps_its_old_mid_level_answer() {
    let s = trains();
    close(&s, "duty(V(a))", 45.0, 1e-9);
    close(&s, "dutyCycle(V(a))", 45.0, 1e-9);
    close(&s, "dutyCycle(V(a), 1)", 54.0, 1e-9);
}

#[test]
fn period_and_frequency_average_over_the_cycles_asked_for() {
    // Periods of 10, 12 and 14: rising mid crossings at 1.5, 11.5, 23.5, 37.5.
    let mut points = vec![(0.0, 0.0)];
    let mut t = 0.0;
    for length in [10.0, 12.0, 14.0, 10.0] {
        points.extend([
            (t + 1.0, 0.0),
            (t + 2.0, 5.0),
            (t + 5.0, 5.0),
            (t + 7.0, 0.0),
        ]);
        t += length;
    }
    let s = Signals(vec![("V(p)", pwl(&points, 2))]);
    close(&s, "period(V(p))", 12.0, 1e-12);
    close(&s, "period(V(p), 1)", 10.0, 1e-12);
    close(&s, "period(V(p), 2)", 11.0, 1e-12);
    close(&s, "period(V(p), 2, 1)", 11.0, 1e-12);
    close(&s, "frequency(V(p), 2)", 1.0 / 11.0, 1e-12);
    close(&s, "freq(V(p))", 1.0 / 12.0, 1e-12);
    refuses(&s, "period(V(p), 4)", "finds 3 whole cycle(s)");
}

// -- every cycle ----------------------------------------------------------------

/// The trace a `"plot"` reading gives, as (x, y) pairs.
#[track_caller]
fn traced(signals: &Signals, expression: &str) -> Vec<(f64, f64)> {
    let parsed = try_parse(expression, plain_number).unwrap();
    match evaluate(&parsed, signals) {
        Ok(CalcValue::Real(crate::calculator::value::RealValue::Waveform(x, y))) => {
            x.into_iter().zip(y).collect()
        }
        other => panic!("{expression} is not a trace: {other:?}"),
    }
}

#[track_caller]
fn assert_points(actual: &[(f64, f64)], expected: &[(f64, f64)], tolerance: f64) {
    assert_eq!(actual.len(), expected.len(), "{actual:?}");
    for (&(x, y), &(at, value)) in actual.iter().zip(expected) {
        assert!(
            (x - at).abs() <= tolerance && (y - value).abs() <= tolerance,
            "({x}, {y}) against ({at}, {value})"
        );
    }
}

/// A linear chirp sin(2π(f0·t + k·t²/2)) from 1 kHz rising 1 MHz/s, on an
/// uneven grid: its rising zero crossings are where the phase is a whole
/// number m of cycles, t = (√(f0² + 2km) − f0) / k, so each period plotted
/// against the time it ends follows the chirp.
#[test]
fn the_period_of_a_chirp_plotted_against_time_follows_the_chirp() {
    let (f0, k) = (1e3, 1e6);
    let (start, stop) = (-1e-4, 9e-3);
    let phase = move |t: f64| f0 * t + k * t * t / 2.0;
    let s = Signals(vec![(
        "V(c)",
        sampled(start, stop, 200_001, move |t| (2.0 * PI * phase(t)).sin()),
    )]);
    let crossings: Vec<f64> = (0..)
        .map(|m| ((f0 * f0 + 2.0 * k * m as f64).sqrt() - f0) / k)
        .take_while(|&t| t < stop)
        .collect();
    let periods: Vec<(f64, f64)> = crossings
        .windows(2)
        .map(|pair| (pair[1], pair[1] - pair[0]))
        .collect();
    assert_eq!(periods.len(), 49);
    assert_points(&traced(&s, "period(V(c), \"plot\")"), &periods, 1e-9);
    // A count reads the first n cycles.
    assert_points(
        &traced(&s, "period(V(c), 5, \"plot\")"),
        &periods[..5],
        1e-9,
    );
    // Each frequency is its period's reciprocal, standing at the same time.
    let frequencies = traced(&s, "frequency(V(c), \"plot\")");
    let plotted = traced(&s, "period(V(c), \"plot\")");
    for (&(at, hz), &(x, length)) in frequencies.iter().zip(&plotted) {
        assert_eq!(at.to_bits(), x.to_bits());
        assert_eq!(hz.to_bits(), (1.0 / length).to_bits());
    }
    // Over a whole cycle a linear chirp's phase advances by one, so each
    // plotted cycle's frequency is the chirp's at the middle of the cycle.
    for &(end, length) in &plotted {
        let expected = f0 + k * (end - length / 2.0);
        assert!(
            (1.0 / length - expected).abs() <= 1e-5 * expected,
            "{} Hz against {expected} Hz",
            1.0 / length
        );
    }
}

/// The pulse train's every cycle, duty and edge: its cycles end at the
/// rises at 11.5 and 21.5, each 45 % high; its 0-5 V rises take 0.8 from
/// 10 % to 90 % and end at 1.9 past each period's start; its 2-long falls
/// slope at -2.5 V per unit and end at 6.8 past it.
#[test]
fn duty_cycle_and_edges_plot_one_value_per_cycle() {
    let s = trains();
    assert_points(
        &traced(&s, "dutyCycle(V(a), \"plot\")"),
        &[(11.5, 45.0), (21.5, 45.0)],
        1e-12,
    );
    let starts = [0.0, 10.0, 20.0];
    let rises: Vec<(f64, f64)> = starts.iter().map(|t| (t + 1.9, 0.8)).collect();
    assert_points(
        &traced(&s, "rise(V(a), 10, 90, 1, 0, 5, \"plot\")"),
        &rises,
        1e-12,
    );
    assert_points(
        &traced(&s, "rise(V(a), \"plot\", 10, 90, 2, 0, 5)"),
        &rises[1..],
        1e-12,
    );
    let falls: Vec<(f64, f64)> = starts.iter().map(|t| (t + 6.8, 1.6)).collect();
    assert_points(
        &traced(&s, "fall(V(a), 10, 90, 1, 5, 0, \"plot\")"),
        &falls,
        1e-12,
    );
    let slopes: Vec<(f64, f64)> = starts.iter().map(|t| (t + 6.8, -2.5)).collect();
    assert_points(
        &traced(&s, "slewRate(V(a), 10, 90, 1, 5, 0, \"plot\")"),
        &slopes,
        1e-12,
    );
    refuses(
        &s,
        "rise(V(a), 10, 90, 4, 0, 5, \"plot\")",
        "none from the 4th on",
    );
}

// -- step response ------------------------------------------------------------

/// The unit step response of `ωn² / (s² + 2ζωn s + ωn²)`, delayed by `t0`.
fn second_order(zeta: f64, t0: f64) -> impl Fn(f64) -> f64 {
    let wd = (1.0 - zeta * zeta).sqrt();
    move |t: f64| {
        if t <= t0 {
            return 0.0;
        }
        let t = t - t0;
        1.0 - (-zeta * t).exp() * ((wd * t).cos() + zeta / wd * (wd * t).sin())
    }
}

/// The last instant the response leaves the ±`band` tolerance about 1,
/// found by scanning the closed form and bisecting it.
fn exact_settling(response: &impl Fn(f64) -> f64, band: f64, until: f64) -> f64 {
    let outside = |t: f64| (response(t) - 1.0).abs() - band;
    let steps = 200_000;
    let mut last = 0.0;
    for step in 0..steps {
        let (a, b) = (
            until * step as f64 / steps as f64,
            until * (step + 1) as f64 / steps as f64,
        );
        if outside(a) > 0.0 && outside(b) <= 0.0 {
            let (mut lo, mut hi) = (a, b);
            for _ in 0..100 {
                let mid = 0.5 * (lo + hi);
                if outside(mid) > 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            last = 0.5 * (lo + hi);
        }
    }
    last
}

#[test]
fn a_second_order_step_has_its_closed_form_overshoot_and_settling() {
    let zeta = 0.5;
    let response = second_order(zeta, 0.0);
    let s = Signals(vec![
        ("V(y)", sampled(0.0, 30.0, 60_001, &response)),
        // A record cut off before the response has settled to 1.
        ("V(cut)", sampled(0.0, 12.0, 24_001, &response)),
        // The same response, stepped at t = 3.
        (
            "V(late)",
            sampled(0.0, 33.0, 66_001, second_order(zeta, 3.0)),
        ),
    ]);
    let overshoot = 100.0 * (-zeta * PI / (1.0 - zeta * zeta).sqrt()).exp();
    close(&s, "overshoot(V(y))", overshoot, 1e-4);
    close(&s, "overshoot(V(y), 0, 1)", overshoot, 1e-4);
    close(&s, "undershoot(V(y))", 0.0, 0.0);
    let settling = exact_settling(&response, 0.02, 30.0);
    close(&s, "settling(V(y), 2)", settling, 1e-4);
    close(&s, "settlingTime(V(y), 2)", settling, 1e-4);
    // The cut record's last sample is not the final value; stating it is.
    close(&s, "settlingTime(V(cut), 2, 1)", settling, 1e-4);
    // Measured from the instant of the step.
    close(&s, "settlingTime(V(late), 2, 1, 3)", settling, 1e-4);
    refuses(
        &s,
        "settlingTime(V(y), 2, 1, 40)",
        "lies outside the window",
    );
}

#[test]
fn undershoot_is_the_wrong_way_dip_of_a_non_minimum_phase_step() {
    // (1 − s)/(1 + s)²: y = 1 − e^−t (1 + 2t), lowest at t = 1/2.
    let s = Signals(vec![(
        "V(n)",
        sampled(0.0, 30.0, 60_001, |t| 1.0 - (-t).exp() * (1.0 + 2.0 * t)),
    )]);
    close(
        &s,
        "undershoot(V(n))",
        100.0 * (2.0 * (-0.5f64).exp() - 1.0),
        1e-5,
    );
    close(&s, "overshoot(V(n))", 0.0, 0.0);
}

#[test]
fn a_complex_signal_or_a_stray_option_is_refused_by_name() {
    let complex = Signals(vec![(
        "V(ac)",
        CalcValue::Complex(crate::calculator::value::ComplexValue::Waveform(
            vec![1.0, 2.0],
            vec![num_complex::Complex64::new(1.0, 1.0); 2],
        )),
    )]);
    refuses(
        &complex,
        "rise(V(ac))",
        "rise requires a real-valued signal",
    );
    let s = trains();
    refuses(&s, "avg(V(a), \"rising\")", "avg takes no option words");
    refuses(&s, "\"rising\"", "is an option of a measurement function");
}

/// A stored column that records the longest read it is asked for.
struct Column {
    values: Vec<f64>,
    ascending: bool,
    longest: std::sync::atomic::AtomicUsize,
}

impl crate::waveform::ColumnSource for Column {
    fn name(&self) -> &str {
        "column"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.longest
            .fetch_max(out.len(), std::sync::atomic::Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }

    fn ascending(&self) -> bool {
        self.ascending
    }
}

fn stored(values: &[f64], ascending: bool) -> (std::sync::Arc<Column>, WaveformValues) {
    let column = std::sync::Arc::new(Column {
        values: values.to_vec(),
        ascending,
        longest: std::sync::atomic::AtomicUsize::new(0),
    });
    let window = crate::waveform::StoredWindow {
        first: 0,
        rows: values.len() as u64,
        lead: crate::waveform::WindowLead::None,
    };
    let stored = WaveformValues::stored(column.clone(), window).expect("the window fits");
    (column, stored)
}

/// What an expression gives, bit for bit: a scalar, a trace's x and y, or the refusal.
fn bits(context: &impl EvaluationContext, expression: &str) -> Result<Vec<u64>, String> {
    use crate::calculator::value::RealValue;
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, context).map_err(|error| error.to_string())? {
        CalcValue::Real(RealValue::Scalar(value)) => Ok(vec![value.to_bits()]),
        CalcValue::Real(RealValue::Waveform(x, y)) => {
            Ok(x.iter().chain(&y).map(|value| value.to_bits()).collect())
        }
        other => Err(format!("{expression} gave {other:?}")),
    }
}

/// Every time measurement and level measure of a stored trace reads it a block at a time, and
/// answers what the same samples answer in memory, bit for bit, refusals included: on clip of
/// it, and with `delay` reading one stored trace against one in memory.
#[test]
fn every_measure_of_a_stored_trace_is_read_a_block_at_a_time_and_equals_its_twin_in_memory() {
    use crate::calculator::retained::WaveformsContext;
    use crate::waveform::{BLOCK_SAMPLES, RetainedWaveform};
    // Two and a bit blocks over 10 ms, on a grid whose steps grow and shrink.
    let points = 2 * BLOCK_SAMPLES + 777;
    let x: Vec<f64> = (0..points)
        .map(|i| {
            let u = i as f64 / (points - 1) as f64;
            0.01 * (u + 0.3 * (2.0 * PI * u).sin() / (2.0 * PI))
        })
        .collect();
    // A 1 kHz clock between 0 and 5 V with finite edges, and an underdamped step.
    let clock: Vec<f64> = x
        .iter()
        .map(|t| 2.5 + 2.5 * (8.0 * (2.0 * PI * 1.0e3 * t).sin()).tanh())
        .collect();
    let late: Vec<f64> = x
        .iter()
        .map(|t| 2.5 + 2.5 * (8.0 * (2.0 * PI * 1.0e3 * (t - 5.0e-5)).sin()).tanh())
        .collect();
    let (zeta, omega) = (0.3, 2.0 * PI * 2.0e3);
    let damped = omega * (1.0f64 - zeta * zeta).sqrt();
    let step: Vec<f64> = x
        .iter()
        .map(|t| {
            1.0 - (-zeta * omega * t).exp()
                * ((damped * t).cos() + zeta * omega / damped * (damped * t).sin())
        })
        .collect();
    let (axis, x_stored) = stored(&x, true);
    let (clock_column, clock_stored) = stored(&clock, false);
    let (step_column, step_stored) = stored(&step, false);
    let on_disk = vec![
        RetainedWaveform::new("V(clk)", x_stored.clone(), clock_stored),
        RetainedWaveform::new("V(step)", x_stored, step_stored),
        RetainedWaveform::new("V(late)", x.clone(), late.clone()),
    ];
    let in_memory = vec![
        RetainedWaveform::new("V(clk)", x.clone(), clock),
        RetainedWaveform::new("V(step)", x.clone(), step),
        RetainedWaveform::new("V(late)", x, late),
    ];
    let (disk, memory) = (
        WaveformsContext::new(&on_disk),
        WaveformsContext::new(&in_memory),
    );
    let mut measured = 0;
    for expression in [
        "cross(V(clk), 2.5, 3)",
        "cross(V(clk), 2.5, 2, \"falling\")",
        "cross(V(clk), 2.5, 99)",
        "rise(V(step))",
        "rise(V(clk), 10, 90, 3, 0, 5)",
        "fall(V(clk), 10, 90, 2, 5, 0)",
        "slewRate(V(step))",
        "slewRate(V(clk))",
        "delay(V(clk), V(late))",
        "delay(V(clk), V(late), 2.5, \"rising\", 2)",
        "delay(V(late), V(clk), 2.5, \"falling\", 3, 2.5, \"rising\", 4)",
        "pulseWidth(V(clk))",
        "pulseWidth(V(clk), 2.5, \"low\", 2)",
        "dutyCycle(V(clk))",
        "dutyCycle(V(clk), 1)",
        "period(V(clk))",
        "period(V(clk), 3)",
        "period(V(clk), 30)",
        "freq(V(clk))",
        "overshoot(V(step))",
        "undershoot(V(step))",
        "settlingTime(V(step), 2)",
        "settlingTime(V(step), 2, 1, 0.001)",
        "period(V(clk), \"plot\")",
        "freq(V(clk), 4, \"plot\")",
        "dutyCycle(V(clk), \"plot\")",
        "rise(V(clk), 10, 90, 2, 0, 5, \"plot\")",
        "fall(V(clk), 20, 80, 1, 5, 0, \"plot\")",
        "slewRate(V(step), \"plot\")",
        "avg(V(clk))",
        "rms(V(clk))",
        "avg(V(clk), 0.001, 0.003)",
        "min(V(step))",
        "max(V(step))",
        "pp(V(clk))",
        "min(V(clk), 0.002, 0.0025)",
        "yval(V(step), 0.00123)",
        "yval(V(step), 1)",
        "avg(clip(V(clk), 0.0011, 0.0051))",
        "cross(clip(V(clk), 0.0021, 0.009), 2.5, 1)",
        "max(clip(V(step), 0.0002, 0.0004))",
        "yval(clip(V(step), 0.0002, 0.0004), 0.0003)",
        "avg(V(clk), \"rising\")",
    ] {
        let (from_disk, from_memory) = (bits(&disk, expression), bits(&memory, expression));
        assert_eq!(
            from_disk, from_memory,
            "{expression}: stored and in memory disagree"
        );
        measured += usize::from(from_disk.is_ok());
    }
    assert!(measured >= 35, "only {measured} expressions measured");
    for column in [&axis, &clock_column, &step_column] {
        let longest = column.longest.load(std::sync::atomic::Ordering::SeqCst);
        assert!(
            longest <= BLOCK_SAMPLES,
            "a read of {longest} samples took more than a block"
        );
    }
}
