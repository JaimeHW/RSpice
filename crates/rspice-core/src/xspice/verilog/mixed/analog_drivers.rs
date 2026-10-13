//! Analog-owned integer output ports.
//!
//! `output integer code;` written by the analog body and by no discrete
//! process is the port's one driver: Verilog-AMS LRM 2.4 section 7.3 lets
//! only a variable's own domain write it. The discrete net carries the value
//! the analog half assigned, so the variable's value at the trial candidate is
//! published into the port's discrete signal in the coupled trial that
//! produced it — beside that trial's A/D bank and analog event occurrences,
//! through the same analog causal lane, dated at the trial's timepoint on its
//! nearest tick and never before a tick the trial already published at (the
//! rule section 7.3.6.1 gives an analog event). A rejected trial takes the
//! publication back with the rest of its discrete state.
//!
//! Only a change is published: a settle pass that republished an unchanged
//! value would never report the boundary quiet.

use super::*;

/// One analog `integer` variable driving one discrete signal.
#[derive(Clone)]
pub(super) struct AnalogDriver {
    /// The analog variable, by the name the compiled model reads it by.
    variable: String,
    /// The local digital signal it drives: the port.
    signal: DigitalSignalId,
}

/// Resolve the artifact's analog drivers against its digital plan.
pub(super) fn wire(
    canonical_ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<Vec<AnalogDriver>, MixedSignalError> {
    canonical_ir
        .hir
        .analog_drivers
        .iter()
        .map(|driver| {
            let variable = canonical_ir
                .hir
                .variables
                .get(usize::from(driver.variable))
                .ok_or_else(|| MixedSignalError::Compile {
                    detail: format!("analog driver of `{}` has no variable", driver.signal),
                })?;
            let signal = canonical_ir
                .digital
                .signals
                .iter()
                .find(|signal| signal.name == driver.signal)
                .ok_or_else(|| MixedSignalError::Compile {
                    detail: format!("analog driver of `{}` has no signal", driver.signal),
                })?;
            Ok(AnalogDriver {
                variable: variable.name.to_string(),
                signal: signal.id,
            })
        })
        .collect()
}

impl MixedSignalHost {
    /// Every analog-owned port whose candidate value differs from what its
    /// discrete signal holds, as (local signal, 32-bit pattern).
    ///
    /// Evaluates the analog half at the candidate first, through the same
    /// preparation the matrix assembly uses, so the value is the one this
    /// candidate's equations computed.
    pub(super) fn analog_driver_targets(
        &mut self,
        solution: &[f64],
    ) -> Result<Vec<(DigitalSignalId, u32)>, MixedSignalError> {
        if self.analog_drivers.is_empty() {
            return Ok(Vec::new());
        }
        self.sample_discrete_inputs()?;
        let classify = if self.trial.as_ref().is_some_and(|trial| trial.probe) {
            analog_trial_error
        } else {
            analog_accepted_error
        };
        self.prepared_analog.prepare(
            self.analog.make_mut(),
            &self.discrete_inputs,
            solution,
            classify,
        )?;
        let mut targets = Vec::new();
        for driver in &self.analog_drivers {
            let value = self
                .analog
                .variable(&driver.variable)
                .filter(|value| value.is_finite())
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!(
                        "analog-owned port `{}` has no finite integer value",
                        driver.variable
                    ),
                })?;
            // The analog half carries an `integer` in an `f64`; the discrete
            // net holds its 32-bit two's-complement pattern.
            let bits = value.round() as i64 as u32;
            if self.state.digital.read(driver.signal)
                != Some(&FourStateValue::from_u64(32, u64::from(bits)))
            {
                targets.push((driver.signal, bits));
            }
        }
        Ok(targets)
    }
}
