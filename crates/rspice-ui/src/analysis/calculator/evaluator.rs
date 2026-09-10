//! Calculator Evaluator
//!
//! Executes the `CalculatorExpr` AST against an `EvaluationContext`.
//! Handles vector arithmetic logic.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, UnaryOp};
use super::functions::FunctionRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalcValue {
    Scalar(f64),
    /// Waveform data (x, y)
    Waveform(Vec<f64>, Vec<f64>),
}

impl CalcValue {
    pub fn create_waveform(x: Vec<f64>, y: Vec<f64>) -> Self {
        // Enforce same length
        debug_assert_eq!(x.len(), y.len());
        Self::Waveform(x, y)
    }
}

/// Interface for retrieving simulation data
pub trait EvaluationContext {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError>;
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
    IdentifierNotFound(String),
    UnknownFunction(String),
    ArgCountMismatch {
        func: String,
        expected: usize,
        actual: usize,
    },
    TypeMismatch(String),
    MathError(String),
    WaveformMismatch(String),
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentifierNotFound(id) => write!(f, "Identifier not found: {}", id),
            Self::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            Self::ArgCountMismatch {
                func,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Argument count mismatch for {}: expected {}, got {}",
                    func, expected, actual
                )
            }
            Self::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            Self::MathError(msg) => write!(f, "Math error: {}", msg),
            Self::WaveformMismatch(reason) => write!(f, "Cannot combine waveforms: {reason}"),
        }
    }
}

impl std::error::Error for EvaluationError {}

pub fn evaluate(
    expr: &CalculatorExpr,
    ctx: &impl EvaluationContext,
) -> Result<CalcValue, EvaluationError> {
    match expr {
        CalculatorExpr::Number(val) => Ok(CalcValue::Scalar(*val)),

        CalculatorExpr::Constant(c) => match c {
            // These should probably be handled by context or expanded earlier if they depend on context
            // But if we treat them as abstract signals:
            CalculatorConstant::Time => ctx.get_waveform("TIME", None),
            CalculatorConstant::Frequency => ctx.get_waveform("FREQ", None),
        },

        CalculatorExpr::WaveformRef { signal, dataset } => {
            ctx.get_waveform(signal, dataset.as_deref())
        }

        CalculatorExpr::UnaryOp { op, operand } => {
            let val = evaluate(operand, ctx)?;
            match op {
                UnaryOp::Neg => neg_value(val),
            }
        }

        CalculatorExpr::BinaryOp { op, left, right } => {
            let l_val = evaluate(left, ctx)?;
            let r_val = evaluate(right, ctx)?;
            apply_binary_op(*op, l_val, r_val)
        }

        CalculatorExpr::FunctionCall { name, args } => {
            let mut arg_values = Vec::with_capacity(args.len());
            for arg in args {
                arg_values.push(evaluate(arg, ctx)?);
            }
            FunctionRegistry::dispatch(name, arg_values)
        }
    }
}

fn neg_value(val: CalcValue) -> Result<CalcValue, EvaluationError> {
    match val {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(-s)),
        CalcValue::Waveform(x, y) => {
            let new_y = y.into_iter().map(|v| -v).collect();
            Ok(CalcValue::Waveform(x, new_y))
        }
    }
}

fn apply_binary_op(
    op: BinaryOp,
    left: CalcValue,
    right: CalcValue,
) -> Result<CalcValue, EvaluationError> {
    for value in [&left, &right] {
        if let CalcValue::Waveform(x, y) = value {
            super::interpolation::validate_samples(x, y)
                .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
        }
    }
    match (left, right) {
        (CalcValue::Scalar(l), CalcValue::Scalar(r)) => {
            Ok(CalcValue::Scalar(apply_op_scalar(op, l, r)))
        }
        (CalcValue::Scalar(l), CalcValue::Waveform(rx, ry)) => {
            let new_y = ry
                .into_iter()
                .map(|r_val| apply_op_scalar(op, l, r_val))
                .collect();
            Ok(CalcValue::create_waveform(rx, new_y))
        }
        (CalcValue::Waveform(lx, ly), CalcValue::Scalar(r)) => {
            let new_y = ly
                .into_iter()
                .map(|l_val| apply_op_scalar(op, l_val, r))
                .collect();
            Ok(CalcValue::create_waveform(lx, new_y))
        }
        (CalcValue::Waveform(lx, ly), CalcValue::Waveform(rx, ry)) => {
            // Equal complete axes permit pointwise operations, including
            // repeated coordinates and matching sweep branches. Unequal axes
            // require an unambiguous interpolation domain.
            let (out_x, left_y, right_y) = if lx == rx {
                (lx, ly, ry)
            } else {
                use super::interpolation::{InterpolationMethod, align_waveforms};
                align_waveforms(&lx, &ly, &rx, &ry, InterpolationMethod::Linear)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?
            };

            let new_y: Vec<f64> = left_y
                .iter()
                .zip(right_y.iter())
                .map(|(l, r)| apply_op_scalar(op, *l, *r))
                .collect();

            Ok(CalcValue::create_waveform(out_x, new_y))
        }
    }
}

fn apply_op_scalar(op: BinaryOp, l: f64, r: f64) -> f64 {
    match op {
        BinaryOp::Add => l + r,
        BinaryOp::Sub => l - r,
        BinaryOp::Mul => l * r,
        BinaryOp::Div => l / r, // Div by zero handled by returning Inf/NaN which is spec compliant
        BinaryOp::Pow => l.powf(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn difference(
        left: (&[f64], &[f64]),
        right: (&[f64], &[f64]),
    ) -> Result<CalcValue, EvaluationError> {
        apply_binary_op(
            BinaryOp::Sub,
            CalcValue::Waveform(left.0.to_vec(), left.1.to_vec()),
            CalcValue::Waveform(right.0.to_vec(), right.1.to_vec()),
        )
    }

    #[test]
    fn equal_endpoints_do_not_imply_equal_interior_coordinates() {
        let first = [0.0, 1.0, 3.0];
        let second = [0.0, 2.0, 3.0];
        for (left, right) in [(&first, &second), (&second, &first)] {
            assert_eq!(
                difference((left, left), (right, right)).unwrap(),
                CalcValue::Waveform(vec![0.0, 1.0, 2.0, 3.0], vec![0.0; 4])
            );
        }
    }

    #[test]
    fn alignment_preserves_breakpoints_from_both_operands() {
        assert_eq!(
            difference(
                (&[0.0, 2.0], &[0.0, 0.0]),
                (&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0])
            )
            .unwrap(),
            CalcValue::Waveform(vec![0.0, 1.0, 2.0], vec![0.0, -1.0, 0.0])
        );
    }

    #[test]
    fn alignment_preserves_the_left_sweeps_direction() {
        let left = [3.0, 1.0, 0.0];
        let right = [0.0, 2.0, 3.0];
        assert_eq!(
            difference((&left, &left), (&right, &right)).unwrap(),
            CalcValue::Waveform(vec![3.0, 2.0, 1.0, 0.0], vec![0.0; 4])
        );
    }

    #[test]
    fn arithmetic_uses_only_the_shared_domain() {
        assert_eq!(
            difference((&[0.0, 2.0], &[0.0, 2.0]), (&[1.0, 3.0], &[2.0, 6.0])).unwrap(),
            CalcValue::Waveform(vec![1.0, 2.0], vec![-1.0, -2.0])
        );
        assert!(difference((&[0.0, 1.0], &[0.0, 1.0]), (&[2.0, 3.0], &[2.0, 3.0])).is_err());
    }

    #[test]
    fn a_single_point_has_no_extrapolated_extent() {
        assert_eq!(
            difference((&[0.0, 2.0], &[0.0, 2.0]), (&[1.0], &[1.0])).unwrap(),
            CalcValue::Waveform(vec![1.0], vec![0.0])
        );
    }

    #[test]
    fn malformed_operands_cannot_be_silently_zipped_or_broadcast() {
        for malformed in [
            CalcValue::Waveform(vec![0.0, 1.0], vec![1.0]),
            CalcValue::Waveform(vec![0.0], vec![1.0, 2.0]),
        ] {
            for other in [
                CalcValue::Scalar(2.0),
                CalcValue::Waveform(vec![0.0, 1.0], vec![1.0, 2.0]),
            ] {
                assert!(apply_binary_op(BinaryOp::Add, malformed.clone(), other.clone()).is_err());
                assert!(apply_binary_op(BinaryOp::Add, other, malformed.clone()).is_err());
            }
        }
    }

    #[test]
    fn matching_branches_remain_pointwise_but_ambiguous_resampling_is_rejected() {
        for x in [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
            assert_eq!(
                difference((&x, &[1.0, 2.0, 3.0]), (&x, &[1.0, 2.0, 3.0])).unwrap(),
                CalcValue::Waveform(x.to_vec(), vec![0.0; 3])
            );
            assert!(difference((&x, &[1.0, 2.0, 3.0]), (&[0.0, 1.0], &[1.0, 2.0])).is_err());
        }
    }
}
