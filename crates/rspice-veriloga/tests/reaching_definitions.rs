//! An equation reads the definition reaching where it was written.
//!
//! A Verilog-AMS analog block is a sequence of statements, so a contribution
//! reads the values in effect at its own point (LRM 1800.2-2023 section 5). The
//! executable routes do not execute it that way: they run the whole assignment
//! pass and then the equations, which is the same answer only while nothing
//! reassigns the variable below the contribution.
//!
//! Every module here reassigns one, and every expected value is the LRM's
//! answer rather than another route's — arithmetic is the oracle, so a pin can
//! never agree with a defect two routes share. Each keeps the later write
//! observable in a reporting variable, so a run that passed because the write
//! was optimised away would fail on the very next assertion.
//!
//! # Which route each pin holds
//!
//! Every route, unconditionally. Bare, these exercise the bytecode VM: the
//! compiler splices the snapshot into `CompiledModel::assignment_steps` and
//! redirects the stamp programs to read it, and the VM executes that sequence
//! as it stands.
//!
//! Under `native` — and under `wasm-jit`, which takes the same plan — the
//! executable plan does not lower the bytecode when a canonical artifact is
//! present, which is every production build. It replays the canonical HIR's
//! statements instead, and a spliced copy has no statement of its own, so
//! `CompiledModel::reaching_snapshots` tells `jit::plan_builder` which
//! statement each copy belongs after and which read of which equation the
//! snapshot answers. Without that the conductance the first pin measures was
//! 1.25 where the block says 0.0025, with the snapshot slot holding 0.
//!
//! These run on the host, so they reach the WASM route's *plan* but not its
//! emitted module; `wasm_jit`'s own test module executes the first of them as
//! WebAssembly, in wasmi, against the same frame the browser backend uses.
//!
//! The `cfg_mir` census slice over `ekv3_rf` measures the same property on a
//! shipped model.

use rspice_veriloga::device::VerilogADevice;
use std::collections::HashMap;

mod support;

use support::DeviceFixture;

/// The definition reaching the contribution.
const REACHING: f64 = 2.5e-3;
/// What the assignment pass leaves in the slot afterwards. Nothing may read it.
const OVERWRITTEN: f64 = 1.25;
/// The bias every stamp below is taken at.
const BIAS: f64 = 4.0;

fn collect_stamps(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    device.stamp(
        voltages,
        |row, col, value| *matrix.entry((row, col)).or_insert(0.0) += value,
        |node, value| *rhs.entry(node).or_insert(0.0) += value,
    );
    (matrix, rhs)
}

/// The shape `ekv3_302.00` has, at its smallest: a scratch variable a
/// contribution reads and a later statement reassigns.
const REUSED_AFTER_READ: &str = r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    real reported;
    analog begin
        tmp = 2.5e-3;
        I(p, n) <+ V(p, n) * tmp;
        tmp = 1.25;
        reported = tmp;
    end
endmodule
"#;

#[test]
fn a_conductance_is_stamped_from_the_definition_reaching_the_contribution() {
    let fixture = DeviceFixture::compile(REUSED_AFTER_READ);
    let mut device = fixture.device("R1", &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[BIAS]);

    assert_eq!(
        matrix[&(0, 0)],
        REACHING,
        "the conductance is d/dV of V*tmp with tmp at its reaching definition"
    );
    fixture.observe(&mut device);
    assert_eq!(
        device.variable("reported"),
        Some(OVERWRITTEN),
        "the later write has to have happened, or this pin proves nothing"
    );
}

/// The same, with the reassignment written under a condition that holds at the
/// bias. The analyzer folds the guard into `guard ? value : previous`, so the
/// slot is assigned on every evaluation and an equation above it must still not
/// see the result.
const REASSIGNED_UNDER_A_CONDITION: &str = r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    real reported;
    analog begin
        tmp = 2.5e-3;
        I(p, n) <+ V(p, n) * tmp;
        if (V(p, n) > 0.5) begin
            tmp = 1.25;
        end
        reported = tmp;
    end
endmodule
"#;

#[test]
fn a_reassignment_under_a_taken_condition_does_not_reach_the_equation_above_it() {
    let fixture = DeviceFixture::compile(REASSIGNED_UNDER_A_CONDITION);
    let mut device = fixture.device("R1", &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[BIAS]);

    assert_eq!(matrix[&(0, 0)], REACHING);
    fixture.observe(&mut device);
    assert_eq!(
        device.variable("reported"),
        Some(OVERWRITTEN),
        "the guard has to have been taken at this bias"
    );
}

/// A voltage-dependent scratch variable, so the reassignment reaches the
/// derivative as well as the value.
///
/// `tmp = 1.0e-3 * V(p,n)` carries a derivative shadow, and the constant
/// written below it carries a zero one. Reading the slot after the pass would
/// therefore get *both* wrong and get them wrong differently: the residual
/// would be `V * 1.25` and the Jacobian `1.25 + V*0`, where the block says
/// `1e-3*V²` and `2e-3*V`. The noise magnitude reads the same variable, so a
/// route that snapshots only the residual is caught here too.
const REASSIGNED_FEEDS_A_JACOBIAN_AND_A_NOISE_MAGNITUDE: &str = r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    real reported;
    analog begin
        tmp = 1.0e-3 * V(p, n);
        I(p, n) <+ V(p, n) * tmp;
        I(p, n) <+ white_noise(tmp, "reused");
        tmp = 1.25;
        reported = tmp;
    end
endmodule
"#;

#[test]
fn a_jacobian_entry_and_a_noise_magnitude_read_the_reaching_definition() {
    let fixture = DeviceFixture::compile(REASSIGNED_FEEDS_A_JACOBIAN_AND_A_NOISE_MAGNITUDE);
    let mut device = fixture.device("R1", &[1, 0]);
    let (matrix, rhs) = collect_stamps(&mut device, &[BIAS]);

    // I = 1e-3*V², so dI/dV = 2e-3*V and Ieq = I - G*V = -1e-3*V².
    assert_eq!(matrix[&(0, 0)], 2.0e-3 * BIAS);
    assert_eq!(rhs[&0], 1.0e-3 * BIAS * BIAS);
    fixture.observe(&mut device);
    assert_eq!(device.variable("reported"), Some(OVERWRITTEN));

    let noise = device.noise_sources(&[BIAS]);
    assert_eq!(noise.len(), 1, "one white-noise process: {noise:?}");
    assert_eq!(
        noise[0].psd,
        1.0e-3 * BIAS,
        "the noise magnitude reads the same definition the residual does"
    );
}

/// The property that keeps every model without the construct exactly where it
/// was: nothing is captured, so nothing is allocated and no program changes.
#[test]
fn a_module_without_reuse_allocates_no_snapshot_slot() {
    let fixture = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module no_reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    real other;
    analog begin
        tmp = 2.5e-3;
        I(p, n) <+ V(p, n) * tmp;
        other = 1.25;
    end
endmodule
"#,
    );
    assert!(
        !fixture
            .model
            .variable_names
            .iter()
            .any(|name| name.contains("@snap")),
        "{:?}",
        fixture.model.variable_names
    );

    let mut device = fixture.device("R1", &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[BIAS]);
    assert_eq!(matrix[&(0, 0)], REACHING);
}
