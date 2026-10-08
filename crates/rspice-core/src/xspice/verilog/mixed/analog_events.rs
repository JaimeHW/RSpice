//! Publish retained analog occurrences through the existing analog causal lane.
use super::*;

pub(super) fn event_bank(
    targets: &[(DigitalSignalId, u32)],
    read: impl Fn(DigitalSignalId) -> Option<u64>,
) -> Result<Vec<(DigitalSignalId, FourStateValue)>, MixedSignalError> {
    let mut drives = Vec::new();
    for &(signal, target) in targets {
        let held = read(signal)
            .filter(|value| *value <= i32::MAX as u64)
            .ok_or_else(|| MixedSignalError::InvalidBridge {
                detail: "analog event counter lost its initialized signal".into(),
            })? as u32;
        if held != target {
            let next = if held == i32::MAX as u32 { 0 } else { held + 1 };
            drives.push((signal, FourStateValue::from_u64(32, next as u64)));
        }
    }
    Ok(drives)
}
impl MixedSignalHost {
    /// Prepare this candidate through the same evaluation used for matrix assembly.
    /// Settlement may precede stamping, especially when accepting a refined root.
    /// Reading retained counters before this preparation would publish old events.
    pub(super) fn analog_event_targets(
        &mut self,
        solution: &[f64],
    ) -> Result<Vec<(DigitalSignalId, u32)>, MixedSignalError> {
        if !self
            .state
            .digital
            .plan()
            .analog_probes
            .iter()
            .any(|probe| probe.event_signal.is_some())
        {
            return Ok(Vec::new());
        }
        self.sample_discrete_inputs()?;
        // A non-finite Newton candidate remains rejectable even when settlement
        // needs its event counters before the matrix assembly does.
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
        for probe in &self.state.digital.plan().analog_probes {
            let Some(signal) = probe.event_signal else {
                continue;
            };
            let rspice_veriloga::canonical_ir::digital::DigitalAnalogProbeTarget::Variable { name } =
                &probe.target
            else {
                unreachable!("validated event probe")
            };
            let value =
                self.analog
                    .variable(name)
                    .ok_or_else(|| MixedSignalError::InvalidBridge {
                        detail: format!("analog event counter '{name}' has no retained evaluation"),
                    })?;
            if !value.is_finite() || value.fract() != 0.0 || value < 0.0 || value > i32::MAX as f64
            {
                return Err(MixedSignalError::InvalidBridge {
                    detail: format!("invalid analog event counter '{name}': {value}"),
                });
            }
            targets.push((signal, value as u32));
        }
        Ok(targets)
    }
    pub(super) fn publish_local_analog_events(
        &mut self,
        solution: &[f64],
    ) -> Result<bool, MixedSignalError> {
        if self.state.digital.is_view() {
            return Ok(false);
        }
        let targets = self.analog_event_targets(solution)?;
        if targets.is_empty() {
            return Ok(false);
        }
        let time = self.trial.as_ref().expect("active trial").time_seconds;
        let tick = hdl_tick(time, |at| at.nearest_tick(self.resolution))?;
        let trial = self.trial.as_mut().unwrap();
        trial.published_tick = trial.published_tick.max(tick);
        let tick = trial.published_tick;
        let mut published = false;
        for wave in 0..=self.max_bridge_iterations {
            let drives = event_bank(&targets, |signal| {
                self.state
                    .digital
                    .read(signal)
                    .and_then(FourStateValue::to_u64)
            })?;
            if drives.is_empty() {
                return Ok(published);
            }
            if wave == self.max_bridge_iterations {
                return Err(MixedSignalError::BridgeIterationLimit {
                    tick,
                    limit: self.max_bridge_iterations,
                });
            }
            self.with_analog_participant(solution, |digital, producer| {
                digital.force_many_from_analog_at(&drives, tick, time, time, producer)
            })?;
            published = true;
        }
        unreachable!("the final wave returns without publishing")
    }
}
