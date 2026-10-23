//! Bidirectional switch islands (IEEE 1364-2005 sections 7.6 and 7.12).
//!
//! `tran`, `rtran`, `tranif0`/`tranif1` and their `r` forms join net bits.
//! Each switch is two gated drivers, one per direction, and what either
//! contributes is what the switch passes from the other terminal. The bits
//! the switches join make an *island*, resolved as a whole whenever one of
//! its nets changes: every bit starts at the resolution of its other drivers,
//! and every switch then passes its source bit's strength, reduced, until
//! nothing changes. Starting over each time is what keeps a value from holding
//! itself up through the loop the two directions of one switch make: a bit is
//! only ever driven by what some other driver in the island drives.
//!
//! A switch under an unknown control passes the range from nothing to what it
//! would pass on, the ambiguous strength of section 7.6; off, it passes
//! nothing. Supply passes as strong, and a resistive switch reduces by table
//! 7-14, exactly as a MOS switch does (`strength::mos`).

use super::strength::{self, StrengthBit};
use super::{ContributionValue, DigitalSignalStore};
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use std::collections::HashMap;
use std::sync::Arc;

/// One direction of a bidirectional switch: what contribution `slot` passes
/// onto its island's node `target` from node `source`.
#[derive(Debug, Clone, Copy)]
struct Bridge {
    slot: usize,
    resistive: bool,
    target: usize,
    source: usize,
}

/// Net bits joined by switches, and the switch directions joining them.
#[derive(Debug, Default)]
struct Island {
    /// `(signal, storage position)` per node.
    nodes: Vec<(DigitalSignalId, u32)>,
    bridges: Vec<usize>,
    /// The distinct signals of `nodes`, to re-resolve once each.
    signals: Vec<DigitalSignalId>,
}

/// Every island of a plan, fixed by it.
#[derive(Debug, Default)]
pub(super) struct Islands {
    bridges: Vec<Bridge>,
    islands: Vec<Island>,
    by_signal: HashMap<DigitalSignalId, Vec<usize>>,
    /// Per island, whether a `trireg` bit is in it (`store/charge.rs`).
    charged: Vec<bool>,
}

/// One switch direction as the plan gives it, before islands are found.
pub(super) struct BridgeEnd {
    pub(super) slot: usize,
    pub(super) resistive: bool,
    pub(super) target: (DigitalSignalId, u32),
    pub(super) source: (DigitalSignalId, u32),
}

impl Islands {
    /// Join the net bits the switch directions name into islands; bridge `i`
    /// is `ends[i]`.
    pub(super) fn new(ends: &[BridgeEnd]) -> Self {
        let mut keys: HashMap<(DigitalSignalId, u32), usize> = HashMap::new();
        let mut parent: Vec<usize> = Vec::new();
        let mut node = |key: (DigitalSignalId, u32), parent: &mut Vec<usize>| {
            *keys.entry(key).or_insert_with(|| {
                parent.push(parent.len());
                parent.len() - 1
            })
        };
        fn root(parent: &mut [usize], mut at: usize) -> usize {
            while parent[at] != at {
                parent[at] = parent[parent[at]];
                at = parent[at];
            }
            at
        }
        let mut pairs = Vec::with_capacity(ends.len());
        for end in ends {
            let target = node(end.target, &mut parent);
            let source = node(end.source, &mut parent);
            let (a, b) = (root(&mut parent, target), root(&mut parent, source));
            parent[a] = b;
            pairs.push((target, source));
        }
        let mut islands = Self::default();
        let mut island_of_root: HashMap<usize, usize> = HashMap::new();
        let mut local: Vec<Option<(usize, usize)>> = vec![None; parent.len()];
        let mut ordered: Vec<_> = keys.into_iter().collect();
        ordered.sort_unstable_by_key(|&(_, id)| id);
        for ((signal, bit), id) in ordered {
            let top = root(&mut parent, id);
            let island = *island_of_root.entry(top).or_insert_with(|| {
                islands.islands.push(Island::default());
                islands.islands.len() - 1
            });
            let entry = &mut islands.islands[island];
            local[id] = Some((island, entry.nodes.len()));
            entry.nodes.push((signal, bit));
            if !entry.signals.contains(&signal) {
                entry.signals.push(signal);
                islands.by_signal.entry(signal).or_default().push(island);
            }
        }
        for (end, (target, source)) in ends.iter().zip(pairs) {
            let (Some((island, target)), Some((_, source))) = (local[target], local[source]) else {
                continue;
            };
            islands.islands[island].bridges.push(islands.bridges.len());
            islands.bridges.push(Bridge {
                slot: end.slot,
                resistive: end.resistive,
                target,
                source,
            });
        }
        islands
    }

    pub(super) fn len(&self) -> usize {
        self.islands.len()
    }

    pub(super) fn bridge_count(&self) -> usize {
        self.bridges.len()
    }

    /// Note which islands hold a `trireg` bit, given each signal's charge.
    pub(super) fn mark_charged<T>(&mut self, charges: &[Option<T>]) {
        self.charged = self
            .islands
            .iter()
            .map(|island| {
                island.signals.iter().any(|signal| {
                    charges
                        .get(usize::from(*signal))
                        .is_some_and(Option::is_some)
                })
            })
            .collect();
    }

    fn charged(&self, island: usize) -> bool {
        self.charged.get(island).copied().unwrap_or(false)
    }
}

impl DigitalSignalStore {
    /// What switch direction `bridge` passes, as of the last time its island
    /// resolved.
    pub(super) fn bridged(&self, bridge: usize) -> StrengthBit {
        self.bridged
            .get(bridge)
            .copied()
            .unwrap_or(StrengthBit::HIGH_Z)
    }

    /// Resolve every island `signal` belongs to and publish its nets; returns
    /// `false`, doing nothing, for a signal in no island.
    pub(super) fn settle_islands(&mut self, signal: DigitalSignalId, depth: u32) -> bool {
        let tables = Arc::clone(&self.strengths);
        let Some(owned) = tables.islands.by_signal.get(&signal) else {
            return false;
        };
        for &island in owned {
            self.solve_island(island);
        }
        for &island in owned {
            for &member in &tables.islands.islands[island].signals {
                self.note_ambiguous_charge(member);
                let resolved = self.resolve(member);
                self.publish_connected(member, resolved);
                self.note_holding(member);
                self.pass_strengths(member, depth);
            }
        }
        true
    }

    /// Re-derive what every switch passes, without publishing: a restored
    /// store's contributions decide it.
    pub(super) fn solve_islands(&mut self) -> bool {
        let before = self.bridged.clone();
        for island in 0..self.strengths.islands.len() {
            self.solve_island(island);
        }
        before != self.bridged
    }

    /// Resolve one island to its least fixed point; see the module
    /// documentation.
    ///
    /// An island holding a `trireg` bit is resolved twice: with every charge
    /// in it, which is what its nets carry, and with none, which says which
    /// bits only charge reaches (`store/charge.rs`).
    fn solve_island(&mut self, index: usize) {
        let tables = Arc::clone(&self.strengths);
        let island = &tables.islands.islands[index];
        let charged = tables.islands.charged(index);
        for &bridge in &island.bridges {
            self.bridged[bridge] = StrengthBit::HIGH_Z;
            if charged {
                self.charge.clear_bridged_free(bridge);
            }
        }
        let mut resolved: Vec<(DigitalSignalId, Vec<StrengthBit>, Vec<StrengthBit>)> = Vec::new();
        let mut base = Vec::with_capacity(island.nodes.len());
        let mut base_free = Vec::with_capacity(island.nodes.len());
        // A forced bit (section 9.3.2) holds its value whatever the island
        // passes it, and drives the island with that value at strong.
        let mut pinned = vec![false; island.nodes.len()];
        for (node, &(signal, bit)) in island.nodes.iter().enumerate() {
            if let Some(forced) = self.forced_bit(signal, bit) {
                pinned[node] = true;
                base.push(StrengthBit::strong(forced));
                base_free.push(StrengthBit::strong(forced));
                continue;
            }
            let at = match resolved.iter().position(|(seen, _, _)| *seen == signal) {
                Some(at) => at,
                None => {
                    let free = if charged {
                        self.resolve_strengths_as(signal, true)
                    } else {
                        Vec::new()
                    };
                    resolved.push((signal, self.resolve_strengths(signal), free));
                    resolved.len() - 1
                }
            };
            let at_bit = |bits: &[StrengthBit]| {
                bits.get(bit as usize)
                    .copied()
                    .unwrap_or(StrengthBit::HIGH_Z)
            };
            base.push(at_bit(&resolved[at].1));
            base_free.push(at_bit(&resolved[at].2));
        }
        // A switch whose driver has not yet run is off.
        let enables: Vec<FourStateBit> = island
            .bridges
            .iter()
            .map(
                |&bridge| match &self.contributions[tables.islands.bridges[bridge].slot].value {
                    ContributionValue::FourState(Some(value)) if value.width() >= 2 => {
                        value.bit(value.width() - 1)
                    }
                    _ => FourStateBit::Zero,
                },
            )
            .collect();
        let walk = |base: &[StrengthBit], passes: &mut dyn FnMut(usize, StrengthBit)| {
            // Every step can only strengthen or widen a bit, so the walk
            // settles; the bound is a backstop against a resolution that does
            // not.
            let mut values = base.to_vec();
            for _ in 0..4 * island.nodes.len() + 8 {
                let mut next = base.to_vec();
                for (&bridge, &enable) in island.bridges.iter().zip(&enables) {
                    let direction = tables.islands.bridges[bridge];
                    let passed =
                        strength::mos(values[direction.source], enable, direction.resistive);
                    passes(bridge, passed);
                    if !pinned[direction.target] {
                        next[direction.target] = next[direction.target].resolve(passed);
                    }
                }
                if next == values {
                    break;
                }
                values = next;
            }
        };
        walk(&base, &mut |bridge, passed| self.bridged[bridge] = passed);
        if charged {
            walk(&base_free, &mut |bridge, passed| {
                self.charge.set_bridged_free(bridge, passed);
            });
        }
    }
}
