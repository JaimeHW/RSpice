//! Analytic derivative of a compiled behavioral expression.
//!
//! One evaluator carries a value and its derivative through an expression
//! tree, with respect to a node voltage, a branch current, a caller-supplied
//! direction or time. Behavioral sources, behavioral-value passives and the
//! `.PARAM` directional evaluator all differentiate through it, so their
//! derivative semantics cannot drift apart.

use super::{
    BinaryOp, CompiledExpr, Derivative, EXPR_ZERO_TOLERANCE, Expr, Function, LogarithmDomain,
    UnaryOp, XYCE_ATANH_EPSILON, XYCE_TANH_SATURATION_THRESHOLD, derivative_pair,
    lookup_table_interpolate_with_derivative, normalize_expression_boundary, ordered_limit,
    ordered_sign, real_function_pow_with_derivative, real_function_pwr_with_derivative,
    real_function_pwrs_with_derivative, real_pow_with_derivative,
};
use crate::Value;
use crate::config::ExpressionDialect;

/// The point a behavioral expression is evaluated at, beyond its own operands:
/// the analysis coordinate, the ambient temperature, the conductance floor the
/// solver adds, and the dialect whose function set and rounding apply.
#[derive(Clone, Copy)]
pub(crate) struct BehavioralEnvironment {
    pub(crate) time: Value,
    pub(crate) frequency: Value,
    pub(crate) temperature: Value,
    pub(crate) gmin: Value,
    pub(crate) expression_dialect: ExpressionDialect,
    /// Whether a logarithm outside its domain is guarded or left to IEEE.
    ///
    /// The same switch the bytecode VM carries on its `Context`, and named
    /// rather than defaulted for the same reason: a `B` source's value comes
    /// from the VM and its Jacobian entry from this evaluator, so a caller
    /// that lets the two disagree has fabricated a tangent plane for a value
    /// it never published.
    pub(crate) logarithm_domain: LogarithmDomain,
}

#[derive(Clone, Copy)]
pub(crate) enum DerivativeTarget<'a> {
    Node(usize),
    /// Physical event Jacobian, including unregularized polynomial slopes.
    PhysicalNode(usize),
    Branch(usize),
    /// Physical event derivative with the controlling finite current held as a coordinate.
    PhysicalBranch(usize),
    NodeDirection(&'a [Derivative]),
    Time,
}

pub(crate) struct BehavioralDerivativeContext<'a> {
    pub(crate) program: &'a CompiledExpr,
    pub(crate) node_values: &'a [Value],
    pub(crate) branch_values: &'a [Value],
    pub(crate) time: Value,
    pub(crate) frequency: Value,
    pub(crate) temperature: Value,
    pub(crate) gmin: Value,
    pub(crate) expression_dialect: ExpressionDialect,
    /// Whether a logarithm outside its domain is guarded or left to IEEE. The
    /// evaluator that owns the expression decides, exactly as it does for the
    /// bytecode VM's `Context`.
    pub(crate) logarithm_domain: LogarithmDomain,
    pub(crate) target: DerivativeTarget<'a>,
}

/// Propagate a caller-supplied direction through expression-local scalar
/// leaves. Keep the derivative exponent until the owning consumer converts it.
pub(crate) fn compiled_expression_node_direction(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    node_directions: &[Derivative],
    environment: BehavioralEnvironment,
) -> Option<(Value, Derivative)> {
    if node_values.len() != program.node_map.len()
        || node_directions.len() != node_values.len()
        || !program.branch_map.is_empty()
    {
        return None;
    }
    eval_behavioral_expr_with_derivative(
        expr,
        &BehavioralDerivativeContext {
            program,
            node_values,
            branch_values: &[],
            time: environment.time,
            frequency: environment.frequency,
            temperature: environment.temperature,
            gmin: environment.gmin,
            expression_dialect: environment.expression_dialect,
            logarithm_domain: environment.logarithm_domain,
            target: DerivativeTarget::NodeDirection(node_directions),
        },
    )
}

/// The argument this evaluator's logarithm is evaluated at, and its slope.
///
/// The policy is the caller's, carried on the context as
/// [`LogarithmDomain`] — the same switch the bytecode VM keeps on its
/// `Context`. The two must agree for any one expression, because a `B`
/// source's value comes from the VM and its Jacobian entry comes from here;
/// this function existing with a rule of its own is what made them two
/// policies instead of one.
///
/// Under [`LogarithmDomain::Ieee`] a strictly negative argument is NaN.
/// Clamping it is what let the R1.14 deck `B1 a 0 I={ln(v(a)+0.1)}` report
/// V(a) = -4.999995 as an operating point: the clamp fabricates
/// `ln(1e-38) = -87.5` for the value and `1/1e-38 = 1e38` for the slope, and a
/// 1e38 conductance pins the node so hard that the next Newton update is zero
/// and the `vntol` check calls it converged. R1.14's
/// `StampError::NonFiniteTrial` is what turns the NaN into a rejected iterate
/// rather than a refused run.
#[inline]
fn behavioral_logarithm_argument(domain: LogarithmDomain, value: Value) -> Value {
    domain.argument(value)
}

pub(crate) fn eval_behavioral_expr_with_derivative_at_boundary(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    let (value, derivative) = eval_behavioral_expr_with_derivative(expr, context)?;
    Some((
        normalize_expression_boundary(value, context.expression_dialect),
        normalize_expression_boundary(derivative.binary64(), context.expression_dialect),
    ))
}

pub(crate) fn eval_behavioral_expr_with_derivative(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    if matches!(context.target, DerivativeTarget::Time) && !crate::expr::constant_over_time(expr) {
        // Newton's regularized slopes and pointwise Boolean/table derivatives
        // are not necessarily outgoing time derivatives. Only use the rules
        // qualified below for physical displacement current; unsupported
        // corners must not silently publish a zero or regularized current.
        let qualified = match expr {
            Expr::Time
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::Unary {
                op: UnaryOp::Neg, ..
            } => true,
            Expr::Binary {
                op:
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge,
                ..
            } => true,
            Expr::Binary {
                op: BinaryOp::Pow,
                right,
                ..
            } => crate::expr::constant_over_time(right),
            Expr::Function {
                func:
                    Function::Abs
                    | Function::Exp
                    | Function::Sin
                    | Function::Cos
                    | Function::Tan
                    | Function::Atan
                    | Function::Sinh
                    | Function::Cosh
                    | Function::Tanh
                    | Function::Asinh
                    | Function::Sqr
                    | Function::Min
                    | Function::Max
                    | Function::Uramp
                    | Function::If,
                ..
            } => true,
            Expr::Function {
                func:
                    Function::SpicePulse | Function::SpiceSin | Function::SpiceExp | Function::SpiceSffm,
                args,
            } => args.iter().all(crate::expr::constant_over_time),
            _ => false,
        };
        if !qualified {
            return None;
        }
    }
    match expr {
        Expr::Const(value) => derivative_pair(*value, 0.0),
        Expr::Time => derivative_pair(
            context.time,
            Value::from(matches!(context.target, DerivativeTarget::Time)),
        ),
        Expr::Frequency => derivative_pair(context.frequency, 0.0),
        Expr::Temperature => derivative_pair(context.temperature, 0.0),
        Expr::ThermalVoltage => derivative_pair(
            crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(
                context.temperature,
            )),
            0.0,
        ),
        Expr::Gmin => derivative_pair(context.gmin, 0.0),
        Expr::StringLiteral(_) => derivative_pair(0.0, 0.0),
        Expr::LookupTable { input, table } => {
            let (input_value, input_derivative) =
                eval_behavioral_expr_with_derivative(input, context)?;
            let (value, derivative) = lookup_table_interpolate_with_derivative(
                input_value,
                table,
                context.expression_dialect,
            );
            derivative_pair(
                value,
                if input_derivative == 0.0 {
                    0.0.into()
                } else {
                    derivative * input_derivative
                },
            )
        }
        Expr::NodeVoltage(name) => {
            let idx = *context.program.node_map.get(name)?;
            let value = *context.node_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Node(target_idx) | DerivativeTarget::PhysicalNode(target_idx)
                    if target_idx == idx =>
                {
                    1.0.into()
                }
                DerivativeTarget::NodeDirection(direction) => *direction.get(idx)?,
                _ => 0.0.into(),
            };
            derivative_pair(value, derivative)
        }
        Expr::BranchCurrent(name) => {
            let idx = *context.program.branch_map.get(name)?;
            let value = *context.branch_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Branch(target_idx)
                | DerivativeTarget::PhysicalBranch(target_idx)
                    if target_idx == idx =>
                {
                    1.0
                }
                _ => 0.0,
            };
            derivative_pair(value, derivative)
        }
        Expr::Unary { op, operand } => {
            let (value, derivative) = eval_behavioral_expr_with_derivative(operand, context)?;
            match op {
                UnaryOp::Neg => derivative_pair(-value, -derivative),
                UnaryOp::Not => derivative_pair(if value == 0.0 { 1.0 } else { 0.0 }, 0.0),
            }
        }
        Expr::Binary { op, left, right } => {
            let (left_value, left_derivative) =
                eval_behavioral_expr_with_derivative(left, context)?;
            let (right_value, right_derivative) =
                eval_behavioral_expr_with_derivative(right, context)?;
            if matches!(context.target, DerivativeTarget::Time)
                && matches!(
                    op,
                    BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
                )
                && left_value == right_value
                && !(crate::expr::constant_over_time(left)
                    && crate::expr::constant_over_time(right))
            {
                if left_derivative == right_derivative {
                    return None;
                }
                let positive = left_derivative > right_derivative;
                let truth = match op {
                    BinaryOp::Gt | BinaryOp::Ge => positive,
                    _ => !positive,
                };
                return derivative_pair(bool_value(truth), 0.0);
            }
            if matches!(
                context.target,
                DerivativeTarget::Time
                    | DerivativeTarget::PhysicalNode(_)
                    | DerivativeTarget::PhysicalBranch(_)
            ) && *op == BinaryOp::Pow
                && !crate::expr::constant_over_time(expr)
            {
                if !right_value.is_finite()
                    || right_value < 0.0
                    || right_value.fract() != 0.0
                    || right_derivative != 0.0
                {
                    return None;
                }
                // The physical rule uses the polynomial derivative at zero too;
                // Xyce's Newton rule deliberately zeroes that derivative.
                return derivative_pair(
                    left_value.powf(right_value),
                    if right_value == 0.0 {
                        0.0.into()
                    } else {
                        left_derivative * right_value * left_value.powf(right_value - 1.0)
                    },
                );
            }
            eval_binary_with_derivative(
                *op,
                left_value,
                left_derivative,
                right_value,
                right_derivative,
                context.expression_dialect,
            )
        }
        Expr::Function { func, args } => eval_function_with_derivative(*func, args, context),
    }
}

pub(crate) fn eval_binary_with_derivative(
    op: BinaryOp,
    left: Value,
    d_left: Derivative,
    right: Value,
    d_right: Derivative,
    expression_dialect: ExpressionDialect,
) -> Option<(Value, Derivative)> {
    match op {
        BinaryOp::Add => derivative_pair(left + right, d_left + d_right),
        BinaryOp::Sub => derivative_pair(left - right, d_left - d_right),
        BinaryOp::Mul => derivative_pair(left * right, d_left * right + left * d_right),
        BinaryOp::Div => {
            if right == 0.0 {
                None
            } else {
                derivative_pair(
                    left / right,
                    Derivative::product_ratio(
                        [(d_left, right), (-d_right, left)],
                        [(right, right), (0.0, 0.0)],
                    ),
                )
            }
        }
        BinaryOp::Mod => {
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                derivative_pair(left % right, d_left - quotient * d_right)
            }
        }
        BinaryOp::Pow => real_pow_with_derivative(left, d_left, right, d_right, expression_dialect),
        BinaryOp::Lt => derivative_pair(bool_value(left < right), 0.0),
        BinaryOp::Le => derivative_pair(bool_value(left <= right), 0.0),
        BinaryOp::Gt => derivative_pair(bool_value(left > right), 0.0),
        BinaryOp::Ge => derivative_pair(bool_value(left >= right), 0.0),
        BinaryOp::Eq => {
            derivative_pair(bool_value((left - right).abs() < EXPR_ZERO_TOLERANCE), 0.0)
        }
        BinaryOp::Ne => {
            derivative_pair(bool_value((left - right).abs() >= EXPR_ZERO_TOLERANCE), 0.0)
        }
        BinaryOp::And => derivative_pair(bool_value(left != 0.0 && right != 0.0), 0.0),
        BinaryOp::Or => derivative_pair(bool_value(left != 0.0 || right != 0.0), 0.0),
    }
}

fn eval_function_with_derivative(
    func: Function,
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    let eval_arg = |index: usize| {
        args.get(index)
            .and_then(|arg| eval_behavioral_expr_with_derivative(arg, context))
    };

    match func {
        Function::Abs => {
            let (x, dx) = eval_arg(0)?;
            if x == 0.0 && matches!(context.target, DerivativeTarget::Time) {
                return derivative_pair(0.0, dx.abs());
            }
            derivative_pair(x.abs(), x.signum() * dx)
        }
        Function::Sqrt => {
            let (x, dx) = eval_arg(0)?;
            let value = x.max(0.0).sqrt();
            if value == 0.0 {
                derivative_pair(value, 0.0)
            } else {
                derivative_pair(value, 0.5 * dx / value)
            }
        }
        Function::Exp => unary_derivative(eval_arg(0)?, |x| x.exp(), |x| x.exp()),
        Function::Log => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(context.logarithm_domain, x);
            if context.expression_dialect == ExpressionDialect::Xyce {
                derivative_pair(clamped.log10(), dx / clamped / std::f64::consts::LN_10)
            } else {
                derivative_pair(clamped.ln(), dx / clamped)
            }
        }
        Function::Ln => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(context.logarithm_domain, x);
            derivative_pair(clamped.ln(), dx / clamped)
        }
        Function::Log10 => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(context.logarithm_domain, x);
            derivative_pair(clamped.log10(), dx / clamped / std::f64::consts::LN_10)
        }
        Function::Sin => unary_derivative(eval_arg(0)?, |x| x.sin(), |x| x.cos()),
        Function::Cos => unary_derivative(eval_arg(0)?, |x| x.cos(), |x| -x.sin()),
        Function::Tan => unary_derivative(
            eval_arg(0)?,
            |x| x.tan(),
            |x| {
                let cos_x = x.cos();
                1.0 / (cos_x * cos_x)
            },
        ),
        Function::Asin => unary_derivative(
            eval_arg(0)?,
            |x| x.clamp(-1.0, 1.0).asin(),
            |x| 1.0 / (1.0 - x * x).sqrt(),
        ),
        Function::Acos => unary_derivative(
            eval_arg(0)?,
            |x| x.clamp(-1.0, 1.0).acos(),
            |x| -1.0 / (1.0 - x * x).sqrt(),
        ),
        Function::Atan => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(
                x.atan(),
                Derivative::product_ratio([(dx, 1.0), (0.0.into(), 0.0)], [(1.0, 1.0), (x, x)]),
            )
        }
        Function::Atan2 => {
            let (y, dy) = eval_arg(0)?;
            let (x, dx) = eval_arg(1)?;
            if x == 0.0 && y == 0.0 {
                None
            } else {
                derivative_pair(
                    y.atan2(x),
                    Derivative::product_ratio([(dy, x), (-dx, y)], [(x, x), (y, y)]),
                )
            }
        }
        Function::Sinh => unary_derivative(eval_arg(0)?, |x| x.sinh(), |x| x.cosh()),
        Function::Cosh => unary_derivative(eval_arg(0)?, |x| x.cosh(), |x| x.sinh()),
        Function::Tanh => {
            let (x, dx) = eval_arg(0)?;
            if context.expression_dialect == ExpressionDialect::Xyce {
                let value = xyce_tanh_behavioral(x);
                let derivative = if (-XYCE_TANH_SATURATION_THRESHOLD
                    ..=XYCE_TANH_SATURATION_THRESHOLD)
                    .contains(&x)
                {
                    let cosh_x = x.cosh();
                    dx / (cosh_x * cosh_x)
                } else {
                    0.0.into()
                };
                derivative_pair(value, derivative)
            } else {
                let value = x.tanh();
                // 1-tanh(x)^2 cancels near saturation. Keep the decaying
                // exponential separate from its incoming tangent as well.
                let argument = -2.0 * x.abs();
                let denominator = 1.0 + argument.exp();
                derivative_pair(
                    value,
                    (dx * 4.0 / denominator / denominator).multiply_exp(argument),
                )
            }
        }
        Function::Asinh => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x.asinh(), dx / x.hypot(1.0))
        }
        Function::Acosh => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x.acosh(), dx / (x - 1.0).sqrt() / (x + 1.0).sqrt())
        }
        Function::Atanh => {
            let (x, dx) = eval_arg(0)?;
            if context.expression_dialect == ExpressionDialect::Xyce {
                let lower = XYCE_ATANH_EPSILON - 1.0;
                let upper = 1.0 - XYCE_ATANH_EPSILON;
                let clamped = x.clamp(lower, upper);
                let derivative = if x >= lower && x <= upper {
                    dx / (1.0 - x * x)
                } else {
                    0.0.into()
                };
                derivative_pair(clamped.atanh(), derivative)
            } else {
                derivative_pair(x.atanh(), dx / (1.0 - x * x))
            }
        }
        Function::Trunc => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.trunc(), 0.0)
        }
        Function::Floor => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.floor(), 0.0)
        }
        Function::Ceil => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.ceil(), 0.0)
        }
        Function::Round => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.round_ties_even(), 0.0)
        }
        Function::Sqr => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x * x, dx * 2.0 * x)
        }
        Function::Pwr => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            real_function_pwr_with_derivative(
                base,
                d_base,
                exponent,
                d_exponent,
                context.expression_dialect,
            )
        }
        Function::Pwrs => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            real_function_pwrs_with_derivative(
                base,
                d_base,
                exponent,
                d_exponent,
                context.expression_dialect,
            )
        }
        Function::Limit => match args.len() {
            2 => {
                let (nom, d_nom) = eval_arg(0)?;
                derivative_pair(nom, d_nom)
            }
            3 => {
                let (x, dx) = eval_arg(0)?;
                let (min, _) = eval_arg(1)?;
                let (max, _) = eval_arg(2)?;
                let (value, retains_input_derivative) =
                    ordered_limit(x, min, max, context.expression_dialect);
                derivative_pair(
                    value,
                    if retains_input_derivative {
                        dx
                    } else {
                        0.0.into()
                    },
                )
            }
            _ => None,
        },
        Function::Min => {
            let mut best = eval_arg(0)?;
            for index in 1..args.len() {
                let candidate = eval_arg(index)?;
                if candidate.0 < best.0
                    || (candidate.0 == best.0
                        && matches!(context.target, DerivativeTarget::Time)
                        && candidate.1 < best.1)
                {
                    best = candidate;
                }
            }
            Some(best)
        }
        Function::Max => {
            let mut best = eval_arg(0)?;
            for index in 1..args.len() {
                let candidate = eval_arg(index)?;
                if candidate.0 > best.0
                    || (candidate.0 == best.0
                        && matches!(context.target, DerivativeTarget::Time)
                        && candidate.1 > best.1)
                {
                    best = candidate;
                }
            }
            Some(best)
        }
        Function::Sign => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(ordered_sign(x), 0.0)
        }
        Function::HspiceSign => {
            let (magnitude, d_magnitude) = eval_arg(0)?;
            let (polarity, _) = eval_arg(1)?;
            let sign = ordered_sign(polarity);
            let magnitude_derivative = if magnitude >= 0.0 {
                d_magnitude
            } else {
                -d_magnitude
            };
            derivative_pair(magnitude.abs() * sign, magnitude_derivative * sign)
        }
        Function::Uramp => {
            let (x, dx) = eval_arg(0)?;
            if x == 0.0 && matches!(context.target, DerivativeTarget::Time) {
                return derivative_pair(0.0, dx.max(0.0));
            }
            derivative_pair(x.max(0.0), if x > 0.0 { dx } else { 0.0.into() })
        }
        Function::Stp => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x > EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Ustep => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(
                if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    0.0
                } else {
                    0.5
                },
                0.0,
            )
        }
        Function::U2 => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(
                x.clamp(0.0, 1.0),
                if x > 0.0 && x < 1.0 { dx } else { 0.0.into() },
            )
        }
        Function::Eq0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x.abs() < EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Ne0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x.abs() >= EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Gt0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x > 0.0), 0.0)
        }
        Function::Lt0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x < 0.0), 0.0)
        }
        Function::Ge0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x >= 0.0), 0.0)
        }
        Function::Le0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x <= 0.0), 0.0)
        }
        Function::Pow => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            real_function_pow_with_derivative(
                left,
                d_left,
                right,
                d_right,
                context.expression_dialect,
            )
        }
        Function::Mod => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                derivative_pair(left % right, d_left - quotient * d_right)
            }
        }
        Function::Table => eval_table_function_with_derivative(args, context),
        Function::Pwl => eval_pwl_function_with_derivative(args, context),
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
        | Function::BarycentricFile => derivative_pair(0.0, 0.0),
        Function::Sdt => None,
        Function::SpicePulse | Function::SpiceSin | Function::SpiceExp | Function::SpiceSffm => {
            if !matches!(context.target, DerivativeTarget::Time) {
                return None;
            }
            let mut values = Vec::with_capacity(args.len());
            for index in 0..args.len() {
                let (value, derivative) = eval_arg(index)?;
                if derivative != 0.0 {
                    return None;
                }
                values.push(value);
            }
            crate::expr::spice_waveform_value_and_time_derivative(func, &values, context.time)
                .and_then(|(value, derivative)| derivative_pair(value, derivative))
        }
        Function::If => {
            let (condition, derivative) = eval_arg(0)?;
            if condition == 0.0
                && matches!(context.target, DerivativeTarget::Time)
                && !crate::expr::constant_over_time(&args[0])
                && !matches!(
                    &args[0],
                    Expr::Binary {
                        op: BinaryOp::Lt
                            | BinaryOp::Le
                            | BinaryOp::Gt
                            | BinaryOp::Ge
                            | BinaryOp::Eq
                            | BinaryOp::Ne
                            | BinaryOp::And
                            | BinaryOp::Or,
                        ..
                    }
                )
            {
                if derivative == 0.0 {
                    return None;
                }
                return eval_arg(1);
            }
            if condition != 0.0 {
                eval_arg(1)
            } else {
                eval_arg(2)
            }
        }
    }
}

fn unary_derivative(
    input: (Value, Derivative),
    value_fn: impl FnOnce(Value) -> Value,
    derivative_fn: impl FnOnce(Value) -> Value,
) -> Option<(Value, Derivative)> {
    let (x, dx) = input;
    derivative_pair(value_fn(x), derivative_fn(x) * dx)
}

fn eval_table_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    eval_piecewise_function_with_derivative(args, context, eval_table_points_with_derivative)
}

fn eval_pwl_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    eval_piecewise_function_with_derivative(args, context, eval_pwl_points_with_derivative)
}

/// Interpolate a breakpoint table and report the value and its slope.
///
/// Takes the point to interpolate at, that point's derivative with respect to
/// the differentiation variable, and the `(x, y)` breakpoints.
type PiecewiseDerivativeEvaluator =
    fn(Value, Derivative, &[(Value, Value)]) -> Option<(Value, Derivative)>;

fn eval_piecewise_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
    evaluator: PiecewiseDerivativeEvaluator,
) -> Option<(Value, Derivative)> {
    if args.len() < 3 {
        return None;
    }
    let (x, dx) = eval_behavioral_expr_with_derivative(&args[0], context)?;
    let mut points = Vec::new();
    for pair in args[1..].chunks(2) {
        let x_expr = pair.first()?;
        let y_expr = pair.get(1)?;
        let (px, _) = eval_behavioral_expr_with_derivative(x_expr, context)?;
        let (py, _) = eval_behavioral_expr_with_derivative(y_expr, context)?;
        points.push((px, py));
    }
    evaluator(x, dx, &points)
}

fn eval_table_points_with_derivative(
    x: Value,
    dx: Derivative,
    points: &[(Value, Value)],
) -> Option<(Value, Derivative)> {
    if points.is_empty() {
        return derivative_pair(0.0, 0.0);
    }
    if points.len() == 1 {
        return derivative_pair(points[0].1, 0.0);
    }
    if x <= points[0].0 {
        return derivative_pair(points[0].1, 0.0);
    }
    let last = points.len() - 1;
    if x >= points[last].0 {
        return derivative_pair(points[last].1, 0.0);
    }
    let mut segment = (points[0], points[1]);
    if x > points[0].0 {
        for pair in points.windows(2) {
            let left = pair[0];
            let right = pair[1];
            segment = (left, right);
            if x <= right.0 {
                break;
            }
        }
    }
    eval_linear_piecewise_segment_with_derivative(x, dx, segment)
}

fn eval_pwl_points_with_derivative(
    x: Value,
    dx: Derivative,
    points: &[(Value, Value)],
) -> Option<(Value, Derivative)> {
    if points.is_empty() {
        return derivative_pair(0.0, 0.0);
    }
    if points.len() == 1 {
        return derivative_pair(points[0].1, 0.0);
    }

    let last = points.len() - 1;
    let ascending = points[last].0 >= points[0].0;
    let segment = if ascending {
        if x <= points[0].0 {
            (points[0], points[1])
        } else if x >= points[last].0 {
            (points[last - 1], points[last])
        } else {
            points
                .windows(2)
                .find(|pair| x >= pair[0].0 && x <= pair[1].0)
                .map(|pair| (pair[0], pair[1]))
                .unwrap_or((points[last - 1], points[last]))
        }
    } else if x >= points[0].0 {
        (points[0], points[1])
    } else if x <= points[last].0 {
        (points[last - 1], points[last])
    } else {
        points
            .windows(2)
            .find(|pair| x <= pair[0].0 && x >= pair[1].0)
            .map(|pair| (pair[0], pair[1]))
            .unwrap_or((points[last - 1], points[last]))
    };

    eval_linear_piecewise_segment_with_derivative(x, dx, segment)
}

fn eval_linear_piecewise_segment_with_derivative(
    x: Value,
    dx: Derivative,
    ((x0, y0), (x1, y1)): ((Value, Value), (Value, Value)),
) -> Option<(Value, Derivative)> {
    let span = x1 - x0;
    if !span.is_finite() || span == 0.0 {
        return derivative_pair(y0, 0.0);
    }
    let slope = (y1 - y0) / span;
    derivative_pair(y0 + (x - x0) * slope, slope * dx)
}

fn xyce_tanh_behavioral(value: Value) -> Value {
    if value > XYCE_TANH_SATURATION_THRESHOLD {
        1.0
    } else if value < -XYCE_TANH_SATURATION_THRESHOLD {
        -1.0
    } else {
        value.tanh()
    }
}

fn bool_value(value: bool) -> Value {
    if value { 1.0 } else { 0.0 }
}
