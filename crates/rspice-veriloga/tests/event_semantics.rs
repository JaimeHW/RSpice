use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_analysis_type(2);
    device.set_time(time);
    device.set_timestep(time.max(0.0));
    device.update_voltages(&[voltage]);
    device.try_evaluate().expect("event evaluation succeeds")[0]
}

#[test]
fn cross_is_newton_idempotent_and_uses_expression_tolerance() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module tolerant_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 1, 0.0, 0.1, 1.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        1.0_f64.to_bits(),
        "repeated Newton evaluations must preserve the crossing event"
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 1.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn cross_enable_disables_events_without_freezing_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module disabled_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 0, 0.0, 0.0, 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn above_fires_initially_and_only_on_subsequent_rising_events() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module stateful_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n) - 1.5, 0.0, 0.0, 1.0);
endmodule
"#,
    );

    let mut initially_positive = model.device("A1", &[1, 0]);
    assert_eq!(
        evaluate(&mut initially_positive, 0.0, 2.0).to_bits(),
        1.0_f64.to_bits(),
        "above must trigger during initialization when already positive"
    );

    let mut crossing = model.device("A2", &[1, 0]);
    assert_eq!(
        evaluate(&mut crossing, 0.0, 1.0).to_bits(),
        0.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 2.0, 2.5).to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn full_event_control_argument_lists_compile() {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module event_argument_lists(p, n);
    inout p, n;
    electrical p, n;
    real enabled, state;
    analog begin
        enabled = 1.0;
        @(cross(V(p, n), 1, 1.0e-12, 1.0e-6, enabled)) state = 1.0;
        @(above(V(p, n), 1.0e-12, 1.0e-6, enabled)) state = 2.0;
        I(p, n) <+ state;
    end
endmodule
"#,
    );
}

#[test]
fn step_events_follow_phase_and_analysis_filters() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module step_events(p, n);
    inout p, n;
    electrical p, n;
    real event_value;
    analog begin
        event_value = 0.0;
        @(initial_step("tran", "vendor_extension")) event_value = event_value + 1.0;
        @(final_step("tran")) event_value = event_value + 2.0;
        I(p, n) <+ event_value;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[0.0]);

    device.set_analysis_type(2);
    device.set_analysis_step(false, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );

    device.set_analysis_step(true, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        1.0_f64.to_bits()
    );

    device.set_analysis_step(false, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        2.0_f64.to_bits()
    );

    device.set_analysis_step(true, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        3.0_f64.to_bits()
    );

    device.set_analysis_type(0);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn step_events_reject_non_string_analysis_filters() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let error = compiler
        .compile(
            r#"
`include "disciplines.vams"
module invalid_step_event(p, n);
    inout p, n;
    electrical p, n;
    analog @(initial_step("tran", 1.0)) I(p, n) <+ 1.0;
endmodule
"#,
        )
        .expect_err("non-string step-event filter must be rejected");
    assert!(error.to_string().contains("string literals"), "{error}");
}
