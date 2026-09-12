//! Real scalar directions on the prepared, scope-aware expression traversal.
//! Arithmetic stays in the shared behavioral kernel; this adapter owns only
//! leaf directions and operand indexing, without converting a second AST.

use super::eval::PreparedEvaluation;
use super::{BinOpKind, ComplexValue, ExprError, ParamContext, UnaryOpKind, is_real};
use crate::Value;
use crate::config::ExpressionDialect;
use crate::device::behavioral::{
    BehavioralEnvironment, compiled_expression_node_direction, eval_binary_with_derivative,
};
use crate::expr::{BinaryOp, CompiledExpr, Derivative, Expr, Function, LogarithmDomain, compile};
use std::collections::HashMap;

pub(super) struct ScalarDirection<'a, F> {
    resolver: &'a mut F,
    directions: Vec<Derivative>,
    functions: HashMap<(usize, usize), ScalarFunction>,
}

struct ScalarFunction {
    expression: Expr,
    program: CompiledExpr,
    argument_indices: Vec<usize>,
    values: Vec<Value>,
    directions: Vec<Derivative>,
}

fn real(value: ComplexValue) -> Result<Value, ExprError> {
    if is_real(value) {
        Ok(value.re)
    } else {
        Err(ExprError::InvalidArgument(
            "scalar directional evaluation requires real-valued leaves".to_owned(),
        ))
    }
}

impl<'a, F> ScalarDirection<'a, F> {
    pub(super) fn new(resolver: &'a mut F) -> Self {
        Self {
            resolver,
            directions: Vec::new(),
            functions: HashMap::new(),
        }
    }

    fn pop(&mut self) -> Result<Derivative, ExprError> {
        self.directions.pop().ok_or_else(|| {
            ExprError::InvalidArgument("expression direction stack underflow".to_owned())
        })
    }

    pub(super) fn finish(mut self) -> Result<Derivative, ExprError> {
        if self.directions.len() != 1 {
            return Err(ExprError::InvalidArgument(
                "expression direction stack is inconsistent".to_owned(),
            ));
        }
        self.pop()
    }
}

impl<F> PreparedEvaluation for ScalarDirection<'_, F>
where
    F: FnMut(&str) -> Result<Option<(ComplexValue, Derivative)>, ExprError>,
{
    // The real derivative kernel normalizes at its consumer boundary. Applying
    // the complex parameter evaluator's literal folding would change DDX.
    const FOLD_NUMERIC_CONSTANTS: bool = false;

    fn resolve(&mut self, name: &str) -> Result<Option<ComplexValue>, ExprError> {
        (self.resolver)(name)?
            .map(|(value, direction)| {
                real(value)?;
                self.directions.push(direction);
                Ok(value)
            })
            .transpose()
    }

    fn constant(&mut self, value: ComplexValue) -> Result<ComplexValue, ExprError> {
        real(value)?;
        self.directions.push(0.0.into());
        Ok(value)
    }

    fn unary(&mut self, op: UnaryOpKind, value: ComplexValue) -> Result<ComplexValue, ExprError> {
        let direction = self.pop()?;
        let (value, direction) = match op {
            UnaryOpKind::Neg => (-value.re, -direction),
            UnaryOpKind::Pos => (value.re, direction),
            UnaryOpKind::Not => (if value.re == 0.0 { 1.0 } else { 0.0 }, 0.0.into()),
        };
        self.directions.push(direction);
        Ok(value.into())
    }

    fn binary(
        &mut self,
        op: BinOpKind,
        left: ComplexValue,
        right: ComplexValue,
        dialect: ExpressionDialect,
    ) -> Result<ComplexValue, ExprError> {
        let op = match op {
            BinOpKind::Add => BinaryOp::Add,
            BinOpKind::Sub => BinaryOp::Sub,
            BinOpKind::Mul => BinaryOp::Mul,
            BinOpKind::Div => BinaryOp::Div,
            BinOpKind::Mod => BinaryOp::Mod,
            BinOpKind::Pow => BinaryOp::Pow,
            BinOpKind::Gt => BinaryOp::Gt,
            BinOpKind::Lt => BinaryOp::Lt,
            BinOpKind::Ge => BinaryOp::Ge,
            BinOpKind::Le => BinaryOp::Le,
            BinOpKind::Eq => BinaryOp::Eq,
            BinOpKind::Ne => BinaryOp::Ne,
            BinOpKind::And => BinaryOp::And,
            BinOpKind::Or => BinaryOp::Or,
        };
        let right_direction = self.pop()?;
        let left_direction = self.pop()?;
        let (value, direction) = eval_binary_with_derivative(
            op,
            left.re,
            left_direction,
            right.re,
            right_direction,
            dialect,
        )
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!(
                "directional derivative of {op:?} could not be evaluated analytically"
            ))
        })?;
        self.directions.push(direction);
        Ok(value.into())
    }

    fn builtin(
        &mut self,
        name: &str,
        args: &[ComplexValue],
        ctx: &ParamContext,
    ) -> Result<ComplexValue, ExprError> {
        let function = Function::from_name(name)
            .filter(|function| {
                !matches!(
                    function,
                    Function::TableFile
                        | Function::FastTable
                        | Function::FastTableFile
                        | Function::Cubic
                        | Function::CubicFile
                        | Function::Akima
                        | Function::AkimaFile
                        | Function::Wodicka
                        | Function::WodickaFile
                        | Function::Barycentric
                        | Function::BarycentricFile
                        | Function::Sdt
                )
            })
            .ok_or_else(|| {
                ExprError::InvalidArgument(format!(
                    "cannot analytically differentiate unresolved or stateful function '{name}'"
                ))
            })?;
        let start = self
            .directions
            .len()
            .checked_sub(args.len())
            .ok_or_else(|| {
                ExprError::InvalidArgument(
                    "expression direction argument stack underflow".to_owned(),
                )
            })?;
        let kernel = self
            .functions
            .entry((function as usize, args.len()))
            .or_insert_with(|| ScalarFunction::new(function, args.len()));
        for (argument, &index) in kernel.argument_indices.iter().enumerate() {
            kernel.values[index] = args[argument].re;
            kernel.directions[index] = self.directions[start + argument];
        }
        let (value, direction) = compiled_expression_node_direction(
            &kernel.expression,
            &kernel.program,
            &kernel.values,
            &kernel.directions,
            BehavioralEnvironment {
                time: ctx.get("TIME").unwrap_or(0.0),
                frequency: ctx.get("FREQ").unwrap_or(0.0),
                temperature: ctx.get("TEMP").unwrap_or(27.0),
                gmin: ctx.get("GMIN").unwrap_or(crate::constants::GMIN),
                expression_dialect: ctx.expression_dialect(),
                // A scalar direction program is a one-shot output-domain
                // derivative with no Newton unknown in it: nothing here has an
                // iterate to reject, so the guarded logarithm stays.
                logarithm_domain: LogarithmDomain::Guarded,
            },
        )
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!(
                "directional derivative of '{name}' could not be evaluated analytically"
            ))
        })?;
        self.directions.truncate(start);
        self.directions.push(direction);
        Ok(value.into())
    }

    fn discard_condition(&mut self, _condition: ComplexValue) -> Result<(), ExprError> {
        self.pop().map(|_| ())
    }
}

impl ScalarFunction {
    fn new(function: Function, argc: usize) -> Self {
        let names = (0..argc)
            .map(|index| format!("\0direction:{index}"))
            .collect::<Vec<_>>();
        let expression = Expr::Function {
            func: function,
            args: names.iter().cloned().map(Expr::NodeVoltage).collect(),
        };
        let program = compile(&expression);
        let argument_indices = names.iter().map(|name| program.node_map[name]).collect();
        Self {
            expression,
            program,
            argument_indices,
            values: vec![0.0; argc],
            directions: vec![0.0.into(); argc],
        }
    }
}
