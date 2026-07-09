use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice) -> f64 {
    device.update_voltages(&[0.0]);
    device.try_evaluate().expect("evaluation succeeds")[0]
}

#[test]
fn tiny_nonzero_reals_use_exact_equality_and_truth_rules() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module exact_real_semantics(p, n);
    inout p, n;
    electrical p, n;
    parameter real eps = 1.0e-16;
    analog I(p, n) <+ ((eps == 0.0) ? 1.0 : 0.0) + (eps ? 10.0 : 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device).to_bits(), 10.0_f64.to_bits());
}

#[test]
fn positive_and_negative_zero_are_false_and_equal() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module signed_zero_semantics(p, n);
    inout p, n;
    electrical p, n;
    parameter real value = -0.0;
    analog I(p, n) <+ (value ? 100.0 : 1.0)
                       + ((value == 0.0) ? 10.0 : 0.0)
                       + ((value != 0.0) ? 1000.0 : 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device).to_bits(), 11.0_f64.to_bits());
}
