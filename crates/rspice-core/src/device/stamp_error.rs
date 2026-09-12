//! How a failed device stamp is classified for the Newton loops.
//!
//! A device evaluation can fail for two reasons that call for opposite
//! answers, and the solver has to be able to tell them apart *before* it
//! decides what to do.
//!
//! `ln(V(p,n)+0.1)` is defined at every accepted point of a well-posed deck
//! and undefined at an overshooting Newton iterate. Spectre and ngspice reject
//! such an iterate — cut the timestep, step the sources — and end the run only
//! when every retry is still non-finite. That is
//! [`StampError::NonFiniteTrial`]: a property of the *point* the solver handed
//! the device.
//!
//! A model whose compiled layout the runtime refuses, an index outside its
//! declared bounds, a missing branch: those fail identically at every point.
//! Retrying one spends the entire convergence ladder — in transient, every dt
//! cut; in DC, source stepping, pseudo-transient and gmin — to report the
//! message it already had at the first iterate. That is
//! [`StampError::Structural`], and it ends the run where it happens.
//!
//! The classification used to travel inside the message string, because the
//! stamping surface was `Result<(), String>` from the device adapter all the
//! way up to `SimulationError::Circuit`. It is a type now, so every route that
//! stamps a device — runtime Verilog-A, the generated/builtin adapter, a mixed
//! Verilog-AMS host, behavioral sources, the Xyce memristors — answers the
//! same question in the same vocabulary, and the loops match a variant instead
//! of searching a sentence for a phrase.

use std::fmt;

/// A device evaluation that left its own numeric domain at the trial point the
/// solver handed it.
///
/// The iterate itself is finite: it is the module's own arithmetic that left
/// its domain. `detail` is the whole rendered diagnostic — the instance, the
/// operator and the terminal voltages — because a rejecting loop keeps it and
/// hands it back when its retry budget is exhausted, and by then nothing else
/// remembers why the step failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonFiniteTrialError {
    /// Instance whose evaluation was not finite.
    pub instance: String,
    /// Complete diagnostic, ready to be reported verbatim.
    pub detail: String,
}

impl fmt::Display for NonFiniteTrialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for NonFiniteTrialError {}

/// Why one device could not be stamped into the Newton system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StampError {
    /// The iterate is rejectable: cut dt or step the sources and hand the
    /// device another point.
    NonFiniteTrial(Box<NonFiniteTrialError>),
    /// Every other point fails the same way, so this ends the run here.
    Structural(String),
}

impl StampError {
    /// Classify a non-finite evaluation at a trial point.
    pub(crate) fn nonfinite_trial(instance: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::NonFiniteTrial(Box::new(NonFiniteTrialError {
            instance: instance.into(),
            detail: detail.into(),
        }))
    }
}

impl fmt::Display for StampError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteTrial(error) => error.fmt(formatter),
            Self::Structural(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for StampError {}

/// Every stamping surface that still reports with a string reports something
/// structural: a non-finite trial is constructed through
/// [`StampError::nonfinite_trial`], which names its instance.
impl From<String> for StampError {
    fn from(message: String) -> Self {
        Self::Structural(message)
    }
}

#[cfg(test)]
mod tests {
    use super::StampError;

    #[test]
    fn a_string_stamping_failure_is_structural_not_a_rejectable_trial() {
        let error = StampError::from("no such branch".to_string());
        assert!(matches!(error, StampError::Structural(_)));
        assert_eq!(error.to_string(), "no such branch");
        assert!(
            crate::SimulationError::from(error)
                .nonfinite_trial_detail()
                .is_none()
        );
    }

    #[test]
    fn a_nonfinite_trial_carries_its_instance_and_its_whole_diagnostic() {
        let error = StampError::nonfinite_trial("x1", "Verilog-A device 'x1' ... [p=-1.0]");
        let StampError::NonFiniteTrial(trial) = &error else {
            panic!("a non-finite trial must keep its variant: {error}");
        };
        assert_eq!(trial.instance, "x1");
        assert_eq!(
            crate::SimulationError::from(error).nonfinite_trial_detail(),
            Some("Verilog-A device 'x1' ... [p=-1.0]")
        );
    }
}
