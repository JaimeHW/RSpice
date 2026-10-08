//! Publish retained analog occurrences through the existing analog causal lane.
use super::*;

const COUNTER_MASK: u32 = i32::MAX as u32;

/// Analog evaluation may withdraw an occurrence after its digital consequence
/// changes an event operand. Delivery cannot withdraw that consequence. Count
/// occurrences relative to each domain's accepted origin, retaining the largest
/// count observed during this trial. Rejection discards this speculative bank.
#[derive(Clone)]
pub(super) struct AnalogEventTrial {
    analog_origin: u32,
    digital_origin: u32,
    occurrences: u32,
}

impl AnalogEventTrial {
    fn observe(&mut self, counter: u32) -> u32 {
        self.occurrences = self
            .occurrences
            .max(counter.wrapping_sub(self.analog_origin) & COUNTER_MASK);
        self.digital_origin.wrapping_add(self.occurrences) & COUNTER_MASK
    }
}

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
    fn analog_event_counter(&self, name: &str) -> Result<u32, MixedSignalError> {
        let value = self
            .analog
            .variable(name)
            .ok_or_else(|| MixedSignalError::InvalidBridge {
                detail: format!("analog event counter '{name}' has no retained evaluation"),
            })?;
        if !value.is_finite()
            || value.fract() != 0.0
            || value < 0.0
            || value > f64::from(COUNTER_MASK)
        {
            return Err(MixedSignalError::InvalidBridge {
                detail: format!("invalid analog event counter '{name}': {value}"),
            });
        }
        Ok(value as u32)
    }

    pub(super) fn prepare_analog_event_trial(&mut self) -> Result<(), MixedSignalError> {
        self.scratch.trial.analog_events.clear();
        for probe in &self.state.digital.plan().analog_probes {
            let Some(signal) = probe.event_signal else {
                continue;
            };
            let rspice_veriloga::canonical_ir::digital::DigitalAnalogProbeTarget::Variable { name } =
                &probe.target
            else {
                unreachable!("validated event probe")
            };
            let analog_origin = if self.state.started {
                self.analog_event_counter(name)?
            } else {
                0
            };
            // Shared views acquire their initialized counter values when the
            // coordinator starts the first trial. Their logical origin is zero.
            let digital_origin = if self.state.started {
                self.state
                    .digital
                    .read(signal)
                    .and_then(FourStateValue::to_u64)
                    .filter(|value| *value <= u64::from(COUNTER_MASK))
                    .ok_or_else(|| MixedSignalError::InvalidBridge {
                        detail: "analog event counter lost its initialized signal".into(),
                    })? as u32
            } else {
                0
            };
            self.scratch.trial.analog_events.push(AnalogEventTrial {
                analog_origin,
                digital_origin,
                occurrences: 0,
            });
        }
        Ok(())
    }

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
            let value = self.analog_event_counter(name)?;
            let trial = self.trial.as_mut().expect("active analog event trial");
            let target = trial.vectors.analog_events[targets.len()].observe(value);
            targets.push((signal, target));
        }
        // Preserve the cause before its digital handler can change the event
        // operand and erase the model's candidate root on reevaluation.
        if targets.iter().any(|(signal, target)| {
            self.state
                .digital
                .read(*signal)
                .and_then(FourStateValue::to_u64)
                != Some(u64::from(*target))
        }) && let Some(root) = self
            .analog
            .try_transient_event_refinement_time()
            .map_err(|error| classify(&error))?
        {
            let trial = self.trial.as_mut().expect("active analog event trial");
            let start = self.state.accepted_time;
            let tolerance = endpoint_root_window(trial.time_seconds, start, self.analog_step_floor);
            if self.state.started && root > start && trial.time_seconds - root > tolerance {
                trial.observation_refinement = Some(
                    trial
                        .observation_refinement
                        .map_or(root, |previous| previous.min(root)),
                );
            }
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
