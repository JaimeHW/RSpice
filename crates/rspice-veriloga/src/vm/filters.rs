//! Stateful runtime primitives behind the time-domain analog operators.
//!
//! [`DelayBuffer`] backs `absdelay`, [`TransitionFilter`] and [`SlewFilter`]
//! back `transition` and `slew`, and [`CrossDetector`] backs `cross`, `above`,
//! `timer`, and `last_crossing`.
//!
//! All of them separate the candidate value from committed history, for the
//! reason given in [`super::context`]: Newton revisits a timepoint many times
//! and an operator that advanced its history on every evaluation would make
//! the answer depend on iteration count. Each type therefore computes from the
//! last accepted state and exposes a separate commit.

use crate::timing_contract::SlewRateMagnitudes;

/// A stateful operator's speculative primal result and its exact local
/// coefficient with respect to the current input argument.
///
/// The coefficient follows the same branch decisions as `output`. It does not
/// include derivatives through accepted history, time, delay, or rate
/// arguments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StatefulEvaluation {
    pub output: f64,
    pub input_coefficient: f64,
}

/// Circular buffer for absdelay transport delay implementation.
///
/// Stores a history of (time, value) pairs to enable looking up
/// delayed values via linear interpolation.
#[derive(Debug, Clone)]
pub struct DelayBuffer {
    /// Buffer capacity (max samples)
    pub(crate) capacity: usize,
    /// Circular buffer of (time, value) pairs
    samples: Vec<(f64, f64)>,
    /// Current write position
    write_pos: usize,
    /// Number of samples currently stored
    pub(crate) count: usize,
    /// Candidate sample produced by the current Newton evaluation.
    ///
    /// This is deliberately separate from the circular history. Repeated
    /// evaluations of one timepoint must not consume transport-delay history;
    /// the candidate enters the committed history only when the simulator
    /// accepts the timestep.
    candidate: Option<(f64, f64)>,
}

/// Accepted transport-delay history. Speculative Newton candidates are never
/// part of a checkpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayCheckpoint {
    pub samples: Vec<(f64, f64)>,
}

/// Evaluate a Verilog-A timer and return both its event level and the next
/// absolute event time that the transient stepper must not skip.
///
/// A non-positive period denotes a one-shot timer.  The event is level-true
/// throughout repeated Newton evaluations of the same candidate timepoint,
/// but is excluded once that timepoint becomes the lower endpoint of the next
/// step.  `time_tol` is an event-coalescing tolerance, while a small
/// scale-aware floating-point tolerance protects endpoint comparisons.
pub(crate) fn timer_event_evaluation(
    start_time: f64,
    period: f64,
    time_tol: f64,
    enable: f64,
    current_time: f64,
    timestep: f64,
) -> (f64, Option<f64>) {
    if !start_time.is_finite()
        || !period.is_finite()
        || !time_tol.is_finite()
        || !enable.is_finite()
        || !current_time.is_finite()
        || !timestep.is_finite()
        || enable == 0.0
        || period < 0.0
    {
        return (0.0, None);
    }

    let scale = start_time.abs().max(current_time.abs());
    let numeric_tol = (16.0 * f64::EPSILON * scale).max(f64::MIN_POSITIVE);
    let event_tol = time_tol.max(0.0).max(numeric_tol);
    let horizon = current_time + event_tol;
    let previous_time = current_time - timestep.max(0.0);

    let scheduled = if horizon + numeric_tol < start_time {
        None
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0);
        let candidate = start_time + cycles * period;
        candidate.is_finite().then_some(candidate)
    } else {
        Some(start_time)
    };

    let fired = scheduled.is_some_and(|event_time| {
        event_time <= horizon
            && if timestep > numeric_tol {
                event_time > previous_time + numeric_tol
            } else {
                (current_time - event_time).abs() <= event_tol
            }
    });

    let next_event = if horizon + numeric_tol < start_time {
        Some(start_time)
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0) + 1.0;
        let next = start_time + cycles * period;
        (next.is_finite() && next > current_time + numeric_tol).then_some(next)
    } else {
        None
    };

    (if fired { 1.0 } else { 0.0 }, next_event)
}

impl DelayBuffer {
    /// Create a new delay buffer with specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(2),
            samples: vec![(0.0, 0.0); capacity.max(2)],
            write_pos: 0,
            count: 0,
            candidate: None,
        }
    }

    /// Record a sample at the given time.
    pub fn record(&mut self, time: f64, value: f64) {
        if self.count == self.capacity {
            // Preserve the entire accepted history. A fixed sample count is
            // not a valid delay horizon because timestep control is adaptive.
            // The optional absdelay max-delay argument is applied separately
            // as a time-domain retention bound.
            let new_capacity = self.capacity.saturating_mul(2).max(self.capacity + 1);
            let mut grown = vec![(0.0, 0.0); new_capacity];
            for (index, slot) in grown.iter_mut().enumerate().take(self.count) {
                *slot = self.samples[(self.write_pos + index) % self.capacity];
            }
            self.samples = grown;
            self.capacity = new_capacity;
            self.write_pos = self.count;
        }
        self.samples[self.write_pos] = (time, value);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Evaluate a delayed value without mutating accepted history.
    pub fn eval(&mut self, time: f64, value: f64, delay: f64) -> f64 {
        self.eval_with_input_coefficient(time, value, delay).output
    }

    /// Evaluate a transport-delay candidate and expose its exact local input
    /// coefficient without changing accepted history.
    pub(crate) fn eval_with_input_coefficient(
        &mut self,
        time: f64,
        value: f64,
        delay: f64,
    ) -> StatefulEvaluation {
        let evaluation = self.candidate_evaluation(time, value, delay);
        self.candidate = Some((time, value));
        evaluation
    }

    fn candidate_evaluation(&self, time: f64, value: f64, delay: f64) -> StatefulEvaluation {
        if delay <= 0.0 {
            StatefulEvaluation {
                output: value,
                input_coefficient: 1.0,
            }
        } else {
            let (output, input_coefficient) =
                self.lookup_delayed_affine(time, delay, Some((time, value, 1.0)));
            StatefulEvaluation {
                output,
                input_coefficient,
            }
        }
    }

    /// Start one complete device evaluation pass.
    ///
    /// A speculative sample belongs only to the pass that produced it. If a
    /// later Newton pass skips this operator, the earlier sample must not be
    /// admitted into accepted delay history.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate = None;
    }

    /// Validate the candidate that would be committed, without changing
    /// either accepted or speculative state.
    pub(crate) fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        let Some((time, value)) = self.candidate else {
            return Ok(());
        };
        if !time.is_finite() || !value.is_finite() {
            return Err("delay candidate sample is not finite".into());
        }
        if time != accepted_time {
            return Err(format!(
                "delay candidate time {time} does not equal accepted time {accepted_time}"
            ));
        }
        Ok(())
    }

    /// Commit the latest candidate after the transient step is accepted.
    pub fn commit(&mut self) {
        if let Some((time, value)) = self.candidate.take() {
            // An accepted-time reevaluation replaces the existing endpoint
            // instead of adding a duplicate timestamp.
            if self.count > 0 {
                let newest = if self.count == self.capacity {
                    (self.write_pos + self.capacity - 1) % self.capacity
                } else {
                    self.count - 1
                };
                if self.samples[newest].0 == time {
                    self.samples[newest] = (time, value);
                    return;
                }
            }
            self.record(time, value);
        }
    }

    /// Get the delayed value at (current_time - delay).
    ///
    /// Uses linear interpolation between stored samples.
    /// If delay exceeds buffer history, returns oldest value.
    /// If delay is zero or negative, returns current value (or most recent).
    pub fn get_delayed(&self, current_time: f64, delay: f64) -> f64 {
        self.lookup_delayed(current_time, delay, None)
    }

    fn lookup_delayed(&self, current_time: f64, delay: f64, candidate: Option<(f64, f64)>) -> f64 {
        self.lookup_delayed_affine(
            current_time,
            delay,
            candidate.map(|(time, value)| (time, value, 0.0)),
        )
        .0
    }

    fn lookup_delayed_affine(
        &self,
        current_time: f64,
        delay: f64,
        candidate: Option<(f64, f64, f64)>,
    ) -> (f64, f64) {
        if self.count == 0 && candidate.is_none() {
            return (0.0, 0.0);
        }

        let target_time = current_time - delay;

        // Find the two samples bracketing target_time.
        // Samples are stored in circular order, so we need to search.
        let mut before: Option<(f64, f64, f64)> = None;
        let mut after: Option<(f64, f64, f64)> = None;

        for i in 0..self.count {
            let idx = if self.count == self.capacity {
                (self.write_pos + i) % self.capacity
            } else {
                i
            };
            let (t, v) = self.samples[idx];

            if t <= target_time {
                before = Some((t, v, 0.0));
            }
            if t >= target_time && after.is_none() {
                after = Some((t, v, 0.0));
            }
        }

        if let Some((t, v, coefficient)) = candidate {
            if t <= target_time {
                before = Some((t, v, coefficient));
            }
            if t >= target_time
                && after
                    .map(|(after_time, _, _)| t < after_time)
                    .unwrap_or(true)
            {
                after = Some((t, v, coefficient));
            }
        }

        match (before, after) {
            (Some((t0, v0, c0)), Some((t1, v1, c1))) if t0 != t1 => {
                // Linear interpolation.
                let alpha = (target_time - t0) / (t1 - t0);
                (v0 + alpha * (v1 - v0), c0 + alpha * (c1 - c0))
            }
            (Some((_, value, coefficient)), _) => (value, coefficient),
            (_, Some((_, value, coefficient))) => (value, coefficient),
            _ => (0.0, 0.0),
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.count = 0;
        self.write_pos = 0;
        self.candidate = None;
    }

    pub(crate) fn checkpoint(&self) -> DelayCheckpoint {
        let mut samples = Vec::with_capacity(self.count);
        for index in 0..self.count {
            let physical = if self.count == self.capacity {
                (self.write_pos + index) % self.capacity
            } else {
                index
            };
            samples.push(self.samples[physical]);
        }
        DelayCheckpoint { samples }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate.is_some() {
            return Err("delay has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &DelayCheckpoint) -> Result<(), String> {
        let mut previous = None;
        for (index, &(time, value)) in checkpoint.samples.iter().enumerate() {
            if !time.is_finite() || !value.is_finite() {
                return Err(format!("delay sample {index} is not finite"));
            }
            if previous.is_some_and(|accepted| time <= accepted) {
                return Err(format!(
                    "delay sample times are not strictly increasing at sample {index}"
                ));
            }
            previous = Some(time);
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &DelayCheckpoint) {
        let capacity = checkpoint.samples.len().max(2);
        self.capacity = capacity;
        self.samples.clear();
        self.samples.resize(capacity, (0.0, 0.0));
        self.samples[..checkpoint.samples.len()].copy_from_slice(&checkpoint.samples);
        self.count = checkpoint.samples.len();
        self.write_pos = self.count % capacity;
        self.candidate = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter, timer_event_evaluation};
    use crate::timing_contract::SlewRateMagnitudes;

    const TEST_SLEW_RATES: SlewRateMagnitudes = SlewRateMagnitudes {
        rise: 0.5,
        fall: 0.5,
    };

    #[test]
    fn one_shot_timer_fires_once_and_requests_its_timepoint() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            (0.0, Some(1.0))
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (1.0, None)
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (1.0, None),
            "repeated Newton evaluations must see the same event"
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.1, 0.1),
            (0.0, None),
            "an accepted event must not fire again at the next step"
        );
    }

    #[test]
    fn periodic_timer_reports_each_next_breakpoint() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 1.0, 1.0, 1.0),
            (1.0, Some(1.5))
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 1.0, 1.5, 0.5),
            (1.0, Some(2.0))
        );
    }

    #[test]
    fn disabled_and_invalid_timers_do_not_fire_or_schedule() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 0.0, 0.0, 0.0),
            (0.0, None)
        );
        assert_eq!(
            timer_event_evaluation(1.0, -0.5, 0.0, 1.0, 0.0, 0.0),
            (0.0, None)
        );
    }

    #[test]
    fn delay_history_grows_without_dropping_accepted_samples() {
        let mut buffer = DelayBuffer::new(2);
        for time in 0..4096 {
            buffer.record(time as f64, time as f64);
        }

        assert_eq!(buffer.get_delayed(4095.0, 4095.0), 0.0);
        assert_eq!(buffer.get_delayed(4095.0, 2047.5), 2047.5);
    }

    #[test]
    fn delay_candidate_reports_exact_input_interpolation_coefficient() {
        let mut empty = DelayBuffer::new(2);
        let evaluation = empty.eval_with_input_coefficient(2.0, 7.0, 0.5);
        assert_eq!(evaluation.output, 7.0);
        assert_eq!(evaluation.input_coefficient, 1.0);

        let mut interpolating = DelayBuffer::new(2);
        interpolating.record(0.0, 0.0);
        let accepted_before = interpolating.checkpoint();
        let evaluation = interpolating.eval_with_input_coefficient(2.0, 2.0, 0.5);
        assert_eq!(evaluation.output, 1.5);
        assert_eq!(evaluation.input_coefficient, 0.75);
        assert_eq!(interpolating.checkpoint(), accepted_before);

        let epsilon = 1.0e-6;
        let upper = interpolating.eval(2.0, 2.0 + epsilon, 0.5);
        let lower = interpolating.eval(2.0, 2.0 - epsilon, 0.5);
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - evaluation.input_coefficient).abs() <= 1.0e-9);
    }

    #[test]
    fn delay_candidate_coefficient_covers_history_only_and_zero_delay() {
        let mut buffer = DelayBuffer::new(2);
        buffer.record(0.0, 0.0);
        buffer.record(1.0, 1.0);

        let history_only = buffer.eval_with_input_coefficient(2.0, 3.0, 1.5);
        assert_eq!(history_only.output, 0.5);
        assert_eq!(history_only.input_coefficient, 0.0);

        let direct = buffer.eval_with_input_coefficient(2.0, 3.0, 0.0);
        assert_eq!(direct.output, 3.0);
        assert_eq!(direct.input_coefficient, 1.0);
    }

    #[test]
    fn transition_candidate_reports_branch_exact_input_coefficient() {
        let mut transition = TransitionFilter::default();
        let accepted_before = transition.checkpoint();

        let before_start = transition.eval_with_input_coefficient(1.0, 1.0, 1.0, 2.0, 2.0);
        assert_eq!(before_start.output, 0.0);
        assert_eq!(before_start.input_coefficient, 0.0);
        assert_eq!(transition.checkpoint(), accepted_before);

        let ramp = transition.eval_with_input_coefficient(1.0, 1.0, -1.0, 2.0, 2.0);
        assert_eq!(ramp.output, 0.5);
        assert_eq!(ramp.input_coefficient, 0.5);

        let epsilon = 1.0e-6;
        let upper = transition.eval(1.0 + epsilon, 1.0, -1.0, 2.0, 2.0);
        let lower = transition.eval(1.0 - epsilon, 1.0, -1.0, 2.0, 2.0);
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - ramp.input_coefficient).abs() <= 1.0e-9);

        let instantaneous = transition.eval_with_input_coefficient(1.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(instantaneous.output, 1.0);
        assert_eq!(instantaneous.input_coefficient, 1.0);
    }

    #[test]
    fn unchanged_transition_target_has_no_current_input_coefficient() {
        let mut transition = TransitionFilter::default();
        transition.eval(1.0, 1.0, 0.0, 0.0, 0.0);
        transition.commit();

        let evaluation = transition.eval_with_input_coefficient(1.0, 2.0, 0.0, 1.0, 1.0);
        assert_eq!(evaluation.output, 1.0);
        assert_eq!(evaluation.input_coefficient, 0.0);
    }

    #[test]
    fn slew_candidate_reports_branch_exact_input_coefficient() {
        let mut slew = SlewFilter::default();
        slew.eval_operating_point(0.0, 0.0);
        slew.promote_operating_point_candidate();
        let accepted_before = slew.checkpoint();

        let saturated = slew.eval_with_input_coefficient(1.0, 1.0, TEST_SLEW_RATES);
        assert_eq!(saturated.output, 0.5);
        assert_eq!(saturated.input_coefficient, 0.0);
        assert_eq!(slew.checkpoint(), accepted_before);

        let unsaturated = slew.eval_with_input_coefficient(0.25, 1.0, TEST_SLEW_RATES);
        assert_eq!(unsaturated.output, 0.25);
        assert_eq!(unsaturated.input_coefficient, 1.0);

        let boundary = slew.eval_with_input_coefficient(0.5, 1.0, TEST_SLEW_RATES);
        assert_eq!(boundary.output, 0.5);
        assert_eq!(boundary.input_coefficient, 1.0);

        let no_time_advance = slew.eval_with_input_coefficient(1.0, 0.0, TEST_SLEW_RATES);
        assert_eq!(no_time_advance.output, 0.0);
        assert_eq!(no_time_advance.input_coefficient, 0.0);

        let epsilon = 1.0e-6;
        let upper = slew.eval(0.25 + epsilon, 1.0, TEST_SLEW_RATES);
        let lower = slew.eval(0.25 - epsilon, 1.0, TEST_SLEW_RATES);
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - unsaturated.input_coefficient).abs() <= 1.0e-9);
    }

    #[test]
    fn slew_direct_transient_start_seeds_from_first_input() {
        let mut slew = SlewFilter::default();
        let evaluation = slew.eval_with_input_coefficient(7.0, 0.0, TEST_SLEW_RATES);
        assert_eq!(evaluation.output, 7.0);
        assert_eq!(evaluation.input_coefficient, 1.0);
        slew.commit();

        let falling = slew.eval_with_input_coefficient(0.0, 1.0, TEST_SLEW_RATES);
        assert_eq!(falling.output, 6.5);
        assert_eq!(falling.input_coefficient, 0.0);
    }

    #[test]
    fn slew_operating_point_promotion_seeds_first_transient_candidate() {
        let mut slew = SlewFilter::default();
        assert_eq!(slew.eval_operating_point(5.0, 0.0), 5.0);
        slew.promote_operating_point_candidate();

        let candidate = slew.eval_with_input_coefficient(10.0, 0.25, TEST_SLEW_RATES);
        assert_eq!(candidate.output, 5.125);
        assert_eq!(candidate.input_coefficient, 0.0);
    }

    #[test]
    fn slew_derivative_is_branch_exact_and_read_only() {
        let mut slew = SlewFilter::default();
        slew.eval_operating_point(0.0, 0.0);
        slew.promote_operating_point_candidate();
        let accepted = slew.checkpoint();

        assert_eq!(
            slew.eval_derivative(10.0, 3.0, 4.0, 5.0, 0.5, TEST_SLEW_RATES),
            2.0,
            "rising saturation uses dt times the positive-rate derivative"
        );
        assert_eq!(
            slew.eval_derivative(-10.0, 3.0, 4.0, 5.0, 0.5, TEST_SLEW_RATES),
            2.5,
            "falling saturation uses dt times the authored negative-rate derivative"
        );
        assert_eq!(
            slew.eval_derivative(0.1, 3.0, 4.0, 5.0, 0.5, TEST_SLEW_RATES),
            3.0,
            "unsaturated branch uses the input derivative"
        );
        assert_eq!(slew.checkpoint(), accepted);
    }

    #[test]
    fn stateful_filter_candidates_do_not_mutate_committed_state() {
        let mut transition = TransitionFilter::default();
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), 0.0);
        assert_eq!(transition.eval(2.0, 1.0, 0.0, 2.0, 2.0), 0.0);
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), 0.0);

        let mut slew = SlewFilter::default();
        slew.eval_operating_point(0.0, 0.0);
        slew.promote_operating_point_candidate();
        assert_eq!(slew.eval(1.0, 1.0, TEST_SLEW_RATES), 0.5);
        assert_eq!(slew.eval(0.0, 1.0, TEST_SLEW_RATES), 0.0);
        assert_eq!(slew.eval(1.0, 1.0, TEST_SLEW_RATES), 0.5);

        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval(-1.0, 0.0, 1).unwrap(), 0.0);
        cross.commit();
        assert_eq!(cross.eval(1.0, 1.0, 1).unwrap(), 1.0);
        assert_eq!(cross.eval(1.0, 1.0, 1).unwrap(), 1.0);
    }

    #[test]
    fn cross_tolerances_control_root_landing_without_coalescing_events() {
        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval_event(-1.0, 0.0, 0, 1.0, 0.0, true), Ok(0.0));
        cross.commit();
        assert_eq!(cross.eval_event(1.0, 1.0, 0, 1.0, 0.0, true), Ok(1.0));
        assert_eq!(cross.candidate_refinement_time(), Some(0.5000000000000001));
        cross.commit();

        assert_eq!(cross.eval_event(-1.0, 1.2, 0, 1.0, 0.0, true), Ok(1.0));
        cross.commit();
        assert_eq!(cross.eval_event(1.0, 1.4, 0, 1.0, 0.0, true), Ok(1.0));
        cross.commit();
    }

    #[test]
    fn cross_refinement_requires_a_real_enabled_matching_sign_crossing() {
        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval_event(-1.0, 0.0, 0, 0.01, 0.1, true), Ok(0.0));
        cross.commit();

        assert_eq!(
            cross.eval_event(-0.05, 1.0, 0, 0.01, 0.1, true),
            Ok(0.0),
            "entering the expression tolerance on the negative side is not a crossing"
        );
        assert_eq!(cross.candidate_refinement_time(), None);

        assert_eq!(cross.eval_event(1.0, 1.0, -1, 0.01, 0.1, true), Ok(0.0));
        assert_eq!(cross.candidate_refinement_time(), None);
        assert_eq!(cross.eval_event(1.0, 1.0, 1, 0.01, 0.1, false), Ok(0.0));
        assert_eq!(cross.candidate_refinement_time(), None);
        assert_eq!(cross.eval_event(1.0, 1.0, 2, 0.01, 0.1, true), Ok(0.0));
        assert_eq!(cross.candidate_refinement_time(), None);
    }

    #[test]
    fn cross_requires_both_root_tolerances_and_validates_them() {
        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval_event(-1.0, 0.0, 1, 0.6, 0.5, true), Ok(0.0));
        cross.commit();

        assert_eq!(cross.eval_event(0.25, 1.0, 1, 0.6, 0.5, true), Ok(1.0));
        assert_eq!(
            cross.candidate_refinement_time(),
            None,
            "both requested tolerances are satisfied"
        );
        assert_eq!(cross.eval_event(0.25, 1.0, 1, 0.1, 0.5, true), Ok(1.0));
        assert!(
            cross.candidate_refinement_time().is_some(),
            "expression convergence alone cannot waive time_tol"
        );
        assert_eq!(cross.eval_event(1.0, 1.0, 1, 0.6, 0.5, true), Ok(1.0));
        assert!(
            cross.candidate_refinement_time().is_some(),
            "time convergence alone cannot waive expr_tol"
        );

        assert!(cross.eval_event(1.0, 1.0, 1, -1.0, 0.0, true).is_err());
        assert!(cross.eval_event(1.0, 1.0, 1, 0.0, f64::NAN, true).is_err());
    }

    #[test]
    fn above_initial_positive_event_ignores_root_tolerance_deadband() {
        let mut above = CrossDetector::default();
        assert_eq!(above.eval_above(1.0e-9, 0.0, 0.0, 1.0, true), Ok(1.0));
        assert_eq!(above.candidate_refinement_time(), None);
    }

    #[test]
    fn cross_interpolation_is_overflow_safe_and_unrepresentable_roots_fail_closed() {
        let mut extreme = CrossDetector::default();
        extreme.eval(-f64::MAX, 0.0, 1).unwrap();
        extreme.commit();
        assert_eq!(extreme.eval(f64::MAX, 1.0, 1), Ok(1.0));
        assert_eq!(
            extreme.candidate_refinement_time(),
            Some(0.5000000000000001)
        );

        let mut adjacent = CrossDetector::default();
        adjacent.eval(-1.0, 1.0, 1).unwrap();
        adjacent.commit();
        let next_time = f64::from_bits(1.0_f64.to_bits() + 1);
        let error = adjacent
            .eval(1.0, next_time, 1)
            .expect_err("an interval without an interior float must not accept an inaccurate root");
        assert!(error.contains("representable interior time"), "{error}");
    }
}

impl Default for DelayBuffer {
    fn default() -> Self {
        Self::new(1024) // Default to 1024 samples
    }
}

#[derive(Debug, Clone, Copy)]
struct TransitionState {
    /// Current output value
    pub output: f64,
    /// Target value we're transitioning to
    pub target: f64,
    /// Time when transition started
    pub start_time: f64,
    /// Time when transition ends
    pub end_time: f64,
    /// Initial value at start of transition
    pub start_value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionCheckpoint {
    pub output: f64,
    pub target: f64,
    pub start_time: f64,
    pub end_time: f64,
    pub start_value: f64,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            output: 0.0,
            target: 0.0,
            start_time: 0.0,
            end_time: 0.0,
            start_value: 0.0,
        }
    }
}

/// Transition filter state for piecewise-linear signal smoothing.
#[derive(Debug, Clone, Default)]
pub struct TransitionFilter {
    committed: TransitionState,
    candidate: TransitionState,
    candidate_time: f64,
    candidate_valid: bool,
}

impl TransitionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the filter with new input and return filtered output.
    pub fn eval(
        &mut self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> f64 {
        self.eval_with_input_coefficient(input, time, delay, rise_time, fall_time)
            .output
    }

    /// Evaluate a transition candidate and expose its exact local input
    /// coefficient without changing accepted history.
    pub(crate) fn eval_with_input_coefficient(
        &mut self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> StatefulEvaluation {
        let (state, evaluation) =
            self.candidate_evaluation(input, time, delay, rise_time, fall_time);
        self.candidate = state;
        self.candidate_time = time;
        self.candidate_valid = true;
        evaluation
    }

    fn candidate_evaluation(
        &self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> (TransitionState, StatefulEvaluation) {
        let mut state = self.committed;
        // Check if input target changed.
        let target_changed = (input - state.target).abs() > 1e-15;
        if target_changed {
            // New transition started.
            let trans_time = if input > state.output {
                rise_time
            } else {
                fall_time
            };
            state.target = input;
            state.start_value = state.output;
            state.start_time = time + delay;
            state.end_time = state.start_time + trans_time;
        }

        // Compute output based on current transition state.
        let (output, input_coefficient) = if time < state.start_time {
            // Before transition starts.
            (state.output, 0.0)
        } else if time >= state.end_time || (state.end_time - state.start_time).abs() < 1e-15 {
            // Transition complete or instantaneous.
            state.output = state.target;
            (state.target, if target_changed { 1.0 } else { 0.0 })
        } else {
            // In transition - linear interpolation.
            let t = (time - state.start_time) / (state.end_time - state.start_time);
            state.output = state.start_value + t * (state.target - state.start_value);
            (state.output, if target_changed { t } else { 0.0 })
        };
        (
            state,
            StatefulEvaluation {
                output,
                input_coefficient,
            },
        )
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }

    /// Discard a candidate produced by an earlier complete device pass.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate_valid = false;
    }

    /// Validate the candidate that would be committed, without mutation.
    pub(crate) fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        if !self.candidate_valid {
            return Ok(());
        }
        Self::validate_checkpoint(&TransitionCheckpoint {
            output: self.candidate.output,
            target: self.candidate.target,
            start_time: self.candidate.start_time,
            end_time: self.candidate.end_time,
            start_value: self.candidate.start_value,
        })?;
        if self.candidate_time != accepted_time {
            return Err(format!(
                "transition candidate time {} does not equal accepted time {accepted_time}",
                self.candidate_time
            ));
        }
        Ok(())
    }

    /// Clear accepted and speculative transition history for a new analysis.
    pub(crate) fn reset_analysis(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn checkpoint(&self) -> TransitionCheckpoint {
        TransitionCheckpoint {
            output: self.committed.output,
            target: self.committed.target,
            start_time: self.committed.start_time,
            end_time: self.committed.end_time,
            start_value: self.committed.start_value,
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate_valid {
            return Err("transition has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &TransitionCheckpoint) -> Result<(), String> {
        let values = [
            checkpoint.output,
            checkpoint.target,
            checkpoint.start_time,
            checkpoint.end_time,
            checkpoint.start_value,
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err("transition accepted state is not finite".into());
        }
        if checkpoint.end_time < checkpoint.start_time {
            return Err("transition end time precedes its start time".into());
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &TransitionCheckpoint) {
        self.committed = TransitionState {
            output: checkpoint.output,
            target: checkpoint.target,
            start_time: checkpoint.start_time,
            end_time: checkpoint.end_time,
            start_value: checkpoint.start_value,
        };
        self.candidate = self.committed;
        self.candidate_time = 0.0;
        self.candidate_valid = false;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SlewState {
    output: f64,
    prev_time: f64,
    initialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlewCheckpoint {
    pub output: f64,
    pub prev_time: f64,
    pub initialized: bool,
}

/// Slew rate filter for limiting rate of change.
#[derive(Debug, Clone, Default)]
pub struct SlewFilter {
    committed: SlewState,
    candidate: SlewState,
    candidate_valid: bool,
}

impl SlewFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the filter with new input, limiting slew rate.
    pub(crate) fn eval(&mut self, input: f64, time: f64, rates: SlewRateMagnitudes) -> f64 {
        self.eval_with_input_coefficient(input, time, rates).output
    }

    /// Evaluate a slew candidate and expose its exact local input coefficient
    /// without changing accepted history.
    pub(crate) fn eval_with_input_coefficient(
        &mut self,
        input: f64,
        time: f64,
        rates: SlewRateMagnitudes,
    ) -> StatefulEvaluation {
        let (state, evaluation) = self.candidate_evaluation(input, time, rates);
        self.candidate = state;
        self.candidate_valid = true;
        evaluation
    }

    fn candidate_evaluation(
        &self,
        input: f64,
        time: f64,
        rates: SlewRateMagnitudes,
    ) -> (SlewState, StatefulEvaluation) {
        let mut state = self.committed;
        if !state.initialized {
            state = SlewState {
                output: input,
                prev_time: time,
                initialized: true,
            };
            return (
                state,
                StatefulEvaluation {
                    output: input,
                    input_coefficient: 1.0,
                },
            );
        }
        let dt = time - state.prev_time;
        if dt <= 0.0 {
            return (
                state,
                StatefulEvaluation {
                    output: state.output,
                    input_coefficient: 0.0,
                },
            );
        }

        let delta = input - state.output;
        let rate = delta / dt;

        // Apply slew rate limiting.
        let (limited_rate, input_coefficient) = if rate > rates.rise {
            (rates.rise, 0.0)
        } else if rate < -rates.fall {
            (-rates.fall, 0.0)
        } else {
            (rate, 1.0)
        };

        state.output += limited_rate * dt;
        state.prev_time = time;
        state.initialized = true;
        (
            state,
            StatefulEvaluation {
                output: state.output,
                input_coefficient,
            },
        )
    }

    /// Evaluate the exact local derivative of the current candidate without
    /// changing accepted or speculative state. Rate derivatives retain their
    /// source-level signs.
    pub(crate) fn eval_derivative(
        &self,
        input: f64,
        input_derivative: f64,
        positive_rate_derivative: f64,
        negative_rate_derivative: f64,
        time: f64,
        rates: SlewRateMagnitudes,
    ) -> f64 {
        if !self.committed.initialized {
            return input_derivative;
        }
        let dt = time - self.committed.prev_time;
        if dt <= 0.0 {
            return 0.0;
        }
        let rate = (input - self.committed.output) / dt;
        if rate > rates.rise {
            positive_rate_derivative * dt
        } else if rate < -rates.fall {
            negative_rate_derivative * dt
        } else {
            input_derivative
        }
    }

    /// Record the final operating-point Newton candidate without changing
    /// accepted history. It is promoted only when integration becomes active.
    pub(crate) fn eval_operating_point(&mut self, input: f64, time: f64) -> f64 {
        self.candidate = SlewState {
            output: input,
            prev_time: time,
            initialized: true,
        };
        self.candidate_valid = true;
        input
    }

    pub(crate) fn promote_operating_point_candidate(&mut self) {
        if self.candidate_valid
            && self.candidate.output.is_finite()
            && self.candidate.prev_time.is_finite()
        {
            self.committed = self.candidate;
        }
        self.candidate = self.committed;
        self.candidate_valid = false;
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }

    /// Discard a candidate produced by an earlier complete device pass.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate_valid = false;
    }

    /// Validate the candidate that would be committed, without mutation.
    pub(crate) fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        if !self.candidate_valid {
            return Ok(());
        }
        Self::validate_checkpoint(&SlewCheckpoint {
            output: self.candidate.output,
            prev_time: self.candidate.prev_time,
            initialized: self.candidate.initialized,
        })?;
        if self.candidate.prev_time != accepted_time {
            return Err(format!(
                "slew candidate time {} does not equal accepted time {accepted_time}",
                self.candidate.prev_time
            ));
        }
        Ok(())
    }

    /// Clear accepted and speculative slew history for a new analysis.
    pub(crate) fn reset_analysis(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn checkpoint(&self) -> SlewCheckpoint {
        SlewCheckpoint {
            output: self.committed.output,
            prev_time: self.committed.prev_time,
            initialized: self.committed.initialized,
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate_valid {
            return Err("slew has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &SlewCheckpoint) -> Result<(), String> {
        if !checkpoint.output.is_finite() || !checkpoint.prev_time.is_finite() {
            return Err("slew accepted state is not finite".into());
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &SlewCheckpoint) {
        self.committed = SlewState {
            output: checkpoint.output,
            prev_time: checkpoint.prev_time,
            initialized: checkpoint.initialized,
        };
        self.candidate = self.committed;
        self.candidate_valid = false;
    }
}

#[derive(Debug, Clone, Copy)]
struct CrossState {
    value: f64,
    time: f64,
    side: i8,
    last_event_time: f64,
    last_crossing_time: f64,
    initialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossCheckpoint {
    pub value: f64,
    pub time: f64,
    pub side: i8,
    pub last_event_time: f64,
    pub last_crossing_time: f64,
    pub initialized: bool,
}

impl Default for CrossState {
    fn default() -> Self {
        Self {
            value: 0.0,
            time: 0.0,
            side: 0,
            last_event_time: f64::NEG_INFINITY,
            last_crossing_time: -1.0,
            initialized: false,
        }
    }
}

/// Cross detector for threshold crossing events.
#[derive(Debug, Clone, Default)]
pub struct CrossDetector {
    committed: CrossState,
    candidate: CrossState,
    candidate_valid: bool,
    /// Absolute time at which the transient solver should retry the current
    /// interval. This is speculative solver guidance, never accepted history.
    candidate_refinement_time: Option<f64>,
}

impl CrossDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check for zero crossing, returns 1.0 if crossed, 0.0 otherwise.
    /// direction: +1 = rising only, -1 = falling only, 0 = both.
    pub fn eval(&mut self, value: f64, time: f64, direction: i32) -> Result<f64, String> {
        self.eval_event(value, time, direction, 0.0, 0.0, true)
    }

    /// Evaluate `cross(expr, direction, time_tol, expr_tol, enable)`.
    pub fn eval_event(
        &mut self,
        value: f64,
        time: f64,
        direction: i32,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
    ) -> Result<f64, String> {
        self.eval_impl(value, time, direction, time_tol, expr_tol, enabled, false)
    }

    /// Evaluate `above(expr, time_tol, expr_tol, enable)`.
    ///
    /// Unlike `cross`, `above` also fires during the initial operating-point
    /// evaluation when the expression starts on the positive side.
    pub fn eval_above(
        &mut self,
        value: f64,
        time: f64,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
    ) -> Result<f64, String> {
        self.eval_impl(value, time, 1, time_tol, expr_tol, enabled, true)
    }

    /// Return the linearly interpolated time of the most recent crossing.
    /// Returns -1.0 until a crossing matching `direction` has occurred.
    pub fn eval_last_crossing(
        &mut self,
        value: f64,
        time: f64,
        direction: i32,
    ) -> Result<f64, String> {
        Self::validate_value_and_time(value, time)?;
        if !self.committed.initialized {
            self.candidate = CrossState {
                value,
                time,
                side: Self::side(value, 0.0),
                initialized: true,
                ..CrossState::default()
            };
            self.candidate_valid = true;
            self.candidate_refinement_time = None;
            return Ok(-1.0);
        }

        if time <= self.committed.time {
            self.candidate = self.committed;
            self.candidate_valid = true;
            self.candidate_refinement_time = None;
            return Ok(self.committed.last_crossing_time);
        }

        let rising = self.committed.value < 0.0 && value >= 0.0;
        let falling = self.committed.value > 0.0 && value <= 0.0;
        let crossing_direction = if rising {
            1
        } else if falling {
            -1
        } else {
            0
        };
        let mut candidate = CrossState {
            value,
            time,
            side: Self::side(value, 0.0),
            ..self.committed
        };
        if crossing_direction != 0 && (direction == 0 || direction == crossing_direction) {
            candidate.last_crossing_time = self.estimate_crossing_time(value, time);
        }
        self.candidate = candidate;
        self.candidate_valid = true;
        self.candidate_refinement_time = None;
        Ok(candidate.last_crossing_time)
    }

    fn eval_impl(
        &mut self,
        value: f64,
        time: f64,
        direction: i32,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
        initial_above: bool,
    ) -> Result<f64, String> {
        Self::validate_value_and_time(value, time)?;
        Self::validate_tolerance("time_tol", time_tol)?;
        Self::validate_tolerance("expr_tol", expr_tol)?;
        self.candidate_refinement_time = None;

        if !self.committed.initialized {
            let side = Self::side(value, 0.0);
            let mut candidate = CrossState {
                value,
                time,
                side,
                initialized: true,
                ..CrossState::default()
            };
            let fired = initial_above && enabled && value > 0.0;
            if fired {
                candidate.last_event_time = time;
                candidate.last_crossing_time = time;
            }
            self.candidate = candidate;
            self.candidate_valid = true;
            return Ok(if fired { 1.0 } else { 0.0 });
        }

        if time <= self.committed.time {
            self.candidate = self.committed;
            self.candidate_valid = true;
            return Ok(0.0);
        }

        // A tolerance is a landing criterion, not permission to manufacture a
        // crossing between two values on the same side of zero.
        let rising = self.committed.side < 0 && value >= 0.0;
        let falling = self.committed.side > 0 && value <= 0.0;
        let crossing_direction = if rising {
            1
        } else if falling {
            -1
        } else {
            0
        };
        let crossing_time = if crossing_direction != 0 {
            self.estimate_crossing_time(value, time)
        } else {
            -1.0
        };
        let direction_matches = direction == 0 || direction == crossing_direction;
        let fired = enabled && crossing_direction != 0 && direction_matches;

        if fired {
            let effective_time_tol =
                Self::effective_tolerance(time_tol, self.committed.time.abs().max(time.abs()));
            let effective_expr_tol =
                Self::effective_tolerance(expr_tol, self.committed.value.abs().max(value.abs()));
            let time_error = (time - crossing_time).max(0.0);
            let converged = time_error <= effective_time_tol && value.abs() <= effective_expr_tol;
            if !converged {
                self.candidate_refinement_time = Some(
                    Self::strictly_interior_time(self.committed.time, crossing_time, time)
                        .ok_or_else(|| {
                            format!(
                                "crossing in ({}, {time}) cannot be refined to a representable interior time",
                                self.committed.time
                            )
                        })?,
                );
            }
        }

        let stable_side = Self::side(value, 0.0);
        let side = if stable_side != 0 {
            stable_side
        } else if crossing_direction != 0 {
            crossing_direction as i8
        } else {
            self.committed.side
        };
        let mut candidate = CrossState {
            value,
            time,
            side,
            ..self.committed
        };
        if fired {
            candidate.last_event_time = crossing_time;
            candidate.last_crossing_time = crossing_time;
        }
        self.candidate = candidate;
        self.candidate_valid = true;
        Ok(if fired { 1.0 } else { 0.0 })
    }

    fn validate_value_and_time(value: f64, time: f64) -> Result<(), String> {
        if !value.is_finite() {
            return Err(format!("cross expression must be finite, got {value}"));
        }
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "cross evaluation time must be finite and non-negative, got {time}"
            ));
        }
        Ok(())
    }

    fn validate_tolerance(name: &str, tolerance: f64) -> Result<(), String> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(format!(
                "cross {name} must be finite and non-negative, got {tolerance}"
            ));
        }
        Ok(())
    }

    fn effective_tolerance(requested: f64, scale: f64) -> f64 {
        if requested > 0.0 {
            requested
        } else {
            (64.0 * f64::EPSILON * scale).max(f64::MIN_POSITIVE)
        }
    }

    fn strictly_interior_time(start: f64, estimate: f64, end: f64) -> Option<f64> {
        if !estimate.is_finite() || end <= start {
            return None;
        }
        let target = Self::next_time_after(estimate.max(start));
        (target > start && target < end).then_some(target)
    }

    fn next_time_after(time: f64) -> f64 {
        debug_assert!(time.is_finite() && time >= 0.0);
        if time == 0.0 {
            f64::from_bits(1)
        } else {
            f64::from_bits(time.to_bits() + 1)
        }
    }

    fn side(value: f64, tolerance: f64) -> i8 {
        if value > tolerance {
            1
        } else if value < -tolerance {
            -1
        } else {
            0
        }
    }

    fn estimate_crossing_time(&self, value: f64, time: f64) -> f64 {
        let accepted_magnitude = self.committed.value.abs();
        let candidate_magnitude = value.abs();
        let scale = accepted_magnitude.max(candidate_magnitude);
        if scale == 0.0 || !scale.is_finite() {
            return time;
        }
        let accepted_scaled = accepted_magnitude / scale;
        let candidate_scaled = candidate_magnitude / scale;
        let fraction = (accepted_scaled / (accepted_scaled + candidate_scaled)).clamp(0.0, 1.0);
        self.committed.time + fraction * (time - self.committed.time)
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
        self.candidate_refinement_time = None;
    }

    /// Absolute interior root estimate produced by the latest evaluation.
    pub(crate) fn candidate_refinement_time(&self) -> Option<f64> {
        self.candidate_refinement_time
    }

    /// Discard a candidate produced by an earlier complete device pass.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate_valid = false;
        self.candidate_refinement_time = None;
    }

    /// Validate the candidate that would be committed, without mutation.
    pub(crate) fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        if !self.candidate_valid {
            return Ok(());
        }
        Self::validate_checkpoint(&CrossCheckpoint {
            value: self.candidate.value,
            time: self.candidate.time,
            side: self.candidate.side,
            last_event_time: self.candidate.last_event_time,
            last_crossing_time: self.candidate.last_crossing_time,
            initialized: self.candidate.initialized,
        })?;
        if self.candidate.initialized && self.candidate.time != accepted_time {
            return Err(format!(
                "cross candidate time {} does not equal accepted time {accepted_time}",
                self.candidate.time
            ));
        }
        Ok(())
    }

    /// Clear crossing, event, and interpolation history for a new analysis.
    pub(crate) fn reset_analysis(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn checkpoint(&self) -> CrossCheckpoint {
        CrossCheckpoint {
            value: self.committed.value,
            time: self.committed.time,
            side: self.committed.side,
            last_event_time: self.committed.last_event_time,
            last_crossing_time: self.committed.last_crossing_time,
            initialized: self.committed.initialized,
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate_valid {
            return Err("cross detector has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &CrossCheckpoint) -> Result<(), String> {
        if !(-1..=1).contains(&checkpoint.side) {
            return Err("cross side is outside -1..=1".into());
        }
        if !checkpoint.value.is_finite()
            || !checkpoint.time.is_finite()
            || !(checkpoint.last_event_time.is_finite()
                || checkpoint.last_event_time == f64::NEG_INFINITY)
            || !checkpoint.last_crossing_time.is_finite()
        {
            return Err("cross accepted state is malformed".into());
        }
        if checkpoint.last_crossing_time < -1.0 {
            return Err("cross last-crossing sentinel is invalid".into());
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &CrossCheckpoint) {
        self.committed = CrossState {
            value: checkpoint.value,
            time: checkpoint.time,
            side: checkpoint.side,
            last_event_time: checkpoint.last_event_time,
            last_crossing_time: checkpoint.last_crossing_time,
            initialized: checkpoint.initialized,
        };
        self.candidate = self.committed;
        self.candidate_valid = false;
        self.candidate_refinement_time = None;
    }
}
