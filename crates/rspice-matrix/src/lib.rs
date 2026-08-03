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

pub use klu::KluSolver;
pub use sparse::{
    ComplexMatrix, CscIndex, SparseLuSolver, StaticMatrix, TripletMatrix, solve_sparse,
};

/// Numeric scalar used by RSpice's real-valued circuit matrices.
pub type Value = f64;

/// Preferred real-valued numeric backend.
///
/// `Klu` uses RSpice's values-only refactorization kernel and automatically
/// falls back to equilibrated faer LU when the KLU result cannot satisfy the
/// shared backward-error acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealSolverBackend {
    /// RSpice's circuit-specialized, allocation-free numeric refactorization
    /// and triangular-solve kernel.
    Klu,
    /// Faer's general sparse LU with row and column equilibration.
    Faer,
}

/// Per-matrix solver policy.
///
/// Commercial embedding code should pass this explicitly through
/// [`StaticMatrix::from_triplets_with_options`]. [`Default`] is deterministic
/// and independent of process-global environment state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverOptions {
    /// Backend used for real-valued sparse systems.
    pub real_backend: RealSolverBackend,
}

impl SolverOptions {
    /// Compatibility policy used by RSpice's existing application layer.
    /// `RSPICE_SOLVER=faer` selects faer; every other value selects KLU.
    pub fn from_env() -> Self {
        let real_backend = if std::env::var("RSPICE_SOLVER")
            .is_ok_and(|value| value.eq_ignore_ascii_case("faer"))
        {
            RealSolverBackend::Faer
        } else {
            RealSolverBackend::Klu
        };
        Self { real_backend }
    }
}

impl Default for SolverOptions {
    fn default() -> Self {
        Self {
            real_backend: RealSolverBackend::Klu,
        }
    }
}

/// Matrix construction and numeric-factorization failures.
#[derive(Debug, thiserror::Error)]
pub enum SolverError {
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
