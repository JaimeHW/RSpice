//! Independent phase coordinates for quasi-periodic spectral analyses.
//!
//! A coefficient retains its integer tone tuple. No approximate common period
//! is introduced: differentiation uses the signed physical frequency k·f,
//! while nonlinear evaluation uses a Cartesian grid of independent phases.
//! Constructing a grid alone does not constitute a QPSS solution; circuit
//! equations and convergence evidence belong to the consuming solver.

mod grid;
pub(crate) mod small_signal;
pub(crate) mod solve;
mod transform;

pub use grid::{QuasiPeriodicGrid, QuasiPeriodicGridConfig, QuasiPeriodicSampling};
pub use small_signal::{
    QuasiPeriodicAcConfig, QuasiPeriodicAcSolution, QuasiPeriodicAdjointSolution,
};
pub use solve::{
    QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod, QuasiPeriodicSolution,
    QuasiPeriodicSolveConfig,
};
pub use transform::QuasiPeriodicTransform;

use crate::abort_signal::AbortSignal;
use crate::{Complex64, ResourceLimitError};

#[derive(Debug, thiserror::Error)]
pub enum QuasiPeriodicError {
    #[error("Quasi-periodic analysis aborted")]
    Aborted,
    #[error("Invalid quasi-periodic configuration: {0}")]
    InvalidConfig(String),
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
    #[error("Quasi-periodic numerical failure: {0}")]
    Numerical(String),
    #[error("Invalid quasi-periodic circuit: {0}")]
    InvalidCircuit(String),
    #[error("Quasi-periodic linear solve failed: {0}")]
    LinearSolve(#[from] crate::solver::SolverError),
    #[error(
        "Quasi-periodic Newton solve failed after {iterations} updates (normalized residual {merit:e})"
    )]
    ConvergenceFailed { iterations: usize, merit: f64 },
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), QuasiPeriodicError> {
    if abort.is_aborted() {
        Err(QuasiPeriodicError::Aborted)
    } else {
        Ok(())
    }
}

fn finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

fn zero_buffer(size: usize) -> Result<Vec<Complex64>, QuasiPeriodicError> {
    let mut result = Vec::new();
    result.try_reserve_exact(size).map_err(|error| {
        QuasiPeriodicError::Numerical(format!("Fourier buffer allocation failed: {error}"))
    })?;
    result.resize(size, Complex64::ZERO);
    Ok(result)
}

#[cfg(test)]
mod tests;
