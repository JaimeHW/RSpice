//! A complex-step scalar, for checking derivatives without any derivative rule.
//!
//! Evaluate a real function at `x + ih` in complex arithmetic and its imaginary
//! part divided by `h` is the derivative, accurate to `O(h²)`. Because nothing
//! is subtracted, `h` can be made small enough — 1e-200 — that the truncation
//! error vanishes below rounding, so the answer is exact to machine precision.
//! A finite difference cannot do this: shrinking its step trades truncation
//! error for cancellation.
//!
//! What makes it a *good* oracle here is what it does not contain. There is no
//! chain rule anywhere in this file. Every derivative comes out of the complex
//! arithmetic of the primal function, so it cannot repeat a mistake the
//! derivative pass made — which is the failure mode a second forward-mode
//! implementation would share.
//!
//! ## Where it does not apply
//!
//! Complex step needs the function to be analytic along the path. `abs`, `min`,
//! `max`, `floor`, and comparisons are not, and are defined here by their real
//! part's branch — the same branch the primal took. That is correct away from
//! the kink and undefined at it, which is exactly the status of the derivative
//! itself there.

use super::cfg_eval::CfgScalar;

/// The perturbation. Small enough that `h²` underflows, so the truncation term
/// is not merely small but absent.
pub const COMPLEX_STEP: f64 = 1.0e-200;

/// A complex number carrying a real value and one derivative direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexStep {
    pub re: f64,
    pub im: f64,
}

impl ComplexStep {
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// A value being differentiated with respect to.
    pub const fn seed(value: f64) -> Self {
        Self::new(value, COMPLEX_STEP)
    }

    /// The derivative this value accumulated.
    pub fn derivative(self) -> f64 {
        self.im / COMPLEX_STEP
    }

    fn modulus(self) -> f64 {
        self.re.hypot(self.im)
    }

    fn argument(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// `e^z` in polar form.
    fn exponential(self) -> Self {
        let magnitude = self.re.exp();
        Self::new(magnitude * self.im.cos(), magnitude * self.im.sin())
    }

    fn logarithm(self) -> Self {
        Self::new(self.modulus().ln(), self.argument())
    }

    fn reciprocal(self) -> Self {
        let denominator = self.re * self.re + self.im * self.im;
        Self::new(self.re / denominator, -self.im / denominator)
    }
}

impl CfgScalar for ComplexStep {
    fn from_f64(value: f64) -> Self {
        Self::new(value, 0.0)
    }

    fn real(self) -> f64 {
        self.re
    }

    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }

    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }

    fn div(self, rhs: Self) -> Self {
        self.mul(rhs.reciprocal())
    }

    /// Piecewise constant in the quotient, so the derivative is the dividend's.
    fn rem(self, rhs: Self) -> Self {
        let quotient = (self.re / rhs.re).trunc();
        Self::new(self.re % rhs.re, self.im - quotient * rhs.im)
    }

    fn powf(self, rhs: Self) -> Self {
        if self.re == 0.0 && self.im == 0.0 {
            return Self::from_f64(0.0);
        }
        self.logarithm().mul(rhs).exponential()
    }

    /// `sqrt(x² + y²)` in complex arithmetic rather than the library's
    /// `f64::hypot`. The library form exists to avoid squaring, and squaring is
    /// exactly what carries the perturbation here; the oracle runs at drawn
    /// bias points, not at the ends of the exponent range.
    fn hypot(self, rhs: Self) -> Self {
        self.mul(self).add(rhs.mul(rhs)).sqrt()
    }

    /// `2 atan(y / (hypot(y, x) + x))`, the half-angle form.
    ///
    /// Not `atan(y/x)` plus a quadrant offset: that is singular on the whole
    /// `x = 0` line, where `atan2` itself is perfectly smooth and its `x`
    /// derivative is `-1/y`. This form is singular only on the negative real
    /// axis, which is the branch cut the function really has.
    fn atan2(self, rhs: Self) -> Self {
        let denominator = self.hypot(rhs).add(rhs);
        if denominator.re == 0.0 {
            // The cut itself: `atan2(0, x<0)` is π from one side and −π from
            // the other, so there is no derivative to report, only the value.
            return Self::from_f64(self.re.atan2(rhs.re));
        }
        let half = self.div(denominator).atan();
        half.add(half)
    }

    fn exp(self) -> Self {
        self.exponential()
    }

    fn ln(self) -> Self {
        self.logarithm()
    }

    fn log10(self) -> Self {
        self.logarithm()
            .div(Self::from_f64(std::f64::consts::LN_10))
    }

    fn sqrt(self) -> Self {
        if self.re == 0.0 && self.im == 0.0 {
            return Self::from_f64(0.0);
        }
        let magnitude = self.modulus().sqrt();
        let half_argument = self.argument() / 2.0;
        Self::new(
            magnitude * half_argument.cos(),
            magnitude * half_argument.sin(),
        )
    }

    /// Not the complex modulus: `|z|` would discard the perturbation. The real
    /// function is `sign(x) * x`, and this follows the branch the primal took.
    fn abs(self) -> Self {
        if self.re < 0.0 { self.neg() } else { self }
    }

    fn sin(self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    fn cos(self) -> Self {
        Self::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }

    fn tan(self) -> Self {
        self.sin().div(self.cos())
    }

    fn sinh(self) -> Self {
        Self::new(
            self.re.sinh() * self.im.cos(),
            self.re.cosh() * self.im.sin(),
        )
    }

    fn cosh(self) -> Self {
        Self::new(
            self.re.cosh() * self.im.cos(),
            self.re.sinh() * self.im.sin(),
        )
    }

    fn tanh(self) -> Self {
        self.sinh().div(self.cosh())
    }

    /// `atan`, written so the perturbation never lands beside a one.
    ///
    /// The textbook identity `(i/2) ln((i + z)/(i − z))` is correct and useless
    /// here: it forms `1 ± ih`, and `1 + 1e-200` is `1`. The perturbation is
    /// gone before the logarithm sees it, and the oracle silently reports a
    /// derivative of zero for this term.
    ///
    /// This form keeps it. The imaginary part is
    /// `¼ ln(N/D)` with `N − D = 4b` exactly, so writing it as `¼ ln1p(4b/D)`
    /// puts the perturbation in the argument of a function built to resolve
    /// small ones. `D` may lose its own `−2b` without harm: that only moves the
    /// answer at second order.
    fn atan(self) -> Self {
        let Self { re: a, im: b } = self;
        let real = 0.5 * (2.0 * a).atan2(1.0 - a * a - b * b);
        let denominator = a * a + (b - 1.0) * (b - 1.0);
        let imaginary = 0.25 * (4.0 * b / denominator).ln_1p();
        Self::new(real, imaginary)
    }

    /// `asin z = atan(z / sqrt(1 − z²))`, routed through [`Self::atan`] for the
    /// reason that function is written the way it is.
    ///
    /// The textbook `−i ln(iz + sqrt(1 − z²))` adds the perturbation to a real
    /// part of order one and loses it there; here every perturbation stays in
    /// an imaginary component of its own until the arctangent resolves it.
    fn asin(self) -> Self {
        let one = Self::from_f64(1.0);
        let root = one.sub(self.mul(self)).sqrt();
        self.div(root).atan()
    }

    /// `acos z = π/2 − asin z`. Subtracting a real constant leaves the
    /// perturbation exactly where `asin` put it.
    fn acos(self) -> Self {
        Self::from_f64(std::f64::consts::FRAC_PI_2).sub(self.asin())
    }

    /// `asinh z = ln(z + sqrt(z² + 1))`.
    fn asinh(self) -> Self {
        let inner = self.mul(self).add(Self::from_f64(1.0)).sqrt();
        self.add(inner).logarithm()
    }

    /// `acosh z = ln(z + sqrt(z² − 1))`.
    ///
    /// Safe as written, unlike `asin`: the answer is the logarithm's argument
    /// angle, and the perturbation reaches it as the imaginary part of a sum
    /// whose real part is order one — which is what `argument` divides by.
    fn acosh(self) -> Self {
        let inner = self.mul(self).sub(Self::from_f64(1.0)).sqrt();
        self.add(inner).logarithm()
    }

    /// `atanh z = ½ ln((1 + z)/(1 − z))`.
    fn atanh(self) -> Self {
        let one = Self::from_f64(1.0);
        let quotient = one.add(self).div(one.sub(self));
        quotient.logarithm().mul(Self::from_f64(0.5))
    }

    fn floor(self) -> Self {
        Self::from_f64(self.re.floor())
    }

    fn ceil(self) -> Self {
        Self::from_f64(self.re.ceil())
    }

    /// The real `limexp` is affine beyond the clamp, and this reproduces both
    /// pieces analytically rather than deferring to `exp`.
    fn limexp(self) -> Self {
        if self.re < LIMEXP_THRESHOLD {
            self.exponential()
        } else {
            let scale = Self::from_f64(LIMEXP_THRESHOLD.exp());
            let excess = self.sub(Self::from_f64(LIMEXP_THRESHOLD));
            scale.mul(excess.add(Self::from_f64(1.0)))
        }
    }

    fn limited_exp(self) -> Self {
        if self.re > LIMEXP_THRESHOLD {
            let scale = Self::from_f64(LIMEXP_MAX);
            let excess = self.sub(Self::from_f64(LIMEXP_THRESHOLD));
            scale.mul(excess.add(Self::from_f64(1.0)))
        } else if self.re < -LIMEXP_THRESHOLD {
            Self::from_f64(LIMITED_EXP_FLOOR)
        } else {
            self.exponential()
        }
    }

    fn limited_exp_derivative(self) -> Self {
        if self.re > LIMEXP_THRESHOLD {
            Self::from_f64(LIMEXP_MAX)
        } else if self.re < -LIMEXP_THRESHOLD {
            Self::from_f64(0.0)
        } else {
            self.exponential()
        }
    }
}

const LIMEXP_THRESHOLD: f64 = 80.0;
const LIMEXP_MAX: f64 = 5.540_622_384_393_51e34;
const LIMITED_EXP_FLOOR: f64 = 1.804_851_387e-35;

#[cfg(test)]
mod tests {
    use super::*;

    /// Each function checked against its derivative written out by hand, so a
    /// mistake in the complex identities cannot travel into the oracle.
    #[test]
    fn the_elementary_functions_reproduce_their_derivatives() {
        let x = 0.37;
        let cases: Vec<(&str, fn(ComplexStep) -> ComplexStep, f64)> = vec![
            ("exp", CfgScalar::exp, x.exp()),
            ("ln", CfgScalar::ln, 1.0 / x),
            ("sqrt", CfgScalar::sqrt, 0.5 / x.sqrt()),
            ("sin", CfgScalar::sin, x.cos()),
            ("cos", CfgScalar::cos, -x.sin()),
            ("tan", CfgScalar::tan, 1.0 / (x.cos() * x.cos())),
            ("sinh", CfgScalar::sinh, x.cosh()),
            ("cosh", CfgScalar::cosh, x.sinh()),
            ("tanh", CfgScalar::tanh, 1.0 - x.tanh() * x.tanh()),
            ("atan", CfgScalar::atan, 1.0 / (1.0 + x * x)),
            ("asinh", CfgScalar::asinh, 1.0 / (1.0 + x * x).sqrt()),
            ("abs", CfgScalar::abs, 1.0),
            (
                "log10",
                CfgScalar::log10,
                1.0 / (x * std::f64::consts::LN_10),
            ),
            ("asin", CfgScalar::asin, 1.0 / (1.0 - x * x).sqrt()),
            ("acos", CfgScalar::acos, -1.0 / (1.0 - x * x).sqrt()),
            ("atanh", CfgScalar::atanh, 1.0 / (1.0 - x * x)),
        ];
        for (name, function, expected) in cases {
            let actual = function(ComplexStep::seed(x)).derivative();
            assert!(
                (actual - expected).abs() <= 1.0e-12 * expected.abs().max(1.0),
                "{name}' is {actual}, expected {expected}"
            );
        }
    }

    /// `acosh` needs an argument above one, so it is checked on its own domain
    /// rather than folded into the table above.
    #[test]
    fn the_inverse_hyperbolic_cosine_reproduces_its_derivative() {
        let x = 1.83;
        let actual = CfgScalar::acosh(ComplexStep::seed(x)).derivative();
        let expected = 1.0 / (x * x - 1.0).sqrt();
        assert!(
            (actual - expected).abs() <= 1.0e-12 * expected.abs(),
            "acosh' is {actual}, expected {expected}"
        );
    }

    /// Both operands, because the two-argument functions have a gradient rather
    /// than a derivative and a form that is right in one argument can still be
    /// wrong in the other.
    #[test]
    fn the_two_argument_functions_reproduce_both_partials() {
        let (a, b) = (1.31, -0.77);
        let magnitude = a.hypot(b);

        let by_first =
            CfgScalar::hypot(ComplexStep::seed(a), ComplexStep::from_f64(b)).derivative();
        let by_second =
            CfgScalar::hypot(ComplexStep::from_f64(a), ComplexStep::seed(b)).derivative();
        assert!((by_first - a / magnitude).abs() <= 1.0e-12);
        assert!((by_second - b / magnitude).abs() <= 1.0e-12);

        let squared = a * a + b * b;
        let by_ordinate =
            CfgScalar::atan2(ComplexStep::seed(a), ComplexStep::from_f64(b)).derivative();
        let by_abscissa =
            CfgScalar::atan2(ComplexStep::from_f64(a), ComplexStep::seed(b)).derivative();
        assert!((by_ordinate - b / squared).abs() <= 1.0e-12);
        assert!((by_abscissa + a / squared).abs() <= 1.0e-12);
    }

    /// The half-angle form exists for this: `atan(y/x)` plus an offset is
    /// singular along the whole `x = 0` line, where `atan2` is smooth.
    #[test]
    fn the_arctangent_of_two_arguments_differentiates_across_the_ordinate_axis() {
        let y = 0.94;
        let derivative = CfgScalar::atan2(ComplexStep::from_f64(y), ComplexStep::seed(0.0));
        assert!((derivative.derivative() + 1.0 / y).abs() <= 1.0e-12);
        assert!((derivative.re - std::f64::consts::FRAC_PI_2).abs() <= 1.0e-12);
    }

    #[test]
    fn a_negative_argument_takes_the_other_branch_of_abs() {
        let derivative = CfgScalar::abs(ComplexStep::seed(-0.37)).derivative();
        assert!((derivative + 1.0).abs() <= 1.0e-12);
    }

    #[test]
    fn a_power_differentiates_through_both_operands() {
        let base = 1.7;
        let exponent = 2.5;
        let by_base =
            CfgScalar::powf(ComplexStep::seed(base), ComplexStep::from_f64(exponent)).derivative();
        let expected = exponent * base.powf(exponent - 1.0);
        assert!((by_base - expected).abs() <= 1.0e-12 * expected.abs());

        let by_exponent =
            CfgScalar::powf(ComplexStep::from_f64(base), ComplexStep::seed(exponent)).derivative();
        let expected = base.powf(exponent) * base.ln();
        assert!((by_exponent - expected).abs() <= 1.0e-12 * expected.abs());
    }

    /// The step must not perturb the primal at all: a path that branched
    /// differently would be answering about a different function.
    #[test]
    fn the_perturbation_does_not_disturb_the_value() {
        let x = 0.37;
        let seeded = CfgScalar::exp(ComplexStep::seed(x));
        assert_eq!(seeded.re, x.exp());
    }
}
