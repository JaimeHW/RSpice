//! Parameter tangents preserve complex arithmetic and authored random draws.
//! The ordinary evaluator remains responsible for every nominal value.

use super::eval::{
    PreparedEvaluation, apply_binary, apply_unary, complex_ln, complex_sqrt,
    eval_builtin_with_sample, xyce_constant_fold_builtin,
};
use super::{BinOpKind, ComplexValue, ExprError, ParamContext, UnaryOpKind, is_real};
use crate::Value;
use crate::config::ExpressionDialect;
use crate::expr::Derivative;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Both components keep their exponent until an owning consumer projects them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ComplexDirection {
    pub(crate) re: Derivative,
    pub(crate) im: Derivative,
}

impl ComplexDirection {
    pub(crate) fn zero() -> Self {
        Self::from(0.0)
    }

    /// An undefined direction travels as a NaN tangent so that only the values
    /// that actually consume it are undefined. An operation whose own result is
    /// locally constant in the parameter absorbs it instead: a comparison away
    /// from its switching point, or a step away from its jump, has a defined
    /// derivative no matter what its argument's tangent is. A dependency whose
    /// own stored direction is undefined enters a later binding as this same
    /// tangent, so one definition's kink refuses only the values that read it.
    pub(crate) fn undefined() -> Self {
        Self::from(Value::NAN)
    }

    pub(crate) fn is_undefined(self) -> bool {
        self.re.binary64().is_nan() || self.im.binary64().is_nan()
    }

    pub(crate) fn is_zero(self) -> bool {
        self.re == 0.0 && self.im == 0.0
    }

    pub(crate) fn binary64(self) -> ComplexValue {
        ComplexValue::new(self.re.binary64(), self.im.binary64())
    }

    fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    fn exp_multiply(self, exponent: ComplexValue) -> Self {
        let rotated = self * ComplexValue::new(exponent.im.cos(), exponent.im.sin());
        Self {
            re: if rotated.re == 0.0 {
                0.0.into()
            } else {
                rotated.re.multiply_exp(exponent.re)
            },
            im: if rotated.im == 0.0 {
                0.0.into()
            } else {
                rotated.im.multiply_exp(exponent.re)
            },
        }
    }
}

impl From<Value> for ComplexDirection {
    fn from(value: Value) -> Self {
        Self {
            re: value.into(),
            im: 0.0.into(),
        }
    }
}
impl From<Derivative> for ComplexDirection {
    fn from(re: Derivative) -> Self {
        Self { re, im: 0.0.into() }
    }
}
impl From<ComplexValue> for ComplexDirection {
    fn from(value: ComplexValue) -> Self {
        Self {
            re: value.re.into(),
            im: value.im.into(),
        }
    }
}
impl Add for ComplexDirection {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl Neg for ComplexDirection {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}
impl Sub for ComplexDirection {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl Mul for ComplexDirection {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // A structurally absent component cannot acquire NaN from an unused
        // singular coefficient in the other component's expression.
        let product = |left: Derivative, right: Derivative| {
            if left == 0.0 || right == 0.0 {
                0.0.into()
            } else {
                left * right
            }
        };
        Self {
            re: product(self.re, rhs.re) - product(self.im, rhs.im),
            im: product(self.re, rhs.im) + product(self.im, rhs.re),
        }
    }
}
impl Div for ComplexDirection {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let numerator = self * rhs.conjugate();
        let denominator = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: numerator.re / denominator,
            im: numerator.im / denominator,
        }
    }
}
impl Mul<ComplexValue> for ComplexDirection {
    type Output = Self;
    fn mul(self, rhs: ComplexValue) -> Self {
        self * Self::from(rhs)
    }
}
impl Div<ComplexValue> for ComplexDirection {
    type Output = Self;
    fn div(self, rhs: ComplexValue) -> Self {
        self / Self::from(rhs)
    }
}
impl Mul<Value> for ComplexDirection {
    type Output = Self;
    fn mul(self, rhs: Value) -> Self {
        self * Self::from(rhs)
    }
}
impl Div<Value> for ComplexDirection {
    type Output = Self;
    fn div(self, rhs: Value) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

pub(super) struct ParameterDirection<'a, F> {
    resolver: &'a mut F,
    directions: Vec<ComplexDirection>,
    error: Option<ExprError>,
    consumed: bool,
}

impl<'a, F> ParameterDirection<'a, F> {
    pub(super) fn new(resolver: &'a mut F) -> Self {
        Self {
            resolver,
            directions: Vec::new(),
            error: None,
            consumed: false,
        }
    }

    fn pop(&mut self) -> Result<ComplexDirection, ExprError> {
        self.directions
            .pop()
            .ok_or_else(|| invalid("parameter direction stack underflow"))
    }

    pub(super) fn finish(mut self) -> Result<ComplexDirection, ExprError> {
        if self.directions.len() != 1 {
            return Err(invalid("parameter direction stack is inconsistent"));
        }
        let direction = self.pop()?;
        if (self.consumed || direction.is_undefined())
            && let Some(error) = self.error.take()
        {
            return Err(error);
        }
        Ok(direction)
    }

    // Finish the nominal traversal and consume its authored random draws even
    // when the derivative is undefined. Only a consumer of this direction
    // should report that error; unrelated parameter definitions remain usable.
    fn push_direction(&mut self, direction: Result<ComplexDirection, ExprError>) {
        match direction {
            Ok(direction) => self.directions.push(direction),
            Err(error) => {
                self.error.get_or_insert(error);
                self.directions.push(ComplexDirection::undefined());
            }
        }
    }
}

fn invalid(message: &str) -> ExprError {
    ExprError::InvalidArgument(message.to_owned())
}

impl<F> PreparedEvaluation for ParameterDirection<'_, F>
where
    F: FnMut(&str) -> Result<Option<(ComplexValue, ComplexDirection)>, ExprError>,
{
    fn resolve(&mut self, name: &str) -> Result<Option<ComplexValue>, ExprError> {
        (self.resolver)(name)?
            .map(|(value, direction)| {
                self.directions.push(direction);
                Ok(value)
            })
            .transpose()
    }

    fn constant(&mut self, value: ComplexValue) -> Result<ComplexValue, ExprError> {
        self.directions.push(ComplexDirection::zero());
        Ok(value)
    }

    fn unary(&mut self, op: UnaryOpKind, value: ComplexValue) -> Result<ComplexValue, ExprError> {
        let direction = self.pop()?;
        self.push_direction(match op {
            UnaryOpKind::Neg => Ok(-direction),
            UnaryOpKind::Pos => Ok(direction),
            // A logical negation is a step: its value is locally constant
            // unless the argument crosses zero here, and only then undefined.
            UnaryOpKind::Not if value == ComplexValue::from(0.0) && !direction.is_zero() => Err(
                invalid("boolean boundary has no two-sided parameter derivative"),
            ),
            UnaryOpKind::Not => Ok(ComplexDirection::zero()),
        });
        Ok(apply_unary(op, value))
    }

    fn binary(
        &mut self,
        op: BinOpKind,
        left: ComplexValue,
        right: ComplexValue,
        dialect: ExpressionDialect,
    ) -> Result<ComplexValue, ExprError> {
        let value = apply_binary(op, left, right, dialect)?;
        let dr = self.pop()?;
        let dl = self.pop()?;
        let difference = dl - dr;
        let boundary = match op {
            BinOpKind::Gt | BinOpKind::Lt | BinOpKind::Ge | BinOpKind::Le => {
                left.re == right.re && difference.re != 0.0
            }
            BinOpKind::Eq | BinOpKind::Ne => {
                let re = (left.re - right.re).abs();
                let im = (left.im - right.im).abs();
                (re == 1e-12 && im <= 1e-12 && difference.re != 0.0)
                    || (im == 1e-12 && re <= 1e-12 && difference.im != 0.0)
            }
            BinOpKind::And | BinOpKind::Or => {
                (left == ComplexValue::from(0.0) && !dl.is_zero())
                    || (right == ComplexValue::from(0.0) && !dr.is_zero())
            }
            BinOpKind::Mod => remainder_boundary(left.re, right.re, dl.re, dr.re),
            _ => false,
        };
        let direction = (|| {
            if boundary {
                return Err(invalid(
                    "expression boundary has no two-sided parameter derivative",
                ));
            }
            let direction = if dl.is_zero() && dr.is_zero() {
                ComplexDirection::zero()
            } else {
                match op {
                    BinOpKind::Add => dl + dr,
                    BinOpKind::Sub => dl - dr,
                    BinOpKind::Mul => dl * right + dr * left,
                    BinOpKind::Div => {
                        (dl * right - dr * left) / (ComplexDirection::from(right) * right)
                    }
                    BinOpKind::Mod => (dl.re - dr.re * (left.re / right.re).trunc()).into(),
                    BinOpKind::Pow => power_direction(left, right, value, dl, dr, false)?,
                    _ => ComplexDirection::zero(),
                }
            };
            Ok(direction)
        })();
        self.push_direction(direction);
        Ok(value)
    }

    fn builtin(
        &mut self,
        name: &str,
        args: &[ComplexValue],
        ctx: &ParamContext,
    ) -> Result<ComplexValue, ExprError> {
        let mut draw = None;
        let value = eval_builtin_with_sample(name, args, ctx, &mut |sample| draw = Some(sample))?;
        let start = self
            .directions
            .len()
            .checked_sub(args.len())
            .ok_or_else(|| invalid("parameter direction argument stack underflow"))?;
        let directions = &self.directions[start..];
        let direction = if directions.iter().all(|direction| direction.is_zero()) {
            Ok(ComplexDirection::zero())
        } else {
            builtin_direction(
                name,
                args,
                directions,
                value,
                ctx.expression_dialect(),
                draw,
            )
        };
        self.directions.truncate(start);
        self.push_direction(direction);
        Ok(value)
    }

    fn folded_builtin(
        &mut self,
        name: &str,
        args: &[ComplexValue],
    ) -> Result<ComplexValue, ExprError> {
        let value = xyce_constant_fold_builtin(name, args)?;
        let start = self
            .directions
            .len()
            .checked_sub(args.len())
            .ok_or_else(|| invalid("parameter direction argument stack underflow"))?;
        self.directions.truncate(start);
        self.directions.push(ComplexDirection::zero());
        Ok(value)
    }

    fn discard_condition(&mut self, condition: ComplexValue) -> Result<(), ExprError> {
        let direction = self.pop()?;
        // The selection is part of the result, so a condition that is itself
        // undefined or is switching here leaves the result undefined. A
        // condition built from an undefined tangent whose own truth is locally
        // constant has already absorbed it and selects one branch throughout.
        if direction.is_undefined()
            || (condition == ComplexValue::from(0.0) && !direction.is_zero())
        {
            self.consumed = true;
            self.error.get_or_insert_with(|| {
                invalid("conditional boundary has no two-sided parameter derivative")
            });
        }
        Ok(())
    }
}

fn power_direction(
    base: ComplexValue,
    exponent: ComplexValue,
    value: ComplexValue,
    db: ComplexDirection,
    de: ComplexDirection,
    native_log: bool,
) -> Result<ComplexDirection, ExprError> {
    if base == ComplexValue::from(0.0) {
        if is_real(exponent) && exponent.re > 0.0 {
            if db.is_zero() || exponent.re > 1.0 {
                return Ok(ComplexDirection::zero());
            }
            if exponent.re == 1.0 && de.is_zero() {
                return Ok(db);
            }
        }
        if exponent == ComplexValue::from(0.0) && de.is_zero() {
            return Ok(ComplexDirection::zero());
        }
        return Err(invalid(
            "power has no finite parameter derivative at a zero base",
        ));
    }
    let logarithm = if native_log {
        base.ln()
    } else {
        complex_ln(base)
    };
    let power = if value.re.is_normal() || value.im.is_normal() {
        ComplexDirection::from(value)
    } else if is_real(base) && is_real(exponent) && exponent.re.fract() == 0.0 {
        let sign = if base.re < 0.0 && exponent.re.rem_euclid(2.0) == 1.0 {
            -1.0
        } else {
            1.0
        };
        ComplexDirection::from(sign)
            .exp_multiply(ComplexValue::from(exponent.re * base.re.abs().ln()))
    } else {
        ComplexDirection::from(1.0).exp_multiply(exponent * logarithm)
    };
    let base_part = if db.is_zero() {
        ComplexDirection::zero()
    } else {
        db * exponent / base
    };
    let exponent_part = if de.is_zero() {
        ComplexDirection::zero()
    } else {
        de * logarithm
    };
    Ok((base_part + exponent_part) * power)
}

fn magnitude_direction(
    value: ComplexValue,
    direction: ComplexDirection,
) -> Result<Derivative, ExprError> {
    if direction.is_zero() {
        return Ok(0.0.into());
    }
    let magnitude = value.norm();
    if magnitude == 0.0 {
        return Err(invalid("magnitude has no two-sided derivative at zero"));
    }
    Ok(direction.re * (value.re / magnitude) + direction.im * (value.im / magnitude))
}

fn remainder_boundary(left: Value, right: Value, dl: Derivative, dr: Derivative) -> bool {
    let quotient = left / right;
    quotient != 0.0 && quotient.fract() == 0.0 && dl - dr * quotient != 0.0
}

fn builtin_direction(
    name: &str,
    a: &[ComplexValue],
    d: &[ComplexDirection],
    value: ComplexValue,
    dialect: ExpressionDialect,
    draw: Option<Value>,
) -> Result<ComplexDirection, ExprError> {
    let z = a.first().copied().unwrap_or_default();
    let dz = d.first().copied().unwrap_or_else(ComplexDirection::zero);
    let one = ComplexValue::from(1.0);
    let zero = ComplexDirection::zero();
    let boundary = match name {
        "FLOOR" | "CEIL" => z.re.fract() == 0.0 && dz.re != 0.0,
        "INT" | "TRUNC" => z.re != 0.0 && z.re.fract() == 0.0 && dz.re != 0.0,
        "ROUND" | "NINT" => z.re.fract().abs() == 0.5 && dz.re != 0.0,
        "EQ0" | "NE0" => z.re.abs() == 1e-12 && dz.re != 0.0,
        "SGN" | "U" | "USTEP" | "URAMP" | "GT0" | "LT0" | "GE0" | "LE0" => {
            z.re == 0.0 && dz.re != 0.0
        }
        "SIGN" if dialect == ExpressionDialect::Xyce => a[1].re == 0.0 && d[1].re != 0.0,
        "SIGN" => z.re == 0.0 && dz.re != 0.0,
        "U2" => (z.re == 0.0 || z.re == 1.0) && dz.re != 0.0,
        "LIMIT" if a.len() == 3 => {
            (a[0].re == a[1].re && d[0].re != d[1].re) || (a[0].re == a[2].re && d[0].re != d[2].re)
        }
        "FMOD" | "MOD" => remainder_boundary(a[0].re, a[1].re, d[0].re, d[1].re),
        "TANH" if dialect == ExpressionDialect::Xyce => z.re.abs() == 20.0 && dz.re != 0.0,
        "ATANH" if dialect == ExpressionDialect::Xyce => z.re.abs() == 1.0 - 1e-12 && dz.re != 0.0,
        _ => false,
    };
    if boundary {
        return Err(invalid(
            "function boundary has no two-sided parameter derivative",
        ));
    }
    Ok(match name {
        "R" | "RE" | "REAL" => dz.re.into(),
        "IMG" | "IMAG" => dz.im.into(),
        "ABS" | "M" | "MAG" => magnitude_direction(z, dz)?.into(),
        "DB" => (magnitude_direction(z, dz)? / z.norm() * (20.0 / std::f64::consts::LN_10)).into(),
        "PH" | "PHASE" => ((dz / z).im * (180.0 / std::f64::consts::PI)).into(),
        "SQRT" => dz / value / 2.0,
        "EXP" => dz.exp_multiply(z),
        "LOG" if dialect == ExpressionDialect::Xyce => dz / z / std::f64::consts::LN_10,
        "LOG" | "LN" => dz / z,
        "LOG10" => dz / z / std::f64::consts::LN_10,
        "SIN" => dz * z.cos(),
        "COS" => -dz * z.sin(),
        "TAN" => dz / z.cos() / z.cos(),
        "SINH" => (dz.exp_multiply(z) + dz.exp_multiply(-z)) / 2.0,
        "COSH" => (dz.exp_multiply(z) - dz.exp_multiply(-z)) / 2.0,
        "TANH" if dialect == ExpressionDialect::Xyce && z.re.abs() > 20.0 => zero,
        "TANH" if is_real(z) => {
            let argument = -2.0 * z.re.abs();
            let denominator = 1.0 + argument.exp();
            (dz * 4.0 / denominator / denominator).exp_multiply(argument.into())
        }
        "TANH" => dz / z.cosh() / z.cosh(),
        "ASIN" => dz / complex_sqrt(one - z * z),
        "ACOS" => -dz / complex_sqrt(one - z * z),
        "ATAN" | "ARCTAN" => dz / (ComplexDirection::from(1.0) + ComplexDirection::from(z) * z),
        "ASINH" if is_real(z) => dz / z.re.hypot(1.0),
        "ASINH" => dz / complex_sqrt(one + z * z),
        "ACOSH" => dz / complex_sqrt(z - one) / complex_sqrt(z + one),
        "ATANH" if dialect == ExpressionDialect::Xyce && z.re.abs() > 1.0 - 1e-12 => zero,
        "ATANH" => dz / (ComplexDirection::from(1.0) - ComplexDirection::from(z) * z),
        "ATAN2" => Derivative::product_ratio(
            [(d[0].re, a[1].re), (-d[1].re, a[0].re)],
            [(a[0].re, a[0].re), (a[1].re, a[1].re)],
        )
        .into(),
        "SQR" => dz * z * 2.0,
        "POW" => power_direction(
            a[0],
            a[1],
            value,
            d[0],
            d[1],
            dialect == ExpressionDialect::Xyce,
        )?,
        "PWR" if dialect == ExpressionDialect::Xyce => {
            power_direction(a[0], a[1], value, d[0], d[1], true)?
        }
        "PWR" => power_direction(
            a[0].re.abs().into(),
            a[1],
            value,
            d[0] * crate::expr::ordered_sign(a[0].re),
            d[1],
            false,
        )?,
        "PWRS" if a[0].re < 0.0 => -power_direction(
            -a[0],
            a[1],
            -value,
            -d[0],
            d[1],
            dialect == ExpressionDialect::Xyce,
        )?,
        "PWRS" => power_direction(
            a[0],
            a[1],
            value,
            d[0],
            d[1],
            dialect == ExpressionDialect::Xyce,
        )?,
        "MIN" | "MAX" => {
            let mut best = 0;
            for index in 1..a.len() {
                if a[best].re.is_nan()
                    || (name == "MIN" && a[index].re < a[best].re)
                    || (name == "MAX" && a[index].re > a[best].re)
                {
                    best = index;
                }
            }
            if a.iter()
                .zip(d)
                .any(|(value, direction)| value.re == a[best].re && *direction != d[best])
            {
                return Err(invalid(
                    "equal min/max arguments have different parameter directions",
                ));
            }
            d[best]
        }
        "LIMIT" if a.len() == 3 => {
            if a[0].re < a[1].re {
                d[1]
            } else if a[0].re > a[2].re {
                d[2]
            } else {
                d[0]
            }
        }
        "LIMIT" => d[0] + d[1] * draw.unwrap_or(0.0),
        "UNIF" | "AUNIF" | "GAUSS" | "AGAUSS" => {
            if let Some(draw) = draw {
                let relative = matches!(name, "UNIF" | "GAUSS");
                let deviation = if relative { a[0].re * a[1].re } else { a[1].re };
                let dd = if relative {
                    d[0] * a[1] + d[1] * a[0]
                } else {
                    d[1]
                };
                let sigma = a.get(2).map_or(1.0, |value| value.re);
                let ds = d.get(2).copied().unwrap_or(zero);
                d[0] + (dd / sigma - ds / sigma * deviation / sigma) * draw
            } else {
                d[0]
            }
        }
        "FMOD" | "MOD" => (d[0].re - d[1].re * (a[0].re / a[1].re).trunc()).into(),
        "TABLE" | "PWL" => table_direction(a, d)?,
        "SIGN" if dialect == ExpressionDialect::Xyce => {
            (magnitude_direction(a[0], d[0])? * crate::expr::ordered_sign(a[1].re)).into()
        }
        "URAMP" => {
            if z.re > 0.0 {
                dz
            } else {
                zero
            }
        }
        "U2" => {
            if z.re > 0.0 && z.re < 1.0 {
                dz
            } else {
                zero
            }
        }
        "FLOOR" | "CEIL" | "ROUND" | "INT" | "TRUNC" | "NINT" | "SGN" | "SIGN" | "U" | "USTEP"
        | "EQ0" | "NE0" | "GT0" | "LT0" | "GE0" | "LE0" | "RAND" | "RANDOM" => zero,
        _ => return Err(ExprError::UnknownFunction(name.to_owned())),
    })
}

fn table_direction(
    a: &[ComplexValue],
    d: &[ComplexDirection],
) -> Result<ComplexDirection, ExprError> {
    let last = 2 * ((a.len() - 1) / 2) - 1;
    let x = a[0].re;
    for index in (1..=last).step_by(2) {
        if x == a[index].re && d[0].re != d[index].re {
            let motion = d[0] - d[index];
            let left = if index == 1 {
                d[index + 1]
            } else {
                d[index + 1]
                    + motion / (a[index].re - a[index - 2].re) * (a[index + 1].re - a[index - 1].re)
            };
            let right = if index == last {
                d[index + 1]
            } else {
                d[index + 1]
                    + motion / (a[index + 2].re - a[index].re) * (a[index + 3].re - a[index + 1].re)
            };
            if left != right {
                return Err(invalid(
                    "table knot has unequal one-sided parameter derivatives",
                ));
            }
        }
    }
    if last == 1 || x <= a[1].re {
        return Ok(d[2]);
    }
    if x >= a[last].re {
        return Ok(d[last + 1]);
    }
    for index in (1..last).step_by(2) {
        let x0 = a[index].re;
        let x1 = a[index + 2].re;
        if x >= x0 && x <= x1 {
            let width = x1 - x0;
            if width.abs() < 1e-18 {
                return Ok(d[index + 1]);
            }
            let weight = (x - x0) / width;
            let dw = (d[0] - d[index] - (d[index + 2] - d[index]) * weight) / width;
            return Ok(d[index + 1]
                + (d[index + 3] - d[index + 1]) * weight
                + dw * (a[index + 3].re - a[index + 1].re));
        }
    }
    Ok(d[last + 1])
}
