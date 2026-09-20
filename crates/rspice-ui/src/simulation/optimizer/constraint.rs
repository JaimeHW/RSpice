//! Hard measurement limits and feasibility-first candidate ordering.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimizationConstraint {
    pub measurement: String,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    /// Absolute allowance in the measurement's physical units.
    pub tolerance: f64,
    /// Physical normalization used to compare infeasible candidates.
    pub scale: f64,
}
impl OptimizationConstraint {
    pub fn validate(&self) -> Result<(), String> {
        if self.measurement.trim().is_empty() || self.measurement.chars().any(char::is_control) {
            return Err("Each constraint needs a measurement name".into());
        }
        if self.lower.is_none() && self.upper.is_none()
            || self.lower.is_some_and(|v| !v.is_finite())
            || self.upper.is_some_and(|v| !v.is_finite())
            || matches!((self.lower, self.upper), (Some(lo), Some(hi)) if lo > hi)
        {
            return Err(
                "Constraint limits must be finite and ordered, with at least one limit".into(),
            );
        }
        if !self.tolerance.is_finite()
            || self.tolerance < 0.0
            || !self.scale.is_finite()
            || self.scale <= 0.0
        {
            return Err("Constraint tolerance must be finite and nonnegative; scale must be finite and positive".into());
        }
        Ok(())
    }
    pub fn violation(&self, value: f64) -> Result<f64, String> {
        self.validate()?;
        let below = self
            .lower
            .map_or(0.0, |lo| (lo - value - self.tolerance).max(0.0));
        let above = self
            .upper
            .map_or(0.0, |hi| (value - hi - self.tolerance).max(0.0));
        let excess = below.max(above);
        let mut violation = excess / self.scale;
        // Scaling changes ranking, never whether a hard limit is satisfied.
        if excess > 0.0 && violation == 0.0 {
            violation = f64::from_bits(1);
        }
        if value.is_finite() && violation.is_finite() {
            Ok(violation)
        } else {
            Err(format!(
                "Constraint {:?} has a non-finite value or violation",
                self.measurement
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimizationConstraintObservation {
    pub constraint: OptimizationConstraint,
    pub value: f64,
    pub violation: f64,
}

/// Recompute retained evidence before it can be used to claim feasibility.
pub fn validate_optimization_constraints(
    observations: &[OptimizationConstraintObservation],
) -> Result<f64, String> {
    let mut total = 0.0;
    for observation in observations {
        let expected = observation.constraint.violation(observation.value)?;
        if expected.to_bits() != observation.violation.to_bits() {
            return Err(
                "Optimization constraint violation disagrees with its value and limits".into(),
            );
        }
        total += expected;
    }
    if !total.is_finite() {
        return Err("Combined constraint violation is non-finite".into());
    }
    Ok(total)
}

/// Lexicographic order: feasibility, total normalized violation, objective cost.
/// No finite objective reward can buy permission to violate a hard limit.
#[derive(Debug, Clone, Copy)]
pub struct OptimizationScore {
    pub cost: f64,
    pub violation: f64,
}
impl From<f64> for OptimizationScore {
    fn from(cost: f64) -> Self {
        Self {
            cost,
            violation: if cost.is_finite() { 0.0 } else { f64::INFINITY },
        }
    }
}
impl OptimizationScore {
    pub fn is_valid(self) -> bool {
        self.cost.is_finite() && self.violation.is_finite() && self.violation >= 0.0
    }
    pub fn better_than(self, other: Self) -> bool {
        self.is_valid()
            && (!other.is_valid()
                || self.violation < other.violation
                || self.violation == other.violation && self.cost < other.cost)
    }
    pub(super) fn phase_value(self, feasibility: bool) -> f64 {
        if !self.is_valid() {
            f64::INFINITY
        } else if feasibility {
            self.violation
        } else if self.violation == 0.0 {
            self.cost
        } else {
            f64::INFINITY
        }
    }
}

pub fn validate_optimization_constraint_result(
    observations: &[OptimizationConstraintObservation],
    converged: bool,
) -> Result<(), String> {
    if validate_optimization_constraints(observations)? > 0.0 && converged {
        return Err("An infeasible optimization result cannot claim convergence".into());
    }
    Ok(())
}
