//! Compiler/device delivery of sub-ULP delay values on portable and native routes.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;

#[test]
fn device_absdelay_retains_tiny_values_and_cancellation() {
    let source = include_str!("fixtures/precision_delay.va");
    let compiler = VerilogACompiler::default();
    for (delay, anchor, middle, trial, expected) in [
        (2.0_f64.powi(-80), 1.0, Some(1.0), 0.0, 2.0_f64.powi(-79)),
        (f64::from_bits(1), 1.0, Some(1.0), 0.0, f64::from_bits(2)),
        (0.5_f64.next_down(), 0.0, Some(0.0), 1.0, 2.0_f64.powi(-53)),
        (1.0 / 3.0, 2.0, None, -1.0, -2.0_f64.powi(-54)),
    ] {
        let model = compiler.compile(source).unwrap();
        let artifact = compiler.compile_canonical_ir(source).unwrap();
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("DPRECISION", model, &artifact, &[1, 0, 2])
                .unwrap();
        #[cfg(feature = "native")]
        assert!(device.is_using_native());
        device.set_analysis_type(2);
        device.set_time(0.0);
        device.update_voltages(&[anchor, delay]);
        assert_eq!(device.try_evaluate().unwrap()[0], anchor);
        device.try_advance_state().unwrap();
        if let Some(value) = middle {
            device.set_time(0.5);
            device.update_voltages(&[value, delay]);
            device.try_evaluate().unwrap();
            device.try_advance_state().unwrap();
        }
        device.set_time(1.0);
        for value in [trial, trial + 1.0, trial] {
            device.update_voltages(&[value, delay]);
            let output = device.try_evaluate().unwrap()[0];
            if value == trial {
                assert_eq!(output.to_bits(), expected.to_bits(), "delay={delay}");
            }
        }
        device.try_advance_state().unwrap();
        let checkpoint = device.checkpoint_state().unwrap();
        assert_eq!(checkpoint.accepted.delay_buffers.len(), 1);
        assert_eq!(
            checkpoint.accepted.delay_buffers[0].samples.last(),
            Some(&(1.0, trial))
        );
    }
}
