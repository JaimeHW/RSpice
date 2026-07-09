use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice, time: f64, timestep: f64) -> f64 {
    device.set_analysis_type(2);
    device.set_time(time);
    device.set_timestep(timestep);
    device.update_voltages(&[0.0]);
    device.try_evaluate().expect("timer evaluation succeeds")[0]
}

#[test]
fn one_shot_timer_fires_once_and_schedules_an_exact_timepoint() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module one_shot_timer(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ timer(1.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), Some(1.0));

    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0).to_bits(),
        1.0_f64.to_bits(),
        "Newton reevaluation must preserve the event level"
    );
    assert_eq!(device.transient_bound_step(), None);

    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 1.1, 0.1).to_bits(),
        0.0_f64.to_bits(),
        "the accepted one-shot event must not repeat"
    );
}

#[test]
fn periodic_timer_schedules_each_future_event() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module periodic_timer(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ timer(1.0, 0.5, 0.0, 1.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), Some(1.0));
    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 1.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), Some(0.5));
    assert_eq!(evaluate(&mut device, 1.5, 0.5).to_bits(), 1.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), Some(0.5));
}

#[test]
fn disabled_timer_neither_fires_nor_limits_the_stepper() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module disabled_timer(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ timer(1.0, 0.5, 1.0e-12, 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), None);
    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(device.transient_bound_step(), None);
}
