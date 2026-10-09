//! Accepted shared execution only. Per-instance analog/bridge state and the
//! surrounding circuit must join this replacement in one installation.
use super::*;
use crate::xspice::verilog::host::checkpoint::{HostCheckpoint, HostCheckpointLimits};
use crate::xspice::verilog::store::checkpoint::fingerprint;
use rspice_veriloga_runtime::absdelta::{AbsDeltaControls, AbsDeltaInterval};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CoordinatorCheckpoint {
    version: u32,
    topology: [u8; 32],
    time: u64,
    digital: HostCheckpoint,
    observers: Vec<[u64; 8]>,
    probes: Vec<Option<u64>>,
}

impl MixedDigitalCoordinator {
    fn checkpoint_topology(&self) -> Result<[u8; 32], String> {
        let maps: Vec<_> = self
            .maps
            .iter()
            .map(|map| {
                (
                    &map.name,
                    &map.signals,
                    &map.processes,
                    &map.drivers,
                    &map.connection_processes,
                    &map.analog_probes,
                    &map.source_files,
                )
            })
            .collect();
        fingerprint(&(
            1u32,
            self.digital.plan().content_identity,
            self.resolution.seconds_per_tick().to_bits(),
            self.analog_step_floor.to_bits(),
            maps,
            &self.port_signals,
            &self.event_nodes,
            &self.real_event_nodes,
        ))
    }

    fn checkpoint_budget(
        &self,
        mut limits: HostCheckpointLimits,
    ) -> Result<HostCheckpointLimits, String> {
        let items = self
            .accepted_observers
            .len()
            .checked_add(self.accepted_observation_probes.len())
            .ok_or("coordinator checkpoint dimensions overflow")?;
        limits.digital.max_items = limits
            .digital
            .max_items
            .checked_sub(items)
            .ok_or("coordinator observations exceed checkpoint limit")?;
        Ok(limits)
    }

    fn validate_checkpoint_history(&self, time: f64) -> Result<(), String> {
        hdl_tick(time, |at| at.floor_tick(self.resolution)).map_err(|e| e.to_string())?;
        for ((state, observer), index) in self
            .accepted_observers
            .iter()
            .zip(&self.digital.plan().absdelta)
            .zip(0usize..)
        {
            let words = state.checkpoint_words();
            AbsDeltaState::from_checkpoint_words(&words)?;
            let values: Result<Vec<_>, _> = observer
                .operands
                .iter()
                .map(|probe| {
                    self.accepted_observation_probes
                        .get(usize::from(*probe))
                        .copied()
                        .flatten()
                        .ok_or_else(|| format!("observer {index} checkpoint has no operand sample"))
                })
                .collect();
            let values = values?;
            let sample = state
                .last_sample()
                .ok_or("accepted observer has no sample")?;
            if sample.time != time || sample.value.to_bits() != values[0].to_bits() {
                return Err("observer checkpoint does not match accepted time and probe".into());
            }
            let precision = observer
                .time_scale
                .parameter_value("timePrecision")
                .map_err(|e| e.to_string())?
                .ok_or("observer has no time precision")?;
            // Constructing an interval validates controls without evaluating an
            // expression, enumerating events or recapturing the baseline.
            AbsDeltaInterval::new(
                *state,
                sample,
                AbsDeltaControls {
                    delta: values[1],
                    time_tolerance: values[2],
                    expression_tolerance: values[3],
                    enable: values[4],
                },
                precision,
                false,
            )
            .map_err(|e| e.to_string())?;
            if (words[1] & 4 != 0) != (values[4] != 0.0) {
                return Err("observer enable state differs from accepted control".into());
            }
        }
        Ok(())
    }

    pub(crate) fn checkpoint(
        &self,
        limits: HostCheckpointLimits,
    ) -> Result<CoordinatorCheckpoint, String> {
        if !self.enabled || self.trial_open {
            return Err("coordinator must be initialized with no open trial before capture".into());
        }
        let time = self
            .accepted_time
            .ok_or("coordinator has no accepted timepoint")?;
        self.validate_checkpoint_history(time)?;
        self.digital.validate_checkpoint_time(time)?;
        Ok(CoordinatorCheckpoint {
            version: 1,
            topology: self.checkpoint_topology()?,
            time: time.to_bits(),
            digital: self.digital.checkpoint(self.checkpoint_budget(limits)?)?,
            observers: self
                .accepted_observers
                .iter()
                .map(|state| state.checkpoint_words())
                .collect(),
            probes: self
                .accepted_observation_probes
                .iter()
                .map(|value| value.map(f64::to_bits))
                .collect(),
        })
    }

    /// Build a replacement against an already elaborated coordinator. The
    /// caller bounds input bytes before deserialization and installs this only
    /// after all circuit participants have also validated. Receiving resource
    /// policies are retained; the saved image cannot raise them.
    pub(crate) fn restored_checkpoint(
        &self,
        image: &CoordinatorCheckpoint,
        limits: HostCheckpointLimits,
    ) -> Result<Self, String> {
        if self.trial_open {
            return Err("cannot restore coordinator during a trial".into());
        }
        if image.version != 1 || image.topology != self.checkpoint_topology()? {
            return Err("coordinator checkpoint schema, topology or time policy differs".into());
        }
        if image.observers.len() != self.accepted_observers.len()
            || image.probes.len() != self.accepted_observation_probes.len()
        {
            return Err("coordinator checkpoint observation dimensions differ".into());
        }
        let limits = self.checkpoint_budget(limits)?;
        let time = f64::from_bits(image.time);
        let mut restored = self.fresh();
        restored.accepted_observers = image
            .observers
            .iter()
            .map(|words| AbsDeltaState::from_checkpoint_words(words))
            .collect::<Result<_, _>>()?;
        restored.accepted_observation_probes = image
            .probes
            .iter()
            .map(|value| value.map(f64::from_bits))
            .collect();
        restored.validate_checkpoint_history(time)?;
        restored.digital =
            MixedCell::new(self.digital.restored_checkpoint(&image.digital, limits)?);
        restored.digital.validate_checkpoint_time(time)?;
        restored.enabled = true;
        restored.accepted_time = Some(time);
        Ok(restored)
    }
}

#[cfg(test)]
mod tests;
