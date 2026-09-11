//! Bit-level net identities over the compiled signal view.
//! Only wires are collapsed here. Variable ports first receive the linker's
//! explicit continuous connection process, preserving HDL scheduling semantics.
use super::*;

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

struct BitTopology {
    nets: Vec<BitNet>,
    by_signal: Vec<Vec<usize>>,
}

#[derive(Clone)]
pub(super) struct ConnectedBits {
    topology: Arc<BitTopology>,
    resolved: Vec<FourStateBit>,
    pending: Vec<Option<FourStateValue>>,
    touched: Vec<DigitalSignalId>,
}

impl ConnectedBits {
    fn fresh(topology: Arc<BitTopology>) -> Self {
        Self {
            resolved: vec![FourStateBit::HighImpedance; topology.nets.len()],
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
        self.connected.as_ref()?.resolved.get(net).copied()
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
        for &net_index in &topology.by_signal[usize::from(signal)] {
            let net = &topology.nets[net_index];
            let resolved = net.contributions.iter().fold(
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
            connected.resolved[net_index] = resolved;
            for member in &net.members {
                connected
                    .edit(member.signal, &self.values[usize::from(member.signal)])
                    .set_bit(member.bit, resolved);
            }
        }
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
        self.connected = Some(connected);
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
