//! $bound_step / $discontinuity lowering: hidden per-evaluation
//! variables drive the engine's stepper. The bound resets to +inf and
//! takes the min of every active call; the discontinuity flag resets to
//! zero and reports rising edges across accepted steps.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
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
    let model = compile(BOUNDED);
    let mut device = VerilogADevice::new("B1", model, &[1, 0]);

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
    let model = compile(UNBOUNDED);
    let mut device = VerilogADevice::new("P1", model, &[1, 0]);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.transient_bound_step(), None);
    assert!(!device.discontinuity_pending());
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
    let model = compile(DISCONTINUOUS);
    let mut device = VerilogADevice::new("D1", model, &[1, 0]);

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
