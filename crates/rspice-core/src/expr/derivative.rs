//! Forward derivatives retain their exponent until the expression boundary.
//! Primal expression values continue to follow the selected binary64 dialect.

use crate::Value;
use rspice_veriloga_runtime::arithmetic::ScaledValue;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Derivative(ScaledValue);

impl Derivative {
    #[inline]
    pub(crate) fn binary64(self) -> Value {
        self.0.binary64()
    }

    #[inline]
    pub(crate) fn abs(self) -> Self {
        if self.binary64().is_sign_negative() {
            -self
        } else {
            self
        }
    }

    #[inline]
    pub(crate) fn max(self, other: Value) -> Self {
        if self.binary64().is_nan() || self < other {
            other.into()
        } else {
            self
        }
    }

    /// Apply an exponential without rounding its coefficient to zero or
    /// infinity before it reaches the retained incoming derivative.
    pub(crate) fn multiply_exp(self, argument: Value) -> Self {
        let ordinary = argument.exp();
        if ordinary.is_normal() || !argument.is_finite() {
            return self * ordinary;
        }
        let exponent = (argument / std::f64::consts::LN_2).floor();
        if exponent.abs() >= i64::MAX as Value {
            return self * ordinary;
        }
        // The low part of ln(2) prevents reduction error from growing with
        // the exponent. mul_add preserves the cancellation in the high part.
        const LN_2_LOW: Value = 2.319_046_813_846_299_6e-17;
        let remainder = (-exponent).mul_add(std::f64::consts::LN_2, argument) - exponent * LN_2_LOW;
        let magnitude = exponent.abs() as u64;
        let mut scale = ScaledValue::new(2.0).powu(magnitude as u32);
        if magnitude >> 32 != 0 {
            let high = ScaledValue::new(2.0)
                .powu(1 << 31)
                .powu(2)
                .powu((magnitude >> 32) as u32);
            scale = scale.multiply(high);
        }
        let incoming = if exponent < 0.0 {
            self.0.divide(scale)
        } else {
            self.0.multiply(scale)
        };
        Self(incoming.multiply(ScaledValue::new(remainder.exp())))
    }

    pub(crate) fn product_ratio(
        numerator: [(Self, Value); 2],
        denominator: [(Value, Value); 2],
    ) -> Self {
        let ordinary = denominator[0].0 * denominator[0].1 + denominator[1].0 * denominator[1].1;
        let divisor = if ordinary.is_normal() {
            ScaledValue::new(ordinary)
        } else {
            ScaledValue::product_sum(
                ScaledValue::new(denominator[0].0),
                ScaledValue::new(denominator[0].1),
                ScaledValue::new(denominator[1].0),
                ScaledValue::new(denominator[1].1),
            )
        };
        if !divisor.is_finite()
            || numerator
                .iter()
                .any(|(d, factor)| !d.0.is_finite() || !factor.is_finite())
        {
            return (numerator[0].0 * numerator[0].1 + numerator[1].0 * numerator[1].1)
                / Self(divisor);
        }
        Self(
            ScaledValue::sum_products_div(
                numerator
                    .into_iter()
                    .map(|(d, factor)| [d.0, ScaledValue::new(factor)]),
                divisor,
            )
            .unwrap_or_else(|_| ScaledValue::new(Value::NAN)),
        )
    }
}

#[inline]
pub(crate) fn derivative_pair(
    value: Value,
    derivative: impl Into<Derivative>,
) -> Option<(Value, Derivative)> {
    Some((value, derivative.into()))
}

impl From<Value> for Derivative {
    #[inline]
    fn from(value: Value) -> Self {
        Self(ScaledValue::new(value))
    }
}
impl Neg for Derivative {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.negated())
    }
}
impl Add for Derivative {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.plus(rhs.0))
    }
}
impl AddAssign for Derivative {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Derivative {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl Mul for Derivative {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.multiply(rhs.0))
    }
}
impl Div for Derivative {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.0.divide(rhs.0))
    }
}
impl Mul<Value> for Derivative {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Value) -> Self {
        self * Self::from(rhs)
    }
}
impl Div<Value> for Derivative {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Value) -> Self {
        self / Self::from(rhs)
    }
}
impl Mul<Derivative> for Value {
    type Output = Derivative;
    #[inline]
    fn mul(self, rhs: Derivative) -> Derivative {
        rhs * self
    }
}
impl PartialEq for Derivative {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 || (*self - *rhs).0.is_zero()
    }
}
impl PartialOrd for Derivative {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self == rhs {
            return Some(Ordering::Equal);
        }
        let difference = (*self - *rhs).binary64();
        if difference.is_nan() {
            None
        } else if difference.is_sign_negative() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl PartialEq<Value> for Derivative {
    #[inline]
    fn eq(&self, rhs: &Value) -> bool {
        *self == Self::from(*rhs)
    }
}
impl PartialOrd<Value> for Derivative {
    #[inline]
    fn partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*rhs))
    }
}
