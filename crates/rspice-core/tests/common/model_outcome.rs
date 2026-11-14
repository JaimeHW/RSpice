//! One analysis run with an explicit normal-completion outcome.
//!
//! A Verilog-A model may `$finish` during initialization, before any numerical
//! result exists. Tests that pin that behaviour run the analysis under a fresh
//! model-run scope and read the finish back from it, the way the simulation
//! crate's runners do.

use rspice_core::abort_signal::{AbortSignal, ModelRunSignal};
use rspice_core::{Engine, SimulationError, SimulationOutcome};

pub trait RunWithOutcome {
    /// Run one analysis under its own model-run scope. The callback must pass
    /// the supplied signal to its analysis entry point.
    fn run_with_outcome<T>(
        &self,
        abort: &dyn AbortSignal,
        run: impl FnOnce(&Engine, &dyn AbortSignal) -> Result<T, SimulationError>,
    ) -> Result<SimulationOutcome<T>, SimulationError>;
}

impl RunWithOutcome for Engine {
    fn run_with_outcome<T>(
        &self,
        abort: &dyn AbortSignal,
        run: impl FnOnce(&Engine, &dyn AbortSignal) -> Result<T, SimulationError>,
    ) -> Result<SimulationOutcome<T>, SimulationError> {
        let signal = ModelRunSignal::new(abort);
        match run(self, &signal) {
            Ok(result) => match signal.model_control().and_then(|control| control.finish()) {
                Some(finish) => Ok(SimulationOutcome::Finished {
                    result: Some(result),
                    finish,
                }),
                None => Ok(SimulationOutcome::Completed(result)),
            },
            Err(SimulationError::ModelFinished(finish)) => Ok(SimulationOutcome::Finished {
                result: None,
                finish: *finish,
            }),
            Err(error) => Err(error),
        }
    }
}
