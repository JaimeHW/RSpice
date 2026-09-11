//! Binary64 significands with a retained exponent for frequency-domain arithmetic.

use super::scalar::{ArithmeticError, BigMagnitude, ExactValue, exact_f64_parts, sum_products};

/// A binary64 significand with a separate exponent for AC intermediates.
/// Values in the normal binary64 range retain their ordinary representation.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaledValue {
    value: f64,
    exponent: i64,
}

impl ScaledValue {
    /// Preserve the ordinary derivative numerator's rounding when it stays in
    /// range, then retain the quotient exponent for subsequent AC operations.
    pub fn sum_products_div(
        terms: impl ExactSizeIterator<Item = [Self; 2]> + Clone,
        divisor: Self,
    ) -> Result<Self, ArithmeticError> {
        if terms
            .clone()
            .all(|pair| pair.iter().all(|factor| factor.is_regular()))
            && let Some(sum) =
                super::scalar::ordinary_sum_products(terms.clone().map(|[a, b]| [a.value, b.value]))
        {
            return Ok(Self::new(sum).divide(divisor));
        }
        let one = Self::new(1.0);
        Self::sum_triple_products_ratio(
            terms.map(|[a, b]| [a, b, one]),
            [[divisor, one, one]].into_iter(),
        )
    }

    /// Divide complete sums of triple products without narrowing their factors.
    /// Recovery retains cancellation across terms and independent exponents.
    /// An excessive exponent span is reported before allocating an accumulator.
    pub fn sum_triple_products_ratio(
        numerator: impl Iterator<Item = [Self; 3]> + Clone,
        denominator: impl Iterator<Item = [Self; 3]> + Clone,
    ) -> Result<Self, ArithmeticError> {
        let (numerator, numerator_floor) = Self::exact_sum(numerator)?;
        let (denominator, denominator_floor) = Self::exact_sum(denominator)?;
        let Some(denominator_top) = denominator.magnitude.top_bit() else {
            return Err(ArithmeticError::ZeroDenominator);
        };
        let Some(numerator_top) = numerator.magnitude.top_bit() else {
            return Ok(Self::new(0.0));
        };
        let adjustment = numerator_top as i32 - denominator_top as i32;
        let value = super::scalar::scaled_exact_ratio_to_f64(
            &numerator.magnitude,
            &denominator.magnitude,
            numerator.negative ^ denominator.negative,
            -adjustment,
        )?;
        let exponent = numerator_floor
            .checked_sub(denominator_floor)
            .and_then(|exponent| exponent.checked_add(i64::from(adjustment)))
            .ok_or(ArithmeticError::MantissaBounds)?;
        Ok(Self::scaled(value, exponent))
    }

    fn exact_sum(
        terms: impl Iterator<Item = [Self; 3]> + Clone,
    ) -> Result<(ExactValue, i64), ArithmeticError> {
        // This is a cold recovery path. Bound storage independently of an
        // adversarial operator order; never clamp or discard a distant term.
        const MAX_SPAN_BITS: u32 = 65_536;
        let mut floor = i64::MAX;
        let mut ceiling = i64::MIN;
        for factors in terms.clone() {
            if let Some((_, _, exponent)) = Self::exact_term(factors)? {
                floor = floor.min(exponent);
                ceiling = ceiling.max(exponent);
            }
        }
        if floor != i64::MAX
            && ceiling
                .checked_sub(floor)
                .is_none_or(|span| span > i64::from(MAX_SPAN_BITS) - 159)
        {
            return Err(ArithmeticError::PrecisionLimit {
                bits: MAX_SPAN_BITS,
            });
        }
        let mut positive = BigMagnitude::default();
        let mut negative = BigMagnitude::default();
        for factors in terms {
            let Some((sign, mantissas, exponent)) = Self::exact_term(factors)? else {
                continue;
            };
            let pair = u128::from(mantissas[0]) * u128::from(mantissas[1]);
            let third = u128::from(mantissas[2]);
            let shift = (exponent - floor) as usize;
            let accumulator = if sign { &mut negative } else { &mut positive };
            accumulator.add_shifted(u128::from(pair as u64) * third, shift);
            accumulator.add_shifted((pair >> 64) * third, shift + 64);
        }
        let (negative_result, magnitude) = match positive.compare(&negative) {
            std::cmp::Ordering::Greater => (false, positive.subtract(&negative)),
            std::cmp::Ordering::Less => (true, negative.subtract(&positive)),
            std::cmp::Ordering::Equal => (false, BigMagnitude::default()),
        };
        Ok((
            ExactValue {
                negative: negative_result,
                magnitude,
            },
            floor,
        ))
    }

    fn exact_term(factors: [Self; 3]) -> Result<Option<(bool, [u64; 3], i64)>, ArithmeticError> {
        if factors.iter().any(|factor| !factor.is_finite()) {
            return Err(ArithmeticError::NonFiniteTerm);
        }
        if factors.iter().any(|factor| factor.is_zero()) {
            return Ok(None);
        }
        let mut sign = false;
        let mut exponent = 0_i64;
        let mut mantissas = [0; 3];
        for (factor, mantissa) in factors.into_iter().zip(&mut mantissas) {
            let (negative, bits, power) = exact_f64_parts(factor.value);
            let trailing = bits.trailing_zeros();
            *mantissa = bits >> trailing;
            exponent = exponent
                .checked_add(factor.exponent)
                .and_then(|exponent| exponent.checked_add(i64::from(power) + i64::from(trailing)))
                .ok_or(ArithmeticError::MantissaBounds)?;
            sign ^= negative;
        }
        Ok(Some((sign, mantissas, exponent)))
    }

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

    pub(super) fn scaled(value: f64, exponent: i64) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotient_recovery_retains_distant_canceling_terms() {
        let one = ScaledValue::new(1.0);
        let huge = ScaledValue::new(1e200).multiply(ScaledValue::new(1e200));
        let tiny = ScaledValue::new(1e-200).multiply(ScaledValue::new(1e-200));
        for values in [
            [huge, tiny, huge.negated()],
            [tiny, huge.negated(), huge],
            [huge, huge.negated(), tiny],
        ] {
            let value =
                ScaledValue::sum_products_div(values.into_iter().map(|value| [value, one]), tiny)
                    .unwrap();
            assert_eq!(value.binary64(), 1.0);
        }
        let value = ScaledValue::sum_triple_products_ratio(
            [
                [huge, one, tiny],
                [tiny, one, tiny],
                [huge.negated(), one, tiny],
            ]
            .into_iter(),
            [[tiny, tiny, one]].into_iter(),
        )
        .unwrap();
        assert_eq!(value.binary64(), 1.0);
    }

    #[test]
    fn quotient_recovery_preserves_ordinary_rounding_and_reports_limits() {
        let one = ScaledValue::new(1.0);
        let value = ScaledValue::sum_products_div(
            [
                [ScaledValue::new(2.0), one],
                [ScaledValue::new(-2.0 / 3.0), ScaledValue::new(3.0)],
            ]
            .into_iter(),
            ScaledValue::new(1e-309),
        )
        .unwrap();
        assert_eq!(value.binary64(), 0.0);
        let huge = ScaledValue::new(2.0).powu(1_000_000);
        assert_eq!(
            ScaledValue::sum_triple_products_ratio(
                [[huge, one, one], [one, one, one]].into_iter(),
                [[one, one, one]].into_iter(),
            ),
            Err(ArithmeticError::PrecisionLimit { bits: 65_536 })
        );
        assert_eq!(
            ScaledValue::sum_triple_products_ratio(
                [[one, one, one]].into_iter(),
                [[ScaledValue::new(0.0), one, one]].into_iter(),
            ),
            Err(ArithmeticError::ZeroDenominator)
        );
        assert_eq!(
            ScaledValue::sum_triple_products_ratio(
                [[ScaledValue::new(f64::INFINITY), one, one]].into_iter(),
                [[one, one, one]].into_iter(),
            ),
            Err(ArithmeticError::NonFiniteTerm)
        );
    }
}
