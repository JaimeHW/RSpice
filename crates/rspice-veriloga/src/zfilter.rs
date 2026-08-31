//! Z-domain (sampled-data) filters for the `zi_*` Verilog-A operators.
//!
//! The input samples every `period` seconds; on each sample instant the
//! difference equation
//!
//! ```text
//! y[n] = (b0·x[n] + b1·x[n-1] + ... - a1·y[n-1] - ...) / a0
//! ```
//!
//! updates, and the output holds (zero-order hold) between samples.
//! Coefficient arrays are ascending in z⁻¹ (b0, b1, ...), matching the
//! ascending-s convention of the `laplace_*` operators.
//!
//! Evaluation is speculative. Only [`ZiFilter::commit`] changes accepted
//! history, so rejected Newton or timestep proposals cannot leak into the
//! next attempt. The exact next sample edge is available through
//! [`ZiFilter::next_sample_step_bound`]; a caller that nevertheless crosses an
//! edge is refused because the input at the missed edge is unknowable.

use serde::{Deserialize, Deserializer, Serialize};

const TIME_ULPS: f64 = 8.0;

/// Platform-uniform ceiling for one Zi helper invocation. The four fixed
/// operands (period, first-transition time, input/action, and transition time)
/// are included; coefficient scalars consume one slot and complex roots two.
pub const MAX_ZI_RUNTIME_OPERANDS: usize = 1_024;
pub const ZI_FIXED_RUNTIME_OPERANDS: usize = 4;

/// A malformed sampled filter or an evaluation that cannot be represented
/// faithfully in `f64`.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ZiFilterError {
    /// The authored coefficient or sample-period definition is invalid.
    #[error("invalid zi transfer function: {0}")]
    InvalidDefinition(String),
    /// The DC equilibrium gain H(1) has no unique finite value.
    #[error("singular zi DC equilibrium: denominator polynomial is zero at z=1")]
    SingularDc,
    /// Runtime inputs, time, or arithmetic cannot produce an exact finite value.
    #[error("invalid zi evaluation: {0}")]
    InvalidEvaluation(String),
    /// Accepted history or scheduling state is internally inconsistent.
    #[error("invalid zi accepted state: {0}")]
    InvalidState(String),
}

/// Validate and return the complete flattened operand count for one Zi site.
/// Checked arithmetic is intentional: malformed serialized layouts must fail
/// before any backend performs slice, stack, or pointer arithmetic.
pub fn validate_zi_runtime_operand_budget(
    operator: &str,
    numerator_scalars: usize,
    denominator_scalars: usize,
) -> Result<usize, ZiFilterError> {
    let total = numerator_scalars
        .checked_add(denominator_scalars)
        .and_then(|count| count.checked_add(ZI_FIXED_RUNTIME_OPERANDS))
        .ok_or_else(|| {
            ZiFilterError::InvalidDefinition(format!(
                "{operator} runtime operand count overflows usize"
            ))
        })?;
    if total > MAX_ZI_RUNTIME_OPERANDS {
        return Err(ZiFilterError::InvalidDefinition(format!(
            "{operator} requires {total} runtime operands ({numerator_scalars} numerator-definition scalars + {denominator_scalars} denominator-definition scalars + {ZI_FIXED_RUNTIME_OPERANDS} fixed), exceeding the platform-uniform maximum {MAX_ZI_RUNTIME_OPERANDS}"
        )));
    }
    Ok(total)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Candidate {
    Failed,
    Hold,
    Sample {
        time: f64,
        input: f64,
        output: f64,
        transition: f64,
        ramp_end_time: f64,
        visible_start: f64,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ZiFilter {
    /// Whether this site's constant arguments have been frozen for the
    /// current analysis. Compiled models start with validated placeholders;
    /// the first value or derivative view installs the per-instance values.
    definition_frozen: bool,
    /// Numerator coefficients b_k (ascending powers of z⁻¹).
    num: Vec<f64>,
    /// Denominator coefficients a_k (a_0 first); a_0 is finite and nonzero.
    den: Vec<f64>,
    /// Sample period (s).
    period: f64,
    /// First sample transition time. Negative values are rejected so no
    /// pre-zero history has to be synthesized.
    first_transition: f64,
    /// Committed input history x[n-1], x[n-2], ...
    x_hist: Vec<f64>,
    /// Committed output history y[n-1], y[n-2], ...
    y_hist: Vec<f64>,
    /// Committed held output between samples.
    held: f64,
    /// Accepted output-transition segment. `held` is the difference-equation
    /// target; these fields describe the visible linear transition to it.
    ramp_start_time: f64,
    ramp_end_time: f64,
    ramp_start_value: f64,
    /// Integer index of the next sample. Its time is
    /// `first_transition + index * period`.
    next_sample_index: u64,
    /// Last globally accepted transient time, including non-sampling steps.
    accepted_time: Option<f64>,
    /// Whether this compiled state slot has participated in an accepted
    /// transient point. Only accepted activity is allowed to schedule future
    /// sample edges.
    transient_active: bool,
    /// Whether this slot has completed at least one transient evaluation in
    /// the current analysis. This is not accepted numeric state: it only lets
    /// commit validation distinguish a truly dormant analysis-specific slot
    /// from an operator that was evaluated in an earlier Newton pass and then
    /// skipped by the final pass.
    transient_seen: bool,
    /// Candidate produced by the latest complete device evaluation. In-flight
    /// Newton state is intentionally not checkpoint state.
    #[serde(skip)]
    candidate: Option<Candidate>,
}

/// Complete accepted Zi state, including a lazily frozen per-instance
/// definition. In-flight Newton candidates are intentionally excluded.
#[derive(Debug, Clone, PartialEq)]
pub struct ZiCheckpoint {
    pub definition_frozen: bool,
    pub num: Vec<f64>,
    pub den: Vec<f64>,
    pub period: f64,
    pub first_transition: f64,
    pub x_hist: Vec<f64>,
    pub y_hist: Vec<f64>,
    pub held: f64,
    pub ramp_start_time: f64,
    pub ramp_end_time: f64,
    pub ramp_start_value: f64,
    pub next_sample_index: u64,
    pub accepted_time: Option<f64>,
    pub transient_active: bool,
    pub transient_seen: bool,
}

/// Deserialization-only representation. Runtime candidates are deliberately
/// absent: they belong to an in-flight Newton pass and are never checkpoint
/// state. Keeping this type private ensures every external serde format goes
/// through [`ZiFilter::validate_integrity`] before it can construct a filter.
#[derive(Deserialize)]
struct ZiFilterWire {
    #[serde(default = "definition_frozen_default")]
    definition_frozen: bool,
    num: Vec<f64>,
    den: Vec<f64>,
    period: f64,
    first_transition: f64,
    x_hist: Vec<f64>,
    y_hist: Vec<f64>,
    held: f64,
    ramp_start_time: f64,
    ramp_end_time: f64,
    ramp_start_value: f64,
    next_sample_index: u64,
    #[serde(default)]
    accepted_time: Option<f64>,
    #[serde(default)]
    transient_active: bool,
    #[serde(default)]
    transient_seen: bool,
}

impl<'de> Deserialize<'de> for ZiFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ZiFilterWire::deserialize(deserializer)?;
        let filter = Self {
            definition_frozen: wire.definition_frozen,
            num: wire.num,
            den: wire.den,
            period: wire.period,
            first_transition: wire.first_transition,
            x_hist: wire.x_hist,
            y_hist: wire.y_hist,
            held: wire.held,
            ramp_start_time: wire.ramp_start_time,
            ramp_end_time: wire.ramp_end_time,
            ramp_start_value: wire.ramp_start_value,
            next_sample_index: wire.next_sample_index,
            accepted_time: wire.accepted_time,
            transient_active: wire.transient_active,
            transient_seen: wire.transient_seen,
            candidate: None,
        };
        filter
            .validate_integrity()
            .map_err(serde::de::Error::custom)?;
        Ok(filter)
    }
}

impl ZiFilter {
    /// Construct and validate a sampled-data filter.
    pub fn new(num: Vec<f64>, den: Vec<f64>, period: f64) -> Result<Self, ZiFilterError> {
        Self::new_with_timing(num, den, period, 0.0)
    }

    pub fn new_with_timing(
        num: Vec<f64>,
        den: Vec<f64>,
        period: f64,
        first_transition: f64,
    ) -> Result<Self, ZiFilterError> {
        validate_definition(&num, &den, period, first_transition)?;
        let x_len = num.len().saturating_sub(1);
        let y_len = den.len().saturating_sub(1);
        Ok(Self {
            definition_frozen: true,
            num,
            den,
            period,
            first_transition,
            x_hist: vec![0.0; x_len],
            y_hist: vec![0.0; y_len],
            held: 0.0,
            ramp_start_time: 0.0,
            ramp_end_time: 0.0,
            ramp_start_value: 0.0,
            next_sample_index: 0,
            accepted_time: None,
            transient_active: false,
            transient_seen: false,
            candidate: None,
        })
    }

    /// Validate all serialized definition, history, transition, and schedule
    /// invariants before this filter is admitted to runtime evaluation.
    ///
    /// Constructors and runtime mutation preserve these invariants. The
    /// explicit validator exists for artifact integrity checks, while the
    /// custom [`Deserialize`] implementation applies it automatically at
    /// every serde boundary.
    pub fn validate_integrity(&self) -> Result<(), ZiFilterError> {
        validate_definition(&self.num, &self.den, self.period, self.first_transition)?;

        if matches!(
            self.candidate,
            Some(Candidate::Hold | Candidate::Sample { .. })
        ) && !self.transient_seen
        {
            return Err(ZiFilterError::InvalidState(
                "a successful transient candidate requires a completed Zi evaluation".into(),
            ));
        }
        if self.transient_seen
            && self.accepted_time.is_none()
            && !self.transient_active
            && self.candidate.is_none()
        {
            return Err(ZiFilterError::InvalidState(
                "transient_seen requires accepted, candidate, or active Zi state".into(),
            ));
        }

        let expected_x = self.num.len().saturating_sub(1);
        if self.x_hist.len() != expected_x {
            return Err(ZiFilterError::InvalidState(format!(
                "input history length is {}, expected {expected_x} for {} numerator coefficients",
                self.x_hist.len(),
                self.num.len()
            )));
        }
        let expected_y = self.den.len().saturating_sub(1);
        if self.y_hist.len() != expected_y {
            return Err(ZiFilterError::InvalidState(format!(
                "output history length is {}, expected {expected_y} for {} denominator coefficients",
                self.y_hist.len(),
                self.den.len()
            )));
        }
        for (label, history) in [("input", &self.x_hist), ("output", &self.y_hist)] {
            if let Some((index, value)) = history
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                return Err(ZiFilterError::InvalidState(format!(
                    "{label} history value {index} must be finite, got {value}"
                )));
            }
        }
        for (label, value) in [
            ("held output", self.held),
            ("ramp start time", self.ramp_start_time),
            ("ramp end time", self.ramp_end_time),
            ("ramp start value", self.ramp_start_value),
        ] {
            if !value.is_finite() {
                return Err(ZiFilterError::InvalidState(format!(
                    "{label} must be finite, got {value}"
                )));
            }
        }
        if self.ramp_start_time < 0.0 || self.ramp_end_time < 0.0 {
            return Err(ZiFilterError::InvalidState(format!(
                "ramp times must be non-negative, got start {} and end {}",
                self.ramp_start_time, self.ramp_end_time
            )));
        }
        if self.ramp_end_time < self.ramp_start_time {
            return Err(ZiFilterError::InvalidState(format!(
                "ramp end time {} precedes ramp start time {}",
                self.ramp_end_time, self.ramp_start_time
            )));
        }

        if let Some(accepted) = self.accepted_time {
            if !accepted.is_finite() || accepted < 0.0 {
                return Err(ZiFilterError::InvalidState(format!(
                    "accepted time must be finite and non-negative, got {accepted}"
                )));
            }
            if !self.transient_active {
                return Err(ZiFilterError::InvalidState(
                    "an accepted transient time requires an active Zi state".into(),
                ));
            }
            if !self.transient_seen {
                return Err(ZiFilterError::InvalidState(
                    "an active Zi state must have completed a transient evaluation".into(),
                ));
            }
            if self.ramp_start_time > accepted {
                return Err(ZiFilterError::InvalidState(format!(
                    "ramp start time {} is later than accepted time {accepted}",
                    self.ramp_start_time
                )));
            }
            let next = self.next_sample_time().map_err(|error| {
                ZiFilterError::InvalidState(format!("next sample schedule is invalid: {error}"))
            })?;
            if next <= accepted {
                return Err(ZiFilterError::InvalidState(format!(
                    "next sample time {next} is not strictly after accepted time {accepted}"
                )));
            }
        } else {
            if self.transient_active {
                return Err(ZiFilterError::InvalidState(
                    "an active Zi state requires an accepted transient time".into(),
                ));
            }
            if self.next_sample_index != 0 {
                return Err(ZiFilterError::InvalidState(format!(
                    "sample index {} requires an accepted transient time",
                    self.next_sample_index
                )));
            }
            if self.x_hist.iter().any(|value| *value != 0.0)
                || self.y_hist.iter().any(|value| *value != 0.0)
                || self.held != 0.0
                || self.ramp_start_time != 0.0
                || self.ramp_end_time != 0.0
                || self.ramp_start_value != 0.0
            {
                return Err(ZiFilterError::InvalidState(
                    "history and ramp state must be zero before the first accepted transient point"
                        .into(),
                ));
            }
        }

        if !self.definition_frozen
            && (self.accepted_time.is_some()
                || self.next_sample_index != 0
                || self.transient_active
                || self.transient_seen
                || self.candidate.is_some()
                || self.x_hist.iter().any(|value| *value != 0.0)
                || self.y_hist.iter().any(|value| *value != 0.0)
                || self.held != 0.0
                || self.ramp_start_time != 0.0
                || self.ramp_end_time != 0.0
                || self.ramp_start_value != 0.0)
        {
            return Err(ZiFilterError::InvalidState(
                "an unfrozen Zi definition must retain reset analysis state".into(),
            ));
        }

        Ok(())
    }

    pub(crate) fn checkpoint(&self) -> ZiCheckpoint {
        ZiCheckpoint {
            definition_frozen: self.definition_frozen,
            num: self.num.clone(),
            den: self.den.clone(),
            period: self.period,
            first_transition: self.first_transition,
            x_hist: self.x_hist.clone(),
            y_hist: self.y_hist.clone(),
            held: self.held,
            ramp_start_time: self.ramp_start_time,
            ramp_end_time: self.ramp_end_time,
            ramp_start_value: self.ramp_start_value,
            next_sample_index: self.next_sample_index,
            accepted_time: self.accepted_time,
            transient_active: self.transient_active,
            transient_seen: self.transient_seen,
        }
    }

    pub(crate) fn validate_checkpoint_ready(&self) -> Result<(), ZiFilterError> {
        if self.candidate.is_some() {
            return Err(ZiFilterError::InvalidState(
                "Zi filter has an in-flight Newton candidate".into(),
            ));
        }
        self.validate_integrity()
    }

    pub(crate) fn validate_checkpoint(
        &self,
        checkpoint: &ZiCheckpoint,
    ) -> Result<(), ZiFilterError> {
        if checkpoint.num.len() != self.num.len() || checkpoint.den.len() != self.den.len() {
            return Err(ZiFilterError::InvalidState(format!(
                "checkpoint coefficient shape is ({}, {}), expected ({}, {})",
                checkpoint.num.len(),
                checkpoint.den.len(),
                self.num.len(),
                self.den.len()
            )));
        }
        Self::from_checkpoint(checkpoint.clone())?.validate_integrity()
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: &ZiCheckpoint) {
        *self = Self::from_checkpoint(checkpoint.clone())
            .expect("validated Zi checkpoint must remain constructible");
    }

    fn from_checkpoint(checkpoint: ZiCheckpoint) -> Result<Self, ZiFilterError> {
        let filter = Self {
            definition_frozen: checkpoint.definition_frozen,
            num: checkpoint.num,
            den: checkpoint.den,
            period: checkpoint.period,
            first_transition: checkpoint.first_transition,
            x_hist: checkpoint.x_hist,
            y_hist: checkpoint.y_hist,
            held: checkpoint.held,
            ramp_start_time: checkpoint.ramp_start_time,
            ramp_end_time: checkpoint.ramp_end_time,
            ramp_start_value: checkpoint.ramp_start_value,
            next_sample_index: checkpoint.next_sample_index,
            accepted_time: checkpoint.accepted_time,
            transient_active: checkpoint.transient_active,
            transient_seen: checkpoint.transient_seen,
            candidate: None,
        };
        filter.validate_integrity()?;
        Ok(filter)
    }

    /// DC gain H(1) = Σb / Σa.
    pub fn dc_gain(&self) -> Result<f64, ZiFilterError> {
        coefficient_dc_gain(&self.num, &self.den)
    }

    /// Instantaneous feedthrough b0/a0, used by an exact transient Jacobian
    /// at a sampling edge.
    pub fn feedthrough(&self) -> Result<f64, ZiFilterError> {
        checked_sum_products_ratio(
            [(self.num[0], 1.0)].into_iter(),
            [(self.den[0], 1.0)].into_iter(),
            "feedthrough gain",
            false,
        )
    }

    pub(crate) fn definition_is_frozen(&self) -> bool {
        self.definition_frozen
    }

    pub(crate) fn invalidate_definition(&mut self) {
        self.definition_frozen = false;
        self.reset_analysis();
    }

    /// Whether the latest evaluation produced a candidate sample.
    pub fn sampling_now(&self) -> bool {
        matches!(self.candidate, Some(Candidate::Sample { .. }))
    }

    /// Whether this filter must constrain the current transient schedule.
    /// Dormant analysis-specific slots have neither accepted activity nor a
    /// successful candidate from the latest device pass and must not impose
    /// placeholder or stale-definition breakpoints.
    pub(crate) fn participates_in_transient_schedule(&self) -> bool {
        self.transient_active
            || matches!(
                self.candidate,
                Some(Candidate::Hold | Candidate::Sample { .. })
            )
    }

    /// Start one complete device evaluation pass.
    ///
    /// A candidate is meaningful only if this state slot executes in the
    /// final Newton pass at the accepted timepoint. Clearing it here prevents
    /// an earlier pass from being committed when later control flow skips the
    /// operator. VAMS-2023 section 4.5.15 requires analog operators to be
    /// evaluated on every iteration and consequently forbids them in dynamic
    /// conditionals; this guard also fails closed if malformed IR violates
    /// that rule.
    pub(crate) fn begin_evaluation(&mut self) {
        self.candidate = None;
    }

    /// Clear all transient history for a newly started analysis while
    /// preserving the validated filter definition. VAMS-2023 section 4.5.15
    /// defines no analog-operator history before time zero; reusing a device
    /// for another analysis must therefore not inherit a prior run's hold,
    /// histories, schedule, or speculative candidate.
    pub(crate) fn reset_analysis(&mut self) {
        self.x_hist.fill(0.0);
        self.y_hist.fill(0.0);
        self.held = 0.0;
        self.ramp_start_time = 0.0;
        self.ramp_end_time = 0.0;
        self.ramp_start_value = 0.0;
        self.next_sample_index = 0;
        self.accepted_time = None;
        self.transient_active = false;
        self.transient_seen = false;
        self.candidate = None;
    }

    /// Evaluate at `time` with the present `input`. Repeated calls at the same
    /// timepoint recompute the candidate from committed history and are
    /// idempotent.
    pub fn eval(&mut self, input: f64, time: f64, transient: bool) -> Result<f64, ZiFilterError> {
        self.eval_with_transition(input, time, transient, 0.0)
    }

    pub fn eval_with_transition(
        &mut self,
        input: f64,
        time: f64,
        transient: bool,
        transition: f64,
    ) -> Result<f64, ZiFilterError> {
        self.eval_with_transition_constraint(input, time, transient, transition, false)
    }

    pub(crate) fn eval_with_transition_constraint(
        &mut self,
        input: f64,
        time: f64,
        transient: bool,
        transition: f64,
        direct_assignment: bool,
    ) -> Result<f64, ZiFilterError> {
        // Mark a transient attempt as failed until every fallible operation
        // completes and publishes a fresh candidate. This prevents an error
        // from leaving an older same-time sample eligible for commit.
        self.candidate = transient.then_some(Candidate::Failed);
        self.validate_runtime(input, time)?;
        validate_transition(transition)?;
        validate_direct_transition(transition, direct_assignment)?;
        if !transient {
            self.candidate = None;
            return coefficient_dc_action(&self.num, &self.den, input, "DC output");
        }

        match self.classify_time(time)? {
            SamplePosition::Before => {
                let output = self.visible_output(time)?;
                self.candidate = Some(Candidate::Hold);
                self.transient_seen = true;
                Ok(output)
            }
            SamplePosition::At => {
                let output = self.sample_output(input)?;
                let visible_start = self.visible_output(time)?;
                let ramp_end_time = validated_ramp_end(time, transition)?;
                self.candidate = Some(Candidate::Sample {
                    time,
                    input,
                    output,
                    transition,
                    ramp_end_time,
                    visible_start,
                });
                self.transient_seen = true;
                if transition == 0.0 {
                    Ok(output)
                } else {
                    Ok(visible_start)
                }
            }
        }
    }

    /// Exact Jacobian action for the current operating point: H(1) in an
    /// equilibrium analysis, b0/a0 on a sample edge, and zero while holding.
    pub fn eval_derivative(
        &self,
        operand_derivative: f64,
        time: f64,
        transient: bool,
        transition: f64,
    ) -> Result<f64, ZiFilterError> {
        self.eval_derivative_with_constraint(operand_derivative, time, transient, transition, false)
    }

    pub(crate) fn eval_derivative_with_constraint(
        &self,
        operand_derivative: f64,
        time: f64,
        transient: bool,
        transition: f64,
        direct_assignment: bool,
    ) -> Result<f64, ZiFilterError> {
        self.validate_runtime_readonly(operand_derivative, time)?;
        validate_transition(transition)?;
        validate_direct_transition(transition, direct_assignment)?;
        if !transient {
            return coefficient_dc_action(&self.num, &self.den, operand_derivative, "DC Jacobian");
        }
        match self.classify_time_readonly(time)? {
            SamplePosition::Before => Ok(0.0),
            SamplePosition::At => {
                if transition == 0.0 {
                    checked_sum_products_ratio(
                        [(self.num[0], operand_derivative)].into_iter(),
                        [(self.den[0], 1.0)].into_iter(),
                        "sample Jacobian",
                        false,
                    )
                } else {
                    Ok(0.0)
                }
            }
        }
    }

    /// Maximum positive step from `time` that lands on the next sample edge.
    /// When `time` is the current edge, the following edge is returned so a
    /// just-evaluated point never asks the solver for a zero timestep.
    pub fn next_sample_step_bound(&self, time: f64) -> Result<f64, ZiFilterError> {
        let target = self.next_event_time(time)?;
        let bound = target - time;
        if !bound.is_finite() || bound <= 0.0 {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "next sample edge {target} is not representably after time {time}"
            )));
        }
        Ok(bound)
    }

    /// Exact absolute time of the next sample edge or visible ramp corner.
    pub(crate) fn next_event_time(&self, time: f64) -> Result<f64, ZiFilterError> {
        if !time.is_finite() || time < 0.0 {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "sample breakpoint requested at invalid simulation time {time}"
            )));
        }
        let due = self.next_sample_time()?;
        let sample_target = match compare_sample_time(time, due, self.period) {
            SampleComparison::Before => due,
            SampleComparison::At => self.sample_time_after_next()?,
            SampleComparison::After => return Err(self.missed_sample_error(time, due)),
        };
        let ramp_target = self.next_ramp_corner_after(time);
        let target = ramp_target.map_or(sample_target, |corner| corner.min(sample_target));
        if !target.is_finite() || target <= time {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "next sample edge {target} is not representably after time {time}"
            )));
        }
        Ok(target)
    }

    /// Commit the latest in-flight candidate after the engine accepts the
    /// step ending at `time`.
    pub fn commit(&mut self, time: f64) -> Result<(), ZiFilterError> {
        self.validate_commit(time)?;
        self.apply_validated_commit(time);
        Ok(())
    }

    /// First phase of an atomic multi-filter commit. This performs every
    /// fallible check without changing accepted or candidate state.
    pub(crate) fn validate_commit(&self, time: f64) -> Result<(), ZiFilterError> {
        if !time.is_finite() || time < 0.0 {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "cannot commit invalid simulation time {time}"
            )));
        }
        if matches!(self.candidate, Some(Candidate::Failed)) {
            return Err(ZiFilterError::InvalidEvaluation(
                "cannot commit a zi state whose latest transient evaluation failed".into(),
            ));
        }
        if self.transient_active && self.candidate.is_none() {
            return Err(ZiFilterError::InvalidEvaluation(
                "cannot commit active zi state that was skipped by the final device pass".into(),
            ));
        }
        if !self.transient_active && !self.transient_seen && self.candidate.is_none() {
            // A statically dormant analysis-specific slot owns no transient
            // clock in this analysis. It must neither validate a stale
            // placeholder schedule nor acquire accepted state.
            return Ok(());
        }
        if let Some(accepted) = self.accepted_time {
            if time == accepted {
                return Err(ZiFilterError::InvalidEvaluation(format!(
                    "transient time {time} was already accepted; Zi accepted time must advance strictly"
                )));
            }
            if time < accepted {
                return Err(ZiFilterError::InvalidEvaluation(format!(
                    "accepted time moved backwards from {accepted} to {time}"
                )));
            }
        }

        let due = self.next_sample_time()?;
        match compare_sample_time(time, due, self.period) {
            SampleComparison::Before => {}
            SampleComparison::At => {
                let next_sample_index = self.next_sample_index.checked_add(1).ok_or_else(|| {
                    ZiFilterError::InvalidEvaluation(
                        "sample index exhausted the representable u64 range".into(),
                    )
                })?;
                let next = sample_time(next_sample_index, self.period, self.first_transition)?;
                if next <= due {
                    return Err(ZiFilterError::InvalidEvaluation(format!(
                        "sample period {} cannot advance representably beyond time {due}",
                        self.period
                    )));
                }
                match &self.candidate {
                    Some(Candidate::Sample {
                        time: candidate_time,
                        ..
                    }) => {
                        if candidate_time.to_bits() != time.to_bits() {
                            return Err(ZiFilterError::InvalidEvaluation(format!(
                                "sample candidate belongs to time {candidate_time}, not accepted time {time}"
                            )));
                        }
                    }
                    _ => {
                        return Err(ZiFilterError::InvalidEvaluation(format!(
                            "accepted sample edge {due} was not evaluated by active zi state in the final device pass"
                        )));
                    }
                }
            }
            SampleComparison::After => return Err(self.missed_sample_error(time, due)),
        }
        Ok(())
    }

    /// Second phase of a commit previously accepted by
    /// [`Self::validate_commit`]. No validation or allocation occurs here.
    pub(crate) fn apply_validated_commit(&mut self, time: f64) {
        let latest_pass_executed = matches!(
            self.candidate,
            Some(Candidate::Hold | Candidate::Sample { .. })
        );
        if !self.transient_active && !latest_pass_executed {
            // `validate_commit` has already distinguished a dormant slot from
            // a failed or skipped active operator. Dormant slots retain their
            // initial schedule and acquire no accepted time.
            self.candidate = None;
            return;
        }
        if latest_pass_executed {
            self.transient_active = true;
        }
        let due = self
            .next_sample_time()
            .expect("validated Zi next sample remains representable");
        if compare_sample_time(time, due, self.period) == SampleComparison::At {
            if let Some(Candidate::Sample {
                input,
                output,
                transition,
                ramp_end_time,
                visible_start,
                ..
            }) = self.candidate.take()
            {
                if !self.x_hist.is_empty() {
                    self.x_hist.rotate_right(1);
                    self.x_hist[0] = input;
                }
                if !self.y_hist.is_empty() {
                    self.y_hist.rotate_right(1);
                    self.y_hist[0] = output;
                }
                self.held = output;
                self.ramp_start_time = time;
                self.ramp_end_time = ramp_end_time;
                self.ramp_start_value = if transition == 0.0 {
                    output
                } else {
                    visible_start
                };
            }
            self.next_sample_index += 1;
        }
        self.accepted_time = Some(time);
        self.candidate = None;
    }

    fn validate_runtime(&self, input: f64, time: f64) -> Result<(), ZiFilterError> {
        validate_definition(&self.num, &self.den, self.period, self.first_transition)?;
        if !input.is_finite() {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "input must be finite, got {input}"
            )));
        }
        if !time.is_finite() || time < 0.0 {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "simulation time must be finite and non-negative, got {time}"
            )));
        }
        Ok(())
    }

    fn validate_runtime_readonly(&self, input: f64, time: f64) -> Result<(), ZiFilterError> {
        validate_definition(&self.num, &self.den, self.period, self.first_transition)?;
        if !input.is_finite() {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "input must be finite, got {input}"
            )));
        }
        if !time.is_finite() || time < 0.0 {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "simulation time must be finite and non-negative, got {time}"
            )));
        }
        Ok(())
    }

    fn classify_time(&mut self, time: f64) -> Result<SamplePosition, ZiFilterError> {
        let due = self.next_sample_time()?;
        match compare_sample_time(time, due, self.period) {
            SampleComparison::Before => Ok(SamplePosition::Before),
            SampleComparison::At => Ok(SamplePosition::At),
            SampleComparison::After => Err(self.missed_sample_error(time, due)),
        }
    }

    fn classify_time_readonly(&self, time: f64) -> Result<SamplePosition, ZiFilterError> {
        let due = self.next_sample_time()?;
        match compare_sample_time(time, due, self.period) {
            SampleComparison::Before => Ok(SamplePosition::Before),
            SampleComparison::At => Ok(SamplePosition::At),
            SampleComparison::After => Err(self.missed_sample_error(time, due)),
        }
    }

    fn visible_output(&self, time: f64) -> Result<f64, ZiFilterError> {
        if self.ramp_end_time <= self.ramp_start_time || time >= self.ramp_end_time {
            return Ok(self.held);
        }
        if time <= self.ramp_start_time {
            return Ok(self.ramp_start_value);
        }
        let fraction = (time - self.ramp_start_time) / (self.ramp_end_time - self.ramp_start_time);
        // Form the convex combination exactly before the single final
        // IEEE-754 rounding.  `start + f * (end - start)` overflows its
        // subtraction for opposite-sign finite endpoints such as ±MAX even
        // when the mathematical interpolant is finite.
        let output = checked_sum_products_binary(
            [
                (self.ramp_start_value, 1.0 - fraction),
                (self.held, fraction),
            ]
            .into_iter(),
            "Zi transition output",
        )?;
        if output.is_finite() {
            Ok(output)
        } else {
            Err(ZiFilterError::InvalidEvaluation(
                "non-finite output during Zi transition".into(),
            ))
        }
    }

    fn next_ramp_corner_after(&self, time: f64) -> Option<f64> {
        let candidate_end = match &self.candidate {
            Some(Candidate::Sample {
                transition,
                ramp_end_time,
                ..
            }) if *transition > 0.0 => Some(*ramp_end_time),
            _ => None,
        };
        [candidate_end, Some(self.ramp_end_time)]
            .into_iter()
            .flatten()
            .filter(|corner| corner.is_finite() && *corner > time)
            .reduce(f64::min)
    }

    fn sample_output(&self, input: f64) -> Result<f64, ZiFilterError> {
        let feedforward = self.num.iter().enumerate().map(|(index, coefficient)| {
            let value = if index == 0 {
                input
            } else {
                self.x_hist[index - 1]
            };
            (*coefficient, value)
        });
        let feedback = self
            .den
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, coefficient)| (-*coefficient, self.y_hist[index - 1]));
        checked_sum_products_ratio(
            feedforward.chain(feedback),
            [(self.den[0], 1.0)].into_iter(),
            "difference-equation output",
            false,
        )
    }

    fn next_sample_time(&self) -> Result<f64, ZiFilterError> {
        sample_time(self.next_sample_index, self.period, self.first_transition)
    }

    fn sample_time_after_next(&self) -> Result<f64, ZiFilterError> {
        let index = self.next_sample_index.checked_add(1).ok_or_else(|| {
            ZiFilterError::InvalidEvaluation("sample index exhausted the u64 range".into())
        })?;
        let current = self.next_sample_time()?;
        let next = sample_time(index, self.period, self.first_transition)?;
        if next <= current {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "sample period {} cannot advance representably beyond time {current}",
                self.period
            )));
        }
        Ok(next)
    }

    fn missed_sample_error(&self, time: f64, due: f64) -> ZiFilterError {
        let missed = ((time - due) / self.period).floor().max(0.0) + 1.0;
        ZiFilterError::InvalidEvaluation(format!(
            "step to time {time} crossed the required sample edge at {due} (at least {missed:.0} sample instant(s)); limit the step to the reported zi breakpoint"
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SamplePosition {
    Before,
    At,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SampleComparison {
    Before,
    At,
    After,
}

fn compare_sample_time(time: f64, due: f64, period: f64) -> SampleComparison {
    let tolerance = time_tolerance(time, due, period);
    let delta = time - due;
    if delta.abs() <= tolerance {
        SampleComparison::At
    } else if delta < 0.0 {
        SampleComparison::Before
    } else {
        SampleComparison::After
    }
}

fn time_tolerance(time: f64, due: f64, period: f64) -> f64 {
    let ulps = ulp(time).max(ulp(due)) * TIME_ULPS;
    let cap = period * 0.25;
    if cap == 0.0 { 0.0 } else { ulps.min(cap) }
}

fn ulp(value: f64) -> f64 {
    if value == 0.0 {
        return f64::from_bits(1);
    }
    let bits = value.to_bits();
    if value == f64::MAX {
        return value - f64::from_bits(bits - 1);
    }
    let next = f64::from_bits(bits + 1);
    next - value
}

fn sample_time(index: u64, period: f64, first_transition: f64) -> Result<f64, ZiFilterError> {
    let time = first_transition + (index as f64) * period;
    if !time.is_finite() {
        return Err(ZiFilterError::InvalidEvaluation(format!(
            "sample index {index} times period {period} is outside the finite f64 time range"
        )));
    }
    Ok(time)
}

fn validate_definition(
    num: &[f64],
    den: &[f64],
    period: f64,
    first_transition: f64,
) -> Result<(), ZiFilterError> {
    if num.is_empty() {
        return Err(ZiFilterError::InvalidDefinition(
            "numerator coefficient vector must not be empty".into(),
        ));
    }
    if den.is_empty() {
        return Err(ZiFilterError::InvalidDefinition(
            "denominator coefficient vector must not be empty".into(),
        ));
    }
    validate_zi_runtime_operand_budget("Zi coefficient filter", num.len(), den.len())?;
    for (label, coefficients) in [("numerator", num), ("denominator", den)] {
        if let Some((index, coefficient)) = coefficients
            .iter()
            .enumerate()
            .find(|(_, coefficient)| !coefficient.is_finite())
        {
            return Err(ZiFilterError::InvalidDefinition(format!(
                "{label} coefficient {index} must be finite, got {coefficient}"
            )));
        }
    }
    if den[0] == 0.0 {
        return Err(ZiFilterError::InvalidDefinition(
            "leading denominator coefficient a0 must be nonzero".into(),
        ));
    }
    if !period.is_finite() || period <= 0.0 {
        return Err(ZiFilterError::InvalidDefinition(format!(
            "sample period must be finite and greater than zero, got {period}"
        )));
    }
    if !first_transition.is_finite() || first_transition < 0.0 {
        return Err(ZiFilterError::InvalidDefinition(format!(
            "first transition time must be finite and non-negative, got {first_transition}"
        )));
    }
    Ok(())
}

fn validate_transition(transition: f64) -> Result<(), ZiFilterError> {
    if transition.is_finite() && transition >= 0.0 {
        Ok(())
    } else {
        Err(ZiFilterError::InvalidEvaluation(format!(
            "transition time must be finite and non-negative, got {transition}"
        )))
    }
}

fn validate_direct_transition(
    transition: f64,
    direct_assignment: bool,
) -> Result<(), ZiFilterError> {
    if direct_assignment && transition == 0.0 {
        Err(ZiFilterError::InvalidEvaluation(
            "zero-transition Zi output cannot be assigned directly to an analog branch (VAMS-2023 section 4.5.12)".into(),
        ))
    } else {
        Ok(())
    }
}

const fn definition_frozen_default() -> bool {
    true
}

fn validated_ramp_end(time: f64, transition: f64) -> Result<f64, ZiFilterError> {
    if transition == 0.0 {
        return Ok(time);
    }
    let end = time + transition;
    if !end.is_finite() {
        return Err(ZiFilterError::InvalidEvaluation(format!(
            "transition from time {time} by {transition} overflows the finite simulation-time range"
        )));
    }
    if end <= time {
        return Err(ZiFilterError::InvalidEvaluation(format!(
            "positive transition {transition} is not representably after simulation time {time}"
        )));
    }
    Ok(end)
}

/// Return the exact representable DC gain for coefficient-form zi filters.
pub fn coefficient_dc_gain(num: &[f64], den: &[f64]) -> Result<f64, ZiFilterError> {
    if num.is_empty() || den.is_empty() {
        return Err(ZiFilterError::InvalidDefinition(
            "DC gain requires non-empty numerator and denominator vectors".into(),
        ));
    }
    if num.iter().chain(den).any(|value| !value.is_finite()) {
        return Err(ZiFilterError::InvalidDefinition(
            "DC gain coefficients must all be finite".into(),
        ));
    }
    checked_sum_products_ratio(
        num.iter().copied().map(|value| (value, 1.0)),
        den.iter().copied().map(|value| (value, 1.0)),
        "DC gain",
        true,
    )
}

/// Apply a coefficient-form DC transfer ratio to one operand with a single
/// final IEEE-754 rounding. Computing and rounding H(1) first can underflow or
/// overflow even when the complete action remains finite.
fn coefficient_dc_action(
    num: &[f64],
    den: &[f64],
    operand: f64,
    context: &str,
) -> Result<f64, ZiFilterError> {
    checked_sum_products_ratio(
        num.iter()
            .copied()
            .map(|coefficient| (coefficient, operand)),
        den.iter().copied().map(|value| (value, 1.0)),
        context,
        true,
    )
}

/// Allocation-free exact representation for the common case where a sum of
/// binary64 products fits a normalized signed 128-bit integer. Unsupported
/// exponent spans or carries fall back to `BigMagnitude`; they never round in
/// this representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SmallExact {
    value: i128,
    exponent: i32,
}

impl SmallExact {
    const ZERO: Self = Self {
        value: 0,
        exponent: 0,
    };

    fn normalized(value: i128, exponent: i32) -> Option<Self> {
        if value == 0 {
            return Some(Self::ZERO);
        }
        let trailing = value.unsigned_abs().trailing_zeros();
        Some(Self {
            value: value >> trailing,
            exponent: exponent.checked_add(trailing as i32)?,
        })
    }

    fn product(left: f64, right: f64) -> Option<Self> {
        let (left_negative, left_mantissa, left_exponent) = exact_f64_parts(left);
        let (right_negative, right_mantissa, right_exponent) = exact_f64_parts(right);
        let magnitude = u128::from(left_mantissa) * u128::from(right_mantissa);
        let magnitude = i128::try_from(magnitude).ok()?;
        let value = if left_negative ^ right_negative {
            magnitude.checked_neg()?
        } else {
            magnitude
        };
        Self::normalized(value, left_exponent.checked_add(right_exponent)?)
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        if self.value == 0 {
            return Some(other);
        }
        if other.value == 0 {
            return Some(self);
        }
        let common = self.exponent.min(other.exponent);
        let left_shift = u32::try_from(self.exponent.checked_sub(common)?).ok()?;
        let right_shift = u32::try_from(other.exponent.checked_sub(common)?).ok()?;
        let left = checked_scale_i128(self.value, left_shift)?;
        let right = checked_scale_i128(other.value, right_shift)?;
        Self::normalized(left.checked_add(right)?, common)
    }

    fn signed_parts(self) -> (bool, u128, i32) {
        (self.value < 0, self.value.unsigned_abs(), self.exponent)
    }
}

fn checked_scale_i128(value: i128, shift: u32) -> Option<i128> {
    if value == 0 {
        return Some(0);
    }
    if shift >= 127 {
        return None;
    }
    value.checked_mul(1_i128 << shift)
}

fn checked_shift_u128(value: u128, shift: u32) -> Option<u128> {
    if value == 0 {
        return Some(0);
    }
    if shift >= u128::BITS || shift > value.leading_zeros() {
        return None;
    }
    Some(value << shift)
}

fn small_sum_products(
    terms: impl IntoIterator<Item = (f64, f64)>,
) -> Result<Option<SmallExact>, ZiFilterError> {
    let mut accumulator = SmallExact::ZERO;
    for (left, right) in terms {
        if !left.is_finite() || !right.is_finite() {
            return Err(ZiFilterError::InvalidEvaluation(
                "non-finite term in sampled-filter arithmetic".into(),
            ));
        }
        if left == 0.0 || right == 0.0 {
            continue;
        }
        let Some(term) = SmallExact::product(left, right) else {
            return Ok(None);
        };
        let Some(sum) = accumulator.checked_add(term) else {
            return Ok(None);
        };
        accumulator = sum;
    }
    Ok(Some(accumulator))
}

fn rounded_small_ratio(numerator: u128, denominator: u128, binary_scale: i32) -> Option<u64> {
    let (scaled_numerator, scaled_denominator) = if binary_scale >= 0 {
        (
            checked_shift_u128(numerator, binary_scale as u32)?,
            denominator,
        )
    } else {
        (
            numerator,
            checked_shift_u128(denominator, binary_scale.unsigned_abs())?,
        )
    };
    let quotient = scaled_numerator / scaled_denominator;
    let remainder = scaled_numerator % scaled_denominator;
    let complement = scaled_denominator - remainder;
    let round_up = remainder > complement || (remainder == complement && quotient & 1 != 0);
    let rounded = quotient.checked_add(u128::from(round_up))?;
    u64::try_from(rounded).ok()
}

fn small_exact_ratio_to_f64(
    numerator: SmallExact,
    denominator: SmallExact,
    context: &str,
) -> Option<Result<f64, ZiFilterError>> {
    if denominator.value == 0 {
        return None;
    }
    if numerator.value == 0 {
        return Some(Ok(0.0));
    }
    let (numerator_negative, numerator, numerator_exponent) = numerator.signed_parts();
    let (denominator_negative, denominator, denominator_exponent) = denominator.signed_parts();
    let numerator_top = (u128::BITS - 1 - numerator.leading_zeros()) as i32;
    let denominator_top = (u128::BITS - 1 - denominator.leading_zeros()) as i32;
    let mut ratio_exponent = numerator_top - denominator_top;
    let below_candidate = if ratio_exponent >= 0 {
        let shifted = checked_shift_u128(denominator, ratio_exponent as u32)?;
        numerator < shifted
    } else {
        let shifted = checked_shift_u128(numerator, ratio_exponent.unsigned_abs())?;
        shifted < denominator
    };
    if below_candidate {
        ratio_exponent -= 1;
    }
    let binary_exponent = numerator_exponent.checked_sub(denominator_exponent)?;
    let unbiased = binary_exponent.checked_add(ratio_exponent)?;
    let scale = if unbiased < -1022 {
        binary_exponent.checked_add(1074)?
    } else {
        52_i32.checked_sub(ratio_exponent)?
    };
    let mut quotient = rounded_small_ratio(numerator, denominator, scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent = output_exponent.checked_add(1)?;
    }
    if unbiased < -1022 && quotient > 1_u64 << 52 {
        return None;
    }
    Some(encode_f64(
        quotient,
        output_exponent,
        numerator_negative ^ denominator_negative,
        context,
    ))
}

#[cfg(test)]
static SMALL_EXACT_FAST_PATH_HITS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
static BIG_EXACT_FALLBACK_HITS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

fn record_small_exact_fast_path() {
    #[cfg(test)]
    SMALL_EXACT_FAST_PATH_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

fn record_big_exact_fallback() {
    #[cfg(test)]
    BIG_EXACT_FALLBACK_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

fn checked_sum_products_ratio<I, D>(
    numerator_terms: I,
    denominator_terms: D,
    context: &str,
    singular_dc: bool,
) -> Result<f64, ZiFilterError>
where
    I: Iterator<Item = (f64, f64)> + Clone,
    D: Iterator<Item = (f64, f64)> + Clone,
{
    let small_numerator = small_sum_products(numerator_terms.clone())?;
    let small_denominator = small_sum_products(denominator_terms.clone())?;
    if let (Some(numerator), Some(denominator)) = (small_numerator, small_denominator) {
        if denominator.value == 0 {
            return if singular_dc {
                Err(ZiFilterError::SingularDc)
            } else {
                Err(ZiFilterError::InvalidEvaluation(format!(
                    "{context} has a zero denominator"
                )))
            };
        }
        if let Some(result) = small_exact_ratio_to_f64(numerator, denominator, context) {
            record_small_exact_fast_path();
            return result;
        }
    }

    record_big_exact_fallback();
    let numerator = scaled_sum_products(numerator_terms)?;
    let denominator = scaled_sum_products(denominator_terms)?;
    if singular_dc && denominator.magnitude.is_zero() {
        return Err(ZiFilterError::SingularDc);
    }
    checked_exact_ratio(numerator, denominator, context)
}

fn checked_sum_products_binary<I>(terms: I, context: &str) -> Result<f64, ZiFilterError>
where
    I: Iterator<Item = (f64, f64)> + Clone,
{
    if let Some(exact) = small_sum_products(terms.clone())? {
        if let Some(result) = small_exact_ratio_to_f64(
            exact,
            SmallExact {
                value: 1,
                exponent: 0,
            },
            context,
        ) {
            record_small_exact_fast_path();
            return result;
        }
    }

    record_big_exact_fallback();
    let exact = scaled_sum_products(terms)?;
    exact_binary_to_f64(&exact.magnitude, exact.negative, -2148, context)
}

#[derive(Debug, Clone)]
struct ExactValue {
    negative: bool,
    magnitude: BigMagnitude,
}

fn scaled_sum_products(
    terms: impl IntoIterator<Item = (f64, f64)>,
) -> Result<ExactValue, ZiFilterError> {
    // Every finite f64 is an integer mantissa times a power of two. Products
    // therefore fit an exact signed integer accumulator when all exponents are
    // referred to the smallest possible product exponent (-1074 * 2). Keeping
    // positive and negative magnitudes separately prevents a huge pair from
    // erasing a representable residual before cancellation.
    const PRODUCT_EXPONENT_FLOOR: i32 = -2148;
    let mut positive = BigMagnitude::default();
    let mut negative = BigMagnitude::default();
    for (left, right) in terms {
        if !left.is_finite() || !right.is_finite() {
            return Err(ZiFilterError::InvalidEvaluation(
                "non-finite term in sampled-filter arithmetic".into(),
            ));
        }
        if left == 0.0 || right == 0.0 {
            continue;
        }
        let (left_negative, left_mantissa, left_exponent) = exact_f64_parts(left);
        let (right_negative, right_mantissa, right_exponent) = exact_f64_parts(right);
        let product = u128::from(left_mantissa) * u128::from(right_mantissa);
        let shift = left_exponent + right_exponent - PRODUCT_EXPONENT_FLOOR;
        debug_assert!(shift >= 0);
        if left_negative ^ right_negative {
            negative.add_shifted(product, shift as usize);
        } else {
            positive.add_shifted(product, shift as usize);
        }
    }
    let (negative_result, magnitude) = match positive.compare(&negative) {
        std::cmp::Ordering::Greater => (false, positive.subtract(&negative)),
        std::cmp::Ordering::Less => (true, negative.subtract(&positive)),
        std::cmp::Ordering::Equal => {
            return Ok(ExactValue {
                negative: false,
                magnitude: BigMagnitude::default(),
            });
        }
    };
    Ok(ExactValue {
        negative: negative_result,
        magnitude,
    })
}

/// Exact IEEE-754 decomposition `value = (-1)^sign * mantissa * 2^exponent`.
fn exact_f64_parts(value: f64) -> (bool, u64, i32) {
    debug_assert!(value.is_finite() && value != 0.0);
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let biased_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    if biased_exponent == 0 {
        (negative, fraction, -1074)
    } else {
        (
            negative,
            (1_u64 << 52) | fraction,
            biased_exponent - 1023 - 52,
        )
    }
}

const INLINE_ACCUMULATOR_LIMBS: usize = 68;

#[derive(Debug, Clone)]
struct BigMagnitude {
    // 4352 inline bits cover the entire 4196-bit finite-f64 product span plus
    // 156 carry bits. Normal filter orders therefore do not allocate in the
    // Newton loop. Pathological coefficient counts grow explicitly rather
    // than losing arithmetic information.
    inline: [u64; INLINE_ACCUMULATOR_LIMBS],
    extra: Vec<u64>,
    /// One past the highest nonzero limb. This is maintained on every write,
    /// so hot arithmetic visits only the active span instead of rescanning
    /// all 68 inline limbs for every comparison and conversion.
    active_len: usize,
}

impl Default for BigMagnitude {
    fn default() -> Self {
        Self {
            inline: [0; INLINE_ACCUMULATOR_LIMBS],
            extra: Vec::new(),
            active_len: 0,
        }
    }
}

impl BigMagnitude {
    fn add_shifted(&mut self, value: u128, shift: usize) {
        let limb = shift / 64;
        let intra = shift % 64;
        let low = value as u64;
        let high = (value >> 64) as u64;
        if intra == 0 {
            self.add_word(limb, low);
            self.add_word(limb + 1, high);
        } else {
            self.add_word(limb, low << intra);
            self.add_word(limb + 1, low >> (64 - intra));
            self.add_word(limb + 1, high << intra);
            self.add_word(limb + 2, high >> (64 - intra));
        }
    }

    fn add_word(&mut self, mut index: usize, word: u64) {
        if word == 0 {
            return;
        }
        let (sum, mut carry) = self.word(index).overflowing_add(word);
        self.set_word(index, sum);
        while carry {
            index += 1;
            let (sum, next_carry) = self.word(index).overflowing_add(1);
            self.set_word(index, sum);
            carry = next_carry;
        }
    }

    fn word(&self, index: usize) -> u64 {
        if index < INLINE_ACCUMULATOR_LIMBS {
            self.inline[index]
        } else {
            self.extra
                .get(index - INLINE_ACCUMULATOR_LIMBS)
                .copied()
                .unwrap_or(0)
        }
    }

    fn set_word(&mut self, index: usize, value: u64) {
        if index < INLINE_ACCUMULATOR_LIMBS {
            self.inline[index] = value;
        } else {
            let extra_index = index - INLINE_ACCUMULATOR_LIMBS;
            if self.extra.len() <= extra_index {
                if value == 0 {
                    return;
                }
                self.extra.resize(extra_index + 1, 0);
            }
            self.extra[extra_index] = value;
        }
        if value != 0 {
            self.active_len = self.active_len.max(index + 1);
        } else if index + 1 == self.active_len {
            while self.active_len != 0 && self.word(self.active_len - 1) == 0 {
                self.active_len -= 1;
            }
        }
    }

    fn significant_len(&self) -> usize {
        self.active_len
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let self_len = self.significant_len();
        let other_len = other.significant_len();
        self_len.cmp(&other_len).then_with(|| {
            (0..self_len)
                .rev()
                .find_map(|index| {
                    let ordering = self.word(index).cmp(&other.word(index));
                    (ordering != std::cmp::Ordering::Equal).then_some(ordering)
                })
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Exact unsigned subtraction. `self` must be at least `other`.
    fn subtract(&self, other: &Self) -> Self {
        debug_assert!(self.compare(other) != std::cmp::Ordering::Less);
        let mut result = Self::default();
        let mut borrow = false;
        for index in 0..self.significant_len() {
            let left = self.word(index);
            let right = other.word(index);
            let (difference, first_borrow) = left.overflowing_sub(right);
            let (difference, second_borrow) = difference.overflowing_sub(u64::from(borrow));
            result.set_word(index, difference);
            borrow = first_borrow || second_borrow;
        }
        debug_assert!(!borrow);
        while result.extra.last() == Some(&0) {
            result.extra.pop();
        }
        result
    }

    fn is_zero(&self) -> bool {
        self.significant_len() == 0
    }

    fn one() -> Self {
        let mut value = Self::default();
        value.set_word(0, 1);
        value
    }

    fn top_bit(&self) -> Option<usize> {
        let significant_len = self.significant_len();
        (significant_len != 0).then(|| {
            let top_limb = self.word(significant_len - 1);
            (significant_len - 1) * 64 + (u64::BITS - 1 - top_limb.leading_zeros()) as usize
        })
    }

    fn shift_left(&self, shift: usize) -> Self {
        if self.is_zero() || shift == 0 {
            return self.clone();
        }
        let limb_shift = shift / 64;
        let intra = shift % 64;
        let mut result = Self::default();
        for source in 0..self.significant_len() {
            let word = self.word(source);
            let target = source + limb_shift;
            if intra == 0 {
                result.set_word(target, word);
            } else {
                result.add_word(target, word << intra);
                result.add_word(target + 1, word >> (64 - intra));
            }
        }
        result
    }
}

fn checked_exact_ratio(
    numerator: ExactValue,
    denominator: ExactValue,
    context: &str,
) -> Result<f64, ZiFilterError> {
    if denominator.magnitude.is_zero() {
        return Err(ZiFilterError::InvalidEvaluation(format!(
            "{context} has a zero denominator"
        )));
    }
    if numerator.magnitude.is_zero() {
        return Ok(0.0);
    }
    exact_ratio_to_f64(
        &numerator.magnitude,
        &denominator.magnitude,
        numerator.negative ^ denominator.negative,
        context,
    )
}

#[cfg(test)]
fn checked_product(left: f64, right: f64, context: &str) -> Result<f64, ZiFilterError> {
    let exact = scaled_sum_products([(left, right)])?;
    exact_binary_to_f64(&exact.magnitude, exact.negative, -2148, context)
}

fn exact_binary_to_f64(
    magnitude: &BigMagnitude,
    negative: bool,
    exponent_floor: i32,
    context: &str,
) -> Result<f64, ZiFilterError> {
    let Some(top_bit) = magnitude.top_bit() else {
        return Ok(0.0);
    };
    let unbiased = exponent_floor + top_bit as i32;
    let scale = if unbiased < -1022 {
        exponent_floor + 1074
    } else {
        52 - top_bit as i32
    };
    let mut quotient = rounded_scaled_ratio(magnitude, &BigMagnitude::one(), scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent += 1;
    }
    encode_f64(quotient, output_exponent, negative, context)
}

fn exact_ratio_to_f64(
    numerator: &BigMagnitude,
    denominator: &BigMagnitude,
    negative: bool,
    context: &str,
) -> Result<f64, ZiFilterError> {
    let numerator_top = numerator.top_bit().expect("nonzero numerator");
    let denominator_top = denominator.top_bit().expect("nonzero denominator");
    let mut unbiased = numerator_top as i32 - denominator_top as i32;
    let below_candidate = if unbiased >= 0 {
        numerator.compare(&denominator.shift_left(unbiased as usize)) == std::cmp::Ordering::Less
    } else {
        numerator
            .shift_left((-unbiased) as usize)
            .compare(denominator)
            == std::cmp::Ordering::Less
    };
    if below_candidate {
        unbiased -= 1;
    }
    let scale = if unbiased < -1022 {
        1074
    } else {
        52 - unbiased
    };
    let mut quotient = rounded_scaled_ratio(numerator, denominator, scale)?;
    let mut output_exponent = unbiased;
    if unbiased >= -1022 && quotient == 1_u64 << 53 {
        quotient >>= 1;
        output_exponent += 1;
    }
    encode_f64(quotient, output_exponent, negative, context)
}

fn rounded_scaled_ratio(
    numerator: &BigMagnitude,
    denominator: &BigMagnitude,
    binary_scale: i32,
) -> Result<u64, ZiFilterError> {
    let (mut remainder, divisor) = if binary_scale >= 0 {
        (
            numerator.shift_left(binary_scale as usize),
            denominator.clone(),
        )
    } else {
        (
            numerator.clone(),
            denominator.shift_left((-binary_scale) as usize),
        )
    };

    let mut quotient = 0_u64;
    while remainder.compare(&divisor) != std::cmp::Ordering::Less {
        let mut shift = remainder.top_bit().expect("nonzero remainder")
            - divisor.top_bit().expect("nonzero divisor");
        let mut shifted = divisor.shift_left(shift);
        if shifted.compare(&remainder) == std::cmp::Ordering::Greater {
            shift -= 1;
            shifted = divisor.shift_left(shift);
        }
        if shift >= u64::BITS as usize {
            return Err(ZiFilterError::InvalidEvaluation(
                "sampled-filter exact quotient exceeds internal f64 mantissa bounds".into(),
            ));
        }
        quotient |= 1_u64 << shift;
        remainder = remainder.subtract(&shifted);
    }

    let twice_remainder = remainder.shift_left(1);
    let ordering = twice_remainder.compare(&divisor);
    if ordering == std::cmp::Ordering::Greater
        || (ordering == std::cmp::Ordering::Equal && quotient & 1 != 0)
    {
        quotient = quotient.checked_add(1).ok_or_else(|| {
            ZiFilterError::InvalidEvaluation(
                "sampled-filter exact rounding overflowed its f64 mantissa".into(),
            )
        })?;
    }
    Ok(quotient)
}

fn encode_f64(
    quotient: u64,
    unbiased_exponent: i32,
    negative: bool,
    context: &str,
) -> Result<f64, ZiFilterError> {
    let sign = u64::from(negative) << 63;
    if quotient == 0 {
        return Ok(f64::from_bits(sign));
    }
    let bits = if unbiased_exponent < -1022 {
        if quotient >= 1_u64 << 52 {
            sign | (1_u64 << 52)
        } else {
            sign | quotient
        }
    } else {
        if unbiased_exponent > 1023 || !((1_u64 << 52)..(1_u64 << 53)).contains(&quotient) {
            return Err(ZiFilterError::InvalidEvaluation(format!(
                "{context} is outside the finite f64 range"
            )));
        }
        sign | ((unbiased_exponent + 1023) as u64) << 52 | (quotient - (1_u64 << 52))
    };
    Ok(f64::from_bits(bits))
}

/// Expand z-plane roots into z⁻¹-ascending polynomial coefficients:
/// Π(z - r_k) gives ascending-in-z coefficients, which reverse into the
/// difference-equation form (highest power of z becomes z⁰).
pub fn z_roots_to_coefficients(roots: &[(f64, f64)]) -> Result<Vec<f64>, String> {
    let ascending_in_z = crate::laplace::roots_to_polynomial(roots)?;
    let mut coefficients = ascending_in_z;
    coefficients.reverse();
    Ok(coefficients)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted_ramping_filter() -> ZiFilter {
        let mut filter = ZiFilter::new(vec![1.0, 0.25], vec![1.0, -0.5], 1.0).unwrap();
        filter.eval_with_transition(2.0, 0.0, true, 0.5).unwrap();
        filter.commit(0.0).unwrap();
        filter
    }

    fn assert_nonfinite_json_is_rejected(filter: &ZiFilter) {
        if let Ok(encoded) = serde_json::to_value(filter) {
            assert!(
                serde_json::from_value::<ZiFilter>(encoded).is_err(),
                "JSON containing a non-finite Zi state must not deserialize"
            );
        }
    }

    #[test]
    fn serde_round_trip_preserves_accepted_state_and_discards_candidate() {
        let mut filter = accepted_ramping_filter();
        filter.eval_with_transition(9.0, 0.25, true, 0.5).unwrap();
        assert!(matches!(filter.candidate, Some(Candidate::Hold)));
        filter
            .eval_with_transition(f64::NAN, 0.25, true, 0.5)
            .expect_err("failed speculative evaluation marks the candidate failed");
        assert!(matches!(filter.candidate, Some(Candidate::Failed)));

        let encoded = serde_json::to_string(&filter).expect("serialize valid Zi state");
        let restored: ZiFilter =
            serde_json::from_str(&encoded).expect("deserialize valid Zi state");

        restored.validate_integrity().unwrap();
        assert!(restored.candidate.is_none());
        assert_eq!(restored.x_hist, filter.x_hist);
        assert_eq!(restored.y_hist, filter.y_hist);
        assert_eq!(restored.held.to_bits(), filter.held.to_bits());
        assert_eq!(
            restored.ramp_start_time.to_bits(),
            filter.ramp_start_time.to_bits()
        );
        assert_eq!(
            restored.ramp_end_time.to_bits(),
            filter.ramp_end_time.to_bits()
        );
        assert_eq!(restored.next_sample_index, filter.next_sample_index);
        assert_eq!(restored.accepted_time, filter.accepted_time);
        assert_eq!(restored.transient_active, filter.transient_active);
        assert_eq!(restored.transient_seen, filter.transient_seen);
    }

    #[test]
    fn serde_rejects_malformed_history_lengths_before_evaluation() {
        let encoded =
            serde_json::to_value(accepted_ramping_filter()).expect("serialize valid Zi state");
        for (field, expected_message) in [
            ("x_hist", "input history length"),
            ("y_hist", "output history length"),
        ] {
            let mut malformed = encoded.clone();
            malformed[field] = serde_json::json!([]);
            let error = serde_json::from_value::<ZiFilter>(malformed)
                .expect_err("malformed Zi history shape must fail deserialization");
            assert!(error.to_string().contains(expected_message), "got: {error}");
        }
    }

    #[test]
    fn serde_rejects_invalid_accepted_and_ramp_times() {
        let encoded =
            serde_json::to_value(accepted_ramping_filter()).expect("serialize valid Zi state");

        let mut negative_accepted = encoded.clone();
        negative_accepted["accepted_time"] = serde_json::json!(-1.0);
        let error = serde_json::from_value::<ZiFilter>(negative_accepted)
            .expect_err("negative accepted time must fail deserialization");
        assert!(error.to_string().contains("accepted time"), "got: {error}");

        let mut reversed_ramp = encoded;
        reversed_ramp["ramp_start_time"] = serde_json::json!(2.0);
        reversed_ramp["ramp_end_time"] = serde_json::json!(1.0);
        let error = serde_json::from_value::<ZiFilter>(reversed_ramp)
            .expect_err("reversed ramp interval must fail deserialization");
        assert!(
            error.to_string().contains("precedes ramp start"),
            "got: {error}"
        );
    }

    #[test]
    fn serde_enforces_the_shared_operand_ceiling() {
        let boundary = ZiFilter::new(vec![1.0; 1019], vec![1.0], 1.0)
            .expect("1,024 flattened operands are supported");
        let encoded = serde_json::to_string(&boundary).expect("serialize boundary Zi filter");
        serde_json::from_str::<ZiFilter>(&encoded)
            .expect("deserialize boundary Zi filter through checked wire format");

        let mut over_limit =
            serde_json::to_value(ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap())
                .expect("serialize seed Zi filter");
        over_limit["num"] = serde_json::to_value(vec![1.0; 1020]).unwrap();
        let error = serde_json::from_value::<ZiFilter>(over_limit)
            .expect_err("an over-budget serialized Zi definition must fail closed");
        assert!(
            error.to_string().contains("platform-uniform maximum 1024"),
            "got: {error}"
        );
    }

    #[test]
    fn serde_rejects_seen_state_without_accepted_candidate_or_active_ownership() {
        let filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        let mut impossible = serde_json::to_value(filter).expect("serialize reset Zi filter");
        impossible["transient_seen"] = serde_json::json!(true);
        let error = serde_json::from_value::<ZiFilter>(impossible)
            .expect_err("transient_seen alone cannot reconstruct a valid lifecycle state");
        assert!(
            error
                .to_string()
                .contains("transient_seen requires accepted, candidate, or active Zi state"),
            "got: {error}"
        );

        let mut pending = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        pending.eval(2.0, 0.0, true).unwrap();
        pending
            .validate_integrity()
            .expect("a live successful candidate owns the seen state");
        let encoded = serde_json::to_string(&pending).expect("serialize pending Zi filter");
        let error = serde_json::from_str::<ZiFilter>(&encoded)
            .expect_err("a skipped candidate cannot be reconstructed from a checkpoint");
        assert!(error.to_string().contains("transient_seen"), "got: {error}");
    }

    #[test]
    fn nonfinite_history_accepted_and_ramp_state_fail_integrity_and_json() {
        let mut history = accepted_ramping_filter();
        history.x_hist[0] = f64::NAN;
        assert!(matches!(
            history.validate_integrity(),
            Err(ZiFilterError::InvalidState(message)) if message.contains("input history")
        ));
        assert_nonfinite_json_is_rejected(&history);

        let mut accepted = accepted_ramping_filter();
        accepted.accepted_time = Some(f64::INFINITY);
        assert!(matches!(
            accepted.validate_integrity(),
            Err(ZiFilterError::InvalidState(message)) if message.contains("accepted time")
        ));
        assert_nonfinite_json_is_rejected(&accepted);

        let mut ramp = accepted_ramping_filter();
        ramp.ramp_end_time = f64::INFINITY;
        assert!(matches!(
            ramp.validate_integrity(),
            Err(ZiFilterError::InvalidState(message)) if message.contains("ramp end time")
        ));
        assert_nonfinite_json_is_rejected(&ramp);
    }

    #[test]
    fn missed_many_samples_is_refused_without_iteration() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0e-12).unwrap();
        let error = filter
            .eval(1.0, 1.0e100, true)
            .expect_err("a step crossing sample edges must fail");
        assert!(error.to_string().contains("sample instant"));
    }

    #[test]
    fn rejected_candidate_never_reaches_accepted_history() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        assert_eq!(filter.eval(3.0, 0.0, true).unwrap(), 3.0);
        filter.commit(0.0).unwrap();
        assert_eq!(filter.eval(7.0, 1.0, true).unwrap(), 7.0);
        // Reject t=1 and accept a retry before it. The future candidate must
        // be discarded without touching accepted history.
        assert_eq!(filter.eval(99.0, 0.5, true).unwrap(), 3.0);
        filter.commit(0.5).unwrap();
        assert_eq!(filter.eval(4.0, 1.0, true).unwrap(), 4.0);
    }

    #[test]
    fn only_the_latest_complete_newton_pass_can_commit_a_sample() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();

        assert_eq!(filter.eval(7.0, 0.0, true).unwrap(), 7.0);
        filter.begin_evaluation();
        let error = filter
            .commit(0.0)
            .expect_err("stale early-pass candidate must be invalidated");
        assert!(error.to_string().contains("final device pass"));

        assert_eq!(filter.eval(3.0, 0.0, true).unwrap(), 3.0);
        filter.begin_evaluation();
        assert_eq!(filter.eval(5.0, 0.0, true).unwrap(), 5.0);
        filter.commit(0.0).unwrap();
        assert_eq!(filter.eval(99.0, 0.5, true).unwrap(), 5.0);
    }

    #[test]
    fn active_hold_cannot_commit_when_the_final_device_pass_skips_the_site() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        filter.eval(2.0, 0.0, true).unwrap();
        filter.commit(0.0).unwrap();
        filter.begin_evaluation();
        let before = format!("{filter:#?}");

        let error = filter
            .commit(0.5)
            .expect_err("an active Zi site must execute in the final device pass");
        assert!(
            error
                .to_string()
                .contains("skipped by the final device pass")
        );
        assert_eq!(format!("{filter:#?}"), before);
    }

    #[test]
    fn singular_dc_and_subnormal_gain_are_distinguished() {
        let singular = ZiFilter::new(vec![1.0], vec![1.0, -1.0], 1.0).unwrap();
        assert_eq!(singular.dc_gain(), Err(ZiFilterError::SingularDc));

        let tiny = ZiFilter::new(vec![f64::from_bits(1)], vec![1.0], 1.0).unwrap();
        assert_eq!(tiny.dc_gain().unwrap().to_bits(), 1);
        assert_eq!(
            tiny.eval_derivative(1.0, 0.0, false, 0.0)
                .unwrap()
                .to_bits(),
            1
        );
    }

    #[test]
    fn dc_and_edge_jacobian_actions_round_only_after_the_complete_ratio() {
        let tiny = f64::from_bits(1);
        let underflowing_gain = ZiFilter::new(vec![tiny], vec![f64::MAX], 1.0).unwrap();
        assert_eq!(
            underflowing_gain
                .clone()
                .eval(f64::MAX, 0.0, false)
                .unwrap()
                .to_bits(),
            tiny.to_bits(),
            "a subnormal DC action must not be erased by first rounding H(1)"
        );
        assert_eq!(
            underflowing_gain
                .eval_derivative(f64::MAX, 0.0, false, 0.0)
                .unwrap()
                .to_bits(),
            tiny.to_bits(),
            "the DC Jacobian action must use the same one-rounding ratio"
        );
        assert_eq!(
            underflowing_gain
                .eval_derivative(f64::MAX, 0.0, true, 0.0)
                .unwrap()
                .to_bits(),
            tiny.to_bits(),
            "the sample-edge Jacobian must not round b0/a0 before its seed"
        );

        let overflowing_gain = ZiFilter::new(vec![f64::MAX], vec![tiny], 1.0).unwrap();
        assert_eq!(
            overflowing_gain.clone().eval(tiny, 0.0, false).unwrap(),
            f64::MAX,
            "a finite DC action must survive an unrepresentable standalone gain"
        );
        assert_eq!(
            overflowing_gain
                .eval_derivative(tiny, 0.0, true, 0.0)
                .unwrap(),
            f64::MAX
        );
    }

    #[test]
    fn small_exact_ratio_is_bit_identical_to_big_fallback_for_typical_terms() {
        let numerator_terms = [(0.25, 1.25), (-0.5, 0.75), (0.125, -2.0), (0.0625, 4.0)];
        let denominator_terms = [(1.0, 1.0), (-0.125, 1.0)];
        let small_numerator = small_sum_products(numerator_terms).unwrap().unwrap();
        let small_denominator = small_sum_products(denominator_terms).unwrap().unwrap();
        let small = small_exact_ratio_to_f64(
            small_numerator,
            small_denominator,
            "small exact differential",
        )
        .expect("ordinary terms must fit the compact exact ratio")
        .unwrap();

        let big = checked_exact_ratio(
            scaled_sum_products(numerator_terms).unwrap(),
            scaled_sum_products(denominator_terms).unwrap(),
            "big exact differential",
        )
        .unwrap();
        assert_eq!(small.to_bits(), big.to_bits());
    }

    #[test]
    fn ordinary_filter_paths_select_small_exact_arithmetic() {
        let before = SMALL_EXACT_FAST_PATH_HITS.load(std::sync::atomic::Ordering::Relaxed);
        let mut filter = ZiFilter::new(vec![0.25], vec![1.0, -0.75], 1.0).unwrap();
        assert_eq!(filter.eval(1.0, 0.0, true).unwrap(), 0.25);
        assert_eq!(filter.eval_derivative(1.0, 0.0, true, 0.0).unwrap(), 0.25);
        assert_eq!(filter.eval(2.0, 0.0, false).unwrap(), 2.0);
        assert_eq!(filter.feedthrough().unwrap(), 0.25);
        assert_eq!(filter.dc_gain().unwrap(), 1.0);

        let mut ramp = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        assert_eq!(ramp.eval_with_transition(2.0, 0.0, true, 0.5).unwrap(), 0.0);
        ramp.commit(0.0).unwrap();
        assert_eq!(ramp.eval(99.0, 0.25, true).unwrap(), 1.0);
        let after = SMALL_EXACT_FAST_PATH_HITS.load(std::sync::atomic::Ordering::Relaxed);
        assert!(
            after >= before + 7,
            "sample, edge Jacobian, DC, gain, feedthrough, and ramp must use SmallExact"
        );
    }

    #[test]
    fn wide_exponent_span_uses_bit_identical_big_fallback() {
        let tiny = f64::from_bits(1);
        let terms = [(f64::MAX, 1.0), (tiny, 1.0)];
        assert!(
            small_sum_products(terms).unwrap().is_none(),
            "a span wider than i128 must explicitly decline the compact path"
        );
        let before = BIG_EXACT_FALLBACK_HITS.load(std::sync::atomic::Ordering::Relaxed);
        let actual = checked_sum_products_binary(terms.into_iter(), "wide-span fallback").unwrap();
        let expected = {
            let exact = scaled_sum_products(terms).unwrap();
            exact_binary_to_f64(
                &exact.magnitude,
                exact.negative,
                -2148,
                "wide-span reference",
            )
            .unwrap()
        };
        assert_eq!(actual.to_bits(), expected.to_bits());
        let after = BIG_EXACT_FALLBACK_HITS.load(std::sync::atomic::Ordering::Relaxed);
        assert!(
            after > before,
            "unsupported compact sums must reach BigMagnitude"
        );
    }

    #[test]
    fn failed_evaluation_cannot_commit_an_older_same_time_candidate() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        assert_eq!(filter.eval(1.0, 0.0, true).unwrap(), 1.0);
        assert!(filter.sampling_now());

        filter
            .eval_with_transition(2.0, 0.0, true, -1.0)
            .expect_err("invalid retry must replace the older candidate with failure");
        assert!(!filter.sampling_now());
        let error = filter
            .commit(0.0)
            .expect_err("a failed latest evaluation must never commit stale state");
        assert!(
            error
                .to_string()
                .contains("latest transient evaluation failed")
        );

        filter.begin_evaluation();
        assert_eq!(filter.eval(3.0, 0.0, true).unwrap(), 3.0);
        filter.commit(0.0).unwrap();
        assert_eq!(filter.eval(99.0, 0.5, true).unwrap(), 3.0);
    }

    #[test]
    fn dormant_filter_neither_acquires_accepted_state_nor_advances_its_clock() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 0.25).unwrap();
        assert!(!filter.participates_in_transient_schedule());
        filter.commit(0.0).unwrap();
        assert_eq!(filter.next_sample_index, 0);
        assert_eq!(filter.accepted_time, None);
        assert!(!filter.transient_active);

        assert_eq!(filter.eval(2.0, 0.0, true).unwrap(), 2.0);
        assert!(filter.participates_in_transient_schedule());
        filter.commit(0.0).unwrap();
        assert_eq!(filter.next_sample_index, 1);
        assert_eq!(filter.accepted_time, Some(0.0));
        assert!(filter.transient_active);
    }

    #[test]
    fn accepted_time_rejects_even_a_one_ulp_regression() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        filter.eval(1.0, 0.0, true).unwrap();
        filter.commit(0.0).unwrap();
        filter.eval(1.0, 0.5, true).unwrap();
        filter.commit(0.5).unwrap();

        let earlier = f64::from_bits(0.5_f64.to_bits() - 1);
        filter.eval(1.0, earlier, true).unwrap();
        let error = filter
            .commit(earlier)
            .expect_err("accepted transient time must be strictly monotone");
        assert!(error.to_string().contains("moved backwards"));
        assert_eq!(filter.accepted_time, Some(0.5));
    }

    #[test]
    fn duplicate_acceptance_is_rejected_without_mutating_zi_state() {
        let mut filter = ZiFilter::new(vec![1.0, 0.25], vec![1.0, -0.5], 1.0).unwrap();
        filter.eval(2.0, 0.0, true).unwrap();
        filter.commit(0.0).unwrap();
        filter.eval(9.0, 0.0, true).unwrap();
        let before = format!("{filter:#?}");

        let error = filter
            .commit(0.0)
            .expect_err("an active Zi filter cannot accept the same time twice");
        assert!(
            error.to_string().contains("already accepted"),
            "got: {error}"
        );
        assert_eq!(
            format!("{filter:#?}"),
            before,
            "duplicate acceptance must preserve accepted history and the in-flight candidate"
        );
    }

    #[test]
    fn huge_period_does_not_widen_the_first_edge_acceptance_window() {
        let mut filter = ZiFilter::new_with_timing(vec![1.0], vec![1.0], f64::MAX, 1.0).unwrap();
        let error = filter
            .eval(1.0, 1.0e300, true)
            .expect_err("time far beyond the first edge must not compare equal via ulp(period)");
        assert!(
            error
                .to_string()
                .contains("crossed the required sample edge")
        );
        assert!(!filter.sampling_now());

        let error = filter
            .next_sample_step_bound(1.0e300)
            .expect_err("the breakpoint query must report the same missed edge");
        assert!(
            error
                .to_string()
                .contains("crossed the required sample edge")
        );
    }

    #[test]
    fn exact_accumulator_retains_tiny_residual_after_huge_cancellation() {
        let tiny = f64::from_bits(1);
        assert_eq!(
            coefficient_dc_gain(&[f64::MAX, -f64::MAX, tiny], &[1.0])
                .unwrap()
                .to_bits(),
            tiny.to_bits()
        );

        let mut filter = ZiFilter::new(vec![f64::MAX, -f64::MAX, tiny], vec![1.0], 1.0).unwrap();
        assert_eq!(filter.eval(1.0, 0.0, true).unwrap(), f64::MAX);
        filter.commit(0.0).unwrap();
        assert_eq!(filter.eval(1.0, 1.0, true).unwrap().to_bits(), 0);
        filter.commit(1.0).unwrap();
        assert_eq!(filter.eval(1.0, 2.0, true).unwrap().to_bits(), 1);
    }

    #[test]
    fn subnormal_difference_equation_rounds_exactly_once() {
        let tiny = f64::from_bits(1);
        let mut filter = ZiFilter::new(vec![tiny, 2.0 * tiny, tiny], vec![1.0], 1.0).unwrap();
        filter.x_hist = vec![1.0, 0.5];

        // 2*tiny + 0.5*tiny is an exact halfway case, while tiny*tiny is
        // positive but far below f64. Preserving that sticky residual until
        // the one final RN-even conversion must round upward to bit pattern 3.
        assert_eq!(filter.eval(tiny, 0.0, true).unwrap().to_bits(), 3);
    }

    #[test]
    fn exact_conversion_obeys_ieee_underflow_and_boundary_rounding() {
        let tiny = f64::from_bits(1);
        assert_eq!(
            checked_product(tiny, 0.25, "quarter tiny")
                .unwrap()
                .to_bits(),
            0
        );
        assert_eq!(
            checked_product(tiny, 0.5, "half tiny").unwrap().to_bits(),
            0
        );
        assert_eq!(
            checked_product(tiny, 0.75, "above half tiny")
                .unwrap()
                .to_bits(),
            1
        );
        assert_eq!(
            checked_product(-tiny, 0.5, "negative half tiny")
                .unwrap()
                .to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            checked_product(-tiny, 0.75, "negative above half tiny")
                .unwrap()
                .to_bits(),
            (-tiny).to_bits()
        );

        let max_subnormal = f64::from_bits((1_u64 << 52) - 1);
        assert_eq!(
            checked_product(max_subnormal, 1.0, "max subnormal").unwrap(),
            max_subnormal
        );
        assert_eq!(
            checked_product(f64::MIN_POSITIVE, 1.0, "min normal").unwrap(),
            f64::MIN_POSITIVE
        );
        assert_eq!(
            checked_product(f64::MAX, 1.0, "max finite").unwrap(),
            f64::MAX
        );
        assert!(
            checked_product(
                f64::MAX,
                f64::from_bits(1.0_f64.to_bits() + 1),
                "overflow threshold"
            )
            .is_err()
        );
    }

    #[test]
    fn breakpoint_is_exact_and_advances_after_current_edge() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 0.25).unwrap();
        assert_eq!(filter.next_sample_step_bound(0.0).unwrap(), 0.25);
        filter.eval(1.0, 0.0, true).unwrap();
        filter.commit(0.0).unwrap();
        assert_eq!(filter.next_sample_step_bound(0.125).unwrap(), 0.125);
        assert_eq!(filter.next_sample_step_bound(0.25).unwrap(), 0.25);
    }

    #[test]
    fn positive_transition_endpoint_must_be_finite_and_representably_later() {
        let mut overflow =
            ZiFilter::new_with_timing(vec![1.0], vec![1.0], f64::MAX, f64::MAX).unwrap();
        let error = overflow
            .eval_with_transition(1.0, f64::MAX, true, f64::MAX)
            .expect_err("overflowing ramp endpoint must fail before publishing a candidate");
        assert!(error.to_string().contains("overflows"), "got: {error}");
        assert!(!overflow.sampling_now());

        let time: f64 = 1.0e300;
        let next = f64::from_bits(time.to_bits() + 1);
        let underresolved_tau = (next - time) * 0.25;
        let mut underresolved =
            ZiFilter::new_with_timing(vec![1.0], vec![1.0], time, time).unwrap();
        let error = underresolved
            .eval_with_transition(1.0, time, true, underresolved_tau)
            .expect_err("positive unrepresentable ramp must fail closed");
        assert!(
            error.to_string().contains("not representably after"),
            "got: {error}"
        );
        assert!(!underresolved.sampling_now());
    }

    #[test]
    fn opposite_sign_extreme_ramp_midpoint_remains_finite() {
        let mut filter = ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap();
        filter.ramp_start_time = 0.0;
        filter.ramp_end_time = 2.0;
        filter.ramp_start_value = -f64::MAX;
        filter.held = f64::MAX;
        assert_eq!(
            filter.visible_output(1.0).unwrap().to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn exact_accumulator_tracks_only_its_active_limb_span() {
        let mut low = BigMagnitude::default();
        low.add_shifted(1, 0);
        assert_eq!(low.significant_len(), 1);

        let mut high = BigMagnitude::default();
        high.add_shifted(1, INLINE_ACCUMULATOR_LIMBS * 64 + 7);
        assert_eq!(
            high.significant_len(),
            INLINE_ACCUMULATOR_LIMBS + 1,
            "cached active span must include the first overflow limb"
        );
        let cancelled = high.subtract(&high);
        assert_eq!(cancelled.significant_len(), 0);
        assert!(cancelled.is_zero());
    }
}
