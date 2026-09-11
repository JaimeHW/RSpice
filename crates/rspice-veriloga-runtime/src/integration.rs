//! Pure integration candidates and shared circular-integrator history handling.

use super::arithmetic::{IdtModOrigin, sum_products, sum_products_div};
use super::{GeneratedDdtCoefficients, Value};

/// Accepted history consumed by one generated `idt` candidate evaluation.
///
/// The history is immutable: evaluating a Newton candidate must not publish it
/// as accepted state. An uninitialized operator starts from its initial
/// condition and uses the current input as its synthetic previous input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedIdtAcceptedHistory {
    pub initialized: bool,
    pub integral_previous: Value,
    pub integral_older: Value,
    pub input_previous: Value,
}

/// Pure result of applying the selected companion rule to one `idt` slot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedIdtCandidate {
    pub value: Value,
    /// Exact partial derivative of the candidate integral with respect to its
    /// current input. This is zero outside active transient integration.
    pub jacobian_scale: Value,
}

/// Malformed numeric input to [`evaluate_generated_idt_candidate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedIdtCandidateError {
    NonFiniteInput { field: &'static str },
    ZeroDerivativeScale,
    NonFiniteResult { field: &'static str },
}

impl std::fmt::Display for GeneratedIdtCandidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput { field } => {
                write!(f, "generated idt {field} must be finite")
            }
            Self::ZeroDerivativeScale => {
                f.write_str("generated idt active derivative scale must be nonzero")
            }
            Self::NonFiniteResult { field } => {
                write!(f, "generated idt produced a non-finite {field}")
            }
        }
    }
}

impl std::error::Error for GeneratedIdtCandidateError {}

/// Evaluate one generalized `idt` candidate without mutating accepted state.
///
/// For an active companion rule, this algebraically inverts the same
/// derivative formula used by generated `ddt`:
///
/// `integral = (input + pv*previous + ov*older + pd*previous_input) / scale`.
///
/// Inactive integration returns the initial condition with a zero Jacobian.
/// Every numeric operand is validated even when the history is uninitialized,
/// so corrupted accepted lanes cannot remain latent until a later step.
#[inline]
pub fn evaluate_generated_idt_candidate(
    coefficients: GeneratedDdtCoefficients,
    input: Value,
    initial_condition: Value,
    history: GeneratedIdtAcceptedHistory,
) -> Result<GeneratedIdtCandidate, GeneratedIdtCandidateError> {
    let finite_inputs = [
        ("input", input),
        ("initial condition", initial_condition),
        ("accepted previous integral", history.integral_previous),
        ("accepted older integral", history.integral_older),
        ("accepted previous input", history.input_previous),
        ("derivative scale", coefficients.derivative_scale),
        ("previous-value scale", coefficients.previous_value_scale),
        ("older-value scale", coefficients.older_value_scale),
        (
            "previous-derivative scale",
            coefficients.previous_derivative_scale,
        ),
    ];
    if let Some((field, _)) = finite_inputs
        .into_iter()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(GeneratedIdtCandidateError::NonFiniteInput { field });
    }

    if !coefficients.active {
        return Ok(GeneratedIdtCandidate {
            value: initial_condition,
            jacobian_scale: 0.0,
        });
    }
    if coefficients.derivative_scale == 0.0 {
        return Err(GeneratedIdtCandidateError::ZeroDerivativeScale);
    }

    let previous = if history.initialized {
        history.integral_previous
    } else {
        initial_condition
    };
    let older = if history.initialized {
        history.integral_older
    } else {
        previous
    };
    let previous_input = if history.initialized {
        history.input_previous
    } else {
        input
    };
    // Keep the ordinary companion rounding, but rescue intermediate overflow
    // and underflow before dividing. A large accepted integral times 1/dt can
    // overflow even when the final integral is unchanged and representable.
    let value = sum_products_div(
        &[
            [input, 1.0],
            [coefficients.previous_value_scale, previous],
            [coefficients.older_value_scale, older],
            [coefficients.previous_derivative_scale, previous_input],
        ],
        coefficients.derivative_scale,
    );
    if !value.is_finite() {
        return Err(GeneratedIdtCandidateError::NonFiniteResult {
            field: "candidate value",
        });
    }
    let jacobian_scale =
        evaluate_generated_idt_derivative(coefficients, history.initialized, [1.0, 0.0]).map_err(
            |error| match error {
                GeneratedIdtCandidateError::NonFiniteResult { .. } => {
                    GeneratedIdtCandidateError::NonFiniteResult {
                        field: "Jacobian scale",
                    }
                }
                error => error,
            },
        )?;

    Ok(GeneratedIdtCandidate {
        value,
        jacobian_scale,
    })
}

fn idt_derivative_terms(
    coefficients: GeneratedDdtCoefficients,
    initialized: bool,
    [input, ic]: [Value; 2],
) -> Result<([[Value; 2]; 4], Value), GeneratedIdtCandidateError> {
    for (field, value) in [
        ("input derivative", input),
        ("initial-condition derivative", ic),
        ("derivative scale", coefficients.derivative_scale),
        ("previous-value scale", coefficients.previous_value_scale),
        ("older-value scale", coefficients.older_value_scale),
        (
            "previous-derivative scale",
            coefficients.previous_derivative_scale,
        ),
    ] {
        if !value.is_finite() {
            return Err(GeneratedIdtCandidateError::NonFiniteInput { field });
        }
    }
    if !coefficients.active {
        return Ok(([[ic, 1.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]], 1.0));
    }
    if coefficients.derivative_scale == 0.0 {
        return Err(GeneratedIdtCandidateError::ZeroDerivativeScale);
    }
    let mut terms = [[input, 1.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
    if !initialized {
        terms[1] = [input, coefficients.previous_derivative_scale];
        terms[2] = [ic, coefficients.previous_value_scale];
        terms[3] = [ic, coefficients.older_value_scale];
    }
    Ok((terms, coefficients.derivative_scale))
}

/// Differentiate an integral using this site's initialization state.
pub fn evaluate_generated_idt_derivative(
    coefficients: GeneratedDdtCoefficients,
    initialized: bool,
    derivatives: [Value; 2],
) -> Result<Value, GeneratedIdtCandidateError> {
    let (terms, divisor) = idt_derivative_terms(coefficients, initialized, derivatives)?;
    let value = sum_products_div(&terms, divisor);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(GeneratedIdtCandidateError::NonFiniteResult {
            field: "derivative",
        })
    }
}

/// The exact current branch published by a circular-integrator candidate.
#[derive(Debug, Clone, Copy)]
pub struct GeneratedIdtModBranch<'a> {
    pub origin: &'a IdtModOrigin,
    pub value: Value,
    pub modulus: Value,
    pub offset: Value,
}

/// Differentiate the input, initial condition, and modulus on one wrap branch.
pub fn evaluate_generated_idtmod_derivative(
    coefficients: GeneratedDdtCoefficients,
    initialized: bool,
    derivatives: [Value; 3],
    branch: GeneratedIdtModBranch<'_>,
) -> Result<Value, GeneratedIdtModCandidateError> {
    let [input, ic, modulus] = derivatives;
    let (terms, divisor) = idt_derivative_terms(coefficients, initialized, [input, ic])
        .map_err(GeneratedIdtModCandidateError::Integral)?;
    branch
        .origin
        .branch_derivative(
            branch.value,
            branch.modulus,
            branch.offset,
            &terms,
            divisor,
            modulus,
        )
        .map_err(GeneratedIdtModCandidateError::Wrapping)
}

/// Evaluate one generated `idt` Newton candidate without publishing it as
/// accepted history.
#[doc(hidden)]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn rspice_eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    candidate_previous: &mut [f64; STATE_COUNT],
    input_current: &mut [f64; STATE_COUNT],
    previous: &[f64; STATE_COUNT],
    older: &[f64; STATE_COUNT],
    input_previous: &[f64; STATE_COUNT],
    initialized: &[bool; STATE_COUNT],
    candidate_valid: &mut [bool; STATE_COUNT],
    coefficients: GeneratedDdtCoefficients,
    slot: usize,
    value: f64,
    ic: f64,
) -> Result<GeneratedIdtCandidate, GeneratedIdtCandidateError> {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    candidate_valid[slot] = false;
    let history = GeneratedIdtAcceptedHistory {
        initialized: initialized[slot],
        integral_previous: previous[slot],
        integral_older: older[slot],
        input_previous: input_previous[slot],
    };
    let candidate = evaluate_generated_idt_candidate(coefficients, value, ic, history)?;
    current[slot] = candidate.value;
    candidate_previous[slot] = if history.initialized {
        history.integral_previous
    } else {
        ic
    };
    input_current[slot] = value;
    candidate_valid[slot] = true;
    Ok(candidate)
}

/// Fold a finite integral into a finite, representable circular interval.
/// History translation is evaluated separately, because `raw - wrapped` may
/// overflow or lose low bits even when the translated history is representable.
pub fn idtmod_wrapped_value(raw: f64, modulus: f64, offset: f64) -> Result<f64, &'static str> {
    IdtModOrigin::ZERO.wrapped_value(raw, modulus, offset)
}

/// Circular-integrator candidate and its previous value on the same wrap branch.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedIdtModCandidate {
    pub value: Value,
    pub previous: Value,
    pub jacobian_scale: Value,
    pub origin: IdtModOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedIdtModCandidateError {
    Integral(GeneratedIdtCandidateError),
    Wrapping(&'static str),
    NonFiniteHistory,
}

impl std::fmt::Display for GeneratedIdtModCandidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integral(source) => write!(f, "idtmod integral: {source}"),
            Self::Wrapping(detail) => write!(f, "idtmod {detail}"),
            Self::NonFiniteHistory => {
                f.write_str("idtmod common-branch previous history must be finite")
            }
        }
    }
}

impl std::error::Error for GeneratedIdtModCandidateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integral(source) => Some(source),
            _ => None,
        }
    }
}

/// Apply the integration rule and wrap both retained integral values together.
/// A rejected candidate never changes accepted history.
#[inline]
pub fn evaluate_generated_idtmod_candidate(
    coefficients: GeneratedDdtCoefficients,
    input: Value,
    initial_condition: Value,
    modulus: Value,
    offset: Value,
    history: GeneratedIdtAcceptedHistory,
    origin: &IdtModOrigin,
) -> Result<GeneratedIdtModCandidate, GeneratedIdtModCandidateError> {
    // Seed the bounded local history before integrating. Adding a small first
    // increment to a large initial integral would otherwise erase that phase.
    let mut local_initial = initial_condition;
    let initial_origin;
    let origin = if coefficients.active && history.initialized {
        origin
    } else if coefficients.active {
        local_initial = IdtModOrigin::ZERO
            .wrapped_value(initial_condition, modulus, offset)
            .map_err(GeneratedIdtModCandidateError::Wrapping)?;
        initial_origin = IdtModOrigin::ZERO
            .rebased(initial_condition, local_initial)
            .map_err(GeneratedIdtModCandidateError::Wrapping)?;
        &initial_origin
    } else {
        &IdtModOrigin::ZERO
    };
    let candidate = evaluate_generated_idt_candidate(coefficients, input, local_initial, history)
        .map_err(GeneratedIdtModCandidateError::Integral)?;
    let value = origin
        .wrapped_value(candidate.value, modulus, offset)
        .map_err(GeneratedIdtModCandidateError::Wrapping)?;
    let origin = origin
        .rebased(candidate.value, value)
        .map_err(GeneratedIdtModCandidateError::Wrapping)?;
    let previous = if history.initialized {
        history.integral_previous
    } else {
        local_initial
    };
    // Subtracting the rounded translation can erase the entire local history
    // when the initial integral is large. Cancel first and round only once.
    let previous = if previous == candidate.value {
        value
    } else if value == candidate.value {
        previous
    } else {
        sum_products([(previous, 1.0), (candidate.value, -1.0), (value, 1.0)].into_iter())
            .map_err(|_| GeneratedIdtModCandidateError::NonFiniteHistory)?
    };
    Ok(GeneratedIdtModCandidate {
        value,
        previous,
        jacobian_scale: candidate.jacobian_scale,
        origin,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_derivatives_follow_initialization_and_companion_weights() {
        let trap = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 8.0,
            previous_value_scale: 8.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 1.0,
        };
        for (coefficients, initialized, derivatives, expected) in [
            (GeneratedDdtCoefficients::inactive(), false, [7.0, 3.0], 3.0),
            (trap, false, [1.0, 0.0], 0.25),
            (trap, false, [0.0, 1.0], 1.0),
            (trap, true, [1.0, 0.0], 0.125),
            (trap, true, [0.0, 1.0], 0.0),
        ] {
            assert_eq!(
                evaluate_generated_idt_derivative(coefficients, initialized, derivatives).unwrap(),
                expected
            );
        }
        let history = GeneratedIdtAcceptedHistory {
            initialized: false,
            integral_previous: 0.0,
            integral_older: 0.0,
            input_previous: 0.0,
        };
        assert_eq!(
            evaluate_generated_idt_candidate(trap, 1.5, 5.0, history)
                .unwrap()
                .jacobian_scale,
            0.25
        );
        let candidate = evaluate_generated_idtmod_candidate(
            trap,
            1.5,
            5.0,
            3.0,
            0.25,
            history,
            &IdtModOrigin::ZERO,
        )
        .unwrap();
        for (derivatives, expected) in [
            ([1.0, 0.0, 0.0], 0.25),
            ([0.0, 1.0, 0.0], 1.0),
            ([0.0, 0.0, 1.0], -1.0),
            ([4.0, 2.0, 3.0], 0.0),
        ] {
            assert_eq!(
                evaluate_generated_idtmod_derivative(
                    trap,
                    false,
                    derivatives,
                    GeneratedIdtModBranch {
                        origin: &candidate.origin,
                        value: candidate.value,
                        modulus: 3.0,
                        offset: 0.25,
                    },
                )
                .unwrap(),
                expected
            );
        }
    }

    #[test]
    fn circular_derivatives_preserve_wide_branch_counts_and_cancellation() {
        use crate::arithmetic::ScaledValue;
        let origin = IdtModOrigin::ZERO.rebased(1.0e308, 1.0).unwrap();
        assert_eq!(
            origin
                .branch_derivative_scaled(
                    1.0,
                    1.0,
                    0.25,
                    ScaledValue::new(1.0e308),
                    ScaledValue::new(1.0)
                )
                .unwrap()
                .binary64(),
            1.0
        );
        assert_eq!(
            evaluate_generated_idtmod_derivative(
                GeneratedDdtCoefficients::inactive(),
                false,
                [0.0, 1.0e308, 1.0],
                GeneratedIdtModBranch {
                    origin: &origin,
                    value: 1.0,
                    modulus: 1.0,
                    offset: 0.25
                },
            )
            .unwrap(),
            1.0
        );
        let origin = IdtModOrigin::ZERO
            .rebased(f64::MAX, 0.0)
            .unwrap()
            .rebased(f64::MAX, 0.0)
            .unwrap();
        let branch = GeneratedIdtModBranch {
            origin: &origin,
            value: 0.0,
            modulus: 1.0,
            offset: 0.0,
        };
        let wide = origin
            .branch_derivative_scaled(0.0, 1.0, 0.0, ScaledValue::new(0.0), ScaledValue::new(1.0))
            .unwrap();
        assert!(wide.is_finite());
        assert_eq!(
            wide.multiply(ScaledValue::new(f64::MIN_POSITIVE))
                .binary64(),
            -2.0 * (f64::MAX * f64::MIN_POSITIVE)
        );
        assert_eq!(
            evaluate_generated_idtmod_derivative(
                GeneratedDdtCoefficients::inactive(),
                false,
                [0.0, 0.0, f64::MIN_POSITIVE],
                branch,
            )
            .unwrap(),
            -2.0 * (f64::MAX * f64::MIN_POSITIVE)
        );
        for derivatives in [
            [f64::NAN, 0.0, 0.0],
            [0.0, f64::INFINITY, 0.0],
            [0.0, 0.0, f64::NAN],
        ] {
            assert!(
                evaluate_generated_idtmod_derivative(
                    GeneratedDdtCoefficients::inactive(),
                    false,
                    derivatives,
                    branch,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn circular_integral_seeds_local_phase_before_direct_transient() {
        let candidate = evaluate_generated_idtmod_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 4.0,
                previous_value_scale: 4.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            1.0,
            1.0e300,
            1.0,
            0.0,
            GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
            &IdtModOrigin::ZERO,
        )
        .unwrap();
        assert_eq!(candidate.value, 0.25);
        assert_eq!(candidate.previous, 0.0);
        assert_eq!(
            candidate
                .origin
                .wrapped_value(candidate.value, 3.0, 0.0)
                .unwrap(),
            (1.0e300_f64 % 3.0) + 0.25
        );
    }

    #[test]
    fn circular_integral_preserves_phase_when_offset_subtraction_rounds() {
        assert_eq!(idtmod_wrapped_value(1.0e16, 1.0, -0.5).unwrap(), 0.0);
        let candidate = evaluate_generated_idtmod_candidate(
            GeneratedDdtCoefficients::inactive(),
            0.0,
            1.0e16,
            1.0,
            0.25,
            GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
            &IdtModOrigin::ZERO,
        )
        .unwrap();
        assert_eq!(candidate.value, 1.0);
        assert_eq!(candidate.previous, 1.0);

        let offset = f64::MAX / 4.0;
        let candidate = evaluate_generated_idtmod_candidate(
            GeneratedDdtCoefficients::inactive(),
            0.0,
            -f64::MAX,
            offset,
            offset,
            GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
            &IdtModOrigin::ZERO,
        )
        .unwrap();
        assert_eq!(candidate.value, offset);
        assert_eq!(candidate.previous, offset);
    }

    #[test]
    fn circular_wrapping_matches_exact_dyadic_arithmetic() {
        // All scaled operands fit i128 exactly, including inputs whose ULP is
        // larger than the offset. This oracle does no floating-point reduction.
        for raw in [
            -1.0e16, -17.9375, -1.0625, -0.0625, 0.0, 0.0625, 1.0625, 17.9375, 1.0e16,
        ] {
            for modulus in [0.0625, 0.25, 0.75, 1.0, 1.5, 17.0] {
                for offset in [-19.9375, -1.0625, -0.5, 0.0, 0.25, 1.0625, 19.9375] {
                    let scaled_raw = (raw * 16.0) as i128;
                    let scaled_offset = (offset * 16.0) as i128;
                    let scaled_modulus = (modulus * 16.0) as i128;
                    let expected = (scaled_offset
                        + (scaled_raw - scaled_offset).rem_euclid(scaled_modulus))
                        as f64
                        / 16.0;
                    assert_eq!(
                        idtmod_wrapped_value(raw, modulus, offset).unwrap(),
                        expected,
                        "raw={raw}, modulus={modulus}, offset={offset}"
                    );
                }
            }
        }
    }

    #[test]
    fn integral_preserves_finite_results_through_intermediate_overflow() {
        let candidate = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 4.0,
                previous_value_scale: 4.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            0.0,
            1.0e308,
            GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: 1.0e308,
                integral_older: 1.0e308,
                input_previous: 0.0,
            },
        )
        .unwrap();
        assert_eq!(candidate.value, 1.0e308);

        let candidate = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 0.25,
                previous_value_scale: 0.25,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            0.0,
            0.0,
            GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: f64::from_bits(1),
                integral_older: 0.0,
                input_previous: 0.0,
            },
        )
        .unwrap();
        assert_eq!(
            candidate.value.to_bits(),
            1,
            "a subnormal accepted integral survives scaling"
        );
    }

    #[test]
    fn circular_integral_rebases_multistep_history_across_both_wrap_directions() {
        // Constant input must advance by exactly dt with BE, trapezoidal, or BDF2.
        for (
            derivative_scale,
            previous_value_scale,
            older_value_scale,
            previous_derivative_scale,
        ) in [
            (4.0, 4.0, 0.0, 0.0),
            (8.0, 8.0, 0.0, 1.0),
            (6.0, 8.0, -2.0, 0.0),
        ] {
            let coefficients = GeneratedDdtCoefficients {
                active: true,
                derivative_scale,
                previous_value_scale,
                older_value_scale,
                previous_derivative_scale,
            };
            for direction in [-1.0, 1.0] {
                let mut history = GeneratedIdtAcceptedHistory {
                    initialized: true,
                    integral_previous: 0.875,
                    integral_older: 0.875 - 0.25 * direction,
                    input_previous: direction,
                };
                let mut origin = IdtModOrigin::ZERO;
                for step in 1..=12 {
                    let candidate = evaluate_generated_idtmod_candidate(
                        coefficients,
                        direction,
                        0.875,
                        1.0,
                        0.0,
                        history,
                        &origin,
                    )
                    .unwrap();
                    assert_eq!(
                        candidate,
                        evaluate_generated_idtmod_candidate(
                            coefficients,
                            direction,
                            0.875,
                            1.0,
                            0.0,
                            history,
                            &origin,
                        )
                        .unwrap()
                    );
                    assert_eq!(
                        candidate.value,
                        (0.875_f64 + 0.25 * direction * f64::from(step)).rem_euclid(1.0)
                    );
                    assert_eq!(candidate.jacobian_scale, derivative_scale.recip());
                    origin = candidate.origin;
                    history = GeneratedIdtAcceptedHistory {
                        initialized: true,
                        integral_previous: candidate.value,
                        integral_older: candidate.previous,
                        input_previous: direction,
                    };
                }
            }
        }
    }

    #[test]
    fn circular_integral_validates_operands_and_wraps_initial_conditions() {
        let history = GeneratedIdtAcceptedHistory {
            initialized: false,
            integral_previous: 0.0,
            integral_older: 0.0,
            input_previous: 0.0,
        };
        for (input, ic, modulus, offset) in [
            (1.0, 1.25, 0.0, -0.5),
            (1.0, 1.25, -1.0, -0.5),
            (f64::NAN, 1.25, 1.0, -0.5),
            (1.0, f64::INFINITY, 1.0, -0.5),
            (1.0, 1.25, 1.0, f64::NAN),
        ] {
            assert!(
                evaluate_generated_idtmod_candidate(
                    GeneratedDdtCoefficients::inactive(),
                    input,
                    ic,
                    modulus,
                    offset,
                    history,
                    &IdtModOrigin::ZERO,
                )
                .is_err()
            );
        }
        let candidate = evaluate_generated_idtmod_candidate(
            GeneratedDdtCoefficients::inactive(),
            1.0,
            1.25,
            1.0,
            -0.5,
            history,
            &IdtModOrigin::ZERO,
        )
        .unwrap();
        assert_eq!(candidate.value, 0.25);
        assert_eq!(candidate.previous, 0.25);
        assert_eq!(candidate.jacobian_scale, 0.0);
    }
}
