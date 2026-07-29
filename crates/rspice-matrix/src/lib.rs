//! Sparse matrix storage and numeric factorization shared by the simulator and
//! precompiled generated-device crates.
//!
//! Keeping these concrete types below `rspice-core` would force generated
//! models to be source modules of that crate. This dependency-neutral package
//! is the artifact boundary that lets model shards compile once and be reused
//! by desktop, browser, mobile, and command-line products.

mod klu;
mod sparse;

pub use klu::KluSolver;
pub use sparse::{
    ComplexMatrix, CscIndex, SparseLuSolver, StaticMatrix, TripletMatrix, solve_sparse,
};

/// Numeric scalar used by RSpice's real-valued circuit matrices.
pub type Value = f64;

/// Matrix construction and numeric-factorization failures.
#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    #[error("Matrix is singular or near-singular")]
    SingularMatrix,

    #[error("Failed to converge after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Numerical overflow detected")]
    Overflow,

    #[error("Invalid circuit configuration: {0}")]
    InvalidCircuit(String),

    #[error("Stored pivot sequence is numerically inadequate for the new values")]
    PivotGrowth,

    #[error("Sparse solve failed its backward-error check ({0:.3e})")]
    InaccurateSolution(Value),
}

#[doc(hidden)]
pub fn klu_backend_enabled() -> bool {
    sparse::klu_backend_enabled()
}
