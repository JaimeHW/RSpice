//! Range-preserving evaluation of a positive power law's logarithm.
//!
//! Separately rounded logarithms cannot resolve cancellation after multiplying
//! by arbitrary finite binary64 exponents. Use fixed-point arithmetic only on
//! the exceptional power-overflow/underflow path. Precision grows with the
//! largest exponent, retaining 128 guard bits after the final rescaling.
//! The atanh series has |t| <= 1/3, so its work and truncation error are bounded.
//! At most 18 fractional words and two integer/working words are needed.

use crate::Value;
use core::cmp::Ordering;

const FRACTION_WORDS: usize = 18;
const WORDS: usize = FRACTION_WORDS + 2;
const MANTISSA_MASK: u64 = (1 << 52) - 1;

#[derive(Clone, Copy)]
struct Fixed([u64; WORDS]);

impl Fixed {
    const ZERO: Self = Self([0; WORDS]);

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&word| word == 0)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }

    fn add(&mut self, other: &Self) {
        let mut carry = false;
        for (a, &b) in self.0.iter_mut().zip(&other.0) {
            let (sum, first) = a.overflowing_add(b);
            let (sum, second) = sum.overflowing_add(u64::from(carry));
            *a = sum;
            carry = first || second;
        }
        debug_assert!(!carry);
    }

    /// The caller establishes self >= other.
    fn subtract(&mut self, other: &Self) {
        let mut borrow = false;
        for (a, &b) in self.0.iter_mut().zip(&other.0) {
            let (difference, first) = a.overflowing_sub(b);
            let (difference, second) = difference.overflowing_sub(u64::from(borrow));
            *a = difference;
            borrow = first || second;
        }
        debug_assert!(!borrow);
    }

    fn multiply_word(mut self, factor: u64) -> Self {
        let mut carry = 0_u128;
        for word in &mut self.0 {
            let product = u128::from(*word) * u128::from(factor) + carry;
            *word = product as u64;
            carry = product >> 64;
        }
        debug_assert_eq!(carry, 0);
        self
    }

    fn divide_word(mut self, divisor: u64) -> Self {
        let mut remainder = 0_u128;
        for word in self.0.iter_mut().rev() {
            let numerator = (remainder << 64) | u128::from(*word);
            *word = (numerator / u128::from(divisor)) as u64;
            remainder = numerator % u128::from(divisor);
        }
        self
    }

    fn shift_right(&self, bits: usize) -> Self {
        let mut result = Self::ZERO;
        let words = bits / 64;
        if words < WORDS {
            for i in 0..WORDS - words {
                result.0[i] = self.word_at(i * 64 + bits);
            }
        }
        result
    }

    /// The low word of the value shifted right by `bits`.
    fn word_at(&self, bits: usize) -> u64 {
        let index = bits / 64;
        let offset = bits % 64;
        let mut word = self.0.get(index).copied().unwrap_or(0) >> offset;
        if offset != 0 {
            word |= self.0.get(index + 1).copied().unwrap_or(0) << (64 - offset);
        }
        word
    }

    fn any_below(&self, bits: usize) -> bool {
        let words = bits / 64;
        let remainder = bits % 64;
        self.0[..words].iter().any(|&word| word != 0)
            || (remainder != 0 && self.0[words] & ((1 << remainder) - 1) != 0)
    }

    fn highest_bit(&self) -> Option<usize> {
        self.0
            .iter()
            .rposition(|&word| word != 0)
            .map(|index| index * 64 + 63 - self.0[index].leading_zeros() as usize)
    }

    fn rounded_float(&self, binary_scale: i32, highest_bit: usize) -> Value {
        let shift = highest_bit.saturating_sub(52);
        let mut mantissa = self.word_at(shift);
        if shift != 0
            && self.word_at(shift - 1) & 1 != 0
            && (self.any_below(shift - 1) || mantissa & 1 != 0)
        {
            mantissa += 1;
        }
        libm::scalbn(mantissa as Value, binary_scale + shift as i32)
    }

    /// Truncated fixed-point fraction, with numerator < denominator < 2^54.
    fn ratio(numerator: u64, denominator: u64, fraction_words: usize) -> Self {
        let mut result = Self::ZERO;
        let mut remainder = u128::from(numerator);
        for word in result.0[..fraction_words].iter_mut().rev() {
            let numerator = remainder << 64;
            *word = (numerator / u128::from(denominator)) as u64;
            remainder = numerator % u128::from(denominator);
        }
        result
    }

    /// Both operands are fractions below one; drop the low fractional words.
    fn multiply_fraction(&self, other: &Self, fraction_words: usize) -> Self {
        let mut product = [0_u64; FRACTION_WORDS * 2];
        for i in 0..fraction_words {
            let mut carry = 0_u128;
            for j in 0..fraction_words {
                let value = u128::from(self.0[i]) * u128::from(other.0[j])
                    + u128::from(product[i + j])
                    + carry;
                product[i + j] = value as u64;
                carry = value >> 64;
            }
            product[i + fraction_words] = carry as u64;
        }
        let mut result = Self::ZERO;
        result.0[..fraction_words].copy_from_slice(&product[fraction_words..2 * fraction_words]);
        result
    }

    /// 2*atanh(numerator/denominator), where the ratio is in [0, 1/3].
    fn logarithm_series(numerator: u64, denominator: u64, fraction_words: usize) -> Self {
        let t = Self::ratio(numerator, denominator, fraction_words);
        let square = t.multiply_fraction(&t, fraction_words);
        let mut power = t;
        let mut sum = Self::ZERO;
        let mut divisor = 1;
        loop {
            let term = power.divide_word(divisor);
            if term.is_zero() {
                break;
            }
            sum.add(&term);
            power = power.multiply_fraction(&square, fraction_words);
            divisor += 2;
        }
        sum.multiply_word(2)
    }
}

struct SignedFixed {
    magnitude: Fixed,
    negative: bool,
}

impl SignedFixed {
    fn logarithm(value: Value, ln_two: &Fixed, fraction_words: usize) -> Self {
        let exponent = libm::ilogb(value);
        let mantissa = libm::scalbn(value, -exponent);
        let significand = (mantissa.to_bits() & MANTISSA_MASK) | (1 << 52);
        let fractional = Fixed::logarithm_series(
            significand - (1 << 52),
            significand + (1 << 52),
            fraction_words,
        );
        let mut magnitude = ln_two.multiply_word(u64::from(exponent.unsigned_abs()));
        if exponent < 0 {
            magnitude.subtract(&fractional);
        } else {
            magnitude.add(&fractional);
        }
        Self {
            magnitude,
            negative: exponent < 0,
        }
    }

    /// Add coefficient*logarithm after scaling all coefficients by 2^-scale.
    fn accumulate(&mut self, logarithm: &Self, coefficient: Value, scale: i32) {
        if coefficient == 0.0 || logarithm.magnitude.is_zero() {
            return;
        }
        let bits = coefficient.abs().to_bits();
        let encoded_exponent = ((bits >> 52) & 0x7ff) as i32;
        let (mantissa, exponent) = if encoded_exponent == 0 {
            (bits & MANTISSA_MASK, -1074)
        } else {
            (
                (bits & MANTISSA_MASK) | (1 << 52),
                encoded_exponent - 1023 - 52,
            )
        };
        let magnitude = logarithm
            .magnitude
            .multiply_word(mantissa)
            .shift_right((scale - exponent) as usize);
        let negative = logarithm.negative ^ coefficient.is_sign_negative();
        if self.negative == negative {
            self.magnitude.add(&magnitude);
        } else if self.magnitude.cmp(&magnitude) == Ordering::Less {
            let mut difference = magnitude;
            difference.subtract(&self.magnitude);
            self.magnitude = difference;
            self.negative = negative;
        } else {
            self.magnitude.subtract(&magnitude);
        }
    }
}

/// coefficient*2^binary_scale*current^current_exponent/frequency^frequency_exponent.
/// Bases/coefficient must be positive finite and exponents finite. The caller
/// keeps ordinary powers/products on its fast path and handles exact-zero bases.
#[cold]
pub(crate) fn scaled_power_law(
    coefficient: Value,
    binary_scale: i32,
    current: Value,
    current_exponent: Value,
    frequency: Value,
    frequency_exponent: Value,
) -> Value {
    let scale = libm::ilogb(
        current_exponent
            .abs()
            .max(frequency_exponent.abs())
            .max(Value::from(binary_scale).abs())
            .max(1.0),
    );
    let fraction_words = (scale as usize + 128).div_ceil(64);
    let ln_two = Fixed::logarithm_series(1, 3, fraction_words);
    let mut logarithm = SignedFixed {
        magnitude: Fixed::ZERO,
        negative: false,
    };
    for (base, exponent) in [
        (coefficient, 1.0),
        (current, current_exponent),
        (frequency, -frequency_exponent),
    ] {
        if exponent != 0.0 && base != 1.0 {
            let term = SignedFixed::logarithm(base, &ln_two, fraction_words);
            logarithm.accumulate(&term, exponent, scale);
        }
    }
    logarithm.accumulate(
        &SignedFixed {
            magnitude: ln_two,
            negative: false,
        },
        Value::from(binary_scale),
        scale,
    );
    let Some(highest_bit) = logarithm.magnitude.highest_bit() else {
        return 1.0;
    };
    let binary_scale = scale - (fraction_words * 64) as i32;
    let exponent = highest_bit as i32 + binary_scale;
    // All factors have already been composed. These magnitudes unambiguously
    // overflow/underflow exp(), or round exp() to one, respectively.
    if exponent >= 10 {
        return if logarithm.negative {
            0.0
        } else {
            Value::INFINITY
        };
    }
    if exponent < -56 {
        return 1.0;
    }
    let value = logarithm.magnitude.rounded_float(binary_scale, highest_bit);
    let value = if logarithm.negative { -value } else { value };
    // Reduce before converting the residual to binary64. Rounding a large
    // logarithm directly would lose relative precision in the final density.
    let power = (value / core::f64::consts::LN_2).round() as i32;
    logarithm.accumulate(
        &SignedFixed {
            magnitude: ln_two,
            negative: false,
        },
        -Value::from(power),
        scale,
    );
    let residual = logarithm.magnitude.highest_bit().map_or(0.0, |bit| {
        let value = logarithm.magnitude.rounded_float(binary_scale, bit);
        if logarithm.negative { -value } else { value }
    });
    libm::scalbn(residual.exp(), power)
}
