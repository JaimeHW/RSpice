//! Exact K-card mutual-inductance equations through HB and periodic consumers.

use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const OFFSET: f64 = 1.0e4;
const L1: f64 = 100.0e-6;
const L2: f64 = 25.0e-6;
const K: f64 = 0.8;
const RS: f64 = 20.0;
const RL: f64 = 50.0;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("mutual-inductance fixture parses")
}

fn driven_deck() -> &'static str {
    "exact periodic mutual inductance\n\
     VDRIVE drive 0 PULSE(-0.2 0.8 0 1n 1n 0.498u 1u)\n\
     RS drive pri 20 NOISY=0\n\
     LPRI pri 0 100u\n\
     LSEC sec 0 25u\n\
     K1 LPRI LSEC 0.8\n\
     RLOAD sec 0 50 NOISY=0\n\
     .end\n"
}

fn transfer(frequency: f64) -> Complex64 {
    let jw = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
    let mutual = K * (L1 * L2).sqrt();
    let z11 = jw * L1;
    let z22 = jw * L2;
    let z12 = jw * mutual;
    let secondary_per_primary_current = z12 / (Complex64::new(1.0, 0.0) + z22 / RL);
    let primary_per_primary_current = z11 - z12 * secondary_per_primary_current / RL;
    secondary_per_primary_current / (RS + primary_per_primary_current)
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

fn assert_complex_relative(actual: Complex64, expected: Complex64, tolerance: f64, label: &str) {
    let scale = expected.norm().max(1.0e-12);
    assert!(
        (actual - expected).norm() <= tolerance * scale,
        "{label}: actual={actual}, expected={expected}"
    );
}

#[test]
fn hb_k_card_stamps_mutual_terms_at_every_retained_harmonic() {
    let result = engine()
        .run_hb(&parse(driven_deck()), HbConfig::new(F0).with_harmonics(4))
        .expect("coupled-inductor HB completes");
    assert!(result.converged);
    assert!(hb_coefficient(&result, "drive", 3).norm() > 0.05);
    assert!(hb_coefficient(&result, "sec", 0).norm() <= 1.0e-10);
    for harmonic in 1..=4 {
        let expected = hb_coefficient(&result, "drive", harmonic) * transfer(harmonic as f64 * F0);
        assert_complex_relative(
            hb_coefficient(&result, "sec", harmonic),
            expected,
            3.0e-9,
            &format!("secondary harmonic {harmonic}"),
        );
    }
    assert_eq!(
        result.operating_point.mna_branch_names(),
        ["VDRIVE", "LPRI", "LSEC"]
    );
}

#[test]
fn pac_and_pxf_include_mutual_impedance_at_each_sideband() {
    let result = engine()
        .run_pac(
            &parse(driven_deck()),
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(OFFSET, OFFSET, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("VDRIVE")
                .with_output_node("sec"),
        )
        .expect("coupled-inductor PAC completes");
    for sideband in -1..=1 {
        let actual = result
            .result
            .conversion_matrix
            .get(0, sideband, sideband)
            .expect("diagonal PXF conversion coefficient is retained");
        let expected = transfer(OFFSET + sideband as f64 * F0);
        assert_complex_relative(
            actual,
            expected,
            3.0e-9,
            &format!("PAC/PXF sideband {sideband}"),
        );
    }
}

#[test]
fn pnoise_adjoint_includes_mutual_inductance_transfer() {
    let deck = "coupled-inductor pnoise\n\
                RNOISE pri 0 20\n\
                LPRI pri 0 100u\n\
                LSEC sec 0 25u\n\
                K1 LPRI LSEC 0.8\n\
                RLOAD sec 0 50 NOISY=0\n\
                .end\n";
    let result = engine()
        .run_pnoise(&parse(deck), F0, &[OFFSET], "sec", None, None, 0)
        .expect("coupled-inductor PNoise completes");
    let actual = result
        .contributors
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RNOISE thermal"))
        .expect("RNOISE contributor is retained")
        .1[0];
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * RS * transfer(OFFSET).norm_sqr();
    let relative = (actual - expected).abs() / expected;
    assert!(
        actual.is_finite() && relative <= 3.0e-8,
        "PNoise mutual transfer: actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
    );
}
