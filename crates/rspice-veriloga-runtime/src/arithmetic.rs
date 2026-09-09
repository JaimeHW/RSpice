//! Exact sums of binary64 products with one final round-to-nearest-even conversion.
//! Shared by sampled filters and derivative evaluation; intermediate products
//! may exceed the binary64 range without discarding a finite final result.

/// Failure to represent an exact arithmetic result as a finite binary64 value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    NonFiniteTerm,
    ZeroDenominator,
    Overflow { negative: bool },
    MantissaBounds,
}

/// Evaluate `(a * b) / (c * d)` with one final binary64 rounding.
/// Finite, nonzero factors never overflow or underflow an intermediate product.
/// Zero numerators retain the quotient sign. Zero denominator factors and
/// non-finite inputs follow ordinary IEEE products/division; a finite exact
/// result outside binary64 becomes a signed infinity.
#[inline]
pub fn product_ratio(a: f64, b: f64, c: f64, d: f64) -> f64 {
    if !a.is_finite() || !b.is_finite() || !c.is_finite() || !d.is_finite() || c == 0.0 || d == 0.0
    {
        return (a * b) / (c * d);
    }
    let sign = (a.to_bits() ^ b.to_bits() ^ c.to_bits() ^ d.to_bits()) & (1_u64 << 63);
    if a == 0.0 || b == 0.0 {
        return f64::from_bits(sign);
    }
    match sum_products_ratio([(a, b)].into_iter(), [(c, d)].into_iter()) {
        Ok(value) => value,
        Err(ArithmeticError::Overflow { .. }) => f64::from_bits(sign | f64::INFINITY.to_bits()),
        Err(_) => f64::NAN,
    }
}

/// Allocation-free exact representation for the common case where a sum of
/// binary64 products fits a normalized signed 128-bit integer. Unsupported
/// exponent spans or carries fall back to `BigMagnitude`; they never round in
/// this representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SmallExact {
    value: i128,
    exponent: i32,
}

impl SmallExact {
    const ZERO: Self = Self {
        value: 0,
        exponent: 0,
    };

    fn normalized(value: i128, exponent: i32) -> Option<Self> {
        if value == 0 {
            return Some(Self::ZERO);
        }
        let trailing = value.unsigned_abs().trailing_zeros();
        Some(Self {
            value: value >> trailing,
            exponent: exponent.checked_add(trailing as i32)?,
        })
    }

    fn product(left: f64, right: f64) -> Option<Self> {
        let (left_negative, left_mantissa, left_exponent) = exact_f64_parts(left);
        let (right_negative, right_mantissa, right_exponent) = exact_f64_parts(right);
        let magnitude = u128::from(left_mantissa) * u128::from(right_mantissa);
        let magnitude = i128::try_from(magnitude).ok()?;
        let value = if left_negative ^ right_negative {
            magnitude.checked_neg()?
        } else {
            magnitude
        };
        Self::normalized(value, left_exponent.checked_add(right_exponent)?)
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        if self.value == 0 {
            return Some(other);
        }
        if other.value == 0 {
            return Some(self);
        }
        let common = self.exponent.min(other.exponent);
        let left_shift = u32::try_from(self.exponent.checked_sub(common)?).ok()?;
        let right_shift = u32::try_from(other.exponent.checked_sub(common)?).ok()?;
        let left = checked_scale_i128(self.value, left_shift)?;
        let right = checked_scale_i128(other.value, right_shift)?;
        Self::normalized(left.checked_add(right)?, common)
    }

    fn signed_parts(self) -> (bool, u128, i32) {
        (self.value < 0, self.value.unsigned_abs(), self.exponent)
    }
}

fn checked_scale_i128(value: i128, shift: u32) -> Option<i128> {
    if value == 0 {
        return Some(0);
    }
    if shift >= 127 {
        return None;
    }
    value.checked_mul(1_i128 << shift)
}

fn checked_shift_u128(value: u128, shift: u32) -> Option<u128> {
    if value == 0 {
        return Some(0);
    }
    if shift >= u128::BITS || shift > value.leading_zeros() {
        return None;
    }
    Some(value << shift)
}

fn small_sum_products(
    terms: impl IntoIterator<Item = (f64, f64)>,
) -> Result<Option<SmallExact>, ArithmeticError> {
    let mut accumulator = SmallExact::ZERO;
    for (left, right) in terms {
        if !left.is_finite() || !right.is_finite() {
            return Err(ArithmeticError::NonFiniteTerm);
        }
        if left == 0.0 || right == 0.0 {
            continue;
        }
        let Some(term) = SmallExact::product(left, right) else {
            return Ok(None);
        };
        let Some(sum) = accumulator.checked_add(term) else {
            return Ok(None);
        };
        accumulator = sum;
    }
    Ok(Some(accumulator))
}

fn rounded_small_ratio(numerator: u128, denominator: u128, binary_scale: i32) -> Option<u64> {
    if binary_scale < 0 {
        let shift = binary_scale.unsigned_abs();
        let Some(divisor) = checked_shift_u128(denominator, shift) else {
            // The scaled denominator exceeds every possible numerator. Only
            // its half can affect rounding of this zero integer quotient.
            return Some(u64::from(
                checked_shift_u128(denominator, shift - 1).is_some_and(|half| numerator > half),
            ));
        };
        return round_small_quotient(numerator / divisor, numerator % divisor, divisor);
    }

    if let Some(scaled) = checked_shift_u128(numerator, binary_scale as u32) {
        return round_small_quotient(scaled / denominator, scaled % denominator, denominator);
    }

    // A product contains up to 106 significant bits, so shifting it to obtain
    // 53 quotient bits need not fit u128. Divide before shifting and carry the
    // exact remainder one bit at a time. SmallExact denominators are at most
    // 2^127, which leaves a carry bit for every doubled remainder.
    let mut quotient = u64::try_from(numerator / denominator).ok()?;
    let mut remainder = numerator % denominator;
    for _ in 0..binary_scale {
        quotient = quotient.checked_mul(2)?;
        remainder = remainder.checked_mul(2)?;
        if remainder >= denominator {
            remainder -= denominator;
            quotient = quotient.checked_add(1)?;
        }
    }
    round_small_quotient(u128::from(quotient), remainder, denominator)
}

fn round_small_quotient(quotient: u128, remainder: u128, denominator: u128) -> Option<u64> {
    let complement = denominator - remainder;
    let round_up = remainder > complement || (remainder == complement && quotient & 1 != 0);
    u64::try_from(quotient.checked_add(u128::from(round_up))?).ok()
}

fn small_exact_ratio_to_f64(
    numerator: SmallExact,
    denominator: SmallExact,
) -> Option<Result<f64, ArithmeticError>> {
    if denominator.value == 0 {
        return None;
    }
    if numerator.value == 0 {
        return Some(Ok(0.0));
    }
    let (numerator_negative, numerator, numerator_exponent) = numerator.signed_parts();
    let (denominator_negative, denominator, denominator_exponent) = denominator.signed_parts();
    let numerator_top = (u128::BITS - 1 - numerator.leading_zeros()) as i32;
    let denominator_top = (u128::BITS - 1 - denominator.leading_zeros()) as i32;
    let mut ratio_exponent = numerator_top - denominator_top;
    let below_candidate = if ratio_exponent >= 0 {
        let shifted = checked_shift_u128(denominator, ratio_exponent as u32)?;
        numerator < shifted
    } else {
        let shifted = checked_shift_u128(numerator, ratio_exponent.unsigned_abs())?;
        shifted < denominator
    };
    if below_candidate {
        ratio_exponent -= 1;
    }
    let binary_exponent = numerator_exponent.checked_sub(denominator_exponent)?;
    let unbiased = binary_exponent.checked_add(ratio_exponent)?;
    let scale = if unbiased < -1022 {
        binary_exponent.checked_add(1074)?
    } else {
        52_i32.checked_sub(ratio_exponent)?
    };
    let mut quotient = rounded_small_ratio(numerator, denominator, scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent = output_exponent.checked_add(1)?;
    }
    if unbiased < -1022 && quotient > 1_u64 << 52 {
        return None;
    }
    Some(encode_f64(
        quotient,
        output_exponent,
        numerator_negative ^ denominator_negative,
    ))
}

/// Divide two exact sums of products, rounding only the complete ratio.
/// Exact cancellation returns positive zero; nonzero underflow retains its sign.
pub fn sum_products_ratio<I, D>(
    numerator_terms: I,
    denominator_terms: D,
) -> Result<f64, ArithmeticError>
where
    I: Iterator<Item = (f64, f64)> + Clone,
    D: Iterator<Item = (f64, f64)> + Clone,
{
    let small_numerator = small_sum_products(numerator_terms.clone())?;
    let small_denominator = small_sum_products(denominator_terms.clone())?;
    if let (Some(numerator), Some(denominator)) = (small_numerator, small_denominator) {
        if denominator.value == 0 {
            return Err(ArithmeticError::ZeroDenominator);
        }
        if let Some(result) = small_exact_ratio_to_f64(numerator, denominator) {
            return result;
        }
    }

    let numerator = scaled_sum_products(numerator_terms)?;
    let denominator = scaled_sum_products(denominator_terms)?;
    checked_exact_ratio(numerator, denominator)
}

/// Whether an exact sum of finite products cancels to zero, before rounding.
/// This distinguishes cancellation from a nonzero result rounded below binary64.
pub fn sum_products_is_zero<I>(terms: I) -> Result<bool, ArithmeticError>
where
    I: Iterator<Item = (f64, f64)> + Clone,
{
    if let Some(exact) = small_sum_products(terms.clone())? {
        return Ok(exact.value == 0);
    }
    Ok(scaled_sum_products(terms)?.magnitude.is_zero())
}

/// Sum exact products, rounding once after cancellation.
pub fn sum_products<I>(terms: I) -> Result<f64, ArithmeticError>
where
    I: Iterator<Item = (f64, f64)> + Clone,
{
    if let Some(exact) = small_sum_products(terms.clone())?
        && let Some(result) = small_exact_ratio_to_f64(
            exact,
            SmallExact {
                value: 1,
                exponent: 0,
            },
        )
    {
        return result;
    }

    let exact = scaled_sum_products(terms)?;
    exact_binary_to_f64(&exact.magnitude, exact.negative, -2148)
}

#[derive(Debug, Clone)]
struct ExactValue {
    negative: bool,
    magnitude: BigMagnitude,
}

fn scaled_sum_products(
    terms: impl IntoIterator<Item = (f64, f64)>,
) -> Result<ExactValue, ArithmeticError> {
    // Every finite f64 is an integer mantissa times a power of two. Products
    // therefore fit an exact signed integer accumulator when all exponents are
    // referred to the smallest possible product exponent (-1074 * 2). Keeping
    // positive and negative magnitudes separately prevents a huge pair from
    // erasing a representable residual before cancellation.
    const PRODUCT_EXPONENT_FLOOR: i32 = -2148;
    let mut positive = BigMagnitude::default();
    let mut negative = BigMagnitude::default();
    for (left, right) in terms {
        if !left.is_finite() || !right.is_finite() {
            return Err(ArithmeticError::NonFiniteTerm);
        }
        if left == 0.0 || right == 0.0 {
            continue;
        }
        let (left_negative, left_mantissa, left_exponent) = exact_f64_parts(left);
        let (right_negative, right_mantissa, right_exponent) = exact_f64_parts(right);
        let product = u128::from(left_mantissa) * u128::from(right_mantissa);
        let shift = left_exponent + right_exponent - PRODUCT_EXPONENT_FLOOR;
        debug_assert!(shift >= 0);
        if left_negative ^ right_negative {
            negative.add_shifted(product, shift as usize);
        } else {
            positive.add_shifted(product, shift as usize);
        }
    }
    let (negative_result, magnitude) = match positive.compare(&negative) {
        std::cmp::Ordering::Greater => (false, positive.subtract(&negative)),
        std::cmp::Ordering::Less => (true, negative.subtract(&positive)),
        std::cmp::Ordering::Equal => {
            return Ok(ExactValue {
                negative: false,
                magnitude: BigMagnitude::default(),
            });
        }
    };
    Ok(ExactValue {
        negative: negative_result,
        magnitude,
    })
}

/// Exact IEEE-754 decomposition `value = (-1)^sign * mantissa * 2^exponent`.
fn exact_f64_parts(value: f64) -> (bool, u64, i32) {
    debug_assert!(value.is_finite() && value != 0.0);
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let biased_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    if biased_exponent == 0 {
        (negative, fraction, -1074)
    } else {
        (
            negative,
            (1_u64 << 52) | fraction,
            biased_exponent - 1023 - 52,
        )
    }
}

const INLINE_ACCUMULATOR_LIMBS: usize = 68;

#[derive(Debug, Clone)]
struct BigMagnitude {
    // 4352 inline bits cover the entire 4196-bit finite-f64 product span plus
    // 156 carry bits. Ordinary sums therefore do not allocate in the
    // Newton loop. Very large term counts grow explicitly rather
    // than losing arithmetic information.
    inline: [u64; INLINE_ACCUMULATOR_LIMBS],
    extra: Vec<u64>,
    /// One past the highest nonzero limb. This is maintained on every write,
    /// so hot arithmetic visits only the active span instead of rescanning
    /// all 68 inline limbs for every comparison and conversion.
    active_len: usize,
}

impl Default for BigMagnitude {
    fn default() -> Self {
        Self {
            inline: [0; INLINE_ACCUMULATOR_LIMBS],
            extra: Vec::new(),
            active_len: 0,
        }
    }
}

impl BigMagnitude {
    fn add_shifted(&mut self, value: u128, shift: usize) {
        let limb = shift / 64;
        let intra = shift % 64;
        let low = value as u64;
        let high = (value >> 64) as u64;
        if intra == 0 {
            self.add_word(limb, low);
            self.add_word(limb + 1, high);
        } else {
            self.add_word(limb, low << intra);
            self.add_word(limb + 1, low >> (64 - intra));
            self.add_word(limb + 1, high << intra);
            self.add_word(limb + 2, high >> (64 - intra));
        }
    }

    fn add_word(&mut self, mut index: usize, word: u64) {
        if word == 0 {
            return;
        }
        let (sum, mut carry) = self.word(index).overflowing_add(word);
        self.set_word(index, sum);
        while carry {
            index += 1;
            let (sum, next_carry) = self.word(index).overflowing_add(1);
            self.set_word(index, sum);
            carry = next_carry;
        }
    }

    fn word(&self, index: usize) -> u64 {
        if index < INLINE_ACCUMULATOR_LIMBS {
            self.inline[index]
        } else {
            self.extra
                .get(index - INLINE_ACCUMULATOR_LIMBS)
                .copied()
                .unwrap_or(0)
        }
    }

    fn set_word(&mut self, index: usize, value: u64) {
        if index < INLINE_ACCUMULATOR_LIMBS {
            self.inline[index] = value;
        } else {
            let extra_index = index - INLINE_ACCUMULATOR_LIMBS;
            if self.extra.len() <= extra_index {
                if value == 0 {
                    return;
                }
                self.extra.resize(extra_index + 1, 0);
            }
            self.extra[extra_index] = value;
        }
        if value != 0 {
            self.active_len = self.active_len.max(index + 1);
        } else if index + 1 == self.active_len {
            while self.active_len != 0 && self.word(self.active_len - 1) == 0 {
                self.active_len -= 1;
            }
        }
    }

    fn significant_len(&self) -> usize {
        self.active_len
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let self_len = self.significant_len();
        let other_len = other.significant_len();
        self_len.cmp(&other_len).then_with(|| {
            (0..self_len)
                .rev()
                .find_map(|index| {
                    let ordering = self.word(index).cmp(&other.word(index));
                    (ordering != std::cmp::Ordering::Equal).then_some(ordering)
                })
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Exact unsigned subtraction. `self` must be at least `other`.
    fn subtract(&self, other: &Self) -> Self {
        debug_assert!(self.compare(other) != std::cmp::Ordering::Less);
        let mut result = Self::default();
        let mut borrow = false;
        for index in 0..self.significant_len() {
            let left = self.word(index);
            let right = other.word(index);
            let (difference, first_borrow) = left.overflowing_sub(right);
            let (difference, second_borrow) = difference.overflowing_sub(u64::from(borrow));
            result.set_word(index, difference);
            borrow = first_borrow || second_borrow;
        }
        debug_assert!(!borrow);
        while result.extra.last() == Some(&0) {
            result.extra.pop();
        }
        result
    }

    fn is_zero(&self) -> bool {
        self.significant_len() == 0
    }

    fn one() -> Self {
        let mut value = Self::default();
        value.set_word(0, 1);
        value
    }

    fn top_bit(&self) -> Option<usize> {
        let significant_len = self.significant_len();
        (significant_len != 0).then(|| {
            let top_limb = self.word(significant_len - 1);
            (significant_len - 1) * 64 + (u64::BITS - 1 - top_limb.leading_zeros()) as usize
        })
    }

    fn shift_left(&self, shift: usize) -> Self {
        if self.is_zero() || shift == 0 {
            return self.clone();
        }
        let limb_shift = shift / 64;
        let intra = shift % 64;
        let mut result = Self::default();
        for source in 0..self.significant_len() {
            let word = self.word(source);
            let target = source + limb_shift;
            if intra == 0 {
                result.set_word(target, word);
            } else {
                result.add_word(target, word << intra);
                result.add_word(target + 1, word >> (64 - intra));
            }
        }
        result
    }
}

fn checked_exact_ratio(
    numerator: ExactValue,
    denominator: ExactValue,
) -> Result<f64, ArithmeticError> {
    if denominator.magnitude.is_zero() {
        return Err(ArithmeticError::ZeroDenominator);
    }
    if numerator.magnitude.is_zero() {
        return Ok(0.0);
    }
    exact_ratio_to_f64(
        &numerator.magnitude,
        &denominator.magnitude,
        numerator.negative ^ denominator.negative,
    )
}

#[cfg(test)]
fn checked_product(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    let exact = scaled_sum_products([(left, right)])?;
    exact_binary_to_f64(&exact.magnitude, exact.negative, -2148)
}

fn exact_binary_to_f64(
    magnitude: &BigMagnitude,
    negative: bool,
    exponent_floor: i32,
) -> Result<f64, ArithmeticError> {
    let Some(top_bit) = magnitude.top_bit() else {
        return Ok(0.0);
    };
    let unbiased = exponent_floor + top_bit as i32;
    let scale = if unbiased < -1022 {
        exponent_floor + 1074
    } else {
        52 - top_bit as i32
    };
    let mut quotient = rounded_scaled_ratio(magnitude, &BigMagnitude::one(), scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent += 1;
    }
    encode_f64(quotient, output_exponent, negative)
}

fn exact_ratio_to_f64(
    numerator: &BigMagnitude,
    denominator: &BigMagnitude,
    negative: bool,
) -> Result<f64, ArithmeticError> {
    let numerator_top = numerator.top_bit().expect("nonzero numerator");
    let denominator_top = denominator.top_bit().expect("nonzero denominator");
    let mut unbiased = numerator_top as i32 - denominator_top as i32;
    let below_candidate = if unbiased >= 0 {
        numerator.compare(&denominator.shift_left(unbiased as usize)) == std::cmp::Ordering::Less
    } else {
        numerator
            .shift_left((-unbiased) as usize)
            .compare(denominator)
            == std::cmp::Ordering::Less
    };
    if below_candidate {
        unbiased -= 1;
    }
    let scale = if unbiased < -1022 {
        1074
    } else {
        52 - unbiased
    };
    let mut quotient = rounded_scaled_ratio(numerator, denominator, scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent += 1;
    }
    encode_f64(quotient, output_exponent, negative)
}

fn rounded_scaled_ratio(
    numerator: &BigMagnitude,
    denominator: &BigMagnitude,
    binary_scale: i32,
) -> Result<u64, ArithmeticError> {
    let (mut remainder, divisor) = if binary_scale >= 0 {
        (
            numerator.shift_left(binary_scale as usize),
            denominator.clone(),
        )
    } else {
        (
            numerator.clone(),
            denominator.shift_left((-binary_scale) as usize),
        )
    };

    let mut quotient = 0_u64;
    while remainder.compare(&divisor) != std::cmp::Ordering::Less {
        let mut shift = remainder.top_bit().expect("nonzero remainder")
            - divisor.top_bit().expect("nonzero divisor");
        let mut shifted = divisor.shift_left(shift);
        if shifted.compare(&remainder) == std::cmp::Ordering::Greater {
            shift -= 1;
            shifted = divisor.shift_left(shift);
        }
        if shift >= u64::BITS as usize {
            return Err(ArithmeticError::MantissaBounds);
        }
        quotient |= 1_u64 << shift;
        remainder = remainder.subtract(&shifted);
    }

    let twice_remainder = remainder.shift_left(1);
    let ordering = twice_remainder.compare(&divisor);
    if ordering == std::cmp::Ordering::Greater
        || (ordering == std::cmp::Ordering::Equal && quotient & 1 != 0)
    {
        quotient = quotient
            .checked_add(1)
            .ok_or(ArithmeticError::MantissaBounds)?;
    }
    Ok(quotient)
}

fn encode_f64(
    quotient: u64,
    unbiased_exponent: i32,
    negative: bool,
) -> Result<f64, ArithmeticError> {
    let sign = u64::from(negative) << 63;
    if quotient == 0 {
        return Ok(f64::from_bits(sign));
    }
    let bits = if unbiased_exponent < -1022 {
        if quotient >= 1_u64 << 52 {
            sign | (1_u64 << 52)
        } else {
            sign | quotient
        }
    } else {
        if unbiased_exponent > 1023 || !((1_u64 << 52)..(1_u64 << 53)).contains(&quotient) {
            return Err(ArithmeticError::Overflow { negative });
        }
        sign | ((unbiased_exponent + 1023) as u64) << 52 | (quotient - (1_u64 << 52))
    };
    Ok(f64::from_bits(bits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_zero_detection_distinguishes_underflow_and_cancellation() {
        let tiny = f64::from_bits(1);
        for terms in [
            vec![(tiny, tiny)],
            vec![(f64::MAX, f64::MAX), (tiny, tiny), (-f64::MAX, f64::MAX)],
        ] {
            assert_eq!(sum_products_is_zero(terms.into_iter()), Ok(false));
        }
        for terms in [
            vec![],
            vec![(f64::MAX, f64::MAX), (-f64::MAX, f64::MAX)],
            vec![
                (f64::MAX, f64::MAX),
                (tiny, tiny),
                (-f64::MAX, f64::MAX),
                (-tiny, tiny),
            ],
        ] {
            assert_eq!(sum_products_is_zero(terms.into_iter()), Ok(true));
        }
        assert_eq!(
            sum_products_is_zero([(f64::MAX, f64::MAX), (tiny, tiny), (f64::NAN, 0.)].into_iter()),
            Err(ArithmeticError::NonFiniteTerm)
        );
    }

    #[test]
    fn product_ratio_retains_finite_results_and_signed_underflow() {
        let tiny = f64::from_bits(1);
        for scale in [tiny, 1e-200, 0.125, 1.0, 1e200, f64::MAX] {
            for negative in [false, true] {
                let a = if negative { -scale } else { scale };
                assert_eq!(
                    product_ratio(a, scale, scale, scale),
                    if negative { -1.0 } else { 1.0 }
                );
                assert_eq!(
                    product_ratio(a, tiny, scale, tiny),
                    if negative { -1.0 } else { 1.0 }
                );
                assert_eq!(
                    product_ratio(a, f64::MAX, scale, f64::MAX),
                    if negative { -1.0 } else { 1.0 }
                );
            }
        }
        assert_eq!(product_ratio(tiny, 0.5, 1.0, 1.0).to_bits(), 0);
        assert_eq!(
            product_ratio(-tiny, 0.5, 1.0, 1.0).to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            product_ratio(-0.0, 1.0, tiny, tiny).to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            product_ratio(-f64::MAX, f64::MAX, tiny, tiny),
            f64::NEG_INFINITY
        );
        assert!(product_ratio(0.0, 1.0, 0.0, 1.0).is_nan());
        assert!(product_ratio(f64::INFINITY, 1.0, f64::INFINITY, 1.0).is_nan());
    }

    #[test]
    fn full_precision_products_use_compact_correctly_rounded_ratios() {
        // Deterministic, non-dyadic mantissas exercise the long-division path
        // against the independent multi-limb exact implementation.
        let mut state = 0xd17e_45a3_096c_f821_u64;
        for exponent in [-1000_i32, -300, 0, 300, 1000] {
            for _ in 0..80 {
                let mut factor = || {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    f64::from_bits(((exponent + 1023) as u64) << 52 | (state & ((1_u64 << 52) - 1)))
                };
                let numerator = [(factor(), factor())];
                let denominator = [(factor(), factor())];
                let compact = small_exact_ratio_to_f64(
                    small_sum_products(numerator).unwrap().unwrap(),
                    small_sum_products(denominator).unwrap().unwrap(),
                )
                .expect("two full precision products must fit compact ratio division")
                .unwrap();
                let reference = checked_exact_ratio(
                    scaled_sum_products(numerator).unwrap(),
                    scaled_sum_products(denominator).unwrap(),
                )
                .unwrap();
                assert_eq!(compact.to_bits(), reference.to_bits());
            }
        }
    }

    #[test]
    fn small_exact_ratio_is_bit_identical_to_big_fallback_for_typical_terms() {
        let numerator_terms = [(0.25, 1.25), (-0.5, 0.75), (0.125, -2.0), (0.0625, 4.0)];
        let denominator_terms = [(1.0, 1.0), (-0.125, 1.0)];
        let small_numerator = small_sum_products(numerator_terms).unwrap().unwrap();
        let small_denominator = small_sum_products(denominator_terms).unwrap().unwrap();
        let small = small_exact_ratio_to_f64(small_numerator, small_denominator)
            .expect("ordinary terms must fit the compact exact ratio")
            .unwrap();

        let big = checked_exact_ratio(
            scaled_sum_products(numerator_terms).unwrap(),
            scaled_sum_products(denominator_terms).unwrap(),
        )
        .unwrap();
        assert_eq!(small.to_bits(), big.to_bits());
    }

    #[test]
    fn wide_exponent_span_uses_bit_identical_big_fallback() {
        let tiny = f64::from_bits(1);
        let terms = [(f64::MAX, 1.0), (tiny, 1.0)];
        assert!(
            small_sum_products(terms).unwrap().is_none(),
            "a span wider than i128 must explicitly decline the compact path"
        );
        let actual = sum_products(terms.into_iter()).unwrap();
        let expected = {
            let exact = scaled_sum_products(terms).unwrap();
            exact_binary_to_f64(&exact.magnitude, exact.negative, -2148).unwrap()
        };
        assert_eq!(actual.to_bits(), expected.to_bits());
    }

    #[test]
    fn exact_conversion_obeys_ieee_underflow_and_boundary_rounding() {
        let tiny = f64::from_bits(1);
        assert_eq!(checked_product(tiny, 0.25).unwrap().to_bits(), 0);
        assert_eq!(checked_product(tiny, 0.5).unwrap().to_bits(), 0);
        assert_eq!(checked_product(tiny, 0.75).unwrap().to_bits(), 1);
        assert_eq!(
            checked_product(-tiny, 0.5).unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            checked_product(-tiny, 0.75).unwrap().to_bits(),
            (-tiny).to_bits()
        );

        let max_subnormal = f64::from_bits((1_u64 << 52) - 1);
        assert_eq!(checked_product(max_subnormal, 1.0).unwrap(), max_subnormal);
        assert_eq!(
            checked_product(f64::MIN_POSITIVE, 1.0).unwrap(),
            f64::MIN_POSITIVE
        );
        assert_eq!(checked_product(f64::MAX, 1.0).unwrap(), f64::MAX);
        assert!(checked_product(f64::MAX, f64::from_bits(1.0_f64.to_bits() + 1)).is_err());
    }

    #[test]
    fn exact_accumulator_tracks_only_its_active_limb_span() {
        let mut low = BigMagnitude::default();
        low.add_shifted(1, 0);
        assert_eq!(low.significant_len(), 1);

        let mut high = BigMagnitude::default();
        high.add_shifted(1, INLINE_ACCUMULATOR_LIMBS * 64 + 7);
        assert_eq!(
            high.significant_len(),
            INLINE_ACCUMULATOR_LIMBS + 1,
            "cached active span must include the first overflow limb"
        );
        let cancelled = high.subtract(&high);
        assert_eq!(cancelled.significant_len(), 0);
        assert!(cancelled.is_zero());
    }
}
