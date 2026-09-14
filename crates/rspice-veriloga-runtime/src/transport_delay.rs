//! Accepted transport-delay history shared by native and Verilog-A devices.
//!
//! The numerical and frozen-definition contract is the existing `absdelay`
//! implementation. Moving it below the language compiler lets native devices
//! use the same candidate coefficients and accepted-state lifecycle without
//! enabling a Verilog-A compiler feature. Delay interpolation error control and
//! delayed breakpoint scheduling remain responsibilities of the owning solver.

use std::collections::VecDeque;

use crate::arithmetic::{product_div, product_sum_div, sum_products_ratio};

/// A clamped delayed time, held exactly as two binary64
/// words. Rounding the absolute target alone can lose a physical delay or
/// select the opposite side of an accepted interpolation knot.
#[derive(Clone, Copy)]
struct DelayTarget {
    high: f64,
    low: f64,
    prehistory: bool,
}

impl DelayTarget {
    fn new(time: f64, delay: f64) -> Self {
        if time <= delay {
            return Self {
                high: 0.0,
                low: 0.0,
                prehistory: time < delay,
            };
        }
        // Error-free TwoDiff. All operands are finite and 0 < delay < time,
        // so the high difference and reconstruction cannot overflow.
        let high = time - delay;
        let recovered_delay = time - high;
        let recovered_time = high + recovered_delay;
        let low = (time - recovered_time) + (recovered_delay - delay);
        Self {
            high,
            low,
            prehistory: false,
        }
    }

    fn at_or_after(self, sample: f64) -> bool {
        !self.prehistory && (self.high > sample || (self.high == sample && self.low >= 0.0))
    }

    fn after(self, sample: f64) -> f64 {
        (self.high - sample) + self.low
    }

    fn before(self, sample: f64) -> f64 {
        (sample - self.high) - self.low
    }

    /// Retain cancellation before rounding either interpolation weight or
    /// large absolute-time products. This uses the shared exact scalar
    /// arithmetic only when opposite-sign endpoints can cancel.
    fn interpolate_cancellation(
        self,
        left_time: f64,
        left_value: f64,
        right_time: f64,
        right_value: f64,
    ) -> Result<f64, String> {
        sum_products_ratio(
            [
                (self.high, right_value),
                (self.low, right_value),
                (-left_time, right_value),
                (right_time, left_value),
                (-self.high, left_value),
                (-self.low, left_value),
            ]
            .into_iter(),
            [(right_time, 1.0), (left_time, -1.0)].into_iter(),
        )
        .map_err(|error| {
            format!("absdelay interpolation cancellation is not representable: {error:?}")
        })
    }
}

/// Maximum number of accepted sample records retained by one `absdelay` site,
/// counting both ordinary right samples and any separate jump left limits.
///
/// The time horizon is normally the tighter bound, but an adaptive solver can
/// accept arbitrarily many points inside a finite interval. Refusing an
/// over-budget candidate is preferable to either unbounded allocation or
/// silently discarding interpolation data.
pub const MAX_DELAY_HISTORY_SAMPLES: usize = 1_048_576;

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
    left_limit: Option<f64>,
    configuration: DelayConfiguration,
}

/// A transport-delay candidate's exact local affine coefficients.
///
/// Accepted history and the frozen definition are constants. The two
/// coefficients are with respect to the current `expr` and `td` arguments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayEvaluation {
    pub output: f64,
    pub input_coefficient: f64,
    pub delay_coefficient: f64,
}

/// A directly evaluated delayed-minus-present signal. The current-input
/// action retains its ratio until applied to a device derivative: rounding
/// a tiny interpolation weight first can erase a representable Jacobian.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DelayDifferenceEvaluation {
    pub output: f64,
    pub delay_coefficient: f64,
    input_numerator: f64,
    input_denominator: f64,
}

impl DelayDifferenceEvaluation {
    /// Apply the local current-input action to one device derivative.
    /// Accepted samples are constants; their derivatives do not propagate.
    pub fn apply_input_derivative(&self, derivative: f64) -> Result<f64, String> {
        if !derivative.is_finite() {
            return Err("transport correction input derivative must be finite".into());
        }
        let result = product_div(self.input_numerator, derivative, self.input_denominator);
        if !result.is_finite() {
            return Err("transport correction derivative is not representable".into());
        }
        Ok(result)
    }
}

/// Accepted/candidate state for the Verilog-A `absdelay` operator.
///
/// Accepted samples are strictly increasing and begin at time zero. The
/// candidate is separate so repeated Newton evaluations and rejected steps do
/// not mutate trajectory history. Accepted history is pruned to the configured
/// delay horizon while retaining the predecessor needed for exact linear
/// interpolation.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayBuffer {
    samples: VecDeque<(f64, f64)>,
    left_limits: VecDeque<(f64, f64)>,
    configuration: Option<DelayConfiguration>,
    candidate: Option<DelayCandidate>,
}

/// Accepted transport-delay state. Speculative Newton candidates are never
/// part of a checkpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayCheckpoint {
    pub configuration: Option<DelayConfiguration>,
    pub samples: Vec<(f64, f64)>,
    /// Left values at explicitly declared events; `samples` retains right values.
    /// Equal sides preserve a continuous corner that also needs an arrival.
    pub left_limits: Vec<(f64, f64)>,
}

impl DelayBuffer {
    /// Create a delay buffer with an allocation hint, not a retention limit.
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(capacity.min(MAX_DELAY_HISTORY_SAMPLES)),
            left_limits: VecDeque::new(),
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
    pub fn eval_operating_point(
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
                left_limit: None,
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
    pub fn small_signal_delay(
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
    pub fn eval_with_coefficients(
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
            left_limit: None,
            configuration,
        });
        Ok(evaluation)
    }

    /// Read transport history with the settled current sample held fixed.
    /// A variable delay can move the read point through that history, but the
    /// observation cannot replace the sample or change its frozen definition.
    pub fn static_dae_with_coefficients(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayEvaluation, String> {
        let configuration = self
            .configuration
            .or_else(|| self.candidate.map(|sample| sample.configuration));
        Self::validate_runtime_configuration(configuration, time, value, delay, max_delay)?;
        let retained_value = match self.candidate {
            Some(sample) if sample.time.to_bits() == time.to_bits() => sample.value,
            Some(sample) => {
                return Err(format!(
                    "absdelay candidate belongs to time {}, not observation time {time}",
                    sample.time
                ));
            }
            None => self
                .samples
                .back()
                .filter(|sample| sample.0.to_bits() == time.to_bits())
                .map(|sample| sample.1)
                .ok_or_else(|| {
                    format!("static observation requires a settled absdelay sample at time {time}")
                })?,
        };
        let (_, effective_delay, delay_scale) =
            Self::resolve_frozen_configuration(configuration, delay, max_delay)?;
        let mut evaluation =
            self.candidate_evaluation(time, retained_value, effective_delay, delay_scale)?;
        // Even when interpolation uses the in-flight endpoint, its value is
        // retained history in this observation rather than the new input.
        evaluation.input_coefficient = 0.0;
        Ok(evaluation)
    }

    /// Read `delayed(value) - value` directly, without rounding the delayed
    /// value first. Neither accepted history nor a staged candidate changes.
    /// The owning device stages its input with `eval_with_coefficients` only
    /// when preparing the eventual accepted-state transaction.
    pub fn difference_with_coefficients(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayDifferenceEvaluation, String> {
        self.difference_evaluation(time, value, delay, max_delay, None)
    }

    /// Read a right-side Newton candidate with its already solved left limit
    /// held fixed. The incoming interpolation interval ends at `left`; its
    /// value does not acquire a derivative with respect to the right candidate.
    pub fn difference_at_discontinuity(
        &self,
        time: f64,
        left: f64,
        right: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayDifferenceEvaluation, String> {
        if !left.is_finite() {
            return Err("delay left limit must be finite".into());
        }
        if let Some(&(accepted_time, accepted_right)) = self.samples.back()
            && time == accepted_time
            && left.to_bits() != self.left_value(time).unwrap_or(accepted_right).to_bits()
        {
            return Err("delay probe left limit differs from its accepted knot".into());
        }
        self.difference_evaluation(time, right, delay, max_delay, Some(left))
    }

    fn difference_evaluation(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
        left_limit: Option<f64>,
    ) -> Result<DelayDifferenceEvaluation, String> {
        self.validate_runtime(time, value, delay, max_delay)?;
        if self.samples.back().is_some_and(|sample| time < sample.0) {
            return Err("transport correction time precedes the latest accepted sample".into());
        }
        let (_, effective_delay, delay_scale) = self.resolve_configuration(delay, max_delay)?;
        if self.samples.is_empty() {
            if time != 0.0 {
                return Err("transport correction requires an accepted time-zero anchor".into());
            }
            return Ok(DelayDifferenceEvaluation {
                output: 0.0,
                delay_coefficient: 0.0,
                input_numerator: 0.0,
                input_denominator: 1.0,
            });
        }
        let target = DelayTarget::new(time, effective_delay);
        let candidate = left_limit.map_or((time, value, 1.0), |left| (time, left, 0.0));
        let [left, right] = self.interpolation_endpoints(target, candidate);
        let result = match (left, right) {
            (Some((lt, lv, li)), Some((rt, rv, ri))) => {
                let interval = rt - lt;
                if !(interval.is_finite() && interval > 0.0) {
                    return Err("transport correction interval is not representable".into());
                }
                let from_right = target.before(rt).clamp(0.0, interval);
                let output = if ri == 1.0 {
                    // The right endpoint is the present input. Cancel it
                    // algebraically before applying the small delay fraction.
                    product_sum_div(lv, from_right, value, -from_right, interval)
                } else {
                    // Retain the target's low word and cancellation against
                    // the present input across an entirely accepted bracket.
                    sum_products_ratio(
                        [
                            (target.high, rv),
                            (target.low, rv),
                            (-lt, rv),
                            (rt, lv),
                            (-target.high, lv),
                            (-target.low, lv),
                            (-rt, value),
                            (lt, value),
                        ]
                        .into_iter(),
                        [(rt, 1.0), (lt, -1.0)].into_iter(),
                    )
                    .map_err(|error| {
                        format!("transport correction is not representable: {error:?}")
                    })?
                };
                let input_numerator = if ri == 1.0 {
                    -from_right
                } else if li == 1.0 {
                    -target.after(lt).clamp(0.0, interval)
                } else {
                    -interval
                };
                let delay_coefficient = if time <= effective_delay || delay_scale == 0.0 {
                    0.0
                } else {
                    -delay_scale * product_sum_div(rv, 1.0, lv, -1.0, interval)
                };
                DelayDifferenceEvaluation {
                    output,
                    delay_coefficient,
                    input_numerator,
                    input_denominator: interval,
                }
            }
            (Some((_, output, input)), None) | (None, Some((_, output, input))) => {
                DelayDifferenceEvaluation {
                    output: output - value,
                    delay_coefficient: 0.0,
                    input_numerator: input - 1.0,
                    input_denominator: 1.0,
                }
            }
            (None, None) => return Err("transport correction has no accepted anchor".into()),
        };
        if !result.output.is_finite() || !result.delay_coefficient.is_finite() {
            return Err("transport correction produced an unrepresentable result".into());
        }
        Ok(result)
    }

    fn validate_runtime(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        Self::validate_runtime_configuration(self.configuration, time, value, delay, max_delay)
    }

    fn validate_runtime_configuration(
        configuration: Option<DelayConfiguration>,
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
        match (configuration, max_delay) {
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
        Self::resolve_frozen_configuration(self.configuration, delay, max_delay)
    }

    fn resolve_frozen_configuration(
        configuration: Option<DelayConfiguration>,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(DelayConfiguration, f64, f64), String> {
        match (configuration, max_delay) {
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

        let target = DelayTarget::new(time, effective_delay);
        let target_delay_derivative = if time > effective_delay {
            -delay_scale
        } else {
            // The time-zero clamp and its exact kink are deterministic: the
            // anchored branch has zero local delay derivative.
            0.0
        };
        self.interpolate(target, (time, value), target_delay_derivative)
    }

    fn interpolation_endpoints(
        &self,
        target: DelayTarget,
        candidate: (f64, f64, f64),
    ) -> [Option<(f64, f64, f64)>; 2] {
        // Accepted times are strictly increasing, including across the
        // deque's wrap. Find the first strictly later sample in logarithmic
        // time without copying or rotating the retained history. Equality
        // stays on the left so exact knots retain their right-hand slope.
        let right_index = self
            .samples
            .partition_point(|sample| target.at_or_after(sample.0));
        let mut left = right_index
            .checked_sub(1)
            .and_then(|index| self.samples.get(index))
            .map(|&(time, value)| (time, value, 0.0));
        let mut right = self
            .samples
            .get(right_index)
            .map(|&(time, value)| (time, self.left_value(time).unwrap_or(value), 0.0));
        if target.at_or_after(candidate.0) {
            left = Some(candidate);
        } else if right.is_none() {
            right = Some(candidate);
        }
        [left, right]
    }

    fn interpolate(
        &self,
        target: DelayTarget,
        candidate: (f64, f64),
        target_delay_derivative: f64,
    ) -> Result<DelayEvaluation, String> {
        let [left, right] = self.interpolation_endpoints(target, (candidate.0, candidate.1, 1.0));
        let evaluation = match (left, right) {
            (
                Some((left_time, left_value, left_input)),
                Some((right_time, right_value, right_input)),
            ) => {
                let interval = right_time - left_time;
                if !(interval.is_finite() && interval > 0.0) {
                    return Err("absdelay interpolation interval is not representable".into());
                }
                let from_left = target.after(left_time).clamp(0.0, interval);
                let from_right = target.before(right_time).clamp(0.0, interval);
                // Compute the smaller weight directly. Subtracting a weight
                // near one would discard the tiny but physically meaningful
                // contribution from the other endpoint.
                let (alpha, one_minus_alpha) = if from_left <= from_right {
                    let alpha = from_left / interval;
                    (alpha, 1.0 - alpha)
                } else {
                    let beta = from_right / interval;
                    (1.0 - beta, beta)
                };
                let output = if left_value.to_bits() == right_value.to_bits() {
                    left_value
                } else if left_value != 0.0
                    && right_value != 0.0
                    && left_value.is_sign_negative() != right_value.is_sign_negative()
                {
                    target.interpolate_cancellation(
                        left_time,
                        left_value,
                        right_time,
                        right_value,
                    )?
                } else if !alpha.is_normal() && from_left != 0.0 {
                    // The weight can underflow before multiplication by a
                    // large endpoint would recover a representable signal.
                    one_minus_alpha
                        .mul_add(left_value, product_div(from_left, right_value, interval))
                } else if !one_minus_alpha.is_normal() && from_right != 0.0 {
                    alpha.mul_add(right_value, product_div(from_right, left_value, interval))
                } else {
                    alpha.mul_add(right_value, one_minus_alpha * left_value)
                };
                let input_coefficient = alpha.mul_add(right_input, one_minus_alpha * left_input);
                let delay_coefficient = if target_delay_derivative == 0.0 {
                    // A fixed/saturated delay has no delay-operand action.
                    // Its unused slope need not fit in binary64.
                    0.0
                } else {
                    product_sum_div(right_value, 1.0, left_value, -1.0, interval)
                        * target_delay_derivative
                };
                DelayEvaluation {
                    output,
                    input_coefficient,
                    delay_coefficient,
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
    pub fn begin_evaluation(&mut self) {
        self.candidate = None;
    }

    /// Validate the candidate that would be committed without mutation.
    pub fn validate_commit(&self, accepted_time: f64) -> Result<(), String> {
        let Some(candidate) = self.candidate else {
            return Ok(());
        };
        self.validate_candidate_commit(candidate, accepted_time)
    }

    fn validate_candidate_commit(
        &self,
        candidate: DelayCandidate,
        accepted_time: f64,
    ) -> Result<(), String> {
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
        let first = self
            .samples
            .get(self.samples.len() - retained)
            .map_or(candidate.time, |sample| sample.0);
        let retained_left =
            self.left_limits.len() - self.left_limits.partition_point(|sample| sample.0 < first);
        if retained + retained_left + 1 + usize::from(candidate.left_limit.is_some())
            > MAX_DELAY_HISTORY_SAMPLES
        {
            return Err(format!(
                "delay history requires more than the supported {MAX_DELAY_HISTORY_SAMPLES} accepted samples inside its configured horizon"
            ));
        }
        Ok(())
    }

    fn direct_sample(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayCandidate, String> {
        self.validate_runtime(time, value, delay, max_delay)?;
        let (configuration, _, _) = self.resolve_configuration(delay, max_delay)?;
        let sample = DelayCandidate {
            time,
            value,
            left_limit: None,
            configuration,
        };
        self.validate_candidate_commit(sample, time)?;
        Ok(sample)
    }

    /// Validate a native solver's accepted sample without staging it or
    /// copying retained history. This shares the VM commit contract.
    pub fn validate_sample(
        &self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        self.direct_sample(time, value, delay, max_delay)
            .map(|_| ())
    }

    /// Append a selected accepted sample after validation. Failure preserves
    /// both accepted history and any staged VM candidate; success replaces
    /// that candidate with the sample selected by the caller.
    pub fn accept_sample(
        &mut self,
        time: f64,
        value: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        let sample = self.direct_sample(time, value, delay, max_delay)?;
        self.candidate = Some(sample);
        self.apply_validated_commit();
        Ok(())
    }

    fn discontinuity_sample(
        &self,
        time: f64,
        left: f64,
        right: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<DelayCandidate, String> {
        if !left.is_finite() {
            return Err("delay left limit must be finite".into());
        }
        let mut sample = self.direct_sample(time, right, delay, max_delay)?;
        sample.left_limit = Some(left);
        self.validate_candidate_commit(sample, time)?;
        Ok(sample)
    }

    /// Preflight both sides of an event without changing accepted or staged
    /// state. The owning solver must supply the actual left and settled right
    /// limits; this buffer does not infer a jump from sample differences.
    pub fn validate_discontinuity(
        &self,
        time: f64,
        left: f64,
        right: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        self.discontinuity_sample(time, left, right, delay, max_delay)
            .map(|_| ())
    }

    /// Atomically append a declared event with left and right limits. Equal
    /// values retain a continuous corner as an explicit event. The left
    /// value closes the incoming interval; the right value starts the outgoing
    /// interval and is selected exactly at the knot. A time-zero left value
    /// defines prehistory, including queries strictly before its delayed arrival.
    pub fn accept_discontinuity(
        &mut self,
        time: f64,
        left: f64,
        right: f64,
        delay: f64,
        max_delay: Option<f64>,
    ) -> Result<(), String> {
        let sample = self.discontinuity_sample(time, left, right, delay, max_delay)?;
        self.candidate = Some(sample);
        self.apply_validated_commit();
        Ok(())
    }

    fn left_value(&self, time: f64) -> Option<f64> {
        let index = self.left_limits.partition_point(|sample| sample.0 < time);
        self.left_limits
            .get(index)
            .filter(|sample| sample.0 == time)
            .map(|sample| sample.1)
    }

    /// Next representable time at or after an accepted jump's exact fixed-delay
    /// arrival. The scheduler must land this event; interpolation-knot spacing
    /// itself is not an event and places no timestep-at-most-delay requirement.
    pub fn next_discontinuity_after(&self, time: f64) -> Result<Option<f64>, String> {
        if !time.is_finite() || time < 0.0 {
            return Err("delay arrival query time must be finite and non-negative".into());
        }
        let delay = match self.configuration {
            Some(DelayConfiguration::Fixed { delay }) => delay,
            Some(DelayConfiguration::Bounded { .. }) => {
                return Err(
                    "variable-delay arrivals require the owning solver's delay trajectory".into(),
                );
            }
            None => return Ok(None),
        };
        let target = DelayTarget::new(time, delay);
        let index = self
            .left_limits
            .partition_point(|sample| target.at_or_after(sample.0));
        let Some(&(source_time, _)) = self.left_limits.get(index) else {
            return Ok(None);
        };
        let high = source_time + delay;
        if !high.is_finite() {
            return Ok(None);
        }
        // Error-free TwoSum: round upward only if binary64 addition landed
        // before the physical arrival, including delays below the source ulp.
        let recovered_delay = high - source_time;
        let low = (source_time - (high - recovered_delay)) + (delay - recovered_delay);
        let arrival = if low > 0.0 { high.next_up() } else { high };
        Ok(arrival.is_finite().then_some(arrival))
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
    fn apply_validated_commit(&mut self) {
        let Some(candidate) = self.candidate.take() else {
            return;
        };
        if self.configuration.is_none() {
            self.configuration = Some(candidate.configuration);
        }
        self.samples.push_back((candidate.time, candidate.value));
        if let Some(left) = candidate.left_limit {
            self.left_limits.push_back((candidate.time, left));
        }
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
        if let Some(&(first, _)) = self.samples.front() {
            while self
                .left_limits
                .front()
                .is_some_and(|sample| sample.0 < first)
            {
                self.left_limits.pop_front();
            }
        }
    }

    /// Clear accepted definition, history, and speculative state.
    pub fn clear(&mut self) {
        self.samples.clear();
        self.left_limits.clear();
        self.configuration = None;
        self.candidate = None;
    }

    /// Allocated sample slots, for solver memory accounting.
    pub fn allocation_capacity(&self) -> usize {
        self.samples.capacity() + self.left_limits.capacity()
    }

    /// Number of retained accepted samples; the candidate is excluded.
    pub fn accepted_sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn accepted_configuration(&self) -> Option<DelayConfiguration> {
        self.configuration
    }

    /// Iterate accepted samples without allocating a checkpoint copy.
    pub fn accepted_samples(
        &self,
    ) -> impl DoubleEndedIterator<Item = (f64, f64)> + ExactSizeIterator + '_ {
        self.samples.iter().copied()
    }

    /// Accepted (time, left, right) knots, without copying retained history.
    pub fn accepted_knots(
        &self,
    ) -> impl DoubleEndedIterator<Item = (f64, f64, f64)> + ExactSizeIterator + '_ {
        self.samples
            .iter()
            .map(|&(time, right)| (time, self.left_value(time).unwrap_or(right), right))
    }

    /// Stored left limits only, in strictly increasing accepted-time order.
    pub fn accepted_left_limits(
        &self,
    ) -> impl DoubleEndedIterator<Item = (f64, f64)> + ExactSizeIterator + '_ {
        self.left_limits.iter().copied()
    }

    pub fn checkpoint(&self) -> DelayCheckpoint {
        DelayCheckpoint {
            configuration: self.configuration,
            samples: self.samples.iter().copied().collect(),
            left_limits: self.left_limits.iter().copied().collect(),
        }
    }

    pub fn validate_checkpoint_ready(&self) -> Result<(), String> {
        if self.candidate.is_some() {
            return Err("delay has an in-flight Newton candidate".into());
        }
        Ok(())
    }

    /// Check that a solver capture ends at its accepted time and retains the
    /// left interpolation bracket needed at that time. Runtime construction
    /// and restoration already enforce finite, strictly increasing samples.
    pub fn validate_accepted_time(&self, time: f64) -> Result<(), String> {
        self.validate_checkpoint_ready()?;
        let configuration = self
            .configuration
            .ok_or("delay history has no accepted definition")?;
        let &(last_time, _) = self
            .samples
            .back()
            .ok_or("delay history has no accepted samples")?;
        if !time.is_finite() || time < 0.0 || last_time.to_bits() != time.to_bits() {
            return Err("delay history does not end at the solver's accepted time".into());
        }
        let first_time = self.samples.front().expect("nonempty accepted history").0;
        if first_time != 0.0
            && !DelayTarget::new(time, configuration.retention()).at_or_after(first_time)
        {
            return Err("delay history is missing its retained interpolation bracket".into());
        }
        Ok(())
    }

    pub fn validate_checkpoint(checkpoint: &DelayCheckpoint) -> Result<(), String> {
        if checkpoint
            .samples
            .len()
            .saturating_add(checkpoint.left_limits.len())
            > MAX_DELAY_HISTORY_SAMPLES
        {
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
            None if !checkpoint.samples.is_empty() || !checkpoint.left_limits.is_empty() => {
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
        let mut previous_left = None;
        for &(time, left) in &checkpoint.left_limits {
            if !time.is_finite()
                || !left.is_finite()
                || previous_left.is_some_and(|previous| time <= previous)
            {
                return Err("delay left limits must be finite and strictly increasing".into());
            }
            let index = checkpoint.samples.partition_point(|sample| sample.0 < time);
            let Some(&(right_time, _)) = checkpoint.samples.get(index) else {
                return Err("delay left limit has no accepted right sample".into());
            };
            if time.to_bits() != right_time.to_bits() {
                return Err("delay left limit must match an accepted right sample".into());
            }
            previous_left = Some(time);
        }
        Ok(())
    }

    /// Validate and restore accepted state atomically. Invalid input leaves
    /// both accepted history and any current candidate unchanged.
    pub fn restore_checkpoint(&mut self, checkpoint: &DelayCheckpoint) -> Result<(), String> {
        Self::validate_checkpoint(checkpoint)?;
        self.samples.clear();
        self.samples.extend(checkpoint.samples.iter().copied());
        self.left_limits.clear();
        self.left_limits
            .extend(checkpoint.left_limits.iter().copied());
        self.configuration = checkpoint.configuration;
        self.candidate = None;
        Ok(())
    }

    /// Take a validated checkpoint's sample allocation without a second copy.
    pub fn from_checkpoint(checkpoint: DelayCheckpoint) -> Result<Self, String> {
        Self::validate_checkpoint(&checkpoint)?;
        Ok(Self {
            samples: checkpoint.samples.into(),
            left_limits: checkpoint.left_limits.into(),
            configuration: checkpoint.configuration,
            candidate: None,
        })
    }
}

impl Default for DelayBuffer {
    fn default() -> Self {
        Self::new(1024) // Default to 1024 samples
    }
}
