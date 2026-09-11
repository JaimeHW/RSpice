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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DigitalBitConnection {
    pub signal: DigitalSignalId,
    pub bit: u32,
}

#[derive(Clone)]
struct BitNet {
    members: Vec<DigitalBitConnection>,
    contributions: Vec<(usize, u32)>,
}

#[derive(Clone)]
struct BitTopology {
    nets: Vec<BitNet>,
    by_signal: Vec<Vec<usize>>,
    external_sources: Vec<(usize, EventTarget)>,
    external_by_net: Vec<Vec<usize>>,
    observed: Vec<bool>,
    external_attached: bool,
}

#[derive(Clone)]
pub(super) struct ConnectedBits {
    topology: Arc<BitTopology>,
    resolved: Vec<DigitalValue>,
    external_values: Vec<DigitalValue>,
    changes: Vec<DigitalBitChange>,
    pending: Vec<Option<FourStateValue>>,
    touched: Vec<DigitalSignalId>,
}

impl ConnectedBits {
    fn fresh(topology: Arc<BitTopology>) -> Self {
        Self {
            resolved: vec![DigitalValue::high_z(); topology.nets.len()],
            external_values: vec![DigitalValue::high_z(); topology.external_sources.len()],
            changes: Vec::new(),
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
        if self.connected.is_some() {
            return Err("digital bit connections are already installed".into());
        }
        let mut topology = BitTopology {
            nets: Vec::new(),
            by_signal: vec![Vec::new(); self.values.len()],
            external_sources: Vec::new(),
            external_by_net: vec![Vec::new(); nets.len()],
            observed: vec![false; nets.len()],
            external_attached: false,
        };
        let mut claimed = BTreeSet::new();
        for (net_index, offered) in nets.iter().enumerate() {
            let mut members = offered.clone();
            members.sort_unstable();
            members.dedup();
            if members.is_empty() {
                return Err(format!("digital event net {net_index} has no endpoints"));
            }
            let mut contributions = BTreeSet::new();
            for member in &members {
                let signal = usize::from(member.signal);
                if signal >= self.values.len()
                    || self.kinds[signal].is_real()
                    || self.variables[signal]
                    || member.bit >= self.widths[signal]
                {
                    return Err(format!(
                        "digital event net {net_index} requires a declared wire bit, got signal {signal} bit {}",
                        member.bit
                    ));
                }
                if !claimed.insert(*member) {
                    return Err(format!(
                        "signal {signal} bit {} belongs to two distinct event nets",
                        member.bit
                    ));
                }
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
            topology.nets.push(BitNet {
                members,
                contributions: contributions.into_iter().collect(),
            });
        }
        self.connected = Some(ConnectedBits::fresh(Arc::new(topology)));
        Ok(())
    }

    pub(crate) fn inherit_bit_connections(&mut self, source: &Self) {
        self.connected = source
            .connected
            .as_ref()
            .map(|connected| ConnectedBits::fresh(Arc::clone(&connected.topology)));
    }

    pub(crate) fn connected_bit(&self, net: usize) -> Option<FourStateBit> {
        self.connected_value(net).map(hdl_bit)
    }

    pub(crate) fn connected_value(&self, net: usize) -> Option<DigitalValue> {
        self.connected.as_ref()?.resolved.get(net).copied()
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
        let count = connected.topology.nets.len();
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
        for &net in observed {
            topology.observed[net] = true;
        }
        for (index, (net, _)) in drivers.iter().enumerate() {
            topology.external_by_net[*net].push(index);
            topology.observed[*net] = true;
        }
        let sources = &topology.external_sources;
        for slots in &mut topology.external_by_net {
            slots.sort_unstable_by(|left, right| sources[*left].1.cmp(&sources[*right].1));
        }
        connected
            .external_values
            .resize(drivers.len(), DigitalValue::high_z());
        Ok((0..drivers.len()).map(ExternalBitDriverId).collect())
    }

    pub(crate) fn has_external_participants(&self) -> bool {
        self.connected
            .as_ref()
            .is_some_and(|bits| bits.topology.external_attached)
    }

    pub(crate) fn external_sources(&self) -> &[(usize, EventTarget)] {
        self.connected
            .as_ref()
            .map_or(&[], |bits| bits.topology.external_sources.as_slice())
    }

    pub(crate) fn take_external_bit_changes(&mut self) -> Vec<DigitalBitChange> {
        self.connected
            .as_mut()
            .map_or_else(Vec::new, |bits| std::mem::take(&mut bits.changes))
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
            nets.insert(connected.topology.external_sources[driver.0].0);
        }
        let publication_start = connected.changes.len();
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
        let publication_start = connected.changes.len();
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
        let mut resolved = match hdl {
            FourStateBit::Zero => Some(DigitalValue::zero()),
            FourStateBit::One => Some(DigitalValue::one()),
            FourStateBit::Unknown => Some(DigitalValue::unknown()),
            FourStateBit::HighImpedance => None,
        };
        for &slot in &topology.external_by_net[net_index] {
            let value = connected.external_values[slot];
            if value.state != DigitalState::HighZ {
                resolved = Some(resolved.map_or(value, |existing| existing.resolve(&value)));
            }
        }
        let resolved = resolved.unwrap_or_else(DigitalValue::high_z);
        let previous = std::mem::replace(&mut connected.resolved[net_index], resolved);
        if previous != resolved && topology.observed[net_index] {
            connected.changes.push(DigitalBitChange {
                net: net_index,
                previous,
                value: resolved,
                starts_publication: connected.changes.len() == publication_start,
            });
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
