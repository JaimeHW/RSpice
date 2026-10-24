//! Three-voltage forward derivatives shared by native MOS equations.

use crate::Value;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub(super) struct Dual3 {
    pub(super) value: Value,
    pub(super) derivative: [Value; 3],
}

impl Dual3 {
    #[inline]
    pub(super) fn constant(value: Value) -> Self {
        Self {
            value,
            derivative: [0.0; 3],
        }
    }

    #[inline]
    pub(super) fn variable(value: Value, index: usize) -> Self {
        let mut derivative = [0.0; 3];
        derivative[index] = 1.0;
        Self { value, derivative }
    }

    #[inline]
    pub(super) fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        if value > 0.0 && value.is_finite() {
            self.map_unary(value, 0.5 / value)
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    pub(super) fn powf(self, exponent: Value) -> Self {
        let value = self.value.powf(exponent);
        if self.value > 0.0 && value.is_finite() {
            self.map_unary(value, exponent * self.value.powf(exponent - 1.0))
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    pub(super) fn exp(self) -> Self {
        let value = self.value.exp();
        if value.is_finite() {
            self.map_unary(value, value)
        } else {
            Self::constant(value)
        }
    }

    #[inline]
    pub(super) fn max_const(self, floor: Value) -> Self {
        if self.value > floor {
            self
        } else {
            Self::constant(floor)
        }
    }

    #[inline]
    pub(super) fn max(self, other: Self) -> Self {
        if self.value >= other.value {
            self
        } else {
            other
        }
    }

    #[inline]
    pub(super) fn map_unary(self, value: Value, scale: Value) -> Self {
        Self {
            value,
            derivative: [
                self.derivative[0] * scale,
                self.derivative[1] * scale,
                self.derivative[2] * scale,
            ],
        }
    }

    #[inline]
    pub(super) fn sanitized_derivative(self, index: usize) -> Value {
        let value = self.derivative[index];
        if value.is_finite() { value } else { 0.0 }
    }
}

impl Add for Dual3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            derivative: [
                self.derivative[0] + rhs.derivative[0],
                self.derivative[1] + rhs.derivative[1],
                self.derivative[2] + rhs.derivative[2],
            ],
        }
    }
}

impl Add<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value + rhs,
            derivative: self.derivative,
        }
    }
}

impl Add<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn add(self, rhs: Dual3) -> Self::Output {
        rhs + self
    }
}

impl Sub for Dual3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            derivative: [
                self.derivative[0] - rhs.derivative[0],
                self.derivative[1] - rhs.derivative[1],
                self.derivative[2] - rhs.derivative[2],
            ],
        }
    }
}

impl Sub<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value - rhs,
            derivative: self.derivative,
        }
    }
}

impl Sub<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn sub(self, rhs: Dual3) -> Self::Output {
        Dual3 {
            value: self - rhs.value,
            derivative: [-rhs.derivative[0], -rhs.derivative[1], -rhs.derivative[2]],
        }
    }
}

impl Mul for Dual3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            derivative: [
                self.derivative[0] * rhs.value + self.value * rhs.derivative[0],
                self.derivative[1] * rhs.value + self.value * rhs.derivative[1],
                self.derivative[2] * rhs.value + self.value * rhs.derivative[2],
            ],
        }
    }
}

impl Mul<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value * rhs,
            derivative: [
                self.derivative[0] * rhs,
                self.derivative[1] * rhs,
                self.derivative[2] * rhs,
            ],
        }
    }
}

impl Mul<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn mul(self, rhs: Dual3) -> Self::Output {
        rhs * self
    }
}

impl Div for Dual3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.value * rhs.value;
        Self {
            value: self.value / rhs.value,
            derivative: [
                (self.derivative[0] * rhs.value - self.value * rhs.derivative[0]) / denominator,
                (self.derivative[1] * rhs.value - self.value * rhs.derivative[1]) / denominator,
                (self.derivative[2] * rhs.value - self.value * rhs.derivative[2]) / denominator,
            ],
        }
    }
}

impl Div<Value> for Dual3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Value) -> Self::Output {
        Self {
            value: self.value / rhs,
            derivative: [
                self.derivative[0] / rhs,
                self.derivative[1] / rhs,
                self.derivative[2] / rhs,
            ],
        }
    }
}

impl Div<Dual3> for Value {
    type Output = Dual3;

    #[inline]
    fn div(self, rhs: Dual3) -> Self::Output {
        Dual3::constant(self) / rhs
    }
}

impl Neg for Dual3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            derivative: [
                -self.derivative[0],
                -self.derivative[1],
                -self.derivative[2],
            ],
        }
    }
}
