//! Cooperative Active-region execution over the existing HDL scheduler.
//! The owning circuit must retain the participant and roll it back with the host;
//! this interface neither launches a second simulator nor promotes an HDL region.
use super::super::store::{DigitalBitChange, ExternalBitDriverId};
use super::*;

/// One external Active wave. Return true while external work remains, even if
/// it did not change an HDL bit. Consume resolved changes in publication order;
/// reading only the last value can lose intervening edges.
pub(crate) trait DigitalActiveParticipant {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError>;

    /// All Active prefixes have drained. Evaluate the requested analog
    /// producers at this physical time and publish their common sample bank.
    fn sample_analog(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<(), DigitalRunError> {
        Err(DigitalRunError::ExternalExecution {
            detail: format!(
                "analog-variable sample requests {:?} require a circuit evaluation participant",
                exchange.analog_sample_requests()
            ),
        })
    }
}

/// Restricted access: an event participant can observe resolved nets and drive
/// its enrolled outputs, but cannot advance HDL time or promote a later region.
pub(crate) struct DigitalActiveExchange<'a> {
    pub(super) host: &'a mut DigitalHost,
    pub(super) tick: u64,
    pub(super) physical_seconds: f64,
}

impl DigitalActiveExchange<'_> {
    pub(crate) fn require_standalone_execution(&self) -> Result<(), DigitalRunError> {
        self.host.require_standalone_execution()
    }
    pub(crate) fn read_signal(&self, signal: DigitalSignalId) -> Option<&FourStateValue> {
        self.host.read(signal)
    }

    pub(crate) fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64> {
        self.host.read_real(signal)
    }
    pub(crate) fn analog_sample_requests(&self) -> Vec<DigitalAnalogProbeId> {
        let mut probes = BTreeSet::new();
        for index in &self.host.analog_waiters {
            if let ProcessStatus::AwaitingAnalog(probe) = self.host.slots[*index].status {
                probes.insert(probe);
            }
        }
        probes.into_iter().collect()
    }

    /// Validate the entire producer bank before publishing any value.
    pub(crate) fn publish_analog_variables(
        &mut self,
        samples: &[(DigitalAnalogProbeId, f64)],
    ) -> Result<(), DigitalRunError> {
        use rspice_veriloga::canonical_ir::digital::DigitalAnalogQuantity;
        let mut seen = BTreeSet::new();
        for &(id, value) in samples {
            let valid = self
                .host
                .plan
                .analog_probe(id)
                .is_some_and(|probe| match probe.quantity {
                    DigitalAnalogQuantity::RealVariable => value.is_finite(),
                    DigitalAnalogQuantity::IntegerVariable => {
                        value.is_finite()
                            && value.fract() == 0.0
                            && value >= i32::MIN as f64
                            && value <= i32::MAX as f64
                    }
                    _ => false,
                });
            if !valid || !seen.insert(id) {
                return Err(DigitalRunError::ExternalExecution {
                    detail: format!("invalid or repeated analog-variable sample {id}: {value}"),
                });
            }
        }
        self.host.store.sample_analog_variables(samples);
        Ok(())
    }
    #[cfg(test)]
    pub(crate) fn tick(&self) -> u64 {
        self.tick
    }
    pub(crate) fn physical_seconds(&self) -> f64 {
        self.physical_seconds
    }
    pub(crate) fn read_net(&self, net: usize) -> Result<DigitalValue, DigitalRunError> {
        self.host
            .store
            .connected_value(net)
            .ok_or_else(|| DigitalRunError::ExternalExecution {
                detail: format!("unknown shared digital net {net}"),
            })
    }
    pub(crate) fn take_changes(&mut self) -> Vec<DigitalBitChange> {
        self.host.store.take_external_bit_changes()
    }
    pub(crate) fn drive_many(
        &mut self,
        drives: &[(ExternalBitDriverId, DigitalValue)],
    ) -> Result<(), DigitalRunError> {
        self.host
            .store
            .check_external_drives(drives)
            .map_err(|detail| DigitalRunError::ExternalExecution { detail })?;
        for (driver, _) in drives {
            self.host
                .scheduler
                .note_external_activation(self.tick, self.host.external_targets[driver.index()])?;
        }
        self.host.store.publish_external_drives(drives);
        Ok(())
    }
}

pub(super) struct NoActiveParticipant;
impl DigitalActiveParticipant for NoActiveParticipant {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        exchange.host.require_standalone_execution()?;
        Ok(false)
    }
}

impl DigitalHost {
    /// Bind every analog-variable producer to the discrete inputs its equations
    /// consume. Until this is provided, all resolved digital changes invalidate
    /// all variable samples. Electrical probes always follow the trial solution.
    pub(crate) fn bind_analog_variable_inputs(
        &mut self,
        dependencies: &[(DigitalAnalogProbeId, Vec<DigitalSignalId>)],
    ) -> Result<(), DigitalRunError> {
        use rspice_veriloga::canonical_ir::digital::DigitalAnalogProbeTarget;
        if self.elaboration_closed {
            return Err(DigitalRunError::ExternalExecution {
                detail: "analog sample inputs must be bound before digital execution starts".into(),
            });
        }
        let expected: BTreeSet<_> = self
            .plan
            .analog_probes
            .iter()
            .filter_map(|probe| {
                matches!(probe.target, DigitalAnalogProbeTarget::Variable { .. })
                    .then_some(probe.id)
            })
            .collect();
        let mut seen = BTreeSet::new();
        let mut inputs = std::collections::HashMap::<_, Vec<_>>::new();
        for (probe, signals) in dependencies {
            if !expected.contains(probe)
                || !seen.insert(*probe)
                || signals
                    .iter()
                    .any(|signal| self.plan.signal(*signal).is_none())
            {
                return Err(DigitalRunError::ExternalExecution {
                    detail: format!("invalid analog-variable input binding for probe {probe}"),
                });
            }
            for signal in signals {
                let probes = inputs.entry(*signal).or_default();
                if !probes.contains(probe) {
                    probes.push(*probe);
                }
            }
        }
        if seen != expected {
            return Err(DigitalRunError::ExternalExecution {
                detail: "analog input bindings must cover every variable probe".into(),
            });
        }
        self.store.bind_analog_variable_inputs(inputs);
        Ok(())
    }
    pub(super) fn resume_analog_waiters(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        use rspice_veriloga::canonical_ir::digital_eval::DigitalEnvironment;
        // Check every request first. A participant that forgot a producer must
        // not resume a prefix of the waiting readers and silently finish.
        for index in &self.analog_waiters {
            let ProcessStatus::AwaitingAnalog(probe) = self.slots[*index].status else {
                unreachable!("sample waiter must have an analog continuation")
            };
            if self.store.read_analog_variable(probe).is_none() {
                return Err(DigitalRunError::Evaluation {
                    process: self.describe(*index),
                    error: DigitalEvalError::AnalogProbeUnavailable(probe),
                });
            }
        }
        let mut waiters = std::mem::take(&mut self.analog_waiters);
        for index in waiters.drain(..) {
            self.queue_ready(index, tick)?;
        }
        self.analog_waiters = waiters;
        Ok(())
    }
    pub(super) fn require_standalone_execution(&self) -> Result<(), DigitalRunError> {
        if self.store.has_external_participants() {
            return Err(DigitalRunError::ExternalExecution {
                detail: "enrolled event drivers require their circuit Active participant".into(),
            });
        }
        Ok(())
    }

    pub(crate) fn is_external_target(&self, target: &EventTarget) -> bool {
        self.store
            .external_sources()
            .iter()
            .any(|(_, external)| external == target)
    }

    pub(crate) fn remap_external_nodes(&mut self, remap: impl Fn(usize) -> usize) {
        assert!(
            !self.elaboration_closed,
            "external nodes can only remap during elaboration"
        );
        self.store.remap_external_nodes(remap);
        self.external_targets = self
            .store
            .external_sources()
            .iter()
            .map(|(_, target)| self.scheduler.intern_target(target.clone()))
            .collect();
    }

    /// Enroll external driver identities at elaboration, before process execution.
    /// The returned IDs and immutable topology survive fresh analysis and cloning.
    pub(crate) fn attach_external_bits(
        &mut self,
        observed: &[usize],
        drivers: &[(usize, EventTarget)],
    ) -> Result<Vec<ExternalBitDriverId>, DigitalRunError> {
        if self.elaboration_closed {
            return Err(DigitalRunError::ExternalExecution {
                detail: "event topology cannot change after digital execution starts".into(),
            });
        }
        let identities = self
            .store
            .attach_external_bits(observed, drivers)
            .map_err(|detail| DigitalRunError::ExternalExecution { detail })?;
        self.external_targets = drivers
            .iter()
            .map(|(_, target)| self.scheduler.intern_target(target.clone()))
            .collect();
        Ok(identities)
    }
}
