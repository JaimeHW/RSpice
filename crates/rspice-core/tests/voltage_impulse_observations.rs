//! Independent distributional oracles for voltage observations.
use num_complex::Complex64;
use rspice_core::analysis::measure_signals::evaluate_tran_measurements;
use rspice_core::analysis::transient::TransientResult;
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};
use rspice_core::{
    Engine, Netlist, NoAbort, ResourceLimits, VoltageImpulseDerivative, VoltageImpulsePoint,
    VoltageImpulseTrace,
};
use std::f64::consts::TAU;

fn fixture() -> TransientResult {
    let mut step_sizes = vec![1.0 / 128.0; 129];
    step_sizes[0] = 0.0;
    TransientResult {
        time: (0..=128).map(|i| i as f64 / 128.0).collect(),
        step_sizes,
        voltages: vec![vec![3.0; 129], vec![1.0; 129]],
        branch_currents: vec![],
        num_nodes: 2,
        node_names: vec!["out".into(), "ref".into()],
        branch_names: vec![],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        device_op_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        current_impulses: None,
        voltage_impulses: Some(vec![
            VoltageImpulseTrace {
                node_name: "out".into(),
                complete: true,
                points: vec![VoltageImpulsePoint {
                    time: 0.3125,
                    volt_seconds: 0.002,
                }],
                derivatives: vec![VoltageImpulseDerivative {
                    time: 0.3125,
                    order: 1,
                    coefficient: -1e-6,
                }],
            },
            VoltageImpulseTrace {
                node_name: "ref".into(),
                complete: true,
                points: vec![VoltageImpulsePoint {
                    time: 0.3125,
                    volt_seconds: -0.001,
                }],
                derivatives: vec![],
            },
        ]),
    }
}

fn deck(cards: &str) -> Netlist {
    Netlist::parse(&format!(
        "Voltage actions\nR1 out 0 1k\nR2 ref 0 1k\n{cards}\n.end\n"
    ))
    .unwrap()
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 3e-13 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fourier_voltage_actions_use_exact_time_sign_and_derivative_order() {
    let mut result = fixture();
    result.voltages[0].fill(0.0);
    result.voltages[1].fill(0.0);
    for order in 1..=5 {
        result.voltage_impulses.as_mut().unwrap()[0].derivatives[0].order = order;
        let request = deck(".four 1 4 V(out,ref) {2*V(out)-2*V(ref)}");
        let spectra = rspice_core::engine::evaluate_transient_fourier_results(
            &request,
            &result,
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        for (spectrum, weight) in spectra.iter().zip([1.0, 2.0]) {
            close(spectrum.spectrum.dc_component, weight * 0.003);
            for n in 1..=4 {
                let omega = TAU * n as f64;
                let expected = 2.0
                    * weight
                    * Complex64::from_polar(1.0, -omega * 0.3125)
                    * (0.003 - 1e-6 * Complex64::new(0.0, omega).powu(order));
                let h = &spectrum.spectrum.harmonics[n];
                let actual = Complex64::from_polar(h.magnitude, h.phase.to_radians());
                close((actual - expected).norm(), 0.0);
            }
        }
        let direct = FourierAnalysis::new(FourierConfig::new(1.0).with_harmonics(4))
            .analyze_voltage_with_abort(
                &result.time,
                &result.voltages[0],
                &result.voltage_impulses.as_ref().unwrap()[0],
                &NoAbort,
            )
            .unwrap();
        close(direct.dc_component, 0.002);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fft_voltage_actions_include_window_derivatives_and_finite_samples() {
    let result = fixture();
    let request =
        deck(".options fft fft_mode=1\n.fft {2*V(out,ref)} np=8 window=hann format=unorm");
    let spectra = Engine::default()
        .evaluate_transient_fft_results(&request, &result, &NoAbort)
        .unwrap();
    let x = 0.3125;
    let taper = 0.5 - 0.5 * (TAU * x).cos();
    let slope = std::f64::consts::PI * (TAU * x).sin();
    for (n, bin) in spectra[0].bins.iter().enumerate() {
        let one_sided = if n == 0 || n == 4 { 1.0 } else { 2.0 };
        let omega = TAU * n as f64;
        let expected = 2.0 * one_sided / 0.5
            * Complex64::from_polar(1.0, -omega * x)
            * (Complex64::new(0.003 * taper, 0.0) - 1e-6 * Complex64::new(-slope, omega * taper))
            + match n {
                0 => 4.0,
                1 => -4.0,
                _ => 0.0,
            };
        close(
            (Complex64::new(bin.real, bin.imaginary) - expected).norm(),
            0.0,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_integrals_include_actions_and_reject_singular_boundaries() {
    let result = fixture();
    let request = deck(
        ".meas tran area INTEG V(out,ref)\n.meas tran avg AVG V(out,ref)\n.meas tran middle INTEG V(out,ref) FROM=.25 TO=.5\n.meas tran boundary INTEG V(out) TO=.3125\n.meas tran picked FIND area AT=.3125\n.meas tran later FIND area AT=.5\n.meas tran final PARAM {area}\n.meas tran nonlinear RMS V(out)",
    );
    let measured = evaluate_tran_measurements(&request, &result);
    for (name, expected) in [
        ("area", 2.003),
        ("avg", 2.003),
        ("middle", 0.503),
        ("later", 1.003),
        ("final", 2.003),
    ] {
        let value = measured
            .iter()
            .find(|value| value.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(value.passed, "{value:?}");
        close(value.value.unwrap(), expected);
    }
    for name in ["boundary", "picked", "nonlinear"] {
        let value = measured
            .iter()
            .find(|value| value.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(!value.passed && value.value.is_none(), "{value:?}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_history_coverage_is_explicit_and_legacy_results_stay_readable() {
    let mut result = fixture();
    let request = deck(
        ".meas tran area INTEG V(out)\n.meas tran ground INTEG V(0)\n.meas tran zero INTEG V(out,out)",
    );
    for history in [
        Some(vec![]),
        Some(vec![VoltageImpulseTrace {
            complete: false,
            ..result.voltage_impulses.as_ref().unwrap()[0].clone()
        }]),
    ] {
        result.voltage_impulses = history;
        let values = evaluate_tran_measurements(&request, &result);
        assert!(!values[0].passed, "{:?}", values[0]);
        for value in &values[1..] {
            assert!(value.passed, "{value:?}");
            close(value.value.unwrap(), 0.0);
        }
    }
    result.voltage_impulses = None;
    let values = evaluate_tran_measurements(&request, &result);
    close(values[0].value.unwrap(), 3.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_actions_round_trip_typed_documents_without_changing_finite_samples() {
    let result = fixture();
    result.validate_impulses().unwrap();
    let document = AnalysisResultDocument::from_transient(
        AnalysisInstanceId::new(AnalysisKind::Tran, 0),
        &result,
        None,
        vec![],
    )
    .unwrap()
    .build()
    .unwrap();
    let json = document.to_json().unwrap();
    let restored = AnalysisResultDocument::from_json(&json).unwrap();
    assert_eq!(document, restored);
    let mut wire: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(
        wire["payload"]["voltageImpulses"][0]["points"][0]["voltSeconds"],
        0.002
    );
    wire["schemaVersion"] = 16.into();
    assert!(AnalysisResultDocument::from_json(&wire.to_string()).is_err());
    wire["payload"]
        .as_object_mut()
        .unwrap()
        .remove("voltageImpulses");
    AnalysisResultDocument::from_json(&wire.to_string()).unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_actions_validate_identity_order_and_finite_coefficients() {
    for case in 0..8 {
        let mut result = fixture();
        let traces = result.voltage_impulses.as_mut().unwrap();
        match case {
            0 => traces[0].node_name.clear(),
            1 => traces[0].node_name = "unknown".into(),
            2 => traces[1].node_name = "OUT".into(),
            3 => traces[0].points[0].volt_seconds = f64::NAN,
            4 => traces[0].points[0].volt_seconds = 0.0,
            5 => traces[0].points[0].time = 2.0,
            6 => traces[0].derivatives[0].order = 0,
            7 => {
                let point = traces[0].points[0];
                traces[0].points.push(point);
            }
            _ => unreachable!(),
        }
        assert!(result.validate_impulses().is_err(), "case {case}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_actions_follow_hierarchy_aliases_and_authoritative_numeric_nodes() {
    let mut result = fixture();
    result.node_names[0] = "7".into();
    result.voltage_impulses.as_mut().unwrap()[0].node_name = "7".into();
    let request = Netlist::parse("Alias voltage actions\nX1 7 0 CELL\n.SUBCKT CELL A B\nR1 A B 1\n.ENDS\n.meas tran direct INTEG V(7)\n.meas tran alias INTEG V(X1:A)\n.meas tran zero INTEG V(X1:B)\n.meas tran wrong INTEG V(1)\n.end\n").unwrap();
    let values = evaluate_tran_measurements(&request, &result);
    for value in &values[..2] {
        assert!(value.passed, "{value:?}");
        close(value.value.unwrap(), 3.002);
    }
    assert!(values[2].passed, "{:?}", values[2]);
    close(values[2].value.unwrap(), 0.0);
    assert!(!values[3].passed, "{:?}", values[3]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_actions_survive_compression_and_abort_is_retryable() {
    use rspice_core::abort_signal::CountingAbort;
    use rspice_core::engine::CompressionConfig;
    let result = fixture();
    let request = deck(".tran .01 1\n.meas tran area INTEG V(out)");
    let engine = Engine::default();
    let compressed = engine
        .compress_transient_result_with_abort(
            &request,
            &result,
            &CompressionConfig::default(),
            &NoAbort,
        )
        .unwrap();
    assert!(compressed.time.len() < result.time.len());
    assert_eq!(compressed.voltage_impulses, result.voltage_impulses);
    compressed.validate().unwrap();
    let expanded = TransientResult::try_from(compressed).unwrap();
    assert_eq!(expanded.voltage_impulses, result.voltage_impulses);
    let request = deck(".four 1 4 V(out,ref)");
    let mut completed = false;
    for polls in 0..256 {
        let abort = CountingAbort::new(polls);
        match rspice_core::engine::evaluate_transient_fourier_results(
            &request,
            &result,
            ResourceLimits::default(),
            &abort,
        ) {
            Err(rspice_core::SimulationError::Aborted) => {}
            Ok(_) => {
                completed = true;
                break;
            }
            Err(error) => panic!("{error}"),
        }
    }
    assert!(completed);
    rspice_core::engine::evaluate_transient_fourier_results(
        &request,
        &result,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_integral_seams_own_each_action_once() {
    let mut result = fixture();
    let trace = &mut result.voltage_impulses.as_mut().unwrap()[0];
    trace.derivatives.clear();
    trace.points = [0.0, 0.5, 1.0]
        .map(|time| VoltageImpulsePoint {
            time,
            volt_seconds: 0.002,
        })
        .to_vec();
    let values = evaluate_tran_measurements(
        &deck(
            ".meas tran all INTEG V(out)\n.meas tran first INTEG V(out) TO=.5\n.meas tran last INTEG V(out) FROM=.5",
        ),
        &result,
    );
    for (value, expected) in values.iter().zip([3.004, 1.502, 1.502]) {
        assert!(value.passed, "{value:?}");
        close(value.value.unwrap(), expected);
    }
}
