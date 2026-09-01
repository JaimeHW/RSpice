//! Exact distributed transmission-line equations through HB and its periodic
//! AC, conversion, and noise consumers.

use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const OFFSET: f64 = 10.0e3;
const Z0: f64 = 50.0;
const TD: f64 = 137.0e-9;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("transmission-line fixture parses")
}

fn scalar_deck() -> &'static str {
    "exact periodic delay line\n\
     VDRIVE drive 0 PULSE(-0.2 0.8 0 1n 1n 0.498u 1u)\n\
     RS drive near 50 NOISY=0\n\
     T1 near 0 far 0 Z0=50 TD=137n\n\
     RLOAD far 0 50 NOISY=0\n\
     .end\n"
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

fn delay_transfer(frequency: f64) -> Complex64 {
    0.5 * Complex64::from_polar(1.0, -2.0 * std::f64::consts::PI * frequency * TD)
}

fn assert_complex_relative(actual: Complex64, expected: Complex64, tolerance: f64, label: &str) {
    let scale = expected.norm().max(1.0e-12);
    assert!(
        actual.re.is_finite()
            && actual.im.is_finite()
            && (actual - expected).norm() <= tolerance * scale,
        "{label}: actual={actual}, expected={expected}"
    );
}

#[test]
fn hb_lossless_delay_line_has_exact_dc_and_multi_harmonic_phase() {
    let result = engine()
        .run_hb(&parse(scalar_deck()), HbConfig::new(F0).with_harmonics(4))
        .expect("lossless delay-line HB completes");
    assert!(result.converged);
    assert!(hb_coefficient(&result, "drive", 3).norm() > 0.05);
    for harmonic in 0..=4 {
        let expected =
            hb_coefficient(&result, "drive", harmonic) * delay_transfer(harmonic as f64 * F0);
        assert_complex_relative(
            hb_coefficient(&result, "far", harmonic),
            expected,
            5.0e-9,
            &format!("lossless line harmonic {harmonic}"),
        );
    }
}

#[test]
fn pac_and_pxf_use_exact_delay_at_positive_and_negative_sidebands() {
    let result = engine()
        .run_pac(
            &parse(scalar_deck()),
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(OFFSET, OFFSET, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("VDRIVE")
                .with_output_node("far"),
        )
        .expect("delay-line PAC completes");
    for sideband in -1..=1 {
        let actual = result
            .result
            .conversion_matrix
            .get(0, sideband, sideband)
            .expect("diagonal PXF coefficient is retained");
        assert_complex_relative(
            actual,
            delay_transfer(OFFSET + sideband as f64 * F0),
            5.0e-9,
            &format!("PAC/PXF delay at sideband {sideband}"),
        );
    }
}

#[test]
fn pnoise_adjoint_transfers_matched_source_noise_without_line_loss() {
    let deck = "exact delay-line pnoise\n\
                RNOISE near 0 50\n\
                T1 near 0 far 0 Z0=50 TD=137n\n\
                RLOAD far 0 50 NOISY=0\n\
                .end\n";
    let result = engine()
        .run_pnoise(&parse(deck), F0, &[OFFSET], "far", None, None, 0)
        .expect("delay-line PNoise completes");
    let actual = result
        .contributors
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RNOISE thermal"))
        .expect("source resistor contributor is retained")
        .1[0];
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * Z0 * 0.25;
    let relative = (actual - expected).abs() / expected;
    assert!(
        actual.is_finite() && relative <= 5.0e-8,
        "delay-line PNoise actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
    );
}

fn ltra_transfer(frequency: f64) -> Complex64 {
    const RS: f64 = 50.0;
    const RL: f64 = 75.0;
    const R_TOTAL: f64 = 10.0;
    const L_TOTAL: f64 = 2.5e-6;
    const C_TOTAL: f64 = 1.0e-9;
    if frequency == 0.0 {
        return Complex64::new(RL / (RS + R_TOTAL + RL), 0.0);
    }
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let z = Complex64::new(R_TOTAL, omega * L_TOTAL);
    let y = Complex64::new(0.0, omega * C_TOTAL);
    let gamma = (z * y).sqrt();
    let y0 = (y / z).sqrt();
    let z0 = Complex64::new(1.0, 0.0) / y0;
    Complex64::new(1.0, 0.0) / (gamma.cosh() * (1.0 + RS / RL) + gamma.sinh() * (z0 / RL + RS * y0))
}

fn ltra_deck() -> &'static str {
    "exact periodic LTRA\n\
     VDRIVE drive 0 PULSE(-0.2 0.8 0 1n 1n 0.498u 1u)\n\
     RS drive near 50 NOISY=0\n\
     O1 near 0 far 0 line\n\
     RLOAD far 0 75 NOISY=0\n\
     .model line ltra r=10 l=2.5u g=0 c=1n len=1 rel=1\n\
     .end\n"
}

#[test]
fn hb_ltra_uses_exact_telegrapher_operator_at_dc_and_each_harmonic() {
    let result = engine()
        .run_hb(&parse(ltra_deck()), HbConfig::new(F0).with_harmonics(3))
        .expect("LTRA HB completes");
    for harmonic in 0..=3 {
        let expected =
            hb_coefficient(&result, "drive", harmonic) * ltra_transfer(harmonic as f64 * F0);
        assert_complex_relative(
            hb_coefficient(&result, "far", harmonic),
            expected,
            2.0e-8,
            &format!("LTRA harmonic {harmonic}"),
        );
    }
}

#[test]
fn pac_and_pxf_use_exact_ltra_operator_at_each_sideband() {
    let result = engine()
        .run_pac(
            &parse(ltra_deck()),
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(OFFSET, OFFSET, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("VDRIVE")
                .with_output_node("far"),
        )
        .expect("LTRA PAC completes");
    for sideband in -1..=1 {
        let actual = result
            .result
            .conversion_matrix
            .get(0, sideband, sideband)
            .expect("LTRA diagonal PXF coefficient is retained");
        assert_complex_relative(
            actual,
            ltra_transfer(OFFSET + sideband as f64 * F0),
            2.0e-8,
            &format!("LTRA PAC/PXF sideband {sideband}"),
        );
    }
}

#[test]
fn pnoise_adjoint_uses_exact_ltra_transfer() {
    let deck = "exact LTRA pnoise\n\
                RNOISE near 0 50\n\
                O1 near 0 far 0 line\n\
                RLOAD far 0 75 NOISY=0\n\
                .model line ltra r=10 l=2.5u g=0 c=1n len=1 rel=1\n\
                .end\n";
    let result = engine()
        .run_pnoise(&parse(deck), F0, &[OFFSET], "far", None, None, 0)
        .expect("LTRA PNoise completes");
    let actual = result
        .contributors
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RNOISE thermal"))
        .expect("LTRA source resistor contributor is retained")
        .1[0];
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * Z0 * ltra_transfer(OFFSET).norm_sqr();
    let relative = (actual - expected).abs() / expected;
    assert!(
        actual.is_finite() && relative <= 5.0e-8,
        "LTRA PNoise actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
    );
}

const LOSSLESS_CPL_DECK: &str = "exact lossless CPL\n\
    VDRIVE drive 0 PULSE(-0.2 0.8 0 1n 1n 0.498u 1u)\n\
    RS drive near1 50 NOISY=0\n\
    P1 near1 near2 0 far1 far2 0 line\n\
    RLOAD far1 0 50 NOISY=0\n\
    RN2 near2 0 94.8683298051 NOISY=0\n\
    RF2 far2 0 94.8683298051 NOISY=0\n\
    .model line cpl\n\
    + r = 0 0 0\n\
    + l = 2.5u 0 9u\n\
    + c = 1n 0 1n\n\
    + g = 0 0 0\n\
    + length = 1\n\
    .end\n";

fn cpl_conductor1_transfer(frequency: f64) -> Complex64 {
    let delay = (2.5e-6_f64 * 1.0e-9).sqrt();
    0.5 * Complex64::from_polar(1.0, -2.0 * std::f64::consts::PI * frequency * delay)
}

#[test]
fn lossless_cpl_retains_authored_zero_r_and_g_for_periodic_analysis() {
    let result = engine()
        .run_hb(
            &parse(LOSSLESS_CPL_DECK),
            HbConfig::new(F0).with_harmonics(3),
        )
        .expect("lossless CPL HB completes");
    assert_eq!(
        result.operating_point.mna_branch_names(),
        ["VDRIVE", "P1#b1[1]", "P1#b2[1]", "P1#b1[2]", "P1#b2[2]"],
        "periodic CPL must preserve the canonical authored branch registry order"
    );
    for harmonic in 0..=3 {
        let expected = hb_coefficient(&result, "drive", harmonic)
            * cpl_conductor1_transfer(harmonic as f64 * F0);
        assert_complex_relative(
            hb_coefficient(&result, "far1", harmonic),
            expected,
            2.0e-8,
            &format!("lossless CPL conductor harmonic {harmonic}"),
        );
    }
}

#[test]
fn pac_and_pxf_use_lossless_cpl_modal_delay_at_each_sideband() {
    let result = engine()
        .run_pac(
            &parse(LOSSLESS_CPL_DECK),
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(OFFSET, OFFSET, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("VDRIVE")
                .with_output_node("far1"),
        )
        .expect("lossless CPL PAC completes");
    for sideband in -1..=1 {
        let actual = result
            .result
            .conversion_matrix
            .get(0, sideband, sideband)
            .expect("lossless CPL diagonal PXF coefficient is retained");
        assert_complex_relative(
            actual,
            cpl_conductor1_transfer(OFFSET + sideband as f64 * F0),
            2.0e-8,
            &format!("lossless CPL PAC/PXF sideband {sideband}"),
        );
    }
}

#[test]
fn pnoise_adjoint_uses_lossless_cpl_modal_operator() {
    let deck = "exact lossless CPL pnoise\n\
                RNOISE near1 0 50\n\
                P1 near1 near2 0 far1 far2 0 line\n\
                RLOAD far1 0 50 NOISY=0\n\
                RN2 near2 0 94.8683298051 NOISY=0\n\
                RF2 far2 0 94.8683298051 NOISY=0\n\
                .model line cpl\n\
                + r = 0 0 0\n\
                + l = 2.5u 0 9u\n\
                + c = 1n 0 1n\n\
                + g = 0 0 0\n\
                + length = 1\n\
                .end\n";
    let result = engine()
        .run_pnoise(&parse(deck), F0, &[OFFSET], "far1", None, None, 0)
        .expect("lossless CPL PNoise completes");
    let actual = result
        .contributors
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RNOISE thermal"))
        .expect("lossless CPL source resistor contributor is retained")
        .1[0];
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * Z0 * 0.25;
    let relative = (actual - expected).abs() / expected;
    assert!(
        actual.is_finite() && relative <= 5.0e-8,
        "lossless CPL PNoise actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
    );
}

#[test]
fn native_txl_and_lossy_cpl_fail_closed_with_retained_state_diagnostics() {
    let txl = "unsupported exact TXL\n\
               V1 near 0 1\n\
               O1 near 0 far 0 line\n\
               R1 far 0 50\n\
               .model line txl r=12.45 l=8.972n g=0 c=0.468p len=16\n\
               .end\n";
    let txl_error = engine()
        .run_hb(&parse(txl), HbConfig::new(F0).with_harmonics(1))
        .expect_err("native TXL must fail before periodic solving")
        .to_string();
    assert!(
        txl_error.contains("TXL") && txl_error.contains("physical frequency-domain RLGC"),
        "unexpected TXL diagnostic: {txl_error}"
    );
    let txl_pac_error = engine()
        .run_pac(
            &parse(txl),
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(OFFSET, OFFSET, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(0, 0)
                .with_input_source("V1")
                .with_output_node("far"),
        )
        .expect_err("native TXL must fail before PAC solving")
        .to_string();
    assert!(
        txl_pac_error.contains("TXL") && txl_pac_error.contains("physical frequency-domain RLGC"),
        "unexpected TXL PAC diagnostic: {txl_pac_error}"
    );

    let lossy_cpl = "unsupported lossy CPL\n\
                     V1 near1 0 1\n\
                     P1 near1 near2 0 far1 far2 0 line\n\
                     R1 far1 0 50\n\
                     RN2 near2 0 50\n\
                     RF2 far2 0 50\n\
                     .model line cpl\n\
                     + r = 1 0 1\n\
                     + l = 2.5u 0 2.5u\n\
                     + c = 1n 0 1n\n\
                     + g = 0 0 0\n\
                     + length = 1\n\
                     .end\n";
    let cpl_error = engine()
        .run_hb(&parse(lossy_cpl), HbConfig::new(F0).with_harmonics(1))
        .expect_err("lossy CPL must fail before periodic solving")
        .to_string();
    assert!(
        cpl_error.contains("lossy CPL") && cpl_error.contains("physical RLGC"),
        "unexpected CPL diagnostic: {cpl_error}"
    );
    let cpl_pnoise_error = engine()
        .run_pnoise(&parse(lossy_cpl), F0, &[OFFSET], "far1", None, None, 0)
        .expect_err("lossy CPL must fail before PNoise solving")
        .to_string();
    assert!(
        cpl_pnoise_error.contains("lossy CPL") && cpl_pnoise_error.contains("physical RLGC"),
        "unexpected CPL PNoise diagnostic: {cpl_pnoise_error}"
    );
}
