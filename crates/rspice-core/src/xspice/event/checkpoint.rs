//! XSPICE's circuit event state, separate from each code-model context. Restore
//! returns replacements; the enclosing circuit installs every participant once.
use super::super::event_scheduler::{SchedulerCheckpoint, SchedulerCheckpointLimits};
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub(crate) struct EventCheckpointLimits {
    pub scheduler: SchedulerCheckpointLimits,
    pub max_nodes: usize,
    pub max_drivers: usize,
    pub max_name_bytes: usize,
}
impl Default for EventCheckpointLimits {
    fn default() -> Self {
        Self {
            scheduler: Default::default(),
            max_nodes: 1_048_576,
            max_drivers: 4_194_304,
            max_name_bytes: 64 * 1024 * 1024,
        }
    }
}

/// Built from the receiving circuit's elaborated output ports and its restored
/// shared HDL owner. Saved bytes cannot declare new producers or node domains.
pub(crate) struct EventCheckpointTopology {
    drivers: BTreeMap<EventTarget, bool>,
    shared: BTreeMap<(NodeId, bool), ValueImage>,
    shared_drivers: BTreeMap<EventTarget, ValueImage>,
}
impl EventCheckpointTopology {
    pub(crate) fn new(
        drivers: impl IntoIterator<Item = (EventTarget, bool)>,
        shared: impl IntoIterator<Item = (NodeId, EventValue)>,
        shared_drivers: impl IntoIterator<Item = (EventTarget, EventValue)>,
    ) -> Result<Self, String> {
        let mut topology = Self {
            drivers: BTreeMap::new(),
            shared: BTreeMap::new(),
            shared_drivers: BTreeMap::new(),
        };
        for (driver, real) in drivers {
            if topology.drivers.insert(driver, real).is_some() {
                return Err("duplicate elaborated XSPICE output identity".into());
            }
        }
        for (node, value) in shared {
            if topology
                .shared
                .insert((node, matches!(value, EventValue::Real(_))), value.into())
                .is_some()
            {
                return Err("duplicate shared XSPICE node domain".into());
            }
        }
        for (target, value) in shared_drivers {
            let value = ValueImage::from(value);
            if topology.drivers.get(&target) != Some(&value.real())
                || !topology
                    .shared
                    .contains_key(&(target.node_id, value.real()))
                || topology.shared_drivers.insert(target, value).is_some()
            {
                return Err(
                    "HDL owner has an unknown, duplicated or incompatible XSPICE driver".into(),
                );
            }
        }
        if topology.drivers.iter().any(|(target, real)| {
            topology.shared.contains_key(&(target.node_id, *real))
                && !topology.shared_drivers.contains_key(target)
        }) {
            return Err("HDL owner is missing a shared XSPICE driver".into());
        }
        Ok(topology)
    }

    fn identity(&self, retention: &[bool]) -> [u8; 32] {
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice-xspice-event-topology-v1\0");
        hash.update(&(self.drivers.len() as u64).to_le_bytes());
        for (target, real) in &self.drivers {
            hash.update(&(target.node_id as u64).to_le_bytes());
            hash.update(&(target.driver_index as u64).to_le_bytes());
            for name in [&target.instance, &target.port_name] {
                hash.update(&(name.len() as u64).to_le_bytes());
                hash.update(name.as_bytes());
            }
            hash.update(&[u8::from(*real)]);
        }
        hash.update(&(self.shared.len() as u64).to_le_bytes());
        for &(node, real) in self.shared.keys() {
            hash.update(&(node as u64).to_le_bytes());
            hash.update(&[u8::from(real)]);
        }
        hash.update(&(retention.len() as u64).to_le_bytes());
        for retained in retention {
            hash.update(&[u8::from(*retained)]);
        }
        *hash.finalize().as_bytes()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct XspiceEventCheckpoint {
    version: u32,
    topology: [u8; 32],
    time: u64,
    scheduler: SchedulerCheckpoint,
    nodes: Vec<NodeImage>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeImage {
    node: NodeId,
    value: ValueImage,
    time: Option<u64>,
    drivers: Option<Vec<(XspiceDriverId, ValueImage)>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum ValueImage {
    Digital(DigitalValue),
    Real(u64),
}
impl ValueImage {
    fn real(self) -> bool {
        matches!(self, Self::Real(_))
    }
}
impl From<EventValue> for ValueImage {
    fn from(value: EventValue) -> Self {
        match value {
            EventValue::Digital(v) => Self::Digital(v),
            EventValue::Real(v) => Self::Real(v.to_bits()),
        }
    }
}

fn check_budget(
    nodes: &[NodeImage],
    limits: EventCheckpointLimits,
) -> Result<SchedulerCheckpointLimits, String> {
    if nodes.len() > limits.max_nodes {
        return Err("XSPICE checkpoint node limit exceeded".into());
    }
    driver_budget(
        nodes
            .iter()
            .flat_map(|node| node.drivers.iter().flatten())
            .map(|(id, _)| id),
        limits,
    )
}
fn driver_budget<'a>(
    ids: impl Iterator<Item = &'a XspiceDriverId>,
    limits: EventCheckpointLimits,
) -> Result<SchedulerCheckpointLimits, String> {
    let mut drivers = 0usize;
    let mut names = 0usize;
    for (instance, port, _) in ids {
        drivers = drivers
            .checked_add(1)
            .filter(|n| *n <= limits.max_drivers)
            .ok_or("XSPICE checkpoint driver limit exceeded")?;
        names = names
            .checked_add(instance.len())
            .and_then(|n| n.checked_add(port.len()))
            .filter(|n| *n <= limits.max_name_bytes)
            .ok_or("XSPICE checkpoint name limit exceeded")?;
    }
    Ok(SchedulerCheckpointLimits {
        max_name_bytes: limits
            .scheduler
            .max_name_bytes
            .min(limits.max_name_bytes - names),
        ..limits.scheduler
    })
}

fn accepted_time(time: f64) -> Result<(), String> {
    if !time.is_finite() || time < 0.0 {
        return Err("invalid accepted XSPICE checkpoint time".into());
    }
    Ok(())
}

impl XspiceEventCheckpoint {
    pub(crate) fn capture(
        queue: &XspiceEventScheduler,
        values: &XspiceEventValues,
        time: f64,
        topology: &EventCheckpointTopology,
        limits: EventCheckpointLimits,
    ) -> Result<Self, String> {
        accepted_time(time)?;
        let identity = topology.identity(values.traces.checkpoint_retention()?);
        if values
            .digital_drivers
            .keys()
            .chain(values.digital_event_times.keys())
            .any(|node| !values.digital_values.contains_key(node))
            || values
                .real_drivers
                .keys()
                .chain(values.real_event_times.keys())
                .any(|node| !values.real_values.contains_key(node))
        {
            return Err("XSPICE driver/time history has no resolved node value".into());
        }
        let count = values
            .digital_values
            .len()
            .checked_add(values.real_values.len())
            .filter(|n| *n <= limits.max_nodes)
            .ok_or("XSPICE checkpoint node limit exceeded")?;
        // Check variable-size banks before cloning their keys or allocating the image.
        driver_budget(
            values
                .digital_drivers
                .values()
                .flat_map(|bank| bank.keys())
                .chain(values.real_drivers.values().flat_map(|bank| bank.keys())),
            limits,
        )?;
        let mut nodes = Vec::with_capacity(count);
        for (&node, &value) in &values.digital_values {
            nodes.push(NodeImage {
                node,
                value: ValueImage::Digital(value),
                time: values.digital_event_times.get(&node).map(|v| v.to_bits()),
                drivers: values.digital_drivers.get(&node).map(|drivers| {
                    drivers
                        .iter()
                        .map(|(key, value)| (key.clone(), ValueImage::Digital(*value)))
                        .collect()
                }),
            });
        }
        for (&node, &value) in &values.real_values {
            nodes.push(NodeImage {
                node,
                value: ValueImage::Real(value.to_bits()),
                time: values.real_event_times.get(&node).map(|v| v.to_bits()),
                drivers: values.real_drivers.get(&node).map(|drivers| {
                    drivers
                        .iter()
                        .map(|(key, value)| (key.clone(), ValueImage::Real(value.to_bits())))
                        .collect()
                }),
            });
        }
        nodes.sort_unstable_by_key(|node| (node.node, node.value.real()));
        let scheduler = queue
            .inner
            .checkpoint(check_budget(&nodes, limits)?)
            .map_err(|e| e.to_string())?;
        let image = Self {
            version: 1,
            topology: identity,
            time: time.to_bits(),
            scheduler,
            nodes,
        };
        image.validate_queue(&queue.inner, topology, time)?;
        image.validate_nodes(topology, time)?;
        Ok(image)
    }

    /// Decode into temporary values and a temporary queue. Input bytes and
    /// deserialization must be bounded by the enclosing circuit file reader.
    pub(crate) fn restore(
        &self,
        queue: &XspiceEventScheduler,
        values: &XspiceEventValues,
        time: f64,
        topology: &EventCheckpointTopology,
        limits: EventCheckpointLimits,
    ) -> Result<(SharedXspiceEventQueue, SharedXspiceEventValues), String> {
        accepted_time(time)?;
        if self.version != 1
            || self.time != time.to_bits()
            || self.topology != topology.identity(values.traces.checkpoint_retention()?)
        {
            return Err("XSPICE checkpoint schema, topology, trace policy or time differs".into());
        }
        // XSPICE target enrollment is lazy runtime state, not elaboration order.
        // Validate every saved identity against actual output ports below.
        let mut inner = EventScheduler::new(queue.inner.limits());
        inner
            .restore_checkpoint(&self.scheduler, check_budget(&self.nodes, limits)?)
            .map_err(|e| e.to_string())?;
        self.validate_queue(&inner, topology, time)?;
        self.validate_nodes(topology, time)?;
        let mut restored = XspiceEventValues {
            traces: values.traces.clone(),
            ..Default::default()
        };
        for node in &self.nodes {
            match node.value {
                ValueImage::Digital(value) => {
                    restored.digital_values.insert(node.node, value);
                    if let Some(time) = node.time {
                        restored
                            .digital_event_times
                            .insert(node.node, f64::from_bits(time));
                    }
                    if let Some(drivers) = &node.drivers {
                        restored.digital_drivers.insert(
                            node.node,
                            drivers
                                .iter()
                                .map(|(id, value)| {
                                    let ValueImage::Digital(value) = value else {
                                        unreachable!("validated driver domain")
                                    };
                                    (id.clone(), *value)
                                })
                                .collect(),
                        );
                    }
                }
                ValueImage::Real(value) => {
                    restored
                        .real_values
                        .insert(node.node, f64::from_bits(value));
                    if let Some(time) = node.time {
                        restored
                            .real_event_times
                            .insert(node.node, f64::from_bits(time));
                    }
                    if let Some(drivers) = &node.drivers {
                        restored.real_drivers.insert(
                            node.node,
                            drivers
                                .iter()
                                .map(|(id, value)| {
                                    let ValueImage::Real(value) = value else {
                                        unreachable!("validated driver domain")
                                    };
                                    (id.clone(), f64::from_bits(*value))
                                })
                                .collect(),
                        );
                    }
                }
            }
        }
        Ok((
            SharedXspiceEventQueue(Arc::new(XspiceEventScheduler { inner })),
            SharedXspiceEventValues(Arc::new(restored)),
        ))
    }

    fn validate_queue(
        &self,
        inner: &EventScheduler,
        topology: &EventCheckpointTopology,
        time: f64,
    ) -> Result<(), String> {
        if inner.current_instant().seconds() > time
            || inner.next_instant().is_some_and(|at| at.seconds() <= time)
        {
            return Err("XSPICE event queue is not settled at accepted time".into());
        }
        for target in self.scheduler.targets() {
            if !topology.drivers.contains_key(target) {
                return Err("checkpoint queue has an unknown XSPICE producer".into());
            }
        }
        for (_, region, target, value) in self.scheduler.live_events() {
            let target = &self.scheduler.targets()[usize::from(target)];
            if region != SchedulerRegion::Active
                || topology.drivers.get(target) != Some(&matches!(value, EventValue::Real(_)))
            {
                return Err("XSPICE queued event has an incompatible domain or region".into());
            }
        }
        Ok(())
    }

    fn validate_nodes(&self, topology: &EventCheckpointTopology, time: f64) -> Result<(), String> {
        let mut previous = None;
        for node in &self.nodes {
            let key = (node.node, node.value.real());
            if previous.is_some_and(|prior| prior >= key) {
                return Err("unordered or duplicated XSPICE node domain".into());
            }
            previous = Some(key);
            if node.time.is_some_and(|bits| {
                let at = f64::from_bits(bits);
                !at.is_finite() || at < 0.0 || at > time
            }) {
                return Err("invalid XSPICE node event time".into());
            }
            let mut prior = None;
            if node
                .drivers
                .as_ref()
                .is_some_and(|drivers| !drivers.is_empty())
                && node.time.is_none()
            {
                return Err("executed XSPICE drivers have no event time".into());
            }
            for (id, value) in node.drivers.iter().flatten() {
                if prior.is_some_and(|previous| previous >= id) {
                    return Err("unordered or duplicated XSPICE driver".into());
                }
                prior = Some(id);
                let target = EventTarget {
                    node_id: node.node,
                    instance: id.0.clone(),
                    port_name: id.1.clone(),
                    driver_index: id.2,
                };
                if value.real() != node.value.real()
                    || topology.drivers.get(&target) != Some(&value.real())
                {
                    return Err("XSPICE saved driver identity or domain differs".into());
                }
            }
            if let Some(shared) = topology.shared.get(&key) {
                if *shared != node.value {
                    return Err("XSPICE observation differs from the restored HDL owner".into());
                }
            } else {
                let drivers = node
                    .drivers
                    .as_ref()
                    .filter(|drivers| !drivers.is_empty())
                    .ok_or("XSPICE local node has no output drivers")?;
                let expected = match node.value {
                    ValueImage::Digital(_) => {
                        let mut values = drivers.iter().map(|(_, value)| {
                            let ValueImage::Digital(value) = value else {
                                unreachable!("validated domain")
                            };
                            *value
                        });
                        let first = values.next().expect("nonempty bank");
                        ValueImage::Digital(values.fold(first, |value, next| value.resolve(&next)))
                    }
                    ValueImage::Real(_) => ValueImage::Real(
                        drivers
                            .iter()
                            .map(|(_, value)| {
                                let ValueImage::Real(value) = value else {
                                    unreachable!("validated domain")
                                };
                                f64::from_bits(*value)
                            })
                            .sum::<f64>()
                            .to_bits(),
                    ),
                };
                if expected != node.value {
                    return Err("XSPICE resolved value differs from saved drivers".into());
                }
            }
        }
        // A resolved observation can agree even when a weaker bit driver is
        // wrong, or when real contributions cancel. Compare each original
        // producer as well, including ones absent from the XSPICE bank.
        for (target, expected) in &topology.shared_drivers {
            let key = (target.node_id, expected.real());
            let saved = self
                .nodes
                .binary_search_by_key(&key, |n| (n.node, n.value.real()))
                .ok()
                .and_then(|index| self.nodes[index].drivers.as_ref())
                .and_then(|bank| {
                    bank.binary_search_by(|(id, _)| {
                        (&id.0, &id.1, id.2).cmp(&(
                            &target.instance,
                            &target.port_name,
                            target.driver_index,
                        ))
                    })
                    .ok()
                    .map(|index| bank[index].1)
                });
            // HDL enrollment initializes untouched digital/real sources to
            // high-Z/+0.0. An absent XSPICE entry means that source has never
            // executed. Do not canonicalize an explicit -0.0 or NaN payload.
            let initial = if expected.real() {
                ValueImage::Real(0.0_f64.to_bits())
            } else {
                ValueImage::Digital(DigitalValue::high_z())
            };
            if saved.unwrap_or(initial) != *expected {
                return Err("XSPICE contribution differs from its restored HDL driver".into());
            }
        }
        Ok(())
    }
}
