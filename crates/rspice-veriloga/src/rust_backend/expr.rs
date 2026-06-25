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
}

pub fn lower_equation_expr(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
) -> Result<LoweredExpr, RustBackendError> {
    let mut emitter = ExprEmitter {
        artifact,
        prefix,
        parameter_fields,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };
    let value = emitter.lower(expr)?;
    Ok(LoweredExpr {
        lines: emitter.lines,
        value: value.value,
        derivatives: value.derivatives,
    })
}

#[derive(Debug, Clone)]
struct ExprValue {
    value: String,
    derivatives: Vec<String>,
}

struct ExprEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
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
                self.emit_value(&base, unary_value(op.as_str(), &operand.value)?)
            }
            HirExprKind::Binary { op, left, right } => {
                let left = self.lower(*left)?;
                let right = self.lower(*right)?;
                self.emit_value(&base, binary_value(op.as_str(), &left.value, &right.value)?)
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let expr = self.lower(*expr)?;
                self.emit_value(&base, format!("({}).exp()", expr.value))
            }
            other => {
                return Err(self.unsupported(format!("expression kind {other:?}")));
            }
        };

        let derivatives = match &expression.kind {
            HirExprKind::Number { .. } | HirExprKind::Identifier { .. } => {
                zero_derivatives(node_count)
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
                    .map(|derivative| unary_value(op.as_str(), derivative))
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
                binary_derivatives(op.as_str(), left, right)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => {
                let expr = self
                    .emitted
                    .get(expr)
                    .expect("limexp operand must be emitted before derivative");
                expr.derivatives
                    .iter()
                    .map(|derivative| format!("({value_expr} * {derivative})"))
                    .collect()
            }
            _ => unreachable!("unsupported expression kinds returned earlier"),
        };

        let mut derivative_vars = Vec::with_capacity(derivatives.len());
        for (index, derivative) in derivatives.into_iter().enumerate() {
            let derivative_var = format!("{base}_d_n{index}");
            self.lines
                .push(format!("let {derivative_var}: f64 = {derivative};"));
            derivative_vars.push(derivative_var);
        }

        let lowered = ExprValue {
            value: value_expr,
            derivatives: derivative_vars,
        };
        self.emitted.insert(id, lowered.clone());
        Ok(lowered)
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("self.params.{field}"))
        } else {
            Err(self.unsupported(format!("identifier '{name}' is not a parameter")))
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
