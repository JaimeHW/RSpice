//! Analytic Taylor coefficients of prescribed time expressions.
//!
//! Coefficients are derivatives divided by factorials. Their binary exponent
//! remains separate until the requested physical derivative is materialized.
//! This is a local evaluator; periodicity and regularity over an orbit must be
//! certified separately before these derivatives become displacement currents.

use super::{CompiledExpr, Context, Instruction};
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::resource::{ResourceKind, ResourceLimitError};
use rspice_veriloga_runtime::arithmetic::ScaledValue as S;

#[derive(Debug, thiserror::Error)]
pub(crate) enum TimeDerivativeError {
    #[error("time-expression differentiation was cancelled")]
    Aborted,
    #[error(transparent)]
    Resource(#[from] ResourceLimitError),
    #[error("time-expression derivative is not finite or representable")]
    NonFinite,
    #[error("time-expression derivative exceeds the retained precision of {0} bits")]
    PrecisionLimit(u32),
    #[error("time-expression derivative requires an unqualified operator or branch")]
    Unsupported,
}

type Result<T> = std::result::Result<T, TimeDerivativeError>;

#[derive(Clone)]
struct Jet {
    coefficients: Vec<S>,
    constant: bool,
}

impl Jet {
    fn point(&self) -> Value {
        self.coefficients[0].binary64()
    }

    fn use_point(&mut self, point: Value) {
        // Keep an underflowed analytic intermediate until the final value and
        // physical derivatives are checked against the VM boundary.
        if point != 0.0 || self.coefficients[0].is_zero() {
            self.coefficients[0] = S::new(point);
        }
    }
}

struct Operations<'a> {
    count: usize,
    abort: &'a dyn AbortSignal,
}

impl Operations<'_> {
    fn sign(&self, value: &Jet, skip_point: bool) -> Result<Option<bool>> {
        for (index, coefficient) in value
            .coefficients
            .iter()
            .enumerate()
            .skip(usize::from(skip_point))
        {
            self.check(index)?;
            if !coefficient.is_zero() {
                return Ok(Some(coefficient.binary64().is_sign_negative()));
            }
        }
        Ok(None)
    }
    fn check(&self, index: usize) -> Result<()> {
        if index.is_multiple_of(64) && self.abort.is_aborted() {
            Err(TimeDerivativeError::Aborted)
        } else {
            Ok(())
        }
    }

    fn constant(&self, value: Value) -> Jet {
        let mut coefficients = vec![S::new(0.0); self.count];
        coefficients[0] = S::new(value);
        Jet {
            coefficients,
            constant: true,
        }
    }

    fn sum(&self, count: usize, terms: impl Fn(usize) -> [S; 2] + Clone, divisor: S) -> Result<S> {
        let value = S::sum_products_div(
            (0..count).map(|index| {
                if index.is_multiple_of(64) && self.abort.is_aborted() {
                    [S::new(Value::NAN), S::new(1.0)]
                } else {
                    terms(index)
                }
            }),
            divisor,
        );
        self.check(0)?;
        value.map_err(|error| match error {
            rspice_veriloga_runtime::arithmetic::ArithmeticError::PrecisionLimit { bits } => {
                TimeDerivativeError::PrecisionLimit(bits)
            }
            _ => TimeDerivativeError::NonFinite,
        })
    }

    fn add(&self, a: &Jet, b: &Jet, subtract: bool) -> Result<Jet> {
        let mut result = self.constant(0.0);
        result.constant = a.constant && b.constant;
        for index in 0..self.count {
            self.check(index)?;
            let second = b.coefficients[index];
            result.coefficients[index] =
                a.coefficients[index].plus(if subtract { second.negated() } else { second });
        }
        Ok(result)
    }

    fn multiply(&self, a: &Jet, b: &Jet) -> Result<Jet> {
        let mut result = self.constant(0.0);
        result.coefficients[0] = a.coefficients[0].multiply(b.coefficients[0]);
        result.constant = a.constant && b.constant;
        for index in 1..self.count {
            result.coefficients[index] = self.sum(
                index + 1,
                |term| [a.coefficients[term], b.coefficients[index - term]],
                S::new(1.0),
            )?;
        }
        Ok(result)
    }

    fn divide(&self, a: &Jet, b: &Jet) -> Result<Jet> {
        if b.coefficients[0].is_zero() {
            return Err(TimeDerivativeError::Unsupported);
        }
        let mut result = self.constant(0.0);
        result.coefficients[0] = a.coefficients[0].divide(b.coefficients[0]);
        result.constant = a.constant && b.constant;
        for index in 1..self.count {
            result.coefficients[index] = self.sum(
                index + 1,
                |term| {
                    if term == 0 {
                        [a.coefficients[index], S::new(1.0)]
                    } else {
                        [
                            b.coefficients[term].negated(),
                            result.coefficients[index - term],
                        ]
                    }
                },
                b.coefficients[0],
            )?;
        }
        Ok(result)
    }

    fn scale(&self, mut value: Jet, scale: Value) -> Result<Jet> {
        for (index, coefficient) in value.coefficients.iter_mut().enumerate() {
            self.check(index)?;
            *coefficient = coefficient.multiply(S::new(scale));
        }
        Ok(value)
    }

    fn differentiated(&self, value: &Jet) -> Result<Jet> {
        let mut result = self.constant(0.0);
        result.constant = value.constant;
        for index in 1..self.count {
            self.check(index)?;
            result.coefficients[index - 1] =
                value.coefficients[index].multiply(S::new(index as Value));
        }
        Ok(result)
    }

    fn integrated(&self, slope: Jet, point: Value) -> Result<Jet> {
        let mut result = self.constant(point);
        result.constant = slope.constant && slope.coefficients[0].is_zero();
        for index in 1..self.count {
            self.check(index)?;
            result.coefficients[index] =
                slope.coefficients[index - 1].divide(S::new(index as Value));
        }
        Ok(result)
    }

    fn exponential(&self, a: &Jet) -> Result<Jet> {
        let mut result = self.constant(a.point().exp());
        if !result.point().is_normal() {
            let exponent = (a.point() / std::f64::consts::LN_2).floor();
            if !exponent.is_finite() || exponent.abs() > u32::MAX as Value {
                return Err(TimeDerivativeError::NonFinite);
            }
            let remainder = (-exponent).mul_add(std::f64::consts::LN_2, a.point());
            let scale = S::new(2.0).powu(exponent.abs() as u32);
            let mantissa = S::new(remainder.exp());
            result.coefficients[0] = if exponent < 0.0 {
                mantissa.divide(scale)
            } else {
                mantissa.multiply(scale)
            };
        }
        result.constant = a.constant;
        for index in 1..self.count {
            result.coefficients[index] = self.sum(
                index,
                |term| {
                    let order = term + 1;
                    [
                        a.coefficients[order].multiply(S::new(order as Value)),
                        result.coefficients[index - order],
                    ]
                },
                S::new(index as Value),
            )?;
        }
        Ok(result)
    }

    fn logarithm(&self, a: &Jet) -> Result<Jet> {
        if a.point() <= 0.0 {
            return Err(TimeDerivativeError::Unsupported);
        }
        self.integrated(self.divide(&self.differentiated(a)?, a)?, a.point().ln())
    }

    fn square_root(&self, a: &Jet) -> Result<Jet> {
        if a.point() <= 0.0 {
            return Err(TimeDerivativeError::Unsupported);
        }
        let mut result = self.constant(a.point().sqrt());
        result.constant = a.constant;
        let divisor = S::new(2.0).multiply(result.coefficients[0]);
        for index in 1..self.count {
            result.coefficients[index] = self.sum(
                index,
                |term| {
                    if term == 0 {
                        [a.coefficients[index], S::new(1.0)]
                    } else {
                        [
                            result.coefficients[term].negated(),
                            result.coefficients[index - term],
                        ]
                    }
                },
                divisor,
            )?;
        }
        Ok(result)
    }

    fn sine_cosine(&self, a: &Jet, hyperbolic: bool) -> Result<(Jet, Jet)> {
        let sine = if hyperbolic {
            a.point().sinh()
        } else {
            a.point().sin()
        };
        let cosine = if hyperbolic {
            a.point().cosh()
        } else {
            a.point().cos()
        };
        self.sine_cosine_at(a, hyperbolic, (sine, cosine))
    }

    fn sine_cosine_at(
        &self,
        a: &Jet,
        hyperbolic: bool,
        point: (Value, Value),
    ) -> Result<(Jet, Jet)> {
        let mut sine = self.constant(point.0);
        let mut cosine = self.constant(point.1);
        sine.constant = a.constant;
        cosine.constant = a.constant;
        for index in 1..self.count {
            sine.coefficients[index] = self.sum(
                index,
                |term| {
                    let order = term + 1;
                    [
                        a.coefficients[order].multiply(S::new(order as Value)),
                        cosine.coefficients[index - order],
                    ]
                },
                S::new(index as Value),
            )?;
            cosine.coefficients[index] = self.sum(
                index,
                |term| {
                    let order = term + 1;
                    [
                        a.coefficients[order].multiply(S::new(order as Value)),
                        sine.coefficients[index - order],
                    ]
                },
                S::new(if hyperbolic {
                    index as Value
                } else {
                    -(index as Value)
                }),
            )?;
        }
        Ok((sine, cosine))
    }

    fn integer_power(&self, a: &Jet, exponent: Value) -> Result<Jet> {
        if !exponent.is_finite() || exponent < 0.0 || exponent.fract() != 0.0 {
            return Err(TimeDerivativeError::Unsupported);
        }
        let mut power = exponent;
        let mut base = a.clone();
        let mut result = self.constant(1.0);
        while power >= 1.0 {
            self.check(0)?;
            if power.rem_euclid(2.0) == 1.0 {
                result = self.multiply(&result, &base)?;
            }
            power = (0.5 * power).floor();
            if power >= 1.0 {
                base = self.multiply(&base, &base)?;
            }
        }
        // Keep the VM's powf point while using polynomial derivatives at zero.
        result.use_point(a.point().powf(exponent));
        Ok(result)
    }

    fn power(&self, a: &Jet, b: &Jet) -> Result<Jet> {
        if b.constant && b.point().fract() == 0.0 {
            if b.point() >= 0.0 {
                return self.integer_power(a, b.point());
            }
            return self.divide(&self.constant(1.0), &self.integer_power(a, -b.point())?);
        }
        let mut result = self.exponential(&self.multiply(b, &self.logarithm(a)?)?)?;
        result.use_point(a.point().powf(b.point()));
        Ok(result)
    }

    fn absolute(&self, a: &Jet) -> Result<Jet> {
        if self.sign(a, false)? == Some(true) {
            self.scale(a.clone(), -1.0)
        } else {
            Ok(a.clone())
        }
    }

    fn projected_power(
        &self,
        a: &Jet,
        b: &Jet,
        instruction: &Instruction,
        context: &Context<'_>,
    ) -> Result<Jet> {
        use crate::config::ExpressionDialect;
        let xyce = context.expression_dialect == ExpressionDialect::Xyce;
        let negative = self.sign(a, false)? == Some(true);
        let absolute = self.absolute(a)?;
        let signed_magnitude = matches!(instruction, Instruction::Pwrs)
            || (!xyce && matches!(instruction, Instruction::Pwr));
        let mut result = if signed_magnitude {
            let value = self.power(&absolute, b)?;
            if negative {
                self.scale(value, -1.0)?
            } else {
                value
            }
        } else if !xyce && matches!(instruction, Instruction::FunctionPow) {
            self.power(&absolute, b)?
        } else if xyce && negative && !(b.constant && b.point().fract() == 0.0) {
            let magnitude = self.power(&absolute, b)?;
            let phase = self.scale(b.clone(), std::f64::consts::PI)?;
            let point = super::power::sin_cos_pi(b.point());
            self.multiply(&magnitude, &self.sine_cosine_at(&phase, false, point)?.1)?
        } else {
            self.power(a, b)?
        };
        result.use_point(match instruction {
            Instruction::FunctionPow => {
                super::real_function_pow(a.point(), b.point(), context.expression_dialect)
            }
            Instruction::Pwr => {
                super::real_function_pwr(a.point(), b.point(), context.expression_dialect)
            }
            Instruction::Pwrs => super::real_function_pwrs(a.point(), b.point()),
            _ => super::real_pow(a.point(), b.point(), context.expression_dialect),
        });
        Ok(result)
    }

    fn inverse_function(
        &self,
        a: &Jet,
        instruction: &Instruction,
        context: &Context<'_>,
    ) -> Result<Jet> {
        let point = a.point();
        let value = match instruction {
            Instruction::Asin => point.clamp(-1.0, 1.0).asin(),
            Instruction::Acos => point.clamp(-1.0, 1.0).acos(),
            Instruction::Atan => point.atan(),
            Instruction::Asinh => point.asinh(),
            Instruction::Acosh => point.acosh(),
            Instruction::Atanh
                if context.expression_dialect == crate::config::ExpressionDialect::Xyce =>
            {
                let limit = 1.0 - super::XYCE_ATANH_EPSILON;
                point.clamp(-limit, limit).atanh()
            }
            Instruction::Atanh => point.atanh(),
            _ => return Err(TimeDerivativeError::Unsupported),
        };
        if a.constant {
            return Ok(self.constant(value));
        }
        if matches!(instruction, Instruction::Asin | Instruction::Acos) && point.abs() > 1.0 {
            return Ok(self.constant(if matches!(instruction, Instruction::Asin) {
                point.clamp(-1.0, 1.0).asin()
            } else {
                point.clamp(-1.0, 1.0).acos()
            }));
        }
        if matches!(instruction, Instruction::Atanh)
            && context.expression_dialect == crate::config::ExpressionDialect::Xyce
        {
            let limit = 1.0 - super::XYCE_ATANH_EPSILON;
            if point.abs() > limit {
                return Ok(self.constant(point.clamp(-limit, limit).atanh()));
            }
            if point.abs() == limit
                && self
                    .sign(a, true)?
                    .is_none_or(|negative| negative == point.is_sign_negative())
            {
                return Ok(self.constant(value));
            }
        }
        // Normalize the quadratic before taking its root. The physical slope
        // of asinh/acosh remains finite even when the unscaled square does not.
        let scale = if matches!(
            instruction,
            Instruction::Atan | Instruction::Asinh | Instruction::Acosh
        ) {
            point.abs().max(1.0)
        } else {
            1.0
        };
        let normalized = self.scale(a.clone(), 1.0 / scale)?;
        let square = self.multiply(&normalized, &normalized)?;
        let one = self.scale(self.scale(self.constant(1.0), 1.0 / scale)?, 1.0 / scale)?;
        let positive = matches!(instruction, Instruction::Atan | Instruction::Asinh);
        let quadratic = if matches!(instruction, Instruction::Acosh) {
            self.add(&square, &one, true)?
        } else {
            self.add(&one, &square, !positive)?
        };
        let denominator = if matches!(instruction, Instruction::Atan | Instruction::Atanh) {
            quadratic
        } else {
            self.square_root(&quadratic)?
        };
        let mut numerator = self.scale(self.differentiated(a)?, 1.0 / scale)?;
        if matches!(instruction, Instruction::Atan) {
            numerator = self.scale(numerator, 1.0 / scale)?;
        }
        let mut slope = self.divide(&numerator, &denominator)?;
        if matches!(instruction, Instruction::Acos) {
            slope = self.scale(slope, -1.0)?;
        }
        self.integrated(slope, value)
    }

    fn polar_angle(&self, y: &Jet, x: &Jet) -> Result<Jet> {
        let point = y.point().atan2(x.point());
        if x.constant && y.constant {
            return Ok(self.constant(point));
        }
        if x.point() < 0.0 && y.point() == 0.0 && !y.constant {
            return Err(TimeDerivativeError::Unsupported);
        }
        let scale = x.point().abs().max(y.point().abs()).max(1.0);
        let x = self.scale(x.clone(), 1.0 / scale)?;
        let y = self.scale(y.clone(), 1.0 / scale)?;
        let numerator = self.add(
            &self.multiply(&x, &self.differentiated(&y)?)?,
            &self.multiply(&y, &self.differentiated(&x)?)?,
            true,
        )?;
        let denominator = self.add(&self.multiply(&x, &x)?, &self.multiply(&y, &y)?, false)?;
        self.integrated(self.divide(&numerator, &denominator)?, point)
    }

    fn hyperbolic_tangent(&self, a: &Jet, context: &Context<'_>) -> Result<Jet> {
        let point = a.point();
        if point.abs() <= 1.0 {
            let (sinh, cosh) = self.sine_cosine(a, true)?;
            let mut result = self.divide(&sinh, &cosh)?;
            result.coefficients[0] = S::new(point.tanh());
            return Ok(result);
        }
        if context.expression_dialect == crate::config::ExpressionDialect::Xyce {
            if point.abs() > super::XYCE_TANH_SATURATION_THRESHOLD {
                return Ok(self.constant(point.signum()));
            }
            if point.abs() == super::XYCE_TANH_SATURATION_THRESHOLD
                && self
                    .sign(a, true)?
                    .is_none_or(|negative| negative == point.is_sign_negative())
            {
                return Ok(self.constant(point.signum()));
            }
        }
        // Use the decaying exponential on either half-plane. sinh/cosh would
        // overflow, and 1-tanh(x)^2 erases small but finite derivatives.
        let sign = if point < 0.0 { -1.0 } else { 1.0 };
        let exponential = self.exponential(&self.scale(a.clone(), -2.0 * sign)?)?;
        let one = self.constant(1.0);
        let mut result = self.divide(
            &self.add(&one, &exponential, true)?,
            &self.add(&one, &exponential, false)?,
        )?;
        if sign < 0.0 {
            result = self.scale(result, -1.0)?;
        }
        result.coefficients[0] = S::new(point.tanh());
        Ok(result)
    }
}

/// A borrowed compiled program and a checked bound for its Taylor workspace.
pub(crate) struct TimeDerivatives<'a> {
    program: &'a CompiledExpr,
    order: usize,
    stack_capacity: usize,
}

impl<'a> TimeDerivatives<'a> {
    pub(crate) fn supports(program: &CompiledExpr) -> bool {
        program.node_map.is_empty()
            && program.branch_map.is_empty()
            && program.sdt_count == 0
            && program
                .instructions
                .iter()
                .all(|instruction| Self::arity(instruction).is_some())
    }

    fn arity(instruction: &Instruction) -> Option<usize> {
        Some(match instruction {
            Instruction::PushConst(_)
            | Instruction::PushTime
            | Instruction::PushFreq
            | Instruction::PushTemperature
            | Instruction::PushThermalVoltage
            | Instruction::PushGmin => 0,
            Instruction::Dup => 0,
            Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Pow
            | Instruction::FunctionPow
            | Instruction::Pwr
            | Instruction::Pwrs
            | Instruction::Atan2 => 2,
            Instruction::Neg
            | Instruction::Abs
            | Instruction::Sqrt
            | Instruction::Exp
            | Instruction::Log
            | Instruction::Ln
            | Instruction::Log10
            | Instruction::Sin
            | Instruction::Cos
            | Instruction::Tan
            | Instruction::Asin
            | Instruction::Acos
            | Instruction::Atan
            | Instruction::Sinh
            | Instruction::Cosh
            | Instruction::Tanh
            | Instruction::Asinh
            | Instruction::Acosh
            | Instruction::Atanh
            | Instruction::Sqr => 1,
            _ => return None,
        })
    }

    pub(crate) fn new(
        program: &'a CompiledExpr,
        order: usize,
        max_values: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        if abort.is_aborted() {
            return Err(TimeDerivativeError::Aborted);
        }
        if !program.node_map.is_empty() || !program.branch_map.is_empty() || program.sdt_count != 0
        {
            return Err(TimeDerivativeError::Unsupported);
        }
        let mut depth = 0_usize;
        let mut maximum = 0_usize;
        for (index, instruction) in program.instructions.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(TimeDerivativeError::Aborted);
            }
            let arity = Self::arity(instruction).ok_or(TimeDerivativeError::Unsupported)?;
            if matches!(instruction, Instruction::Dup) && depth == 0 {
                return Err(TimeDerivativeError::Unsupported);
            }
            depth = depth
                .checked_sub(arity)
                .ok_or(TimeDerivativeError::Unsupported)?
                + 1;
            maximum = maximum.max(depth);
        }
        if depth != 1 {
            return Err(TimeDerivativeError::Unsupported);
        }
        // Include stack headers and the maximum simultaneous intermediate jets
        // used by composed functions, before allocating any coefficient array.
        let words = order
            .checked_add(1)
            .and_then(|count| count.checked_mul(2))
            .and_then(|words| words.checked_add(4))
            .and_then(|words| words.checked_mul(maximum.saturating_add(16)))
            // Interval certification has a wider scalar stack. The exact sum
            // recovery bounds each accumulator's exponent span to 65536 bits;
            // include its signed accumulators, quotient and growth headroom.
            .and_then(|words| words.checked_add(maximum.saturating_mul(24)))
            .and_then(|words| words.checked_add(32 * 1024))
            .filter(|words| *words <= isize::MAX as usize / 8)
            .ok_or(TimeDerivativeError::Resource(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: usize::MAX,
                limit: max_values,
            }))?;
        ResourceLimitError::ensure(ResourceKind::ResultValues, words, max_values)?;
        Ok(Self {
            program,
            order,
            stack_capacity: maximum,
        })
    }

    pub(crate) fn evaluate(
        &self,
        context: &Context<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>> {
        let ops = Operations {
            count: self.order + 1,
            abort,
        };
        ops.check(0)?;
        if self.order == 0 {
            let point = super::Vm::new().execute(self.program, context);
            ops.check(0)?;
            return if point.is_finite() {
                Ok(vec![point])
            } else {
                Err(TimeDerivativeError::NonFinite)
            };
        }
        let mut stack = Vec::<Jet>::with_capacity(self.stack_capacity);
        let mut vm = super::Vm::new();
        let mut constant_program = CompiledExpr::new();
        constant_program.instructions = Vec::with_capacity(3);
        for (index, instruction) in self.program.instructions.iter().enumerate() {
            ops.check(index)?;
            let arity = Self::arity(instruction).ok_or(TimeDerivativeError::Unsupported)?;
            if arity != 0
                && stack[stack.len() - arity..]
                    .iter()
                    .all(|value| value.constant)
            {
                // A fixed subexpression obeys the VM's binary64 semantics.
                // Rescuing its underflow would turn a zero-valued coefficient
                // into a nonzero displacement current after differentiation.
                constant_program.instructions.clear();
                for value in stack.drain(stack.len() - arity..) {
                    constant_program
                        .instructions
                        .push(Instruction::PushConst(value.point()));
                }
                constant_program.instructions.push(instruction.clone());
                let point = vm.execute_raw(&constant_program, context);
                if !point.is_finite() {
                    return Err(TimeDerivativeError::NonFinite);
                }
                stack.push(ops.constant(point));
                continue;
            }
            let value = match instruction {
                Instruction::PushConst(value) => ops.constant(*value),
                Instruction::PushTime => {
                    let mut time = ops.constant(context.time);
                    time.constant = false;
                    if self.order > 0 {
                        time.coefficients[1] = S::new(1.0);
                    }
                    time
                }
                Instruction::PushFreq => ops.constant(context.frequency),
                Instruction::PushTemperature => ops.constant(context.temperature),
                Instruction::PushThermalVoltage => ops.constant(crate::constants::thermal_voltage(
                    crate::constants::celsius_to_kelvin(context.temperature),
                )),
                Instruction::PushGmin => ops.constant(context.gmin),
                Instruction::Dup => stack
                    .last()
                    .ok_or(TimeDerivativeError::Unsupported)?
                    .clone(),
                Instruction::Add
                | Instruction::Sub
                | Instruction::Mul
                | Instruction::Div
                | Instruction::Pow
                | Instruction::FunctionPow
                | Instruction::Pwr
                | Instruction::Pwrs
                | Instruction::Atan2 => {
                    let b = stack.pop().ok_or(TimeDerivativeError::Unsupported)?;
                    let a = stack.pop().ok_or(TimeDerivativeError::Unsupported)?;
                    match instruction {
                        Instruction::Add => ops.add(&a, &b, false)?,
                        Instruction::Sub => ops.add(&a, &b, true)?,
                        Instruction::Mul => ops.multiply(&a, &b)?,
                        Instruction::Div => ops.divide(&a, &b)?,
                        Instruction::Atan2 => ops.polar_angle(&a, &b)?,
                        _ => ops.projected_power(&a, &b, instruction, context)?,
                    }
                }
                _ => {
                    let a = stack.pop().ok_or(TimeDerivativeError::Unsupported)?;
                    match instruction {
                        Instruction::Neg => ops.scale(a, -1.0)?,
                        Instruction::Abs => ops.absolute(&a)?,
                        Instruction::Sqr => ops.multiply(&a, &a)?,
                        Instruction::Sqrt => {
                            if a.point() < 0.0 || (a.point() == 0.0 && a.constant) {
                                ops.constant(0.0)
                            } else {
                                ops.square_root(&a)?
                            }
                        }
                        Instruction::Exp => ops.exponential(&a)?,
                        Instruction::Ln | Instruction::Log | Instruction::Log10 => {
                            let base10 = matches!(instruction, Instruction::Log10)
                                || (matches!(instruction, Instruction::Log)
                                    && context.expression_dialect
                                        == crate::config::ExpressionDialect::Xyce);
                            let point = a.point().max(super::LOGARITHM_MIN_ARGUMENT);
                            let value = if base10 { point.log10() } else { point.ln() };
                            if a.point() < super::LOGARITHM_MIN_ARGUMENT
                                || a.constant
                                || (a.point() == super::LOGARITHM_MIN_ARGUMENT
                                    && ops.sign(&a, true)? != Some(false))
                            {
                                ops.constant(value)
                            } else {
                                let mut result = ops.logarithm(&a)?;
                                if base10 {
                                    result = ops.scale(result, 1.0 / std::f64::consts::LN_10)?;
                                }
                                result.coefficients[0] = S::new(value);
                                result
                            }
                        }
                        Instruction::Sin => ops.sine_cosine(&a, false)?.0,
                        Instruction::Cos => ops.sine_cosine(&a, false)?.1,
                        Instruction::Tan => {
                            let (sine, cosine) = ops.sine_cosine(&a, false)?;
                            let mut value = ops.divide(&sine, &cosine)?;
                            value.coefficients[0] = S::new(a.point().tan());
                            value
                        }
                        Instruction::Sinh => ops.sine_cosine(&a, true)?.0,
                        Instruction::Cosh => ops.sine_cosine(&a, true)?.1,
                        Instruction::Tanh => ops.hyperbolic_tangent(&a, context)?,
                        Instruction::Asin
                        | Instruction::Acos
                        | Instruction::Atan
                        | Instruction::Asinh
                        | Instruction::Acosh
                        | Instruction::Atanh => ops.inverse_function(&a, instruction, context)?,
                        _ => return Err(TimeDerivativeError::Unsupported),
                    }
                }
            };
            if !value.point().is_finite() {
                return Err(TimeDerivativeError::NonFinite);
            }
            stack.push(value);
        }
        let jet = stack.pop().ok_or(TimeDerivativeError::Unsupported)?;
        let mut factorial = S::new(1.0);
        let mut values = Vec::with_capacity(self.order + 1);
        for (index, coefficient) in jet.coefficients.into_iter().enumerate() {
            ops.check(index)?;
            if index != 0 {
                factorial = factorial.multiply(S::new(index as Value));
            }
            let scaled = coefficient.multiply(factorial);
            let value = scaled.binary64();
            if !value.is_finite() || (index != 0 && value == 0.0 && !scaled.is_zero()) {
                return Err(TimeDerivativeError::NonFinite);
            }
            values.push(value);
        }
        ops.check(0)?;
        let point = vm.execute(self.program, context);
        ops.check(0)?;
        let scale = point.abs().max(values[0].abs());
        if !point.is_finite()
            || (point != values[0] && (point - values[0]).abs() > 8.0 * Value::EPSILON * scale)
        {
            return Err(TimeDerivativeError::Unsupported);
        }
        values[0] = point;
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    #[test]
    fn time_taylor_derivatives_match_independent_closed_forms() {
        for (expression, time) in [
            ("exp(3*time)", 0.2),
            ("sin(2*time)", 0.0),
            ("1/(1-time)", 0.0),
            ("ln(1+time)", 0.0),
            ("(sin(time))^4", 0.0),
        ] {
            let program =
                super::super::compile(&super::super::parse_expression_strict(expression).unwrap());
            let derivatives = TimeDerivatives::new(&program, 12, 100_000, &NoAbort)
                .unwrap()
                .evaluate(&Context::transient(&[], &[], time), &NoAbort)
                .unwrap();
            let mut factorial = 1.0;
            for (order, &actual) in derivatives.iter().enumerate().skip(1) {
                let previous_factorial = factorial;
                factorial *= order as Value;
                let phase = order % 4;
                let sine = [0.0, 1.0, 0.0, -1.0][phase];
                let cosine = [1.0, 0.0, -1.0, 0.0][phase];
                let expected = match expression {
                    "exp(3*time)" => 3.0_f64.powi(order as i32) * (3.0 * time).exp(),
                    "sin(2*time)" => 2.0_f64.powi(order as i32) * sine,
                    "1/(1-time)" => factorial,
                    "ln(1+time)" => {
                        if order % 2 == 0 {
                            -previous_factorial
                        } else {
                            previous_factorial
                        }
                    }
                    _ => {
                        (-0.5 * 2.0_f64.powi(order as i32) + 0.125 * 4.0_f64.powi(order as i32))
                            * cosine
                    }
                };
                assert!(
                    (actual - expected).abs() <= 2e-13 * expected.abs().max(1.0),
                    "{expression}, order {order}: {actual} vs {expected}"
                );
            }
        }
    }

    #[test]
    fn time_taylor_workspace_and_cancellation_precede_evaluation() {
        let program = super::super::compile(
            &super::super::parse_expression_strict("exp(sin(time))").unwrap(),
        );
        assert!(matches!(
            TimeDerivatives::new(&program, usize::MAX, usize::MAX, &NoAbort),
            Err(TimeDerivativeError::Resource(_))
        ));
        assert!(matches!(
            TimeDerivatives::new(&program, 20, 1, &NoAbort),
            Err(TimeDerivativeError::Resource(_))
        ));
        let evaluation = TimeDerivatives::new(&program, 128, 100_000, &NoAbort).unwrap();
        assert!(matches!(
            evaluation.evaluate(&Context::transient(&[], &[], 0.0), &CountingAbort::new(8)),
            Err(TimeDerivativeError::Aborted)
        ));
    }

    #[test]
    fn time_taylor_inverse_functions_and_range_match_closed_forms() {
        let root3 = 3.0_f64.sqrt();
        let cases = [
            ("asin(time)", vec![0.0, 1.0, 0.0, 1.0, 0.0, 9.0]),
            ("asinh(time)", vec![0.0, 1.0, 0.0, -1.0, 0.0, 9.0]),
            ("atan(time)", vec![0.0, 1.0, 0.0, -2.0, 0.0, 24.0]),
            ("atanh(time)", vec![0.0, 1.0, 0.0, 2.0, 0.0, 24.0]),
            ("tanh(time)", vec![0.0, 1.0, 0.0, -2.0, 0.0, 16.0]),
            (
                "acosh(2+time)",
                vec![
                    2.0_f64.acosh(),
                    1.0 / root3,
                    -2.0 / (3.0 * root3),
                    1.0 / root3,
                ],
            ),
            (
                "acos(0.5+time)",
                vec![0.5_f64.acos(), -2.0 / root3, -4.0 / (3.0 * root3)],
            ),
            ("acos(1)", vec![0.0, 0.0, 0.0]),
            ("acosh(1)", vec![0.0, 0.0, 0.0]),
            (
                "atan(1e200+1e200*time)",
                vec![std::f64::consts::FRAC_PI_2, 1e-200, -2e-200],
            ),
            (
                "asinh(1e200+1e200*time)",
                vec![1e200_f64.asinh(), 1.0, -1.0, 2.0],
            ),
            (
                "acosh(1e200+1e200*time)",
                vec![1e200_f64.acosh(), 1.0, -1.0, 2.0],
            ),
            ("atan2(1e200*time,1e200)", vec![0.0, 1.0, 0.0, -2.0]),
            (
                "tanh(400+1e300*time)",
                vec![1.0, 4.0 * (1e300_f64.ln() - 800.0).exp()],
            ),
            ("(1+time)^(-2)", vec![1.0, -2.0, 6.0, -24.0]),
            (
                "(1+time)^4294967296",
                vec![1.0, 4294967296.0, 4294967296.0 * 4294967295.0],
            ),
        ];
        for (expression, expected) in cases {
            let program =
                super::super::compile(&super::super::parse_expression_strict(expression).unwrap());
            let values = TimeDerivatives::new(&program, expected.len() - 1, 100_000, &NoAbort)
                .unwrap()
                .evaluate(&Context::transient(&[], &[], 0.0), &NoAbort)
                .unwrap_or_else(|error| panic!("{expression}: {error}"));
            for (order, (actual, expected)) in values.into_iter().zip(expected).enumerate() {
                assert!(
                    (actual - expected).abs() <= 2e-13 * expected.abs().max(1e-300),
                    "{expression}, order {order}: {actual:e} vs {expected:e}"
                );
            }
        }
    }

    #[test]
    fn time_taylor_preserves_dialect_power_and_clamp_semantics() {
        use crate::config::ExpressionDialect::{Ngspice, Xyce};
        let cosine = std::f64::consts::FRAC_PI_2.cos();
        for (expression, dialect, expected) in [
            (
                "pow(-2+time,0.5)",
                Ngspice,
                vec![2.0_f64.sqrt(), -0.5 / 2.0_f64.sqrt()],
            ),
            (
                "pwr(-2+time,0.5)",
                Ngspice,
                vec![-2.0_f64.sqrt(), 0.5 / 2.0_f64.sqrt()],
            ),
            (
                "pwrs(-2+time,0.5)",
                Xyce,
                vec![-2.0_f64.sqrt(), 0.5 / 2.0_f64.sqrt()],
            ),
            (
                "(-2+time)^0.5",
                Xyce,
                vec![
                    2.0_f64.sqrt() * cosine,
                    -0.5 / 2.0_f64.sqrt() * cosine,
                    -0.125 / 2.0_f64.sqrt() * cosine,
                ],
            ),
            (
                "(-1)^(0.5+time)",
                Xyce,
                vec![
                    cosine,
                    -std::f64::consts::PI,
                    -std::f64::consts::PI.powi(2) * cosine,
                    std::f64::consts::PI.powi(3),
                ],
            ),
            ("tanh(400+1e300*time)", Xyce, vec![1.0, 0.0]),
            (
                "asin(2+time)",
                Ngspice,
                vec![std::f64::consts::FRAC_PI_2, 0.0, 0.0],
            ),
            ("sqrt(-1+time)", Ngspice, vec![0.0, 0.0, 0.0]),
            (
                "ln(-1+time)",
                Ngspice,
                vec![super::super::LOGARITHM_MIN_ARGUMENT.ln(), 0.0],
            ),
            ("log(1+time)", Ngspice, vec![0.0, 1.0, -1.0]),
            (
                "log(1+time)",
                Xyce,
                vec![
                    0.0,
                    1.0 / std::f64::consts::LN_10,
                    -1.0 / std::f64::consts::LN_10,
                ],
            ),
        ] {
            let program =
                super::super::compile(&super::super::parse_expression_strict(expression).unwrap());
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            let values = TimeDerivatives::new(&program, expected.len() - 1, 100_000, &NoAbort)
                .unwrap()
                .evaluate(&context, &NoAbort)
                .unwrap();
            assert_eq!(
                values[0],
                super::super::Vm::new().execute(&program, &context),
                "{expression}"
            );
            for (actual, expected) in values.into_iter().zip(expected) {
                assert!(
                    (actual - expected).abs() <= 2e-13 * expected.abs().max(1e-300),
                    "{expression}: {actual:e} vs {expected:e}"
                );
            }
        }
    }

    #[test]
    fn time_taylor_retains_small_intermediates_without_inventing_vm_values() {
        let evaluate = |expression, order| {
            let program =
                super::super::compile(&super::super::parse_expression_strict(expression).unwrap());
            TimeDerivatives::new(&program, order, 100_000, &NoAbort)
                .unwrap()
                .evaluate(&Context::transient(&[], &[], 0.0), &NoAbort)
        };
        assert_eq!(evaluate("exp(-800)+time", 1).unwrap(), [0.0, 1.0]);
        assert_eq!(evaluate("exp(-800)*time*1e300", 1).unwrap(), [0.0, 0.0]);
        assert_eq!(evaluate("(0^0.5)*time", 1).unwrap(), [0.0, 0.0]);
        assert_eq!(evaluate("(1/0)*time", 1).unwrap(), [0.0, 0.0]);
        let square = evaluate("(1e-200+time)^2", 2).unwrap();
        assert_eq!(square, [0.0, 2e-200, 2.0]);
        assert!(matches!(
            evaluate("(1e-200+time)^2/(1e-200+time)", 1),
            Err(TimeDerivativeError::Unsupported)
        ));
        let derivatives = evaluate("exp(-800+1e300*time)", 1).unwrap();
        let expected = (1e300_f64.ln() - 800.0).exp();
        assert!((derivatives[1] / expected - 1.0).abs() < 2e-13);
        assert!(matches!(
            evaluate("(exp(-50000+time)+time)*(1+time)", 1),
            Err(TimeDerivativeError::PrecisionLimit(_))
        ));
    }
}
