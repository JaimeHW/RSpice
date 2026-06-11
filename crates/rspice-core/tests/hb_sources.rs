//! Nonlinear-HB source-stamping orientation pinned against basic circuit
//! physics.
//!
//! The nonlinear Newton path converts ideal voltage sources into stiff Norton
//! equivalents. The injected current must reproduce the source polarity: a
//! +2 V rail has to come out at +2 V, and an AC drive at phase 0 must appear
//! in-phase at the driven node. A mirrored injection produces an inverted
//! bias that a symmetric-drive rectifier test can never catch.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

fn run_hb(deck: &str, freq: f64, harmonics: usize) -> HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = HbConfig::new(freq).with_harmonics(harmonics);
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
fn dc_rail_keeps_its_polarity_through_norton_conversion() {
    // Forward-biased diode behind a +2 V rail: V(vin) must sit at +2 V and the
    // diode node at one forward drop, ~0.72 V for Is=1e-14 at ~13 mA.
    let deck = "\
* forward-biased diode behind a dc rail
v1 vin 0 dc 2.0
r1 vin vd 100
d1 vd 0 dmod
c1 vd 0 1n
.model dmod D IS=1e-14 N=1.0
.end
";
    let result = run_hb(deck, 1.0e6, 4);
    assert!(result.converged, "HB must converge");

    let vin_dc = coefficient(&result, "vin", 0).re;
    let vd_dc = coefficient(&result, "vd", 0).re;

    assert!(
        (vin_dc - 2.0).abs() < 0.05,
        "rail node must hold +2 V, got {vin_dc}"
    );
    assert!(
        (0.55..0.85).contains(&vd_dc),
        "diode node must sit one forward drop above ground, got {vd_dc}"
    );
}

#[test]
fn ac_drive_is_in_phase_at_the_driven_node() {
    // Small-signal divider with a mostly-off diode: the fundamental at the
    // driven node must match the source phasor 10 mV at 0 degrees.
    let deck = "\
* in-phase ac drive check
v1 a 0 dc 0 ac 0.01
r1 a b 1k
r2 b 0 1k
d1 b 0 dmod
c1 b 0 1p
.model dmod D IS=1e-14 N=1.0
.end
";
    let result = run_hb(deck, 1.0e6, 4);
    assert!(result.converged, "HB must converge");

    let a_fund = coefficient(&result, "a", 1);
    assert!(
        a_fund.re > 0.004,
        "driven node fundamental must be in phase with the source, got {a_fund}"
    );
    assert!(
        a_fund.im.abs() < 0.002,
        "driven node fundamental must be nearly real, got {a_fund}"
    );
}
