//! Analytic higher input derivatives of the accepted/trial delay interpolant.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::vm::VerilogAEvaluationMode;

#[test]
fn absdelay_higher_input_derivatives_preserve_primal_history() {
    for bounded in [false, true] {
        let source = include_str!("fixtures/higher_delay.va");
        let source = if bounded {
            source.replace(", td);", ", td, 1.0);")
        } else {
            source.to_owned()
        };
        let report = VerilogACompiler::default()
            .compile_runtime(&source, None)
            .unwrap();
        for delay in [0.125, 0.5] {
            let mut device = VerilogADevice::try_new_with_canonical_ir(
                "D3",
                report.model.clone(),
                &report.canonical_ir,
                &[1, 0],
            )
            .unwrap();
            #[cfg(feature = "native")]
            assert!(device.is_using_native());
            assert!(device.set_parameter("td", delay));
            device.set_analysis_type(2);
            device.set_timestep(1.0);
            for (time, coefficient) in [(0.0, 1.0), (1.0, 1.0 - delay), (1.1, 0.0)] {
                device.set_time(time);
                for voltage in [-0.2_f64, 0.3, -0.2] {
                    device.update_voltages(&[voltage]);
                    // Query the tangent first: it must not install derivative
                    // samples in the shared primal history.
                    let jacobian = device.try_compute_jacobian().unwrap();
                    let expected = coefficient * 8.0 * (2.0 * voltage).exp();
                    assert!(!jacobian.is_empty());
                    for entry in jacobian {
                        assert!(
                            (entry.value.abs() - 2.0 * expected).abs() < 1e-11,
                            "t={time}, td={delay}, bounded={bounded}: {entry:?}"
                        );
                    }
                    let actual = device.try_evaluate().unwrap()[0];
                    assert!(
                        (actual - expected).abs() < 1e-11,
                        "t={time}, td={delay}, bounded={bounded}: {actual} != {expected}"
                    );
                    assert_eq!(
                        device
                            .try_evaluate_with_mode(VerilogAEvaluationMode::StaticDaeProbe)
                            .unwrap()[0],
                        0.0
                    );
                }
                device.try_advance_state().unwrap();
                let checkpoint = device.checkpoint_state().unwrap();
                assert_eq!(checkpoint.accepted.delay_buffers.len(), 1);
                assert_eq!(
                    checkpoint.accepted.delay_buffers[0].samples.last(),
                    Some(&(time, (-0.4_f64).exp()))
                );
                assert_eq!(
                    device
                        .try_evaluate_with_mode(VerilogAEvaluationMode::StaticDaeProbe)
                        .unwrap()[0],
                    0.0
                );
                assert_eq!(device.checkpoint_state().unwrap(), checkpoint);
            }
        }
    }
}
