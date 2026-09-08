//! Continuous time-expression bounds for event isolation, below device code.
//!
//! Point values still come from the ordinary VM. This interpreter propagates
//! value and derivative intervals through that same compiled program. Its
//! derivative is with respect to normalized analysis time, avoiding an
//! unnecessary reciprocal of extremely small or large physical periods.

use super::{CompiledExpr, Context, Instruction, LOGARITHM_MIN_ARGUMENT};
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

    fn magnitude(self) -> Value {
        self.lower.abs().max(self.upper.abs())
    }

    fn rounding_error(self, transcendental: bool) -> Value {
        let magnitude = self.magnitude();
        if !magnitude.is_finite() {
            return Value::INFINITY;
        }
        // One outward ULP also covers gradual underflow. Transcendentals
        // use the same libm allowance as their endpoint enclosures.
        ((magnitude.next_up() - magnitude)
            + if transcendental {
                16.0 * Value::EPSILON * magnitude
            } else {
                0.0
            })
        .next_up()
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
        // FastTwoSum identifies the direction of the rounding error. Keep
        // exact endpoints exact: widening 2 + [-1,1] below 1 would falsely
        // put an acosh argument outside its real domain at every extremum.
        let directed = |a: Value, b: Value, upper: bool| {
            let sum = a + b;
            let error = if a.abs() >= b.abs() {
                (a - sum) + b
            } else {
                (b - sum) + a
            };
            if upper && (error > 0.0 || !sum.is_finite()) {
                sum.next_up()
            } else if !upper && (error < 0.0 || !sum.is_finite()) {
                sum.next_down()
            } else {
                sum
            }
        };
        let lower = directed(self.lower, other.lower, false);
        let upper = directed(self.upper, other.upper, true);
        if lower.is_nan() || upper.is_nan() {
            Self::WHOLE
        } else {
            Self { lower, upper }
        }
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
        .with_product_sign(self, other)
    }

    fn with_product_sign(mut self, left: Self, right: Self) -> Self {
        if (left.lower >= 0.0 && right.lower >= 0.0) || (left.upper <= 0.0 && right.upper <= 0.0) {
            self.lower = self.lower.max(0.0);
        }
        if (left.lower >= 0.0 && right.upper <= 0.0) || (left.upper <= 0.0 && right.lower >= 0.0) {
            self.upper = self.upper.min(0.0);
        }
        self
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
        Some(
            Self::outward(
                quotients.into_iter().fold(Value::INFINITY, Value::min),
                quotients.into_iter().fold(Value::NEG_INFINITY, Value::max),
            )
            .with_product_sign(self, other),
        )
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
            let mut value = Self::outward(minimum * minimum, maximum * maximum);
            value.lower = value.lower.max(0.0);
            value
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
    center: Value,
    constant: bool,
    continuous: bool,
    roundoff: Value,
}

/// Bounds on one evaluation domain. Continuity is separate from a finite
/// derivative: a continuous cusp may have an unbounded derivative, whereas
/// a signed zero power has a finite-valued jump.
pub(crate) struct TimeBounds {
    pub value: TimeInterval,
    pub slope: TimeInterval,
    pub continuous: bool,
    roundoff: Value,
}

impl TimeBounds {
    /// Maximum deviation from the line through the VM endpoint values.
    /// If a continuous derivative is in [m,M], the secant error is at most
    /// h*(M-m)/4. The two roundoff terms cover the waveform and interpolated
    /// endpoints. The value diameter is independently valid even at a jump.
    pub fn interpolation_error(&self, normalized_width: Value) -> Value {
        let diameter = (self.value.upper - self.value.lower).next_up();
        if !self.value.is_finite() {
            return Value::INFINITY;
        }
        if self.value.lower == self.value.upper {
            return 0.0;
        }
        if !self.continuous || !self.slope.is_finite() {
            return diameter;
        }
        // Quarter the endpoints before subtraction to avoid width overflow.
        let quarter_width = (0.25 * self.slope.upper - 0.25 * self.slope.lower).next_up();
        let smooth = (normalized_width.next_up() * quarter_width).next_up();
        diameter.min((smooth + 2.0 * self.roundoff).next_up())
    }
}

fn propagated_error(sensitivity: Value, error: Value) -> Value {
    if error == 0.0 || sensitivity == 0.0 {
        0.0
    } else {
        (sensitivity * error).next_up()
    }
}

impl Dual {
    fn constant(value: Value) -> Self {
        Self {
            value: TimeInterval::point(value),
            slope: TimeInterval::ZERO,
            center: value,
            constant: true,
            continuous: true,
            roundoff: 0.0,
        }
    }

    fn add(self, other: Self) -> Self {
        if self.constant && other.constant {
            return Self::constant(self.value.lower + other.value.lower);
        }
        let value = self.value.add(other.value);
        Self {
            value,
            slope: self.slope.add(other.slope),
            center: self.center + other.center,
            constant: false,
            continuous: self.continuous && other.continuous,
            roundoff: (self.roundoff + other.roundoff + value.rounding_error(false)).next_up(),
        }
    }

    fn neg(self) -> Self {
        Self {
            value: self.value.neg(),
            slope: self.slope.neg(),
            center: -self.center,
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
        let value = self.value.mul(other.value);
        Self {
            value,
            slope: self.slope.mul(other.value).add(self.value.mul(other.slope)),
            center: self.center * other.center,
            constant: false,
            continuous: self.continuous && other.continuous,
            roundoff: (propagated_error(other.value.magnitude(), self.roundoff)
                + propagated_error(self.value.magnitude(), other.roundoff)
                + value.rounding_error(false))
            .next_up(),
        }
    }

    fn square(self) -> Self {
        if self.constant {
            return Self::constant(self.value.lower * self.value.lower);
        }
        let value = self.value.square();
        Self {
            value,
            slope: TimeInterval::point(2.0).mul(self.value).mul(self.slope),
            center: self.center * self.center,
            constant: false,
            continuous: self.continuous,
            roundoff: (propagated_error(2.0 * self.value.magnitude(), self.roundoff)
                + value.rounding_error(false))
            .next_up(),
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
        let denominator = other.value.lower.abs().min(other.value.upper.abs());
        Some(Self {
            value,
            center: self.center / other.center,
            // u'/v - (u/v)*(v'/v) avoids both v^2 underflow and reciprocal
            // overflow. Each quotient is independently enclosed.
            slope: self
                .slope
                .div(other.value)?
                .add(value.mul(other.slope.div(other.value)?).neg()),
            constant: false,
            continuous: self.continuous && other.continuous,
            roundoff: ((self.roundoff / denominator).next_up()
                + propagated_error(value.magnitude(), (other.roundoff / denominator).next_up())
                + value.rounding_error(false))
            .next_up(),
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
        let mut roundoff = value.rounding_error(true);
        if !self.constant {
            let partial = exponent.value.mul(
                self.value
                    .positive_power(exponent.value.add(TimeInterval::point(-1.0))),
            );
            slope = partial.mul(self.slope);
            roundoff += propagated_error(partial.magnitude(), self.roundoff);
        }
        if !exponent.constant {
            let partial = value.mul(self.value.logarithm());
            slope = slope.add(partial.mul(exponent.slope));
            roundoff += propagated_error(partial.magnitude(), exponent.roundoff);
        }
        Self {
            value,
            slope,
            center: self.center.powf(exponent.center),
            constant: false,
            roundoff: roundoff.next_up(),
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
                    center: evaluate(self.center, exponent.center),
                    continuous: positive.continuous
                        && powered.continuous
                        && !(exponent.constant
                            && exponent.value.lower == 0.0
                            && positive.value.lower != powered.value.upper),
                    constant: false,
                    roundoff: positive.roundoff.max(powered.roundoff),
                },
            });
        }
        result.map(|power| Self {
            center: evaluate(self.center, exponent.center),
            ..power
        })
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
        let value = self.value.trigonometric(cosine);
        Self {
            value,
            slope: self
                .slope
                .mul(if cosine { derivative.neg() } else { derivative }),
            center: if cosine {
                self.center.cos()
            } else {
                self.center.sin()
            },
            constant: false,
            continuous: self.continuous,
            roundoff: (propagated_error(derivative.magnitude(), self.roundoff)
                + value.rounding_error(true))
            .next_up(),
        }
    }

    fn logarithm(self, base10: bool) -> Self {
        let evaluate = |argument: Value| {
            let argument = argument.max(LOGARITHM_MIN_ARGUMENT);
            if base10 {
                argument.log10()
            } else {
                argument.ln()
            }
        };
        if self.constant || self.value.upper <= LOGARITHM_MIN_ARGUMENT {
            // The input floor is an exact VM plateau, even across an input
            // jump. It must not retain the hidden input's slope or roundoff.
            return Self::constant(evaluate(self.value.lower));
        }
        let argument = TimeInterval {
            lower: self.value.lower.max(LOGARITHM_MIN_ARGUMENT),
            upper: self.value.upper,
        };
        let value =
            TimeInterval::transcendental(evaluate(argument.lower), evaluate(argument.upper));
        // Divide directly before changing logarithm base. A denominator
        // square or the reciprocal of physical analysis time is unnecessary.
        let mut slope = self.slope.div(argument).unwrap_or(TimeInterval::WHOLE);
        let mut roundoff = (self.roundoff / argument.lower).next_up();
        if base10 {
            let factor =
                TimeInterval::transcendental(std::f64::consts::LOG10_E, std::f64::consts::LOG10_E);
            slope = slope.mul(factor);
            roundoff = propagated_error(factor.upper, roundoff);
        }
        if self.value.lower <= LOGARITHM_MIN_ARGUMENT {
            // The floor transition is continuous with a derivative corner.
            slope = slope.union(TimeInterval::ZERO);
        }
        Self {
            value,
            slope,
            center: evaluate(self.center),
            constant: false,
            continuous: self.continuous,
            roundoff: (roundoff + value.rounding_error(true)).next_up(),
        }
    }

    fn hyperbolic_or_atan(
        mut self,
        instruction: &Instruction,
        context: &Context<'_>,
    ) -> Option<Self> {
        use crate::config::ExpressionDialect;
        let xyce = context.expression_dialect == ExpressionDialect::Xyce;
        let evaluate: fn(Value) -> Value = match instruction {
            Instruction::Atan => Value::atan,
            Instruction::Sinh => Value::sinh,
            Instruction::Cosh => Value::cosh,
            Instruction::Tanh if xyce => super::vm::xyce_tanh,
            Instruction::Tanh => Value::tanh,
            Instruction::Asinh => Value::asinh,
            Instruction::Acosh => Value::acosh,
            Instruction::Atanh if xyce => super::vm::xyce_atanh,
            Instruction::Atanh => Value::atanh,
            _ => return None,
        };
        if self.constant {
            return Some(Self::constant(evaluate(self.value.lower)));
        }
        // The VM clamp propagates NaN, whereas min/max select a finite
        // operand. An unbounded input enclosure can include invalid values
        // such as 0*infinity and must not become a finite saturation proof.
        if !self.value.is_finite() {
            return None;
        }
        if matches!(instruction, Instruction::Atanh) && xyce {
            let limit = 1.0 - super::vm::XYCE_ATANH_EPSILON;
            self = self
                .extremum(Self::constant(-limit), true)
                .extremum(Self::constant(limit), false);
            if self.constant {
                return Some(Self::constant(evaluate(self.value.lower)));
            }
        }
        if matches!(instruction, Instruction::Tanh) && xyce {
            let threshold = super::vm::XYCE_TANH_SATURATION_THRESHOLD;
            if self.value.lower > threshold || self.value.upper < -threshold {
                return Some(Self::constant(evaluate(self.value.lower)));
            }
        }
        // Undefined real domains remain unresolved. In particular, native
        // atanh must not inherit the Xyce argument clamp.
        if (matches!(instruction, Instruction::Acosh) && self.value.lower < 1.0)
            || (matches!(instruction, Instruction::Atanh)
                && (self.value.lower <= -1.0 || self.value.upper >= 1.0))
        {
            return None;
        }
        let nearest = if self.value.contains(0.0) {
            0.0
        } else {
            self.value.lower.abs().min(self.value.upper.abs())
        };
        let farthest = self.value.magnitude();
        let positive_range = |lower: Value, upper: Value| {
            let mut result = TimeInterval::transcendental(lower, upper);
            result.lower = result.lower.max(0.0);
            result
        };
        let mut value = if matches!(instruction, Instruction::Cosh) {
            let mut value = positive_range(nearest.cosh(), farthest.cosh());
            value.lower = value.lower.max(1.0);
            value
        } else {
            TimeInterval::transcendental(evaluate(self.value.lower), evaluate(self.value.upper))
        };
        if matches!(instruction, Instruction::Acosh) {
            value.lower = value.lower.max(0.0);
        } else if matches!(instruction, Instruction::Tanh) {
            value.lower = value.lower.max(-1.0);
            value.upper = value.upper.min(1.0);
        }
        // Apply reciprocal derivatives to the incoming slope by successive
        // divisions. Squaring a large argument or forming a tiny derivative
        // first can lose a representable normalized-time derivative.
        let chain = |incoming: TimeInterval| -> Option<TimeInterval> {
            Some(match instruction {
                Instruction::Atan | Instruction::Asinh => {
                    let norm = positive_range(nearest.hypot(1.0), farthest.hypot(1.0));
                    let first = incoming.div(norm)?;
                    if matches!(instruction, Instruction::Atan) {
                        first.div(norm)?
                    } else {
                        first
                    }
                }
                Instruction::Sinh => incoming.mul(positive_range(nearest.cosh(), farthest.cosh())),
                Instruction::Cosh => incoming.mul(TimeInterval::transcendental(
                    self.value.lower.sinh(),
                    self.value.upper.sinh(),
                )),
                Instruction::Tanh => {
                    // sech(x) = 2 exp(-|x|)/(1+exp(-2|x|)); this remains
                    // useful after cosh(x) would overflow.
                    let exponential = positive_range((-farthest).exp(), (-nearest).exp());
                    let denominator = TimeInterval::point(1.0).add(exponential.square());
                    incoming
                        .mul(exponential)
                        .div(denominator)?
                        .mul(exponential)
                        .div(denominator)?
                        .mul(TimeInterval::point(4.0))
                }
                Instruction::Acosh => {
                    let lower = self.value.lower;
                    let upper = self.value.upper;
                    incoming
                        .div(positive_range((lower - 1.0).sqrt(), (upper - 1.0).sqrt()))?
                        .div(positive_range((lower + 1.0).sqrt(), (upper + 1.0).sqrt()))?
                }
                Instruction::Atanh => incoming
                    .div(TimeInterval::outward(1.0 - farthest, 1.0 - nearest))?
                    .div(TimeInterval::outward(1.0 + nearest, 1.0 + farthest))?,
                _ => return None,
            })
        };
        let mut slope = chain(self.slope).unwrap_or(TimeInterval::WHOLE);
        if matches!(instruction, Instruction::Tanh)
            && xyce
            && farthest > super::vm::XYCE_TANH_SATURATION_THRESHOLD
        {
            slope = slope.union(TimeInterval::ZERO);
        }
        let sensitivity = chain(TimeInterval::point(1.0))
            .unwrap_or(TimeInterval::WHOLE)
            .magnitude();
        let mut roundoff = propagated_error(sensitivity, self.roundoff);
        if matches!(instruction, Instruction::Acosh) {
            // acosh is 1/2-Holder at one: its maximum change over delta is
            // at most sqrt(2*delta), also when the derivative is unbounded.
            let holder = (std::f64::consts::SQRT_2 * self.roundoff.sqrt()).next_up();
            roundoff = roundoff.min(holder);
        }
        Some(Self {
            value,
            slope,
            center: evaluate(self.center),
            constant: false,
            continuous: self.continuous,
            roundoff: (roundoff + value.rounding_error(true)).next_up(),
        })
    }

    fn absolute(self) -> Self {
        if self.constant {
            return Self::constant(self.value.lower.abs());
        }
        if self.value.lower >= 0.0 {
            return self;
        }
        if self.value.upper <= 0.0 {
            return self.neg();
        }
        Self {
            value: TimeInterval {
                lower: 0.0,
                upper: self.value.magnitude(),
            },
            slope: self.slope.union(self.slope.neg()),
            center: self.center.abs(),
            // Absolute value is exact on finite VM values and is globally
            // 1-Lipschitz. Its corner needs no extra rounding allowance.
            ..self
        }
    }

    fn extremum(self, other: Self, maximum: bool) -> Self {
        let select = |left: Value, right: Value| {
            if maximum {
                left.max(right)
            } else {
                left.min(right)
            }
        };
        let center = select(self.center, other.center);
        if self.constant && other.constant {
            return Self::constant(select(self.value.lower, other.value.lower));
        }
        let left_wins = if maximum {
            self.value.lower >= other.value.upper
        } else {
            self.value.upper <= other.value.lower
        };
        let right_wins = if maximum {
            other.value.lower >= self.value.upper
        } else {
            other.value.upper <= self.value.lower
        };
        if left_wins {
            return Self { center, ..self };
        }
        if right_wins {
            return Self { center, ..other };
        }
        Self {
            value: TimeInterval {
                lower: select(self.value.lower, other.value.lower),
                upper: select(self.value.upper, other.value.upper),
            },
            slope: self.slope.union(other.slope),
            center,
            constant: false,
            continuous: self.continuous && other.continuous,
            // Selection is exact and 1-Lipschitz in the maximum input error.
            // An inactive operand contributes neither slope nor error above.
            roundoff: self.roundoff.max(other.roundoff),
        }
    }

    fn square_root(self) -> Self {
        if self.constant || self.value.upper <= 0.0 {
            return Self::constant(self.value.lower.max(0.0).sqrt());
        }
        let mut value =
            TimeInterval::transcendental(self.value.lower.max(0.0).sqrt(), self.value.upper.sqrt());
        value.lower = value.lower.max(0.0);
        // The square root is continuous at the VM's zero floor, with an
        // unbounded derivative. Value bounds still qualify a small cusp.
        let slope = self
            .slope
            .mul(TimeInterval::point(0.5))
            .div(value)
            .unwrap_or(TimeInterval::WHOLE);
        // |sqrt(max(x,0))-sqrt(max(y,0))| <= sqrt(|x-y|), including
        // across zero. This avoids an infinite roundoff estimate at a cusp.
        let mut roundoff =
            TimeInterval::transcendental(self.roundoff.sqrt(), self.roundoff.sqrt()).upper;
        if value.lower > 0.0 {
            roundoff = roundoff.min(((0.5 * self.roundoff).next_up() / value.lower).next_up());
        }
        Self {
            value,
            slope,
            center: self.center.max(0.0).sqrt(),
            constant: false,
            continuous: self.continuous,
            roundoff: (roundoff + value.rounding_error(true)).next_up(),
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
            center: self.center.exp(),
            constant: false,
            continuous: self.continuous,
            roundoff: (propagated_error(value.magnitude(), self.roundoff)
                + value.rounding_error(true))
            .next_up(),
        }
    }

    fn tighten_centered(&mut self, radius: Value) {
        if !self.constant && self.continuous && self.center.is_finite() && self.slope.is_finite() {
            // The midpoint VM value and every other VM value differ from
            // their continuous counterparts by at most the propagated error.
            // Intersect the ordinary range with a centered mean-value bound
            // before a following operation can amplify interval dependency.
            let error = (2.0 * self.roundoff).next_up();
            let centered = TimeInterval::point(self.center)
                .add(self.slope.mul(TimeInterval {
                    lower: -radius,
                    upper: radius,
                }))
                .add(TimeInterval {
                    lower: -error,
                    upper: error,
                });
            let lower = self.value.lower.max(centered.lower);
            let upper = self.value.upper.min(centered.upper);
            if lower <= upper {
                self.value = TimeInterval { lower, upper };
            }
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
                    | Instruction::Dup
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
                    | Instruction::Abs
                    | Instruction::Sqrt
                    | Instruction::Sin
                    | Instruction::Cos
                    | Instruction::Atan
                    | Instruction::Sinh
                    | Instruction::Cosh
                    | Instruction::Tanh
                    | Instruction::Asinh
                    | Instruction::Acosh
                    | Instruction::Atanh
                    | Instruction::Exp
                    | Instruction::Ln
                    | Instruction::Log10
                    | Instruction::Log
                    | Instruction::Sqr
                    | Instruction::Min(_)
                    | Instruction::Max(_)
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
        self.evaluate_internal(time, context, false)
    }

    pub fn evaluate_centered(
        &mut self,
        time: TimeInterval,
        context: &Context<'_>,
    ) -> Option<TimeBounds> {
        self.evaluate_internal(time, context, true)
    }

    fn evaluate_internal(
        &mut self,
        time: TimeInterval,
        context: &Context<'_>,
        centered: bool,
    ) -> Option<TimeBounds> {
        let center = time.lower + 0.5 * (time.upper - time.lower);
        let radius =
            (((center - time.lower).abs().max((time.upper - center).abs())) / self.stop).next_up();
        self.stack.clear();
        for instruction in &self.program.instructions {
            let mut value = match instruction {
                Instruction::PushConst(value) => Dual::constant(*value),
                Instruction::Dup => *self.stack.last()?,
                Instruction::PushTime => Dual {
                    value: time,
                    slope: TimeInterval::point(self.stop),
                    center,
                    constant: false,
                    continuous: true,
                    roundoff: 0.0,
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
                Instruction::Abs => self.stack.pop()?.absolute(),
                Instruction::Sqrt => self.stack.pop()?.square_root(),
                Instruction::Sin => self.stack.pop()?.trigonometric(false),
                Instruction::Cos => self.stack.pop()?.trigonometric(true),
                Instruction::Atan
                | Instruction::Sinh
                | Instruction::Cosh
                | Instruction::Tanh
                | Instruction::Asinh
                | Instruction::Acosh
                | Instruction::Atanh => {
                    self.stack.pop()?.hyperbolic_or_atan(instruction, context)?
                }
                Instruction::Exp => self.stack.pop()?.exponential(),
                Instruction::Ln => self.stack.pop()?.logarithm(false),
                Instruction::Log10 => self.stack.pop()?.logarithm(true),
                Instruction::Log => self.stack.pop()?.logarithm(
                    context.expression_dialect == crate::config::ExpressionDialect::Xyce,
                ),
                Instruction::Sqr => self.stack.pop()?.square(),
                Instruction::Min(count) | Instruction::Max(count) => {
                    let start = self.stack.len().checked_sub(*count)?;
                    let mut values = self.stack.drain(start..);
                    let first = values.next()?;
                    values.fold(first, |left, right| {
                        left.extremum(right, matches!(instruction, Instruction::Max(_)))
                    })
                }
                _ => return None,
            };
            if value.value.lower.is_nan() || value.value.upper.is_nan() {
                return None;
            }
            if centered {
                value.tighten_centered(radius);
            }
            self.stack.push(value);
        }
        (self.stack.len() == 1)
            .then(|| {
                self.stack.first().map(|value| TimeBounds {
                    value: value.value,
                    slope: value.slope,
                    continuous: value.continuous,
                    roundoff: value.roundoff,
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

    #[test]
    fn logarithm_bounds_match_vm_floors_dialects_and_normalized_derivatives() {
        use crate::config::ExpressionDialect;

        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context {
                expression_dialect: dialect,
                ..Context::transient(&[], &[], 0.0)
            };
            for function in ["ln", "log10", "log"] {
                let base10 = function == "log10"
                    || (function == "log" && dialect == ExpressionDialect::Xyce);
                for stop in [1e-300, 1.0, 1e300] {
                    for scale in [1e-310, 1e-38, 1.0, 1e300] {
                        // Cross the VM's input floor, including negative values.
                        let expression = format!("{function}({scale:e}*(4*(time/{stop:e})-2))");
                        let program = compile(&parse_expression_strict(&expression).unwrap());
                        let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                        let mut vm = Vm::new();
                        for interval in 0..16 {
                            let lower = stop * (interval as Value / 16.0);
                            let upper = stop * ((interval + 1) as Value / 16.0);
                            let domain = bounds
                                .evaluate(TimeInterval { lower, upper }, &context)
                                .unwrap();
                            assert!(domain.continuous);
                            let error = domain.interpolation_error((upper - lower) / stop);
                            let left = vm.execute(
                                &program,
                                &Context {
                                    time: lower,
                                    ..context
                                },
                            );
                            let right = vm.execute(
                                &program,
                                &Context {
                                    time: upper,
                                    ..context
                                },
                            );
                            for sample in 0..=32 {
                                let fraction = sample as Value / 32.0;
                                let time = lower + fraction * (upper - lower);
                                let actual = vm.execute(&program, &Context { time, ..context });
                                let argument = scale * (4.0 * (time / stop) - 2.0);
                                let derivative = if argument <= 1e-38 {
                                    0.0
                                } else {
                                    4.0 * (scale / argument)
                                        / if base10 { std::f64::consts::LN_10 } else { 1.0 }
                                };
                                assert!(domain.value.contains(actual), "{expression}: {actual:e}");
                                assert!(
                                    domain.slope.contains(derivative),
                                    "{expression}: {derivative:e} outside {:?}",
                                    domain.slope
                                );
                                assert!(
                                    (actual - (left + fraction * (right - left))).abs() <= error,
                                    "{expression}: secant error exceeds {error:e}"
                                );
                            }
                        }
                    }
                }
                let program = compile(
                    &parse_expression_strict(&format!("{function}(-2+pwrs(time-0.5,0))")).unwrap(),
                );
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
                let actual = Vm::new().execute(&program, &context);
                assert_eq!((domain.value.lower, domain.value.upper), (actual, actual));
                assert_eq!((domain.slope.lower, domain.slope.upper), (0.0, 0.0));
                assert!(
                    domain.continuous,
                    "the floor hides the internal jump exactly"
                );
                assert_eq!(domain.interpolation_error(1.0), 0.0);
            }
        }
    }

    #[test]
    fn square_root_and_absolute_bounds_preserve_clamps_cusps_and_vm_roundoff() {
        let context = Context::transient(&[], &[], 0.0);
        for function in ["sqrt", "abs"] {
            for stop in [1e-300, 1.0, 1e300] {
                for scale in [-1e300, -1.0, -1e-310, 1e-310, 1.0, 1e300] {
                    let expression = format!("{function}({scale:e}*(4*(time/{stop:e})-2))");
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                    let mut vm = Vm::new();
                    for interval in 0..16 {
                        // Overlap windows so a cusp also lies inside a window.
                        let lower = stop * (interval as Value / 17.0);
                        let upper = stop * ((interval + 2) as Value / 17.0);
                        let domain = bounds
                            .evaluate(TimeInterval { lower, upper }, &context)
                            .unwrap();
                        assert!(domain.continuous);
                        let error = domain.interpolation_error((upper - lower) / stop);
                        let left = vm.execute(
                            &program,
                            &Context {
                                time: lower,
                                ..context
                            },
                        );
                        let right = vm.execute(
                            &program,
                            &Context {
                                time: upper,
                                ..context
                            },
                        );
                        for sample in 0..=32 {
                            let fraction = sample as Value / 32.0;
                            let time = lower + fraction * (upper - lower);
                            let actual = vm.execute(&program, &Context { time, ..context });
                            let argument = scale * (4.0 * (time / stop) - 2.0);
                            let derivative = if function == "abs" {
                                if argument == 0.0 {
                                    0.0
                                } else {
                                    argument.signum() * (4.0 * scale)
                                }
                            } else if argument > 0.0 {
                                (2.0 * scale) / argument.sqrt()
                            } else {
                                0.0
                            };
                            assert!(domain.value.contains(actual), "{expression}: {actual:e}");
                            assert!(
                                domain.slope.contains(derivative),
                                "{expression}: {derivative:e} outside {:?}",
                                domain.slope
                            );
                            assert!(
                                (actual - (left + fraction * (right - left))).abs() <= error,
                                "{expression}: secant error exceeds {error:e}"
                            );
                        }
                    }
                }
            }
        }
        let program = compile(&parse_expression_strict("sqrt(-2+pwrs(time-0.5,0))").unwrap());
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
        assert!(domain.continuous);
        assert_eq!(domain.interpolation_error(1.0), 0.0);
    }

    #[test]
    fn nary_extrema_enclose_active_slopes_corners_and_hidden_branches() {
        let context = Context::transient(&[], &[], 0.0);
        for maximum in [false, true] {
            let function = if maximum { "max" } else { "min" };
            for stop in [1e-300, 1.0, 1e300] {
                for gain in [-1e200, -1e-200, 1e-200, 1e200] {
                    let expression = format!(
                        "{function}({gain:e}*(time/{stop:e}),{gain:e}*(1-time/{stop:e}),{gain:e}*0.3)"
                    );
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                    let mut vm = Vm::new();
                    for interval in 0..16 {
                        let lower = stop * (interval as Value / 17.0);
                        let upper = stop * ((interval + 2) as Value / 17.0);
                        for centered in [false, true] {
                            let domain = bounds
                                .evaluate_internal(
                                    TimeInterval { lower, upper },
                                    &context,
                                    centered,
                                )
                                .unwrap();
                            let error = domain.interpolation_error((upper - lower) / stop);
                            assert!(domain.continuous);
                            let left = vm.execute(
                                &program,
                                &Context {
                                    time: lower,
                                    ..context
                                },
                            );
                            let right = vm.execute(
                                &program,
                                &Context {
                                    time: upper,
                                    ..context
                                },
                            );
                            for sample in 0..=32 {
                                let fraction = sample as Value / 32.0;
                                let time = lower + fraction * (upper - lower);
                                let actual = vm.execute(&program, &Context { time, ..context });
                                let phase = time / stop;
                                let candidates = [
                                    (gain * phase, gain),
                                    (gain * (1.0 - phase), -gain),
                                    (gain * 0.3, 0.0),
                                ];
                                let chosen = candidates
                                    .iter()
                                    .copied()
                                    .reduce(|left, right| {
                                        if (maximum && right.0 > left.0)
                                            || (!maximum && right.0 < left.0)
                                        {
                                            right
                                        } else {
                                            left
                                        }
                                    })
                                    .unwrap();
                                assert!(domain.value.contains(actual), "{expression}: {actual:e}");
                                if candidates
                                    .iter()
                                    .filter(|candidate| candidate.0 == chosen.0)
                                    .count()
                                    == 1
                                {
                                    assert!(
                                        domain.slope.contains(chosen.1),
                                        "{expression}, t={time:e}: {:?} excludes {}",
                                        domain.slope,
                                        chosen.1
                                    );
                                }
                                assert!(
                                    (actual - (left + fraction * (right - left))).abs() <= error,
                                    "{expression}, t={time:e}: secant error exceeds {error:e}"
                                );
                            }
                        }
                    }
                }
            }
            let expression = if maximum {
                "max(pwrs(time-0.5,0),2,3)"
            } else {
                "min(pwrs(time-0.5,0),-2,-3)"
            };
            let program = compile(&parse_expression_strict(expression).unwrap());
            let domain = TimeEnclosure::new(&program, 1.0)
                .unwrap()
                .evaluate_centered(
                    TimeInterval {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    &context,
                )
                .unwrap();
            let expected = if maximum { 3.0 } else { -3.0 };
            assert_eq!(
                (domain.value.lower, domain.value.upper),
                (expected, expected)
            );
            assert_eq!((domain.slope.lower, domain.slope.upper), (0.0, 0.0));
            assert!(domain.continuous);
            assert_eq!(domain.interpolation_error(1.0), 0.0);
        }
    }

    #[test]
    fn centered_bounds_enclose_vm_values_and_secants_through_cancellation_and_branches() {
        use crate::config::ExpressionDialect;

        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context {
                expression_dialect: dialect,
                ..Context::transient(&[], &[], 0.0)
            };
            for stop in [1e-30, 1.0, 1e300] {
                for expression in [
                    "(1e16+T)-1e16",
                    "(1e-310*sin(T))/(1e-310*(2+cos(T)))",
                    "exp(-1000000*(abs(cos(2*pi*T+0.1))+0.5*abs(cos(4*pi*T+0.2))-0.75)^2)",
                    "exp(-1000000*(sqrt(2+cos(2*pi*T+0.1))+0.5*sqrt(2+cos(4*pi*T+0.2))-2.4)^2)",
                    "exp(-1000000*(ln(2+cos(2*pi*T+0.1))+0.5*log10(2+cos(4*pi*T+0.2))-0.7)^2)",
                    "sqrt(T-0.5)",
                    "sqrt(1e16+T)-sqrt(1e16)",
                    "abs(T-0.5)",
                    "log(1e-38*(4*T-2))",
                    "exp(pwrs(T-0.5,0))",
                    "pow(-1-T,0.2+T)",
                    "pwr(T-0.5,1.5)",
                    "pwrs(T-0.5,2)",
                    "sqr(T-0.5)",
                    "1e200*(cos(2*pi*T)+0.5*cos(4*pi*T))",
                    "sin(1e-200*(T-0.5))",
                    "atan(1e200*(T+1))",
                    "sinh(2*T-1)",
                    "cosh(2*T-1)",
                    "tanh(60*T-30)",
                    "asinh(1e200*(T+1))",
                    "acosh(1+T)",
                    "acosh(2+cos(2*pi*T))",
                    "atanh(0.8*(2*T-1))",
                ] {
                    let expression = expression.replace('T', &format!("(time/{stop:e})"));
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                    let mut vm = Vm::new();
                    for (start, width) in [
                        (0.0, 1.0),
                        (0.1, 0.05),
                        (0.15, 1e-5),
                        (0.5 - 1e-5, 2e-5),
                        (0.7, 1e-10),
                        (0.9, 0.1),
                    ] {
                        let lower = start * stop;
                        let upper = (start + width) * stop;
                        let domain = bounds
                            .evaluate_centered(TimeInterval { lower, upper }, &context)
                            .unwrap_or_else(|| panic!("{expression}, {lower:e}..{upper:e}"));
                        let error = domain.interpolation_error((upper - lower) / stop);
                        let left = vm.execute(
                            &program,
                            &Context {
                                time: lower,
                                ..context
                            },
                        );
                        let right = vm.execute(
                            &program,
                            &Context {
                                time: upper,
                                ..context
                            },
                        );
                        for sample in 0..=64 {
                            let fraction = sample as Value / 64.0;
                            let time = lower + fraction * (upper - lower);
                            let actual = vm.execute(&program, &Context { time, ..context });
                            assert!(
                                domain.value.contains(actual),
                                "{dialect:?}, {expression}, t={time:e}: {actual:e} outside {:?}",
                                domain.value
                            );
                            assert!(
                                (actual - (left + fraction * (right - left))).abs() <= error,
                                "{dialect:?}, {expression}, t={time:e}: secant error exceeds {error:e}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn secant_bounds_include_vm_roundoff_and_subnormal_quotient_quantization() {
        let context = Context::transient(&[], &[], 0.0);
        for (expression, lower, upper) in [
            ("(1e16+time)-1e16", 0.9, 1.1),
            (
                "(1e-310*sin(time))/(1e-310*(2+cos(time)))",
                0.5,
                0.5 + 1e-14,
            ),
            ("exp(-1000000*(time-0.5)^2)", 0.5003, 0.50031),
            ("pwrs(time-0.5,0)", 0.49, 0.51),
            ("sqrt(1e16+time)-sqrt(1e16)", 0.9, 1.1),
            ("sqrt(time-0.5)", 0.5_f64.next_down(), 0.5_f64.next_up()),
            ("abs((1e16+time)-1e16)", 0.9, 1.1),
        ] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let domain = TimeEnclosure::new(&program, 2.0)
                .unwrap()
                .evaluate(TimeInterval { lower, upper }, &context)
                .unwrap();
            let error = domain.interpolation_error((upper - lower) / 2.0);
            let mut vm = Vm::new();
            let mut evaluate = |time| vm.execute(&program, &Context { time, ..context });
            let left = evaluate(lower);
            let right = evaluate(upper);
            for sample in 0..=64 {
                let fraction = sample as Value / 64.0;
                let time = lower + (upper - lower) * fraction;
                let difference = (evaluate(time) - (left + (right - left) * fraction)).abs();
                assert!(
                    difference <= error,
                    "{expression}: {difference:e} exceeds {error:e}"
                );
            }
            if expression.starts_with("(1e16") {
                assert!(error >= 1.0, "a smooth derivative alone misses the VM step");
            }
        }
    }

    #[test]
    fn hyperbolic_bounds_enclose_independent_chain_rules_across_time_scales() {
        use crate::config::ExpressionDialect;
        type Derivative = fn(Value) -> Value;
        let cases: [(&str, Derivative); 8] = [
            ("atan(4*T-2)", |t| 4.0 / (1.0 + (4.0 * t - 2.0).powi(2))),
            ("sinh(2*T-1)", |t| 2.0 * (2.0 * t - 1.0).cosh()),
            ("cosh(2*T-1)", |t| 2.0 * (2.0 * t - 1.0).sinh()),
            ("tanh(40*T-20)", |t| 40.0 / (40.0 * t - 20.0).cosh().powi(2)),
            ("asinh(6*T-3)", |t| 6.0 / (6.0 * t - 3.0).hypot(1.0)),
            ("acosh(1+4*T)", |t| 4.0 / (4.0 * t * (2.0 + 4.0 * t)).sqrt()),
            ("atanh(0.8*(2*T-1))", |t| {
                1.6 / (1.0 - (0.8 * (2.0 * t - 1.0)).powi(2))
            }),
            // The partial derivative underflows if formed before the chain.
            ("atan(1e200*(1+T))", |t| 1e-200 / (1.0 + t).powi(2)),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for stop in [1e-300, 1.0, 1e300] {
                for (expression, derivative) in cases {
                    let expression = expression.replace('T', &format!("(time/{stop:e})"));
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                    let mut vm = Vm::new();
                    for index in 0..32 {
                        let lower = index as Value / 32.0 * stop;
                        let upper = (index + 1) as Value / 32.0 * stop;
                        let domain = bounds
                            .evaluate_centered(TimeInterval { lower, upper }, &context)
                            .unwrap_or_else(|| panic!("{expression}, {lower:e}..{upper:e}"));
                        for sample in 0..=8 {
                            let time = lower + (upper - lower) * sample as Value / 8.0;
                            let actual = vm.execute(&program, &Context { time, ..context });
                            assert!(
                                domain.value.contains(actual),
                                "{expression}: {actual:e} outside {:?}",
                                domain.value
                            );
                            let slope = derivative(time / stop);
                            assert!(
                                domain.slope.contains(slope),
                                "{expression}: {slope:e} outside {:?}",
                                domain.slope
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn hyperbolic_domains_and_dialect_plateaus_follow_the_vm() {
        use crate::config::ExpressionDialect;
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            let program = compile(&parse_expression_strict("atanh(0*(1e308*time))").unwrap());
            let actual = Vm::new().execute(
                &program,
                &Context {
                    time: 2.0,
                    ..context
                },
            );
            if dialect == ExpressionDialect::Xyce {
                // Xyce normalizes NaN only at the completed expression
                // boundary; this is not the ordinary atanh plateau.
                assert_eq!(actual.abs(), 1e50);
            } else {
                assert!(actual.is_nan());
            }
            assert!(
                TimeEnclosure::new(&program, 2.0)
                    .unwrap()
                    .evaluate(
                        TimeInterval {
                            lower: 0.0,
                            upper: 2.0
                        },
                        &context
                    )
                    .is_none(),
                "a saturation clamp must not conceal invalid input"
            );
            let program =
                compile(&parse_expression_strict("acosh(1+sqr(1e-200*(time+1)))").unwrap());
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
            assert!(domain.value.contains(0.0));
            assert!(domain.interpolation_error(1.0) < 1e-6);
            for expression in [
                "atan(-0)",
                "sinh(-0)",
                "cosh(0)",
                "tanh(21)",
                "asinh(-0)",
                "acosh(1)",
                "atanh(0.9)",
            ] {
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
                let expected = Vm::new().execute(&program, &context).to_bits();
                assert_eq!(domain.value.lower.to_bits(), expected, "{expression}");
                assert_eq!(domain.value.upper.to_bits(), expected, "{expression}");
            }
            for expression in ["acosh(time)", "atanh(2*time)", "atanh(-2*time)"] {
                let program = compile(&parse_expression_strict(expression).unwrap());
                let domain = TimeEnclosure::new(&program, 1.0).unwrap().evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    &context,
                );
                if dialect == ExpressionDialect::Xyce && expression.starts_with("atanh") {
                    assert!(domain.unwrap().value.is_finite());
                } else {
                    assert!(domain.is_none(), "invalid real domain: {expression}");
                }
            }
            if dialect == ExpressionDialect::Xyce {
                for expression in [
                    "atanh(1+time)",
                    "atanh(-1-time)",
                    "tanh(21+time)",
                    "tanh(-21-time)",
                ] {
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
                    let expected = Vm::new().execute(&program, &context);
                    assert_eq!(
                        (domain.value.lower, domain.value.upper),
                        (expected, expected)
                    );
                    assert_eq!((domain.slope.lower, domain.slope.upper), (0.0, 0.0));
                    assert_eq!(domain.interpolation_error(1.0), 0.0);
                }
            }
        }
    }

    #[test]
    fn directed_addition_preserves_exact_domains_and_encloses_inexact_sums() {
        let exact = TimeInterval::point(2.0).add(TimeInterval {
            lower: -1.0,
            upper: 1.0,
        });
        assert_eq!((exact.lower, exact.upper), (1.0, 3.0));
        for sign in [-1.0, 1.0] {
            let sum =
                TimeInterval::point(sign).add(TimeInterval::point(sign * Value::EPSILON / 4.0));
            let expected = if sign < 0.0 {
                (-1.0_f64.next_up(), -1.0)
            } else {
                (1.0, 1.0_f64.next_up())
            };
            assert_eq!((sum.lower, sum.upper), expected);
            let overflow =
                TimeInterval::point(sign * Value::MAX).add(TimeInterval::point(sign * Value::MAX));
            assert!(overflow.contains(sign * Value::INFINITY));
        }
        let sum =
            TimeInterval::point(Value::INFINITY).add(TimeInterval::point(Value::NEG_INFINITY));
        assert_eq!(
            (sum.lower, sum.upper),
            (Value::NEG_INFINITY, Value::INFINITY)
        );
    }
}
