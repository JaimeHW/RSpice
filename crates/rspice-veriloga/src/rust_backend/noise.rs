use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalNoiseSourceKind, ExprId, HirAnalogOperator, HirAssignment,
    HirExprKind, HirLoop, HirStatement, NodeId,
};

use super::expr::{
    BranchCurrentSlot, LoweredVariable, branch_pair_key, lower_noise_value_expr,
    parameter_field_names,
};
use super::{GeneratedRustFile, RustBackendError, RustTranspileOptions};

type StatementKey = (u32, u32, u32, u32);
type LoopKey = (u32, u32, u32);

#[derive(Default)]
struct NoiseScheduleSelection {
    assignments: HashSet<StatementKey>,
    loops: HashSet<LoopKey>,
}

#[derive(Default)]
struct MergedNoiseSchedule {
    assignments: HashMap<StatementKey, Vec<usize>>,
    loops: HashMap<LoopKey, Vec<usize>>,
}

struct NoiseLivenessIndex {
    variable_ids: HashMap<String, u32>,
    expr_dependencies: HashMap<ExprId, HashSet<u32>>,
    loop_assignments: HashMap<LoopKey, HashSet<u32>>,
    expression_walks: usize,
}

impl NoiseLivenessIndex {
    fn new(artifact: &CanonicalIrArtifact) -> Self {
        Self {
            variable_ids: artifact
                .hir
                .variables
                .iter()
                .map(|variable| (variable.name.to_string(), variable.id.index()))
                .collect(),
            expr_dependencies: HashMap::new(),
            loop_assignments: HashMap::new(),
            expression_walks: 0,
        }
    }

    fn extend_expr_dependencies(
        &mut self,
        artifact: &CanonicalIrArtifact,
        root: ExprId,
        dependencies: &mut HashSet<u32>,
    ) {
        if let Some(cached) = self.expr_dependencies.get(&root) {
            dependencies.extend(cached.iter().copied());
            return;
        }
        self.expression_walks += 1;
        let mut found = HashSet::new();
        collect_expr_variable_ids(artifact, root, &self.variable_ids, &mut found);
        dependencies.extend(found.iter().copied());
        self.expr_dependencies.insert(root, found);
    }

    fn assigned_variables(&mut self, loop_statement: &HirLoop) -> HashSet<u32> {
        let key = loop_key(loop_statement);
        if let Some(cached) = self.loop_assignments.get(&key) {
            return cached.clone();
        }
        let mut assigned = HashSet::new();
        collect_assigned_variable_ids(&loop_statement.body, &mut assigned);
        self.loop_assignments.insert(key, assigned.clone());
        assigned
    }
}

impl MergedNoiseSchedule {
    fn insert(&mut self, source_index: usize, selection: &NoiseScheduleSelection) {
        for &key in &selection.assignments {
            self.assignments.entry(key).or_default().push(source_index);
        }
        for &key in &selection.loops {
            self.loops.entry(key).or_default().push(source_index);
        }
    }
}

pub(super) fn generate_noise_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustFile, RustBackendError> {
    let parameter_fields = parameter_field_names(artifact);
    let variables = noise_variables(artifact);
    let branch_unknowns = noise_branch_unknowns(artifact);
    let guarded_replay_safety = guarded_assignment_replay_safety(artifact);
    let mut liveness = NoiseLivenessIndex::new(artifact);
    let mut out = String::new();
    out.push_str("#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    writeln!(
        out,
        "use {}::GeneratedEvalContext;\npub use {}::{{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind}};\n",
        options.runtime_path, options.runtime_path
    )
    .expect("write generated noise imports");
    out.push_str("const LIMEXP_MAX: f64 = 5.54062238439351e34;\n");
    out.push_str("const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;\n\n");

    writeln!(
        out,
        "pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; {}] = [",
        artifact.noise_sources.sources.len()
    )
    .expect("write noise descriptor header");
    for source in &artifact.noise_sources.sources {
        let table = source.table.as_ref();
        writeln!(
            out,
            "    GeneratedNoiseDescriptor {{ mechanism: {:?}, label: {}, kind: GeneratedNoiseKind::{}, equation: {}, is_current: {}, branch_ordinal: {}, pos: {}, neg: {}, table_len: {}, table_log_interp: {} }},",
            source.mechanism.as_str(),
            option_str(source.label.as_deref()),
            noise_kind(source.kind),
            source.equation.index(),
            source.is_current,
            option_usize(source.branch_ordinal.map(|ordinal| ordinal.index())),
            endpoint_literal(source.pos.node, &source.pos.name, source.pos.is_internal),
            endpoint_literal(source.neg.node, &source.neg.name, source.neg.is_internal),
            table.map_or(0, |table| table.operands.len()),
            table.is_some_and(|table| table.log_interp),
        )
        .expect("write noise descriptor");
    }
    out.push_str("];\n\nimpl Instance {\n");
    if artifact.noise_sources.sources.is_empty() {
        out.push_str(
            "    pub fn evaluate_noise_source(&self, source_index: usize, _ctx: &GeneratedEvalContext<'_>) -> Result<GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError> {\n\
             \x20       Err(GeneratedNoiseEvaluationError::SourceIndexOutOfRange { index: source_index, count: 0 })\n\
             \x20   }\n\
             }\n",
        );
        return Ok(GeneratedRustFile {
            relative_path: "noise.rs".to_string(),
            contents: out,
        });
    }
    out.push_str(
        "    pub fn evaluate_noise_source(&self, source_index: usize, ctx: &GeneratedEvalContext<'_>) -> Result<GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError> {\n\
         \x20       if source_index >= NOISE_SOURCES.len() {\n\
         \x20           return Err(GeneratedNoiseEvaluationError::SourceIndexOutOfRange { index: source_index, count: NOISE_SOURCES.len() });\n\
         \x20       }\n\
         \x20       if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {\n\
         \x20           return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });\n\
         \x20       }\n\
         \x20       let params = &*self.params;\n",
    );
    let mut activation_schedule = MergedNoiseSchedule::default();
    let mut metadata_schedule = MergedNoiseSchedule::default();
    for (index, source) in artifact.noise_sources.sources.iter().enumerate() {
        let activation_roots: Vec<_> = source
            .activation
            .iter()
            .map(|activation| activation.id)
            .collect();
        activation_schedule.insert(
            index,
            &noise_schedule_selection(artifact, &mut liveness, &activation_roots),
        );
        let mut metadata_roots = vec![source.psd.id];
        metadata_roots.extend(source.exponent.iter().map(|exponent| exponent.id));
        if let Some(table) = &source.table {
            metadata_roots.extend(table.operands.iter().map(|operand| operand.id));
        }
        metadata_schedule.insert(
            index,
            &noise_schedule_selection(artifact, &mut liveness, &metadata_roots),
        );
    }
    debug_assert!(liveness.expression_walks <= artifact.hir.expressions.len());
    emit_noise_schedule(
        &mut out,
        artifact,
        &parameter_fields,
        &variables,
        &branch_unknowns,
        &activation_schedule,
        &guarded_replay_safety,
        true,
        8,
        "noise_activation_schedule",
    )?;
    out.push_str("        let noise_source_active = match source_index {\n");
    for (index, source) in artifact.noise_sources.sources.iter().enumerate() {
        writeln!(out, "            {index} => {{").expect("write noise activation arm");
        if let Some(activation) = &source.activation {
            let activation = lower_noise_value_expr(
                artifact,
                activation.id,
                &format!("noise_{index}_activation"),
                &parameter_fields,
                &variables,
                &branch_unknowns,
            )?;
            emit_lines(&mut out, &activation.lines, 16);
            writeln!(out, "                {} != 0.0", activation.value)
                .expect("write noise activation value");
        } else {
            out.push_str("                true\n");
        }
        out.push_str("            }\n");
    }
    out.push_str(
        "            _ => unreachable!(\"noise source index was range checked\"),\n        };\n",
    );
    out.push_str("        if !noise_source_active { return Ok(GeneratedNoiseEvaluation { active: false, psd: 0.0, exponent: None, table_operands: Vec::new() }); }\n");
    for (index, _) in artifact.hir.variables.iter().enumerate() {
        writeln!(out, "        noise_variable_{index} = 0.0;")
            .expect("reset noise metadata variable");
    }
    emit_noise_schedule(
        &mut out,
        artifact,
        &parameter_fields,
        &variables,
        &branch_unknowns,
        &metadata_schedule,
        &guarded_replay_safety,
        false,
        8,
        "noise_metadata_schedule",
    )?;
    out.push_str("        match source_index {\n");
    for (index, source) in artifact.noise_sources.sources.iter().enumerate() {
        writeln!(out, "            {index} => {{").expect("write noise evaluator arm");
        let psd = lower_noise_value_expr(
            artifact,
            source.psd.id,
            &format!("noise_{index}_psd"),
            &parameter_fields,
            &variables,
            &branch_unknowns,
        )?;
        emit_lines(&mut out, &psd.lines, 16);
        let exponent = if let Some(exponent) = &source.exponent {
            let exponent = lower_noise_value_expr(
                artifact,
                exponent.id,
                &format!("noise_{index}_exponent"),
                &parameter_fields,
                &variables,
                &branch_unknowns,
            )?;
            emit_lines(&mut out, &exponent.lines, 16);
            format!("Some({})", exponent.value)
        } else {
            "None".to_string()
        };
        writeln!(out, "                let psd = {};", psd.value)
            .expect("write noise PSD evaluation");
        emit_finite_check(&mut out, index, "psd", "psd", 16);
        writeln!(
            out,
            "                if psd < 0.0 {{ return Err(GeneratedNoiseEvaluationError::NegativePower {{ index: {index}, value: psd }}); }}"
        )
        .expect("write nonnegative PSD validation");
        writeln!(
            out,
            "                let exponent: Option<f64> = {exponent};"
        )
        .expect("write noise exponent evaluation");
        emit_optional_finite_check(&mut out, index, "exponent", "exponent", 16);
        let mut table_values = Vec::new();
        if let Some(table) = &source.table {
            for (operand_index, operand) in table.operands.iter().enumerate() {
                let value = lower_noise_value_expr(
                    artifact,
                    operand.id,
                    &format!("noise_{index}_table_{operand_index}"),
                    &parameter_fields,
                    &variables,
                    &branch_unknowns,
                )?;
                emit_lines(&mut out, &value.lines, 16);
                let local = format!("noise_table_operand_{operand_index}");
                writeln!(out, "                let {local} = {};", value.value)
                    .expect("write noise table operand");
                emit_finite_check(
                    &mut out,
                    index,
                    &format!("table operand {operand_index}"),
                    &local,
                    16,
                );
                table_values.push(local);
            }
        }
        writeln!(
            out,
            "                let table_operands = vec![{}];",
            table_values.join(", ")
        )
        .expect("write owned noise table operands");
        let scaled_psd = if source.is_current {
            "psd * self.multiplicity"
        } else {
            "psd / self.multiplicity"
        };
        writeln!(out, "                let psd = {scaled_psd};")
            .expect("write multiplicity scaling");
        emit_finite_check(&mut out, index, "scaled psd", "psd", 16);
        out.push_str(
            "                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })\n",
        );
        out.push_str("            }\n");
    }
    out.push_str("            _ => unreachable!(\"noise source index was range checked\"),\n        }\n    }\n}\n");

    Ok(GeneratedRustFile {
        relative_path: "noise.rs".to_string(),
        contents: out,
    })
}

#[allow(clippy::too_many_arguments)]
fn emit_noise_schedule(
    out: &mut String,
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    branch_unknowns: &HashMap<String, BranchCurrentSlot>,
    schedule: &MergedNoiseSchedule,
    guarded_replay_safety: &HashMap<StatementKey, bool>,
    declare_variables: bool,
    indentation: usize,
    prefix: &str,
) -> Result<(), RustBackendError> {
    let padding = " ".repeat(indentation);
    if declare_variables {
        for (index, _) in artifact.hir.variables.iter().enumerate() {
            writeln!(out, "{padding}let mut noise_variable_{index} = 0.0;")
                .expect("write noise variable");
        }
    }
    emit_noise_statements(
        out,
        artifact,
        &artifact.hir.statements,
        parameter_fields,
        variables,
        branch_unknowns,
        schedule,
        guarded_replay_safety,
        indentation,
        prefix,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_noise_statements(
    out: &mut String,
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    branch_unknowns: &HashMap<String, BranchCurrentSlot>,
    schedule: &MergedNoiseSchedule,
    guarded_replay_safety: &HashMap<StatementKey, bool>,
    indentation: usize,
    prefix: &str,
) -> Result<(), RustBackendError> {
    let padding = " ".repeat(indentation);
    for (index, statement) in statements.iter().enumerate() {
        let statement_prefix = format!("{prefix}_{index}");
        match statement {
            HirStatement::Assignment(assignment) => {
                let Some(sources) = schedule.assignments.get(&assignment_key(assignment)) else {
                    continue;
                };
                if assignment.index.is_some() {
                    return Err(unsupported(
                        artifact,
                        "array assignment in generated noise evaluation",
                    ));
                }
                if assignment.unfiltered_initial_step_guard.is_some()
                    && !guarded_replay_safety
                        .get(&assignment_key(assignment))
                        .copied()
                        .unwrap_or(false)
                {
                    return Err(unsupported(
                        artifact,
                        &format!(
                            "initial-step guarded noise dependency '{}' at {}..{} is not instance-static and requires persisted generated state",
                            assignment.target_name, assignment.span.start, assignment.span.end
                        ),
                    ));
                }
                // Canonical OptIR treats an unfiltered initial-step assignment
                // as the instance-static initializer for its target. Replaying
                // the selected initializer here reconstructs that persisted
                // value without spuriously leaving the local at zero.
                let lowered = lower_noise_value_expr(
                    artifact,
                    assignment.expr.id,
                    &statement_prefix,
                    parameter_fields,
                    variables,
                    branch_unknowns,
                )?;
                writeln!(
                    out,
                    "{padding}if matches!(source_index, {}) {{",
                    source_pattern(sources)
                )
                .expect("write noise assignment source guard");
                emit_lines(out, &lowered.lines, indentation + 4);
                writeln!(
                    out,
                    "{padding}    noise_variable_{} = {};",
                    assignment.target.index(),
                    lowered.value
                )
                .expect("write noise assignment");
                writeln!(out, "{padding}}}").expect("write noise assignment source guard end");
            }
            HirStatement::Loop(loop_statement) => {
                let Some(sources) = schedule.loops.get(&loop_key(loop_statement)) else {
                    continue;
                };
                writeln!(
                    out,
                    "{padding}if matches!(source_index, {}) {{",
                    source_pattern(sources)
                )
                .expect("write noise loop source guard");
                writeln!(
                    out,
                    "{padding}    let mut {statement_prefix}_iterations = 0usize;"
                )
                .expect("write noise loop counter");
                writeln!(out, "{padding}    loop {{").expect("write noise loop");
                let condition = lower_noise_value_expr(
                    artifact,
                    loop_statement.condition.id,
                    &format!("{statement_prefix}_condition"),
                    parameter_fields,
                    variables,
                    branch_unknowns,
                )?;
                emit_lines(out, &condition.lines, indentation + 8);
                writeln!(
                    out,
                    "{padding}        if {} == 0.0 {{ break; }}",
                    condition.value
                )
                .expect("write noise loop condition");
                emit_noise_statements(
                    out,
                    artifact,
                    &loop_statement.body,
                    parameter_fields,
                    variables,
                    branch_unknowns,
                    schedule,
                    guarded_replay_safety,
                    indentation + 8,
                    &format!("{statement_prefix}_body"),
                )?;
                writeln!(out, "{padding}        {statement_prefix}_iterations += 1;")
                    .expect("write noise loop increment");
                writeln!(
                    out,
                    "{padding}        assert!({statement_prefix}_iterations <= Self::MAX_ANALOG_LOOP_ITERATIONS, \"generated Verilog-A noise evaluation loop exceeded iteration limit\");"
                )
                .expect("write noise loop bound");
                writeln!(out, "{padding}    }}").expect("write noise loop end");
                writeln!(out, "{padding}}}").expect("write noise loop source guard end");
            }
        }
    }
    Ok(())
}

fn noise_schedule_selection(
    artifact: &CanonicalIrArtifact,
    liveness: &mut NoiseLivenessIndex,
    roots: &[ExprId],
) -> NoiseScheduleSelection {
    let mut live = HashSet::new();
    for &root in roots {
        liveness.extend_expr_dependencies(artifact, root, &mut live);
    }
    let mut selection = NoiseScheduleSelection::default();
    select_statements_backward(
        artifact,
        liveness,
        &artifact.hir.statements,
        &mut live,
        &mut selection,
    );
    selection
}

#[cfg(test)]
pub(super) fn noise_liveness_expression_walks(artifact: &CanonicalIrArtifact) -> usize {
    let mut liveness = NoiseLivenessIndex::new(artifact);
    for source in &artifact.noise_sources.sources {
        let activation_roots: Vec<_> = source
            .activation
            .iter()
            .map(|activation| activation.id)
            .collect();
        let _ = noise_schedule_selection(artifact, &mut liveness, &activation_roots);
        let mut metadata_roots = vec![source.psd.id];
        metadata_roots.extend(source.exponent.iter().map(|exponent| exponent.id));
        if let Some(table) = &source.table {
            metadata_roots.extend(table.operands.iter().map(|operand| operand.id));
        }
        let _ = noise_schedule_selection(artifact, &mut liveness, &metadata_roots);
    }
    liveness.expression_walks
}

fn select_statements_backward(
    artifact: &CanonicalIrArtifact,
    liveness: &mut NoiseLivenessIndex,
    statements: &[HirStatement],
    live: &mut HashSet<u32>,
    selection: &mut NoiseScheduleSelection,
) {
    for statement in statements.iter().rev() {
        match statement {
            HirStatement::Assignment(assignment) if live.contains(&assignment.target.index()) => {
                selection.assignments.insert(assignment_key(assignment));
                live.remove(&assignment.target.index());
                liveness.extend_expr_dependencies(artifact, assignment.expr.id, live);
                if let Some(index) = &assignment.index {
                    liveness.extend_expr_dependencies(artifact, index.id, live);
                }
            }
            HirStatement::Assignment(_) => {}
            HirStatement::Loop(loop_statement) => {
                let assigned = liveness.assigned_variables(loop_statement);
                if assigned.iter().any(|variable| live.contains(variable)) {
                    selection.loops.insert(loop_key(loop_statement));
                    let exit_live = live.clone();
                    let mut loop_live = exit_live.clone();
                    loop {
                        let mut body_live = loop_live.clone();
                        select_statements_backward(
                            artifact,
                            liveness,
                            &loop_statement.body,
                            &mut body_live,
                            selection,
                        );
                        liveness.extend_expr_dependencies(
                            artifact,
                            loop_statement.condition.id,
                            &mut body_live,
                        );
                        body_live.extend(exit_live.iter().cloned());
                        let before = loop_live.len();
                        loop_live.extend(body_live);
                        if loop_live.len() == before {
                            break;
                        }
                    }
                    *live = loop_live;
                }
            }
        }
    }
}

fn collect_assigned_variable_ids(statements: &[HirStatement], assigned: &mut HashSet<u32>) {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                assigned.insert(assignment.target.index());
            }
            HirStatement::Loop(loop_statement) => {
                collect_assigned_variable_ids(&loop_statement.body, assigned);
            }
        }
    }
}

fn assigned_variables(statements: &[HirStatement]) -> HashSet<String> {
    let mut assigned = HashSet::new();
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                assigned.insert(assignment.target_name.to_string());
            }
            HirStatement::Loop(loop_statement) => {
                assigned.extend(assigned_variables(&loop_statement.body));
            }
        }
    }
    assigned
}

fn assignment_key(assignment: &HirAssignment) -> StatementKey {
    (
        assignment.span.source_file_id,
        assignment.span.start,
        assignment.span.end,
        assignment.target.index(),
    )
}

fn loop_key(loop_statement: &HirLoop) -> LoopKey {
    (
        loop_statement.span.source_file_id,
        loop_statement.span.start,
        loop_statement.span.end,
    )
}

fn collect_expr_variable_ids(
    artifact: &CanonicalIrArtifact,
    root: ExprId,
    variable_ids: &HashMap<String, u32>,
    dependencies: &mut HashSet<u32>,
) {
    let mut stack = vec![root];
    let mut visited = HashSet::new();
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        match &artifact.hir.expressions[usize::from(expr)].kind {
            HirExprKind::Identifier { name } => {
                if let Some(&variable) = variable_ids.get(name.as_str()) {
                    dependencies.insert(variable);
                }
            }
            HirExprKind::Binary { left, right, .. } => stack.extend([*left, *right]),
            HirExprKind::Unary { operand, .. } => stack.push(*operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                stack.extend([*condition, *then_expr, *else_expr]);
            }
            HirExprKind::Call { args, .. }
            | HirExprKind::SystemFunction { args, .. }
            | HirExprKind::ArrayLiteral { elements: args } => stack.extend(args.iter().copied()),
            HirExprKind::ArrayAccess { index, .. } => stack.push(*index),
            HirExprKind::AnalogOperator { op } => push_analog_children(op, &mut stack),
            HirExprKind::Laplace { expr, .. } | HirExprKind::Zi { expr, .. } => stack.push(*expr),
            HirExprKind::NoiseSource { operands, .. } => stack.extend(operands.iter().copied()),
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
        }
    }
}

fn push_analog_children(op: &HirAnalogOperator, stack: &mut Vec<ExprId>) {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            stack.push(*expr);
            stack.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(ic.iter().chain(assert).chain(abstol).copied());
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(
                ic.iter()
                    .chain(modulus)
                    .chain(offset)
                    .chain(abstol)
                    .copied(),
            );
        }
        HirAnalogOperator::Ddx { expr, probe } => stack.extend([*expr, *probe]),
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            stack.push(*expr);
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            stack.extend([*expr, *delay]);
            stack.extend(max_delay.iter().copied());
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            stack.push(*expr);
            stack.extend(
                delay
                    .iter()
                    .chain(rise)
                    .chain(fall)
                    .chain(tolerance)
                    .copied(),
            );
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            stack.push(*expr);
            stack.extend(max_rise.iter().chain(max_fall).copied());
        }
    }
}

fn guarded_assignment_replay_safety(artifact: &CanonicalIrArtifact) -> HashMap<StatementKey, bool> {
    let variable_names: HashSet<_> = artifact
        .hir
        .variables
        .iter()
        .map(|variable| variable.name.to_string())
        .collect();
    let mut variable_safety = HashMap::new();
    let mut assignment_safety = HashMap::new();
    classify_statement_replay_safety(
        artifact,
        &artifact.hir.statements,
        &variable_names,
        &mut variable_safety,
        &mut assignment_safety,
    );
    assignment_safety
}

fn classify_statement_replay_safety(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    variable_names: &HashSet<String>,
    variable_safety: &mut HashMap<String, bool>,
    assignment_safety: &mut HashMap<StatementKey, bool>,
) {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                let safe = assignment.index.is_none()
                    && expr_is_instance_static(
                        artifact,
                        assignment.expr.id,
                        assignment.target_name.as_str(),
                        variable_names,
                        variable_safety,
                    );
                variable_safety.insert(assignment.target_name.to_string(), safe);
                assignment_safety.insert(assignment_key(assignment), safe);
            }
            HirStatement::Loop(loop_statement) => {
                classify_statement_replay_safety(
                    artifact,
                    &loop_statement.body,
                    variable_names,
                    variable_safety,
                    assignment_safety,
                );
                for variable in assigned_variables(&loop_statement.body) {
                    variable_safety.insert(variable, false);
                }
            }
        }
    }
}

fn expr_is_instance_static(
    artifact: &CanonicalIrArtifact,
    root: ExprId,
    current_target: &str,
    variable_names: &HashSet<String>,
    variable_safety: &HashMap<String, bool>,
) -> bool {
    let mut stack = vec![root];
    let mut visited = HashSet::new();
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        match &artifact.hir.expressions[usize::from(expr)].kind {
            HirExprKind::Identifier { name } => {
                if variable_names.contains(name.as_str())
                    && name.as_str() != current_target
                    && !variable_safety.get(name.as_str()).copied().unwrap_or(false)
                {
                    return false;
                }
            }
            HirExprKind::Binary { left, right, .. } => stack.extend([*left, *right]),
            HirExprKind::Unary { operand, .. } => stack.push(*operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => stack.extend([*condition, *then_expr, *else_expr]),
            HirExprKind::Call { args, .. } => stack.extend(args.iter().copied()),
            HirExprKind::SystemFunction { name, args } => {
                if matches!(
                    name.trim_start_matches('$'),
                    "abstime"
                        | "realtime"
                        | "initial_step"
                        | "final_step"
                        | "analysis"
                        | "frequency"
                ) {
                    return false;
                }
                stack.extend(args.iter().copied());
            }
            HirExprKind::ArrayLiteral { elements } => stack.extend(elements.iter().copied()),
            HirExprKind::ArrayAccess { .. }
            | HirExprKind::AnalogOperator { .. }
            | HirExprKind::Laplace { .. }
            | HirExprKind::Zi { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => return false,
            HirExprKind::NoiseSource { operands, .. } => stack.extend(operands.iter().copied()),
            HirExprKind::Number { .. } | HirExprKind::StringLiteral { .. } => {}
        }
    }
    true
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: &str) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

fn noise_variables(artifact: &CanonicalIrArtifact) -> HashMap<String, LoweredVariable> {
    let node_zeros = vec!["0.0".to_string(); artifact.mir.nodes.len()];
    let branch_zeros = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
    artifact
        .hir
        .variables
        .iter()
        .enumerate()
        .map(|(index, variable)| {
            (
                variable.name.to_string(),
                LoweredVariable {
                    value: format!("noise_variable_{index}"),
                    condition: None,
                    derivatives: node_zeros.clone(),
                    branch_derivatives: branch_zeros.clone(),
                    has_reactive: false,
                    reactive_value: "0.0".to_string(),
                    reactive_derivatives: node_zeros.clone(),
                    reactive_branch_derivatives: branch_zeros.clone(),
                },
            )
        })
        .collect()
}

fn noise_branch_unknowns(artifact: &CanonicalIrArtifact) -> HashMap<String, BranchCurrentSlot> {
    let mut slots = HashMap::new();
    for unknown in &artifact.mir.branch_unknowns {
        let slot = unknown.id.index() as usize;
        if let Some(name) = &unknown.declared_name {
            slots.insert(name.to_string(), BranchCurrentSlot::forward(slot));
        }
        let pos = unknown.pos_node.map(|node| node.index() as usize);
        let neg = unknown.neg_node.map(|node| node.index() as usize);
        slots.insert(branch_pair_key(pos, neg), BranchCurrentSlot::forward(slot));
        slots.insert(branch_pair_key(neg, pos), BranchCurrentSlot::reverse(slot));
    }
    slots
}

fn emit_lines(out: &mut String, lines: &[String], indentation: usize) {
    let padding = " ".repeat(indentation);
    for line in lines {
        writeln!(out, "{padding}{line}").expect("write lowered noise expression");
    }
}

fn source_pattern(sources: &[usize]) -> String {
    sources
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn emit_finite_check(
    out: &mut String,
    index: usize,
    quantity: &str,
    value: &str,
    indentation: usize,
) {
    let padding = " ".repeat(indentation);
    writeln!(
        out,
        "{padding}if !{value}.is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: {quantity:?}, value: {value} }}); }}"
    )
    .expect("write finite noise value validation");
}

fn emit_optional_finite_check(
    out: &mut String,
    index: usize,
    quantity: &str,
    value: &str,
    indentation: usize,
) {
    let padding = " ".repeat(indentation);
    writeln!(
        out,
        "{padding}if let Some(value) = {value} {{ if !value.is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: {quantity:?}, value }}); }} }}"
    )
    .expect("write finite optional noise value validation");
}

fn noise_kind(kind: CanonicalNoiseSourceKind) -> &'static str {
    match kind {
        CanonicalNoiseSourceKind::White => "White",
        CanonicalNoiseSourceKind::Flicker => "Flicker",
        CanonicalNoiseSourceKind::Table => "Table",
    }
}

fn endpoint_literal(node: Option<NodeId>, name: &str, is_internal: bool) -> String {
    format!(
        "GeneratedNoiseEndpoint {{ local_node: {}, name: {:?}, is_internal: {} }}",
        option_usize(node.map(NodeId::index)),
        name,
        is_internal
    )
}

fn option_str(value: Option<&str>) -> String {
    value.map_or_else(|| "None".to_string(), |value| format!("Some({value:?})"))
}

fn option_usize(value: Option<u32>) -> String {
    value.map_or_else(|| "None".to_string(), |value| format!("Some({value})"))
}
