//! End-to-end tests for the SFFM and AM transient sources.
//!
//! The waveform formulas are transcribed from ngspice-46 vsrcload.c; these
//! tests pin them analytically (a voltage source's node equals the source
//! value exactly at every accepted timepoint) plus the parity quirks:
//! exact 0 before TD, degree phases, the MDI clamp, and the DC operating
//! point seeing 0.

use std::f64::consts::PI;

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Netlist, SourceSpec};

const TOL: f64 = 1e-9;

fn sffm_reference(t: f64, vo: f64, va: f64, fc: f64, mdi: f64, fm: f64, td: f64) -> f64 {
    let tp = t - td;
    if tp <= 0.0 {
        0.0
    } else {
        vo + va * ((2.0 * PI * fc * tp) + mdi * (2.0 * PI * fm * tp).sin()).sin()
    }
}

fn am_reference(t: f64, vo: f64, vmo: f64, vma: f64, fm: f64, fc: f64, td: f64) -> f64 {
    let tp = t - td;
    if tp <= 0.0 {
        0.0
    } else {
        vo + (vmo + vma * (2.0 * PI * fm * tp).sin()) * (2.0 * PI * fc * tp).sin()
    }
}

fn transient_node_values(deck: &str, node: &str, tstop: f64, max_step: f64) -> (Vec<f64>, Vec<f64>) {
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine.run_tran(&netlist, tstop, max_step).expect("transient");
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    (result.time.clone(), result.voltages[idx].clone())
}

#[test]
fn sffm_parses_full_and_short_forms() {
    let deck = "\
* sffm parse
v1 1 0 sffm(0.2 1 1meg 5 100k 2u 30 60)
r1 1 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let Some(rspice_core::netlist::ElementKind::VoltageSource(SourceSpec::Sffm {
        offset,
        amplitude,
        carrier_freq,
        modulation_index,
        signal_freq,
        delay,
        phase_modulation,
        phase_carrier,
    })) = netlist
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("v1"))
        .map(|e| &e.kind)
    else {
        panic!("expected SFFM source");
    };
    assert_eq!(*offset, 0.2);
    assert_eq!(*amplitude, 1.0);
    assert_eq!(*carrier_freq, 1.0e6);
    assert_eq!(*modulation_index, 5.0);
    assert_eq!(*signal_freq, 1.0e5);
    assert_eq!(*delay, 2.0e-6);
    assert_eq!(*phase_modulation, 30.0);
    assert_eq!(*phase_carrier, 60.0);

    let short = "\
* sffm short
v1 1 0 sffm(0 1)
r1 1 0 1k
.end
";
    let netlist = Netlist::parse(short).expect("parse");
    let Some(rspice_core::netlist::ElementKind::VoltageSource(SourceSpec::Sffm {
        carrier_freq,
        signal_freq,
        ..
    })) = netlist
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("v1"))
        .map(|e| &e.kind)
    else {
        panic!("expected SFFM source");
    };
    assert!(
        carrier_freq.is_nan() && signal_freq.is_nan(),
        "omitted frequencies must stay NaN for tstop-based defaults"
    );
}

#[test]
fn sffm_transient_matches_ngspice_formula() {
    // 5 us window of SFFM(0.2 1 1meg 5 100k): fc=1 MHz carrier, 100 kHz
    // modulation, MDI=5 (below FC/FM=10 so no clamp).
    let deck = "\
* sffm waveform
v1 1 0 sffm(0.2 1 1meg 5 100k)
r1 1 0 1k
.tran 10n 5u
.end
";
    let (times, values) = transient_node_values(deck, "1", 5.0e-6, 10.0e-9);
    assert!(times.len() > 100, "expected a dense transient");
    for (t, v) in times.iter().zip(values.iter()) {
        let expected = sffm_reference(*t, 0.2, 1.0, 1.0e6, 5.0, 1.0e5, 0.0);
        assert!(
            (v - expected).abs() < TOL,
            "sffm at t={t}: got {v}, formula {expected}"
        );
    }
}

#[test]
fn sffm_clamps_modulation_index_like_ngspice() {
    // MDI=25 exceeds FC/FM=10, so ngspice clamps it to 10.
    let deck = "\
* sffm clamp
v1 1 0 sffm(0 1 1meg 25 100k)
r1 1 0 1k
.tran 10n 2u
.end
";
    let (times, values) = transient_node_values(deck, "1", 2.0e-6, 10.0e-9);
    for (t, v) in times.iter().zip(values.iter()) {
        let expected = sffm_reference(*t, 0.0, 1.0, 1.0e6, 10.0, 1.0e5, 0.0);
        assert!(
            (v - expected).abs() < TOL,
            "clamped sffm at t={t}: got {v}, formula {expected}"
        );
    }
}

#[test]
fn sffm_is_exactly_zero_before_delay_with_breakpoint() {
    let deck = "\
* sffm delay gating
v1 1 0 sffm(0.5 1 1meg 2 100k 1u)
r1 1 0 1k
.tran 5n 3u
.end
";
    let (times, values) = transient_node_values(deck, "1", 3.0e-6, 5.0e-9);
    let mut saw_pre_delay = false;
    let mut saw_post_delay = false;
    for (t, v) in times.iter().zip(values.iter()) {
        if *t < 1.0e-6 - 1e-15 {
            saw_pre_delay = true;
            assert_eq!(*v, 0.0, "ngspice SFFM is exactly 0 before TD (t={t})");
        }
        if *t > 1.0e-6 {
            saw_post_delay = true;
            let expected = sffm_reference(*t, 0.5, 1.0, 1.0e6, 2.0, 1.0e5, 1.0e-6);
            assert!(
                (v - expected).abs() < TOL,
                "sffm after delay at t={t}: got {v}, formula {expected}"
            );
        }
    }
    assert!(saw_pre_delay && saw_post_delay);
    assert!(
        times.iter().any(|t| (t - 1.0e-6).abs() < 1e-12),
        "TD must be a breakpoint timepoint"
    );
}

#[test]
fn am_transient_matches_ngspice_formula_including_phases() {
    // AM(0.1 0.5 2 20k 1meg 0 45 90): both phase arguments in degrees.
    let deck = "\
* am waveform
v1 1 0 am(0.1 0.5 2 20k 1meg 0 45 90)
r1 1 0 1k
.tran 10n 5u
.end
";
    let (times, values) = transient_node_values(deck, "1", 5.0e-6, 10.0e-9);
    for (t, v) in times.iter().zip(values.iter()) {
        let expected = if *t <= 0.0 {
            0.0
        } else {
            0.1 + (0.5 + 2.0 * (2.0 * PI * 2.0e4 * t + 45.0_f64.to_radians()).sin())
                * (2.0 * PI * 1.0e6 * t + 90.0_f64.to_radians()).sin()
        };
        assert!(
            (v - expected).abs() < TOL,
            "am at t={t}: got {v}, formula {expected}"
        );
    }
}

#[test]
fn am_delay_gates_to_exact_zero() {
    let deck = "\
* am delay gating
v1 1 0 am(0 1 1 50k 1meg 2u)
r1 1 0 1k
.tran 10n 6u
.end
";
    let (times, values) = transient_node_values(deck, "1", 6.0e-6, 10.0e-9);
    for (t, v) in times.iter().zip(values.iter()) {
        if *t < 2.0e-6 - 1e-15 {
            assert_eq!(*v, 0.0, "ngspice AM is exactly 0 before TD (t={t})");
        } else if *t > 2.0e-6 {
            let expected = am_reference(*t, 0.0, 1.0, 1.0, 5.0e4, 1.0e6, 2.0e-6);
            assert!(
                (v - expected).abs() < TOL,
                "am after delay at t={t}: got {v}, formula {expected}"
            );
        }
    }
}

#[test]
fn modulated_sources_contribute_zero_to_the_operating_point() {
    // ngspice's vsrcload.c yields exactly 0 at t<=TD, which is what the DC
    // operating point evaluates.
    let deck = "\
* op sees zero
v1 1 0 sffm(0.7 1 1meg 5 100k)
r1 1 2 1k
r2 2 0 1k
v2 3 0 am(0.3 1 1 10k 1meg)
r3 3 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("op");
    for node in ["1", "2", "3"] {
        let idx = op
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing"));
        assert!(
            op.node_voltages[idx].abs() < 1e-12,
            "OP voltage at node {node} must be 0, got {}",
            op.node_voltages[idx]
        );
    }
}
