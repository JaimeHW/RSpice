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

    fn reciprocal(self) -> Option<Self> {
        (!self.contains(0.0)).then(|| Self::outward(1.0 / self.upper, 1.0 / self.lower))
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
}

impl Dual {
    fn constant(value: Value) -> Self {
        Self {
            value: TimeInterval::point(value),
            slope: TimeInterval::ZERO,
            constant: true,
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
        Self {
            value: self.value.mul(other.value),
            slope: self.slope.mul(other.value).add(self.value.mul(other.slope)),
            constant: false,
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
        }
    }

    fn reciprocal(self) -> Option<Self> {
        let value = self.value.reciprocal()?;
        Some(Self {
            value,
            slope: self.slope.neg().mul(value.square()),
            constant: self.constant,
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
        Self {
            value: self.value.trigonometric(cosine),
            slope: self
                .slope
                .mul(if cosine { derivative.neg() } else { derivative }),
            constant: false,
        }
    }
}

/// Reusable scratch for continuous expressions whose bounds are implemented.
/// Unsupported or singular operators yield `None`, never a zero range.
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
                    | Instruction::Neg
                    | Instruction::Sin
                    | Instruction::Cos
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

    pub fn evaluate(
        &mut self,
        time: TimeInterval,
        context: &Context<'_>,
    ) -> Option<(TimeInterval, TimeInterval)> {
        self.stack.clear();
        for instruction in &self.program.instructions {
            let value = match instruction {
                Instruction::PushConst(value) => Dual::constant(*value),
                Instruction::PushTime => Dual {
                    value: time,
                    slope: TimeInterval::point(self.stop),
                    constant: false,
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
                        Instruction::Div
                            if left.constant && right.constant && right.value.lower != 0.0 =>
                        {
                            Dual::constant(left.value.lower / right.value.lower)
                        }
                        Instruction::Div => left.mul(right.reciprocal()?),
                        _ => return None,
                    }
                }
                Instruction::Neg => self.stack.pop()?.neg(),
                Instruction::Sin => self.stack.pop()?.trigonometric(false),
                Instruction::Cos => self.stack.pop()?.trigonometric(true),
                Instruction::Sqr => self.stack.pop()?.square(),
                _ => return None,
            };
            if value.value.lower.is_nan() || value.value.upper.is_nan() {
                return None;
            }
            self.stack.push(value);
        }
        (self.stack.len() == 1)
            .then(|| self.stack.first().map(|value| (value.value, value.slope)))
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
                let (value, slope) = bounds
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
        let (value, slope) = TimeEnclosure::new(&program, 1.0)
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
            .0;
        assert!(
            !value.is_finite(),
            "zero cannot conceal a potentially nonfinite intermediate"
        );
    }

    #[test]
    fn dynamic_products_quotients_and_squares_enclose_their_derivatives() {
        let stop = std::f64::consts::TAU;
        let context = Context::transient(&[], &[], 0.0);
        for (expression, derivative) in [
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
                let (value, slope) = bounds
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
}
