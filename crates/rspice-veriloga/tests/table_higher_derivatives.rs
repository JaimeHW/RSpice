//! Derivatives of the existing piecewise-linear table interpolation contract.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;

#[test]
fn higher_table_derivatives_follow_the_active_segment() {
    check_higher_table_derivatives(include_str!("fixtures/higher_table.va"));
}

#[test]
fn higher_tiny_table_derivatives_follow_the_active_segment() {
    let source = include_str!("fixtures/higher_table.va")
        .replace("u=exp", "u=1e-40*exp")
        .replace("0.0,1.0,1.0,3.0,2.0,9.0", "0.0,1.0,1e-40,3.0,2e-40,9.0");
    check_higher_table_derivatives(&source);
}

#[test]
fn higher_subnormal_table_derivatives_follow_the_active_segment() {
    let source = include_str!("fixtures/higher_table.va")
        .replace("u=exp", "u=1e-310*exp")
        .replace("0.0,1.0,1.0,3.0,2.0,9.0", "0.0,1.0,1e-310,3.0,2e-310,9.0");
    check_higher_table_derivatives(&source);
}

#[test]
fn table_chain_rule_retains_finite_derivatives_with_unrepresentable_slopes() {
    // The local slopes overflow/underflow binary64, while the complete
    // Jacobians remain representable (one and 1e-310 respectively).
    for (input, data) in [
        ("1e-310*V(p)", "0.0,0.0,1e-310,1.0"),
        ("1e308*V(p)", "0.0,0.0,1e308,1e-310"),
    ] {
        let source = format!(
            "module table_range(p); inout p; electrical p; analog I(p)<+$table_model({input},{data}); endmodule"
        );
        let report = VerilogACompiler::default()
            .compile_runtime(&source, None)
            .unwrap();
        let mut device = VerilogADevice::try_new_with_canonical_ir(
            "T",
            report.model,
            &report.canonical_ir,
            &[1],
        )
        .unwrap();
        #[cfg(feature = "native")]
        assert!(device.is_using_native());
        device.update_voltages(&[0.5]);
        let expected = if input.starts_with("1e-310") {
            1.0
        } else {
            1e-310
        };
        let jacobian = device.try_compute_jacobian().unwrap();
        assert!(!jacobian.is_empty());
        for entry in jacobian {
            assert!(
                (entry.value.abs() / expected - 1.0).abs() < 1e-12,
                "{input}: {entry:?}, expected {expected}"
            );
        }
        let current = device.try_evaluate().unwrap()[0];
        assert!(
            (current / (0.5 * expected) - 1.0).abs() < 1e-12,
            "{current}"
        );
    }
}

#[test]
fn table_derivative_action_nonrepresentable_result_is_numeric_error() {
    let source = "module table_overflow(p); inout p; electrical p; analog I(p)<+$table_model(V(p),0.0,0.0,1e-310,1.0); endmodule";
    let report = VerilogACompiler::default()
        .compile_runtime(source, None)
        .unwrap();
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "Toverflow",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    device.update_voltages(&[0.0]);
    let error = device.try_compute_jacobian().unwrap_err();
    assert!(
        matches!(error, rspice_veriloga::vm::VmError::InvalidNumericResult(_)),
        "{error:?}"
    );
}

#[test]
fn table_derivative_action_preserves_direct_readback_site_identity() {
    let source = "module direct_tables(p); inout p; electrical p; analog I(p)<+ddx($table_model(1e-310*V(p),0.0,0.0,1e-310,1.0),V(p))+ddx($table_model(2*V(p),0.0,0.0,2.0,6.0),V(p)); endmodule";
    let report = VerilogACompiler::default()
        .compile_runtime(source, None)
        .unwrap();
    assert_eq!(report.model.lookup_tables.len(), 2);
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "Tdirect",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    #[cfg(feature = "native")]
    assert!(device.is_using_native());
    for voltage in [-0.5, 0.5, 1.5] {
        device.update_voltages(&[voltage]);
        assert!((device.try_evaluate().unwrap()[0] - 7.0).abs() < 1e-12);
        for entry in device.try_compute_jacobian().unwrap() {
            assert_eq!(entry.value, 0.0);
        }
    }
}

fn check_higher_table_derivatives(source: &str) {
    let report = VerilogACompiler::default()
        .compile_runtime(source, None)
        .unwrap();
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "T3",
        report.model,
        &report.canonical_ir,
        &[1, 0],
    )
    .unwrap();
    #[cfg(feature = "native")]
    assert!(device.is_using_native());
    for voltage in [-0.4_f64, 0.0, 0.3, 0.8, -0.4] {
        device.update_voltages(&[voltage]);
        let u = (2.0 * voltage).exp();
        // The active line has slope two below u=1 and six at/above u=1,
        // including extrapolation beyond u=2. Its derivatives beyond first
        // vanish; the input exp(2*v) contributes 2^n at derivative order n.
        let slope = if u < 1.0 { 2.0 } else { 6.0 };
        let expected = 8.0 * slope * u;
        let jacobian = device.try_compute_jacobian().unwrap();
        assert!(!jacobian.is_empty());
        for entry in jacobian {
            assert!(
                (entry.value.abs() - 2.0 * expected).abs() < 1e-10,
                "v={voltage}: {entry:?}, expected magnitude {}",
                2.0 * expected
            );
        }
        let actual = device.try_evaluate().unwrap()[0];
        assert!(
            (actual - expected).abs() < 1e-10,
            "v={voltage}: {actual} != {expected}"
        );
    }
}
