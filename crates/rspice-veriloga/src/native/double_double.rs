//! Double-double arithmetic: a reference the two routes' `f64` rounding is
//! *measured* against rather than bounded.
//!
//! # Why a second precision and not a second bound
//!
//! [`super::cfg_mir_census`] asks whether two implementations of the chain
//! rule compute the same derivative. In `f64` that question has no clean
//! answer on an ill-conditioned cone: `l_utsoi_102`'s `jacobians[36][6]` forms
//! a factor at `3e7` and lands it at `4e-2`, so one unit round-off amplified by
//! the cone's condition number is already `5e-9` and two correct evaluations
//! differ in the eighth figure.
//!
//! W-F10c-2 answered that with a first-order forward *bound* — the error a
//! walk could have committed. It was tight on well-conditioned cones and
//! useless on that one, because a scalar bound adds the magnitudes of two
//! operands' errors while the subtraction that follows cancels them; the bound
//! came out at four times the entry itself.
//!
//! A measurement has no such failure mode. Walk the same cone twice — once in
//! `f64`, once in an arithmetic with more than twice the precision — and the
//! difference *is* the `f64` walk's rounding error, correlations and all. It
//! costs a constant factor rather than the quadratic an affine bound would,
//! and it needs no tolerance: at `2^-104` relative, a condition number of `5e7`
//! still leaves thirty digits, so the reference is exact for every purpose this
//! census has.
//!
//! # The representation
//!
//! An unevaluated sum of two doubles, `hi + lo`, normalized so that
//! `hi == fl(hi + lo)` and therefore `|lo| <= ulp(hi)/2`. That carries about
//! 106 significant bits — `u_dd ≈ 6e-33` — and it is *not* a wider exponent
//! range: overflow and underflow happen where `f64` has them, which is right,
//! because a cone that overflows in `f64` is not one a reference should rescue.
//!
//! The algorithms are the standard error-free transformations (Dekker 1971,
//! Knuth's 2Sum, and the Bailey/Hida/Li `qd` formulations). Two choices are
//! worth naming:
//!
//! * **`two_prod` splits rather than calling `mul_add`.** Rust's `f64::mul_add`
//!   is a *fused* multiply-add by contract, and the baseline `x86-64` target
//!   this crate builds for has no FMA instruction, so it lowers to a libm call.
//!   A census cone is twenty-six thousand operations deep and every one of them
//!   would take that call. Dekker's splitting is seventeen native flops and
//!   exact for every operand under `2^996`; past that the code falls back to
//!   `mul_add`, which is exact there too.
//! * **Non-finite results collapse to a single double.** `2Sum` of an infinity
//!   produces a NaN in the residual, which would poison an otherwise honest
//!   `±inf`. Where `hi` leaves the reals the low word is zero and the value is
//!   whatever `f64` would have said.
//!
//! # The transcendentals
//!
//! `exp` reduces to `|r| ≤ ln2/1024`, sums the Taylor series to the working
//! precision, and squares nine times; `ln` takes one Newton step on `exp` from
//! the `f64` logarithm, which doubles sixteen correct digits into thirty-two.
//! `sin`/`cos` reduce modulo `π/2` against a double-double `π/2` and sum the
//! series on `|r| ≤ π/4`; `atan2` takes two Newton steps against them, and
//! `asin`, `acos` and `atan` are written through it. The hyperbolics compose
//! from `exp` and `ln` with a series near zero where the closed form cancels.
//!
//! Each is accurate to within a few units in the last place of the
//! double-double format — the unit tests pin that against `f64` and against
//! identities the format cannot satisfy by accident. None of it needs to be
//! better: what the census needs is a reference far enough below `1e-9/κ ≈
//! 2e-17` that the `f64` error it measures is the whole of the difference, and
//! `1e-30` is twelve orders of margin.
//!
//! # Where the argument reduction stops
//!
//! `sin`, `cos` and `tan` past `1e15` return the `f64` library's answer lifted,
//! because a double-double `π/2` cannot reduce an argument that large and a
//! reference that silently lost every digit would be worse than none. The
//! corpus reaches no such argument; [`DoubleDouble::reduction_exhausted`] says
//! so rather than leaving it to be assumed.

use crate::canonical_ir::{CfgEvalInputs, CfgScalar};

/// Dekker's splitting constant, `2^27 + 1`.
const SPLIT: f64 = 134_217_729.0;

/// Above this magnitude Dekker's splitting overflows, so `two_prod` uses the
/// fused multiply-add instead. `2^996`.
const SPLIT_CEILING: f64 = 6.696_928_794_914_171e299;

/// `ln 2`, to double-double precision.
const LN2: DoubleDouble = DoubleDouble {
    hi: 6.931_471_805_599_452_9e-1,
    lo: 2.319_046_813_846_299_6e-17,
    real: 6.931_471_805_599_452_9e-1,
};

/// `ln 10`, to double-double precision.
const LN10: DoubleDouble = DoubleDouble {
    hi: 2.302_585_092_994_046e0,
    lo: -2.170_756_223_382_249_4e-16,
    real: 2.302_585_092_994_046e0,
};

/// `π/2`, to double-double precision.
const PI_OVER_2: DoubleDouble = DoubleDouble {
    hi: 1.570_796_326_794_896_6e0,
    lo: 6.123_233_995_736_766e-17,
    real: 1.570_796_326_794_896_6e0,
};

/// `π`, to double-double precision.
const PI: DoubleDouble = DoubleDouble {
    hi: 3.141_592_653_589_793e0,
    lo: 1.224_646_799_147_353_2e-16,
    real: 3.141_592_653_589_793e0,
};

/// Past this argument magnitude the double-double `π/2` has no digits left to
/// reduce against. See the module documentation.
const TRIG_REDUCTION_CEILING: f64 = 1.0e15;

/// Relative size at which a series term stops moving the sum.
const SERIES_EPSILON: f64 = 1.0e-34;

/// The largest number of terms any series here is allowed to take. Reached
/// only if a convergence assumption is wrong, and then it truncates rather
/// than spinning.
const SERIES_TERM_LIMIT: usize = 64;

/// A value carried as an unevaluated sum of two doubles, alongside what the
/// same sequence of operations would have produced in `f64`.
///
/// # Why the `f64` value travels with it
///
/// [`CfgScalar`]'s own contract says why: "predicates go through `Self::real`
/// rather than being part of the trait, so a derivative-carrying scalar
/// branches on its value and not on its infinitesimal part — which is what
/// makes a dual number reproduce the primal walk exactly." A wider precision is
/// in the same position. A cone that computes `a − b` where the two agree to
/// the last bit of `f64` gets `+0` there and `−1e-40` here, and a `>= 0`
/// downstream then sends the two walks into different arms of a piecewise
/// function. `vbic13_4t` does exactly that: its `jacobians[2][0]` is `1.0` on
/// both compiled routes and the unguided double-double walk returned `0.0`,
/// having taken the other arm of a selection.
///
/// A reference that evaluates a different branch is not a reference for the
/// walk it is measuring. So `real` carries the `f64` result of the same
/// operation sequence, every predicate reads it, and the double-double walk
/// follows the `f64` walk's path exactly — which makes `|value − real|` the
/// rounding error of *that* path and nothing else.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct DoubleDouble {
    hi: f64,
    lo: f64,
    /// What the `f64` walk of the same operations reached. Not a rounding of
    /// `hi`: it is its own computation, and the distance between them is the
    /// measurement this module exists to take.
    real: f64,
}

/// `fl(a + b)` and the exact residual, by 2Sum. No assumption about the
/// operands' relative magnitudes.
#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let sum = a + b;
    let b_part = sum - a;
    (sum, (a - (sum - b_part)) + (b - b_part))
}

/// The same, when `|a| >= |b|` is already known.
#[inline]
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let sum = a + b;
    (sum, b - (sum - a))
}

/// Dekker's split of a double into two 26-bit halves whose sum is exact.
#[inline]
fn split(a: f64) -> (f64, f64) {
    let temp = SPLIT * a;
    let hi = temp - (temp - a);
    (hi, a - hi)
}

/// `fl(a * b)` and the exact residual.
#[inline]
fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let product = a * b;
    if !product.is_finite() {
        return (product, 0.0);
    }
    if a.abs() > SPLIT_CEILING || b.abs() > SPLIT_CEILING {
        return (product, a.mul_add(b, -product));
    }
    let (a_hi, a_lo) = split(a);
    let (b_hi, b_lo) = split(b);
    let residual = (((a_hi * b_hi - product) + a_hi * b_lo) + a_lo * b_hi) + a_lo * b_lo;
    (product, residual)
}

impl DoubleDouble {
    /// A normalized value, with a non-finite high word collapsing the low one.
    ///
    /// The `f64` companion defaults to the double-double value itself, which is
    /// right for the intermediates of a transcendental — those are steps of one
    /// mathematical function, not of the walk being measured. Every operation
    /// the [`CfgScalar`] impl exposes overrides it with the `f64` rule for that
    /// operation; see [`Self::carrying`].
    #[inline]
    fn from_parts(hi: f64, lo: f64) -> Self {
        if hi.is_finite() && lo.is_finite() {
            Self { hi, lo, real: hi }
        } else {
            Self { hi, lo: 0.0, real: hi }
        }
    }

    /// The same double-double value, with the `f64` walk's own result attached.
    #[inline]
    fn carrying(mut self, real: f64) -> Self {
        self.real = real;
        self
    }

    /// What the `f64` walk of the same operations reached.
    pub(super) fn narrow(self) -> f64 {
        self.real
    }

    /// The exact value `hi + lo` names, rounded once to `f64`.
    pub(super) fn to_f64(self) -> f64 {
        self.hi
    }

    /// `|self − value|` as a share of `self`, which is the form an error
    /// measurement is reported in. Infinite where `self` is zero and `value`
    /// is not; zero where both are.
    pub(super) fn relative_distance_to(self, value: f64) -> f64 {
        let difference = self.absolute_distance_to(value);
        if difference == 0.0 {
            return 0.0;
        }
        let scale = self.hi.abs();
        if scale == 0.0 {
            return f64::INFINITY;
        }
        difference / scale
    }

    /// `|self − value|`, taken in double-double and rounded once.
    pub(super) fn absolute_distance_to(self, value: f64) -> f64 {
        if self.hi == value && self.lo == 0.0 {
            return 0.0;
        }
        self.sub(Self::from_f64(value)).hi.abs()
    }

    /// Whether the argument was too large for the double-double `π/2` to
    /// reduce, so a trigonometric result fell back to the `f64` library.
    pub(super) fn reduction_exhausted(value: f64) -> bool {
        !value.is_finite() || value.abs() > TRIG_REDUCTION_CEILING
    }

    /// Multiply by a double, which is exact when the double is.
    ///
    /// Every call site passes a power of two or a small integer, and that is
    /// not a convention but the correctness condition: `scale(1.0 / 3.0)`
    /// would multiply by a *rounded* third and throw away half the format's
    /// precision on the spot. A series term divides by [`Self::over`].
    #[inline]
    fn scale(self, factor: f64) -> Self {
        self.mul(Self::from_f64(factor))
    }

    /// Divide by an exactly-representable integer, in double-double.
    #[inline]
    fn over(self, divisor: f64) -> Self {
        self.div(Self::from_f64(divisor))
    }

    /// `self * self`, one multiplication rather than two.
    #[inline]
    fn square(self) -> Self {
        let (p1, p2) = two_prod(self.hi, self.hi);
        let (h, l) = quick_two_sum(p1, p2 + 2.0 * self.hi * self.lo);
        Self::from_parts(h, l)
    }

    fn recip(self) -> Self {
        Self::from_f64(1.0).div(self)
    }

    fn is_zero(self) -> bool {
        self.hi == 0.0
    }

    fn is_negative(self) -> bool {
        self.hi < 0.0
    }

    /// `exp(self) − 1` on an already-reduced argument, by its Taylor series.
    ///
    /// The series is summed *without* the leading one, and the argument
    /// doubling that follows is written on the same quantity, for a reason
    /// that is the difference between a reference and a rounding: `exp(r)` for
    /// `|r| ≤ 7e-4` is a number just above one, so its absolute error is set by
    /// the one and not by the part that varies, and nine squarings of it
    /// multiply that error by five hundred. `s = exp(r) − 1` is `7e-4`, its
    /// absolute error is five hundred times smaller for the same relative
    /// error, and `s ← 2s + s²` doubles the argument while *keeping* the
    /// relative error. The one is added back at the end, once.
    fn exp_minus_one_series(self) -> Self {
        let mut term = self;
        let mut sum = self;
        let mut index = 1.0_f64;
        for _ in 0..SERIES_TERM_LIMIT {
            index += 1.0;
            term = term.mul(self).over(index);
            if term.hi == 0.0 || term.hi.abs() <= sum.hi.abs() * SERIES_EPSILON {
                break;
            }
            sum = sum.add(term);
        }
        sum
    }

    /// `sin` and `cos` on `|self| <= π/4`, by their Taylor series.
    fn sin_cos_series(self) -> (Self, Self) {
        let square = self.square();
        let mut sine_term = self;
        let mut sine = self;
        let mut cosine_term = Self::from_f64(1.0);
        let mut cosine = cosine_term;
        let mut index = 0.0_f64;
        for _ in 0..SERIES_TERM_LIMIT {
            index += 1.0;
            let even = 2.0 * index;
            cosine_term = cosine_term.mul(square).over(-((even - 1.0) * even));
            cosine = cosine.add(cosine_term);
            sine_term = sine_term.mul(square).over(-(even * (even + 1.0)));
            sine = sine.add(sine_term);
            let settled = (sine_term.hi == 0.0 || sine_term.hi.abs() <= SERIES_EPSILON)
                && (cosine_term.hi == 0.0 || cosine_term.hi.abs() <= SERIES_EPSILON);
            if settled {
                break;
            }
        }
        (sine, cosine)
    }

    /// `sin` and `cos` together, which is what the reduction produces and what
    /// `atan2`'s Newton step needs.
    fn sin_cos(self) -> (Self, Self) {
        if Self::reduction_exhausted(self.hi) {
            let (sine, cosine) = self.hi.sin_cos();
            return (Self::from_f64(sine), Self::from_f64(cosine));
        }
        let quadrant = (self.hi / PI_OVER_2.hi).round();
        let reduced = self.sub(PI_OVER_2.scale(quadrant));
        let (sine, cosine) = reduced.sin_cos_series();
        match quadrant_index(quadrant) {
            0 => (sine, cosine),
            1 => (cosine, sine.negated()),
            2 => (sine.negated(), cosine.negated()),
            _ => (cosine.negated(), sine),
        }
    }

    #[inline]
    fn negated(self) -> Self {
        Self::from_parts(-self.hi, -self.lo)
    }

    /// `atan2`, by Newton's method on `sin` and `cos` from the `f64` estimate.
    fn atan2_of(self, rhs: Self) -> Self {
        let (y, x) = (self, rhs);
        if x.is_zero() && y.is_zero() {
            return Self::from_f64(f64::atan2(y.hi, x.hi));
        }
        if !x.hi.is_finite() || !y.hi.is_finite() {
            return Self::from_f64(f64::atan2(y.hi, x.hi));
        }
        if x.is_zero() {
            return if y.is_negative() {
                PI_OVER_2.negated()
            } else {
                PI_OVER_2
            };
        }
        if y.is_zero() {
            return if x.is_negative() {
                if self.hi.is_sign_negative() { PI.negated() } else { PI }
            } else {
                Self::from_f64(0.0)
            };
        }
        let radius = y.square().add(x.square()).sqrt();
        let sine = y.div(radius);
        let cosine = x.div(radius);
        let mut angle = Self::from_f64(f64::atan2(y.hi, x.hi));
        for _ in 0..2 {
            let (s, c) = angle.sin_cos();
            angle = if cosine.hi.abs() > sine.hi.abs() {
                angle.add(sine.sub(s).div(c))
            } else {
                angle.sub(cosine.sub(c).div(s))
            };
        }
        angle
    }
}

/// The double-double arithmetic itself, without the `f64` companion.
///
/// Inherent rather than the trait impl, and the split is load-bearing: these
/// are the operations a transcendental composes itself out of, where carrying a
/// separate `f64` walk would be meaningless — `exp`'s Taylor series is one
/// function's own steps, not two evaluations of a model. The [`CfgScalar`] impl
/// below is a thin layer over these that attaches the `f64` rule for each
/// operation, and it is the one an interpreted walk sees.
impl DoubleDouble {
    fn from_f64(value: f64) -> Self {
        Self {
            hi: value,
            lo: 0.0,
            real: value,
        }
    }

    fn neg(self) -> Self {
        self.negated()
    }

    fn add(self, rhs: Self) -> Self {
        let (s1, s2) = two_sum(self.hi, rhs.hi);
        if !s1.is_finite() {
            return Self::from_parts(s1, 0.0);
        }
        let (t1, t2) = two_sum(self.lo, rhs.lo);
        let (s1, s2) = quick_two_sum(s1, s2 + t1);
        let (s1, s2) = quick_two_sum(s1, s2 + t2);
        Self::from_parts(s1, s2)
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.negated())
    }

    fn mul(self, rhs: Self) -> Self {
        let (p1, p2) = two_prod(self.hi, rhs.hi);
        if !p1.is_finite() {
            return Self::from_parts(p1, 0.0);
        }
        let (h, l) = quick_two_sum(p1, p2 + (self.hi * rhs.lo + self.lo * rhs.hi));
        Self::from_parts(h, l)
    }

    fn div(self, rhs: Self) -> Self {
        let quotient = self.hi / rhs.hi;
        if !quotient.is_finite() || !self.hi.is_finite() || !rhs.hi.is_finite() {
            return Self::from_parts(quotient, 0.0);
        }
        let remainder = self.sub(rhs.scale(quotient));
        let correction = remainder.hi / rhs.hi;
        let remainder = remainder.sub(rhs.scale(correction));
        let residual = remainder.hi / rhs.hi;
        let (h, l) = quick_two_sum(quotient, correction);
        Self::from_parts(h, l).add(Self::from_f64(residual))
    }

    // `fmod`, taken in double-double rather than borrowed from `f64::%`: the
    // operands here are double-double values, and the exactness `%` has for
    // two doubles says nothing about them.
    fn rem(self, rhs: Self) -> Self {
        let quotient = self.div(rhs);
        if !quotient.hi.is_finite() || quotient.hi.abs() >= 9.007_199_254_740_992e15 {
            return Self::from_f64(self.hi % rhs.hi);
        }
        let whole = Self::from_f64(quotient.hi.trunc());
        self.sub(whole.mul(rhs))
    }

    // Integer exponents by repeated squaring, which is exact up to the
    // roundings the products themselves commit; everything else through
    // `exp(y ln x)`, with the sign and zero cases `f64::powf` states.
    fn powf(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            return Self::from_f64(1.0);
        }
        if self.is_zero() || !self.hi.is_finite() || !rhs.hi.is_finite() {
            return Self::from_f64(self.hi.powf(rhs.hi));
        }
        let integral = rhs.lo == 0.0 && rhs.hi.fract() == 0.0 && rhs.hi.abs() <= 4096.0;
        if integral {
            return self.powi(rhs.hi as i64);
        }
        if self.is_negative() {
            return Self::from_f64(f64::NAN);
        }
        rhs.mul(self.ln()).exp()
    }

    fn hypot(self, rhs: Self) -> Self {
        if !self.hi.is_finite() || !rhs.hi.is_finite() {
            return Self::from_f64(f64::hypot(self.hi, rhs.hi));
        }
        let largest = self.hi.abs().max(rhs.hi.abs());
        if largest == 0.0 {
            return Self::from_f64(0.0);
        }
        // Scale by a power of two, which is exact, so the squares cannot
        // overflow or flush a legitimate operand to zero.
        let exponent = i32::try_from((largest.to_bits() >> 52) & 0x7ff).unwrap_or(1023) - 1023;
        if !(-500..=500).contains(&exponent) {
            return Self::from_f64(f64::hypot(self.hi, rhs.hi));
        }
        let scale = 2.0_f64.powi(-exponent);
        let scaled = self
            .scale(scale)
            .square()
            .add(rhs.scale(scale).square())
            .sqrt();
        scaled.scale(2.0_f64.powi(exponent))
    }

    fn atan2(self, rhs: Self) -> Self {
        self.atan2_of(rhs)
    }

    fn exp(self) -> Self {
        if self.hi.is_nan() {
            return Self::from_f64(f64::NAN);
        }
        if self.hi > 709.782_712_893_384 {
            return Self::from_f64(f64::INFINITY);
        }
        if self.hi < -745.133_219_101_941_2 {
            return Self::from_f64(0.0);
        }
        if self.is_zero() && self.lo == 0.0 {
            return Self::from_f64(1.0);
        }
        // `a = m ln2 + r'`, with `|r'| <= ln2/2`, and `r = r'/512` so the
        // series converges in a handful of terms.
        let multiple = (self.hi / LN2.hi).round();
        let reduced = self.sub(LN2.scale(multiple)).scale(1.0 / 512.0);
        let mut excess = reduced.exp_minus_one_series();
        for _ in 0..9 {
            excess = excess.scale(2.0).add(excess.square());
        }
        let value = excess.add(Self::from_f64(1.0));
        if !(-1000.0..=1000.0).contains(&multiple) {
            return Self::from_f64(self.hi.exp());
        }
        value.scale(2.0_f64.powi(multiple as i32))
    }

    // Newton on `exp` from the `f64` logarithm: sixteen correct digits become
    // thirty-two, which is the whole of the double-double format.
    //
    // The argument is first halved or doubled into `[1/2, 2]`. That is exact —
    // the scalings are powers of two — and it is not an optimization: Newton
    // asks for `exp(−estimate)`, and the logarithm of a number near `f64`'s
    // exponent limits is an estimate near `±745`, whose exponential is an
    // overflow or a flush to zero. Reduced, the estimate is under `0.7` and the
    // iteration converges on the first step.
    fn ln(self) -> Self {
        if self.hi <= 0.0 || !self.hi.is_finite() {
            return Self::from_f64(self.hi.ln());
        }
        if self.hi == 1.0 && self.lo == 0.0 {
            return Self::from_f64(0.0);
        }
        let mut reduced = self;
        let mut doublings = 0.0_f64;
        while reduced.hi > 2.0 {
            reduced = reduced.scale(0.5);
            doublings += 1.0;
        }
        while reduced.hi < 0.5 {
            reduced = reduced.scale(2.0);
            doublings -= 1.0;
        }
        let mut estimate = Self::from_f64(reduced.hi.ln());
        for _ in 0..2 {
            let correction = reduced.mul(estimate.negated().exp()).sub(Self::from_f64(1.0));
            estimate = estimate.add(correction);
        }
        estimate.add(LN2.scale(doublings))
    }

    fn log10(self) -> Self {
        self.ln().div(LN10)
    }

    // Newton on the reciprocal square root, which needs no division: the
    // residual `a - s^2` is formed in double-double, where the cancellation
    // that makes it small is exact.
    fn sqrt(self) -> Self {
        if self.hi == 0.0 {
            return Self::from_f64(0.0);
        }
        if self.hi < 0.0 || !self.hi.is_finite() {
            return Self::from_f64(self.hi.sqrt());
        }
        let inverse_root = 1.0 / self.hi.sqrt();
        let root = self.hi * inverse_root;
        let residual = self.sub(Self::from_f64(root).square()).hi;
        let (h, l) = quick_two_sum(root, residual * inverse_root * 0.5);
        Self::from_parts(h, l)
    }

    fn abs(self) -> Self {
        if self.hi < 0.0 { self.negated() } else { self }
    }

    fn sin(self) -> Self {
        self.sin_cos().0
    }

    fn cos(self) -> Self {
        self.sin_cos().1
    }

    fn tan(self) -> Self {
        let (sine, cosine) = self.sin_cos();
        sine.div(cosine)
    }

    // Below the threshold the closed form subtracts two quantities that agree
    // to `|a|^2`, so the series is what carries the digits; above it the two
    // exponentials are far enough apart that the subtraction is harmless.
    fn sinh(self) -> Self {
        if self.hi.abs() < 0.05 {
            let square = self.square();
            let mut term = self;
            let mut sum = self;
            let mut index = 1.0_f64;
            for _ in 0..SERIES_TERM_LIMIT {
                index += 2.0;
                term = term.mul(square).over((index - 1.0) * index);
                if term.hi == 0.0 || term.hi.abs() <= sum.hi.abs() * SERIES_EPSILON {
                    break;
                }
                sum = sum.add(term);
            }
            return sum;
        }
        let raised = self.exp();
        raised.sub(raised.recip()).scale(0.5)
    }

    fn cosh(self) -> Self {
        let raised = self.exp();
        raised.add(raised.recip()).scale(0.5)
    }

    fn tanh(self) -> Self {
        if self.hi.abs() < 0.05 {
            return self.sinh().div(self.cosh());
        }
        if self.hi.abs() > 30.0 {
            return Self::from_f64(self.hi.signum());
        }
        let raised = self.scale(2.0).exp();
        raised
            .sub(Self::from_f64(1.0))
            .div(raised.add(Self::from_f64(1.0)))
    }

    fn asin(self) -> Self {
        if self.hi.abs() > 1.0 {
            return Self::from_f64(f64::NAN);
        }
        let complement = Self::from_f64(1.0).sub(self.square()).sqrt();
        if complement.is_zero() {
            return if self.is_negative() {
                PI_OVER_2.negated()
            } else {
                PI_OVER_2
            };
        }
        self.atan2_of(complement)
    }

    fn acos(self) -> Self {
        if self.hi.abs() > 1.0 {
            return Self::from_f64(f64::NAN);
        }
        let complement = Self::from_f64(1.0).sub(self.square()).sqrt();
        complement.atan2_of(self)
    }

    fn atan(self) -> Self {
        self.atan2_of(Self::from_f64(1.0))
    }

    fn asinh(self) -> Self {
        if self.hi.abs() < 1.0e-4 {
            let square = self.square();
            let mut term = self;
            let mut sum = self;
            let mut index = 0.0_f64;
            for _ in 0..SERIES_TERM_LIMIT {
                index += 1.0;
                let odd = 2.0 * index;
                // `(-1)^k (2k-1)!! / ((2k)!! (2k+1))`, built from the term
                // before it: multiply by `-(2k-1)(2k)/((2k)(2k+1)) * ...`
                term = term
                    .mul(square)
                    .scale(-((odd - 1.0) * (odd - 1.0)))
                    .over(odd * (odd + 1.0));
                if term.hi == 0.0 || term.hi.abs() <= sum.hi.abs() * SERIES_EPSILON {
                    break;
                }
                sum = sum.add(term);
            }
            return sum;
        }
        let magnitude = self.abs();
        let value = magnitude
            .add(magnitude.square().add(Self::from_f64(1.0)).sqrt())
            .ln();
        if self.is_negative() { value.negated() } else { value }
    }

    fn acosh(self) -> Self {
        if self.hi < 1.0 {
            return Self::from_f64(f64::NAN);
        }
        self.add(self.square().sub(Self::from_f64(1.0)).sqrt()).ln()
    }

    fn atanh(self) -> Self {
        if self.hi.abs() > 1.0 {
            return Self::from_f64(f64::NAN);
        }
        if self.hi.abs() < 1.0e-4 {
            let square = self.square();
            let mut power = self;
            let mut sum = self;
            let mut index = 1.0_f64;
            for _ in 0..SERIES_TERM_LIMIT {
                index += 2.0;
                power = power.mul(square);
                let term = power.over(index);
                if term.hi == 0.0 || term.hi.abs() <= sum.hi.abs() * SERIES_EPSILON {
                    break;
                }
                sum = sum.add(term);
            }
            return sum;
        }
        let one = Self::from_f64(1.0);
        one.add(self).div(one.sub(self)).ln().scale(0.5)
    }

    fn floor(self) -> Self {
        let hi = self.hi.floor();
        if hi == self.hi {
            let (h, l) = quick_two_sum(hi, self.lo.floor());
            Self::from_parts(h, l)
        } else {
            Self::from_parts(hi, 0.0)
        }
    }

    fn ceil(self) -> Self {
        let hi = self.hi.ceil();
        if hi == self.hi {
            let (h, l) = quick_two_sum(hi, self.lo.ceil());
            Self::from_parts(h, l)
        } else {
            Self::from_parts(hi, 0.0)
        }
    }
}

/// The scalar an interpreted walk sees: the double-double arithmetic above,
/// with the `f64` walk of the same operations carried alongside.
///
/// Every method is the inherent operation plus [`DoubleDouble::carrying`] of
/// the corresponding `f64` rule — the *same* rule
/// [`impl CfgScalar for f64`](CfgScalar) uses, so the companion is the walk
/// being measured and not an approximation of it. [`Self::real`] returns that
/// companion, which is what makes every predicate take the `f64` walk's branch.
///
/// `limexp`, `limited_exp` and `limited_exp_derivative` are left to their
/// default bodies and need no rule here: they are written in terms of these
/// operations and of `real`, so they inherit both halves.
impl CfgScalar for DoubleDouble {
    fn from_f64(value: f64) -> Self {
        Self::from_f64(value)
    }

    /// The `f64` walk's value, not the double-double one. See the type's
    /// documentation for why the reference must branch on the walk it measures.
    fn real(self) -> f64 {
        self.real
    }

    fn neg(self) -> Self {
        Self::neg(self).carrying(-self.real)
    }

    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs).carrying(self.real + rhs.real)
    }

    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs).carrying(self.real - rhs.real)
    }

    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs).carrying(self.real * rhs.real)
    }

    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs).carrying(self.real / rhs.real)
    }

    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs).carrying(self.real % rhs.real)
    }

    fn powf(self, rhs: Self) -> Self {
        Self::powf(self, rhs).carrying(self.real.powf(rhs.real))
    }

    fn hypot(self, rhs: Self) -> Self {
        Self::hypot(self, rhs).carrying(f64::hypot(self.real, rhs.real))
    }

    fn atan2(self, rhs: Self) -> Self {
        Self::atan2(self, rhs).carrying(f64::atan2(self.real, rhs.real))
    }

    fn exp(self) -> Self {
        Self::exp(self).carrying(self.real.exp())
    }

    fn ln(self) -> Self {
        Self::ln(self).carrying(self.real.ln())
    }

    fn log10(self) -> Self {
        Self::log10(self).carrying(self.real.log10())
    }

    fn sqrt(self) -> Self {
        Self::sqrt(self).carrying(self.real.sqrt())
    }

    fn abs(self) -> Self {
        Self::abs(self).carrying(self.real.abs())
    }

    fn sin(self) -> Self {
        Self::sin(self).carrying(self.real.sin())
    }

    fn cos(self) -> Self {
        Self::cos(self).carrying(self.real.cos())
    }

    fn tan(self) -> Self {
        Self::tan(self).carrying(self.real.tan())
    }

    fn sinh(self) -> Self {
        Self::sinh(self).carrying(self.real.sinh())
    }

    fn cosh(self) -> Self {
        Self::cosh(self).carrying(self.real.cosh())
    }

    fn tanh(self) -> Self {
        Self::tanh(self).carrying(self.real.tanh())
    }

    fn asin(self) -> Self {
        Self::asin(self).carrying(self.real.asin())
    }

    fn acos(self) -> Self {
        Self::acos(self).carrying(self.real.acos())
    }

    fn atan(self) -> Self {
        Self::atan(self).carrying(self.real.atan())
    }

    fn asinh(self) -> Self {
        Self::asinh(self).carrying(self.real.asinh())
    }

    fn acosh(self) -> Self {
        Self::acosh(self).carrying(self.real.acosh())
    }

    fn atanh(self) -> Self {
        Self::atanh(self).carrying(self.real.atanh())
    }

    fn floor(self) -> Self {
        Self::floor(self).carrying(self.real.floor())
    }

    fn ceil(self) -> Self {
        Self::ceil(self).carrying(self.real.ceil())
    }
}

/// Which of the four quadrants a reduced argument came out of.
fn quadrant_index(multiple: f64) -> i64 {
    (multiple as i64).rem_euclid(4)
}

impl DoubleDouble {
    /// Repeated squaring, which is what makes an integer exponent exact where
    /// `exp(y ln x)` would round twice through transcendentals.
    fn powi(self, exponent: i64) -> Self {
        let negative = exponent < 0;
        let mut remaining = exponent.unsigned_abs();
        let mut base = self;
        let mut accumulated = Self::from_f64(1.0);
        while remaining > 0 {
            if remaining & 1 == 1 {
                accumulated = accumulated.mul(base);
            }
            remaining >>= 1;
            if remaining > 0 {
                base = base.square();
            }
        }
        if negative {
            accumulated.recip()
        } else {
            accumulated
        }
    }
}

/// The same operating point, lifted into double-double with every input exact.
///
/// Exact is the only right seed: the question is how far the `f64` walk of this
/// expression is from the exact real value of *the same expression at the same
/// inputs*, so the inputs' own provenance is outside it. Both routes read the
/// identical doubles.
pub(super) fn lift_inputs(real: &CfgEvalInputs<f64>) -> CfgEvalInputs<DoubleDouble> {
    let lift = |values: &[f64]| -> Vec<DoubleDouble> {
        values
            .iter()
            .copied()
            .map(DoubleDouble::from_f64)
            .collect()
    };
    CfgEvalInputs {
        parameters: lift(&real.parameters),
        parameter_given: real.parameter_given.clone(),
        port_connected: real.port_connected.clone(),
        event_state: lift(&real.event_state),
        node_potentials: lift(&real.node_potentials),
        branch_flows: lift(&real.branch_flows),
        branch_unknown_flows: lift(&real.branch_unknown_flows),
        temperature: DoubleDouble::from_f64(real.temperature),
        thermal_voltage: DoubleDouble::from_f64(real.thermal_voltage),
        multiplicity: DoubleDouble::from_f64(real.multiplicity),
        time: DoubleDouble::from_f64(real.time),
        analyses: real.analyses.clone(),
        simparams: real.simparams.clone(),
        ddt: DoubleDouble::from_f64(real.ddt),
        ddt_scale: DoubleDouble::from_f64(real.ddt_scale),
        idt: DoubleDouble::from_f64(real.idt),
        idt_scale: DoubleDouble::from_f64(real.idt_scale),
        event_controls: real
            .event_controls
            .iter()
            .map(|(key, value)| (*key, DoubleDouble::from_f64(*value)))
            .collect(),
        staged: lift(&real.staged),
    }
}

#[cfg(test)]
mod tests {
    use super::{DoubleDouble, LN10, LN2, PI, PI_OVER_2, two_prod, two_sum};
    use crate::canonical_ir::CfgScalar;

    fn dd(value: f64) -> DoubleDouble {
        DoubleDouble::from_f64(value)
    }

    /// How many double-double units in the last place `value` is from `exact`.
    ///
    /// `2^-106` is the format's own resolution, so a function accurate to a
    /// handful of these is accurate to everything below it.
    fn dd_ulp_error(value: DoubleDouble, exact: DoubleDouble) -> f64 {
        let difference = value.sub(exact);
        let scale = exact.hi.abs();
        if scale == 0.0 {
            return difference.hi.abs() / f64::EPSILON.powi(2);
        }
        (difference.hi.abs() / scale) / (f64::EPSILON * f64::EPSILON / 4.0)
    }

    /// The error-free transformations are exact, which is what every other
    /// claim in this module rests on.
    #[test]
    fn the_transformations_recover_the_rounding_exactly() {
        let cases = [
            (1.0, 1.0e-20),
            (1.0e300, 1.0e280),
            (0.1, 0.2),
            (-7.5, 7.499_999_999),
            (3.0, 1.0 / 3.0),
        ];
        for (a, b) in cases {
            let (sum, residual) = two_sum(a, b);
            // The residual is exactly what the sum dropped, so adding it back
            // in double-double reproduces `a + b` with no rounding of its own.
            assert_eq!(dd(a).add(dd(b)).hi, sum, "{a}+{b}");
            assert_eq!(dd(a).add(dd(b)).lo, residual, "{a}+{b} residual");

            let (product, error) = two_prod(a, b);
            assert_eq!(product, a * b);
            // Recomputed in a way that cannot use the same code path: the
            // exact product of two doubles fits in a double-double.
            let recovered = dd(product).add(dd(error));
            assert_eq!(recovered.hi, a * b, "{a}*{b}");
        }
    }

    /// A double-double sum of a large and a small quantity keeps the small one,
    /// which is the whole point: `f64` loses it.
    #[test]
    fn the_format_holds_what_f64_drops() {
        let large = dd(1.0e16);
        let small = dd(1.0);
        assert_eq!(1.0e16_f64 + 1.0, 1.0e16, "f64 drops the unit");
        let sum = large.add(small);
        assert_eq!(sum.hi, 1.0e16);
        assert_eq!(sum.lo, 1.0);
        // And subtracting the large part back returns the unit exactly.
        assert_eq!(sum.sub(large).hi, 1.0);
    }

    /// The cancellation that defeats a forward bound is exact here.
    ///
    /// Two quantities at `1e28` differing in the twentieth figure: `f64`
    /// returns a difference with no correct digits, and the double-double
    /// walk returns it to full precision. This is `l_utsoi_102`'s shape.
    #[test]
    fn a_catastrophic_cancellation_is_exact() {
        let a = dd(1.0e28).add(dd(3.0));
        let b = dd(1.0e28);
        let difference = a.sub(b);
        assert_eq!(difference.hi, 3.0);
        assert_eq!(difference.lo, 0.0);
        // The same subtraction in `f64` loses the three entirely.
        assert_eq!((1.0e28_f64 + 3.0) - 1.0e28, 0.0);
    }

    /// The constants are the double-double values they claim to be, checked
    /// against the functions rather than against a copied table.
    #[test]
    fn the_named_constants_are_accurate() {
        assert!(dd_ulp_error(dd(2.0).ln(), LN2) < 16.0);
        assert!(dd_ulp_error(dd(10.0).ln(), LN10) < 16.0);
        assert!(dd_ulp_error(PI_OVER_2.scale(2.0), PI) < 4.0);
        // `sin(π) = 0` to the format's resolution: the residual is the
        // constant's own error, not the series'.
        assert!(PI.sin().hi.abs() < 1.0e-31, "{:e}", PI.sin().hi);
        assert!(dd_ulp_error(PI_OVER_2.sin(), dd(1.0)) < 16.0);
    }

    /// Every primitive round-trips against the identity that defines it, to
    /// within a few units in the double-double last place.
    #[test]
    fn the_primitives_satisfy_their_identities() {
        let values = [
            0.5,
            1.0,
            2.0,
            7.0,
            1.0 / 3.0,
            1.234_567_890_123_456_7e5,
            9.876e-8,
        ];
        for value in values {
            let x = dd(value);
            // (x/3)*3 == x
            let round_trip = x.div(dd(3.0)).mul(dd(3.0));
            assert!(dd_ulp_error(round_trip, x) < 8.0, "div/mul {value}");
            // sqrt(x)^2 == x
            let squared = x.sqrt().square();
            assert!(dd_ulp_error(squared, x) < 8.0, "sqrt {value}");
            // exp(ln x) == x. `ln` commits an absolute error of a few units in
            // the last place of `ln x`, and `exp` turns that into the same
            // relative error, so the round trip is relative.
            assert!(dd_ulp_error(x.ln().exp(), x) < 64.0, "exp/ln {value}");
            // ln(exp x) == x, on arguments `exp` does not overflow — and
            // *absolutely* rather than relatively, because it cannot be
            // relative: `exp` of a small `x` is a number near one, whose own
            // resolution is `u_dd`, so `ln` of it cannot recover `x` to better
            // than `u_dd` however exact both functions are. That is the format
            // speaking, not the implementation.
            if value.abs() < 100.0 {
                let round_trip = x.exp().ln().sub(x);
                let allowed = 64.0 * f64::EPSILON * f64::EPSILON / 4.0 * (1.0 + value.abs());
                assert!(
                    round_trip.hi.abs() <= allowed,
                    "ln/exp {value}: {:e} > {allowed:e}",
                    round_trip.hi
                );
            }
        }
    }

    /// The transcendentals agree with `f64`'s to within the `f64` library's
    /// own accuracy, which is what says the reduction and the series are
    /// computing the intended function rather than a different one.
    #[test]
    fn the_transcendentals_round_to_the_f64_library() {
        let mut checked = 0_usize;
        for step in -40_i32..=40 {
            let value = f64::from(step) * 0.37;
            let x = dd(value);
            let pairs: [(f64, f64, &str); 11] = [
                (x.exp().hi, value.exp(), "exp"),
                (x.sin().hi, value.sin(), "sin"),
                (x.cos().hi, value.cos(), "cos"),
                (x.tan().hi, value.tan(), "tan"),
                (x.atan().hi, value.atan(), "atan"),
                (x.sinh().hi, value.sinh(), "sinh"),
                (x.cosh().hi, value.cosh(), "cosh"),
                (x.tanh().hi, value.tanh(), "tanh"),
                (x.asinh().hi, value.asinh(), "asinh"),
                (
                    x.atan2(dd(1.5)).hi,
                    f64::atan2(value, 1.5),
                    "atan2",
                ),
                (x.abs().hi, value.abs(), "abs"),
            ];
            for (ours, theirs, name) in pairs {
                if !theirs.is_finite() || theirs == 0.0 {
                    continue;
                }
                let relative = (ours - theirs).abs() / theirs.abs();
                assert!(
                    relative < 4.0 * f64::EPSILON,
                    "{name}({value}): {ours:.17e} vs {theirs:.17e} ({relative:e})"
                );
                checked += 1;
            }
        }
        for step in 1_i32..=60 {
            let value = f64::from(step) * 0.11;
            let x = dd(value);
            let mut pairs: Vec<(f64, f64, &str)> = vec![
                (x.ln().hi, value.ln(), "ln"),
                (x.log10().hi, value.log10(), "log10"),
                (x.sqrt().hi, value.sqrt(), "sqrt"),
                (x.powf(dd(1.7)).hi, value.powf(1.7), "powf"),
                (x.hypot(dd(2.5)).hi, f64::hypot(value, 2.5), "hypot"),
            ];
            if value <= 1.0 {
                pairs.push((x.asin().hi, value.asin(), "asin"));
                pairs.push((x.acos().hi, value.acos(), "acos"));
                pairs.push((x.atanh().hi, value.atanh(), "atanh"));
            }
            if value >= 1.0 {
                pairs.push((x.acosh().hi, value.acosh(), "acosh"));
            }
            for (ours, theirs, name) in pairs {
                if !theirs.is_finite() || theirs == 0.0 {
                    continue;
                }
                let relative = (ours - theirs).abs() / theirs.abs();
                assert!(
                    relative < 4.0 * f64::EPSILON,
                    "{name}({value}): {ours:.17e} vs {theirs:.17e} ({relative:e})"
                );
                checked += 1;
            }
        }
        assert!(checked > 800, "the sweep has to actually check something");
    }

    /// The reference is far more accurate than `f64`, which is the claim the
    /// census rests its measurement on.
    ///
    /// A sum of terms whose partial sums cancel: `f64` returns a result with a
    /// handful of correct digits and the double-double walk returns one whose
    /// residual is thirty orders down.
    #[test]
    fn the_reference_is_orders_more_accurate_than_the_walk_it_measures() {
        // (1 + d)^2 - 1 - 2d == d^2, computed the way that cancels.
        let delta = 1.0e-8_f64;
        let in_f64 = (1.0 + delta) * (1.0 + delta) - 1.0 - 2.0 * delta;
        let one = dd(1.0);
        let d = dd(delta);
        let in_dd = one
            .add(d)
            .mul(one.add(d))
            .sub(one)
            .sub(d.scale(2.0));
        let exact = delta * delta;
        let dd_error = (in_dd.hi - exact).abs() / exact;
        let f64_error = (in_f64 - exact).abs() / exact;
        // The reference resolves the cancelled term to the format's absolute
        // resolution — `u_dd` against the `1` the expression subtracts off —
        // while the `f64` walk resolves it to `u`, sixteen orders coarser.
        assert!(dd_error < 1.0e-14, "{dd_error:e}");
        assert!(f64_error > 1.0e-2, "{f64_error:e}");
        assert!(f64_error / dd_error > 1.0e10, "{:e}", f64_error / dd_error);
    }

    /// An integer exponent is taken by squaring, so it is not the double
    /// rounding `exp(y ln x)` would commit.
    #[test]
    fn an_integer_exponent_avoids_the_transcendentals() {
        let base = dd(1.5);
        assert_eq!(base.powf(dd(0.0)).hi, 1.0);
        assert_eq!(base.powf(dd(2.0)).hi, 2.25);
        assert_eq!(base.powf(dd(2.0)).lo, 0.0, "2.25 is exact");
        // A negative base is defined for an integer exponent and not
        // otherwise, which is `f64::powf`'s rule.
        assert_eq!(dd(-2.0).powf(dd(3.0)).hi, -8.0);
        assert!(dd(-2.0).powf(dd(1.5)).hi.is_nan());
        // And a wide integer exponent still agrees with the library, which
        // reaches the same value through the logarithm this path avoids.
        let large = dd(1.000_000_1).powf(dd(1000.0));
        let library = 1.000_000_1_f64.powf(1000.0);
        assert!(
            (large.hi - library).abs() <= 4.0 * f64::EPSILON * library,
            "{:.17e} vs {library:.17e}",
            large.hi
        );
    }

    /// Non-finite values pass through as the single doubles they are, rather
    /// than as a NaN an error-free transformation manufactured.
    #[test]
    fn the_non_finite_cases_do_not_manufacture_a_nan() {
        let infinite = dd(f64::INFINITY);
        assert!(infinite.add(dd(1.0)).hi.is_infinite());
        assert_eq!(infinite.add(dd(1.0)).lo, 0.0);
        assert!(infinite.mul(dd(2.0)).hi.is_infinite());
        assert!(dd(1.0).div(dd(0.0)).hi.is_infinite());
        assert!(dd(0.0).ln().hi.is_infinite());
        assert!(dd(-1.0).sqrt().hi.is_nan());
        assert!(dd(f64::NAN).add(dd(1.0)).hi.is_nan());
    }

    /// `%` and the sign-bit operations behave as the interpreter expects.
    #[test]
    fn the_exact_operations_agree_with_f64() {
        assert_eq!(dd(7.5).rem(dd(2.0)).hi, 7.5_f64 % 2.0);
        assert_eq!(dd(-7.5).rem(dd(2.0)).hi, -7.5_f64 % 2.0);
        assert_eq!(dd(2.7).floor().hi, 2.0);
        assert_eq!(dd(-2.7).floor().hi, -3.0);
        assert_eq!(dd(2.1).ceil().hi, 3.0);
        assert_eq!(dd(-2.1).ceil().hi, -2.0);
        assert_eq!(dd(-3.5).abs().hi, 3.5);
        assert_eq!(dd(3.5).neg().hi, -3.5);
    }

    /// `limexp` and the limited exponential compose from the primitive rules,
    /// on both sides of their thresholds, exactly as they do for `f64`.
    #[test]
    fn the_default_bodies_compose_from_the_primitives() {
        for argument in [-10.0, 0.0, 1.0, 40.0, 1.0e3] {
            let ours = dd(argument).limexp().hi;
            let theirs = argument.limexp();
            assert!(
                (ours - theirs).abs() <= 4.0 * f64::EPSILON * theirs.abs(),
                "limexp({argument}): {ours:e} vs {theirs:e}"
            );
            let ours = dd(argument).limited_exp().hi;
            let theirs = argument.limited_exp();
            assert!(
                (ours - theirs).abs() <= 4.0 * f64::EPSILON * theirs.abs(),
                "limited_exp({argument}): {ours:e} vs {theirs:e}"
            );
        }
    }

    /// The distance measurements the census reports are taken in
    /// double-double, so a difference below the `f64` resolution is still a
    /// number rather than zero.
    #[test]
    fn a_distance_below_the_f64_resolution_is_still_measured() {
        let value = dd(1.0).add(dd(1.0e-20));
        assert_eq!(value.hi, 1.0, "the high word cannot hold it");
        assert_eq!(value.absolute_distance_to(1.0), 1.0e-20);
        assert_eq!(value.relative_distance_to(1.0), 1.0e-20);
        assert_eq!(value.relative_distance_to(value.to_f64()), 1.0e-20);
        // An exact agreement is exactly zero, with no epsilon anywhere.
        assert_eq!(dd(2.5).relative_distance_to(2.5), 0.0);
        assert_eq!(dd(0.0).relative_distance_to(0.0), 0.0);
        assert!(dd(0.0).relative_distance_to(1.0).is_infinite());
    }

    /// The scalar an interpreted walk sees branches on the `f64` walk, not on
    /// its own wider value.
    ///
    /// This is what makes the reference a reference. A cancellation the `f64`
    /// walk rounds to zero is a real quantity here, and a predicate that read
    /// the wider value would send the two walks into different arms of a
    /// piecewise function — which is what `vbic13_4t`'s `jacobians[2][0]` did
    /// before the companion was carried: `1.0` on both compiled routes, `0.0`
    /// from the reference.
    #[test]
    fn the_reference_branches_on_the_walk_it_measures() {
        let large = dd(1.0e16);
        let cancelled = CfgScalar::sub(CfgScalar::add(large, dd(1.0)), large);
        // The wider walk kept the unit; the `f64` walk lost it.
        assert_eq!(cancelled.to_f64(), 1.0);
        assert_eq!(cancelled.narrow(), 0.0);
        assert_eq!(CfgScalar::real(cancelled), 0.0, "predicates read the walk");
        // So a selection driven by it takes the arm the `f64` walk takes, and
        // the reference is then the exact value *of that arm*.
        let selected = if CfgScalar::real(cancelled) > 0.5 {
            dd(10.0)
        } else {
            dd(20.0)
        };
        assert_eq!(selected.to_f64(), 20.0);
        // And the measurement of that cancellation is the whole unit it lost.
        assert_eq!(cancelled.absolute_distance_to(cancelled.narrow()), 1.0);
        // The companion follows the `f64` rule at every step, transcendentals
        // included: `exp` of the two differs because the arguments do.
        let raised = CfgScalar::exp(cancelled);
        assert_eq!(raised.narrow(), 0.0_f64.exp());
        assert!((raised.to_f64() - 1.0_f64.exp()).abs() < 1.0e-15);
    }

    /// The logarithm holds at both ends of `f64`'s exponent range, where a
    /// Newton step on the unreduced argument would ask `exp` for `±745`.
    #[test]
    fn the_logarithm_holds_at_the_exponent_limits() {
        for value in [
            f64::MIN_POSITIVE,
            1.0e-300,
            5.0e-324,
            1.0e-30,
            1.0e30,
            1.0e300,
            f64::MAX,
        ] {
            let ours = dd(value).ln();
            let theirs = value.ln();
            assert!(ours.hi.is_finite(), "ln({value:e}) = {:e}", ours.hi);
            assert!(
                (ours.hi - theirs).abs() <= 4.0 * f64::EPSILON * theirs.abs(),
                "ln({value:e}): {:.17e} vs {theirs:.17e}",
                ours.hi
            );
            // And it is a double-double logarithm, not the library's lifted:
            // exponentiating it returns the argument to the wider precision.
            // Only where the format has a low word to return it in — within a
            // few hundred orders of `f64`'s exponent limits the low word is
            // subnormal or absent, and a double-double is a double there.
            if (1.0e-290..1.0e290).contains(&value.abs()) {
                assert!(dd_ulp_error(ours.exp(), dd(value)) < 4096.0, "{value:e}");
            }
        }
    }

    /// The trigonometric reduction says where it stops rather than returning a
    /// number with no digits left in it.
    #[test]
    fn the_reduction_states_its_own_ceiling() {
        assert!(!DoubleDouble::reduction_exhausted(1.0e14));
        assert!(DoubleDouble::reduction_exhausted(1.0e16));
        assert!(DoubleDouble::reduction_exhausted(f64::INFINITY));
        // Below the ceiling the reduction still delivers: sin of a large
        // multiple of π/2 is what it should be.
        let large = PI_OVER_2.scale(1.0e6).add(PI_OVER_2.scale(1.0 / 3.0));
        let expected = (PI_OVER_2.scale(1.0 / 3.0)).sin();
        assert!(dd_ulp_error(large.sin(), expected) < 1.0e10, "reduction");
    }
}
