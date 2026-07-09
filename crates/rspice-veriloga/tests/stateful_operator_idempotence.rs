#![cfg(not(feature = "native"))]

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::sync::Arc;

fn device(source: &str) -> VerilogADevice {
    let model = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("stateful-operator model compiles");
    VerilogADevice::try_new("A1", Arc::new(model), &[1, 0]).expect("device constructs")
}

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_time(time);
    device.update_voltages(&[voltage]);
    device.try_evaluate().expect("evaluation succeeds")[0]
}

#[test]
fn laplace_recomputes_from_the_last_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module laplace_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(1.0, {1.0}, {1.0, 1.0});
endmodule
"#,
    );
    device.set_analysis_type(2);
    device.set_timestep(1.0);

    let first = evaluate(&mut device, 1.0, 0.0);
    let repeated = evaluate(&mut device, 1.0, 0.0);
    assert!((first - 0.5).abs() < 1.0e-12, "first={first}");
    assert_eq!(first.to_bits(), repeated.to_bits());

    device.advance_state();
    let next = evaluate(&mut device, 2.0, 0.0);
    assert!((next - 0.75).abs() < 1.0e-12, "next={next}");
}

#[test]
fn absdelay_does_not_consume_history_during_newton_reevaluation() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module delay_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 2.0, 10.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 1.0), 1.0);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 2.0);
    for _ in 0..2048 {
        assert_eq!(evaluate(&mut device, 2.0, 2.0), first);
    }
    assert_eq!(first, 1.0);
}

#[test]
fn transition_restarts_each_candidate_from_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module transition_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 2.0, 2.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 0.0);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 1.0);
    let alternate = evaluate(&mut device, 2.0, 2.0);
    let repeated = evaluate(&mut device, 2.0, 1.0);
    assert!((first - 0.5).abs() < 1.0e-12, "first={first}");
    assert_eq!(first.to_bits(), repeated.to_bits());
    assert_eq!(alternate.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn slew_restarts_each_candidate_from_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 0.5, 0.5);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 0.5);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 1.0);
    let alternate = evaluate(&mut device, 2.0, 0.0);
    let repeated = evaluate(&mut device, 2.0, 1.0);
    assert_eq!(first.to_bits(), 1.0_f64.to_bits());
    assert_eq!(alternate.to_bits(), 0.0_f64.to_bits());
    assert_eq!(first.to_bits(), repeated.to_bits());
}

#[test]
fn crossing_remains_visible_for_every_newton_reevaluation() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module cross_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), +1);
endmodule
"#,
    );

    device.set_analysis_type(0);
    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();

    device.set_analysis_type(2);
    let first = evaluate(&mut device, 1.0, 1.0);
    let repeated = evaluate(&mut device, 1.0, 1.0);
    assert_eq!(first.to_bits(), 1.0_f64.to_bits());
    assert_eq!(first.to_bits(), repeated.to_bits());
}
