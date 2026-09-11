//! Transactional circular-integrator history for generated models.

use super::arithmetic::{IdtModOrigin, IdtModOriginCheckpoint};
use super::{
    GeneratedDdtCoefficients, GeneratedIdtAcceptedHistory, GeneratedIdtCandidateError,
    GeneratedIdtModBranch, GeneratedIdtModCandidate, GeneratedIdtModCandidateError, Value,
    evaluate_generated_idtmod_candidate, evaluate_generated_idtmod_derivative,
};

/// Accepted circular-integrator history, including its exact accumulated origin.
/// The numeric values share the origin's local branch; speculative candidates
/// are deliberately excluded.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedIdtModPersistentState {
    pub initialized: bool,
    pub previous: Value,
    pub older: Value,
    pub input_previous: Value,
    pub origin: IdtModOriginCheckpoint,
}

#[derive(Debug, Clone, PartialEq)]
struct Candidate {
    integral: GeneratedIdtModCandidate,
    input: Value,
}

/// One circular-integrator site's accepted and speculative histories.
///
/// Cloning preserves the entire trial state for nonlinear rollback. A
/// checkpoint preserves only accepted history. Keeping the fields private
/// makes acceptance infallible: every candidate and restored checkpoint has
/// already passed the shared integration and exact-origin validation.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedIdtModState {
    accepted: GeneratedIdtAcceptedHistory,
    origin: IdtModOrigin,
    candidate: Option<Candidate>,
}

impl Default for GeneratedIdtModState {
    fn default() -> Self {
        Self {
            accepted: GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
            origin: IdtModOrigin::ZERO,
            candidate: None,
        }
    }
}

impl GeneratedIdtModState {
    /// Discard the previous trial without changing accepted history.
    #[inline]
    pub fn begin_evaluation(&mut self) {
        self.candidate = None;
    }

    /// Evaluate a replacement trial from accepted history. Failure invalidates
    /// the previous candidate, so it cannot accidentally be accepted later.
    #[inline]
    pub fn evaluate(
        &mut self,
        coefficients: GeneratedDdtCoefficients,
        input: Value,
        initial_condition: Value,
        modulus: Value,
        offset: Value,
    ) -> Result<Value, GeneratedIdtModCandidateError> {
        self.begin_evaluation();
        let integral = evaluate_generated_idtmod_candidate(
            coefficients,
            input,
            initial_condition,
            modulus,
            offset,
            self.accepted,
            &self.origin,
        )?;
        let value = integral.value;
        self.candidate = Some(Candidate { integral, input });
        Ok(value)
    }

    /// Observe the current trial, or the accepted value when there is no trial.
    /// An unevaluated site has no value to freeze yet.
    #[inline]
    pub fn current(&self) -> Option<Value> {
        self.candidate
            .as_ref()
            .map(|candidate| candidate.integral.value)
            .or_else(|| {
                self.accepted
                    .initialized
                    .then_some(self.accepted.integral_previous)
            })
    }

    /// Differentiate on the current trial's exact wrap branch. The primal
    /// dependency must have run before this read; no history is changed here.
    #[inline]
    pub fn derivative(
        &self,
        coefficients: GeneratedDdtCoefficients,
        primal: Value,
        modulus: Value,
        offset: Value,
        derivatives: [Value; 3],
    ) -> Result<Value, GeneratedIdtModCandidateError> {
        let candidate = self
            .candidate
            .as_ref()
            .ok_or(GeneratedIdtModCandidateError::Wrapping(
                "derivative requires its current primal candidate",
            ))?;
        evaluate_generated_idtmod_derivative(
            coefficients,
            self.accepted.initialized,
            derivatives,
            GeneratedIdtModBranch {
                origin: &candidate.integral.origin,
                value: primal,
                modulus,
                offset,
            },
        )
    }

    /// Publish a successful candidate, translating both retained values onto
    /// its branch. Skipped or failed sites retain their accepted history.
    #[inline]
    pub fn commit(&mut self) {
        if let Some(candidate) = self.candidate.take() {
            self.accepted = GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: candidate.integral.value,
                integral_older: candidate.integral.previous,
                input_previous: candidate.input,
            };
            self.origin = candidate.integral.origin;
        }
    }

    pub fn checkpoint(&self) -> GeneratedIdtModPersistentState {
        GeneratedIdtModPersistentState {
            initialized: self.accepted.initialized,
            previous: self.accepted.integral_previous,
            older: self.accepted.integral_older,
            input_previous: self.accepted.input_previous,
            origin: self.origin.checkpoint(),
        }
    }

    /// Validate the entire payload before replacing accepted or trial history.
    pub fn restore(
        &mut self,
        checkpoint: &GeneratedIdtModPersistentState,
    ) -> Result<(), GeneratedIdtModCandidateError> {
        for (field, value) in [
            ("accepted previous integral", checkpoint.previous),
            ("accepted older integral", checkpoint.older),
            ("accepted previous input", checkpoint.input_previous),
        ] {
            if !value.is_finite() {
                return Err(GeneratedIdtModCandidateError::Integral(
                    GeneratedIdtCandidateError::NonFiniteInput { field },
                ));
            }
        }
        let origin = IdtModOrigin::from_checkpoint(&checkpoint.origin)
            .map_err(GeneratedIdtModCandidateError::Wrapping)?;
        if !checkpoint.initialized
            && (checkpoint.previous != 0.0
                || checkpoint.older != 0.0
                || checkpoint.input_previous != 0.0
                || origin != IdtModOrigin::ZERO)
        {
            return Err(GeneratedIdtModCandidateError::Wrapping(
                "uninitialized checkpoint history must be zero",
            ));
        }
        *self = Self {
            accepted: GeneratedIdtAcceptedHistory {
                initialized: checkpoint.initialized,
                integral_previous: checkpoint.previous,
                integral_older: checkpoint.older,
                input_previous: checkpoint.input_previous,
            },
            origin,
            candidate: None,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn be() -> GeneratedDdtCoefficients {
        GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 2.0,
            previous_value_scale: 2.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }

    #[test]
    fn circular_state_commits_common_branch_history_across_methods_and_periods() {
        let mut state = GeneratedIdtModState::default();
        let trap = GeneratedDdtCoefficients {
            derivative_scale: 4.0,
            previous_value_scale: 4.0,
            previous_derivative_scale: 1.0,
            ..be()
        };
        let gear = GeneratedDdtCoefficients {
            derivative_scale: 3.0,
            previous_value_scale: 4.0,
            older_value_scale: -1.0,
            ..be()
        };
        // The unwrapped trajectory is 1, 1.5, 2, 2.5, 3 at half-second
        // intervals. Its previous value must move whenever its branch does.
        for (coefficients, modulus, offset, value, previous) in [
            (be(), 1.0, 0.0, 0.0, -0.5),
            (trap, 1.0, 0.0, 0.5, 0.0),
            (gear, 1.0, 0.0, 0.0, -0.5),
            (be(), 3.0, -0.25, 2.5, 2.0),
            (be(), 3.0, -0.25, 0.0, -0.5),
        ] {
            let accepted = state.checkpoint();
            assert_eq!(
                state
                    .evaluate(coefficients, 1.0, 0.5, modulus, offset)
                    .unwrap(),
                value
            );
            assert_eq!(
                state.checkpoint(),
                accepted,
                "evaluation published accepted history"
            );
            state.commit();
            assert_eq!(state.checkpoint().previous, value);
            assert_eq!(state.checkpoint().older, previous);
            let mut restored = GeneratedIdtModState::default();
            restored.restore(&state.checkpoint()).unwrap();
            assert_eq!(restored, state);
        }
    }

    #[test]
    fn circular_state_retries_and_failed_candidates_cannot_publish_history() {
        let mut state = GeneratedIdtModState::default();
        assert_eq!(state.current(), None);
        let empty = state.checkpoint();
        assert_eq!(state.evaluate(be(), 1.0, 0.5, 1.0, 0.0).unwrap(), 0.0);
        let rollback = state.clone();
        assert_eq!(state.evaluate(be(), 2.0, 0.5, 1.0, 0.0).unwrap(), 0.5);
        assert_eq!(state.checkpoint(), empty);
        state.clone_from(&rollback);
        assert_eq!(state.current(), Some(0.0));
        state.commit();
        let accepted = state.checkpoint();
        for (input, modulus) in [(f64::INFINITY, 1.0), (1.0, 0.0)] {
            state.evaluate(be(), 1.0, 0.5, 1.0, 0.0).unwrap();
            assert!(state.evaluate(be(), input, 0.5, modulus, 0.0).is_err());
            assert_eq!(state.current(), Some(accepted.previous));
            state.commit();
            assert_eq!(state.checkpoint(), accepted);
        }
        state.evaluate(be(), 1.0, 0.5, 1.0, 0.0).unwrap();
        state.begin_evaluation();
        state.commit();
        assert_eq!(state.checkpoint(), accepted, "a skipped site advanced");
    }

    #[test]
    fn circular_state_restore_is_atomic_and_rejects_malformed_history() {
        let mut state = GeneratedIdtModState::default();
        state.evaluate(be(), 1.0, 5.0, 3.0, 0.0).unwrap();
        state.commit();
        let accepted = state.checkpoint();
        state.evaluate(be(), 2.0, 5.0, 3.0, 0.0).unwrap();
        let before = state.clone();
        let corruptions: [fn(&mut GeneratedIdtModPersistentState); 7] = [
            |s| s.previous = f64::NAN,
            |s| s.older = f64::INFINITY,
            |s| s.input_previous = f64::NEG_INFINITY,
            |s| s.initialized = false,
            |s| s.origin.words = vec![0],
            |s| s.origin.exponent = i32::MAX,
            |s| s.origin.words = vec![1; IdtModOrigin::MAX_CHECKPOINT_WORDS + 1],
        ];
        for corrupt in corruptions {
            let mut malformed = accepted.clone();
            corrupt(&mut malformed);
            assert!(state.restore(&malformed).is_err());
            assert_eq!(state, before);
        }
        state.restore(&accepted).unwrap();
        assert_eq!(state.current(), Some(accepted.previous));
        state.commit();
        assert_eq!(state.checkpoint(), accepted);
    }

    #[test]
    fn circular_state_resumes_tiny_increments_on_a_wide_exact_origin() {
        let coefficients = GeneratedDdtCoefficients {
            derivative_scale: 16.0,
            previous_value_scale: 16.0,
            ..be()
        };
        for sign in [-1_i64, 1] {
            let initial = f64::from(sign as i32) * 2.0_f64.powi(900);
            let mut state = GeneratedIdtModState::default();
            let mut resumed = GeneratedIdtModState::default();
            let mut ticks = 0_i64;
            for step in 0..128 {
                let input = (step * 17 % 29) - 14;
                ticks += input;
                let period = [3_i64, 7, 11, 22][step as usize % 4];
                let offset = [-5_i64, 0, 3][step as usize % 3];
                // In units of 1/16, the exact total is sign*2^904+ticks.
                // Modular integer arithmetic supplies an independent oracle
                // without ever rounding the large total into binary64.
                let mut initial_remainder = 1_i64;
                for _ in 0..904 {
                    initial_remainder = initial_remainder * 2 % period;
                }
                let expected = ((sign * initial_remainder + ticks - offset).rem_euclid(period)
                    + offset) as f64
                    / 16.0;
                let modulus = period as f64 / 16.0;
                let offset = offset as f64 / 16.0;
                for current in [&mut state, &mut resumed] {
                    assert_eq!(
                        current
                            .evaluate(coefficients, input as f64, initial, modulus, offset)
                            .unwrap(),
                        expected,
                        "sign {sign}, step {step}",
                    );
                    current.commit();
                }
                let accepted = state.checkpoint();
                assert_eq!(resumed.checkpoint(), accepted);
                resumed.restore(&accepted).unwrap();
            }
        }
    }

    #[test]
    fn circular_state_derivatives_use_the_trial_origin_without_mutating_it() {
        let mut state = GeneratedIdtModState::default();
        let coefficients = GeneratedDdtCoefficients {
            derivative_scale: 8.0,
            previous_value_scale: 8.0,
            previous_derivative_scale: 1.0,
            ..be()
        };
        for initialized in [false, true] {
            if initialized {
                state.commit();
            }
            let point = [1.5, 5.0, 3.0];
            let primal = state
                .evaluate(coefficients, point[0], point[1], point[2], 0.25)
                .unwrap();
            let before = state.clone();
            for (axis, expected) in [
                if initialized { 0.125 } else { 0.25 },
                if initialized { 0.0 } else { 1.0 },
                -1.0,
            ]
            .into_iter()
            .enumerate()
            {
                let mut directions = [0.0; 3];
                directions[axis] = 1.0;
                assert_eq!(
                    state
                        .derivative(coefficients, primal, point[2], 0.25, directions)
                        .unwrap(),
                    expected
                );
                let mut samples = [0.0; 2];
                for (sample, delta) in samples.iter_mut().zip([-1.0e-6, 1.0e-6]) {
                    let mut probe = point;
                    probe[axis] += delta;
                    *sample = state
                        .clone()
                        .evaluate(coefficients, probe[0], probe[1], probe[2], 0.25)
                        .unwrap();
                }
                assert!(((samples[1] - samples[0]) / 2.0e-6 - expected).abs() < 1.0e-8);
            }
            assert_eq!(state, before);
        }
        state.begin_evaluation();
        assert!(
            state
                .derivative(coefficients, 0.0, 3.0, 0.25, [1.0, 0.0, 0.0])
                .is_err()
        );
    }
}
