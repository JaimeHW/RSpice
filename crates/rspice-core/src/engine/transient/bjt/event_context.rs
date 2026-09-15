//! Read-only event-side selection shared by trial and acceptance consumers.

use super::*;

/// An incoming equation trial reads arriving delayed events on their left.
/// An outgoing trial may carry separately solved input limits for selected
/// devices. A `Some` slot declares a jump or continuous corner; an absent slot
/// does not create an event merely because another circuit device had one.
#[derive(Clone, Copy, Default)]
pub(in crate::engine) struct BjtPhaseContext<'a> {
    pub incoming_arrival: bool,
    pub input_left_limits: Option<&'a [Option<Value>]>,
}

impl<'a> BjtPhaseContext<'a> {
    pub(in crate::engine::transient) fn bind<'h>(
        self,
        history: &'h BjtTransientHistory,
    ) -> Result<BjtPhaseHistoryView<'h, 'a>, SimulationError> {
        if history.weil_phase.len() != history.phase.len()
            || history
                .phase
                .iter()
                .zip(&history.weil_phase)
                .any(|(delay, weil)| delay.is_some() && weil.is_some())
            || history.phase_outgoing_slopes.len() != history.phase.len()
            || history
                .phase_outgoing_slopes
                .iter()
                .zip(&history.phase)
                .any(|(slope, phase)| {
                    slope.is_some_and(|slope| !slope.is_finite() || phase.is_none())
                })
        {
            return Err(SimulationError::Circuit(
                "GP outgoing slopes do not match accepted phase history".into(),
            ));
        }
        if self.incoming_arrival && self.input_left_limits.is_some() {
            return Err(SimulationError::Circuit(
                "incoming GP phase context cannot hold outgoing input limits".into(),
            ));
        }
        if let Some(limits) = self.input_left_limits {
            if limits.len() != history.phase.len() {
                return Err(SimulationError::Circuit(
                    "GP phase input limits do not match the BJT population".into(),
                ));
            }
            for (index, (&left, phase)) in limits.iter().zip(&history.phase).enumerate() {
                if let Some(left) = left {
                    if !left.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "BJT index {index} has a non-finite incoming GP current"
                        )));
                    }
                    if phase.is_none() {
                        return Err(SimulationError::Circuit(format!(
                            "BJT index {index} has an incoming GP current without phase history"
                        )));
                    }
                }
            }
        }
        Ok(BjtPhaseHistoryView {
            history,
            context: self,
        })
    }
}

/// A complete event context checked once before the device-family walk.
pub(in crate::engine::transient) struct BjtPhaseHistoryView<'h, 'a> {
    history: &'h BjtTransientHistory,
    context: BjtPhaseContext<'a>,
}

impl<'h> BjtPhaseHistoryView<'h, '_> {
    pub(in crate::engine::transient) fn trial(
        &self,
        index: usize,
        time: Value,
    ) -> Option<BjtPhaseTrial<'h>> {
        self.history.phase_trial(index, time).map(|mut trial| {
            trial.incoming_arrival = self.context.incoming_arrival;
            trial.left_limit = self
                .context
                .input_left_limits
                .and_then(|limits| limits[index]);
            trial
        })
    }
}
