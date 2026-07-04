//! Rust backend selection frontier for shipped Verilog-A models.
//!
//! Run explicitly with:
//! `cargo test -p rspice-veriloga --test rust_backend_frontier shipped_rust_backend_frontier -- --ignored --nocapture`
//!
//! To focus on one package, file, or module:
//! `RSPICE_RUST_BACKEND_FRONTIER_FILTER=asmhemt cargo test -p rspice-veriloga --test rust_backend_frontier shipped_rust_backend_frontier -- --ignored --nocapture`
//!
//! To print the scalar error for every non-scalar selection:
//! `RSPICE_RUST_BACKEND_FRONTIER_TRACE_NON_SCALAR=1 ...`
//!
//! To make the frontier a pure-scalar production gate:
//! `RSPICE_RUST_BACKEND_FRONTIER_REQUIRE_NO_HYBRID=1 RSPICE_RUST_BACKEND_FRONTIER_REQUIRE_NO_LEGACY=1 ...`

use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, ExprId, HirAnalogOperator, HirAssignment, HirExprKind, HirStatement,
    InvalidationClass, OptOp,
};
use rspice_veriloga::rust_backend::{
    RustBackendSelection, RustTranspileOptions, RustTranspiler, discover_veriloga_sources,
};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::BTreeSet;
use std::env;
use std::io::Write as _;
use std::path::Path;
use std::time::Instant;

const FILTER_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_FILTER";
const REQUIRE_NO_LEGACY_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_REQUIRE_NO_LEGACY";
const REQUIRE_NO_HYBRID_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_REQUIRE_NO_HYBRID";
const TRACE_NON_SCALAR_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_TRACE_NON_SCALAR";

#[test]
#[ignore = "full shipped Rust-backend frontier audit; run explicitly while scalar coverage is still moving"]
fn shipped_rust_backend_frontier() {
    std::thread::Builder::new()
        .name("rspice-rust-backend-frontier".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(run_shipped_rust_backend_frontier)
        .expect("spawn Rust backend frontier worker")
        .join()
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
}

fn run_shipped_rust_backend_frontier() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    let mut sources =
        discover_veriloga_sources(&root).expect("discover shipped Verilog-A model sources");

    if let Ok(filter) = env::var(FILTER_ENV)
        && !filter.trim().is_empty()
    {
        sources.retain(|source| source_matches_filter(source, &root, &filter));
        assert!(
            !sources.is_empty(),
            "{FILTER_ENV}={filter:?} did not match any shipped Verilog-A source or module"
        );
    }

    let mut options = CompilerOptions::default();
    options.include_paths.push(root.clone());
    let compiler = VerilogACompiler::new(options);
    let transpiler = RustTranspiler::new_auto(RustTranspileOptions::default());
    let mut counts = BackendSelectionCounts::default();
    let mut failures = Vec::new();
    let total_modules: usize = sources.iter().map(|source| source.modules.len()).sum();
    let mut module_index = 0usize;

    for source in sources {
        for module in &source.modules {
            module_index += 1;
            let relative_path = source
                .path
                .strip_prefix(&root)
                .unwrap_or(&source.path)
                .display();
            eprintln!(
                "frontier compiling {module_index}/{total_modules}: {relative_path} :: {module}"
            );
            let _ = std::io::stderr().flush();
            let started = Instant::now();
            let compiled = match compiler
                .compile_file_canonical_ir_with_metadata(&source.path, Some(module))
            {
                Ok(compiled) => compiled,
                Err(error) => {
                    failures.push(format!(
                        "{} :: {} failed to compile canonical IR: {error}",
                        source
                            .path
                            .strip_prefix(&root)
                            .unwrap_or(&source.path)
                            .display(),
                        module
                    ));
                    continue;
                }
            };
            let compile_elapsed = started.elapsed();

            let transpile_started = Instant::now();
            match transpiler.transpile_with_report(&compiled.artifact) {
                Ok(report) => {
                    let transpile_elapsed = transpile_started.elapsed();
                    counts.record(report.backend);
                    if should_trace_scalar_gap(report.backend) {
                        eprintln!(
                            "scalar diagnostic for {:?} :: {} :: {}",
                            report.backend, relative_path, module
                        );
                        trace_scalar_transpile_error(&compiled.artifact, &relative_path, module);
                        trace_scalar_gap(&compiled.artifact);
                    }
                    eprintln!(
                        "{:?} :: {} :: {} (compile {:.2?}, transpile {:.2?}, total {:.2?})",
                        report.backend,
                        relative_path,
                        module,
                        compile_elapsed,
                        transpile_elapsed,
                        started.elapsed()
                    );
                }
                Err(error) => failures.push(format!(
                    "{} :: {} failed to transpile Rust backend: {error}",
                    relative_path, module
                )),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "frontier failures:\n{}",
        failures.join("\n")
    );
    eprintln!(
        "backend frontier: scalar={}, scalar-hybrid={}, legacy-device={}",
        counts.scalar, counts.hybrid, counts.legacy_device
    );

    if env::var_os(REQUIRE_NO_LEGACY_ENV).is_some() {
        assert_eq!(
            counts.legacy_device, 0,
            "legacy backend selections remain in the shipped frontier"
        );
    }
    if env::var_os(REQUIRE_NO_HYBRID_ENV).is_some() {
        assert_eq!(
            counts.hybrid, 0,
            "scalar-hybrid backend selections remain in the shipped frontier; rerun with {TRACE_NON_SCALAR_ENV}=1 to print scalar lowering gaps"
        );
    }
}

fn should_trace_scalar_gap(backend: RustBackendSelection) -> bool {
    env::var_os(TRACE_NON_SCALAR_ENV).is_some() && backend != RustBackendSelection::ScalarOptIr
}

fn trace_scalar_transpile_error(
    artifact: &CanonicalIrArtifact,
    relative_path: &dyn std::fmt::Display,
    module: &str,
) {
    match RustTranspiler::new_scalar(RustTranspileOptions::default()).transpile(artifact) {
        Ok(_) => eprintln!(
            "  scalar diagnostic unexpectedly succeeded for {} :: {}",
            relative_path, module
        ),
        Err(error) => eprintln!("  scalar diagnostic error: {error}"),
    }
}

fn trace_scalar_gap(artifact: &CanonicalIrArtifact) {
    let roots = available_scalar_equation_roots(artifact);
    let mut traced_missing_detail = false;
    for equation in &artifact.mir.equations {
        let expression = artifact
            .mir
            .expressions
            .get(usize::from(equation.expression.id))
            .map(|expression| &expression.kind);
        let opt_root = roots.get(&equation.id).copied();
        eprintln!(
            "  equation {:?}: kind={:?}, branch={}, expr={:?}, opt_root={:?}",
            equation.id, equation.kind, equation.branch.label, expression, opt_root
        );
        if opt_root.is_none() && !traced_missing_detail {
            traced_missing_detail = true;
            trace_expression_tree(artifact, equation.expression.id, 2, &mut Vec::new());
            trace_missing_identifier_assignments(artifact, equation.expression.id);
        }
    }
}

fn trace_missing_identifier_assignments(artifact: &CanonicalIrArtifact, expr: ExprId) {
    let mut identifiers = BTreeSet::new();
    collect_expression_identifiers(artifact, expr, &mut identifiers, &mut Vec::new());
    for identifier in identifiers {
        trace_identifier_assignment_chain(artifact, &identifier, 2, &mut Vec::new(), 5);
    }
}

fn collect_expression_identifiers(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    identifiers: &mut BTreeSet<String>,
    stack: &mut Vec<ExprId>,
) {
    if stack.contains(&expr) {
        return;
    }
    let Some(expression) = artifact.mir.expressions.get(usize::from(expr)) else {
        return;
    };
    if let HirExprKind::Identifier { name } = &expression.kind {
        identifiers.insert(name.to_string());
    }
    stack.push(expr);
    for child in expression_children(&expression.kind) {
        collect_expression_identifiers(artifact, child, identifiers, stack);
    }
    stack.pop();
}

fn trace_identifier_assignment_chain(
    artifact: &CanonicalIrArtifact,
    name: &str,
    indent: usize,
    stack: &mut Vec<String>,
    depth: usize,
) {
    if depth == 0 || stack.iter().any(|seen| seen == name) {
        return;
    }
    let Some(variable) = artifact
        .hir
        .variables
        .iter()
        .find(|variable| variable.name.as_str() == name)
    else {
        return;
    };
    let mut assignments = Vec::new();
    collect_variable_assignments(&artifact.hir.statements, variable.id, &mut assignments);
    if assignments.last().is_none() {
        eprintln!(
            "{:indent$}identifier {name}: no assignment history",
            "",
            indent = indent
        );
        return;
    }
    let mut next_identifiers = BTreeSet::new();
    for (offset, assignment) in assignments.iter().rev().take(4).enumerate() {
        let expression = artifact
            .mir
            .expressions
            .get(usize::from(assignment.expr.id));
        eprintln!(
            "{:indent$}identifier {name}: assignment -{offset} expr {:?}: {:?}",
            "",
            assignment.expr.id,
            expression.map(|expression| &expression.kind),
            indent = indent
        );
        trace_expression_tree(artifact, assignment.expr.id, indent + 2, &mut Vec::new());
        collect_expression_identifiers(
            artifact,
            assignment.expr.id,
            &mut next_identifiers,
            &mut Vec::new(),
        );
    }

    stack.push(name.to_string());
    for identifier in next_identifiers {
        trace_identifier_assignment_chain(artifact, &identifier, indent + 2, stack, depth - 1);
    }
    stack.pop();
}

fn collect_variable_assignments<'a>(
    statements: &'a [HirStatement],
    variable: rspice_veriloga::canonical_ir::VariableId,
    assignments: &mut Vec<&'a HirAssignment>,
) {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) if assignment.target == variable => {
                assignments.push(assignment);
            }
            HirStatement::Assignment(_) => {}
            HirStatement::Loop(loop_statement) => {
                collect_variable_assignments(&loop_statement.body, variable, assignments);
            }
        }
    }
}

fn available_scalar_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> std::collections::HashMap<
    rspice_veriloga::canonical_ir::EquationId,
    rspice_veriloga::canonical_ir::ValueId,
> {
    let mut roots = std::collections::HashMap::new();
    for schedule in &artifact.opt.schedules {
        if schedule.invalidation != InvalidationClass::NewtonIteration {
            continue;
        }

        let mut pending_value = None;
        for op in &schedule.ops {
            match *op {
                OptOp::ComputeValue { value } => pending_value = Some(value),
                OptOp::EvaluateEquation { equation } => {
                    if let Some(value) = pending_value.take() {
                        roots.insert(equation, value);
                    }
                }
            }
        }
    }
    roots
}

fn trace_expression_tree(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    indent: usize,
    stack: &mut Vec<ExprId>,
) {
    if stack.contains(&expr) {
        eprintln!("{:indent$}expr {:?}: <cycle>", "", expr, indent = indent);
        return;
    }
    let Some(expression) = artifact.mir.expressions.get(usize::from(expr)) else {
        eprintln!("{:indent$}expr {:?}: <missing>", "", expr, indent = indent);
        return;
    };
    eprintln!(
        "{:indent$}expr {:?}: {:?}",
        "",
        expr,
        expression.kind,
        indent = indent
    );

    stack.push(expr);
    for child in expression_children(&expression.kind) {
        trace_expression_tree(artifact, child, indent + 2, stack);
    }
    stack.pop();
}

fn expression_children(kind: &HirExprKind) -> Vec<ExprId> {
    match kind {
        HirExprKind::Binary { left, right, .. } => vec![*left, *right],
        HirExprKind::Unary { operand, .. } => vec![*operand],
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => vec![*condition, *then_expr, *else_expr],
        HirExprKind::Call { args, .. }
        | HirExprKind::SystemFunction { args, .. }
        | HirExprKind::ArrayLiteral { elements: args } => args.clone(),
        HirExprKind::ArrayAccess { index, .. } => vec![*index],
        HirExprKind::AnalogOperator { op } => analog_operator_children(op),
        HirExprKind::Laplace { expr, .. } | HirExprKind::Zi { expr, .. } => vec![*expr],
        HirExprKind::NoiseSource { operands, .. } => operands.clone(),
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => Vec::new(),
    }
}

fn analog_operator_children(op: &HirAnalogOperator) -> Vec<ExprId> {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            let mut children = vec![*expr];
            children.extend(*abstol);
            children
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            let mut children = vec![*expr];
            children.extend(*ic);
            children.extend(*assert);
            children.extend(*abstol);
            children
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            let mut children = vec![*expr];
            children.extend(*ic);
            children.extend(*modulus);
            children.extend(*offset);
            children.extend(*abstol);
            children
        }
        HirAnalogOperator::Ddx { expr, probe } => vec![*expr, *probe],
        HirAnalogOperator::Limexp { expr } => vec![*expr],
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            let mut children = vec![*expr, *delay];
            children.extend(*max_delay);
            children
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            let mut children = vec![*expr];
            children.extend(*delay);
            children.extend(*rise);
            children.extend(*fall);
            children.extend(*tolerance);
            children
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            let mut children = vec![*expr];
            children.extend(*max_rise);
            children.extend(*max_fall);
            children
        }
        HirAnalogOperator::LastCrossing { expr, .. } => vec![*expr],
    }
}

fn source_matches_filter(
    source: &rspice_veriloga::rust_backend::VerilogASourceCandidate,
    root: &Path,
    filter: &str,
) -> bool {
    let filter = filter.to_ascii_lowercase();
    let path = source
        .path
        .strip_prefix(root)
        .unwrap_or(&source.path)
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();

    path.contains(&filter)
        || source
            .modules
            .iter()
            .any(|module| module.to_ascii_lowercase().contains(&filter))
}

#[derive(Debug, Default)]
struct BackendSelectionCounts {
    scalar: usize,
    hybrid: usize,
    legacy_device: usize,
}

impl BackendSelectionCounts {
    fn record(&mut self, selection: RustBackendSelection) {
        match selection {
            RustBackendSelection::ScalarOptIr => self.scalar += 1,
            RustBackendSelection::ScalarHybrid => self.hybrid += 1,
            RustBackendSelection::LegacyDevice => self.legacy_device += 1,
        }
    }
}
