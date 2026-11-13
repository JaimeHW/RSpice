//! PSpice `FREQ` controlled sources. Small-signal analyses read the table
//! exactly, between entries as QSPICE reads it; operating points pass the
//! input through unscaled, as QSPICE does; the transient runs a stable
//! rational fit of the entries from the operating point.

use super::op_point;

use num_complex::Complex64;
use op_point::op_voltage;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::f64::consts::TAU;

const F0: f64 = 1e3;
const Q: f64 = 0.7;

fn biquad(frequency: f64) -> Complex64 {
    let s = Complex64::new(0.0, frequency / F0);
    1.0 / (s * s + s / Q + 1.0)
}

fn ac(deck: &str, node: &str, frequency: f64) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("ac solves");
    let index = results[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .expect("node present");
    results[0].voltages[index]
}

fn transient(deck: &str, node: &str) -> (Vec<f64>, Vec<f64>) {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 3e-3, 1e-6)
        .expect("transient runs");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .expect("node present");
    (result.time.clone(), result.voltages[index].clone())
}

/// Root-mean-square difference of `(time, value)` against `reference`,
/// read at the nearest reference time at or after each point.
fn rms_difference(time: &[f64], value: &[f64], reference: &(Vec<f64>, Vec<f64>)) -> f64 {
    let (reference_time, reference_value) = reference;
    let squares: f64 = time
        .iter()
        .zip(value)
        .map(|(&t, &v)| {
            let k = reference_time
                .partition_point(|&x| x < t)
                .min(reference_time.len() - 1);
            (v - reference_value[k]).powi(2)
        })
        .sum();
    (squares / time.len() as f64).sqrt()
}

/// Ten entries per decade from 10 Hz to 100 kHz, after a 0 Hz entry.
fn entry_frequencies() -> Vec<f64> {
    (0..=40)
        .map(|k| 10f64.powf(1.0 + f64::from(k) / 10.0))
        .collect()
}

/// `transfer` sampled at 0 Hz and [`entry_frequencies`], in dB and degrees.
fn sampled_table(transfer: impl Fn(f64) -> Complex64) -> String {
    let mut table = String::from("(0,0,0)");
    for frequency in entry_frequencies() {
        let gain = transfer(frequency);
        table.push_str(&format!(
            " ({frequency},{},{})",
            20.0 * gain.norm().log10(),
            gain.arg().to_degrees()
        ));
    }
    table
}

#[test]
fn ac_reads_a_sampled_rational_exactly_at_and_between_entries() {
    let deck = format!(
        "* sampled biquad\nvin in 0 dc 2 ac 1\nE1 out 0 FREQ {{V(in)}} = {}\nrl out 0 1k\n.end\n",
        sampled_table(biquad)
    );
    assert!((op_voltage(&deck, "out") - 2.0).abs() < 1e-9);
    let entries = entry_frequencies();
    for k in [3, 17, 20, 26, 39] {
        let (low, high) = (entries[k], entries[k + 1]);
        let got = ac(&deck, "out", low);
        assert!(
            (got - biquad(low)).norm() < 1e-9,
            "entry {low} Hz: got {got}"
        );
        // Adjacent entries turn less than a quarter turn apart, so halfway
        // in log frequency the dB and phase are the means of the entries'.
        let (a, b) = (biquad(low), biquad(high));
        let midway = Complex64::from_polar((a.norm() * b.norm()).sqrt(), 0.5 * (a.arg() + b.arg()));
        let got = ac(&deck, "out", (low * high).sqrt());
        assert!(
            (got - midway).norm() < 1e-9,
            "between {low} and {high} Hz: got {got}, want {midway}"
        );
    }
}

#[test]
fn the_transient_of_a_sampled_rational_is_its_laplace_realization() {
    let w0 = TAU * F0;
    let step = "* step\nvin in 0 pulse(0 1 0.1m 1u 1u 1 2)\nrl out 0 1k\n";
    let (time, fitted) = transient(
        &format!(
            "{step}E1 out 0 FREQ {{V(in)}} = {}\n.end\n",
            sampled_table(biquad)
        ),
        "out",
    );
    let reference = transient(
        &format!(
            "{step}E1 out 0 LAPLACE {{V(in)}} = {{{w0}*{w0}/(s*s+s*{}+{w0}*{w0})}}\n.end\n",
            w0 / Q
        ),
        "out",
    );
    let rms = rms_difference(&time, &fitted, &reference);
    assert!(rms < 1e-3, "step response RMS difference {rms}");
}

#[test]
fn delay_names_the_transport_delay_the_table_already_holds() {
    // QSPICE and LTspice 26.1.1 both leave the AC answer of a table with
    // DELAY= unchanged, so the delay is part of the written phase. The
    // transient takes it out, fits the rest, and adds it back as a pure delay.
    let delay = 0.2e-3;
    let pole = TAU * 2e3;
    let lowpass = |frequency: f64| 1.0 / Complex64::new(1.0, TAU * frequency / pole);
    let delayed =
        |frequency: f64| lowpass(frequency) * Complex64::from_polar(1.0, -TAU * frequency * delay);
    let step = "* step\nvin in 0 pulse(0 1 0.1m 1u 1u 1 2)\nrl out 0 1k\n";
    let freq_deck = format!(
        "{step}E1 out 0 FREQ {{V(in)}} = {} DELAY={delay}\n.end\n",
        sampled_table(delayed)
    );
    let ac_deck = freq_deck.replace("pulse(0 1 0.1m 1u 1u 1 2)", "dc 0 ac 1");
    for k in [10, 20, 27] {
        let frequency = entry_frequencies()[k];
        let got = ac(&ac_deck, "out", frequency);
        assert!(
            (got - delayed(frequency)).norm() < 1e-9,
            "H at {frequency} Hz: got {got}"
        );
    }
    let (time, fitted) = transient(&freq_deck, "out");
    let reference = transient(
        &format!("{step}E1 out 0 LAPLACE {{V(in)}} = {{exp(-s*{delay})/(1+s/{pole})}}\n.end\n"),
        "out",
    );
    let rms = rms_difference(&time, &fitted, &reference);
    assert!(rms < 1e-3, "delayed step response RMS difference {rms}");
}

#[test]
fn entries_read_between_as_qspice_reads_them() {
    // QSPICE64 (build 2026-10-27) AC of a third-order low-pass table: from
    // the 0 Hz entry the real and imaginary parts are linear in frequency;
    // within a quarter turn dB and phase are linear in log frequency; across
    // the -135/+145 deg wrap the parts are linear again.
    let deck = "* qspice table
vin in 0 dc 1 ac 1
E1 out 0 FREQ {V(in)} = (0,0,0) (100,-0.129641,-17.1318) (300,-1.122795,-50.0977)
+ (1000,-9.030900,-135.0000) (3000,-30.000000,145.3048) (10000,-60.129641,107.1318)
rl out 0 1k
.end
";
    for (frequency, want) in [
        (50.0, Complex64::new(0.970736, -0.145103)),
        (
            200.0,
            Complex64::from_polar(10f64.powf(-0.75625 / 20.0), (-37.9310f64).to_radians()),
        ),
        (2e3, Complex64::new(-0.138, -0.116)),
        (
            5e3,
            Complex64::from_polar(10f64.powf(-42.78351 / 20.0), 129.1086f64.to_radians()),
        ),
        (2e4, Complex64::new(-0.000290207, 0.000941472)),
    ] {
        let got = ac(deck, "out", frequency);
        assert!(
            (got - want).norm() < 1e-5 * want.norm().max(1e-2),
            "H at {frequency} Hz: got {got}, want {want}"
        );
    }
}

#[test]
fn the_operating_point_passes_the_input_and_ac_reads_the_table() {
    // QSPICE (build 2026-10-27) .op passes every input through unscaled,
    // whatever the table's lowest entry: 1.5 V in, 1.5 V out, 3 V for
    // {2*V(in)}, and 1.5 A from the G source. AC reads the table, its lowest
    // entry below the first frequency.
    let deck = "* op passes the input
vin in 0 dc 1.5 ac 1
E6 o6 0 FREQ {V(in)} = (10,6,180) (1k,6,180)
E7 o7 0 FREQ {V(in)} = (0,6,0) (1k,0,-45)
E9 o9 0 FREQ {2*V(in)} = (0,-6,0) (1k,-9,-45) (10k,-26,-90)
G1 0 g1 FREQ {V(in)} = (0,6,0) (1k,0,-45)
r1 g1 0 1
X1 in xo inverting
.subckt inverting a b
E1 b 0 FREQ {V(a)} = (10,6,180) (1k,6,180)
.ends
.end
";
    for (node, want) in [
        ("o6", 1.5),
        ("o7", 1.5),
        ("o9", 3.0),
        ("g1", 1.5),
        ("xo", 1.5),
    ] {
        let got = op_voltage(deck, node);
        assert!((got - want).abs() < 1e-9, "{node}: got {got}, want {want}");
    }
    let inverting = -(10f64.powf(6.0 / 20.0));
    for node in ["o6", "xo"] {
        assert!((ac(deck, node, 1.0) - inverting).norm() < 1e-9, "{node}");
    }
    let doubling = 10f64.powf(6.0 / 20.0);
    for node in ["o7", "g1"] {
        let got = ac(deck, node, 1e-3);
        assert!((got - doubling).norm() < 1e-5, "{node}: {got}");
    }
}

#[test]
fn the_transient_leaves_the_operating_point_for_the_table() {
    // LTspice 26.1.1 batch: 1 V in, the output is 1 V at the operating point
    // and -1.99526 V from the first step on. QSPICE has no FREQ transient.
    let deck = "* tran from a dc op
vin in 0 dc 1
E6 o6 0 FREQ {V(in)} = (10,6,180) (1k,6,180)
X1 in xo inverting
.subckt inverting a b
E1 b 0 FREQ {V(a)} = (10,6,180) (1k,6,180)
.ends
.end
";
    let inverting = -(10f64.powf(6.0 / 20.0));
    for node in ["o6", "xo"] {
        let (time, value) = transient(deck, node);
        assert_eq!(time[0], 0.0);
        assert!((value[0] - 1.0).abs() < 1e-9, "{node} at t=0: {}", value[0]);
        for (t, v) in time.iter().zip(&value).skip(1) {
            assert!(
                (v - inverting).abs() < 1e-3 * inverting.abs(),
                "{node} at t={t}: {v}, want {inverting}"
            );
        }
    }
}

#[test]
fn units_keywords_state_the_entries() {
    // LTspice 26.1.1 decides the MAG, RAD and R_I keywords, which QSPICE
    // does not read. Both tables are 2/(1+jf/1k) and 1/(1+jf/1k).
    let deck = "* units
vin in 0 dc 1 ac 1
E1 a 0 FREQ {V(in)} MAG RAD = (0,2,0) (1k,1.4142135623730951,-0.7853981633974483)
E2 b 0 FREQ {V(in)} R_I = (0,1,0) (1k,0.5,-0.5)
ra a 0 1k
rb b 0 1k
.end
";
    assert!((ac(deck, "a", 1.0) - Complex64::new(1.999, -0.001)).norm() < 1e-9);
    assert!((ac(deck, "b", 1.0) - Complex64::new(0.9995, -0.0005)).norm() < 1e-9);
    assert!((ac(deck, "a", 1e3) - Complex64::new(1.0, -1.0)).norm() < 1e-9);
    assert!((ac(deck, "b", 1e3) - Complex64::new(0.5, -0.5)).norm() < 1e-9);
}

#[test]
fn a_table_no_stable_rational_follows_is_refused() {
    let deck = "* wrap
vin in 0 1
E1 out 0 FREQ {V(in)} = (100,0,0) (1k,-6,-170) (2k,-6,170)
.end
";
    let error = Netlist::parse(deck).expect_err("refused").to_string();
    assert!(error.contains("no stable rational transfer"), "{error}");
}
