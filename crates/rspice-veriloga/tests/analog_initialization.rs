//! Pre-simulation assignments must not be replayed by Newton or observations.

mod support;
use support::DeviceFixture;

#[test]
fn reinitialization_discards_newton_tasks_and_publishes_only_initial_tasks() {
    use rspice_veriloga_runtime::AnalogTaskArgument;
    let fixture = DeviceFixture::compile(
        r#"module initialization_tasks(p,n);
inout p,n; electrical p,n; parameter real gain=1;
analog initial $finish(0);
analog begin $finish(1); I(p,n)<+gain*V(p,n); end
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    device.set_time(2.0);
    device.set_timestep(0.1);
    device.try_evaluate().unwrap();
    let initial = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
    assert_eq!(initial.len(), 1);
    assert_eq!(initial[0].time, 0.0);
    assert_eq!(&*initial[0].arguments, &[AnalogTaskArgument::Integer(0)]);
    device.try_set_parameter("gain", 2.0).unwrap();
    device.try_initialize_analysis().unwrap();
    let initial = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
    assert_eq!(initial.len(), 1);
    assert_eq!(&*initial[0].arguments, &[AnalogTaskArgument::Integer(0)]);
    device.checkpoint_state().unwrap();
}

#[test]
fn illegal_initialization_constructs_are_rejected_before_folding() {
    for body in [
        "if (0) I(p,n)<+1;",
        "if (0) value=ddt(1);",
        "if (0) value=V(p,n);",
        "if (0) @(initial_step) value=1;",
        "if (0) $stop;",
    ] {
        let source = format!(
            "module bad(p,n); inout p,n; electrical p,n; real value; analog initial begin {body} end analog I(p,n)<+0; endmodule"
        );
        let error = rspice_veriloga::VerilogACompiler::default()
            .compile(&source)
            .expect_err(body)
            .to_string();
        assert!(error.contains("pre-simulation initialization"), "{error}");
    }
}

#[test]
fn initializers_cannot_hide_analog_operators_in_function_calls() {
    let error = rspice_veriloga::VerilogACompiler::default()
        .compile(
            r#"module bad(p,n);
inout p,n; electrical p,n; real value;
analog function real inner;
input x; real x;
begin if (0) inner=ddt(x); else inner=x; end
endfunction
analog function real outer;
input x; real x;
begin outer=inner(x); end
endfunction
analog initial value=outer(1);
analog I(p,n)<+value;
endmodule"#,
        )
        .unwrap_err()
        .to_string();
    assert!(error.contains("pre-simulation initialization"), "{error}");
}

#[test]
fn declarations_and_analog_initial_run_once_per_analysis() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module initialized(p,n);
inout p,n; electrical p,n;
parameter real gain=2;
localparam real offset=gain+1;
integer count=5;
real scale;
analog initial begin count=count+1; scale=offset; end
analog begin count=count+1; I(p,n)<+count+scale*V(p,n); end
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    assert!(device.checkpoint_state().is_err());
    assert!(device.validate_advance_state().is_err());
    device.try_begin_analysis(0).unwrap();
    device.checkpoint_state().unwrap();
    assert_eq!(device.variable("count"), Some(6.0));
    device.update_voltages(&[2.0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![13.0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![13.0]);
    fixture.observe(&mut device);
    device.try_compute_jacobian().unwrap();
    device.try_advance_state().unwrap();
    assert_eq!(device.try_evaluate().unwrap(), vec![14.0]);
    device.try_begin_analysis(0).unwrap();
    assert_eq!(device.try_evaluate().unwrap(), vec![13.0]);
    device.try_set_parameter("gain", 4.0).unwrap();
    device.try_begin_analysis(0).unwrap();
    assert_eq!(device.try_evaluate().unwrap(), vec![17.0]);
}

#[test]
fn lazy_initialization_precedes_static_topology_probes() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module initialized(p,n);
inout p,n; electrical p,n;
real divisor=0;
analog initial divisor=2;
analog begin
  if (divisor>0) V(p,n)<+1.0/divisor;
end
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![0.5]);
    assert_eq!(device.try_evaluate().unwrap(), vec![0.5]);
}

#[test]
fn initialized_array_values_survive_newton_replay() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module initialized(p,n);
inout p,n; electrical p,n;
real a[1:3]='{1,2,3};
integer index;
analog initial begin index=1; while(index<=3) begin a[index]=a[index]+10; index=index+1; end end
analog I(p,n)<+a[1]+a[2]+a[3];
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![36.0]);
    device.try_advance_state().unwrap();
    assert_eq!(device.try_evaluate().unwrap(), vec![36.0]);
}

#[test]
fn a_retained_counter_cannot_freeze_a_contribution_guard() {
    let fixture = DeviceFixture::compile(
        r#"module counter(p,n);
inout p,n; electrical p,n;
integer count=0;
analog begin count=count+1; if(count>1) I(p,n)<+1; end
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![0.0]);
    device.try_advance_state().unwrap();
    assert_eq!(device.try_evaluate().unwrap(), vec![1.0]);
}
