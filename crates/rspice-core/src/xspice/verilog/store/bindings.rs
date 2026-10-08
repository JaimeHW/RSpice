//! Bit-level net identities over the compiled signal view.
//! Only wires are collapsed here. Variable ports first receive the linker's
//! explicit continuous connection process, preserving HDL scheduling semantics.
use super::*;
use crate::xspice::event_scheduler::EventTarget;
use crate::xspice::{DigitalState, DigitalValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExternalBitDriverId(pub(super) usize);

impl ExternalBitDriverId {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

/// One resolved-net change. The first change of each atomic publication is
/// marked so an event participant can reconstruct simultaneous vector updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DigitalBitChange {
    pub net: usize,
    pub previous: DigitalValue,
    pub value: DigitalValue,
    pub starts_publication: bool,
}

pub(crate) use rspice_veriloga::canonical_ir::digital::DigitalNetBit as DigitalBitConnection;

#[derive(Clone)]
struct BitNet {
    members: Vec<DigitalBitConnection>,
    contributions: Vec<(usize, u32)>,
    external_nets: Vec<usize>,
}

#[derive(Clone)]
struct BitTopology {
    nets: Vec<BitNet>,
    /// Stable caller net identities mapped onto connected components.
    external_nets: Vec<usize>,
    by_signal: Vec<Vec<usize>>,
    external_sources: Vec<(usize, EventTarget)>,
    external_by_net: Vec<Vec<usize>>,
    other_driver_observers: Vec<bool>,
    observed: Vec<bool>,
    external_attached: bool,
}

#[derive(Clone)]
pub(super) struct ConnectedBits {
    topology: Arc<BitTopology>,
    resolved: Vec<DigitalValue>,
    external_values: Vec<DigitalValue>,
    other_driver_values: Vec<DigitalValue>,
    pending: Vec<Option<FourStateValue>>,
    touched: Vec<DigitalSignalId>,
}

impl ConnectedBits {
    fn fresh(topology: Arc<BitTopology>) -> Self {
        Self {
            resolved: vec![DigitalValue::high_z(); topology.nets.len()],
            external_values: vec![DigitalValue::high_z(); topology.external_sources.len()],
            other_driver_values: vec![DigitalValue::high_z(); topology.external_sources.len()],
            pending: vec![None; topology.by_signal.len()],
            touched: Vec::new(),
            topology,
        }
    }

    fn edit<'a>(
        &'a mut self,
        signal: DigitalSignalId,
        current: &FourStateValue,
    ) -> &'a mut FourStateValue {
        let slot = &mut self.pending[usize::from(signal)];
        if slot.is_none() {
            *slot = Some(current.clone());
            self.touched.push(signal);
        }
        slot.as_mut().unwrap()
    }
}

impl DigitalSignalStore {
    /// Install elaborated wire-bit groups before any process execution.
    /// Validation and allocation finish before the existing topology changes.
    pub(crate) fn connect_bits(
        &mut self,
        nets: &[Vec<DigitalBitConnection>],
    ) -> Result<(), String> {
        if self.connected.as_ref().is_some_and(|bits| {
            !bits.topology.external_nets.is_empty() || bits.topology.external_attached
        }) {
            return Err("digital bit connections are already installed".into());
        }
        // Source aliases and circuit event nets share one union graph. Caller
        // net IDs stay stable even when HDL hierarchy joins two circuit nets.
        let mut groups = nets.to_vec();
        groups.extend(
            self.plan
                .bit_aliases
                .iter()
                .map(|alias| vec![alias.left, alias.right]),
        );
        let mut parents: Vec<_> = (0..groups.len()).collect();
        fn root(parents: &mut [usize], mut index: usize) -> usize {
            while parents[index] != index {
                parents[index] = parents[parents[index]];
                index = parents[index];
            }
            index
        }
        let mut claimed = std::collections::BTreeMap::new();
        for (group, members) in groups.iter().enumerate() {
            if members.is_empty() {
                return Err(format!("digital event net {group} has no endpoints"));
            }
            for &member in members {
                let signal = usize::from(member.signal);
                if signal >= self.values.len()
                    || self.kinds[signal].is_real()
                    || self.variables[signal]
                    || member.bit >= self.widths[signal]
                {
                    return Err(format!(
                        "digital event net {group} requires a declared wire bit, got signal {signal} bit {}",
                        member.bit
                    ));
                }
                if let Some(&previous) = claimed.get(&member) {
                    let left = root(&mut parents, previous);
                    let right = root(&mut parents, group);
                    parents[left.max(right)] = left.min(right);
                } else {
                    claimed.insert(member, group);
                }
            }
        }
        let mut topology = BitTopology {
            nets: Vec::new(),
            external_nets: Vec::with_capacity(nets.len()),
            by_signal: vec![Vec::new(); self.values.len()],
            external_sources: Vec::new(),
            external_by_net: Vec::new(),
            other_driver_observers: Vec::new(),
            observed: vec![false; nets.len()],
            external_attached: false,
        };
        let mut components = std::collections::BTreeMap::new();
        for (group, members) in groups.into_iter().enumerate() {
            let representative = root(&mut parents, group);
            let net = *components.entry(representative).or_insert_with(|| {
                let index = topology.nets.len();
                topology.nets.push(BitNet {
                    members: Vec::new(),
                    contributions: Vec::new(),
                    external_nets: Vec::new(),
                });
                index
            });
            topology.nets[net].members.extend(members);
            if group < nets.len() {
                topology.external_nets.push(net);
                topology.nets[net].external_nets.push(group);
            }
        }
        for (net_index, net) in topology.nets.iter_mut().enumerate() {
            net.members.sort_unstable();
            net.members.dedup();
            let mut contributions = BTreeSet::new();
            for member in &net.members {
                let signal = usize::from(member.signal);
                if topology.by_signal[signal].last() != Some(&net_index) {
                    topology.by_signal[signal].push(net_index);
                }
                let span = self.spans[signal];
                for slot in span.start..span.start + span.count {
                    if let Ok(offset) =
                        u32::try_from(i64::from(member.bit) - self.contributions[slot].low)
                    {
                        contributions.insert((slot, offset));
                    }
                }
            }
            net.contributions = contributions.into_iter().collect();
        }
        topology.external_by_net = vec![Vec::new(); topology.nets.len()];
        self.connected = Some(ConnectedBits::fresh(Arc::new(topology)));
        Ok(())
    }

    pub(crate) fn inherit_bit_connections(&mut self, source: &Self) {
        self.connected = source
            .connected
            .as_ref()
            .map(|connected| ConnectedBits::fresh(Arc::clone(&connected.topology)));
        self.external_reals = source.external_reals.as_ref().map(ExternalReals::fresh);
    }

    pub(crate) fn connected_value(&self, net: usize) -> Option<DigitalValue> {
        let bits = self.connected.as_ref()?;
        bits.resolved
            .get(*bits.topology.external_nets.get(net)?)
            .copied()
    }

    /// Validate the complete external topology before assigning any identity.
    pub(crate) fn attach_external_bits(
        &mut self,
        observed: &[usize],
        drivers: &[(usize, EventTarget)],
    ) -> Result<Vec<ExternalBitDriverId>, String> {
        let connected = self.connected.as_mut().ok_or("no connected bit topology")?;
        if connected.topology.external_attached {
            return Err("external bit participants are already attached".into());
        }
        let count = connected.topology.external_nets.len();
        if let Some(net) = observed
            .iter()
            .copied()
            .chain(drivers.iter().map(|(net, _)| *net))
            .find(|net| *net >= count)
        {
            return Err(format!(
                "external participant refers to unknown event net {net}"
            ));
        }
        let mut identities = std::collections::HashSet::new();
        for (_, target) in drivers {
            if !identities.insert(target.clone()) {
                return Err(format!(
                    "external output driver {}.{}[{}] is declared twice",
                    target.instance, target.port_name, target.driver_index
                ));
            }
        }
        let topology = Arc::make_mut(&mut connected.topology);
        topology.external_attached = true;
        topology.external_sources.extend_from_slice(drivers);
        topology.other_driver_observers.resize(drivers.len(), false);
        for &net in observed {
            topology.observed[net] = true;
        }
        for (index, (net, _)) in drivers.iter().enumerate() {
            topology.external_by_net[topology.external_nets[*net]].push(index);
            topology.observed[*net] = true;
        }
        let sources = &topology.external_sources;
        for slots in &mut topology.external_by_net {
            slots.sort_unstable_by(|left, right| sources[*left].1.cmp(&sources[*right].1));
        }
        connected
            .external_values
            .resize(drivers.len(), DigitalValue::high_z());
        connected
            .other_driver_values
            .resize(drivers.len(), DigitalValue::high_z());
        Ok((0..drivers.len()).map(ExternalBitDriverId).collect())
    }

    /// Inout receivers observe the net with their own contribution excluded.
    /// This observation can change while the ordinary resolved bit stays still.
    pub(crate) fn observe_other_drivers(
        &mut self,
        drivers: &[ExternalBitDriverId],
    ) -> Result<(), String> {
        let connected = self.connected.as_mut().ok_or("no connected bit topology")?;
        if let Some(driver) = drivers
            .iter()
            .find(|driver| driver.0 >= connected.external_values.len())
        {
            return Err(format!("unknown external bit observer {}", driver.0));
        }
        let topology = Arc::make_mut(&mut connected.topology);
        for driver in drivers {
            topology.other_driver_observers[driver.0] = true;
        }
        Ok(())
    }

    pub(crate) fn other_driver_value(&self, driver: ExternalBitDriverId) -> Option<DigitalValue> {
        let connected = self.connected.as_ref()?;
        connected
            .topology
            .other_driver_observers
            .get(driver.0)
            .copied()
            .filter(|observed| *observed)?;
        connected.other_driver_values.get(driver.0).copied()
    }

    pub(crate) fn has_external_participants(&self) -> bool {
        self.connected
            .as_ref()
            .is_some_and(|bits| bits.topology.external_attached)
            || self.external_reals.is_some()
    }

    pub(crate) fn external_sources(&self) -> &[(usize, EventTarget)] {
        self.connected
            .as_ref()
            .map_or(&[], |bits| bits.topology.external_sources.as_slice())
    }

    pub(crate) fn remap_external_nodes(&mut self, remap: impl Fn(usize) -> usize) {
        if let Some(bits) = &mut self.connected {
            for (_, target) in &mut Arc::make_mut(&mut bits.topology).external_sources {
                target.node_id = remap(target.node_id);
            }
        }
        if let Some(reals) = &mut self.external_reals {
            for (_, target) in &mut Arc::make_mut(&mut reals.topology).sources {
                target.node_id = remap(target.node_id);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn take_external_bit_changes(&mut self) -> Vec<DigitalBitChange> {
        self.take_external_changes()
            .into_iter()
            .filter_map(|change| match change {
                ExternalNetChange::Bits(change) => Some(change),
                ExternalNetChange::Real { .. } | ExternalNetChange::DriverInput { .. } => None,
            })
            .collect()
    }

    pub(crate) fn check_external_drives(
        &self,
        drives: &[(ExternalBitDriverId, DigitalValue)],
    ) -> Result<(), String> {
        let count = self
            .connected
            .as_ref()
            .map_or(0, |bits| bits.external_values.len());
        for (driver, _) in drives {
            if driver.0 >= count {
                return Err(format!("unknown external bit driver {}", driver.0));
            }
        }
        Ok(())
    }

    /// The caller has checked every identity. Driver values enter as one bank;
    /// no resolved alias is ever copied back into a contribution slot.
    pub(crate) fn publish_external_drives(
        &mut self,
        drives: &[(ExternalBitDriverId, DigitalValue)],
    ) {
        if drives.is_empty() {
            return;
        }
        let mut connected = self.connected.take().expect("validated external drives");
        let mut nets = BTreeSet::new();
        for (driver, value) in drives {
            connected.external_values[driver.0] = *value;
            nets.insert(
                connected.topology.external_nets[connected.topology.external_sources[driver.0].0],
            );
        }
        let publication_start = self.external_changes.len();
        for net in nets {
            self.resolve_connected_net(&mut connected, net, publication_start);
        }
        self.flush_connected_values(&mut connected);
        self.connected = Some(connected);
    }

    /// Project one driver's update into all aliases before observing any
    /// expression/event subscription. Aliases never become extra drivers.
    pub(super) fn publish_connected(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        let Some(mut connected) = self.connected.take() else {
            self.publish(signal, value);
            return;
        };
        if connected.topology.by_signal[usize::from(signal)].is_empty() {
            self.connected = Some(connected);
            self.publish(signal, value);
            return;
        }
        connected
            .edit(signal, &self.values[usize::from(signal)])
            .clone_from(&value);
        let topology = Arc::clone(&connected.topology);
        let publication_start = self.external_changes.len();
        for &net_index in &topology.by_signal[usize::from(signal)] {
            self.resolve_connected_net(&mut connected, net_index, publication_start);
        }
        self.flush_connected_values(&mut connected);
        self.connected = Some(connected);
    }

    fn resolve_connected_net(
        &mut self,
        connected: &mut ConnectedBits,
        net_index: usize,
        publication_start: usize,
    ) {
        let topology = Arc::clone(&connected.topology);
        let net = &topology.nets[net_index];
        let hdl = net.contributions.iter().fold(
            FourStateBit::HighImpedance,
            |resolved, &(slot, offset)| {
                let ContributionValue::FourState(Some(value)) = &self.contributions[slot].value
                else {
                    return resolved;
                };
                if offset >= value.width() {
                    return resolved;
                }
                resolve_bit(resolved, value.bit(offset))
            },
        );
        let hdl_value = match hdl {
            FourStateBit::Zero => Some(DigitalValue::zero()),
            FourStateBit::One => Some(DigitalValue::one()),
            FourStateBit::Unknown => Some(DigitalValue::unknown()),
            FourStateBit::HighImpedance => None,
        };
        let resolve_external = |excluded: Option<usize>| {
            let mut resolved = hdl_value;
            for &slot in &topology.external_by_net[net_index] {
                if excluded == Some(slot) {
                    continue;
                }
                let value = connected.external_values[slot];
                if value.state != DigitalState::HighZ {
                    resolved = Some(resolved.map_or(value, |existing| existing.resolve(&value)));
                }
            }
            resolved.unwrap_or_else(DigitalValue::high_z)
        };
        let resolved = resolve_external(None);
        for &slot in &topology.external_by_net[net_index] {
            if topology.other_driver_observers[slot] {
                let value = resolve_external(Some(slot));
                let previous = std::mem::replace(&mut connected.other_driver_values[slot], value);
                if previous != value {
                    self.external_changes.push(ExternalNetChange::DriverInput {
                        driver: ExternalBitDriverId(slot),
                        previous,
                        value,
                        starts_publication: self.external_changes.len() == publication_start,
                    });
                }
            }
        }
        let previous = std::mem::replace(&mut connected.resolved[net_index], resolved);
        if previous != resolved {
            for &net in &net.external_nets {
                self.trace_resolved_bit(net, resolved);
                if topology.observed[net] {
                    self.external_changes
                        .push(ExternalNetChange::Bits(DigitalBitChange {
                            net,
                            previous,
                            value: resolved,
                            starts_publication: self.external_changes.len() == publication_start,
                        }));
                }
            }
        }
        for member in &net.members {
            connected
                .edit(member.signal, &self.values[usize::from(member.signal)])
                .set_bit(member.bit, hdl_bit(resolved));
        }
    }

    fn flush_connected_values(&mut self, connected: &mut ConnectedBits) {
        // A whole-vector write changes every connected bit as one publication.
        // Install the complete bank before computed sensitivity can read it.
        for &member in &connected.touched {
            let index = usize::from(member);
            let value = connected.pending[index].take().unwrap();
            if self.values[index] != value {
                connected.pending[index] = Some(std::mem::replace(&mut self.values[index], value));
            }
        }
        for &member in &connected.touched {
            if let Some(previous) = connected.pending[usize::from(member)].take() {
                self.record_bit_change(member, previous, self.values[usize::from(member)].clone());
            }
        }
        connected.touched.clear();
    }

    pub(super) fn force_changes_connected_bit(
        &self,
        signal: DigitalSignalId,
        value: &FourStateValue,
    ) -> bool {
        let Some(connected) = &self.connected else {
            return false;
        };
        connected.topology.by_signal[usize::from(signal)]
            .iter()
            .any(|&net| {
                connected.topology.nets[net].members.iter().any(|member| {
                    member.signal == signal
                        && value.bit(member.bit) != self.values[usize::from(signal)].bit(member.bit)
                })
            })
    }
}

fn hdl_bit(value: DigitalValue) -> FourStateBit {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => FourStateBit::Zero,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => FourStateBit::One,
        DigitalState::HighZ => FourStateBit::HighImpedance,
        DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => {
            FourStateBit::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_alias_connections_join_external_groups_without_renumbering_or_feedback() {
        let source = "module leaf(io); inout io; wire io; endmodule module top(bus); inout [3:2] bus; wire [3:2] bus; leaf child(bus[2]); endmodule";
        let artifact = rspice_veriloga::VerilogACompiler::new(Default::default())
            .compile_canonical_ir_module(source, Some("top"))
            .unwrap();
        let plan = Arc::new(artifact.digital);
        let signal = |name: &str| {
            plan.signals
                .iter()
                .find(|signal| signal.name == name)
                .unwrap()
                .id
        };
        let bus = signal("bus");
        let child = signal("child.io");
        let mut store = DigitalSignalStore::from_plan(plan);
        store
            .connect_bits(&[
                vec![DigitalBitConnection {
                    signal: bus,
                    bit: 0,
                }],
                vec![DigitalBitConnection {
                    signal: child,
                    bit: 0,
                }],
            ])
            .unwrap();
        let target = |node_id, instance: &str| EventTarget {
            node_id,
            instance: instance.into(),
            port_name: "io".into(),
            driver_index: 0,
        };
        let drivers = store
            .attach_external_bits(&[0, 1], &[(0, target(10, "a")), (1, target(20, "b"))])
            .unwrap();
        store.observe_other_drivers(&drivers).unwrap();
        for (left, right, expected) in [
            (
                DigitalValue::one(),
                DigitalValue::high_z(),
                DigitalValue::one(),
            ),
            (
                DigitalValue::one(),
                DigitalValue::zero(),
                DigitalValue::unknown(),
            ),
            (
                DigitalValue::high_z(),
                DigitalValue::zero(),
                DigitalValue::zero(),
            ),
            (
                DigitalValue::high_z(),
                DigitalValue::high_z(),
                DigitalValue::high_z(),
            ),
        ] {
            store.publish_external_drives(&[(drivers[0], left), (drivers[1], right)]);
            for net in [0, 1] {
                assert_eq!(store.connected_value(net), Some(expected));
            }
            assert_eq!(store.other_driver_value(drivers[0]), Some(right));
            assert_eq!(store.other_driver_value(drivers[1]), Some(left));
            let changes = store.take_external_bit_changes();
            assert_eq!(
                changes.iter().map(|change| change.net).collect::<Vec<_>>(),
                [0, 1]
            );
            assert_eq!(store.values[usize::from(bus)].bit(0), hdl_bit(expected));
            assert_eq!(
                store.values[usize::from(bus)].bit(1),
                FourStateBit::HighImpedance
            );
            assert_eq!(store.values[usize::from(child)].bit(0), hdl_bit(expected));
        }
        let mut fresh = DigitalSignalStore::from_plan(Arc::clone(&store.plan));
        fresh.inherit_bit_connections(&store);
        assert_eq!(fresh.external_sources(), store.external_sources());
        fresh.publish_external_drives(&[(drivers[1], DigitalValue::one())]);
        assert_eq!(fresh.connected_value(0), Some(DigitalValue::one()));
        assert_eq!(fresh.connected_value(1), Some(DigitalValue::one()));
    }
}
