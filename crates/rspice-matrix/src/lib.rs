//! Sparse matrix storage and numeric factorization shared by the simulator and
//! precompiled generated-device crates.
//!
//! Keeping these concrete types below `rspice-core` would force generated
//! models to be source modules of that crate. This dependency-neutral package
//! is the artifact boundary that lets model shards compile once and be reused
//! by desktop, browser, mobile, and command-line products.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod klu;
mod sparse;

pub use klu::{KluDiagnostics, KluSolver};
pub use sparse::{
    ComplexMatrix, CscIndex, CscPatternToken, SparseLuSolver, StaticMatrix, TripletMatrix,
    solve_sparse,
};

/// Numeric scalar used by RSpice's real-valued circuit matrices.
pub type Value = f64;

/// Preferred real-valued numeric backend.
///
/// Circuit LU uses RSpice's values-only refactorization kernel. `Auto` also
/// routes measured high-fill patterns to equilibrated faer LU, and every
/// Circuit LU solve fails safely to faer if it cannot satisfy the shared
/// backward-error acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealSolverBackend {
    /// Select Circuit LU for circuit-like fill and automatically retain faer
    /// for patterns whose measured factor fill makes a supernodal solver the
    /// better choice.
    Auto,
    /// RSpice's circuit-specialized, allocation-free numeric refactorization
    /// and triangular-solve kernel.
    Klu,
    /// Faer's general sparse LU with row and column equilibration.
    Faer,
}

/// Numeric-factorization lifecycle for RSpice's circuit LU backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericFactorizationPolicy {
    /// Reuse the established pivot sequence while the changing numeric values
    /// remain safe for values-only refactorization.
    ReusePivotSequence,
    /// Select a fresh pivot sequence for every ordinary numeric solve request.
    /// Callers solving another right-hand side against the same factors can use
    /// an explicit [`FactorizationRequest::ReuseExisting`] solve.
    FreshPivotSelection,
}

/// Arithmetic used for a circuit-LU division site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivisionPolicy {
    /// Multiply by a cached reciprocal on the ordinary finite path.
    ReciprocalMultiplication,
    /// Perform the hardware division directly.
    DirectDivision,
}

/// Matrix orientation presented to the circuit-LU numeric factorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitLuOrientation {
    /// Factor the matrix's native compressed-column representation and use the
    /// matching normal/transpose triangular solve.
    Native,
    /// Match Trilinos Amesos_Klu's serial Epetra adapter: factor the row-CRS
    /// arrays as CSC (therefore factor `A^T`) and invert the KLU solve choice so
    /// the public normal and transpose systems remain `A*x=b` and `A^T*x=b`.
    AmesosRowCrs,
}

/// Row scaling applied before circuit-LU factorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitLuRowScaling {
    /// Retain RSpice's protective scaling for values near the floating-point
    /// range limits.
    AdaptiveExtremeRows,
    /// Do not scale matrix rows, matching Amesos `ScaleMethod=0`.
    Disabled,
}

/// Error-handling behavior around the circuit-LU backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitLuRobustness {
    /// Retry dangerous growth, refine inaccurate solutions, and fall back to
    /// the supernodal backend when circuit LU cannot certify a result.
    Enhanced,
    /// Return the circuit-LU result or error directly, without RSpice-specific
    /// growth retries, iterative refinement, or backend fallback.
    BackendFaithful,
}

/// Per-solve numeric-factorization lifecycle request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorizationRequest {
    /// Follow the matrix's [`NumericFactorizationPolicy`]. Fresh-pivot matrices
    /// factor on every call; reuse-pivot matrices reuse factors for
    /// bit-identical values and refactor changed values.
    Automatic,
    /// Run numeric factorization for this solve even when values are unchanged.
    Factorize,
    /// Reuse the existing numeric factors without consulting current values.
    /// This mirrors Xyce's `reuse_factors=true` contract; callers must ensure
    /// the matrix represented by the factors is still the intended matrix.
    ReuseExisting,
}

/// Per-matrix solver policy.
///
/// Commercial embedding code should pass this explicitly through
/// [`StaticMatrix::from_triplets_with_options`]. [`Default`] is deterministic
/// and independent of process-global environment state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverOptions {
    /// Backend used for real-valued sparse systems.
    pub real_backend: RealSolverBackend,
    /// Relative threshold-pivoting tolerance in `(0, 1]`. A structurally
    /// preferred diagonal is retained when it is at least this fraction of
    /// the largest eligible entry in its factor column.
    pub pivot_tolerance: Value,
    /// Absolute minimum accepted pivot magnitude in original matrix units.
    /// Zero disables the absolute threshold.
    pub absolute_pivot_tolerance: Value,
    /// Whether changing matrix values reuse the prior pivot sequence or run a
    /// fresh numeric pivot search.
    pub numeric_factorization: NumericFactorizationPolicy,
    /// Arithmetic used to form lower-triangular factors from pivot columns.
    pub factorization_division: DivisionPolicy,
    /// Arithmetic used for circuit-LU diagonal triangular solves.
    pub diagonal_solve: DivisionPolicy,
    /// Orientation presented to circuit LU and corresponding solve dispatch.
    pub circuit_lu_orientation: CircuitLuOrientation,
    /// Circuit-LU row-scaling policy.
    pub circuit_lu_row_scaling: CircuitLuRowScaling,
    /// Circuit-LU retry, refinement, and fallback policy.
    pub circuit_lu_robustness: CircuitLuRobustness,
}

impl SolverOptions {
    /// Compatibility policy used by RSpice's existing application layer.
    /// `RSPICE_SOLVER=faer` selects faer, `klu` forces Circuit LU preference,
    /// and every other value uses automatic measured routing.
    pub fn from_env() -> Self {
        let real_backend = match std::env::var("RSPICE_SOLVER") {
            Ok(value) if value.eq_ignore_ascii_case("faer") => RealSolverBackend::Faer,
            Ok(value) if value.eq_ignore_ascii_case("klu") => RealSolverBackend::Klu,
            _ => RealSolverBackend::Auto,
        };
        let pivot_tolerance = std::env::var("RSPICE_PIVREL")
            .ok()
            .and_then(|value| value.parse::<Value>().ok())
            .filter(|value| value.is_finite() && *value > 0.0 && *value <= 1.0)
            .unwrap_or(1.0e-3);
        let absolute_pivot_tolerance = std::env::var("RSPICE_PIVTOL")
            .ok()
            .and_then(|value| value.parse::<Value>().ok())
            .filter(|value| value.is_finite() && *value >= 0.0)
            .unwrap_or(0.0);
        Self {
            real_backend,
            pivot_tolerance,
            absolute_pivot_tolerance,
            numeric_factorization: NumericFactorizationPolicy::ReusePivotSequence,
            factorization_division: DivisionPolicy::ReciprocalMultiplication,
            diagonal_solve: DivisionPolicy::ReciprocalMultiplication,
            circuit_lu_orientation: CircuitLuOrientation::Native,
            circuit_lu_row_scaling: CircuitLuRowScaling::AdaptiveExtremeRows,
            circuit_lu_robustness: CircuitLuRobustness::Enhanced,
        }
    }
}

impl Default for SolverOptions {
    fn default() -> Self {
        Self {
            real_backend: RealSolverBackend::Auto,
            pivot_tolerance: 1.0e-3,
            absolute_pivot_tolerance: 0.0,
            numeric_factorization: NumericFactorizationPolicy::ReusePivotSequence,
            factorization_division: DivisionPolicy::ReciprocalMultiplication,
            diagonal_solve: DivisionPolicy::ReciprocalMultiplication,
            circuit_lu_orientation: CircuitLuOrientation::Native,
            circuit_lu_row_scaling: CircuitLuRowScaling::AdaptiveExtremeRows,
            circuit_lu_robustness: CircuitLuRobustness::Enhanced,
        }
    }
}

/// Matrix construction and numeric-factorization failures.
#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    /// A solver workspace could not be allocated without aborting the process.
    #[error("Insufficient memory for sparse solver workspace")]
    OutOfMemory,

    /// The matrix is structurally or numerically singular.
    #[error("Matrix is singular or near-singular")]
    SingularMatrix,

    /// An iterative algorithm exhausted the supplied iteration budget.
    #[error("Failed to converge after {0} iterations")]
    ConvergenceFailed(usize),

    /// An input or intermediate numeric value was not finite.
    #[error("Numerical overflow detected")]
    Overflow,

    /// Matrix structure, dimensions, or a stamping operation was invalid.
    #[error("Invalid circuit configuration: {0}")]
    InvalidCircuit(String),

    /// A values-only KLU refactor needs fresh pivot selection.
    #[error("Stored pivot sequence is numerically inadequate for the new values")]
    PivotGrowth,

    /// A finite solution failed the componentwise backward-error criterion.
    #[error("Sparse solve failed its backward-error check ({0:.3e})")]
    InaccurateSolution(Value),
}

#[doc(hidden)]
pub fn klu_backend_enabled() -> bool {
    sparse::klu_backend_enabled()
}
