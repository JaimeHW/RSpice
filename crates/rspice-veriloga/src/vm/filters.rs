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
use std::collections::VecDeque;
use std::sync::Arc;

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

/// Maximum number of accepted samples retained by one `absdelay` site.
///
/// The time horizon is normally the tighter bound, but an adaptive solver can
/// accept arbitrarily many points inside a finite interval. Refusing an
/// over-budget candidate is preferable to either unbounded allocation or
/// silently discarding interpolation data.
const MAX_DELAY_HISTORY_SAMPLES: usize = 1_048_576;

/// The definition frozen by the first accepted transient evaluation.
///
/// With the optional `maxdelay` omitted, Verilog-A freezes `td` itself. With
/// `maxdelay` present, `td` remains a runtime input but is clamped to the
/// accepted maximum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelayConfiguration {
    Fixed { delay: f64 },
    Bounded { max_delay: f64 },
}

impl DelayConfiguration {
    fn retention(self) -> f64 {
        match self {
            Self::Fixed { delay } => delay,
            Self::Bounded { max_delay } => max_delay,
        }
    }
}

/// A speculative transport-delay sample and the definition it would freeze
/// if this is the first accepted evaluation.
#[derive(Debug, Clone, Copy, PartialEq)]
struct DelayCandidate {
    time: f64,
    value: f64,
    configuration: DelayConfiguration,
}

/// A transport-delay candidate's exact local affine coefficients.
///
/// Accepted history and the frozen definition are constants. The two
/// coefficients are with respect to the current `expr` and `td` arguments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DelayEvaluation {
    pub output: f64,
    pub input_coefficient: f64,
    pub delay_coefficient: f64,
}

/// Accepted/candidate state for the Verilog-A `absdelay` operator.
///
/// Accepted samples are strictly increasing and begin at time zero. The
/// candidate is separate so repeated Newton evaluations and rejected steps do
/// not mutate trajectory history. Accepted history is pruned to the configured
/// delay horizon while retaining the predecessor needed for exact linear
/// interpolation.
#[derive(Debug, Clone)]
pub struct DelayBuffer {
    samples: VecDeque<(f64, f64)>,
    configuration: Option<DelayConfiguration>,
    candidate: Option<DelayCandidate>,
}

/// Accepted transport-delay state. Speculative Newton candidates are never
/// part of a checkpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayCheckpoint {
    pub configuration: Option<DelayConfiguration>,
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
    /// Create a delay buffer with an allocation hint, not a retention limit.
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(capacity.min(MAX_DELAY_HISTORY_SAMPLES)),
            configuration: None,
            candidate: None,
        }
    }

    /// Evaluate a delayed value without mutating accepted history.
    pub fn eval(
        &mut self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<f64, String> {
        Ok(self
            .eval_with_coefficients(time, value, delay, max_delay)?
            .output)
    }

    /// Validate and stage the analysis-wide delay definition at an
    /// equilibrium operating point while preserving the required unity DC
    /// value.  The first accepted point freezes omitted `maxdelay` to its
    /// authored `td`, or freezes the explicit maximum when present. Later
    /// frequency points only validate/read that definition and never append
    /// duplicate time-zero history.
    pub(crate) fn eval_operating_point(
        &mut self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<f64, String> {
        self.validate_runtime(time, value, delay, max_delay)?;
        let (configuration, _, _) = self.resolve_configuration(delay, max_delay)?;
        if self.configuration.is_none() {
            if time != 0.0 {
                return Err(format!(
                    "absdelay's first equilibrium definition must be established at time zero, got {time}"
                ));
            }
            self.candidate = Some(DelayCandidate {
                time,
                value,
                configuration,
            });
        } else {
            self.candidate = None;
        }
        Ok(value)
    }

    /// Read the effective transport delay for AC/noise without mutating
    /// accepted history. The result honors the definition frozen by the
    /// accepted operating point.
    pub(crate) fn small_signal_delay(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<f64, String> {
        self.validate_runtime(time, value, delay, max_delay)?;
        self.resolve_configuration(delay, max_delay)
            .map(|(_, effective, _)| effective)
    }

    /// Evaluate a candidate and expose exact current-input and delay
    /// coefficients without changing accepted history.
    pub(crate) fn eval_with_coefficients(
        &mut self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayEvaluation, String> {
        self.validate_runtime(time, value, delay, max_delay)?;
        if self
            .samples
            .back()
            .is_some_and(|(accepted_time, _)| time < *accepted_time)
        {
            return Err(format!(
                "absdelay evaluation time {time} precedes the latest accepted sample at {}",
                self.samples.back().map_or(0.0, |sample| sample.0)
            ));
        }

        let (configuration, effective_delay, delay_scale) =
            self.resolve_configuration(delay, max_delay)?;
        let evaluation = self.candidate_evaluation(time, value, effective_delay, delay_scale)?;
        self.candidate = Some(DelayCandidate {
            time,
            value,
            configuration,
        });
        Ok(evaluation)
    }

    fn validate_runtime(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "absdelay evaluation time must be finite and non-negative, got {time}"
            ));
        }
        if !value.is_finite() {
            return Err(format!("absdelay input must be finite, got {value}"));
        }
        match (self.configuration, max_delay) {
            (None, maximum) => {
                Self::validate_delay(delay)?;
                if maximum.is_some_and(|value| !value.is_finite() || value <= 0.0) {
                    return Err(format!(
                        "absdelay maxdelay must be finite and strictly positive, got {}",
                        maximum.unwrap_or(f64::NAN)
                    ));
                }
            }
            (Some(DelayConfiguration::Fixed { .. }), None) => {
                // `td` is part of the accepted fixed operator definition. Its
                // current expression value is no longer a runtime operand.
            }
            (Some(DelayConfiguration::Bounded { .. }), Some(_)) => {
                // `td` remains dynamic, while the accepted maximum replaces
                // the current authored maxdelay value.
                Self::validate_delay(delay)?;
            }
            (Some(DelayConfiguration::Fixed { .. }), Some(_)) => {
                return Err(
                    "absdelay call changed from omitted maxdelay to present maxdelay after initialization"
                        .into(),
                );
            }
            (Some(DelayConfiguration::Bounded { .. }), None) => {
                return Err(
                    "absdelay call changed from present maxdelay to omitted maxdelay after initialization"
                        .into(),
                );
            }
        }
        Ok(())
    }

    fn validate_delay(delay: f64) -> Result<(), String> {
        if !delay.is_finite() || delay <= 0.0 {
            return Err(format!(
                "absdelay td must be finite and strictly positive, got {delay}"
            ));
        }
        Ok(())
    }

    fn resolve_configuration(
        &self,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(DelayConfiguration, f64, f64), String> {
        match (self.configuration, max_delay) {
            (None, None) => Ok((DelayConfiguration::Fixed { delay }, delay, 0.0)),
            (None, Some(maximum)) => {
                let effective = delay.min(maximum);
                let derivative = if delay < maximum { 1.0 } else { 0.0 };
                Ok((
                    DelayConfiguration::Bounded { max_delay: maximum },
                    effective,
                    derivative,
                ))
            }
            (Some(configuration @ DelayConfiguration::Fixed { delay: fixed }), None) => {
                Ok((configuration, fixed, 0.0))
            }
            (
                Some(configuration @ DelayConfiguration::Bounded { max_delay: maximum }),
                Some(_),
            ) => {
                let effective = delay.min(maximum);
                let derivative = if delay < maximum { 1.0 } else { 0.0 };
                Ok((configuration, effective, derivative))
            }
            (Some(DelayConfiguration::Fixed { .. }), Some(_)) => Err(
                "absdelay call changed from omitted maxdelay to present maxdelay after initialization"
                    .into(),
            ),
            (Some(DelayConfiguration::Bounded { .. }), None) => Err(
                "absdelay call changed from present maxdelay to omitted maxdelay after initialization"
                    .into(),
            ),
        }
    }

    fn candidate_evaluation(
        &self,
        time: f64,
        value: f64,
        effective_delay: f64,
        delay_scale: f64,
    ) -> Result<DelayEvaluation, String> {
        if self.samples.is_empty() {
            if time != 0.0 {
                return Err(format!(
                    "absdelay requires an accepted time-zero anchor before evaluation at {time}"
                ));
            }
            return Ok(DelayEvaluation {
                output: value,
                input_coefficient: 1.0,
                delay_coefficient: 0.0,
            });
        }

        let unclamped_target = time - effective_delay;
        if unclamped_target > 0.0 && unclamped_target == time {
            return Err(format!(
                "absdelay td {effective_delay} is not representable relative to time {time}"
            ));
        }
        let target = unclamped_target.max(0.0);
        let target_delay_derivative = if unclamped_target > 0.0 {
            -delay_scale
        } else {
            // The time-zero clamp and its exact kink are deterministic: the
            // anchored branch has zero local delay derivative.
            0.0
        };
        self.interpolate(target, (time, value), target_delay_derivative)
    }

    fn interpolate(
        &self,
        target: f64,
        candidate: (f64, f64),
        target_delay_derivative: f64,
    ) -> Result<DelayEvaluation, String> {
        let mut left = None;
        let mut right = None;
        for &(sample_time, sample_value) in &self.samples {
            if sample_time <= target {
                left = Some((sample_time, sample_value, 0.0));
            } else {
                right = Some((sample_time, sample_value, 0.0));
                break;
            }
        }
        if candidate.0 <= target {
            left = Some((candidate.0, candidate.1, 1.0));
        } else if right.is_none() {
            right = Some((candidate.0, candidate.1, 1.0));
        }

        let evaluation = match (left, right) {
            (
                Some((left_time, left_value, left_input)),
                Some((right_time, right_value, right_input)),
            ) => {
                let interval = right_time - left_time;
                if !(interval.is_finite() && interval > 0.0) {
                    return Err("absdelay interpolation interval is not representable".into());
                }
                let alpha = ((target - left_time) / interval).clamp(0.0, 1.0);
                let one_minus_alpha = 1.0 - alpha;
                let output = alpha.mul_add(right_value, one_minus_alpha * left_value);
                let input_coefficient = alpha.mul_add(right_input, one_minus_alpha * left_input);
                let slope = (right_value - left_value) / interval;
                DelayEvaluation {
                    output,
                    input_coefficient,
                    delay_coefficient: slope * target_delay_derivative,
                }
            }
            (Some((_, output, input_coefficient)), None)
            | (None, Some((_, output, input_coefficient))) => DelayEvaluation {
                output,
                input_coefficient,
                delay_coefficient: 0.0,
            },
            (None, None) => {
                return Err("absdelay has no time-zero anchor or candidate sample".into());
            }
        };
        if !evaluation.output.is_finite()
            || !evaluation.input_coefficient.is_finite()
            || !evaluation.delay_coefficient.is_finite()
        {
            return Err("absdelay interpolation produced an unrepresentable result".into());
        }
        Ok(evaluation)
    }

    /// Start one complete device evaluation pass, rolling back any earlier
    /// candidate from a rejected or incomplete Newton pass.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate = None;
    }

    /// Validate the candidate that would be committed without mutation.
    pub(crate) fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        let Some(candidate) = self.candidate else {
            return Ok(());
        };
        if candidate.time != accepted_time {
            return Err(format!(
                "delay candidate time {} does not equal accepted time {accepted_time}",
                candidate.time
            ));
        }
        if let Some((latest_time, _)) = self.samples.back() {
            if candidate.time <= *latest_time {
                return Err(format!(
                    "delay candidate time {} is not strictly after latest accepted time {latest_time}",
                    candidate.time
                ));
            }
        } else if candidate.time != 0.0 {
            return Err(format!(
                "delay's first accepted sample must be the time-zero anchor, got {}",
                candidate.time
            ));
        }
        if self
            .configuration
            .is_some_and(|accepted| accepted != candidate.configuration)
        {
            return Err("delay candidate configuration differs from accepted configuration".into());
        }

        let cutoff = (candidate.time - candidate.configuration.retention()).max(0.0);
        let retained = self.retained_sample_count(cutoff);
        if retained >= MAX_DELAY_HISTORY_SAMPLES {
            return Err(format!(
                "delay history requires more than the supported {MAX_DELAY_HISTORY_SAMPLES} accepted samples inside its configured horizon"
            ));
        }
        Ok(())
    }

    /// Commit a candidate for direct users that do not use the VM's two-phase
    /// accepted-state transaction.
    pub fn commit(&mut self) -> Result<(), String> {
        let Some(candidate) = self.candidate else {
            return Ok(());
        };
        self.validate_commit(candidate.time)?;
        self.apply_validated_commit();
        Ok(())
    }

    /// Apply a candidate after the VM has validated every device atomically.
    pub(crate) fn apply_validated_commit(&mut self) {
        let Some(candidate) = self.candidate.take() else {
            return;
        };
        if self.configuration.is_none() {
            self.configuration = Some(candidate.configuration);
        }
        self.samples.push_back((candidate.time, candidate.value));
        self.prune(candidate.time, candidate.configuration.retention());
    }

    fn retained_sample_count(&self, cutoff: f64) -> usize {
        let removable = self
            .samples
            .iter()
            .zip(self.samples.iter().skip(1))
            .take_while(|(_, next)| next.0 < cutoff)
            .count();
        self.samples.len() - removable
    }

    fn prune(&mut self, accepted_time: f64, retention: f64) {
        let cutoff = (accepted_time - retention).max(0.0);
        while self.samples.len() >= 2 && self.samples[1].0 < cutoff {
            self.samples.pop_front();
        }
    }

    /// Clear accepted definition, history, and speculative state.
    pub fn clear(&mut self) {
        self.samples.clear();
        self.configuration = None;
        self.candidate = None;
    }

    #[cfg(test)]
    pub(crate) fn allocation_capacity(&self) -> usize {
        self.samples.capacity()
    }

    #[cfg(test)]
    pub(crate) fn accepted_sample_count(&self) -> usize {
        self.samples.len()
    }

    pub(crate) fn checkpoint(&self) -> DelayCheckpoint {
        DelayCheckpoint {
            configuration: self.configuration,
            samples: self.samples.iter().copied().collect(),
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate.is_some() {
            return Err("delay has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &DelayCheckpoint) -> Result<(), String> {
        if checkpoint.samples.len() > MAX_DELAY_HISTORY_SAMPLES {
            return Err(format!(
                "delay checkpoint exceeds the supported {MAX_DELAY_HISTORY_SAMPLES} samples"
            ));
        }
        match checkpoint.configuration {
            Some(configuration) => {
                let retention = configuration.retention();
                if !retention.is_finite() || retention <= 0.0 {
                    return Err("delay checkpoint configuration is not finite and positive".into());
                }
                if checkpoint.samples.is_empty() {
                    return Err("configured delay checkpoint has no accepted samples".into());
                }
            }
            None if !checkpoint.samples.is_empty() => {
                return Err("unconfigured delay checkpoint contains accepted samples".into());
            }
            None => return Ok(()),
        }
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
        if checkpoint
            .samples
            .first()
            .is_some_and(|sample| sample.0 < 0.0)
        {
            return Err("delay checkpoint contains a negative sample time".into());
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &DelayCheckpoint) {
        self.samples.clear();
        self.samples.extend(checkpoint.samples.iter().copied());
        self.configuration = checkpoint.configuration;
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
    fn delay_history_is_time_bounded_and_retains_the_interpolation_predecessor() {
        let mut buffer = DelayBuffer::new(2);
        for time in 0..4096 {
            buffer
                .eval(time as f64, time as f64, 1.0, Some(2.0))
                .unwrap();
            buffer.commit().unwrap();
        }

        assert_eq!(
            buffer.checkpoint().samples,
            vec![
                (4092.0, 4092.0),
                (4093.0, 4093.0),
                (4094.0, 4094.0),
                (4095.0, 4095.0)
            ]
        );
        let interpolation = buffer
            .eval_with_coefficients(4096.0, 4096.0, 2.0, Some(2.0))
            .unwrap();
        assert_eq!(interpolation.output, 4094.0);
    }

    #[test]
    fn delay_candidate_reports_exact_input_and_delay_coefficients() {
        let mut buffer = DelayBuffer::new(2);
        buffer.eval(0.0, 0.0, 0.5, Some(10.0)).unwrap();
        buffer.commit().unwrap();
        buffer.eval(1.0, 2.0, 0.5, Some(10.0)).unwrap();
        buffer.commit().unwrap();
        let accepted_before = buffer.checkpoint();

        let evaluation = buffer
            .eval_with_coefficients(2.0, 6.0, 0.5, Some(10.0))
            .unwrap();
        assert_eq!(evaluation.output, 4.0);
        assert_eq!(evaluation.input_coefficient, 0.5);
        assert_eq!(evaluation.delay_coefficient, -4.0);
        assert_eq!(buffer.checkpoint(), accepted_before);

        let epsilon = 1.0e-6;
        let upper = buffer.eval(2.0, 6.0 + epsilon, 0.5, Some(10.0)).unwrap();
        let lower = buffer.eval(2.0, 6.0 - epsilon, 0.5, Some(10.0)).unwrap();
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - evaluation.input_coefficient).abs() <= 1.0e-9);

        let upper = buffer.eval(2.0, 6.0, 0.5 + epsilon, Some(10.0)).unwrap();
        let lower = buffer.eval(2.0, 6.0, 0.5 - epsilon, Some(10.0)).unwrap();
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - evaluation.delay_coefficient).abs() <= 1.0e-9);
    }

    #[test]
    fn delay_kinks_choose_the_anchored_and_right_hand_branches_deterministically() {
        let mut buffer = DelayBuffer::new(2);
        buffer.eval(0.0, 0.0, 1.0, Some(5.0)).unwrap();
        buffer.commit().unwrap();
        buffer.eval(1.0, 2.0, 1.0, Some(5.0)).unwrap();
        buffer.commit().unwrap();

        let anchor = buffer
            .eval_with_coefficients(1.0, 99.0, 1.0, Some(5.0))
            .unwrap();
        assert_eq!(anchor.output, 0.0);
        assert_eq!(anchor.input_coefficient, 0.0);
        assert_eq!(anchor.delay_coefficient, 0.0);

        let exact_sample = buffer
            .eval_with_coefficients(2.0, 6.0, 1.0, Some(5.0))
            .unwrap();
        assert_eq!(exact_sample.output, 2.0);
        assert_eq!(exact_sample.input_coefficient, 0.0);
        assert_eq!(
            exact_sample.delay_coefficient, -4.0,
            "an exact interior sample uses the causal right-hand segment"
        );

        let mut maximum = DelayBuffer::new(2);
        maximum.eval(0.0, 0.0, 1.0, Some(1.0)).unwrap();
        maximum.commit().unwrap();
        maximum.eval(1.0, 2.0, 1.0, Some(1.0)).unwrap();
        maximum.commit().unwrap();
        let maximum_kink = maximum
            .eval_with_coefficients(2.0, 6.0, 1.0, Some(1.0))
            .unwrap();
        assert_eq!(maximum_kink.output, 2.0);
        assert_eq!(
            maximum_kink.delay_coefficient, 0.0,
            "td == maxdelay uses the saturated branch"
        );
    }

    #[test]
    fn omitted_delay_and_present_maximum_freeze_only_on_acceptance() {
        let mut fixed = DelayBuffer::new(2);
        fixed.eval(0.0, 0.0, 1.0, None).unwrap();
        assert_eq!(fixed.checkpoint().configuration, None);
        fixed.begin_evaluation();
        fixed.eval(0.0, 0.0, 2.0, None).unwrap();
        fixed.commit().unwrap();
        assert_eq!(
            fixed.checkpoint().configuration,
            Some(super::DelayConfiguration::Fixed { delay: 2.0 })
        );
        fixed.eval(1.0, 1.0, 0.1, None).unwrap();
        fixed.commit().unwrap();
        let frozen = fixed.eval_with_coefficients(3.0, 3.0, 0.1, None).unwrap();
        assert_eq!(frozen.output, 1.0);
        assert_eq!(frozen.delay_coefficient, 0.0);

        let mut bounded = DelayBuffer::new(2);
        bounded.eval(0.0, 0.0, 1.0, Some(3.0)).unwrap();
        assert_eq!(bounded.checkpoint().configuration, None);
        bounded.commit().unwrap();
        bounded.eval(1.0, 1.0, 1.0, Some(99.0)).unwrap();
        bounded.commit().unwrap();
        let clamped = bounded
            .eval_with_coefficients(4.0, 4.0, 4.0, Some(99.0))
            .unwrap();
        assert_eq!(clamped.output, 1.0);
        assert_eq!(clamped.delay_coefficient, 0.0);
        assert_eq!(
            bounded.checkpoint().configuration,
            Some(super::DelayConfiguration::Bounded { max_delay: 3.0 })
        );
    }

    #[test]
    fn operating_point_freezes_omitted_delay_for_small_signal_phase() {
        let mut buffer = DelayBuffer::new(2);
        assert_eq!(buffer.eval_operating_point(0.0, 2.0, 0.25, None), Ok(2.0));
        buffer.commit().expect("accept operating-point definition");

        assert_eq!(
            buffer.small_signal_delay(0.0, 2.0, 0.75, None),
            Ok(0.25),
            "AC/noise must use the accepted fixed delay, not a later operand value"
        );
        assert_eq!(
            buffer.checkpoint().configuration,
            Some(super::DelayConfiguration::Fixed { delay: 0.25 })
        );

        assert_eq!(
            buffer.small_signal_delay(0.0, 2.0, f64::NAN, None),
            Ok(0.25),
            "a frozen fixed delay must not revalidate the discarded td operand"
        );
        assert_eq!(buffer.eval_operating_point(0.0, 2.0, -1.0, None), Ok(2.0));

        let mut bounded = DelayBuffer::new(2);
        bounded
            .eval_operating_point(0.0, 2.0, 0.25, Some(1.0))
            .unwrap();
        bounded.commit().expect("accept bounded definition");
        assert_eq!(
            bounded.small_signal_delay(0.0, 2.0, 0.5, Some(f64::NAN)),
            Ok(0.5),
            "the accepted maximum must replace later maxdelay operand values"
        );
        assert!(
            bounded
                .small_signal_delay(0.0, 2.0, f64::NAN, Some(1.0))
                .is_err(),
            "bounded td remains a live runtime operand"
        );
    }

    #[test]
    fn delay_rejects_invalid_and_nonmonotonic_state_without_mutating_history() {
        let mut buffer = DelayBuffer::new(2);
        for invalid in [
            buffer.eval(0.0, 1.0, 0.0, None),
            buffer.eval(0.0, 1.0, f64::NAN, None),
            buffer.eval(0.0, 1.0, 1.0, Some(f64::INFINITY)),
        ] {
            assert!(invalid.is_err());
        }
        buffer.eval(0.0, 0.0, 1.0, None).unwrap();
        buffer.commit().unwrap();
        buffer.eval(1.0, 1.0, 1.0, None).unwrap();
        buffer.commit().unwrap();
        let accepted = buffer.checkpoint();

        assert!(buffer.eval(0.5, 2.0, 1.0, None).is_err());
        assert_eq!(buffer.checkpoint(), accepted);
        buffer.eval(1.0, 2.0, 1.0, None).unwrap();
        assert!(buffer.commit().is_err());
        assert_eq!(buffer.checkpoint(), accepted);
        buffer.begin_evaluation();
        assert!(buffer.validate_checkpoint_ready().is_ok());
    }

    #[test]
    fn transition_candidate_reports_branch_exact_input_coefficient() {
        let mut transition = TransitionFilter::default();
        transition
            .eval_operating_point(0.0, 0.0, 0.0, 2.0, 2.0)
            .unwrap();
        transition.promote_operating_point_candidate();
        let accepted_before = transition.checkpoint();

        let before_start = transition
            .eval_with_input_coefficient(1.0, 1.0, 1.0, 2.0, 2.0)
            .unwrap();
        assert_eq!(before_start.output, 0.0);
        assert_eq!(before_start.input_coefficient, 0.0);
        assert_eq!(transition.checkpoint(), accepted_before);

        let ramp_start = transition
            .eval_with_input_coefficient(1.0, 1.0, 0.0, 2.0, 2.0)
            .unwrap();
        assert_eq!(ramp_start.output, 0.0);
        assert_eq!(ramp_start.input_coefficient, 0.0);

        let instantaneous = transition
            .eval_with_input_coefficient(1.0, 1.0, 0.0, 0.0, 0.0)
            .unwrap();
        assert_eq!(instantaneous.output, 1.0);
        assert_eq!(instantaneous.input_coefficient, 1.0);
    }

    #[test]
    fn unchanged_transition_target_has_no_current_input_coefficient() {
        let mut transition = TransitionFilter::default();
        transition
            .eval_operating_point(1.0, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        transition.promote_operating_point_candidate();
        transition.eval(1.0, 1.0, 0.0, 0.0, 0.0).unwrap();
        transition.commit();

        let evaluation = transition
            .eval_with_input_coefficient(1.0, 2.0, 0.0, 1.0, 1.0)
            .unwrap();
        assert_eq!(evaluation.output, 1.0);
        assert_eq!(evaluation.input_coefficient, 0.0);
    }

    #[test]
    fn transition_derivative_is_branch_exact_and_read_only() {
        let transition = seeded_transition(0.0);
        let accepted = transition.checkpoint();

        assert_eq!(
            transition.eval_derivative(1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 2),
            Ok(0.0),
            "delayed output depends only on accepted history"
        );
        assert_eq!(
            transition.eval_derivative(1.0, 3.0, 1.0, 0.0, 2.0, 2.0, 2),
            Ok(0.0),
            "a finite active ramp depends only on accepted history"
        );
        assert_eq!(
            transition.eval_derivative(1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2),
            Ok(3.0),
            "an instantaneous transition tracks the current input"
        );
        let direct = seeded_transition(1.0);
        for _ in 0..16 {
            assert_eq!(
                direct.eval_derivative(1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2),
                Ok(3.0),
                "an unchanged direct transition must retain a nonsingular Jacobian"
            );
        }
        for analysis_type in [0, 1, 3, 4] {
            assert_eq!(
                transition.eval_derivative(1.0, 3.0, 1.0, 1.0, 2.0, 2.0, analysis_type),
                Ok(3.0),
                "analysis {analysis_type} must use unity small-signal action"
            );
        }
        assert_eq!(transition.checkpoint(), accepted);

        let mut cancelling = seeded_transition(0.0);
        cancelling.eval(2.0, 0.0, 0.0, 2.0, 2.0).unwrap();
        cancelling.commit();
        cancelling.eval(2.0, 1.0, 0.0, 2.0, 2.0).unwrap();
        cancelling.commit();
        let active = cancelling.checkpoint();
        assert_eq!(active.output, 1.0);
        assert!(active.active.is_some());
        assert_eq!(
            cancelling.eval_derivative(1.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2),
            Ok(3.0),
            "an instantaneous history cancellation still maps output directly to input"
        );
        assert_eq!(cancelling.checkpoint(), active);
    }

    fn seeded_transition(value: f64) -> TransitionFilter {
        let mut filter = TransitionFilter::default();
        filter
            .eval_operating_point(value, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        filter.promote_operating_point_candidate();
        filter
    }

    #[test]
    fn transition_keeps_ordered_pending_transport_events_and_cancels_only_later_work() {
        let mut filter = seeded_transition(0.0);
        filter.eval(1.0, 0.1, 2.0, 1.0, 1.0).unwrap();
        filter.commit();
        filter.eval(0.0, 0.2, 2.0, 1.0, 1.0).unwrap();
        filter.commit();
        filter.eval(2.0, 0.3, 1.85, 1.0, 1.0).unwrap();
        filter.commit();

        let checkpoint = filter.checkpoint();
        let starts: Vec<_> = checkpoint
            .pending
            .iter()
            .map(|event| event.start_time)
            .collect();
        assert_eq!(starts, vec![2.1, 2.15]);
        assert_eq!(filter.next_event_time(0.3), Some(2.1));

        assert_eq!(filter.eval(2.0, 2.1, 1.85, 1.0, 1.0), Ok(0.0));
        filter.commit();
        assert_eq!(filter.next_event_time(2.1), Some(2.15));
        let output = filter.eval(2.0, 2.15, 1.85, 1.0, 1.0).unwrap();
        assert!((output - 0.05).abs() < 1.0e-12, "output={output}");
        filter.commit();
        assert_eq!(filter.next_event_time(2.15), Some(3.125));
    }

    #[test]
    fn transition_interruption_uses_lrm_origin_and_destination_slope_rules() {
        let mut continuing = seeded_transition(0.0);
        continuing.eval(1.0, 1.0, 0.0, 2.0, 2.0).unwrap();
        continuing.commit();
        assert_eq!(continuing.eval(2.0, 2.0, 0.0, 4.0, 4.0), Ok(0.5));
        continuing.commit();
        assert_eq!(continuing.next_event_time(2.0), Some(5.0));
        assert_eq!(continuing.eval(2.0, 3.0, 0.0, 4.0, 4.0), Ok(1.0));

        let mut reversing = seeded_transition(0.0);
        reversing.eval(2.0, 1.0, 0.0, 4.0, 4.0).unwrap();
        reversing.commit();
        assert_eq!(reversing.eval(-1.0, 2.0, 0.0, 2.0, 2.0), Ok(0.5));
        reversing.commit();
        assert_eq!(reversing.next_event_time(2.0), Some(3.0));
        assert_eq!(reversing.eval(-1.0, 2.5, 0.0, 2.0, 2.0), Ok(-0.25));
    }

    #[test]
    fn immediate_transition_cancels_pending_work_and_exact_target_cancels_active_ramp() {
        let mut filter = seeded_transition(0.0);
        filter.eval(1.0, 1.0, 5.0, 2.0, 2.0).unwrap();
        filter.commit();
        filter.eval(2.0, 2.0, 5.0, 2.0, 2.0).unwrap();
        filter.commit();
        assert_eq!(filter.checkpoint().pending.len(), 2);

        filter.eval(3.0, 3.0, 0.0, 2.0, 2.0).unwrap();
        filter.commit();
        assert!(filter.checkpoint().pending.is_empty());
        assert_eq!(filter.next_event_time(3.0), Some(5.0));
        assert_eq!(filter.eval(1.5, 4.0, 0.0, 2.0, 2.0), Ok(1.5));
        filter.commit();
        assert_eq!(filter.next_event_time(4.0), None);
    }

    #[test]
    fn transition_rejects_invalid_timing_without_mutating_accepted_history() {
        let mut filter = seeded_transition(0.0);
        let accepted = filter.checkpoint();
        for result in [
            filter.eval(1.0, 1.0, -1.0, 1.0, 1.0),
            filter.eval(1.0, 1.0, 0.0, f64::NAN, 1.0),
            filter.eval(1.0, 1.0, 0.0, 1.0, f64::INFINITY),
        ] {
            assert!(result.is_err());
            assert_eq!(filter.checkpoint(), accepted);
        }
    }

    /// Focused transition hot-path benchmark. Run with:
    /// `cargo test -p rspice-veriloga --release --lib
    /// transition_candidate_microbenchmark -- --ignored --nocapture`.
    ///
    /// This deliberately measures repeated Newton candidates with a queued
    /// event. The accepted queue is shared copy-on-write, so a read-only
    /// reevaluation must not allocate or copy arbitrary pending history.
    #[test]
    #[ignore = "manual runtime microbenchmark"]
    fn transition_candidate_microbenchmark() {
        const ITERATIONS: usize = 1_000_000;
        const QUEUED_EVENTS: usize = 64;
        let mut filter = seeded_transition(0.0);
        for index in 1..=QUEUED_EVENTS {
            let time = index as f64 * 1.0e-3;
            filter.eval(index as f64, time, 100.0, 0.4, 0.6).unwrap();
            filter.commit();
        }
        assert_eq!(filter.committed.pending.len(), QUEUED_EVENTS);

        let started = std::time::Instant::now();
        let mut checksum = 0.0;
        for _ in 0..ITERATIONS {
            checksum += std::hint::black_box(
                filter
                    .eval(
                        std::hint::black_box(QUEUED_EVENTS as f64),
                        0.1,
                        100.0,
                        0.4,
                        0.6,
                    )
                    .unwrap(),
            );
        }
        let cow_elapsed = started.elapsed();
        let cow_ns = cow_elapsed.as_nanos() as f64 / ITERATIONS as f64;

        let started = std::time::Instant::now();
        let mut derivative_checksum = 0.0;
        for _ in 0..ITERATIONS {
            derivative_checksum += std::hint::black_box(
                filter
                    .eval_derivative(
                        std::hint::black_box(QUEUED_EVENTS as f64),
                        1.0,
                        0.1,
                        100.0,
                        0.4,
                        0.6,
                        2,
                    )
                    .unwrap(),
            );
        }
        let derivative_elapsed = started.elapsed();
        let derivative_ns = derivative_elapsed.as_nanos() as f64 / ITERATIONS as f64;

        // Paired reference for the pre-COW representation: eagerly clone the
        // full accepted queue on every speculative Newton evaluation before
        // running the identical candidate. This keeps the comparison inside
        // one optimized binary and avoids machine/load drift between commits.
        let started = std::time::Instant::now();
        let mut eager_checksum = 0_usize;
        for _ in 0..ITERATIONS {
            let copied = std::hint::black_box(filter.committed.pending.as_ref().clone());
            eager_checksum ^= std::hint::black_box(copied.len());
            checksum += std::hint::black_box(
                filter
                    .eval(
                        std::hint::black_box(QUEUED_EVENTS as f64),
                        0.1,
                        100.0,
                        0.4,
                        0.6,
                    )
                    .unwrap(),
            );
        }
        let eager_elapsed = started.elapsed();
        let eager_ns = eager_elapsed.as_nanos() as f64 / ITERATIONS as f64;
        eprintln!(
            "transition_candidate_microbenchmark iterations={ITERATIONS} queued_events={QUEUED_EVENTS} cow_elapsed={cow_elapsed:?} cow_ns_per_candidate={cow_ns:.3} derivative_elapsed={derivative_elapsed:?} derivative_ns_per_candidate={derivative_ns:.3} eager_reference_elapsed={eager_elapsed:?} eager_reference_ns_per_candidate={eager_ns:.3} avoided_eager_copy_ratio={:.3} checksum={checksum} derivative_checksum={derivative_checksum} eager_checksum={eager_checksum}",
            eager_ns / cow_ns,
        );
        assert_eq!(checksum.to_bits(), 0.0_f64.to_bits());
        assert_eq!(derivative_checksum.to_bits(), 0.0_f64.to_bits());
        assert_eq!(eager_checksum, 0);
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
    fn slew_persists_exact_rising_and_falling_catch_up_corners() {
        let mut slew = SlewFilter::default();
        slew.eval_operating_point(0.0, 0.0);
        slew.promote_operating_point_candidate();

        assert_eq!(slew.eval(2.0, 1.0, TEST_SLEW_RATES), 0.5);
        assert_eq!(slew.next_corner_time(1.0), None, "candidate is speculative");
        slew.validate_commit(1.0).unwrap();
        slew.commit();
        assert_eq!(slew.next_corner_time(1.0), Some(4.0));

        assert_eq!(slew.eval(-1.0, 2.0, TEST_SLEW_RATES), 0.0);
        slew.validate_commit(2.0).unwrap();
        slew.commit();
        assert_eq!(slew.next_corner_time(2.0), Some(4.0));
    }

    #[test]
    fn slew_exact_corner_makes_long_and_split_steps_identical() {
        let seeded = || {
            let mut slew = SlewFilter::default();
            slew.eval_operating_point(0.0, 0.0);
            slew.promote_operating_point_candidate();
            slew
        };

        let mut long = seeded();
        assert_eq!(long.eval(1.0, 2.0, TEST_SLEW_RATES), 1.0);

        let mut split = seeded();
        assert_eq!(split.eval(1.0, 1.0, TEST_SLEW_RATES), 0.5);
        split.commit();
        assert_eq!(split.next_corner_time(1.0), Some(2.0));
        assert_eq!(split.eval(1.0, 2.0, TEST_SLEW_RATES), 1.0);

        assert_eq!(
            long.candidate.output.to_bits(),
            split.candidate.output.to_bits()
        );
        assert_eq!(long.candidate.next_corner_time, None);
        assert_eq!(split.candidate.next_corner_time, None);
    }

    #[test]
    fn slew_rejected_candidate_and_checkpoint_restore_preserve_accepted_corner() {
        let mut slew = SlewFilter::default();
        slew.eval_operating_point(0.0, 0.0);
        slew.promote_operating_point_candidate();
        slew.eval(1.0, 1.0, TEST_SLEW_RATES);
        slew.commit();
        let accepted = slew.checkpoint();
        assert_eq!(accepted.next_corner_time, Some(2.0));

        slew.begin_evaluation();
        slew.eval(-10.0, 1.5, TEST_SLEW_RATES);
        assert_eq!(slew.next_corner_time(1.0), Some(2.0));
        slew.begin_evaluation();

        let mut restored = SlewFilter::default();
        restored.restore_checkpoint(&accepted);
        assert_eq!(restored.next_corner_time(1.0), Some(2.0));
        assert_eq!(restored.checkpoint(), accepted);
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
        transition
            .eval_operating_point(0.0, 0.0, 0.0, 2.0, 2.0)
            .unwrap();
        transition.promote_operating_point_candidate();
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), Ok(0.0));
        assert_eq!(transition.eval(2.0, 1.0, 0.0, 2.0, 2.0), Ok(0.0));
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), Ok(0.0));

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
    fn above_detects_same_time_dc_sweep_crossings_without_refinement() {
        let mut above = CrossDetector::default();
        assert_eq!(above.eval_above_static(-1.0, 0.0, 0.0, 0.0, true), Ok(0.0));
        above.commit();

        assert_eq!(above.eval_above_static(1.0, 0.0, 0.0, 0.0, true), Ok(1.0));
        assert_eq!(above.candidate_refinement_time(), None);
        assert_eq!(
            above.eval_above_static(1.0, 0.0, 0.0, 0.0, true),
            Ok(1.0),
            "repeated Newton evaluation must replay from accepted DC history"
        );
        above.commit();
        assert_eq!(above.eval_above_static(2.0, 0.0, 0.0, 0.0, true), Ok(0.0));
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionSegmentCheckpoint {
    pub origin_time: f64,
    pub origin_value: f64,
    pub destination: f64,
    pub end_time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingTransitionCheckpoint {
    pub start_time: f64,
    pub destination: f64,
    pub rise_time: f64,
    pub fall_time: f64,
}

#[derive(Debug, Clone)]
struct TransitionState {
    input: f64,
    output: f64,
    time: f64,
    active: Option<TransitionSegmentCheckpoint>,
    pending: Arc<VecDeque<PendingTransitionCheckpoint>>,
    initialized: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionCheckpoint {
    pub input: f64,
    pub output: f64,
    pub time: f64,
    pub active: Option<TransitionSegmentCheckpoint>,
    pub pending: Vec<PendingTransitionCheckpoint>,
    pub initialized: bool,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            input: 0.0,
            output: 0.0,
            time: 0.0,
            active: None,
            pending: Arc::new(VecDeque::new()),
            initialized: false,
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
    ) -> Result<f64, String> {
        self.eval_with_input_coefficient(input, time, delay, rise_time, fall_time)
            .map(|evaluation| evaluation.output)
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
    ) -> Result<StatefulEvaluation, String> {
        let (state, evaluation) =
            self.candidate_evaluation(input, time, delay, rise_time, fall_time)?;
        self.candidate = state;
        self.candidate_time = time;
        self.candidate_valid = true;
        Ok(evaluation)
    }

    /// Read-only exact local Jacobian action for one transition candidate.
    ///
    /// Transient evaluation recomputes the same branch from accepted history
    /// as the primal candidate. Delayed and finite-duration segments therefore
    /// have zero dependence on the current Newton input, while an
    /// instantaneous direct transition has unit dependence. Operating-point
    /// and small-signal analyses use the LRM unity approximation.
    pub(crate) fn eval_derivative(
        &self,
        input: f64,
        input_derivative: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
        analysis_type: u8,
    ) -> Result<f64, String> {
        if !input_derivative.is_finite() {
            return Err(format!(
                "transition input derivative must be finite, got {input_derivative}"
            ));
        }
        match analysis_type {
            2 => self
                .candidate_evaluation(input, time, delay, rise_time, fall_time)
                .map(|(_, evaluation)| evaluation.input_coefficient * input_derivative),
            0 | 1 | 3 | 4 => {
                Self::validate_operands(input, time, delay, rise_time, fall_time)?;
                Ok(input_derivative)
            }
            _ => Err(format!(
                "transition derivative received invalid analysis type {analysis_type}"
            )),
        }
    }

    fn candidate_evaluation(
        &self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> Result<(TransitionState, StatefulEvaluation), String> {
        Self::validate_operands(input, time, delay, rise_time, fall_time)?;
        let mut state = self.committed.clone();
        if !state.initialized {
            state = TransitionState {
                input,
                output: input,
                time,
                active: None,
                pending: Arc::new(VecDeque::new()),
                initialized: true,
            };
            return Ok((
                state,
                StatefulEvaluation {
                    output: input,
                    input_coefficient: 1.0,
                },
            ));
        }
        if time < state.time {
            return Err(format!(
                "transition candidate time {time} precedes accepted time {}",
                state.time
            ));
        }

        Self::advance_to(&mut state, time)?;
        let input_changed = input != state.input;
        let mut input_coefficient = if delay == 0.0
            && rise_time == 0.0
            && fall_time == 0.0
            && state.active.is_none()
            && state.pending.is_empty()
            && state.output == input
        {
            1.0
        } else {
            0.0
        };
        if input_changed {
            let start_time = time + delay;
            if !start_time.is_finite() {
                return Err("transition delay produces a non-finite event time".into());
            }
            let event = PendingTransitionCheckpoint {
                start_time,
                destination: input,
                rise_time,
                fall_time,
            };
            state.input = input;
            if delay == 0.0 {
                Arc::make_mut(&mut state.pending).clear();
                input_coefficient = Self::apply_transition(&mut state, event)?;
            } else {
                let pending = Arc::make_mut(&mut state.pending);
                while pending
                    .back()
                    .is_some_and(|scheduled| scheduled.start_time >= start_time)
                {
                    pending.pop_back();
                }
                pending.push_back(event);
            }
        }
        state.time = time;
        Ok((
            state.clone(),
            StatefulEvaluation {
                output: state.output,
                input_coefficient,
            },
        ))
    }

    pub(crate) fn validate_operands(
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> Result<(), String> {
        if !input.is_finite() {
            return Err(format!("transition input must be finite, got {input}"));
        }
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "transition evaluation time must be finite and non-negative, got {time}"
            ));
        }
        for (name, value) in [
            ("delay", delay),
            ("rise time", rise_time),
            ("fall time", fall_time),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!(
                    "transition {name} must be finite and non-negative, got {value}"
                ));
            }
        }
        Ok(())
    }

    fn output_at(state: &TransitionState, time: f64) -> f64 {
        let Some(active) = state.active else {
            return state.output;
        };
        if time >= active.end_time {
            return active.destination;
        }
        let duration = active.end_time - active.origin_time;
        let fraction = (time - active.origin_time) / duration;
        active.origin_value + fraction * (active.destination - active.origin_value)
    }

    fn advance_to(state: &mut TransitionState, time: f64) -> Result<(), String> {
        loop {
            let Some(event) = state.pending.front().copied() else {
                break;
            };
            if event.start_time > time {
                break;
            }
            state.output = Self::output_at(state, event.start_time);
            if state
                .active
                .is_some_and(|active| event.start_time >= active.end_time)
            {
                state.active = None;
            }
            Arc::make_mut(&mut state.pending).pop_front();
            Self::apply_transition(state, event)?;
        }

        state.output = Self::output_at(state, time);
        if state.active.is_some_and(|active| time >= active.end_time) {
            state.active = None;
        }
        state.time = time;
        Ok(())
    }

    /// Apply one transition at its exact leading corner. The interruption
    /// rules follow VAMS-2023 4.5.8: continuing in the same direction uses
    /// the original origin to establish the new slope, while reversing uses
    /// the original destination. The resulting line is shifted through the
    /// interruption point so repeated interruptions retain the required
    /// effective origin.
    fn apply_transition(
        state: &mut TransitionState,
        event: PendingTransitionCheckpoint,
    ) -> Result<f64, String> {
        let output = state.output;
        if event.destination == output {
            state.active = None;
            state.output = event.destination;
            return Ok(if event.rise_time == 0.0 && event.fall_time == 0.0 {
                // With no smoothing on either side the operator is the
                // identity even when a history cancellation happens to land
                // exactly on the candidate input. Keeping unity here avoids
                // a branch-dependent singular Jacobian at that base point.
                1.0
            } else {
                0.0
            });
        }

        let transition_time = if event.destination > output {
            event.rise_time
        } else {
            event.fall_time
        };
        if transition_time == 0.0 {
            state.active = None;
            state.output = event.destination;
            return Ok(1.0);
        }

        let reference = match state.active {
            Some(active) if event.start_time < active.end_time => {
                let was_rising = active.destination > active.origin_value;
                let reverses = if was_rising {
                    event.destination < output
                } else {
                    event.destination > output
                };
                if reverses {
                    active.destination
                } else {
                    active.origin_value
                }
            }
            _ => output,
        };
        let slope = (event.destination - reference) / transition_time;
        if !slope.is_finite() || slope == 0.0 {
            return Err("transition interruption produced an invalid slope".into());
        }
        let origin_time = event.start_time + (reference - output) / slope;
        let end_time = event.start_time + (event.destination - output) / slope;
        if !origin_time.is_finite() || !end_time.is_finite() || end_time <= event.start_time {
            return Err("transition interruption produced an invalid corner time".into());
        }
        state.active = Some(TransitionSegmentCheckpoint {
            origin_time,
            origin_value: reference,
            destination: event.destination,
            end_time,
        });
        state.output = output;
        Ok(0.0)
    }

    /// Record the final operating-point Newton candidate. It becomes the
    /// accepted transient seed only when the engine activates integration.
    pub(crate) fn eval_operating_point(
        &mut self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> Result<f64, String> {
        Self::validate_operands(input, time, delay, rise_time, fall_time)?;
        self.candidate = TransitionState {
            input,
            output: input,
            time,
            active: None,
            pending: Arc::new(VecDeque::new()),
            initialized: true,
        };
        self.candidate_time = time;
        self.candidate_valid = true;
        Ok(input)
    }

    pub(crate) fn promote_operating_point_candidate(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate.clone();
        }
        self.candidate = self.committed.clone();
        self.candidate_valid = false;
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate.clone();
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
        Self::validate_checkpoint(&Self::checkpoint_from_state(&self.candidate))?;
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
        Self::checkpoint_from_state(&self.committed)
    }

    fn checkpoint_from_state(state: &TransitionState) -> TransitionCheckpoint {
        TransitionCheckpoint {
            input: state.input,
            output: state.output,
            time: state.time,
            active: state.active,
            pending: state.pending.iter().copied().collect(),
            initialized: state.initialized,
        }
    }

    /// Exact accepted leading/trailing corner strictly after `time`.
    pub(crate) fn next_event_time(&self, time: f64) -> Option<f64> {
        let pending = self
            .committed
            .pending
            .front()
            .map(|event| event.start_time)
            .filter(|event| *event > time);
        let active = self
            .committed
            .active
            .map(|segment| segment.end_time)
            .filter(|event| *event > time);
        [pending, active].into_iter().flatten().reduce(f64::min)
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate_valid {
            return Err("transition has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    pub(crate) fn validate_checkpoint(checkpoint: &TransitionCheckpoint) -> Result<(), String> {
        if [checkpoint.input, checkpoint.output, checkpoint.time]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err("transition accepted state is not finite".into());
        }
        if checkpoint.time < 0.0 {
            return Err("transition accepted time is negative".into());
        }
        if !checkpoint.initialized
            && (checkpoint.active.is_some() || !checkpoint.pending.is_empty())
        {
            return Err("uninitialized transition contains active or pending state".into());
        }
        if let Some(active) = checkpoint.active {
            if [
                active.origin_time,
                active.origin_value,
                active.destination,
                active.end_time,
            ]
            .iter()
            .any(|value| !value.is_finite())
                || active.end_time <= checkpoint.time
                || active.end_time <= active.origin_time
            {
                return Err("transition active segment has invalid corners".into());
            }
        }
        let mut previous = checkpoint.time;
        for event in &checkpoint.pending {
            if [
                event.start_time,
                event.destination,
                event.rise_time,
                event.fall_time,
            ]
            .iter()
            .any(|value| !value.is_finite())
                || event.start_time <= previous
                || event.rise_time < 0.0
                || event.fall_time < 0.0
            {
                return Err("transition pending queue is invalid or unordered".into());
            }
            previous = event.start_time;
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &TransitionCheckpoint) {
        self.committed = TransitionState {
            input: checkpoint.input,
            output: checkpoint.output,
            time: checkpoint.time,
            active: checkpoint.active,
            pending: Arc::new(checkpoint.pending.iter().copied().collect()),
            initialized: checkpoint.initialized,
        };
        self.candidate = self.committed.clone();
        self.candidate_time = 0.0;
        self.candidate_valid = false;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SlewState {
    output: f64,
    prev_time: f64,
    /// Exact future time at which a rate-limited segment reaches the input
    /// accepted at `prev_time`.  The transient engine must not step over this
    /// corner: doing so extends the saturated slope beyond its target and
    /// makes the result depend on the solver's otherwise-unrelated step size.
    next_corner_time: Option<f64>,
    initialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlewCheckpoint {
    pub output: f64,
    pub prev_time: f64,
    pub next_corner_time: Option<f64>,
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
                next_corner_time: None,
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
        state.next_corner_time = if input_coefficient == 0.0 && state.output != input {
            // Keep a malformed/overflowed result visible to acceptance
            // validation instead of silently dropping an exact breakpoint.
            // Normalized slew magnitudes are finite and strictly positive.
            let remaining_time = (input - state.output).abs() / limited_rate.abs();
            Some(time + remaining_time)
        } else {
            None
        };
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
            next_corner_time: None,
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
            next_corner_time: self.candidate.next_corner_time,
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
            next_corner_time: self.committed.next_corner_time,
            initialized: self.committed.initialized,
        }
    }

    /// Exact accepted catch-up corner strictly after `time`, if the latest
    /// accepted slew segment is still rate limited.
    pub(crate) fn next_corner_time(&self, time: f64) -> Option<f64> {
        self.committed
            .next_corner_time
            .filter(|corner| corner.is_finite() && *corner > time)
    }

    /// Accepted local gain used when a caller intentionally carries transient
    /// state into AC/noise. A still-active rate-limited segment is determined
    /// entirely by history and has zero gain from the current input; a settled
    /// or fresh operating point follows the input with unity gain.
    pub(crate) fn small_signal_input_gain(&self, time: f64) -> f64 {
        if self.next_corner_time(time).is_some() {
            0.0
        } else {
            1.0
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
        if checkpoint.next_corner_time.is_some_and(|corner| {
            !corner.is_finite() || corner <= checkpoint.prev_time || !checkpoint.initialized
        }) {
            return Err(
                "slew catch-up corner must be finite, initialized, and strictly future".into(),
            );
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &SlewCheckpoint) {
        self.committed = SlewState {
            output: checkpoint.output,
            prev_time: checkpoint.prev_time,
            next_corner_time: checkpoint.next_corner_time,
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
        self.eval_impl(
            value, time, direction, time_tol, expr_tol, enabled, false, false,
        )
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
        self.eval_impl(value, time, 1, time_tol, expr_tol, enabled, true, false)
    }

    /// Evaluate `above` at accepted DC/IC sweep points. These analyses keep
    /// simulation time fixed, so an upward crossing must compare consecutive
    /// accepted values without requesting transient root refinement.
    pub fn eval_above_static(
        &mut self,
        value: f64,
        time: f64,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
    ) -> Result<f64, String> {
        self.eval_impl(value, time, 1, time_tol, expr_tol, enabled, true, true)
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
        allow_same_time_crossing: bool,
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

        let same_time_static = allow_same_time_crossing && time == self.committed.time;
        if time < self.committed.time || (time == self.committed.time && !same_time_static) {
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

        if fired && !same_time_static {
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
