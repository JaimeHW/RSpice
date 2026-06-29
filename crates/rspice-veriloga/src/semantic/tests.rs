//! Tests for Verilog-A semantic analysis.

use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::source::SourceId;

fn analyze(source: &str) -> CompileResult<AnalyzedFile> {
    let tokens = Lexer::new(source, SourceId::new(0))
        .collect_tokens()
        .expect("lex failed");
    let file = Parser::new(&tokens).parse().expect("parse failed");
    SemanticAnalyzer::new().analyze(&file)
}

fn analyze_one(source: &str) -> AnalyzedModule {
    let analyzed = analyze(source).expect("semantic analysis failed");
    analyzed.modules.into_values().next().expect("one module")
}

const PREAMBLE: &str = r#"
        module dut(p, n);
        inout p, n;
        electrical p, n;
    "#;

fn module_src(body: &str) -> String {
    format!("{PREAMBLE}{body}\nendmodule")
}

/// Flatten the statement tree into the assignments it contains
/// (loop bodies included), preserving order
fn flat_assignments(m: &AnalyzedModule) -> Vec<&AnalyzedAssignment> {
    fn walk<'a>(stmts: &'a [AnalyzedStatement], out: &mut Vec<&'a AnalyzedAssignment>) {
        for stmt in stmts {
            match stmt {
                AnalyzedStatement::Assignment(a) => out.push(a),
                AnalyzedStatement::Loop(l) => walk(&l.body, out),
            }
        }
    }
    let mut out = Vec::new();
    walk(&m.statements, &mut out);
    out
}

#[test]
fn conditional_assignment_lowered_to_guarded_expression() {
    let m = analyze_one(&module_src(
        r#"
            parameter integer mode = 0;
            real x;
            analog begin
                if (mode > 0)
                    x = 1.0;
                else
                    x = 2.0;
                I(p, n) <+ x * V(p, n);
            end
            "#,
    ));
    // Guard snapshot + the two guarded branch assignments
    assert_eq!(flat_assignments(&m).len(), 3);
    // The condition is snapshotted once so branch bodies cannot
    // perturb it
    assert!(flat_assignments(&m)[0].target.starts_with("__guard"));
    // Both branch assignments must be guarded conditionals, not raw values
    assert!(matches!(
        flat_assignments(&m)[1].expression,
        Expression::Conditional(_)
    ));
    assert!(matches!(
        flat_assignments(&m)[2].expression,
        Expression::Conditional(_)
    ));
    // The else-branch guard preserves the previous value via the variable
    let Expression::Conditional(c) = &flat_assignments(&m)[2].expression else {
        unreachable!()
    };
    assert!(matches!(*c.else_expr, Expression::Identifier(_)));
}

#[test]
fn branch_body_cannot_perturb_its_own_guard() {
    // The classic NOT_GIVEN defaulting idiom: only ONE arm may run.
    // Without condition snapshotting the then-branch assignment makes
    // the re-evaluated else-guard true as well.
    let m = analyze_one(&module_src(
        r#"
            real t;
            analog begin
                t = -1.0;
                if (t < 0.0)
                    t = 25.0;
                else
                    t = t + 273.15;
                I(p, n) <+ t * 1e-6 * V(p, n);
            end
            "#,
    ));

    // Emulate the VM: execute assignments in order over a variable map
    let mut vars: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    fn eval(expr: &Expression, vars: &std::collections::HashMap<&str, f64>) -> f64 {
        match expr {
            Expression::Number(n) => n.value,
            Expression::Identifier(id) => vars.get(id.name.as_str()).copied().unwrap_or(0.0),
            Expression::Unary(u) => {
                let v = eval(&u.operand, vars);
                match u.op {
                    UnaryOp::Neg => -v,
                    UnaryOp::Pos => v,
                    UnaryOp::Not => f64::from(v == 0.0),
                    UnaryOp::BitNot => !(v as i64) as f64,
                }
            }
            Expression::Binary(b) => {
                let l = eval(&b.left, vars);
                let r = eval(&b.right, vars);
                match b.op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Lt => f64::from(l < r),
                    BinaryOp::And => f64::from(l != 0.0 && r != 0.0),
                    _ => f64::NAN,
                }
            }
            Expression::Conditional(c) => {
                if eval(&c.condition, vars) != 0.0 {
                    eval(&c.then_expr, vars)
                } else {
                    eval(&c.else_expr, vars)
                }
            }
            _ => f64::NAN,
        }
    }
    for assign in flat_assignments(&m) {
        let value = eval(&assign.expression, &vars);
        // Keys live as long as the module borrow
        let key: &str = Box::leak(assign.target.to_string().into_boxed_str());
        vars.insert(key, value);
    }
    assert_eq!(
        vars.get("t").copied(),
        Some(25.0),
        "only the then-arm may execute; got {vars:?}"
    );
}

#[test]
fn conditional_contribution_contributes_zero_when_inactive() {
    let m = analyze_one(&module_src(
        r#"
            parameter integer on = 1;
            analog begin
                if (on)
                    I(p, n) <+ V(p, n);
            end
            "#,
    ));
    assert_eq!(m.contributions.len(), 1);
    let Expression::Conditional(c) = &m.contributions[0].expression else {
        panic!("expected guarded contribution");
    };
    let Expression::Number(zero) = &*c.else_expr else {
        panic!("expected zero fallback");
    };
    assert_eq!(zero.value, 0.0);
}

#[test]
fn for_loop_unrolls_statically() {
    let m = analyze_one(&module_src(
        r#"
            integer i;
            real x;
            analog begin
                for (i = 0; i < 3; i = i + 1)
                    x = x + 1.0;
            end
            "#,
    ));
    assert_eq!(flat_assignments(&m).len(), 3);
}

#[test]
fn large_constant_for_loop_lowers_to_runtime_loop() {
    let m = analyze_one(&module_src(
        r#"
            integer i;
            real x;
            analog begin
                for (i = 0; i < 40; i = i + 1)
                    x = x + 1.0;
            end
            "#,
    ));
    assert!(
        m.statements
            .iter()
            .any(|stmt| matches!(stmt, AnalyzedStatement::Loop(_))),
        "large constant loops should stay compact instead of being unrolled"
    );
}

#[test]
fn localparam_becomes_computed_variable() {
    let m = analyze_one(&module_src(
        r#"
            parameter real w = 2.0;
            localparam real area = w * 3.0;
            analog I(p, n) <+ area * V(p, n);
            "#,
    ));
    assert!(m.variables.iter().any(|v| v.name == "area"));
    assert_eq!(flat_assignments(&m).len(), 1);
    assert_eq!(flat_assignments(&m)[0].target.as_str(), "area");
}

#[test]
fn variable_initializer_recorded() {
    let m = analyze_one(&module_src(
        r#"
            real x = 4.5;
            analog I(p, n) <+ x * V(p, n);
            "#,
    ));
    assert_eq!(flat_assignments(&m).len(), 1);
    assert_eq!(flat_assignments(&m)[0].target.as_str(), "x");
}

#[test]
fn named_branch_contribution_resolves_nodes() {
    let m = analyze_one(&module_src(
        r#"
            branch (p, n) res;
            analog I(res) <+ V(res) / 2.0;
            "#,
    ));
    assert_eq!(m.contributions.len(), 1);
    assert_eq!(m.contributions[0].branch.as_str(), "p,n");
    assert!(m.contributions[0].is_current);
}

#[test]
fn parameter_default_out_of_range_is_an_error() {
    let result = analyze(&module_src(
        r#"
            parameter real r = -1.0 from (0:inf);
            analog I(p, n) <+ V(p, n) / r;
            "#,
    ));
    assert!(result.is_err(), "out-of-range default must fail");
}

#[test]
fn assignment_to_parameter_is_an_error() {
    let result = analyze(&module_src(
        r#"
            parameter real r = 1.0;
            analog begin
                r = 2.0;
                I(p, n) <+ V(p, n) / r;
            end
            "#,
    ));
    assert!(result.is_err(), "assigning a parameter must fail");
}

#[test]
fn undeclared_node_in_contribution_is_an_error() {
    let result = analyze(&module_src(
        r#"
            analog I(p, ghost) <+ V(p, n);
            "#,
    ));
    assert!(result.is_err());
}

#[test]
fn block_local_variable_shadowing_is_hoisted() {
    let m = analyze_one(&module_src(
        r#"
            real tmp;
            analog begin : outer
                real tmp;
                tmp = V(p, n);
                I(p, n) <+ tmp;
            end
            "#,
    ));
    // Module-level tmp plus a hoisted (renamed) block-local tmp
    assert_eq!(
        m.variables
            .iter()
            .filter(|v| v.name.starts_with("tmp"))
            .count(),
        2
    );
    // The block assignment targets the hoisted name, not the outer tmp
    assert_ne!(flat_assignments(&m)[0].target.as_str(), "tmp");
}

#[test]
fn user_function_is_inlined() {
    let m = analyze_one(&module_src(
        r#"
            parameter real gain = 2.0;
            analog function real double_it;
                input v;
                real scratch;
                begin
                    scratch = 2.0 * v;
                    double_it = scratch;
                end
            endfunction
            analog I(p, n) <+ double_it(V(p, n)) * gain;
            "#,
    ));
    assert_eq!(m.contributions.len(), 1);
    // No residual user-function calls may remain after inlining
    fn has_user_call(expr: &Expression) -> bool {
        match expr {
            Expression::Call(c) => c.name == "double_it" || c.args.iter().any(has_user_call),
            Expression::Binary(b) => has_user_call(&b.left) || has_user_call(&b.right),
            Expression::Unary(u) => has_user_call(&u.operand),
            Expression::Conditional(c) => {
                has_user_call(&c.condition)
                    || has_user_call(&c.then_expr)
                    || has_user_call(&c.else_expr)
            }
            _ => false,
        }
    }
    assert!(!has_user_call(&m.contributions[0].expression));
}

#[test]
fn function_with_conditional_inlines_to_ternary() {
    let m = analyze_one(&module_src(
        r#"
            analog function real clip;
                input v;
                begin
                    if (v > 1.0)
                        clip = 1.0;
                    else
                        clip = v;
                end
            endfunction
            analog I(p, n) <+ clip(V(p, n));
            "#,
    ));
    assert!(matches!(
        m.contributions[0].expression,
        Expression::Conditional(_)
    ));
}

#[test]
fn function_formal_redeclaration_keeps_bound_input_value() {
    let m = analyze_one(&module_src(
        r#"
            real y;
            analog function real add_one;
                input v;
                real v;
                begin
                    add_one = v + 1.0;
                end
            endfunction
            analog y = add_one(4.0);
            "#,
    ));

    fn eval(expr: &Expression) -> f64 {
        match expr {
            Expression::Number(n) => n.value,
            Expression::Binary(b) => {
                let l = eval(&b.left);
                let r = eval(&b.right);
                match b.op {
                    BinaryOp::Add => l + r,
                    _ => f64::NAN,
                }
            }
            _ => f64::NAN,
        }
    }

    assert_eq!(eval(&flat_assignments(&m)[0].expression), 5.0);
}

#[test]
fn function_output_argument_updates_caller_variable_before_assignment_result() {
    let m = analyze_one(&module_src(
        r#"
            real y, z;
            analog function real set_pair;
                output yout;
                input xin;
                real yout, xin;
                begin
                    yout = xin + 2.0;
                    set_pair = yout + 1.0;
                end
            endfunction
            analog z = set_pair(y, 3.0);
            "#,
    ));

    let assignments = flat_assignments(&m);
    let y_pos = assignments
        .iter()
        .position(|assignment| assignment.target == "y")
        .expect("output argument must update caller variable");
    let z_pos = assignments
        .iter()
        .position(|assignment| assignment.target == "z")
        .expect("function return must assign caller target");
    assert!(
        y_pos < z_pos,
        "output argument must update before return target"
    );
    assert!(matches!(
        assignments[z_pos].expression,
        Expression::Identifier(_)
    ));
}

#[test]
fn nested_function_output_argument_materializes_before_outer_assignment() {
    let m = analyze_one(&module_src(
        r#"
            real y, z;
            analog function real set_pair;
                output yout;
                input xin;
                real yout, xin;
                begin
                    yout = xin + 2.0;
                    set_pair = yout + 1.0;
                end
            endfunction
            analog z = 1.0 + set_pair(y, 3.0);
            "#,
    ));

    let assignments = flat_assignments(&m);
    let y_pos = assignments
        .iter()
        .position(|assignment| assignment.target == "y")
        .expect("nested output argument must update caller variable");
    let z_pos = assignments
        .iter()
        .position(|assignment| assignment.target == "z")
        .expect("outer assignment must still be emitted");
    assert!(y_pos < z_pos);
    assert!(matches!(
        assignments[z_pos].expression,
        Expression::Binary(_)
    ));
}

#[test]
fn function_output_argument_in_conditional_expression_is_a_compile_error() {
    let error = analyze(&module_src(
        r#"
            real y, z;
            analog function real set_pair;
                output yout;
                input xin;
                real yout, xin;
                begin
                    yout = xin + 2.0;
                    set_pair = yout + 1.0;
                end
            endfunction
            analog z = V(p, n) > 0.0 ? set_pair(y, 3.0) : 0.0;
            "#,
    ))
    .expect_err("conditional output-function side effects must be rejected");

    assert!(
        error
            .to_string()
            .contains("not supported inside conditional expressions"),
        "unexpected error: {error}"
    );
}

#[test]
fn thermal_contribution_is_flow() {
    let m = analyze_one(
        r#"
            module heater(p, n, t);
            inout p, n, t;
            electrical p, n;
            thermal t;
            analog Pwr(t) <+ V(p, n) * V(p, n) / 10.0;
            endmodule
            "#,
    );
    assert_eq!(m.contributions.len(), 1);
    assert!(
        m.contributions[0].is_current,
        "power into a thermal node is a flow contribution"
    );
}

#[test]
fn ground_net_does_not_allocate_internal_node() {
    let m = analyze_one(&module_src(
        r#"
            ground gnd;
            electrical mid;
            analog begin
                I(p, mid) <+ V(p, mid);
                I(mid, gnd) <+ V(mid, gnd);
            end
            "#,
    ));
    assert_eq!(m.internal_nodes.len(), 1);
    assert_eq!(m.internal_nodes[0].name.as_str(), "mid");
    assert_eq!(m.ground_nodes, vec![SmolStr::from("gnd")]);
}

#[test]
fn numeric_zero_branch_endpoint_references_global_ground() {
    let m = analyze_one(&module_src(
        r#"
            analog begin
                I(p, 0) <+ V(p, 0);
                I(0, p) <+ V(0, p);
            end
            "#,
    ));

    assert_eq!(m.contributions.len(), 2);
    assert_eq!(m.contributions[0].branch.as_str(), "p,0");
    assert!(m.contributions[0].is_current);
    assert_eq!(m.contributions[1].branch.as_str(), "0,p");
    assert!(m.contributions[1].is_current);
}

#[test]
fn initial_step_lowered_to_static_analysis_guard() {
    let m = analyze_one(&module_src(
        r#"
            real seed;
            analog begin
                @(initial_step) seed = 1.0;
                I(p, n) <+ seed * V(p, n);
            end
            "#,
    ));
    // [0] is the snapshotted analysis() guard, [1] the guarded seed
    let snapshot = flat_assignments(&m)[0];
    assert!(snapshot.target.starts_with("__guard"));
    let Expression::Call(call) = &snapshot.expression else {
        panic!(
            "expected analysis() snapshot, got {:?}",
            snapshot.expression
        );
    };
    assert_eq!(call.name.as_str(), "analysis");
    let Expression::Conditional(c) = &flat_assignments(&m)[1].expression else {
        panic!("expected guarded assignment");
    };
    assert!(matches!(*c.condition, Expression::Identifier(_)));
}

#[test]
fn while_loop_with_runtime_condition_lowers_to_runtime_loop() {
    let m = analyze_one(&module_src(
        r#"
            real x;
            analog begin
                while (x < 10.0) x = x + 1.0;
            end
            "#,
    ));
    assert!(
        m.statements
            .iter()
            .any(|s| matches!(s, AnalyzedStatement::Loop(_))),
        "runtime while must lower to a loop statement"
    );
}

#[test]
fn parameter_bounded_for_loop_lowers_to_runtime_loop() {
    let m = analyze_one(&module_src(
        r#"
            parameter integer nf = 4;
            integer i;
            real acc;
            analog begin
                acc = 0.0;
                for (i = 0; i < nf; i = i + 1)
                    acc = acc + 2.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
    ));
    let loops: Vec<_> = m
        .statements
        .iter()
        .filter(|s| matches!(s, AnalyzedStatement::Loop(_)))
        .collect();
    assert_eq!(loops.len(), 1, "parameter-bounded loop stays a loop");
    let AnalyzedStatement::Loop(l) = loops[0] else {
        unreachable!()
    };
    // Body: accumulator update + loop variable update
    assert_eq!(l.body.len(), 2);
}

#[test]
fn guarded_runtime_loop_condition_includes_guard() {
    let m = analyze_one(&module_src(
        r#"
            parameter integer nf = 4;
            parameter integer en = 1;
            integer i;
            real acc;
            analog begin
                if (en > 0)
                    for (i = 0; i < nf; i = i + 1)
                        acc = acc + 1.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
    ));
    let AnalyzedStatement::Loop(l) = m
        .statements
        .iter()
        .find(|s| matches!(s, AnalyzedStatement::Loop(_)))
        .expect("loop present")
    else {
        unreachable!()
    };
    // The enclosing guard is ANDed into the loop condition
    assert!(
        matches!(&l.condition, Expression::Binary(b) if b.op == BinaryOp::And),
        "guard must be folded into the loop condition, got {:?}",
        l.condition
    );
}

#[test]
fn contribution_inside_runtime_loop_is_an_error() {
    let result = analyze(&module_src(
        r#"
            parameter integer nf = 4;
            integer i;
            analog begin
                for (i = 0; i < nf; i = i + 1)
                    I(p, n) <+ V(p, n);
            end
            "#,
    ));
    assert!(
        result.is_err(),
        "contributions need compile-time-constant loop bounds"
    );
}

#[test]
fn runtime_repeat_synthesizes_counter() {
    let m = analyze_one(&module_src(
        r#"
            parameter integer nf = 3;
            real acc;
            analog begin
                repeat (nf) acc = acc + 1.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
    ));
    assert!(
        m.statements
            .iter()
            .any(|s| matches!(s, AnalyzedStatement::Loop(_))),
        "runtime repeat lowers to a loop"
    );
    assert!(
        m.variables.iter().any(|v| v.name.starts_with("__repeat")),
        "synthesized counter variables registered"
    );
}
