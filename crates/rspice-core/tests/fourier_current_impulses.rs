//! Authored Fourier cards retain both sampled current and singular charge.
use rspice_core::analysis::transient::{TransientDeviceOpTrace, TransientResult};
use rspice_core::resource::ResourceLimits;
use rspice_core::{
    CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace, Netlist, NoAbort, Value,
};

fn fixture() -> TransientResult {
    let time = (0..=256).map(|n| n as Value / 128.0).collect::<Vec<_>>();
    TransientResult {
        step_sizes: vec![1.0 / 128.0; time.len()],
        voltages: vec![vec![3.0; time.len()]],
        branch_currents: vec![vec![0.0; time.len()]],
        device_op_traces: vec![TransientDeviceOpTrace {
            device_name: "X1.Q1".into(),
            parameter: "ic".into(),
            values: vec![0.0; time.len()],
        }],
        time,
        num_nodes: 1,
        node_names: vec!["out".into()],
        branch_names: vec!["X1.V1".into()],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        voltage_impulses: None,
        current_impulses: Some(vec![
            CurrentImpulseTrace {
                derivatives: Vec::new(),
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "X1.V1".into(),
                },
                complete: true,
                points: vec![CurrentImpulsePoint {
                    time: 1.25,
                    charge_coulombs: 0.002,
                }],
            },
            CurrentImpulseTrace {
                derivatives: Vec::new(),
                owner: CurrentImpulseOwner::DeviceLead {
                    device_name: "X1.Q1".into(),
                    parameter: "ic".into(),
                },
                complete: true,
                points: vec![CurrentImpulsePoint {
                    time: 1.25,
                    charge_coulombs: -0.002,
                }],
            },
        ]),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fourier_current_impulse_authored_cards_use_shared_physical_transform() {
    let result = fixture();
    let netlist = Netlist::parse("current Fourier\nV1 out 0 0\nR1 out 0 1k\n.tran 1m 2\n.four 1 4 I(X1.V1) {2*I(X1.V1)}\n.end\n").unwrap();
    let spectra = rspice_core::engine::evaluate_transient_fourier_results(
        &netlist,
        &result,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    assert_eq!(spectra.len(), 2);
    assert!((spectra[0].spectrum.dc_component - 0.002).abs() < 1e-14);
    assert!((spectra[1].spectrum.dc_component - 0.004).abs() < 1e-14);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn fourier_current_derivatives_keep_order_sign_and_exact_event_time() {
    use num_complex::Complex64;
    use std::f64::consts::TAU;
    for order in 1..=5 {
        for (coefficient, event_time) in [(-1e-6, 1.3), (1e-6, 1.3), (-1e-6, 2.0), (1e-6, 2.0)] {
            let mut result = fixture();
            let trace = &mut result.current_impulses.as_mut().unwrap()[0];
            trace.points.clear();
            trace.derivatives = vec![
                rspice_core::CurrentImpulseDerivative {
                    // The resumed/window start belongs to the preceding interval.
                    // Its huge coefficient must never enter this transform.
                    time: 1.0,
                    order,
                    coefficient: 1e300,
                },
                rspice_core::CurrentImpulseDerivative {
                    time: event_time,
                    order,
                    coefficient,
                },
            ];
            let netlist = Netlist::parse("Derivative Fourier\nV1 out 0 0\nR1 out 0 1k\n.tran 1m 2\n.four 1 4 {2*I(X1.V1)}\n.end\n").unwrap();
            let spectra = rspice_core::engine::evaluate_transient_fourier_results(
                &netlist,
                &result,
                ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap();
            let spectrum = &spectra[0].spectrum;
            assert_eq!(spectrum.dc_component, 0.0);
            for harmonic in &spectrum.harmonics[1..] {
                let omega = TAU * harmonic.frequency;
                let expected = 4.0
                    * coefficient
                    * Complex64::new(0.0, omega).powu(order)
                    * Complex64::from_polar(1.0, -omega * (event_time - 1.0));
                let actual = Complex64::from_polar(harmonic.magnitude, harmonic.phase.to_radians());
                assert!(
                    (actual - expected).norm() < 2e-13 * expected.norm(),
                    "order {order}: {actual} vs {expected}"
                );
            }
        }
    }
}
