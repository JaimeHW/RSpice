//! Physical public-API oracles for Volterra harmonic products.

use rspice_core::analysis::{DistortionAnalysisResult, DistortionProduct};
use rspice_core::constants::{TEMP_REFERENCE, thermal_voltage};
use rspice_core::{Engine, Netlist};

const BIAS: f64 = 0.5;
const SATURATION_CURRENT: f64 = 1.0e-12;

fn run_diode(amplitude: f64, frequencies: &[f64]) -> DistortionAnalysisResult {
    let deck = format!(
        "diode distortion public oracle\n\
         V1 out 0 DC {BIAS:.17e} DISTOF1 {amplitude:.17e} 0\n\
         D1 out 0 DM\n\
         .model DM D(IS={SATURATION_CURRENT:.17e} N=1 CJO=0 TT=0)\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("diode distortion oracle parses");
    Engine::default()
        .run_distortion(&netlist, frequencies, None)
        .expect("diode distortion oracle solves")
}

fn expected_product_current(amplitude: f64, product: DistortionProduct) -> f64 {
    let vt = thermal_voltage(TEMP_REFERENCE);
    let bias_current = SATURATION_CURRENT * (BIAS / vt).exp();
    match product {
        DistortionProduct::SecondHarmonic => bias_current * amplitude.powi(2) / (4.0 * vt.powi(2)),
        DistortionProduct::ThirdHarmonic => bias_current * amplitude.powi(3) / (24.0 * vt.powi(3)),
        _ => panic!("harmonic-mode oracle received a two-tone product"),
    }
}

fn response_indices(
    result: &DistortionAnalysisResult,
    product: DistortionProduct,
) -> (usize, usize) {
    let response = &result.points[0]
        .product(product)
        .expect("requested harmonic product is retained")
        .response;
    let node = response
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("output node is retained");
    let branch = response
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("V1"))
        .expect("driving voltage-source branch is retained");
    (node, branch)
}

#[test]
fn clamped_diode_voltage_is_zero_but_branch_products_match_closed_form() {
    const AMPLITUDE: f64 = 1.0e-3;
    let frequencies = [1.0e3, 2.0e3];
    let result = run_diode(AMPLITUDE, &frequencies);

    for product in [
        DistortionProduct::SecondHarmonic,
        DistortionProduct::ThirdHarmonic,
    ] {
        let (node, branch) = response_indices(&result, product);
        let expected = expected_product_current(AMPLITUDE, product);
        let frequency_multiplier = product.order() as f64;

        for (point, fundamental) in result.points.iter().zip(frequencies) {
            let response = &point
                .product(product)
                .expect("requested harmonic product is retained")
                .response;
            assert_eq!(response.frequency, frequency_multiplier * fundamental);
            assert_eq!(
                response.voltages[node].re, 0.0,
                "the ideal V1 source must clamp the product voltage"
            );
            assert_eq!(
                response.voltages[node].im, 0.0,
                "the ideal V1 source must clamp the product voltage"
            );

            let actual = response.currents[branch].norm();
            let relative_error = (actual - expected).abs() / expected;
            let tolerance = if product == DistortionProduct::SecondHarmonic {
                2.0e-5
            } else {
                2.0e-3
            };
            assert!(
                actual > 0.0 && relative_error < tolerance,
                "{} branch product {actual:.12e}, expected {expected:.12e}, relerr={relative_error:.3e}",
                product.label()
            );
        }
    }
}

#[test]
fn diode_harmonics_scale_with_their_volterra_order() {
    const BASE_AMPLITUDE: f64 = 0.5e-3;
    const SCALE: f64 = 2.0;
    let base = run_diode(BASE_AMPLITUDE, &[1.0e3]);
    let scaled = run_diode(SCALE * BASE_AMPLITUDE, &[1.0e3]);

    for product in [
        DistortionProduct::SecondHarmonic,
        DistortionProduct::ThirdHarmonic,
    ] {
        let (_, branch) = response_indices(&base, product);
        let base_current = base.points[0]
            .product(product)
            .expect("base product")
            .response
            .currents[branch]
            .norm();
        let scaled_current = scaled.points[0]
            .product(product)
            .expect("scaled product")
            .response
            .currents[branch]
            .norm();
        let expected_ratio = SCALE.powi(product.order() as i32);
        let actual_ratio = scaled_current / base_current;
        assert!(
            (actual_ratio - expected_ratio).abs() <= 3.0e-12 * expected_ratio,
            "{} scaled by {actual_ratio:.12e}, expected {expected_ratio:.12e}",
            product.label()
        );
    }
}
