//! Portable worker failure report. The application maps execution errors at the boundary.

use rspice_results::convergence_attribution::ConvergenceAttribution;
use rspice_results::validation::ResultSchemaMismatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerSimulationError {
    ParseError(String),
    BehavioralReference {
        owner_name: String,
        canonical_owner_name: String,
        dependency_name: String,
        canonical_dependency_name: String,
        reason: String,
    },
    CircuitError(String),
    /// A Verilog-A or mixed elaboration refusal, still typed on the far side
    /// of the browser worker boundary: a schematic on this side has the same
    /// right to mark the instance the engine named.
    Elaboration {
        instance: Option<String>,
        module: Option<String>,
        kind: String,
        location: Option<String>,
        message: String,
    },
    SolverError(String),
    RequestedSignalUnavailable {
        signal: String,
        analysis: String,
        coordinate: Option<String>,
    },
    ResultSchemaMismatch(Box<ResultSchemaMismatch>),
    ConvergenceFailed {
        iterations: usize,
        message: String,
    },
    /// A failure the engine attributed to named design objects. The objects
    /// cross the worker boundary with the error because a browser run's
    /// schematic is on this side of it and has the same right to mark them.
    Attributed {
        message: String,
        attribution: ConvergenceAttribution,
    },
    Aborted,
    AlreadyRunning,
    ThreadPanic,
    InvalidConfig(String),
    UnsupportedOutcome(String),
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}
