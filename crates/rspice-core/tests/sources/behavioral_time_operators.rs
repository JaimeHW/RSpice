//! `ddt`, `idt` and `delay` inside behavioral expressions, checked in every
//! analysis domain against closed forms and QSPICE (the authority wherever it
//! implements the function; `absdelay` is LTspice-only).

use super::op_point;

use num_complex::Complex64;
use op_point::op_voltage;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::f64::consts::PI;

/// The trapezoid `PULSE(0.25 1 1m 1m 1m 2m 6m)` drives every transient deck.
const TRAPEZOID: &str = "PULSE(0.25 1 1m 1m 1m 2m 6m)";

fn trapezoid(t: f64) -> f64 {
    if t < 1e-3 {
        return 0.25;
    }
    let local = (t - 1e-3) % 6e-3;
    match local {
        x if x < 1e-3 => 0.25 + 0.75 * x / 1e-3,
        x if x < 3e-3 => 1.0,
        x if x < 4e-3 => 1.0 - 0.75 * (x - 3e-3) / 1e-3,
        _ => 0.25,
    }
}

fn trapezoid_integral(from: f64, to: f64) -> f64 {
    let steps = 200_000;
    let h = (to - from) / steps as f64;
    (0..steps)
        .map(|k| {
            let t = from + k as f64 * h;
            0.5 * h * (trapezoid(t) + trapezoid(t + h))
        })
        .sum()
}

struct Transient {
    time: Vec<f64>,
    names: Vec<String>,
    voltages: Vec<Vec<f64>>,
}

impl Transient {
    fn run(deck: &str, stop: f64, step: f64) -> Self {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, stop, step)
            .expect("transient runs");
        Self {
            time: result.time.clone(),
            names: result.node_names.clone(),
            voltages: result.voltages.clone(),
        }
    }

    fn at(&self, node: &str, t: f64) -> f64 {
        let index = self
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing"));
        let values = &self.voltages[index];
        let k = self.time.partition_point(|&time| time < t);
        if k == 0 {
            return values[0];
        }
        if k == self.time.len() {
            return values[k - 1];
        }
        let (t0, t1) = (self.time[k - 1], self.time[k]);
        values[k - 1] + (values[k] - values[k - 1]) * (t - t0) / (t1 - t0)
    }
}

fn ac(deck: &str, node: &str, frequency: f64) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("ac solves");
    let result = &results[0];
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing"));
    result.voltages[index]
}

fn operator_deck(source: &str) -> String {
    format!(
        "* behavioral time operators
V1 in 0 DC 0.25 AC 1 {TRAPEZOID}
Vr rst 0 PULSE(0 1 5m 1u 1u 0.5m 100)
B1 d 0 V=ddt(V(in))
B2 i 0 V=idt(V(in),2)
B5 r 0 V=idt(V(in),0.5,V(rst))
B3 y 0 V=delay(V(in),1.5m)
B4 a 0 V=absdelay(V(in),{{tdel}})
.param tdel=1.5m
R1 d 0 1k
R2 i 0 1k
R3 y 0 1k
{source}
.end
"
    )
}

#[test]
fn operators_hold_their_operating_point_values() {
    // QSPICE .op: ddt 0, idt its initial condition, delay the input itself.
    let deck = operator_deck("");
    for (node, want) in [("d", 0.0), ("i", 2.0), ("r", 0.5), ("y", 0.25), ("a", 0.25)] {
        let got = op_voltage(&deck, node);
        assert!((got - want).abs() < 1e-8, "{node}: got {got}, want {want}");
    }
}

#[test]
fn operators_linearize_to_their_exact_transfers_in_ac() {
    // QSPICE 2026 AC: jω, 1/(jω) and e^(-jω·1.5m) to machine precision.
    let deck = operator_deck("");
    for frequency in [10.0, 100.0, 250.0, 1000.0] {
        let omega = 2.0 * PI * frequency;
        let expected = [
            ("d", Complex64::new(0.0, omega)),
            ("i", Complex64::new(0.0, -1.0 / omega)),
            ("y", Complex64::from_polar(1.0, -omega * 1.5e-3)),
            ("a", Complex64::from_polar(1.0, -omega * 1.5e-3)),
        ];
        for (node, want) in expected {
            let got = ac(&deck, node, frequency);
            assert!(
                (got - want).norm() <= 1e-6 * want.norm(),
                "{node} at {frequency} Hz: got {got}, want {want}"
            );
        }
    }
}

#[test]
fn operators_follow_the_transient_closed_forms() {
    let run = Transient::run(&operator_deck(""), 10e-3, 1e-6);
    for t in [0.5e-3, 1.5e-3, 2.5e-3, 3.5e-3, 4.5e-3, 6e-3, 7.5e-3, 9e-3] {
        let slope = (trapezoid(t + 1e-7) - trapezoid(t - 1e-7)) / 2e-7;
        let checks = [
            ("d", slope, 1e-3 * 750.0),
            ("i", 2.0 + trapezoid_integral(0.0, t), 1e-6),
            ("y", trapezoid((t - 1.5e-3).max(0.0)), 1e-3),
            ("a", trapezoid((t - 1.5e-3).max(0.0)), 1e-3),
        ];
        for (node, want, tolerance) in checks {
            let got = run.at(node, t);
            assert!(
                (got - want).abs() <= tolerance,
                "{node} at {t}: got {got}, want {want}"
            );
        }
    }
}

#[test]
fn reset_restarts_the_integral_from_its_initial_condition() {
    // QSPICE 2026: held at 0.5 while the reset is high (5m..5.5m), then
    // integrating from 0.5 again: 0.500125 at 6m, 0.500593 at 7.5m, 0.502 at 9m.
    let run = Transient::run(&operator_deck(""), 10e-3, 1e-6);
    let release = 5.5e-3 + 1e-6;
    for (t, want) in [
        (4.5e-3, 0.5 + trapezoid_integral(0.0, 4.5e-3)),
        (5.2e-3, 0.5),
        (6e-3, 0.5 + trapezoid_integral(release, 6e-3)),
        (7.5e-3, 0.5 + trapezoid_integral(release, 7.5e-3)),
        (9e-3, 0.5 + trapezoid_integral(release, 9e-3)),
    ] {
        let got = run.at("r", t);
        assert!(
            (got - want).abs() <= 2e-6,
            "r at {t}: got {got}, want {want}"
        );
    }
}

#[test]
fn integrating_then_differentiating_recovers_the_input() {
    let deck = format!(
        "* nested operators
V1 in 0 {TRAPEZOID}
B1 out 0 V=ddt(idt(V(in)))
R1 out 0 1k
.end
"
    );
    let run = Transient::run(&deck, 10e-3, 1e-6);
    for t in [0.5e-3, 1.5e-3, 2.5e-3, 4.5e-3, 6e-3, 7.5e-3, 9e-3] {
        let (got, want) = (run.at("out", t), trapezoid(t));
        assert!(
            (got - want).abs() < 1e-3,
            "out at {t}: got {got}, want {want}"
        );
    }
}

#[test]
fn value_sources_and_subcircuits_lower_each_instance_privately() {
    // Two instances of one subcircuit each own their integrator and delay.
    let deck = "* operators in a subcircuit and an E VALUE source
V1 in 0 DC 1 AC 1
.subckt stage a b k=1
E1 b 0 VALUE={k*idt(V(a),0.5)+delay(V(a),{k*1m})}
.ends
X1 in o1 stage k=1
X2 in o2 stage k=2
R1 o1 0 1k
R2 o2 0 1k
.end
";
    for (node, want) in [("o1", 1.5), ("o2", 2.0)] {
        let got = op_voltage(deck, node);
        assert!((got - want).abs() < 1e-8, "{node}: got {got}, want {want}");
    }
    let omega = 2.0 * PI * 100.0;
    for (node, k) in [("o1", 1.0), ("o2", 2.0)] {
        let want = Complex64::new(0.0, -k / omega) + Complex64::from_polar(1.0, -omega * k * 1e-3);
        let got = ac(deck, node, 100.0);
        assert!(
            (got - want).norm() < 1e-6 * want.norm(),
            "{node}: got {got}, want {want}"
        );
    }
}

/// A delay that follows its own input, `1.5m + 0.5m·V(in)`.
const VARYING_DELAY: &str = "delay(V(in),1.5m+0.5m*V(in),3m)";

#[test]
fn a_varying_delay_reads_the_history_at_its_own_delay() {
    // QSPICE 2026: 0.625 at 3.5 ms, 1 at 4 ms and 0.71875 at 6 ms, the
    // trapezoid read at t - td(t) with td following the input.
    let deck = format!(
        "* varying delay
V1 in 0 {TRAPEZOID}
B1 w 0 V={VARYING_DELAY}
R1 w 0 1k
.end
"
    );
    assert!((op_voltage(&deck, "w") - 0.25).abs() < 1e-9);
    let run = Transient::run(&deck, 10e-3, 1e-6);
    for (t, want) in [(3.5e-3, 0.625), (4e-3, 1.0), (6e-3, 0.71875)] {
        let got = run.at("w", t);
        assert!(
            (got - want).abs() < 2e-3,
            "w at {t}: got {got}, want {want}"
        );
    }
    for k in 1..200 {
        let t = 10e-3 * f64::from(k) / 200.0;
        let want = trapezoid((t - (1.5e-3 + 0.5e-3 * trapezoid(t))).max(0.0));
        let got = run.at("w", t);
        assert!(
            (got - want).abs() < 5e-3,
            "w at {t}: got {got}, want {want}"
        );
    }
}

#[test]
fn a_varying_delay_linearizes_at_its_operating_point_delay() {
    // QSPICE 2026 AC at 1 kHz: e^(-jω·0.2m) = 0.309017-0.951057j, and four
    // times that for 2·V(in)² delayed by V(td)·V(in). The subcircuit lowers
    // its own private delay.
    let deck = "* varying delay ac
V1 in 0 DC 1 AC 1
Vt td 0 DC 0.2m
B1 a 0 V=delay(V(in),V(td),1m)
B2 c 0 V=delay(2*V(in)*V(in),V(td)*V(in),1m)
X1 in x lag
.subckt lag p q
B1 q 0 V=delay(V(p),0.2m*V(p),1m)
.ends
.end
";
    let rotation = Complex64::from_polar(1.0, -2.0 * PI * 1e3 * 0.2e-3);
    for (node, want) in [("a", rotation), ("c", 4.0 * rotation), ("x", rotation)] {
        let got = ac(deck, node, 1e3);
        assert!(
            (got - want).norm() < 1e-9 * want.norm(),
            "{node}: got {got}, want {want}"
        );
    }
    assert!((op_voltage(deck, "c") - 2.0).abs() < 1e-9);
}

#[test]
fn a_varying_delay_past_its_maximum_stops_the_transient() {
    // QSPICE stops with "MAXDELAY was given as 0.0005 but that is less than
    // the runtime delay encountered".
    let deck = "* over maximum
V1 in 0 SIN(0 1 1k)
Vt td 0 PWL(0 0.1m 2m 0.6m)
B1 o 0 V=delay(V(in),V(td),0.5m)
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 2e-3, 1e-6)
        .expect_err("refused")
        .to_string();
    assert!(error.contains("exceeds its maximum"), "{error}");
}

#[test]
fn malformed_operator_calls_are_refused_by_name() {
    for (expression, needle) in [
        (
            "delay(V(in),1m,0)",
            "maximum delay must be finite and positive",
        ),
        ("delay(V(in))", "delay() takes 2 to 3 arguments"),
        ("ddt(V(in),1)", "ddt() takes 1 to 1 arguments"),
        ("idt(V(in),0,0,1)", "idt() takes 1 to 3 arguments"),
        ("delay(V(in),2m,1m)", "exceeds its maximum"),
    ] {
        let deck = format!(
            "* refusal
V1 in 0 1
B1 out 0 V={expression}
.end
"
        );
        let error = Netlist::parse(&deck).expect_err("refused").to_string();
        assert!(error.contains(needle), "{expression}: {error}");
    }
}
