//! Exact nonlinear-HB source-constraint orientation pinned against basic
//! circuit physics.
//!
//! The nonlinear Newton path retains ideal voltage sources as MNA branch
//! constraints. Their incidence and authored spectrum must reproduce source
//! polarity: a +2 V rail has to come out at +2 V, and an AC drive at phase 0
//! must appear in-phase at the driven node. A mirrored constraint produces an
//! inverted bias that a symmetric-drive rectifier test can never catch.

use rspice_core::analysis::harmonic_balance::HbConfig;
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
fn dc_rail_keeps_its_polarity_through_exact_mna_constraint() {
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

#[test]
fn devices_see_the_physical_voltage_swing() {
    // Diode clipper driven at 0.45 V amplitude: the diode barely conducts
    // (peak current ~0.4 uA into 1k), so the DC shift and second harmonic at
    // the diode node must be sub-millivolt. Before the amplitude-to-Fourier-
    // coefficient boundary conversion, devices were evaluated at twice the
    // drive (an effective 0.9 V swing), which clips hard and parks tens of
    // millivolts of DC and HD2 here.
    let deck = "\
* clipper swing pin (conduction only: junction charge pinned off so this
* test isolates the amplitude convention through the exponential)
v1 in 0 sin(0 0.45 1meg)
r1 in d 1k
d1 d 0 dmod
c1 d 0 1p
.model dmod D IS=1e-14 N=1.0 CJ0=0 TT=0
.end
";
    let result = run_hb(deck, 1.0e6, 8);
    assert!(result.converged, "HB must converge");

    let dc = coefficient(&result, "d", 0).re;
    let h1 = coefficient(&result, "d", 1);
    let h2 = coefficient(&result, "d", 2).norm();

    assert!(
        dc.abs() < 2e-3,
        "a 0.45 V drive must not rectify visibly at the diode node: dc = {dc:.6}"
    );
    assert!(
        (h1.norm() - 0.45).abs() < 0.01 * 0.45,
        "fundamental must pass at the physical amplitude: |h1| = {:.6}",
        h1.norm()
    );
    assert!(
        h2 < 2e-3,
        "second harmonic must be sub-millivolt at this drive: |h2| = {h2:.6}"
    );
}

#[test]
fn rshunt_is_present_in_the_harmonic_balance_conductance_operator() {
    let deck = "\
* RSHUNT-only HB transfer
i1 0 out sin(0 1m 1meg)
.options rshunt=1k
    .end
";
    let result = run_hb(deck, 1.0e6, 4);
    let fundamental = coefficient(&result, "out", 1);
    assert!(
        result.converged,
        "HB must converge; computed fundamental was {fundamental}"
    );
    assert!(
        (fundamental.norm() - 1.0).abs() <= 1.0e-6,
        "1 mA through the 1 kOhm RSHUNT must produce a 1 V peak, got {fundamental}"
    );
}

#[test]
fn high_impedance_linear_hb_response_is_not_discarded_as_a_tiny_pivot() {
    let deck = "\
* 1 A through 1 EOhm is a valid, deliberately scaled HB problem
i1 0 out sin(0 1 1meg)
r1 out 0 1e18
.end
";
    let result = run_hb(deck, 1.0e6, 2);
    let fundamental = coefficient(&result, "out", 1);

    assert!(result.converged, "high-impedance HB solve must converge");
    assert!(
        (fundamental.norm() - 1.0e18).abs() <= 1.0e3,
        "1 A through 1 EOhm must produce a 1e18 V amplitude, got {fundamental}"
    );
}

#[test]
fn subpicovolt_hb_drive_is_not_reclassified_as_zero() {
    let deck = "\
* exact source topology must be invariant under physical scaling
v1 in 0 sin(0 1e-15 1meg)
r1 in out 1k
r2 out 0 1k
.end
";
    let result = run_hb(deck, 1.0e6, 2);
    let input = coefficient(&result, "in", 1);
    let output = coefficient(&result, "out", 1);

    assert!(input.norm() > 0.0, "the authored 1 fV drive was erased");
    assert!(
        (output / input - num_complex::Complex64::new(0.5, 0.0)).norm() < 1.0e-12,
        "scaled divider transfer changed: input={input}, output={output}"
    );
}

#[test]
fn distortion_metadata_preserves_the_wrapped_hb_waveform() {
    let bare = run_hb(
        "wrapped-source reference\n\
         v1 out 0 sin(0.25 0.1 1meg)\n\
         r1 out 0 1k\n\
         .end\n",
        1.0e6,
        2,
    );
    let wrapped = run_hb(
        "wrapped distortion source\n\
         v1 out 0 sin(0.25 0.1 1meg) distof1 2m 30\n\
         r1 out 0 1k\n\
         .end\n",
        1.0e6,
        2,
    );

    for harmonic in 0..=2 {
        assert_eq!(
            coefficient(&wrapped, "out", harmonic),
            coefficient(&bare, "out", harmonic),
            "DISTOF metadata changed HB harmonic {harmonic}"
        );
    }
}

#[test]
fn unsupported_time_waveforms_fail_instead_of_becoming_dc() {
    for (source, waveform, expected) in [
        ("v1", "exp(0 1 1n 1n 2n 1n)", "EXP is not periodic"),
        ("v1", "pwl(0 0 1u 1 2u 0) r=0", "periodic PWL"),
        ("v1", "trnoise(1m 1n 0 0)", "TRNOISE is stochastic"),
        ("i1", "trrandom(2 1u 2u 3 4)", "TRRANDOM is stochastic"),
    ] {
        let deck =
            format!("unsupported HB waveform\n{source} out 0 {waveform}\nr1 out 0 1k\n.end\n");
        let netlist = Netlist::parse(&deck).expect("unsupported waveform deck parses");
        let error = match Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
        {
            Ok(_) => panic!("unsupported HB waveform {waveform} must not silently become DC"),
            Err(error) => error,
        };
        assert!(
            error.to_string().contains(expected),
            "unexpected waveform error for {waveform}: {error}"
        );
    }
}

#[test]
fn floating_linear_hb_component_is_reported_as_singular() {
    let deck = "\
* The driven component is grounded; f1-f2 is a separate floating island.
v1 driven 0 ac 1
r1 driven 0 1k
rfloat f1 f2 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
        .expect_err("a non-unique floating HB component must fail closed");

    assert!(
        error.to_string().to_ascii_lowercase().contains("singular"),
        "HB must surface the singular solve instead of publishing zeros: {error}"
    );
}

#[test]
fn nonlinear_hb_is_certified_without_a_final_gmin_shunt() {
    let deck = "\
* Reverse-biased weak diode selects nonlinear HB without loading the node.
i1 0 out dc 1p
r1 out 0 1e12
d1 0 out dweak
.model dweak d is=1e-30 n=1 cj0=0 tt=0
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut config = HbConfig::new(1.0e6).with_harmonics(2);
    config.tolerance = 1.0e-9;
    config.abstol = 1.0e-21;
    let result = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, config)
        .expect("nonlinear high-impedance HB converges");
    let dc = coefficient(&result, "out", 0);

    assert!(result.converged);
    assert!(
        (dc.re - 1.0).abs() <= 1.0e-6,
        "1 pA through 1 TOhm must remain 1 V after nonlinear HB, got {dc}"
    );
    assert_eq!(dc.im, 0.0);
}
