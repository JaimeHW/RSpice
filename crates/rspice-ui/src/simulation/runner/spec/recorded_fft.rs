//! Publishing the spectrum a bound transient already computed.
//!
//! This is the whole of a recorded FFT's execution: take the trajectory
//! artifact the transient produced, select the spectrum whose request is this
//! analysis's own card, and hand it back. No deck is parsed, no solve is
//! started, and the netlist never reaches this module — which is what makes
//! "one solve" checkable rather than asserted.

use rspice_core::abort_signal::AbortSignal;

use crate::simulation::config::FftRequest;
use crate::simulation::execution::ResolvedExecutionDependencies;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SimulationError;

pub(super) fn run(
    request: &FftRequest,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let trajectory = dependencies.transient_trajectory().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "recorded FFT dependency artifact is unavailable: {error}"
        ))
    })?;
    let key = request
        .engine_key()
        .map_err(SimulationError::InvalidConfig)?;
    let spectrum = trajectory.spectrum(&key).ok_or_else(|| {
        SimulationError::InvalidConfig(format!(
            "the bound transient produced no spectrum for {key}"
        ))
    })?;
    Ok(SimulationResult::Fft {
        spectrum: std::sync::Arc::clone(spectrum),
        convergence: trajectory.convergence().cloned(),
    })
}
