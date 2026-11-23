//! Preserve shared-runner failure classes at the process boundary.

use crate::cli::{CliError, ErrorDetails, FailureCategory};
use rspice_core::SimulationErrorCategory as Category;
use rspice_simulation::error::SimulationError;

pub(super) fn classified(message: String, code: &'static str, category: Category) -> CliError {
    let category = FailureCategory::Engine(category);
    CliError::Reported {
        message,
        details: Box::new(ErrorDetails::new(code, category.as_str(), false)),
        category,
    }
}

pub(super) fn execution(error: SimulationError, task: &str, timeout: Option<f64>) -> CliError {
    use SimulationError as E;
    if error == E::Aborted {
        return super::super::run::cancellation_cli_error(timeout);
    }
    let (code, category) = match &error {
        E::ParseError(_) | E::BehavioralReference { .. } => ("netlist_error", Category::Netlist),
        E::CircuitError(_) => ("circuit_error", Category::Simulation),
        E::Elaboration { kind, .. } => match kind.as_str() {
            "cache_corrupt" | "internal" => ("circuit_error", Category::Simulation),
            _ => ("netlist_error", Category::Netlist),
        },
        E::UnsupportedCapability { .. } => ("unsupported_capability", Category::Capability),
        E::SolverError(_) => ("solver_error", Category::Solver),
        E::RequestedSignalUnavailable { .. } => {
            ("requested_signal_unavailable", Category::SignalUnavailable)
        }
        E::ResultSchemaMismatch(_) => ("result_schema_mismatch", Category::ResultSchema),
        E::ConvergenceFailed { .. } => ("convergence_failed", Category::Convergence),
        E::Attributed { attribution, .. } => {
            use rspice_results::convergence_attribution::ConvergenceFailureClass;
            match attribution.class {
                ConvergenceFailureClass::SingularSystem => ("solver_error", Category::Solver),
                _ => ("convergence_failed", Category::Convergence),
            }
        }
        E::InvalidConfig(_) => ("invalid_configuration", Category::Configuration),
        E::UnsupportedOutcome(_) => ("unsupported_outcome", Category::Capability),
        E::ResourceLimit { .. } => ("resource_limit", Category::ResourceLimit),
        E::ResourceFailure(failure) => (failure.code(), Category::ResourceLimit),
        E::ResultStorage(failure) => (failure.code(), failure.category()),
        E::AlreadyRunning | E::ThreadPanic | E::EngineStopped(_) => {
            return CliError::InternalError {
                message: error.to_string(),
            };
        }
        E::Aborted => unreachable!("handled above"),
    };
    let category = FailureCategory::Engine(category);
    let mut details = ErrorDetails::new(code, category.as_str(), false);
    details.analysis_id = Some(task.into());
    match &error {
        E::ConvergenceFailed { iterations, .. } => details.iterations = Some(*iterations),
        E::RequestedSignalUnavailable {
            analysis,
            coordinate,
            ..
        } => {
            details.analysis = Some(analysis.clone());
            details.coordinate_id = coordinate.clone();
        }
        E::ResourceLimit {
            resource,
            requested,
            limit,
        } => {
            details.resource = resource_name(resource);
            details.requested = Some(*requested);
            details.limit = Some(*limit);
        }
        E::ResourceFailure(rspice_simulation::error::ResourceFailure::DeviceLimit {
            instance,
            resource,
            requested,
            limit,
        }) => {
            details.instance_name = Some(instance.clone());
            details.resource = resource_name(resource);
            details.requested = Some(*requested);
            details.limit = Some(*limit);
        }
        E::BehavioralReference {
            owner_name,
            canonical_owner_name,
            canonical_dependency_name,
            ..
        } => {
            details.instance_name = Some(owner_name.clone());
            details.canonical_instance_name = Some(canonical_owner_name.clone());
            details.missing_dependency = Some(canonical_dependency_name.clone());
        }
        E::Elaboration { instance, .. } => details.instance_name = instance.clone(),
        // The token stays in the message: this schema carries it as a
        // static string, which a shared-runner refusal no longer is.
        E::UnsupportedCapability {
            analysis_kind,
            instance,
            ..
        } => {
            details.instance_name = instance.clone();
            details.analysis = analysis_kind.clone();
        }
        _ => {}
    }
    CliError::Reported {
        message: said(&error),
        category,
        details: Box::new(details),
    }
}

/// What `error` says, a limit the run went past said as the desktop app
/// says it, then in the engine's words.
fn said(error: &SimulationError) -> String {
    let refusal = |resource: &str, requested: usize, limit: usize| {
        resource_kind(resource).map(|resource| {
            crate::cli::refusal::said(&rspice_core::ResourceLimitError {
                resource,
                requested,
                limit,
            })
        })
    };
    match error {
        SimulationError::ResourceLimit {
            resource,
            requested,
            limit,
        } => refusal(resource, *requested, *limit),
        SimulationError::ResourceFailure(
            rspice_simulation::error::ResourceFailure::DeviceLimit {
                instance,
                resource,
                requested,
                limit,
            },
        ) => {
            refusal(resource, *requested, *limit).map(|said| format!("Device '{instance}': {said}"))
        }
        _ => None,
    }
    .unwrap_or_else(|| error.to_string())
}

fn resource_name(name: &str) -> Option<&'static str> {
    resource_kind(name).map(rspice_core::ResourceKind::as_str)
}

/// The resource the engine names `name`.
fn resource_kind(name: &str) -> Option<rspice_core::ResourceKind> {
    use rspice_core::ResourceKind::*;
    [
        NetlistBytes,
        NetlistLines,
        ExpandedSourceBytes,
        DependencySourceBytes,
        ExternalDataBytes,
        ExternalDataValues,
        SharedCacheBytes,
        IncludeDepth,
        HierarchyDepth,
        FlattenedElements,
        CircuitNodes,
        MatrixUnknowns,
        ResultValues,
        TransportHistoryBytes,
        TransportHistoryRecords,
        MixedIntervalEvents,
        ParallelWorkers,
        BatchRuns,
    ]
    .into_iter()
    .find(|kind| kind.as_str() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_resource_failure_keeps_budget_and_instance_context() {
        let error = execution(
            SimulationError::ResourceFailure(
                rspice_simulation::error::ResourceFailure::DeviceLimit {
                    instance: "XAMP:delay".into(),
                    resource: "transport_history_bytes".into(),
                    requested: 5000,
                    limit: 4000,
                },
            ),
            "waveform",
            None,
        );
        assert_eq!(error.exit_code() as u8, 75);
        let details = error.details();
        assert_eq!(details.resource, Some("transport_history_bytes"));
        assert_eq!(details.instance_name.as_deref(), Some("XAMP:delay"));
        assert_eq!(details.analysis_id.as_deref(), Some("waveform"));
        assert_eq!((details.requested, details.limit), (Some(5000), Some(4000)));
        assert!(
            error.to_string().starts_with(
                "Device 'XAMP:delay': The delay lines' histories need 4.9 KB; they can take at \
                 most 3.9 KB. (transport_history_bytes limit exceeded"
            ),
            "{error}"
        );
    }

    /// A run past a limit says it as the desktop app does, then in the
    /// engine's words.
    #[test]
    fn a_resource_limit_reads_as_the_app_says_it() {
        let error = execution(
            SimulationError::ResourceLimit {
                resource: "flattened_elements".into(),
                requested: 250_001,
                limit: 250_000,
            },
            "op",
            None,
        );
        assert_eq!(
            error.to_string(),
            "This circuit has more than 250,000 devices with its subcircuits expanded; a circuit \
             can have at most 250,000. (flattened_elements limit exceeded: requested 250001, \
             limit 250000)"
        );
    }
}
