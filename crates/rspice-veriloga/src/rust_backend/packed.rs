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

use std::collections::HashSet;

use crate::canonical_ir::{OptBinaryOp, OptModel, OptUnaryOp, OptValueKind, ValueId};

/// Where a value's derivative comes from, structurally.
///
/// Derived from the value's own operator rather than from a `derivatives`
/// list, because the primal graph the packed lowering reads has none — that is
/// the whole point of reading it before the expansion pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Dependence {
    /// Independent of every unknown; costs one binding, not two.
    None,
    /// A seed: this value *is* an unknown, so its array is a unit vector.
    Seed,
    /// Differentiated exactly when its listed operands are.
    Propagates,
    /// Differentiated regardless of its operands.
    ///
    /// `$limit` is the case: it contributes an affine correction lane whether
    /// or not the proposed value depends on an unknown.
    Always,
}

/// How a value's derivative relates to its operands, before any lane arithmetic.
pub(super) fn dependence(kind: &OptValueKind) -> Dependence {
    match kind {
        // The three ways an unknown enters a model.
        OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. } => Dependence::Seed,

        OptValueKind::Limit { .. } => Dependence::Always,

        // Runtime loop carriers keep a derivative on every lane, because the
        // trip count is not known until the model runs.
        OptValueKind::RuntimeLoopVariable { .. } | OptValueKind::RuntimeLoopResult { .. } => {
            Dependence::Always
        }

        OptValueKind::Unary { op, .. } => {
            if unary_rule(*op, "", "") == LaneRule::Zero {
                Dependence::None
            } else {
                Dependence::Propagates
            }
        }
        OptValueKind::Binary { op, .. } => {
            if binary_rule(*op, "", "", "") == LaneRule::Zero {
                Dependence::None
            } else {
                Dependence::Propagates
            }
        }

        OptValueKind::Select { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::CountedSum { .. }
        | OptValueKind::SimParam { .. } => Dependence::Propagates,

        // Constants, model parameters, ambient quantities, and the values that
        // deliberately break the derivative chain. `Ddx` is among them: it
        // reports a derivative rather than carrying one, and `LimitPrevious`
        // holds the previous iterate, which Newton treats as fixed.
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::SimParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::DdtScale
        | OptValueKind::LimitPrevious { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. }
        | OptValueKind::EquationValue { .. } => Dependence::None,
    }
}

/// Operands whose derivative feeds this value's.
///
/// Only the operands the chain rule actually reads: a `Select`'s condition is
/// excluded because it is boolean, and `Limit`'s candidate because the affine
/// correction is formed from the primal displacement rather than a derivative.
fn differentiating_operands(kind: &OptValueKind) -> Vec<ValueId> {
    match kind {
        OptValueKind::Unary { input, .. } => vec![*input],
        OptValueKind::Binary { left, right, .. } => vec![*left, *right],
        OptValueKind::Select {
            then_value,
            else_value,
            ..
        } => vec![*then_value, *else_value],
        OptValueKind::Ddt { input, .. } => vec![*input],
        OptValueKind::SimParam { fallback, .. } => vec![*fallback],
        OptValueKind::CountedSum { initial, term, .. } => vec![*initial, *term],
        OptValueKind::Limit { proposed, .. } => vec![*proposed],
        _ => Vec::new(),
    }
}

/// Values that need a derivative array emitted.
///
/// Everything reachable from a seed through operators that propagate. Values
/// outside this set — a compact model's temperature scaling, geometry
/// arithmetic and parameter preprocessing, which is a large share of it — cost
/// a single primal binding.
///
/// Relies on operands preceding their consumers in the value list, which the
/// graph builder guarantees by construction.
pub(super) fn differentiated_values(opt: &OptModel) -> HashSet<ValueId> {
    let mut differentiated = HashSet::new();
    for value in &opt.values {
        let needs = match dependence(&value.kind) {
            Dependence::None => false,
            Dependence::Seed | Dependence::Always => true,
            Dependence::Propagates => differentiating_operands(&value.kind)
                .into_iter()
                .any(|operand| differentiated.contains(&operand)),
        };
        if needs {
            differentiated.insert(value.id);
        }
    }
    differentiated
}

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
    fn the_three_unknown_sources_seed_derivatives() {
        use crate::canonical_ir::{BranchId, BranchUnknownId, NodeId};
        assert_eq!(
            dependence(&OptValueKind::NodePotential {
                node: NodeId::from(0)
            }),
            Dependence::Seed
        );
        assert_eq!(
            dependence(&OptValueKind::BranchFlow {
                branch: BranchId::from(0)
            }),
            Dependence::Seed
        );
        assert_eq!(
            dependence(&OptValueKind::BranchUnknownFlow {
                branch_unknown: BranchUnknownId::from(0)
            }),
            Dependence::Seed
        );
    }

    #[test]
    fn derivative_killing_operators_stop_propagation() {
        // A comparison over a node potential is still independent of every
        // unknown. Treating it as dependent would emit a derivative array for
        // every guard in the model.
        assert_eq!(
            dependence(&OptValueKind::Binary {
                op: OptBinaryOp::Lt,
                left: ValueId::from(0),
                right: ValueId::from(1),
            }),
            Dependence::None
        );
        assert_eq!(
            dependence(&OptValueKind::Unary {
                op: OptUnaryOp::Floor,
                input: ValueId::from(0),
            }),
            Dependence::None
        );
    }

    #[test]
    fn ddx_and_previous_iterate_break_the_chain() {
        // `ddx` reports a derivative rather than carrying one, and Newton holds
        // the previous iterate fixed. Both must terminate propagation or the
        // Jacobian picks up terms the reference does not have.
        assert_eq!(
            dependence(&OptValueKind::Ddx {
                value: ValueId::from(0),
                pos_node: None,
                neg_node: None,
            }),
            Dependence::None
        );
        assert_eq!(
            dependence(&OptValueKind::LimitPrevious {
                operator: crate::canonical_ir::ExprId::from(0),
                proposed: ValueId::from(0),
            }),
            Dependence::None
        );
    }

    #[test]
    fn limit_is_differentiated_even_when_its_input_is_not() {
        // $limit contributes an affine correction lane regardless, so it cannot
        // be gated on the proposed value depending on an unknown.
        assert_eq!(
            dependence(&OptValueKind::Limit {
                operator: crate::canonical_ir::ExprId::from(0),
                proposed: ValueId::from(0),
                candidate: ValueId::from(1),
            }),
            Dependence::Always
        );
    }

    #[test]
    fn select_propagates_through_arms_not_its_condition() {
        // The condition is boolean; only the arms can carry a derivative.
        let operands = differentiating_operands(&OptValueKind::Select {
            condition: ValueId::from(7),
            then_value: ValueId::from(8),
            else_value: ValueId::from(9),
        });
        assert_eq!(operands, vec![ValueId::from(8), ValueId::from(9)]);
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
