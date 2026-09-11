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
fn indirect_validation_rejects_duplicate_constraints_in_both_compilers() {
    for statements in [
        "V(p,n): V(q,n)==1; V(p,n): V(q,n)==2;",
        "V(p,n): V(q,n)==1; I(n,p): V(q,n)==2;",
        "V(a): V(q,n)==1; V(a): V(q,n)==2;",
    ] {
        let source = format!(
            "module bad(p,n,q); inout p,n,q; electrical p,n,q;
            branch(p,n) a; analog begin {statements} end endmodule"
        );
        let compiler = VerilogACompiler::default();
        let error = compiler
            .compile_canonical_ir(&source)
            .expect_err("duplicate constraint must fail");
        assert!(error.to_string().contains("over-determined"), "{error}");
        assert!(compile_err(&source).contains("over-determined"));
    }
}

#[test]
fn indirect_validation_requires_analysis_constant_controls() {
    for control in [
        "if(V(q,n)>0)",
        "if($abstime>0)",
        "if(analysis(\"ic\"))",
        "if(analysis(\"nodeset\"))",
        "if(analysis(\"static\"))",
        "case(V(q,n)) 1:",
    ] {
        let end = if control.starts_with("case") {
            "endcase"
        } else {
            ""
        };
        let source = format!(
            "module bad(p,n,q); inout p,n,q; electrical p,n,q;
            analog {control} V(p,n): V(q,n)==0; {end} endmodule"
        );
        let compiler = VerilogACompiler::default();
        let error = compiler
            .compile_canonical_ir(&source)
            .expect_err("dynamic constraint control must fail");
        assert!(
            error.to_string().contains("constant during an analysis"),
            "{error}"
        );
        assert!(compile_err(&source).contains("constant during an analysis"));
    }
    for condition in [
        "enabled",
        "!enabled",
        "analysis(\"ac\")",
        "analysis(\"tran\",\"noise\")",
        "analysis(\"unknown_analysis\")",
    ] {
        let source = format!(
            "module allowed(p,n,q); inout p,n,q; electrical p,n,q;
            parameter integer enabled=1;
            analog if({condition}) V(p,n): ddt(V(q,n))==I(p,n); endmodule"
        );
        let compiler = VerilogACompiler::default();
        compiler
            .compile_canonical_ir(&source)
            .unwrap_or_else(|error| panic!("{condition}: {error}"));
        compiler
            .compile(&source)
            .unwrap_or_else(|error| panic!("{condition}: {error}"));
    }
}

#[test]
fn indirect_validation_requires_an_access_on_the_equation_left_side() {
    for lhs in [
        "1",
        "2*V(q,n)",
        "sin(V(q,n))",
        "ddt(2*V(q,n))",
        "ddt(ddt(V(q,n)))",
    ] {
        let source = format!(
            "module bad(p,n,q); inout p,n,q; electrical p,n,q;
            analog V(p,n): {lhs}==I(p,n); endmodule"
        );
        let compiler = VerilogACompiler::default();
        let error = compiler
            .compile_canonical_ir(&source)
            .expect_err("invalid constraint LHS must fail");
        assert!(error.to_string().contains("left side"), "{error}");
        assert!(compile_err(&source).contains("left side"));
    }
    for lhs in [
        "V(q,n)",
        "ddt(V(q,n))",
        "idt(V(q,n),0)",
        "idtmod(V(q,n),0,1)",
        "I(<q>)",
    ] {
        let source = format!(
            "module allowed(p,n,q); inout p,n,q; electrical p,n,q;
            analog V(p,n): {lhs}==I(p,n); endmodule"
        );
        let compiler = VerilogACompiler::default();
        compiler
            .compile_canonical_ir(&source)
            .unwrap_or_else(|error| panic!("{lhs}: {error}"));
        compiler
            .compile(&source)
            .unwrap_or_else(|error| panic!("{lhs}: {error}"));
    }
}

#[test]
fn guarded_indirect_sources_stamp_active_constraints_and_inactive_identity_rows() {
    let model = compile(
        "module gated(p,n); inout p,n; electrical p,n;
        parameter integer en=1; analog if(en) V(p,n): V(p,n)==3; endmodule",
    );
    let mut device = model.device("X", &[1, 0]);
    assert!(model.stamp_programs[0].static_condition.is_some());
    device.set_branch_current_indices(&[2]);
    for enabled in [1.0, 0.0, 1.0] {
        assert!(device.set_parameter("en", enabled));
        device.try_resolve_parameter_defaults().unwrap();
        for bias in [[0.25, 0.0], [3.0, -0.00275]] {
            let mut matrix = [[0.0; 2]; 2];
            let mut rhs = [0.0; 2];
            device
                .try_stamp(&bias, |r, c, v| matrix[r][c] += v, |r, v| rhs[r] += v)
                .unwrap();
            assert_eq!(
                matrix,
                [[0.0, enabled], [enabled, 1.0 - enabled]],
                "en={enabled}, bias={bias:?}"
            );
            assert_eq!(rhs, [0.0, 3.0 * enabled], "en={enabled}, bias={bias:?}");
        }
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
    assert!(err.contains("direct and indirect"), "got: {err}");
}

#[test]
fn direct_current_cannot_share_an_indirect_source_pair() {
    for statements in [
        "V(p,n): V(q,n)==0; I(p,n)<+1;",
        "I(p,n)<+1; V(p,n): V(q,n)==0;",
        "V(p,n): V(q,n)==0; I(n,p)<+1;",
        "I(p,n): V(q,n)==0; I(p,n)<+1;",
    ] {
        let source = format!(
            "module bad(p,n,q); inout p,n,q; electrical p,n,q; analog begin {statements} end endmodule"
        );
        let err = compile_err(&source);
        assert!(err.contains("direct") && err.contains("indirect"), "{err}");
    }
}

#[test]
fn parallel_named_branches_cannot_mix_direct_and_indirect_sources() {
    for statements in [
        "V(a): V(q,n)==0; I(b)<+1;",
        "V(a): V(q,n)==0; V(b)<+1;",
        "I(b)<+1; V(a): V(q,n)==0;",
        "V(b)<+1; V(a): V(q,n)==0;",
        "V(a): V(q,n)==0; I(p,n)<+1;",
    ] {
        let source = format!(
            "module bad(p,n,q); inout p,n,q; electrical p,n,q; branch(p,n) a; branch(n,p) b; analog begin {statements} end endmodule"
        );
        let err = compile_err(&source);
        assert!(err.contains("direct") && err.contains("indirect"), "{err}");
    }
}

#[test]
fn indirect_conflicts_resolve_ground_aliases_and_hierarchy_bindings() {
    for source in [
        "module bad(p,q); inout p,q; electrical p,q,g; ground g; analog begin V(p,g): V(q)==0; I(p)<+1; end endmodule",
        "module child(p,n,q); inout p,n,q; electrical p,n,q; analog V(p,n): V(q,n)==0; endmodule
         module bad(p,n,q); inout p,n,q; electrical p,n,q; child a(p,n,q); analog I(n,p)<+1; endmodule",
    ] {
        let compiler = VerilogACompiler::default();
        let error = compiler.compile_module(source, Some("bad")).expect_err("mixed sources must fail");
        assert!(error.to_string().contains("direct and indirect"), "{error}");
        let error = compiler.compile_canonical_ir_module(source, Some("bad")).expect_err("canonical compilation must reject the same source");
        assert!(error.to_string().contains("direct and indirect"), "{error}");
    }
}

#[test]
fn indirect_sources_allow_direct_contributions_on_different_pairs() {
    compile(
        "module allowed(p,n,q); inout p,n,q; electrical p,n,q; branch(p,n) a; branch(q,n) b; analog begin V(a): V(q,n)==0; I(b)<+1; end endmodule",
    );
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

#[test]
fn indirect_equation_tolerances_follow_the_left_hand_nature() {
    for (lhs, expected) in [
        ("V(q,n)", 1e-6),
        ("I(q,n)", 1e-12),
        ("I(<q>)", 1e-12),
        ("idt(V(q,n),0)", 1e-9),
        ("idt(V(q,n))", 1e-9),
        ("idt(I(q,n),0)", 1e-14),
        ("ddt(V(q,n),Current)", 1e-12),
    ] {
        let model = compile(&format!(
            "module tolerances(p,q,n); inout p,q,n; electrical p,q,n;
            analog V(p,n): {lhs}==I(p,n); endmodule"
        ));
        let mut device = model.device("X", &[1, 2, 0]);
        device.set_branch_current_indices(&(3..3 + model.branch_sources.len()).collect::<Vec<_>>());
        let mut values = Vec::new();
        device.visit_equation_abstols(9e-12, |row, tol| values.push((row, tol)));
        assert_eq!(values, [(3, expected)], "{lhs}");
    }
    let model = compile(
        "nature FineTemperature; units=\"K\"; access=Heat; abstol=2e-8;
        idt_nature=IntegratedTemperature; endnature
        nature IntegratedTemperature; units=\"K*s\"; access=HeatIntegral; abstol=3e-10; endnature
        discipline customthermal; potential FineTemperature; flow Power; enddiscipline
        module custom(p,q); inout p,q; electrical p; customthermal q;
        analog V(p): idt(Heat(q),0)==I(p); endmodule",
    );
    let mut device = model.device("X", &[1, 2]);
    device.set_branch_current_indices(&[3]);
    device.visit_equation_abstols(1e-12, |row, tol| assert_eq!((row, tol), (3, 3e-10)));
}

#[test]
fn indirect_equation_tolerances_resolve_parameters_atomically() {
    let model = compile(
        "module tolerance(p); inout p; electrical p;
        parameter real tolerance=2e-9; parameter integer enabled=1;
        analog if(enabled) V(p): ddt(V(p),tolerance*2)==I(p); endmodule",
    );
    let mut device = model.device("X", &[1]);
    device.set_branch_current_indices(&[2]);
    for (value, enabled, expected) in [(3e-9, 1.0, 6e-9), (8e-9, 0.0, 1e-12), (0.0, 1.0, 0.0)] {
        assert!(device.set_parameter("tolerance", value));
        assert!(device.set_parameter("enabled", enabled));
        device.try_resolve_parameter_defaults().unwrap();
        let mut tolerances = Vec::new();
        device.visit_equation_abstols(1e-12, |row, tol| tolerances.push((row, tol)));
        assert_eq!(tolerances, [(2, expected)]);
    }
    assert!(device.set_parameter("tolerance", -1.0));
    let error = device.try_resolve_parameter_defaults().unwrap_err();
    assert!(error.to_string().contains("absolute tolerance"), "{error}");
    device.visit_equation_abstols(1e-12, |_, tol| assert_eq!(tol, 0.0));
    assert!(device.set_parameter("tolerance", f64::MAX));
    assert!(device.try_resolve_parameter_defaults().is_err());
    assert!(device.set_parameter("tolerance", 1e-9));
    device.try_resolve_parameter_defaults().unwrap();
    device.visit_equation_abstols(1e-12, |_, tol| assert_eq!(tol, 2e-9));
}

#[test]
fn indirect_equation_tolerances_reject_dynamic_operands() {
    for tolerance in [
        "V(p)",
        "$abstime",
        "sin(V(p))",
        "ddt(V(p))",
        "analysis(\"tran\")",
    ] {
        let source = format!(
            "module bad(p); inout p; electrical p;
            analog V(p): ddt(V(p),{tolerance})==I(p); endmodule"
        );
        let compiler = VerilogACompiler::default();
        assert!(
            compiler.compile_canonical_ir(&source).is_err(),
            "{tolerance}"
        );
        assert!(compiler.compile(&source).is_err(), "{tolerance}");
    }
}

#[test]
fn indirect_equation_tolerances_survive_hierarchy_parameter_binding() {
    let source = "module child(p); inout p; electrical p; parameter real tol=1e-9;
        analog V(p): ddt(V(p),tol)==I(p); endmodule
        module top(p); inout p; electrical p; parameter real scale=2;
        child #(.tol(scale*1e-12)) x(p); endmodule";
    let report = VerilogACompiler::default()
        .compile_runtime(source, Some("top"))
        .unwrap();
    let model = DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    let mut device = model.device("X", &[1]);
    device.set_branch_current_indices(&[2]);
    assert!(device.set_parameter("scale", 3.0));
    device.try_resolve_parameter_defaults().unwrap();
    device.visit_equation_abstols(1e-12, |_, tol| assert_eq!(tol, 3e-12));
}

#[test]
fn indirect_named_branch_tolerances_include_both_endpoints() {
    for lhs in ["V(q,r)", "V(pair)"] {
        let model = compile(&format!(
            "nature TightVoltage : Voltage; abstol=2e-12; endnature
            discipline tight; potential TightVoltage; flow Current; enddiscipline
            module named(p,q,r); inout p,q,r; electrical p,q; tight r;
            branch(q,r) pair; analog V(p): {lhs}==0; endmodule"
        ));
        let mut device = model.device("X", &[1, 2, 3]);
        device.set_branch_current_indices(&[4]);
        let mut values = Vec::new();
        device.visit_equation_abstols(1e-12, |row, tol| values.push((row, tol)));
        assert_eq!(values, [(4, 2e-12)], "{lhs}");
    }
    let model = compile(
        "nature ColdTemperature : Temperature; abstol=4e-8; endnature
        discipline cold; potential ColdTemperature; flow Power; enddiscipline
        module groundprobe(p); inout p; electrical p; cold reference; ground reference;
        analog V(p): Temp(reference)==I(p); endmodule",
    );
    let mut device = model.device("X", &[1]);
    device.set_branch_current_indices(&[2]);
    let mut values = Vec::new();
    device.visit_equation_abstols(1e-12, |row, tol| values.push((row, tol)));
    assert_eq!(values, [(2, 4e-8)]);
}

#[test]
fn indirect_tolerance_resolution_preserves_scope_and_initial_condition_effects() {
    for (source, expected) in [
        (
            "module scope(p); inout p; electrical p; parameter real Current=2e-9;
          analog V(p): ddt(V(p),Current)==I(p); endmodule",
            2e-9,
        ),
        (
            "module initial_effect(p); inout p; electrical p; real seen;
          analog function real initial_value; output observed; real observed;
          begin observed=7; initial_value=0; end endfunction
          analog V(p): idt(V(p),initial_value(seen))==I(p); endmodule",
            1e-9,
        ),
    ] {
        let model = compile(source);
        let mut device = model.device("X", &[1]);
        device.set_branch_current_indices(&[2]);
        let mut values = Vec::new();
        device.visit_equation_abstols(1e-12, |row, tol| values.push((row, tol)));
        assert_eq!(values, [(2, expected)]);
    }
}
