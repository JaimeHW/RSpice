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
    /// Stable identity of the failing analysis card, e.g. `"ac-002"`.
    analysis_id: Option<String>,
    /// Stable identity of the failing run coordinate.
    coordinate_id: Option<String>,
    /// Netlist line the offending construct was authored on.
    line: Option<usize>,
    /// Netlist file the offending construct was authored in.
    path: Option<String>,
    /// Dotted token naming a refused capability boundary.
    capability: Option<&'static str>,
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
    let capability = match error {
        rspice_core::engine::SimulationError::UnsupportedCapability(refusal) => {
            Some(refusal.capability)
        }
        _ => None,
    };
    let location = error.source_location();
    SimulationErrorAttributes {
        kind,
        code: descriptor.code.as_str(),
        category: descriptor.category.as_str(),
        retryable: descriptor.retryable,
        analysis_id: descriptor.analysis.map(|id| id.tag()),
        coordinate_id: descriptor.coordinate.map(|id| id.to_string()),
        line: location.map(|location| location.line),
        path: location.and_then(|location| location.path.as_deref().map(super::public_path_string)),
        capability,
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
        // An expired budget is not a cancellation, and a caller decides
        // differently about the two: one is retried as-is, the other needs a
        // longer budget or a smaller deck.
        CoreSimulationError::TimeLimitExceeded => TimeoutError::new_err(err.to_string()),
        _ => SimulationError::new_err(err.to_string()),
    };
    let _attribute_result = Python::attach(|py| {
        let value = error.value(py);
        value.setattr("kind", attributes.kind)?;
        value.setattr("code", attributes.code)?;
        value.setattr("category", attributes.category)?;
        value.setattr("retryable", attributes.retryable)?;
        value.setattr("analysis_id", attributes.analysis_id)?;
        value.setattr("coordinate_id", attributes.coordinate_id)?;
        value.setattr("line", attributes.line)?;
        value.setattr("path", attributes.path)?;
        value.setattr("capability", attributes.capability)?;
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
    fn capability_refusals_publish_their_token_and_source_span() {
        let attributes = simulation_error_attributes(&rspice_core::engine::SimulationError::from(
            rspice_core::UnsupportedCapabilityError::new(
                "device.ltra.rg_finite_length",
                "finite-length RG LTRA is not stamped",
            )
            .at(rspice_core::netlist::NetlistSourceLocation::in_file(
                "deck.cir", 12,
            )),
        ));
        assert_eq!(attributes.kind, "unsupported_capability");
        assert_eq!(attributes.category, "capability");
        assert_eq!(attributes.capability, Some("device.ltra.rg_finite_length"));
        assert_eq!(attributes.line, Some(12));
        assert_eq!(attributes.path.as_deref(), Some("deck.cir"));
        assert!(!attributes.retryable);
    }

    #[test]
    fn an_expired_budget_is_not_reported_as_a_cancellation() {
        let cancelled = simulation_error_attributes(&rspice_core::engine::SimulationError::Aborted);
        assert_eq!(cancelled.category, "cancellation");
        assert_eq!(cancelled.code, "aborted");

        let expired =
            simulation_error_attributes(&rspice_core::engine::SimulationError::TimeLimitExceeded);
        assert_eq!(expired.category, "timeout");
        assert_eq!(expired.code, "time_limit_exceeded");
        assert!(
            expired.retryable,
            "a budget the caller can raise is safe to retry"
        );
    }

    #[test]
    fn result_errors_publish_their_typed_identities() {
        let netlist =
            rspice_core::Netlist::parse("identity\nV1 in 0 1\nR1 in 0 1k\n.ac dec 1 1 10\n.end\n")
                .expect("identity deck parses");
        let plan = rspice_core::execution::DeckPlan::from_netlist_with_abort(
            &netlist,
            &rspice_core::ResourceLimits::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("canonical deck plan");
        let analysis = plan.analyses()[0].id();

        let attributes = simulation_error_attributes(&rspice_core::engine::SimulationError::from(
            rspice_core::RequestedSignalUnavailableError::new("@M1[Id]", "AC", None)
                .with_analysis(analysis),
        ));
        assert_eq!(attributes.category, "signal_unavailable");
        assert_eq!(attributes.analysis_id.as_deref(), Some("ac-001"));
        assert_eq!(attributes.coordinate_id, None);
    }

    #[test]
    fn simulation_error_stub_exposes_the_published_attributes() {
        let stub = include_str!("../../rspice.pyi");
        let simulation_error = stub
            .split("class SimulationError(RSpiceError):")
            .nth(1)
            .and_then(|tail| tail.split("class ConvergenceError").next())
            .expect("SimulationError stub block exists");
        for field in [
            "analysis_id: str | None",
            "coordinate_id: str | None",
            "line: int | None",
            "path: str | None",
            "capability: str | None",
            "instance_name: str | None",
            "canonical_instance_name: str | None",
            "missing_dependency: str | None",
            "reason: str | None",
        ] {
            assert!(simulation_error.contains(field), "stub omitted {field}");
        }
        assert!(
            stub.contains("class TimeoutError(CancelledError):"),
            "the stub must publish the timeout distinction"
        );
        assert!(
            stub.contains("\"TimeoutError\","),
            "the stub's __all__ must list TimeoutError"
        );
    }
}
