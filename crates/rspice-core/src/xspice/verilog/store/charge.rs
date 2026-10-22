//! `trireg` nets: charge storage (IEEE 1364-2005 sections 4.6.3, 7.13.2 and
//! 7.14.2).
//!
//! A `trireg` bit holds its value as a charge, at the net's charge strength:
//! `small`, `medium` or `large`, `medium` by default (section 4.4.1). Section
//! 4.6.3 gives the bit two states, decided by its own drivers:
//!
//! - **Driven:** a driver of the bit has the value 0, 1 or `x` (an `L` or `H`
//!   reads as `x`, section 7.10.1). The bit is what its drivers resolve to, at
//!   their strength, and the charge takes no part, however large: a `weak`
//!   driver of a `large` `trireg` gives `We1`.
//! - **Capacitive:** every driver of the bit is `z`. The charge is then one
//!   more contribution to the bit's resolution, a drive of the net's own
//!   current value at its charge strength. Alone it decides the bit; inside a
//!   switch island (section 4.6.3.1) it resolves by section 7.10 against what
//!   the island passes, so the larger of two charges wins the charge sharing,
//!   and a large charge outweighs a weak driver of another net of the island.
//!
//! What a bidirectional switch passes is not a driver of the bit here: it is
//! the island's resolution, of which the charge is a part. The net starts at
//! `x` at its charge strength (section 4.2.1).
//!
//! # Decay
//!
//! A bit is *holding* when its drivers alone leave it at high impedance: the
//! bit's contributions, and through a switch island what reaches it from
//! drivers rather than from charges. When a bit starts holding the store
//! reports it, and the host schedules the decay after the net's charge decay
//! time, the third delay of its declaration (section 7.14.2). A driver that
//! drives the bit again before then stops the bit holding, and the host cancels
//! the decay. At the decay the bit's charge becomes `x`, still at its charge
//! strength. Holding is derived from the drivers, so a checkpoint rebuilds it,
//! and the host's image carries the timers.
//!
//! A MOS switch is a driver of the net it drives (section 7.14.2 asks only
//! whether the drivers are off), and passes the strength of the net it reads,
//! charge included. So a `trireg` behind an on `nmos` from another one follows
//! it, decaying when it does, and starts holding of its own when the switch
//! turns off. Only the bidirectional switches share charge.

use super::DigitalSignalStore;
use super::StrengthTables;
use super::strength::StrengthBit;
use rspice_veriloga::canonical_ir::digital::{DigitalDriveStrength, DigitalNetType};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use std::collections::BTreeMap;

/// One `trireg` net's charge strength and decay time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Charge {
    /// The charge strength, as a drive of both values.
    strength: DigitalDriveStrength,
    /// The charge decay time in the plan's ticks; `None` holds for ever.
    decay: Option<u64>,
}

impl Charge {
    /// The charge a net type gives a net, at the plan's precision.
    pub(super) fn of(net_type: DigitalNetType, precision: i8) -> Option<Self> {
        let DigitalNetType::Trireg { charge, decay } = net_type else {
            return None;
        };
        Some(Self {
            strength: DigitalDriveStrength {
                zero: charge,
                one: charge,
            },
            // A decay time this precision cannot count is past any run's end.
            decay: decay.and_then(|decay| decay.ticks_at(precision)),
        })
    }
}

/// A `trireg` bit that started or stopped holding charge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChargeChange {
    pub(crate) signal: DigitalSignalId,
    pub(crate) bit: u32,
    /// The decay time in ticks when the bit started holding, `None` when it
    /// stopped.
    pub(crate) decay: Option<u64>,
}

/// The store's `trireg` state beyond its values.
#[derive(Debug, Clone, Default)]
pub(super) struct ChargeState {
    /// Per `trireg` net that decays, whether each bit holds charge. Derived
    /// from the contributions.
    holding: BTreeMap<DigitalSignalId, Vec<bool>>,
    /// Bits that started or stopped holding since the host last drained.
    changes: Vec<ChargeChange>,
    /// Per bidirectional switch direction of an island with a `trireg` bit,
    /// what it passes with no charge counted. Derived like `bridged`.
    bridged_free: Vec<StrengthBit>,
    /// The bit whose charge is decaying in the settle under way.
    decaying: Option<(DigitalSignalId, u32)>,
}

impl ChargeState {
    /// Every `trireg` starts at `x` (section 4.2.1), and every bit of one that
    /// decays starts holding: no driver has run.
    pub(super) fn new(
        tables: &StrengthTables,
        widths: &[u32],
        values: &mut [FourStateValue],
    ) -> Self {
        let mut state = Self {
            bridged_free: vec![StrengthBit::HIGH_Z; tables.islands.bridge_count()],
            ..Self::default()
        };
        for (index, charge) in tables.charges.iter().enumerate() {
            let Some(charge) = charge else {
                continue;
            };
            values[index] = FourStateValue::splat(widths[index], FourStateBit::Unknown);
            if charge.decay.is_some() {
                state.holding.insert(
                    DigitalSignalId::from(index),
                    vec![true; widths[index] as usize],
                );
            }
        }
        state
    }

    pub(super) fn bridged_free(&self, bridge: usize) -> StrengthBit {
        self.bridged_free
            .get(bridge)
            .copied()
            .unwrap_or(StrengthBit::HIGH_Z)
    }

    pub(super) fn clear_bridged_free(&mut self, bridge: usize) {
        self.set_bridged_free(bridge, StrengthBit::HIGH_Z);
    }

    pub(super) fn set_bridged_free(&mut self, bridge: usize, passed: StrengthBit) {
        if let Some(entry) = self.bridged_free.get_mut(bridge) {
            *entry = passed;
        }
    }
}

impl DigitalSignalStore {
    pub(super) fn charge_of(&self, signal: DigitalSignalId) -> Option<Charge> {
        self.strengths
            .charges
            .get(usize::from(signal))
            .copied()
            .flatten()
    }

    /// Resolve a `trireg`'s charge into the bits its own drivers, `own`, leave
    /// at `z` (the capacitive state): the net's own value at its charge
    /// strength, and `x` for a bit whose charge is decaying. Any other bit is
    /// in the driven state and keeps what its drivers resolve to.
    pub(super) fn resolve_charge(
        &self,
        signal: DigitalSignalId,
        own: &[StrengthBit],
        bits: &mut [StrengthBit],
    ) {
        let Some(charge) = self.charge_of(signal) else {
            return;
        };
        let value = &self.values[usize::from(signal)];
        for (position, (bit, own)) in (0u32..).zip(bits.iter_mut().zip(own)) {
            if *own != StrengthBit::HIGH_Z {
                continue;
            }
            let held = if self.charge.decaying == Some((signal, position)) {
                FourStateBit::Unknown
            } else {
                value.bit(position)
            };
            *bit = bit.resolve(StrengthBit::driven(held, charge.strength));
        }
    }

    /// Re-read which bits of a decaying `trireg` hold charge after it
    /// resolved, and report each that changed.
    pub(super) fn note_holding(&mut self, signal: DigitalSignalId) {
        let Some(previous) = self.charge.holding.get(&signal) else {
            return;
        };
        let holding = self.holding_bits(signal);
        if *previous == holding {
            return;
        }
        let decay = self.charge_of(signal).and_then(|charge| charge.decay);
        for (bit, (&was, &now)) in (0u32..).zip(previous.iter().zip(&holding)) {
            if was != now {
                self.charge.changes.push(ChargeChange {
                    signal,
                    bit,
                    decay: decay.filter(|_| now),
                });
            }
        }
        self.charge.holding.insert(signal, holding);
    }

    /// Which bits of `signal` its drivers alone leave at high impedance. A
    /// forced bit is driven by its force.
    fn holding_bits(&self, signal: DigitalSignalId) -> Vec<bool> {
        let free = self.resolve_strengths_as(signal, true);
        (0u32..)
            .zip(free)
            .map(|(bit, free)| {
                free == StrengthBit::HIGH_Z && self.forced_bit(signal, bit).is_none()
            })
            .collect()
    }

    /// Re-derive which bits hold charge from restored contributions, without
    /// reporting: the host's image carries the timers.
    pub(super) fn rederive_holding(&mut self) {
        let signals: Vec<DigitalSignalId> = self.charge.holding.keys().copied().collect();
        for signal in signals {
            let holding = self.holding_bits(signal);
            self.charge.holding.insert(signal, holding);
        }
        self.charge.changes.clear();
    }

    /// The bits that started or stopped holding since the last call.
    pub(crate) fn take_charge_changes(&mut self) -> Vec<ChargeChange> {
        std::mem::take(&mut self.charge.changes)
    }

    pub(crate) fn has_charge_changes(&self) -> bool {
        !self.charge.changes.is_empty()
    }

    /// Whether bit `bit` of `signal` is a decaying `trireg` bit that holds
    /// charge, which is what a decay timer may name.
    pub(crate) fn holds_charge(&self, signal: DigitalSignalId, bit: u32) -> bool {
        self.charge
            .holding
            .get(&signal)
            .and_then(|holding| holding.get(bit as usize))
            .copied()
            .unwrap_or(false)
    }

    /// Section 7.14.2: a held bit's charge decays to `x`, at its charge
    /// strength, and the net settles with it. A bit a driver has taken back
    /// meanwhile is left alone.
    pub(crate) fn decay_charge(&mut self, signal: DigitalSignalId, bit: u32) {
        if !self.holds_charge(signal, bit) {
            return;
        }
        self.charge.decaying = Some((signal, bit));
        self.settle(signal, 0);
        self.charge.decaying = None;
    }
}
