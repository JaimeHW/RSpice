//! Harmonic-balance charge-storage validation against closed-form varactor
//! physics.
//!
//! A reverse-biased diode is a pure depletion capacitor (no conduction), so
//! a series-R drive has exact small-signal answers:
//!
//! - the fundamental sees the RC divider with C0 = Cj(Vdc): magnitude
//!   |1/(1 + j w R C0)| and phase -atan(w R C0);
//! - the capacitance slope C' = dC/dV converts drive amplitude A into a
//!   second harmonic of amplitude R*C'*A^2*w/2 at the diode node.
//!
//! Both predictions are absolute (volts, not ratios), so any factor-of-two
//! anywhere in the spectral conventions, the charge formulas, or the jw_k
//! coupling fails them.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::f64::consts::PI;

const F0: f64 = 1.0e6;
const R: f64 = 1.0e3;
const CJ0: f64 = 10.0e-12;
const VJ: f64 = 0.7;
const M: f64 = 0.5;
const VBIAS: f64 = 5.0;
const A: f64 = 0.05;

fn run_varactor() -> HbAnalysisResult {
    // d1 0 vc: anode grounded, cathode at +5 V -> vd = -5 V, deep reverse.
    let deck = "\
* reverse-biased varactor behind a series resistor
v1 in 0 dc 5 sin(5 0.05 1meg)
r1 in vc 1k
d1 0 vc dmod
.model dmod D IS=1e-16 N=1.0 CJ0=10p VJ=0.7 M=0.5
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = HbConfig::new(F0).with_harmonics(8);
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

#[test]
fn varactor_fundamental_sees_the_bias_point_capacitance() {
    let result = run_varactor();
    assert!(result.converged, "HB must converge");

    // C0 = CJ0 * (1 + VBIAS/VJ)^-M at vd = -5 V.
    let c0 = CJ0 * (1.0 + VBIAS / VJ).powf(-M);
    let w = 2.0 * PI * F0;
    let h_expected =
        num_complex::Complex64::new(1.0, 0.0) / num_complex::Complex64::new(1.0, w * R * c0);

    let vdc = coefficient(&result, "vc", 0).re;
    assert!(
        (vdc - VBIAS).abs() < 1e-3,
        "bias point must hold +5 V (no conduction), got {vdc}"
    );

    let h1 = coefficient(&result, "vc", 1) / num_complex::Complex64::new(A, 0.0);
    assert!(
        (h1 - h_expected).norm() < 5e-3 * h_expected.norm(),
        "fundamental transfer must be the RC divider at C(Vdc): got {h1}, want {h_expected}"
    );
}

#[test]
fn varactor_second_harmonic_matches_the_capacitance_slope() {
    let result = run_varactor();

    // C' = dC/dv at vd = -5: CJ0*(M/VJ)*(1 + VBIAS/VJ)^-(M+1).
    let c_slope = CJ0 * (M / VJ) * (1.0 + VBIAS / VJ).powf(-(M + 1.0));
    let w = 2.0 * PI * F0;
    let expected_h2 = R * c_slope * A * A * w / 2.0;

    let h2 = coefficient(&result, "vc", 2).norm();
    assert!(
        (h2 - expected_h2).abs() < 0.03 * expected_h2,
        "second harmonic must be R*C'*A^2*w/2 = {expected_h2:.4e} V, got {h2:.4e} V"
    );

    // Third harmonic is another factor ~A*C''/C' down: well under the second.
    let h3 = coefficient(&result, "vc", 3).norm();
    assert!(
        h3 < 0.1 * h2,
        "third harmonic must be far below the second: h2 = {h2:.3e}, h3 = {h3:.3e}"
    );
}
