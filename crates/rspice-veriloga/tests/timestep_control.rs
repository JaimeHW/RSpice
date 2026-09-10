//! $bound_step / $discontinuity lowering: hidden per-evaluation
//! variables drive the engine's stepper. The bound resets to +inf and
//! takes the min of every active call; the discontinuity flag resets to
//! zero and reports rising edges across accepted steps.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile_device(instance: &str, source: &str) -> VerilogADevice {
    compile_selected_device(instance, source, None)
}

fn compile_selected_device(instance: &str, source: &str, module: Option<&str>) -> VerilogADevice {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile_module(source, module)
        .expect("compile timestep-control model");
    #[cfg(feature = "native")]
    {
        let canonical_ir = compiler
            .compile_canonical_ir_module(source, module)
            .expect("compile timestep-control canonical IR");
        VerilogADevice::try_new_with_canonical_ir(instance, model, &canonical_ir, &[1, 0])
            .expect("construct timestep-control device from canonical IR")
    }
    #[cfg(not(feature = "native"))]
    {
        VerilogADevice::try_new(instance, model, &[1, 0])
            .expect("construct timestep-control bytecode device")
    }
}

fn stamp_once(device: &mut VerilogADevice, voltages: &[f64]) {
    device.stamp(voltages, |_, _, _| {}, |_, _| {});
}

const BOUNDED: &str = r#"
`include "disciplines.vams"
module bounded(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    analog begin
        $bound_step(1.0e-6);
        if (V(p, n) > 0.5)
            $bound_step(cap);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn bound_step_takes_the_min_of_active_calls() {
    let mut device = compile_device("B1", BOUNDED);

    // Guard inactive: only the unconditional 1 us bound applies
    stamp_once(&mut device, &[0.2]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-6));

    // Guard active: min(1u, 1n) = 1 ns
    stamp_once(&mut device, &[0.8]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-9));

    // And it resets per evaluation rather than latching
    stamp_once(&mut device, &[0.2]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-6));
}

#[test]
fn control_tasks_inside_runtime_loops_reset_once_per_evaluation() {
    let mut device = compile_device(
        "LOOP",
        r#"
module loop_controls(p,n);
    inout p,n; electrical p,n;
    parameter integer passes=3;
    integer i;
    analog begin
        for (i=1; i<=passes; i=i+1) begin
            $bound_step(i*1e-9);
            if (i==1) $discontinuity(0);
        end
        I(p,n) <+ V(p,n)*1e-3;
    end
endmodule
"#,
    );
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1e-9));
    assert!(device.discontinuity_pending());
    device.advance_state();

    assert!(device.try_set_parameter("passes", 0.0).unwrap());
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), None);
    assert!(!device.discontinuity_pending());
    device.advance_state();

    assert!(device.try_set_parameter("passes", 2.0).unwrap());
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1e-9));
    assert!(device.discontinuity_rising());
}

const UNBOUNDED: &str = r#"
`include "disciplines.vams"
module plain(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3;
endmodule
"#;

#[test]
fn models_without_bound_step_report_none() {
    let mut device = compile_device("P1", UNBOUNDED);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.transient_bound_step(), None);
    assert!(!device.discontinuity_pending());
}

const PARAMETERIZED_BOUND: &str = r#"
`include "disciplines.vams"
module requested_bound(p, n);
    inout p, n;
    electrical p, n;
    parameter real limit = 0.0;
    analog begin
        $bound_step(limit);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn zero_bound_requests_the_solver_minimum_and_invalid_bounds_fail_closed() {
    let mut device = compile_device("BZERO", PARAMETERIZED_BOUND);
    stamp_once(&mut device, &[0.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(0.0));

    assert!(device.try_set_parameter("limit", -1.0).unwrap());
    stamp_once(&mut device, &[0.0]);
    let error = device
        .try_transient_bound_step()
        .expect_err("negative $bound_step requests must fail closed");
    assert!(error.to_string().contains("$bound_step"), "{error}");
}

const DISCONTINUOUS: &str = r#"
`include "disciplines.vams"
module disco(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (V(p, n) > 1.0)
            $discontinuity(0);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn discontinuity_reports_rising_edges_only() {
    let mut device = compile_device("D1", DISCONTINUOUS);

    // Below threshold: quiet
    stamp_once(&mut device, &[0.5]);
    assert!(!device.discontinuity_pending());
    assert!(!device.discontinuity_rising());
    device.advance_state();

    // Crossing: pending and rising
    stamp_once(&mut device, &[1.5]);
    assert!(device.discontinuity_pending());
    assert!(device.discontinuity_rising());
    device.advance_state();

    // Still above threshold: pending but no longer rising (a level-true
    // region must not pin tiny steps forever)
    stamp_once(&mut device, &[1.6]);
    assert!(device.discontinuity_pending());
    assert!(!device.discontinuity_rising());
    device.advance_state();

    // Dropping back re-arms the edge detector
    stamp_once(&mut device, &[0.4]);
    assert!(!device.discontinuity_pending());
    device.advance_state();
    stamp_once(&mut device, &[1.2]);
    assert!(device.discontinuity_rising());
}

#[test]
fn child_control_tasks_reach_the_solver_after_hierarchy_flattening() {
    let source = r#"
module leaf(p,n);
 inout p,n; electrical p,n;
 parameter real bound=1e-9;
 analog begin
  if(V(p,n)>0.0) begin $bound_step(bound); $discontinuity(0); end
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
module nested(p,n);
 inout p,n; electrical p,n;
 leaf #(.bound(2e-9)) inner(p,n);
endmodule
module top(p,n);
 inout p,n; electrical p,n;
 nested a(p,n);
 leaf #(.bound(1e-9)) b(p,n);
 analog $bound_step(3e-9);
endmodule
"#;
    let mut device = compile_selected_device("hierarchy", source, Some("top"));
    stamp_once(&mut device, &[1.0]);
    assert_eq!(
        device.try_transient_bound_step().unwrap(),
        Some(1e-9),
        "{:?}",
        device.variables().collect::<Vec<_>>()
    );
    assert!(device.discontinuity_pending());
    device.advance_state();
    stamp_once(&mut device, &[-1.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(3e-9));
    assert!(!device.discontinuity_pending());
}
