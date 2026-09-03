//! A first-order forward rounding-error bound, carried alongside the value.
//!
//! # What this is
//!
//! [`ErrorBounded`] is a [`CfgScalar`], so the reference interpreter
//! ([`crate::canonical_ir::evaluate_cfg`]) walks a CFG with it exactly as it
//! walks one with `f64` or with [`ComplexStep`](crate::canonical_ir::ComplexStep).
//! Alongside each value `v` it carries `e_v`, a bound on `|computed − exact|`
//! to first order: the inputs are exact as given (`e = 0`), and each operation
//! adds its own rounding to the error its operands already carried, weighted by
//! that operation's partial derivatives.
//!
//! This is the standard first-order forward error analysis — Higham,
//! *Accuracy and Stability of Numerical Algorithms*, ch. 3 — and not interval
//! arithmetic: nothing is widened to contain the true value, and the walk takes
//! the same branches the `f64` walk takes, because every predicate reads
//! [`CfgScalar::real`]. It costs one extra `f64` per value and one extra
//! multiply-add per operation.
//!
//! # Why it is not a tolerance
//!
//! The number it produces is a property of the *computation*: the same cone at
//! a different bias gives a different bound, and a cone with no cancellation in
//! it gives a bound of a few units in the last place no matter how the run came
//! out. [`super::cfg_mir_census`] uses it as a floor under the agreement
//! criterion for a Jacobian entry, so an entry whose own arithmetic cannot
//! deliver nine figures is judged against what `f64` can deliver on it, while a
//! genuinely wrong derivative — which is orders outside a rounding bound —
//! still fails.
//!
//! # Where the bound is charged
//!
//! Two kinds of term compose into `e_out`:
//!
//! * **propagation**, `Σ_i |∂op/∂x_i|(x) · e_i`: the error the operands already
//!   carried, amplified by this operation's sensitivity to them. This is where
//!   an ill-conditioned intermediate shows up — a subtraction of two nearly
//!   equal large numbers returns a small result while carrying both operands'
//!   errors forward undiminished.
//! * **rounding**, `u · |op(x)|` with `u = f64::EPSILON / 2`: the unit
//!   round-off this operation itself commits. It is exact for the
//!   correctly-rounded operations (`+ − × ÷ √`, and `%`, which is exact), and it
//!   is the nominal charge for the library transcendentals, whose worst-case
//!   error is about one unit in the last place — twice `u`. The census's
//!   [`AGREEMENT_ERROR_FACTOR`](super::cfg_mir_census) carries that factor of
//!   two rather than this module inflating every operation by it.
//!
//! Three op classes carry no rounding at all, and each for a reason rather than
//! for convenience:
//!
//! * `neg` and `abs` change only the sign bit;
//! * `%` (`fmod`) is exact for finite operands, so it charges propagation only;
//! * comparisons, `!`, and the integer/bitwise operators produce a value from
//!   [`CfgScalar::from_f64`], which is exact by construction — a Boolean mask is
//!   `0.0` or `1.0` and carries no error into what selects on it. `min`, `max`
//!   and the block terminators *select* rather than blend, so the winning arm's
//!   error is what continues.
//!
//! # What the analysis assumes
//!
//! First order: products of two error terms are dropped, which is right while
//! every `e_i` is small against its own `|x_i|` and is the same assumption the
//! `(1 + u)^n − 1 ≈ n·u` reading in
//! [`REASSOCIATION_BUDGET`](super::cfg_mir_census) makes.
//!
//! Differentiability at the point: `min`, `max`, `abs`, `floor` and `ceil` are
//! charged their one-sided partial, which is the honest bound everywhere except
//! exactly at a kink. A cone sitting on a kink is not a place where two chain
//! rules can be asked to agree in the first place — the census's significance
//! gate is what excludes those — so the assumption is shared rather than new.

use crate::canonical_ir::{CfgEvalInputs, CfgScalar};

/// The unit round-off: half an ulp, relative.
pub(super) const UNIT_ROUNDOFF: f64 = f64::EPSILON / 2.0;

/// A value and a first-order bound on how far it is from the exact result of
/// the same real-arithmetic expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ErrorBounded {
    pub(super) value: f64,
    /// `≥ |value − exact|`, to first order. Never negative for a well-formed
    /// walk; non-finite where the computation itself is.
    pub(super) error: f64,
}

impl ErrorBounded {
    /// The bound as a share of the value it belongs to, which is the form the
    /// census compares against a relative deviation.
    pub(super) fn relative(self) -> f64 {
        if self.value == 0.0 {
            return if self.error == 0.0 { 0.0 } else { f64::INFINITY };
        }
        self.error / self.value.abs()
    }
}

/// One propagation term, `|∂op/∂x| · e`.
///
/// The guard is not defensive rounding: an operand that is *exact* contributes
/// nothing however singular the partial is, and `0.0 * f64::INFINITY` is a NaN
/// that would poison a bound the arithmetic never earned. `ln(x)` at `x = 0`
/// with an exact operand is the case in the corpus.
fn term(partial: f64, error: f64) -> f64 {
    if error == 0.0 {
        0.0
    } else {
        partial.abs() * error
    }
}

/// A correctly-rounded operation: propagated error plus this step's own
/// half-ulp.
fn rounded(value: f64, propagated: f64) -> ErrorBounded {
    ErrorBounded {
        value,
        error: propagated + UNIT_ROUNDOFF * value.abs(),
    }
}

/// An operation that commits no rounding of its own.
fn exact(value: f64, propagated: f64) -> ErrorBounded {
    ErrorBounded {
        value,
        error: propagated,
    }
}

impl CfgScalar for ErrorBounded {
    fn from_f64(value: f64) -> Self {
        // A literal, a parameter, a node potential and a Boolean mask are all
        // exact *as given*: the exact expression this bound is measured against
        // is the one that uses these same doubles.
        Self { value, error: 0.0 }
    }

    fn real(self) -> f64 {
        self.value
    }

    fn neg(self) -> Self {
        exact(-self.value, self.error)
    }

    fn add(self, rhs: Self) -> Self {
        rounded(self.value + rhs.value, self.error + rhs.error)
    }

    // Both operands' errors survive undiminished while the result may be
    // arbitrarily small: this is where cancellation enters the bound, and it is
    // the whole reason an ill-conditioned cone reports one.
    fn sub(self, rhs: Self) -> Self {
        rounded(self.value - rhs.value, self.error + rhs.error)
    }

    fn mul(self, rhs: Self) -> Self {
        rounded(
            self.value * rhs.value,
            term(rhs.value, self.error) + term(self.value, rhs.error),
        )
    }

    fn div(self, rhs: Self) -> Self {
        rounded(
            self.value / rhs.value,
            term(1.0 / rhs.value, self.error)
                + term(self.value / (rhs.value * rhs.value), rhs.error),
        )
    }

    // `fmod` is exact for finite operands, so there is no rounding term. The
    // partial in the second operand is the (integral) quotient the remainder
    // subtracts off.
    fn rem(self, rhs: Self) -> Self {
        let quotient = (self.value / rhs.value).trunc();
        exact(
            self.value % rhs.value,
            self.error + term(quotient, rhs.error),
        )
    }

    fn powf(self, rhs: Self) -> Self {
        let value = self.value.powf(rhs.value);
        rounded(
            value,
            term(rhs.value * self.value.powf(rhs.value - 1.0), self.error)
                + term(value * self.value.ln(), rhs.error),
        )
    }

    fn hypot(self, rhs: Self) -> Self {
        let value = f64::hypot(self.value, rhs.value);
        rounded(
            value,
            term(self.value / value, self.error) + term(rhs.value / value, rhs.error),
        )
    }

    fn atan2(self, rhs: Self) -> Self {
        let square = self.value * self.value + rhs.value * rhs.value;
        rounded(
            f64::atan2(self.value, rhs.value),
            term(rhs.value / square, self.error) + term(self.value / square, rhs.error),
        )
    }

    fn exp(self) -> Self {
        let value = self.value.exp();
        rounded(value, term(value, self.error))
    }

    fn ln(self) -> Self {
        rounded(self.value.ln(), term(1.0 / self.value, self.error))
    }

    fn log10(self) -> Self {
        rounded(
            self.value.log10(),
            term(1.0 / (self.value * std::f64::consts::LN_10), self.error),
        )
    }

    fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        rounded(value, term(0.5 / value, self.error))
    }

    fn abs(self) -> Self {
        exact(self.value.abs(), self.error)
    }

    fn sin(self) -> Self {
        rounded(self.value.sin(), term(self.value.cos(), self.error))
    }

    fn cos(self) -> Self {
        rounded(self.value.cos(), term(self.value.sin(), self.error))
    }

    fn tan(self) -> Self {
        let value = self.value.tan();
        rounded(value, term(1.0 + value * value, self.error))
    }

    fn sinh(self) -> Self {
        rounded(self.value.sinh(), term(self.value.cosh(), self.error))
    }

    fn cosh(self) -> Self {
        rounded(self.value.cosh(), term(self.value.sinh(), self.error))
    }

    fn tanh(self) -> Self {
        let value = self.value.tanh();
        rounded(value, term(1.0 - value * value, self.error))
    }

    fn asin(self) -> Self {
        rounded(
            self.value.asin(),
            term(1.0 / (1.0 - self.value * self.value).sqrt(), self.error),
        )
    }

    fn acos(self) -> Self {
        rounded(
            self.value.acos(),
            term(1.0 / (1.0 - self.value * self.value).sqrt(), self.error),
        )
    }

    fn atan(self) -> Self {
        rounded(
            self.value.atan(),
            term(1.0 / (1.0 + self.value * self.value), self.error),
        )
    }

    fn asinh(self) -> Self {
        rounded(
            self.value.asinh(),
            term(1.0 / (1.0 + self.value * self.value).sqrt(), self.error),
        )
    }

    fn acosh(self) -> Self {
        rounded(
            self.value.acosh(),
            term(1.0 / (self.value * self.value - 1.0).sqrt(), self.error),
        )
    }

    fn atanh(self) -> Self {
        rounded(
            self.value.atanh(),
            term(1.0 / (1.0 - self.value * self.value), self.error),
        )
    }

    // Piecewise constant, and exactly representable: zero derivative, zero
    // rounding. The step itself is a kink, and is excluded by the same
    // differentiability assumption `min` and `max` rest on.
    fn floor(self) -> Self {
        exact(self.value.floor(), 0.0)
    }

    fn ceil(self) -> Self {
        exact(self.value.ceil(), 0.0)
    }
}

/// The same operating point, lifted into the error lane with every input exact.
///
/// Exact is the right seed and not an approximation: the bound answers "how far
/// is this evaluation from the exact real-arithmetic value of the same
/// expression at these same inputs", so an input's own provenance is outside
/// the question. Both routes read the identical doubles.
pub(super) fn lift_inputs(real: &CfgEvalInputs<f64>) -> CfgEvalInputs<ErrorBounded> {
    let lift = |values: &[f64]| -> Vec<ErrorBounded> {
        values.iter().copied().map(ErrorBounded::from_f64).collect()
    };
    CfgEvalInputs {
        parameters: lift(&real.parameters),
        parameter_given: real.parameter_given.clone(),
        port_connected: real.port_connected.clone(),
        event_state: lift(&real.event_state),
        node_potentials: lift(&real.node_potentials),
        branch_flows: lift(&real.branch_flows),
        branch_unknown_flows: lift(&real.branch_unknown_flows),
        temperature: ErrorBounded::from_f64(real.temperature),
        thermal_voltage: ErrorBounded::from_f64(real.thermal_voltage),
        multiplicity: ErrorBounded::from_f64(real.multiplicity),
        time: ErrorBounded::from_f64(real.time),
        analyses: real.analyses.clone(),
        simparams: real.simparams.clone(),
        ddt: ErrorBounded::from_f64(real.ddt),
        ddt_scale: ErrorBounded::from_f64(real.ddt_scale),
        idt: ErrorBounded::from_f64(real.idt),
        idt_scale: ErrorBounded::from_f64(real.idt_scale),
        event_controls: real
            .event_controls
            .iter()
            .map(|(key, value)| (*key, ErrorBounded::from_f64(*value)))
            .collect(),
        staged: lift(&real.staged),
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorBounded, UNIT_ROUNDOFF};
    use crate::canonical_ir::CfgScalar;

    fn seed(value: f64) -> ErrorBounded {
        ErrorBounded::from_f64(value)
    }

    /// An input is exact, and a chain of well-conditioned operations stays
    /// within a few units in the last place of it.
    #[test]
    fn a_well_conditioned_chain_stays_within_a_few_ulp() {
        let x = seed(3.0);
        let y = seed(7.0);
        let product = x.mul(y);
        let sum = product.add(seed(1.0));
        let scaled = sum.div(seed(2.0));
        assert_eq!(seed(3.0).error, 0.0);
        // Three roundings, each at most `u` of its own result. Charged against
        // the final magnitude the bound is a small multiple of `u`.
        assert!(
            scaled.relative() <= 4.0 * UNIT_ROUNDOFF,
            "{:e}",
            scaled.relative()
        );
        assert_eq!(scaled.value, 11.0);
    }

    /// Cancellation is what the lane exists to see: the operands' errors carry
    /// through a subtraction undiminished while the result collapses, so the
    /// bound *relative to the result* grows by the condition number.
    #[test]
    fn cancellation_reports_the_condition_number() {
        // Two large numbers computed with one rounding each, differing in the
        // ninth figure of their difference.
        let large = seed(1.0e8).mul(seed(1.000_000_01));
        let other = seed(1.0e8).mul(seed(1.0));
        let difference = large.sub(other);
        assert!(difference.value > 0.0);
        // The absolute bound is set by the large operands, so relative to a
        // result eight orders smaller it is eight orders larger than `u`.
        let ratio = difference.relative() / UNIT_ROUNDOFF;
        assert!(ratio > 1.0e7, "{ratio:e}");
        assert!(ratio < 1.0e9, "{ratio:e}");
    }

    /// A mask is `0.0` or `1.0` exactly and carries no error, which is what
    /// makes a selection driven by one carry only the winning arm's error.
    /// Selection itself lives in the interpreter's `apply_binary`, and is
    /// exercised end to end by
    /// [`super::super::cfg_mir_census`]'s mis-differentiated-`max` fixture.
    #[test]
    fn a_predicate_result_is_exact() {
        assert_eq!(seed(1.0).error, 0.0);
        assert_eq!(seed(0.0).error, 0.0);
    }

    /// A singular partial with an exact operand contributes nothing rather than
    /// a NaN out of `0 * inf`. `sqrt(0)` and `ln(0)` are the corpus cases.
    #[test]
    fn an_exact_operand_survives_a_singular_partial() {
        let root = seed(0.0).sqrt();
        assert_eq!(root.value, 0.0);
        assert_eq!(root.error, 0.0, "the 1/(2√x) partial multiplies an exact 0");

        let logarithm = seed(0.0).ln();
        assert!(logarithm.value.is_infinite());
        // `u * |−inf|` is infinite, which is honest. What must not happen is a
        // NaN, which would be the propagation term reading `0 * inf`.
        assert!(!logarithm.error.is_nan(), "{:e}", logarithm.error);
    }

    /// `%` and the sign-bit operations charge nothing of their own.
    #[test]
    fn the_exact_operations_charge_no_rounding() {
        let x = seed(7.5).mul(seed(1.0));
        assert!(x.error > 0.0);
        assert_eq!(x.neg().error, x.error);
        assert_eq!(x.abs().error, x.error);
        assert_eq!(x.rem(seed(2.0)).error, x.error);
        assert_eq!(seed(2.7).floor().error, 0.0);
        assert_eq!(seed(2.7).ceil().error, 0.0);
    }

    /// `limexp`'s default trait body is built from `sub`, `add` and `mul`, so
    /// the lane covers it without a rule of its own — on both sides of the
    /// threshold.
    #[test]
    fn the_default_bodies_compose_from_the_primitive_rules() {
        let below = seed(1.0).limexp();
        assert!(below.error > 0.0);
        assert!(below.relative() <= 4.0 * UNIT_ROUNDOFF, "{:e}", below.error);
        let above = seed(1.0e3).limexp();
        assert!(above.relative() <= 8.0 * UNIT_ROUNDOFF, "{:e}", above.error);
    }
}
