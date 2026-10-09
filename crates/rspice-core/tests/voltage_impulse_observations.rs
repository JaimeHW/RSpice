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
        let histories = result.voltage_impulses.as_ref().unwrap();
        let weighted = FourierAnalysis::new(FourierConfig::new(1.0).with_harmonics(4))
            .analyze_impulses_with_abort(
                &result.time,
                &result.voltages[0],
                &[
                    (rspice_core::ImpulseTraceRef::Voltage(&histories[0]), 1.0),
                    (rspice_core::ImpulseTraceRef::Voltage(&histories[1]), -1.0),
                ],
                &NoAbort,
            )
            .unwrap();
        close(weighted.dc_component, 0.003);
        for (actual, expected) in weighted
            .harmonics
            .iter()
            .zip(&spectra[0].spectrum.harmonics)
        {
            close(actual.magnitude, expected.magnitude);
        }
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_capacitive_event_keeps_voltage_actions_out_of_finite_samples() {
    let deck=Netlist::parse("CCVS differentiator event\nV1 in 0 PWL(0 0 1n 0 1n 1 2n 1)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\nR2 out 0 10\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save v(out) i(v1) i(h1) i(c1) i(c2) i(r2)\n.end\n").unwrap();
    let mut config = rspice_core::SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config).run_tran(&deck, 2e-9, 5e-12).unwrap();
    for (&time, &voltage) in result
        .time
        .iter()
        .zip(result.try_voltage_waveform_named("out").unwrap())
    {
        assert!(
            voltage.abs() < 1e-10,
            "finite V(out) at {time:e}: {voltage:e}; the voltage action is separate"
        );
    }
    let trace = result
        .voltage_impulses
        .as_ref()
        .expect("voltage action coverage")
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(trace.complete);
    let point = trace
        .points
        .iter()
        .find(|point| point.time == 1e-9)
        .unwrap();
    // I(V1)=-Cin*delta, V(out)=Rm*I(V1). The output capacitor
    // contributes Cout*dV(out)/dt, so I(H1) also has delta-prime.
    assert!((point.volt_seconds + 6e-12).abs() < 1e-24);
    let current=result.current_impulses.as_ref().unwrap().iter().find(|trace| matches!(&trace.owner,rspice_core::CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case("h1"))).unwrap();
    assert!(current.complete);
    let point = current
        .points
        .iter()
        .find(|point| point.time == 1e-9)
        .unwrap();
    assert!((point.charge_coulombs - 6e-13).abs() < 1e-25);
    let derivative = current
        .derivatives
        .iter()
        .find(|point| point.time == 1e-9 && point.order == 1)
        .unwrap();
    assert!((derivative.coefficient - 3e-23).abs() < 1e-35);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_voltage_action_changes_inductor_flux_at_the_exact_event() {
    let deck=Netlist::parse("CCVS winding event\nV1 in 0 PWL(0 0 1n 0 1n 1 3n 1)\nC1 in 0 2p\nH1 out 0 V1 3\nR1 out winding 1\nL1 winding 0 5n\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save v(out) v(winding) i(v1) i(h1) i(l1)\n.end\n").unwrap();
    let mut config = rspice_core::SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config).run_tran(&deck, 3e-9, 5e-12).unwrap();
    for (&time, &current) in result
        .time
        .iter()
        .zip(result.try_branch_current_waveform_named("l1").unwrap())
    {
        let expected = if time < 1e-9 {
            0.0
        } else {
            -1.2e-3 * (-(time - 1e-9) / 5e-9).exp()
        };
        assert!(
            (current - expected).abs() < 1e-9,
            "I(L1) at {time:e}: {current:e} != {expected:e}"
        );
    }
    for node in ["out", "winding"] {
        let trace = result
            .voltage_impulses
            .as_ref()
            .expect("voltage action coverage")
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
            .unwrap();
        assert!(trace.complete);
        let point = trace
            .points
            .iter()
            .find(|point| point.time == 1e-9)
            .unwrap();
        assert!((point.volt_seconds + 6e-12).abs() < 1e-24);
    }
}

fn ccvs_engine() -> Engine {
    let mut config = rspice_core::SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    Engine::new(config)
}

fn action_current<'a>(
    result: &'a TransientResult,
    name: &str,
) -> &'a rspice_core::CurrentImpulseTrace {
    result.current_impulses.as_ref().unwrap().iter().find(|trace| matches!(&trace.owner,
        rspice_core::CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case(name))).unwrap()
}

fn action_voltage<'a>(result: &'a TransientResult, name: &str) -> &'a VoltageImpulseTrace {
    result
        .voltage_impulses
        .as_ref()
        .unwrap()
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
        .unwrap()
}

fn action_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 2e-12 * expected.abs(),
        "{actual:e} != {expected:e}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_cascade_preserves_second_derivatives_and_controlled_observations() {
    let deck=Netlist::parse("Cascaded CCVS actions\nV1 in 0 PWL(0 0 1n 0 1n 1 2n 1)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\nR2 out 0 10\nH2 next 0 H1 7\nC3 next 0 11p\nR3 next 0 100\nG1 g 0 out 0 2\nRG g 0 1\nF1 f 0 H1 -2\nRF f 0 1\nE1 e 0 out 0 -2\nRE e 0 1\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    let result = ccvs_engine().run_tran(&deck, 2e-9, 5e-12).unwrap();
    for (name, delta, derivatives) in [
        ("v1", -2e-12, vec![]),
        ("c1", 2e-12, vec![]),
        ("h1", 6e-13, vec![3e-23]),
        ("c2", 0.0, vec![-3e-23]),
        ("r2", -6e-13, vec![]),
        ("h2", -4.2e-14, vec![-4.83e-23, -2.31e-33]),
        ("c3", 0.0, vec![4.62e-23, 2.31e-33]),
        ("g1", -1.2e-11, vec![]),
        ("f1", -1.2e-12, vec![-6e-23]),
        ("e1", -12e-12, vec![]),
    ] {
        let trace = action_current(&result, name);
        assert!(trace.complete, "{name}");
        let points: Vec<_> = trace.points.iter().filter(|p| p.time == 1e-9).collect();
        if delta == 0.0 {
            assert!(points.is_empty(), "{name}");
        } else {
            assert_eq!(points.len(), 1, "{name}");
            action_close(points[0].charge_coulombs, delta);
        }
        let actual: Vec<_> = trace
            .derivatives
            .iter()
            .filter(|p| p.time == 1e-9)
            .collect();
        assert_eq!(actual.len(), derivatives.len(), "{name}");
        for (order, (point, expected)) in actual.iter().zip(derivatives).enumerate() {
            assert_eq!(point.order, order as u32 + 1);
            action_close(point.coefficient, expected);
        }
    }
    let voltage = action_voltage(&result, "next");
    action_close(
        voltage
            .points
            .iter()
            .find(|p| p.time == 1e-9)
            .unwrap()
            .volt_seconds,
        4.2e-12,
    );
    action_close(
        voltage
            .derivatives
            .iter()
            .find(|p| p.time == 1e-9 && p.order == 1)
            .unwrap()
            .coefficient,
        2.1e-22,
    );
    for node in ["out", "next", "g", "f", "e"] {
        assert!(
            result
                .try_voltage_waveform_named(node)
                .unwrap()
                .iter()
                .all(|v| v.abs() < 1e-10),
            "{node}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_source_slope_changes_have_finite_voltage_and_capacitor_charge() {
    let deck=Netlist::parse("CCVS source jets\nV1 in 0 PWL(0 0 1n 1 2n 1 3n 0)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\nR2 out 0 10\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    let result = ccvs_engine().run_tran(&deck, 4e-9, 5e-12).unwrap();
    let voltage = result.try_voltage_waveform_named("out").unwrap();
    for (time, expected) in [(0.0, -0.006), (1e-9, 0.0), (2e-9, 0.006), (3.0 * 1e-9, 0.0)] {
        let i = result.time.iter().position(|&t| t == time).unwrap();
        assert!(
            (voltage[i] - expected).abs() < 1e-12,
            "V(out) at {time:e}: {}",
            voltage[i]
        );
    }
    for (time, expected) in [
        (0.0, 3e-14),
        (1e-9, -3e-14),
        (2e-9, -3e-14),
        (3.0 * 1e-9, 3e-14),
    ] {
        action_close(
            action_current(&result, "h1")
                .points
                .iter()
                .find(|p| p.time == time)
                .unwrap()
                .charge_coulombs,
            expected,
        );
    }
    assert!(
        action_voltage(&result, "out")
            .points
            .iter()
            .all(|p| p.volt_seconds.abs() < 1e-24)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_actions_survive_live_delivery_compression_and_packed_restart() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode, SpiceDialect,
        CompressionConfig,
    };
    use rspice_core::numerics::integration::IntegrationMethod;
    #[derive(Default)]
    struct Live(
        std::sync::Mutex<(
            Vec<VoltageImpulseTrace>,
            Vec<rspice_core::CurrentImpulseTrace>,
        )>,
    );
    impl rspice_core::AbortSignal for Live {
        fn is_aborted(&self) -> bool {
            false
        }
        fn observe_transient_sample(&self, sample: rspice_core::abort_signal::TransientSample<'_>) {
            if let (Some(voltage), Some(current)) =
                (sample.voltage_impulses, sample.current_impulses)
            {
                let mut saved = self.0.lock().unwrap();
                *saved = (voltage.to_vec(), current.to_vec());
            }
        }
    }
    let deck=Netlist::parse("CCVS retained actions\nV1 in 0 PWL(0 0 1n 0 1n 1 2n 1 2n 0 4n 0)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\nR2 out 0 10\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for method in [
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            let mut config = rspice_core::SimulationConfig {
                spice_dialect: dialect,
                integration_method: method,
                ..Default::default()
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let live = Live::default();
            let (full, schedule) = engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    &deck,
                    4e-9,
                    5e-12,
                    TransientStartupMode::OperatingPoint,
                    &[1e-9, 1.5e-9],
                    &live,
                )
                .unwrap();
            let observed = live.0.lock().unwrap();
            assert_eq!(Some(&observed.0), full.voltage_impulses.as_ref());
            assert_eq!(Some(&observed.1), full.current_impulses.as_ref());
            drop(observed);
            let compressed = engine
                .compress_transient_result_with_abort(
                    &deck,
                    &full,
                    &CompressionConfig::default(),
                    &NoAbort,
                )
                .unwrap();
            compressed.validate().unwrap();
            assert_eq!(compressed.voltage_impulses, full.voltage_impulses);
            assert_eq!(compressed.current_impulses, full.current_impulses);
            for saved in schedule {
                let checkpoint = TransientCheckpoint::from_bytes(
                    &saved
                        .checkpoint
                        .to_bytes(TransientCheckpointEncoding::Packed)
                        .unwrap(),
                )
                .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&deck, &checkpoint, 4e-9, 5e-12)
                    .unwrap();
                let offset = full
                    .time
                    .iter()
                    .position(|&t| t == checkpoint.time)
                    .unwrap();
                assert_eq!(resumed.time, full.time[offset..], "{dialect:?}/{method:?}");
                for (a, b) in resumed
                    .voltages
                    .iter()
                    .zip(&full.voltages)
                    .chain(resumed.branch_currents.iter().zip(&full.branch_currents))
                {
                    assert_eq!(a, &b[offset..], "{dialect:?}/{method:?}");
                }
                let mut voltages = full.voltage_impulses.clone().unwrap();
                for trace in &mut voltages {
                    trace.points.retain(|p| p.time > checkpoint.time);
                    trace.derivatives.retain(|p| p.time > checkpoint.time);
                }
                assert_eq!(resumed.voltage_impulses, Some(voltages));
                let mut currents = full.current_impulses.clone().unwrap();
                for trace in &mut currents {
                    trace.points.retain(|p| p.time > checkpoint.time);
                    trace.derivatives.retain(|p| p.time > checkpoint.time);
                }
                assert_eq!(resumed.current_impulses, Some(currents));
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_uic_preserves_authored_capacitor_charge_and_winding_flux() {
    use rspice_core::engine::TransientStartupMode;
    let deck=Netlist::parse("CCVS IC transition\nV1 in 0 1\nC1 in 0 2p IC=.25\nH1 out 0 V1 3\nL1 out 0 5n IC=.002\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    let result = ccvs_engine()
        .run_tran_with_startup_mode(&deck, 1e-9, 5e-12, TransientStartupMode::Uic)
        .unwrap();
    // Cin*(1-.25) transfers -4.5 pV*s, changing the winding current
    // by -0.9 mA from the authored 2 mA, independently of the first dt.
    action_close(
        action_voltage(&result, "out").points[0].volt_seconds,
        -4.5e-12,
    );
    for &current in result.try_branch_current_waveform_named("l1").unwrap() {
        action_close(current, 0.0011);
    }
    action_close(
        action_current(&result, "v1").points[0].charge_coulombs,
        -1.5e-12,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_current_forcing_and_mutual_flux_keep_original_polarities() {
    let deck=Netlist::parse("CCVS coupled flux\nV1 in 0 PWL(0 0 1n 0 1n 1 4n 1)\nI1 0 in PWL(0 0 1n 0 1n .0001 4n .0001)\nC1 in 0 2p\nH1 out 0 V1 3\nL1 out 0 5n\nL2 secondary 0 20n\nK1 L1 L2 .2\nR2 secondary 0 10\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    let result = ccvs_engine().run_tran(&deck, 4e-9, 5e-12).unwrap();
    let event = result.time.iter().position(|&time| time == 1e-9).unwrap();
    // [L1 M; M L2] * delta_i = [-6p,0], with M=2n.
    action_close(
        result.try_branch_current_waveform_named("l1").unwrap()[event],
        -0.00125,
    );
    action_close(
        result.try_branch_current_waveform_named("l2").unwrap()[event],
        0.000125,
    );
    action_close(
        result.try_voltage_waveform_named("out").unwrap()[event],
        0.0003,
    );
    action_close(
        result.try_voltage_waveform_named("secondary").unwrap()[event],
        -0.00125,
    );
    action_close(
        action_voltage(&result, "out")
            .points
            .iter()
            .find(|p| p.time == 1e-9)
            .unwrap()
            .volt_seconds,
        -6e-12,
    );
    assert!(action_voltage(&result, "secondary").points.is_empty());
    assert!(action_current(&result, "i1").points.is_empty());
    assert!(action_current(&result, "i1").derivatives.is_empty());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ccvs_nonbinary_startup_preserves_exact_stored_charge() {
    let deck = Netlist::parse("Unchanged stored charge\nV1 in 0 DC .4 PWL(0 .4 1n .4 2n .4)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\nR2 out 0 10\n.options GMIN=0\n.save all\n.end\n").unwrap();
    let result = ccvs_engine().run_tran(&deck, 0.5e-9, 5e-12).unwrap();
    for trace in result.voltage_impulses.as_ref().unwrap() {
        assert!(
            trace.complete && trace.points.is_empty() && trace.derivatives.is_empty(),
            "{trace:?}"
        );
    }
    for trace in result.current_impulses.as_ref().unwrap() {
        assert!(
            trace.complete && trace.points.is_empty() && trace.derivatives.is_empty(),
            "{trace:?}"
        );
    }
}
