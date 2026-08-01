//! Lowering canonical IR *values* to Rust expression text, for noise.
//!
//! This used to be the shared expression emitter for the whole backend, and it
//! carried a derivative lane per node and per branch unknown alongside every
//! value. The canonical CFG emitter took that job: it differentiates over the
//! CFG, where a guard is a branch rather than a flattened product, and it does
//! not call in here.
//!
//! What remains is the one caller left — [`super::noise`], lowering PSDs, their
//! exponents, and the conditions guarding them. Those are values; nothing in a
//! noise block is differentiated. So a [`LoweredExpr`] is a value and the lines
//! that bind it, and the derivative machinery that used to live here is gone
//! rather than dormant.
//!
//! Branch currents still get dedicated handling, because a branch unknown is
//! the one case where an expression's value comes from solver state rather than
//! from another emitted value. `ddt` and `idt` are refused: a noise expression
//! has no state table to bind a slot in.

use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{CanonicalIrArtifact, ExprId, HirAnalogOperator, HirExprKind};

use super::RustBackendError;

#[derive(Debug, Clone)]
pub struct LoweredExpr {
    pub lines: Vec<String>,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct LoweredVariable {
    pub value: String,
    pub condition: Option<String>,
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

/// Lower one noise expression to straight-line Rust.
///
/// The only entry point into this emitter. Phase 6 deleted the five siblings
/// it used to share `lower_expr_with_variables` with — equation, assignment,
/// value-assignment and the two reactive forms — when the canonical CFG
/// emitter took over every path that produced a derivative. What is left
/// lowers a *value*: noise PSDs, their exponents, and the conditions guarding
/// them. Nothing here differentiates anything.
pub(crate) fn lower_noise_value_expr(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> Result<LoweredExpr, RustBackendError> {
    let mut emitter = ExprEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        branch_current_unknowns,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };
    let value = emitter.lower(expr)?;
    Ok(LoweredExpr {
        lines: emitter.lines,
        value: value.value,
    })
}

#[derive(Debug, Clone)]
struct ExprValue {
    value: String,
}

struct ExprEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
    variables: &'a HashMap<String, LoweredVariable>,
    branch_current_unknowns: &'a HashMap<String, BranchCurrentSlot>,
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

        let lowered = ExprValue { value: value_expr };
        self.emitted.insert(id, lowered.clone());
        Ok(lowered)
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("params[{field}]"))
        } else if let Some(variable) = self.variables.get(name) {
            Ok(variable.value.clone())
        } else {
            Err(self.unsupported(format!(
                "identifier '{name}' is not a parameter or scalar variable"
            )))
        }
    }

    fn zero_value(&self) -> ExprValue {
        ExprValue {
            value: "0.0".to_string(),
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
            // A branch current is never a known zero here: noise lowering runs
            // with no contributions in scope, so there is nothing to inspect.
            HirExprKind::NamedBranchAccess { access, .. } if access.as_str() == "I" => Ok(false),
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
            if let Some(slot) = self.branch_current_unknowns.get(name) {
                return Ok(
                    slot.signed_value(format!("ctx.branch_current(self.branches[{}])", slot.slot))
                );
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

        self.lines.push(lazy_conditional_tuple(
            &names,
            &condition,
            &then_branch.lines,
            &then_values,
            &else_branch.lines,
            &else_values,
        ));

        Ok(ExprValue { value: base })
    }

    fn lower_isolated_branch(&self, expr: ExprId) -> Result<ConditionalBranch, RustBackendError> {
        let mut branch = ExprEmitter {
            artifact: self.artifact,
            prefix: self.prefix,
            parameter_fields: self.parameter_fields,
            variables: self.variables,
            branch_current_unknowns: self.branch_current_unknowns,
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
            "pow" => power_value_expr(&args[0].value, &args[1].value),
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
        if args.is_empty() {
            return Err(self.unsupported("analysis expects at least one argument"));
        }
        let mut predicates = Vec::with_capacity(args.len());
        for name in args {
            let expression = self
                .artifact
                .mir
                .expressions
                .get(usize::from(*name))
                .ok_or_else(|| {
                    self.internal(format!("analysis query {name} is outside MIR arena"))
                })?;
            let HirExprKind::StringLiteral { value } = &expression.kind else {
                return Err(self.unsupported("analysis expects string literal arguments"));
            };
            let predicate = normalize_analysis_query(value)
                .map(|query| analysis_predicate_expr(&query))
                .unwrap_or("false");
            predicates.push(predicate);
        }
        if predicates.len() == 1 {
            Ok(predicates[0].to_string())
        } else {
            Ok(format!("({})", predicates.join(" || ")))
        }
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

    fn lower_intrinsic_args(
        &mut self,
        normalized: &str,
        args: &[ExprId],
    ) -> Result<Vec<ExprValue>, RustBackendError> {
        self.validate_intrinsic_arity(normalized, args)?;
        args.iter().map(|arg| self.lower(*arg)).collect()
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
        // Lowered for its side effects — the operand's own diagnostics fire
        // before this one, exactly as they did when a slot lookup followed.
        self.lower(operand_id)?;
        let _ = base;
        // Noise lowering carries no `ddt` state table, so there is no slot to
        // bind. The refusal, and its wording, are what the empty table
        // produced before the table itself was removed.
        Err(self.internal(format!("ddt expression {id} has no generated state slot")))
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
        // Both operands are lowered for their side effects, so their own
        // diagnostics still fire first, exactly as they did when a slot lookup
        // followed them.
        self.lower(expr)?;
        if let Some(ic) = ic {
            self.lower(ic)?;
        }
        let _ = base;
        // As for `ddt`: noise lowering has no `idt` state table to bind a slot
        // in, and this is the refusal the empty table produced.
        Err(self.internal(format!("idt expression {id} has no generated state slot")))
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
        self.lower(expr)?;
        let projection = self.ddx_projection(probe)?;
        Ok(self.emit_value(base, projection))
    }

    /// The value of a `ddx` inside a noise expression.
    ///
    /// This emitter carries no derivative lanes, so every lane the projection
    /// could select is zero and the result is a constant. The probe is still
    /// resolved and validated — an unknown branch or a current probe is an
    /// error here exactly as it was when the lanes were real — and the text
    /// returned is what projecting over a zero vector used to produce.
    fn ddx_projection(&self, probe: ExprId) -> Result<String, RustBackendError> {
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
        let _ = pos;
        if neg.is_some() {
            Ok("(0.5 * (0.0 - 0.0))".to_string())
        } else {
            Ok("0.0".to_string())
        }
    }

    fn ddx_operands(&self, args: &[ExprId]) -> Result<(ExprId, ExprId), RustBackendError> {
        match args {
            [expr, probe] => Ok((*expr, *probe)),
            _ => Err(self.unsupported(format!("ddx expects two operands, found {}", args.len()))),
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

pub fn parameter_field_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    artifact
        .mir
        .parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| (parameter.name.to_string(), index.to_string()))
        .collect()
}

fn lowered_variable_is_constant_zero(variable: &LoweredVariable) -> bool {
    is_zero_derivative(&variable.value)
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
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => {
            children.extend([*proposed, *candidate]);
            children.extend(type_metadata.iter().copied());
        }
        HirAnalogOperator::LimiterArgument { .. } => {}
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

fn limexp_value_expr(arg: &str) -> String {
    format!(
        "{{ let limexp_arg = {arg}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) }} }}"
    )
}

fn limited_exp_value_expr(arg: &str) -> String {
    format!(
        "{{ let limited_exp_arg = {arg}; if limited_exp_arg > 80.0 {{ LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) }} else if limited_exp_arg < -80.0 {{ 1.804851387e-35 }} else {{ limited_exp_arg.exp() }} }}"
    )
}

fn is_zero_derivative(derivative: &str) -> bool {
    let derivative = strip_redundant_outer_parens(derivative.trim());
    derivative == "0.0" || derivative == "-0.0" || product_derivative_has_zero_factor(derivative)
}

fn is_one_derivative(derivative: &str) -> bool {
    derivative.trim() == "1.0"
}

fn is_negative_one_derivative(derivative: &str) -> bool {
    derivative.trim() == "-1.0"
}

fn product_derivative_has_zero_factor(derivative: &str) -> bool {
    let mut depth = 0usize;
    let mut factor_start = 0usize;
    let mut saw_product = false;
    for (index, ch) in derivative.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => depth = depth.saturating_sub(1),
            '*' if depth == 0 => {
                saw_product = true;
                if is_zero_derivative(&derivative[factor_start..index]) {
                    return true;
                }
                factor_start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    saw_product && is_zero_derivative(&derivative[factor_start..])
}

fn strip_redundant_outer_parens(mut value: &str) -> &str {
    loop {
        let trimmed = value.trim();
        let Some(inner) = trimmed
            .strip_prefix('(')
            .and_then(|candidate| candidate.strip_suffix(')'))
        else {
            return trimmed;
        };
        if !outer_parens_wrap_entire_expr(trimmed) {
            return trimmed;
        }
        value = inner;
    }
}

fn outer_parens_wrap_entire_expr(value: &str) -> bool {
    let mut depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 && index + ch.len_utf8() != value.len() {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
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

#[cfg(test)]
mod tests {
    use super::{binary_value, power_value_expr};

    #[test]
    fn expression_backend_specializes_integer_power_values() {
        assert_eq!(power_value_expr("x", "0.0"), "1.0");
        assert_eq!(power_value_expr("x", "1.0"), "x");
        assert_eq!(power_value_expr("x", "2.0"), "{let pb=x;pb*pb}");
        assert_eq!(power_value_expr("x", "(3.0)"), "{let pb=x;pb*pb*pb}");
        assert_eq!(
            power_value_expr("x", "4.0_f64"),
            "{let pb=x;let ps=pb*pb;ps*ps}"
        );
        assert_eq!(power_value_expr("x", "0.5"), "(x).powf(0.5)");
        assert_eq!(
            binary_value("Pow", "x", "2.0").expect("pow value"),
            "{let pb=x;pb*pb}"
        );
    }
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
        "Pow" => Ok(power_value_expr(left, right)),
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

fn power_value_expr(base: &str, exponent: &str) -> String {
    if let Some(exponent) = integer_power_exponent_literal(exponent) {
        constant_integer_power_value_expr(base, exponent)
    } else {
        format!("{}.powf({exponent})", f64_binary_receiver(base))
    }
}

fn constant_integer_power_value_expr(base: &str, exponent: i32) -> String {
    match exponent {
        0 => "1.0".to_string(),
        1 => base.to_string(),
        2 => repeated_integer_power_value_expr(base, 2),
        3 => repeated_integer_power_value_expr(base, 3),
        4 => quartic_integer_power_value_expr(base),
        _ => format!("{}.powi({exponent})", f64_binary_receiver(base)),
    }
}

fn repeated_integer_power_value_expr(base: &str, factors: usize) -> String {
    debug_assert!(factors >= 2);
    let mut product = String::from("pb");
    for _ in 1..factors {
        product.push_str("*pb");
    }
    format!("{{let pb={base};{product}}}")
}

fn quartic_integer_power_value_expr(base: &str) -> String {
    format!("{{let pb={base};let ps=pb*pb;ps*ps}}")
}

fn integer_power_exponent_literal(value: &str) -> Option<i32> {
    let value = trim_enclosing_parentheses(value);
    let value = value.strip_suffix("_f64").unwrap_or(value);
    integer_power_exponent_value(value.parse::<f64>().ok()?)
}

fn integer_power_exponent_value(value: f64) -> Option<i32> {
    if !value.is_finite() || value.fract() != 0.0 {
        return None;
    }
    if value < i32::MIN as f64 || value > i32::MAX as f64 {
        return None;
    }
    Some(value as i32)
}

pub(super) fn is_ddt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddt")
}

pub(super) fn is_idt_name(name: &str) -> bool {
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
        "__rspice_initial_step" => Some("__rspice_initial_step".to_string()),
        "__rspice_final_step" => Some("__rspice_final_step".to_string()),
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
        "__rspice_initial_step" => "ctx.analysis_initial_step()",
        "__rspice_final_step" => "ctx.analysis_final_step()",
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
