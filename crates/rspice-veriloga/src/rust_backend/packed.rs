//! Array-form chain rules for the width-parameterized lowering.
//!
//! Where the scalarized lowering materializes one value per live lane, this
//! emits one `[f64; L]` binding per differentiated value and lets LLVM unroll
//! it. The rules here mirror `ScalarGraphBuilder::lower_*_derivatives` in
//! `canonical_ir::opt` operation for operation — same factors, same order of
//! operands — because matching that exactly is what keeps the two backends
//! numerically interchangeable while both exist.
//!
//! ## Uniform width instead of per-lane presence
//!
//! The scalarized rules track which lanes an operand actually has and emit only
//! those: `a - b` with a derivative on the right alone yields `-db`, not
//! `0 - db`. Here every value carries all `L` lanes, with absent ones held at
//! zero, so the same case yields `0.0 - db`.
//!
//! Those agree on every finite value. They differ only in the sign of zero —
//! `0.0 - 0.0` is `+0.0` where `-(0.0)` is `-0.0` — which no matrix stamp can
//! observe, since both add the same quantity to the same entry. The wasted
//! arithmetic on absent lanes is real but small: the probe in
//! `benchmarks/reference/lowering-probe` measured the array form scaling
//! sub-linearly in `L`, which is why uniform width beats tracking presence.
//!
//! Unused lanes must hold exactly `0.0` rather than being left undefined. Add,
//! sub and mul propagate zeros; division does too as long as the primal is
//! finite. A garbage lane would otherwise turn into a NaN that spreads to
//! every downstream lane.

use crate::canonical_ir::{OptBinaryOp, OptUnaryOp};

/// How a value's derivative array is built from its operands'.
///
/// `Scaled` covers every unary rule that is a single factor times the operand
/// derivative, which is most of them; the factor is emitted once outside the
/// lane loop rather than recomputed per lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LaneRule {
    /// The value does not depend on any unknown; no array is emitted.
    Zero,
    /// `d[k] = <factor> * input_d[k]`, with `factor` bound once.
    Scaled { factor: String },
    /// `d[k] = input_d[k] / <divisor>`, with `divisor` bound once.
    ///
    /// Kept as a division rather than multiplication by a reciprocal because
    /// the scalarized rules divide, and `x / y` is not `x * (1/y)` in binary
    /// floating point.
    Divided { divisor: String },
    /// `d[k] = -input_d[k]`.
    Negated,
    /// `d[k] = input_d[k]`.
    Copied,
    /// A rule over two operand arrays, written as a per-lane expression in
    /// terms of `l[k]` and `r[k]`.
    Binary { lane_expr: String },
}

/// Array-form derivative of a unary operation.
///
/// `input` and `value` are expressions for the operand's primal and this
/// value's own primal; several rules are cheaper in terms of the output (the
/// derivative of `exp` is the result itself) and the scalarized rules already
/// take that shortcut, so matching it also matches their rounding.
pub(super) fn unary_rule(op: OptUnaryOp, input: &str, value: &str) -> LaneRule {
    match op {
        OptUnaryOp::Pos => LaneRule::Copied,
        OptUnaryOp::Neg => LaneRule::Negated,
        // d(exp(x)) = exp(x) dx, and exp(x) is this value.
        OptUnaryOp::Exp => LaneRule::Scaled {
            factor: value.to_string(),
        },
        OptUnaryOp::LimExp => LaneRule::Scaled {
            factor: format!("limexp_derivative({input})"),
        },
        OptUnaryOp::LimitedExp => LaneRule::Scaled {
            factor: format!("limited_exp_derivative({input})"),
        },
        OptUnaryOp::Ln => LaneRule::Divided {
            divisor: input.to_string(),
        },
        // d(sqrt(x)) = dx / (2 sqrt(x)), and sqrt(x) is this value.
        OptUnaryOp::Sqrt => LaneRule::Divided {
            divisor: format!("(2.0 * {value})"),
        },
        OptUnaryOp::Sin => LaneRule::Scaled {
            factor: format!("({input}).cos()"),
        },
        OptUnaryOp::Cos => LaneRule::Scaled {
            factor: format!("(-(({input}).sin()))"),
        },
        OptUnaryOp::Tan => LaneRule::Divided {
            divisor: format!("(({input}).cos() * ({input}).cos())"),
        },
        OptUnaryOp::Sinh => LaneRule::Scaled {
            factor: format!("({input}).cosh()"),
        },
        OptUnaryOp::Cosh => LaneRule::Scaled {
            factor: format!("({input}).sinh()"),
        },
        // d(tanh(x)) = (1 - tanh(x)^2) dx, in terms of this value.
        OptUnaryOp::Tanh => LaneRule::Scaled {
            factor: format!("(1.0 - {value} * {value})"),
        },
        OptUnaryOp::Atan => LaneRule::Divided {
            divisor: format!("(1.0 + {input} * {input})"),
        },
        OptUnaryOp::Asinh => LaneRule::Divided {
            divisor: format!("((1.0 + {input} * {input}).sqrt())"),
        },
        // Piecewise-constant or boolean: no derivative contribution at all.
        // `abs` is deliberately among these — the scalarized rules drop it
        // rather than pick a subgradient at the kink, and diverging here would
        // silently change every model that takes an absolute value.
        OptUnaryOp::Abs
        | OptUnaryOp::Floor
        | OptUnaryOp::Ceil
        | OptUnaryOp::Not
        | OptUnaryOp::LimExpDerivative
        | OptUnaryOp::LimitedExpDerivative => LaneRule::Zero,
    }
}

/// Array-form derivative of a binary operation.
///
/// `left`/`right` are the operand primals and `value` this value's primal.
/// Lane arrays are referred to as `l` and `r`.
pub(super) fn binary_rule(
    op: OptBinaryOp,
    left: &str,
    right: &str,
    value: &str,
) -> LaneRule {
    match op {
        OptBinaryOp::Add => LaneRule::Binary {
            lane_expr: "l[k] + r[k]".to_string(),
        },
        OptBinaryOp::Sub => LaneRule::Binary {
            lane_expr: "l[k] - r[k]".to_string(),
        },
        // Product rule, operands in the same order the scalarized builder uses.
        OptBinaryOp::Mul => LaneRule::Binary {
            lane_expr: format!("l[k] * {right} + {left} * r[k]"),
        },
        // Quotient rule expressed through the already-computed quotient, which
        // is what the scalarized builder does: (dl - q dr) / rhs.
        OptBinaryOp::Div => LaneRule::Binary {
            lane_expr: format!("(l[k] - {value} * r[k]) / {right}"),
        },
        // d(a^b) = a^b (b/a da + ln(a) db). Left as a single expression so the
        // exponent term vanishes cleanly when the exponent is constant.
        OptBinaryOp::Pow => LaneRule::Binary {
            lane_expr: format!(
                "{value} * (({right} / {left}) * l[k] + ({left}).ln() * r[k])"
            ),
        },
        // Comparisons, logic and modulo are piecewise constant.
        OptBinaryOp::Mod
        | OptBinaryOp::Eq
        | OptBinaryOp::Ne
        | OptBinaryOp::Lt
        | OptBinaryOp::Le
        | OptBinaryOp::Gt
        | OptBinaryOp::Ge
        | OptBinaryOp::And
        | OptBinaryOp::Or => LaneRule::Zero,
    }
}

impl LaneRule {
    /// Render the rule as a `[f64; L]` initializer.
    ///
    /// `input` names the single operand array for the unary forms; `left` and
    /// `right` name the two arrays the binary form reads as `l` and `r`.
    pub(super) fn emit(&self, width: usize, input: &str, left: &str, right: &str) -> Option<String> {
        match self {
            Self::Zero => None,
            Self::Copied => Some(input.to_string()),
            Self::Negated => Some(format!(
                "core::array::from_fn::<f64, {width}, _>(|k| -{input}[k])"
            )),
            Self::Scaled { factor } => Some(format!(
                "{{ let s = {factor}; core::array::from_fn::<f64, {width}, _>(|k| s * {input}[k]) }}"
            )),
            Self::Divided { divisor } => Some(format!(
                "{{ let q = {divisor}; core::array::from_fn::<f64, {width}, _>(|k| {input}[k] / q) }}"
            )),
            Self::Binary { lane_expr } => Some(format!(
                "{{ let l = &{left}; let r = &{right}; core::array::from_fn::<f64, {width}, _>(|k| {lane_expr}) }}"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_scales_by_its_own_result_not_a_recomputed_exp() {
        // Recomputing exp(x) here would round differently from the primal and
        // desynchronize the two backends.
        let rule = unary_rule(OptUnaryOp::Exp, "v3", "v7");
        assert_eq!(
            rule,
            LaneRule::Scaled {
                factor: "v7".to_string()
            }
        );
    }

    #[test]
    fn logarithm_divides_rather_than_scaling_by_a_reciprocal() {
        // x / y and x * (1/y) are not the same in binary floating point, and
        // the scalarized rules divide.
        let rule = unary_rule(OptUnaryOp::Ln, "v3", "v7");
        assert_eq!(
            rule,
            LaneRule::Divided {
                divisor: "v3".to_string()
            }
        );
        let emitted = rule.emit(4, "d3", "", "").expect("ln has a derivative");
        assert!(emitted.contains("d3[k] / q"), "{emitted}");
        assert!(!emitted.contains("* q"), "{emitted}");
    }

    #[test]
    fn abs_has_no_derivative() {
        // The scalarized rules refuse to pick a subgradient at the kink.
        // Diverging would silently change every model that takes an absolute
        // value, so this must stay Zero rather than become sign(x).
        assert_eq!(unary_rule(OptUnaryOp::Abs, "v1", "v2"), LaneRule::Zero);
        assert_eq!(unary_rule(OptUnaryOp::Floor, "v1", "v2"), LaneRule::Zero);
        assert_eq!(unary_rule(OptUnaryOp::Ceil, "v1", "v2"), LaneRule::Zero);
    }

    #[test]
    fn comparisons_and_modulo_have_no_derivative() {
        for op in [
            OptBinaryOp::Mod,
            OptBinaryOp::Eq,
            OptBinaryOp::Lt,
            OptBinaryOp::And,
            OptBinaryOp::Or,
        ] {
            assert_eq!(binary_rule(op, "a", "b", "v"), LaneRule::Zero);
        }
    }

    #[test]
    fn product_rule_keeps_the_scalarized_operand_order() {
        let rule = binary_rule(OptBinaryOp::Mul, "va", "vb", "vc");
        assert_eq!(
            rule,
            LaneRule::Binary {
                lane_expr: "l[k] * vb + va * r[k]".to_string()
            }
        );
    }

    #[test]
    fn quotient_rule_reuses_the_quotient_it_already_computed() {
        let rule = binary_rule(OptBinaryOp::Div, "va", "vb", "vq");
        assert_eq!(
            rule,
            LaneRule::Binary {
                lane_expr: "(l[k] - vq * r[k]) / vb".to_string()
            }
        );
    }

    #[test]
    fn emitted_arrays_carry_their_width() {
        // A `from_fn` without an explicit width infers it from context, which
        // silently compiles to the wrong length if a binding's type changes.
        let scaled = unary_rule(OptUnaryOp::Exp, "v1", "v2")
            .emit(19, "d1", "", "")
            .expect("exp has a derivative");
        assert!(scaled.contains("from_fn::<f64, 19, _>"), "{scaled}");

        let binary = binary_rule(OptBinaryOp::Add, "a", "b", "v")
            .emit(7, "", "da", "db")
            .expect("add has a derivative");
        assert!(binary.contains("from_fn::<f64, 7, _>"), "{binary}");
        assert!(binary.contains("let l = &da;"), "{binary}");
        assert!(binary.contains("let r = &db;"), "{binary}");
    }

    #[test]
    fn zero_rules_emit_nothing_at_all() {
        // Not an array of zeros: a value with no dependence on any unknown
        // should cost no binding, which is most of a compact model's
        // temperature and geometry arithmetic.
        assert_eq!(unary_rule(OptUnaryOp::Floor, "a", "b").emit(8, "d", "", ""), None);
        assert_eq!(
            binary_rule(OptBinaryOp::Gt, "a", "b", "v").emit(8, "", "l", "r"),
            None
        );
    }
}
