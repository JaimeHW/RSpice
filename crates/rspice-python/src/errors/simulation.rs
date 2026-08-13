//! Mapping `SimulationError` onto typed Python exceptions.
//!
//! Convergence failures become `ConvergenceError` and cancellation becomes
//! `CancelledError`, so a caller can retry the first and must not retry the
//! second. Everything else becomes `SimulationError` with its structured
//! detail attached.

use super::*;

#[derive(Debug, PartialEq, Eq)]
struct SimulationErrorAttributes {
    kind: &'static str,
    code: &'static str,
    category: &'static str,
    retryable: bool,
    iterations: Option<usize>,
    resource: Option<&'static str>,
    requested: Option<usize>,
    limit: Option<usize>,
    instance_name: Option<String>,
    canonical_instance_name: Option<String>,
    missing_dependency: Option<String>,
    reason: Option<&'static str>,
}

fn simulation_error_attributes(
    error: &rspice_core::engine::SimulationError,
) -> SimulationErrorAttributes {
    let descriptor = error.descriptor();
    // Preserve the original Python `kind` vocabulary while publishing the
    // shared cross-interface `code` alongside it.
    let kind = match descriptor.code.as_str() {
        "invalid_configuration" => "configuration",
        "circuit_error" => "circuit",
        "solver_error" => "solver",
        "netlist_error" => "netlist",
        "convergence_error" => "convergence",
        other => other,
    };
    let (resource, requested, limit) =
        descriptor
            .resource_limit
            .map_or((None, None, None), |error| {
                (
                    Some(error.resource.as_str()),
                    Some(error.requested),
                    Some(error.limit),
                )
            });
    let (instance_name, canonical_instance_name, missing_dependency, reason) = match error {
        rspice_core::engine::SimulationError::BehavioralReference(error) => (
            Some(error.owner_name.clone()),
            Some(error.canonical_owner_name.clone()),
            Some(error.canonical_dependency_name.clone()),
            Some(error.reason.as_str()),
        ),
        _ => (None, None, None, None),
    };
    SimulationErrorAttributes {
        kind,
        code: descriptor.code.as_str(),
        category: descriptor.category.as_str(),
        retryable: descriptor.retryable,
        iterations: descriptor.iterations,
        resource,
        requested,
        limit,
        instance_name,
        canonical_instance_name,
        missing_dependency,
        reason,
    }
}

/// Convert a simulation error to PyErr
pub fn simulation_error_to_pyerr(err: rspice_core::engine::SimulationError) -> PyErr {
    use rspice_core::engine::SimulationError as CoreSimulationError;

    let attributes = simulation_error_attributes(&err);
    let error = match &err {
        CoreSimulationError::ConvergenceFailed(_) => ConvergenceError::new_err(err.to_string()),
        CoreSimulationError::Aborted => CancelledError::new_err(err.to_string()),
        _ => SimulationError::new_err(err.to_string()),
    };
    let _attribute_result = Python::attach(|py| {
        let value = error.value(py);
        value.setattr("kind", attributes.kind)?;
        value.setattr("code", attributes.code)?;
        value.setattr("category", attributes.category)?;
        value.setattr("retryable", attributes.retryable)?;
        value.setattr("iterations", attributes.iterations)?;
        value.setattr("resource", attributes.resource)?;
        value.setattr("requested", attributes.requested)?;
        value.setattr("limit", attributes.limit)?;
        value.setattr("instance_name", attributes.instance_name)?;
        value.setattr(
            "canonical_instance_name",
            attributes.canonical_instance_name,
        )?;
        value.setattr("missing_dependency", attributes.missing_dependency)?;
        value.setattr("reason", attributes.reason)?;
        Ok::<_, PyErr>(())
    });
    error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_exceptions_publish_shared_error_contract() {
        let attributes = simulation_error_attributes(
            &rspice_core::engine::SimulationError::ConvergenceFailed(19),
        );
        assert_eq!(attributes.kind, "convergence");
        assert_eq!(attributes.code, "convergence_error");
        assert_eq!(attributes.category, "convergence");
        assert!(!attributes.retryable);
        assert_eq!(attributes.iterations, Some(19));
        assert_eq!(attributes.resource, None);
        assert_eq!(attributes.instance_name, None);
    }

    #[test]
    fn behavioral_reference_exception_preserves_typed_identity() {
        let attributes = simulation_error_attributes(
            &rspice_core::engine::SimulationError::BehavioralReference(Box::new(
                rspice_core::device::BehavioralReferenceError {
                    owner_name: "b2".to_string(),
                    canonical_owner_name: "B2".to_string(),
                    dependency_name: "b1".to_string(),
                    canonical_dependency_name: "B1".to_string(),
                    reason: rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
                },
            )),
        );
        assert_eq!(attributes.kind, "behavioral_reference_error");
        assert_eq!(attributes.instance_name.as_deref(), Some("b2"));
        assert_eq!(attributes.canonical_instance_name.as_deref(), Some("B2"));
        assert_eq!(attributes.missing_dependency.as_deref(), Some("B1"));
        assert_eq!(
            attributes.reason,
            Some("lead_current_not_solution_variable")
        );
    }

    #[test]
    fn simulation_error_stub_exposes_behavioral_reference_fields() {
        let stub = include_str!("../../rspice.pyi");
        let simulation_error = stub
            .split("class SimulationError(RSpiceError):")
            .nth(1)
            .and_then(|tail| tail.split("class ConvergenceError").next())
            .expect("SimulationError stub block exists");
        for field in [
            "instance_name: str | None",
            "canonical_instance_name: str | None",
            "missing_dependency: str | None",
            "reason: str | None",
        ] {
            assert!(simulation_error.contains(field), "stub omitted {field}");
        }
    }
}
