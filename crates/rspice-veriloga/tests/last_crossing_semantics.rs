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

#[test]
fn last_crossing_retains_parameter_and_procedural_direction_operands() {
    let model = DeviceFixture::compile(
        "module crossing(p,n); inout p,n; electrical p,n; parameter integer dir=1; integer choice; real first; analog begin choice=dir; first=last_crossing(V(p,n),choice); choice=-dir; I(p,n) <+ first+10*last_crossing(V(p,n),choice); end endmodule",
    );
    for (direction, rising, falling) in [(1.0, -9.5, 28.0), (-1.0, 4.0, 7.75)] {
        let mut device = model.device("A1", &[1, 0]);
        device.try_set_parameter("dir", direction).unwrap();
        assert_eq!(evaluate(&mut device, 0.0, -1.0), -11.0);
        device.advance_state();
        assert_eq!(evaluate(&mut device, 2.0, 3.0), rising);
        device.advance_state();
        let accepted = device.checkpoint_state().unwrap();
        assert!(
            accepted
                .accepted
                .cross_detectors
                .iter()
                .all(|state| state.value == 3.0 && state.time == 2.0),
            "{:?}",
            accepted.accepted.cross_detectors
        );
        let result = evaluate(&mut device, 3.0, -1.0);
        device.try_advance_state().unwrap();
        assert_eq!(
            result,
            falling,
            "{:?}",
            device.checkpoint_state().unwrap().accepted.cross_detectors
        );
    }
}

#[test]
fn last_crossing_rejects_out_of_domain_direction() {
    let model = DeviceFixture::compile(
        "module crossing(p,n); inout p,n; electrical p,n; parameter integer dir=2; analog I(p,n) <+ last_crossing(V(p,n),dir); endmodule",
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    device.update_voltages(&[-1.0]);
    let error = device
        .try_evaluate()
        .expect_err("direction outside -1..=1 must fail");
    assert!(
        error
            .to_string()
            .contains("last_crossing direction must be -1, 0, or 1"),
        "{error}"
    );
}
