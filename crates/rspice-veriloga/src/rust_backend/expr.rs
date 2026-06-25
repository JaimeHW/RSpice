use std::collections::HashMap;

use crate::canonical_ir::{
    CanonicalIrArtifact, ExprId, HirAnalogOperator, HirExprKind, MirBranchRef,
};

use super::{RustBackendError, sanitize_identifier};

#[derive(Debug, Clone)]
pub struct LoweredExpr {
    pub lines: Vec<String>,
    pub value: String,
    pub derivatives: Vec<String>,
    pub has_reactive: bool,
    pub reactive_value: String,
    pub reactive_derivatives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoweredVariable {
    pub value: String,
    pub derivatives: Vec<String>,
    pub has_reactive: bool,
    pub reactive_value: String,
    pub reactive_derivatives: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DdtSlots {
    slots: HashMap<ExprId, usize>,
}

impl DdtSlots {
    pub fn new(slots: HashMap<ExprId, usize>) -> Self {
        Self { slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    fn slot_for(&self, expr: ExprId) -> Option<usize> {
        self.slots.get(&expr).copied()
    }
}

pub fn lower_equation_expr_with_variables(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        ExprMode::Transient,
    )
}

pub fn lower_reactive_expr_with_variables(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
) -> Result<LoweredExpr, RustBackendError> {
    lower_expr_with_variables(
        artifact,
        expr,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        ExprMode::Reactive,
    )
}

fn lower_expr_with_variables(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    mode: ExprMode,
) -> Result<LoweredExpr, RustBackendError> {
    let mut emitter = ExprEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        mode,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };
    let value = emitter.lower(expr)?;
    Ok(LoweredExpr {
        lines: emitter.lines,
        value: value.value,
        derivatives: value.derivatives,
        has_reactive: value.has_reactive,
        reactive_value: value.reactive_value,
        reactive_derivatives: value.reactive_derivatives,
    })
}

#[derive(Debug, Clone, Copy)]
enum ExprMode {
    Transient,
    Reactive,
}

#[derive(Debug, Clone)]
struct ExprValue {
    value: String,
    derivatives: Vec<String>,
    has_reactive: bool,
    reactive_value: String,
    reactive_derivatives: Vec<String>,
}

struct ExprEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
    variables: &'a HashMap<String, LoweredVariable>,
    ddt_slots: &'a DdtSlots,
    mode: ExprMode,
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
        let base = format!("{}_e{}", self.prefix, id.index());

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
                let operand = self.lower(*operand)?;
                let value = unary_value(op.as_str(), &operand.value)
                    .map_err(|_| self.unsupported(format!("unary operator {op}")))?;
                self.emit_value(&base, value)
            }
            HirExprKind::Binary { op, left, right } => {
                let left = self.lower(*left)?;
                let right = self.lower(*right)?;
                let value = binary_value(op.as_str(), &left.value, &right.value)
                    .map_err(|_| self.unsupported(format!("binary operator {op}")))?;
                self.emit_value(&base, value)
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.lower_ddt_value(id, args.as_slice(), &base)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.lower_ddt_value(id, &[*expr], &base)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let _ = expr;
                return Err(self.unsupported("convergence-limited limexp operator"));
            }
            other => {
                return Err(self.unsupported(format!("expression kind {other:?}")));
            }
        };

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
                        pos_node: branch.pos_node,
                        neg_node: branch.neg_node,
                    },
                )?
            }
            HirExprKind::Unary { op, operand } => {
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
            HirExprKind::Binary { op, left, right } => {
                let left = self
                    .emitted
                    .get(left)
                    .expect("left operand must be emitted before binary derivative");
                let right = self
                    .emitted
                    .get(right)
                    .expect("right operand must be emitted before binary derivative");
                binary_derivatives(op.as_str(), left, right)
                    .map_err(|_| self.unsupported(format!("binary operator {op}")))?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.ddt_derivatives(id, args.as_slice())?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.ddt_derivatives(id, &[*expr])?
            }
            _ => unreachable!("unsupported expression kinds returned earlier"),
        };

        let reactive = match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {
                ReactiveValue::none(zero_derivatives(node_count))
            }
            HirExprKind::Identifier { name } => {
                if let Some(variable) = self.variables.get(name.as_str()) {
                    ReactiveValue {
                        has_reactive: variable.has_reactive,
                        value: variable.reactive_value.clone(),
                        derivatives: variable.reactive_derivatives.clone(),
                    }
                } else {
                    ReactiveValue::none(zero_derivatives(node_count))
                }
            }
            HirExprKind::Unary { op, operand } => {
                let operand = self
                    .emitted
                    .get(operand)
                    .expect("operand must be emitted before unary reactive derivative");
                reactive_unary(op.as_str(), operand)
                    .map_err(|_| self.unsupported(format!("unary operator {op}")))?
            }
            HirExprKind::Binary { op, left, right } => {
                let left = self
                    .emitted
                    .get(left)
                    .expect("left operand must be emitted before binary reactive derivative");
                let right = self
                    .emitted
                    .get(right)
                    .expect("right operand must be emitted before binary reactive derivative");
                reactive_binary(op.as_str(), left, right)
                    .map_err(|_| self.unsupported(format!("binary operator {op}")))?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.ddt_reactive_value(args.as_slice())?
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
                op: HirAnalogOperator::Limexp { .. },
            } => unreachable!("limexp rejected before reactive lowering"),
            _ => unreachable!("unsupported expression kinds returned earlier"),
        };

        let mut derivative_vars = Vec::with_capacity(derivatives.len());
        for (index, derivative) in derivatives.into_iter().enumerate() {
            let derivative_var = format!("{base}_d_n{index}");
            self.lines
                .push(format!("let {derivative_var}: f64 = {derivative};"));
            derivative_vars.push(derivative_var);
        }

        let (reactive_value, reactive_derivatives) = if reactive.has_reactive {
            let value_var = format!("{base}_q");
            self.lines
                .push(format!("let {value_var}: f64 = {};", reactive.value));
            let mut derivative_vars = Vec::with_capacity(reactive.derivatives.len());
            for (index, derivative) in reactive.derivatives.into_iter().enumerate() {
                let derivative_var = format!("{base}_q_d_n{index}");
                self.lines
                    .push(format!("let {derivative_var}: f64 = {derivative};"));
                derivative_vars.push(derivative_var);
            }
            (value_var, derivative_vars)
        } else {
            ("0.0".to_string(), zero_derivatives(node_count))
        };

        let lowered = ExprValue {
            value: value_expr,
            derivatives: derivative_vars,
            has_reactive: reactive.has_reactive,
            reactive_value,
            reactive_derivatives,
        };
        self.emitted.insert(id, lowered.clone());
        Ok(lowered)
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("self.params.{field}"))
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
        access: &str,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<String, RustBackendError> {
        if access != "V" {
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
        if access != "V" {
            return Err(self.unsupported(format!("named branch access '{access}' in expression")));
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
        if access != "V" {
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
        if access != "V" {
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
                Ok(self.emit_value(base, format!("self.eval_ddt({slot}, {})", operand.value)))
            }
            ExprMode::Reactive => Ok(operand.value),
        }
    }

    fn ddt_derivatives(
        &self,
        _id: ExprId,
        args: &[ExprId],
    ) -> Result<Vec<String>, RustBackendError> {
        let operand_id = self.ddt_operand(args)?;
        let operand = self
            .emitted
            .get(&operand_id)
            .expect("ddt operand must be emitted before derivative");
        let derivatives = match self.mode {
            ExprMode::Transient => operand
                .derivatives
                .iter()
                .map(|derivative| format!("self.ddt_jacobian({derivative})"))
                .collect(),
            ExprMode::Reactive => operand.derivatives.clone(),
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
        })
    }

    fn ddt_operand(&self, args: &[ExprId]) -> Result<ExprId, RustBackendError> {
        match args {
            [operand] => Ok(*operand),
            [_, _] => Err(self.unsupported("ddt abstol argument")),
            _ => Err(self.unsupported(format!("ddt expects one operand, found {}", args.len()))),
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

#[derive(Debug, Clone)]
struct ReactiveValue {
    has_reactive: bool,
    value: String,
    derivatives: Vec<String>,
}

impl ReactiveValue {
    fn none(derivatives: Vec<String>) -> Self {
        Self {
            has_reactive: false,
            value: "0.0".to_string(),
            derivatives,
        }
    }
}

pub fn parameter_field_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    let names = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.name.to_string())
        .collect::<Vec<_>>();
    unique_identifiers(&names)
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

fn unary_value(op: &str, operand: &str) -> Result<String, RustBackendError> {
    match op {
        "Neg" => Ok(format!("(-{operand})")),
        "Pos" => Ok(format!("({operand})")),
        _ => Err(RustBackendError::unsupported(
            "<generated>",
            "<expr>",
            format!("unary operator {op}"),
        )),
    }
}

fn binary_value(op: &str, left: &str, right: &str) -> Result<String, RustBackendError> {
    let operator = match op {
        "Add" => "+",
        "Sub" => "-",
        "Mul" => "*",
        "Div" => "/",
        _ => {
            return Err(RustBackendError::unsupported(
                "<generated>",
                "<expr>",
                format!("binary operator {op}"),
            ));
        }
    };
    Ok(format!("({left} {operator} {right})"))
}

fn binary_derivatives(
    op: &str,
    left: &ExprValue,
    right: &ExprValue,
) -> Result<Vec<String>, RustBackendError> {
    left.derivatives
        .iter()
        .zip(&right.derivatives)
        .map(|(left_derivative, right_derivative)| match op {
            "Add" => Ok(format!("({left_derivative} + {right_derivative})")),
            "Sub" => Ok(format!("({left_derivative} - {right_derivative})")),
            "Mul" => Ok(format!(
                "(({left_derivative} * {right}) + ({left} * {right_derivative}))",
                left = left.value,
                right = right.value,
            )),
            "Div" => Ok(format!(
                "((({left_derivative} * {right}) - ({left} * {right_derivative})) / ({right} * {right}))",
                left = left.value,
                right = right.value,
            )),
            _ => Err(RustBackendError::unsupported(
                "<generated>",
                "<expr>",
                format!("binary operator {op}"),
            )),
        })
        .collect()
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
    Ok(ReactiveValue {
        has_reactive: true,
        value,
        derivatives,
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
        }),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: left.reactive_value.clone(),
            derivatives: left.reactive_derivatives.clone(),
        }),
        (false, true) if op == "+" => Ok(ReactiveValue {
            has_reactive: true,
            value: right.reactive_value.clone(),
            derivatives: right.reactive_derivatives.clone(),
        }),
        (false, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: format!("(-{})", right.reactive_value),
            derivatives: right
                .reactive_derivatives
                .iter()
                .map(|derivative| format!("(-{derivative})"))
                .collect(),
        }),
        (true, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: format!("({} {op} {})", left.reactive_value, right.reactive_value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left, right)| format!("({left} {op} {right})"))
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
            value: format!("({} * {})", left.reactive_value, right.value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.derivatives)
                .map(|(left_derivative, right_derivative)| {
                    format!(
                        "(({left_derivative} * {right}) + ({left} * {right_derivative}))",
                        left = left.reactive_value,
                        right = right.value,
                    )
                })
                .collect(),
        }),
        (false, true) => Ok(ReactiveValue {
            has_reactive: true,
            value: format!("({} * {})", left.value, right.reactive_value),
            derivatives: left
                .derivatives
                .iter()
                .zip(&right.reactive_derivatives)
                .map(|(left_derivative, right_derivative)| {
                    format!(
                        "(({left_derivative} * {right}) + ({left} * {right_derivative}))",
                        left = left.value,
                        right = right.reactive_value,
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

fn reactive_div(left: &ExprValue, right: &ExprValue) -> Result<ReactiveValue, RustBackendError> {
    match (left.has_reactive, right.has_reactive) {
        (false, false) => Ok(ReactiveValue::none(zero_derivatives(
            left.derivatives.len(),
        ))),
        (true, false) => Ok(ReactiveValue {
            has_reactive: true,
            value: format!("({} / {})", left.reactive_value, right.value),
            derivatives: left
                .reactive_derivatives
                .iter()
                .zip(&right.derivatives)
                .map(|(left_derivative, right_derivative)| {
                    format!(
                        "((({left_derivative} * {right}) - ({left} * {right_derivative})) / ({right} * {right}))",
                        left = left.reactive_value,
                        right = right.value,
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
