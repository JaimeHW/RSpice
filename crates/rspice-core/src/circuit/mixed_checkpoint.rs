//! The discrete and mixed-model part of an accepted circuit checkpoint.
//! The outer transient format authenticates sources/configuration and restores
//! native devices, the solution and the step controller alongside this image.
use super::CircuitData;
use crate::xspice::event_checkpoint::{EventCheckpointLimits, XspiceEventCheckpoint};
use crate::xspice::instance_checkpoint::{InstanceCheckpointLimits, InstanceRuntimeCheckpoint};
use crate::xspice::verilog::coordinator_checkpoint::CoordinatorCheckpoint;
use crate::xspice::verilog::host::checkpoint::HostCheckpointLimits;
use crate::xspice::verilog::participant_checkpoint::{
    ParticipantCheckpoint, ParticipantCheckpointLimits,
};
use crate::xspice::verilog::{MixedDigitalCoordinator, MixedSignalHost};
use crate::xspice::{
    SharedXspiceEventQueue, SharedXspiceEventValues, SharedXspiceInstance, XspiceEventInputs,
};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub(crate) struct MixedCheckpointLimits {
    pub max_bytes: usize,
    pub max_instances: usize,
    pub host: HostCheckpointLimits,
    pub participant: ParticipantCheckpointLimits,
    pub instance: InstanceCheckpointLimits,
    pub events: EventCheckpointLimits,
}
impl Default for MixedCheckpointLimits {
    fn default() -> Self {
        Self {
            max_bytes: 64 * 1024 * 1024,
            max_instances: 1_048_576,
            host: Default::default(),
            participant: Default::default(),
            instance: Default::default(),
            events: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MixedCircuitCheckpoint {
    version: u32,
    time: u64,
    num_nodes: usize,
    matrix_size: usize,
    coordinator: Option<CoordinatorCheckpoint>,
    participants: Vec<ParticipantCheckpoint>,
    instances: Vec<InstanceRuntimeCheckpoint>,
    events: XspiceEventCheckpoint,
}

/// Prepared replacements own no mutable references to the receiving circuit.
/// The caller installs them only after every other transient component validates.
pub(crate) struct RestoredMixedCircuit {
    coordinator: Option<MixedDigitalCoordinator>,
    participants: Vec<MixedSignalHost>,
    instances: Vec<SharedXspiceInstance>,
    queue: SharedXspiceEventQueue,
    values: SharedXspiceEventValues,
}
impl RestoredMixedCircuit {
    pub(crate) fn install(self, circuit: &mut CircuitData) {
        circuit.scheduler.mixed_digital_coordinator = self.coordinator;
        circuit.mixed_signal_hosts = self.participants;
        circuit.xspice_instances = self.instances;
        circuit.scheduler.xspice_event_queue = self.queue;
        circuit.scheduler.xspice_event_values = self.values;
        circuit.scheduler.ledgers.clear();
        circuit.invalidate_xspice_event_dispatch();
        circuit.xspice_touched_digital_nodes.clear();
        circuit.xspice_touched_real_nodes.clear();
        circuit.xspice_dispatch_pending.clear();
        circuit.xspice_dispatch_next_pending.clear();
        circuit.xspice_output_iterates = Default::default();
        circuit.xspice_evaluation_warning = None;
    }
}

/// Counts canonical encoded bytes without allocating another copy of an image.
struct ByteBudget(usize);
impl Write for ByteBudget {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self.0.checked_sub(bytes.len()).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "mixed checkpoint byte limit exceeded",
            )
        })?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl ByteBudget {
    fn account(&mut self, value: &impl Serialize) -> Result<(), String> {
        serde_json::to_writer(self, value).map_err(|error| error.to_string())
    }
}

fn count_instances(circuit: &CircuitData, limits: MixedCheckpointLimits) -> Result<(), String> {
    circuit
        .xspice_instances
        .len()
        .checked_add(circuit.mixed_signal_hosts.len())
        .filter(|count| *count <= limits.max_instances)
        .ok_or("mixed checkpoint instance limit exceeded")?;
    Ok(())
}

impl CircuitData {
    fn validate_mixed_checkpoint_idle(&self) -> Result<(), String> {
        if self.xspice_evaluation_error.is_some()
            || self
                .xspice_resource_failure
                .as_ref()
                .is_some_and(|failure| failure.get().is_some())
        {
            return Err("failed XSPICE execution cannot be checkpointed or resumed".into());
        }
        if self
            .scheduler
            .ledgers
            .iter()
            .any(|ledger| ledger.has_pending_candidate())
        {
            return Err("mixed checkpoint has a pending candidate ledger".into());
        }
        if self.scheduler.mixed_digital_coordinator.is_some() != !self.mixed_signal_hosts.is_empty()
        {
            return Err("mixed checkpoint requires a complete enrolled circuit".into());
        }
        // A trial temporarily takes both owners out of CircuitData. Its parked
        // ledger slots remain, so empty owners cannot masquerade as an idle
        // circuit during the acceptance callback.
        if !self.scheduler.ledgers.is_empty()
            && self.scheduler.ledgers.len() != self.mixed_signal_hosts.len()
        {
            return Err("mixed checkpoint owners are lent to an open trial".into());
        }
        Ok(())
    }

    fn validate_checkpoint_instance_observations(
        &self,
        instance: &crate::xspice::XspiceInstance,
        values: &crate::xspice::XspiceEventValues,
        coordinator: Option<&MixedDigitalCoordinator>,
    ) -> Result<(), String> {
        instance
            .validate_runtime_event_observations(
                XspiceEventInputs {
                    digital_values: &values.digital_values,
                    digital_event_times: &values.digital_event_times,
                    event_total_loads: &self.xspice_event_loads,
                    real_values: &values.real_values,
                    real_event_times: &values.real_event_times,
                },
                &values.digital_drivers,
                |target| match (&self.scheduler.mixed_xspice_bindings, coordinator) {
                    (Some(bindings), Some(owner)) => {
                        bindings.checkpoint_other_drivers(owner, target)
                    }
                    (None, _) => Ok(None),
                    _ => Err("shared XSPICE checkpoint has no restored HDL owner".into()),
                },
            )
            .map_err(|error| format!("{}({}): {error}", instance.name, instance.model_name()))
    }

    pub(crate) fn mixed_runtime_checkpoint(
        &self,
        time: f64,
        limits: MixedCheckpointLimits,
    ) -> Result<MixedCircuitCheckpoint, String> {
        self.validate_mixed_checkpoint_idle()?;
        count_instances(self, limits)?;
        let mut budget = ByteBudget(limits.max_bytes);
        let owner = self.scheduler.mixed_digital_coordinator.as_ref();
        let coordinator = owner
            .map(|owner| {
                owner.validate_checkpoint_boundary(time)?;
                owner.checkpoint(limits.host)
            })
            .transpose()?;
        budget.account(&coordinator)?;
        let topology = self.xspice_event_checkpoint_topology(owner)?;
        let events = XspiceEventCheckpoint::capture(
            &self.scheduler.xspice_event_queue,
            &self.scheduler.xspice_event_values,
            time,
            &topology,
            limits.events,
        )?;
        budget.account(&events)?;
        let mut participants = Vec::with_capacity(self.mixed_signal_hosts.len());
        for host in &self.mixed_signal_hosts {
            host.validate_participant_checkpoint_boundary(time)?;
            let image = host.participant_checkpoint(limits.participant)?;
            budget.account(&image)?;
            participants.push(image);
        }
        let mut instances = Vec::with_capacity(self.xspice_instances.len());
        for instance in &self.xspice_instances {
            self.validate_checkpoint_instance_observations(
                instance,
                &self.scheduler.xspice_event_values,
                owner,
            )?;
            let image = instance.runtime_checkpoint(time, self.matrix_size(), limits.instance)?;
            budget.account(&image)?;
            instances.push(image);
        }
        let image = MixedCircuitCheckpoint {
            version: 1,
            time: time.to_bits(),
            num_nodes: self.num_nodes(),
            matrix_size: self.matrix_size(),
            coordinator,
            participants,
            instances,
            events,
        };
        ByteBudget(limits.max_bytes).account(&image)?;
        Ok(image)
    }
}

impl MixedCircuitCheckpoint {
    /// Bound transport bytes before serde can allocate nested arrays or names.
    /// Per-component semantic limits are checked again before reconstruction.
    pub(crate) fn decode(bytes: &[u8], limits: MixedCheckpointLimits) -> Result<Self, String> {
        if bytes.len() > limits.max_bytes {
            return Err("mixed checkpoint byte limit exceeded".into());
        }
        serde_json::from_slice(bytes).map_err(|error| error.to_string())
    }

    pub(crate) fn restore(
        &self,
        circuit: &CircuitData,
        time: f64,
        limits: MixedCheckpointLimits,
    ) -> Result<RestoredMixedCircuit, String> {
        circuit.validate_mixed_checkpoint_idle()?;
        count_instances(circuit, limits)?;
        if self.version != 1
            || self.time != time.to_bits()
            || self.num_nodes != circuit.num_nodes()
            || self.matrix_size != circuit.matrix_size()
            || self.instances.len() != circuit.xspice_instances.len()
            || self.participants.len() != circuit.mixed_signal_hosts.len()
        {
            return Err("mixed checkpoint schema, time or circuit dimensions differ".into());
        }
        ByteBudget(limits.max_bytes).account(self)?;
        let coordinator = match (
            &self.coordinator,
            &circuit.scheduler.mixed_digital_coordinator,
        ) {
            (Some(image), Some(owner)) => {
                let restored = owner.restored_checkpoint(image, limits.host)?;
                restored.validate_checkpoint_boundary(time)?;
                Some(restored)
            }
            (None, None) => None,
            _ => return Err("mixed checkpoint coordinator differs from circuit".into()),
        };
        let topology = circuit.xspice_event_checkpoint_topology(coordinator.as_ref())?;
        let (queue, values) = self.events.restore(
            &circuit.scheduler.xspice_event_queue,
            &circuit.scheduler.xspice_event_values,
            time,
            &topology,
            limits.events,
        )?;
        let mut participants = Vec::with_capacity(self.participants.len());
        for (image, host) in self.participants.iter().zip(&circuit.mixed_signal_hosts) {
            participants.push(
                host.restored_participant_checkpoint(
                    image,
                    coordinator
                        .as_ref()
                        .ok_or("missing mixed checkpoint coordinator")?,
                    limits.participant,
                )?,
            );
        }
        let mut instances = Vec::with_capacity(self.instances.len());
        for (image, instance) in self.instances.iter().zip(&circuit.xspice_instances) {
            let mut template = (**instance).clone();
            template.prepare_checkpoint_loads(&circuit.xspice_event_loads)?;
            let restored = template.restored_runtime_checkpoint(
                image,
                time,
                self.num_nodes,
                self.matrix_size,
                limits.instance,
            )?;
            circuit.validate_checkpoint_instance_observations(
                &restored,
                &values,
                coordinator.as_ref(),
            )?;
            instances.push(SharedXspiceInstance::new(restored));
        }
        Ok(RestoredMixedCircuit {
            coordinator,
            participants,
            instances,
            queue,
            values,
        })
    }
}

#[cfg(test)]
mod tests;
