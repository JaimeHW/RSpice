//! Exact complex polynomial evaluation and bounded unit-circle responses.
//!
//! Frequency-period products are reduced as exact dyadics. Axis values and
//! true poles use exact arithmetic; ordinary responses use propagated error
//! bounds. Cancellation and range failures recover through bounded extended
//! precision, preserving exact component zeros and final rounding boundaries.

use crate::arithmetic::{
    ArithmeticError, BigMagnitude, SmallExact, scaled_exact_ratio_to_f64, small_exact_ratio_to_f64,
};
mod bounded;
mod recovery;
use bounded::{dd_circle, ordinary_ratio};

/// Evaluate real-coefficient polynomials at exp(-2*pi*i*frequency*period).
///
/// Coefficients are in ascending powers. Inputs must be finite. Exact poles,
/// nonzero components rounded to zero, and final overflow are reported as
/// errors. Recovery that cannot certify a component within its bounded
/// precision budget returns [`ArithmeticError::PrecisionLimit`].
pub fn unit_circle_polynomial_ratio(
    n: &[f64],
    d: &[f64],
    f: f64,
    p: f64,
) -> Result<[f64; 2], ArithmeticError> {
    if n.iter().chain(d).chain([&f, &p]).any(|v| !v.is_finite()) {
        return Err(ArithmeticError::NonFiniteTerm);
    }
    let n = trim_coefficients(n);
    let d = trim_coefficients(d);
    let (quarter, offset, bits) = reduced_product_phase(f, p);
    if offset.value == 0 {
        return axis_ratio(n, d, quarter);
    }
    if has_dyadic_root_factor(d, bits)? {
        return Err(ArithmeticError::ZeroDenominator);
    }
    if has_dyadic_root_factor(n, bits)? {
        return Ok([0.0, 0.0]);
    }
    let mut exact_angle = SmallExact::product(offset.value as f64, -std::f64::consts::TAU).unwrap();
    exact_angle.exponent += offset.exponent;
    let theta = small_exact_ratio_to_f64(exact_angle, small_finite(1.0)).unwrap()?;
    if theta == 0.0 {
        return recover_ratio(n, d, f, p);
    }
    let sine = if theta.abs() < 2.0_f64.powi(-30) {
        theta
    } else {
        sine_kernel(theta)
    };
    let half_sine = if theta.abs() < 2.0_f64.powi(-30) {
        theta * 0.5
    } else {
        sine_kernel(theta * 0.5)
    };
    let delta = (-2.0 * half_sine) * half_sine;
    let cosine = 1.0 + delta;
    let cosine_error = (((delta - (cosine - 1.0)).abs() + 8.0 * f64::EPSILON * delta.abs())
        * (1.0 + 8.0 * f64::EPSILON))
        .next_up();
    let sine_error = (4.0 * f64::EPSILON * sine.abs()).next_up();
    let argument = match quarter {
        0 => [cosine, sine],
        1 => [sine, -cosine],
        2 => [-cosine, -sine],
        _ => [-sine, cosine],
    };
    let error = if quarter % 2 == 0 {
        [cosine_error, sine_error]
    } else {
        [sine_error, cosine_error]
    };
    // Use the unit-circle norm when rectangular radii amplify at each step.
    // Near the axes, rectangular bounds better preserve small components.
    let ordinary = if n.len().max(d.len()) >= 8 && argument[0].abs().min(argument[1].abs()) > 0.125
    {
        bounded::ordinary_circle_ratio(n, d, argument, error)
            .or_else(|| ordinary_ratio(n, d, argument, error))
    } else {
        ordinary_ratio(n, d, argument, error)
    };
    if let Some(value) = ordinary {
        return Ok(value);
    }
    if let Some(value) = dd_circle(n, d, quarter, offset) {
        return Ok(value);
    }
    recover_ratio(n, d, f, p)
}

fn axis_ratio(
    numerator: &[f64],
    denominator: &[f64],
    quarter: u8,
) -> Result<[f64; 2], ArithmeticError> {
    if quarter.is_multiple_of(2) {
        let term = |(index, &value): (usize, &f64)| {
            (
                value,
                if quarter == 2 && index % 2 != 0 {
                    -1.0
                } else {
                    1.0
                },
            )
        };
        let n = numerator.iter().enumerate().map(term);
        let d = denominator.iter().enumerate().map(term);
        let real = crate::arithmetic::sum_products_ratio(n.clone(), d)?;
        return Ok([
            nonzero_component(
                real,
                real != 0.0 || crate::arithmetic::sum_products_is_zero(n)?,
            )?,
            0.0,
        ]);
    }
    let argument = [0.0, if quarter == 1 { -1.0 } else { 1.0 }];
    ordinary_ratio(numerator, denominator, argument, [0.0; 2])
        .map(Ok)
        .unwrap_or_else(|| complex_polynomial_ratio(numerator, denominator, argument))
}
fn trim_coefficients(coefficients: &[f64]) -> &[f64] {
    &coefficients[..coefficients
        .iter()
        .rposition(|&v| v != 0.0)
        .map_or(0, |i| i + 1)]
}
#[cold]
#[inline(never)]
fn recover_ratio(n: &[f64], d: &[f64], f: f64, p: f64) -> Result<[f64; 2], ArithmeticError> {
    if let Some(result) = reduce_polynomial_ratio(n, d, f, p) {
        return result;
    }
    recovery::recover(n, d, f, p)
}

fn exact_complex_product(a: &[Dyadic; 2], b: &[Dyadic; 2]) -> Result<[Dyadic; 2], ArithmeticError> {
    Ok([
        a[0].multiply(&b[0])?
            .add(&a[1].multiply(&b[1])?.negated())?,
        a[0].multiply(&b[1])?.add(&a[1].multiply(&b[0])?)?,
    ])
}

/// Return a quadrant and an exact signed offset of at most one eighth cycle.
/// A finite binary64 product has at most 106 significant bits, so fractional
/// reduction needs no heap allocation or overflowing/underflowing float product.
fn reduced_product_phase(frequency: f64, period: f64) -> (u8, SmallExact, u32) {
    if frequency == 0.0 || period == 0.0 {
        return (0, SmallExact::ZERO, 0);
    }
    let product = SmallExact::product(frequency, period).expect("finite product fits i128");
    reduced_phase(product)
}

fn reduced_phase(product: SmallExact) -> (u8, SmallExact, u32) {
    if product.value == 0 || product.exponent >= 0 {
        return (0, SmallExact::ZERO, 0);
    }
    let fraction_bits = -product.exponent as u32;
    let magnitude = product.value.unsigned_abs();
    let fraction = if fraction_bits >= 128 {
        magnitude
    } else {
        magnitude & ((1_u128 << fraction_bits) - 1)
    };
    let (quarter, remainder) = if fraction_bits <= 2 {
        ((fraction << (2 - fraction_bits)) as u8, 0_i128)
    } else {
        let offset_bits = fraction_bits - 2;
        if offset_bits >= 128 {
            (0, fraction as i128)
        } else {
            let quarter = (fraction >> offset_bits) as u8;
            let remainder = fraction & ((1_u128 << offset_bits) - 1);
            if remainder >= 1_u128 << (offset_bits - 1) {
                // The significand's 106-bit ceiling implies offset_bits<=106
                // whenever this branch is taken; both signed values fit i128.
                (quarter + 1, remainder as i128 - (1_i128 << offset_bits))
            } else {
                (quarter, remainder as i128)
            }
        }
    };
    let (quarter, remainder) = if product.value < 0 {
        ((4 - quarter) % 4, -remainder)
    } else {
        (quarter % 4, remainder)
    };
    (
        quarter,
        SmallExact::normalized(remainder, product.exponent)
            .expect("reduced exponent stays in range"),
        fraction_bits,
    )
}

fn has_dyadic_root_factor(coefficients: &[f64], cycle_bits: u32) -> Result<bool, ArithmeticError> {
    let Some(degree) = cycle_bits
        .checked_sub(1)
        .and_then(|power| 1_usize.checked_shl(power))
    else {
        return Ok(false);
    };
    if degree >= coefficients.len() {
        return Ok(false);
    }
    for remainder in 0..degree {
        let terms = coefficients
            .iter()
            .skip(remainder)
            .step_by(degree)
            .enumerate()
            .map(|(block, &coefficient)| (coefficient, if block % 2 == 0 { 1.0 } else { -1.0 }));
        if !crate::arithmetic::sum_products_is_zero(terms)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn rotate_exact([real, imaginary]: [Dyadic; 2], quarter: u8) -> [Dyadic; 2] {
    match quarter {
        0 => [real, imaginary],
        1 => [imaginary, real.negated()],
        2 => [real.negated(), imaginary.negated()],
        _ => [imaginary.negated(), real],
    }
}

fn exact_complex_ratio(n: [Dyadic; 2], d: [Dyadic; 2]) -> Result<[f64; 2], ArithmeticError> {
    let norm = d[0].multiply(&d[0])?.add(&d[1].multiply(&d[1])?)?;
    if norm.magnitude.is_zero() {
        return Err(ArithmeticError::ZeroDenominator);
    }
    let real = n[0].multiply(&d[0])?.add(&n[1].multiply(&d[1])?)?;
    let imaginary = n[1]
        .multiply(&d[0])?
        .add(&n[0].multiply(&d[1])?.negated())?;
    Ok([real.divide(&norm)?, imaginary.divide(&norm)?])
}

/// Evaluate a complex polynomial response with a checked relative error bound.
///
/// Coefficients are in ascending powers. Binary64 and two-component arithmetic
/// are accepted only when each component's propagated error is at most
/// `8 * f64::EPSILON * (numerator.len() + denominator.len() + 1)` times its
/// magnitude. Otherwise [`complex_polynomial_ratio`] recovers the correctly
/// rounded result. Exact zeros and final range failures retain that function's
/// contract; intermediate overflow or underflow does not discard the response.
pub fn bounded_complex_polynomial_ratio(
    numerator: &[f64],
    denominator: &[f64],
    argument: [f64; 2],
) -> Result<[f64; 2], ArithmeticError> {
    if numerator
        .iter()
        .chain(denominator)
        .chain(&argument)
        .any(|v| !v.is_finite())
    {
        return Err(ArithmeticError::NonFiniteTerm);
    }
    let numerator = trim_coefficients(numerator);
    let denominator = trim_coefficients(denominator);
    ordinary_ratio(numerator, denominator, argument, [0.0; 2])
        .or_else(|| bounded::dd_complex_ratio(numerator, denominator, argument))
        .map(Ok)
        .unwrap_or_else(|| complex_polynomial_ratio(numerator, denominator, argument))
}

/// Evaluate a ratio of real-coefficient polynomials at a complex binary64
/// argument. Horner products, coefficient sums and the conjugate division are
/// exact; only the two final components are rounded. This is the recovery path
/// for ill-conditioned or out-of-range frequency-domain intermediates.
/// A zero denominator, non-finite input, final overflow or nonzero component
/// below the binary64 rounding range is an error.
pub fn complex_polynomial_ratio(
    numerator: &[f64],
    denominator: &[f64],
    argument: [f64; 2],
) -> Result<[f64; 2], ArithmeticError> {
    if numerator
        .iter()
        .chain(denominator)
        .chain(&argument)
        .any(|v| !v.is_finite())
    {
        return Err(ArithmeticError::NonFiniteTerm);
    }
    let small_argument = argument.map(small_finite);
    if let Some(result) = small_polynomial(numerator, small_argument)
        .zip(small_polynomial(denominator, small_argument))
        .and_then(|(n, d)| small_complex_ratio(n, d))
    {
        return result;
    }
    let argument = small_argument.map(Dyadic::from_small);
    let n = exact_polynomial(numerator, &argument)?;
    let d = exact_polynomial(denominator, &argument)?;
    exact_complex_ratio(n, d)
}

fn small_finite(value: f64) -> SmallExact {
    if value == 0.0 {
        SmallExact::ZERO
    } else {
        SmallExact::product(value, 1.0).expect("one finite binary64 value fits SmallExact")
    }
}

impl SmallExact {
    fn checked_multiply(self, other: Self) -> Option<Self> {
        Self::normalized(
            self.value.checked_mul(other.value)?,
            self.exponent.checked_add(other.exponent)?,
        )
    }

    fn checked_negated(self) -> Option<Self> {
        Some(Self {
            value: self.value.checked_neg()?,
            exponent: self.exponent,
        })
    }
}

fn small_polynomial(coefficients: &[f64], argument: [SmallExact; 2]) -> Option<[SmallExact; 2]> {
    let [zr, zi] = argument;
    let mut value = [SmallExact::ZERO; 2];
    for &coefficient in coefficients.iter().rev() {
        let [re, im] = value;
        value = [
            re.checked_multiply(zr)?
                .checked_add(im.checked_multiply(zi)?.checked_negated()?)?
                .checked_add(small_finite(coefficient))?,
            re.checked_multiply(zi)?
                .checked_add(im.checked_multiply(zr)?)?,
        ];
    }
    Some(value)
}

fn small_complex_ratio(
    n: [SmallExact; 2],
    d: [SmallExact; 2],
) -> Option<Result<[f64; 2], ArithmeticError>> {
    let norm = d[0]
        .checked_multiply(d[0])?
        .checked_add(d[1].checked_multiply(d[1])?)?;
    if norm.value == 0 {
        return Some(Err(ArithmeticError::ZeroDenominator));
    }
    let real = n[0]
        .checked_multiply(d[0])?
        .checked_add(n[1].checked_multiply(d[1])?)?;
    let imaginary = n[1]
        .checked_multiply(d[0])?
        .checked_add(n[0].checked_multiply(d[1])?.checked_negated()?)?;
    let real_result = small_exact_ratio_to_f64(real, norm)?;
    let imaginary_result = small_exact_ratio_to_f64(imaginary, norm)?;
    Some((|| {
        Ok([
            nonzero_component(real_result?, real.value == 0)?,
            nonzero_component(imaginary_result?, imaginary.value == 0)?,
        ])
    })())
}

fn nonzero_component(value: f64, exactly_zero: bool) -> Result<f64, ArithmeticError> {
    if value == 0.0 && !exactly_zero {
        Err(ArithmeticError::Underflow)
    } else {
        Ok(value)
    }
}

/// An exact signed integer times a power of two. Normalization keeps long
/// Horner chains compact, especially for the four exact unit-circle axes.
#[derive(Clone)]
struct Dyadic {
    negative: bool,
    magnitude: BigMagnitude,
    exponent: i32,
}

impl Dyadic {
    fn from_small(value: SmallExact) -> Self {
        let mut magnitude = BigMagnitude::default();
        magnitude.add_shifted(value.value.unsigned_abs(), 0);
        Self {
            negative: value.value < 0,
            magnitude,
            exponent: value.exponent,
        }
    }

    fn negated(mut self) -> Self {
        self.negative = !self.negative;
        self
    }

    fn multiply(&self, other: &Self) -> Result<Self, ArithmeticError> {
        if self.magnitude.is_zero() || other.magnitude.is_zero() {
            return Ok(Self::from_small(SmallExact::ZERO));
        }
        Ok(Self {
            negative: self.negative ^ other.negative,
            magnitude: self.magnitude.multiply(&other.magnitude),
            exponent: self
                .exponent
                .checked_add(other.exponent)
                .ok_or(ArithmeticError::MantissaBounds)?,
        })
    }

    fn add(&self, other: &Self) -> Result<Self, ArithmeticError> {
        if self.magnitude.is_zero() {
            return Ok(other.clone());
        }
        if other.magnitude.is_zero() {
            return Ok(self.clone());
        }
        let exponent = self.exponent.min(other.exponent);
        let left_shift = self
            .exponent
            .checked_sub(exponent)
            .ok_or(ArithmeticError::MantissaBounds)? as usize;
        let right_shift = other
            .exponent
            .checked_sub(exponent)
            .ok_or(ArithmeticError::MantissaBounds)? as usize;
        let mut left = self.magnitude.shift_left(left_shift);
        let right = other.magnitude.shift_left(right_shift);
        let (negative, magnitude) = if self.negative == other.negative {
            for i in 0..right.significant_len() {
                left.add_word(i, right.word(i));
            }
            (self.negative, left)
        } else {
            match left.compare(&right) {
                std::cmp::Ordering::Greater => (self.negative, left.subtract(&right)),
                std::cmp::Ordering::Less => (other.negative, right.subtract(&left)),
                std::cmp::Ordering::Equal => return Ok(Self::from_small(SmallExact::ZERO)),
            }
        };
        let zeros = magnitude.trailing_zeros();
        Ok(Self {
            negative,
            magnitude: magnitude.shift_right(zeros),
            exponent: exponent
                .checked_add(i32::try_from(zeros).map_err(|_| ArithmeticError::MantissaBounds)?)
                .ok_or(ArithmeticError::MantissaBounds)?,
        })
    }

    fn divide(&self, other: &Self) -> Result<f64, ArithmeticError> {
        if other.magnitude.is_zero() {
            return Err(ArithmeticError::ZeroDenominator);
        }
        if self.magnitude.is_zero() {
            return Ok(0.0);
        }
        let exponent = self
            .exponent
            .checked_sub(other.exponent)
            .ok_or(ArithmeticError::MantissaBounds)?;
        nonzero_component(
            scaled_exact_ratio_to_f64(
                &self.magnitude,
                &other.magnitude,
                self.negative ^ other.negative,
                exponent,
            )?,
            false,
        )
    }
}

fn exact_polynomial(
    coefficients: &[f64],
    argument: &[Dyadic; 2],
) -> Result<[Dyadic; 2], ArithmeticError> {
    let mut value = [
        Dyadic::from_small(SmallExact::ZERO),
        Dyadic::from_small(SmallExact::ZERO),
    ];
    for &coefficient in coefficients.iter().rev() {
        let [re, im] = value;
        value = [
            re.multiply(&argument[0])?
                .add(&im.multiply(&argument[1])?.negated())?
                .add(&Dyadic::from_small(small_finite(coefficient)))?,
            re.multiply(&argument[1])?
                .add(&im.multiply(&argument[0])?)?,
        ];
    }
    Ok(value)
}

impl BigMagnitude {
    fn multiply(&self, other: &Self) -> Self {
        let mut result = Self::default();
        for i in 0..self.significant_len() {
            let left = self.word(i);
            if left == 0 {
                continue;
            }
            for j in 0..other.significant_len() {
                result.add_shifted(u128::from(left) * u128::from(other.word(j)), (i + j) * 64);
            }
        }
        result
    }

    fn trailing_zeros(&self) -> usize {
        for i in 0..self.significant_len() {
            let word = self.word(i);
            if word != 0 {
                return i * 64 + word.trailing_zeros() as usize;
            }
        }
        0
    }

    fn shift_right(&self, shift: usize) -> Self {
        let mut result = Self::default();
        let words = shift / 64;
        let bits = shift % 64;
        for i in words..self.significant_len() {
            let value = if bits == 0 {
                self.word(i)
            } else {
                (self.word(i) >> bits) | (self.word(i + 1) << (64 - bits))
            };
            result.set_word(i - words, value);
        }
        result
    }
}

// |x| <= pi/4 < 4/5. Let u=2^-53 and gamma(k)=ku/(1-ku).
// The degree-17 Horner polynomial's constant term accumulates gamma(2);
// term r>0 accumulates at most gamma(3r+3), including x*x, its rounded
// coefficient, and the final product. The alternating remainder is at most
// |x|^19/19!. Together with three roundings in the physical-angle seed,
// the error is <6.351u relative to the returned sine (the caller reserves8u).
// Squaring the half-angle sine gives <13.701u relative delta error (16u
// reserved). Arguments below2^-30 use the separately bounded linear term.
fn sine_kernel(x: f64) -> f64 {
    let t = x * x;
    let mut h = 1.0 / 355687428096000.0; // 17!
    for coefficient in [
        -1.0 / 1307674368000.0,
        1.0 / 6227020800.0,
        -1.0 / 39916800.0,
        1.0 / 362880.0,
        -1.0 / 5040.0,
        1.0 / 120.0,
        -1.0 / 6.0,
        1.0,
    ] {
        h = coefficient + t * h;
    }
    x * h
}

// A bounded optimization: accept only a binary64 quotient whose polynomial
// identity is verified in exact arithmetic. Other cases retain full recovery.
fn exact_polynomial_quotient(numerator: &[f64], denominator: &[f64]) -> Option<Vec<f64>> {
    let n_len = numerator.iter().rposition(|&v| v != 0.0)? + 1;
    let d_len = denominator.iter().rposition(|&v| v != 0.0)? + 1;
    if d_len < 2 || n_len < d_len || n_len - d_len > 8 {
        return None;
    }
    let mut quotient = vec![0.0; n_len - d_len + 1];
    for k in (0..quotient.len()).rev() {
        let index = k + d_len - 1;
        let terms = std::iter::once((numerator[index], 1.0)).chain(
            (k + 1..quotient.len())
                .filter_map(|j| index.checked_sub(j).map(|i| (-quotient[j], denominator[i]))),
        );
        quotient[k] = crate::arithmetic::sum_products_ratio(
            terms,
            std::iter::once((denominator[d_len - 1], 1.0)),
        )
        .ok()?;
        if !quotient[k].is_finite() {
            return None;
        }
    }
    for (index, &coefficient) in numerator[..n_len].iter().enumerate() {
        let products = quotient.iter().enumerate().filter_map(|(j, &q)| {
            index
                .checked_sub(j)
                .filter(|&i| i < d_len)
                .map(|i| (q, denominator[i]))
        });
        if !crate::arithmetic::sum_products_is_zero(
            products.chain(std::iter::once((-coefficient, 1.0))),
        )
        .ok()?
        {
            return None;
        }
    }
    Some(quotient)
}
fn reduce_polynomial_ratio(
    n: &[f64],
    d: &[f64],
    f: f64,
    p: f64,
) -> Option<Result<[f64; 2], ArithmeticError>> {
    let reduction = exact_polynomial_quotient(n, d)
        .map(|q| (q, false))
        .or_else(|| exact_polynomial_quotient(d, n).map(|q| (q, true)))?;
    // Preserve the original denominator's pole diagnostics even when its
    // factor could be cancelled algebraically from the rational expression.
    let (_, _, bits) = reduced_product_phase(f, p);
    match has_dyadic_root_factor(d, bits) {
        Ok(true) => return Some(Err(ArithmeticError::ZeroDenominator)),
        Err(error) => return Some(Err(error)),
        Ok(false) => {}
    }
    Some(if reduction.1 {
        unit_circle_polynomial_ratio(&[1.0], &reduction.0, f, p)
    } else {
        unit_circle_polynomial_ratio(&reduction.0, &[1.0], f, p)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_complex_response_recovers_small_reactive_components() {
        let numerator: Vec<_> = (1..=16).map(|i| 0.3 / f64::from(i)).collect();
        let denominator: Vec<_> = (1..=16)
            .map(|i| if i == 1 { 1.0 } else { -0.15 / f64::from(i) })
            .collect();
        let argument = [0.0, f64::from_bits(0x4002_bfc0_197c_9660)];
        // Exact rational Horner evaluation at the binary64 argument, with
        // one final ties-to-even rounding per component.
        let expected = [
            f64::from_bits(0xbfff_feb4_d18d_83fa),
            f64::from_bits(0xbf46_c127_9873_db09),
        ];
        for value in [
            bounded::dd_complex_ratio(&numerator, &denominator, argument)
                .expect("bounded recovery resolves the reactive cancellation"),
            bounded_complex_polynomial_ratio(&numerator, &denominator, argument).unwrap(),
        ] {
            for (actual, expected) in value.into_iter().zip(expected) {
                assert!((actual / expected - 1.0).abs() <= 128.0 * f64::EPSILON);
            }
        }
        assert_eq!(
            bounded_complex_polynomial_ratio(&[1.0], &[0.0], argument),
            Err(ArithmeticError::ZeroDenominator)
        );
        assert_eq!(
            bounded_complex_polynomial_ratio(&[f64::MIN_POSITIVE], &[f64::MAX], argument),
            Err(ArithmeticError::Underflow)
        );
    }

    #[test]
    fn exact_phase_retains_fractional_cycles_after_wide_products() {
        assert_eq!(
            unit_circle_polynomial_ratio(&[0.0, 1.0], &[1.0], 0.0, 1.0),
            Ok([1.0, 0.0])
        );
        assert_eq!(
            unit_circle_polynomial_ratio(&[0.0, 1.0], &[1.0], 1.0, 0.0),
            Ok([1.0, 0.0])
        );
        for (factor, expected) in [(1.25, [0.0, -1.0]), (1.5, [-1.0, 0.0]), (1.75, [0.0, 1.0])] {
            assert_eq!(
                unit_circle_polynomial_ratio(
                    &[0.0, 1.0],
                    &[1.0],
                    factor * (1_u64 << 52) as f64,
                    1.0 + f64::EPSILON
                ),
                Ok(expected)
            );
        }
        assert_eq!(
            unit_circle_polynomial_ratio(&[0.0, 1.0], &[1.0], f64::MAX, f64::MAX),
            Ok([1.0, 0.0])
        );
        let (quarter, offset, cycle_bits) =
            reduced_product_phase(f64::from_bits(1), f64::from_bits(1));
        assert_eq!(cycle_bits, 2148);
        assert_eq!(quarter, 0);
        assert_eq!(
            offset,
            SmallExact {
                value: 1,
                exponent: -2148
            }
        );
        for frequency in [0.25 - f64::EPSILON, 0.25 + f64::EPSILON] {
            let (_, offset, _) = reduced_product_phase(frequency, 1.0);
            assert_ne!(
                offset.value, 0,
                "neighbors must not become exact cardinal phases"
            );
        }
    }

    #[test]
    fn quarter_cycle_residuals_and_poles_are_exact() {
        for residual in [1.0, -1e-200, f64::from_bits(1)] {
            for (frequency, sign) in [(0.25, -1.0), (0.75, 1.0), (1.25, -1.0), (1.75, 1.0)] {
                assert_eq!(
                    unit_circle_polynomial_ratio(&[1e200, residual, 1e200], &[1.0], frequency, 1.0),
                    Ok([0.0, sign * residual])
                );
            }
        }
        for frequency in [0.25, 0.75] {
            assert_eq!(
                unit_circle_polynomial_ratio(&[1.0], &[1.0, 0.0, 1.0], frequency, 1.0),
                Err(ArithmeticError::ZeroDenominator)
            );
        }
        let max = f64::MAX;
        assert_eq!(
            unit_circle_polynomial_ratio(&[max, max, -max], &[max, -max, -max], 0.75, 1.0),
            Ok([0.6, 0.8])
        );
        let square = f64::EPSILON * f64::EPSILON;
        assert_eq!(
            complex_polynomial_ratio(&[1.0, 1.0, square], &[1.0, 1.0], [0.0, 1.0]),
            Ok([1.0, square / 2.0])
        );
    }

    #[test]
    fn tiny_angles_retain_cosine_cancellation_and_sine_range() {
        let result = unit_circle_polynomial_ratio(&[0.0, 1e300], &[1.0], 1e-200, 1e-200).unwrap();
        assert_eq!(result[0], 1e300);
        assert!((result[1] / (-std::f64::consts::TAU * 1e-100) - 1.0).abs() < 4.0 * f64::EPSILON);
        let result = unit_circle_polynomial_ratio(&[1e300, -1e300], &[1.0], 1e-160, 1.0).unwrap();
        let tau = std::f64::consts::TAU;
        for (actual, expected) in result
            .into_iter()
            .zip([0.5 * tau * tau * 1e-20, tau * 1e140])
        {
            assert!(
                (actual / expected - 1.0).abs() < 4.0 * f64::EPSILON,
                "{actual:e} vs {expected:e}"
            );
        }
        assert_eq!(
            unit_circle_polynomial_ratio(&[1e300, -1e300], &[1.0], 1e-200, 1e-200),
            Err(ArithmeticError::Underflow)
        );
    }

    #[test]
    fn near_unity_poles_retain_the_real_component_across_phase_scales() {
        for power in [-100, -40, -32, -31, -30, -29, -28, -27, -10] {
            let frequency = 2.0_f64.powi(power);
            let response =
                unit_circle_polynomial_ratio(&[1.0], &[1.0, -1.0], frequency, 1.0).unwrap();
            assert!(
                (response[0] - 0.5).abs() <= 4.0 * f64::EPSILON,
                "power={power}, response={response:?}"
            );
            let angle = std::f64::consts::TAU * frequency;
            let expected = -0.5 / (angle * 0.5).tan();
            assert!(
                (response[1] / expected - 1.0).abs() <= 8.0 * f64::EPSILON,
                "power={power}, response={response:?}"
            );
        }
    }

    #[test]
    fn finer_dyadic_phases_detect_exact_poles_and_transmission_zeros() {
        for degree in [4, 8, 16, 32, 64, 128, 256, 512] {
            let mut factor = vec![0.0; degree + 1];
            factor[0] = 1.0;
            factor[degree] = 1.0;
            for multiple in [1.0, 3.0, 5.0] {
                let frequency = multiple / (2 * degree) as f64;
                assert_eq!(
                    unit_circle_polynomial_ratio(&[1.0], &factor, frequency, 1.0),
                    Err(ArithmeticError::ZeroDenominator)
                );
                assert_eq!(
                    unit_circle_polynomial_ratio(&factor, &[1.0], frequency, 1.0),
                    Ok([0.0, 0.0])
                );
                // A nonzero remainder must not be erased by an approximate
                // coefficient norm or a trigonometric near-zero threshold.
                let mut residual = factor.clone();
                residual[1] = f64::from_bits(1);
                assert!(!has_dyadic_root_factor(&residual, (2 * degree).trailing_zeros()).unwrap());
            }
        }
        let near = f64::from_bits(0.125_f64.to_bits() + 1);
        assert!(
            !has_dyadic_root_factor(
                &[1.0, 0.0, 0.0, 0.0, 1.0],
                reduced_product_phase(near, 1.0).2
            )
            .unwrap()
        );
    }

    #[test]
    fn neighbors_of_dyadic_poles_retain_phase_distance_and_repeated_factors() {
        for degree in [1, 2, 4, 8, 16, 64] {
            let center = 0.5 / degree as f64;
            let mut denominator = vec![0.0; degree + 1];
            denominator[0] = 1.0;
            denominator[degree] = 1.0;
            for (frequency, period, distance) in [
                (center.next_down(), 1.0, center.next_down() - center),
                (center.next_up(), 1.0, center.next_up() - center),
                (
                    center * (1.0 + f64::EPSILON),
                    1.0 - f64::EPSILON,
                    -center * f64::EPSILON * f64::EPSILON,
                ),
            ] {
                let angle = std::f64::consts::TAU * degree as f64 * distance;
                let imaginary = -0.5 / (angle * 0.5).tan();
                let response =
                    unit_circle_polynomial_ratio(&[1.0], &denominator, frequency, period).unwrap();
                for (actual, expected) in response.into_iter().zip([0.5, imaginary]) {
                    assert!(
                        (actual / expected - 1.0).abs() <= 8.0 * f64::EPSILON,
                        "degree={degree}, f={frequency}, period={period}: {actual:e} vs {expected:e}"
                    );
                }
                let mut repeated = vec![0.0; 2 * degree + 1];
                repeated[0] = 1.0;
                repeated[degree] = 2.0;
                repeated[2 * degree] = 1.0;
                let response =
                    unit_circle_polynomial_ratio(&[1.0], &repeated, frequency, period).unwrap();
                for (actual, expected) in response
                    .into_iter()
                    .zip([0.25 - imaginary * imaginary, imaginary])
                {
                    assert!((actual / expected - 1.0).abs() <= 8.0 * f64::EPSILON);
                }
            }
        }
    }

    #[test]
    fn final_range_and_nonfinite_coefficients_are_not_hidden() {
        let tiny = f64::from_bits(1);
        assert_eq!(
            complex_polynomial_ratio(&[tiny], &[2.0], [0.0, 1.0]),
            Err(ArithmeticError::Underflow)
        );
        assert_eq!(
            complex_polynomial_ratio(&[f64::MAX], &[tiny], [0.0, 1.0]),
            Err(ArithmeticError::Overflow { negative: false })
        );
        assert_eq!(
            unit_circle_polynomial_ratio(&[1.0, f64::INFINITY], &[1.0], 0.0, 1.0),
            Err(ArithmeticError::NonFiniteTerm)
        );
        assert_eq!(
            unit_circle_polynomial_ratio(&[1.0], &[1.0], f64::NAN, 1.0),
            Err(ArithmeticError::NonFiniteTerm)
        );
    }
    #[test]
    fn non_axis_cardinal_components_preserve_zeros_and_round_ties() {
        let tiny = f64::from_bits(1);
        for degree in [2, 4, 16, 128] {
            for amplitude in [1.0, 3.0 * tiny, 5.0 * tiny] {
                let mut numerator = vec![0.0; degree + 1];
                numerator[degree] = amplitude;
                let rounded_gain = amplitude / 2.0;
                for (quarter, expected) in [
                    (1.0, [0.0, -rounded_gain]),
                    (2.0, [-rounded_gain, 0.0]),
                    (3.0, [0.0, rounded_gain]),
                    (-1.0, [0.0, rounded_gain]),
                ] {
                    assert_eq!(
                        unit_circle_polynomial_ratio(
                            &numerator,
                            &[2.0],
                            quarter / (4 * degree) as f64,
                            1.0
                        ),
                        Ok(expected),
                        "degree={degree}, amplitude={amplitude:e}, quarter={quarter}"
                    );
                }
            }
        }
    }

    #[test]
    fn partial_component_rounding_boundaries_preserve_range_diagnostics() {
        let tiny = f64::from_bits(1);
        // The real component is exactly half-minsub while the imaginary
        // component is nonzero: 2*tiny/(2+2*z) has real part tiny/2.
        assert_eq!(
            unit_circle_polynomial_ratio(&[2.0 * tiny], &[2.0, 2.0], 0.373, 1.0),
            Err(ArithmeticError::Underflow)
        );
        for degree in [2, 16, 128] {
            let mut numerator = vec![0.0; degree + 2];
            numerator[degree] = -1025.0 * tiny;
            numerator[degree + 1] = 1023.0 * tiny;
            // Here z^degree=-i. The real component is representable and
            // the imaginary component is exactly half-minsub.
            assert_eq!(
                unit_circle_polynomial_ratio(
                    &numerator,
                    &[2.0, 2.0],
                    1.0 / (4 * degree) as f64,
                    1.0
                ),
                Err(ArithmeticError::Underflow)
            );
        }
        for sign in [-1.0, 1.0] {
            let mut numerator = vec![0.0; 9];
            numerator[0] = sign * f64::MAX;
            numerator[2] = -17.0;
            numerator[8] = sign * 2.0_f64.powi(970);
            assert_eq!(
                unit_circle_polynomial_ratio(&numerator, &[1.0], 0.125, 1.0),
                Err(ArithmeticError::Overflow {
                    negative: sign < 0.0
                })
            );
            for frequency in [0.125_f64.next_down(), 0.125_f64.next_up()] {
                let value =
                    unit_circle_polynomial_ratio(&numerator, &[1.0], frequency, 1.0).unwrap();
                assert_eq!(value[0], sign * f64::MAX);
                assert!(value[1].is_finite());
            }
        }
    }

    #[test]
    fn non_dyadic_pole_neighbors_and_reactive_symmetry_are_preserved() {
        // Rounded from 250-digit evaluation of 0.5*tan(pi*degree*f), with
        // each binary64 frequency represented as an exact rational.
        for (degree, frequency_bits, imaginary_bits) in [
            (3, 0x3fc5555555555554, 0x43145f306dc9c883),
            (3, 0x3fc5555555555556, 0xc3245f306dc9c883),
            (5, 0x3fb9999999999999, 0x432b2995e7b7b604),
            (5, 0x3fb999999999999b, 0xc317483758e69c03),
            (6, 0x3fb5555555555554, 0x43145f306dc9c883),
            (6, 0x3fb5555555555556, 0xc3245f306dc9c883),
            (10, 0x3fa9999999999999, 0x432b2995e7b7b604),
            (10, 0x3fa999999999999b, 0xc317483758e69c03),
        ] {
            let frequency = f64::from_bits(frequency_bits);
            let imaginary = f64::from_bits(imaginary_bits);
            let mut denominator = vec![0.0; degree + 1];
            denominator[0] = 1.0;
            denominator[degree] = 1.0;
            let mut numerator = denominator.clone();
            numerator[degree] = -1.0;
            for (n, expected) in [
                (&[1.0][..], [0.5, imaginary]),
                (&numerator[..], [0.0, 2.0 * imaginary]),
            ] {
                let actual = unit_circle_polynomial_ratio(n, &denominator, frequency, 1.0).unwrap();
                for (actual, expected) in actual.into_iter().zip(expected) {
                    if expected == 0.0 {
                        assert_eq!(actual, 0.0);
                    } else {
                        assert!(
                            (actual / expected - 1.0).abs() <= 8.0 * f64::EPSILON,
                            "degree={degree}, f={frequency:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn trailing_zeros_do_not_inflate_recovery_degree() {
        let mut numerator = vec![0.0; 1003];
        let mut denominator = vec![0.0; 1001];
        numerator[2] = 1.0;
        denominator[0] = 1.0;
        assert_eq!(
            unit_circle_polynomial_ratio(&numerator, &denominator, 0.125, 1.0),
            Ok([0.0, -1.0])
        );
    }
}
