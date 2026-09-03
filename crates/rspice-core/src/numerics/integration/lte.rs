//! Local truncation error estimation.
//!
//! The LTE estimate is what tells the transient loop whether the step it just
//! took was accurate enough to keep. It compares the solution against a
//! prediction built from the accepted history, so it needs the same companion
//! coefficients the devices stamped with.

use crate::Value;
use crate::numerics::integration::IntegrationMethod;

/// Reference magnitude used to normalize transient local-truncation error.
///
/// Four policies correspond exactly to Xyce's `.OPTIONS TIMEINT NEWLTE`
/// selectors; `PredictorLocal` preserves RSpice's non-Xyce adaptive default.
/// Historical modes retain only accepted solution magnitudes, so a rejected
/// candidate cannot relax future error weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransientLteReference {
    /// RSpice legacy policy: scale by the larger candidate/predictor magnitude.
    #[default]
    PredictorLocal,
    /// Scale each state by its magnitude at the prior accepted point (`NEWLTE=0`).
    PointLocal,
    /// Scale every state by the prior accepted point's infinity norm (`NEWLTE=1`).
    PointGlobal,
    /// Scale every state by the largest magnitude seen over the transient (`NEWLTE=2`).
    SignalGlobal,
    /// Scale each state by its own largest magnitude seen over the transient (`NEWLTE=3`).
    SignalLocal,
}

impl TransientLteReference {
    /// Convert a Xyce `NEWLTE` selector to its reference policy.
    pub fn from_xyce_selector(selector: u8) -> Option<Self> {
        match selector {
            0 => Some(Self::PointLocal),
            1 => Some(Self::PointGlobal),
            2 => Some(Self::SignalGlobal),
            3 => Some(Self::SignalLocal),
            _ => None,
        }
    }

    /// Return the corresponding Xyce `NEWLTE` selector.
    pub fn xyce_selector(self) -> Option<u8> {
        match self {
            Self::PredictorLocal => None,
            Self::PointLocal => Some(0),
            Self::PointGlobal => Some(1),
            Self::SignalGlobal => Some(2),
            Self::SignalLocal => Some(3),
        }
    }
}

/// Current internal wire contract for accepted-boundary LTE estimator state.
pub(crate) const ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION: u32 = 1;

/// Versioned snapshot of the estimator state needed to make the next
/// predictor and LTE decision identical after a transient restart.
///
/// This DTO deliberately represents only an accepted boundary. The
/// candidate-local rollback checkpoint in [`LteEstimator`] is not persistent:
/// capture and restore reject an estimator while such an attempt is active.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AcceptedBoundaryLteEstimatorCheckpoint {
    pub(crate) version: u32,
    pub(crate) solution_dimension: usize,
    pub(crate) history_count: usize,
    pub(crate) prev_solution: Vec<Value>,
    pub(crate) prev_prev_solution: Vec<Value>,
    pub(crate) prev_prev_prev_solution: Vec<Value>,
    pub(crate) prev_dt: Value,
    pub(crate) prev_prev_dt: Value,
    pub(crate) reltol: Value,
    pub(crate) abstol: Value,
    pub(crate) reference: TransientLteReference,
    pub(crate) accepted_reference_solution: Vec<Value>,
    pub(crate) signal_global_reference: Value,
    pub(crate) signal_local_reference: Vec<Value>,
    pub(crate) method_order: u32,
    pub(crate) xyce_order_two_difference: Vec<Value>,
    pub(crate) xyce_order_two_difference_dt: Value,
    pub(crate) xyce_attempt_dt: Value,
    pub(crate) xyce_attempt_prev_dt: Value,
    pub(crate) xyce_attempt_prev_prev_dt: Value,
}

/// Local Truncation Error (LTE) estimator for adaptive timestep
///
/// Uses difference between predicted and calculated values to estimate
/// the integration error. This allows rejecting timesteps that converge
/// numerically but are physically inaccurate.
///
/// Supports both standard extrapolation and Richardson extrapolation for
/// higher accuracy LTE estimates.
#[derive(Debug)]
pub struct LteEstimator {
    /// Previous solution vector (t - dt)
    prev_solution: Vec<Value>,
    /// Solution before previous (t - 2*dt)
    prev_prev_solution: Vec<Value>,
    /// Solution from 3 steps ago (t - 3*dt) for Richardson extrapolation
    prev_prev_prev_solution: Vec<Value>,
    /// Previous timestep
    prev_dt: Value,
    /// Timestep before previous
    prev_prev_dt: Value,
    /// Relative LTE tolerance.
    reltol: Value,
    /// Absolute LTE tolerance floor for state variables.
    abstol: Value,
    /// Reference magnitude policy used by the normalized LTE weights.
    reference: TransientLteReference,
    /// Most recent accepted solution used by Xyce point-reference modes.
    accepted_reference_solution: Vec<Value>,
    /// Largest accepted state magnitude seen by signal-global weighting.
    signal_global_reference: Value,
    /// Largest accepted magnitude per state seen by signal-local weighting.
    signal_local_reference: Vec<Value>,
    /// Number of valid history entries
    history_count: usize,
    /// Current integration method (for order-aware scaling)
    method_order: u32,
    /// Xyce OneStep's accepted first-difference history (`xHistory[2]`).
    /// OneStep promotes this only on an accepted order-two step, so it must
    /// not be reconstructed from absolute accepted points after an order-one
    /// interval.
    xyce_order_two_difference: Vec<Value>,
    xyce_order_two_difference_dt: Value,
    /// Xyce OneStep's attempted coefficient history (`psi_[0..2]`). This
    /// advances when a candidate reaches the predictor and is restored with
    /// the one-sided shift used by `OneStep::restoreHistory` on rejection.
    xyce_attempt_dt: Value,
    xyce_attempt_prev_dt: Value,
    xyce_attempt_prev_prev_dt: Value,
    xyce_attempt_checkpoint: Option<(Value, Value, Value, u8)>,
}

fn validate_finite_lte_values(values: &[Value], name: &str) -> Result<(), String> {
    if let Some(index) = values.iter().position(|value| !value.is_finite()) {
        return Err(format!(
            "accepted-boundary LTE {name} contains a non-finite value at index {index}"
        ));
    }
    Ok(())
}

fn validate_lte_vector_shape(values: &[Value], expected: usize, name: &str) -> Result<(), String> {
    if values.len() != expected {
        return Err(format!(
            "accepted-boundary LTE {name} has length {}; expected {expected}",
            values.len()
        ));
    }
    validate_finite_lte_values(values, name)
}

fn validate_canonical_zero_lte_value(value: Value, name: &str) -> Result<(), String> {
    if value.to_bits() != 0.0_f64.to_bits() {
        return Err(format!("accepted-boundary LTE {name} must be canonical +0"));
    }
    Ok(())
}

fn validate_positive_lte_value(value: Value, name: &str) -> Result<(), String> {
    if !value.is_finite() || value <= 0.0 {
        return Err(format!(
            "accepted-boundary LTE {name} must be finite and positive"
        ));
    }
    Ok(())
}

fn validate_nonnegative_lte_value(value: Value, name: &str) -> Result<(), String> {
    if !value.is_finite() || value < 0.0 {
        return Err(format!(
            "accepted-boundary LTE {name} must be finite and nonnegative"
        ));
    }
    if value == 0.0 {
        validate_canonical_zero_lte_value(value, name)?;
    }
    Ok(())
}

fn lte_values_bit_equal(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

impl LteEstimator {
    /// Create a new LTE estimator with legacy single-tolerance semantics.
    ///
    /// `tolerance` is used for both relative and absolute LTE control.
    pub fn new(tolerance: Value) -> Self {
        Self::with_tolerances(tolerance, tolerance)
    }

    /// Create a new LTE estimator with explicit relative and absolute tolerances.
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        Self::with_tolerances_and_reference(reltol, abstol, TransientLteReference::PredictorLocal)
    }

    /// Create an LTE estimator with explicit tolerances and reference policy.
    pub fn with_tolerances_and_reference(
        reltol: Value,
        abstol: Value,
        reference: TransientLteReference,
    ) -> Self {
        let reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };
        let abstol = if abstol.is_finite() && abstol > 0.0 {
            abstol
        } else {
            reltol
        };

        Self {
            prev_solution: Vec::new(),
            prev_prev_solution: Vec::new(),
            prev_prev_prev_solution: Vec::new(),
            prev_dt: 0.0,
            prev_prev_dt: 0.0,
            reltol,
            abstol,
            reference,
            accepted_reference_solution: Vec::new(),
            signal_global_reference: 0.0,
            signal_local_reference: Vec::new(),
            history_count: 0,
            method_order: 2, // Default to trapezoidal order
            xyce_order_two_difference: Vec::new(),
            xyce_order_two_difference_dt: 0.0,
            xyce_attempt_dt: 0.0,
            xyce_attempt_prev_dt: 0.0,
            xyce_attempt_prev_prev_dt: 0.0,
            xyce_attempt_checkpoint: None,
        }
    }

    /// Capture a restart snapshot at an accepted solution boundary.
    ///
    /// `latest_accepted_solution` is supplied independently by the caller so
    /// capture cannot bless a stale or mismatched estimator history. Equality
    /// is bit-exact, including the sign of zero.
    pub(crate) fn capture_accepted_boundary_checkpoint(
        &self,
        latest_accepted_solution: &[Value],
    ) -> Result<AcceptedBoundaryLteEstimatorCheckpoint, String> {
        let checkpoint = AcceptedBoundaryLteEstimatorCheckpoint {
            version: ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION,
            solution_dimension: latest_accepted_solution.len(),
            history_count: self.history_count,
            prev_solution: self.prev_solution.clone(),
            prev_prev_solution: self.prev_prev_solution.clone(),
            prev_prev_prev_solution: self.prev_prev_prev_solution.clone(),
            prev_dt: self.prev_dt,
            prev_prev_dt: self.prev_prev_dt,
            reltol: self.reltol,
            abstol: self.abstol,
            reference: self.reference,
            accepted_reference_solution: self.accepted_reference_solution.clone(),
            signal_global_reference: self.signal_global_reference,
            signal_local_reference: self.signal_local_reference.clone(),
            method_order: self.method_order,
            xyce_order_two_difference: self.xyce_order_two_difference.clone(),
            xyce_order_two_difference_dt: self.xyce_order_two_difference_dt,
            xyce_attempt_dt: self.xyce_attempt_dt,
            xyce_attempt_prev_dt: self.xyce_attempt_prev_dt,
            xyce_attempt_prev_prev_dt: self.xyce_attempt_prev_prev_dt,
        };
        self.validate_accepted_boundary_checkpoint(&checkpoint, latest_accepted_solution)?;
        Ok(checkpoint)
    }

    /// Validate an accepted-boundary snapshot against this estimator's
    /// configured reference mode and the caller-owned accepted solution.
    pub(crate) fn validate_accepted_boundary_checkpoint(
        &self,
        checkpoint: &AcceptedBoundaryLteEstimatorCheckpoint,
        latest_accepted_solution: &[Value],
    ) -> Result<(), String> {
        if self.xyce_attempt_checkpoint.is_some() {
            return Err(
                "cannot validate or restore an LTE checkpoint while an Xyce attempt is active"
                    .to_string(),
            );
        }
        if checkpoint.version != ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION {
            return Err(format!(
                "unsupported accepted-boundary LTE checkpoint version {} (runtime requires {})",
                checkpoint.version, ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION
            ));
        }
        if checkpoint.reference != self.reference {
            return Err(format!(
                "accepted-boundary LTE reference mode mismatch: checkpoint {:?}, runtime {:?}",
                checkpoint.reference, self.reference
            ));
        }
        for (checkpoint_value, runtime_value, name) in [
            (checkpoint.reltol, self.reltol, "relative tolerance"),
            (checkpoint.abstol, self.abstol, "absolute tolerance"),
        ] {
            validate_positive_lte_value(checkpoint_value, name)?;
            if checkpoint_value.to_bits() != runtime_value.to_bits() {
                return Err(format!(
                    "accepted-boundary LTE {name} mismatch: checkpoint {checkpoint_value}, runtime {runtime_value}"
                ));
            }
        }
        if checkpoint.history_count > 3 {
            return Err(format!(
                "accepted-boundary LTE history count {} exceeds the supported maximum of 3",
                checkpoint.history_count
            ));
        }
        if checkpoint.solution_dimension != latest_accepted_solution.len() {
            return Err(format!(
                "accepted-boundary LTE solution dimension {} does not match the caller's latest accepted solution dimension {}",
                checkpoint.solution_dimension,
                latest_accepted_solution.len()
            ));
        }
        validate_finite_lte_values(latest_accepted_solution, "latest accepted solution")?;

        let history_shapes = [
            (
                &checkpoint.prev_solution,
                usize::from(checkpoint.history_count >= 1) * checkpoint.solution_dimension,
                "latest accepted solution history",
            ),
            (
                &checkpoint.prev_prev_solution,
                usize::from(checkpoint.history_count >= 2) * checkpoint.solution_dimension,
                "second accepted solution history",
            ),
            (
                &checkpoint.prev_prev_prev_solution,
                usize::from(checkpoint.history_count >= 3) * checkpoint.solution_dimension,
                "third accepted solution history",
            ),
        ];
        for (values, expected, name) in history_shapes {
            validate_lte_vector_shape(values, expected, name)?;
        }
        if checkpoint.history_count != 0
            && !lte_values_bit_equal(&checkpoint.prev_solution, latest_accepted_solution)
        {
            return Err(
                "accepted-boundary LTE latest history does not equal the caller's accepted solution"
                    .to_string(),
            );
        }

        match checkpoint.history_count {
            0 | 1 => {
                validate_canonical_zero_lte_value(checkpoint.prev_dt, "previous timestep")?;
                validate_canonical_zero_lte_value(
                    checkpoint.prev_prev_dt,
                    "second previous timestep",
                )?;
            }
            2 => {
                validate_positive_lte_value(checkpoint.prev_dt, "previous timestep")?;
                validate_canonical_zero_lte_value(
                    checkpoint.prev_prev_dt,
                    "second previous timestep",
                )?;
            }
            3 => {
                validate_positive_lte_value(checkpoint.prev_dt, "previous timestep")?;
                validate_positive_lte_value(checkpoint.prev_prev_dt, "second previous timestep")?;
            }
            _ => unreachable!("history count was bounded above"),
        }

        let has_accepted_reference = checkpoint.reference != TransientLteReference::PredictorLocal;
        let expected_reference_len =
            usize::from(has_accepted_reference) * checkpoint.solution_dimension;
        validate_lte_vector_shape(
            &checkpoint.accepted_reference_solution,
            expected_reference_len,
            "accepted LTE reference solution",
        )?;
        if has_accepted_reference
            && !lte_values_bit_equal(
                &checkpoint.accepted_reference_solution,
                latest_accepted_solution,
            )
        {
            return Err(
                "accepted LTE reference solution does not equal the caller's accepted solution"
                    .to_string(),
            );
        }

        validate_nonnegative_lte_value(
            checkpoint.signal_global_reference,
            "signal-global LTE reference",
        )?;
        match checkpoint.reference {
            TransientLteReference::SignalGlobal => {
                if !checkpoint.signal_local_reference.is_empty() {
                    return Err(
                        "signal-global LTE checkpoint contains signal-local references".to_string(),
                    );
                }
                let accepted_max = latest_accepted_solution
                    .iter()
                    .map(|value| value.abs())
                    .fold(0.0, Value::max);
                if checkpoint.signal_global_reference < accepted_max {
                    return Err(
                        "signal-global LTE reference does not cover the latest accepted solution"
                            .to_string(),
                    );
                }
            }
            TransientLteReference::SignalLocal => {
                validate_canonical_zero_lte_value(
                    checkpoint.signal_global_reference,
                    "signal-global LTE reference",
                )?;
                validate_lte_vector_shape(
                    &checkpoint.signal_local_reference,
                    expected_reference_len,
                    "signal-local LTE references",
                )?;
                for (index, (reference, accepted)) in checkpoint
                    .signal_local_reference
                    .iter()
                    .zip(latest_accepted_solution)
                    .enumerate()
                {
                    validate_nonnegative_lte_value(
                        *reference,
                        &format!("signal-local LTE reference {index}"),
                    )?;
                    if *reference < accepted.abs() {
                        return Err(format!(
                            "signal-local LTE reference {index} does not cover the latest accepted solution"
                        ));
                    }
                }
            }
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::PointGlobal => {
                validate_canonical_zero_lte_value(
                    checkpoint.signal_global_reference,
                    "signal-global LTE reference",
                )?;
                if !checkpoint.signal_local_reference.is_empty() {
                    return Err(
                        "non-signal LTE checkpoint contains signal-local references".to_string()
                    );
                }
            }
        }

        if !(1..=2).contains(&checkpoint.method_order) {
            return Err(format!(
                "accepted-boundary LTE method order {} is outside the supported range 1..=2",
                checkpoint.method_order
            ));
        }
        if checkpoint.xyce_order_two_difference.is_empty() {
            validate_canonical_zero_lte_value(
                checkpoint.xyce_order_two_difference_dt,
                "order-two difference timestep",
            )?;
        } else {
            if checkpoint.history_count != 3 {
                return Err(
                    "order-two LTE difference history requires three accepted solutions"
                        .to_string(),
                );
            }
            validate_lte_vector_shape(
                &checkpoint.xyce_order_two_difference,
                checkpoint.solution_dimension,
                "order-two LTE difference history",
            )?;
            validate_positive_lte_value(
                checkpoint.xyce_order_two_difference_dt,
                "order-two difference timestep",
            )?;
        }

        for (value, name) in [
            (
                checkpoint.xyce_attempt_dt,
                "current OneStep psi coefficient",
            ),
            (
                checkpoint.xyce_attempt_prev_dt,
                "previous OneStep psi coefficient",
            ),
            (
                checkpoint.xyce_attempt_prev_prev_dt,
                "second previous OneStep psi coefficient",
            ),
        ] {
            validate_nonnegative_lte_value(value, name)?;
        }
        if checkpoint.reference == TransientLteReference::PredictorLocal {
            validate_canonical_zero_lte_value(
                checkpoint.xyce_attempt_dt,
                "current OneStep psi coefficient",
            )?;
            validate_canonical_zero_lte_value(
                checkpoint.xyce_attempt_prev_dt,
                "previous OneStep psi coefficient",
            )?;
            validate_canonical_zero_lte_value(
                checkpoint.xyce_attempt_prev_prev_dt,
                "second previous OneStep psi coefficient",
            )?;
        } else if (checkpoint.xyce_attempt_prev_dt > 0.0 && checkpoint.xyce_attempt_dt == 0.0)
            || (checkpoint.xyce_attempt_prev_prev_dt > 0.0
                && checkpoint.xyce_attempt_prev_dt == 0.0)
        {
            return Err(
                "accepted-boundary OneStep psi coefficients have impossible zero provenance"
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Restore an accepted-boundary snapshot after validating every field.
    /// No estimator state is changed if validation fails.
    pub(crate) fn restore_accepted_boundary_checkpoint(
        &mut self,
        checkpoint: &AcceptedBoundaryLteEstimatorCheckpoint,
        latest_accepted_solution: &[Value],
    ) -> Result<(), String> {
        self.validate_accepted_boundary_checkpoint(checkpoint, latest_accepted_solution)?;

        self.prev_solution.clone_from(&checkpoint.prev_solution);
        self.prev_prev_solution
            .clone_from(&checkpoint.prev_prev_solution);
        self.prev_prev_prev_solution
            .clone_from(&checkpoint.prev_prev_prev_solution);
        self.prev_dt = checkpoint.prev_dt;
        self.prev_prev_dt = checkpoint.prev_prev_dt;
        self.accepted_reference_solution
            .clone_from(&checkpoint.accepted_reference_solution);
        self.signal_global_reference = checkpoint.signal_global_reference;
        self.signal_local_reference
            .clone_from(&checkpoint.signal_local_reference);
        self.history_count = checkpoint.history_count;
        self.method_order = checkpoint.method_order;
        self.xyce_order_two_difference
            .clone_from(&checkpoint.xyce_order_two_difference);
        self.xyce_order_two_difference_dt = checkpoint.xyce_order_two_difference_dt;
        self.xyce_attempt_dt = checkpoint.xyce_attempt_dt;
        self.xyce_attempt_prev_dt = checkpoint.xyce_attempt_prev_dt;
        self.xyce_attempt_prev_prev_dt = checkpoint.xyce_attempt_prev_prev_dt;
        debug_assert!(self.xyce_attempt_checkpoint.is_none());
        Ok(())
    }

    #[inline]
    fn lte_scale_denominator(&self, magnitude: Value) -> Value {
        // Enforce SPICE-like weighted tolerance:
        // |err| <= ABSTOL + RELTOL * |x|
        // Rearranged to a normalized metric compared against RELTOL:
        // |err| / (ABSTOL/RELTOL + |x|) <= RELTOL
        let reltol = self.reltol.max(1e-30);
        (self.abstol / reltol + magnitude.abs()).max(1e-30)
    }

    #[inline]
    fn accepted_point_global_reference(&self) -> Option<Value> {
        self.accepted_reference_solution
            .iter()
            .try_fold(0.0_f64, |reference, value| {
                value.is_finite().then(|| reference.max(value.abs()))
            })
    }

    #[inline]
    fn accepted_point_global_reference_prefix(&self, prefix_len: usize) -> Option<Value> {
        self.accepted_reference_solution
            .iter()
            .take(prefix_len)
            .try_fold(0.0_f64, |reference, value| {
                value.is_finite().then(|| reference.max(value.abs()))
            })
    }

    #[inline]
    fn accepted_reference_magnitude(&self, index: usize, accepted_point_global: Value) -> Value {
        match self.reference {
            TransientLteReference::PredictorLocal => 0.0,
            TransientLteReference::PointLocal => self.accepted_reference_solution[index].abs(),
            TransientLteReference::PointGlobal => accepted_point_global,
            TransientLteReference::SignalGlobal => {
                self.signal_global_reference.max(accepted_point_global)
            }
            TransientLteReference::SignalLocal => self
                .signal_local_reference
                .get(index)
                .copied()
                .unwrap_or_else(|| self.accepted_reference_solution[index].abs()),
        }
    }

    fn update_signal_reference_prefix(&mut self, solution: &[Value], prefix_len: usize) {
        let reference_len = prefix_len.min(solution.len());
        match self.reference {
            TransientLteReference::SignalGlobal => {
                let accepted_max = solution[..reference_len]
                    .iter()
                    .filter(|value| value.is_finite())
                    .map(|value| value.abs())
                    .fold(0.0, Value::max);
                self.signal_global_reference = self.signal_global_reference.max(accepted_max);
            }
            TransientLteReference::SignalLocal => {
                if self.signal_local_reference.len() < reference_len {
                    self.signal_local_reference.resize(reference_len, 0.0);
                }
                for (reference, value) in self
                    .signal_local_reference
                    .iter_mut()
                    .zip(&solution[..reference_len])
                {
                    if value.is_finite() {
                        *reference = (*reference).max(value.abs());
                    }
                }
            }
            TransientLteReference::PointLocal | TransientLteReference::PointGlobal => {}
            TransientLteReference::PredictorLocal => {}
        }
        if self.reference != TransientLteReference::PredictorLocal {
            self.accepted_reference_solution.clear();
            self.accepted_reference_solution
                .extend_from_slice(&solution[..reference_len]);
        }
    }

    /// Seed historical signal references without adding predictor history.
    #[cfg(test)]
    pub(crate) fn seed_reference_prefix(&mut self, solution: &[Value], prefix_len: usize) {
        self.update_signal_reference_prefix(solution, prefix_len);
    }

    /// Seed both Xyce reference state and the accepted t0 predictor history.
    pub(crate) fn seed_initial_solution(&mut self, solution: &[Value]) {
        self.update_signal_reference_prefix(solution, solution.len());
        self.restart_history_from(solution);
    }

    /// Snapshot historical signal references for transient checkpointing.
    pub(crate) fn signal_reference_snapshot(&self) -> (Value, &[Value]) {
        (self.signal_global_reference, &self.signal_local_reference)
    }

    /// Restore historical signal references from a validated checkpoint.
    pub(crate) fn restore_signal_reference_snapshot(
        &mut self,
        global: Value,
        local: &[Value],
    ) -> Result<(), String> {
        if !global.is_finite()
            || global < 0.0
            || local.iter().any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err("checkpoint contains invalid transient LTE references".to_string());
        }
        match self.reference {
            TransientLteReference::SignalGlobal => {
                let accepted = self.accepted_point_global_reference().ok_or_else(|| {
                    "checkpoint accepted solution contains a non-finite LTE reference".to_string()
                })?;
                if !local.is_empty() || global < accepted {
                    return Err(
                        "checkpoint signal-global LTE state is inconsistent with the accepted solution"
                            .to_string(),
                    );
                }
            }
            TransientLteReference::SignalLocal => {
                if self
                    .accepted_reference_solution
                    .iter()
                    .any(|value| !value.is_finite())
                    || global != 0.0
                    || local.len() != self.accepted_reference_solution.len()
                    || local
                        .iter()
                        .zip(&self.accepted_reference_solution)
                        .any(|(reference, accepted)| *reference < accepted.abs())
                {
                    return Err(
                        "checkpoint signal-local LTE references do not cover the accepted solution"
                            .to_string(),
                    );
                }
            }
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::PointGlobal => {
                if global != 0.0 || !local.is_empty() {
                    return Err(
                        "checkpoint point-reference LTE state contains signal history".to_string(),
                    );
                }
            }
        }
        self.signal_global_reference = global;
        self.signal_local_reference.clear();
        self.signal_local_reference.extend_from_slice(local);
        Ok(())
    }

    #[inline]
    fn predict_next_value(
        &self,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
        method_order: u32,
    ) -> Value {
        if method_order >= 2 && self.history_count >= 3 && self.prev_prev_dt > 0.0 {
            // Evaluate the quadratic interpolant through the previous three
            // accepted points at the proposed next time. This keeps the LTE
            // estimate order-consistent with trapezoidal/Gear2 instead of
            // falling back to a first-order predictor that can over-restrict
            // smooth waveforms.
            let h1 = self.prev_dt;
            let h2 = self.prev_prev_dt;
            let t0 = 0.0;
            let t1 = -h1;
            let t2 = -(h1 + h2);
            let t = dt;

            let l0 = (t - t1) * (t - t2) / ((t0 - t1) * (t0 - t2));
            let l1 = (t - t0) * (t - t2) / ((t1 - t0) * (t1 - t2));
            let l2 = (t - t0) * (t - t1) / ((t2 - t0) * (t2 - t1));
            prev * l0 + prev_prev * l1 + prev_prev_prev * l2
        } else if self.history_count >= 2 && self.prev_dt > 0.0 {
            let slope = (prev - prev_prev) / self.prev_dt;
            prev + slope * dt
        } else {
            prev
        }
    }

    #[inline]
    fn predict_trapezoidal_order1_value(&self, prev: Value, prev_prev: Value, dt: Value) -> Value {
        let predictor_dt = if self.uses_accepted_solution_reference()
            && self.xyce_attempt_prev_dt.is_finite()
            && self.xyce_attempt_prev_dt > 0.0
        {
            self.xyce_attempt_prev_dt
        } else {
            self.prev_dt
        };
        if self.history_count >= 2 && predictor_dt > 0.0 {
            prev + (dt / predictor_dt) * (prev - prev_prev)
        } else {
            prev
        }
    }

    #[inline]
    fn predict_trapezoidal_order2_value(
        &self,
        index: usize,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
    ) -> Value {
        let predictor_dt = if self.uses_accepted_solution_reference()
            && self.xyce_attempt_prev_dt.is_finite()
            && self.xyce_attempt_prev_dt > 0.0
        {
            self.xyce_attempt_prev_dt
        } else {
            self.prev_dt
        };
        let predictor_prev_dt = if self.uses_accepted_solution_reference()
            && self.xyce_attempt_prev_prev_dt.is_finite()
            && self.xyce_attempt_prev_prev_dt > 0.0
        {
            self.xyce_attempt_prev_prev_dt
        } else {
            self.xyce_order_two_difference_dt
        };
        if self.history_count >= 3 && predictor_dt > 0.0 && predictor_prev_dt > 0.0 {
            // Mirror OneStep::updateCoeffs() and obtainPredictor() in their
            // source order. Xyce forms beta1 from the timestep ratio and
            // beta2 from three sequential divisions; algebraically reducing
            // those products changes the low bits that decide an adaptive
            // retry at hysteresis reversals.
            let difference = prev - prev_prev;
            let previous_difference = self
                .xyce_order_two_difference
                .get(index)
                .copied()
                .unwrap_or(prev_prev - prev_prev_prev);
            let ratio = dt / predictor_dt;
            let beta1 = ratio + (ratio * ratio) / 2.0;
            let beta2 = -(dt * dt / predictor_dt / predictor_prev_dt / 2.0);
            prev + beta1 * difference + beta2 * previous_difference
        } else {
            self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
        }
    }

    /// Predict the next solution vector prefix from accepted history.
    ///
    /// The predicted vector is initialized from the most recently accepted
    /// solution, then the first `predicted_len` entries are extrapolated.
    /// This mirrors ngspice's `NIpred` behavior for trapezoidal integration and
    /// falls back to order-consistent polynomial extrapolation for Gear methods.
    pub fn predict_solution_prefix(
        &self,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        predicted_len: usize,
    ) -> Option<Vec<Value>> {
        if self.history_count == 0 || self.prev_solution.is_empty() || !dt.is_finite() || dt <= 0.0
        {
            return None;
        }

        let mut predicted = self.prev_solution.clone();
        let predicted_len = predicted_len.min(predicted.len());
        let method_order = Self::integration_method_order(method, trap_order);
        for (idx, value) in predicted.iter_mut().enumerate().take(predicted_len) {
            let prev = self.prev_solution[idx];
            let prev_prev = self.prev_prev_solution.get(idx).copied().unwrap_or(prev);
            let prev_prev_prev = self
                .prev_prev_prev_solution
                .get(idx)
                .copied()
                .unwrap_or(prev_prev);
            *value = match method {
                IntegrationMethod::BackwardEuler => {
                    self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
                }
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                    if trap_order >= 2 {
                        self.predict_trapezoidal_order2_value(
                            idx,
                            prev,
                            prev_prev,
                            prev_prev_prev,
                            dt,
                        )
                    } else {
                        self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
                    }
                }
                IntegrationMethod::Gear2 => {
                    self.predict_next_value(prev, prev_prev, prev_prev_prev, dt, method_order)
                }
            };
        }

        Some(predicted)
    }

    /// Predict the next full solution vector from accepted history.
    pub fn predict_solution(
        &self,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Option<Vec<Value>> {
        self.predict_solution_prefix(dt, method, trap_order, usize::MAX)
    }

    /// Set the integration method order for accurate timestep scaling
    /// - BackwardEuler: order 1
    /// - Trapezoidal, Gear2: order 2
    #[inline]
    pub fn set_method_order(&mut self, order: u32) {
        self.method_order = order.max(1);
    }

    #[inline]
    fn integration_method_order(method: IntegrationMethod, trap_order: u8) -> u32 {
        match method {
            IntegrationMethod::BackwardEuler => 1,
            IntegrationMethod::Trapezoidal
            | IntegrationMethod::TrapGear
            | IntegrationMethod::Gear2
                if trap_order <= 1 =>
            {
                1
            }
            IntegrationMethod::Trapezoidal
            | IntegrationMethod::Gear2
            | IntegrationMethod::TrapGear => 2,
        }
    }

    /// Whether this estimator uses Xyce accepted-solution weighting and WRMS.
    pub fn uses_accepted_solution_reference(&self) -> bool {
        self.reference != TransientLteReference::PredictorLocal
    }

    /// Advance Xyce OneStep's attempted coefficient history for a candidate
    /// that reaches the predictor. A rejected candidate is rolled back with
    /// the matching one-sided `restoreHistory` shift.
    pub(crate) fn begin_xyce_attempt(&mut self, dt: Value, order: u8) {
        if !self.uses_accepted_solution_reference() || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.xyce_attempt_checkpoint = Some((
            self.xyce_attempt_dt,
            self.xyce_attempt_prev_dt,
            self.xyce_attempt_prev_prev_dt,
            order,
        ));
        if order >= 2 {
            self.xyce_attempt_prev_prev_dt = self.xyce_attempt_prev_dt;
        }
        self.xyce_attempt_prev_dt = self.xyce_attempt_dt;
        self.xyce_attempt_dt = dt;
    }

    pub(crate) fn rollback_xyce_attempt(&mut self) {
        if let Some((dt, prev_dt, prev_prev_dt, order)) = self.xyce_attempt_checkpoint.take() {
            // `begin_xyce_attempt` mirrors OneStep::updateCoeffs before the
            // candidate solve.  OneStep::restoreHistory shifts that
            // post-update array in place: an order-two retry leaves
            // (checkpoint psi0, psi1, psi1), while an order-one retry leaves
            // (checkpoint psi0, psi0, psi1).
            self.xyce_attempt_dt = dt;
            self.xyce_attempt_prev_dt = if order >= 2 { prev_dt } else { dt };
            // OneStep::restoreHistory() shifts psi[0] from psi[1] for an
            // order-one retry but leaves psi[2] untouched.  Preserve that
            // untouched third coefficient instead of collapsing it onto
            // psi[1]; the distinction matters when the next predictor uses
            // variable-step order two.
            self.xyce_attempt_prev_prev_dt = if order >= 2 { prev_dt } else { prev_prev_dt };
        }
    }

    /// Apply Xyce's policy for a rejected OneStep/Gear12 attempt. LTE control
    /// restores coefficient history; nonlinear-iteration control deliberately
    /// retains the attempted update, matching `ERROPTION=1`'s no-restore path.
    pub(crate) fn reject_xyce_attempt(&mut self, restore_history: bool) {
        if restore_history {
            self.rollback_xyce_attempt();
        } else {
            self.xyce_attempt_checkpoint = None;
        }
    }

    pub(crate) fn requires_signal_reference_history(&self) -> bool {
        matches!(
            self.reference,
            TransientLteReference::SignalGlobal | TransientLteReference::SignalLocal
        )
    }

    pub(crate) fn reference_mode(&self) -> TransientLteReference {
        self.reference
    }

    /// Record a solution point for history and reference weighting.
    pub fn record(&mut self, solution: &[Value], dt: Value) {
        self.record_prefix(solution, solution.len(), dt);
    }

    /// Record an accepted point together with the OneStep order that produced
    /// it. Xyce promotes the first-difference history only on order two.
    pub(crate) fn record_with_order(
        &mut self,
        solution: &[Value],
        prefix_len: usize,
        dt: Value,
        order: u8,
    ) {
        self.method_order = u32::from(order.max(1));
        if order >= 2
            && self.history_count >= 2
            && self.prev_dt.is_finite()
            && self.prev_dt > 0.0
            && self.prev_solution.len() == self.prev_prev_solution.len()
        {
            self.xyce_order_two_difference = self
                .prev_solution
                .iter()
                .zip(&self.prev_prev_solution)
                .map(|(current, previous)| current - previous)
                .collect();
            self.xyce_order_two_difference_dt = self.prev_dt;
        }
        self.record_prefix(solution, prefix_len, dt);
    }

    /// Record a solution while limiting LTE reference state to a vector prefix.
    ///
    /// Transient solution vectors append algebraic branch-current unknowns after
    /// node voltages. Callers that apply generic voltage LTE to only the nodal
    /// prefix should use the same prefix here so mixed-unit branch magnitudes do
    /// not relax voltage error weights.
    pub fn record_prefix(&mut self, solution: &[Value], prefix_len: usize, dt: Value) {
        self.update_signal_reference_prefix(solution, prefix_len);

        // Shift history: prev_prev_prev <- prev_prev <- prev <- new
        let prior_history_count = self.history_count;
        self.prev_prev_prev_solution = std::mem::take(&mut self.prev_prev_solution);
        self.prev_prev_solution = std::mem::take(&mut self.prev_solution);
        self.prev_solution = solution.to_vec();
        // A timestep is an interval between two accepted solutions. The first
        // recorded solution therefore has no associated interval, and the
        // synthetic restart timestep attached to a one-solution history must
        // not be shifted into persisted accepted-interval history.
        self.prev_prev_dt = if prior_history_count >= 2 {
            self.prev_dt
        } else {
            0.0
        };
        self.prev_dt = if prior_history_count >= 1 { dt } else { 0.0 };
        if self.history_count < 3 {
            self.history_count += 1;
        }
        // Every call to this shared record sink is an accepted boundary. Its
        // pre-attempt rollback image is therefore no longer live state,
        // including the force-accepted path that enters through `record()`.
        self.xyce_attempt_checkpoint = None;
    }

    /// Estimate LTE using linear extrapolation vs actual value
    /// Returns (lte_estimate, should_accept)
    pub fn estimate(&self, current: &[Value], dt: Value) -> (Value, bool) {
        self.estimate_prefix(current, current.len(), dt)
    }

    /// Estimate LTE for the first `prefix_len` solution entries.
    ///
    /// Transient Newton vectors include algebraic branch-current unknowns for
    /// voltage sources and inductors. Those currents can legitimately jump to
    /// enforce KCL around capacitive edges, so generic LTE control should be
    /// applied to dynamic node-voltage state unless a device-local charge
    /// controller is being used.
    pub fn estimate_prefix(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
    ) -> (Value, bool) {
        self.estimate_prefix_excluding(current, prefix_len, dt, &[])
    }

    /// Estimate LTE for the first `prefix_len` solution entries, skipping
    /// sorted zero-based indices owned by model-specific timestep semantics.
    pub fn estimate_prefix_excluding(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
    ) -> (Value, bool) {
        self.estimate_prefix_excluding_with_coefficient(
            current,
            None,
            prefix_len,
            dt,
            excluded_indices,
            1.0,
            self.method_order,
        )
    }

    /// Estimate LTE using Xyce's integration-method correction coefficient.
    ///
    /// Xyce multiplies the weighted RMS predictor correction by `ck`: for
    /// one-step order 1 it is `h/(h+h_prev)`, for trapezoidal order 2 it is
    /// one third of that value, and for Gear order 2 it is
    /// `h/(h+h_prev+h_prev_prev)`. Native predictor-local control preserves its
    /// established uncorrected metric.
    pub(crate) fn estimate_prefix_excluding_for_integration(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        method: IntegrationMethod,
        trap_order: u8,
    ) -> (Value, bool) {
        let coefficient = self.integration_error_coefficient(method, trap_order, dt);
        let method_order = Self::integration_method_order(method, trap_order);
        self.estimate_prefix_excluding_with_coefficient(
            current,
            None,
            prefix_len,
            dt,
            excluded_indices,
            coefficient,
            method_order,
        )
    }

    /// Estimate LTE from the exact predictor vector used to seed Newton.
    pub(crate) fn estimate_correction_prefix_excluding_for_integration(
        &self,
        current: &[Value],
        predicted: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        method: IntegrationMethod,
        trap_order: u8,
    ) -> (Value, bool) {
        let coefficient = self.integration_error_coefficient(method, trap_order, dt);
        let method_order = Self::integration_method_order(method, trap_order);
        self.estimate_prefix_excluding_with_coefficient(
            current,
            Some(predicted),
            prefix_len,
            dt,
            excluded_indices,
            coefficient,
            method_order,
        )
    }

    fn integration_error_coefficient(
        &self,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
    ) -> Value {
        if !self.uses_accepted_solution_reference()
            || !dt.is_finite()
            || dt <= 0.0
            || !self.prev_dt.is_finite()
            || self.prev_dt <= 0.0
        {
            return 1.0;
        }

        let one_step = dt / (dt + self.prev_dt);
        match method {
            IntegrationMethod::BackwardEuler => one_step,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if trap_order >= 2 {
                    one_step / 3.0
                } else {
                    one_step
                }
            }
            IntegrationMethod::Gear2 => {
                if trap_order >= 2
                    && self.history_count >= 2
                    && self.prev_prev_dt.is_finite()
                    && self.prev_prev_dt > 0.0
                {
                    dt / (dt + self.prev_dt + self.prev_prev_dt)
                } else {
                    one_step
                }
            }
        }
    }

    fn estimate_prefix_excluding_with_coefficient(
        &self,
        current: &[Value],
        predicted_solution: Option<&[Value]>,
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        error_coefficient: Value,
        method_order: u32,
    ) -> (Value, bool) {
        let len = prefix_len.min(current.len());
        if predicted_solution.is_some_and(|predicted| predicted.len() < len) {
            return (Value::INFINITY, false);
        }
        // Need at least one previous point to estimate
        if len == 0 || self.history_count < 1 || self.prev_solution.len() < len {
            return (0.0, true); // Accept, no history yet
        }

        let mut aggregate = 0.0_f64;
        let accepted_point_global = match self.reference {
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::SignalLocal => 0.0,
            TransientLteReference::PointGlobal | TransientLteReference::SignalGlobal => {
                // Xyce NEWLTE=1/2 forms the global reference from the full
                // accepted solution before assigning MachineBig weights to
                // device-masked variables. Masks suppress only their error
                // numerator below; their values still scale every global
                // reference weight.
                let Some(reference) = self.accepted_point_global_reference_prefix(len) else {
                    return (Value::INFINITY, false);
                };
                reference
            }
        };

        // For trapezoidal, LTE ~ (dt^3 / 12) * d^3v/dt^3
        // We approximate by comparing predicted (linear extrapolation) vs actual
        for (i, &curr_val) in current.iter().take(len).enumerate() {
            if excluded_indices.binary_search(&i).is_ok() {
                continue;
            }

            let prev_val = self.prev_solution[i];
            let prev_prev_val = self.prev_prev_solution.get(i).copied().unwrap_or(prev_val);
            let prev_prev_prev_val = self
                .prev_prev_prev_solution
                .get(i)
                .copied()
                .unwrap_or(prev_prev_val);
            let predicted = predicted_solution.map_or_else(
                || {
                    self.predict_next_value(
                        prev_val,
                        prev_prev_val,
                        prev_prev_prev_val,
                        dt,
                        method_order,
                    )
                },
                |predicted| predicted[i],
            );
            if !curr_val.is_finite() || !predicted.is_finite() {
                return (Value::INFINITY, false);
            }

            // LTE estimate: |actual - predicted| with weighted SPICE-like scaling.
            let lte = (curr_val - predicted).abs();
            let reference = if self.reference == TransientLteReference::PredictorLocal {
                curr_val.abs().max(predicted.abs())
            } else {
                self.accepted_reference_magnitude(i, accepted_point_global)
            };
            // Xyce's accepted-solution error weights are formed directly as
            // `RELTOL * |x_ref| + ABSTOL`, and the resulting WRMS estimate is
            // compared with `errTolAcceptance = 1`.  Keep the predictor-local
            // metric on the legacy `(ABSTOL / RELTOL) + |x_ref|` scale.
            let normalized_lte = if self.uses_accepted_solution_reference() {
                lte / (self.reltol * reference + self.abstol).max(1.0e-30)
            } else {
                let scale = self.lte_scale_denominator(reference);
                lte / scale
            };

            if self.reference == TransientLteReference::PredictorLocal {
                aggregate = aggregate.max(normalized_lte);
            } else {
                aggregate += normalized_lte * normalized_lte;
            }
        }

        let raw_lte = if self.reference == TransientLteReference::PredictorLocal {
            aggregate
        } else {
            // Xyce's device-mask entries receive effectively infinite weights,
            // so they contribute zero to the sum but remain in the WRMS global
            // vector length used as the denominator.
            let one_over_len = 1.0 / len as Value;
            (aggregate * one_over_len).sqrt()
        };
        let lte = raw_lte * error_coefficient;
        let accept = if self.uses_accepted_solution_reference() {
            lte <= 1.0
        } else {
            lte <= self.reltol
        };
        (lte, accept)
    }

    /// Get recommended timestep scaling factor based on LTE
    /// Uses method order for proper scaling exponent
    pub fn recommend_scale(&self, lte: Value) -> Value {
        self.recommend_scale_for_order(lte, self.method_order)
    }

    /// Return the timestep scale for the method/order of the candidate step.
    pub(crate) fn recommend_scale_for_integration(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Value {
        self.recommend_scale_for_order(lte, Self::integration_method_order(method, trap_order))
    }

    /// Xyce's successful-step ratio policy for accepted-solution LTE modes.
    ///
    /// Ratios at or above two grow by exactly two, ratios between one and two
    /// keep the current step, and ratios at or below one shrink within Xyce's
    /// `[0.25, 0.9]` bounds.
    pub(crate) fn xyce_accepted_step_scale(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Value {
        let ratio =
            self.xyce_raw_step_ratio(lte, Self::integration_method_order(method, trap_order));
        if ratio >= 2.0 {
            2.0
        } else if ratio <= 1.0 {
            ratio.clamp(0.25, 0.9)
        } else {
            1.0
        }
    }

    /// Xyce's failed-LTE step ratio policy.
    pub(crate) fn xyce_rejected_step_scale(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
        first_failure: bool,
    ) -> Value {
        if !first_failure {
            return 0.25;
        }
        self.xyce_raw_step_ratio(lte, Self::integration_method_order(method, trap_order))
            .clamp(0.25, 0.9)
    }

    /// Whether Xyce's order-2 promotion test passes for the current error.
    pub(crate) fn xyce_should_promote_order_two(&self, lte: Value) -> bool {
        self.history_count >= 2 && self.xyce_raw_step_ratio(lte, 2) > 1.05
    }

    fn xyce_raw_step_ratio(&self, lte: Value, method_order: u32) -> Value {
        if !lte.is_finite() || lte < 0.0 {
            return 0.25;
        }
        let est_over_tol = if self.uses_accepted_solution_reference() {
            lte
        } else {
            lte / self.reltol.max(1.0e-30)
        };
        (0.5 / (est_over_tol + 0.0001)).powf(1.0 / (method_order.max(1) as Value + 1.0))
    }

    fn recommend_scale_for_order(&self, lte: Value, method_order: u32) -> Value {
        if !lte.is_finite() {
            0.25
        } else if lte < 1e-15 {
            2.0 // Error negligible, can increase
        } else {
            // Optimal scaling: (tol/lte)^(1/(p+1)) where p is method order
            // For order 2: exponent = 1/3, for order 1: exponent = 1/2
            let exponent = 1.0 / (method_order.max(1) as Value + 1.0);
            let ratio = self.reltol / lte;
            ratio.powf(exponent).clamp(0.25, 2.0)
        }
    }

    /// Charge-based LTE estimation for capacitive elements (MOSFET gates)
    ///
    /// For charge-conserving models (BSIM4, etc.), the fundamental relationship is:
    /// ```text
    /// dQ/dt = I
    /// ```
    ///
    /// This method estimates LTE based on charge conservation errors rather than
    /// voltage errors. This is more accurate for capacitive elements because:
    /// 1. It directly tracks the physical quantity being integrated (charge)
    /// 2. It catches errors in current integration that voltage-based LTE might miss
    /// 3. It ensures charge conservation across the simulation
    ///
    /// # Arguments
    /// * `charges` - Current charge values at each node
    /// * `prev_charges` - Charge values from previous timestep
    /// * `currents` - Current values (dQ/dt) at each node
    /// * `dt` - Current timestep
    ///
    /// # Returns
    /// (max_charge_lte, should_accept)
    ///
    /// # Theory
    /// For trapezoidal integration: Q_{n+1} = Q_n + dt/2 * (I_n + I_{n+1})
    /// The LTE is estimated as: |Q_actual - Q_predicted| / |Q_actual|
    pub fn estimate_charge_lte(
        &self,
        charges: &[Value],
        prev_charges: &[Value],
        currents: &[Value],
        prev_currents: &[Value],
        dt: Value,
    ) -> (Value, bool) {
        if charges.len() != prev_charges.len() || charges.is_empty() {
            return (0.0, true);
        }

        if charges.len() != currents.len() || charges.len() != prev_currents.len() {
            return (0.0, true);
        }

        let mut max_lte = 0.0_f64;

        // For each charge node, check conservation error
        for i in 0..charges.len() {
            let q_curr = charges[i];
            let q_prev = prev_charges[i];
            let i_curr = currents[i];
            let i_prev = prev_currents[i];

            // Trapezoidal prediction: Q_n+1 = Q_n + dt/2 * (I_n + I_{n+1})
            let q_predicted = q_prev + dt * 0.5 * (i_prev + i_curr);

            // Charge conservation error
            let charge_error = (q_curr - q_predicted).abs();

            // Normalize by charge magnitude (with minimum to avoid division issues)
            let q_magnitude = q_curr.abs().max(q_prev.abs()).max(1e-18);
            let normalized_lte = charge_error / q_magnitude;

            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Restart predictor history while retaining signal-history LTE references.
    pub fn restart_history(&mut self) {
        self.prev_solution.clear();
        self.prev_prev_solution.clear();
        self.prev_prev_prev_solution.clear();
        self.prev_dt = 0.0;
        self.prev_prev_dt = 0.0;
        self.history_count = 0;
        self.xyce_order_two_difference.clear();
        self.xyce_order_two_difference_dt = 0.0;
        self.xyce_attempt_dt = 0.0;
        self.xyce_attempt_prev_dt = 0.0;
        self.xyce_attempt_prev_prev_dt = 0.0;
        self.xyce_attempt_checkpoint = None;
    }

    /// Restart predictor history at an accepted breakpoint while retaining
    /// signal-history reference maxima.
    pub fn restart_history_from(&mut self, accepted_solution: &[Value]) {
        self.restart_history();
        self.prev_solution.extend_from_slice(accepted_solution);
        self.history_count = 1;
    }

    /// Seed the synthetic equal-step history used by a Xyce integration restart.
    ///
    /// OneStep/Gear12 restart with a zero first-difference vector, but initialize
    /// the previous step size to the first attempted post-breakpoint step. This
    /// makes the first predictor equal to the accepted breakpoint solution while
    /// retaining the correct `h / (h + h_prev)` LTE coefficient. Retries keep the
    /// original seed, matching Xyce's `lastTimeStep` restart state.
    pub(crate) fn seed_restart_timestep(&mut self, dt: Value) {
        if self.history_count == 1 && self.prev_dt == 0.0 && dt.is_finite() && dt > 0.0 {
            self.prev_dt = dt;
            if self.uses_accepted_solution_reference() {
                self.xyce_attempt_dt = dt;
            }
        }
    }

    /// Reset predictor history and all historical LTE references.
    pub fn reset(&mut self) {
        self.restart_history();
        self.accepted_reference_solution.clear();
        self.signal_global_reference = 0.0;
        self.signal_local_reference.clear();
    }
}

#[cfg(test)]
mod lte_estimator_tests {
    use super::*;

    fn accepted_estimator(
        reference: TransientLteReference,
        initial: &[Value],
        accepted: &[Value],
    ) -> LteEstimator {
        let mut estimator = LteEstimator::with_tolerances_and_reference(0.1, 1.0e-6, reference);
        estimator.seed_reference_prefix(initial, initial.len());
        estimator.record(accepted, 1.0);
        estimator
    }

    fn rich_accepted_boundary_estimator() -> (LteEstimator, Vec<Value>) {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        estimator.seed_initial_solution(&[10.0, -2.0]);

        estimator.begin_xyce_attempt(1.0, 1);
        estimator.record_with_order(&[3.0, 4.0], 2, 1.0, 1);

        estimator.begin_xyce_attempt(2.0, 2);
        let latest = vec![5.0, 1.0];
        estimator.record_with_order(&latest, latest.len(), 2.0, 2);
        assert!(estimator.xyce_attempt_checkpoint.is_none());
        (estimator, latest)
    }

    #[test]
    fn accepted_boundary_checkpoint_restores_next_predictor_and_lte_bit_exactly() {
        let (estimator, latest) = rich_accepted_boundary_estimator();
        let checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&latest)
            .expect("accepted-boundary state captures");
        assert_eq!(
            checkpoint.version,
            ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION
        );
        assert_eq!(checkpoint.history_count, 3);
        assert_eq!(checkpoint.xyce_order_two_difference, [-7.0, 6.0]);
        assert_eq!(checkpoint.xyce_attempt_dt, 2.0);
        assert_eq!(checkpoint.xyce_attempt_prev_dt, 1.0);
        assert_eq!(checkpoint.xyce_attempt_prev_prev_dt, 0.0);

        let expected_prediction = estimator
            .predict_solution(0.75, IntegrationMethod::Trapezoidal, 2)
            .expect("captured history predicts");
        let expected_lte = estimator.estimate(&[5.25, 0.5], 0.75);
        let expected_scale = estimator.recommend_scale(0.025);

        let mut restored = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        restored
            .restore_accepted_boundary_checkpoint(&checkpoint, &latest)
            .expect("validated checkpoint restores");
        assert_eq!(
            restored
                .capture_accepted_boundary_checkpoint(&latest)
                .expect("restored state recaptures"),
            checkpoint
        );
        let restored_prediction = restored
            .predict_solution(0.75, IntegrationMethod::Trapezoidal, 2)
            .expect("restored history predicts");
        assert!(lte_values_bit_equal(
            &restored_prediction,
            &expected_prediction
        ));
        let restored_lte = restored.estimate(&[5.25, 0.5], 0.75);
        assert_eq!(restored_lte.0.to_bits(), expected_lte.0.to_bits());
        assert_eq!(restored_lte.1, expected_lte.1);
        assert_eq!(
            restored.recommend_scale(0.025).to_bits(),
            expected_scale.to_bits()
        );
    }

    #[test]
    fn accepted_boundary_checkpoint_rejects_active_attempts_without_mutation() {
        let (mut source, latest) = rich_accepted_boundary_estimator();
        let checkpoint = source
            .capture_accepted_boundary_checkpoint(&latest)
            .expect("idle source captures");
        source.begin_xyce_attempt(3.0, 2);
        let capture_error = source
            .capture_accepted_boundary_checkpoint(&latest)
            .expect_err("an in-flight rollback checkpoint must not serialize");
        assert!(capture_error.contains("attempt is active"));

        let mut target = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        target.seed_initial_solution(&latest);
        target.begin_xyce_attempt(0.5, 1);
        let before = (
            target.prev_solution.clone(),
            target.xyce_attempt_dt,
            target.xyce_attempt_prev_dt,
            target.xyce_attempt_prev_prev_dt,
            target.xyce_attempt_checkpoint,
        );
        let restore_error = target
            .restore_accepted_boundary_checkpoint(&checkpoint, &latest)
            .expect_err("restore into an in-flight estimator must fail closed");
        assert!(restore_error.contains("attempt is active"));
        assert_eq!(
            (
                target.prev_solution.clone(),
                target.xyce_attempt_dt,
                target.xyce_attempt_prev_dt,
                target.xyce_attempt_prev_prev_dt,
                target.xyce_attempt_checkpoint,
            ),
            before
        );
    }

    #[test]
    fn force_accepted_record_canonicalizes_attempt_state_before_capture() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_initial_solution(&[0.0, 1.0]);
        estimator.begin_xyce_attempt(0.5, 1);
        assert!(estimator.xyce_attempt_checkpoint.is_some());

        let latest = [2.0, -1.0];
        estimator.record(&latest, 0.5);
        assert!(estimator.xyce_attempt_checkpoint.is_none());
        let checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&latest)
            .expect("a force-accepted record must leave canonical boundary state");
        assert_eq!(checkpoint.history_count, 2);
        assert_eq!(checkpoint.xyce_attempt_dt, 0.5);
        assert_eq!(checkpoint.xyce_attempt_prev_dt, 0.0);
        assert_eq!(checkpoint.xyce_attempt_prev_prev_dt, 0.0);
    }

    #[test]
    fn first_record_has_no_interval_and_second_record_starts_interval_history() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PredictorLocal,
        );

        let first = [1.0, -2.0];
        estimator.record(&first, 0.125);
        let first_checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&first)
            .expect("a one-solution history has no accepted interval");
        assert_eq!(first_checkpoint.history_count, 1);
        assert_eq!(first_checkpoint.prev_dt.to_bits(), 0.0_f64.to_bits());
        assert_eq!(first_checkpoint.prev_prev_dt.to_bits(), 0.0_f64.to_bits());

        let second = [3.0, 4.0];
        estimator.record(&second, 0.25);
        let second_checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&second)
            .expect("two accepted solutions retain exactly one interval");
        assert_eq!(second_checkpoint.history_count, 2);
        assert_eq!(second_checkpoint.prev_dt.to_bits(), 0.25_f64.to_bits());
        assert_eq!(second_checkpoint.prev_prev_dt.to_bits(), 0.0_f64.to_bits());

        let third = [5.0, 6.0];
        estimator.record(&third, 0.5);
        let third_checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&third)
            .expect("three accepted solutions retain two real intervals");
        assert_eq!(third_checkpoint.history_count, 3);
        assert_eq!(third_checkpoint.prev_dt.to_bits(), 0.5_f64.to_bits());
        assert_eq!(third_checkpoint.prev_prev_dt.to_bits(), 0.25_f64.to_bits());
    }

    #[test]
    fn synthetic_restart_timestep_does_not_become_an_accepted_interval() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        let restart = [1.0, -2.0];
        estimator.seed_initial_solution(&restart);
        estimator.seed_restart_timestep(0.125);

        let first_after_restart = [3.0, 4.0];
        estimator.record(&first_after_restart, 0.25);
        let first_checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&first_after_restart)
            .expect("the first post-restart point retains one real interval");
        assert_eq!(first_checkpoint.history_count, 2);
        assert_eq!(first_checkpoint.prev_dt.to_bits(), 0.25_f64.to_bits());
        assert_eq!(first_checkpoint.prev_prev_dt.to_bits(), 0.0_f64.to_bits());

        let second_after_restart = [5.0, 6.0];
        estimator.record(&second_after_restart, 0.5);
        let second_checkpoint = estimator
            .capture_accepted_boundary_checkpoint(&second_after_restart)
            .expect("the second post-restart point retains both real intervals");
        assert_eq!(second_checkpoint.history_count, 3);
        assert_eq!(second_checkpoint.prev_dt.to_bits(), 0.5_f64.to_bits());
        assert_eq!(second_checkpoint.prev_prev_dt.to_bits(), 0.25_f64.to_bits());
    }

    #[test]
    fn accepted_boundary_checkpoint_validates_shape_provenance_and_identity() {
        let (estimator, latest) = rich_accepted_boundary_estimator();
        let valid = estimator
            .capture_accepted_boundary_checkpoint(&latest)
            .expect("valid fixture captures");
        let rejects = |checkpoint: &AcceptedBoundaryLteEstimatorCheckpoint, message: &str| {
            let error = estimator
                .validate_accepted_boundary_checkpoint(checkpoint, &latest)
                .expect_err("corrupt accepted-boundary state must fail closed");
            assert!(error.contains(message), "unexpected error: {error}");
        };

        let mut corrupt = valid.clone();
        corrupt.version += 1;
        rejects(&corrupt, "version");

        let mut corrupt = valid.clone();
        corrupt.history_count = 4;
        rejects(&corrupt, "history count");

        let mut corrupt = valid.clone();
        corrupt.prev_prev_solution.pop();
        rejects(&corrupt, "second accepted solution history");

        let mut corrupt = valid.clone();
        corrupt.prev_solution[0] = 6.0;
        rejects(&corrupt, "does not equal");

        let mut corrupt = valid.clone();
        corrupt.prev_prev_dt = -0.0;
        rejects(&corrupt, "positive");

        let mut corrupt = valid.clone();
        corrupt.accepted_reference_solution[0] = 6.0;
        rejects(&corrupt, "reference solution does not equal");

        let mut corrupt = valid.clone();
        corrupt.signal_local_reference[0] = 4.0;
        rejects(&corrupt, "does not cover");

        let mut corrupt = valid.clone();
        corrupt.method_order = 3;
        rejects(&corrupt, "method order");

        let mut corrupt = valid.clone();
        corrupt.xyce_order_two_difference[0] = Value::NAN;
        rejects(&corrupt, "non-finite");

        let mut corrupt = valid.clone();
        corrupt.xyce_attempt_prev_dt = 0.0;
        corrupt.xyce_attempt_prev_prev_dt = 1.0;
        rejects(&corrupt, "zero provenance");

        let mut corrupt = valid;
        corrupt.xyce_attempt_dt = Value::NAN;
        rejects(&corrupt, "finite and nonnegative");

        let latest_mismatch = [5.0, 1.5];
        let error = estimator
            .validate_accepted_boundary_checkpoint(
                &estimator
                    .capture_accepted_boundary_checkpoint(&latest)
                    .expect("valid fixture captures"),
                &latest_mismatch,
            )
            .expect_err("caller-owned accepted solution must be authoritative");
        assert!(
            error.contains("does not equal"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn accepted_boundary_checkpoint_enforces_reference_mode_and_canonical_empty_state() {
        let empty = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PredictorLocal,
        );
        let latest = [1.0, -2.0];
        let checkpoint = empty
            .capture_accepted_boundary_checkpoint(&latest)
            .expect("canonical empty estimator captures");
        assert_eq!(checkpoint.solution_dimension, latest.len());
        assert_eq!(checkpoint.history_count, 0);

        let incompatible = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointLocal,
        );
        let error = incompatible
            .validate_accepted_boundary_checkpoint(&checkpoint, &latest)
            .expect_err("reference modes must agree");
        assert!(error.contains("reference mode mismatch"));

        let incompatible_tolerance = LteEstimator::with_tolerances_and_reference(
            0.2,
            1.0e-6,
            TransientLteReference::PredictorLocal,
        );
        let error = incompatible_tolerance
            .validate_accepted_boundary_checkpoint(&checkpoint, &latest)
            .expect_err("configuration that changes the next LTE decision must agree");
        assert!(error.contains("relative tolerance mismatch"));

        let mut negative_zero = checkpoint;
        negative_zero.prev_dt = -0.0;
        let error = empty
            .validate_accepted_boundary_checkpoint(&negative_zero, &latest)
            .expect_err("unused provenance must use canonical positive zero");
        assert!(error.contains("canonical +0"));
    }

    #[test]
    fn candidate_magnitude_cannot_relax_accepted_solution_weights() {
        let estimator = accepted_estimator(TransientLteReference::SignalGlobal, &[1.0], &[1.0]);
        let (small_lte, _) = estimator.estimate(&[2.0], 1.0);
        let (large_lte, _) = estimator.estimate(&[200.0], 1.0);

        assert!(large_lte > small_lte * 100.0);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            1.0f64.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [1.0]);
    }

    #[test]
    fn signal_global_remembers_a_peak_that_point_global_discards() {
        let point = accepted_estimator(TransientLteReference::PointGlobal, &[100.0], &[1.0]);
        let signal = accepted_estimator(TransientLteReference::SignalGlobal, &[100.0], &[1.0]);

        let (_, point_accepts) = point.estimate(&[1.2], 1.0);
        let (_, signal_accepts) = signal.estimate(&[1.2], 1.0);

        assert!(!point_accepts);
        assert!(signal_accepts);
    }

    #[test]
    fn signal_local_retains_independent_per_variable_peaks() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        estimator.seed_reference_prefix(&[10.0, 1.0], 2);
        estimator.record(&[2.0, 20.0], 1.0);
        estimator.record(&[3.0, 4.0], 1.0);

        assert_eq!(estimator.signal_local_reference, [10.0, 20.0]);
        assert_eq!(estimator.accepted_reference_solution, [3.0, 4.0]);
    }

    #[test]
    fn point_local_uses_each_prior_accepted_state_magnitude() {
        let estimator = accepted_estimator(
            TransientLteReference::PointLocal,
            &[1.0, 100.0],
            &[1.0, 100.0],
        );
        let candidate = [1.1, 100.1];

        let (first_lte, _) = estimator.estimate_prefix_excluding(&candidate, 2, 1.0, &[1]);
        let (second_lte, _) = estimator.estimate_prefix_excluding(&candidate, 2, 1.0, &[0]);

        assert!(first_lte > second_lte * 50.0);
    }

    #[test]
    fn accepted_solution_modes_include_the_full_solution_vector_in_wrms() {
        let estimator = accepted_estimator(
            TransientLteReference::SignalGlobal,
            &[1.0, 1.0],
            &[1.0, 1.0],
        );
        let candidate = [1.0, 1.2];

        let (nodal_prefix_lte, _) = estimator.estimate_prefix(&candidate, 1, 1.0);
        let (full_lte, _) = estimator.estimate(&candidate, 1.0);

        assert_eq!(nodal_prefix_lte, 0.0);
        assert!(full_lte > 0.0);
    }

    #[test]
    fn xyce_device_mask_zeroes_error_but_keeps_global_wrms_length() {
        let estimator = accepted_estimator(
            TransientLteReference::SignalGlobal,
            &[1.0, 100.0],
            &[1.0, 100.0],
        );

        let (lte, accepts) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.0, 200.0],
            &[1.0, 100.0],
            2,
            1.0,
            &[1],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn xyce_device_mask_retains_algebraic_values_in_point_global_reference() {
        let estimator = accepted_estimator(
            TransientLteReference::PointGlobal,
            &[1.0, 1000.0],
            &[1.0, 1000.0],
        );

        let (masked_lte, _) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.6, 2000.0],
            &[1.0, 1000.0],
            2,
            1.0,
            &[1],
            IntegrationMethod::Trapezoidal,
            2,
        );
        let (reference_only_lte, _) = estimator
            .estimate_correction_prefix_excluding_for_integration(
                &[1.6, 1000.0],
                &[1.0, 1000.0],
                2,
                1.0,
                &[1],
                IntegrationMethod::Trapezoidal,
                2,
            );
        let (unmasked_lte, _) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.6, 2000.0],
            &[1.0, 1000.0],
            2,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert_eq!(masked_lte.to_bits(), reference_only_lte.to_bits());
        assert!(unmasked_lte > masked_lte * 100.0);

        let without_large_algebraic_reference =
            accepted_estimator(TransientLteReference::PointGlobal, &[1.0, 1.0], &[1.0, 1.0]);
        let (local_scale_lte, _) = without_large_algebraic_reference
            .estimate_correction_prefix_excluding_for_integration(
                &[1.6, 1.0],
                &[1.0, 1.0],
                2,
                1.0,
                &[1],
                IntegrationMethod::Trapezoidal,
                2,
            );
        assert!(local_scale_lte > masked_lte * 100.0);
    }

    #[test]
    fn lte_uses_the_exact_predictor_vector_that_seeded_newton() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_initial_solution(&[0.0]);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[123.0],
            &[123.0],
            1,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn initial_state_seeds_reference_and_rejection_does_not_mutate_it() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&[1.0, -3.0], 2);
        estimator.record(&[0.5, -2.0], 1.0);
        let before = estimator.signal_reference_snapshot().0;

        let (_, accepts) = estimator.estimate(&[Value::INFINITY, 1000.0], 1.0);

        assert!(!accepts);
        assert_eq!(before.to_bits(), 3.0f64.to_bits());
        assert_eq!(
            estimator.signal_reference_snapshot().0.to_bits(),
            before.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [0.5, -2.0]);
    }

    #[test]
    fn initial_state_seeds_predictor_history_for_the_second_step() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_initial_solution(&[0.0]);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate_prefix_excluding_for_integration(
            &[2.0],
            1,
            1.0,
            &[],
            IntegrationMethod::BackwardEuler,
            1,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn restart_preserves_signal_reference_while_reset_clears_it() {
        let mut estimator = accepted_estimator(TransientLteReference::SignalGlobal, &[5.0], &[1.0]);

        estimator.restart_history();
        assert_eq!(estimator.history_count, 0);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            5.0f64.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [1.0]);

        estimator.restart_history_from(&[1.0]);
        assert_eq!(estimator.history_count, 1);
        assert_eq!(estimator.prev_solution, [1.0]);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            5.0f64.to_bits()
        );

        estimator.reset();
        assert_eq!(estimator.signal_global_reference, 0.0);
        assert!(estimator.accepted_reference_solution.is_empty());
    }

    #[test]
    fn xyce_restart_timestep_seed_is_stable_across_retries() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_reference_prefix(&[7.0, -2.0], 2);
        estimator.restart_history_from(&[7.0, -2.0]);

        estimator.seed_restart_timestep(0.25);
        estimator.seed_restart_timestep(0.125);

        assert_eq!(estimator.prev_dt.to_bits(), 0.25f64.to_bits());
        assert_eq!(
            estimator
                .predict_solution(0.125, IntegrationMethod::BackwardEuler, 1)
                .expect("restart history must predict from the accepted breakpoint"),
            [7.0, -2.0]
        );
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::BackwardEuler, 1, 0.125)
                .to_bits(),
            (1.0f64 / 3.0).to_bits()
        );
    }

    #[test]
    fn predictor_local_metric_retains_legacy_candidate_scaling() {
        let mut estimator = LteEstimator::with_tolerances(0.1, 0.01);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate(&[2.0], 1.0);

        assert!((lte - 1.0 / 2.1).abs() <= 1.0e-15);
        assert!(!accepts);
    }

    #[test]
    fn non_finite_candidate_fails_closed() {
        let estimator = accepted_estimator(TransientLteReference::PointGlobal, &[1.0], &[1.0]);

        let (lte, accepts) = estimator.estimate(&[Value::NAN], 1.0);

        assert!(lte.is_infinite());
        assert!(!accepts);
    }

    #[test]
    fn timeint_absolute_tolerance_changes_the_weight_and_decision() {
        let mut strict = LteEstimator::with_tolerances_and_reference(
            0.1,
            0.001,
            TransientLteReference::PointGlobal,
        );
        let mut loose = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0,
            TransientLteReference::PointGlobal,
        );
        strict.seed_reference_prefix(&[0.0], 1);
        loose.seed_reference_prefix(&[0.0], 1);
        strict.record(&[0.0], 1.0);
        loose.record(&[0.0], 1.0);

        assert!(!strict.estimate(&[0.05], 1.0).1);
        assert!(loose.estimate(&[0.05], 1.0).1);
    }

    #[test]
    fn xyce_integration_error_coefficients_follow_method_and_step_history() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_initial_solution(&[0.0]);
        estimator.record(&[0.0], 2.0);

        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::BackwardEuler, 1, 1.0)
                .to_bits(),
            (1.0f64 / 3.0).to_bits()
        );
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::Trapezoidal, 2, 1.0)
                .to_bits(),
            (1.0f64 / 9.0).to_bits()
        );

        estimator.record(&[0.0], 3.0);
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::Gear2, 2, 1.0)
                .to_bits(),
            (1.0f64 / 6.0).to_bits()
        );

        let mut native = LteEstimator::with_tolerances(0.1, 1.0e-6);
        native.record(&[0.0], 2.0);
        assert_eq!(
            native.integration_error_coefficient(IntegrationMethod::Trapezoidal, 2, 1.0),
            1.0
        );
    }

    #[test]
    fn candidate_method_order_controls_prediction_and_scale_exponent() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&[0.0], 1);
        estimator.record(&[0.0], 1.0);
        estimator.record(&[1.0], 1.0);
        estimator.record(&[4.0], 1.0);

        let (order_one_lte, _) = estimator.estimate_prefix_excluding_for_integration(
            &[9.0],
            1,
            1.0,
            &[],
            IntegrationMethod::BackwardEuler,
            1,
        );
        let (order_two_lte, _) = estimator.estimate_prefix_excluding_for_integration(
            &[9.0],
            1,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert!(order_one_lte > 0.0);
        assert_eq!(order_two_lte, 0.0);
        let order_one_scale =
            estimator.recommend_scale_for_integration(0.4, IntegrationMethod::BackwardEuler, 1);
        let order_two_scale =
            estimator.recommend_scale_for_integration(0.4, IntegrationMethod::Trapezoidal, 2);
        assert!((order_one_scale - 0.5).abs() <= 1.0e-15);
        assert!((order_two_scale - 0.25f64.powf(1.0 / 3.0)).abs() <= 1.0e-15);
    }

    #[test]
    fn xyce_step_ratio_policy_matches_success_and_failure_bounds() {
        let estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );

        assert_eq!(
            estimator.xyce_accepted_step_scale(1.0e-4, IntegrationMethod::Trapezoidal, 2),
            2.0
        );
        assert_eq!(
            estimator.xyce_accepted_step_scale(0.1, IntegrationMethod::Trapezoidal, 2),
            1.0
        );

        let boundary_scale =
            estimator.xyce_accepted_step_scale(1.0, IntegrationMethod::Trapezoidal, 2);
        let expected = (0.5f64 / 1.0001).powf(1.0 / 3.0);
        assert!((boundary_scale - expected).abs() <= 1.0e-15);

        let first_reject =
            estimator.xyce_rejected_step_scale(1.0, IntegrationMethod::Trapezoidal, 2, true);
        assert!(first_reject > 0.25 && first_reject < 0.9);
        assert_eq!(
            estimator.xyce_rejected_step_scale(1.0, IntegrationMethod::Trapezoidal, 2, false),
            0.25
        );
    }

    #[test]
    fn xyce_attempt_rollback_replays_one_step_restore_history_shift() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.xyce_attempt_dt = 3.0;
        estimator.xyce_attempt_prev_dt = 2.0;
        estimator.xyce_attempt_prev_prev_dt = 1.0;

        estimator.begin_xyce_attempt(4.0, 2);
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (4.0, 3.0, 2.0)
        );
        estimator.rollback_xyce_attempt();
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (3.0, 2.0, 2.0)
        );

        estimator.begin_xyce_attempt(5.0, 1);
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (5.0, 3.0, 2.0)
        );
        estimator.rollback_xyce_attempt();
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (3.0, 3.0, 2.0)
        );

        // Exercise an order-one retry without a preceding order-two shift:
        // psi[2] must remain the checkpoint's third coefficient.
        estimator.xyce_attempt_dt = 3.0;
        estimator.xyce_attempt_prev_dt = 2.0;
        estimator.xyce_attempt_prev_prev_dt = 1.0;
        estimator.begin_xyce_attempt(5.0, 1);
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (5.0, 3.0, 1.0)
        );
        estimator.rollback_xyce_attempt();
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (3.0, 3.0, 1.0)
        );
    }

    #[test]
    fn xyce_iteration_control_rejection_retains_attempted_history() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.xyce_attempt_dt = 3.0;
        estimator.xyce_attempt_prev_dt = 2.0;
        estimator.xyce_attempt_prev_prev_dt = 1.0;

        estimator.begin_xyce_attempt(4.0, 2);
        estimator.reject_xyce_attempt(false);
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (4.0, 3.0, 2.0)
        );

        estimator.begin_xyce_attempt(5.0, 2);
        assert_eq!(
            (
                estimator.xyce_attempt_dt,
                estimator.xyce_attempt_prev_dt,
                estimator.xyce_attempt_prev_prev_dt,
            ),
            (5.0, 4.0, 3.0),
            "the next attempt must advance from the rejected mode-1 attempt"
        );
    }
}
