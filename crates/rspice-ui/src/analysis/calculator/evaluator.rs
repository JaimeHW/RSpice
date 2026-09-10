//! Calculator Evaluator
//!
//! Executes the `CalculatorExpr` AST against an `EvaluationContext`.
//! Handles vector arithmetic logic.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, UnaryOp};
pub use super::value::{CalcValue, RealValue};

/// Interface for retrieving simulation data
pub trait EvaluationContext {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError>;

    /// A direct magnitude projection can read a historical magnitude-only
    /// source without claiming its phase is known.
    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        super::complex_functions::dispatch("mag", vec![self.get_waveform(signal, dataset)?])
    }
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
    PhaseUnavailable(String),
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
            Self::PhaseUnavailable(signal) => write!(
                f,
                "Phase is not retained for {signal}; use mag() or dB() explicitly, or rerun to retain complex components"
            ),
        }
    }
}

impl std::error::Error for EvaluationError {}

pub fn evaluate(
    expr: &CalculatorExpr,
    ctx: &impl EvaluationContext,
) -> Result<CalcValue, EvaluationError> {
    let value = match expr {
        CalculatorExpr::Number(val) => Ok(CalcValue::Real(RealValue::Scalar(*val))),

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
            if matches!(
                name.to_ascii_lowercase().as_str(),
                "mag" | "magnitude" | "abs" | "db"
            ) && let [CalculatorExpr::WaveformRef { signal, dataset }] = args.as_slice()
            {
                return if name.eq_ignore_ascii_case("db") {
                    let value = match ctx.get_waveform(signal, dataset.as_deref()) {
                        Err(EvaluationError::PhaseUnavailable(_)) => {
                            ctx.get_magnitude(signal, dataset.as_deref())?
                        }
                        value => value?,
                    };
                    super::complex_functions::dispatch("db", vec![value])?.checked()
                } else {
                    ctx.get_magnitude(signal, dataset.as_deref())?.checked()
                };
            }
            let mut arg_values = Vec::with_capacity(args.len());
            for arg in args {
                arg_values.push(evaluate(arg, ctx)?);
            }
            super::complex_functions::dispatch(name, arg_values)
        }
    };
    value?.checked()
}

fn neg_value(value: CalcValue) -> Result<CalcValue, EvaluationError> {
    match value {
        CalcValue::Real(value) => neg_real_value(value).map(CalcValue::Real),
        CalcValue::Complex(value) => Ok(CalcValue::Complex(super::complex_functions::map_complex(
            value,
            |value| -value,
        ))),
    }
}

fn apply_binary_op(
    op: BinaryOp,
    left: CalcValue,
    right: CalcValue,
) -> Result<CalcValue, EvaluationError> {
    match (left, right) {
        (CalcValue::Real(left), CalcValue::Real(right)) => {
            apply_real_binary_op(op, left, right).map(CalcValue::Real)
        }
        (left, right) => super::complex_ops::binary(op, left.into_complex(), right.into_complex())
            .map(CalcValue::Complex),
    }
}
fn neg_real_value(val: RealValue) -> Result<RealValue, EvaluationError> {
    match val {
        RealValue::Scalar(s) => Ok(RealValue::Scalar(-s)),
        RealValue::Waveform(x, y) => {
            let new_y = y.into_iter().map(|v| -v).collect();
            Ok(RealValue::Waveform(x, new_y))
        }
    }
}

fn apply_real_binary_op(
    op: BinaryOp,
    left: RealValue,
    right: RealValue,
) -> Result<RealValue, EvaluationError> {
    for value in [&left, &right] {
        if let RealValue::Waveform(x, y) = value {
            super::interpolation::validate_samples(x, y)
                .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
        }
    }
    match (left, right) {
        (RealValue::Scalar(l), RealValue::Scalar(r)) => {
            Ok(RealValue::Scalar(apply_op_scalar(op, l, r)))
        }
        (RealValue::Scalar(l), RealValue::Waveform(rx, ry)) => {
            let new_y = ry
                .into_iter()
                .map(|r_val| apply_op_scalar(op, l, r_val))
                .collect();
            Ok(RealValue::create_waveform(rx, new_y))
        }
        (RealValue::Waveform(lx, ly), RealValue::Scalar(r)) => {
            let new_y = ly
                .into_iter()
                .map(|l_val| apply_op_scalar(op, l_val, r))
                .collect();
            Ok(RealValue::create_waveform(lx, new_y))
        }
        (RealValue::Waveform(lx, ly), RealValue::Waveform(rx, ry)) => {
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

            Ok(RealValue::create_waveform(out_x, new_y))
        }
    }
}

fn apply_op_scalar(op: BinaryOp, l: f64, r: f64) -> f64 {
    if !l.is_finite() || !r.is_finite() {
        return f64::NAN;
    }
    match op {
        BinaryOp::Add => l + r,
        BinaryOp::Sub => l - r,
        BinaryOp::Mul => l * r,
        BinaryOp::Div => l / r, // Undefined samples remain gaps; checked scalars report an error.
        BinaryOp::Pow => l.powf(r),
    }
}

#[cfg(test)]
mod tests {
    use super::RealValue as CalcValue;
    use super::*;

    fn difference(
        left: (&[f64], &[f64]),
        right: (&[f64], &[f64]),
    ) -> Result<CalcValue, EvaluationError> {
        apply_real_binary_op(
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
                assert!(
                    apply_real_binary_op(BinaryOp::Add, malformed.clone(), other.clone()).is_err()
                );
                assert!(apply_real_binary_op(BinaryOp::Add, other, malformed.clone()).is_err());
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
