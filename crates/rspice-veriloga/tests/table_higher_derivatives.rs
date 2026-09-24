//! Derivatives of the existing piecewise-linear table interpolation contract.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;

#[test]
fn higher_table_derivatives_follow_the_active_segment() {
    let source = include_str!("fixtures/higher_table.va");
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
