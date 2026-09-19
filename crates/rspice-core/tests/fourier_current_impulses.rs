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
        current_impulses: Some(vec![
            CurrentImpulseTrace {
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

#[test]
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
