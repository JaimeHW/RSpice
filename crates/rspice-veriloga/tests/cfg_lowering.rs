//! HIR body to control-flow-graph lowering.
//!
//! Two things are checked here. The focused tests pin the shapes the lowering
//! must produce — a conditional contribution is a diamond whose join merges a
//! running total, not a guard folded into an expression — and the corpus survey
//! reports what fraction of the shipped models the level can already consume.
//!
//! The survey is a ratchet, not a pass/fail on individual models: the rebuild
//! (`design/VERILOGA_BACKEND_PLAN.md`) is mid-flight, and a number that can only
//! go up is what keeps it honest while it is.

use rspice_veriloga::canonical_ir::cfg::{CfgBinaryOp, CfgTerminator, CfgUnaryOp, CfgValueKind};
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CfgEvalInputs, IrDiagnostic, evaluate_cfg,
};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

fn lower(source: &str) -> CfgModel {
    let artifact = artifact(source);
    match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
        Ok(model) => model,
        Err(diagnostics) => panic!("{}", render(&diagnostics)),
    }
}

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
}

fn render(diagnostics: &[IrDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The shape the whole rebuild is about: a guarded contribution is a branch and
/// a merge, not a `guard ? value : 0` folded into one expression.
#[test]
fn a_guarded_contribution_becomes_a_branch_and_a_merge() {
    let model = lower(
        r#"
module guarded(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0;
    analog begin
        if (gain > 0.0) begin
            I(p, n) <+ gain * V(p, n);
        end
    end
endmodule
"#,
    );

    let function = &model.function;
    let branches = function
        .blocks
        .iter()
        .filter(|block| matches!(block.terminator, CfgTerminator::Branch { .. }))
        .count();
    assert_eq!(branches, 1, "the `if` must be the only branch");

    let joins: Vec<_> = function
        .blocks
        .iter()
        .filter(|block| !block.params.is_empty())
        .collect();
    assert_eq!(
        joins.len(),
        1,
        "exactly one join carries the accumulated residual"
    );
    assert_eq!(joins[0].params.len(), 1);

    // No value in the graph is a select: that kind does not exist here.
    assert_eq!(model.residuals.len(), 1);
    assert_eq!(
        function.value(model.residuals[0]).kind,
        CfgValueKind::BlockParameter,
        "the residual leaving a guarded contribution is the merge"
    );
}

/// An unguarded module needs no control flow at all — one block, no parameters.
#[test]
fn a_straight_line_module_lowers_to_a_single_block() {
    let model = lower(
        r#"
module straight(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    real g;
    analog begin
        g = 1.0 / r;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    assert_eq!(model.function.blocks.len(), 1);
    assert!(model.function.blocks[0].params.is_empty());
    assert!(matches!(
        model.function.blocks[0].terminator,
        CfgTerminator::Return
    ));
}

/// A variable written in one arm and read after the `if` merges once. This is
/// the case the level being replaced answers by searching an assignment
/// history; here the builder simply knows.
#[test]
fn a_variable_assigned_in_one_arm_merges_at_the_join() {
    let model = lower(
        r#"
module merged(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    real g;
    analog begin
        g = 1.0;
        if (sel > 0.0) begin
            g = 2.0;
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    let parameters: usize = model
        .function
        .blocks
        .iter()
        .map(|block| block.params.len())
        .sum();
    assert_eq!(
        parameters, 1,
        "one variable crosses the join, so one block parameter"
    );
}

/// Both arms writing the same value must not manufacture a merge; a redundant
/// parameter is a redundant derivative lane downstream.
#[test]
fn arms_that_agree_need_no_merge() {
    let model = lower(
        r#"
module agreeing(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    real g;
    analog begin
        if (sel > 0.0) begin
            g = 4.0;
        end else begin
            g = 4.0;
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    let parameters: usize = model
        .function
        .blocks
        .iter()
        .map(|block| block.params.len())
        .sum();
    assert_eq!(parameters, 0, "identical arms collapse to the value itself");
}

/// `?:` is a diamond, because evaluating only the taken side is the point.
#[test]
fn a_conditional_expression_lowers_to_a_diamond() {
    let model = lower(
        r#"
module ternary(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    analog begin
        I(p, n) <+ (sel > 0.0 ? exp(V(p, n)) : 0.0);
    end
endmodule
"#,
    );

    let branches = model
        .function
        .blocks
        .iter()
        .filter(|block| matches!(block.terminator, CfgTerminator::Branch { .. }))
        .count();
    assert_eq!(branches, 1);
    assert!(
        model
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::Unary { .. })),
        "the expensive arm must still be in the graph, just not on both paths"
    );
}

/// A run-time loop is a back edge, and the loop-carried variable is a header
/// parameter. Getting the sealing order wrong here reads the initial value
/// forever, which is exactly the bug that never surfaces in a unit-free test.
#[test]
fn a_runtime_loop_carries_its_variable_through_a_header_parameter() {
    let model = lower(
        r#"
module counted(p, n);
    inout p, n;
    electrical p, n;
    parameter integer steps = 4;
    real total;
    integer i;
    analog begin
        total = 0.0;
        i = 0;
        while (i < steps) begin
            total = total + 1.0;
            i = i + 1;
        end
        I(p, n) <+ total * V(p, n);
    end
endmodule
"#,
    );

    let header = model
        .function
        .blocks
        .iter()
        .find(|block| block.params.len() == 2)
        .expect("the header merges both loop-carried variables");
    assert!(
        model.function.predecessors(header.id).len() == 2,
        "a header is reached by the entry edge and the back edge"
    );
}

/// Contributions to the same branch from different arms each keep their own
/// accumulator, so the stamping layer sees the same equation count either way.
#[test]
fn each_contribution_statement_keeps_its_own_residual() {
    let model = lower(
        r#"
module twice(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    analog begin
        I(p, n) <+ V(p, n);
        if (sel > 0.0) begin
            I(p, n) <+ 2.0 * V(p, n);
        end
    end
endmodule
"#,
    );

    assert_eq!(model.residuals.len(), 2);
}

#[test]
fn event_written_variables_are_seeded_and_returned_as_explicit_candidates() {
    let source = r#"
module event_candidate(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("ac", "noise")) count = count + 1.0;
        @(final_step("ac", "noise")) count = count + 10.0;
        I(p, n) <+ count * V(p, n);
    end
endmodule
"#;
    let artifact = artifact(source);
    let model = lower(source);
    assert_eq!(model.event_state_candidates.len(), 1);
    assert!(
        model
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::EventState(0)))
    );

    let bias = bias_point(&artifact);
    let mut inputs = cfg_inputs(&bias);
    inputs.event_state = vec![4.0];
    let inactive = evaluate_cfg(&model.function, &inputs).expect("inactive event evaluation");
    assert_eq!(inactive.value(model.event_state_candidates[0]), Some(4.0));

    inputs.analyses.insert("ac".into());
    inputs.analyses.insert("__rspice_initial_step".into());
    let initial = evaluate_cfg(&model.function, &inputs).expect("initial-step evaluation");
    assert_eq!(initial.value(model.event_state_candidates[0]), Some(5.0));

    inputs.analyses.remove("__rspice_initial_step");
    inputs.analyses.insert("__rspice_final_step".into());
    let final_step = evaluate_cfg(&model.function, &inputs).expect("final-step evaluation");
    assert_eq!(
        final_step.value(model.event_state_candidates[0]),
        Some(14.0)
    );
}

#[test]
fn analog_event_controls_lower_to_keyed_cfg_values_with_explicit_defaults() {
    let source = r#"
module event_operators(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(cross(V(p, n), 1)) count = count + 1.0;
        @(above(V(p, n))) count = count + 10.0;
        @(timer(1.0)) count = count + 100.0;
        I(p, n) <+ count * V(p, n);
    end
endmodule
"#;
    let artifact = artifact(source);
    let model = lower(source);
    assert_eq!(model.event_state_candidates.len(), 1);

    let mut cross = None;
    let mut above = None;
    let mut timer = None;
    for value in &model.function.values {
        match &value.kind {
            CfgValueKind::Cross {
                operator,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                assert!(matches!(
                    &model.function.value(*direction).kind,
                    CfgValueKind::RealConstant(1.0)
                ));
                for default in [time_tol, expr_tol] {
                    assert!(matches!(
                        &model.function.value(*default).kind,
                        CfgValueKind::RealConstant(0.0)
                    ));
                }
                assert!(matches!(
                    &model.function.value(*enable).kind,
                    CfgValueKind::RealConstant(1.0)
                ));
                assert!(cross.replace(*operator).is_none(), "one cross site");
            }
            CfgValueKind::Above {
                operator,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                for default in [time_tol, expr_tol] {
                    assert!(matches!(
                        &model.function.value(*default).kind,
                        CfgValueKind::RealConstant(0.0)
                    ));
                }
                assert!(matches!(
                    &model.function.value(*enable).kind,
                    CfgValueKind::RealConstant(1.0)
                ));
                assert!(above.replace(*operator).is_none(), "one above site");
            }
            CfgValueKind::Timer {
                operator,
                period,
                time_tol,
                enable,
                ..
            } => {
                for default in [period, time_tol] {
                    assert!(matches!(
                        &model.function.value(*default).kind,
                        CfgValueKind::RealConstant(0.0)
                    ));
                }
                assert!(matches!(
                    &model.function.value(*enable).kind,
                    CfgValueKind::RealConstant(1.0)
                ));
                assert!(timer.replace(*operator).is_none(), "one timer site");
            }
            _ => {}
        }
    }
    let cross = cross.expect("cross CFG value");
    let above = above.expect("above CFG value");
    let timer = timer.expect("timer CFG value");
    assert_ne!(cross, above);
    assert_ne!(cross, timer);
    assert_ne!(above, timer);

    let bias = bias_point(&artifact);
    let mut inputs = cfg_inputs(&bias);
    inputs.event_state = vec![10.0];
    for (operator, expected) in [(cross, 11.0), (above, 20.0), (timer, 110.0)] {
        inputs.event_controls.clear();
        inputs.event_controls.insert(operator, 1.0);
        let snapshot = evaluate_cfg(&model.function, &inputs).expect("event CFG evaluation");
        assert_eq!(
            snapshot.value(model.event_state_candidates[0]),
            Some(expected)
        );
    }

    inputs.event_controls = [(cross, 1.0), (above, 1.0), (timer, 1.0)]
        .into_iter()
        .collect();
    let all = evaluate_cfg(&model.function, &inputs).expect("combined event CFG evaluation");
    assert_eq!(all.value(model.event_state_candidates[0]), Some(121.0));
}

#[test]
fn single_argument_cross_expression_gets_the_either_direction_default() {
    let model = lower(
        r#"
module cross_default(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (cross(V(p, n)))
            I(p, n) <+ 1.0;
    end
endmodule
"#,
    );
    let mut crosses = model.function.values.iter().filter_map(|value| {
        let CfgValueKind::Cross { direction, .. } = &value.kind else {
            return None;
        };
        Some(*direction)
    });
    let direction = crosses.next().expect("cross CFG value");
    assert!(crosses.next().is_none(), "one cross site");
    assert!(matches!(
        &model.function.value(direction).kind,
        CfgValueKind::RealConstant(0.0)
    ));
}

/// An operand skipped inside an event operator's argument list means the
/// default, exactly as leaving it off the end does.
///
/// `cross(v, 1, 1e-9, , 1.0)` is how a model reaches the enable without naming
/// an expression tolerance, and it is the only way to: the front end decides
/// which operands may be skipped, and everything it lets through is a spelling
/// the executable backends have always read. This level used to refuse the
/// whole module over the null.
#[test]
fn a_skipped_event_operator_argument_takes_the_same_default_as_an_omitted_one() {
    let skipped = lower(
        r#"
module skipped(p, n);
    inout p, n;
    electrical p, n;
    parameter real on = 1.0;
    analog begin
        if (cross(V(p, n) - 0.5, 1, 1.0e-9, , on))
            I(p, n) <+ 1.0;
    end
endmodule
"#,
    );
    let (expr_tol, enable) = skipped
        .function
        .values
        .iter()
        .find_map(|value| match &value.kind {
            CfgValueKind::Cross {
                expr_tol, enable, ..
            } => Some((*expr_tol, *enable)),
            _ => None,
        })
        .expect("cross CFG value");
    assert!(
        matches!(
            skipped.function.value(expr_tol).kind,
            CfgValueKind::RealConstant(0.0)
        ),
        "a skipped expression tolerance is the default"
    );
    assert!(
        matches!(
            skipped.function.value(enable).kind,
            CfgValueKind::Parameter(_)
        ),
        "the operand after the skipped one keeps its own value"
    );
}

/// Every standard math function the executable backends lower must lower here
/// too.
///
/// The CFG is meant to become the only level a backend reads, and a level that
/// cannot spell `asin` cannot replace one that can. These are exactly the names
/// `native/expr.rs` maps to a `NativeOp` and this level used to refuse; keeping
/// the list here means a future addition to one side shows up as a failure on
/// the other rather than as a model that quietly stops compiling.
#[test]
fn every_math_intrinsic_the_executable_backends_lower_also_lowers_here() {
    // `0.31 * V` keeps the inverse circular functions inside their domain at
    // any bias the fixtures use, and `acosh` is given the other side of one.
    let model = lower(
        r#"
module intrinsics(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    analog begin
        I(p, n) <+ g * (exp(V(p, n)) + ln(2.0 + V(p, n)) + log10(2.0 + V(p, n)))
                 + g * (sqrt(1.0 + abs(V(p, n))) + floor(V(p, n)) + ceil(V(p, n)))
                 + g * (sin(V(p, n)) + cos(V(p, n)) + tan(V(p, n)))
                 + g * (sinh(V(p, n)) + cosh(V(p, n)) + tanh(V(p, n)))
                 + g * (asin(0.31 * V(p, n)) + acos(0.31 * V(p, n)) + atan(V(p, n)))
                 + g * (asinh(V(p, n)) + acosh(1.5 + V(p, n) * V(p, n)))
                 + g * atanh(0.31 * V(p, n))
                 + g * (min(V(p, n), 1.0) + max(V(p, n), -1.0) + pow(2.0 + V(p, n), 1.5))
                 + g * (hypot(V(p, n), 1.0) + atan2(V(p, n), 1.0));
    end
endmodule
"#,
    );
    let unary: HashSet<CfgUnaryOp> = model
        .function
        .values
        .iter()
        .filter_map(|value| match value.kind {
            CfgValueKind::Unary { op, .. } => Some(op),
            _ => None,
        })
        .collect();
    for op in [
        CfgUnaryOp::Log10,
        CfgUnaryOp::Asin,
        CfgUnaryOp::Acos,
        CfgUnaryOp::Acosh,
        CfgUnaryOp::Atanh,
    ] {
        assert!(unary.contains(&op), "{op:?} did not survive lowering");
    }
    let binary: HashSet<CfgBinaryOp> = model
        .function
        .values
        .iter()
        .filter_map(|value| match value.kind {
            CfgValueKind::Binary { op, .. } => Some(op),
            _ => None,
        })
        .collect();
    for op in [CfgBinaryOp::Hypot, CfgBinaryOp::Atan2] {
        assert!(binary.contains(&op), "{op:?} did not survive lowering");
    }
}

#[test]
fn analysis_lists_are_static_but_lifecycle_event_topology_remains_dynamic() {
    let analysis_artifact = artifact(
        r#"
module analysis_list_topology(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (analysis("dc", "tran"))
            V(p, n) <+ 1.0;
    end
endmodule
"#,
    );
    let analysis_list = CfgModel::from_hir(&analysis_artifact.hir, &analysis_artifact.mir)
        .expect("analysis-list fixture must lower");
    let analysis_bias = bias_point(&analysis_artifact);
    let mut analysis_inputs = cfg_inputs(&analysis_bias);
    let inactive = evaluate_cfg(&analysis_list.function, &analysis_inputs)
        .expect("inactive analysis-list evaluation");
    assert_eq!(
        inactive.value(analysis_list.activations[0].expect("potential activation")),
        Some(0.0),
        "an instance-static analysis list must leave the branch open outside the listed analyses"
    );
    analysis_inputs.analyses.insert("tran".into());
    let active = evaluate_cfg(&analysis_list.function, &analysis_inputs)
        .expect("active analysis-list evaluation");
    assert_eq!(
        active.value(analysis_list.activations[0].expect("potential activation")),
        Some(1.0)
    );

    let lifecycle_artifact = artifact(
        r#"
module lifecycle_event_topology(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        @(final_step("ac")) V(p, n) <+ 1.0;
    end
endmodule
"#,
    );
    let lifecycle_event = CfgModel::from_hir(&lifecycle_artifact.hir, &lifecycle_artifact.mir)
        .expect("lifecycle-event fixture must lower");
    let lifecycle_bias = bias_point(&lifecycle_artifact);
    let lifecycle_inputs = cfg_inputs(&lifecycle_bias);
    let before_final = evaluate_cfg(&lifecycle_event.function, &lifecycle_inputs)
        .expect("pre-final lifecycle evaluation");
    assert_eq!(
        before_final.value(lifecycle_event.activations[0].expect("potential activation")),
        Some(1.0),
        "final-step topology is runtime-dependent, so its physical branch must stay closed before the event"
    );
}

// --- numeric equivalence ---------------------------------------------------

/// The two levels must agree on numbers, not just on shape.
///
/// The CFG interpreter also checks numeric execution rather than shape alone.
/// different routes — one folds conditionals into selects and evaluates every
/// arm, the other branches — so agreeing to the last bit is evidence the
/// lowering preserved meaning. A shape assertion could not have told us that.
#[test]
fn the_cfg_interpreter_evaluates_every_residual() {
    for (name, source) in equivalence_fixtures() {
        let artifact = artifact(source);
        let cfg = match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
            Ok(model) => model,
            Err(diagnostics) => panic!("{name}: {}", render(&diagnostics)),
        };

        let bias = bias_point(&artifact);
        let snapshot = evaluate_cfg(&cfg.function, &cfg_inputs(&bias))
            .unwrap_or_else(|error| panic!("{name}: CFG reference failed: {error}"));

        for (equation, residual) in cfg.residuals.iter().copied().enumerate() {
            let actual = snapshot
                .value(residual)
                .unwrap_or_else(|| panic!("{name}: equation {equation} has no CFG residual"));
            assert!(
                actual.is_finite(),
                "{name}: equation {equation} produced non-finite residual {actual}"
            );
        }
    }
}

fn equivalence_fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "linear",
            r#"
module linear(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        ),
        (
            "guarded",
            r#"
module guarded(p, n);
    inout p, n;
    electrical p, n;
    parameter real ion = 1.0;
    real g;
    analog begin
        g = 0.001;
        if (ion > 0.5) begin
            g = 0.004;
        end else begin
            g = 0.002;
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
        ),
        (
            "nested",
            r#"
module nested(p, n);
    inout p, n;
    electrical p, n;
    parameter real mode = 2.0;
    real g;
    analog begin
        g = 1.0e-3;
        if (mode > 0.5) begin
            if (mode > 1.5) begin
                g = 3.0e-3;
            end else begin
                g = 2.0e-3;
            end
        end
        I(p, n) <+ g * V(p, n) + 1.0e-9 * (exp(V(p, n) / 0.026) - 1.0);
    end
endmodule
"#,
        ),
        (
            "case",
            r#"
module selected(p, n);
    inout p, n;
    electrical p, n;
    parameter integer sel = 1;
    real g;
    analog begin
        g = 0.0;
        case (sel)
            0: g = 1.0e-3;
            1: g = 2.0e-3;
            2: g = 4.0e-3;
            default: g = 8.0e-3;
        endcase
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
        ),
        (
            "ternary",
            r#"
module ternary(p, n);
    inout p, n;
    electrical p, n;
    parameter real is = 1.0e-14;
    analog I(p, n) <+ (V(p, n) > 0.0)
        ? is * (exp(V(p, n) / 0.02585) - 1.0)
        : is * V(p, n) / 0.02585;
endmodule
"#,
        ),
        (
            "internal node",
            r#"
module series(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r1;
        I(mid, n) <+ V(mid, n) / r2;
    end
endmodule
"#,
        ),
        (
            "min and max",
            r#"
module clamped(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    analog I(p, n) <+ g * max(min(V(p, n), 0.7), -0.7);
endmodule
"#,
        ),
    ]
}

/// Bias values shared by both interpreters. The pattern matters only in that it
/// is not symmetric: equal potentials would hide a sign error in a difference.
struct BiasPoint {
    parameters: Vec<f64>,
    node_potentials: Vec<f64>,
    branch_flows: Vec<f64>,
}

fn bias_point(artifact: &CanonicalIrArtifact) -> BiasPoint {
    let parameters = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default.unwrap_or(0.0))
        .collect();
    let node_potentials = (0..artifact.mir.nodes.len())
        .map(|index| 0.35 - 0.11 * index as f64)
        .collect();
    let flow_count = artifact
        .mir
        .branches
        .len()
        .max(artifact.mir.branch_unknowns.len());
    let branch_flows = (0..flow_count)
        .map(|index| 1.0e-4 * (index as f64 + 1.0))
        .collect();
    BiasPoint {
        parameters,
        node_potentials,
        branch_flows,
    }
}

fn cfg_inputs(bias: &BiasPoint) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameters: bias.parameters.clone(),
        parameter_given: vec![false; bias.parameters.len()],
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: bias.node_potentials.clone(),
        branch_flows: bias.branch_flows.clone(),
        branch_unknown_flows: bias.branch_flows.clone(),
        // Use a deterministic ambient environment.
        temperature: 300.15,
        thermal_voltage: 300.15 * 8.617_333_262e-5,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        staged: Vec::new(),
    }
}

// --- corpus survey ---------------------------------------------------------

/// The subset small enough to survey on every run.
///
/// Real foundry models — a bipolar, two HEMTs, an EKV MOSFET, and the CMC
/// resistor, junction, and diode — but not the ones that take minutes each to
/// compile. The exhaustive walk is the `#[ignore]`d test below.
const FAST_CORPUS: &[&str] = &[
    "vbic_1.3",
    "angelov_2.0",
    "epfl_hemt_3.0.0",
    "ekv26_2.6",
    "cmc/r3_cmc_release1.1.2_2023Jun16",
    "cmc/r2_cmc_v1.0.2",
    "cmc/diode_cmc_3.0_20250714",
];

/// How many of [`FAST_CORPUS`]'s modules must lower.
///
/// Measured, not aspirational: raise it when the number goes up, and never past
/// what a run has actually produced. 12 of 12 as of 2026-07-27, having started
/// at 5 — `ddx`, `__rspice_limited_exp`, and flow access on a branch with more
/// than one unknown were what the first run found missing.
const FAST_CORPUS_FLOOR: usize = 12;

#[test]
fn the_fast_model_subset_lowers_to_control_flow_graphs() {
    let root = model_root();
    let roots: Vec<PathBuf> = FAST_CORPUS
        .iter()
        .map(|relative| {
            let path = relative
                .split('/')
                .fold(root.clone(), |path, part| path.join(part));
            assert!(
                path.exists(),
                "fast-corpus entry missing: {}",
                path.display()
            );
            path
        })
        .collect();

    let census = survey(&root, &roots);
    census.report("fast subset");
    assert!(
        census.lowered >= FAST_CORPUS_FLOOR,
        "CFG lowering covered {} of the fast subset, below the recorded floor of \
         {FAST_CORPUS_FLOOR}",
        census.lowered
    );
}

#[test]
#[ignore = "walks the whole shipped model tree, which takes tens of minutes"]
fn the_shipped_model_corpus_lowers_to_control_flow_graphs() {
    let root = model_root();
    survey(&root, std::slice::from_ref(&root)).report("full corpus");
}

#[derive(Default)]
struct Census {
    lowered: usize,
    failed: usize,
    reasons: BTreeMap<String, usize>,
    failures: Vec<String>,
}

impl Census {
    fn report(&self, label: &str) {
        eprintln!(
            "CFG lowering {label}: {} lowered, {} not yet",
            self.lowered, self.failed
        );
        for (reason, count) in &self.reasons {
            eprintln!("  {count:>6}  {reason}");
        }
        if !self.failures.is_empty() {
            eprintln!("  models: {}", self.failures.join(", "));
        }
    }
}

fn survey(include_root: &Path, roots: &[PathBuf]) -> Census {
    let mut census = Census::default();
    for search_root in roots {
        let candidates =
            discover_veriloga_sources(search_root).expect("model tree must be discoverable");
        for candidate in &candidates {
            let mut options = CompilerOptions::default();
            options.include_paths.push(include_root.to_path_buf());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiler = VerilogACompiler::new(options);

            for module in &candidate.modules {
                let Ok(compiled) =
                    compiler.compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))
                else {
                    // Front-end failures are a separate concern; this measures
                    // the CFG level, not the parser.
                    continue;
                };
                let artifact = compiled.artifact;
                match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
                    Ok(model) => {
                        census.lowered += 1;
                        assert_eq!(
                            model.residuals.len(),
                            artifact.mir.equations.len(),
                            "{module}: every equation needs a residual"
                        );
                    }
                    Err(diagnostics) => {
                        census.failed += 1;
                        for diagnostic in &diagnostics {
                            *census
                                .reasons
                                .entry(summarize(&diagnostic.message))
                                .or_default() += 1;
                        }
                        census
                            .failures
                            .push(format!("{module} ({} diagnostics)", diagnostics.len()));
                    }
                }
            }
        }
    }
    census
}

/// Collapse a diagnostic to the construct it names, so the census counts kinds
/// rather than occurrences of the same kind at different spans.
fn summarize(message: &str) -> String {
    let trimmed = message
        .strip_prefix("CFG lowering does not support ")
        .unwrap_or(message);
    match trimmed.split_once('\'') {
        Some((prefix, rest)) => match rest.split_once('\'') {
            Some((name, _)) => format!("{prefix}'{name}'"),
            None => trimmed.to_string(),
        },
        None => trimmed.to_string(),
    }
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}
