use thiserror::Error;

use super::opt::{THERMAL_VOLTAGE_PER_K, limexp_derivative, limexp_value, real_truth_value};
use super::{
    DerivativeLane, EquationId, IrDiagnostic, OptBinaryOp, OptModel, OptUnaryOp, OptValueKind,
    ValueId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OptEvalInputs {
    pub parameters: Vec<f64>,
    pub node_potentials: Vec<f64>,
    pub branch_flows: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptEvalValue {
    Real(f64),
    Boolean(bool),
}

impl OptEvalValue {
    pub fn real(self) -> Option<f64> {
        match self {
            Self::Real(value) => Some(value),
            Self::Boolean(_) => None,
        }
    }

    pub fn boolean(self) -> Option<bool> {
        match self {
            Self::Real(_) => None,
            Self::Boolean(value) => Some(value),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Real(_) => "real",
            Self::Boolean(_) => "boolean",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptEvaluatedDerivative {
    pub lane: DerivativeLane,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptEvalSnapshot {
    values: Vec<OptEvalValue>,
    derivatives: Vec<Vec<OptEvaluatedDerivative>>,
}

impl OptEvalSnapshot {
    pub fn value(&self, value: ValueId) -> Option<OptEvalValue> {
        self.values.get(usize::from(value)).copied()
    }

    pub fn real(&self, value: ValueId) -> Option<f64> {
        self.value(value).and_then(OptEvalValue::real)
    }

    pub fn boolean(&self, value: ValueId) -> Option<bool> {
        self.value(value).and_then(OptEvalValue::boolean)
    }

    pub fn derivative(&self, value: ValueId, lane: DerivativeLane) -> Option<f64> {
        self.derivatives
            .get(usize::from(value))?
            .iter()
            .find(|derivative| derivative.lane == lane)
            .map(|derivative| derivative.value)
    }
}

#[derive(Debug, Error)]
pub enum OptEvalError {
    #[error("OptIR validation failed")]
    Validation(#[source] OptValidationError),
    #[error("OptIR input '{kind}' index {index} is outside input length {len}")]
    MissingInput {
        kind: &'static str,
        index: u32,
        len: usize,
    },
    #[error("OptIR value {value} expected {expected}, found {found}")]
    TypeMismatch {
        value: ValueId,
        expected: &'static str,
        found: &'static str,
    },
    #[error("OptIR equation value {equation} is not evaluable by the scalar reference evaluator")]
    UnsupportedEquationValue { equation: EquationId },
}

#[derive(Debug, Error)]
#[error("{diagnostics:?}")]
pub struct OptValidationError {
    pub diagnostics: Vec<IrDiagnostic>,
}

impl OptModel {
    pub fn evaluate(&self, inputs: &OptEvalInputs) -> Result<OptEvalSnapshot, OptEvalError> {
        evaluate_opt_model(self, inputs)
    }
}

pub fn evaluate_opt_model(
    model: &OptModel,
    inputs: &OptEvalInputs,
) -> Result<OptEvalSnapshot, OptEvalError> {
    model
        .validate()
        .map_err(|diagnostics| OptEvalError::Validation(OptValidationError { diagnostics }))?;

    let mut values = Vec::with_capacity(model.values.len());
    for value in &model.values {
        values.push(evaluate_value(value.id, &value.kind, &values, inputs)?);
    }

    let mut derivatives = vec![Vec::new(); values.len()];
    for value in &model.values {
        let slot = &mut derivatives[usize::from(value.id)];
        slot.reserve(value.derivatives.len());

        for derivative in &value.derivatives {
            let derivative_value = real_value(&values, derivative.value)?;
            slot.push(OptEvaluatedDerivative {
                lane: derivative.lane,
                value: derivative_value,
            });
        }
    }

    Ok(OptEvalSnapshot {
        values,
        derivatives,
    })
}

fn evaluate_value(
    value: ValueId,
    kind: &OptValueKind,
    values: &[OptEvalValue],
    inputs: &OptEvalInputs,
) -> Result<OptEvalValue, OptEvalError> {
    match kind {
        OptValueKind::RealConstant(constant) => Ok(OptEvalValue::Real(*constant)),
        OptValueKind::BooleanConstant(constant) => Ok(OptEvalValue::Boolean(*constant)),
        OptValueKind::Parameter { parameter } => Ok(OptEvalValue::Real(input_at(
            "parameter",
            &inputs.parameters,
            parameter.index(),
        )?)),
        OptValueKind::ParamGiven { .. } => Ok(OptEvalValue::Real(0.0)),
        OptValueKind::Temperature => Ok(OptEvalValue::Real(300.15)),
        OptValueKind::ThermalVoltage => Ok(OptEvalValue::Real(300.15 * THERMAL_VOLTAGE_PER_K)),
        OptValueKind::Multiplicity => Ok(OptEvalValue::Real(1.0)),
        OptValueKind::Time => Ok(OptEvalValue::Real(0.0)),
        OptValueKind::NodePotential { node } => Ok(OptEvalValue::Real(input_at(
            "node_potential",
            &inputs.node_potentials,
            node.index(),
        )?)),
        OptValueKind::BranchFlow { branch } => Ok(OptEvalValue::Real(input_at(
            "branch_flow",
            &inputs.branch_flows,
            branch.index(),
        )?)),
        OptValueKind::Unary { op, input } => evaluate_unary(value, *op, *input, values),
        OptValueKind::Binary { op, left, right } => {
            evaluate_binary(value, *op, *left, *right, values)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            let condition = boolean_value(values, *condition)?;
            if condition {
                Ok(value_at(values, *then_value))
            } else {
                Ok(value_at(values, *else_value))
            }
        }
        OptValueKind::EquationValue { equation } => Err(OptEvalError::UnsupportedEquationValue {
            equation: *equation,
        }),
    }
}

fn evaluate_unary(
    owner: ValueId,
    op: OptUnaryOp,
    input: ValueId,
    values: &[OptEvalValue],
) -> Result<OptEvalValue, OptEvalError> {
    let result = (|| match op {
        OptUnaryOp::Pos => Ok(OptEvalValue::Real(real_value(values, input)?)),
        OptUnaryOp::Neg => Ok(OptEvalValue::Real(-real_value(values, input)?)),
        OptUnaryOp::Not => Ok(OptEvalValue::Boolean(!truth_value(values, input)?)),
        OptUnaryOp::Exp => Ok(OptEvalValue::Real(real_value(values, input)?.exp())),
        OptUnaryOp::LimExp => Ok(OptEvalValue::Real(limexp_value(real_value(values, input)?))),
        OptUnaryOp::LimExpDerivative => Ok(OptEvalValue::Real(limexp_derivative(real_value(
            values, input,
        )?))),
        OptUnaryOp::Ln => Ok(OptEvalValue::Real(real_value(values, input)?.ln())),
        OptUnaryOp::Sqrt => Ok(OptEvalValue::Real(real_value(values, input)?.sqrt())),
        OptUnaryOp::Abs => Ok(OptEvalValue::Real(real_value(values, input)?.abs())),
        OptUnaryOp::Sin => Ok(OptEvalValue::Real(real_value(values, input)?.sin())),
        OptUnaryOp::Cos => Ok(OptEvalValue::Real(real_value(values, input)?.cos())),
        OptUnaryOp::Tan => Ok(OptEvalValue::Real(real_value(values, input)?.tan())),
        OptUnaryOp::Sinh => Ok(OptEvalValue::Real(real_value(values, input)?.sinh())),
        OptUnaryOp::Cosh => Ok(OptEvalValue::Real(real_value(values, input)?.cosh())),
        OptUnaryOp::Tanh => Ok(OptEvalValue::Real(real_value(values, input)?.tanh())),
        OptUnaryOp::Atan => Ok(OptEvalValue::Real(real_value(values, input)?.atan())),
        OptUnaryOp::Asinh => Ok(OptEvalValue::Real(real_value(values, input)?.asinh())),
    })();

    result.map_err(|error| remap_type_mismatch(owner, error))
}

fn evaluate_binary(
    owner: ValueId,
    op: OptBinaryOp,
    left: ValueId,
    right: ValueId,
    values: &[OptEvalValue],
) -> Result<OptEvalValue, OptEvalError> {
    let result = (|| match op {
        OptBinaryOp::Add => Ok(OptEvalValue::Real(
            real_value(values, left)? + real_value(values, right)?,
        )),
        OptBinaryOp::Sub => Ok(OptEvalValue::Real(
            real_value(values, left)? - real_value(values, right)?,
        )),
        OptBinaryOp::Mul => Ok(OptEvalValue::Real(
            real_value(values, left)? * real_value(values, right)?,
        )),
        OptBinaryOp::Div => Ok(OptEvalValue::Real(
            real_value(values, left)? / real_value(values, right)?,
        )),
        OptBinaryOp::Pow => Ok(OptEvalValue::Real(
            real_value(values, left)?.powf(real_value(values, right)?),
        )),
        OptBinaryOp::Eq => Ok(OptEvalValue::Boolean(
            value_at(values, left) == value_at(values, right),
        )),
        OptBinaryOp::Ne => Ok(OptEvalValue::Boolean(
            value_at(values, left) != value_at(values, right),
        )),
        OptBinaryOp::Lt => Ok(OptEvalValue::Boolean(
            real_value(values, left)? < real_value(values, right)?,
        )),
        OptBinaryOp::Le => Ok(OptEvalValue::Boolean(
            real_value(values, left)? <= real_value(values, right)?,
        )),
        OptBinaryOp::Gt => Ok(OptEvalValue::Boolean(
            real_value(values, left)? > real_value(values, right)?,
        )),
        OptBinaryOp::Ge => Ok(OptEvalValue::Boolean(
            real_value(values, left)? >= real_value(values, right)?,
        )),
        OptBinaryOp::And => Ok(OptEvalValue::Boolean(
            truth_value(values, left)? && truth_value(values, right)?,
        )),
        OptBinaryOp::Or => Ok(OptEvalValue::Boolean(
            truth_value(values, left)? || truth_value(values, right)?,
        )),
    })();

    result.map_err(|error| remap_type_mismatch(owner, error))
}

fn remap_type_mismatch(owner: ValueId, error: OptEvalError) -> OptEvalError {
    match error {
        OptEvalError::TypeMismatch {
            expected, found, ..
        } => OptEvalError::TypeMismatch {
            value: owner,
            expected,
            found,
        },
        other => other,
    }
}

fn input_at(kind: &'static str, inputs: &[f64], index: u32) -> Result<f64, OptEvalError> {
    inputs
        .get(index as usize)
        .copied()
        .ok_or(OptEvalError::MissingInput {
            kind,
            index,
            len: inputs.len(),
        })
}

fn value_at(values: &[OptEvalValue], value: ValueId) -> OptEvalValue {
    values[usize::from(value)]
}

fn real_value(values: &[OptEvalValue], value: ValueId) -> Result<f64, OptEvalError> {
    let evaluated = value_at(values, value);
    evaluated.real().ok_or(OptEvalError::TypeMismatch {
        value,
        expected: "real",
        found: evaluated.label(),
    })
}

fn boolean_value(values: &[OptEvalValue], value: ValueId) -> Result<bool, OptEvalError> {
    let evaluated = value_at(values, value);
    evaluated.boolean().ok_or(OptEvalError::TypeMismatch {
        value,
        expected: "boolean",
        found: evaluated.label(),
    })
}

fn truth_value(values: &[OptEvalValue], value: ValueId) -> Result<bool, OptEvalError> {
    match value_at(values, value) {
        OptEvalValue::Real(value) => Ok(real_truth_value(value)),
        OptEvalValue::Boolean(value) => Ok(value),
    }
}
