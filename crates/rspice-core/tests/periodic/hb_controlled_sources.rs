//! Exact controlled-source equations through HB and its periodic consumers.

use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const OFFSET: f64 = 1.0e4;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("controlled-source fixture parses")
}

fn hb_coefficient(result: &HbAnalysisResult, node: &str, harmonic: usize) -> Complex64 {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing HB node '{node}'"))
        .coefficients[harmonic]
}

fn hb_branch_coefficient(result: &HbAnalysisResult, branch: &str, harmonic: usize) -> Complex64 {
    result
        .result
        .mna_branch_currents
        .iter()
        .find(|spectrum| spectrum.device_name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing HB branch '{branch}'"))
        .coefficients[harmonic]
}

fn assert_complex_relative(actual: Complex64, expected: Complex64, tolerance: f64, label: &str) {
    let scale = expected.norm().max(1.0);
    assert!(
        (actual - expected).norm() <= tolerance * scale,
        "{label}: actual={actual}, expected={expected}"
    );
}

fn all_families_deck() -> &'static str {
    "exact HB controlled sources\n\
     VDRIVE ctrl 0 PULSE(-0.25 0.75 0 1n 1n 0.498u 1u)\n\
     RCTRL ctrl 0 1k\n\
     G1 gout 0 ctrl 0 2m\n\
     RG gout 0 200\n\
     E1 eout 0 ctrl 0 3\n\
     RE eout 0 1k\n\
     F1 fout 0 VDRIVE 2\n\
     RF fout 0 500\n\
     H1 hout 0 VDRIVE 400\n\
     RH hout 0 1k\n\
     .end\n"
}

#[test]
fn hb_stamps_all_four_controlled_source_families_at_every_retained_harmonic() {
    let result = engine()
        .run_hb(
            &parse(all_families_deck()),
            HbConfig::new(F0).with_harmonics(4),
        )
        .expect("controlled-source HB completes");
    assert!(result.converged);
    assert!(
        hb_coefficient(&result, "ctrl", 3).norm() > 0.05,
        "the pulse fixture must exercise a nonzero third harmonic"
    );

    for harmonic in 0..=4 {
        let control = hb_coefficient(&result, "ctrl", harmonic);
        for (node, gain) in [("gout", -0.4), ("eout", 3.0), ("fout", 1.0), ("hout", -0.4)] {
            assert_complex_relative(
                hb_coefficient(&result, node, harmonic),
                control * gain,
                2.0e-10,
                &format!("{node} harmonic {harmonic}"),
            );
        }
    }

    assert_eq!(
        result.operating_point.mna_branch_names(),
        ["VDRIVE", "E1", "H1"]
    );
    for harmonic in 0..=4 {
        assert_complex_relative(
            hb_branch_coefficient(&result, "VDRIVE", harmonic),
            -hb_coefficient(&result, "ctrl", harmonic) / 1.0e3,
            2.0e-10,
            &format!("control branch harmonic {harmonic}"),
        );
    }
}

#[test]
fn pac_and_pxf_conversion_matrix_preserve_each_controlled_source_gain() {
    let netlist = parse(all_families_deck());
    for (node, gain) in [("gout", -0.4), ("eout", 3.0), ("fout", 1.0), ("hout", -0.4)] {
        let config = PacConfig::new()
            .with_fundamental(F0)
            .with_sweep(OFFSET, OFFSET, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(-1, 1)
            .with_input_source("VDRIVE")
            .with_output_node(node);
        let result = engine()
            .run_pac(&netlist, config)
            .unwrap_or_else(|error| panic!("PAC for {node} completes: {error}"));
        for sideband in -1..=1 {
            let transfer = result
                .result
                .conversion_matrix
                .get(0, sideband, sideband)
                .expect("diagonal PXF conversion coefficient is retained");
            assert_complex_relative(
                transfer,
                Complex64::new(gain, 0.0),
                2.0e-10,
                &format!("PAC/PXF transfer to {node} at sideband {sideband}"),
            );
        }
    }
}

fn pnoise_contribution(deck: &str, output: &str) -> f64 {
    let result = engine()
        .run_pnoise(&parse(deck), F0, &[OFFSET], output, None, None, 0)
        .unwrap_or_else(|error| panic!("PNoise for {output} completes: {error}"));
    let contribution = result
        .contributors
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RNOISE thermal"))
        .unwrap_or_else(|| panic!("missing RNOISE contributor in {:?}", result.contributors))
        .1[0];
    assert_complex_relative(
        Complex64::new(result.output_noise[0], 0.0),
        Complex64::new(contribution, 0.0),
        2.0e-10,
        "single-source total PNoise",
    );
    contribution
}

#[test]
fn pnoise_adjoint_includes_voltage_and_current_controlled_couplings() {
    let voltage_noise = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1.0e3;
    let current_noise = 4.0 * K_BOLTZMANN * TEMP_REFERENCE / 1.0e3;
    let cases = [
        (
            "VCCS",
            "VCCS pnoise\nRNOISE ctrl 0 1k\nG1 out 0 ctrl 0 2m\nROUT out 0 200 NOISY=0\n.end\n",
            0.4_f64.powi(2) * voltage_noise,
        ),
        (
            "VCVS",
            "VCVS pnoise\nRNOISE ctrl 0 1k\nE1 out 0 ctrl 0 3\nROUT out 0 200 NOISY=0\n.end\n",
            3.0_f64.powi(2) * voltage_noise,
        ),
        (
            "CCCS",
            "CCCS pnoise\nVCTRL ctrl 0 0\nRNOISE ctrl 0 1k\nF1 out 0 VCTRL 2\nROUT out 0 200 NOISY=0\n.end\n",
            (2.0_f64 * 200.0).powi(2) * current_noise,
        ),
        (
            "CCVS",
            "CCVS pnoise\nVCTRL ctrl 0 0\nRNOISE ctrl 0 1k\nH1 out 0 VCTRL 400\nROUT out 0 200 NOISY=0\n.end\n",
            400.0_f64.powi(2) * current_noise,
        ),
    ];

    for (family, deck, expected) in cases {
        let actual = pnoise_contribution(deck, "out");
        let relative = (actual - expected).abs() / expected;
        assert!(
            actual.is_finite() && relative <= 2.0e-8,
            "{family} PNoise: actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
        );
    }
}
