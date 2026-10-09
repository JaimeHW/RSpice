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
        E::AlreadyRunning | E::ThreadPanic => {
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
        _ => {}
    }
    CliError::Reported {
        message: error.to_string(),
        category,
        details: Box::new(details),
    }
}

fn resource_name(name: &str) -> Option<&'static str> {
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
        AnalysisPoints,
        ResultValues,
        TransportHistoryBytes,
        TransportHistoryRecords,
        MixedIntervalEvents,
        ParallelWorkers,
        BatchRuns,
    ]
    .into_iter()
    .map(|kind| kind.as_str())
    .find(|known| *known == name)
}
