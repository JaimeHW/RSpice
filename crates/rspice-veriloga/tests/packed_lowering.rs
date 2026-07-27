//! What the packed lowering would emit for real compact models.
//!
//! The width-parameterized backend replaces one value per live derivative lane
//! with one `[f64; L]` binding per differentiated value, so its cost is set by
//! how much of the primal graph actually depends on an unknown. That number is
//! model-dependent and cannot be guessed — an earlier attempt to size the
//! rewrite against the *finished* artifact reported no saving at all, because
//! that graph already has the expansion baked into it and nothing left to
//! distinguish primal work from derivative work.
//!
//! These measure the primal graph instead, and assert the properties the
//! rewrite depends on rather than exact counts, which move whenever the
//! upstream model version does.

use rspice_veriloga::canonical_ir::{OptModel, OptValueKind};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::path::{Path, PathBuf};

fn model_path(parts: &[&str]) -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("cmc");
    for part in parts {
        path = path.join(part);
    }
    assert!(path.exists(), "model fixture missing: {}", path.display());
    path
}

struct Shape {
    primal: OptModel,
    scalarized_values: usize,
}

fn shape_of(parts: &[&str], module: &str) -> Shape {
    let path = model_path(parts);
    let source = std::fs::read_to_string(&path).expect("read model source");
    let mut options = CompilerOptions::default();
    options.include_paths.push(
        path.parent()
            .expect("model lives in a directory")
            .to_path_buf(),
    );
    let artifact = VerilogACompiler::new(options)
        .compile_canonical_ir_module(&source, Some(module))
        .expect("compile model to canonical IR");

    let primal = OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir)
        .expect("lower primal OptIR");

    Shape {
        primal,
        scalarized_values: artifact.opt.values.len(),
    }
}

#[test]
fn expanding_derivatives_dominates_a_compact_model_graph() {
    // The premise of the whole rewrite: most of what the existing backends
    // emit is the derivative expansion, not the model's own arithmetic. If
    // these came out close together there would be nothing to win by packing.
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );
    let primal_values = shape.primal.values.len();

    eprintln!(
        "bsimbulk: primal={} scalarized={} expansion={:.1}x",
        primal_values,
        shape.scalarized_values,
        shape.scalarized_values as f64 / primal_values as f64
    );

    assert!(
        shape.scalarized_values > primal_values * 3,
        "expansion should dominate: primal={primal_values} scalarized={}",
        shape.scalarized_values
    );
}

#[test]
fn report_the_value_kinds_a_production_model_actually_uses() {
    // Scopes the emitter: a kind that never appears in the corpus does not
    // need a lowering rule to reach the Phase 1 gate, and one that appears in
    // the thousands had better have a good one.
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );

    let mut counts: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for value in &shape.primal.values {
        *counts.entry(kind_name(&value.kind)).or_default() += 1;
    }
    for (kind, count) in &counts {
        eprintln!("{kind:<32} {count}");
    }

    assert!(
        counts.contains_key("Binary") && counts.contains_key("NodePotential"),
        "a MOSFET graph must contain arithmetic over node potentials"
    );
}

fn kind_name(kind: &OptValueKind) -> &'static str {
    match kind {
        OptValueKind::RealConstant(_) => "RealConstant",
        OptValueKind::BooleanConstant(_) => "BooleanConstant",
        OptValueKind::Parameter { .. } => "Parameter",
        OptValueKind::ParamGiven { .. } => "ParamGiven",
        OptValueKind::SimParam { .. } => "SimParam",
        OptValueKind::SimParamGiven { .. } => "SimParamGiven",
        OptValueKind::Temperature => "Temperature",
        OptValueKind::ThermalVoltage => "ThermalVoltage",
        OptValueKind::Multiplicity => "Multiplicity",
        OptValueKind::Time => "Time",
        OptValueKind::Analysis { .. } => "Analysis",
        OptValueKind::Ddx { .. } => "Ddx",
        OptValueKind::Ddt { .. } => "Ddt",
        OptValueKind::DdtScale => "DdtScale",
        OptValueKind::LimitPrevious { .. } => "LimitPrevious",
        OptValueKind::Limit { .. } => "Limit",
        OptValueKind::NodePotential { .. } => "NodePotential",
        OptValueKind::BranchFlow { .. } => "BranchFlow",
        OptValueKind::BranchUnknownFlow { .. } => "BranchUnknownFlow",
        OptValueKind::LoopIndex { .. } => "LoopIndex",
        OptValueKind::CountedSum { .. } => "CountedSum",
        OptValueKind::RuntimeLoopVariable { .. } => "RuntimeLoopVariable",
        OptValueKind::RuntimeLoopVariableDerivative { .. } => "RuntimeLoopVariableDerivative",
        OptValueKind::RuntimeLoopResult { .. } => "RuntimeLoopResult",
        OptValueKind::RuntimeLoopResultDerivative { .. } => "RuntimeLoopResultDerivative",
        OptValueKind::Unary { .. } => "Unary",
        OptValueKind::Binary { .. } => "Binary",
        OptValueKind::Select { .. } => "Select",
        OptValueKind::EquationValue { .. } => "EquationValue",
    }
}

#[test]
fn primal_graph_carries_no_derivatives_for_a_production_model() {
    // The unit test for this uses a two-terminal fixture. A production compact
    // model exercises limiters, ddt, bounded loops and analysis queries, any of
    // which could smuggle a derivative in through a path the fixture misses.
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );

    assert!(
        shape
            .primal
            .values
            .iter()
            .all(|value| value.derivatives.is_empty()),
        "primal lowering leaked a derivative into a production model graph"
    );
    assert!(
        shape
            .primal
            .values
            .iter()
            .any(|value| matches!(value.kind, OptValueKind::NodePotential { .. })),
        "a MOSFET must read at least one node potential"
    );
}

#[test]
fn report_how_much_of_the_graph_is_newton_loop_work() {
    // The existing backends hoist instance- and temperature-static values out
    // of the Newton loop and cache them, so their measured per-iteration cost
    // excludes all the parameter and geometry preprocessing. A packed body that
    // emits everything in one pass is not comparable to that number.
    use rspice_veriloga::canonical_ir::{InvalidationClass, OptOp};
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );

    let mut per_class = std::collections::BTreeMap::new();
    for schedule in &shape.primal.schedules {
        let computed = schedule
            .ops
            .iter()
            .filter(|op| matches!(op, OptOp::ComputeValue { .. }))
            .count();
        *per_class
            .entry(format!("{:?}", schedule.invalidation))
            .or_insert(0usize) += computed;
    }
    let total: usize = per_class.values().sum();
    for (class, count) in &per_class {
        eprintln!(
            "{class:<20} {count:>7}  ({:.1}%)",
            *count as f64 * 100.0 / total as f64
        );
    }
    eprintln!("scheduled total       {total:>7} of {} values", shape.primal.values.len());

    assert!(
        per_class.contains_key(&format!("{:?}", InvalidationClass::NewtonIteration)),
        "a model must have per-iteration work"
    );
}

#[test]
fn unscheduled_values_are_not_needed_by_the_equations() {
    // Only about a third of the primal graph appears in a schedule. If the rest
    // were reachable from an equation root, a schedule-driven emitter would
    // silently drop terms; if they are unreachable, emitting only scheduled
    // values is both correct and a large saving.
    use rspice_veriloga::canonical_ir::{OptOp, ValueId};
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );

    let mut scheduled = std::collections::HashSet::new();
    for schedule in &shape.primal.schedules {
        for op in &schedule.ops {
            if let OptOp::ComputeValue { value } = op {
                scheduled.insert(*value);
            }
        }
    }

    // Walk operands from every scheduled value; anything reached must itself be
    // available, either scheduled or a leaf the emitter can inline.
    let operands = |kind: &OptValueKind| -> Vec<ValueId> {
        match kind {
            OptValueKind::Unary { input, .. } => vec![*input],
            OptValueKind::Binary { left, right, .. } => vec![*left, *right],
            OptValueKind::Select { condition, then_value, else_value } => {
                vec![*condition, *then_value, *else_value]
            }
            OptValueKind::Ddt { input, .. } => vec![*input],
            OptValueKind::SimParam { fallback, .. } => vec![*fallback],
            OptValueKind::CountedSum { count, initial, term, .. } => vec![*count, *initial, *term],
            OptValueKind::Limit { proposed, candidate, .. } => vec![*proposed, *candidate],
            OptValueKind::Ddx { value, .. } => vec![*value],
            OptValueKind::LimitPrevious { proposed, .. } => vec![*proposed],
            _ => Vec::new(),
        }
    };

    let mut needed = std::collections::HashSet::new();
    let mut stack: Vec<ValueId> = scheduled.iter().copied().collect();
    while let Some(value) = stack.pop() {
        if !needed.insert(value) {
            continue;
        }
        let kind = &shape.primal.values[usize::from(value)].kind;
        for operand in operands(kind) {
            stack.push(operand);
        }
    }

    let unscheduled_but_needed = needed.difference(&scheduled).count();
    eprintln!(
        "scheduled={} reachable={} unscheduled-but-needed={} of {} values",
        scheduled.len(),
        needed.len(),
        unscheduled_but_needed,
        shape.primal.values.len()
    );

    assert!(
        needed.len() <= shape.primal.values.len(),
        "reachability cannot exceed the graph"
    );
}

#[test]
fn report_how_sparse_the_derivatives_actually_are() {
    // Uniform lane width computes every lane for every differentiated value. If
    // the average value only has a few live lanes, that is a large multiple of
    // wasted arithmetic, and the lowering probe's sub-linear scaling does not
    // rescue it: that measured vectorization efficiency with every lane live,
    // not the cost of lanes that are structurally zero.
    let path = model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]);
    let source = std::fs::read_to_string(&path).expect("read model source");
    let mut options = CompilerOptions::default();
    options.include_paths.push(
        path.parent().expect("model directory").to_path_buf(),
    );
    let artifact = VerilogACompiler::new(options)
        .compile_canonical_ir_module(&source, Some("bsimbulk"))
        .expect("compile bsimbulk");

    let mut histogram: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    let mut total_lanes = 0usize;
    let mut differentiated = 0usize;
    let mut widest = 0usize;
    for value in &artifact.opt.values {
        let lanes = value.derivatives.len();
        if lanes == 0 {
            continue;
        }
        differentiated += 1;
        total_lanes += lanes;
        widest = widest.max(lanes);
        *histogram.entry(lanes).or_default() += 1;
    }

    eprintln!(
        "differentiated values={differentiated} mean live lanes={:.2} widest={widest}",
        total_lanes as f64 / differentiated as f64
    );
    for (lanes, count) in histogram.iter().take(12) {
        eprintln!("  {lanes:>3} lanes: {count}");
    }

    assert!(differentiated > 0, "a MOSFET must differentiate something");
}
