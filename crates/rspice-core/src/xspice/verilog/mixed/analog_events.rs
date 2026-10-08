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
    fn observe(&mut self, counter: u32) -> Option<u32> {
        let occurrence = counter.wrapping_sub(self.analog_origin) & COUNTER_MASK;
        if occurrence <= self.occurrences {
            return None;
        }
        self.occurrences = occurrence;
        Some(occurrence)
    }

    fn pending(&self, occurrence: u32, held: u32) -> bool {
        (held.wrapping_sub(self.digital_origin) & COUNTER_MASK) < occurrence
    }

    fn target(&self, occurrence: u32) -> u32 {
        self.digital_origin.wrapping_add(occurrence) & COUNTER_MASK
    }
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
        self.scratch.trial.analog_event_order.clear();
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
    ) -> Result<Vec<Vec<(DigitalSignalId, u32)>>, MixedSignalError> {
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
        let bindings: Vec<_> = self
            .state
            .digital
            .plan()
            .analog_probes
            .iter()
            .filter_map(|probe| {
                let signal = probe.event_signal?;
                let rspice_veriloga::canonical_ir::digital::DigitalAnalogProbeTarget::Variable {
                    name,
                } = &probe.target
                else {
                    unreachable!("validated event probe")
                };
                Some((name.clone(), signal))
            })
            .collect();
        let indices: std::collections::BTreeMap<_, _> = bindings
            .iter()
            .enumerate()
            .map(|(index, (name, _))| (name.as_str(), index))
            .collect();
        let occurrences = self
            .analog
            .analog_assignment_occurrence_groups()
            .map_err(|error| classify(&error))?;
        let trial = self.trial.as_mut().expect("active analog event trial");
        let mut pending_group = None;
        for (name, counter, group) in occurrences {
            let index = *indices
                .get(name)
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!("analog occurrence {name} has no digital binding"),
                })?;
            if let Some(occurrence) = trial.vectors.analog_events[index].observe(counter) {
                let allocation_error = |_| MixedSignalError::InvalidBridge {
                    detail: "could not retain analog occurrence order".into(),
                };
                if pending_group != Some(group) {
                    trial
                        .vectors
                        .analog_event_order
                        .try_reserve(1)
                        .map_err(allocation_error)?;
                    trial.vectors.analog_event_order.push(Vec::new());
                    pending_group = Some(group);
                }
                let members = trial.vectors.analog_event_order.last_mut().unwrap();
                members.try_reserve(1).map_err(allocation_error)?;
                members.push((index, occurrence));
            }
        }
        // A missing backend record must never silently turn ordered events back
        // into an unordered counter bank.
        for (index, (name, _)) in bindings.iter().enumerate() {
            let value = self.analog_event_counter(name)?;
            let trial = self.trial.as_ref().unwrap();
            let counter = &trial.vectors.analog_events[index];
            if (value.wrapping_sub(counter.analog_origin) & COUNTER_MASK) > counter.occurrences {
                return Err(MixedSignalError::InvalidBridge {
                    detail: format!(
                        "analog event counter {name} advanced without an occurrence record"
                    ),
                });
            }
        }
        let trial = self.trial.as_ref().unwrap();
        let mut targets = Vec::new();
        for group in &trial.vectors.analog_event_order {
            let mut members = Vec::new();
            for &(index, occurrence) in group {
                let signal = bindings[index].1;
                let held = self
                    .state
                    .digital
                    .read(signal)
                    .and_then(FourStateValue::to_u64)
                    .filter(|value| *value <= u64::from(COUNTER_MASK))
                    .ok_or_else(|| MixedSignalError::InvalidBridge {
                        detail: "analog event counter lost its initialized signal".into(),
                    })? as u32;
                let counter = &trial.vectors.analog_events[index];
                if counter.pending(occurrence, held) {
                    members.push((signal, counter.target(occurrence)));
                }
            }
            if !members.is_empty() {
                targets.push(members);
            }
        }
        // Preserve the cause before its digital handler can change the event
        // operand and erase the model's candidate root on reevaluation.
        if targets.iter().flatten().any(|(signal, target)| {
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
        // Publish every aggregate together. Digital controls may rearm between
        // source assignments, never between members of one assignment.
        for group in targets {
            let drives: Vec<_> = group
                .into_iter()
                .map(|(signal, value)| (signal, FourStateValue::from_u64(32, u64::from(value))))
                .collect();
            self.with_analog_participant(solution, |digital, producer| {
                digital.force_many_from_analog_at(&drives, tick, time, time, producer)
            })?;
        }
        Ok(true)
    }
}
