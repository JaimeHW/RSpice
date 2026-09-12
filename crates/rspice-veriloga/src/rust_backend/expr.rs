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

use std::collections::HashMap;

use crate::ast::AccessKind;
use crate::canonical_ir::{CanonicalIrArtifact, ExprId, HirExprKind};

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

        let value_expr = match &expression.kind {
            HirExprKind::Number { value, .. } => format_f64(*value),
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str())?,
            HirExprKind::BranchAccess {
                kind: access,
                pos,
                neg,
                ..
            } => self.lower_branch_access(access, pos.as_str(), neg.as_deref())?,
            HirExprKind::NamedBranchAccess {
                kind: access, name, ..
            } => self.lower_named_branch_access(access, name.as_str())?,
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
                self.lower_ddt_value(args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                let (expr, ic) = self.idt_operands(args.as_slice())?;
                self.lower_idt_value(id, expr, ic)?
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                self.lower_analysis_value(args.as_slice(), &base)?
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                let (expr, probe) = self.ddx_operands(args.as_slice())?;
                self.lower_ddx_value(expr, probe, &base)?
            }
            HirExprKind::Call { name, args } if name.eq_ignore_ascii_case("slew") => {
                match args.as_slice() {
                    [expr] => self.lower(*expr)?.value,
                    [_, ..] => {
                        return Err(self.unsupported(
                        "rate-limited slew analog operator is unsupported by generated Rust; rate limiting cannot be degraded to a passthrough",
                    ));
                    }
                    [] => return Err(self.unsupported("slew analog operator without an input")),
                }
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => "0.0".to_string(),
            HirExprKind::Call { name, args } if is_intrinsic_name(name.as_str()) => {
                self.lower_intrinsic_value(name.as_str(), args.as_slice(), &base)?
            }
            HirExprKind::NoiseSource { .. } => "0.0".to_string(),
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

    fn lower_branch_access(
        &self,
        access: &AccessKind,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<String, RustBackendError> {
        if *access == AccessKind::Flow {
            if let Some(slot) = self.branch_current_slot_for_nodes(pos, neg)? {
                return Ok(
                    slot.signed_value(format!("ctx.branch_current(self.branches[{}])", slot.slot))
                );
            }
            return Err(self.unsupported("flow branch access in expression".to_string()));
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
        access: &AccessKind,
        name: &str,
    ) -> Result<String, RustBackendError> {
        if *access == AccessKind::Flow {
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
            "limexp" => limexp_value_expr(&args[0].value),
            "__rspice_limited_exp" => limited_exp_value_expr(&args[0].value),
            _ => pure_intrinsic_value(
                &normalized,
                &args[0].value,
                args.get(1).map(|argument| argument.value.as_str()),
            )
            .ok_or_else(|| self.unsupported(format!("intrinsic function '{name}'")))?,
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
            // `$abstime` and nothing else: `$realtime` counts the declaring
            // module's time units, and semantic analysis rewrites it into
            // `$abstime` over that unit while the unit is in scope. Emitting
            // `self.time` for the spelling would put seconds in the generated
            // device, so an unrewritten one falls through to the
            // unsupported-system-function refusal. Pinned by
            // `module_timing.rs`,
            // `every_lowering_route_reads_realtime_through_the_module_time_unit`.
            "$abstime" => {
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
        if !(1..=2).contains(&args.len()) {
            return Err(self.unsupported(format!(
                "$simparam expects one or two arguments, found {}",
                args.len()
            )));
        }
        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(args[0]))
            .ok_or_else(|| self.internal("simparam name is outside MIR arena"))?;
        let HirExprKind::StringLiteral { value: name } = &expression.kind else {
            return Err(self.unsupported("$simparam requires a literal query name"));
        };
        let Some(parameter) = rspice_veriloga_runtime::SimulationParameter::from_name(name) else {
            return match args.get(1) {
                Some(fallback) => Ok(self.lower(*fallback)?.value),
                None => {
                    Err(self.unsupported(format!("simulation parameter '{name}' has no fallback")))
                }
            };
        };
        let name = parameter.name();
        let required = format!("ctx.simparam_required({name:?})");
        let Some(fallback) = args.get(1) else {
            return Ok(required);
        };
        let fallback = self.lower_isolated_branch(*fallback)?;
        Ok(format!(
            "if ctx.has_simparam({name:?}) {{ {required} }} else {{ {} {} }}",
            fallback.lines.join(" "),
            fallback.value.value
        ))
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

    fn lower_ddt_value(&mut self, args: &[ExprId]) -> Result<String, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self.lower(operand_id)?;
        // Noise powers use the DC primal and never touch transient companions.
        Ok(format!(
            "ctx.checked_derivative_value({}, 0.0)",
            operand.value
        ))
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
    ) -> Result<String, RustBackendError> {
        let input = self.lower(expr)?;
        let ic = ic.ok_or_else(|| {
            self.internal(format!(
                "implicit idt expression {id} was not materialized before noise lowering"
            ))
        })?;
        let ic = self.lower(ic)?;
        Ok(format!(
            "ctx.checked_derivative_value({}, {})",
            input.value, ic.value
        ))
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
            HirExprKind::BranchAccess {
                kind: access,
                pos,
                neg,
                ..
            } if *access == AccessKind::Potential => (
                self.node_index(pos.as_str())?,
                neg.as_deref()
                    .map(|node| self.node_index(node))
                    .transpose()?
                    .flatten(),
            ),
            HirExprKind::NamedBranchAccess {
                kind: access, name, ..
            } if *access == AccessKind::Potential => {
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

// The two emitters below spell the thresholds as literal text because the
// generated bundle's digest is a function of those bytes, and formatting them
// from a constant would make the bundle hostage to how `f64` happens to
// `Debug`-print. These assertions are what keeps the literals and the
// workspace's one ruling from drifting apart: change
// `rspice_veriloga_runtime::LIMEXP_THRESHOLD` and this crate stops compiling
// until the emitted text follows.
const _: () = assert!(rspice_veriloga_runtime::LIMEXP_THRESHOLD == 80.0);
const _: () = assert!(rspice_veriloga_runtime::LIMITED_EXP_THRESHOLD == 80.0);
const _: () = assert!(rspice_veriloga_runtime::LIMITED_EXP_FLOOR == 1.804851387e-35);

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
    fn mathematical_noise_values_preserve_zero_signs_and_domains() {
        let mut source = String::from(
            "struct Context; impl Context { fn has_simparam(&self, name: &str) -> bool { matches!(name, \"tnom\"|\"pnjmaxi\") } fn simparam_required(&self, name: &str) -> f64 { match name { \"tnom\"=>27.0, \"pnjmaxi\"=>2.0, _=>panic!(\"missing query\") } } } fn main() { let ctx=Context; for x in [-2.0_f64,2.0,-0.0,0.0] { let params=[x];\n",
        );
        for (index, expression) in [
            "atan2(0.0*x,-1.0)",
            "atan2(0.0/x,-1.0)",
            "atan2(-0.0,-1.0)",
            "atan2(0.0+x,-1.0)",
            "atan2(x-(-0.0),-1.0)",
            "atan2(x>0.0 ? -0.0 : 0.0,-1.0)",
            "0.0/(x-x)",
            "$simparam(\"tnom\",x)",
            "$simparam(\"pnjmaxi\",1.0/(x-x))",
            "$simparam(\"unavailable\",x)",
        ]
        .into_iter()
        .enumerate()
        {
            let artifact=crate::VerilogACompiler::default().compile_canonical_ir(&format!(
                "module noise_value(p); inout p; electrical p; parameter real x=2.0; analog I(p)<+white_noise(4.0+({expression}),\"value\"); endmodule"
            )).unwrap();
            let lowered = super::lower_noise_value_expr(
                &artifact,
                artifact.noise_sources.sources[0].psd.id,
                &format!("n{index}"),
                &super::parameter_field_names(&artifact),
                &Default::default(),
                &Default::default(),
            )
            .unwrap();
            let expected = match index {
                0 => "(0.0*x).atan2(-1.0)",
                1 => "(0.0/x).atan2(-1.0)",
                2 => "(-0.0_f64).atan2(-1.0)",
                3 => "(0.0+x).atan2(-1.0)",
                4 => "(x-(-0.0)).atan2(-1.0)",
                5 => "(if x>0.0 {-0.0_f64} else {0.0_f64}).atan2(-1.0)",
                6 => "0.0/(x-x)",
                7 => "27.0_f64",
                8 => "2.0_f64",
                _ => "x",
            };
            source.push_str(&format!(
                "{{ {} let actual={}; let expected=4.0+({expected}); assert!((expected.is_nan() && actual.is_nan()) || expected.to_bits()==actual.to_bits(),\"case {index}, x={{x:?}}, expected {{expected:?}}, got {{actual:?}}\"); }}\n",
                lowered.lines.join("\n"),lowered.value,
            ));
        }
        source.push_str("} }");
        let directory =
            std::env::temp_dir().join(format!("rspice-noise-math-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let input = directory.join("noise_math.rs");
        let executable = directory
            .join("noise_math")
            .with_extension(std::env::consts::EXE_EXTENSION);
        std::fs::write(&input, source).unwrap();
        let built = std::process::Command::new("rustc")
            .args(["--edition=2024", "-C", "debuginfo=0"])
            .arg(&input)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let ran = std::process::Command::new(&executable).output().unwrap();
        std::fs::remove_file(&input).unwrap();
        std::fs::remove_file(&executable).unwrap();
        let _ = std::fs::remove_file(executable.with_extension("pdb"));
        std::fs::remove_dir(&directory).unwrap();
        assert!(
            ran.status.success(),
            "{}",
            String::from_utf8_lossy(&ran.stderr)
        );
    }

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
        "ToInteger" => Ok(format!(
            "ctx.integer_result({})",
            integer_cast_result(operand)
        )),
        "BitNot" => Ok(format!(
            "ctx.integer_result({})",
            integer_binary_result("BitXor", operand, "-1.0")
        )),
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
        "BitAnd" | "BitOr" | "BitXor" | "Shl" | "Shr" | "IntAdd" | "IntSub" | "IntMul"
        | "IntDiv" | "IntMod" | "IntPow" => Ok(format!(
            "ctx.integer_result({})",
            integer_binary_result(op, left, right)
        )),
        "Add" => Ok(format!("({left} + {right})")),
        "Sub" => Ok(format!("({left} - {right})")),
        "Mul" => Ok(format!("({left} * {right})")),
        "Div" => Ok(format!("({left} / {right})")),
        // Even a zero numerator must evaluate its divisor: 0 % 0 is invalid.
        "Mod" => Ok(format!("({left} % {right})")),
        "Pow" => Ok(power_value_expr(left, right)),
        _ => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            format!("binary operator {op}"),
        )),
    }
}

pub(super) fn integer_cast_result(operand: &str) -> String {
    format!("integer::real_to_integer({operand}).map(f64::from)")
}

pub(super) fn integer_binary_result(op: &str, left: &str, right: &str) -> String {
    // Assignment conversion lowers to an integer OR with zero. Keep its
    // generated form as one checked conversion, without a redundant OR.
    if let Some(op) = crate::ast::BinaryOp::integer_arithmetic_from_name(op) {
        return format!(
            "integer::integer_arithmetic(integer::IntegerArithmeticOperation::{op:?}, {left}, {right})"
        );
    }
    if op == "BitOr" && matches!(right, "0.0" | "-0.0" | "0f64" | "-0f64") {
        integer_cast_result(left)
    } else {
        format!("integer::integer_binary(integer::IntegerBinaryOperation::{op}, {left}, {right})")
    }
}

/// Shared value emission for mathematical noise operands and parameter defaults.
/// The caller has lowered operands in source order; the optional second operand
/// also checks arity without silently discarding an argument.
pub(super) fn pure_intrinsic_value(name: &str, left: &str, right: Option<&str>) -> Option<String> {
    let method = match (name, right) {
        ("pow", Some(right)) => return Some(power_value_expr(left, right)),
        ("min" | "max", Some(right)) => {
            return Some(extremum_value_expr(left, right, name == "min"));
        }
        ("hypot" | "atan2", Some(right)) => {
            return Some(format!("{}.{name}({right})", f64_binary_receiver(left)));
        }
        ("abs" | "fabs", None) => "abs",
        ("ln" | "log", None) => "ln",
        (
            "sqrt" | "exp" | "log10" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh"
            | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" | "floor" | "ceil",
            None,
        ) => name,
        _ => return None,
    };
    Some(format!("{}.{method}()", f64_binary_receiver(left)))
}

/// Mirror the shared runtime's numeric-over-NaN and left-tie selection rule.
/// Operands are already lowered values, so selection cannot repeat effects.
pub(super) fn extremum_value_expr(left: &str, right: &str, minimum: bool) -> String {
    let comparison = if minimum { "<=" } else { ">=" };
    let left_receiver = f64_binary_receiver(left);
    let right_receiver = f64_binary_receiver(right);
    format!(
        "if !{left_receiver}.is_nan()&&({left}{comparison}{right}||{right_receiver}.is_nan()){{{left}}}else{{{right}}}"
    )
}

pub(super) fn power_value_expr(base: &str, exponent: &str) -> String {
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
    rspice_veriloga_runtime::analysis_query_id(name)
        .and_then(rspice_veriloga_runtime::analysis_query_name)
        .map(str::to_string)
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
        "nodeset" => "ctx.analysis_nodeset()",
        "__rspice_scope_dc" => "ctx.analysis_dc()",
        "__rspice_scope_ac" => "ctx.analysis_ac()",
        "__rspice_scope_tran" => "ctx.analysis_tran()",
        "__rspice_scope_noise" => "ctx.analysis_noise()",
        "__rspice_scope_ic" => "(ctx.analysis_code() == 4)",
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
            | "asin"
            | "acos"
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
    if let Some(positive) = value.strip_prefix('-') {
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
    if let Some(typed) = typed_f64_literal(trim_enclosing_parentheses(value)) {
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
