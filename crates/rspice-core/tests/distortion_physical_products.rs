//! Physical public-API oracles for Volterra harmonic products.

use rspice_core::analysis::{DistortionAnalysisResult, DistortionProduct};
use rspice_core::constants::{TEMP_REFERENCE, thermal_voltage};
use rspice_core::{Engine, Netlist};

const BIAS: f64 = 0.5;
const SATURATION_CURRENT: f64 = 1.0e-12;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn distortion_honors_cancellation_from_its_last_progress_callback() {
    use rspice_core::{AbortSignal, SimulationError};
    use std::sync::atomic::{AtomicBool, Ordering};

    struct CancelAtCompletion(AtomicBool);
    impl AbortSignal for CancelAtCompletion {
        fn is_aborted(&self) -> bool {
            self.0.load(Ordering::Relaxed)
        }

        fn observe_progress(&self, fraction: f64) {
            if fraction == 1.0 {
                self.0.store(true, Ordering::Relaxed);
            }
        }
    }

    let netlist = Netlist::parse(
        "cancel completed distortion point\nV1 in 0 DISTOF1 1 DISTOF2 .5\nR1 in out 1k\nC1 out 0 1n\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    for frequencies in [&[1e3][..], &[1e3, 2e3][..]] {
        for ratio in [None, Some(0.9)] {
            assert!(engine.run_distortion(&netlist, frequencies, ratio).is_ok());
            let abort = CancelAtCompletion(AtomicBool::new(false));
            let result = engine.run_distortion_with_abort(&netlist, frequencies, ratio, &abort);
            assert!(abort.is_aborted(), "the final progress callback must run");
            assert!(
                matches!(result, Err(SimulationError::Aborted)),
                "a cancelled harmonic or two-tone sweep must not publish success: {result:?}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn tied_current_terminals_cannot_erase_distortion_input_tones() {
    for terminals in ["out out", "0 0"] {
        let netlist = Netlist::parse(&format!(
            "tied distortion source\nI1 0 out DC 0 DISTOF1 1 37 DISTOF2 .5 -90\nI2 {terminals} DC 0 DISTOF1 1e100 37 DISTOF2 1e100 -90\nR1 out 0 1\n.end\n"
        )).unwrap();
        let result = Engine::default()
            .run_distortion(&netlist, &[1e3], Some(0.9))
            .unwrap();
        let point = &result.points[0];
        for (response, amplitude, phase) in [
            (&point.fundamental_f1, 1.0, 37.0_f64),
            (point.fundamental_f2.as_ref().unwrap(), 0.5, -90.0),
        ] {
            let node = response
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let expected = rspice_core::Complex64::from_polar(amplitude, phase.to_radians());
            assert!((response.voltages[node] - expected).norm() < 1e-12);
        }
        assert!(point.products.iter().all(|product| {
            product
                .response
                .voltages
                .iter()
                .all(|value| value.norm() < 1e-14)
        }));
    }
}

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn reactive_loading_preserves_small_polynomial_and_cascaded_products() {
    use rspice_core::{Complex64 as C, SimulationConfig, SpiceDialect};

    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for (a1, a2, p1, p2) in [(0.01, 0.003, 0.0_f64, 0.0_f64), (0.003, 0.01, 37.0, -61.0)] {
            for capacitance in [1e-4, 1e-3] {
                for (quadratic, cubic) in [(0.3, 0.4), (0.3, 0.0), (0.0, 0.0)] {
                    let deck = Netlist::parse(&format!(
                        "reactive polynomial distortion\nV1 in 0 DC 0 DISTOF1 {a1} {p1} DISTOF2 {a2} {p2}\n\
                         R1 in out 1\nB1 out 0 I={{{quadratic}*v(out)^2+{cubic}*v(out)^3}}\n\
                         C1 out 0 {capacitance}\n.options GMIN=0\n.end\n"
                    )).unwrap();
                    for f1 in [1e3, 1.4e3, 2.1e3, 1e4] {
                        let f2 = 700.0;
                        // Expand I = q*v^2 + c*v^3 in peak complex phasors.
                        // These equations include both direct curvature and the
                        // cascaded quadratic response through the RC feedback.
                        let linear = |f| C::new(1.0, std::f64::consts::TAU * f * capacitance);
                        let u1 = C::from_polar(a1, p1.to_radians()) / linear(f1);
                        let u2 = C::from_polar(a2, p2.to_radians()) / linear(f2);
                        let h2 = -0.5 * quadratic * u1 * u1 / linear(2.0 * f1);
                        let sum = -quadratic * u1 * u2 / linear(f1 + f2);
                        let diff = -quadratic * u1 * u2.conj() / linear(f1 - f2);
                        let h3 =
                            -(quadratic * u1 * h2 + 0.25 * cubic * u1 * u1 * u1) / linear(3.0 * f1);
                        let im3 = -(quadratic * (u1 * diff + u2.conj() * h2)
                            + 0.75 * cubic * u1 * u1 * u2.conj())
                            / linear(2.0 * f1 - f2);
                        for ratio in [None, Some(f2 / f1)] {
                            let result = engine.run_distortion(&deck, &[f1], ratio).unwrap();
                            let point = &result.points[0];
                            let node = point
                                .fundamental_f1
                                .node_names
                                .iter()
                                .position(|name| name.eq_ignore_ascii_case("out"))
                                .unwrap();
                            assert!(
                                (point.fundamental_f1.voltages[node] - u1).norm()
                                    < 1e-12 * u1.norm()
                            );
                            let products = if ratio.is_some() {
                                vec![
                                    (DistortionProduct::Sum, sum),
                                    (DistortionProduct::Difference, diff),
                                    (DistortionProduct::ThirdOrderDifference, im3),
                                ]
                            } else {
                                vec![
                                    (DistortionProduct::SecondHarmonic, h2),
                                    (DistortionProduct::ThirdHarmonic, h3),
                                ]
                            };
                            for (product, expected) in products {
                                let actual =
                                    point.product(product).unwrap().response.voltages[node];
                                // Do not let an absolute floor hide tiny higher-order
                                // products. The purely linear case must remain zero.
                                let tolerance = if product.order() == 2 { 2e-5 } else { 2e-3 };
                                let error = (actual - expected).norm();
                                assert!(
                                    error <= tolerance * expected.norm(),
                                    "{dialect:?}, C={capacitance}, q={quadratic}, c={cubic}, F1={f1}, \
                                     A1={a1}, A2={a2}, {product:?}: {actual:?}, expected {expected:?}, error={error}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn saturated_outer_probes_cannot_erase_resolved_local_curvature() {
    use rspice_core::{Complex64 as C, SimulationConfig, SpiceDialect};
    let bias = 0.01;
    let slope = 100.0_f64;
    let scale = 1e-8;
    let t = (slope * bias).tanh();
    let g = 1.0 - t * t;
    let second = -2.0 * scale * slope.powi(2) * t * g;
    let third = scale * slope.powi(3) * (-2.0 * g * g + 4.0 * t * t * g);
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for (amplitude, phase) in [(0.001_f64, 0.0_f64), (0.003, 37.0)] {
            // The local derivatives are nonzero, but distant tanh probes
            // saturate exactly and would give falsely "precise" zero curvature.
            let deck = Netlist::parse(&format!(
                "local saturation curvature\nV1 out 0 DC {bias} DISTOF1 {amplitude} {phase}\n\
                 B1 out 0 I={{{scale}*tanh({slope}*v(out))}}\n.options GMIN=0\n.end\n"
            ))
            .unwrap();
            let result = engine.run_distortion(&deck, &[1e3], None).unwrap();
            for (product, derivative, divisor, tolerance) in [
                (DistortionProduct::SecondHarmonic, second, 4.0, 2e-5),
                (DistortionProduct::ThirdHarmonic, third, 24.0, 2e-3),
            ] {
                let response = &result.points[0].product(product).unwrap().response;
                let branch = response
                    .branch_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case("V1"))
                    .unwrap();
                let order = product.order() as i32;
                let expected = -derivative * amplitude.powi(order) / divisor
                    * C::from_polar(1.0, f64::from(order) * phase.to_radians());
                let actual = response.currents[branch];
                assert!(
                    (actual - expected).norm() < tolerance * expected.norm(),
                    "{dialect:?} {product:?}: {actual:?}, expected {expected:?}"
                );
            }
        }
    }
}
