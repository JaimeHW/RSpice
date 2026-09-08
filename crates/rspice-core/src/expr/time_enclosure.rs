//! Continuous time-expression bounds for event isolation, below device code.
//!
//! Point values still come from the ordinary VM. This interpreter propagates
//! value and derivative intervals through that same compiled program. Its
//! derivative is with respect to normalized analysis time, avoiding an
//! unnecessary reciprocal of extremely small or large physical periods.

use super::{CompiledExpr, Context, Instruction};
use crate::Value;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TimeInterval {
    pub lower: Value,
    pub upper: Value,
}

impl TimeInterval {
    const ZERO: Self = Self::point(0.0);
    const WHOLE: Self = Self {
        lower: Value::NEG_INFINITY,
        upper: Value::INFINITY,
    };

    const fn point(value: Value) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }

    pub fn contains(self, value: Value) -> bool {
        self.lower <= value && value <= self.upper
    }

    pub fn is_finite(self) -> bool {
        self.lower.is_finite() && self.upper.is_finite()
    }

    fn outward(lower: Value, upper: Value) -> Self {
        if lower.is_nan() || upper.is_nan() {
            return Self::WHOLE;
        }
        Self {
            lower: lower.next_down(),
            upper: upper.next_up(),
        }
    }

    fn add(self, other: Self) -> Self {
        if self.lower == 0.0 && self.upper == 0.0 {
            return other;
        }
        if other.lower == 0.0 && other.upper == 0.0 {
            return self;
        }
        Self::outward(self.lower + other.lower, self.upper + other.upper)
    }

    fn neg(self) -> Self {
        Self {
            lower: -self.upper,
            upper: -self.lower,
        }
    }

    fn mul(self, other: Self) -> Self {
        if (self.lower == 0.0 && self.upper == 0.0 && other.is_finite())
            || (other.lower == 0.0 && other.upper == 0.0 && self.is_finite())
        {
            return Self::ZERO;
        }
        let products = [
            self.lower * other.lower,
            self.lower * other.upper,
            self.upper * other.lower,
            self.upper * other.upper,
        ];
        if products.iter().any(|value| value.is_nan()) {
            return Self::WHOLE;
        }
        Self::outward(
            products.into_iter().fold(Value::INFINITY, Value::min),
            products.into_iter().fold(Value::NEG_INFINITY, Value::max),
        )
    }

    fn div(self, other: Self) -> Option<Self> {
        if other.contains(0.0) {
            return None;
        }
        if self.lower == 0.0 && self.upper == 0.0 && other.is_finite() {
            return Some(Self::ZERO);
        }
        // Form the quotient directly: a reciprocal can overflow even when
        // every quotient here is small and representable.
        let quotients = [
            self.lower / other.lower,
            self.lower / other.upper,
            self.upper / other.lower,
            self.upper / other.upper,
        ];
        if quotients.iter().any(|value| value.is_nan()) {
            return Some(Self::WHOLE);
        }
        Some(Self::outward(
            quotients.into_iter().fold(Value::INFINITY, Value::min),
            quotients.into_iter().fold(Value::NEG_INFINITY, Value::max),
        ))
    }

    fn union(self, other: Self) -> Self {
        Self {
            lower: self.lower.min(other.lower),
            upper: self.upper.max(other.upper),
        }
    }

    // Transcendental endpoints include a relative libm rounding allowance
    // plus an outward ULP, including at subnormal/zero results.
    fn transcendental(lower: Value, upper: Value) -> Self {
        Self::outward(
            lower - 16.0 * Value::EPSILON * lower.abs(),
            upper + 16.0 * Value::EPSILON * upper.abs(),
        )
    }

    fn positive_power(self, exponent: Self) -> Self {
        // For a nonnegative base, x^y is exp(y*ln(x)); the bilinear exponent
        // reaches its extremes at rectangle corners. IEEE zero-base limits
        // also follow these corners, including the isolated 0^0 value.
        let corners = [
            self.lower.powf(exponent.lower),
            self.lower.powf(exponent.upper),
            self.upper.powf(exponent.lower),
            self.upper.powf(exponent.upper),
        ];
        if corners.iter().any(|value| value.is_nan()) {
            return Self::WHOLE;
        }
        let minimum = corners.into_iter().fold(Value::INFINITY, Value::min);
        let maximum = corners.into_iter().fold(Value::NEG_INFINITY, Value::max);
        let mut range = Self::transcendental(minimum, maximum);
        range.lower = range.lower.max(0.0);
        range
    }

    fn logarithm(self) -> Self {
        Self::transcendental(self.lower.ln(), self.upper.ln())
    }

    fn square(self) -> Self {
        let maximum = self.lower.abs().max(self.upper.abs());
        if self.contains(0.0) {
            Self {
                lower: 0.0,
                upper: (maximum * maximum).next_up(),
            }
        } else {
            let minimum = self.lower.abs().min(self.upper.abs());
            Self::outward(minimum * minimum, maximum * maximum)
        }
    }

    fn trigonometric(self, cosine: bool) -> Self {
        // Beyond reliable phase-index arithmetic, the full codomain is the
        // useful bound. Isolation may subdivide, or reach its work limit.
        if !self.is_finite()
            || self.upper - self.lower >= std::f64::consts::TAU
            || self.lower.abs().max(self.upper.abs()) >= (1_u64 << 48) as Value
        {
            return Self {
                lower: -1.0,
                upper: 1.0,
            };
        }
        let evaluate = |phase: Value| if cosine { phase.cos() } else { phase.sin() };
        let a = evaluate(self.lower);
        let b = evaluate(self.upper);
        // Include phase and libm rounding around the critical-point test.
        let margin = 16.0 * Value::EPSILON * self.lower.abs().max(self.upper.abs()).max(1.0);
        let shift = if cosine {
            0.0
        } else {
            std::f64::consts::FRAC_PI_2
        };
        let first = ((self.lower - margin - shift) / std::f64::consts::PI).ceil();
        let last = ((self.upper + margin - shift) / std::f64::consts::PI).floor();
        let mut result = Self {
            lower: (a.min(b) - 16.0 * Value::EPSILON).max(-1.0),
            upper: (a.max(b) + 16.0 * Value::EPSILON).min(1.0),
        };
        if last - first >= 1.0 {
            return Self {
                lower: -1.0,
                upper: 1.0,
            };
        }
        if first <= last {
            if first.rem_euclid(2.0) == 0.0 {
                result.upper = 1.0;
            } else {
                result.lower = -1.0;
            }
        }
        result
    }
}

#[derive(Clone, Copy)]
struct Dual {
    value: TimeInterval,
    slope: TimeInterval,
    constant: bool,
    continuous: bool,
}

/// Bounds on one evaluation domain. Continuity is separate from a finite
/// derivative: a continuous cusp may have an unbounded derivative, whereas
/// a signed zero power has a finite-valued jump.
pub(crate) struct TimeBounds {
    pub value: TimeInterval,
    pub slope: TimeInterval,
    pub continuous: bool,
}

impl Dual {
    fn constant(value: Value) -> Self {
        Self {
            value: TimeInterval::point(value),
            slope: TimeInterval::ZERO,
            constant: true,
            continuous: true,
        }
    }

    fn add(self, other: Self) -> Self {
        if self.constant && other.constant {
            return Self::constant(self.value.lower + other.value.lower);
        }
        Self {
            value: self.value.add(other.value),
            slope: self.slope.add(other.slope),
            constant: false,
            continuous: self.continuous && other.continuous,
        }
    }

    fn neg(self) -> Self {
        Self {
            value: self.value.neg(),
            slope: self.slope.neg(),
            ..self
        }
    }

    fn mul(self, other: Self) -> Self {
        if self.constant && other.constant {
            return Self::constant(self.value.lower * other.value.lower);
        }
        if (self.value.lower == 0.0 && self.value.upper == 0.0 && other.value.is_finite())
            || (other.value.lower == 0.0 && other.value.upper == 0.0 && self.value.is_finite())
        {
            return Self::constant(0.0);
        }
        Self {
            value: self.value.mul(other.value),
            slope: self.slope.mul(other.value).add(self.value.mul(other.slope)),
            constant: false,
            continuous: self.continuous && other.continuous,
        }
    }

    fn square(self) -> Self {
        if self.constant {
            return Self::constant(self.value.lower * self.value.lower);
        }
        Self {
            value: self.value.square(),
            slope: TimeInterval::point(2.0).mul(self.value).mul(self.slope),
            constant: false,
            continuous: self.continuous,
        }
    }

    fn div(self, other: Self) -> Option<Self> {
        // Keep the VM's defined zero-denominator value, including locally
        // exact zeros, without declaring an uncertain crossing continuous.
        if other.value.lower == 0.0 && other.value.upper == 0.0 {
            return Some(Self::constant(0.0));
        }
        if self.value.lower == 0.0 && self.value.upper == 0.0 && other.value.is_finite() {
            return Some(Self::constant(0.0));
        }
        if self.constant && other.constant {
            return Some(Self::constant(self.value.lower / other.value.lower));
        }
        let value = self.value.div(other.value)?;
        Some(Self {
            value,
            // u'/v - (u/v)*(v'/v) avoids both v^2 underflow and reciprocal
            // overflow. Each quotient is independently enclosed.
            slope: self
                .slope
                .div(other.value)?
                .add(value.mul(other.slope.div(other.value)?).neg()),
            constant: false,
            continuous: self.continuous && other.continuous,
        })
    }

    fn positive_power(self, exponent: Self) -> Self {
        if exponent.constant {
            if exponent.value.lower == 0.0 {
                return Self::constant(1.0);
            }
            if exponent.value.lower == 1.0 {
                return self;
            }
        }
        if self.constant && self.value.lower == 0.0 && exponent.value.lower > 0.0 {
            return Self::constant(0.0);
        }
        let value = self.value.positive_power(exponent.value);
        let mut slope = TimeInterval::ZERO;
        if !self.constant {
            slope = exponent
                .value
                .mul(
                    self.value
                        .positive_power(exponent.value.add(TimeInterval::point(-1.0))),
                )
                .mul(self.slope);
        }
        if !exponent.constant {
            slope = slope.add(value.mul(self.value.logarithm()).mul(exponent.slope));
        }
        Self {
            value,
            slope,
            constant: false,
            continuous: self.continuous
                && exponent.continuous
                && !(self.value.contains(0.0)
                    && exponent.value.contains(0.0)
                    && !exponent.constant),
        }
    }

    fn power(
        self,
        exponent: Self,
        instruction: &Instruction,
        context: &Context<'_>,
    ) -> Option<Self> {
        let evaluate = |base, exponent| match instruction {
            Instruction::Pow => super::real_pow(base, exponent, context.expression_dialect),
            Instruction::FunctionPow => {
                super::real_function_pow(base, exponent, context.expression_dialect)
            }
            Instruction::Pwr => {
                super::real_function_pwr(base, exponent, context.expression_dialect)
            }
            Instruction::Pwrs => super::real_function_pwrs(base, exponent),
            _ => unreachable!("power instruction was selected by the bounds interpreter"),
        };
        if self.constant && exponent.constant {
            return Some(Self::constant(evaluate(
                self.value.lower,
                exponent.value.lower,
            )));
        }
        if !self.value.is_finite() || !exponent.value.is_finite() {
            return None;
        }
        // A bounded exact constant can arise from a time-dependent spelling
        // such as 0*time+2. Its derivative and value authenticate this fold.
        let exponent = Self {
            constant: exponent.constant
                || (exponent.value.lower == exponent.value.upper
                    && exponent.slope.lower == 0.0
                    && exponent.slope.upper == 0.0),
            ..exponent
        };
        let mut result = None;
        if self.value.upper >= 0.0 {
            let positive = Self {
                value: TimeInterval {
                    lower: self.value.lower.max(0.0),
                    upper: self.value.upper,
                },
                ..self
            };
            result = Some(positive.positive_power(exponent));
        }
        if self.value.lower < 0.0 {
            let negative = Self {
                value: TimeInterval {
                    lower: self.value.lower,
                    upper: self.value.upper.min(0.0),
                },
                ..self
            };
            let coefficient = if exponent.constant {
                // The existing power helpers own sign, magnitude and Xyce's
                // complex projection. Do not duplicate those dialect rules.
                let coefficient = evaluate(-1.0, exponent.value.lower);
                if !coefficient.is_finite() {
                    return None;
                }
                Self::constant(coefficient)
            } else if matches!(instruction, Instruction::Pwrs)
                || (matches!(instruction, Instruction::Pwr)
                    && context.expression_dialect != crate::config::ExpressionDialect::Xyce)
            {
                Self::constant(-1.0)
            } else if context.expression_dialect == crate::config::ExpressionDialect::Xyce {
                exponent
                    .mul(Self::constant(std::f64::consts::PI))
                    .trigonometric(true)
            } else if matches!(instruction, Instruction::FunctionPow) {
                Self::constant(1.0)
            } else {
                // A varying real exponent on a negative base has no open
                // real domain under the ordinary power-operator contract.
                return None;
            };
            let powered = negative.neg().positive_power(exponent).mul(coefficient);
            result = Some(match result {
                None => powered,
                Some(positive) => Self {
                    value: positive.value.union(powered.value),
                    slope: positive.slope.union(powered.slope),
                    continuous: positive.continuous
                        && powered.continuous
                        && !(exponent.constant
                            && exponent.value.lower == 0.0
                            && positive.value.lower != powered.value.upper),
                    constant: false,
                },
            });
        }
        result
    }

    fn trigonometric(self, cosine: bool) -> Self {
        if self.constant {
            return Self::constant(if cosine {
                self.value.lower.cos()
            } else {
                self.value.lower.sin()
            });
        }
        let derivative = self.value.trigonometric(!cosine);
        Self {
            value: self.value.trigonometric(cosine),
            slope: self
                .slope
                .mul(if cosine { derivative.neg() } else { derivative }),
            constant: false,
            continuous: self.continuous,
        }
    }

    fn exponential(self) -> Self {
        if self.constant {
            return Self::constant(self.value.lower.exp());
        }
        // Below one quarter of the least subnormal, every exp result rounds
        // to zero. Keep a full factor of two beyond the rounding boundary;
        // this is an exact VM plateau, not a small-signal tolerance.
        let zero_limit = Value::from_bits(1).ln() - 2.0 * std::f64::consts::LN_2;
        if self.value.upper < zero_limit {
            return Self::constant(0.0);
        }
        let mut value =
            TimeInterval::transcendental(self.value.lower.exp(), self.value.upper.exp());
        value.lower = value.lower.max(0.0);
        Self {
            value,
            slope: value.mul(self.slope),
            constant: false,
            continuous: self.continuous,
        }
    }
}

/// Reusable scratch for continuous expressions whose bounds are implemented.
/// Construction refuses unsupported operators. An unavailable evaluation
/// means the current domain is unresolved and may require subdivision.
pub(crate) struct TimeEnclosure<'a> {
    program: &'a CompiledExpr,
    stack: Vec<Dual>,
    stop: Value,
}

impl<'a> TimeEnclosure<'a> {
    pub fn new(program: &'a CompiledExpr, stop: Value) -> Option<Self> {
        if !stop.is_finite()
            || stop <= 0.0
            || !program.node_map.is_empty()
            || !program.branch_map.is_empty()
            || program.sdt_count != 0
        {
            return None;
        }
        if program.instructions.iter().any(|instruction| {
            !matches!(
                instruction,
                Instruction::PushConst(_)
                    | Instruction::PushTime
                    | Instruction::PushFreq
                    | Instruction::PushTemperature
                    | Instruction::PushThermalVoltage
                    | Instruction::PushGmin
                    | Instruction::Add
                    | Instruction::Sub
                    | Instruction::Mul
                    | Instruction::Div
                    | Instruction::Pow
                    | Instruction::FunctionPow
                    | Instruction::Pwr
                    | Instruction::Pwrs
                    | Instruction::Neg
                    | Instruction::Sin
                    | Instruction::Cos
                    | Instruction::Exp
                    | Instruction::Sqr
            )
        }) {
            return None;
        }
        Some(Self {
            program,
            stack: Vec::with_capacity(32),
            stop,
        })
    }

    pub fn evaluate(&mut self, time: TimeInterval, context: &Context<'_>) -> Option<TimeBounds> {
        self.stack.clear();
        for instruction in &self.program.instructions {
            let value = match instruction {
                Instruction::PushConst(value) => Dual::constant(*value),
                Instruction::PushTime => Dual {
                    value: time,
                    slope: TimeInterval::point(self.stop),
                    constant: false,
                    continuous: true,
                },
                Instruction::PushFreq => Dual::constant(context.frequency),
                Instruction::PushTemperature => Dual::constant(context.temperature),
                Instruction::PushThermalVoltage => {
                    Dual::constant(crate::constants::thermal_voltage(
                        crate::constants::celsius_to_kelvin(context.temperature),
                    ))
                }
                Instruction::PushGmin => Dual::constant(context.gmin),
                Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    match instruction {
                        Instruction::Add => left.add(right),
                        Instruction::Sub => left.add(right.neg()),
                        Instruction::Mul => left.mul(right),
                        Instruction::Div => left.div(right)?,
                        _ => return None,
                    }
                }
                Instruction::Pow
                | Instruction::FunctionPow
                | Instruction::Pwr
                | Instruction::Pwrs => {
                    let exponent = self.stack.pop()?;
                    let base = self.stack.pop()?;
                    base.power(exponent, instruction, context)?
                }
                Instruction::Neg => self.stack.pop()?.neg(),
                Instruction::Sin => self.stack.pop()?.trigonometric(false),
                Instruction::Cos => self.stack.pop()?.trigonometric(true),
                Instruction::Exp => self.stack.pop()?.exponential(),
                Instruction::Sqr => self.stack.pop()?.square(),
                _ => return None,
            };
            if value.value.lower.is_nan() || value.value.upper.is_nan() {
                return None;
            }
            self.stack.push(value);
        }
        (self.stack.len() == 1)
            .then(|| {
                self.stack.first().map(|value| TimeBounds {
                    value: value.value,
                    slope: value.slope,
                    continuous: value.continuous,
                })
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Vm, compile, parse_expression_strict};

    #[test]
    fn combined_clock_bounds_contain_vm_values_and_analytic_derivatives() {
        for stop in [1e-300, 1e-30, 1.0, 1e30, 1e300] {
            let expression = parse_expression_strict(&format!(
                "cos(6*pi*(time/{stop:e})+0.1)+0.5*cos(12*pi*(time/{stop:e})+0.2)-0.25"
            ))
            .unwrap();
            let program = compile(&expression);
            let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
            let mut vm = Vm::new();
            let context = Context::transient(&[], &[], 0.0);
            let rate = 6.0 * std::f64::consts::PI;
            for interval in 0..64 {
                let left = interval as Value / 64.0 * stop;
                let right = (interval + 1) as Value / 64.0 * stop;
                let TimeBounds { value, slope, .. } = bounds
                    .evaluate(
                        TimeInterval {
                            lower: left,
                            upper: right,
                        },
                        &context,
                    )
                    .unwrap();
                for index in 0..=8 {
                    let time = left + (right - left) * (index as Value / 8.0);
                    let actual = vm.execute(&program, &Context { time, ..context });
                    let phase = rate * (time / stop) + 0.1;
                    let derivative = -rate * (phase.sin() + (2.0 * phase).sin());
                    assert!(
                        value.contains(actual),
                        "stop={stop:e}, t={time:e}, value {actual:e} outside {value:?}"
                    );
                    assert!(
                        slope.contains(derivative),
                        "stop={stop:e}, t={time:e}, slope {derivative:e} outside {slope:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn constant_bounds_follow_the_vm_and_singular_or_stateful_bounds_are_unavailable() {
        let expression = parse_expression_strict("sin(0.3)+2/7+temper").unwrap();
        let program = compile(&expression);
        let context = Context::transient(&[], &[], 0.0).with_temperature(-10.0);
        let TimeBounds { value, slope, .. } = TimeEnclosure::new(&program, 1.0)
            .unwrap()
            .evaluate(
                TimeInterval {
                    lower: 0.0,
                    upper: 1.0,
                },
                &context,
            )
            .unwrap();
        let actual = Vm::new().execute(&program, &context);
        assert_eq!(
            (value.lower, value.upper, slope.lower, slope.upper),
            (actual, actual, 0.0, 0.0)
        );
        for expression in [
            "v(out)+cos(time)",
            "sdt(time)",
            "spice_pulse(0,1,0,1,1,1,4)",
            "1/(time-0.5)",
        ] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            assert!(
                TimeEnclosure::new(&program, 1.0)
                    .and_then(|mut bounds| bounds.evaluate(
                        TimeInterval {
                            lower: 0.0,
                            upper: 1.0
                        },
                        &context
                    ))
                    .is_none(),
                "{expression}"
            );
        }
        let program = compile(&parse_expression_strict("0*(1e308*time)").unwrap());
        let value = TimeEnclosure::new(&program, 2.0)
            .unwrap()
            .evaluate(
                TimeInterval {
                    lower: 0.0,
                    upper: 2.0,
                },
                &context,
            )
            .unwrap()
            .value;
        assert!(
            !value.is_finite(),
            "zero cannot conceal a potentially nonfinite intermediate"
        );
    }

    #[test]
    fn scaled_quotients_enclose_values_and_derivatives_without_reciprocal_overflow() {
        let stop = std::f64::consts::TAU;
        let context = Context::transient(&[], &[], 0.0);
        for scale in [-1e300, -1e-310, 1e-310, 1e-200, 1.0, 1e200, 1e300] {
            let expression = format!("({scale:e}*sin(time))/({scale:e}*(2+cos(time)))");
            let program = compile(&parse_expression_strict(&expression).unwrap());
            let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
            let mut vm = Vm::new();
            for index in 0..64 {
                let lower = index as Value / 64.0 * stop;
                let upper = (index + 1) as Value / 64.0 * stop;
                let TimeBounds { value, slope, .. } = bounds
                    .evaluate(TimeInterval { lower, upper }, &context)
                    .unwrap();
                assert!(
                    value.is_finite() && slope.is_finite(),
                    "{expression}: {value:?}, {slope:?}"
                );
                for sample in 0..=8 {
                    let time = lower + (upper - lower) * sample as Value / 8.0;
                    let actual = vm.execute(&program, &Context { time, ..context });
                    let derivative = stop * (2.0 * time.cos() + 1.0) / (2.0 + time.cos()).powi(2);
                    assert!(
                        value.contains(actual),
                        "{expression}, t={time}: {actual} outside {value:?}"
                    );
                    assert!(
                        slope.contains(derivative),
                        "{expression}, t={time}: {derivative} outside {slope:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn power_bounds_cover_dialect_values_and_independent_chain_rule_derivatives() {
        use crate::config::ExpressionDialect;
        type Derivative = fn(Value, ExpressionDialect) -> Value;
        let cases: [(&str, bool, Derivative); 14] = [
            ("(time-1)^3", false, |t, _| 3.0 * (t - 1.0).powi(2)),
            ("(time-1)^(0*time+3)", false, |t, _| 3.0 * (t - 1.0).powi(2)),
            ("(time+1)^(-3)", false, |t, _| -3.0 / (t + 1.0).powi(4)),
            ("(time+1)^0.5", false, |t, _| 0.5 / (t + 1.0).sqrt()),
            ("(1e-310*(time+1))^0.5", false, |t, _| {
                0.5 * 1e-310_f64.sqrt() / (t + 1.0).sqrt()
            }),
            ("pow(time-1,3)", false, |t, dialect| {
                if dialect == ExpressionDialect::Xyce {
                    3.0 * (t - 1.0).powi(2)
                } else {
                    3.0 * (t - 1.0) * (t - 1.0).abs()
                }
            }),
            ("pwr(time-1,2)", false, |t, dialect| {
                if dialect == ExpressionDialect::Xyce {
                    2.0 * (t - 1.0)
                } else {
                    2.0 * (t - 1.0).abs()
                }
            }),
            ("pwrs(time-1,2)", false, |t, _| 2.0 * (t - 1.0).abs()),
            ("(-time-1)^(-0.5)", true, |t, _| {
                -0.5 * std::f64::consts::FRAC_PI_2.cos() / (t + 1.0).powf(1.5)
            }),
            ("2^(sin(time)+2)", false, |t, _| {
                2.0_f64.powf(t.sin() + 2.0) * 2.0_f64.ln() * t.cos()
            }),
            ("(time+1)^(sin(time)+2)", false, |t, _| {
                (t + 1.0).powf(t.sin() + 2.0)
                    * (t.cos() * (t + 1.0).ln() + (t.sin() + 2.0) / (t + 1.0))
            }),
            ("(-time-1)^(sin(time)+2)", true, |t, _| {
                let p = t.sin() + 2.0;
                let (s, c) = (std::f64::consts::PI * p).sin_cos();
                (t + 1.0).powf(p)
                    * (c * (t.cos() * (t + 1.0).ln() + p / (t + 1.0))
                        - std::f64::consts::PI * s * t.cos())
            }),
            ("pow(-time-1,sin(time)+2)", false, |t, dialect| {
                let p = t.sin() + 2.0;
                let derivative = t.cos() * (t + 1.0).ln() + p / (t + 1.0);
                (t + 1.0).powf(p)
                    * if dialect == ExpressionDialect::Xyce {
                        let (s, c) = (std::f64::consts::PI * p).sin_cos();
                        c * derivative - std::f64::consts::PI * s * t.cos()
                    } else {
                        derivative
                    }
            }),
            ("pwrs(-time-1,sin(time)+2)", false, |t, _| {
                -(t + 1.0).powf(t.sin() + 2.0)
                    * (t.cos() * (t + 1.0).ln() + (t.sin() + 2.0) / (t + 1.0))
            }),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            let stop = 3.0;
            for (expression, xyce_only, derivative) in cases {
                if xyce_only && dialect != ExpressionDialect::Xyce {
                    continue;
                }
                let program = compile(&parse_expression_strict(expression).unwrap());
                let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                let mut vm = Vm::new();
                for index in 0..48 {
                    let lower = index as Value / 48.0 * stop;
                    let upper = (index + 1) as Value / 48.0 * stop;
                    let TimeBounds { value, slope, .. } = bounds
                        .evaluate(TimeInterval { lower, upper }, &context)
                        .unwrap();
                    assert!(value.is_finite(), "{expression} ({dialect:?}): {value:?}");
                    for sample in 0..=8 {
                        let time = lower + (upper - lower) * sample as Value / 8.0;
                        let actual = vm.execute(&program, &Context { time, ..context });
                        let derivative = stop * derivative(time, dialect);
                        assert!(
                            value.contains(actual),
                            "{expression} ({dialect:?}), t={time}: {actual} outside {value:?}"
                        );
                        assert!(
                            slope.contains(derivative),
                            "{expression} ({dialect:?}), t={time}: {derivative} outside {slope:?}"
                        );
                    }
                }
            }
            for expression in ["pow(-2,3)+pwr(-2,2)+pwrs(-2,2)", "(-2)^0.5", "(-2)^0"] {
                if dialect == ExpressionDialect::Ngspice && expression == "(-2)^0.5" {
                    continue;
                }
                let program = compile(&parse_expression_strict(expression).unwrap());
                let TimeBounds { value, slope, .. } = TimeEnclosure::new(&program, stop)
                    .unwrap()
                    .evaluate(
                        TimeInterval {
                            lower: 0.0,
                            upper: stop,
                        },
                        &context,
                    )
                    .unwrap();
                let actual = Vm::new().execute(&program, &context);
                assert_eq!(
                    (value.lower, value.upper, slope.lower, slope.upper),
                    (actual, actual, 0.0, 0.0)
                );
            }
            let program = compile(&parse_expression_strict("pwrs(time-1,0)").unwrap());
            let TimeBounds {
                value, continuous, ..
            } = TimeEnclosure::new(&program, stop)
                .unwrap()
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: stop,
                    },
                    &context,
                )
                .unwrap();
            assert!(value.contains(-1.0) && value.contains(1.0));
            assert!(
                !continuous,
                "a zero-exponent sign jump is not a smooth constant"
            );
            let program = compile(&parse_expression_strict("0^time").unwrap());
            let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
            let full = bounds
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: stop,
                    },
                    &context,
                )
                .unwrap();
            assert!(!full.continuous && full.value.contains(0.0) && full.value.contains(1.0));
            let positive = bounds
                .evaluate(
                    TimeInterval {
                        lower: 0.1,
                        upper: stop,
                    },
                    &context,
                )
                .unwrap();
            assert!(positive.continuous);
            assert_eq!((positive.value.lower, positive.value.upper), (0.0, 0.0));
        }
    }

    #[test]
    fn quotient_domain_uncertainty_can_resolve_on_smaller_intervals() {
        let program =
            compile(&parse_expression_strict("sin(time)/(sqr(sin(time))+sqr(cos(time)))").unwrap());
        let stop = std::f64::consts::TAU;
        let context = Context::transient(&[], &[], 0.0);
        let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
        assert!(
            bounds
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: stop
                    },
                    &context
                )
                .is_none()
        );
        for index in 0..32 {
            let lower = index as Value / 32.0 * stop;
            let upper = (index + 1) as Value / 32.0 * stop;
            let TimeBounds { value, slope, .. } = bounds
                .evaluate(TimeInterval { lower, upper }, &context)
                .unwrap();
            for sample in 0..=8 {
                let time = lower + (upper - lower) * (sample as Value / 8.0);
                assert!(value.contains(Vm::new().execute(&program, &Context { time, ..context })));
                assert!(slope.contains(stop * time.cos()));
            }
        }
        for expression in ["sin(time)/0", "sin(time)/(0*time)"] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let TimeBounds { value, slope, .. } = TimeEnclosure::new(&program, stop)
                .unwrap()
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: stop,
                    },
                    &context,
                )
                .unwrap();
            assert_eq!(
                (value.lower, value.upper, slope.lower, slope.upper),
                (0.0, 0.0, 0.0, 0.0)
            );
        }
    }

    #[test]
    fn dynamic_products_quotients_and_squares_enclose_their_derivatives() {
        let stop = std::f64::consts::TAU;
        let context = Context::transient(&[], &[], 0.0);
        for (expression, derivative) in [
            (
                "exp(cos(time))+0.5*exp(cos(2*time))",
                (|t: Value| -t.sin() * t.cos().exp() - (2.0 * t).sin() * (2.0 * t).cos().exp())
                    as fn(Value) -> Value,
            ),
            (
                "sin(time)*cos(time)",
                (|t: Value| (2.0 * t).cos()) as fn(Value) -> Value,
            ),
            (
                "sin(time)/(2+cos(time))",
                (|t: Value| (2.0 * t.cos() + 1.0) / (2.0 + t.cos()).powi(2)) as fn(Value) -> Value,
            ),
            (
                "sqr(sin(time))",
                (|t: Value| 2.0 * t.sin() * t.cos()) as fn(Value) -> Value,
            ),
        ] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
            let mut vm = Vm::new();
            for index in 0..32 {
                let left = index as Value / 32.0 * stop;
                let right = (index + 1) as Value / 32.0 * stop;
                let TimeBounds { value, slope, .. } = bounds
                    .evaluate(
                        TimeInterval {
                            lower: left,
                            upper: right,
                        },
                        &context,
                    )
                    .unwrap();
                for sample in 0..=8 {
                    let time = left + (right - left) * (sample as Value / 8.0);
                    let actual = vm.execute(&program, &Context { time, ..context });
                    assert!(
                        value.contains(actual),
                        "{expression}, t={time:e}: {actual:e} outside {value:?}"
                    );
                    assert!(
                        slope.contains(stop * derivative(time)),
                        "{expression}, t={time:e}: derivative outside {slope:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn exponential_bounds_distinguish_exact_underflow_from_small_nonzero_values() {
        let context = Context::transient(&[], &[], 0.0);
        for expression in ["exp(-1000+time)", "exp(-1000+pwrs(time-0.5,0))"] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let domain = TimeEnclosure::new(&program, 1.0)
                .unwrap()
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    &context,
                )
                .unwrap();
            assert_eq!((domain.value.lower, domain.value.upper), (0.0, 0.0));
            assert_eq!((domain.slope.lower, domain.slope.upper), (0.0, 0.0));
            assert!(domain.continuous);
        }
        for expression in ["exp(-746+2*time)", "exp(-708+time)", "exp(709+time)"] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let domain = TimeEnclosure::new(&program, 1.0)
                .unwrap()
                .evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    &context,
                )
                .unwrap();
            assert!(domain.value.upper > 0.0, "{expression}");
            for step in 0..=64 {
                let actual = Vm::new().execute(
                    &program,
                    &Context {
                        time: step as Value / 64.0,
                        ..context
                    },
                );
                assert!(
                    domain.value.contains(actual),
                    "{expression}: {actual:e} outside {:?}",
                    domain.value
                );
            }
        }
        let program = compile(&parse_expression_strict("exp(0*(1e308*time))").unwrap());
        let domain = TimeEnclosure::new(&program, 2.0)
            .unwrap()
            .evaluate(
                TimeInterval {
                    lower: 0.0,
                    upper: 2.0,
                },
                &context,
            )
            .unwrap();
        assert!(
            !domain.value.is_finite(),
            "a nonfinite input cannot establish a zero plateau"
        );
    }
}
