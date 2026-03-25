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

    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            Self::Scalar(v) => Some(*v),
            _ => None,
        }
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
    WaveformMismatch,
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
            Self::WaveformMismatch => write!(f, "Waveform mismatch: X-axes do not align"),
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
            // Vector-Vector operation
            // Commercial tools interpolate mismatched time bases automatically
            let (out_x, left_y, right_y) =
                if lx.len() == rx.len() && lx.first() == rx.first() && lx.last() == rx.last() {
                    // Already aligned - no interpolation needed
                    (lx, ly, ry)
                } else {
                    // Resample second waveform to match first (Spectre default behavior)
                    use super::interpolation::{InterpolationMethod, align_waveforms};
                    let (new_x, new_ly, new_ry) =
                        align_waveforms(&lx, &ly, &rx, &ry, InterpolationMethod::Linear)
                            .map_err(|_| EvaluationError::WaveformMismatch)?;
                    (new_x, new_ly, new_ry)
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
    use std::collections::HashMap;

    struct MockContext {
        waveforms: HashMap<String, CalcValue>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                waveforms: HashMap::new(),
            }
        }
    }

    impl EvaluationContext for MockContext {
        fn get_waveform(
            &self,
            signal: &str,
            _dataset: Option<&str>,
        ) -> Result<CalcValue, EvaluationError> {
            self.waveforms
                .get(signal)
                .cloned()
                .ok_or(EvaluationError::IdentifierNotFound(signal.to_string()))
        }
    }

    #[test]
    fn test_eval_scalar_math() {
        let ctx = MockContext::new();
        let expr = CalculatorExpr::binary(
            BinaryOp::Add,
            CalculatorExpr::number(1.0),
            CalculatorExpr::number(2.0),
        );
        let res = evaluate(&expr, &ctx).unwrap();
        assert_eq!(res, CalcValue::Scalar(3.0));
    }

    #[test]
    fn test_eval_vector_math() {
        let _ctx = MockContext::new();
        // [1,2] + [3,4] = [4,6]
        let v1 = CalcValue::create_waveform(vec![0.0, 1.0], vec![1.0, 2.0]);
        let v2 = CalcValue::create_waveform(vec![0.0, 1.0], vec![3.0, 4.0]);

        // Manual binary op call to test logic
        let res = apply_binary_op(BinaryOp::Add, v1, v2).unwrap();
        match res {
            CalcValue::Waveform(_x, y) => {
                assert_eq!(y, vec![4.0, 6.0]);
            }
            _ => panic!("Expected waveform"),
        }
    }

    #[test]
    fn test_eval_scalar_vector() {
        // [1,2] * 2 = [2,4]
        let v1 = CalcValue::create_waveform(vec![0.0, 1.0], vec![1.0, 2.0]);
        let s = CalcValue::Scalar(2.0);

        let res = apply_binary_op(BinaryOp::Mul, v1, s).unwrap();
        match res {
            CalcValue::Waveform(_x, y) => {
                assert_eq!(y, vec![2.0, 4.0]);
            }
            _ => panic!("Expected waveform"),
        }
    }
}
