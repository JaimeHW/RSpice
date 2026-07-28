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
    SimulationErrorAttributes {
        kind,
        code: descriptor.code.as_str(),
        category: descriptor.category.as_str(),
        retryable: descriptor.retryable,
        iterations: descriptor.iterations,
        resource,
        requested,
        limit,
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
    }
}
