//! Analytic higher input derivatives of the accepted/trial delay interpolant.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::vm::VerilogAEvaluationMode;

#[test]
fn absdelay_moving_delay_independent_axes_commute() {
    use rspice_veriloga::codegen::StampIndex;
    for bounded in [false, true] {
        for axes in [["p", "c", "c"], ["c", "c", "p"]] {
            let maximum = if bounded { ",1.0" } else { "" };
            let mut expression =
                format!("absdelay(exp(2*V(p)),0.25+0.1*V(c)+0.05*V(c)*V(c){maximum})");
            for axis in axes {
                expression = format!("ddx({expression},V({axis}))");
            }
            let source = format!(
                "module mixed_delay(p,c); inout p,c; electrical p,c; analog I(p)<+{expression}; endmodule"
            );
            let report = VerilogACompiler::default()
                .compile_runtime(&source, None)
                .unwrap();
            let mut device = VerilogADevice::try_new_with_canonical_ir(
                "Dpc",
                report.model,
                &report.canonical_ir,
                &[1, 2],
            )
            .unwrap();
            #[cfg(feature = "native")]
            assert!(device.is_using_native());
            device.set_analysis_type(2);
            device.set_timestep(1.0);
            for time in [0.0, 1.0, 1.1] {
                device.set_time(time);
                for (signal, control) in [(-0.2_f64, 0.3_f64), (0.3, -0.2), (-0.2, 0.3)] {
                    device.update_voltages(&[signal, control]);
                    // Only the current segment has F_xd=-1/h; d_cc=0.1.
                    // Omitting maxdelay freezes td even when its expression varies.
                    let expected = if time == 1.0 && bounded {
                        -0.2 * (2.0 * signal).exp()
                    } else {
                        0.0
                    };
                    let jacobian = device.try_compute_jacobian().unwrap();
                    let signal_tangent = jacobian
                        .iter()
                        .filter(|e| {
                            matches!(e.row, StampIndex::Terminal(0))
                                && matches!(e.col, StampIndex::Terminal(0))
                        })
                        .map(|e| e.value)
                        .sum::<f64>();
                    let control_tangent = jacobian
                        .iter()
                        .filter(|e| {
                            matches!(e.row, StampIndex::Terminal(0))
                                && matches!(e.col, StampIndex::Terminal(1))
                        })
                        .map(|e| e.value)
                        .sum::<f64>();
                    assert!(
                        (signal_tangent - 2.0 * expected).abs() < 1e-11,
                        "axes={axes:?}, bounded={bounded}, t={time}: {signal_tangent} != {}",
                        2.0 * expected
                    );
                    assert!(control_tangent.abs() < 1e-11, "{control_tangent}");
                    let actual = device.try_evaluate().unwrap()[0];
                    assert!(
                        (actual - expected).abs() < 1e-11,
                        "axes={axes:?}, bounded={bounded}, t={time}: {actual} != {expected}"
                    );
                }
                device.try_advance_state().unwrap();
                let checkpoint = device.checkpoint_state().unwrap();
                assert_eq!(checkpoint.accepted.delay_buffers.len(), 1);
                assert_eq!(
                    checkpoint.accepted.delay_buffers[0].samples.last(),
                    Some(&(time, (-0.4_f64).exp()))
                );
            }
        }
    }
}

#[test]
fn absdelay_higher_moving_delay_derivatives_preserve_primal_history() {
    let report = VerilogACompiler::default()
        .compile_runtime(include_str!("fixtures/higher_moving_delay.va"), None)
        .unwrap();
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "D3",
        report.model,
        &report.canonical_ir,
        &[1, 0],
    )
    .unwrap();
    #[cfg(feature = "native")]
    assert!(device.is_using_native());
    device.set_analysis_type(2);
    device.set_timestep(1.0);
    for time in [0.0, 1.0, 1.1] {
        device.set_time(time);
        for voltage in [-0.2_f64, 0.3, -0.2] {
            device.update_voltages(&[voltage]);
            // Differentiate the interpolation polynomial independently:
            // F=(1-d)*exp(2*v)+d*accepted, h=1, d=.25+.1*v+.05*v^2.
            let d = 0.25 + 0.1 * voltage + 0.05 * voltage * voltage;
            let dp = 0.1 + 0.1 * voltage;
            let (third, fourth) = if time == 0.0 {
                (8.0, 16.0)
            } else if time == 1.0 {
                (
                    8.0 * (1.0 - d) - 12.0 * dp - 0.6,
                    16.0 * (1.0 - d) - 32.0 * dp - 2.4,
                )
            } else {
                (0.0, 0.0)
            };
            let expected = third * (2.0 * voltage).exp();
            let tangent = fourth * (2.0 * voltage).exp();
            let jacobian = device.try_compute_jacobian().unwrap();
            assert!(!jacobian.is_empty());
            for entry in jacobian {
                assert!(
                    (entry.value.abs() - tangent.abs()).abs() < 1e-11,
                    "t={time}, v={voltage}: {entry:?}, expected magnitude {tangent}"
                );
            }
            let actual = device.try_evaluate().unwrap()[0];
            assert!(
                (actual - expected).abs() < 1e-11,
                "t={time}, v={voltage}: {actual} != {expected}"
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
