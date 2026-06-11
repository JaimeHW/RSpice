//! Harmonic-balance MOSFET capacitance validation against closed-form
//! gate-impedance physics.
//!
//! A MOSFET whose source, drain, and bulk are AC grounds presents a pure
//! capacitance at the gate, so a series-R drive has exact small-signal
//! answers: the fundamental sees the RC divider |1/(1 + j w R C)| with
//!
//! - C = Cox*W*Leff + overlaps in deep accumulation (vgs << vto - phi),
//!   where the gate-bulk wedge saturates at the full oxide capacitance;
//! - C = (2/3) Cox*W*Leff + overlaps in saturation, the square-law
//!   channel-charge limit.
//!
//! Both regions have constant capacitance, so the response must also be
//! distortion-free: the second harmonic is pinned at zero.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::f64::consts::PI;

const F0: f64 = 8.0e6;
const R: f64 = 1.0e6;
const A: f64 = 1.0e-3;

// Mirror of the construction-path oxide capacitance: eps_ox / TOX.
const COX_PER_AREA: f64 = 3.9 * 8.854214871e-12 / 20.0e-9;
const W: f64 = 10.0e-6;
const LEFF: f64 = 1.0e-6; // L = 1u, LD = 0
const COX_WL: f64 = COX_PER_AREA * W * LEFF;
const C_OVERLAP: f64 = 2.0 * 1.0e-10 * W + 1.0e-11 * LEFF; // cgso + cgdo + cgbo

const MODEL: &str =
    ".model nmod NMOS level=1 vto=0.7 kp=2e-5 tox=20n cgso=1e-10 cgdo=1e-10 cgbo=1e-11 phi=0.7";

fn run_gate_divider(deck: &str) -> HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = HbConfig::new(F0).with_harmonics(4);
    engine.run_hb(&netlist, config).expect("HB completes")
}

fn coefficient(result: &HbAnalysisResult, node: &str, k: usize) -> num_complex::Complex64 {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} present"))
        .coefficients[k]
}

fn assert_divider(result: &HbAnalysisResult, c_expected: f64, label: &str) {
    assert!(result.converged, "{label}: HB must converge");

    let w = 2.0 * PI * F0;
    let h_expected =
        num_complex::Complex64::new(1.0, 0.0) / num_complex::Complex64::new(1.0, w * R * c_expected);

    let h1 = coefficient(result, "g", 1) / num_complex::Complex64::new(A, 0.0);
    assert!(
        (h1 - h_expected).norm() < 5e-3 * h_expected.norm(),
        "{label}: gate fundamental must see the RC divider at C = {c_expected:.4e} F: \
         got {h1}, want {h_expected}"
    );

    // Constant-capacitance region: no distortion.
    let h2 = coefficient(result, "g", 2).norm();
    assert!(
        h2 < 1e-9,
        "{label}: constant-C region must be distortion-free, got HD2 = {h2:.3e} V"
    );
}

#[test]
fn accumulation_gate_capacitance_is_the_full_oxide_capacitance() {
    // vgs = -1 V: overdrive t = -1.7 V sits below -phi, so the gate-bulk
    // wedge delivers the full Cox and the channel charge is off.
    let deck = format!(
        "* mos gate capacitance divider, deep accumulation\n\
         v1 in 0 dc -1 ac 0.001\n\
         r1 in g 1meg\n\
         m1 0 g 0 0 nmod l=1u w=10u\n\
         {MODEL}\n\
         .end\n"
    );
    let result = run_gate_divider(&deck);

    // The 1e-12 S convergence GMIN leaks ~1 uV through the megohm.
    let vdc = coefficient(&result, "g", 0).re;
    assert!(
        (vdc + 1.0).abs() < 1e-5,
        "gate draws no DC current, bias must hold -1 V: got {vdc}"
    );

    assert_divider(&result, COX_WL + C_OVERLAP, "accumulation");
}

#[test]
fn saturation_gate_capacitance_is_two_thirds_of_the_oxide_capacitance() {
    // vgs = 2 V, vds = 3 V > vgst: saturation. The drain rail is an AC
    // ground, so there is no Miller multiplication and the gate sees
    // (2/3) Cox plus the overlaps.
    let deck = format!(
        "* mos gate capacitance divider, saturation\n\
         v1 in 0 dc 2 ac 0.001\n\
         vdd d 0 dc 3\n\
         r1 in g 1meg\n\
         m1 d g 0 0 nmod l=1u w=10u\n\
         {MODEL}\n\
         .end\n"
    );
    let result = run_gate_divider(&deck);

    // The 1e-12 S convergence GMIN leaks ~2 uV through the megohm.
    let vdc = coefficient(&result, "g", 0).re;
    assert!(
        (vdc - 2.0).abs() < 1e-5,
        "gate draws no DC current, bias must hold +2 V: got {vdc}"
    );

    assert_divider(&result, 2.0 / 3.0 * COX_WL + C_OVERLAP, "saturation");
}
