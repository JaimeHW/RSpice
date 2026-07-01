//! Indirect branch assignments: `V(x): lhs == rhs`.
//!
//! The target branch carries an unknown source; the solver picks its
//! value so the constraint holds. MNA shape: the branch unknown couples
//! into the KCL rows of the target pair (±1 columns), and the branch row
//! holds the constraint residual f = lhs - rhs = 0 — there is no
//! structural V(p) - V(n) relation on that row.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

mod support;

use support::DeviceFixture;

fn compile(source: &str) -> DeviceFixture {
    DeviceFixture::compile(source)
}

fn compile_err(source: &str) -> String {
    match VerilogACompiler::new(CompilerOptions::default()).compile(source) {
        Ok(_) => panic!("compilation must fail"),
        Err(err) => err.to_string(),
    }
}

#[test]
fn ideal_opamp_compiles_with_one_branch_unknown() {
    // V(out): V(inp, inn) == 0 — the classic ideal-opamp idiom
    let model = compile(
        r#"
`include "disciplines.vams"
module opamp(out, inp, inn);
    inout out, inp, inn;
    electrical out, inp, inn;
    analog V(out): V(inp, inn) == 0.0;
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1, "one constraint unknown");
    assert!(model.branch_sources[0].indirect);
    assert_eq!(model.stamp_programs.len(), 1);
    assert!(model.stamp_programs[0].indirect);
}

/// Solve the device's own 2x2-ish linearized system by collecting stamps
/// against explicit unknown indices and checking the constraint algebra.
/// Terminals: out=row 0, inp=row 1, inn=row 2; branch row = 3.
#[test]
fn constraint_row_carries_the_equation_not_the_branch_voltage() {
    let model = compile(
        r#"
`include "disciplines.vams"
module opamp(out, inp, inn);
    inout out, inp, inn;
    electrical out, inp, inn;
    analog V(out): V(inp, inn) == 0.0;
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 2, 3, 0]);
    device.set_branch_current_indices(&[4]);

    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    device.stamp(
        &[0.2, 0.7, 0.4, 0.05],
        |r, c, v| *matrix.entry((r, c)).or_insert(0.0) += v,
        |n, v| *rhs.entry(n).or_insert(0.0) += v,
    );

    // KCL column couplings: the unknown current enters out's KCL row
    assert!((matrix.get(&(0, 3)).copied().unwrap_or(0.0) - 1.0).abs() < 1e-12);
    // The branch row must NOT carry the structural V(out) entry of a
    // voltage source row; it carries the constraint d(V(inp)-V(inn))
    assert_eq!(matrix.get(&(3, 0)).copied().unwrap_or(0.0), 0.0);
    assert!((matrix.get(&(3, 1)).copied().unwrap_or(0.0) - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(3, 2)).copied().unwrap_or(0.0) + 1.0).abs() < 1e-12);
    // Linear constraint: companion RHS of the branch row is zero
    assert!(rhs.get(&3).copied().unwrap_or(0.0).abs() < 1e-12);
}

#[test]
fn mixing_direct_and_indirect_on_one_branch_is_rejected() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module bad(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        V(p, n) <+ 1.0;
        V(p, n): V(p, n) == 2.0;
    end
endmodule
"#,
    );
    assert!(err.contains("over-determined"), "got: {err}");
}

#[test]
fn two_indirect_constraints_on_one_branch_are_rejected() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module bad2(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        V(p, n): V(p, n) == 1.0;
        V(p, n): V(p, n) == 2.0;
    end
endmodule
"#,
    );
    assert!(err.contains("over-determined"), "got: {err}");
}

#[test]
fn equation_without_equality_is_a_parse_error() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module bad3(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n): V(p, n) + 1.0;
endmodule
"#,
    );
    assert!(err.contains("=="), "got: {err}");
}
