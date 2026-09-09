//! Range-protected complex arithmetic shared by filters and AC evaluation.

use num_complex::Complex64;
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

#[cfg(test)]
mod tests {
    use super::*;

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
