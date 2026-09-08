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
            return Self {
                lower: self.lower + other.lower,
                upper: self.upper + other.upper,
            };
        }
        if other.lower == 0.0 && other.upper == 0.0 {
            return Self {
                lower: self.lower + other.lower,
                upper: self.upper + other.upper,
            };
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
            return self.zero_binary(other, false);
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
        .with_product_sign(self, other, products)
    }

    fn with_product_sign(mut self, left: Self, right: Self, corners: [Value; 4]) -> Self {
        if ((left.lower >= 0.0 && right.lower >= 0.0) || (left.upper <= 0.0 && right.upper <= 0.0))
            && self.lower <= 0.0
        {
            self.lower = if corners
                .iter()
                .any(|value| *value == 0.0 && value.is_sign_negative())
            {
                -0.0
            } else {
                0.0
            };
        }
        if ((left.lower >= 0.0 && right.upper <= 0.0) || (left.upper <= 0.0 && right.lower >= 0.0))
            && self.upper >= 0.0
        {
            self.upper = if corners
                .iter()
                .any(|value| *value == 0.0 && !value.is_sign_negative())
            {
                0.0
            } else {
                -0.0
            };
        }
        self
    }

    fn div(self, other: Self) -> Option<Self> {
        if other.contains(0.0) {
            return None;
        }
        if self.lower == 0.0 && self.upper == 0.0 && other.is_finite() {
            return Some(self.zero_binary(other, true));
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
            .with_product_sign(self, other, quotients),
        )
    }

    fn union(self, other: Self) -> Self {
        Self {
            lower: self.lower.min(other.lower),
            upper: self.upper.max(other.upper),
        }
    }

    // A numeric zero can still change atan2's quadrant. Retain both zero
    // signs when a zero product/quotient crosses the other operand's sign.
    fn zero_binary(self, other: Self, divide: bool) -> Self {
        let evaluate = |a: Value, b: Value| {
            if divide {
                if b == 0.0 { 0.0 } else { a / b }
            } else {
                a * b
            }
        };
        let corners = [
            evaluate(self.lower, other.lower),
            evaluate(self.lower, other.upper),
            evaluate(self.upper, other.lower),
            evaluate(self.upper, other.upper),
        ];
        let mut lower = corners[0];
        let mut upper = corners[0];
        for value in corners {
            if value.total_cmp(&lower).is_lt() {
                lower = value;
            }
            if value.total_cmp(&upper).is_gt() {
                upper = value;
            }
        }
        Self { lower, upper }
    }

    // Transcendental endpoints include a relative libm rounding allowance
    // plus an outward ULP, including at subnormal/zero results.
    fn transcendental(lower: Value, upper: Value) -> Self {
        Self::outward(
            if lower.is_finite() {
                lower - 16.0 * Value::EPSILON * lower.abs()
            } else {
                lower
            },
            if upper.is_finite() {
                upper + 16.0 * Value::EPSILON * upper.abs()
            } else {
                upper
            },
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
        let mut result = Self::transcendental(a.min(b), a.max(b));
        result.lower = result.lower.max(-1.0);
        result.upper = result.upper.min(1.0);
        // Endpoint libm error is relative to its result, including near a
        // zero. A fixed absolute allowance would obscure entire small-time
        // neighborhoods of atan2's branch cut. Interior extrema below still
        // override either endpoint sign when the interval crosses one.
        if !a.is_sign_negative() && !b.is_sign_negative() && result.lower <= 0.0 {
            result.lower = 0.0;
        }
        if a.is_sign_negative() && b.is_sign_negative() && result.upper >= 0.0 {
            result.upper = -0.0;
        }
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

/// Order of actual finite VM results, distinguishing signed-zero order from
/// numeric order. Only exact arithmetic propagates the stronger certificate.
#[derive(Clone, Copy)]
enum TimeOrder {
    Constant,
    Increasing,
    Decreasing,
    IncreasingSign,
    DecreasingSign,
    Unknown,
}

impl TimeOrder {
    fn reversed(self) -> Self {
        match self {
            Self::Increasing => Self::Decreasing,
            Self::Decreasing => Self::Increasing,
            Self::IncreasingSign => Self::DecreasingSign,
            Self::DecreasingSign => Self::IncreasingSign,
            order => order,
        }
    }

    fn scaled(self, negative: bool) -> Self {
        if negative { self.reversed() } else { self }
    }
    fn numeric(self) -> bool {
        matches!(self, Self::Constant | Self::Increasing | Self::Decreasing)
    }
    fn sign(self) -> Self {
        match self {
            Self::Increasing => Self::IncreasingSign,
            Self::Decreasing => Self::DecreasingSign,
            order => order,
        }
    }
    fn zero_product(self) -> Self {
        match self {
            Self::IncreasingSign => Self::Increasing,
            Self::DecreasingSign => Self::Decreasing,
            order => order,
        }
    }
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant, order) | (order, Self::Constant) if order.numeric() => order,
            (Self::Increasing, Self::Increasing) => Self::Increasing,
            (Self::Decreasing, Self::Decreasing) => Self::Decreasing,
            _ => Self::Unknown,
        }
    }
}

/// A value enclosure may include infinity, but must never hide a possible
/// NaN. Operations with an uncertain real domain return None for subdivision.
/// Derivative intervals can still be unbounded or indeterminate independently.
#[derive(Clone, Copy)]
struct Dual {
    value: TimeInterval,
    slope: TimeInterval,
    center: Value,
    constant: bool,
    continuous: bool,
    roundoff: Value,
    order: TimeOrder,
}

/// Bounds on one evaluation domain. Continuity is separate from a finite
/// derivative: a continuous cusp may have an unbounded derivative, whereas
/// a signed zero power has a finite-valued jump.
pub(crate) struct TimeBounds {
    pub value: TimeInterval,
    pub slope: TimeInterval,
    pub continuous: bool,
    pub vm_monotone: bool,
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
            order: TimeOrder::Constant,
        }
    }

    fn zero_binary(self, other: Self, divide: bool) -> Self {
        let value = self.value.zero_binary(other.value, divide);
        let center = if divide {
            if other.center == 0.0 {
                0.0
            } else {
                self.center / other.center
            }
        } else {
            self.center * other.center
        };
        let order = if self.constant {
            other
                .order
                .zero_product()
                .scaled(self.center.is_sign_negative())
        } else if other.constant && !divide {
            self.order
                .zero_product()
                .scaled(other.center.is_sign_negative())
        } else {
            TimeOrder::Unknown
        };
        let constant = value.lower.to_bits() == value.upper.to_bits();
        Self {
            value,
            center,
            slope: TimeInterval::ZERO,
            constant,
            continuous: true,
            roundoff: 0.0,
            order: if constant { TimeOrder::Constant } else { order },
        }
    }

    fn add(self, other: Self) -> Option<Self> {
        if self.constant && other.constant {
            return Some(Self::constant(self.value.lower + other.value.lower));
        }
        if (self.value.lower == Value::NEG_INFINITY && other.value.upper == Value::INFINITY)
            || (self.value.upper == Value::INFINITY && other.value.lower == Value::NEG_INFINITY)
        {
            return None;
        }
        let value = self.value.add(other.value);
        Some(Self {
            value,
            slope: self.slope.add(other.slope),
            center: self.center + other.center,
            constant: false,
            continuous: self.continuous && other.continuous,
            roundoff: (self.roundoff + other.roundoff + value.rounding_error(false)).next_up(),
            order: self.order.add(other.order),
        })
    }

    fn neg(self) -> Self {
        Self {
            value: self.value.neg(),
            slope: self.slope.neg(),
            center: -self.center,
            order: self.order.reversed(),
            ..self
        }
    }

    fn mul(self, other: Self) -> Option<Self> {
        if self.constant && other.constant {
            return Some(Self::constant(self.value.lower * other.value.lower));
        }
        if (self.value.contains(0.0) && !other.value.is_finite())
            || (other.value.contains(0.0) && !self.value.is_finite())
        {
            return None;
        }
        if (self.value.lower == 0.0 && self.value.upper == 0.0 && other.value.is_finite())
            || (other.value.lower == 0.0 && other.value.upper == 0.0 && self.value.is_finite())
        {
            return Some(self.zero_binary(other, false));
        }
        let value = self.value.mul(other.value);
        Some(Self {
            value,
            slope: self.slope.mul(other.value).add(self.value.mul(other.slope)),
            center: self.center * other.center,
            constant: false,
            continuous: self.continuous && other.continuous,
            roundoff: (propagated_error(other.value.magnitude(), self.roundoff)
                + propagated_error(self.value.magnitude(), other.roundoff)
                + value.rounding_error(false))
            .next_up(),
            order: if self.constant {
                other.order.scaled(self.center.is_sign_negative())
            } else if other.constant {
                self.order.scaled(other.center.is_sign_negative())
            } else {
                TimeOrder::Unknown
            },
        })
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
            order: TimeOrder::Unknown,
        }
    }

    fn div(self, other: Self) -> Option<Self> {
        // Keep the VM's defined zero-denominator value, including locally
        // exact zeros, without declaring an uncertain crossing continuous.
        if other.value.lower == 0.0 && other.value.upper == 0.0 {
            return Some(Self::constant(0.0));
        }
        if self.value.lower == 0.0 && self.value.upper == 0.0 && other.value.is_finite() {
            return Some(self.zero_binary(other, true));
        }
        if self.constant && other.constant {
            return Some(Self::constant(self.value.lower / other.value.lower));
        }
        if !self.value.is_finite() && !other.value.is_finite() {
            return None;
        }
        if other.value.contains(0.0) {
            // A quotient crossing zero is defined by the VM (zero at the
            // denominator's exact zero), but it has no finite continuous
            // enclosure. Preserve that distinction for bounded outer
            // functions without admitting an indeterminate infinity ratio.
            let corners = [
                (self.value.lower, other.value.lower),
                (self.value.lower, other.value.upper),
                (self.value.upper, other.value.lower),
                (self.value.upper, other.value.upper),
            ]
            .map(|(numerator, denominator)| {
                if denominator == 0.0 {
                    0.0
                } else {
                    numerator / denominator
                }
            });
            return Some(Self {
                // Retain an authenticated half-line, including the VM's
                // +0 at exact denominator zeros. Losing this sign prevents
                // exp of a nonpositive reciprocal from proving its bound.
                value: TimeInterval::WHOLE.with_product_sign(self.value, other.value, corners),
                slope: TimeInterval::WHOLE,
                center: if other.center == 0.0 {
                    0.0
                } else {
                    self.center / other.center
                },
                constant: false,
                continuous: false,
                roundoff: Value::INFINITY,
                order: TimeOrder::Unknown,
            });
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
            order: if other.constant {
                self.order.scaled(other.center.is_sign_negative())
            } else {
                TimeOrder::Unknown
            },
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
            order: TimeOrder::Unknown,
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
        // Finite exponents have defined magnitude powers at infinite bases.
        // Negative-base projection below still rejects invalid real domains
        // and indeterminate products such as infinity times a zero phase.
        if !exponent.value.is_finite() {
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
                    .mul(Self::constant(std::f64::consts::PI))?
                    .trigonometric(true)?
            } else if matches!(instruction, Instruction::FunctionPow) {
                Self::constant(1.0)
            } else {
                // A varying real exponent on a negative base has no open
                // real domain under the ordinary power-operator contract.
                return None;
            };
            let powered = negative.neg().positive_power(exponent).mul(coefficient)?;
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
                    order: TimeOrder::Unknown,
                },
            });
        }
        result.map(|power| Self {
            center: evaluate(self.center, exponent.center),
            ..power
        })
    }

    fn trigonometric(self, cosine: bool) -> Option<Self> {
        if self.constant {
            return Some(Self::constant(if cosine {
                self.value.lower.cos()
            } else {
                self.value.lower.sin()
            }));
        }
        if !self.value.is_finite() {
            return None;
        }
        let derivative = self.value.trigonometric(!cosine);
        let value = self.value.trigonometric(cosine);
        Some(Self {
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
            // A monotone VM phase traversing one sign change cannot reverse
            // that sign. Keep this weaker proof separate from numeric value
            // monotonicity: libm rounding need not preserve adjacent values.
            order: if self.order.numeric() && derivative.lower >= 0.0 {
                self.order.sign().scaled(cosine)
            } else if self.order.numeric() && derivative.upper <= 0.0 {
                self.order.sign().scaled(!cosine)
            } else {
                TimeOrder::Unknown
            },
        })
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
            order: TimeOrder::Unknown,
        }
    }

    fn circular_inverse(mut self, cosine: bool) -> Option<Self> {
        let evaluate = |value: Value| {
            let value = value.clamp(-1.0, 1.0);
            if cosine { value.acos() } else { value.asin() }
        };
        if self.constant {
            return Some(Self::constant(evaluate(self.value.lower)));
        }
        self = self
            .extremum(Self::constant(-1.0), true)
            .extremum(Self::constant(1.0), false);
        if self.constant {
            return Some(Self::constant(evaluate(self.value.lower)));
        }
        let endpoints = [evaluate(self.value.lower), evaluate(self.value.upper)];
        let value = TimeInterval::transcendental(
            endpoints[usize::from(cosine)],
            endpoints[usize::from(!cosine)],
        );
        let nearest = if self.value.contains(0.0) {
            0.0
        } else {
            self.value.lower.abs().min(self.value.upper.abs())
        };
        let farthest = self.value.magnitude();
        let mut first =
            TimeInterval::transcendental((1.0 - farthest).sqrt(), (1.0 - nearest).sqrt());
        first.lower = first.lower.max(0.0);
        let second = TimeInterval::transcendental((1.0 + nearest).sqrt(), (1.0 + farthest).sqrt());
        let mut slope = self
            .slope
            .div(first)
            .and_then(|slope| slope.div(second))
            .unwrap_or(TimeInterval::WHOLE);
        if cosine {
            slope = slope.neg();
        }
        // Across either clamp, |delta angle| <= pi*sqrt(|delta x|/2).
        // Use the local derivative when it is finite; the Holder bound also
        // covers a rounded argument at an endpoint with infinite derivative.
        let holder = std::f64::consts::FRAC_PI_2 * std::f64::consts::SQRT_2 * self.roundoff.sqrt();
        let holder = TimeInterval::transcendental(holder, holder).upper;
        let local = ((self.roundoff / first.lower).next_up() / second.lower).next_up();
        Some(Self {
            value,
            slope,
            center: evaluate(self.center),
            constant: false,
            continuous: self.continuous,
            roundoff: (holder.min(local) + value.rounding_error(true)).next_up(),
            order: TimeOrder::Unknown,
        })
    }

    fn tangent(self) -> Option<Self> {
        if self.constant {
            return Some(Self::constant(self.value.lower.tan()));
        }
        if !self.value.is_finite() {
            return None;
        }
        let cosine = self.value.trigonometric(true);
        if cosine.contains(0.0) {
            // A pole is not a finite interpolation certificate. Keep its
            // unbounded range so an enclosing operation can still prove a
            // bounded value; raw singular forcing cannot pass resolution.
            return Some(Self {
                value: TimeInterval::WHOLE,
                slope: TimeInterval::WHOLE,
                center: self.center.tan(),
                constant: false,
                continuous: false,
                roundoff: Value::INFINITY,
                order: TimeOrder::Unknown,
            });
        }
        let value = TimeInterval::transcendental(self.value.lower.tan(), self.value.upper.tan());
        let slope = self
            .slope
            .div(cosine)
            .and_then(|slope| slope.div(cosine))
            .unwrap_or(TimeInterval::WHOLE);
        let minimum = cosine.lower.abs().min(cosine.upper.abs());
        let roundoff = ((self.roundoff / minimum).next_up() / minimum).next_up();
        Some(Self {
            value,
            slope,
            center: self.center.tan(),
            constant: false,
            continuous: self.continuous,
            roundoff: (roundoff + value.rounding_error(true)).next_up(),
            order: TimeOrder::Unknown,
        })
    }

    fn polar_angle(self, x: Self) -> Option<Self> {
        let center = self.center.atan2(x.center);
        if self.constant && x.constant {
            return Some(Self::constant(center));
        }
        if !self.value.is_finite() || !x.value.is_finite() {
            return None;
        }
        let zero_coordinate = self.value.lower == 0.0 && self.value.upper == 0.0;
        if zero_coordinate
            && self.value.lower.to_bits() == self.value.upper.to_bits()
            && (x.value.lower > 0.0 || x.value.upper < 0.0)
        {
            return Some(Self::constant(self.value.lower.atan2(x.center)));
        }
        let origin = self.value.contains(0.0) && x.value.contains(0.0);
        let negative_axis = x.value.lower < 0.0
            && self.value.lower.is_sign_negative()
            && !self.value.upper.is_sign_negative();
        if origin || negative_axis {
            return Some(Self {
                value: TimeInterval::transcendental(-std::f64::consts::PI, std::f64::consts::PI),
                slope: TimeInterval::WHOLE,
                center,
                constant: false,
                continuous: false,
                roundoff: Value::INFINITY,
                order: if zero_coordinate && x.value.upper < 0.0 {
                    self.order.zero_product()
                } else {
                    TimeOrder::Unknown
                },
            });
        }
        let corners = [
            self.value.lower.atan2(x.value.lower),
            self.value.lower.atan2(x.value.upper),
            self.value.upper.atan2(x.value.lower),
            self.value.upper.atan2(x.value.upper),
        ];
        let value = TimeInterval::transcendental(
            corners.into_iter().fold(Value::INFINITY, Value::min),
            corners.into_iter().fold(Value::NEG_INFINITY, Value::max),
        );
        // Normalize both coordinates together before forming the squared
        // radius. This is invariant to extreme common voltage/current gain.
        let scale = TimeInterval::point(self.value.magnitude().max(x.value.magnitude()));
        let yn = self.value.div(scale)?;
        let xn = x.value.div(scale)?;
        let radius = yn.square().add(xn.square());
        let slope = self
            .slope
            .div(scale)?
            .mul(xn)
            .add(x.slope.div(scale)?.mul(yn).neg())
            .div(radius)
            .unwrap_or(TimeInterval::WHOLE);
        let nearest = |value: TimeInterval| {
            if value.contains(0.0) {
                0.0
            } else {
                value.lower.abs().min(value.upper.abs())
            }
        };
        let distance = nearest(yn).hypot(nearest(xn));
        let distance = TimeInterval::transcendental(distance, distance)
            .lower
            .max(0.0);
        let error = ((self.roundoff / scale.lower).next_up() / distance)
            .next_up()
            .hypot(((x.roundoff / scale.lower).next_up() / distance).next_up());
        let roundoff = TimeInterval::transcendental(error, error).upper;
        Some(Self {
            value,
            slope,
            center,
            constant: false,
            continuous: self.continuous && x.continuous,
            roundoff: (roundoff + value.rounding_error(true)).next_up(),
            order: TimeOrder::Unknown,
        })
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
        // Every incoming value excludes NaN, even when its range is
        // unbounded. The outer function can therefore use its defined
        // infinite limits without hiding invalid arithmetic such as 0*inf.
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
            order: TimeOrder::Unknown,
        })
    }

    fn absolute(self) -> Self {
        if self.constant {
            return Self::constant(self.value.lower.abs());
        }
        if self.value.lower >= 0.0 {
            return Self {
                value: TimeInterval {
                    lower: self.value.lower.abs(),
                    upper: self.value.upper.abs(),
                },
                center: self.center.abs(),
                order: TimeOrder::Unknown,
                ..self
            };
        }
        if self.value.upper <= 0.0 {
            let result = self.neg();
            return Self {
                value: TimeInterval {
                    lower: result.value.lower.abs(),
                    ..result.value
                },
                center: self.center.abs(),
                order: TimeOrder::Unknown,
                ..result
            };
        }
        Self {
            value: TimeInterval {
                lower: 0.0,
                upper: self.value.magnitude(),
            },
            slope: self.slope.union(self.slope.neg()),
            center: self.center.abs(),
            order: TimeOrder::Unknown,
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
        let mut value = TimeInterval {
            lower: select(self.value.lower, other.value.lower),
            upper: select(self.value.upper, other.value.upper),
        };
        let zero_changes_sign = value.lower == 0.0
            && value.upper == 0.0
            && value.lower.to_bits() != value.upper.to_bits();
        if zero_changes_sign {
            value = TimeInterval {
                lower: -0.0,
                upper: 0.0,
            };
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
            return Self {
                value,
                center,
                constant: self.constant && !zero_changes_sign,
                order: TimeOrder::Unknown,
                ..self
            };
        }
        if right_wins {
            return Self {
                value,
                center,
                constant: other.constant && !zero_changes_sign,
                order: TimeOrder::Unknown,
                ..other
            };
        }
        Self {
            value,
            slope: self.slope.union(other.slope),
            center,
            constant: false,
            continuous: self.continuous && other.continuous,
            // Selection is exact and 1-Lipschitz in the maximum input error.
            // An inactive operand contributes neither slope nor error above.
            roundoff: self.roundoff.max(other.roundoff),
            order: TimeOrder::Unknown,
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
            order: TimeOrder::Unknown,
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
            order: TimeOrder::Unknown,
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
            let mut lower = self.value.lower.max(centered.lower);
            let mut upper = self.value.upper.min(centered.upper);
            // Mean-value bounds constrain numeric values, not the sign of
            // an exact zero. Preserve that sign evidence for atan2.
            if lower == 0.0 {
                lower = lower.copysign(self.value.lower);
            }
            if upper == 0.0 {
                upper = upper.copysign(self.value.upper);
            }
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
                    | Instruction::Tan
                    | Instruction::Asin
                    | Instruction::Acos
                    | Instruction::Atan2
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
                    order: TimeOrder::Increasing,
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
                        Instruction::Add => left.add(right)?,
                        Instruction::Sub => left.add(right.neg())?,
                        Instruction::Mul => left.mul(right)?,
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
                Instruction::Sin => self.stack.pop()?.trigonometric(false)?,
                Instruction::Cos => self.stack.pop()?.trigonometric(true)?,
                Instruction::Tan => self.stack.pop()?.tangent()?,
                Instruction::Asin => self.stack.pop()?.circular_inverse(false)?,
                Instruction::Acos => self.stack.pop()?.circular_inverse(true)?,
                Instruction::Atan2 => {
                    let x = self.stack.pop()?;
                    self.stack.pop()?.polar_angle(x)?
                }
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
                    vm_monotone: value.order.numeric(),
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
    fn zero_denominator_enclosures_preserve_quotient_signs_and_vm_positive_zero() {
        for dialect in [
            crate::config::ExpressionDialect::Ngspice,
            crate::config::ExpressionDialect::Xyce,
        ] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for numerator in [-1e300_f64, -1e-300, -0.0, 0.0, 1e-300, 1e300] {
                for sign in [-1.0, 1.0] {
                    let expression = format!("({numerator:e})/({sign:e}*sqr(time-0.5))");
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let domain = TimeEnclosure::new(&program, 1.0)
                        .unwrap()
                        .evaluate(
                            TimeInterval {
                                lower: 0.49,
                                upper: 0.51,
                            },
                            &context,
                        )
                        .unwrap();
                    assert!(domain.value.contains(0.0), "{expression}");
                    if numerator * sign > 0.0 {
                        assert_eq!(domain.value.lower, 0.0, "{expression}");
                    } else if numerator * sign < 0.0 {
                        assert_eq!(
                            domain.value.upper.to_bits(),
                            0.0_f64.to_bits(),
                            "{expression}"
                        );
                    }
                    let mut vm = Vm::new();
                    for index in 0..=32 {
                        let actual = vm.execute(
                            &program,
                            &Context {
                                time: 0.49 + 0.02 * index as Value / 32.0,
                                ..context
                            },
                        );
                        assert!(
                            domain.value.contains(actual),
                            "{expression}: {actual} outside {:?}",
                            domain.value
                        );
                        if actual == 0.0 {
                            assert!(
                                domain.value.lower.total_cmp(&actual).is_le()
                                    && domain.value.upper.total_cmp(&actual).is_ge(),
                                "{expression}: lost zero sign"
                            );
                        }
                    }
                    assert_eq!(
                        vm.execute(
                            &program,
                            &Context {
                                time: 0.5,
                                ..context
                            }
                        )
                        .to_bits(),
                        0.0_f64.to_bits()
                    );
                }
            }
        }
    }

    #[test]
    fn bounded_compositions_keep_defined_infinite_limits_and_refuse_nan() {
        for dialect in [
            crate::config::ExpressionDialect::Ngspice,
            crate::config::ExpressionDialect::Xyce,
        ] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for stop in [1e-300, 1.0, 1e300] {
                let phase = format!("pi*time/{stop:e}");
                for expression in [
                    format!("atan(tan({phase}))"),
                    format!("tanh(2*tan({phase})+1)"),
                    format!("atan(-tan({phase})/2)"),
                    format!("asin(tan({phase}))"),
                    format!("atan(exp(1000+sin({phase})))"),
                    format!("atan(1/cos({phase}))"),
                    format!("atan(sin({phase})/cos({phase}))"),
                    format!("tanh(tan({phase})^2)"),
                    format!("tanh(pow(tan({phase}),2))"),
                    format!("exp(-1/cos({phase})^2)"),
                ] {
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let domain = TimeEnclosure::new(&program, stop)
                        .unwrap()
                        .evaluate(
                            TimeInterval {
                                lower: 0.49 * stop,
                                upper: 0.51 * stop,
                            },
                            &context,
                        )
                        .unwrap_or_else(|| panic!("{expression}"));
                    assert!(domain.value.is_finite(), "{expression}");
                    let mut vm = Vm::new();
                    for index in 0..=32 {
                        let actual = vm.execute(
                            &program,
                            &Context {
                                time: stop * (0.49 + 0.02 * index as Value / 32.0),
                                ..context
                            },
                        );
                        assert!(
                            actual.is_finite() && domain.value.contains(actual),
                            "{expression}: {actual} outside {:?}",
                            domain.value
                        );
                    }
                }
            }
            for invalid in [
                "atan(0*exp(1000+time))",
                "asin(0*exp(1000+time))",
                "atanh(0*exp(1000+time))",
                "tanh(exp(1000+time)/exp(1000+time))",
                "tanh(exp(1000+time)-exp(1000+time))",
                "atan(sin(exp(1000+time)))",
                "atan(cos(exp(1000+time)))",
                "atan(tan(exp(1000+time)))",
            ] {
                let program = compile(&parse_expression_strict(invalid).unwrap());
                let domain = TimeEnclosure::new(&program, 1.0).unwrap().evaluate(
                    TimeInterval {
                        lower: 0.0,
                        upper: 1.0,
                    },
                    &context,
                );
                assert!(
                    domain.is_none(),
                    "{invalid}: a possible NaN cannot establish a finite bound"
                );
            }
        }
    }

    #[test]
    fn polar_vm_order_proves_local_plateaus_without_qualifying_aliased_cycles() {
        let context = Context::transient(&[], &[], 0.0);
        for wave in ["sin", "cos"] {
            for direction in [-1, 1] {
                for zero in ["0", "-0"] {
                    for operation in ["*", "/"] {
                        let expression =
                            format!("atan2({zero}{operation}{wave}({direction}*8*pi*time+0.1),-1)");
                        let program = compile(&parse_expression_strict(&expression).unwrap());
                        let mut bounds = TimeEnclosure::new(&program, 1.0).unwrap();
                        let mut vm = Vm::new();
                        assert!(
                            !bounds
                                .evaluate(
                                    TimeInterval {
                                        lower: 0.0,
                                        upper: 1.0
                                    },
                                    &context
                                )
                                .unwrap()
                                .vm_monotone,
                            "{expression}"
                        );
                        let mut certified = 0;
                        for index in 0..128 {
                            let lower = index as Value / 128.0;
                            let upper = (index + 1) as Value / 128.0;
                            let domain = bounds
                                .evaluate(TimeInterval { lower, upper }, &context)
                                .unwrap();
                            if !domain.vm_monotone {
                                continue;
                            }
                            certified += 1;
                            let values = (0..=32)
                                .map(|sample| {
                                    vm.execute(
                                        &program,
                                        &Context {
                                            time: lower + (upper - lower) * sample as Value / 32.0,
                                            ..context
                                        },
                                    )
                                })
                                .collect::<Vec<_>>();
                            assert!(
                                values.windows(2).all(|pair| pair[0] <= pair[1])
                                    || values.windows(2).all(|pair| pair[0] >= pair[1]),
                                "{expression} at {lower}"
                            );
                            if values[0] == values[32] {
                                assert!(
                                    values.iter().all(|value| *value == values[0]),
                                    "{expression} at {lower}"
                                );
                            }
                        }
                        assert!(certified > 100, "{expression}: {certified}");
                    }
                }
            }
        }
    }

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
    fn constant_bounds_follow_the_vm_and_stateful_bounds_are_unavailable() {
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
        let program = compile(&parse_expression_strict("1/(time-0.5)").unwrap());
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
        assert!(!domain.value.is_finite() && !domain.continuous);
        let program = compile(&parse_expression_strict("0*(1e308*time)").unwrap());
        let domain = TimeEnclosure::new(&program, 2.0).unwrap().evaluate(
            TimeInterval {
                lower: 0.0,
                upper: 2.0,
            },
            &context,
        );
        assert!(
            domain.is_none(),
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
                .is_some_and(|domain| !domain.value.is_finite() && !domain.continuous)
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
        let domain = TimeEnclosure::new(&program, 2.0).unwrap().evaluate(
            TimeInterval {
                lower: 0.0,
                upper: 2.0,
            },
            &context,
        );
        assert!(
            domain.is_none(),
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
                    "asin(0.8*sin(2*pi*T))",
                    "acos(0.8*cos(2*pi*T))",
                    "asin(4*T-2)",
                    "acos(4*T-2)",
                    "tan(0.6*sin(2*pi*T))",
                    "atan2(sin(2*pi*T),cos(2*pi*T))",
                    "atan2(0*(T-0.5),-1)",
                    "atan2(0/(T-0.5),-1)",
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

    #[test]
    fn circular_bounds_match_chain_rules_and_remain_invariant_to_time_and_polar_gain() {
        use crate::config::ExpressionDialect;
        type Derivative = fn(Value) -> Value;
        let rate = std::f64::consts::TAU;
        let cases: [(&str, Derivative); 3] = [
            ("asin(0.8*sin(2*pi*T))", |phase| {
                0.8 * phase.cos() / (1.0 - (0.8 * phase.sin()).powi(2)).sqrt()
            }),
            ("acos(0.8*sin(2*pi*T))", |phase| {
                -0.8 * phase.cos() / (1.0 - (0.8 * phase.sin()).powi(2)).sqrt()
            }),
            ("tan(0.6*sin(2*pi*T))", |phase| {
                0.6 * phase.cos() / (0.6 * phase.sin()).cos().powi(2)
            }),
        ];
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for stop in [1e-300, 1.0, 1e300] {
                let mut expressions: Vec<(String, Option<Derivative>)> = cases
                    .iter()
                    .map(|(expr, derivative)| (expr.to_string(), Some(*derivative)))
                    .collect();
                for gain in [-1e200, -1e-200, 1e-200, 1.0, 1e200] {
                    expressions.push((
                        format!("atan2({gain:e}*sin(2*pi*T),{gain:e}*cos(2*pi*T))"),
                        None,
                    ));
                }
                for (expression, derivative) in expressions {
                    let expression = expression.replace('T', &format!("(time/{stop:e})"));
                    let program = compile(&parse_expression_strict(&expression).unwrap());
                    let mut bounds = TimeEnclosure::new(&program, stop).unwrap();
                    let mut vm = Vm::new();
                    for index in 0..64 {
                        let lower = index as Value / 64.0 * stop;
                        let upper = (index + 1) as Value / 64.0 * stop;
                        let domain = bounds
                            .evaluate_centered(TimeInterval { lower, upper }, &context)
                            .unwrap();
                        for sample in 0..=8 {
                            let time = lower + (upper - lower) * sample as Value / 8.0;
                            let value = vm.execute(&program, &Context { time, ..context });
                            assert!(
                                domain.value.contains(value),
                                "{expression}: {value:e} outside {:?}",
                                domain.value
                            );
                            let slope = derivative
                                .map_or(rate, |derivative| rate * derivative(rate * time / stop));
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
    fn circular_domains_preserve_vm_clamps_poles_and_signed_zero_quadrants() {
        use crate::config::ExpressionDialect;
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for expression in ["asin(2+time)", "acos(-2-time)"] {
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
                let actual = Vm::new().execute(&program, &context);
                assert_eq!((domain.value.lower, domain.value.upper), (actual, actual));
                assert_eq!(domain.interpolation_error(1.0), 0.0);
            }
            for expression in [
                "tan(pi*time)",
                "atan2(time-0.5,-1)",
                "atan2(0*(time-0.5),-1)",
                "atan2(0/(time-0.5),-1)",
            ] {
                let program = compile(&parse_expression_strict(expression).unwrap());
                let domain = TimeEnclosure::new(&program, 1.0)
                    .unwrap()
                    .evaluate_centered(
                        TimeInterval {
                            lower: 0.25,
                            upper: 0.75,
                        },
                        &context,
                    )
                    .unwrap();
                assert!(!domain.continuous, "{expression}");
                for time in [0.25, 0.5_f64.next_down(), 0.5, 0.5_f64.next_up(), 0.75] {
                    let actual = Vm::new().execute(&program, &Context { time, ..context });
                    assert!(
                        domain.value.contains(actual),
                        "{expression}: {actual:e} outside {:?}",
                        domain.value
                    );
                }
            }
            for (expression, negative) in [
                ("atan2(abs(0*(time-0.5)),-1)", false),
                ("atan2(-abs(0*(time-0.5)),-1)", true),
            ] {
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
                assert!(domain.continuous);
                assert!(domain.interpolation_error(1.0) < 1e-10);
                assert_eq!(domain.value.lower.is_sign_negative(), negative);
                assert_eq!(domain.value.upper.is_sign_negative(), negative);
            }
            for expression in [
                "atan2(0,-1)",
                "atan2(-0,-1)",
                "atan2(-0,0)",
                "atan2(0,-0)",
                "asin(-0)",
                "acos(-0)",
                "tan(-0)",
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
                let actual = Vm::new().execute(&program, &context);
                assert_eq!(
                    domain.value.lower.to_bits(),
                    actual.to_bits(),
                    "{expression}"
                );
                assert_eq!(
                    domain.value.upper.to_bits(),
                    actual.to_bits(),
                    "{expression}"
                );
            }
        }
    }

    #[test]
    fn polar_bounds_preserve_zero_arithmetic_and_small_time_half_planes() {
        let context = Context::transient(&[], &[], 0.0);
        for expression in [
            "atan2(0-time+0,-1)",
            "atan2(0+(-time),-1)",
            "atan2(max(-time,0),-1)",
            "atan2(min(time,-0),-1)",
            "atan2(0*(time-0.5),-1)",
            "atan2(0/(time-0.5),-1)",
        ] {
            let program = compile(&parse_expression_strict(expression).unwrap());
            let mut bounds = TimeEnclosure::new(&program, 1.0).unwrap();
            for time in [0.0, 0.5, 1.0] {
                let domain = bounds
                    .evaluate_centered(
                        TimeInterval {
                            lower: time,
                            upper: time,
                        },
                        &context,
                    )
                    .unwrap();
                let actual = Vm::new().execute(&program, &Context { time, ..context });
                assert!(
                    domain.value.contains(actual),
                    "{expression}, {time}: {actual:e} outside {:?}",
                    domain.value
                );
            }
        }
        for gain in [-1e200, -1.0, 1.0, 1e200] {
            let expression = format!("atan2({gain:e}*sin(pi*time),-1)");
            let program = compile(&parse_expression_strict(&expression).unwrap());
            for width in [1e-300, 1e-100, 1e-20] {
                let domain = TimeEnclosure::new(&program, 1.0)
                    .unwrap()
                    .evaluate_centered(
                        TimeInterval {
                            lower: 0.0,
                            upper: width,
                        },
                        &context,
                    )
                    .unwrap();
                assert!(domain.continuous, "{expression}, {width:e}");
                for time in [0.0, width * 0.5, width] {
                    let actual = Vm::new().execute(&program, &Context { time, ..context });
                    assert!(
                        domain.value.contains(actual),
                        "{expression}: {actual:e} outside {:?}",
                        domain.value
                    );
                }
            }
        }
        let expression = "atan2(1e308*(1.3+0.1*time),1e308*(1.3+0.2*time))";
        let program = compile(&parse_expression_strict(expression).unwrap());
        let domain = TimeEnclosure::new(&program, 1.0)
            .unwrap()
            .evaluate_centered(
                TimeInterval {
                    lower: 0.5,
                    upper: 0.50001,
                },
                &context,
            )
            .unwrap();
        assert!(domain.slope.is_finite());
        assert!(
            domain.interpolation_error(0.00001) < 1e-8,
            "a physical radius overflow must not hide local angular accuracy"
        );
    }
}
