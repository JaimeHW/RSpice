use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{
    CanonicalIrArtifact, ExprId, HirAnalogOperator, HirExprKind, MirBranchRef,
};

use super::{RustBackendError, sanitize_identifier};

#[derive(Debug, Clone)]
pub struct LoweredExpr {
    pub lines: Vec<String>,
    pub value: String,
    pub derivatives: Vec<String>,
    pub branch_derivatives: Vec<String>,
    pub has_reactive: bool,
    pub reactive_value: String,
    pub reactive_derivatives: Vec<String>,
    pub reactive_branch_derivatives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoweredVariable {
    pub value: String,
    pub condition: Option<String>,
    pub derivatives: Vec<String>,
    pub branch_derivatives: Vec<String>,
    pub has_reactive: bool,
    pub reactive_value: String,
    pub reactive_derivatives: Vec<String>,
    pub reactive_branch_derivatives: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DdtSlots {
    slots: HashMap<ExprId, usize>,
    idt_slots: HashMap<ExprId, usize>,
}

impl DdtSlots {
    pub fn with_idt_slots(
        slots: HashMap<ExprId, usize>,
        idt_slots: HashMap<ExprId, usize>,
    ) -> Self {
        Self { slots, idt_slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn idt_len(&self) -> usize {
        self.idt_slots.len()
    }

    pub(crate) fn slot_for(&self, expr: ExprId) -> Option<usize> {
        self.slots.get(&expr).copied()
    }

    pub(crate) fn idt_slot_for(&self, expr: ExprId) -> Option<usize> {
        self.idt_slots.get(&expr).copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BranchCurrentSlot {
    pub slot: usize,
    pub sign: f64,
}

impl BranchCurrentSlot {
    pub(crate) const fn forward(slot: usize) -> Self {
        Self { slot, sign: 1.0 }
    }

    pub(crate) const fn reverse(slot: usize) -> Self {
        Self { slot, sign: -1.0 }
    }

    pub(crate) fn signed_value(self, value: String) -> String {
        if self.sign < 0.0 {
            format!("-({value})")
        } else {
            value
        }
    }
}

pub(crate) fn branch_pair_key(pos: Option<usize>, neg: Option<usize>) -> String {
    format!("\u{0}branch-pair:{pos:?}:{neg:?}")
}

pub(crate) fn lower_equation_expr_with_branch_currents(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        ExprMode::Transient,
        DerivativeEmission::Materialize,
    )
}

pub(crate) fn lower_assignment_expr_with_branch_currents(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        ExprMode::Transient,
        DerivativeEmission::Inline,
    )
}

pub(crate) fn lower_value_assignment_expr_with_branch_currents(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        ExprMode::Transient,
        DerivativeEmission::None,
    )
}

pub(crate) fn lower_reactive_expr_with_branch_currents(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        ExprMode::Reactive,
        DerivativeEmission::Materialize,
    )
}

pub(crate) fn lower_reactive_assignment_expr_with_branch_currents(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        ExprMode::Reactive,
        DerivativeEmission::Inline,
    )
}

fn lower_expr_with_variables(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_currents: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    mode: ExprMode,
    derivative_emission: DerivativeEmission,
) -> Result<LoweredExpr, RustBackendError> {
    let mut emitter = ExprEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_currents,
        branch_current_unknowns,
        mode,
        derivative_emission,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };
    let value = emitter.lower(expr)?;
    Ok(LoweredExpr {
        lines: emitter.lines,
        value: value.value,
        derivatives: value.derivatives,
        branch_derivatives: value.branch_derivatives,
        has_reactive: value.has_reactive,
        reactive_value: value.reactive_value,
        reactive_derivatives: value.reactive_derivatives,
        reactive_branch_derivatives: value.reactive_branch_derivatives,
    })
}

#[derive(Debug, Clone, Copy)]
enum ExprMode {
    Transient,
    Reactive,
}

#[derive(Debug, Clone, Copy)]
enum DerivativeEmission {
    Materialize,
    Inline,
    None,
}

#[derive(Debug, Clone)]
struct ExprValue {
    value: String,
    derivatives: Vec<String>,
    branch_derivatives: Vec<String>,
    has_reactive: bool,
    reactive_value: String,
    reactive_derivatives: Vec<String>,
    reactive_branch_derivatives: Vec<String>,
}

struct ExprEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
    variables: &'a HashMap<String, LoweredVariable>,
    ddt_slots: &'a DdtSlots,
    branch_currents: &'a HashMap<String, LoweredVariable>,
    branch_current_unknowns: &'a HashMap<String, BranchCurrentSlot>,
    mode: ExprMode,
    derivative_emission: DerivativeEmission,
    emitted: HashMap<ExprId, ExprValue>,
    lines: Vec<String>,
}

impl ExprEmitter<'_> {
    fn lower(&mut self, id: ExprId) -> Result<ExprValue, RustBackendError> {
        if let Some(value) = self.emitted.get(&id) {
            return Ok(value.clone());
        }

        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| self.internal(format!("expression {id} is outside MIR arena")))?;
        let node_count = self.artifact.mir.nodes.len();
        let branch_axis_count = branch_derivative_axis_count(self.branch_current_unknowns);
        let base = format!("{}_e{}", self.prefix, id.index());

        if let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        {
            let lowered = self.lower_conditional(id, *condition, *then_expr, *else_expr)?;
            self.emitted.insert(id, lowered.clone());
            return Ok(lowered);
        }

        if let HirExprKind::Binary { op, left, right } = &expression.kind
            && op.as_str() == "Mul"
        {
            let left_has_side_effects =
                self.expression_has_side_effects(*left, &mut HashSet::new())?;
            let right_has_side_effects =
                self.expression_has_side_effects(*right, &mut HashSet::new())?;
            if !left_has_side_effects
                && !right_has_side_effects
                && (self.expression_value_is_known_zero(*left, &mut HashSet::new())?
                    || self.expression_value_is_known_zero(*right, &mut HashSet::new())?)
            {
                let lowered = self.zero_value();
                self.emitted.insert(id, lowered.clone());
                return Ok(lowered);
            }
        }

        let value_expr = match &expression.kind {
            HirExprKind::Number { value, .. } => format_f64(*value),
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str())?,
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.lower_branch_access(access.as_str(), pos.as_str(), neg.as_deref())?
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access.as_str(), name.as_str())?
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    let condition = self.lower_condition(*operand)?;
                    self.emit_value(
                        &base,
                        format!(
                            "if {} {{ 1.0 }} else {{ 0.0 }}",
                            negate_condition(&condition)
                        ),
                    )
                } else {
                    let operand = self.lower(*operand)?;
                    let value = unary_value(op.as_str(), &operand.value)
                        .map_err(|_| self.unsupported(format!("unary operator {op}")))?;
                    self.emit_value(&base, value)
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if let Some(operator) = comparison_operator(op.as_str()) {
                    let left = self.lower(*left)?;
                    let right = self.lower(*right)?;
                    self.emit_value(
                        &base,
                        format!(
                            "if {} {operator} {} {{ 1.0 }} else {{ 0.0 }}",
                            left.value, right.value
                        ),
                    )
                } else if op.as_str() == "And" || op.as_str() == "Or" {
                    let condition = self.lower_condition(id)?;
                    self.emit_value(&base, format!("if {condition} {{ 1.0 }} else {{ 0.0 }}"))
                } else {
                    let left = self.lower(*left)?;
                    let right = self.lower(*right)?;
                    let value = binary_value(op.as_str(), &left.value, &right.value)
                        .map_err(|_| self.unsupported(format!("binary operator {op}")))?;
                    self.emit_value(&base, value)
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.lower_condition(*condition)?;
                let then_value = self.lower(*then_expr)?;
                let else_value = self.lower(*else_expr)?;
                self.emit_value(
                    &base,
                    conditional_expr(&condition, &then_value.value, &else_value.value),
                )
            }
            HirExprKind::SystemFunction { name, args } => {
                self.lower_system_function_value(name.as_str(), args.as_slice(), &base)?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.lower_ddt_value(id, args.as_slice(), &base)?
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                let (expr, ic) = self.idt_operands(args.as_slice())?;
                self.lower_idt_value(id, expr, ic, &base)?
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                self.lower_analysis_value(args.as_slice(), &base)?
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                let (expr, probe) = self.ddx_operands(args.as_slice())?;
                self.lower_ddx_value(expr, probe, &base)?
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => "0.0".to_string(),
            HirExprKind::Call { name, args } if is_intrinsic_name(name.as_str()) => {
                self.lower_intrinsic_value(name.as_str(), args.as_slice(), &base)?
            }
            HirExprKind::NoiseSource { .. } => "0.0".to_string(),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.lower_ddt_value(id, &[*expr], &base)?
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        ic,
                        assert,
                        abstol,
                    },
            } => {
                if assert.is_some() || abstol.is_some() {
                    return Err(self.unsupported("idt assert/abstol argument"));
                }
                self.lower_idt_value(id, *expr, *ic, &base)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { expr, probe },
            } => self.lower_ddx_value(*expr, *probe, &base)?,
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let arg = self.lower(*expr)?;
                self.emit_value(&base, limexp_value_expr(&arg.value))
            }
            other => {
                return Err(self.unsupported(format!("expression kind {other:?}")));
            }
        };

        if matches!(self.derivative_emission, DerivativeEmission::None) {
            let lowered = ExprValue {
                value: value_expr,
                derivatives: zero_derivatives(node_count),
                branch_derivatives: zero_derivatives(branch_axis_count),
                has_reactive: false,
                reactive_value: "0.0".to_string(),
                reactive_derivatives: zero_derivatives(node_count),
                reactive_branch_derivatives: zero_derivatives(branch_axis_count),
            };
            self.emitted.insert(id, lowered.clone());
            return Ok(lowered);
        }

        let derivatives = match &expression.kind {
            HirExprKind::Number { .. } => zero_derivatives(node_count),
            HirExprKind::Identifier { name } => {
                if let Some(variable) = self.variables.get(name.as_str()) {
                    variable.derivatives.clone()
                } else {
                    zero_derivatives(node_count)
                }
            }
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.branch_access_derivatives(access.as_str(), pos.as_str(), neg.as_deref())?
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                if access.as_str() == "I" {
                    if let Some(current) = self.branch_currents.get(name.as_str()) {
                        current.derivatives.clone()
                    } else if self.branch_current_unknowns.contains_key(name.as_str()) {
                        zero_derivatives(node_count)
                    } else if matches!(self.mode, ExprMode::Reactive) {
                        zero_derivatives(node_count)
                    } else {
                        return Err(self.unsupported(format!(
                            "named branch current access '{name}' before a current contribution is available"
                        )));
                    }
                } else {
                    let branch = self
                        .artifact
                        .mir
                        .branches
                        .iter()
                        .find(|branch| branch.name.as_str() == name)
                        .ok_or_else(|| {
                            self.unsupported(format!("unknown named branch access '{name}'"))
                        })?;
                    self.branch_ref_derivatives(
                        access,
                        &MirBranchRef {
                            label: branch.name.clone(),
                            declared_name: Some(branch.name.clone()),
                            pos_node: branch.pos_node,
                            neg_node: branch.neg_node,
                        },
                    )?
                }
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    zero_derivatives(node_count)
                } else {
                    let operand = self
                        .emitted
                        .get(operand)
                        .expect("operand must be emitted before unary derivative");
                    operand
                        .derivatives
                        .iter()
                        .map(|derivative| {
                            unary_value(op.as_str(), derivative)
                                .map_err(|_| self.unsupported(format!("unary operator {op}")))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some()
                    || op.as_str() == "And"
                    || op.as_str() == "Or"
                {
                    zero_derivatives(node_count)
                } else {
                    let left = self
                        .emitted
                        .get(left)
                        .expect("left operand must be emitted before binary derivative");
                    let right = self
                        .emitted
                        .get(right)
                        .expect("right operand must be emitted before binary derivative");
                    binary_derivatives(op.as_str(), &value_expr, left, right)
                        .map_err(|_| self.unsupported(format!("binary operator {op}")))?
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.lower_condition(*condition)?;
                let then_value = self
                    .emitted
                    .get(then_expr)
                    .expect("then expression must be emitted before conditional derivative");
                let else_value = self
                    .emitted
                    .get(else_expr)
                    .expect("else expression must be emitted before conditional derivative");
                then_value
                    .derivatives
                    .iter()
                    .zip(&else_value.derivatives)
                    .map(|(then_derivative, else_derivative)| {
                        conditional_expr(&condition, then_derivative, else_derivative)
                    })
                    .collect()
            }
            HirExprKind::SystemFunction { name, args } => {
                self.system_function_derivatives(name.as_str(), args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.ddt_derivatives(id, args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                let (expr, _) = self.idt_operands(args.as_slice())?;
                self.idt_derivatives(expr)?
            }
            HirExprKind::Call { name, .. } if is_analysis_name(name.as_str()) => {
                zero_derivatives(node_count)
            }
            HirExprKind::Call { name, .. } if is_ddx_name(name.as_str()) => {
                zero_derivatives(node_count)
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                zero_derivatives(node_count)
            }
            HirExprKind::Call { name, args } if is_intrinsic_name(name.as_str()) => {
                self.intrinsic_derivatives(name.as_str(), args.as_slice(), &value_expr)?
            }
            HirExprKind::NoiseSource { .. } => zero_derivatives(node_count),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.ddt_derivatives(id, &[*expr])?
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        assert,
                        abstol,
                        ..
                    },
            } => {
                if assert.is_some() || abstol.is_some() {
                    return Err(self.unsupported("idt assert/abstol argument"));
                }
                self.idt_derivatives(*expr)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { .. },
            } => zero_derivatives(node_count),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let arg = self
                    .emitted
                    .get(expr)
                    .expect("limexp operand must be emitted before derivative");
                arg.derivatives
                    .iter()
                    .map(|d| mul_expr(&limexp_derivative_scale_expr(&arg.value), d))
                    .collect()
            }
            _ => unreachable!("unsupported expression kinds returned earlier"),
        };

        let branch_derivatives = match &expression.kind {
            HirExprKind::Number { .. } => zero_derivatives(branch_axis_count),
            HirExprKind::Identifier { name } => {
                if let Some(variable) = self.variables.get(name.as_str()) {
                    variable.branch_derivatives.clone()
                } else {
                    zero_derivatives(branch_axis_count)
                }
            }
            HirExprKind::BranchAccess { access, .. } => {
                if access == "I" {
                    if let HirExprKind::BranchAccess { pos, neg, .. } = &expression.kind
                        && let Some(slot) =
                            self.branch_current_slot_for_nodes(pos.as_str(), neg.as_deref())?
                    {
                        let mut derivatives = zero_derivatives(branch_axis_count);
                        derivatives[slot.slot] = format_f64(slot.sign);
                        derivatives
                    } else {
                        return Err(
                            self.unsupported(format!("branch access '{access}' in expression"))
                        );
                    }
                } else {
                    zero_derivatives(branch_axis_count)
                }
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                if access.as_str() == "I" {
                    if let Some(current) = self.branch_currents.get(name.as_str()) {
                        current.branch_derivatives.clone()
                    } else if let Some(slot) = self.branch_current_unknowns.get(name.as_str()) {
                        let mut derivatives = zero_derivatives(branch_axis_count);
                        derivatives[slot.slot] = format_f64(slot.sign);
                        derivatives
                    } else if matches!(self.mode, ExprMode::Reactive) {
                        zero_derivatives(branch_axis_count)
                    } else {
                        return Err(self.unsupported(format!(
                            "named branch current access '{name}' before a current contribution is available"
                        )));
                    }
                } else {
                    zero_derivatives(branch_axis_count)
                }
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    zero_derivatives(branch_axis_count)
                } else {
                    let operand = self
                        .emitted
                        .get(operand)
                        .expect("operand must be emitted before unary branch derivative");
                    operand
                        .branch_derivatives
                        .iter()
                        .map(|derivative| {
                            unary_value(op.as_str(), derivative)
                                .map_err(|_| self.unsupported(format!("unary operator {op}")))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some()
                    || op.as_str() == "And"
                    || op.as_str() == "Or"
                {
                    zero_derivatives(branch_axis_count)
                } else {
                    let left = self
                        .emitted
                        .get(left)
                        .expect("left operand must be emitted before binary branch derivative");
                    let right = self
                        .emitted
                        .get(right)
                        .expect("right operand must be emitted before binary branch derivative");
                    binary_derivatives_for(
                        op.as_str(),
                        &value_expr,
                        &left.value,
                        &right.value,
                        &left.branch_derivatives,
                        &right.branch_derivatives,
                    )
                    .map_err(|_| self.unsupported(format!("binary operator {op}")))?
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.lower_condition(*condition)?;
                let then_value = self
                    .emitted
                    .get(then_expr)
                    .expect("then expression must be emitted before conditional branch derivative");
                let else_value = self
                    .emitted
                    .get(else_expr)
                    .expect("else expression must be emitted before conditional branch derivative");
                then_value
                    .branch_derivatives
                    .iter()
                    .zip(&else_value.branch_derivatives)
                    .map(|(then_derivative, else_derivative)| {
                        conditional_expr(&condition, then_derivative, else_derivative)
                    })
                    .collect()
            }
            HirExprKind::SystemFunction { name, args } => {
                self.system_function_branch_derivatives(name.as_str(), args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.ddt_branch_derivatives(args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                let (expr, _) = self.idt_operands(args.as_slice())?;
                self.idt_branch_derivatives(expr)?
            }
            HirExprKind::Call { name, .. } if is_analysis_name(name.as_str()) => {
                zero_derivatives(branch_axis_count)
            }
            HirExprKind::Call { name, .. } if is_ddx_name(name.as_str()) => {
                zero_derivatives(branch_axis_count)
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                zero_derivatives(branch_axis_count)
            }
            HirExprKind::Call { name, args } if is_intrinsic_name(name.as_str()) => {
                self.intrinsic_branch_derivatives(name.as_str(), args.as_slice(), &value_expr)?
            }
            HirExprKind::NoiseSource { .. } => zero_derivatives(branch_axis_count),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.ddt_branch_derivatives(&[*expr])?
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        assert,
                        abstol,
                        ..
                    },
            } => {
                if assert.is_some() || abstol.is_some() {
                    return Err(self.unsupported("idt assert/abstol argument"));
                }
                self.idt_branch_derivatives(*expr)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { .. },
            } => zero_derivatives(branch_axis_count),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let arg = self
                    .emitted
                    .get(expr)
                    .expect("limexp operand must be emitted before branch derivative");
                arg.branch_derivatives
                    .iter()
                    .map(|d| mul_expr(&limexp_derivative_scale_expr(&arg.value), d))
                    .collect()
            }
            _ => unreachable!("unsupported expression kinds returned earlier"),
        };

        let reactive = if matches!(self.mode, ExprMode::Reactive) {
            match &expression.kind {
                HirExprKind::Number { .. } | HirExprKind::BranchAccess { .. } => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::NamedBranchAccess { access, name } => {
                    if access.as_str() == "I" {
                        if let Some(current) = self.branch_currents.get(name.as_str()) {
                            ReactiveValue {
                                has_reactive: current.has_reactive,
                                value: current.reactive_value.clone(),
                                derivatives: current.reactive_derivatives.clone(),
                                branch_derivatives: current.reactive_branch_derivatives.clone(),
                            }
                        } else {
                            ReactiveValue::none(zero_derivatives(node_count))
                        }
                    } else {
                        ReactiveValue::none(zero_derivatives(node_count))
                    }
                }
                HirExprKind::Identifier { name } => {
                    if let Some(variable) = self.variables.get(name.as_str()) {
                        ReactiveValue {
                            has_reactive: variable.has_reactive,
                            value: variable.reactive_value.clone(),
                            derivatives: variable.reactive_derivatives.clone(),
                            branch_derivatives: variable.reactive_branch_derivatives.clone(),
                        }
                    } else {
                        ReactiveValue::none(zero_derivatives(node_count))
                    }
                }
                HirExprKind::Unary { op, operand } => {
                    if op.as_str() == "Not" {
                        ReactiveValue::none(zero_derivatives(node_count))
                    } else {
                        let operand = self
                            .emitted
                            .get(operand)
                            .expect("operand must be emitted before unary reactive derivative");
                        reactive_unary(op.as_str(), operand)
                            .map_err(|_| self.unsupported(format!("unary operator {op}")))?
                    }
                }
                HirExprKind::Binary { op, left, right } => {
                    if comparison_operator(op.as_str()).is_some()
                        || op.as_str() == "And"
                        || op.as_str() == "Or"
                    {
                        ReactiveValue::none(zero_derivatives(node_count))
                    } else {
                        let left = self.emitted.get(left).expect(
                            "left operand must be emitted before binary reactive derivative",
                        );
                        let right = self.emitted.get(right).expect(
                            "right operand must be emitted before binary reactive derivative",
                        );
                        reactive_binary(op.as_str(), left, right)
                            .map_err(|_| self.unsupported(format!("binary operator {op}")))?
                    }
                }
                HirExprKind::Conditional {
                    condition,
                    then_expr,
                    else_expr,
                } => {
                    let condition = self.lower_condition(*condition)?;
                    let then_value = self.emitted.get(then_expr).expect(
                        "then expression must be emitted before conditional reactive derivative",
                    );
                    let else_value = self.emitted.get(else_expr).expect(
                        "else expression must be emitted before conditional reactive derivative",
                    );
                    let has_reactive = then_value.has_reactive || else_value.has_reactive;
                    if has_reactive {
                        let zero_node_derivatives = zero_derivatives(node_count);
                        let then_derivatives = if then_value.has_reactive {
                            &then_value.reactive_derivatives
                        } else {
                            &zero_node_derivatives
                        };
                        let else_derivatives = if else_value.has_reactive {
                            &else_value.reactive_derivatives
                        } else {
                            &zero_node_derivatives
                        };
                        let zero_branch_derivatives = zero_derivatives(branch_axis_count);
                        let then_branch_derivatives = if then_value.has_reactive {
                            &then_value.reactive_branch_derivatives
                        } else {
                            &zero_branch_derivatives
                        };
                        let else_branch_derivatives = if else_value.has_reactive {
                            &else_value.reactive_branch_derivatives
                        } else {
                            &zero_branch_derivatives
                        };
                        ReactiveValue {
                            has_reactive: true,
                            value: format!(
                                "if {condition} {{ {} }} else {{ {} }}",
                                then_value.reactive_value, else_value.reactive_value
                            ),
                            derivatives: then_derivatives
                                .iter()
                                .zip(else_derivatives)
                                .map(|(then_derivative, else_derivative)| {
                                    conditional_expr(&condition, then_derivative, else_derivative)
                                })
                                .collect(),
                            branch_derivatives: then_branch_derivatives
                                .iter()
                                .zip(else_branch_derivatives)
                                .map(|(then_derivative, else_derivative)| {
                                    conditional_expr(&condition, then_derivative, else_derivative)
                                })
                                .collect(),
                        }
                    } else {
                        ReactiveValue::none(zero_derivatives(node_count))
                    }
                }
                HirExprKind::SystemFunction { name, args } => {
                    self.system_function_reactive_value(name.as_str(), args.as_slice())?
                }
                HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                    self.ddt_reactive_value(args.as_slice())?
                }
                HirExprKind::Call { name, .. } if is_idt_name(name.as_str()) => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::Call { name, .. } if is_ddx_name(name.as_str()) => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::Call { name, .. } if is_analysis_name(name.as_str()) => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::Call { name, args } if is_intrinsic_name(name.as_str()) => {
                    self.intrinsic_reactive_value(args.as_slice())?
                }
                HirExprKind::NoiseSource { .. } => {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Ddt { expr, abstol },
                } => {
                    if abstol.is_some() {
                        return Err(self.unsupported("ddt abstol argument"));
                    }
                    self.ddt_reactive_value(&[*expr])?
                }
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Idt { .. },
                } => ReactiveValue::none(zero_derivatives(node_count)),
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Ddx { .. },
                } => ReactiveValue::none(zero_derivatives(node_count)),
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Limexp { expr },
                } => self.intrinsic_reactive_value(&[*expr])?,
                _ => unreachable!("unsupported expression kinds returned earlier"),
            }
        } else {
            ReactiveValue::none(zero_derivatives(node_count))
        };

        let mut derivative_aliases = HashMap::new();
        let mut derivative_vars = Vec::with_capacity(derivatives.len());
        for (index, derivative) in derivatives.into_iter().enumerate() {
            derivative_vars.push(materialize_derivative_expr(
                &mut self.lines,
                format!("{base}_d_n{index}"),
                derivative,
                self.derivative_emission,
                &mut derivative_aliases,
            ));
        }
        let mut branch_derivative_vars = Vec::with_capacity(branch_derivatives.len());
        for (index, derivative) in branch_derivatives.into_iter().enumerate() {
            branch_derivative_vars.push(materialize_derivative_expr(
                &mut self.lines,
                format!("{base}_d_b{index}"),
                derivative,
                self.derivative_emission,
                &mut derivative_aliases,
            ));
        }

        let (reactive_value, reactive_derivatives, reactive_branch_derivatives) = if reactive
            .has_reactive
        {
            let value_var = format!("{base}_q");
            self.lines
                .push(format!("let {value_var}: f64 = {};", reactive.value));
            let mut derivative_vars = Vec::with_capacity(reactive.derivatives.len());
            for (index, derivative) in reactive.derivatives.into_iter().enumerate() {
                derivative_vars.push(materialize_derivative_expr(
                    &mut self.lines,
                    format!("{base}_q_d_n{index}"),
                    derivative,
                    self.derivative_emission,
                    &mut derivative_aliases,
                ));
            }
            let mut branch_derivative_vars = Vec::with_capacity(reactive.branch_derivatives.len());
            for (index, derivative) in reactive.branch_derivatives.into_iter().enumerate() {
                branch_derivative_vars.push(materialize_derivative_expr(
                    &mut self.lines,
                    format!("{base}_q_d_b{index}"),
                    derivative,
                    self.derivative_emission,
                    &mut derivative_aliases,
                ));
            }
            (value_var, derivative_vars, branch_derivative_vars)
        } else {
            (
                "0.0".to_string(),
                zero_derivatives(node_count),
                zero_derivatives(branch_axis_count),
            )
        };

        let lowered = ExprValue {
            value: value_expr,
            derivatives: derivative_vars,
            branch_derivatives: branch_derivative_vars,
            has_reactive: reactive.has_reactive,
            reactive_value,
            reactive_derivatives,
            reactive_branch_derivatives,
        };
        self.emitted.insert(id, lowered.clone());
        Ok(lowered)
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("params.{field}"))
        } else if let Some(variable) = self.variables.get(name) {
            Ok(variable.value.clone())
        } else {
            Err(self.unsupported(format!(
                "identifier '{name}' is not a parameter or scalar variable"
            )))
        }
    }

    fn zero_value(&self) -> ExprValue {
        let node_count = self.artifact.mir.nodes.len();
        let branch_axis_count = branch_derivative_axis_count(self.branch_current_unknowns);
        ExprValue {
            value: "0.0".to_string(),
            derivatives: zero_derivatives(node_count),
            branch_derivatives: zero_derivatives(branch_axis_count),
            has_reactive: false,
            reactive_value: "0.0".to_string(),
            reactive_derivatives: zero_derivatives(node_count),
            reactive_branch_derivatives: zero_derivatives(branch_axis_count),
        }
    }

    fn expression_value_is_known_zero(
        &self,
        id: ExprId,
        visited: &mut HashSet<ExprId>,
    ) -> Result<bool, RustBackendError> {
        if !visited.insert(id) {
            return Ok(false);
        }
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| self.internal(format!("expression {id} is outside MIR arena")))?;

        match &expression.kind {
            HirExprKind::Number { value, .. } => Ok(*value == 0.0),
            HirExprKind::NoiseSource { .. } => Ok(true),
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => Ok(true),
            HirExprKind::Identifier { name } => Ok(self
                .variables
                .get(name.as_str())
                .is_some_and(lowered_variable_is_constant_zero)),
            HirExprKind::NamedBranchAccess { access, name } if access.as_str() == "I" => Ok(self
                .branch_currents
                .get(name.as_str())
                .is_some_and(lowered_variable_is_constant_zero)),
            HirExprKind::Unary { op, operand } if matches!(op.as_str(), "Pos" | "Neg") => {
                self.expression_value_is_known_zero(*operand, visited)
            }
            HirExprKind::Binary { op, left, right } if matches!(op.as_str(), "Add" | "Sub") => Ok(
                self.expression_value_is_known_zero(*left, &mut HashSet::new())?
                    && self.expression_value_is_known_zero(*right, &mut HashSet::new())?,
            ),
            HirExprKind::Binary { op, left, right } if op.as_str() == "Mul" => Ok(self
                .expression_value_is_known_zero(*left, &mut HashSet::new())?
                || self.expression_value_is_known_zero(*right, &mut HashSet::new())?),
            HirExprKind::Binary { op, left, .. } if matches!(op.as_str(), "Div" | "Mod") => {
                self.expression_value_is_known_zero(*left, &mut HashSet::new())
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } if !self.expression_has_side_effects(*condition, &mut HashSet::new())? => Ok(self
                .expression_value_is_known_zero(*then_expr, visited)?
                && self.expression_value_is_known_zero(*else_expr, &mut HashSet::new())?),
            _ => Ok(false),
        }
    }

    fn expression_has_side_effects(
        &self,
        id: ExprId,
        visited: &mut HashSet<ExprId>,
    ) -> Result<bool, RustBackendError> {
        if !visited.insert(id) {
            return Ok(false);
        }
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| self.internal(format!("expression {id} is outside MIR arena")))?;

        match &expression.kind {
            HirExprKind::Call { name, .. }
                if is_ddt_name(name.as_str()) || is_idt_name(name.as_str()) =>
            {
                Ok(true)
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Ddt { .. }
                    | HirAnalogOperator::Idt { .. }
                    | HirAnalogOperator::IdtMod { .. }
                    | HirAnalogOperator::Absdelay { .. }
                    | HirAnalogOperator::Transition { .. }
                    | HirAnalogOperator::Slew { .. }
                    | HirAnalogOperator::LastCrossing { .. },
            } => Ok(true),
            _ => expression_children(&expression.kind).into_iter().try_fold(
                false,
                |has_side_effects, child| {
                    if has_side_effects {
                        Ok(true)
                    } else {
                        self.expression_has_side_effects(child, visited)
                    }
                },
            ),
        }
    }

    fn lower_branch_access(
        &self,
        access: &str,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<String, RustBackendError> {
        if access == "I" {
            if let Some(slot) = self.branch_current_slot_for_nodes(pos, neg)? {
                return Ok(
                    slot.signed_value(format!("ctx.branch_current(self.branches[{}])", slot.slot))
                );
            }
            return Err(self.unsupported(format!("branch access '{access}' in expression")));
        }

        let pos = self.node_voltage_expr(pos)?;
        let neg = neg
            .map(|node| self.node_voltage_expr(node))
            .transpose()?
            .unwrap_or_else(|| "0.0".to_string());
        Ok(format!("({pos} - {neg})"))
    }

    fn lower_named_branch_access(
        &self,
        access: &str,
        name: &str,
    ) -> Result<String, RustBackendError> {
        if access == "I" {
            if let Some(current) = self.branch_currents.get(name) {
                return Ok(current.value.clone());
            }
            if let Some(slot) = self.branch_current_unknowns.get(name) {
                return Ok(
                    slot.signed_value(format!("ctx.branch_current(self.branches[{}])", slot.slot))
                );
            }
            if matches!(self.mode, ExprMode::Reactive) {
                return Ok("0.0".to_string());
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
        let pos = branch
            .pos_node
            .map(|node| format!("ctx.node_voltage(self.nodes[{}])", node.index()))
            .unwrap_or_else(|| "0.0".to_string());
        let neg = branch
            .neg_node
            .map(|node| format!("ctx.node_voltage(self.nodes[{}])", node.index()))
            .unwrap_or_else(|| "0.0".to_string());
        Ok(format!("({pos} - {neg})"))
    }

    fn branch_current_slot_for_nodes(
        &self,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<Option<BranchCurrentSlot>, RustBackendError> {
        let pos = self.node_index(pos)?;
        let neg = neg.map(|node| self.node_index(node)).transpose()?.flatten();
        Ok(self
            .branch_current_unknowns
            .get(&branch_pair_key(pos, neg))
            .copied())
    }

    fn node_voltage_expr(&self, name: &str) -> Result<String, RustBackendError> {
        if self.is_ground(name) {
            return Ok("0.0".to_string());
        }
        let node = self
            .artifact
            .mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown branch access node '{name}'")))?;
        Ok(format!("ctx.node_voltage(self.nodes[{}])", node.id.index()))
    }

    fn branch_access_derivatives(
        &self,
        access: &str,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<Vec<String>, RustBackendError> {
        if access == "I" {
            if self.branch_current_slot_for_nodes(pos, neg)?.is_some() {
                return Ok(zero_derivatives(self.artifact.mir.nodes.len()));
            }
            return Err(self.unsupported(format!("branch access '{access}' in expression")));
        }

        let mut derivatives = zero_derivatives(self.artifact.mir.nodes.len());
        if let Some(index) = self.node_index(pos)? {
            derivatives[index] = "1.0".to_string();
        }
        if let Some(neg) = neg
            && let Some(index) = self.node_index(neg)?
        {
            derivatives[index] = "-1.0".to_string();
        }
        Ok(derivatives)
    }

    fn branch_ref_derivatives(
        &self,
        access: &str,
        branch: &MirBranchRef,
    ) -> Result<Vec<String>, RustBackendError> {
        if access == "I" {
            return Err(self.unsupported(format!("named branch access '{access}' in expression")));
        }

        let mut derivatives = zero_derivatives(self.artifact.mir.nodes.len());
        if let Some(node) = branch.pos_node {
            derivatives[usize::from(node)] = "1.0".to_string();
        }
        if let Some(node) = branch.neg_node {
            derivatives[usize::from(node)] = "-1.0".to_string();
        }
        Ok(derivatives)
    }

    fn node_index(&self, name: &str) -> Result<Option<usize>, RustBackendError> {
        if self.is_ground(name) {
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

    fn is_ground(&self, name: &str) -> bool {
        name == "0"
            || self
                .artifact
                .mir
                .ground_nodes
                .iter()
                .any(|ground| ground.as_str() == name)
    }

    fn emit_value(&mut self, base: &str, expression: String) -> String {
        self.lines.push(format!("let {base}: f64 = {expression};"));
        base.to_string()
    }

    fn lower_conditional(
        &mut self,
        id: ExprId,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> Result<ExprValue, RustBackendError> {
        let condition = self.lower_condition(condition)?;
        let then_branch = self.lower_isolated_branch(then_expr)?;
        let else_branch = self.lower_isolated_branch(else_expr)?;
        let node_count = self.artifact.mir.nodes.len();
        let branch_axis_count = branch_derivative_axis_count(self.branch_current_unknowns);
        let base = format!("{}_e{}", self.prefix, id.index());

        let mut names = Vec::new();
        let mut then_values = Vec::new();
        let mut else_values = Vec::new();
        let mut tuple_aliases = HashMap::new();
        push_conditional_tuple_binding(
            &mut names,
            &mut then_values,
            &mut else_values,
            &mut tuple_aliases,
            base.clone(),
            then_branch.value.value.clone(),
            else_branch.value.value.clone(),
        );

        let mut derivatives = Vec::with_capacity(node_count);
        if matches!(self.derivative_emission, DerivativeEmission::None) {
            derivatives = zero_derivatives(node_count);
        } else {
            for index in 0..node_count {
                let then_derivative = &then_branch.value.derivatives[index];
                let else_derivative = &else_branch.value.derivatives[index];
                if is_zero_derivative(then_derivative) && is_zero_derivative(else_derivative) {
                    derivatives.push("0.0".to_string());
                    continue;
                }
                derivatives.push(push_conditional_tuple_binding(
                    &mut names,
                    &mut then_values,
                    &mut else_values,
                    &mut tuple_aliases,
                    format!("{base}_d_n{index}"),
                    then_derivative.clone(),
                    else_derivative.clone(),
                ));
            }
        }

        let mut branch_derivatives = Vec::with_capacity(branch_axis_count);
        if matches!(self.derivative_emission, DerivativeEmission::None) {
            branch_derivatives = zero_derivatives(branch_axis_count);
        } else {
            for index in 0..branch_axis_count {
                let then_derivative = &then_branch.value.branch_derivatives[index];
                let else_derivative = &else_branch.value.branch_derivatives[index];
                if is_zero_derivative(then_derivative) && is_zero_derivative(else_derivative) {
                    branch_derivatives.push("0.0".to_string());
                    continue;
                }
                branch_derivatives.push(push_conditional_tuple_binding(
                    &mut names,
                    &mut then_values,
                    &mut else_values,
                    &mut tuple_aliases,
                    format!("{base}_d_b{index}"),
                    then_derivative.clone(),
                    else_derivative.clone(),
                ));
            }
        }

        let has_reactive = then_branch.value.has_reactive || else_branch.value.has_reactive;
        let (reactive_value, reactive_derivatives, reactive_branch_derivatives) = if has_reactive {
            let reactive_value = push_conditional_tuple_binding(
                &mut names,
                &mut then_values,
                &mut else_values,
                &mut tuple_aliases,
                format!("{base}_q"),
                then_branch.value.reactive_value.clone(),
                else_branch.value.reactive_value.clone(),
            );

            let mut reactive_derivatives = Vec::with_capacity(node_count);
            for index in 0..node_count {
                let then_derivative = &then_branch.value.reactive_derivatives[index];
                let else_derivative = &else_branch.value.reactive_derivatives[index];
                if is_zero_derivative(then_derivative) && is_zero_derivative(else_derivative) {
                    reactive_derivatives.push("0.0".to_string());
                    continue;
                }
                reactive_derivatives.push(push_conditional_tuple_binding(
                    &mut names,
                    &mut then_values,
                    &mut else_values,
                    &mut tuple_aliases,
                    format!("{base}_q_d_n{index}"),
                    then_derivative.clone(),
                    else_derivative.clone(),
                ));
            }

            let mut reactive_branch_derivatives = Vec::with_capacity(branch_axis_count);
            for index in 0..branch_axis_count {
                let then_derivative = &then_branch.value.reactive_branch_derivatives[index];
                let else_derivative = &else_branch.value.reactive_branch_derivatives[index];
                if is_zero_derivative(then_derivative) && is_zero_derivative(else_derivative) {
                    reactive_branch_derivatives.push("0.0".to_string());
                    continue;
                }
                reactive_branch_derivatives.push(push_conditional_tuple_binding(
                    &mut names,
                    &mut then_values,
                    &mut else_values,
                    &mut tuple_aliases,
                    format!("{base}_q_d_b{index}"),
                    then_derivative.clone(),
                    else_derivative.clone(),
                ));
            }

            (
                reactive_value,
                reactive_derivatives,
                reactive_branch_derivatives,
            )
        } else {
            (
                "0.0".to_string(),
                zero_derivatives(node_count),
                zero_derivatives(branch_axis_count),
            )
        };

        self.lines.push(lazy_conditional_tuple(
            &names,
            &condition,
            &then_branch.lines,
            &then_values,
            &else_branch.lines,
            &else_values,
        ));

        Ok(ExprValue {
            value: base,
            derivatives,
            branch_derivatives,
            has_reactive,
            reactive_value,
            reactive_derivatives,
            reactive_branch_derivatives,
        })
    }

    fn lower_isolated_branch(&self, expr: ExprId) -> Result<ConditionalBranch, RustBackendError> {
        let mut branch = ExprEmitter {
            artifact: self.artifact,
            prefix: self.prefix,
            parameter_fields: self.parameter_fields,
            variables: self.variables,
            ddt_slots: self.ddt_slots,
            branch_currents: self.branch_currents,
            branch_current_unknowns: self.branch_current_unknowns,
            mode: self.mode,
            derivative_emission: self.derivative_emission,
            emitted: self.emitted.clone(),
            lines: Vec::new(),
        };
        let value = branch.lower(expr)?;
        Ok(ConditionalBranch {
            lines: branch.lines,
            value,
        })
    }

    fn lower_condition(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(id)? {
            return Ok(condition);
        }
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                self.internal(format!("condition expression {id} is outside MIR arena"))
            })?;
        match &expression.kind {
            HirExprKind::Binary { op, left, right }
                if comparison_operator(op.as_str()).is_some() =>
            {
                let operator = comparison_operator(op.as_str()).expect("checked above");
                if let Some(condition) =
                    self.boolean_numeric_comparison_condition(operator, *left, *right)?
                {
                    return Ok(condition);
                }
                let left = self.lower(*left)?;
                let right = self.lower(*right)?;
                Ok(format!("({} {operator} {})", left.value, right.value))
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
                Ok(negate_condition(&operand))
            }
            _ => {
                let value = self.lower(id)?;
                Ok(format!("({} != 0.0)", value.value))
            }
        }
    }

    fn direct_boolean_condition_expr(
        &self,
        id: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                self.internal(format!("condition expression {id} is outside MIR arena"))
            })?;
        Ok(match &expression.kind {
            HirExprKind::Identifier { name } => self
                .variables
                .get(name.as_str())
                .and_then(|variable| variable.condition.clone()),
            HirExprKind::SystemFunction { name, args }
                if name.eq_ignore_ascii_case("$param_given") =>
            {
                let index = self.param_given_index(args.as_slice())?;
                Some(format!("self.param_given[{index}]"))
            }
            HirExprKind::SystemFunction { name, args }
                if name.eq_ignore_ascii_case("$port_connected") =>
            {
                self.expect_system_arity("$port_connected", args.as_slice(), 1)?;
                Some("true".to_string())
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                Some(self.analysis_condition(args.as_slice())?)
            }
            _ => None,
        })
    }

    fn boolean_numeric_comparison_condition(
        &self,
        operator: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(left)? {
            if let Some(expected) = self.numeric_boolean_literal(right)? {
                return Ok(boolean_numeric_condition(condition, operator, expected));
            }
        }
        if let Some(condition) = self.direct_boolean_condition_expr(right)? {
            if let Some(expected) = self.numeric_boolean_literal(left)? {
                return Ok(boolean_numeric_condition(condition, operator, expected));
            }
        }
        Ok(None)
    }

    fn numeric_boolean_literal(&self, id: ExprId) -> Result<Option<bool>, RustBackendError> {
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                self.internal(format!("numeric expression {id} is outside MIR arena"))
            })?;
        Ok(match &expression.kind {
            HirExprKind::Number { value, .. } if *value == 0.0 => Some(false),
            HirExprKind::Number { value, .. } if *value == 1.0 => Some(true),
            _ => None,
        })
    }

    fn lower_intrinsic_value(
        &mut self,
        name: &str,
        args: &[ExprId],
        base: &str,
    ) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let args = self.lower_intrinsic_args(&normalized, args)?;
        let value = match normalized.as_str() {
            "abs" | "fabs" => format!("{}.abs()", f64_binary_receiver(&args[0].value)),
            "sqrt" => format!("{}.sqrt()", f64_binary_receiver(&args[0].value)),
            "exp" => format!("{}.exp()", f64_binary_receiver(&args[0].value)),
            "limexp" => limexp_value_expr(&args[0].value),
            "__rspice_limited_exp" => limited_exp_value_expr(&args[0].value),
            "ln" | "log" => format!("{}.ln()", f64_binary_receiver(&args[0].value)),
            "log10" => format!("{}.log10()", f64_binary_receiver(&args[0].value)),
            "sin" => format!("{}.sin()", f64_binary_receiver(&args[0].value)),
            "cos" => format!("{}.cos()", f64_binary_receiver(&args[0].value)),
            "tan" => format!("{}.tan()", f64_binary_receiver(&args[0].value)),
            "atan" => format!("{}.atan()", f64_binary_receiver(&args[0].value)),
            "sinh" => format!("{}.sinh()", f64_binary_receiver(&args[0].value)),
            "cosh" => format!("{}.cosh()", f64_binary_receiver(&args[0].value)),
            "tanh" => format!("{}.tanh()", f64_binary_receiver(&args[0].value)),
            "asinh" => format!("{}.asinh()", f64_binary_receiver(&args[0].value)),
            "acosh" => format!("{}.acosh()", f64_binary_receiver(&args[0].value)),
            "atanh" => format!("{}.atanh()", f64_binary_receiver(&args[0].value)),
            "floor" => format!("{}.floor()", f64_binary_receiver(&args[0].value)),
            "ceil" => format!("{}.ceil()", f64_binary_receiver(&args[0].value)),
            "pow" => format!(
                "{}.powf({})",
                f64_binary_receiver(&args[0].value),
                args[1].value
            ),
            "min" => format!(
                "{}.min({})",
                f64_binary_receiver(&args[0].value),
                args[1].value
            ),
            "max" => format!(
                "{}.max({})",
                f64_binary_receiver(&args[0].value),
                args[1].value
            ),
            "hypot" => format!(
                "{}.hypot({})",
                f64_binary_receiver(&args[0].value),
                args[1].value
            ),
            "atan2" => format!(
                "{}.atan2({})",
                f64_binary_receiver(&args[0].value),
                args[1].value
            ),
            _ => return Err(self.unsupported(format!("intrinsic function '{name}'"))),
        };
        Ok(self.emit_value(base, value))
    }

    fn lower_system_function_value(
        &mut self,
        name: &str,
        args: &[ExprId],
        base: &str,
    ) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let value = match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "ctx.temperature()".to_string()
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "self.time".to_string()
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "self.multiplicity".to_string()
            }
            "$vt" | "$thermal_vt" => match args {
                [] => "ctx.thermal_voltage()".to_string(),
                [temperature] => {
                    let temperature = self.lower(*temperature)?;
                    format!("({} * THERMAL_VOLTAGE_PER_K)", temperature.value)
                }
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => self.lower_simparam_value(args)?,
            "$param_given" => {
                let index = self.param_given_index(args)?;
                format!("if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }}")
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                "1.0".to_string()
            }
            _ => return Err(self.unsupported(format!("system function '{name}'"))),
        };
        Ok(self.emit_value(base, value))
    }

    fn system_function_derivatives(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Vec<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        match normalized.as_str() {
            "$vt" | "$thermal_vt" if args.len() == 1 => {
                let temperature = self
                    .emitted
                    .get(&args[0])
                    .expect("thermal voltage argument must be emitted before derivative");
                Ok(temperature
                    .derivatives
                    .iter()
                    .map(|derivative| format!("({derivative} * THERMAL_VOLTAGE_PER_K)"))
                    .collect())
            }
            "$simparam" if args.len() >= 2 => {
                let default = self
                    .emitted
                    .get(&args[1])
                    .expect("simparam default must be emitted before derivative");
                Ok(default.derivatives.clone())
            }
            "$temperature" | "$abstime" | "$realtime" | "$mfactor" | "$vt" | "$thermal_vt"
            | "$simparam" | "$param_given" | "$port_connected" => {
                Ok(zero_derivatives(self.artifact.mir.nodes.len()))
            }
            _ => Err(self.unsupported(format!("system function '{name}'"))),
        }
    }

    fn system_function_branch_derivatives(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Vec<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let branch_axis_count = branch_derivative_axis_count(self.branch_current_unknowns);
        match normalized.as_str() {
            "$vt" | "$thermal_vt" if args.len() == 1 => {
                let temperature = self
                    .emitted
                    .get(&args[0])
                    .expect("thermal voltage argument must be emitted before derivative");
                Ok(temperature
                    .branch_derivatives
                    .iter()
                    .map(|derivative| format!("({derivative} * THERMAL_VOLTAGE_PER_K)"))
                    .collect())
            }
            "$simparam" if args.len() >= 2 => {
                let default = self
                    .emitted
                    .get(&args[1])
                    .expect("simparam default must be emitted before derivative");
                Ok(default.branch_derivatives.clone())
            }
            "$temperature" | "$abstime" | "$realtime" | "$mfactor" | "$vt" | "$thermal_vt"
            | "$simparam" | "$param_given" | "$port_connected" => {
                Ok(zero_derivatives(branch_axis_count))
            }
            _ => Err(self.unsupported(format!("system function '{name}'"))),
        }
    }

    fn system_function_reactive_value(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<ReactiveValue, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let count = self.artifact.mir.nodes.len();
        match normalized.as_str() {
            "$vt" | "$thermal_vt" if args.len() == 1 => {
                let temperature = self
                    .emitted
                    .get(&args[0])
                    .expect("thermal voltage argument must be emitted before reactive derivative");
                if temperature.has_reactive {
                    Ok(ReactiveValue {
                        has_reactive: true,
                        value: format!("({} * THERMAL_VOLTAGE_PER_K)", temperature.reactive_value),
                        derivatives: temperature
                            .reactive_derivatives
                            .iter()
                            .map(|derivative| format!("({derivative} * THERMAL_VOLTAGE_PER_K)"))
                            .collect(),
                        branch_derivatives: temperature
                            .reactive_branch_derivatives
                            .iter()
                            .map(|derivative| format!("({derivative} * THERMAL_VOLTAGE_PER_K)"))
                            .collect(),
                    })
                } else {
                    Ok(ReactiveValue::none(zero_derivatives(count)))
                }
            }
            "$simparam" if args.len() >= 2 => {
                let default = self
                    .emitted
                    .get(&args[1])
                    .expect("simparam default must be emitted before reactive derivative");
                if default.has_reactive {
                    Ok(ReactiveValue {
                        has_reactive: true,
                        value: default.reactive_value.clone(),
                        derivatives: default.reactive_derivatives.clone(),
                        branch_derivatives: default.reactive_branch_derivatives.clone(),
                    })
                } else {
                    Ok(ReactiveValue::none(zero_derivatives(count)))
                }
            }
            "$temperature" | "$abstime" | "$realtime" | "$mfactor" | "$vt" | "$thermal_vt"
            | "$simparam" | "$param_given" | "$port_connected" => {
                Ok(ReactiveValue::none(zero_derivatives(count)))
            }
            _ => Err(self.unsupported(format!("system function '{name}'"))),
        }
    }

    fn lower_simparam_value(&mut self, args: &[ExprId]) -> Result<String, RustBackendError> {
        match args {
            [name] => Ok(format_f64(self.simparam_default(*name)?)),
            [_, default] => {
                let default = self.lower(*default)?;
                Ok(default.value)
            }
            _ => Err(self.unsupported(format!(
                "$simparam expects one or two arguments, found {}",
                args.len()
            ))),
        }
    }

    fn lower_analysis_value(
        &mut self,
        args: &[ExprId],
        base: &str,
    ) -> Result<String, RustBackendError> {
        let condition = self.analysis_condition(args)?;
        Ok(self.emit_value(base, format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")))
    }

    fn analysis_condition(&self, args: &[ExprId]) -> Result<String, RustBackendError> {
        let query = self.analysis_query(args)?;
        Ok(analysis_predicate_expr(&query).to_string())
    }

    fn analysis_query(&self, args: &[ExprId]) -> Result<String, RustBackendError> {
        let [name] = args else {
            return Err(self.unsupported(format!(
                "analysis expects one argument, found {}",
                args.len()
            )));
        };
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(*name))
            .ok_or_else(|| self.internal(format!("analysis query {name} is outside MIR arena")))?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return Err(self.unsupported("analysis expects a string literal argument"));
        };
        normalize_analysis_query(value)
            .ok_or_else(|| self.unsupported(format!("analysis() unknown analysis name '{value}'")))
    }

    fn simparam_default(&self, name: ExprId) -> Result<f64, RustBackendError> {
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(name))
            .ok_or_else(|| self.internal(format!("simparam name {name} is outside MIR arena")))?;
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
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(*parameter))
            .ok_or_else(|| {
                self.internal(format!(
                    "param_given argument {parameter} is outside MIR arena"
                ))
            })?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return Err(self.unsupported("$param_given parameter argument"));
        };
        self.artifact
            .mir
            .parameters
            .iter()
            .position(|parameter| {
                parameter.name.as_str().eq_ignore_ascii_case(name.as_str())
                    || parameter
                        .aliases
                        .iter()
                        .any(|alias| alias.as_str().eq_ignore_ascii_case(name.as_str()))
            })
            .ok_or_else(|| self.unsupported(format!("unknown $param_given parameter '{name}'")))
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

    fn intrinsic_derivatives(
        &self,
        name: &str,
        args: &[ExprId],
        value: &str,
    ) -> Result<Vec<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let args = self.emitted_intrinsic_args(&normalized, args)?;
        intrinsic_derivatives_from_values(name, &args, value, |value| &value.derivatives)
    }

    fn intrinsic_branch_derivatives(
        &self,
        name: &str,
        args: &[ExprId],
        value: &str,
    ) -> Result<Vec<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let args = self.emitted_intrinsic_args(&normalized, args)?;
        intrinsic_derivatives_from_values(name, &args, value, |value| &value.branch_derivatives)
    }

    fn intrinsic_reactive_value(&self, args: &[ExprId]) -> Result<ReactiveValue, RustBackendError> {
        for arg in args {
            let value = self
                .emitted
                .get(arg)
                .expect("intrinsic argument must be emitted before reactive derivative");
            if value.has_reactive {
                return Err(self.unsupported("ddt() inside intrinsic math call"));
            }
        }
        Ok(ReactiveValue::none(zero_derivatives(
            self.artifact.mir.nodes.len(),
        )))
    }

    fn lower_intrinsic_args(
        &mut self,
        normalized: &str,
        args: &[ExprId],
    ) -> Result<Vec<ExprValue>, RustBackendError> {
        self.validate_intrinsic_arity(normalized, args)?;
        args.iter().map(|arg| self.lower(*arg)).collect()
    }

    fn emitted_intrinsic_args(
        &self,
        normalized: &str,
        args: &[ExprId],
    ) -> Result<Vec<ExprValue>, RustBackendError> {
        self.validate_intrinsic_arity(normalized, args)?;
        Ok(args
            .iter()
            .map(|arg| {
                self.emitted
                    .get(arg)
                    .expect("intrinsic argument must be emitted before derivative")
                    .clone()
            })
            .collect())
    }

    fn validate_intrinsic_arity(
        &self,
        normalized: &str,
        args: &[ExprId],
    ) -> Result<(), RustBackendError> {
        let expected = if is_binary_intrinsic_name(normalized) {
            2
        } else {
            1
        };
        if args.len() == expected {
            Ok(())
        } else {
            Err(self.unsupported(format!(
                "intrinsic function '{normalized}' expects {expected} argument(s), found {}",
                args.len()
            )))
        }
    }

    fn lower_ddt_value(
        &mut self,
        id: ExprId,
        args: &[ExprId],
        base: &str,
    ) -> Result<String, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self.lower(operand_id)?;
        match self.mode {
            ExprMode::Transient => {
                let slot = self.ddt_slots.slot_for(id).ok_or_else(|| {
                    self.internal(format!("ddt expression {id} has no generated state slot"))
                })?;
                Ok(self.emit_value(
                    base,
                    format!(
                        "eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_previous_previous, ddt_state_derivative_current, ddt_state_derivative_previous, ddt_state_initialized, ddt_state_history_depth, ddt_active, ddt_coefficients, ddt_scale, {slot}, {})",
                        operand.value
                    ),
                ))
            }
            ExprMode::Reactive => Ok(operand.value),
        }
    }

    fn ddt_derivatives(
        &mut self,
        _id: ExprId,
        args: &[ExprId],
    ) -> Result<Vec<String>, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self
            .emitted
            .get(&operand_id)
            .expect("ddt operand must be emitted before derivative");
        let operand_derivatives = operand.derivatives.clone();
        let derivatives = match self.mode {
            ExprMode::Transient => operand_derivatives
                .iter()
                .map(|derivative| self.ddt_jacobian_expr(derivative))
                .collect(),
            ExprMode::Reactive => operand_derivatives,
        };
        Ok(derivatives)
    }

    fn ddt_branch_derivatives(&mut self, args: &[ExprId]) -> Result<Vec<String>, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self
            .emitted
            .get(&operand_id)
            .expect("ddt operand must be emitted before branch derivative");
        let operand_branch_derivatives = operand.branch_derivatives.clone();
        let derivatives = match self.mode {
            ExprMode::Transient => operand_branch_derivatives
                .iter()
                .map(|derivative| self.ddt_jacobian_expr(derivative))
                .collect(),
            ExprMode::Reactive => operand_branch_derivatives,
        };
        Ok(derivatives)
    }

    fn ddt_reactive_value(&self, args: &[ExprId]) -> Result<ReactiveValue, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self
            .emitted
            .get(&operand_id)
            .expect("ddt operand must be emitted before reactive derivative");
        Ok(ReactiveValue {
            has_reactive: true,
            value: operand.value.clone(),
            derivatives: operand.derivatives.clone(),
            branch_derivatives: operand.branch_derivatives.clone(),
        })
    }

    fn ddt_operand(&self, args: &[ExprId]) -> Result<ExprId, RustBackendError> {
        match args {
            [operand] => Ok(*operand),
            [_, _] => Err(self.unsupported("ddt abstol argument")),
            _ => Err(self.unsupported(format!("ddt expects one operand, found {}", args.len()))),
        }
    }

    fn lower_idt_value(
        &mut self,
        id: ExprId,
        expr: ExprId,
        ic: Option<ExprId>,
        base: &str,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(expr)?;
        let ic_value = if let Some(ic) = ic {
            self.lower(ic)?.value
        } else {
            "0.0".to_string()
        };
        match self.mode {
            ExprMode::Transient => {
                let slot = self.ddt_slots.idt_slot_for(id).ok_or_else(|| {
                    self.internal(format!("idt expression {id} has no generated state slot"))
                })?;
                Ok(self.emit_value(
                    base,
                    format!(
                        "eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, {slot}, {}, {ic_value})",
                        operand.value
                    ),
                ))
            }
            ExprMode::Reactive => Ok(ic_value),
        }
    }

    fn idt_derivatives(&mut self, expr: ExprId) -> Result<Vec<String>, RustBackendError> {
        let operand = self
            .emitted
            .get(&expr)
            .expect("idt operand must be emitted before derivative");
        let operand_derivatives = operand.derivatives.clone();
        let derivatives = match self.mode {
            ExprMode::Transient => operand_derivatives
                .iter()
                .map(|derivative| self.idt_jacobian_expr(derivative))
                .collect(),
            ExprMode::Reactive => zero_derivatives(operand_derivatives.len()),
        };
        Ok(derivatives)
    }

    fn idt_branch_derivatives(&mut self, expr: ExprId) -> Result<Vec<String>, RustBackendError> {
        let operand = self
            .emitted
            .get(&expr)
            .expect("idt operand must be emitted before branch derivative");
        let operand_branch_derivatives = operand.branch_derivatives.clone();
        let derivatives = match self.mode {
            ExprMode::Transient => operand_branch_derivatives
                .iter()
                .map(|derivative| self.idt_jacobian_expr(derivative))
                .collect(),
            ExprMode::Reactive => zero_derivatives(operand_branch_derivatives.len()),
        };
        Ok(derivatives)
    }

    fn idt_operands(&self, args: &[ExprId]) -> Result<(ExprId, Option<ExprId>), RustBackendError> {
        match args {
            [expr] => Ok((*expr, None)),
            [expr, ic] => Ok((*expr, Some(*ic))),
            _ => Err(self.unsupported(format!(
                "idt expects one or two operands, found {}",
                args.len()
            ))),
        }
    }

    fn lower_ddx_value(
        &mut self,
        expr: ExprId,
        probe: ExprId,
        base: &str,
    ) -> Result<String, RustBackendError> {
        let expr = self.lower(expr)?;
        let projection = self.ddx_projection(&expr.derivatives, probe)?;
        Ok(self.emit_value(base, projection))
    }

    fn ddx_projection(
        &self,
        derivatives: &[String],
        probe: ExprId,
    ) -> Result<String, RustBackendError> {
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(probe))
            .ok_or_else(|| self.internal(format!("ddx probe {probe} is outside MIR arena")))?;
        let (pos, neg) = match &expression.kind {
            HirExprKind::BranchAccess { access, pos, neg } if access.as_str() != "I" => (
                self.node_index(pos.as_str())?,
                neg.as_deref()
                    .map(|node| self.node_index(node))
                    .transpose()?
                    .flatten(),
            ),
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
                (
                    branch.pos_node.map(usize::from),
                    branch.neg_node.map(usize::from),
                )
            }
            other => {
                return Err(self.unsupported(format!(
                    "ddx probe must be a voltage access, found {other:?}"
                )));
            }
        };
        let pos = pos
            .and_then(|index| derivatives.get(index))
            .cloned()
            .unwrap_or_else(|| "0.0".to_string());
        if let Some(neg) = neg {
            let neg = derivatives
                .get(neg)
                .cloned()
                .unwrap_or_else(|| "0.0".to_string());
            Ok(format!("(0.5 * ({pos} - {neg}))"))
        } else {
            Ok(pos)
        }
    }

    fn ddx_operands(&self, args: &[ExprId]) -> Result<(ExprId, ExprId), RustBackendError> {
        match args {
            [expr, probe] => Ok((*expr, *probe)),
            _ => Err(self.unsupported(format!("ddx expects two operands, found {}", args.len()))),
        }
    }

    fn ddt_jacobian_expr(&mut self, derivative: &str) -> String {
        if is_zero_derivative(derivative) {
            return "0.0".to_string();
        }
        scaled_derivative_expr(derivative, "ddt_scale")
    }

    fn idt_jacobian_expr(&mut self, derivative: &str) -> String {
        if is_zero_derivative(derivative) {
            return "0.0".to_string();
        }
        scaled_derivative_expr(derivative, "idt_scale")
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

struct ConditionalBranch {
    lines: Vec<String>,
    value: ExprValue,
}

fn push_conditional_tuple_binding(
    names: &mut Vec<String>,
    then_values: &mut Vec<String>,
    else_values: &mut Vec<String>,
    aliases: &mut HashMap<(String, String), String>,
    name: String,
    then_value: String,
    else_value: String,
) -> String {
    let key = (then_value.trim().to_string(), else_value.trim().to_string());
    if let Some(alias) = aliases.get(&key) {
        return alias.clone();
    }

    names.push(name.clone());
    then_values.push(then_value);
    else_values.push(else_value);
    aliases.insert(key, name.clone());
    name
}

fn lazy_conditional_tuple(
    names: &[String],
    condition: &str,
    then_lines: &[String],
    then_values: &[String],
    else_lines: &[String],
    else_values: &[String],
) -> String {
    debug_assert_eq!(names.len(), then_values.len());
    debug_assert_eq!(names.len(), else_values.len());

    let mut out = String::new();
    out.push_str("let ");
    out.push_str(&tuple_pattern(names));
    out.push_str(" = {\n");
    out.push_str("    if ");
    out.push_str(condition);
    out.push_str(" {\n");
    push_conditional_branch(&mut out, then_lines, then_values);
    out.push_str("    } else {\n");
    push_conditional_branch(&mut out, else_lines, else_values);
    out.push_str("    }\n");
    out.push_str("};");
    out
}

fn push_conditional_branch(out: &mut String, lines: &[String], values: &[String]) {
    for line in lines {
        push_indented_lines(out, "        ", line);
    }
    out.push_str("        ");
    out.push_str(&tuple_values(values));
    out.push('\n');
}

fn push_indented_lines(out: &mut String, indent: &str, text: &str) {
    for line in text.lines() {
        out.push_str(indent);
        out.push_str(line);
        out.push('\n');
    }
}

fn tuple_pattern(names: &[String]) -> String {
    format!("({},)", names.join(", "))
}

fn tuple_values(values: &[String]) -> String {
    format!("({},)", values.join(", "))
}

#[derive(Debug, Clone)]
struct ReactiveValue {
    has_reactive: bool,
    value: String,
    derivatives: Vec<String>,
    branch_derivatives: Vec<String>,
}

impl ReactiveValue {
    fn none(derivatives: Vec<String>) -> Self {
        Self {
            has_reactive: false,
            value: "0.0".to_string(),
            derivatives,
            branch_derivatives: Vec::new(),
        }
    }
}

pub fn parameter_field_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    artifact
        .mir
        .parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| (parameter.name.to_string(), format!("p{index}")))
        .collect()
}

pub fn unique_identifiers(names: &[String]) -> HashMap<String, String> {
    let mut used: HashMap<String, usize> = HashMap::new();
    let mut fields = HashMap::new();

    for name in names {
        let base = sanitize_identifier(name);
        let count = used.entry(base.clone()).or_insert(0);
        let field = if *count == 0 {
            base.clone()
        } else {
            format!("{base}_{count}")
        };
        *count += 1;
        fields.insert(name.clone(), field);
    }

    fields
}

fn zero_derivatives(count: usize) -> Vec<String> {
    vec!["0.0".to_string(); count]
}

fn materialize_derivative_expr(
    lines: &mut Vec<String>,
    local: String,
    derivative: String,
    derivative_emission: DerivativeEmission,
    aliases: &mut HashMap<String, String>,
) -> String {
    if is_zero_derivative(&derivative) {
        return "0.0".to_string();
    }
    if is_inline_derivative_expr(&derivative)
        || matches!(derivative_emission, DerivativeEmission::Inline)
    {
        return derivative;
    }

    let key = derivative.trim().to_string();
    if let Some(alias) = aliases.get(&key) {
        return alias.clone();
    }

    lines.push(format!("let {local}: f64 = {derivative};"));
    aliases.insert(key, local.clone());
    local
}

fn lowered_variable_is_constant_zero(variable: &LoweredVariable) -> bool {
    is_zero_derivative(&variable.value)
        && variable
            .derivatives
            .iter()
            .all(|derivative| is_zero_derivative(derivative))
        && variable
            .branch_derivatives
            .iter()
            .all(|derivative| is_zero_derivative(derivative))
        && (!variable.has_reactive
            || (is_zero_derivative(&variable.reactive_value)
                && variable
                    .reactive_derivatives
                    .iter()
                    .all(|derivative| is_zero_derivative(derivative))
                && variable
                    .reactive_branch_derivatives
                    .iter()
                    .all(|derivative| is_zero_derivative(derivative))))
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

fn branch_derivative_axis_count(
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> usize {
    branch_current_unknowns
        .values()
        .map(|slot| slot.slot)
        .max()
        .map(|slot| slot + 1)
        .unwrap_or(0)
}

fn limexp_value_expr(arg: &str) -> String {
    format!(
        "{{ let limexp_arg = {arg}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) }} }}"
    )
}

fn limexp_derivative_scale_expr(arg: &str) -> String {
    format!(
        "{{ let limexp_arg = {arg}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ LIMEXP_MAX }} }}"
    )
}

fn limited_exp_value_expr(arg: &str) -> String {
    format!(
        "{{ let limited_exp_arg = {arg}; if limited_exp_arg > 80.0 {{ LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) }} else if limited_exp_arg < -80.0 {{ 1.804851387e-35 }} else {{ limited_exp_arg.exp() }} }}"
    )
}

fn limited_exp_derivative_scale_expr(arg: &str) -> String {
    format!(
        "{{ let limited_exp_arg = {arg}; if limited_exp_arg > 80.0 {{ LIMEXP_MAX }} else if limited_exp_arg < -80.0 {{ 0.0 }} else {{ limited_exp_arg.exp() }} }}"
    )
}

fn is_zero_derivative(derivative: &str) -> bool {
    derivative.trim() == "0.0"
}

fn is_one_derivative(derivative: &str) -> bool {
    derivative.trim() == "1.0"
}

fn is_negative_one_derivative(derivative: &str) -> bool {
    derivative.trim() == "-1.0"
}

fn is_inline_derivative_expr(derivative: &str) -> bool {
    let derivative = derivative.trim();
    is_zero_derivative(derivative)
        || is_one_derivative(derivative)
        || is_negative_one_derivative(derivative)
        || is_generated_scratch_derivative_access(derivative)
        || is_dynamic_operator_scaled_inline_derivative_expr(derivative)
        || is_scaled_generated_scratch_derivative_access(derivative)
        || is_simple_inline_operand(derivative)
        || is_negated_simple_inline_operand(derivative)
        || is_rust_identifier(derivative)
}

fn is_dynamic_operator_scaled_inline_derivative_expr(derivative: &str) -> bool {
    dynamic_operator_scaled_operand(derivative, "ddt_scale")
        .or_else(|| dynamic_operator_scaled_operand(derivative, "idt_scale"))
        .is_some_and(|operand| {
            is_simple_inline_operand(operand) || is_negated_simple_inline_operand(operand)
        })
}

fn dynamic_operator_scaled_operand<'a>(derivative: &'a str, scale: &str) -> Option<&'a str> {
    let derivative = derivative.trim();
    let inner = derivative.strip_prefix('(')?.strip_suffix(')')?.trim();
    inner.strip_suffix(&format!(" * {scale}")).map(str::trim)
}

fn is_simple_inline_operand(value: &str) -> bool {
    is_generated_scratch_derivative_access(value)
        || is_rust_identifier(value)
        || is_simple_generated_access(value)
        || is_simple_numeric_literal(value)
}

fn is_negated_simple_inline_operand(value: &str) -> bool {
    let value = value.trim();
    let Some(inner) = value
        .strip_prefix("(-")
        .and_then(|inner| inner.strip_suffix(')'))
    else {
        return false;
    };
    is_simple_inline_operand(inner.trim())
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

fn is_simple_generated_access(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty()
        || value.contains(' ')
        || value.starts_with('.')
        || value.starts_with('[')
        || !(value.contains('.') || value.contains('['))
    {
        return false;
    }
    value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '[' | ']'))
}

fn is_simple_numeric_literal(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value.bytes().any(|byte| byte.is_ascii_digit())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'e' | b'E' | b'+' | b'-'))
        && value.parse::<f64>().is_ok()
}

fn is_scaled_generated_scratch_derivative_access(value: &str) -> bool {
    let value = trim_enclosing_parentheses(value.trim());
    let Some((left, right)) = split_top_level_multiplication(value) else {
        return false;
    };
    let left = trim_enclosing_parentheses(left.trim());
    let right = trim_enclosing_parentheses(right.trim());
    (is_generated_scratch_derivative_access(left) && is_simple_scaled_derivative_factor(right))
        || (is_simple_scaled_derivative_factor(left)
            && is_generated_scratch_derivative_access(right))
}

fn split_top_level_multiplication(expr: &str) -> Option<(&str, &str)> {
    let mut depth = 0usize;
    for (index, ch) in expr.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => depth = depth.saturating_sub(1),
            '*' if depth == 0 => return Some((&expr[..index], &expr[index + 1..])),
            _ => {}
        }
    }
    None
}

fn trim_enclosing_parentheses(mut expr: &str) -> &str {
    loop {
        let trimmed = expr.trim();
        if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
            return trimmed;
        }
        let mut depth = 0usize;
        let mut encloses_all = true;
        for (index, ch) in trimmed.char_indices() {
            match ch {
                '(' => depth = depth.saturating_add(1),
                ')' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 && index + ch.len_utf8() < trimmed.len() {
                        encloses_all = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !encloses_all || depth != 0 {
            return trimmed;
        }
        expr = &trimmed[1..trimmed.len() - 1];
    }
}

fn is_simple_scaled_derivative_factor(value: &str) -> bool {
    let value = value.trim();
    is_rust_identifier(value)
        || is_simple_generated_access(value)
        || is_simple_numeric_literal(value)
}

#[cfg(test)]
mod tests {
    use super::is_inline_derivative_expr;

    #[test]
    fn inline_derivative_expr_accepts_scaled_scratch_rows() {
        assert!(is_inline_derivative_expr(
            "(scratch.node_derivatives[12][0] * ddt_scale)"
        ));
        assert!(is_inline_derivative_expr(
            "(params.p32 * scratch.branch_derivatives[4][1])"
        ));
    }

    #[test]
    fn inline_derivative_expr_rejects_expensive_scaled_scratch_rows() {
        assert!(!is_inline_derivative_expr(
            "(scratch.node_derivatives[12][0] * expensive_scale())"
        ));
    }
}

fn scaled_derivative_expr(derivative: &str, scale: &str) -> String {
    let derivative = derivative.trim();
    if is_zero_derivative(derivative) {
        "0.0".to_string()
    } else if is_one_derivative(derivative) {
        scale.to_string()
    } else if is_negative_one_derivative(derivative) {
        format!("-{scale}")
    } else {
        mul_expr(derivative, scale)
    }
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

fn unary_value(op: &str, operand: &str) -> Result<String, RustBackendError> {
    match op {
        "Neg" if is_zero_derivative(operand) => Ok("0.0".to_string()),
        "Neg" => Ok(negate_value(operand)),
        "Pos" => Ok(operand.to_string()),
        _ => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            format!("unary operator {op}"),
        )),
    }
}

fn conditional_expr(condition: &str, then_expr: &str, else_expr: &str) -> String {
    let then_expr = then_expr.trim();
    let else_expr = else_expr.trim();
    if then_expr == else_expr {
        then_expr.to_string()
    } else if is_zero_derivative(then_expr) && is_zero_derivative(else_expr) {
        "0.0".to_string()
    } else {
        format!("if {condition} {{ {then_expr} }} else {{ {else_expr} }}")
    }
}

fn boolean_numeric_condition(condition: String, operator: &str, expected: bool) -> Option<String> {
    match (operator, expected) {
        ("==", true) | ("!=", false) => Some(condition),
        ("==", false) | ("!=", true) => Some(format!("(!{condition})")),
        _ => None,
    }
}

fn negate_condition(condition: &str) -> String {
    let condition = condition.trim();
    if let Some(inner) = condition
        .strip_prefix("(!")
        .and_then(|inner| inner.strip_suffix(')'))
    {
        return inner.to_string();
    }
    if let Some(inner) = condition
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
    {
        if let Some(value) = inner.strip_suffix(" != 0.0") {
            return format!("({value} == 0.0)");
        }
        if let Some(value) = inner.strip_suffix(" == 0.0") {
            return format!("({value} != 0.0)");
        }
    }
    format!("(!{condition})")
}

fn binary_value(op: &str, left: &str, right: &str) -> Result<String, RustBackendError> {
    match op {
        "Add" => Ok(add_expr(left, right)),
        "Sub" => Ok(sub_expr(left, right)),
        "Mul" => Ok(mul_expr(left, right)),
        "Div" => Ok(div_expr(left, right)),
        "Mod" => Ok(mod_expr(left, right)),
        "Pow" => Ok(format!("{}.powf({right})", f64_binary_receiver(left))),
        _ => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            format!("binary operator {op}"),
        )),
    }
}

fn add_expr(left: &str, right: &str) -> String {
    if is_zero_derivative(left) {
        right.to_string()
    } else if is_zero_derivative(right) {
        left.to_string()
    } else {
        format!("({left} + {right})")
    }
}

fn sub_expr(left: &str, right: &str) -> String {
    if is_zero_derivative(right) {
        left.to_string()
    } else if is_zero_derivative(left) {
        unary_value("Neg", right).expect("negation is supported")
    } else {
        format!("({left} - {right})")
    }
}

fn mul_expr(left: &str, right: &str) -> String {
    if is_zero_derivative(left) || is_zero_derivative(right) {
        "0.0".to_string()
    } else if is_one_derivative(left) {
        right.to_string()
    } else if is_one_derivative(right) {
        left.to_string()
    } else if is_negative_one_derivative(left) {
        unary_value("Neg", right).expect("negation is supported")
    } else if is_negative_one_derivative(right) {
        unary_value("Neg", left).expect("negation is supported")
    } else {
        format!("({left} * {right})")
    }
}

fn div_expr(left: &str, right: &str) -> String {
    if is_zero_derivative(left) {
        "0.0".to_string()
    } else if is_one_derivative(right) {
        left.to_string()
    } else {
        format!("({left} / {right})")
    }
}

fn mod_expr(left: &str, right: &str) -> String {
    if is_zero_derivative(left) {
        "0.0".to_string()
    } else {
        format!("({left} % {right})")
    }
}

fn div_derivative_expr(
    left_value: &str,
    right_value: &str,
    left_derivative: &str,
    right_derivative: &str,
) -> String {
    if is_zero_derivative(left_derivative) && is_zero_derivative(right_derivative) {
        "0.0".to_string()
    } else if is_zero_derivative(right_derivative) {
        div_expr(left_derivative, right_value)
    } else if is_zero_derivative(left_derivative) {
        unary_value(
            "Neg",
            &div_expr(
                &mul_expr(left_value, right_derivative),
                &mul_expr(right_value, right_value),
            ),
        )
        .expect("negation is supported")
    } else {
        div_expr(
            &sub_expr(
                &mul_expr(left_derivative, right_value),
                &mul_expr(left_value, right_derivative),
            ),
            &mul_expr(right_value, right_value),
        )
    }
}

fn mod_derivative_expr(
    left_value: &str,
    right_value: &str,
    left_derivative: &str,
    right_derivative: &str,
) -> String {
    if is_zero_derivative(left_derivative) && is_zero_derivative(right_derivative) {
        "0.0".to_string()
    } else if is_zero_derivative(right_derivative) {
        left_derivative.to_string()
    } else {
        let quotient = format!("(({left_value} / {right_value}).trunc())");
        sub_expr(left_derivative, &mul_expr(&quotient, right_derivative))
    }
}

fn binary_derivatives(
    op: &str,
    value: &str,
    left: &ExprValue,
    right: &ExprValue,
) -> Result<Vec<String>, RustBackendError> {
    binary_derivatives_for(
        op,
        value,
        &left.value,
        &right.value,
        &left.derivatives,
        &right.derivatives,
    )
}

fn binary_derivatives_for(
    op: &str,
    value: &str,
    left_value: &str,
    right_value: &str,
    left_derivatives: &[String],
    right_derivatives: &[String],
) -> Result<Vec<String>, RustBackendError> {
    left_derivatives
        .iter()
        .zip(right_derivatives)
        .map(|(left_derivative, right_derivative)| match op {
            "Add" => Ok(add_expr(left_derivative, right_derivative)),
            "Sub" => Ok(sub_expr(left_derivative, right_derivative)),
            "Mul" => Ok(add_expr(
                &mul_expr(left_derivative, right_value),
                &mul_expr(left_value, right_derivative),
            )),
            "Div" => Ok(div_derivative_expr(
                left_value,
                right_value,
                left_derivative,
                right_derivative,
            )),
            "Mod" => Ok(mod_derivative_expr(
                left_value,
                right_value,
                left_derivative,
                right_derivative,
            )),
            "Pow" => Ok(pow_derivative_expr(
                value,
                left_value,
                right_value,
                left_derivative,
                right_derivative,
            )),
            _ => Err(RustBackendError::unsupported(
                "<generated>",
                "<expr>",
                format!("binary operator {op}"),
            )),
        })
        .collect()
}

fn intrinsic_derivatives_from_values(
    name: &str,
    args: &[ExprValue],
    value: &str,
    axis: impl Fn(&ExprValue) -> &[String],
) -> Result<Vec<String>, RustBackendError> {
    let normalized = name.to_ascii_lowercase();
    let derivatives = match normalized.as_str() {
        "abs" | "fabs" => axis(&args[0])
            .iter()
            .map(|d| {
                let negated = unary_value("Neg", d)?;
                Ok(conditional_expr(
                    &format!("{} >= 0.0", args[0].value),
                    d,
                    &negated,
                ))
            })
            .collect::<Result<Vec<_>, RustBackendError>>()?,
        "sqrt" => axis(&args[0])
            .iter()
            .map(|d| div_expr(d, &format!("(2.0 * {value})")))
            .collect(),
        "exp" => axis(&args[0]).iter().map(|d| mul_expr(value, d)).collect(),
        "limexp" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&limexp_derivative_scale_expr(&args[0].value), d))
            .collect(),
        "__rspice_limited_exp" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&limited_exp_derivative_scale_expr(&args[0].value), d))
            .collect(),
        "ln" | "log" => axis(&args[0])
            .iter()
            .map(|d| div_expr(d, &args[0].value))
            .collect(),
        "log10" => axis(&args[0])
            .iter()
            .map(|d| div_expr(d, &format!("({} * std::f64::consts::LN_10)", args[0].value)))
            .collect(),
        "sin" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&format!("({}).cos()", args[0].value), d))
            .collect(),
        "cos" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&format!("-({}).sin()", args[0].value), d))
            .collect(),
        "tan" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!("(({}).cos() * ({}).cos())", args[0].value, args[0].value),
                )
            })
            .collect(),
        "atan" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!("(1.0 + ({} * {}))", args[0].value, args[0].value),
                )
            })
            .collect(),
        "sinh" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&format!("({}).cosh()", args[0].value), d))
            .collect(),
        "cosh" => axis(&args[0])
            .iter()
            .map(|d| mul_expr(&format!("({}).sinh()", args[0].value), d))
            .collect(),
        "tanh" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!("(({}).cosh() * ({}).cosh())", args[0].value, args[0].value),
                )
            })
            .collect(),
        "asinh" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!("(({} * {}) + 1.0).sqrt()", args[0].value, args[0].value),
                )
            })
            .collect(),
        "acosh" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!(
                        "(({} - 1.0).sqrt() * ({} + 1.0).sqrt())",
                        args[0].value, args[0].value
                    ),
                )
            })
            .collect(),
        "atanh" => axis(&args[0])
            .iter()
            .map(|d| {
                div_expr(
                    d,
                    &format!("(1.0 - ({} * {}))", args[0].value, args[0].value),
                )
            })
            .collect(),
        "floor" | "ceil" => zero_derivatives(axis(&args[0]).len()),
        "pow" => axis(&args[0])
            .iter()
            .zip(axis(&args[1]))
            .map(|(dx, dy)| pow_derivative_expr(value, &args[0].value, &args[1].value, dx, dy))
            .collect(),
        "min" => axis(&args[0])
            .iter()
            .zip(axis(&args[1]))
            .map(|(left, right)| {
                conditional_expr(
                    &format!("{} <= {}", args[0].value, args[1].value),
                    left,
                    right,
                )
            })
            .collect(),
        "max" => axis(&args[0])
            .iter()
            .zip(axis(&args[1]))
            .map(|(left, right)| {
                conditional_expr(
                    &format!("{} >= {}", args[0].value, args[1].value),
                    left,
                    right,
                )
            })
            .collect(),
        "hypot" => axis(&args[0])
            .iter()
            .zip(axis(&args[1]))
            .map(|(dx, dy)| {
                div_expr(
                    &add_expr(&mul_expr(&args[0].value, dx), &mul_expr(&args[1].value, dy)),
                    value,
                )
            })
            .collect(),
        "atan2" => axis(&args[0])
            .iter()
            .zip(axis(&args[1]))
            .map(|(dy, dx)| {
                div_expr(
                    &sub_expr(&mul_expr(&args[1].value, dy), &mul_expr(&args[0].value, dx)),
                    &add_expr(
                        &mul_expr(&args[1].value, &args[1].value),
                        &mul_expr(&args[0].value, &args[0].value),
                    ),
                )
            })
            .collect(),
        _ => {
            return Err(RustBackendError::unsupported(
                "<generated>",
                "<expr>",
                format!("intrinsic function '{name}'"),
            ));
        }
    };
    Ok(derivatives)
}

fn pow_derivative_expr(
    value: &str,
    base: &str,
    exponent: &str,
    dbase: &str,
    dexponent: &str,
) -> String {
    let integer_exponent_derivative = conditional_expr(
        &format!("{exponent} == 0.0"),
        "0.0",
        &mul_expr(
            exponent,
            &mul_expr(&format!("({base}).powf({exponent} - 1.0)"), dbase),
        ),
    );
    let general_derivative = mul_expr(
        value,
        &add_expr(
            &mul_expr(dexponent, &format!("({base}).ln()")),
            &mul_expr(exponent, &div_expr(dbase, base)),
        ),
    );

    conditional_expr(
        &format!(
            "{dexponent} == 0.0 && (({exponent}) as f64).is_finite() && (({exponent}) as f64).fract() == 0.0"
        ),
        &integer_exponent_derivative,
        &general_derivative,
    )
}

fn reactive_unary(op: &str, operand: &ExprValue) -> Result<ReactiveValue, RustBackendError> {
    if !operand.has_reactive {
        return Ok(ReactiveValue::none(zero_derivatives(
            operand.derivatives.len(),
        )));
    }
    let value = unary_value(op, &operand.reactive_value)?;
    let derivatives = operand
        .reactive_derivatives
        .iter()
        .map(|derivative| unary_value(op, derivative))
        .collect::<Result<Vec<_>, _>>()?;
    let branch_derivatives = operand
        .reactive_branch_derivatives
        .iter()
        .map(|derivative| unary_value(op, derivative))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ReactiveValue {
        has_reactive: true,
        value,
        derivatives,
        branch_derivatives,
    })
}

fn reactive_binary(
    op: &str,
    left: &ExprValue,
    right: &ExprValue,
) -> Result<ReactiveValue, RustBackendError> {
    let count = left.derivatives.len();
    match op {
        "Add" => reactive_add_sub(left, right, "+"),
        "Sub" => reactive_add_sub(left, right, "-"),
        "Mul" => reactive_mul(left, right),
        "Div" => reactive_div(left, right),
        "Mod" => reactive_mod(left, right),
        _ => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            format!("binary operator {op}"),
        )),
    }
    .or_else(|err| {
        if !left.has_reactive && !right.has_reactive {
            Ok(ReactiveValue::none(zero_derivatives(count)))
        } else {
            Err(err)
        }
    })
}

fn reactive_add_sub(
    left: &ExprValue,
    right: &ExprValue,
    op: &str,
) -> Result<ReactiveValue, RustBackendError> {
    match (left.has_reactive, right.has_reactive) {
        (false, false) => Ok(ReactiveValue::none(zero_derivatives(
            left.derivatives.len(),
        ))),
        (true, false) if op == "+" => Ok(ReactiveValue {
            has_reactive: true,
            value: left.reactive_value.clone(),
            derivatives: left.reactive_derivatives.clone(),
            branch_derivatives: left.reactive_branch_derivatives.clone(),
        }),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: left.reactive_value.clone(),
            derivatives: left.reactive_derivatives.clone(),
            branch_derivatives: left.reactive_branch_derivatives.clone(),
        }),
        (false, true) if op == "+" => Ok(ReactiveValue {
            has_reactive: true,
            value: right.reactive_value.clone(),
            derivatives: right.reactive_derivatives.clone(),
            branch_derivatives: right.reactive_branch_derivatives.clone(),
        }),
        (false, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: unary_value("Neg", &right.reactive_value)?,
            derivatives: right
                .reactive_derivatives
                .iter()
                .map(|derivative| unary_value("Neg", derivative))
                .collect::<Result<Vec<_>, _>>()?,
            branch_derivatives: right
                .reactive_branch_derivatives
                .iter()
                .map(|derivative| unary_value("Neg", derivative))
                .collect::<Result<Vec<_>, _>>()?,
        }),
        (true, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: if op == "+" {
                add_expr(&left.reactive_value, &right.reactive_value)
            } else {
                sub_expr(&left.reactive_value, &right.reactive_value)
            },
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left, right)| {
                    if op == "+" {
                        add_expr(left, right)
                    } else {
                        sub_expr(left, right)
                    }
                })
                .collect(),
            branch_derivatives: left
                .reactive_branch_derivatives
                .iter()
                .zip(&right.reactive_branch_derivatives)
                .map(|(left, right)| {
                    if op == "+" {
                        add_expr(left, right)
                    } else {
                        sub_expr(left, right)
                    }
                })
                .collect(),
        }),
    }
}

fn reactive_mul(left: &ExprValue, right: &ExprValue) -> Result<ReactiveValue, RustBackendError> {
    match (left.has_reactive, right.has_reactive) {
        (false, false) => Ok(ReactiveValue::none(zero_derivatives(
            left.derivatives.len(),
        ))),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: mul_expr(&left.reactive_value, &right.value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.derivatives)
                .map(|(left_derivative, right_derivative)| {
                    add_expr(
                        &mul_expr(left_derivative, &right.value),
                        &mul_expr(&left.reactive_value, right_derivative),
                    )
                })
                .collect(),
            branch_derivatives: left
                .reactive_branch_derivatives
                .iter()
                .zip(&right.branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    add_expr(
                        &mul_expr(left_derivative, &right.value),
                        &mul_expr(&left.reactive_value, right_derivative),
                    )
                })
                .collect(),
        }),
        (false, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: mul_expr(&left.value, &right.reactive_value),
            derivatives: left
                .derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    add_expr(
                        &mul_expr(left_derivative, &right.reactive_value),
                        &mul_expr(&left.value, right_derivative),
                    )
                })
                .collect(),
            branch_derivatives: left
                .branch_derivatives
                .iter()
                .zip(&right.reactive_branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    add_expr(
                        &mul_expr(left_derivative, &right.reactive_value),
                        &mul_expr(&left.value, right_derivative),
                    )
                })
                .collect(),
        }),
        (true, true) => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            "ddt() on both sides of a product",
        )),
    }
}

fn reactive_mod(left: &ExprValue, right: &ExprValue) -> Result<ReactiveValue, RustBackendError> {
    match (left.has_reactive, right.has_reactive) {
        (false, false) => Ok(ReactiveValue::none(zero_derivatives(
            left.derivatives.len(),
        ))),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: mod_expr(&left.reactive_value, &right.value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.reactive_value,
                        &right.value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
            branch_derivatives: left
                .reactive_branch_derivatives
                .iter()
                .zip(&right.branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.reactive_value,
                        &right.value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
        }),
        (false, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: mod_expr(&left.value, &right.reactive_value),
            derivatives: left
                .derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.value,
                        &right.reactive_value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
            branch_derivatives: left
                .branch_derivatives
                .iter()
                .zip(&right.reactive_branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.value,
                        &right.reactive_value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
        }),
        (true, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: mod_expr(&left.reactive_value, &right.reactive_value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.reactive_value,
                        &right.reactive_value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
            branch_derivatives: left
                .reactive_branch_derivatives
                .iter()
                .zip(&right.reactive_branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    mod_derivative_expr(
                        &left.reactive_value,
                        &right.reactive_value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
        }),
    }
}

fn reactive_div(left: &ExprValue, right: &ExprValue) -> Result<ReactiveValue, RustBackendError> {
    match (left.has_reactive, right.has_reactive) {
        (false, false) => Ok(ReactiveValue::none(zero_derivatives(
            left.derivatives.len(),
        ))),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: div_expr(&left.reactive_value, &right.value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.derivatives)
                .map(|(left_derivative, right_derivative)| {
                    div_derivative_expr(
                        &left.reactive_value,
                        &right.value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
            branch_derivatives: left
                .reactive_branch_derivatives
                .iter()
                .zip(&right.branch_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    div_derivative_expr(
                        &left.reactive_value,
                        &right.value,
                        left_derivative,
                        right_derivative,
                    )
                })
                .collect(),
        }),
        (false, true) | (true, true) => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            "ddt() in denominator of reactive expression",
        )),
    }
}

fn is_ddt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddt")
}

fn is_idt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("idt")
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

pub(crate) fn is_analysis_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("analysis")
}

pub(crate) fn normalize_analysis_query(name: &str) -> Option<String> {
    let normalized = name.to_ascii_lowercase();
    match normalized.as_str() {
        "dc" | "op" => Some("dc".to_string()),
        "ac" => Some("ac".to_string()),
        "tran" | "transient" => Some("tran".to_string()),
        "noise" => Some("noise".to_string()),
        "ic" => Some("ic".to_string()),
        "static" => Some("static".to_string()),
        "smallsig" | "smallsignal" | "small_signal" => Some("smallsig".to_string()),
        _ => None,
    }
}

pub(crate) fn analysis_predicate_expr(query: &str) -> &'static str {
    match query {
        "dc" => "ctx.analysis_dc()",
        "ac" => "ctx.analysis_ac()",
        "tran" => "ctx.analysis_tran()",
        "noise" => "ctx.analysis_noise()",
        "ic" => "ctx.analysis_ic()",
        "static" => "ctx.analysis_static()",
        "smallsig" => "ctx.analysis_smallsig()",
        _ => "false",
    }
}

pub(crate) fn is_intrinsic_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "abs"
            | "fabs"
            | "sqrt"
            | "exp"
            | "limexp"
            | "__rspice_limited_exp"
            | "ln"
            | "log"
            | "log10"
            | "sin"
            | "cos"
            | "tan"
            | "atan"
            | "sinh"
            | "cosh"
            | "tanh"
            | "asinh"
            | "acosh"
            | "atanh"
            | "floor"
            | "ceil"
            | "pow"
            | "min"
            | "max"
            | "hypot"
            | "atan2"
    )
}

fn is_binary_intrinsic_name(name: &str) -> bool {
    matches!(name, "pow" | "min" | "max" | "hypot" | "atan2")
}

pub(crate) fn comparison_operator(op: &str) -> Option<&'static str> {
    match op {
        "Eq" => Some("=="),
        "Ne" => Some("!="),
        "Lt" => Some("<"),
        "Le" => Some("<="),
        "Gt" => Some(">"),
        "Ge" => Some(">="),
        _ => None,
    }
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

fn negate_value(value: &str) -> String {
    let value = value.trim();
    if value == "0.0" || value == "-0.0" {
        "0.0".to_string()
    } else if let Some(positive) = value.strip_prefix('-') {
        if scan_numeric_literal(positive).is_some() {
            positive.to_string()
        } else {
            format!("(-{value})")
        }
    } else {
        format!("(-{value})")
    }
}

fn f64_binary_receiver(value: &str) -> String {
    if let Some(typed) = typed_f64_literal(value) {
        format!("({typed})")
    } else {
        format!("({value})")
    }
}

fn typed_f64_literal(value: &str) -> Option<String> {
    let value = value.trim();
    if value.ends_with("_f64") || value.starts_with("f64::") {
        return None;
    }
    let saw_float_marker = scan_numeric_literal(value)?;
    if saw_float_marker {
        Some(format!("{value}_f64"))
    } else {
        None
    }
}

fn scan_numeric_literal(value: &str) -> Option<bool> {
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    if unsigned.is_empty() || !unsigned.as_bytes()[0].is_ascii_digit() {
        return None;
    }

    let mut saw_digit = false;
    let mut saw_float_marker = false;
    let mut previous_was_exponent = false;
    for byte in unsigned.bytes() {
        match byte {
            b'0'..=b'9' => {
                saw_digit = true;
                previous_was_exponent = false;
            }
            b'.' => {
                saw_float_marker = true;
                previous_was_exponent = false;
            }
            b'e' | b'E' => {
                saw_float_marker = true;
                previous_was_exponent = true;
            }
            b'+' | b'-' if previous_was_exponent => {
                previous_was_exponent = false;
            }
            _ => return None,
        }
    }

    if saw_digit {
        Some(saw_float_marker)
    } else {
        None
    }
}
