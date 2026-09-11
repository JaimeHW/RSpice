//! Exact sums of binary64 products with one final round-to-nearest-even conversion.
//! Shared by sampled filters and derivative evaluation. Derivative quotients
//! preserve ordinary rounding in range and use exact arithmetic to rescue
//! intermediate overflow or underflow.

/// Failure to evaluate or represent an arithmetic result as a finite binary64 value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    NonFiniteTerm,
    ZeroDenominator,
    Overflow {
        negative: bool,
    },
    Underflow,
    MantissaBounds,
    /// Bounded recovery could not certify the result within this precision.
    PrecisionLimit {
        bits: u32,
    },
}

/// RSpice's default Newton limiter. `previous` is the proposal on the first
/// evaluation. Invalid operands return NaN; callers must reject that candidate
/// before publishing history. Probe evaluations bypass the limiter entirely.
///
/// Return an unclipped proposal exactly, avoiding cancellation in
/// `previous + (proposed - previous)`. An overflowing difference still selects
/// the correct direction, and a clipped step stays between finite endpoints.
#[inline]
pub fn default_limit_candidate(proposed: f64, previous: f64, step: f64) -> f64 {
    if !proposed.is_finite() || !previous.is_finite() || !step.is_finite() || step < 0.0 {
        return f64::NAN;
    }
    let delta = proposed - previous;
    if delta > step {
        previous + step
    } else if delta < -step {
        previous - step
    } else {
        proposed
    }
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
        Err(ArithmeticError::Overflow { negative }) => infinity(negative),
        Err(_) => f64::NAN,
    }
}

/// Evaluate a derivative numerator `sum(a_i * b_i) / divisor`.
/// Normal products and partial sums use ordinary, separately rounded IEEE
/// operations in term order. This matters when a coefficient is an already
/// rounded primal quotient: fusing its product can introduce a residual that
/// a small divisor amplifies (for example, `(2 - (2 / 3) * 3) / 1e-309`).
/// If an intermediate overflows or is subnormal, including a nonzero product
/// rounded to zero, recompute the complete numerator exactly and round only
/// after division. Non-finite inputs and zero divisors retain IEEE behavior.
/// The uncommon exact engine is shared across generated expression sizes.
#[inline]
pub fn sum_products_div(terms: &[[f64; 2]], divisor: f64) -> f64 {
    sum_products_div_iter(terms.iter().copied(), divisor)
}

/// One-term form of [`sum_products_div`], with scalar recovery arguments.
/// Generated fixed-width lanes call this directly so their ordinary path does
/// not materialize operand slices or introduce iterator loops into the stamp.
#[inline]
pub fn product_div(a: f64, b: f64, divisor: f64) -> f64 {
    let product = a * b;
    if ordinary_product(a, b, product) {
        product / divisor
    } else {
        exact_product_div(a, b, divisor)
    }
}

/// Two-term form of [`sum_products_div`]: `(a * b + c * d) / divisor`.
/// The products are separately rounded on the ordinary path, including when
/// one factor is an already rounded primal quotient.
#[inline]
pub fn product_sum_div(a: f64, b: f64, c: f64, d: f64, divisor: f64) -> f64 {
    let left = a * b;
    let right = c * d;
    let sum = left + right;
    if ordinary_product(a, b, left)
        && ordinary_product(c, d, right)
        && (sum.is_normal() || sum == 0.0)
    {
        sum / divisor
    } else {
        exact_product_sum_div(a, b, c, d, divisor)
    }
}

#[cold]
#[inline(never)]
fn exact_product_div(a: f64, b: f64, divisor: f64) -> f64 {
    exact_sum_products_div_iter([[a, b]].iter().copied(), divisor)
}

#[cold]
#[inline(never)]
fn exact_product_sum_div(a: f64, b: f64, c: f64, d: f64, divisor: f64) -> f64 {
    exact_sum_products_div_iter([[a, b], [c, d]].iter().copied(), divisor)
}

/// Packed derivative quotient. Every term's input must contain exactly N lanes.
/// The compiler chooses whether to inline the ordinary path for each lane
/// and term count. The exact fallback stays shared and out of line. Forcing
/// thousands of these expansions substantially increases model compilation
/// cost and can exhaust an unoptimized build's thread stack.
#[inline]
pub fn sum_products_div_lanes<const N: usize>(terms: &[(&[f64], f64)], divisor: f64) -> [f64; N] {
    let mut result = [0.0; N];
    fill_sum_products_div_lanes(terms, divisor, &mut result);
    result
}

#[inline(always)]
fn fill_sum_products_div_lanes(terms: &[(&[f64], f64)], divisor: f64, result: &mut [f64]) {
    assert!(
        terms.iter().all(|(input, _)| input.len() == result.len()),
        "quotient lane shape mismatch"
    );
    for (lane, output) in result.iter_mut().enumerate() {
        *output = sum_products_div_iter(
            terms.iter().map(|&(input, scalar)| [input[lane], scalar]),
            divisor,
        );
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn sum_products_div_iter<I>(terms: I, divisor: f64) -> f64
where
    I: Iterator<Item = [f64; 2]> + ExactSizeIterator + Clone,
{
    if let Some(sum) = ordinary_sum_products(terms.clone()) {
        return sum / divisor;
    }
    exact_sum_products_div_iter(terms, divisor)
}

#[inline(always)]
fn ordinary_product(a: f64, b: f64, product: f64) -> bool {
    product.is_normal() || (product == 0.0 && (a == 0.0 || b == 0.0))
}

#[inline(always)]
pub(super) fn ordinary_sum_products(mut terms: impl Iterator<Item = [f64; 2]>) -> Option<f64> {
    let product = |[a, b]: [f64; 2]| {
        let value = a * b;
        ordinary_product(a, b, value).then_some(value)
    };
    let mut sum = product(terms.next()?)?;
    for term in terms {
        sum += product(term)?;
        if !sum.is_normal() && sum != 0.0 {
            return None;
        }
    }
    Some(sum)
}

#[inline(never)]
fn exact_sum_products_div_iter<I>(mut terms: I, divisor: f64) -> f64
where
    I: Iterator<Item = [f64; 2]> + ExactSizeIterator + Clone,
{
    if terms.len() == 1 {
        let [a, b] = terms.next().expect("one term");
        if a == 1.0 {
            return b / divisor;
        }
        if b == 1.0 {
            return a / divisor;
        }
        return product_ratio(a, b, divisor, 1.0);
    }
    if !divisor.is_finite() || divisor == 0.0 {
        return ieee_sum_products_div(terms, divisor);
    }
    match small_sum_products(terms.clone().map(|[a, b]| (a, b))) {
        Ok(Some(numerator)) => {
            if numerator.value == 0 {
                return exact_zero_quotient(terms, divisor);
            }
            if let Some(numerator) = numerator.to_f64_exact() {
                return numerator / divisor;
            }
            let denominator = SmallExact::product(divisor, 1.0)
                .expect("one finite nonzero binary64 value fits SmallExact");
            if let Some(result) = small_exact_ratio_to_f64(numerator, denominator) {
                return arithmetic_value(result);
            }
        }
        Err(_) => return ieee_sum_products_div(terms, divisor),
        Ok(None) => {}
    }
    let Ok(numerator) = scaled_sum_products(terms.clone().map(|[a, b]| (a, b))) else {
        return ieee_sum_products_div(terms, divisor);
    };
    if numerator.magnitude.is_zero() {
        return exact_zero_quotient(terms, divisor);
    }
    let denominator =
        scaled_sum_products([(divisor, 1.0)]).expect("the divisor was checked finite");
    arithmetic_value(checked_exact_ratio(numerator, denominator))
}

fn exact_zero_quotient(mut terms: impl ExactSizeIterator<Item = [f64; 2]>, divisor: f64) -> f64 {
    let negative = terms.len() != 0
        && terms
            .all(|[a, b]| (a == 0.0 || b == 0.0) && a.is_sign_negative() != b.is_sign_negative());
    let negative = negative != divisor.is_sign_negative();
    f64::from_bits(u64::from(negative) << 63)
}

fn ieee_sum_products_div(terms: impl Iterator<Item = [f64; 2]>, divisor: f64) -> f64 {
    terms
        .map(|[a, b]| a * b)
        .reduce(|a, b| a + b)
        .unwrap_or(0.0)
        / divisor
}

fn arithmetic_value(result: Result<f64, ArithmeticError>) -> f64 {
    match result {
        Ok(value) => value,
        Err(ArithmeticError::Overflow { negative }) => infinity(negative),
        Err(_) => f64::NAN,
    }
}

fn infinity(negative: bool) -> f64 {
    f64::from_bits((u64::from(negative) << 63) | f64::INFINITY.to_bits())
}

/// Allocation-free exact representation for the common case where a sum of
/// binary64 products fits a normalized signed 128-bit integer. Unsupported
/// exponent spans or carries fall back to `BigMagnitude`; they never round in
/// this representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SmallExact {
    pub(crate) value: i128,
    pub(crate) exponent: i32,
}

impl SmallExact {
    pub(crate) const ZERO: Self = Self {
        value: 0,
        exponent: 0,
    };

    pub(crate) fn normalized(value: i128, exponent: i32) -> Option<Self> {
        if value == 0 {
            return Some(Self::ZERO);
        }
        let trailing = value.unsigned_abs().trailing_zeros();
        Some(Self {
            value: value >> trailing,
            exponent: exponent.checked_add(trailing as i32)?,
        })
    }

    pub(crate) fn product(left: f64, right: f64) -> Option<Self> {
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

    pub(crate) fn checked_add(self, other: Self) -> Option<Self> {
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

    fn to_f64_exact(self) -> Option<f64> {
        let magnitude = self.value.unsigned_abs();
        let bits = u128::BITS - magnitude.leading_zeros();
        if bits == 0 || bits > 53 || self.exponent < -1074 {
            return None;
        }
        let top = self.exponent + bits as i32 - 1;
        if top > 1023 {
            return None;
        }
        let sign = u64::from(self.value < 0) << 63;
        let magnitude = magnitude as u64;
        let encoded = if top >= -1022 {
            let mantissa = magnitude << (53 - bits);
            (((top + 1023) as u64) << 52) | (mantissa & ((1_u64 << 52) - 1))
        } else {
            magnitude << (self.exponent + 1074)
        };
        Some(f64::from_bits(sign | encoded))
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

pub(crate) fn small_exact_ratio_to_f64(
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
    if let Some(result) = small_sum_products(terms.clone())?.and_then(|exact| {
        small_exact_ratio_to_f64(
            exact,
            SmallExact {
                value: 1,
                exponent: 0,
            },
        )
    }) {
        return result;
    }

    let exact = scaled_sum_products(terms)?;
    exact_binary_to_f64(&exact.magnitude, exact.negative, -2148)
}

#[derive(Debug, Clone)]
pub(super) struct ExactValue {
    pub(super) negative: bool,
    pub(super) magnitude: BigMagnitude,
}

/// Divide exact sums of triple products. Used for complex quotient numerators
/// after multiplication by the conjugate denominator.
pub fn sum_triple_products_ratio(
    numerator: impl IntoIterator<Item = [f64; 3]>,
    denominator: impl IntoIterator<Item = [f64; 3]>,
) -> Result<f64, ArithmeticError> {
    checked_exact_ratio(
        scaled_sum_triple_products(numerator, -3222)?,
        scaled_sum_triple_products(denominator, -3222)?,
    )
}

fn scaled_sum_products(
    terms: impl IntoIterator<Item = (f64, f64)>,
) -> Result<ExactValue, ArithmeticError> {
    scaled_sum_triple_products(terms.into_iter().map(|(a, b)| [a, b, 1.0]), -2148)
}

fn scaled_sum_triple_products(
    terms: impl IntoIterator<Item = [f64; 3]>,
    exponent_floor: i32,
) -> Result<ExactValue, ArithmeticError> {
    // Accumulate positive and negative exact integers separately, so a large
    // canceling pair cannot erase a tiny representable remainder.
    let mut positive = BigMagnitude::default();
    let mut negative = BigMagnitude::default();
    for factors in terms {
        if factors.iter().any(|value| !value.is_finite()) {
            return Err(ArithmeticError::NonFiniteTerm);
        }
        if factors.contains(&0.0) {
            continue;
        }
        let mut sign = false;
        let mut exponent = 0;
        let mut mantissas = [0_u64; 3];
        for (factor, mantissa) in factors.into_iter().zip(&mut mantissas) {
            let (negative, bits, power) = exact_f64_parts(factor);
            let trailing = bits.trailing_zeros();
            *mantissa = bits >> trailing;
            exponent += power + trailing as i32;
            sign ^= negative;
        }
        let pair = u128::from(mantissas[0]) * u128::from(mantissas[1]);
        let third = u128::from(mantissas[2]);
        let shift = exponent - exponent_floor;
        debug_assert!(shift >= 0);
        let accumulator = if sign { &mut negative } else { &mut positive };
        accumulator.add_shifted(u128::from(pair as u64) * third, shift as usize);
        accumulator.add_shifted((pair >> 64) * third, shift as usize + 64);
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
pub(super) fn exact_f64_parts(value: f64) -> (bool, u64, i32) {
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
pub(crate) struct BigMagnitude {
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
    /// A normalized leading-word estimate is at most two integer units above
    /// the quotient when that quotient fits u64. Verify it against the complete
    /// integers before rounding; no low limbs are discarded from the result.
    fn quotient_remainder(&self, divisor: &Self) -> Option<(u64, Self)> {
        let shift = divisor.top_bit()?.saturating_sub(63);
        if self
            .top_bit()
            .is_some_and(|top| top.saturating_sub(shift) >= 128)
        {
            return None;
        }
        let leading_divisor = divisor.window_u128(shift);
        let estimate = self.window_u128(shift) / leading_divisor;
        let mut quotient = u64::try_from(estimate).ok()?;
        let mut product = Self::default();
        for index in 0..divisor.significant_len() {
            product.add_shifted(
                u128::from(divisor.word(index)) * u128::from(quotient),
                index * 64,
            );
        }
        while product.compare(self) == std::cmp::Ordering::Greater {
            quotient = quotient.checked_sub(1)?;
            product = product.subtract(divisor);
        }
        let remainder = self.subtract(&product);
        debug_assert!(remainder.compare(divisor) == std::cmp::Ordering::Less);
        Some((quotient, remainder))
    }

    fn window_u128(&self, shift: usize) -> u128 {
        let index = shift / 64;
        let bits = shift % 64;
        let (low, high) = if bits == 0 {
            (self.word(index), self.word(index + 1))
        } else {
            (
                (self.word(index) >> bits) | (self.word(index + 1) << (64 - bits)),
                (self.word(index + 1) >> bits) | (self.word(index + 2) << (64 - bits)),
            )
        };
        u128::from(low) | (u128::from(high) << 64)
    }

    pub(crate) fn add_shifted(&mut self, value: u128, shift: usize) {
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

    pub(crate) fn add_word(&mut self, mut index: usize, word: u64) {
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

    pub(crate) fn word(&self, index: usize) -> u64 {
        if index < INLINE_ACCUMULATOR_LIMBS {
            self.inline[index]
        } else {
            self.extra
                .get(index - INLINE_ACCUMULATOR_LIMBS)
                .copied()
                .unwrap_or(0)
        }
    }

    pub(crate) fn set_word(&mut self, index: usize, value: u64) {
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

    pub(crate) fn significant_len(&self) -> usize {
        self.active_len
    }

    pub(crate) fn compare(&self, other: &Self) -> std::cmp::Ordering {
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
    pub(crate) fn subtract(&self, other: &Self) -> Self {
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

    pub(crate) fn is_zero(&self) -> bool {
        self.significant_len() == 0
    }

    fn one() -> Self {
        let mut value = Self::default();
        value.set_word(0, 1);
        value
    }

    pub(super) fn top_bit(&self) -> Option<usize> {
        let significant_len = self.significant_len();
        (significant_len != 0).then(|| {
            let top_limb = self.word(significant_len - 1);
            (significant_len - 1) * 64 + (u64::BITS - 1 - top_limb.leading_zeros()) as usize
        })
    }

    pub(crate) fn shift_left(&self, shift: usize) -> Self {
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

pub(super) fn exact_binary_to_f64(
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
    scaled_exact_ratio_to_f64(numerator, denominator, negative, 0)
}

pub(crate) fn scaled_exact_ratio_to_f64(
    numerator: &BigMagnitude,
    denominator: &BigMagnitude,
    negative: bool,
    exponent_offset: i32,
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
    let unbiased = unbiased
        .checked_add(exponent_offset)
        .ok_or(ArithmeticError::MantissaBounds)?;
    if unbiased > 1023 {
        return Err(ArithmeticError::Overflow { negative });
    }
    if unbiased < -1075 {
        return Ok(f64::from_bits(u64::from(negative) << 63));
    }
    let scale = (if unbiased < -1022 {
        1074
    } else {
        52 - unbiased
    })
    .checked_add(exponent_offset)
    .ok_or(ArithmeticError::MantissaBounds)?;
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

    let subtract_bit = |remainder: &BigMagnitude| -> Result<(u64, BigMagnitude), ArithmeticError> {
        let mut shift = remainder.top_bit().expect("nonzero remainder")
            - divisor.top_bit().expect("nonzero divisor");
        let mut shifted = divisor.shift_left(shift);
        let ordering = shifted.compare(remainder);
        if ordering == std::cmp::Ordering::Greater {
            shift -= 1;
            shifted = divisor.shift_left(shift);
        }
        if shift >= u64::BITS as usize {
            return Err(ArithmeticError::MantissaBounds);
        }
        if ordering == std::cmp::Ordering::Equal {
            return Ok((1_u64 << shift, BigMagnitude::default()));
        }
        Ok((1_u64 << shift, remainder.subtract(&shifted)))
    };
    let mut quotient = 0_u64;
    if remainder.compare(&divisor) != std::cmp::Ordering::Less {
        (quotient, remainder) = subtract_bit(&remainder)?;
        // Preserve the single-subtraction path for exact powers of two and
        // small quotients; estimate only the remaining dense quotient bits.
        while remainder.compare(&divisor) != std::cmp::Ordering::Less {
            if let Some((estimate, residual)) = remainder.quotient_remainder(&divisor) {
                quotient |= estimate;
                remainder = residual;
                break;
            }
            let (bit, residual) = subtract_bit(&remainder)?;
            quotient |= bit;
            remainder = residual;
        }
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
    #[test]
    fn default_limiter_preserves_exact_proposals_and_finite_extreme_steps() {
        use super::default_limit_candidate as limit;
        for (proposed, previous, step, expected) in [
            (1.0, 2.0_f64.powi(54), 2.0_f64.powi(54), 1.0_f64),
            (-0.0, 1.0, 1.0, -0.0),
            (f64::MAX, -f64::MAX, f64::MAX, 0.0),
            (-f64::MAX, f64::MAX, f64::MAX, 0.0),
            (1.0, 0.0, 0.0, 0.0),
        ] {
            assert_eq!(
                limit(proposed, previous, step).to_bits(),
                expected.to_bits()
            );
        }
        for bad in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(limit(0.0, 0.0, bad).is_nan());
        }
    }

    use super::*;

    #[test]
    fn leading_quotient_estimates_preserve_carries_and_halfway_rounding() {
        for limbs in [1, 2, 3, 8, 33, 67] {
            let mut divisor = BigMagnitude::default();
            for index in 0..limbs {
                divisor.set_word(index, 0xffff_ffff_ffff_fffe - index as u64 * 2);
            }
            let mut half = BigMagnitude::default();
            for index in 0..limbs {
                half.set_word(
                    index,
                    (divisor.word(index) >> 1) | (divisor.word(index + 1) << 63),
                );
            }
            let one = BigMagnitude::one();
            let below = half.subtract(&one);
            let mut above = half.clone();
            above.add_word(0, 1);
            for quotient in [0_u64, 1, 1_u64 << 52, (1_u64 << 53) - 1, u64::MAX] {
                let mut product = BigMagnitude::default();
                for index in 0..limbs {
                    product.add_shifted(
                        u128::from(divisor.word(index)) * u128::from(quotient),
                        index * 64,
                    );
                }
                for (remainder, increment) in [
                    (&BigMagnitude::default(), false),
                    (&below, false),
                    (&half, quotient & 1 != 0),
                    (&above, true),
                ] {
                    let mut numerator = product.clone();
                    for index in 0..remainder.significant_len() {
                        numerator.add_word(index, remainder.word(index));
                    }
                    let expected = quotient
                        .checked_add(u64::from(increment))
                        .ok_or(ArithmeticError::MantissaBounds);
                    assert_eq!(
                        rounded_scaled_ratio(&numerator, &divisor, 0),
                        expected,
                        "limbs={limbs}, quotient={quotient}, increment={increment}"
                    );
                    assert_eq!(
                        rounded_scaled_ratio(&numerator.shift_left(65), &divisor, -65),
                        expected
                    );
                    assert_eq!(
                        rounded_scaled_ratio(&numerator, &divisor.shift_left(65), 65),
                        expected
                    );
                }
            }
        }
    }

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
    fn quotient_fallback_preserves_exact_values_and_rounding_boundaries() {
        for value in [f64::from_bits(1), f64::MIN_POSITIVE, 1.0, 1.25, f64::MAX] {
            for sign in [-1.0, 1.0] {
                let value = value * sign;
                assert_eq!(
                    SmallExact::product(value, 1.0)
                        .unwrap()
                        .to_f64_exact()
                        .unwrap()
                        .to_bits(),
                    value.to_bits()
                );
            }
        }
        let tiny = 2.0_f64.powi(-200);
        for sign in [-1.0, 1.0] {
            let separated = [[sign, 1.0], [sign * tiny, 1.0]];
            assert_eq!(small_sum_products(separated.map(|[a, b]| (a, b))), Ok(None));
            assert_eq!(sum_products_div(&separated, 3.0), sign / 3.0);
            let halfway = [[sign, 1.0], [sign * 2.0_f64.powi(-53), 1.0]];
            assert_eq!(sum_products_div(&halfway, 1.0), sign);
            let above = [halfway[0], halfway[1], [sign * 2.0_f64.powi(-100), 1.0]];
            assert_eq!(sum_products_div(&above, 1.0), sign);
            assert_eq!(
                exact_sum_products_div_iter(above.into_iter(), 1.0),
                sign * 1.0_f64.next_up()
            );
            let cancellation = [
                [2.0_f64.powi(200), 2.0_f64.powi(100)],
                [sign * tiny, 1.0],
                [-2.0_f64.powi(200), 2.0_f64.powi(100)],
            ];
            assert_eq!(sum_products_div(&cancellation, 3.0), 0.0);
            assert_eq!(
                exact_sum_products_div_iter(cancellation.into_iter(), 3.0),
                sign * tiny / 3.0
            );
        }
        for (sign, expected) in [(-1.0, 1.5), (1.0, 1.5_f64.next_up())] {
            // A product below the subnormal range can still decide which side
            // of a rounding midpoint the exact numerator lies on.
            let terms = [
                [1.5, 1.0],
                [2.0_f64.powi(-53), 1.0],
                [sign * 1e-200, 1e-200],
            ];
            assert_eq!(ordinary_sum_products(terms.into_iter()), None);
            assert_eq!(sum_products_div(&terms, 1.0), expected);
        }
    }

    #[test]
    fn quotient_preserves_rounded_primal_cancellation() {
        for (numerator, denominator) in [(2.0, 3.0), (5.0, 7.0)] {
            let quotient = numerator / denominator;
            for divisor in [1.0, 1e-100, 1e-309] {
                for sign in [-1.0, 1.0] {
                    let terms = [[numerator, 1.0], [-quotient, denominator]];
                    let divisor = divisor * sign;
                    let expected = ieee_sum_products_div(terms.into_iter(), divisor);
                    assert_eq!(
                        sum_products_div(&terms, divisor).to_bits(),
                        expected.to_bits()
                    );
                    assert_eq!(expected, 0.0);
                    let left = [numerator, -numerator];
                    let right = [-quotient, quotient];
                    let lanes = sum_products_div_lanes::<2>(
                        &[(&left, 1.0), (&right, denominator)],
                        divisor,
                    );
                    assert!(lanes.into_iter().all(|value| value == 0.0));
                }
            }
        }
    }

    #[test]
    fn quotient_numerators_keep_cancellation_and_rescued_tangents() {
        for (terms, divisor, expected) in [
            (vec![[-100.0, 1e308]], 1e306, -1e4),
            (vec![[1.6e308, 1.0], [-8e307, 3.0]], 2.0, -4e307),
            (vec![[4e307, 3.0], [4e307, 3.0]], 2.0, 1.2e308),
            (vec![[1.0, 1.0], [-1.0, 1.0]], 1e-309, 0.0),
            (vec![[-1e108, 1e-200]], 1e200, -1e-292),
            (
                vec![
                    [f64::MAX, f64::MAX],
                    [1e-200, 1e-200],
                    [-f64::MAX, f64::MAX],
                ],
                1e-200,
                1e-200,
            ),
        ] {
            let actual = sum_products_div(&terms, divisor);
            if expected == 0.0 {
                assert_eq!(actual, expected);
            } else {
                assert!(
                    (actual / expected - 1.0).abs() < 1e-14,
                    "expected {expected:e}, got {actual:e}"
                );
            }
        }
    }

    #[test]
    fn triple_product_ratios_preserve_cancellation_across_the_full_range() {
        for scale in [f64::from_bits(1), 1e-200, 1.25, 1e200, f64::MAX] {
            for sign in [-1.0, 1.0] {
                assert_eq!(
                    sum_triple_products_ratio(
                        [[sign * scale, scale, scale]],
                        [[scale, scale, scale]]
                    ),
                    Ok(sign)
                );
            }
        }
        assert_eq!(
            sum_triple_products_ratio(
                [
                    [f64::MAX, f64::MAX, f64::MAX],
                    [1.0, 1.0, 1.0],
                    [-f64::MAX, f64::MAX, f64::MAX]
                ],
                [[2.0, 1.0, 1.0]],
            ),
            Ok(0.5)
        );
        assert_eq!(
            sum_triple_products_ratio([[1.0, 1.0, 1.0]], [[0.0, 1.0, 1.0]]),
            Err(ArithmeticError::ZeroDenominator)
        );
    }

    #[test]
    fn packed_quotient_sums_keep_each_lane_and_zero_sign() {
        let left = [1e308, -0.0, 1e-292];
        let right = [1e308, -0.0, -1e-292];
        let result = sum_products_div_lanes::<3>(&[(&left, 2.0), (&right, -1.0)], 2.0);
        for (lane, actual) in result.into_iter().enumerate() {
            assert_eq!(
                actual.to_bits(),
                sum_products_div(&[[left[lane], 2.0], [right[lane], -1.0]], 2.0).to_bits()
            );
        }
    }

    #[test]
    fn derivative_sum_preserves_zero_sign_and_ieee_domains() {
        for divisor in [-2.0_f64, 2.0] {
            let cancel = [[1.0, 1.0], [-1.0, 1.0]];
            assert_eq!(
                sum_products_div(&cancel, divisor).to_bits(),
                (0.0 / divisor).to_bits()
            );
            let zeros = [[-0.0, 1.0], [0.0, -2.0]];
            assert_eq!(
                sum_products_div(&zeros, divisor).to_bits(),
                (-0.0 / divisor).to_bits()
            );
            let mixed = [[-0.0, 1.0], [0.0, 2.0]];
            assert_eq!(
                sum_products_div(&mixed, divisor).to_bits(),
                (0.0 / divisor).to_bits()
            );
        }
        assert_eq!(
            sum_products_div(&[[f64::MAX, 1.0], [f64::MAX, 1.0]], 1.0),
            f64::INFINITY
        );
        assert_eq!(
            sum_products_div(&[[f64::MAX, 1.0], [f64::MAX, 1.0]], -1.0),
            f64::NEG_INFINITY
        );
        assert_eq!(
            sum_products_div(&[[f64::INFINITY, 1.0], [2.0, 1.0]], 1.0),
            f64::INFINITY
        );
        assert!(sum_products_div(&[[f64::INFINITY, 1.0], [f64::NEG_INFINITY, 1.0]], 1.0).is_nan());
        assert!(sum_products_div(&[[1.0, 1.0], [-1.0, 1.0]], 0.0).is_nan());
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
