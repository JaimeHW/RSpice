//! Retain a real product's binary exponent until its consumer rounds to f64.
use crate::Value;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScaledProduct {
    mantissa: Value,
    exponent: i64,
}

impl ScaledProduct {
    pub(crate) const ONE: Self = Self {
        mantissa: 1.0,
        exponent: 0,
    };

    pub(crate) fn multiply(self, factor: Value) -> Self {
        let (factor, exponent) = libm::frexp(factor);
        let (mantissa, normalization) = libm::frexp(self.mantissa * factor);
        Self {
            mantissa,
            exponent: self.exponent + i64::from(exponent) + i64::from(normalization),
        }
    }

    /// Remove a known nonzero factor without materializing the full product.
    pub(crate) fn without_factor(self, factor: Value) -> Self {
        let (factor, exponent) = libm::frexp(factor);
        Self {
            mantissa: self.mantissa / factor,
            exponent: self.exponent - i64::from(exponent),
        }
    }

    pub(crate) fn value(self) -> Value {
        scale(self.mantissa, self.exponent)
    }

    /// Include an additive offset before rounding: a product above f64's
    /// range can cancel against a finite offset to yield a finite result.
    pub(crate) fn plus(self, offset: Value) -> Value {
        if self.mantissa == 0.0 {
            return offset;
        }
        if offset == 0.0 {
            return self.value();
        }
        let (mantissa, exponent) = libm::frexp(offset);
        let exponent = i64::from(exponent);
        let common = self.exponent.max(exponent);
        scale(
            scale(self.mantissa, self.exponent - common) + scale(mantissa, exponent - common),
            common,
        )
    }
}

fn scale(value: Value, exponent: i64) -> Value {
    libm::scalbn(
        value,
        exponent.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    )
}
