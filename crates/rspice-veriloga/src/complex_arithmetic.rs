//! Range-protected complex arithmetic shared by filters and AC evaluation.

use num_complex::Complex64;
use rspice_veriloga_runtime::arithmetic::ScaledValue as Component;
use rspice_veriloga_runtime::arithmetic::{ArithmeticError, sum_products, sum_products_ratio};

fn ordinary_product_sum(pairs: [(f64, f64); 2]) -> Option<f64> {
    let products = pairs.map(|(a, b)| a * b);
    let sum = products[0] + products[1];
    // Near cancellation amplifies the rounding errors of the products.
    // Preserve the complete sum when more than one bit is lost.
    if sum.abs() < 0.5 * products[0].abs().max(products[1].abs()) {
        return None;
    }
    (pairs
        .into_iter()
        .zip(products)
        .all(|((a, b), product)| product.is_normal() || (product == 0.0 && (a == 0.0 || b == 0.0)))
        && (sum.is_normal() || (products[0] == 0.0 && products[1] == 0.0)))
        .then_some(sum)
}

fn arithmetic_value(result: Result<f64, ArithmeticError>) -> f64 {
    match result {
        Ok(value) => value,
        Err(ArithmeticError::Overflow { negative: true }) => f64::NEG_INFINITY,
        Err(ArithmeticError::Overflow { negative: false }) => f64::INFINITY,
        Err(_) => f64::NAN,
    }
}

/// Recover finite complex products whose cross-products overflow, underflow,
/// or cancel. An axis operand has only one nonzero product per component and
/// can use ordinary multiplication without losing an intermediate sum.
#[inline]
pub(crate) fn multiply_complex(left: Complex64, right: Complex64) -> Complex64 {
    if left.im == 0.0
        || right.im == 0.0
        || left.re == 0.0
        || right.re == 0.0
        || ![left.re, left.im, right.re, right.im]
            .into_iter()
            .all(f64::is_finite)
    {
        return left * right;
    }
    let component = |pairs: [(f64, f64); 2]| {
        ordinary_product_sum(pairs)
            .unwrap_or_else(|| arithmetic_value(sum_products(pairs.into_iter())))
    };
    Complex64::new(
        component([(left.re, right.re), (-left.im, right.im)]),
        component([(left.re, right.im), (left.im, right.re)]),
    )
}

/// Use direct division on the axes and ordinary products in a safe range.
/// Exact product sums recover intermediate overflow, underflow, and cancellation.
#[inline]
pub(crate) fn divide_complex(left: Complex64, right: Complex64) -> Complex64 {
    if ![left.re, left.im, right.re, right.im]
        .into_iter()
        .all(f64::is_finite)
        || right == Complex64::new(0.0, 0.0)
    {
        return left / right;
    }
    if right.im == 0.0 {
        return Complex64::new(left.re / right.re, left.im / right.re);
    }
    if right.re == 0.0 {
        return Complex64::new(left.im / right.im, -left.re / right.im);
    }
    let denominator = [(right.re, right.re), (right.im, right.im)];
    let real = [(left.re, right.re), (left.im, right.im)];
    let imaginary = [(left.im, right.re), (-left.re, right.im)];
    if let (Some(norm), Some(real), Some(imaginary)) = (
        ordinary_product_sum(denominator),
        ordinary_product_sum(real),
        ordinary_product_sum(imaginary),
    ) {
        return Complex64::new(real / norm, imaginary / norm);
    }
    let quotient = |numerator: [(f64, f64); 2]| {
        arithmetic_value(sum_products_ratio(
            numerator.into_iter(),
            denominator.into_iter(),
        ))
    };
    Complex64::new(quotient(real), quotient(imaginary))
}

pub(crate) fn sum_complex_products_div(pairs: &[[Complex64; 2]], divisor: Complex64) -> Complex64 {
    use rspice_veriloga_runtime::arithmetic::{sum_products_div_iter, sum_triple_products_ratio};
    if divisor.im == 0.0 {
        // Index the flattened components without allocating temporary vectors.
        let real = (0..pairs.len() * 2).map(|index| {
            let [a, b] = pairs[index / 2];
            if index % 2 == 0 {
                [a.re, b.re]
            } else {
                [-a.im, b.im]
            }
        });
        let imaginary = (0..pairs.len() * 2).map(|index| {
            let [a, b] = pairs[index / 2];
            if index % 2 == 0 {
                [a.re, b.im]
            } else {
                [a.im, b.re]
            }
        });
        return Complex64::new(
            sum_products_div_iter(real, divisor.re),
            sum_products_div_iter(imaginary, divisor.re),
        );
    }
    let denominator = [[divisor.re, divisor.re, 1.0], [divisor.im, divisor.im, 1.0]];
    let real = pairs.iter().flat_map(|&[a, b]| {
        [
            [a.re, b.re, divisor.re],
            [-a.im, b.im, divisor.re],
            [a.re, b.im, divisor.im],
            [a.im, b.re, divisor.im],
        ]
    });
    let imaginary = pairs.iter().flat_map(|&[a, b]| {
        [
            [a.re, b.im, divisor.re],
            [a.im, b.re, divisor.re],
            [-a.re, b.re, divisor.im],
            [a.im, b.im, divisor.im],
        ]
    });
    Complex64::new(
        arithmetic_value(sum_triple_products_ratio(real, denominator)),
        arithmetic_value(sum_triple_products_ratio(imaginary, denominator)),
    )
}

/// Complex AC value whose components can cross binary64 range boundaries
/// independently. Conversion is deferred across arithmetic and assignments.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FrequencyValue {
    real: Component,
    imaginary: Component,
}

impl FrequencyValue {
    pub(crate) fn circular_derivative(
        self,
        origin: &rspice_veriloga_runtime::arithmetic::IdtModOrigin,
        phase: f64,
        modulus: f64,
        offset: f64,
        modulus_derivative: Self,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            real: origin.branch_derivative_scaled(
                phase,
                modulus,
                offset,
                self.real,
                modulus_derivative.real,
            )?,
            imaginary: origin.branch_derivative_scaled(
                phase,
                modulus,
                offset,
                self.imaginary,
                modulus_derivative.imaginary,
            )?,
        })
    }

    pub(crate) fn sum_products_div(
        pairs: &[[Self; 2]],
        divisor: Self,
    ) -> Result<Self, ArithmeticError> {
        if divisor.is_real() {
            let real = (0..pairs.len() * 2).map(|index| {
                let [a, b] = pairs[index / 2];
                if index % 2 == 0 {
                    [a.real, b.real]
                } else {
                    [a.imaginary.negated(), b.imaginary]
                }
            });
            let imaginary = (0..pairs.len() * 2).map(|index| {
                let [a, b] = pairs[index / 2];
                if index % 2 == 0 {
                    [a.real, b.imaginary]
                } else {
                    [a.imaginary, b.real]
                }
            });
            return Ok(Self {
                real: Component::sum_products_div(real, divisor.real)?,
                imaginary: Component::sum_products_div(imaginary, divisor.real)?,
            });
        }
        let one = Component::new(1.0);
        let real = pairs.iter().flat_map(|&[a, b]| {
            [
                [a.real, b.real, divisor.real],
                [a.imaginary.negated(), b.imaginary, divisor.real],
                [a.real, b.imaginary, divisor.imaginary],
                [a.imaginary, b.real, divisor.imaginary],
            ]
        });
        let imaginary = pairs.iter().flat_map(|&[a, b]| {
            [
                [a.real, b.imaginary, divisor.real],
                [a.imaginary, b.real, divisor.real],
                [a.real.negated(), b.real, divisor.imaginary],
                [a.imaginary, b.imaginary, divisor.imaginary],
            ]
        });
        let denominator = [
            [divisor.real, divisor.real, one],
            [divisor.imaginary, divisor.imaginary, one],
        ];
        Ok(Self {
            real: Component::sum_triple_products_ratio(real, denominator.into_iter())?,
            imaginary: Component::sum_triple_products_ratio(imaginary, denominator.into_iter())?,
        })
    }

    #[inline]
    pub(crate) fn new(real: f64, imaginary: f64) -> Self {
        Self {
            real: Component::new(real),
            imaginary: Component::new(imaginary),
        }
    }

    #[inline]
    pub(crate) fn from_complex(value: Complex64) -> Self {
        Self::new(value.re, value.im)
    }

    #[inline]
    pub(crate) fn binary64(self) -> Complex64 {
        Complex64::new(self.real.binary64(), self.imaginary.binary64())
    }

    #[inline]
    pub(crate) fn is_zero(self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }

    pub(crate) fn is_real(self) -> bool {
        self.imaginary.is_zero()
    }

    #[inline]
    pub(crate) fn is_finite(self) -> bool {
        self.real.is_finite() && self.imaginary.is_finite()
    }

    #[inline]
    pub(crate) fn has_regular_components(self) -> bool {
        self.real.is_regular() && self.imaginary.is_regular()
    }

    #[inline]
    pub(crate) fn regular_result(value: Complex64, a: Complex64, b: Complex64) -> bool {
        let real_zero = (a.re == 0.0 || b.re == 0.0) && (a.im == 0.0 || b.im == 0.0);
        let imaginary_zero = (a.re == 0.0 || b.im == 0.0) && (a.im == 0.0 || b.re == 0.0);
        (value.re.is_normal() || (value.re == 0.0 && real_zero))
            && (value.im.is_normal() || (value.im == 0.0 && imaginary_zero))
    }

    #[inline]
    pub(crate) fn multiply(self, other: Self) -> Self {
        if self.has_regular_components() && other.has_regular_components() {
            let a = Complex64::new(self.real.binary64(), self.imaginary.binary64());
            let b = Complex64::new(other.real.binary64(), other.imaginary.binary64());
            let value = multiply_complex(a, b);
            if Self::regular_result(value, a, b) {
                return Self::from_complex(value);
            }
        }
        self.multiply_wide(other)
    }

    #[cold]
    #[inline(never)]
    fn multiply_wide(self, other: Self) -> Self {
        Self {
            real: Component::product_sum(
                self.real,
                other.real,
                self.imaginary.negated(),
                other.imaginary,
            ),
            imaginary: Component::product_sum(
                self.real,
                other.imaginary,
                self.imaginary,
                other.real,
            ),
        }
    }

    #[inline]
    pub(crate) fn divide(self, other: Self) -> Self {
        if self.has_regular_components() && other.has_regular_components() {
            let a = Complex64::new(self.real.binary64(), self.imaginary.binary64());
            let b = Complex64::new(other.real.binary64(), other.imaginary.binary64());
            let value = divide_complex(a, b);
            if Self::regular_result(value, a, b) {
                return Self::from_complex(value);
            }
        }
        self.divide_wide(other)
    }

    #[cold]
    #[inline(never)]
    fn divide_wide(self, other: Self) -> Self {
        if !self.is_finite()
            || !other.is_finite()
            || (other.real.is_zero() && other.imaginary.is_zero())
        {
            return Self::from_complex(divide_complex(self.binary64(), other.binary64()));
        }
        if other.imaginary.is_zero() {
            return Self {
                real: self.real.divide(other.real),
                imaginary: self.imaginary.divide(other.real),
            };
        }
        if other.real.is_zero() {
            return Self {
                real: self.imaginary.divide(other.imaginary),
                imaginary: self.real.negated().divide(other.imaginary),
            };
        }
        let denominator =
            Component::product_sum(other.real, other.real, other.imaginary, other.imaginary);
        Self {
            real: Component::product_sum(self.real, other.real, self.imaginary, other.imaginary)
                .divide(denominator),
            imaginary: Component::product_sum(
                self.imaginary,
                other.real,
                self.real.negated(),
                other.imaginary,
            )
            .divide(denominator),
        }
    }
}

impl std::ops::Neg for FrequencyValue {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            real: self.real.negated(),
            imaginary: self.imaginary.negated(),
        }
    }
}

impl std::ops::Add for FrequencyValue {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real.plus(other.real),
            imaginary: self.imaginary.plus(other.imaginary),
        }
    }
}

impl std::ops::Sub for FrequencyValue {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl std::ops::Mul<f64> for FrequencyValue {
    type Output = Self;
    #[inline]
    fn mul(self, other: f64) -> Self {
        if other == 1.0 {
            return self;
        }
        Self {
            real: self.real.multiply(Component::new(other)),
            imaginary: self.imaginary.multiply(Component::new(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_intermediates_preserve_independent_component_ranges() {
        for gain in [1e-200, 1.0, 1e200] {
            for omega in [1e-200, 1.0, 1e200] {
                let action =
                    FrequencyValue::new(1.0, omega).multiply(FrequencyValue::new(gain, 0.0));
                let result = action.divide(FrequencyValue::new(gain, 0.0)).binary64();
                assert!((result.re - 1.0).abs() <= 4.0 * f64::EPSILON);
                assert!((result.im / omega - 1.0).abs() <= 4.0 * f64::EPSILON);
            }
        }
        let huge = FrequencyValue::new(1e300, 1e-300).multiply(FrequencyValue::new(1e300, 0.0));
        let result = huge.divide(FrequencyValue::new(1e300, 0.0)).binary64();
        assert!((result.re / 1e300 - 1.0).abs() <= 4.0 * f64::EPSILON);
        assert!((result.im / 1e-300 - 1.0).abs() <= 4.0 * f64::EPSILON);
        assert_eq!((huge - huge).binary64(), Complex64::new(0.0, 0.0));
    }

    #[test]
    fn frequency_conversion_preserves_subnormal_rounding_and_signed_zero() {
        let tiny = f64::from_bits(1);
        for sign in [1.0, -1.0] {
            let half = FrequencyValue::new(sign * tiny, 0.0) * 0.5;
            assert_eq!(
                half.binary64().re.to_bits(),
                (0.0_f64.copysign(sign)).to_bits()
            );
            assert_eq!(
                (half * 2.0).binary64().re.to_bits(),
                (sign * tiny).to_bits()
            );
            let above = FrequencyValue::new(sign * tiny, 0.0) * (0.5 + f64::EPSILON);
            assert_eq!(above.binary64().re.to_bits(), (sign * tiny).to_bits());
        }
        let value = FrequencyValue::new(tiny, tiny)
            .multiply(FrequencyValue::new(0.5, 0.5))
            .binary64();
        assert_eq!(value, Complex64::new(0.0, tiny));
        for value in [0.0, -0.0, tiny, f64::MIN_POSITIVE, f64::MAX] {
            assert_eq!(
                FrequencyValue::new(value, 0.0).binary64().re.to_bits(),
                value.to_bits()
            );
        }
    }

    #[test]
    fn complex_products_preserve_range_and_cancellation() {
        let product = multiply_complex(Complex64::new(1.6e308, 8e307), Complex64::new(1.2, 0.3));
        assert!((product.re / 1.68e308 - 1.0).abs() <= f64::EPSILON);
        assert!((product.im / 1.44e308 - 1.0).abs() <= f64::EPSILON);

        let tiny = f64::from_bits(1);
        let product = multiply_complex(Complex64::new(tiny, tiny), Complex64::new(0.5, 0.5));
        assert_eq!(product.re, 0.0);
        assert_eq!(product.im.to_bits(), tiny.to_bits());

        let epsilon = f64::EPSILON;
        let product = multiply_complex(
            Complex64::new(1.0 + epsilon, 1.0),
            Complex64::new(1.0 - epsilon, 1.0),
        );
        assert_eq!(product.re, -epsilon * epsilon);
        assert_eq!(product.im, 2.0);
    }

    #[test]
    fn complex_division_retains_small_cancellation_residuals() {
        let epsilon = f64::EPSILON;
        let result = divide_complex(
            Complex64::new(1.0 + epsilon, -1.0),
            Complex64::new(1.0 - epsilon, 1.0),
        );
        let expected = -epsilon * epsilon / ((1.0 - epsilon) * (1.0 - epsilon) + 1.0);
        assert!((result.re / expected - 1.0).abs() <= f64::EPSILON);

        // Independently rounded exact rational quotient of these binary64 inputs.
        let result = divide_complex(
            Complex64::new(1.0628830584883267e-16, 1.1719599832785925e-181),
            Complex64::new(1.699918502130826e54, 2.2148244420509687e-111),
        );
        assert_eq!(result.re.to_bits(), 0x315b9e4672575cc3);
        assert_eq!(result.im.to_bits(), 0x8ef462b9a56c2c46);
    }
}
