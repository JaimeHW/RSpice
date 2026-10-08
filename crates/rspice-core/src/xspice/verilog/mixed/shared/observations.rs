//! Interleave interpolated analog observations with the shared HDL timeline.
use super::*;
use rspice_veriloga::canonical_ir::digital::DigitalAnalogQuantity;
use rspice_veriloga_runtime::absdelta::{
    AbsDeltaControls, AbsDeltaEvent, AbsDeltaInterval, AbsDeltaSample,
};

fn observation_error(error: impl std::fmt::Display) -> MixedSignalError {
    MixedSignalError::Analog {
        detail: format!("absdelta: {error}"),
    }
}

struct PendingObservation {
    signal: DigitalSignalId,
    interval: AbsDeltaInterval,
    next: Option<AbsDeltaEvent>,
}

/// A sampling activation observes one common interpolation bank. Invalidation
/// after an HDL write must not cause a second analog operator evaluation.
struct InterpolatedParticipant<'a, 'p> {
    samples: &'a [Option<f64>],
    external: Option<&'p mut dyn DigitalActiveParticipant>,
}

impl DigitalActiveParticipant for InterpolatedParticipant<'_, '_> {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        match &mut self.external {
            Some(external) => external.settle_active(exchange),
            None => {
                exchange.require_standalone_execution()?;
                Ok(false)
            }
        }
    }

    fn sample_analog(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<(), DigitalRunError> {
        let samples = exchange
            .analog_sample_requests()
            .into_iter()
            .map(|id| {
                self.samples
                    .get(usize::from(id))
                    .copied()
                    .flatten()
                    .map(|value| (id, value))
                    .ok_or_else(|| DigitalRunError::ExternalExecution {
                        detail: format!("interpolated analog sample {id} is unavailable"),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        exchange.publish_analog_variables(&samples)
    }
}

impl MixedDigitalCoordinator {
    fn capture_observation_probes(
        &mut self,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
    ) -> Result<Vec<Option<f64>>, MixedSignalError> {
        let mut samples = vec![None; self.probes.len()];
        for (host, map) in hosts.iter_mut().zip(&self.maps) {
            host.sample_discrete_inputs()?;
            let classify = if host.trial.as_ref().is_some_and(|trial| trial.probe) {
                analog_trial_error
            } else {
                analog_accepted_error
            };
            host.prepared_analog.prepare(
                host.analog.make_mut(),
                &host.discrete_inputs,
                solution,
                classify,
            )?;
            for (probe, global) in host.analog_probes.iter().zip(&map.analog_probes) {
                samples[usize::from(*global)] = match probe {
                    AnalogProbeWiring::Variable { name, .. } => host.analog.variable(name),
                    _ => probe.sample(solution),
                };
            }
        }
        Ok(samples)
    }

    fn interpolate_observation_probes(
        &self,
        cursor: &SharedTrialCursor,
        endpoint: &[Option<f64>],
        time: f64,
    ) -> Vec<Option<f64>> {
        let start = cursor.observation_time.unwrap_or(cursor.time);
        let fraction = if cursor.time > start {
            ((time - start) / (cursor.time - start)).clamp(0.0, 1.0)
        } else {
            1.0
        };
        self.digital
            .plan()
            .analog_probes
            .iter()
            .enumerate()
            .map(|(index, probe)| {
                let right = endpoint[index]?;
                let left = cursor
                    .observation_probes
                    .get(index)
                    .copied()
                    .flatten()
                    .unwrap_or(right);
                let value = if fraction == 1.0 {
                    right
                } else if probe.retained {
                    left
                } else if (right - left).is_finite() {
                    left + fraction * (right - left)
                } else {
                    (1.0 - fraction) * left + fraction * right
                };
                Some(
                    if probe.quantity == DigitalAnalogQuantity::IntegerVariable {
                        value.round()
                    } else {
                        value
                    },
                )
            })
            .collect()
    }

    pub(super) fn publish_observed_interval(
        &mut self,
        cursor: &mut SharedTrialCursor,
        hosts: &mut [MixedSignalHost],
        solution: &[f64],
        mut participant: Option<&mut dyn DigitalActiveParticipant>,
    ) -> Result<bool, MixedSignalError> {
        if cursor.observation_refinement.is_some() {
            return Ok(false);
        }
        let endpoint = self.capture_observation_probes(hosts, solution)?;
        let mut observers = Vec::with_capacity(cursor.observers.len());
        for (index, observer) in self.digital.plan().absdelta.iter().enumerate() {
            let mut values = [0.0; 5];
            for (value, probe) in values.iter_mut().zip(observer.operands) {
                *value = endpoint[usize::from(probe)].ok_or_else(|| {
                    observation_error(format!("operand {probe} has no analog evaluation"))
                })?;
            }
            let precision = observer
                .time_scale
                .parameter_value("timePrecision")
                .map_err(observation_error)?
                .expect("time precision parameter");
            let mut interval = AbsDeltaInterval::new(
                cursor.observers[index],
                AbsDeltaSample {
                    time: cursor.time,
                    value: values[0],
                },
                AbsDeltaControls {
                    delta: values[1],
                    time_tolerance: values[2],
                    expression_tolerance: values[3],
                    enable: values[4],
                },
                precision,
                cursor.observation_time.is_none(),
            )
            .map_err(observation_error)?;
            let next = interval.next_event().map_err(observation_error)?;
            observers.push(PendingObservation {
                signal: observer.signal,
                interval,
                next,
            });
        }
        self.collect_adc_publications(cursor, hosts)?;
        let mut adc = 0;
        let mut published = false;
        let mut external_endpoint =
            participant.is_some() && cursor.observation_time != Some(cursor.time);
        let mut work = 0;
        let limit = self.digital.scheduler_limits().max_events_per_tick;
        loop {
            let observation_time = observers
                .iter()
                .filter_map(|entry| entry.next.map(|event| event.sample.time))
                .min_by(f64::total_cmp);
            let adc_time = self.publications.get(adc).map(|entry| entry.crossing);
            let scheduled_tick = self.digital.next_tick();
            let scheduled = scheduled_tick
                .map(|tick| self.resolution.ticks_to_seconds(tick))
                .transpose()
                .map_err(DigitalRunError::from)?;
            let external_time = participant
                .as_deref()
                .and_then(DigitalActiveParticipant::next_event_time)
                .filter(|time| *time <= cursor.time);
            let next = [
                observation_time,
                adc_time,
                scheduled.filter(|time| *time <= cursor.time),
                external_time,
                external_endpoint.then_some(cursor.time),
            ]
            .into_iter()
            .flatten()
            .min_by(f64::total_cmp);
            let Some(time) = next else {
                break;
            };
            work += 1;
            if work > limit {
                return Err(MixedSignalError::TrialProtocol {
                    detail: format!(
                        "absdelta interval exceeds the configured event-work budget of {limit}"
                    ),
                });
            }
            let bank = self.interpolate_observation_probes(cursor, &endpoint, time);
            let mut drives = Vec::new();
            let mut publication_tick = cursor.published_tick;
            for (index, observer) in observers.iter_mut().enumerate() {
                if let Some(event) = observer.next
                    && event.sample.time == time
                {
                    publication_tick = publication_tick
                        .max(hdl_tick(time, |at| at.nearest_tick(self.resolution))?);
                    let held = self
                        .digital
                        .read(observer.signal)
                        .and_then(FourStateValue::to_u64)
                        .ok_or_else(|| observation_error("occurrence signal is not initialized"))?;
                    let count = if held >= i32::MAX as u64 { 0 } else { held + 1 };
                    drives.push((observer.signal, FourStateValue::from_u64(32, count)));
                    cursor.observers[index] = event.state;
                    observer.next = observer.interval.next_event().map_err(observation_error)?;
                }
            }
            while let Some(entry) = self.publications.get(adc).copied()
                && entry.crossing == time
            {
                publication_tick = publication_tick.max(entry.tick);
                let host = &hosts[entry.host];
                let bridge = &host.state.bridges.adc[entry.bridge];
                let global = self.port_signals[entry.host][usize::from(bridge.driven_signal())];
                let current = drives.iter().position(|(signal, _)| *signal == global);
                if let Some(index) = current {
                    drives[index].1.set_bit(bridge.bit, entry.bit);
                } else {
                    let mut value = self
                        .digital
                        .read(global)
                        .expect("linked ADC signal")
                        .clone();
                    value.set_bit(bridge.bit, entry.bit);
                    drives.push((global, value));
                }
                adc += 1;
            }
            let external_due =
                external_time == Some(time) || (external_endpoint && time == cursor.time);
            if external_due {
                publication_tick =
                    publication_tick.max(hdl_tick(time, |at| at.ceil_tick(self.resolution))?);
            }
            cursor.published_tick = publication_tick;
            let external = participant
                .as_mut()
                .map(|external| &mut **external as &mut dyn DigitalActiveParticipant);
            let mut active = InterpolatedParticipant {
                samples: &bank,
                external,
            };
            let digital = self.digital.make_mut();
            digital.sample_analog_probes(&bank);
            if !drives.is_empty() || external_due {
                digital.force_many_from_analog_at(
                    &drives,
                    cursor.published_tick,
                    time,
                    time,
                    &mut active,
                )?;
                published = true;
            }
            if scheduled == Some(time) {
                digital
                    .advance_to_with(scheduled_tick.expect("scheduled event tick"), &mut active)?;
                published = true;
            }
            if time == cursor.time {
                external_endpoint = false;
            }
            self.synchronize(hosts)?;
            let external_feedback = participant
                .as_deref()
                .is_some_and(DigitalActiveParticipant::analog_feedback_pending);
            let feedback = hosts.iter().try_fold(external_feedback, |changed, host| {
                host.digital_feedback_since_trial_start()
                    .map(|value| changed || value)
            })?;
            let endpoint_window = endpoint_root_window(
                cursor.time,
                self.accepted_time.unwrap_or(0.0),
                self.analog_step_floor(),
            );
            if feedback && cursor.time - time > endpoint_window {
                cursor.observation_refinement = Some(time);
                for host in hosts.iter_mut() {
                    if let Some(trial) = &mut host.trial {
                        trial.observation_refinement = Some(time);
                    }
                }
                return Ok(true);
            }
        }
        for (state, observer) in cursor.observers.iter_mut().zip(observers) {
            *state = observer
                .interval
                .candidate()
                .expect("drained observer interval");
        }
        cursor.observation_time = Some(cursor.time);
        cursor.observation_probes = endpoint;
        self.probes.clone_from(&cursor.observation_probes);
        Ok(self.publish_counter_events_with(cursor, hosts, solution, participant)? || published)
    }
}
