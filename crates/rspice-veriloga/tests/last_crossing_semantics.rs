use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_analysis_type(2);
    device.set_time(time);
    device.set_timestep(time.max(0.0));
    device.update_voltages(&[voltage]);
    device
        .try_evaluate()
        .expect("last_crossing evaluation succeeds")[0]
}

#[test]
fn last_crossing_interpolates_rising_crossing_time() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module rising_last_crossing(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ last_crossing(V(p, n), 1);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        (-1.0_f64).to_bits()
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 3.0).to_bits(), 0.5_f64.to_bits());
    assert_eq!(
        evaluate(&mut device, 2.0, 3.0).to_bits(),
        0.5_f64.to_bits(),
        "Newton reevaluation must return the same interpolated crossing"
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 3.0, 4.0).to_bits(), 0.5_f64.to_bits());
}

#[test]
fn last_crossing_respects_direction() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module directed_last_crossing(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ last_crossing(V(p, n), -1);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        (-1.0_f64).to_bits()
    );
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0).to_bits(),
        (-1.0_f64).to_bits(),
        "a rising crossing must not satisfy a falling-only query"
    );
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 2.0, -1.0).to_bits(),
        1.5_f64.to_bits()
    );
}
