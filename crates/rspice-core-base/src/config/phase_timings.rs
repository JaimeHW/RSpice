//! Wall-clock phase timings a frontend asks the engine to collect.
//!
//! Off unless [`super::SimulationConfig::phase_timings`] holds a collector.
//! With none, the engine reads no clock for these figures; with one, every
//! engine built from that configuration (and every engine it derives for a
//! netlist) records into the same collector, which the frontend drains.
//! They are diagnostics: nothing here feeds back into a solve.

use std::sync::Mutex;
use std::time::Duration;

/// The collector an engine records its phase timings into.
#[derive(Debug, Default)]
pub struct PhaseTimings {
    record: Mutex<PhaseTimingRecord>,
}

impl PhaseTimings {
    /// Add to the record. A poisoned lock drops the figures rather than
    /// failing the run: timings must never be why a simulation stops.
    pub fn record(&self, update: impl FnOnce(&mut PhaseTimingRecord)) {
        if let Ok(mut record) = self.record.lock() {
            update(&mut record);
        }
    }

    /// Everything recorded since the last take, leaving the record empty.
    pub fn take(&self) -> PhaseTimingRecord {
        self.record
            .lock()
            .map(|mut record| std::mem::take(&mut *record))
            .unwrap_or_default()
    }
}

/// Summed phase times and counts. Every field accumulates over each
/// elaboration, operating point and transient run recorded into it.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PhaseTimingRecord {
    /// Hierarchy flattening inside circuit elaboration.
    pub flatten: Duration,
    /// Whole circuit elaborations, their flattening included.
    pub elaboration: Duration,
    /// Circuits elaborated.
    pub circuit_builds: u64,
    /// DC operating-point solves, including a transient's initial one.
    pub operating_point: Duration,
    /// Operating points solved.
    pub operating_points: u64,
    /// Newton Jacobian assemblies those operating points took.
    pub operating_point_newton_iterations: u64,
    /// Transient runs recorded below.
    pub transient_runs: u64,
    /// Transient stepping loop, initial operating point excluded.
    pub transient_wall: Duration,
    /// Device model evaluation inside the Newton assemblies.
    pub transient_device_evaluation: Duration,
    /// The rest of the Newton assemblies: clearing, stamping, companions.
    pub transient_matrix_assembly: Duration,
    /// The Newton linear solve phase as a whole.
    pub transient_linear_solve: Duration,
    /// Sparse LU numeric factorization and refactorization.
    pub transient_factor: Duration,
    /// Forward and back substitution of the first solve.
    pub transient_solve: Duration,
    /// Backward-error certificate, refinement solves and retries.
    pub transient_certificate: Duration,
    /// Device update at each Newton candidate and the convergence tests.
    pub transient_convergence_check: Duration,
    /// True-residual restamp that proves a converged candidate.
    pub transient_residual_proof: Duration,
    /// Local truncation error estimate.
    pub transient_truncation: Duration,
    /// Recording accepted samples into the result.
    pub transient_output: Duration,
    /// Accepted time steps.
    pub transient_accepted_steps: u64,
    /// Step attempts that were not accepted.
    pub transient_rejected_steps: u64,
    /// Newton Jacobian assemblies in the stepping loop.
    pub transient_newton_iterations: u64,
    /// Full sparse LU factorizations (pivot search) in the stepping loop.
    pub transient_factorizations: u64,
    /// Values-only refactorizations reusing the pivot order.
    pub transient_refactorizations: u64,
}
