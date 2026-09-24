//! Authored optimization terms and authenticated observations retained with results.
//!
//! These value types, scoring rules, and evidence checks are shared by the
//! optimizer and persisted result model. They do not depend on execution state.

mod constraint;
mod objective;

pub use constraint::{
    OptimizationConstraint, OptimizationConstraintObservation, OptimizationScore,
    validate_optimization_constraint_result, validate_optimization_constraints,
};
pub use objective::{
    OptimizationObjectiveGoal, OptimizationObjectiveObservation, OptimizationObjectiveTerm,
    validate_optimization_objectives,
};

/// Blank retains the original measurement scale.
pub fn validate_requested_unit(unit: &str) -> Result<(), String> {
    if unit.chars().any(char::is_control) {
        return Err("Optimization units must not contain control characters".into());
    }
    if !unit.trim().is_empty() {
        rspice_units::MeasurementUnit::known(unit)?;
    }
    Ok(())
}
