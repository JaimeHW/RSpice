//! Translating engine errors into user-facing ones.
//!
//! Engine diagnostics name internal nodes and matrix rows. This maps them
//! back onto the design objects a user can act on.

use super::EngineBridge;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Translate core engine error to UI error.
    pub(super) fn translate_error(&self, err: rspice_core::SimulationError) -> SimulationError {
        match err {
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => {
                SimulationError::ResourceLimit {
                    resource: error.resource.as_str().to_string(),
                    requested: error.requested,
                    limit: error.limit,
                }
            }
            rspice_core::SimulationError::Configuration(error) => {
                SimulationError::InvalidConfig(error.to_string())
            }
            rspice_core::SimulationError::BehavioralReference(error) => {
                SimulationError::BehavioralReference {
                    owner_name: error.owner_name,
                    canonical_owner_name: error.canonical_owner_name,
                    dependency_name: error.dependency_name,
                    canonical_dependency_name: error.canonical_dependency_name,
                    reason: error.reason.as_str().to_string(),
                }
            }
            rspice_core::SimulationError::Circuit(msg) => SimulationError::CircuitError(msg),
            rspice_core::SimulationError::Solver(solver_err) => {
                SimulationError::SolverError(solver_err.to_string())
            }
            rspice_core::SimulationError::Netlist(msg) => SimulationError::ParseError(msg),
            rspice_core::SimulationError::ConvergenceFailed(iterations) => {
                SimulationError::ConvergenceFailed {
                    iterations,
                    message: "Newton-Raphson iteration limit exceeded".to_string(),
                }
            }
            rspice_core::SimulationError::Aborted => SimulationError::Aborted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behavioral_reference_error_preserves_typed_fields() {
        let core_error = rspice_core::SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        ));
        let translated = EngineBridge::new().translate_error(core_error);

        assert_eq!(
            translated,
            SimulationError::BehavioralReference {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason: "lead_current_not_solution_variable".to_string(),
            }
        );
        assert_eq!(
            translated.to_string(),
            "Device instance B2: Problem with value for B1 in B2 \
             (lead_current_not_solution_variable)"
        );
    }
}
