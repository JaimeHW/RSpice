//! Public FFT post-processing of distributional currents.
use num_complex::Complex64;
use rspice_core::abort_signal::CountingAbort;
use rspice_core::analysis::transient::TransientResult;
use rspice_core::{
    CurrentImpulseDerivative, CurrentImpulseOwner, CurrentImpulseTrace, Engine, Netlist, NoAbort,
    ResourceKind, ResourceLimits, SimulationConfig, SimulationError,
};
use serde::Deserialize;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_window_derivatives_combine_with_samples_charge_and_affine_weights() {
    let engine = engine(ResourceLimits::default());
    let mut result = fixture(0.0, 1.0, 0.3125, 1, -1e-6);
    result.branch_currents[0].fill(3.0);
    result.voltages[0].fill(7.0);
    result.current_impulses.as_mut().unwrap()[0]
        .points
        .push(rspice_core::CurrentImpulsePoint {
            time: 0.3125,
            charge_coulombs: 1e-3,
        });
    let x = 0.3125;
    let tau = std::f64::consts::TAU;
    let taper = 0.5 - 0.5 * (tau * x).cos();
    let slope = std::f64::consts::PI * (tau * x).sin();
    let expected: Vec<_> = (0..=4)
        .map(|bin| {
            let one_sided = if bin == 0 || bin == 4 { 1.0 } else { 2.0 };
            let phase = Complex64::from_polar(1.0, -tau * bin as f64 * x);
            let singular = 2.0 * one_sided / 0.5
                * phase
                * (Complex64::new(1e-3 * taper, 0.0)
                    - 1e-6 * Complex64::new(-slope, tau * bin as f64 * taper));
            singular
                + match bin {
                    0 => 13.0,
                    1 => -13.0,
                    _ => 0.0,
                }
        })
        .collect();
    let largest = expected
        .iter()
        .map(|value| value.norm())
        .fold(0.0, f64::max);
    for format in ["unorm", "norm"] {
        let request = Netlist::parse(&format!("Combined currents\nV1 out 0 0\nR1 out 0 1k\n.options fft fft_mode=1 fftout=1\n.fft {{2*I(V1)+V(out)}} np=8 window=hann format={format}\n.end\n")).unwrap();
        let spectra = engine
            .evaluate_transient_fft_results(&request, &result, &NoAbort)
            .unwrap();
        assert!(spectra[0].metrics.is_some());
        for (bin, expected) in spectra[0].bins.iter().zip(&expected) {
            let expected = expected / if format == "norm" { largest } else { 1.0 };
            let actual = Complex64::new(bin.real, bin.imaginary);
            assert!((actual - expected).norm() < 2e-13 * expected.norm().max(1.0));
        }
    }
}

fn fixture(
    start: f64,
    duration: f64,
    fraction: f64,
    order: u32,
    coefficient: f64,
) -> TransientResult {
    TransientResult {
        time: (0..=8).map(|j| start + duration * j as f64 / 8.0).collect(),
        step_sizes: vec![duration / 8.0; 9],
        voltages: vec![vec![0.0; 9]],
        branch_currents: vec![vec![0.0; 9]],
        num_nodes: 1,
        node_names: vec!["out".into()],
        branch_names: vec!["V1".into()],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        device_op_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        voltage_impulses: None,
        current_impulses: Some(vec![CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: vec![],
            derivatives: vec![CurrentImpulseDerivative {
                time: start + duration * fraction,
                order,
                coefficient,
            }],
        }]),
    }
}
fn deck(window: &str, alpha: u32, mode: u32, start: f64, stop: f64) -> Netlist {
    Netlist::parse(&format!("Windowed derivatives\nV1 out 0 0\nR1 out 0 1k\n.options fft fft_mode={mode}\n.fft I(V1) np=8 format=unorm window={window} alfa={alpha} start={start:e} stop={stop:e}\n.end\n")).unwrap()
}
fn engine(limits: ResourceLimits) -> Engine {
    Engine::new(SimulationConfig {
        resource_limits: limits,
        ..Default::default()
    })
}
#[derive(Deserialize)]
struct Oracle {
    start: f64,
    duration: f64,
    coefficient: f64,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
struct Case {
    window: String,
    alpha: u32,
    mode: u32,
    order: u32,
    fraction: f64,
    gain: f64,
    bins: Vec<[f64; 2]>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_window_derivatives_match_independent_high_precision_coefficients() {
    let data: Oracle = serde_json::from_str(include_str!(
        "testdata/qualification/fft-window-derivative-oracle.json"
    ))
    .unwrap();
    assert_eq!(data.cases.len(), 400);
    let engine = engine(ResourceLimits::default());
    let mut worst_relative = 0.0_f64;
    for case in data.cases {
        let result = fixture(
            data.start,
            data.duration,
            case.fraction,
            case.order,
            data.coefficient,
        );
        let request = deck(
            &case.window,
            case.alpha,
            case.mode,
            data.start,
            data.start + data.duration,
        );
        let spectrum = engine
            .evaluate_transient_fft_results(&request, &result, &NoAbort)
            .unwrap_or_else(|e| {
                panic!(
                    "{} mode {} order {}: {e}",
                    case.window, case.mode, case.order
                )
            });
        assert!((spectrum[0].coherent_gain / case.gain - 1.0).abs() < 2e-14);
        for (bin, expected) in spectrum[0].bins.iter().zip(&case.bins) {
            let actual = Complex64::new(bin.real, bin.imaginary);
            let expected = Complex64::new(expected[0], expected[1]);
            let relative = (actual - expected).norm() / expected.norm().max(1e-30);
            worst_relative = worst_relative.max(relative);
            assert!(
                relative < 2e-12,
                "{} alpha {} mode {} order {} fraction {} bin {}: {actual} vs {expected}, relative={relative:e}",
                case.window,
                case.alpha,
                case.mode,
                case.order,
                case.fraction,
                bin.index
            );
        }
    }
    println!("maximum relative complex-coefficient error: {worst_relative:e}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_derivative_support_boundaries_and_corners_follow_window_smoothness() {
    let engine = engine(ResourceLimits::default());
    for (window, zero_order) in [
        ("bartlett", 1),
        ("bartletthann", 1),
        ("hamming", 0),
        ("hann", 2),
        ("black", 0),
        ("blackman", 2),
        ("harris", 0),
        ("nuttall", 0),
        ("halfcyclesine", 1),
        ("halfcyclesine3", 3),
        ("halfcyclesine6", 6),
        ("cosine2", 2),
        ("cosine4", 4),
        ("gaussian", 0),
        ("kaiser", 0),
    ] {
        let request = deck(window, 3, 0, 0.0, 1.0);
        for order in 1..=8 {
            let result = fixture(0.0, 1.0, 0.875, order, -1e-6);
            let output = engine.evaluate_transient_fft_results(&request, &result, &NoAbort);
            if order < zero_order {
                assert!(
                    output.unwrap()[0]
                        .bins
                        .iter()
                        .all(|bin| bin.magnitude == 0.0)
                );
            } else {
                assert!(
                    output.unwrap_err().to_string().contains("support boundary"),
                    "{window} order {order}"
                );
            }
        }
        let outside = fixture(0.0, 1.0, 1.0, u32::MAX, -1e-6);
        assert!(
            engine
                .evaluate_transient_fft_results(&request, &outside, &NoAbort)
                .unwrap()[0]
                .bins
                .iter()
                .all(|bin| bin.magnitude == 0.0)
        );
    }
    for mode in [0, 1] {
        for window in ["bartlett", "bartletthann"] {
            let middle = if mode == 0 { 0.4375 } else { 0.5 };
            let request = deck(window, 3, mode, 0.0, 1.0);
            let result = fixture(0.0, 1.0, middle, 1, 1e-6);
            assert!(
                engine
                    .evaluate_transient_fft_results(&request, &result, &NoAbort)
                    .unwrap_err()
                    .to_string()
                    .contains("triangular corner")
            );
        }
    }
    for window in [
        "bartlett",
        "bartletthann",
        "gaussian",
        "kaiser",
        "halfcyclesine",
    ] {
        let request = deck(window, 3, 1, 0.0, 1.0);
        let result = fixture(0.0, 1.0, 1.0, 1, 1e-6);
        assert!(
            engine
                .evaluate_transient_fft_results(&request, &result, &NoAbort)
                .unwrap_err()
                .to_string()
                .contains("support boundary")
        );
    }
    for (window, order) in [
        ("hann", 1),
        ("cosine4", 3),
        ("halfcyclesine3", 2),
        ("halfcyclesine6", 5),
    ] {
        let request = deck(window, 3, 1, 0.0, 1.0);
        let result = fixture(0.0, 1.0, 1.0, order, 1e-6);
        assert!(
            engine
                .evaluate_transient_fft_results(&request, &result, &NoAbort)
                .unwrap()[0]
                .bins
                .iter()
                .all(|bin| bin.magnitude == 0.0)
        );
    }
    let request = deck("hann", 3, 1, 0.0, 1.0);
    let mut result = fixture(0.0, 1.0, 1.0, 2, 1e-6);
    result.current_impulses.as_mut().unwrap()[0]
        .derivatives
        .insert(
            0,
            CurrentImpulseDerivative {
                time: 0.0,
                order: u32::MAX,
                coefficient: 1e300,
            },
        );
    let fft = engine
        .evaluate_transient_fft_results(&request, &result, &NoAbort)
        .unwrap();
    let expected = 4.0 * std::f64::consts::PI.powi(2) * 1e-6;
    assert!((fft[0].bins[0].real / expected - 1.0).abs() < 1e-14);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_window_derivatives_preserve_extreme_physical_scaling() {
    let engine = engine(ResourceLimits::default());
    for window in ["hann", "blackman", "halfcyclesine3", "gaussian", "kaiser"] {
        let result = fixture(0.0, 1.0, 0.3125, 2, 1.0);
        let reference = engine
            .evaluate_transient_fft_results(&deck(window, 3, 1, 0.0, 1.0), &result, &NoAbort)
            .unwrap();
        for (duration, coefficient, scale) in [(1e-150, 1e-300, 1e150), (1e150, 1e300, 1e-150)] {
            let result = fixture(0.0, duration, 0.3125, 2, coefficient);
            let actual = engine
                .evaluate_transient_fft_results(
                    &deck(window, 3, 1, 0.0, duration),
                    &result,
                    &NoAbort,
                )
                .unwrap();
            for (a, b) in actual[0].bins.iter().zip(&reference[0].bins) {
                let actual = Complex64::new(a.real / scale, a.imaginary / scale);
                let expected = Complex64::new(b.real, b.imaginary);
                assert!(
                    (actual - expected).norm() < 2e-12 * expected.norm().max(1e-30),
                    "{window}: {actual} vs {expected}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_window_derivatives_are_bounded_cancellable_and_retryable() {
    let request = deck("kaiser", 20, 1, 0.0, 1.0);
    let result = fixture(0.0, 1.0, 0.3125, u32::MAX, 1.0);
    assert!(
        matches!(engine(ResourceLimits::default()).evaluate_transient_fft_results(&request,&result,&NoAbort),
        Err(SimulationError::ResourceLimit(error)) if error.resource==ResourceKind::ResultValues)
    );
    let result = fixture(0.0, 1.0, 0.3125, 2, 1.0);
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 128;
    assert!(
        matches!(engine(limits).evaluate_transient_fft_results(&request,&result,&NoAbort),
        Err(SimulationError::ResourceLimit(error)) if error.resource==ResourceKind::ResultValues)
    );
    let result = fixture(0.0, 1.0, 0.3125, 128, 1e-100);
    assert!(matches!(
        engine(ResourceLimits::default()).evaluate_transient_fft_results(
            &request,
            &result,
            &CountingAbort::new(128)
        ),
        Err(SimulationError::Aborted)
    ));
    let result = fixture(0.0, 1.0, 0.3125, 2, 1.0);
    assert!(
        engine(ResourceLimits::default())
            .evaluate_transient_fft_results(&request, &result, &NoAbort)
            .is_ok()
    );
}
