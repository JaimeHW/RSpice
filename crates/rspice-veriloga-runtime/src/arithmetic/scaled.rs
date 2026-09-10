//! Binary64 significands with a retained exponent for frequency-domain arithmetic.

use super::sum_products;

/// A binary64 significand with a separate exponent for AC intermediates.
/// Values in the normal binary64 range retain their ordinary representation.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaledValue {
    value: f64,
    exponent: i64,
}

impl ScaledValue {
    #[inline]
    pub fn is_zero(self) -> bool {
        self.value == 0.0
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    #[inline]
    pub fn is_regular(self) -> bool {
        self.exponent == 0
    }

    /// Exponentiation by squaring, retaining the exponent at every product.
    /// At most 32 squarings are needed for generated operator orders.
    pub fn powu(mut self, mut power: u32) -> Self {
        let mut value = Self::new(1.0);
        while power != 0 {
            if power & 1 != 0 {
                value = value.multiply(self);
            }
            power >>= 1;
            if power != 0 {
                self = self.multiply(self);
            }
        }
        value
    }

    #[inline]
    pub fn new(value: f64) -> Self {
        Self { value, exponent: 0 }
    }

    fn normalized(self) -> (f64, i64) {
        if self.value == 0.0 || !self.value.is_finite() {
            return (self.value, 0);
        }
        let (value, adjustment) = if self.value.is_subnormal() {
            (self.value * 18014398509481984.0, -54)
        } else {
            (self.value, 0)
        };
        let bits = value.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i64 - 1023 + adjustment;
        let significand = f64::from_bits((bits & 0x800f_ffff_ffff_ffff) | (1023_u64 << 52));
        match self.exponent.checked_add(exponent) {
            Some(exponent) => (significand, exponent),
            None => (f64::NAN, 0),
        }
    }

    fn scaled(value: f64, exponent: i64) -> Self {
        let (value, exponent) = Self { value, exponent }.normalized();
        if value == 0.0 || !value.is_finite() {
            return Self::new(value);
        }
        if (-1022..=1023).contains(&exponent) {
            return Self::new(value * f64::from_bits(((exponent + 1023) as u64) << 52));
        }
        Self { value, exponent }
    }

    #[inline]
    pub fn binary64(self) -> f64 {
        if self.exponent == 0 || self.value == 0.0 || !self.value.is_finite() {
            return self.value;
        }
        let (value, exponent) = self.normalized();
        if exponent > 1023 {
            return f64::INFINITY.copysign(value);
        }
        if exponent < -1075 {
            return 0.0_f64.copysign(value);
        }
        if exponent < -1022 {
            // The first product is exact; only the final subnormal conversion
            // rounds, including the half-minimum tie at exponent -1075.
            return (value * f64::MIN_POSITIVE)
                * f64::from_bits(((exponent + 1022 + 1023) as u64) << 52);
        }
        value * f64::from_bits(((exponent + 1023) as u64) << 52)
    }

    #[inline]
    pub fn negated(self) -> Self {
        Self {
            value: -self.value,
            ..self
        }
    }

    #[inline]
    pub fn plus(self, other: Self) -> Self {
        if self.exponent == 0 && other.exponent == 0 {
            let sum = self.value + other.value;
            if sum.is_finite() || !self.value.is_finite() || !other.value.is_finite() {
                return Self::new(sum);
            }
        }
        Self::product_sum(self, Self::new(1.0), other, Self::new(1.0))
    }

    #[inline]
    pub fn multiply(self, other: Self) -> Self {
        if self.exponent == 0 && other.exponent == 0 {
            let product = self.value * other.value;
            if product.is_normal()
                || self.value == 0.0
                || other.value == 0.0
                || !self.value.is_finite()
                || !other.value.is_finite()
            {
                return Self::new(product);
            }
        }
        let (a, ae) = self.normalized();
        let (b, be) = other.normalized();
        match ae.checked_add(be) {
            Some(exponent) => Self::scaled(a * b, exponent),
            None => Self::new(f64::NAN),
        }
    }

    #[inline]
    pub fn multiply_binary64(self, other: f64) -> f64 {
        // This product is the final conversion boundary, so an ordinary
        // stored factor needs no intermediate range recovery.
        if self.exponent == 0 {
            self.value * other
        } else {
            self.wide_multiply_binary64(other)
        }
    }

    #[cold]
    #[inline(never)]
    fn wide_multiply_binary64(self, other: f64) -> f64 {
        self.multiply(Self::new(other)).binary64()
    }

    #[inline]
    pub fn divide(self, other: Self) -> Self {
        if self.exponent == 0 && other.exponent == 0 {
            let quotient = self.value / other.value;
            if quotient.is_normal()
                || self.value == 0.0
                || other.value == 0.0
                || !self.value.is_finite()
                || !other.value.is_finite()
            {
                return Self::new(quotient);
            }
        }
        let (a, ae) = self.normalized();
        let (b, be) = other.normalized();
        match ae.checked_sub(be) {
            Some(exponent) => Self::scaled(a / b, exponent),
            None => Self::new(f64::NAN),
        }
    }

    #[inline]
    pub fn product_sum(a: Self, b: Self, c: Self, d: Self) -> Self {
        if (a.exponent | b.exponent | c.exponent | d.exponent) == 0
            && let Ok(value) = sum_products([(a.value, b.value), (c.value, d.value)].into_iter())
            && (value.is_normal()
                || ((a.value == 0.0 || b.value == 0.0) && (c.value == 0.0 || d.value == 0.0)))
        {
            return Self::new(value);
        }
        Self::wide_product_sum(a, b, c, d)
    }

    #[cold]
    #[inline(never)]
    fn wide_product_sum(a: Self, b: Self, c: Self, d: Self) -> Self {
        if [a, b, c, d].iter().any(|x| !x.value.is_finite()) {
            return Self::new(a.binary64() * b.binary64() + c.binary64() * d.binary64());
        }
        let (a, ae) = a.normalized();
        let (b, be) = b.normalized();
        let (c, ce) = c.normalized();
        let (d, de) = d.normalized();
        let (Some(ab), Some(cd)) = (ae.checked_add(be), ce.checked_add(de)) else {
            return Self::new(f64::NAN);
        };
        let common = if a == 0.0 || b == 0.0 {
            cd
        } else if c == 0.0 || d == 0.0 {
            ab
        } else {
            ab.max(cd)
        };
        let align = |value: f64, exponent: i64| {
            if value == 0.0 {
                return value;
            }
            let shift = exponent.saturating_sub(common);
            if shift < -1074 {
                0.0_f64.copysign(value)
            } else if shift < -1022 {
                value * f64::from_bits(1_u64 << (shift + 1074))
            } else {
                value * f64::from_bits(((shift + 1023) as u64) << 52)
            }
        };
        let first = if a == 0.0 || b == 0.0 {
            (a * b, 1.0)
        } else {
            (align(a, ab), b)
        };
        let second = if c == 0.0 || d == 0.0 {
            (c * d, 1.0)
        } else {
            (align(c, cd), d)
        };
        let value = sum_products([first, second].into_iter()).unwrap_or(f64::NAN);
        Self::scaled(value, common)
    }
}
