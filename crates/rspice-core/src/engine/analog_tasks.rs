//! Host delivery and model completion at public analysis boundaries.

use super::{Engine, SimulationError};
use crate::abort_signal::ModelRunSignal;
use crate::{AbortSignal, CircuitData, ModelFinish, ModelFinishPoint, SimulationOutcome};
use rspice_veriloga_runtime::{AnalogTaskArgument, AnalogTaskEvent, AnalogTaskKind};

fn finish_level(event: &AnalogTaskEvent<'_>) -> Result<u8, &'static str> {
    let [AnalogTaskArgument::Integer(level)] = event.call.arguments.as_ref() else {
        return Err("analog finish call has an invalid argument snapshot");
    };
    if event.call.kind != AnalogTaskKind::Finish || !(0..=2).contains(level) {
        return Err("analog task has no implemented host control handler");
    }
    Ok(*level as u8)
}

impl Engine {
    /// Run one analysis with an explicit normal-completion outcome. A model
    /// may finish during initialization, before any numerical result exists.
    /// The callback must pass the supplied signal to its analysis entry point.
    /// Reusing this Engine or its caller's abort flag does not reuse model control.
    pub fn run_with_outcome<T>(
        &self,
        abort: &dyn AbortSignal,
        run: impl FnOnce(&Self, &dyn AbortSignal) -> Result<T, SimulationError>,
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

    pub(super) fn ensure_model_run_active(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
        if let Some(finish) = abort.model_control().and_then(|control| control.finish()) {
            return Err(SimulationError::ModelFinished(Box::new(finish)));
        }
        Ok(())
    }

    pub(super) fn deliver_accepted_analog_tasks(
        circuit: &mut CircuitData,
        abort: &dyn AbortSignal,
        point: ModelFinishPoint,
    ) -> Result<bool, SimulationError> {
        let control = abort.model_control().ok_or_else(|| {
            SimulationError::Circuit("analog task delivery has no simulation run scope".into())
        })?;
        let mut invalid = None;
        circuit
            .visit_accepted_analog_tasks(&mut |event| {
                let level = match finish_level(&event) {
                    Ok(level) => level,
                    Err(error) => {
                        invalid = Some(error);
                        return;
                    }
                };
                if !control.is_finished() {
                    control.request_finish(ModelFinish {
                        instance: event.instance.into(),
                        model: event.model.into(),
                        site: event.call.site,
                        point,
                        diagnostic_level: level,
                    });
                }
            })
            .map_err(SimulationError::Circuit)?;
        if let Some(message) = invalid {
            return Err(SimulationError::Circuit(message.into()));
        }
        Ok(control.is_finished())
    }

    pub(super) fn candidate_equilibrium_finish(
        circuit: &CircuitData,
        point: ModelFinishPoint,
    ) -> Result<Option<ModelFinish>, SimulationError> {
        circuit
            .first_equilibrium_candidate_analog_task(AnalogTaskKind::Finish)
            .map_err(SimulationError::Circuit)?
            .map(|event| {
                let diagnostic_level =
                    finish_level(&event).map_err(|error| SimulationError::Circuit(error.into()))?;
                Ok(ModelFinish {
                    instance: event.instance.into(),
                    model: event.model.into(),
                    site: event.call.site,
                    point,
                    diagnostic_level,
                })
            })
            .transpose()
    }

    pub(super) fn deliver_initial_analog_tasks(
        circuit: &mut CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        Self::deliver_accepted_analog_tasks(circuit, abort, ModelFinishPoint::Initialization)?;
        Self::ensure_model_run_active(abort)
    }
}
