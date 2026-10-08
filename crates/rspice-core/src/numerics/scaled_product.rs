//! Retain a real product's binary exponent until its consumer rounds to f64.
use crate::Value;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScaledProduct {
    mantissa: Value,
    exponent: i64,
}

impl ScaledProduct {
    pub(crate) const ZERO: Self = Self {
        mantissa: 0.0,
        exponent: 0,
    };
    pub(crate) const ONE: Self = Self {
        mantissa: 1.0,
        exponent: 0,
    };

    pub(crate) fn multiply(self, factor: Value) -> Self {
        let (factor, exponent) = libm::frexp(factor);
        self.multiply_scaled(Self {
            mantissa: factor,
            exponent: i64::from(exponent),
        })
    }

    /// Multiply two retained products without materializing their full values.
    pub(crate) fn multiply_scaled(self, factor: Self) -> Self {
        let (mantissa, normalization) = libm::frexp(self.mantissa * factor.mantissa);
        Self {
            mantissa,
            exponent: self.exponent + factor.exponent + i64::from(normalization),
        }
    }

    /// Remove a known nonzero factor without materializing the full product.
    pub(crate) fn without_factor(self, factor: Value) -> Self {
        let (factor, exponent) = libm::frexp(factor);
        self.divide(Self {
            mantissa: factor,
            exponent: i64::from(exponent),
        })
    }

    pub(crate) fn divide(self, divisor: Self) -> Self {
        let (mantissa, normalization) = libm::frexp(self.mantissa / divisor.mantissa);
        Self {
            mantissa,
            exponent: self.exponent - divisor.exponent + i64::from(normalization),
        }
    }

    pub(crate) fn value(self) -> Value {
        scale(self.mantissa, self.exponent)
    }

    /// Include an additive offset before rounding: a product above f64's
    /// range can cancel against a finite offset to yield a finite result.
    pub(crate) fn plus(self, offset: Value) -> Value {
        self.add(Self::ONE.multiply(offset)).value()
    }

    pub(crate) fn add(self, other: Self) -> Self {
        if self.mantissa == 0.0 {
            return other;
        }
        if other.mantissa == 0.0 {
            return self;
        }
        let common = self.exponent.max(other.exponent);
        let (mantissa, normalization) = libm::frexp(
            scale(self.mantissa, self.exponent - common)
                + scale(other.mantissa, other.exponent - common),
        );
        Self {
            mantissa,
            exponent: common + i64::from(normalization),
        }
    }
}

fn scale(value: Value, exponent: i64) -> Value {
    libm::scalbn(
        value,
        exponent.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    )
}
