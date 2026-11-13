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
    // Rise 1 unit, fall 2 units, both linear.
    close(&s, "rise(V(a))", 0.8, 1e-12);
    close(&s, "rise(V(a), 20, 80)", 0.6, 1e-12);
    close(&s, "rise(V(a), 10, 90, 3)", 0.8, 1e-12);
    close(&s, "fall(V(a))", 1.6, 1e-12);
    close(&s, "fall(V(a), 10, 90, 2)", 1.6, 1e-12);
    // A 0–10 V swing stated absolutely: 10 % and 40 % of it are 1 V and
    // 4 V, three fifths of the 5 V edge.
    close(&s, "rise(V(a), 10, 40, 1, 0, 10)", 0.6, 1e-12);
    close(&s, "fall(V(a), 20, 80, 1, 5, 0)", 1.2, 1e-12);
    refuses(
        &s,
        "rise(V(a), 10, 90, 4)",
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
    close(&s, "slewRate(V(a))", 5.0, 1e-12);
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
