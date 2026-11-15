//! The root a step controller takes of its error ratio, bit-identical on every
//! platform.
//!
//! A controller that resizes a step from its truncation error scales the step
//! by `ratio^(1/(p+1))`. The step it proposes lands in the accepted grid, so
//! every bit of that root reaches the grid's bits, and the next truncation
//! estimate divides by the step and amplifies whatever the root got wrong.
//! `f64::powf` is the platform's libm `pow`, which is not correctly rounded
//! everywhere: macOS's misrounds about one cube root in seven hundred. One
//! such root, a restart scale of 1.2450738825145895 where the correctly
//! rounded value (and Linux's) is 1.2450738825145893, moved a mixed deck's
//! grid by 1.7e-21 s two steps later and parted every later point from the
//! grid Linux accepts.
//!
//! [`root_power`] returns the correctly rounded `x.powf(1.0 / n as f64)` from
//! IEEE basic operations, which round the same way on every target, and the
//! pure-Rust `libm` crate, which is the same code on every target. Where the
//! platform's `pow` is correctly rounded, which is nearly everywhere on
//! glibc, the two agree bit for bit.

use crate::Value;

/// The inputs the double-double steps keep exact: neither Veltkamp's split
/// can overflow nor a product's rounding error fall below the normal range.
/// Every caller clamps the root it takes to a scale between 0.25 and 2, which
/// only ratios between 2^-128 and 2^64 reach.
const EXACT_RANGE: (Value, Value) = (1.0e-180, 1.0e180);

/// Veltkamp's splitting constant for binary64, `2^27 + 1`.
const SPLITTER: Value = 134_217_729.0;

/// `x.powf(1.0 / root as f64)`, correctly rounded and the same on every
/// platform.
///
/// The `n`th root is refined from a seed within an ulp by two Newton steps on
/// `z^n = x` carried in double-double (about 2^-100 relative), then corrected
/// for `1.0 / n as f64` not being `1/n` exactly: `x^e = x^(1/n) * x^d` with
/// `d = (n * e - 1) / n`, whose numerator is exact. `|d * ln x|` is below
/// 1e-14 over the exact range, so `exp` is its two-term series and `ln x`
/// needs only double precision. The sum of the double-double root and its
/// correction is rounded once, which gives the correctly rounded power unless
/// it lies within about 2^-100 of a rounding midpoint.
///
/// Outside the exact range, for a non-finite or non-positive `x`, and for a
/// root below two or above 64, the result is `libm::pow`: portable, within an
/// ulp, and never reached by a step controller, whose scales are clamped.
pub(super) fn root_power(x: Value, root: u32) -> Value {
    let exponent = 1.0 / Value::from(root);
    if !(EXACT_RANGE.0..=EXACT_RANGE.1).contains(&x) || !(2..=64).contains(&root) {
        return libm::pow(x, exponent);
    }
    if root == 2 {
        // `0.5` is exact, and the square root is correctly rounded.
        return x.sqrt();
    }
    let n = Value::from(root);
    let mut hi = libm::pow(x, exponent);
    let mut lo = 0.0;
    for _ in 0..2 {
        let (mut power_hi, mut power_lo) = (hi, lo);
        for _ in 1..root {
            (power_hi, power_lo) = dd_mul((power_hi, power_lo), (hi, lo));
        }
        let (residual_hi, residual_lo) = two_sum(power_hi, -x);
        let residual = residual_hi + (residual_lo + power_lo);
        let newton = residual / (n * (power_hi / hi));
        let (sum, error) = two_sum(hi, -newton);
        (hi, lo) = fast_two_sum(sum, error + lo);
    }
    let (product, product_error) = two_prod(n, exponent);
    let offset = ((product - 1.0) + product_error) / n;
    let log_scale = offset * libm::log(x);
    let correction = log_scale + 0.5 * log_scale * log_scale;
    hi + (lo + hi * correction)
}

/// `a + b` as a rounded sum and its exact error (Knuth).
fn two_sum(a: Value, b: Value) -> (Value, Value) {
    let sum = a + b;
    let b_virtual = sum - a;
    (sum, (a - (sum - b_virtual)) + (b - b_virtual))
}

/// [`two_sum`] for `|a| >= |b|` (Dekker).
fn fast_two_sum(a: Value, b: Value) -> (Value, Value) {
    let sum = a + b;
    (sum, b - (sum - a))
}

/// `a` as two halves whose products are exact (Veltkamp).
fn split(a: Value) -> (Value, Value) {
    let scaled = SPLITTER * a;
    let hi = scaled - (scaled - a);
    (hi, a - hi)
}

/// `a * b` as a rounded product and its exact error (Dekker), without a fused
/// multiply-add: `f64::mul_add` is the platform's `fma` where the target has
/// no instruction for it.
fn two_prod(a: Value, b: Value) -> (Value, Value) {
    let product = a * b;
    let (a_hi, a_lo) = split(a);
    let (b_hi, b_lo) = split(b);
    let error = ((a_hi * b_hi - product) + a_hi * b_lo + a_lo * b_hi) + a_lo * b_lo;
    (product, error)
}

/// The product of two double-double numbers, to about 2^-104 relative.
fn dd_mul(a: (Value, Value), b: (Value, Value)) -> (Value, Value) {
    let (product, error) = two_prod(a.0, b.0);
    fast_two_sum(product, error + (a.0 * b.1 + a.1 * b.0))
}

#[cfg(test)]
mod tests {
    use super::root_power;

    /// Each `(x, root, correctly rounded x^(1/root as f64))`, computed to 80
    /// significant digits. The first five are the ones macOS's `pow` rounds
    /// the other way, met as error ratios on one mixed deck's restarts.
    const CORRECTLY_ROUNDED: &[(f64, u32, f64)] = &[
        (2.514_809_655_104_425_4, 3, 1.359_883_506_296_354_6),
        (1.930_124_704_622_400_6, 3, 1.245_073_882_514_589_3),
        (2.515_378_040_805_307_7, 3, 1.359_985_950_115_326),
        (4.603_734_445_127_003, 3, 1.663_553_432_812_231_7),
        (2.433_399_385_884_251_3, 3, 1.345_048_046_428_234),
        // `8^(1/3 as f64)` is `2 * 8^(-2^-54 / 3)`, 7.7e-17 below two and
        // under half the gap to the double below it, so it rounds to two.
        (8.0, 3, 2.0),
        (1.0, 3, 1.0),
        (1.0, 7, 1.0),
        // The decades and roots a controller's ratio spans. `1e-12` shows the
        // exponent's offset: its cube root rounds to `1e-4`, its power to three
        // ulps above.
        (1.0e-12, 3, 0.000_100_000_000_000_000_05),
        (1.0e12, 3, 9_999.999_999_999_995),
        (0.015_625, 3, 0.25),
        (0.3, 4, 0.740_082_804_492_285_3),
        (17.0, 4, 2.030_543_184_868_930_6),
        (1.0e-6, 5, 0.063_095_734_448_019_32),
        (3.7e5, 5, 12.990_933_056_386_604),
        (0.02, 6, 0.521_000_730_958_691_4),
        (123.456, 6, 2.231_440_786_728_173),
        (1.0e-9, 7, 0.051_794_746_792_312_12),
        (9.9e8, 7, 19.279_276_949_703_423),
        (2.0, 7, 1.104_089_513_673_812_3),
    ];

    #[test]
    fn the_root_is_the_correctly_rounded_power() {
        for &(x, root, expected) in CORRECTLY_ROUNDED {
            let got = root_power(x, root);
            assert_eq!(
                got.to_bits(),
                expected.to_bits(),
                "{x:e}^(1/{root}) gave {got:e}, correctly rounded {expected:e}"
            );
        }
    }

    #[test]
    fn a_square_root_is_the_square_root() {
        for x in [1.0e-9, 0.37, 2.0, 3.0, 1.0e9] {
            assert_eq!(root_power(x, 2).to_bits(), x.sqrt().to_bits());
        }
    }

    #[test]
    fn outside_the_exact_range_the_portable_pow_answers() {
        for (x, root) in [
            (0.0, 3),
            (f64::INFINITY, 3),
            (1.0e-300, 3),
            (5.0, 1),
            (5.0, 65),
        ] {
            let exponent = 1.0 / f64::from(root);
            assert_eq!(
                root_power(x, root).to_bits(),
                libm::pow(x, exponent).to_bits()
            );
        }
        assert!(root_power(f64::NAN, 3).is_nan());
        assert!(root_power(-8.0, 3).is_nan());
    }
}
