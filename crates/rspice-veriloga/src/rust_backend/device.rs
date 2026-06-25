use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, EquationId, ExprId, HirAnalogOperator, HirExprKind,
    HirStatement, MirBranchRef, MirEquation, MirEquationKind, MirParameterSlot,
};

use super::expr::{
    DdtSlots, LoweredExpr, LoweredVariable, comparison_operator,
    is_intrinsic_name as expr_is_intrinsic_name, lower_assignment_expr_with_branch_currents,
    lower_equation_expr_with_branch_currents, lower_reactive_assignment_expr_with_branch_currents,
    lower_reactive_expr_with_branch_currents, lower_value_assignment_expr_with_branch_currents,
    parameter_field_names, unique_identifiers,
};
use super::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames, RustTranspileOptions,
};

const MAX_STAMP_HELPER_LINES: usize = 512;
const MAX_STAMP_HELPERS_PER_MODULE: usize = 4;

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    reject_unsupported_model_shape(artifact)?;

    let source_file_name = artifact.metadata.source_package.as_str();
    let names = RustDeviceNames::new(
        source_file_name,
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);
    let variable_fields = variable_local_names(artifact);
    let ddt_slots = collect_ddt_slots(artifact)?;
    let reactive_liveness = if ddt_slots.len() == 0 {
        ReactiveLiveness::default()
    } else {
        collect_reactive_liveness(artifact)?
    };
    let transient_liveness = collect_transient_liveness(artifact)?;
    let potential_branch_slots = collect_potential_branch_slots(artifact)?;

    let stamp_files = generate_stamp_file(
        artifact,
        options,
        &parameter_fields,
        &variable_fields,
        &ddt_slots,
        &transient_liveness,
        &reactive_liveness,
        &potential_branch_slots,
    )?;
    let mut files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: generate_state_file(
                artifact,
                &parameter_fields,
                ddt_slots.len(),
                potential_branch_slots.len(),
            )?,
        },
        GeneratedRustFile {
            relative_path: "stamp.rs".to_string(),
            contents: stamp_files.stamp,
        },
    ];
    files.extend(stamp_files.helpers);

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

#[derive(Debug, Clone, Default)]
struct PotentialBranchSlots {
    equation_slots: HashMap<EquationId, usize>,
    declared_slots: HashMap<String, usize>,
    branches: Vec<MirBranchRef>,
}

impl PotentialBranchSlots {
    fn len(&self) -> usize {
        self.branches.len()
    }

    fn slot_for(&self, equation: EquationId) -> Option<usize> {
        self.equation_slots.get(&equation).copied()
    }

    fn branches(&self) -> &[MirBranchRef] {
        &self.branches
    }

    fn declared_slots(&self) -> &HashMap<String, usize> {
        &self.declared_slots
    }
}

fn reject_unsupported_model_shape(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
    if !artifact.hir.arrays.is_empty() {
        return Err(unsupported(artifact, "arrays"));
    }
    for variable in &artifact.hir.variables {
        if variable.is_state {
            return Err(unsupported(
                artifact,
                format!("state variable '{}'", variable.name),
            ));
        }
        if !is_supported_scalar_value_type(variable.value_type) {
            return Err(unsupported(
                artifact,
                format!(
                    "non-numeric scalar variable '{}' with type {:?}",
                    variable.name, variable.value_type
                ),
            ));
        }
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }
    reject_unsupported_statements(artifact, &artifact.hir.statements)?;

    for equation in &artifact.mir.equations {
        if equation.kind == MirEquationKind::Indirect {
            return Err(unsupported(artifact, "indirect contributions"));
        }
    }
    for expression in &artifact.mir.expressions {
        match &expression.kind {
            HirExprKind::AnalogOperator { op } => match op {
                HirAnalogOperator::Ddt { abstol, .. } => {
                    if abstol.is_some() {
                        return Err(unsupported(artifact, "ddt abstol argument"));
                    }
                }
                HirAnalogOperator::Ddx { .. } => {}
                HirAnalogOperator::Limexp { .. } => {
                    return Err(unsupported(artifact, "convergence-limited limexp operator"));
                }
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("stateful or effectful analog operator {op:?}"),
                    ));
                }
            },
            HirExprKind::Laplace { .. } | HirExprKind::Zi { .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "stateful or effectful expression kind {:?}",
                        expression.kind
                    ),
                ));
            }
            HirExprKind::NoiseSource { .. } => {}
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                if args.len() != 1 {
                    return Err(unsupported(
                        artifact,
                        format!("ddt expects one operand, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                if args.len() != 2 {
                    return Err(unsupported(
                        artifact,
                        format!("ddx expects two operands, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {}
            HirExprKind::Call { name, .. } if is_stateful_or_effectful_call(name.as_str()) => {
                return Err(unsupported(
                    artifact,
                    format!("stateful or effectful analog operator call {name}"),
                ));
            }
            _ => {}
        }
    }
    reject_ddx_feeding_current_contributions(artifact)?;

    Ok(())
}

fn is_supported_scalar_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real | CanonicalValueType::Integer | CanonicalValueType::Boolean
    )
}

fn reject_unsupported_statements(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                if assignment.index.is_some() {
                    return Err(unsupported(
                        artifact,
                        format!("indexed assignment to '{}'", assignment.target_name),
                    ));
                }
                if usize::from(assignment.target) >= artifact.hir.variables.len() {
                    return Err(RustBackendError::internal(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        format!(
                            "assignment target {} is outside HIR variable arena",
                            assignment.target
                        ),
                    ));
                }
            }
            HirStatement::Loop(loop_statement) => {
                reject_unsupported_statements(artifact, &loop_statement.body)?;
            }
        }
    }
    Ok(())
}

fn reject_ddx_feeding_current_contributions(
    artifact: &CanonicalIrArtifact,
) -> Result<(), RustBackendError> {
    let mut ddx_tainted_variables = HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for statement in &artifact.hir.statements {
            let HirStatement::Assignment(assignment) = statement else {
                continue;
            };
            let mut visited = HashSet::new();
            if expr_contains_ddx_or_tainted_variable(
                artifact,
                assignment.expr.id,
                &ddx_tainted_variables,
                &mut visited,
            )? && ddx_tainted_variables.insert(assignment.target_name.to_string())
            {
                changed = true;
            }
        }
    }

    for equation in &artifact.mir.equations {
        let mut visited = HashSet::new();
        if expr_contains_ddx_or_tainted_variable(
            artifact,
            equation.expression.id,
            &ddx_tainted_variables,
            &mut visited,
        )? {
            return Err(unsupported(
                artifact,
                "ddx feeding a current contribution requires second derivatives",
            ));
        }
    }

    Ok(())
}

fn expr_contains_ddx_or_tainted_variable(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    ddx_tainted_variables: &HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<bool, RustBackendError> {
    if !visited.insert(id) {
        return Ok(false);
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Identifier { name } => Ok(ddx_tainted_variables.contains(name.as_str())),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddx { .. },
        } => Ok(true),
        HirExprKind::Call { name, .. } if is_ddx_name(name.as_str()) => Ok(true),
        HirExprKind::Number { .. } | HirExprKind::StringLiteral { .. } => Ok(false),
        HirExprKind::BranchAccess { .. } | HirExprKind::NamedBranchAccess { .. } => Ok(false),
        HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
            exprs_contain_ddx_or_tainted_variable(artifact, args, ddx_tainted_variables, visited)
        }
        HirExprKind::Binary { left, right, .. } => exprs_contain_ddx_or_tainted_variable(
            artifact,
            &[*left, *right],
            ddx_tainted_variables,
            visited,
        ),
        HirExprKind::Unary { operand, .. } => expr_contains_ddx_or_tainted_variable(
            artifact,
            *operand,
            ddx_tainted_variables,
            visited,
        ),
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => exprs_contain_ddx_or_tainted_variable(
            artifact,
            &[*condition, *then_expr, *else_expr],
            ddx_tainted_variables,
            visited,
        ),
        HirExprKind::ArrayAccess { index, .. } => {
            expr_contains_ddx_or_tainted_variable(artifact, *index, ddx_tainted_variables, visited)
        }
        HirExprKind::ArrayLiteral { elements } => exprs_contain_ddx_or_tainted_variable(
            artifact,
            elements,
            ddx_tainted_variables,
            visited,
        ),
        HirExprKind::AnalogOperator { op } => match op {
            HirAnalogOperator::Ddt { expr, abstol } => {
                let children = [Some(*expr), *abstol];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::Limexp { expr } => expr_contains_ddx_or_tainted_variable(
                artifact,
                *expr,
                ddx_tainted_variables,
                visited,
            ),
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => {
                let children = [Some(*expr), *ic, *assert, *abstol];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => {
                let children = [Some(*expr), *ic, *modulus, *offset, *abstol];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => {
                let children = [Some(*expr), Some(*delay), *max_delay];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
            } => {
                let children = [Some(*expr), *delay, *rise, *fall, *tolerance];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => {
                let children = [Some(*expr), *max_rise, *max_fall];
                optional_exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &children,
                    ddx_tainted_variables,
                    visited,
                )
            }
            HirAnalogOperator::Ddx { .. } => Ok(true),
            HirAnalogOperator::LastCrossing { expr, .. } => expr_contains_ddx_or_tainted_variable(
                artifact,
                *expr,
                ddx_tainted_variables,
                visited,
            ),
        },
        HirExprKind::Laplace { expr, kind } => {
            if expr_contains_ddx_or_tainted_variable(
                artifact,
                *expr,
                ddx_tainted_variables,
                visited,
            )? {
                return Ok(true);
            }
            match kind {
                crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
                | crate::canonical_ir::HirLaplaceKind::ZeroDenominator {
                    zeros,
                    denominator: poles,
                }
                | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
                    numerator: zeros,
                    poles,
                }
                | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                    numerator: zeros,
                    denominator: poles,
                } => exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &zeros
                        .iter()
                        .chain(poles.iter())
                        .copied()
                        .collect::<Vec<_>>(),
                    ddx_tainted_variables,
                    visited,
                ),
            }
        }
        HirExprKind::Zi { expr, kind } => {
            if expr_contains_ddx_or_tainted_variable(
                artifact,
                *expr,
                ddx_tainted_variables,
                visited,
            )? {
                return Ok(true);
            }
            match kind {
                crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
                | crate::canonical_ir::HirZiKind::ZeroDenominator {
                    zeros,
                    denominator: poles,
                }
                | crate::canonical_ir::HirZiKind::NumeratorPole {
                    numerator: zeros,
                    poles,
                }
                | crate::canonical_ir::HirZiKind::NumeratorDenominator {
                    numerator: zeros,
                    denominator: poles,
                } => exprs_contain_ddx_or_tainted_variable(
                    artifact,
                    &zeros
                        .iter()
                        .chain(poles.iter())
                        .copied()
                        .collect::<Vec<_>>(),
                    ddx_tainted_variables,
                    visited,
                ),
            }
        }
        HirExprKind::NoiseSource { operands, .. } => exprs_contain_ddx_or_tainted_variable(
            artifact,
            operands,
            ddx_tainted_variables,
            visited,
        ),
    }
}

fn exprs_contain_ddx_or_tainted_variable(
    artifact: &CanonicalIrArtifact,
    ids: &[ExprId],
    ddx_tainted_variables: &HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<bool, RustBackendError> {
    for id in ids {
        if expr_contains_ddx_or_tainted_variable(artifact, *id, ddx_tainted_variables, visited)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn optional_exprs_contain_ddx_or_tainted_variable(
    artifact: &CanonicalIrArtifact,
    ids: &[Option<ExprId>],
    ddx_tainted_variables: &HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<bool, RustBackendError> {
    for id in ids.iter().flatten() {
        if expr_contains_ddx_or_tainted_variable(artifact, *id, ddx_tainted_variables, visited)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn collect_ddt_slots(artifact: &CanonicalIrArtifact) -> Result<DdtSlots, RustBackendError> {
    let mut collector = DdtSlotCollector {
        artifact,
        slots: HashMap::new(),
        visited: HashSet::new(),
    };

    for statement in &artifact.hir.statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                collector.collect(assignment.expr.id)?;
                if let Some(index) = &assignment.index {
                    collector.collect(index.id)?;
                }
            }
            HirStatement::Loop(loop_statement) => {
                collector.collect(loop_statement.condition.id)?;
                collector.collect_statements(&loop_statement.body)?;
            }
        }
    }
    for equation in &artifact.mir.equations {
        collector.collect(equation.expression.id)?;
    }

    Ok(DdtSlots::new(collector.slots))
}

fn collect_potential_branch_slots(
    artifact: &CanonicalIrArtifact,
) -> Result<PotentialBranchSlots, RustBackendError> {
    let mut slots = PotentialBranchSlots::default();
    for unknown in &artifact.mir.branch_unknowns {
        let slot = usize::from(unknown.id);
        let equation = artifact
            .mir
            .equations
            .get(usize::from(unknown.equation))
            .ok_or_else(|| {
                RustBackendError::internal(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    format!(
                        "branch unknown {} references missing equation {}",
                        unknown.id, unknown.equation
                    ),
                )
            })?;
        slots.equation_slots.insert(unknown.equation, slot);
        if let Some(name) = unknown.declared_name.as_deref() {
            slots.declared_slots.entry(name.to_string()).or_insert(slot);
        }
        slots.branches.push(MirBranchRef {
            label: equation.branch.label.clone(),
            declared_name: unknown.declared_name.clone(),
            pos_node: unknown.pos_node,
            neg_node: unknown.neg_node,
        });
    }

    Ok(slots)
}

#[derive(Debug, Clone, Default)]
struct TransientLiveness {
    values: HashSet<String>,
    derivatives: HashSet<String>,
}

impl TransientLiveness {
    fn is_value_live(&self, name: &str) -> bool {
        self.values.contains(name)
    }

    fn is_derivative_live(&self, name: &str) -> bool {
        self.derivatives.contains(name)
    }

    fn loop_has_live_value_assignment(
        &self,
        loop_statement: &crate::canonical_ir::HirLoop,
    ) -> bool {
        loop_statement
            .body
            .iter()
            .any(|statement| self.statement_has_live_value_assignment(statement))
    }

    fn statement_has_live_value_assignment(&self, statement: &HirStatement) -> bool {
        match statement {
            HirStatement::Assignment(assignment) => {
                self.is_value_live(assignment.target_name.as_str())
            }
            HirStatement::Loop(loop_statement) => {
                self.loop_has_live_value_assignment(loop_statement)
            }
        }
    }
}

fn collect_transient_liveness(
    artifact: &CanonicalIrArtifact,
) -> Result<TransientLiveness, RustBackendError> {
    let mut values = HashSet::new();
    let mut derivatives = HashSet::new();
    for equation in &artifact.mir.equations {
        collect_expression_identifiers(
            artifact,
            equation.expression.id,
            &mut values,
            &mut HashSet::new(),
        )?;
        collect_expression_identifiers(
            artifact,
            equation.expression.id,
            &mut derivatives,
            &mut HashSet::new(),
        )?;
    }

    loop {
        let value_changed =
            collect_live_statement_dependencies(artifact, &artifact.hir.statements, &mut values)?;
        let derivative_changed = collect_live_statement_dependencies(
            artifact,
            &artifact.hir.statements,
            &mut derivatives,
        )?;
        if !value_changed && !derivative_changed {
            break;
        }
    }

    Ok(TransientLiveness {
        values,
        derivatives,
    })
}

#[derive(Debug, Clone, Default)]
struct ReactiveLiveness {
    variables: HashSet<String>,
    equations: HashSet<EquationId>,
}

impl ReactiveLiveness {
    fn is_variable_live(&self, name: &str) -> bool {
        self.variables.contains(name)
    }

    fn is_equation_reactive(&self, equation: EquationId) -> bool {
        self.equations.contains(&equation)
    }

    fn loop_has_live_assignment(&self, loop_statement: &crate::canonical_ir::HirLoop) -> bool {
        loop_statement
            .body
            .iter()
            .any(|statement| self.statement_has_live_assignment(statement))
    }

    fn statement_has_live_assignment(&self, statement: &HirStatement) -> bool {
        match statement {
            HirStatement::Assignment(assignment) => {
                self.is_variable_live(assignment.target_name.as_str())
            }
            HirStatement::Loop(loop_statement) => self.loop_has_live_assignment(loop_statement),
        }
    }
}

fn collect_reactive_liveness(
    artifact: &CanonicalIrArtifact,
) -> Result<ReactiveLiveness, RustBackendError> {
    let dynamic_variables = collect_dynamic_variables(artifact)?;
    let mut live_variables = HashSet::new();
    let mut reactive_equations = HashSet::new();

    for equation in &artifact.mir.equations {
        if expr_depends_on_ddt_or_dynamic(
            artifact,
            equation.expression.id,
            &dynamic_variables,
            &mut HashSet::new(),
        )? {
            reactive_equations.insert(equation.id);
            collect_expression_identifiers(
                artifact,
                equation.expression.id,
                &mut live_variables,
                &mut HashSet::new(),
            )?;
        }
    }

    loop {
        if !collect_live_statement_dependencies(
            artifact,
            &artifact.hir.statements,
            &mut live_variables,
        )? {
            break;
        }
    }

    Ok(ReactiveLiveness {
        variables: live_variables,
        equations: reactive_equations,
    })
}

fn collect_dynamic_variables(
    artifact: &CanonicalIrArtifact,
) -> Result<HashSet<String>, RustBackendError> {
    let mut dynamic_variables = HashSet::new();
    loop {
        let mut next = dynamic_variables.clone();
        let mut changed = false;
        collect_dynamic_statement_targets(
            artifact,
            &artifact.hir.statements,
            &dynamic_variables,
            &mut changed,
            &mut next,
        )?;
        if !changed {
            break;
        }
        dynamic_variables = next;
    }
    Ok(dynamic_variables)
}

fn collect_dynamic_statement_targets(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    current: &HashSet<String>,
    changed: &mut bool,
    next: &mut HashSet<String>,
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                if expr_depends_on_ddt_or_dynamic(
                    artifact,
                    assignment.expr.id,
                    current,
                    &mut HashSet::new(),
                )? && next.insert(assignment.target_name.to_string())
                {
                    *changed = true;
                }
            }
            HirStatement::Loop(loop_statement) => {
                collect_dynamic_statement_targets(
                    artifact,
                    &loop_statement.body,
                    current,
                    changed,
                    next,
                )?;
            }
        }
    }
    Ok(())
}

fn collect_live_statement_dependencies(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    live_variables: &mut HashSet<String>,
) -> Result<bool, RustBackendError> {
    let before = live_variables.len();
    for statement in statements.iter().rev() {
        match statement {
            HirStatement::Assignment(assignment) => {
                if live_variables.contains(assignment.target_name.as_str()) {
                    collect_expression_identifiers(
                        artifact,
                        assignment.expr.id,
                        live_variables,
                        &mut HashSet::new(),
                    )?;
                    if let Some(index) = &assignment.index {
                        collect_expression_identifiers(
                            artifact,
                            index.id,
                            live_variables,
                            &mut HashSet::new(),
                        )?;
                    }
                }
            }
            HirStatement::Loop(loop_statement) => {
                collect_live_statement_dependencies(
                    artifact,
                    &loop_statement.body,
                    live_variables,
                )?;
                if loop_contains_live_assignment(loop_statement, live_variables) {
                    collect_expression_identifiers(
                        artifact,
                        loop_statement.condition.id,
                        live_variables,
                        &mut HashSet::new(),
                    )?;
                }
            }
        }
    }
    Ok(live_variables.len() != before)
}

fn loop_contains_live_assignment(
    loop_statement: &crate::canonical_ir::HirLoop,
    live_variables: &HashSet<String>,
) -> bool {
    loop_statement
        .body
        .iter()
        .any(|statement| statement_contains_live_assignment(statement, live_variables))
}

fn statement_contains_live_assignment(
    statement: &HirStatement,
    live_variables: &HashSet<String>,
) -> bool {
    match statement {
        HirStatement::Assignment(assignment) => {
            live_variables.contains(assignment.target_name.as_str())
        }
        HirStatement::Loop(loop_statement) => {
            loop_contains_live_assignment(loop_statement, live_variables)
        }
    }
}

fn expr_depends_on_ddt_or_dynamic(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    dynamic_variables: &HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<bool, RustBackendError> {
    if !visited.insert(id) {
        return Ok(false);
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Identifier { name } => Ok(dynamic_variables.contains(name.as_str())),
        HirExprKind::Call { name, .. } if is_ddt_name(name.as_str()) => Ok(true),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => Ok(true),
        other => {
            for child in expression_children(other) {
                if expr_depends_on_ddt_or_dynamic(artifact, child, dynamic_variables, visited)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

fn collect_expression_identifiers(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    identifiers: &mut HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<(), RustBackendError> {
    if !visited.insert(id) {
        return Ok(());
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    if let HirExprKind::Identifier { name } = &expression.kind {
        identifiers.insert(name.to_string());
    }
    for child in expression_children(&expression.kind) {
        collect_expression_identifiers(artifact, child, identifiers, visited)?;
    }
    Ok(())
}

fn expression_children(kind: &HirExprKind) -> Vec<ExprId> {
    let mut children = Vec::new();
    match kind {
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => {}
        HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
            children.extend(args.iter().copied());
        }
        HirExprKind::Binary { left, right, .. } => {
            children.push(*left);
            children.push(*right);
        }
        HirExprKind::Unary { operand, .. } => {
            children.push(*operand);
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            children.push(*condition);
            children.push(*then_expr);
            children.push(*else_expr);
        }
        HirExprKind::ArrayAccess { index, .. } => {
            children.push(*index);
        }
        HirExprKind::ArrayLiteral { elements } => {
            children.extend(elements.iter().copied());
        }
        HirExprKind::AnalogOperator { op } => push_analog_operator_children(op, &mut children),
        HirExprKind::Laplace { expr, kind } => {
            children.push(*expr);
            push_laplace_children(kind, &mut children);
        }
        HirExprKind::Zi { expr, kind } => {
            children.push(*expr);
            push_zi_children(kind, &mut children);
        }
        HirExprKind::NoiseSource { operands, .. } => {
            children.extend(operands.iter().copied());
        }
    }
    children
}

fn push_analog_operator_children(op: &HirAnalogOperator, children: &mut Vec<ExprId>) {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            children.push(*expr);
            children.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            children.push(*expr);
            children.extend([*ic, *assert, *abstol].into_iter().flatten());
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            children.push(*expr);
            children.extend([*ic, *modulus, *offset, *abstol].into_iter().flatten());
        }
        HirAnalogOperator::Ddx { expr, probe } => {
            children.push(*expr);
            children.push(*probe);
        }
        HirAnalogOperator::Limexp { expr } => {
            children.push(*expr);
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            children.push(*expr);
            children.push(*delay);
            children.extend(max_delay.iter().copied());
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            children.push(*expr);
            children.extend([*delay, *rise, *fall, *tolerance].into_iter().flatten());
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            children.push(*expr);
            children.extend([*max_rise, *max_fall].into_iter().flatten());
        }
        HirAnalogOperator::LastCrossing { expr, .. } => {
            children.push(*expr);
        }
    }
}

fn push_laplace_children(kind: &crate::canonical_ir::HirLaplaceKind, children: &mut Vec<ExprId>) {
    match kind {
        crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
        | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
            numerator: zeros,
            poles,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(poles.iter().copied());
        }
        crate::canonical_ir::HirLaplaceKind::ZeroDenominator { zeros, denominator }
        | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
            numerator: zeros,
            denominator,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(denominator.iter().copied());
        }
    }
}

fn push_zi_children(kind: &crate::canonical_ir::HirZiKind, children: &mut Vec<ExprId>) {
    match kind {
        crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
        | crate::canonical_ir::HirZiKind::NumeratorPole {
            numerator: zeros,
            poles,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(poles.iter().copied());
        }
        crate::canonical_ir::HirZiKind::ZeroDenominator { zeros, denominator }
        | crate::canonical_ir::HirZiKind::NumeratorDenominator {
            numerator: zeros,
            denominator,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(denominator.iter().copied());
        }
    }
}

struct DdtSlotCollector<'a> {
    artifact: &'a CanonicalIrArtifact,
    slots: HashMap<ExprId, usize>,
    visited: HashSet<ExprId>,
}

impl DdtSlotCollector<'_> {
    fn collect_statements(&mut self, statements: &[HirStatement]) -> Result<(), RustBackendError> {
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    self.collect(assignment.expr.id)?;
                    if let Some(index) = &assignment.index {
                        self.collect(index.id)?;
                    }
                }
                HirStatement::Loop(loop_statement) => {
                    self.collect(loop_statement.condition.id)?;
                    self.collect_statements(&loop_statement.body)?;
                }
            }
        }
        Ok(())
    }

    fn collect(&mut self, id: ExprId) -> Result<(), RustBackendError> {
        if !self.visited.insert(id) {
            return Ok(());
        }

        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                RustBackendError::internal(
                    self.artifact.metadata.source_package.as_str(),
                    self.artifact.mir.module_name.as_str(),
                    format!("expression {id} is outside MIR arena"),
                )
            })?;

        match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
            HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
                if let HirExprKind::Call { name, args } = &expression.kind
                    && is_ddt_name(name.as_str())
                {
                    if args.len() != 1 {
                        return Err(unsupported(
                            self.artifact,
                            format!("ddt expects one operand, found {}", args.len()),
                        ));
                    }
                    let next_slot = self.slots.len();
                    self.slots.entry(id).or_insert(next_slot);
                }
                for arg in args {
                    self.collect(*arg)?;
                }
            }
            HirExprKind::Binary { left, right, .. } => {
                self.collect(*left)?;
                self.collect(*right)?;
            }
            HirExprKind::Unary { operand, .. } => {
                self.collect(*operand)?;
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.collect(*condition)?;
                self.collect(*then_expr)?;
                self.collect(*else_expr)?;
            }
            HirExprKind::ArrayAccess { index, .. } => {
                self.collect(*index)?;
            }
            HirExprKind::ArrayLiteral { elements } => {
                for element in elements {
                    self.collect(*element)?;
                }
            }
            HirExprKind::AnalogOperator { op } => {
                self.collect_analog_operator(id, op)?;
            }
            HirExprKind::Laplace { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirLaplaceKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::Zi { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirZiKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::NoiseSource { operands, .. } => {
                for operand in operands {
                    self.collect(*operand)?;
                }
            }
        }

        Ok(())
    }

    fn collect_analog_operator(
        &mut self,
        id: ExprId,
        op: &HirAnalogOperator,
    ) -> Result<(), RustBackendError> {
        match op {
            HirAnalogOperator::Ddt { expr, abstol } => {
                if abstol.is_some() {
                    return Err(unsupported(self.artifact, "ddt abstol argument"));
                }
                let next_slot = self.slots.len();
                self.slots.entry(id).or_insert(next_slot);
                self.collect(*expr)?;
            }
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => {
                self.collect(*expr)?;
                for child in [*ic, *assert, *abstol].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => {
                self.collect(*expr)?;
                for child in [*ic, *modulus, *offset, *abstol].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Ddx { expr, probe } => {
                self.collect(*expr)?;
                self.collect(*probe)?;
            }
            HirAnalogOperator::Limexp { expr } => {
                self.collect(*expr)?;
            }
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => {
                self.collect(*expr)?;
                self.collect(*delay)?;
                if let Some(max_delay) = max_delay {
                    self.collect(*max_delay)?;
                }
            }
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
            } => {
                self.collect(*expr)?;
                for child in [*delay, *rise, *fall, *tolerance].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => {
                self.collect(*expr)?;
                for child in [*max_rise, *max_fall].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::LastCrossing { expr, .. } => {
                self.collect(*expr)?;
            }
        }
        Ok(())
    }
}

fn is_stateful_or_effectful_call(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "idt"
            | "idtmod"
            | "absdelay"
            | "transition"
            | "slew"
            | "last_crossing"
            | "laplace_zp"
            | "laplace_zd"
            | "laplace_np"
            | "laplace_nd"
            | "zi_zp"
            | "zi_zd"
            | "zi_np"
            | "zi_nd"
            | "noise_table"
            | "noise_table_log"
    )
}

fn is_ddt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddt")
}

fn is_ddx_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddx")
}

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "white_noise" | "$white_noise" | "flicker_noise" | "$flicker_noise"
    )
}

fn generate_mod_file() -> String {
    [
        "pub mod state;",
        "mod stamp;",
        "",
        "pub use state::{Instance, Parameters};",
        "",
    ]
    .join("\n")
}

fn generate_state_file(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    ddt_state_count: usize,
    branch_count: usize,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_variables)]\n\n");
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct Parameters {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!("    pub {field}: f64,\n"));
    }
    out.push_str("}\n\n");

    out.push_str("impl Default for Parameters {\n");
    out.push_str("    fn default() -> Self {\n");
    if artifact.mir.parameters.is_empty() {
        out.push_str("        Self {\n");
        out.push_str("        }\n");
    } else {
        out.push_str("        let mut params = Self {\n");
        for parameter in &artifact.mir.parameters {
            let field = &parameter_fields[parameter.name.as_str()];
            out.push_str(&format!("            {field}: 0.0,\n"));
        }
        out.push_str("        };\n");
        for parameter in &artifact.mir.parameters {
            let field = &parameter_fields[parameter.name.as_str()];
            let default = parameter_default_rust_expr(artifact, parameter, parameter_fields)?;
            out.push_str(&format!("        params.{field} = {default};\n"));
            out.push_str(&format!(
                "        validate_parameter_{field}(params.{field}).expect(\"generated Verilog-A parameter default must satisfy declared range\");\n"
            ));
        }
        out.push_str("        params\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&generate_parameter_validator(
            parameter.name.as_str(),
            field,
            parameter.range.as_ref(),
        )?);
        out.push('\n');
    }

    let node_count = artifact.mir.nodes.len();
    let terminal_count = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| node.is_external)
        .count();
    let internal_node_names: Vec<_> = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| !node.is_external)
        .map(|node| node.name.as_str())
        .collect();
    let internal_node_count = internal_node_names.len();
    let parameter_count = artifact.mir.parameters.len();
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct Instance {\n");
    out.push_str(&format!("    pub nodes: [usize; {node_count}],\n"));
    out.push_str(&format!("    pub branches: [usize; {branch_count}],\n"));
    out.push_str("    pub params: Parameters,\n");
    out.push_str(&format!(
        "    pub(crate) param_given: [bool; {parameter_count}],\n"
    ));
    out.push_str("    pub(crate) multiplicity: f64,\n");
    out.push_str(&format!(
        "    pub(crate) ddt_state_current: [f64; {ddt_state_count}],\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_previous: [f64; {ddt_state_count}],\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_initialized: [bool; {ddt_state_count}],\n"
    ));
    out.push_str("    pub(crate) time: f64,\n");
    out.push_str("    pub(crate) timestep: f64,\n");
    out.push_str("}\n\n");

    out.push_str("impl Instance {\n");
    out.push_str(&format!(
        "    pub const TERMINAL_COUNT: usize = {terminal_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_COUNT: usize = {internal_node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const NODE_COUNT: usize = {node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_NAMES: [&str; {internal_node_count}] = [{}];\n\n",
        internal_node_names
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "    pub const BRANCH_COUNT: usize = {branch_count};\n"
    ));
    out.push_str(&format!(
        "    pub const PARAMETER_COUNT: usize = {parameter_count};\n"
    ));
    out.push_str(&format!(
        "    pub const VARIABLE_COUNT: usize = {};\n",
        artifact.hir.variables.len()
    ));
    out.push_str(&format!(
        "    pub const DDT_STATE_COUNT: usize = {ddt_state_count};\n"
    ));
    out.push_str("    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;\n");
    out.push_str("    pub const DDT_EPSILON: f64 = 1.0e-20;\n\n");
    out.push_str("    pub fn new(nodes: &[usize]) -> Self {\n");
    out.push_str("        assert_eq!(nodes.len(), Self::NODE_COUNT, \"generated Verilog-A node count mismatch\");\n");
    out.push_str("        let mut mapped = [0usize; Self::NODE_COUNT];\n");
    out.push_str("        mapped.copy_from_slice(nodes);\n");
    out.push_str("        Self {\n");
    out.push_str("            nodes: mapped,\n");
    out.push_str("            branches: [0usize; Self::BRANCH_COUNT],\n");
    out.push_str("            params: Parameters::default(),\n");
    out.push_str("            param_given: [false; Self::PARAMETER_COUNT],\n");
    out.push_str("            multiplicity: 1.0,\n");
    out.push_str("            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],\n");
    out.push_str("            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],\n");
    out.push_str("            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],\n");
    out.push_str("            time: 0.0,\n");
    out.push_str("            timestep: 0.0,\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_branch_indices(&mut self, branches: &[usize]) {\n");
    out.push_str("        assert_eq!(branches.len(), Self::BRANCH_COUNT, \"generated Verilog-A branch count mismatch\");\n");
    out.push_str("        self.branches.copy_from_slice(branches);\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {\n",
    );
    out.push_str("        match name.to_ascii_lowercase().as_str() {\n");
    for parameter in &artifact.mir.parameters {
        let parameter_index = usize::from(parameter.id);
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!(
            "            \"{}\" => {{ validate_parameter_{field}(value)?; self.params.{field} = value; self.mark_param_given({parameter_index}); Ok(()) }}\n",
            parameter.name.to_ascii_lowercase()
        ));
        for alias in &parameter.aliases {
            out.push_str(&format!(
                "            \"{}\" => {{ validate_parameter_{field}(value)?; self.params.{field} = value; self.mark_param_given({parameter_index}); Ok(()) }}\n",
                alias.to_ascii_lowercase()
            ));
        }
    }
    out.push_str(&format!(
        "            _ => Err(format!(\"unknown parameter '{{}}' for generated Verilog-A model '{}'\", name)),\n",
        artifact.mir.module_name
    ));
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    #[inline]\n");
    out.push_str("    fn mark_param_given(&mut self, index: usize) {\n");
    out.push_str("        debug_assert!(index < Self::PARAMETER_COUNT, \"generated parameter index out of range\");\n");
    out.push_str("        self.param_given[index] = true;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_multiplicity(&mut self, multiplicity: f64) {\n");
    out.push_str("        if multiplicity.is_finite() && multiplicity > 0.0 {\n");
    out.push_str("            self.multiplicity = multiplicity;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {\n");
    out.push_str("        self.time = time;\n");
    out.push_str("        self.timestep = timestep;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::DDT_STATE_COUNT {\n");
    out.push_str("            self.ddt_state_previous[index] = self.ddt_state_current[index];\n");
    out.push_str("            self.ddt_state_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    if ddt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {\n");
        out.push_str("        debug_assert!(slot < Self::DDT_STATE_COUNT, \"generated ddt state slot out of range\");\n");
        out.push_str("        let previous = if self.ddt_state_initialized[slot] {\n");
        out.push_str("            self.ddt_state_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            value\n");
        out.push_str("        };\n");
        out.push_str("        self.ddt_state_current[slot] = value;\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            (value - previous) / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            self.ddt_state_previous[slot] = value;\n");
        out.push_str("            self.ddt_state_initialized[slot] = true;\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            derivative / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    Ok(out)
}

fn parameter_default_rust_expr(
    artifact: &CanonicalIrArtifact,
    parameter: &MirParameterSlot,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    if let Some(default) = parameter.default {
        validate_parameter_value_for_codegen(
            artifact,
            parameter.name.as_str(),
            default,
            parameter.range.as_ref(),
        )?;
        return Ok(format_f64(default));
    }

    if let Some(default_expr) = &parameter.default_expr {
        return lower_parameter_default_expr(artifact, default_expr.id, parameter_fields);
    }

    Err(unsupported(
        artifact,
        format!(
            "parameter '{}' default that does not fold to a constant",
            parameter.name
        ),
    ))
}

fn lower_parameter_default_expr(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("parameter default expression {expr} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Number { value, .. } => Ok(format_f64(*value)),
        HirExprKind::Identifier { name } => parameter_fields
            .get(name.as_str())
            .map(|field| format!("params.{field}"))
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("parameter default references non-parameter identifier '{name}'"),
                )
            }),
        HirExprKind::Unary { op, operand } => {
            let operand = lower_parameter_default_expr(artifact, *operand, parameter_fields)?;
            match op.as_str() {
                "Neg" => Ok(format!("(-{operand})")),
                "Pos" => Ok(format!("({operand})")),
                _ => Err(unsupported(
                    artifact,
                    format!("parameter default unary operator {op}"),
                )),
            }
        }
        HirExprKind::Binary { op, left, right } => {
            let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
            let operator = match op.as_str() {
                "Add" => "+",
                "Sub" => "-",
                "Mul" => "*",
                "Div" => "/",
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("parameter default binary operator {op}"),
                    ));
                }
            };
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::SystemFunction { name, args }
            if name.as_str().eq_ignore_ascii_case("$simparam")
                || name.as_str().eq_ignore_ascii_case("simparam") =>
        {
            let Some(fallback) = args.get(1) else {
                return Err(unsupported(
                    artifact,
                    "parameter default $simparam without explicit fallback",
                ));
            };
            lower_parameter_default_expr(artifact, *fallback, parameter_fields)
        }
        other => Err(unsupported(
            artifact,
            format!("parameter default expression kind {other:?}"),
        )),
    }
}

fn validate_parameter_value_for_codegen(
    artifact: &CanonicalIrArtifact,
    parameter_name: &str,
    value: f64,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<(), RustBackendError> {
    if !value.is_finite() {
        return Err(unsupported(
            artifact,
            format!("non-finite default for parameter '{parameter_name}'"),
        ));
    }
    if let Some(range) = range {
        if !range_contains(range, value) {
            return Err(unsupported(
                artifact,
                format!("default for parameter '{parameter_name}' violates declared range"),
            ));
        }
        if range.exclude.iter().any(|excluded| !excluded.is_finite()) {
            return Err(unsupported(
                artifact,
                format!("non-finite exclude constraint for parameter '{parameter_name}'"),
            ));
        }
    }
    Ok(())
}

fn range_contains(range: &crate::canonical_ir::HirParamRange, value: f64) -> bool {
    if let Some(min) = range.min {
        if range.min_exclusive {
            if value <= min {
                return false;
            }
        } else if value < min {
            return false;
        }
    }
    if let Some(max) = range.max {
        if range.max_exclusive {
            if value >= max {
                return false;
            }
        } else if value > max {
            return false;
        }
    }
    !range.exclude.contains(&value)
}

fn generate_parameter_validator(
    parameter_name: &str,
    field_name: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str(&format!(
        "fn validate_parameter_{field_name}(value: f64) -> Result<(), String> {{\n"
    ));
    out.push_str("    if !value.is_finite() {\n");
    out.push_str(&format!(
        "        return Err(format!(\"parameter '{}' must be finite, got {{}}\", value));\n",
        parameter_name
    ));
    out.push_str("    }\n");

    if let Some(range) = range {
        if let Some(min) = range.min.filter(|value| value.is_finite()) {
            let op = if range.min_exclusive { ">" } else { ">=" };
            let condition = if range.min_exclusive { "<=" } else { "<" };
            out.push_str(&format!(
                "    if value {condition} {} {{\n",
                format_f64(min)
            ));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must be {op} {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(min)
            ));
            out.push_str("    }\n");
        }
        if let Some(max) = range.max.filter(|value| value.is_finite()) {
            let op = if range.max_exclusive { "<" } else { "<=" };
            let condition = if range.max_exclusive { ">=" } else { ">" };
            out.push_str(&format!(
                "    if value {condition} {} {{\n",
                format_f64(max)
            ));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must be {op} {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(max)
            ));
            out.push_str("    }\n");
        }
        for excluded in &range.exclude {
            if !excluded.is_finite() {
                return Err(RustBackendError::unsupported(
                    "<generated>",
                    parameter_name,
                    "non-finite parameter exclude constraint",
                ));
            }
            out.push_str(&format!("    if value == {} {{\n", format_f64(*excluded)));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must not equal {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(*excluded)
            ));
            out.push_str("    }\n");
        }
    }

    out.push_str("    Ok(())\n");
    out.push_str("}\n");
    Ok(out)
}

fn generate_stamp_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
    transient_liveness: &TransientLiveness,
    reactive_liveness: &ReactiveLiveness,
    potential_branch_slots: &PotentialBranchSlots,
) -> Result<StampFiles, RustBackendError> {
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    out.push_str("const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;\n\n");
    out.push_str(&generate_scratch_struct());
    out.push('\n');
    out.push_str(&generate_ad_value_struct());
    out.push('\n');
    let mut helper_modules = StampHelperModules::default();
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    emit_stamp_body(
        artifact,
        parameter_fields,
        variable_fields,
        ddt_slots,
        transient_liveness,
        reactive_liveness,
        potential_branch_slots,
        false,
        "stamp_transient",
        &mut helper_modules,
        &mut out,
    )?;
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
    );
    emit_stamp_body(
        artifact,
        parameter_fields,
        variable_fields,
        ddt_slots,
        transient_liveness,
        reactive_liveness,
        potential_branch_slots,
        true,
        "stamp_reactive",
        &mut helper_modules,
        &mut out,
    )?;
    out.push_str("    }\n");
    out.push_str("}\n");
    let helpers = helper_modules.finish();
    if !helpers.is_empty() {
        let declarations = helpers
            .iter()
            .map(|file| {
                let module_name = file
                    .relative_path
                    .strip_suffix(".rs")
                    .expect("helper module path must end in .rs")
                    .replace('\\', "/");
                format!(
                    "#[path = \"{}\"]\nmod {};",
                    file.relative_path.replace('\\', "/"),
                    module_name.replace('/', "::")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        out = out.replacen(
            "const THERMAL_VOLTAGE_PER_K",
            &format!("{declarations}\n\nconst THERMAL_VOLTAGE_PER_K"),
            1,
        );
    }
    Ok(StampFiles {
        stamp: out,
        helpers,
    })
}

struct StampFiles {
    stamp: String,
    helpers: Vec<GeneratedRustFile>,
}

fn generate_scratch_struct() -> String {
    [
        "struct Scratch {",
        "    values: [f64; Instance::VARIABLE_COUNT],",
        "    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "}",
        "",
        "impl Scratch {",
        "    fn new() -> Self {",
        "        Self {",
        "            values: [0.0; Instance::VARIABLE_COUNT],",
        "            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn ad_value(&self, index: usize) -> AdValue {",
        "        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ad(&mut self, index: usize, value: &AdValue) {",
        "        self.values[index] = value.value;",
        "        self.node_derivatives[index] = value.node_derivatives;",
        "        self.branch_derivatives[index] = value.branch_derivatives;",
        "    }",
        "}",
        "",
        "struct ReactiveScratch {",
        "    values: [f64; Instance::VARIABLE_COUNT],",
        "    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "    reactive_values: [f64; Instance::VARIABLE_COUNT],",
        "    reactive_node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    reactive_branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "}",
        "",
        "impl ReactiveScratch {",
        "    fn new() -> Self {",
        "        Self {",
        "            values: [0.0; Instance::VARIABLE_COUNT],",
        "            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "            reactive_values: [0.0; Instance::VARIABLE_COUNT],",
        "            reactive_node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            reactive_branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn ad_value(&self, index: usize) -> AdValue {",
        "        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ad(&mut self, index: usize, value: &AdValue) {",
        "        self.values[index] = value.value;",
        "        self.node_derivatives[index] = value.node_derivatives;",
        "        self.branch_derivatives[index] = value.branch_derivatives;",
        "    }",
        "}",
        "",
    ]
    .join("\n")
}

fn generate_ad_value_struct() -> String {
    [
        "struct AdValue {",
        "    value: f64,",
        "    node_derivatives: [f64; Instance::NODE_COUNT],",
        "    branch_derivatives: [f64; Instance::BRANCH_COUNT],",
        "}",
        "",
        "impl AdValue {",
        "    #[inline]",
        "    fn constant(value: f64) -> Self {",
        "        Self { value, node_derivatives: [0.0; Instance::NODE_COUNT], branch_derivatives: [0.0; Instance::BRANCH_COUNT] }",
        "    }",
        "    #[inline]",
        "    fn voltage(ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) -> Self {",
        "        let pos_value = pos.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);",
        "        let mut value = Self::constant(pos_value - neg_value);",
        "        if let Some(index) = pos { value.node_derivatives[index] += 1.0; }",
        "        if let Some(index) = neg { value.node_derivatives[index] -= 1.0; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn branch_current(ctx: &GeneratedEvalContext<'_>, branches: &[usize; Instance::BRANCH_COUNT], slot: usize) -> Self {",
        "        let mut value = Self::constant(ctx.branch_current(branches[slot]));",
        "        value.branch_derivatives[slot] = 1.0;",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn neg(mut value: Self) -> Self {",
        "        value.value = -value.value;",
        "        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }",
        "        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add(left: Self, right: Self) -> Self {",
        "        let mut value = Self::constant(left.value + right.value);",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] + right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] + right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn sub(left: Self, right: Self) -> Self {",
        "        let mut value = Self::constant(left.value - right.value);",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] - right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] - right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul(left: Self, right: Self) -> Self {",
        "        let mut value = Self::constant(left.value * right.value);",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] * right.value + left.value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] * right.value + left.value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn square(arg: Self) -> Self {",
        "        let mut value = Self::constant(arg.value * arg.value);",
        "        let derivative_scale = 2.0 * arg.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = derivative_scale * arg.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = derivative_scale * arg.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div(left: Self, right: Self) -> Self {",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        let mut value = Self::constant(quotient);",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] * reciprocal + right.node_derivatives[index] * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] * reciprocal + right.branch_derivatives[index] * right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_from_scalar(scalar: f64, right: Self) -> Self {",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = scalar * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        let mut value = Self::constant(quotient);",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = right.node_derivatives[index] * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = right.branch_derivatives[index] * right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn scale(mut value: Self, scale: f64) -> Self {",
        "        value.value *= scale;",
        "        for derivative in &mut value.node_derivatives { *derivative *= scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn offset(mut value: Self, offset: f64) -> Self {",
        "        value.value += offset;",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn sub_from_scalar(scalar: f64, mut value: Self) -> Self {",
        "        value.value = scalar - value.value;",
        "        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }",
        "        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn unary_intrinsic(mut arg: Self, value: f64, derivative_scale: f64) -> Self {",
        "        arg.value = value;",
        "        for derivative in &mut arg.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut arg.branch_derivatives { *derivative *= derivative_scale; }",
        "        arg",
        "    }",
        "",
        "    #[inline]",
        "    fn abs(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 }) }",
        "    #[inline]",
        "    fn sqrt(arg: Self) -> Self { let value = arg.value.sqrt(); Self::unary_intrinsic(arg, value, 1.0 / (2.0 * value)) }",
        "    #[inline]",
        "    fn exp(arg: Self) -> Self { let value = arg.value.exp(); Self::unary_intrinsic(arg, value, value) }",
        "    #[inline]",
        "    fn ln(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.ln(), 1.0 / raw) }",
        "    #[inline]",
        "    fn log10(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.log10(), 1.0 / (raw * std::f64::consts::LN_10)) }",
        "    #[inline]",
        "    fn sin(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sin(), raw.cos()) }",
        "    #[inline]",
        "    fn cos(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cos(), -raw.sin()) }",
        "    #[inline]",
        "    fn tan(arg: Self) -> Self { let raw = arg.value; let cos = raw.cos(); Self::unary_intrinsic(arg, raw.tan(), 1.0 / (cos * cos)) }",
        "    #[inline]",
        "    fn sinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sinh(), raw.cosh()) }",
        "    #[inline]",
        "    fn cosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cosh(), raw.sinh()) }",
        "    #[inline]",
        "    fn tanh(arg: Self) -> Self { let raw = arg.value; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), 1.0 / (cosh * cosh)) }",
        "    #[inline]",
        "    fn floor(arg: Self) -> Self { Self::constant(arg.value.floor()) }",
        "    #[inline]",
        "    fn ceil(arg: Self) -> Self { Self::constant(arg.value.ceil()) }",
        "    #[inline]",
        "    fn pow_derivative(value: f64, base: f64, exponent: f64, dbase: f64, dexponent: f64) -> f64 {",
        "        if dexponent == 0.0 && exponent.is_finite() && exponent.fract() == 0.0 {",
        "            if exponent == 0.0 { 0.0 } else { exponent * base.powf(exponent - 1.0) * dbase }",
        "        } else {",
        "            value * (dexponent * base.ln() + exponent * (dbase / base))",
        "        }",
        "    }",
        "    #[inline]",
        "    fn powf(left: Self, exponent: f64) -> Self {",
        "        let value = left.value.powf(exponent);",
        "        let mut result = Self::constant(value);",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, left.value, exponent, left.node_derivatives[index], 0.0); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, left.value, exponent, left.branch_derivatives[index], 0.0); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn pow_from_scalar(base: f64, right: Self) -> Self {",
        "        let value = base.powf(right.value);",
        "        let mut result = Self::constant(value);",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, base, right.value, 0.0, right.node_derivatives[index]); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, base, right.value, 0.0, right.branch_derivatives[index]); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn pow(left: Self, right: Self) -> Self {",
        "        let value = left.value.powf(right.value);",
        "        let mut result = Self::constant(value);",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, left.value, right.value, left.node_derivatives[index], right.node_derivatives[index]); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, left.value, right.value, left.branch_derivatives[index], right.branch_derivatives[index]); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn min(left: Self, right: Self) -> Self { if left.value <= right.value { left } else { right } }",
        "    #[inline]",
        "    fn min_with_scalar(left: Self, right: f64) -> Self { if left.value <= right { left } else { Self::constant(right) } }",
        "    #[inline]",
        "    fn min_from_scalar(left: f64, right: Self) -> Self { if left <= right.value { Self::constant(left) } else { right } }",
        "    #[inline]",
        "    fn max(left: Self, right: Self) -> Self { if left.value >= right.value { left } else { right } }",
        "    #[inline]",
        "    fn max_with_scalar(left: Self, right: f64) -> Self { if left.value >= right { left } else { Self::constant(right) } }",
        "    #[inline]",
        "    fn max_from_scalar(left: f64, right: Self) -> Self { if left >= right.value { Self::constant(left) } else { right } }",
        "    #[inline]",
        "    fn hypot(left: Self, right: Self) -> Self {",
        "        let value = left.value.hypot(right.value);",
        "        let mut result = Self::constant(value);",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (left.value * left.node_derivatives[index] + right.value * right.node_derivatives[index]) / value; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (left.value * left.branch_derivatives[index] + right.value * right.branch_derivatives[index]) / value; }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn atan2(y: Self, x: Self) -> Self {",
        "        let denominator = x.value * x.value + y.value * y.value;",
        "        let mut result = Self::constant(y.value.atan2(x.value));",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (x.value * y.node_derivatives[index] - y.value * x.node_derivatives[index]) / denominator; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (x.value * y.branch_derivatives[index] - y.value * x.branch_derivatives[index]) / denominator; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn ddt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {",
        "        operand.value = value;",
        "        for derivative in &mut operand.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut operand.branch_derivatives { *derivative *= derivative_scale; }",
        "        operand",
        "    }",
        "",
        "    #[inline]",
        "    fn ddx_projection(expr: &Self, pos: Option<usize>, neg: Option<usize>) -> f64 {",
        "        let pos = pos.map(|index| expr.node_derivatives[index]).unwrap_or(0.0);",
        "        if let Some(neg) = neg { 0.5 * (pos - expr.node_derivatives[neg]) } else { pos }",
        "    }",
        "}",
        "",
    ]
    .join("\n")
}

fn emit_stamp_body(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
    transient_liveness: &TransientLiveness,
    reactive_liveness: &ReactiveLiveness,
    potential_branch_slots: &PotentialBranchSlots,
    reactive: bool,
    helper_prefix: &str,
    helper_modules: &mut StampHelperModules,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if reactive && ddt_slots.len() == 0 {
        return Ok(());
    }

    let uses_scratch = !artifact.hir.variables.is_empty();
    if uses_scratch {
        let scratch_type = if reactive {
            "ReactiveScratch"
        } else {
            "Scratch"
        };
        out.push_str(&format!(
            "        let mut scratch = {scratch_type}::new();\n"
        ));
    }
    let mut variables = if uses_scratch {
        emit_variable_initializers(
            artifact,
            variable_fields,
            potential_branch_slots.declared_slots().len(),
            reactive,
            reactive_liveness,
            out,
        )
    } else {
        HashMap::new()
    };
    let assignment_chunks = emit_assignment_statement_chunks(
        artifact,
        parameter_fields,
        variable_fields,
        &mut variables,
        ddt_slots,
        potential_branch_slots.declared_slots(),
        transient_liveness,
        reactive,
        reactive_liveness,
    )?;
    emit_chunked_stamp_helpers(
        helper_prefix,
        reactive,
        assignment_chunks,
        helper_modules,
        out,
    );

    if !reactive {
        for (slot, branch) in potential_branch_slots.branches().iter().enumerate() {
            out.push_str("        stamper.stamp_potential_branch(\n");
            out.push_str(&format!(
                "            {},\n",
                optional_node_expr(branch.pos_node)
            ));
            out.push_str(&format!(
                "            {},\n",
                optional_node_expr(branch.neg_node)
            ));
            out.push_str(&format!("            self.branches[{slot}],\n"));
            out.push_str("            self.multiplicity,\n");
            out.push_str("        );\n");
        }
        if !potential_branch_slots.branches().is_empty() {
            out.push('\n');
        }
    }

    let mut branch_currents = HashMap::new();
    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        if reactive && !reactive_liveness.is_equation_reactive(equation.id) {
            continue;
        }

        let prefix = format!("eq{index}");
        let lowered = if reactive {
            lower_reactive_expr_with_branch_currents(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
                &branch_currents,
                potential_branch_slots.declared_slots(),
            )?
        } else {
            lower_equation_expr_with_branch_currents(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
                &branch_currents,
                potential_branch_slots.declared_slots(),
            )?
        };
        if reactive {
            if lowered.has_reactive {
                for line in &lowered.lines {
                    out.push_str("        ");
                    out.push_str(line);
                    out.push('\n');
                }
                match equation.kind {
                    MirEquationKind::Current => {
                        out.push_str("        stamper.stamp_current_reactive(\n");
                        out.push_str(&format!(
                            "            {},\n",
                            optional_node_expr(equation.branch.pos_node)
                        ));
                        out.push_str(&format!(
                            "            {},\n",
                            optional_node_expr(equation.branch.neg_node)
                        ));
                        out.push_str("            &[\n");
                        for node_index in 0..artifact.mir.nodes.len() {
                            if is_zero_derivative(&lowered.reactive_derivatives[node_index]) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::node(self.nodes[{node_index}], self.multiplicity * ({})),\n",
                                lowered.reactive_derivatives[node_index]
                            ));
                        }
                        for branch_index in 0..lowered.reactive_branch_derivatives.len() {
                            if is_zero_derivative(
                                &lowered.reactive_branch_derivatives[branch_index],
                            ) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::branch(self.branches[{branch_index}], self.multiplicity * ({})),\n",
                                lowered.reactive_branch_derivatives[branch_index]
                            ));
                        }
                        out.push_str("            ],\n");
                        out.push_str("        );\n");
                    }
                    MirEquationKind::Potential => {
                        let slot =
                            potential_branch_slots
                                .slot_for(equation.id)
                                .ok_or_else(|| {
                                    RustBackendError::internal(
                                        artifact.metadata.source_package.as_str(),
                                        artifact.mir.module_name.as_str(),
                                        format!(
                                            "potential equation {} has no generated branch slot",
                                            equation.id
                                        ),
                                    )
                                })?;
                        out.push_str("        stamper.stamp_potential_reactive(\n");
                        out.push_str(&format!("            self.branches[{slot}],\n"));
                        out.push_str("            &[\n");
                        for node_index in 0..artifact.mir.nodes.len() {
                            if is_zero_derivative(&lowered.reactive_derivatives[node_index]) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::node(self.nodes[{node_index}], {}),\n",
                                lowered.reactive_derivatives[node_index]
                            ));
                        }
                        for branch_index in 0..lowered.reactive_branch_derivatives.len() {
                            if is_zero_derivative(
                                &lowered.reactive_branch_derivatives[branch_index],
                            ) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::branch(self.branches[{branch_index}], {}),\n",
                                lowered.reactive_branch_derivatives[branch_index]
                            ));
                        }
                        out.push_str("            ],\n");
                        out.push_str("        );\n");
                    }
                    MirEquationKind::Indirect => {
                        return Err(unsupported(artifact, "indirect contributions"));
                    }
                }
            }
            continue;
        }

        for line in &lowered.lines {
            out.push_str("        ");
            out.push_str(line);
            out.push('\n');
        }

        let value = format!("{prefix}_value");
        out.push_str(&format!("        let {value}: f64 = {};\n", lowered.value));
        let mut node_derivatives = Vec::with_capacity(lowered.derivatives.len());
        for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                node_derivatives.push("0.0".to_string());
            } else if is_inline_derivative_expr(derivative) {
                node_derivatives.push(derivative.clone());
            } else {
                let local = format!("{prefix}_d_n{node_index}");
                out.push_str(&format!("        let {local}: f64 = {derivative};\n"));
                node_derivatives.push(local);
            }
        }
        let mut branch_derivatives = Vec::with_capacity(lowered.branch_derivatives.len());
        for (branch_index, derivative) in lowered.branch_derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                branch_derivatives.push("0.0".to_string());
            } else if is_inline_derivative_expr(derivative) {
                branch_derivatives.push(derivative.clone());
            } else {
                let local = format!("{prefix}_d_b{branch_index}");
                out.push_str(&format!("        let {local}: f64 = {derivative};\n"));
                branch_derivatives.push(local);
            }
        }
        if equation.kind == MirEquationKind::Current {
            cache_named_branch_current(
                artifact,
                equation,
                &mut branch_currents,
                &value,
                &node_derivatives,
                &branch_derivatives,
                &lowered,
            );
        }
        match equation.kind {
            MirEquationKind::Current => {
                out.push_str("        stamper.stamp_current(\n");
                out.push_str(&format!(
                    "            {},\n",
                    optional_node_expr(equation.branch.pos_node)
                ));
                out.push_str(&format!(
                    "            {},\n",
                    optional_node_expr(equation.branch.neg_node)
                ));
                out.push_str(&format!("            self.multiplicity * ({value}),\n"));
                out.push_str("            &[\n");
                for (node_index, derivative) in node_derivatives.iter().enumerate() {
                    if is_zero_derivative(derivative) {
                        continue;
                    }
                    out.push_str(&format!(
                        "                GeneratedDerivative::node(self.nodes[{node_index}], self.multiplicity * {derivative}),\n"
                    ));
                }
                for (branch_index, derivative) in branch_derivatives.iter().enumerate() {
                    if is_zero_derivative(derivative) {
                        continue;
                    }
                    out.push_str(&format!(
                        "                GeneratedDerivative::branch(self.branches[{branch_index}], self.multiplicity * {derivative}),\n"
                    ));
                }
                out.push_str("            ],\n");
                out.push_str("        );\n");
            }
            MirEquationKind::Potential => {
                let slot = potential_branch_slots
                    .slot_for(equation.id)
                    .ok_or_else(|| {
                        RustBackendError::internal(
                            artifact.metadata.source_package.as_str(),
                            artifact.mir.module_name.as_str(),
                            format!(
                                "potential equation {} has no generated branch slot",
                                equation.id
                            ),
                        )
                    })?;
                out.push_str("        stamper.stamp_potential(\n");
                out.push_str(&format!("            self.branches[{slot}],\n"));
                out.push_str(&format!("            {value},\n"));
                out.push_str("            &[\n");
                for (node_index, derivative) in node_derivatives.iter().enumerate() {
                    if is_zero_derivative(derivative) {
                        continue;
                    }
                    out.push_str(&format!(
                        "                GeneratedDerivative::node(self.nodes[{node_index}], {derivative}),\n"
                    ));
                }
                for (branch_index, derivative) in branch_derivatives.iter().enumerate() {
                    if is_zero_derivative(derivative) {
                        continue;
                    }
                    out.push_str(&format!(
                        "                GeneratedDerivative::branch(self.branches[{branch_index}], {derivative}),\n"
                    ));
                }
                out.push_str("            ],\n");
                out.push_str("        );\n");
            }
            MirEquationKind::Indirect => {
                return Err(unsupported(artifact, "indirect contributions"));
            }
        }
    }
    Ok(())
}

fn cache_named_branch_current(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    branch_currents: &mut HashMap<String, LoweredVariable>,
    value: &str,
    derivatives: &[String],
    branch_derivatives: &[String],
    lowered: &LoweredExpr,
) {
    let Some(branch_name) = declared_contribution_branch_name(artifact, equation) else {
        return;
    };

    let next = LoweredVariable {
        value: value.to_string(),
        derivatives: derivatives.to_vec(),
        branch_derivatives: branch_derivatives.to_vec(),
        has_reactive: lowered.has_reactive,
        reactive_value: lowered.reactive_value.clone(),
        reactive_derivatives: lowered.reactive_derivatives.clone(),
        reactive_branch_derivatives: lowered.reactive_branch_derivatives.clone(),
    };

    branch_currents
        .entry(branch_name)
        .and_modify(|current| {
            current.value = format!("({} + {})", current.value, next.value);
            for (current_derivative, next_derivative) in
                current.derivatives.iter_mut().zip(next.derivatives.iter())
            {
                *current_derivative = format!("({current_derivative} + {next_derivative})");
            }
            for (current_derivative, next_derivative) in current
                .branch_derivatives
                .iter_mut()
                .zip(next.branch_derivatives.iter())
            {
                *current_derivative = format!("({current_derivative} + {next_derivative})");
            }
            if next.has_reactive {
                if current.has_reactive {
                    current.reactive_value =
                        format!("({} + {})", current.reactive_value, next.reactive_value);
                    for (current_derivative, next_derivative) in current
                        .reactive_derivatives
                        .iter_mut()
                        .zip(next.reactive_derivatives.iter())
                    {
                        *current_derivative = format!("({current_derivative} + {next_derivative})");
                    }
                    for (current_derivative, next_derivative) in current
                        .reactive_branch_derivatives
                        .iter_mut()
                        .zip(next.reactive_branch_derivatives.iter())
                    {
                        *current_derivative = format!("({current_derivative} + {next_derivative})");
                    }
                } else {
                    current.has_reactive = true;
                    current.reactive_value = next.reactive_value.clone();
                    current.reactive_derivatives = next.reactive_derivatives.clone();
                    current.reactive_branch_derivatives = next.reactive_branch_derivatives.clone();
                }
            }
        })
        .or_insert(next);
}

fn declared_contribution_branch_name(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Option<String> {
    if let Some(name) = equation.branch.declared_name.as_deref() {
        return Some(name.to_string());
    }
    let contribution = artifact
        .hir
        .contributions
        .get(usize::from(equation.contribution))?;
    let branch_name = contribution.branch.as_str();
    if let Some(branch) = artifact
        .mir
        .branches
        .iter()
        .find(|branch| branch.name.as_str() == branch_name)
    {
        return Some(branch.name.to_string());
    }

    let mut matches = artifact.mir.branches.iter().filter(|branch| {
        branch.pos_node == equation.branch.pos_node && branch.neg_node == equation.branch.neg_node
    });
    let first = matches.next()?;
    if matches.next().is_none() {
        Some(first.name.to_string())
    } else {
        None
    }
}

fn variable_local_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    let names = artifact
        .hir
        .variables
        .iter()
        .map(|variable| variable.name.to_string())
        .collect::<Vec<_>>();
    unique_identifiers(&names)
}

fn emit_variable_initializers(
    artifact: &CanonicalIrArtifact,
    _variable_fields: &HashMap<String, String>,
    branch_axis_count: usize,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    out: &mut String,
) -> HashMap<String, LoweredVariable> {
    let mut variables = HashMap::new();
    for variable in &artifact.hir.variables {
        if reactive && !reactive_liveness.is_variable_live(variable.name.as_str()) {
            continue;
        }
        let variable_index = usize::from(variable.id);
        let mut derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        let mut branch_derivatives = Vec::with_capacity(branch_axis_count);
        let reactive_value = format!("scratch.reactive_values[{variable_index}]");
        let mut reactive_derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        let mut reactive_branch_derivatives = Vec::with_capacity(branch_axis_count);
        for node_index in 0..artifact.mir.nodes.len() {
            derivatives.push(format!(
                "scratch.node_derivatives[{variable_index}][{node_index}]"
            ));
            if reactive {
                reactive_derivatives.push(format!(
                    "scratch.reactive_node_derivatives[{variable_index}][{node_index}]"
                ));
            } else {
                reactive_derivatives.push("0.0".to_string());
            }
        }
        for branch_index in 0..branch_axis_count {
            branch_derivatives.push(format!(
                "scratch.branch_derivatives[{variable_index}][{branch_index}]"
            ));
            if reactive {
                reactive_branch_derivatives.push(format!(
                    "scratch.reactive_branch_derivatives[{variable_index}][{branch_index}]"
                ));
            } else {
                reactive_branch_derivatives.push("0.0".to_string());
            }
        }
        variables.insert(
            variable.name.to_string(),
            LoweredVariable {
                value: format!("scratch.values[{variable_index}]"),
                derivatives,
                branch_derivatives,
                has_reactive: false,
                reactive_value: if reactive {
                    reactive_value
                } else {
                    "0.0".to_string()
                },
                reactive_derivatives,
                reactive_branch_derivatives,
            },
        );
    }
    if !artifact.hir.variables.is_empty() {
        out.push('\n');
    }
    variables
}

fn emit_assignment_statement_chunks(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, usize>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
) -> Result<Vec<String>, RustBackendError> {
    let mut chunks = Vec::new();
    for (index, statement) in artifact.hir.statements.iter().enumerate() {
        let statement_prefix = format!("assign{index}");
        let mut chunk = String::new();
        emit_statement_list(
            artifact,
            std::slice::from_ref(statement),
            parameter_fields,
            variable_fields,
            variables,
            &mut chunk,
            ddt_slots,
            branch_current_unknowns,
            transient_liveness,
            reactive,
            reactive_liveness,
            "        ",
            &statement_prefix,
        )?;
        if !chunk.is_empty() {
            chunk.push('\n');
            chunks.push(chunk);
        }
    }
    Ok(chunks)
}

fn emit_chunked_stamp_helpers(
    helper_prefix: &str,
    reactive: bool,
    chunks: Vec<String>,
    helper_modules: &mut StampHelperModules,
    out: &mut String,
) {
    if chunks.is_empty() {
        return;
    }

    let total_lines: usize = chunks.iter().map(|chunk| chunk.lines().count()).sum();
    if total_lines <= MAX_STAMP_HELPER_LINES {
        for chunk in chunks {
            out.push_str(&chunk);
        }
        return;
    }

    let mut block_index = 0usize;
    let mut block = String::new();
    let mut block_lines = 0usize;
    for chunk in chunks {
        let chunk_lines = chunk.lines().count();
        if block_lines > 0 && block_lines + chunk_lines > MAX_STAMP_HELPER_LINES {
            emit_stamp_helper_method(helper_prefix, reactive, block_index, &block, helper_modules);
            out.push_str(&format!(
                "        self.{helper_prefix}_block_{block_index}(ctx, stamper, &mut scratch);\n"
            ));
            block.clear();
            block_lines = 0;
            block_index += 1;
        }
        block_lines += chunk_lines;
        block.push_str(&chunk);
    }
    if !block.is_empty() {
        emit_stamp_helper_method(helper_prefix, reactive, block_index, &block, helper_modules);
        out.push_str(&format!(
            "        self.{helper_prefix}_block_{block_index}(ctx, stamper, &mut scratch);\n"
        ));
    }
    out.push('\n');
}

fn emit_stamp_helper_method(
    helper_prefix: &str,
    reactive: bool,
    block_index: usize,
    block: &str,
    helper_modules: &mut StampHelperModules,
) {
    let stamper_type = if reactive {
        "GeneratedReactiveStamper"
    } else {
        "GeneratedStamper"
    };
    let scratch_type = if reactive {
        "ReactiveScratch"
    } else {
        "Scratch"
    };
    let mut method = format!(
        "\n    pub(super) fn {helper_prefix}_block_{block_index}(\n        &mut self,\n        ctx: &GeneratedEvalContext<'_>,\n        stamper: &mut {stamper_type}<'_>,\n        scratch: &mut {scratch_type},\n    ) {{\n"
    );
    method.push_str("        let _ = stamper;\n");
    method.push_str(block);
    method.push_str("    }\n");
    helper_modules.push_method(method, reactive);
}

#[derive(Debug, Default)]
struct StampHelperModules {
    modules: Vec<StampHelperModule>,
}

impl StampHelperModules {
    fn push_method(&mut self, method_part: String, reactive: bool) {
        let needs_new_module = self
            .modules
            .last()
            .map(|module| module.method_count >= MAX_STAMP_HELPERS_PER_MODULE)
            .unwrap_or(true);
        if needs_new_module {
            let index = self.modules.len();
            self.modules.push(StampHelperModule {
                module_name: format!("stamp_blocks_{index}"),
                contents: String::new(),
                method_count: 0,
                uses_reactive_scratch: false,
                uses_transient_scratch: false,
            });
        }

        let module = self
            .modules
            .last_mut()
            .expect("helper module must exist after allocation");
        if method_part.starts_with("\n    pub(super) fn ") {
            module.method_count += 1;
            if reactive {
                module.uses_reactive_scratch = true;
            } else {
                module.uses_transient_scratch = true;
            }
        }
        module.contents.push_str(&method_part);
    }

    fn finish(self) -> Vec<GeneratedRustFile> {
        self.modules
            .into_iter()
            .map(|module| GeneratedRustFile {
                relative_path: format!("{}.rs", module.module_name),
                contents: module.finish(),
            })
            .collect()
    }
}

#[derive(Debug)]
struct StampHelperModule {
    module_name: String,
    contents: String,
    method_count: usize,
    uses_reactive_scratch: bool,
    uses_transient_scratch: bool,
}

impl StampHelperModule {
    fn finish(self) -> String {
        let mut imports = vec![
            "AdValue",
            "GeneratedEvalContext",
            "GeneratedReactiveStamper",
            "GeneratedStamper",
        ];
        if self.uses_reactive_scratch {
            imports.push("ReactiveScratch");
        }
        if self.uses_transient_scratch {
            imports.push("Scratch");
        }
        imports.push("THERMAL_VOLTAGE_PER_K");

        format!(
            "#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]\n\nuse super::{{{}}};\nuse super::super::state::Instance;\n\nimpl Instance {{\n{}}}\n",
            imports.join(", "),
            self.contents
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_statement_list(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, usize>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    for (index, statement) in statements.iter().enumerate() {
        let statement_prefix = format!("{prefix}{index}");
        match statement {
            HirStatement::Assignment(assignment) => emit_assignment_statement(
                artifact,
                assignment,
                parameter_fields,
                variable_fields,
                variables,
                out,
                ddt_slots,
                branch_current_unknowns,
                transient_liveness,
                reactive,
                reactive_liveness,
                indent,
                &statement_prefix,
            )?,
            HirStatement::Loop(loop_statement) => emit_loop_statement(
                artifact,
                loop_statement,
                parameter_fields,
                variable_fields,
                variables,
                out,
                ddt_slots,
                branch_current_unknowns,
                transient_liveness,
                reactive,
                reactive_liveness,
                indent,
                &statement_prefix,
            )?,
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_assignment_statement(
    artifact: &CanonicalIrArtifact,
    assignment: &crate::canonical_ir::HirAssignment,
    parameter_fields: &HashMap<String, String>,
    _variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, usize>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    if reactive && !reactive_liveness.is_variable_live(assignment.target_name.as_str()) {
        return Ok(());
    }
    if !reactive && !transient_liveness.is_value_live(assignment.target_name.as_str()) {
        return Ok(());
    }
    if reactive
        && !expr_depends_on_ddt_or_dynamic(
            artifact,
            assignment.expr.id,
            &HashSet::new(),
            &mut HashSet::new(),
        )?
    {
        return emit_compact_assignment_statement(
            artifact,
            assignment,
            parameter_fields,
            variables,
            out,
            ddt_slots,
            branch_current_unknowns,
            indent,
            prefix,
        );
    }

    let derivatives_live =
        reactive || transient_liveness.is_derivative_live(assignment.target_name.as_str());
    if !reactive && derivatives_live {
        return emit_compact_assignment_statement(
            artifact,
            assignment,
            parameter_fields,
            variables,
            out,
            ddt_slots,
            branch_current_unknowns,
            indent,
            prefix,
        );
    }
    let lowered = if reactive {
        let branch_currents = HashMap::new();
        lower_reactive_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    } else if derivatives_live {
        let branch_currents = HashMap::new();
        lower_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    } else {
        let branch_currents = HashMap::new();
        lower_value_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    };
    for line in lowered.lines {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    let target = artifact
        .hir
        .variables
        .get(usize::from(assignment.target))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!(
                    "assignment target {} is outside HIR variable arena",
                    assignment.target
                ),
            )
        })?;
    let target_index = usize::from(target.id);
    let target_local = format!("scratch.values[{target_index}]");
    out.push_str(&format!("{indent}{target_local} = {};\n", lowered.value));
    let mut derivative_locals = Vec::with_capacity(lowered.derivatives.len());
    for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
        if is_zero_derivative(derivative) {
            derivative_locals.push("0.0".to_string());
            continue;
        }
        if is_constant_derivative_expr(derivative) {
            derivative_locals.push(derivative.clone());
            continue;
        }
        let derivative_local = format!("scratch.node_derivatives[{target_index}][{node_index}]");
        out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
        derivative_locals.push(derivative_local);
    }
    let mut branch_derivative_locals = Vec::with_capacity(lowered.branch_derivatives.len());
    for (branch_index, derivative) in lowered.branch_derivatives.iter().enumerate() {
        if is_zero_derivative(derivative) {
            branch_derivative_locals.push("0.0".to_string());
            continue;
        }
        if is_constant_derivative_expr(derivative) {
            branch_derivative_locals.push(derivative.clone());
            continue;
        }
        let derivative_local =
            format!("scratch.branch_derivatives[{target_index}][{branch_index}]");
        out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
        branch_derivative_locals.push(derivative_local);
    }
    let target_reactive = format!("scratch.reactive_values[{target_index}]");
    let mut reactive_derivative_locals = Vec::with_capacity(lowered.reactive_derivatives.len());
    let mut reactive_branch_derivative_locals =
        Vec::with_capacity(lowered.reactive_branch_derivatives.len());
    if reactive {
        out.push_str(&format!(
            "{indent}{target_reactive} = {};\n",
            lowered.reactive_value
        ));
        for (node_index, derivative) in lowered.reactive_derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                reactive_derivative_locals.push("0.0".to_string());
                continue;
            }
            if is_constant_derivative_expr(derivative) {
                reactive_derivative_locals.push(derivative.clone());
                continue;
            }
            let derivative_local =
                format!("scratch.reactive_node_derivatives[{target_index}][{node_index}]");
            out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
            reactive_derivative_locals.push(derivative_local);
        }
        for (branch_index, derivative) in lowered.reactive_branch_derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                reactive_branch_derivative_locals.push("0.0".to_string());
                continue;
            }
            if is_constant_derivative_expr(derivative) {
                reactive_branch_derivative_locals.push(derivative.clone());
                continue;
            }
            let derivative_local =
                format!("scratch.reactive_branch_derivatives[{target_index}][{branch_index}]");
            out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
            reactive_branch_derivative_locals.push(derivative_local);
        }
    } else {
        reactive_derivative_locals = zero_derivative_vec(artifact.mir.nodes.len());
        reactive_branch_derivative_locals = zero_derivative_vec(branch_current_unknowns.len());
    }
    variables.insert(
        target.name.to_string(),
        LoweredVariable {
            value: target_local,
            derivatives: derivative_locals,
            branch_derivatives: branch_derivative_locals,
            has_reactive: reactive && lowered.has_reactive,
            reactive_value: if reactive {
                target_reactive
            } else {
                "0.0".to_string()
            },
            reactive_derivatives: reactive_derivative_locals,
            reactive_branch_derivatives: reactive_branch_derivative_locals,
        },
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_compact_assignment_statement(
    artifact: &CanonicalIrArtifact,
    assignment: &crate::canonical_ir::HirAssignment,
    parameter_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, usize>,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    let target = artifact
        .hir
        .variables
        .get(usize::from(assignment.target))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!(
                    "assignment target {} is outside HIR variable arena",
                    assignment.target
                ),
            )
        })?;
    let target_index = usize::from(target.id);
    let mut emitter = CompactAdEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_current_unknowns,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };

    if emit_compact_conditional_noop_assignment(
        &mut emitter,
        assignment.expr.id,
        target.name.as_str(),
        target_index,
        out,
        indent,
    )? {
        drop(emitter);
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_current_unknowns.len(), false),
        );
        return Ok(());
    }

    if emit_compact_conditional_scalar_branch_assignment(
        &mut emitter,
        assignment.expr.id,
        target_index,
        out,
        indent,
    )? {
        drop(emitter);
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_current_unknowns.len(), false),
        );
        return Ok(());
    }

    if let Some(source_index) = emitter.ad_identifier_index(assignment.expr.id)? {
        if source_index != target_index {
            push_compact_ad_copy(out, indent, target_index, source_index);
        }
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_current_unknowns.len(), false),
        );
        return Ok(());
    }

    if let Some(value) = emitter.zero_derivative_value_expr(assignment.expr.id)? {
        out.push_str(&format!(
            "{indent}scratch.values[{target_index}] = {value};\n"
        ));
        variables.insert(
            target.name.to_string(),
            zero_derivative_scratch_variable(artifact, target_index, branch_current_unknowns.len()),
        );
        return Ok(());
    }

    let value = emitter.lower(assignment.expr.id)?;
    for line in emitter.lines {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(&format!(
        "{indent}scratch.store_ad({target_index}, &{value});\n"
    ));
    variables.insert(
        target.name.to_string(),
        scratch_backed_variable(artifact, target_index, branch_current_unknowns.len(), false),
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_compact_conditional_noop_assignment(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_name: &str,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<bool, RustBackendError> {
    let Some((condition, store_expr, invert_condition)) =
        emitter.noop_conditional_assignment(expr, target_name)?
    else {
        return Ok(false);
    };

    let condition = emitter.lower_condition(condition)?;
    let scalar_value = emitter.zero_derivative_value_expr(store_expr)?;
    let branch = if scalar_value.is_none() {
        Some(emitter.lower_isolated_branch(store_expr)?)
    } else {
        None
    };
    for line in emitter.lines.drain(..) {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    if invert_condition {
        out.push_str(&format!("{indent}if !{condition} {{\n"));
    } else {
        out.push_str(&format!("{indent}if {condition} {{\n"));
    }
    let branch_indent = format!("{indent}    ");
    if let Some(value) = scalar_value {
        push_compact_scalar_store(out, &branch_indent, target_index, &value);
    } else if let Some(branch) = branch {
        push_compact_ad_store(out, &branch_indent, target_index, &branch);
    }
    out.push_str(&format!("{indent}}}\n"));

    Ok(true)
}

fn emit_compact_conditional_scalar_branch_assignment(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<bool, RustBackendError> {
    let Some(conditional) = emitter.mixed_scalar_conditional_assignment(expr)? else {
        return Ok(false);
    };

    let condition = emitter.lower_condition(conditional.condition)?;
    let ad_branch = emitter.lower_isolated_branch(conditional.ad_expr)?;
    for line in emitter.lines.drain(..) {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(&format!("{indent}if {condition} {{\n"));
    let branch_indent = format!("{indent}    ");
    if conditional.scalar_when_condition_true {
        push_compact_scalar_store(out, &branch_indent, target_index, &conditional.scalar_value);
    } else {
        push_compact_ad_store(out, &branch_indent, target_index, &ad_branch);
    }
    out.push_str(&format!("{indent}}} else {{\n"));
    if conditional.scalar_when_condition_true {
        push_compact_ad_store(out, &branch_indent, target_index, &ad_branch);
    } else {
        push_compact_scalar_store(out, &branch_indent, target_index, &conditional.scalar_value);
    }
    out.push_str(&format!("{indent}}}\n"));

    Ok(true)
}

fn push_compact_ad_store(
    out: &mut String,
    indent: &str,
    target_index: usize,
    branch: &CompactBranch,
) {
    for line in &branch.lines {
        push_indented_compact_line(out, indent, line);
    }
    if let Some(source_index) = compact_scratch_ad_value_index(&branch.value) {
        if source_index != target_index {
            push_compact_ad_copy(out, indent, target_index, source_index);
        }
        return;
    }
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.store_ad({target_index}, &{});", branch.value),
    );
}

fn push_compact_scalar_store(out: &mut String, indent: &str, target_index: usize, value: &str) {
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.values[{target_index}] = {value};"),
    );
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.node_derivatives[{target_index}] = [0.0; Instance::NODE_COUNT];"),
    );
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.branch_derivatives[{target_index}] = [0.0; Instance::BRANCH_COUNT];"),
    );
}

fn push_compact_ad_copy(out: &mut String, indent: &str, target_index: usize, source_index: usize) {
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.values[{target_index}] = scratch.values[{source_index}];"),
    );
    push_indented_compact_line(
        out,
        indent,
        &format!(
            "scratch.node_derivatives[{target_index}] = scratch.node_derivatives[{source_index}];"
        ),
    );
    push_indented_compact_line(
        out,
        indent,
        &format!(
            "scratch.branch_derivatives[{target_index}] = scratch.branch_derivatives[{source_index}];"
        ),
    );
}

fn compact_scratch_ad_value_index(value: &str) -> Option<usize> {
    value
        .strip_prefix("scratch.ad_value(")?
        .strip_suffix(')')?
        .parse()
        .ok()
}

fn scratch_backed_variable(
    artifact: &CanonicalIrArtifact,
    variable_index: usize,
    branch_axis_count: usize,
    reactive: bool,
) -> LoweredVariable {
    let mut derivatives = Vec::with_capacity(artifact.mir.nodes.len());
    let mut branch_derivatives = Vec::with_capacity(branch_axis_count);
    let mut reactive_derivatives = Vec::with_capacity(artifact.mir.nodes.len());
    let mut reactive_branch_derivatives = Vec::with_capacity(branch_axis_count);
    for node_index in 0..artifact.mir.nodes.len() {
        derivatives.push(format!(
            "scratch.node_derivatives[{variable_index}][{node_index}]"
        ));
        if reactive {
            reactive_derivatives.push(format!(
                "scratch.reactive_node_derivatives[{variable_index}][{node_index}]"
            ));
        } else {
            reactive_derivatives.push("0.0".to_string());
        }
    }
    for branch_index in 0..branch_axis_count {
        branch_derivatives.push(format!(
            "scratch.branch_derivatives[{variable_index}][{branch_index}]"
        ));
        if reactive {
            reactive_branch_derivatives.push(format!(
                "scratch.reactive_branch_derivatives[{variable_index}][{branch_index}]"
            ));
        } else {
            reactive_branch_derivatives.push("0.0".to_string());
        }
    }

    LoweredVariable {
        value: format!("scratch.values[{variable_index}]"),
        derivatives,
        branch_derivatives,
        has_reactive: false,
        reactive_value: if reactive {
            format!("scratch.reactive_values[{variable_index}]")
        } else {
            "0.0".to_string()
        },
        reactive_derivatives,
        reactive_branch_derivatives,
    }
}

fn zero_derivative_scratch_variable(
    artifact: &CanonicalIrArtifact,
    variable_index: usize,
    branch_axis_count: usize,
) -> LoweredVariable {
    LoweredVariable {
        value: format!("scratch.values[{variable_index}]"),
        derivatives: zero_derivative_vec(artifact.mir.nodes.len()),
        branch_derivatives: zero_derivative_vec(branch_axis_count),
        has_reactive: false,
        reactive_value: "0.0".to_string(),
        reactive_derivatives: zero_derivative_vec(artifact.mir.nodes.len()),
        reactive_branch_derivatives: zero_derivative_vec(branch_axis_count),
    }
}

struct CompactAdEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
    variables: &'a HashMap<String, LoweredVariable>,
    ddt_slots: &'a DdtSlots,
    branch_current_unknowns: &'a HashMap<String, usize>,
    emitted: HashMap<ExprId, String>,
    lines: Vec<String>,
}

impl CompactAdEmitter<'_> {
    fn lower(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(value) = self.emitted.get(&id) {
            return Ok(value.clone());
        }
        let kind = self.expression(id)?.kind.clone();
        if let Some(value) = self.inline_leaf(&kind)? {
            return Ok(value);
        }
        let base = format!("{}_ad_e{}", self.prefix, id.index());
        let rhs = match &kind {
            HirExprKind::Number { value, .. } => {
                format!("AdValue::constant({})", format_f64(*value))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str())?,
            HirExprKind::BranchAccess { access, pos, neg } => {
                if access == "I" {
                    return Err(self.unsupported(format!("branch access '{access}' in expression")));
                }
                format!(
                    "AdValue::voltage(ctx, &self.nodes, {}, {})",
                    compact_optional_node(self.node_index(pos.as_str())?),
                    compact_optional_node(
                        neg.as_deref()
                            .map(|node| self.node_index(node))
                            .transpose()?
                            .flatten()
                    )
                )
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access.as_str(), name.as_str())?
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    let condition = self.lower_condition(*operand)?;
                    format!("AdValue::constant(if !{condition} {{ 1.0 }} else {{ 0.0 }})")
                } else {
                    let operand = self.lower(*operand)?;
                    match op.as_str() {
                        "Neg" => format!("AdValue::neg({operand})"),
                        "Pos" => operand,
                        _ => return Err(self.unsupported(format!("unary operator {op}"))),
                    }
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if let Some(operator) = comparison_operator(op.as_str()) {
                    let condition = self.comparison_condition(operator, *left, *right)?;
                    format!("AdValue::constant(if {condition} {{ 1.0 }} else {{ 0.0 }})")
                } else if op.as_str() == "And" || op.as_str() == "Or" {
                    let condition = self.lower_condition(id)?;
                    format!("AdValue::constant(if {condition} {{ 1.0 }} else {{ 0.0 }})")
                } else if let Some(scalar) = self.scalar_binary_value(op.as_str(), *left, *right)? {
                    format!("AdValue::constant({scalar})")
                } else {
                    match op.as_str() {
                        "Add" => {
                            if self.is_numeric_zero(*left)? {
                                self.lower(*right)?
                            } else if self.is_numeric_zero(*right)? {
                                self.lower(*left)?
                            } else if self.same_ad_operand(*left, *right)? {
                                self.lower_scaled(*left, 2.0)?
                            } else if let Some(operand) = self.negated_ad_operand(*left)? {
                                if let Some(scalar) = self.scalar_constant(*right)? {
                                    self.lower_sub_from_scalar(scalar, operand)?
                                } else {
                                    let right = self.lower(*right)?;
                                    let operand = self.lower(operand)?;
                                    format!("AdValue::sub({right}, {operand})")
                                }
                            } else if let Some(operand) = self.negated_ad_operand(*right)? {
                                if let Some(scalar) = self.scalar_constant(*left)? {
                                    self.lower_sub_from_scalar(scalar, operand)?
                                } else {
                                    let left = self.lower(*left)?;
                                    let operand = self.lower(operand)?;
                                    format!("AdValue::sub({left}, {operand})")
                                }
                            } else if let Some(offset) = self.scalar_constant(*left)? {
                                self.lower_offset(*right, offset)?
                            } else if let Some(offset) = self.scalar_constant(*right)? {
                                self.lower_offset(*left, offset)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::add({left}, {right})")
                            }
                        }
                        "Sub" => {
                            if self.is_numeric_zero(*right)? {
                                self.lower(*left)?
                            } else if self.is_numeric_zero(*left)? {
                                self.lower_scaled(*right, -1.0)?
                            } else if self.same_ad_operand(*left, *right)? {
                                "AdValue::constant(0.0)".to_string()
                            } else if let Some(offset) = self.scalar_constant(*right)? {
                                self.lower_offset(*left, compact_scalar_negate(&offset))?
                            } else if let Some(scalar) = self.scalar_constant(*left)? {
                                self.lower_sub_from_scalar(scalar, *right)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::sub({left}, {right})")
                            }
                        }
                        "Mul" => {
                            if let Some(scale) = self.numeric_literal(*left)? {
                                self.lower_scaled(*right, scale)?
                            } else if let Some(scale) = self.numeric_literal(*right)? {
                                self.lower_scaled(*left, scale)?
                            } else if let Some(scale) = self.scalar_constant(*left)? {
                                self.lower_scaled_by_expr(*right, scale)?
                            } else if let Some(scale) = self.scalar_constant(*right)? {
                                self.lower_scaled_by_expr(*left, scale)?
                            } else if self.same_ad_operand(*left, *right)? {
                                let operand = self.lower(*left)?;
                                format!("AdValue::square({operand})")
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::mul({left}, {right})")
                            }
                        }
                        "Div" => {
                            if let Some(scale) = self.numeric_literal(*right)? {
                                self.lower_scaled(*left, 1.0 / scale)?
                            } else if let Some(scale) = self.scalar_constant(*right)? {
                                self.lower_scaled_by_expr(*left, format!("1.0 / ({scale})"))?
                            } else if let Some(scalar) = self.scalar_constant(*left)? {
                                self.lower_div_from_scalar(scalar, *right)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::div({left}, {right})")
                            }
                        }
                        _ => return Err(self.unsupported(format!("binary operator {op}"))),
                    }
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.lower_conditional(*condition, *then_expr, *else_expr)?,
            HirExprKind::SystemFunction { name, args } => {
                self.lower_system_function(name.as_str(), args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.lower_ddt(id, args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                let (expr, probe) = compact_ddx_operands(args.as_slice(), self)?;
                self.lower_ddx(expr, probe)?
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                "AdValue::constant(0.0)".to_string()
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.lower_intrinsic(name.as_str(), args.as_slice())?
            }
            HirExprKind::NoiseSource { .. } => "AdValue::constant(0.0)".to_string(),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.lower_ddt(id, &[*expr])?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { expr, probe },
            } => self.lower_ddx(*expr, *probe)?,
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { .. },
            } => return Err(self.unsupported("convergence-limited limexp operator")),
            other => return Err(self.unsupported(format!("expression kind {other:?}"))),
        };
        if rhs.len() <= 512 && !compact_kind_has_side_effect(&kind) {
            return Ok(rhs);
        }
        self.lines.push(format!("let {base}: AdValue = {rhs};"));
        self.emitted.insert(id, base.clone());
        Ok(base)
    }

    fn lower_conditional(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> Result<String, RustBackendError> {
        let condition = self.lower_condition(condition)?;
        let then_branch = self.lower_isolated_branch(then_expr)?;
        let else_branch = self.lower_isolated_branch(else_expr)?;
        Ok(compact_lazy_conditional(
            &condition,
            &then_branch.lines,
            &then_branch.value,
            &else_branch.lines,
            &else_branch.value,
        ))
    }

    fn lower_isolated_branch(&self, expr: ExprId) -> Result<CompactBranch, RustBackendError> {
        let mut branch = CompactAdEmitter {
            artifact: self.artifact,
            prefix: self.prefix,
            parameter_fields: self.parameter_fields,
            variables: self.variables,
            ddt_slots: self.ddt_slots,
            branch_current_unknowns: self.branch_current_unknowns,
            emitted: self.emitted.clone(),
            lines: Vec::new(),
        };
        let value = branch.lower(expr)?;
        Ok(CompactBranch {
            lines: branch.lines,
            value,
        })
    }

    fn noop_conditional_assignment(
        &self,
        expr: ExprId,
        target_name: &str,
    ) -> Result<Option<(ExprId, ExprId, bool)>, RustBackendError> {
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = self.expression(expr)?.kind.clone()
        else {
            return Ok(None);
        };

        if self.is_identifier_named(else_expr, target_name)? {
            Ok(Some((condition, then_expr, false)))
        } else if self.is_identifier_named(then_expr, target_name)? {
            Ok(Some((condition, else_expr, true)))
        } else {
            Ok(None)
        }
    }

    fn mixed_scalar_conditional_assignment(
        &self,
        expr: ExprId,
    ) -> Result<Option<MixedScalarConditional>, RustBackendError> {
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = self.expression(expr)?.kind.clone()
        else {
            return Ok(None);
        };

        match (
            self.zero_derivative_value_expr(then_expr)?,
            self.zero_derivative_value_expr(else_expr)?,
        ) {
            (Some(scalar_value), None) => Ok(Some(MixedScalarConditional {
                condition,
                scalar_value,
                ad_expr: else_expr,
                scalar_when_condition_true: true,
            })),
            (None, Some(scalar_value)) => Ok(Some(MixedScalarConditional {
                condition,
                scalar_value,
                ad_expr: then_expr,
                scalar_when_condition_true: false,
            })),
            _ => Ok(None),
        }
    }

    fn is_identifier_named(&self, expr: ExprId, expected: &str) -> Result<bool, RustBackendError> {
        Ok(matches!(
            &self.expression(expr)?.kind,
            HirExprKind::Identifier { name } if name.as_str() == expected
        ))
    }

    fn ad_identifier_index(&self, expr: ExprId) -> Result<Option<usize>, RustBackendError> {
        let HirExprKind::Identifier { name } = &self.expression(expr)?.kind else {
            return Ok(None);
        };
        let Some(variable) = self.variables.get(name.as_str()) else {
            return Ok(None);
        };
        if lowered_variable_has_zero_derivatives(variable) {
            return Ok(None);
        }
        self.variable_index(name.as_str())
    }

    fn expression(
        &self,
        id: ExprId,
    ) -> Result<&crate::canonical_ir::HirExpression, RustBackendError> {
        self.artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| self.internal(format!("expression {id} is outside MIR arena")))
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("AdValue::constant(self.params.{field})"))
        } else if let Some(variable) = self.variables.get(name) {
            if lowered_variable_has_zero_derivatives(variable) {
                return Ok(format!("AdValue::constant({})", variable.value));
            }
            let index = self
                .artifact
                .hir
                .variables
                .iter()
                .find(|variable| variable.name.as_str() == name)
                .map(|variable| usize::from(variable.id))
                .ok_or_else(|| self.unsupported(format!("unknown variable '{name}'")))?;
            Ok(format!("scratch.ad_value({index})"))
        } else {
            Err(self.unsupported(format!(
                "identifier '{name}' is not a parameter or scalar variable"
            )))
        }
    }

    fn inline_leaf(&self, kind: &HirExprKind) -> Result<Option<String>, RustBackendError> {
        match kind {
            HirExprKind::Number { value, .. } => {
                Ok(Some(format!("AdValue::constant({})", format_f64(*value))))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str()).map(Some),
            _ => Ok(None),
        }
    }

    fn numeric_literal(&self, id: ExprId) -> Result<Option<f64>, RustBackendError> {
        Ok(match &self.expression(id)?.kind {
            HirExprKind::Number { value, .. } => Some(*value),
            _ => None,
        })
    }

    fn is_numeric_zero(&self, id: ExprId) -> Result<bool, RustBackendError> {
        Ok(self
            .numeric_literal(id)?
            .map(|value| value == 0.0)
            .unwrap_or(false))
    }

    fn scalar_constant(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("self.params.{field}")))
                } else if let Some(variable) = self.variables.get(name.as_str()) {
                    Ok(lowered_variable_has_zero_derivatives(variable)
                        .then(|| variable.value.clone()))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    return self.zero_derivative_value_expr(id);
                }
                let Some(operand) = self.scalar_constant(operand)? else {
                    return Ok(None);
                };
                match op.as_str() {
                    "Pos" => Ok(Some(operand)),
                    "Neg" => Ok(Some(compact_scalar_negate(&operand))),
                    _ => Ok(None),
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some()
                    || op.as_str() == "And"
                    || op.as_str() == "Or"
                {
                    return self.zero_derivative_value_expr(id);
                }
                let Some(left) = self.scalar_constant(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.scalar_constant(right)? else {
                    return Ok(None);
                };
                let value = match op.as_str() {
                    "Add" => compact_scalar_add(&left, &right),
                    "Sub" => compact_scalar_sub(&left, &right),
                    "Mul" => compact_scalar_mul(&left, &right),
                    "Div" => compact_scalar_div(&left, &right),
                    _ => return Ok(None),
                };
                Ok(Some(value))
            }
            HirExprKind::Conditional { .. }
            | HirExprKind::SystemFunction { .. }
            | HirExprKind::NoiseSource { .. } => self.zero_derivative_value_expr(id),
            HirExprKind::Call { name, .. }
                if expr_is_intrinsic_name(name.as_str()) || is_noise_name(name.as_str()) =>
            {
                self.zero_derivative_value_expr(id)
            }
            _ => Ok(None),
        }
    }

    fn scalar_binary_value(
        &self,
        op: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        let Some(left) = self.scalar_constant(left)? else {
            return Ok(None);
        };
        let Some(right) = self.scalar_constant(right)? else {
            return Ok(None);
        };
        let value = match op {
            "Add" => compact_scalar_add(&left, &right),
            "Sub" => compact_scalar_sub(&left, &right),
            "Mul" => compact_scalar_mul(&left, &right),
            "Div" => compact_scalar_div(&left, &right),
            _ => return Ok(None),
        };
        Ok(Some(value))
    }

    fn zero_derivative_value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("self.params.{field}")))
                } else if let Some(variable) = self.variables.get(name.as_str()) {
                    Ok(lowered_variable_has_zero_derivatives(variable)
                        .then(|| variable.value.clone()))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::Unary { op, operand } => match op.as_str() {
                "Pos" => self.zero_derivative_value_expr(operand),
                "Neg" => Ok(self
                    .zero_derivative_value_expr(operand)?
                    .map(|operand| compact_scalar_negate(&operand))),
                "Not" => Ok(self
                    .condition_value_expr(operand)?
                    .map(|condition| format!("if !{condition} {{ 1.0 }} else {{ 0.0 }}"))),
                _ => Ok(None),
            },
            HirExprKind::Binary { op, left, right } => {
                if let Some(operator) = comparison_operator(op.as_str()) {
                    let left = self.value_expr(left)?;
                    let right = self.value_expr(right)?;
                    return Ok(match (left, right) {
                        (Some(left), Some(right)) => Some(format!(
                            "if ({left} {operator} {right}) {{ 1.0 }} else {{ 0.0 }}"
                        )),
                        _ => None,
                    });
                }
                if op.as_str() == "And" || op.as_str() == "Or" {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                let Some(left) = self.zero_derivative_value_expr(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.zero_derivative_value_expr(right)? else {
                    return Ok(None);
                };
                Ok(match op.as_str() {
                    "Add" => Some(compact_scalar_add(&left, &right)),
                    "Sub" => Some(compact_scalar_sub(&left, &right)),
                    "Mul" => Some(compact_scalar_mul(&left, &right)),
                    "Div" => Some(compact_scalar_div(&left, &right)),
                    _ => None,
                })
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let Some(condition) = self.condition_value_expr(condition)? else {
                    return Ok(None);
                };
                let Some(then_expr) = self.zero_derivative_value_expr(then_expr)? else {
                    return Ok(None);
                };
                let Some(else_expr) = self.zero_derivative_value_expr(else_expr)? else {
                    return Ok(None);
                };
                Ok(Some(format!(
                    "(if {condition} {{ {then_expr} }} else {{ {else_expr} }})"
                )))
            }
            HirExprKind::SystemFunction { name, args } => {
                self.zero_derivative_system_function_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.zero_derivative_intrinsic_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                Ok(Some("0.0".to_string()))
            }
            HirExprKind::NoiseSource { .. } => Ok(Some("0.0".to_string())),
            _ => Ok(None),
        }
    }

    fn zero_derivative_system_function_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("ctx.temperature()".to_string())
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.time".to_string())
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.multiplicity".to_string())
            }
            "$vt" | "$thermal_vt" => match args {
                [] => Some("ctx.thermal_voltage()".to_string()),
                [temperature] => self
                    .zero_derivative_value_expr(*temperature)?
                    .map(|temperature| format!("({temperature} * THERMAL_VOLTAGE_PER_K)")),
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => match args {
                [name] => Some(format_f64(self.simparam_default(*name)?)),
                [_, default] => self.zero_derivative_value_expr(*default)?,
                _ => {
                    return Err(self.unsupported(format!(
                        "$simparam expects one or two arguments, found {}",
                        args.len()
                    )));
                }
            },
            "$param_given" => {
                let index = self.param_given_index(args)?;
                Some(format!(
                    "if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }}"
                ))
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                Some("1.0".to_string())
            }
            _ => None,
        })
    }

    fn zero_derivative_intrinsic_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let arg = |index: usize, this: &Self| -> Result<Option<String>, RustBackendError> {
            let Some(id) = args.get(index).copied() else {
                return Err(this.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument {index}"
                )));
            };
            this.zero_derivative_value_expr(id)
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => {
                arg(0, self)?.map(|value| format!("{}.abs()", compact_f64_receiver(&value)))
            }
            "sqrt" => arg(0, self)?.map(|value| format!("{}.sqrt()", compact_f64_receiver(&value))),
            "exp" => arg(0, self)?.map(|value| format!("{}.exp()", compact_f64_receiver(&value))),
            "ln" | "log" => {
                arg(0, self)?.map(|value| format!("{}.ln()", compact_f64_receiver(&value)))
            }
            "log10" => {
                arg(0, self)?.map(|value| format!("{}.log10()", compact_f64_receiver(&value)))
            }
            "sin" => arg(0, self)?.map(|value| format!("{}.sin()", compact_f64_receiver(&value))),
            "cos" => arg(0, self)?.map(|value| format!("{}.cos()", compact_f64_receiver(&value))),
            "tan" => arg(0, self)?.map(|value| format!("{}.tan()", compact_f64_receiver(&value))),
            "sinh" => arg(0, self)?.map(|value| format!("{}.sinh()", compact_f64_receiver(&value))),
            "cosh" => arg(0, self)?.map(|value| format!("{}.cosh()", compact_f64_receiver(&value))),
            "tanh" => arg(0, self)?.map(|value| format!("{}.tanh()", compact_f64_receiver(&value))),
            "floor" => {
                arg(0, self)?.map(|value| format!("{}.floor()", compact_f64_receiver(&value)))
            }
            "ceil" => arg(0, self)?.map(|value| format!("{}.ceil()", compact_f64_receiver(&value))),
            "pow" => match (arg(0, self)?, arg(1, self)?) {
                (Some(base), Some(exponent)) => {
                    Some(format!("{}.powf({exponent})", compact_f64_receiver(&base)))
                }
                _ => None,
            },
            "min" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).min({right})")),
                _ => None,
            },
            "max" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).max({right})")),
                _ => None,
            },
            "hypot" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).hypot({right})")),
                _ => None,
            },
            "atan2" => match (arg(0, self)?, arg(1, self)?) {
                (Some(y), Some(x)) => Some(format!("({y}).atan2({x})")),
                _ => None,
            },
            _ => None,
        })
    }

    fn same_ad_operand(&self, left: ExprId, right: ExprId) -> Result<bool, RustBackendError> {
        if left == right {
            return Ok(true);
        }
        let left = &self.expression(left)?.kind;
        let right = &self.expression(right)?.kind;
        Ok(match (left, right) {
            (HirExprKind::Identifier { name: left }, HirExprKind::Identifier { name: right }) => {
                left == right && self.variables.contains_key(left.as_str())
            }
            (
                HirExprKind::BranchAccess {
                    access: left_access,
                    pos: left_pos,
                    neg: left_neg,
                },
                HirExprKind::BranchAccess {
                    access: right_access,
                    pos: right_pos,
                    neg: right_neg,
                },
            ) => {
                left_access == right_access
                    && left_access != "I"
                    && left_pos == right_pos
                    && left_neg == right_neg
            }
            (
                HirExprKind::NamedBranchAccess {
                    access: left_access,
                    name: left_name,
                },
                HirExprKind::NamedBranchAccess {
                    access: right_access,
                    name: right_name,
                },
            ) => left_access == right_access && left_name == right_name,
            _ => false,
        })
    }

    fn negated_ad_operand(&self, id: ExprId) -> Result<Option<ExprId>, RustBackendError> {
        let HirExprKind::Unary { op, operand } = self.expression(id)?.kind.clone() else {
            return Ok(None);
        };
        if op.as_str() != "Neg" || !self.reorderable_ad_operand(operand)? {
            return Ok(None);
        }
        Ok(Some(operand))
    }

    fn reorderable_ad_operand(&self, id: ExprId) -> Result<bool, RustBackendError> {
        Ok(match &self.expression(id)?.kind {
            HirExprKind::Identifier { name } => self.variables.contains_key(name.as_str()),
            HirExprKind::BranchAccess { access, .. } => access != "I",
            HirExprKind::NamedBranchAccess { .. } => true,
            _ => false,
        })
    }

    fn comparison_condition(
        &mut self,
        operator: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<String, RustBackendError> {
        let left = self.comparison_operand_value(left)?;
        let right = self.comparison_operand_value(right)?;
        Ok(format!("({left} {operator} {right})"))
    }

    fn comparison_operand_value(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(value) = self.value_expr(id)? {
            Ok(value)
        } else {
            let value = self.lower(id)?;
            Ok(format!("{value}.value"))
        }
    }

    fn lower_scaled(&mut self, operand: ExprId, scale: f64) -> Result<String, RustBackendError> {
        if scale == 1.0 {
            self.lower(operand)
        } else if scale == -1.0 {
            let operand = self.lower(operand)?;
            Ok(format!("AdValue::neg({operand})"))
        } else {
            self.lower_scaled_by_expr(operand, format_f64(scale))
        }
    }

    fn lower_scaled_by_expr(
        &mut self,
        operand: ExprId,
        scale: String,
    ) -> Result<String, RustBackendError> {
        let kind = self.expression(operand)?.kind.clone();
        if let HirExprKind::Binary { op, left, right } = kind {
            match op.as_str() {
                "Mul" => {
                    if let Some(inner_scale) = self.scalar_constant(left)? {
                        return self.lower_scaled_by_expr(
                            right,
                            compact_scalar_mul(&inner_scale, scale.as_str()),
                        );
                    }
                    if let Some(inner_scale) = self.scalar_constant(right)? {
                        return self.lower_scaled_by_expr(
                            left,
                            compact_scalar_mul(&inner_scale, scale.as_str()),
                        );
                    }
                    if self.same_ad_operand(left, right)? {
                        let operand = self.lower(left)?;
                        return Ok(format!(
                            "AdValue::scale(AdValue::square({operand}), {scale})"
                        ));
                    }
                }
                "Add" if self.same_ad_operand(left, right)? => {
                    return self.lower_scaled_by_expr(left, compact_scalar_mul("2.0", &scale));
                }
                "Sub" if self.same_ad_operand(left, right)? => {
                    return Ok("AdValue::constant(0.0)".to_string());
                }
                _ => {}
            }
        }
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::scale({operand}, {scale})"))
    }

    fn lower_offset(
        &mut self,
        operand: ExprId,
        offset: String,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::offset({operand}, {offset})"))
    }

    fn lower_sub_from_scalar(
        &mut self,
        scalar: String,
        operand: ExprId,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::sub_from_scalar({scalar}, {operand})"))
    }

    fn lower_div_from_scalar(
        &mut self,
        scalar: String,
        operand: ExprId,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::div_from_scalar({scalar}, {operand})"))
    }

    fn lower_named_branch_access(
        &self,
        access: &str,
        name: &str,
    ) -> Result<String, RustBackendError> {
        if access == "I" {
            if let Some(slot) = self.branch_current_unknowns.get(name) {
                return Ok(format!(
                    "AdValue::branch_current(ctx, &self.branches, {slot})"
                ));
            }
            return Err(self.unsupported(format!(
                "named branch current access '{name}' before a current contribution is available"
            )));
        }
        let branch = self
            .artifact
            .mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown named branch access '{name}'")))?;
        Ok(format!(
            "AdValue::voltage(ctx, &self.nodes, {}, {})",
            compact_optional_node(branch.pos_node.map(usize::from)),
            compact_optional_node(branch.neg_node.map(usize::from))
        ))
    }

    fn lower_condition(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match &kind {
            HirExprKind::Binary { op, left, right }
                if comparison_operator(op.as_str()).is_some() =>
            {
                let operator = comparison_operator(op.as_str()).expect("checked above");
                self.comparison_condition(operator, *left, *right)
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                let left = self.lower_condition(*left)?;
                let right = self.lower_condition(*right)?;
                Ok(format!("({left} && {right})"))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "Or" => {
                let left = self.lower_condition(*left)?;
                let right = self.lower_condition(*right)?;
                Ok(format!("({left} || {right})"))
            }
            HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
                let operand = self.lower_condition(*operand)?;
                Ok(format!("(!{operand})"))
            }
            _ => {
                if let Some(value) = self.value_expr(id)? {
                    Ok(format!("({value} != 0.0)"))
                } else {
                    let value = self.lower(id)?;
                    Ok(format!("({value}.value != 0.0)"))
                }
            }
        }
    }

    fn lower_intrinsic(&mut self, name: &str, args: &[ExprId]) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        if normalized == "pow" {
            let base = args
                .first()
                .copied()
                .ok_or_else(|| self.unsupported("intrinsic function 'pow' missing argument 0"))?;
            let exponent = args
                .get(1)
                .copied()
                .ok_or_else(|| self.unsupported("intrinsic function 'pow' missing argument 1"))?;
            if let Some(exponent) = self.scalar_constant(exponent)? {
                return Ok(format!("AdValue::powf({}, {exponent})", self.lower(base)?));
            }
            if let Some(base) = self.scalar_constant(base)? {
                return Ok(format!(
                    "AdValue::pow_from_scalar({base}, {})",
                    self.lower(exponent)?
                ));
            }
            return Ok(format!(
                "AdValue::pow({}, {})",
                self.lower(base)?,
                self.lower(exponent)?
            ));
        }
        if normalized == "min" || normalized == "max" {
            let left = args.first().copied().ok_or_else(|| {
                self.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument 0"
                ))
            })?;
            let right = args.get(1).copied().ok_or_else(|| {
                self.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument 1"
                ))
            })?;
            if let Some(left_scalar) = self.scalar_constant(left)? {
                return Ok(format!(
                    "AdValue::{normalized}_from_scalar({left_scalar}, {})",
                    self.lower(right)?
                ));
            }
            if let Some(right_scalar) = self.scalar_constant(right)? {
                return Ok(format!(
                    "AdValue::{normalized}_with_scalar({}, {right_scalar})",
                    self.lower(left)?
                ));
            }
        }
        let mut lower_arg = |index: usize| -> Result<String, RustBackendError> {
            args.get(index)
                .copied()
                .ok_or_else(|| {
                    self.unsupported(format!(
                        "intrinsic function '{normalized}' missing argument {index}"
                    ))
                })
                .and_then(|arg| self.lower(arg))
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => format!("AdValue::abs({})", lower_arg(0)?),
            "sqrt" => format!("AdValue::sqrt({})", lower_arg(0)?),
            "exp" => format!("AdValue::exp({})", lower_arg(0)?),
            "ln" | "log" => format!("AdValue::ln({})", lower_arg(0)?),
            "log10" => format!("AdValue::log10({})", lower_arg(0)?),
            "sin" => format!("AdValue::sin({})", lower_arg(0)?),
            "cos" => format!("AdValue::cos({})", lower_arg(0)?),
            "tan" => format!("AdValue::tan({})", lower_arg(0)?),
            "sinh" => format!("AdValue::sinh({})", lower_arg(0)?),
            "cosh" => format!("AdValue::cosh({})", lower_arg(0)?),
            "tanh" => format!("AdValue::tanh({})", lower_arg(0)?),
            "floor" => format!("AdValue::floor({})", lower_arg(0)?),
            "ceil" => format!("AdValue::ceil({})", lower_arg(0)?),
            "min" => format!("AdValue::min({}, {})", lower_arg(0)?, lower_arg(1)?),
            "max" => format!("AdValue::max({}, {})", lower_arg(0)?, lower_arg(1)?),
            "hypot" => format!("AdValue::hypot({}, {})", lower_arg(0)?, lower_arg(1)?),
            "atan2" => format!("AdValue::atan2({}, {})", lower_arg(0)?, lower_arg(1)?),
            _ => return Err(self.unsupported(format!("intrinsic function '{name}'"))),
        })
    }

    fn lower_system_function(
        &mut self,
        name: &str,
        args: &[ExprId],
    ) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(ctx.temperature())".to_string()
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(self.time)".to_string()
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(self.multiplicity)".to_string()
            }
            "$vt" | "$thermal_vt" => match args {
                [] => "AdValue::constant(ctx.thermal_voltage())".to_string(),
                [temperature] => {
                    let temperature = self.lower(*temperature)?;
                    format!("AdValue::scale({temperature}, THERMAL_VOLTAGE_PER_K)")
                }
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => self.lower_simparam(args)?,
            "$param_given" => {
                let index = self.param_given_index(args)?;
                format!("AdValue::constant(if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }})")
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                "AdValue::constant(1.0)".to_string()
            }
            _ => return Err(self.unsupported(format!("system function '{name}'"))),
        })
    }

    fn lower_simparam(&mut self, args: &[ExprId]) -> Result<String, RustBackendError> {
        match args {
            [name] => Ok(format!(
                "AdValue::constant({})",
                format_f64(self.simparam_default(*name)?)
            )),
            [_, default] => self.lower(*default),
            _ => Err(self.unsupported(format!(
                "$simparam expects one or two arguments, found {}",
                args.len()
            ))),
        }
    }

    fn lower_ddt(&mut self, id: ExprId, args: &[ExprId]) -> Result<String, RustBackendError> {
        let [operand] = args else {
            return Err(self.unsupported(format!("ddt expects one operand, found {}", args.len())));
        };
        let operand = self.lower(*operand)?;
        let slot = self.ddt_slots.slot_for(id).ok_or_else(|| {
            self.internal(format!("ddt expression {id} has no generated state slot"))
        })?;
        Ok(format!(
            "AdValue::ddt({operand}, self.ddt_jacobian(1.0), self.eval_ddt({slot}, {operand}.value))"
        ))
    }

    fn lower_ddx(&mut self, expr: ExprId, probe: ExprId) -> Result<String, RustBackendError> {
        let expr = self.lower(expr)?;
        let (pos, neg) = self.ddx_probe_nodes(probe)?;
        Ok(format!(
            "AdValue::constant(AdValue::ddx_projection(&{expr}, {}, {}))",
            compact_optional_node(pos),
            compact_optional_node(neg)
        ))
    }

    fn ddx_probe_nodes(
        &self,
        probe: ExprId,
    ) -> Result<(Option<usize>, Option<usize>), RustBackendError> {
        let expression = self.expression(probe)?;
        match &expression.kind {
            HirExprKind::BranchAccess { access, pos, neg } if access.as_str() != "I" => Ok((
                self.node_index(pos.as_str())?,
                neg.as_deref()
                    .map(|node| self.node_index(node))
                    .transpose()?
                    .flatten(),
            )),
            HirExprKind::NamedBranchAccess { access, name } if access.as_str() != "I" => {
                let branch = self
                    .artifact
                    .mir
                    .branches
                    .iter()
                    .find(|branch| branch.name.as_str() == name)
                    .ok_or_else(|| {
                        self.unsupported(format!("unknown named ddx probe branch '{name}'"))
                    })?;
                Ok((
                    branch.pos_node.map(usize::from),
                    branch.neg_node.map(usize::from),
                ))
            }
            other => Err(self.unsupported(format!(
                "ddx probe must be a voltage access, found {other:?}"
            ))),
        }
    }

    fn node_index(&self, name: &str) -> Result<Option<usize>, RustBackendError> {
        if name == "0"
            || self
                .artifact
                .mir
                .ground_nodes
                .iter()
                .any(|ground| ground.as_str() == name)
        {
            return Ok(None);
        }
        self.artifact
            .mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .map(|node| Some(usize::from(node.id)))
            .ok_or_else(|| self.unsupported(format!("unknown branch access node '{name}'")))
    }

    fn simparam_default(&self, name: ExprId) -> Result<f64, RustBackendError> {
        let expression = self.expression(name)?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return Ok(0.0);
        };
        Ok(match value.as_str() {
            "gmin" => 1.0e-12,
            "tnom" => 300.15,
            "simulatorVersion" => 1.0,
            _ => 0.0,
        })
    }

    fn param_given_index(&self, args: &[ExprId]) -> Result<usize, RustBackendError> {
        let [parameter] = args else {
            return Err(self.unsupported(format!(
                "$param_given expects one parameter argument, found {}",
                args.len()
            )));
        };
        let expression = self.expression(*parameter)?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return Err(self.unsupported("$param_given expects a parameter identifier"));
        };
        self.artifact
            .mir
            .parameters
            .iter()
            .position(|parameter| parameter.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown $param_given parameter '{name}'")))
    }

    fn value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("self.params.{field}")))
                } else if let Some(index) = self.variable_index(name.as_str())? {
                    Ok(Some(format!("scratch.values[{index}]")))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::BranchAccess { access, pos, neg } if access.as_str() != "I" => Ok(Some(
                self.branch_voltage_value(pos.as_str(), neg.as_deref())?,
            )),
            HirExprKind::NamedBranchAccess { access, name } => match access.as_str() {
                "I" => Ok(self
                    .branch_current_unknowns
                    .get(name.as_str())
                    .map(|slot| format!("ctx.branch_current(self.branches[{slot}])"))),
                _ => {
                    let branch = self
                        .artifact
                        .mir
                        .branches
                        .iter()
                        .find(|branch| branch.name.as_str() == name)
                        .ok_or_else(|| {
                            self.unsupported(format!("unknown named branch access '{name}'"))
                        })?;
                    Ok(Some(self.branch_ref_voltage_value(branch)?))
                }
            },
            HirExprKind::Unary { op, operand } => match op.as_str() {
                "Pos" => self.value_expr(operand),
                "Neg" => Ok(self
                    .value_expr(operand)?
                    .map(|operand| compact_scalar_negate(&operand))),
                "Not" => Ok(self
                    .condition_value_expr(operand)?
                    .map(|condition| format!("if !{condition} {{ 1.0 }} else {{ 0.0 }}"))),
                _ => Ok(None),
            },
            HirExprKind::Binary { op, left, right } => {
                if let Some(operator) = comparison_operator(op.as_str()) {
                    let left = self.value_expr(left)?;
                    let right = self.value_expr(right)?;
                    return Ok(match (left, right) {
                        (Some(left), Some(right)) => Some(format!(
                            "if ({left} {operator} {right}) {{ 1.0 }} else {{ 0.0 }}"
                        )),
                        _ => None,
                    });
                }
                if op.as_str() == "And" || op.as_str() == "Or" {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                let Some(left) = self.value_expr(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.value_expr(right)? else {
                    return Ok(None);
                };
                Ok(match op.as_str() {
                    "Add" => Some(compact_scalar_add(&left, &right)),
                    "Sub" => Some(compact_scalar_sub(&left, &right)),
                    "Mul" => Some(compact_scalar_mul(&left, &right)),
                    "Div" => Some(compact_scalar_div(&left, &right)),
                    _ => None,
                })
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let Some(condition) = self.condition_value_expr(condition)? else {
                    return Ok(None);
                };
                let Some(then_expr) = self.value_expr(then_expr)? else {
                    return Ok(None);
                };
                let Some(else_expr) = self.value_expr(else_expr)? else {
                    return Ok(None);
                };
                Ok(Some(format!(
                    "(if {condition} {{ {then_expr} }} else {{ {else_expr} }})"
                )))
            }
            HirExprKind::SystemFunction { name, args } => {
                self.system_function_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.intrinsic_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                Ok(Some("0.0".to_string()))
            }
            HirExprKind::NoiseSource { .. } => Ok(Some("0.0".to_string())),
            _ => Ok(None),
        }
    }

    fn condition_value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match &kind {
            HirExprKind::Binary { op, left, right }
                if comparison_operator(op.as_str()).is_some() =>
            {
                let operator = comparison_operator(op.as_str()).expect("checked above");
                let Some(left) = self.value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} {operator} {right})")))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                let Some(left) = self.condition_value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.condition_value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} && {right})")))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "Or" => {
                let Some(left) = self.condition_value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.condition_value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} || {right})")))
            }
            HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
                let Some(operand) = self.condition_value_expr(*operand)? else {
                    return Ok(None);
                };
                Ok(Some(format!("(!{operand})")))
            }
            _ => {
                let Some(value) = self.value_expr(id)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({value} != 0.0)")))
            }
        }
    }

    fn variable_index(&self, name: &str) -> Result<Option<usize>, RustBackendError> {
        if !self.variables.contains_key(name) {
            return Ok(None);
        }
        self.artifact
            .hir
            .variables
            .iter()
            .find(|variable| variable.name.as_str() == name)
            .map(|variable| Some(usize::from(variable.id)))
            .ok_or_else(|| self.unsupported(format!("unknown variable '{name}'")))
    }

    fn branch_voltage_value(
        &self,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<String, RustBackendError> {
        let pos = self.node_index(pos)?;
        let neg = neg.map(|node| self.node_index(node)).transpose()?.flatten();
        Ok(compact_voltage_value(pos, neg))
    }

    fn branch_ref_voltage_value(
        &self,
        branch: &crate::canonical_ir::MirBranch,
    ) -> Result<String, RustBackendError> {
        Ok(compact_voltage_value(
            branch.pos_node.map(usize::from),
            branch.neg_node.map(usize::from),
        ))
    }

    fn system_function_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("ctx.temperature()".to_string())
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.time".to_string())
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.multiplicity".to_string())
            }
            "$vt" | "$thermal_vt" => match args {
                [] => Some("ctx.thermal_voltage()".to_string()),
                [temperature] => self
                    .value_expr(*temperature)?
                    .map(|temperature| format!("({temperature} * THERMAL_VOLTAGE_PER_K)")),
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => match args {
                [name] => Some(format_f64(self.simparam_default(*name)?)),
                [_, default] => self.value_expr(*default)?,
                _ => {
                    return Err(self.unsupported(format!(
                        "$simparam expects one or two arguments, found {}",
                        args.len()
                    )));
                }
            },
            "$param_given" => {
                let index = self.param_given_index(args)?;
                Some(format!(
                    "if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }}"
                ))
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                Some("1.0".to_string())
            }
            _ => None,
        })
    }

    fn intrinsic_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let arg = |index: usize, this: &Self| -> Result<Option<String>, RustBackendError> {
            let Some(id) = args.get(index).copied() else {
                return Err(this.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument {index}"
                )));
            };
            this.value_expr(id)
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => {
                arg(0, self)?.map(|value| format!("{}.abs()", compact_f64_receiver(&value)))
            }
            "sqrt" => arg(0, self)?.map(|value| format!("{}.sqrt()", compact_f64_receiver(&value))),
            "exp" => arg(0, self)?.map(|value| format!("{}.exp()", compact_f64_receiver(&value))),
            "ln" | "log" => {
                arg(0, self)?.map(|value| format!("{}.ln()", compact_f64_receiver(&value)))
            }
            "log10" => {
                arg(0, self)?.map(|value| format!("{}.log10()", compact_f64_receiver(&value)))
            }
            "sin" => arg(0, self)?.map(|value| format!("{}.sin()", compact_f64_receiver(&value))),
            "cos" => arg(0, self)?.map(|value| format!("{}.cos()", compact_f64_receiver(&value))),
            "tan" => arg(0, self)?.map(|value| format!("{}.tan()", compact_f64_receiver(&value))),
            "sinh" => arg(0, self)?.map(|value| format!("{}.sinh()", compact_f64_receiver(&value))),
            "cosh" => arg(0, self)?.map(|value| format!("{}.cosh()", compact_f64_receiver(&value))),
            "tanh" => arg(0, self)?.map(|value| format!("{}.tanh()", compact_f64_receiver(&value))),
            "floor" => {
                arg(0, self)?.map(|value| format!("{}.floor()", compact_f64_receiver(&value)))
            }
            "ceil" => arg(0, self)?.map(|value| format!("{}.ceil()", compact_f64_receiver(&value))),
            "pow" => match (arg(0, self)?, arg(1, self)?) {
                (Some(base), Some(exponent)) => {
                    Some(format!("{}.powf({exponent})", compact_f64_receiver(&base)))
                }
                _ => None,
            },
            "min" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).min({right})")),
                _ => None,
            },
            "max" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).max({right})")),
                _ => None,
            },
            "hypot" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!("({left}).hypot({right})")),
                _ => None,
            },
            "atan2" => match (arg(0, self)?, arg(1, self)?) {
                (Some(y), Some(x)) => Some(format!("({y}).atan2({x})")),
                _ => None,
            },
            _ => None,
        })
    }

    fn expect_system_arity(
        &self,
        name: &str,
        args: &[ExprId],
        expected: usize,
    ) -> Result<(), RustBackendError> {
        if args.len() == expected {
            Ok(())
        } else {
            Err(self.unsupported(format!(
                "{name} expects {expected} argument(s), found {}",
                args.len()
            )))
        }
    }

    fn unsupported(&self, feature: impl Into<String>) -> RustBackendError {
        RustBackendError::unsupported(
            self.artifact.metadata.source_package.as_str(),
            self.artifact.mir.module_name.as_str(),
            feature,
        )
    }

    fn internal(&self, message: impl Into<String>) -> RustBackendError {
        RustBackendError::internal(
            self.artifact.metadata.source_package.as_str(),
            self.artifact.mir.module_name.as_str(),
            message,
        )
    }
}

fn compact_scalar_add(left: &str, right: &str) -> String {
    if left == "0.0" {
        right.to_string()
    } else if right == "0.0" {
        left.to_string()
    } else {
        format!("({left} + {right})")
    }
}

fn compact_scalar_sub(left: &str, right: &str) -> String {
    if right == "0.0" {
        left.to_string()
    } else if left == "0.0" {
        compact_scalar_negate(right)
    } else {
        format!("({left} - {right})")
    }
}

fn compact_scalar_negate(value: &str) -> String {
    if value == "0.0" {
        "0.0".to_string()
    } else {
        format!("(-{value})")
    }
}

fn compact_scalar_mul(left: &str, right: &str) -> String {
    if left == "0.0" || right == "0.0" {
        "0.0".to_string()
    } else if left == "1.0" {
        right.to_string()
    } else if right == "1.0" {
        left.to_string()
    } else if left == "-1.0" {
        compact_scalar_negate(right)
    } else if right == "-1.0" {
        compact_scalar_negate(left)
    } else {
        format!("({left} * {right})")
    }
}

fn compact_scalar_div(left: &str, right: &str) -> String {
    if left == "0.0" {
        "0.0".to_string()
    } else if right == "1.0" {
        left.to_string()
    } else {
        format!("({left} / {right})")
    }
}

fn compact_f64_receiver(value: &str) -> String {
    format!("(({value}) as f64)")
}

struct CompactBranch {
    lines: Vec<String>,
    value: String,
}

struct MixedScalarConditional {
    condition: ExprId,
    scalar_value: String,
    ad_expr: ExprId,
    scalar_when_condition_true: bool,
}

fn compact_lazy_conditional(
    condition: &str,
    then_lines: &[String],
    then_value: &str,
    else_lines: &[String],
    else_value: &str,
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("    if ");
    out.push_str(condition);
    out.push_str(" {\n");
    push_compact_branch(&mut out, then_lines, then_value);
    out.push_str("    } else {\n");
    push_compact_branch(&mut out, else_lines, else_value);
    out.push_str("    }\n");
    out.push('}');
    out
}

fn push_compact_branch(out: &mut String, lines: &[String], value: &str) {
    for line in lines {
        push_indented_compact_line(out, "        ", line);
    }
    push_indented_compact_line(out, "        ", value);
}

fn push_indented_compact_line(out: &mut String, indent: &str, text: &str) {
    for line in text.lines() {
        out.push_str(indent);
        out.push_str(line);
        out.push('\n');
    }
}

fn compact_optional_node(node: Option<usize>) -> String {
    node.map(|node| format!("Some({node})"))
        .unwrap_or_else(|| "None".to_string())
}

fn compact_voltage_value(pos: Option<usize>, neg: Option<usize>) -> String {
    let pos = pos
        .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
        .unwrap_or_else(|| "0.0".to_string());
    let neg = neg
        .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
        .unwrap_or_else(|| "0.0".to_string());
    compact_scalar_sub(&pos, &neg)
}

fn compact_kind_has_side_effect(kind: &HirExprKind) -> bool {
    match kind {
        HirExprKind::Call { name, .. } if is_ddt_name(name.as_str()) => true,
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => true,
        _ => false,
    }
}

fn compact_ddx_operands(
    args: &[ExprId],
    emitter: &CompactAdEmitter<'_>,
) -> Result<(ExprId, ExprId), RustBackendError> {
    match args {
        [expr, probe] => Ok((*expr, *probe)),
        _ => Err(emitter.unsupported(format!("ddx expects two operands, found {}", args.len()))),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_loop_statement(
    artifact: &CanonicalIrArtifact,
    loop_statement: &crate::canonical_ir::HirLoop,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, usize>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    if reactive && !reactive_liveness.loop_has_live_assignment(loop_statement) {
        return Ok(());
    }
    if !reactive && !transient_liveness.loop_has_live_value_assignment(loop_statement) {
        return Ok(());
    }

    let condition_prefix = format!("{prefix}_cond");
    let branch_currents = HashMap::new();
    let condition = lower_equation_expr_with_branch_currents(
        artifact,
        loop_statement.condition.id,
        &condition_prefix,
        parameter_fields,
        variables,
        ddt_slots,
        &branch_currents,
        branch_current_unknowns,
    )?;
    let guard = format!("{prefix}_loop_guard");
    out.push_str(&format!("{indent}let mut {guard}: usize = 0;\n"));
    out.push_str(&format!("{indent}while {{\n"));
    let inner_indent = format!("{indent}    ");
    for line in condition.lines {
        out.push_str(&inner_indent);
        out.push_str(&line);
        out.push('\n');
    }
    out.push_str(&format!("{inner_indent}{} != 0.0\n", condition.value));
    out.push_str(&format!("{indent}}} {{\n"));
    out.push_str(&format!("{inner_indent}{guard} += 1;\n"));
    out.push_str(&format!(
        "{inner_indent}assert!({guard} <= Self::MAX_ANALOG_LOOP_ITERATIONS, \"generated Verilog-A analog loop exceeded iteration guard\");\n"
    ));
    let body_prefix = format!("{prefix}_body");
    emit_statement_list(
        artifact,
        &loop_statement.body,
        parameter_fields,
        variable_fields,
        variables,
        out,
        ddt_slots,
        branch_current_unknowns,
        transient_liveness,
        reactive,
        reactive_liveness,
        &inner_indent,
        &body_prefix,
    )?;
    out.push_str(&format!("{indent}}}\n"));
    Ok(())
}

fn optional_node_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some(self.nodes[{}])", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "f64::NAN".to_string()
    } else if value == f64::INFINITY {
        "f64::INFINITY".to_string()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_string()
    } else {
        format!("{value:?}")
    }
}

fn is_zero_derivative(derivative: &str) -> bool {
    derivative.trim() == "0.0"
}

fn lowered_variable_has_zero_derivatives(variable: &LoweredVariable) -> bool {
    variable
        .derivatives
        .iter()
        .all(|value| is_zero_derivative(value))
        && variable
            .branch_derivatives
            .iter()
            .all(|value| is_zero_derivative(value))
        && (!variable.has_reactive
            || (is_zero_derivative(&variable.reactive_value)
                && variable
                    .reactive_derivatives
                    .iter()
                    .all(|value| is_zero_derivative(value))
                && variable
                    .reactive_branch_derivatives
                    .iter()
                    .all(|value| is_zero_derivative(value))))
}

fn is_inline_derivative_expr(derivative: &str) -> bool {
    let derivative = derivative.trim();
    is_zero_derivative(derivative)
        || derivative == "1.0"
        || derivative == "-1.0"
        || is_generated_scratch_derivative_access(derivative)
        || is_rust_identifier(derivative)
}

fn is_constant_derivative_expr(derivative: &str) -> bool {
    let derivative = derivative.trim();
    is_zero_derivative(derivative) || derivative == "1.0" || derivative == "-1.0"
}

fn is_generated_scratch_derivative_access(value: &str) -> bool {
    let value = value.trim();
    let has_known_prefix = value.starts_with("scratch.node_derivatives[")
        || value.starts_with("scratch.branch_derivatives[")
        || value.starts_with("scratch.reactive_node_derivatives[")
        || value.starts_with("scratch.reactive_branch_derivatives[");
    has_known_prefix
        && value.ends_with(']')
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '[' | ']'))
}

fn is_rust_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn zero_derivative_vec(count: usize) -> Vec<String> {
    (0..count).map(|_| "0.0".to_string()).collect()
}
