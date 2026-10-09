//! Accepted execution state for an explicitly checkpoint-capable XSPICE model.
//! Circuit source/ABI identity and cross-participant input validation remain
//! outer contracts. No image here enables legacy or full-circuit resume.
use super::super::context::checkpoint::{
    ContextCheckpointLimits, ContextPortLayout, ContextRuntimeCheckpoint,
};
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct InstanceCheckpointLimits {
    pub context: ContextCheckpointLimits,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum SignatureValue {
    Digital(DigitalValue),
    Real(u64),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignatureEntry {
    time: Option<u64>,
    value: SignatureValue,
}
impl From<&EventInputSignatureEntry> for SignatureEntry {
    fn from(value: &EventInputSignatureEntry) -> Self {
        Self {
            time: value.event_time.map(f64::to_bits),
            value: match value.value {
                EventInputSignatureValue::Digital(value) => SignatureValue::Digital(value),
                EventInputSignatureValue::Real(value) => SignatureValue::Real(value),
            },
        }
    }
}
impl From<&SignatureEntry> for EventInputSignatureEntry {
    fn from(value: &SignatureEntry) -> Self {
        Self {
            event_time: value.time.map(f64::from_bits),
            value: match value.value {
                SignatureValue::Digital(value) => EventInputSignatureValue::Digital(value),
                SignatureValue::Real(value) => EventInputSignatureValue::Real(value),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InstanceRuntimeCheckpoint {
    version: u32,
    identity: [u8; 32],
    context: ContextRuntimeCheckpoint,
    signature: Option<Vec<SignatureEntry>>,
    dirty: bool,
}

fn remaining_budget(
    count: usize,
    limits: InstanceCheckpointLimits,
) -> Result<ContextCheckpointLimits, String> {
    let used = count
        .checked_mul(2)
        .ok_or("XSPICE signature dimensions overflow")?;
    Ok(ContextCheckpointLimits {
        max_items: limits
            .context
            .max_items
            .checked_sub(used)
            .ok_or("XSPICE signature checkpoint limit exceeded")?,
        ..limits.context
    })
}

struct HashWriter(blake3::Hasher);
impl std::io::Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl XspiceInstance {
    pub(crate) fn runtime_checkpoint_resume_blocker(&self) -> Option<String> {
        self.require_runtime_checkpoint_support()
            .and_then(|()| {
                if self.context.has_checkpoint_host_resources() {
                    Err("live host resources have no portable checkpoint contract".into())
                } else {
                    Ok(())
                }
            })
            .err()
            .map(|reason| format!("{}({}): {reason}", self.name, self.model_name()))
    }

    /// Resolve receiving-circuit loads without evaluating model inputs or body.
    pub(crate) fn prepare_checkpoint_loads(
        &mut self,
        loads: &HashMap<usize, Value>,
    ) -> Result<(), String> {
        for (port, connection) in self.ports.iter().zip(&self.connections) {
            set_context_event_total_load(&mut self.context, &port.name, connection, loads)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    fn runtime_checkpoint_identity(
        &self,
        num_nodes: usize,
        matrix_size: usize,
    ) -> Result<[u8; 32], String> {
        let ports: Vec<_> = self
            .ports
            .iter()
            .map(|p| {
                (
                    &p.name,
                    p.direction,
                    p.default_type,
                    &p.allowed_types,
                    p.is_vector,
                    p.null_allowed,
                    p.vector_min_len,
                    p.vector_max_len,
                )
            })
            .collect();
        let mut vector_branches: Vec<_> = self.output_vector_branches.iter().collect();
        vector_branches.sort_unstable_by_key(|(key, _)| **key);
        let mut writer = HashWriter(blake3::Hasher::new());
        // All fields have finite structural domains; context parameters use
        // their separate IEEE-word fingerprint. No runtime cache addresses.
        serde_json::to_writer(
            &mut writer,
            &(
                "rspice-xspice-instance-v1",
                &self.name,
                self.model_name(),
                ports,
                &self.connections,
                &self.output_branches,
                vector_branches,
                num_nodes,
                matrix_size,
                self.mixed_input_thresholds_bound,
                self.model.can_skip_unchanged_event_inputs(),
            ),
        )
        .map_err(|error| error.to_string())?;
        Ok(*writer.0.finalize().as_bytes())
    }

    fn require_runtime_checkpoint_support(&self) -> Result<(), String> {
        if !self.initialized {
            return Err("XSPICE runtime checkpoint requires an initialized model".into());
        }
        let support = catch_unwind(AssertUnwindSafe(|| {
            self.model.checkpoint_support(&self.context)
        }))
        .map_err(|panic| {
            self.model_panic_error("checkpoint capability", panic)
                .to_string()
        })?;
        match support {
            XspiceCheckpointSupport::Unsupported { reason } => Err(format!(
                "{}({}): {}",
                self.name,
                self.model_name(),
                sanitize_checkpoint_text(&reason)
            )),
            XspiceCheckpointSupport::Stateless
                if self.context.has_owned_runtime_checkpoint_state() =>
            {
                Err(
                    "XSPICE model declared stateless checkpoint support but owns context history"
                        .into(),
                )
            }
            _ => Ok(()),
        }
    }

    fn validate_runtime_signature(&mut self, accepted: f64) -> Result<(), String> {
        if let Some(signature) = &self.last_event_input_signature {
            if signature.iter().any(|entry| {
                entry
                    .event_time
                    .is_some_and(|t| !t.is_finite() || t < 0.0 || t > accepted)
            }) {
                return Err("invalid XSPICE checkpoint input-signature time".into());
            }
        }
        if self.should_track_event_input_signature(AnalysisType::Transient) {
            if self.event_inputs_dirty || self.last_event_input_signature.is_none() {
                return Err("XSPICE checkpoint has an unfinished event-input dispatch".into());
            }
            self.refresh_event_input_signature();
            if self.last_event_input_signature.as_deref()
                != Some(self.event_input_signature_scratch.as_slice())
            {
                return Err(
                    "XSPICE checkpoint signature differs from its context observations".into(),
                );
            }
        } else if self.last_event_input_signature.is_some() || !self.event_inputs_dirty {
            return Err("XSPICE checkpoint has dispatch state for a non-skipping model".into());
        }
        self.event_input_signature_scratch.clear();
        Ok(())
    }

    pub(crate) fn runtime_checkpoint(
        &self,
        accepted: f64,
        matrix_size: usize,
        limits: InstanceCheckpointLimits,
    ) -> Result<InstanceRuntimeCheckpoint, String> {
        self.require_runtime_checkpoint_support()?;
        if self.solution_num_nodes > matrix_size {
            return Err("XSPICE checkpoint MNA dimensions differ".into());
        }
        let context_limits = remaining_budget(
            self.last_event_input_signature.as_ref().map_or(0, Vec::len),
            limits,
        )?;
        let ports = ContextPortLayout::from_ports(&self.ports, &self.connections)?;
        let context = ContextRuntimeCheckpoint::capture(
            &self.context,
            &ports,
            accepted,
            matrix_size,
            context_limits,
        )?;
        // Validate accepted dispatch metadata against the context without
        // changing the live model, running its body or publishing an event.
        let mut probe = self.clone();
        probe.validate_runtime_signature(accepted)?;
        Ok(InstanceRuntimeCheckpoint {
            version: 1,
            identity: self.runtime_checkpoint_identity(self.solution_num_nodes, matrix_size)?,
            context,
            signature: self
                .last_event_input_signature
                .as_ref()
                .map(|v| v.iter().map(Into::into).collect()),
            dirty: self.event_inputs_dirty,
        })
    }

    /// Rebuild MNA binding caches on a temporary initialized instance. The
    /// receiver must already carry its circuit's resolved parameters and loads.
    /// Validate all circuit input observations before installing this result.
    pub(crate) fn restored_runtime_checkpoint(
        &self,
        image: &InstanceRuntimeCheckpoint,
        accepted: f64,
        num_nodes: usize,
        matrix_size: usize,
        limits: InstanceCheckpointLimits,
    ) -> Result<Self, String> {
        self.require_runtime_checkpoint_support()?;
        if num_nodes > matrix_size
            || image.version != 1
            || image.identity != self.runtime_checkpoint_identity(num_nodes, matrix_size)?
        {
            return Err("XSPICE checkpoint instance identity or MNA dimensions differ".into());
        }
        let context_limits =
            remaining_budget(image.signature.as_ref().map_or(0, Vec::len), limits)?;
        let ports = ContextPortLayout::from_ports(&self.ports, &self.connections)?;
        let mut restored = self.clone();
        restored.solution_num_nodes = num_nodes;
        restored.port_context_solution_num_nodes = None;
        restored.refresh_port_context_bindings();
        restored.context = image.context.restore(
            &restored.context,
            &ports,
            accepted,
            matrix_size,
            context_limits,
        )?;
        restored.last_event_input_signature = image
            .signature
            .as_ref()
            .map(|v| v.iter().map(Into::into).collect());
        restored.event_inputs_dirty = image.dirty;
        restored.validate_runtime_signature(accepted)?;
        Ok(restored)
    }
}

impl XspiceInstance {
    /// Check cached event observations against the restored circuit without
    /// invoking a model or changing its dispatch signature. Shared other-driver
    /// values are returned in circuit polarity; None selects local resolution.
    pub(crate) fn validate_runtime_event_observations(
        &self,
        events: XspiceEventInputs<'_>,
        drivers: &super::super::event::XspiceDigitalDrivers,
        mut shared_other: impl FnMut(
            &super::super::event_scheduler::EventTarget,
        ) -> Result<Option<DigitalValue>, String>,
    ) -> Result<(), String> {
        use super::super::PortDirection;
        for (port, connection) in self.ports.iter().zip(&self.connections) {
            let real = matches!(
                connection,
                PortConnection::Real(_) | PortConnection::RealVector(_)
            );
            let vector = matches!(
                connection,
                PortConnection::DigitalVector(_)
                    | PortConnection::DigitalVectorMapped(_)
                    | PortConnection::RealVector(_)
            );
            let input = matches!(port.direction, PortDirection::In | PortDirection::InOut);
            let mut result: Result<(), String> = Ok(());
            for_each_event_connection_node(connection, |index, node| {
                if result.is_err() {
                    return;
                }
                result = (|| {
                    let load = if vector {
                        self.context.port_vector_total_load(&port.name, index)
                    } else {
                        self.context.port_total_load(&port.name)
                    };
                    if load.to_bits()
                        != events
                            .event_total_loads
                            .get(&node)
                            .copied()
                            .unwrap_or(0.0)
                            .to_bits()
                    {
                        return Err("XSPICE checkpoint event-node load differs from circuit".into());
                    }
                    if !input {
                        return Ok(());
                    }
                    if real {
                        let observed = if vector {
                            self.context
                                .input_real_vector_values(&port.name)
                                .and_then(|v| v.get(index).copied())
                        } else {
                            self.context.input_real(&port.name)
                        };
                        let expected = events.real_values.get(&node).copied().unwrap_or(0.0);
                        if observed.map(f64::to_bits) != Some(expected.to_bits())
                            || (!vector
                                && self.context.input_real_event_time(&port.name)
                                    != events.real_event_times.get(&node).copied())
                        {
                            return Err("XSPICE checkpoint real input differs from circuit".into());
                        }
                        return Ok(());
                    }
                    let inverted = match connection {
                        PortConnection::DigitalInverted(_) => true,
                        PortConnection::DigitalVectorMapped(v) => v[index].inverted,
                        _ => false,
                    };
                    let polarity = |v: DigitalValue| if inverted { v.invert() } else { v };
                    let expected = polarity(
                        events
                            .digital_values
                            .get(&node)
                            .copied()
                            .unwrap_or_default(),
                    );
                    let observed = if vector {
                        self.context
                            .input_digital_vector_values(&port.name)
                            .and_then(|v| v.get(index).copied())
                    } else {
                        self.context.input_digital(&port.name)
                    };
                    let time = if vector {
                        self.context
                            .input_digital_vector_event_time(&port.name, index)
                    } else {
                        self.context.input_digital_event_time(&port.name)
                    };
                    if observed != Some(expected)
                        || time != events.digital_event_times.get(&node).copied()
                    {
                        return Err("XSPICE checkpoint digital input differs from circuit".into());
                    }
                    if port.direction == PortDirection::InOut {
                        let id = (self.name.clone(), port.name.clone(), index);
                        let bank = drivers.get(&node);
                        let committed = bank
                            .and_then(|bank| bank.get(&id))
                            .copied()
                            .unwrap_or_else(DigitalValue::high_z);
                        if self.context.committed_digital_output(&port.name, index)
                            != Some(polarity(committed))
                        {
                            return Err("XSPICE checkpoint committed inout contribution differs from circuit".into());
                        }
                        let target = super::super::event_scheduler::EventTarget {
                            node_id: node,
                            instance: id.0.clone(),
                            port_name: id.1.clone(),
                            driver_index: index,
                        };
                        let other = match shared_other(&target)? {
                            Some(value) => value,
                            None => bank
                                .into_iter()
                                .flat_map(|v| v.iter())
                                .filter(|(key, _)| *key != &id)
                                .fold(DigitalValue::high_z(), |value, (_, next)| {
                                    value.resolve(next)
                                }),
                        };
                        if self
                            .context
                            .other_digital_drivers(&port.name, index)
                            .map(|(v, _)| v)
                            != Some(polarity(other))
                        {
                            return Err(
                                "XSPICE checkpoint other-driver observation differs from circuit"
                                    .into(),
                            );
                        }
                    }
                    Ok(())
                })();
            });
            result?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "checkpoint_tests.rs"]
mod tests;
