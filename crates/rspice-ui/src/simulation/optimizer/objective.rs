//! Normalized, weighted objectives and their retained values at the best design.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationObjectiveGoal {
    Minimize,
    Maximize,
    Target,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimizationObjectiveTerm {
    pub measurement: String,
    pub goal: OptimizationObjectiveGoal,
    pub target: Option<f64>,
    /// Normalization in the measurement's physical units.
    pub scale: f64,
    /// Positive dimensionless importance.
    pub weight: f64,
}

impl OptimizationObjectiveTerm {
    pub fn validate(&self) -> Result<(), String> {
        if self.measurement.trim().is_empty() || self.measurement.chars().any(char::is_control) {
            return Err("Each optimization objective needs a measurement name".into());
        }
        if !self.scale.is_finite() || self.scale <= 0.0 {
            return Err("Objective scale must be finite and positive".into());
        }
        if !self.weight.is_finite() || self.weight <= 0.0 {
            return Err("Objective weight must be finite and positive".into());
        }
        if self.goal == OptimizationObjectiveGoal::Target && self.target.is_none()
            || self.target.is_some_and(|value| !value.is_finite())
        {
            return Err("Target objectives require a finite target".into());
        }
        Ok(())
    }

    pub fn contribution(&self, value: f64) -> Result<f64, String> {
        self.validate()?;
        let normalized = match self.goal {
            OptimizationObjectiveGoal::Minimize => value / self.scale,
            OptimizationObjectiveGoal::Maximize => -value / self.scale,
            OptimizationObjectiveGoal::Target => {
                ((value - self.target.unwrap()) / self.scale).powi(2)
            }
        };
        let contribution = self.weight * normalized;
        if value.is_finite() && contribution.is_finite() {
            Ok(contribution)
        } else {
            Err(format!(
                "Objective {:?} has a non-finite value or weighted cost",
                self.measurement
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimizationObjectiveObservation {
    pub objective: OptimizationObjectiveTerm,
    pub value: f64,
    pub contribution: f64,
}

/// Validate retained evidence, including its relation to the combined best cost.
pub fn validate_optimization_objectives(
    observations: &[OptimizationObjectiveObservation],
    best_cost: f64,
) -> Result<(), String> {
    if observations.is_empty() {
        return Ok(());
    }
    let mut total = 0.0;
    for observation in observations {
        let expected = observation.objective.contribution(observation.value)?;
        if expected.to_bits() != observation.contribution.to_bits() {
            return Err(
                "Optimization objective contribution disagrees with its value and configuration"
                    .into(),
            );
        }
        total += expected;
    }
    if !total.is_finite() || total.to_bits() != best_cost.to_bits() {
        return Err("Optimization objective contributions disagree with the best cost".into());
    }
    Ok(())
}
