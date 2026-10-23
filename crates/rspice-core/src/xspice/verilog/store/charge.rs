//! `trireg` nets: charge storage (IEEE 1364-2005 sections 4.6.3, 7.13.2 and
//! 7.14.2).
//!
//! A `trireg` bit holds its value as a charge, at the net's charge strength:
//! `small`, `medium` or `large`, `medium` by default (section 4.4.1). Section
//! 4.6.3 gives the bit two states, decided by its own drivers:
//!
//! - **Driven:** a driver of the bit has the value 0, 1 or `x`. The bit is
//!   what its drivers resolve to, at their strength, and the charge takes no
//!   part, however large: a `weak` driver of a `large` `trireg` gives `We1`.
//! - **Capacitive:** every driver of the bit is `z`. The charge is then one
//!   more contribution to the bit's resolution, a drive of the net's own
//!   current value at its charge strength. Alone it decides the bit; inside a
//!   switch island (section 4.6.3.1) it resolves by section 7.10 against what
//!   the island passes, so the larger of two charges wins the charge sharing,
//!   and a large charge outweighs a weak driver of another net of the island.
//!
//! A driver that drives `L` or `H` (section 7.10.1), as a three-state gate
//! under an `x` enable does, is one value or `z`, so the bit may be in either
//! state. While no driver surely drives it, the bit is the ambiguous strength
//! covering both: what the drivers that may be on drive, and the charge. An
//! `L` from a strong driver against a stored 1 of medium charge is `St0` or
//! `Me1`, an `x` from `St0` to `Me1`, and the net then holds `x`.
//!
//! The charge is kept only as the net's value, so once the net reads `x` the
//! held case is `x` at the charge strength. That covers the same range while
//! the drivers reach the charge's strength on the side opposite it. A weaker
//! one would have its side overstated, `We0` against `LaX` giving `LaX`
//! where `We0` or `La1` is `34X`, so that case is refused by name.
//!
//! What a bidirectional switch passes is not a driver of the bit here: it is
//! the island's resolution, of which the charge is a part. The net starts at
//! `x` at its charge strength (section 4.2.1).
//!
//! # Decay
//!
//! A bit is *holding* when its drivers alone leave it, or may leave it, at
//! high impedance: the bit's contributions, and through a switch island what
//! reaches it from drivers rather than from charges. When a bit starts holding the store
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

use super::StrengthTables;
use super::strength::{SlotGate, StrengthBit};
use super::{DigitalSignalStore, signal_name};
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

/// What one `trireg` bit has from its own drivers, which decides whether its
/// charge counts (section 4.6.3), and from its switch island.
#[derive(Debug, Clone)]
pub(super) struct OwnDrive {
    /// Whether an own driver surely drives the bit: the driven state.
    sure: bool,
    /// What each own driver that may be off drives when it is on.
    maybe: Vec<StrengthBit>,
    /// What the bit's switch island passes it.
    island: StrengthBit,
}

impl Default for OwnDrive {
    fn default() -> Self {
        Self {
            sure: false,
            maybe: Vec::new(),
            island: StrengthBit::HIGH_Z,
        }
    }
}

/// The most own drivers that may be off whose every combination
/// [`OwnDrive::either`] resolves one by one.
const MAX_COMBINED: usize = 10;

impl OwnDrive {
    /// Add what a bidirectional switch passes the bit.
    pub(super) fn pass(&mut self, passed: StrengthBit) {
        self.island = self.island.resolve(passed);
    }

    /// Whether the own drivers may leave the bit to its charge: none is
    /// surely on and at least one may be off. Such a bit's charge may decay.
    fn may_release(&self) -> bool {
        !self.sure && !self.maybe.is_empty()
    }

    /// Sections 4.6.3 and 7.10.1: with every driver that may be off off, the
    /// bit holds `stored` (the capacitive state), and with any of them on it
    /// is what those resolve to, the charge taking no part (the driven
    /// state). The bit is the range covering every case, so a driver against
    /// an opposite charge gives an ambiguous `x` and neither side alone.
    ///
    /// Every resolution's ends are ends of what it resolves, so without a
    /// switch island the drivers one at a time already reach the ends of any
    /// set of them. Through an island each set is resolved, and past
    /// [`MAX_COMBINED`] drivers the range covering the island and every
    /// driver stands in, which holds every case.
    fn either(&self, stored: StrengthBit) -> StrengthBit {
        let held = self.island.resolve(stored);
        if self.island == StrengthBit::HIGH_Z {
            return self.maybe.iter().fold(held, |cover, on| cover.cover(*on));
        }
        let count = self.maybe.len();
        if count > MAX_COMBINED {
            return self
                .maybe
                .iter()
                .fold(held.cover(self.island), |cover, on| cover.cover(*on));
        }
        (1u32..1 << count).fold(held, |cover, set| {
            let driven = (0..count)
                .filter(|position| set & (1 << position) != 0)
                .fold(StrengthBit::HIGH_Z, |driven, position| {
                    driven.resolve(self.maybe[position])
                });
            cover.cover(self.island.resolve(driven))
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
    /// Why the run is refused: a bit whose exact value the store cannot keep
    /// ([`DigitalSignalStore::note_ambiguous_charge`]). The host drains it.
    refused: Option<String>,
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

    /// Resolve a `trireg`'s charge into the bits its own drivers, `own`, may
    /// leave at `z`: the net's own value at its charge strength, and `x` for a
    /// bit whose charge is decaying. A bit some driver surely drives is in
    /// the driven state and keeps what its drivers resolve to.
    pub(super) fn resolve_charge(
        &self,
        signal: DigitalSignalId,
        own: &[OwnDrive],
        bits: &mut [StrengthBit],
    ) {
        let Some(charge) = self.charge_of(signal) else {
            return;
        };
        let value = &self.values[usize::from(signal)];
        for (position, (bit, own)) in (0u32..).zip(bits.iter_mut().zip(own)) {
            if own.sure {
                continue;
            }
            let held = if self.charge.decaying == Some((signal, position)) {
                FourStateBit::Unknown
            } else {
                value.bit(position)
            };
            let stored = |held| StrengthBit::driven(held, charge.strength);
            *bit = if own.maybe.is_empty() {
                bit.resolve(stored(held))
            } else {
                // The charge is the net's value, so once the cases disagree
                // the bit holds `x`; resolving with it now gives the same bit
                // however often the net settles.
                let covered = own.either(stored(held));
                if covered.value() == held {
                    covered
                } else {
                    own.either(stored(FourStateBit::Unknown))
                }
            };
        }
    }

    /// Before `signal` publishes a new value, note the first of its bits
    /// whose exact value [`Self::resolve_charge`] cannot keep, for the host
    /// to refuse the run with (sections 4.6.3 and 7.10).
    ///
    /// Such a bit holds 0 or 1, and drivers that may be off leave it at the
    /// range covering their drive and that charge, an `x`. That range then
    /// stands on the net's value alone, `x`, which reaches the charge's
    /// strength on both sides; a driver weaker than the charge on the side
    /// opposite it would be overstated.
    pub(super) fn note_ambiguous_charge(&mut self, signal: DigitalSignalId) {
        let Some(charge) = self.charge_of(signal) else {
            return;
        };
        if self.charge.refused.is_some() {
            return;
        }
        let (_, own) = self.fold_strengths(signal, false);
        let value = &self.values[usize::from(signal)];
        let stored = |held| StrengthBit::driven(held, charge.strength);
        for (position, own) in (0u32..).zip(&own) {
            let held = value.bit(position);
            if own.sure
                || own.maybe.is_empty()
                || !matches!(held, FourStateBit::Zero | FourStateBit::One)
                || self.charge.decaying == Some((signal, position))
            {
                continue;
            }
            let exact = own.either(stored(held));
            let kept = own.either(stored(FourStateBit::Unknown));
            if exact.value() == held || exact == kept {
                continue;
            }
            self.charge.refused = Some(format!(
                "the `trireg` net `{}` bit {position} holds {} and a driver under an `x` \
                 enable drives {}, weaker than that charge: by IEEE 1364-2005 sections 4.6.3 \
                 and 7.10 the net is {}, the ambiguous combination of the two, but a `trireg` \
                 keeps its charge only as its value, and the `x` that leaves would make it {}; \
                 give the enable a known value, or the driver at least the charge's strength",
                signal_name(&self.plan, signal),
                stored(held).display(),
                own.either(StrengthBit::HIGH_Z).display(),
                exact.display(),
                kept.display(),
            ));
            return;
        }
    }

    /// The refusal [`Self::note_ambiguous_charge`] noted, once.
    pub(crate) fn take_charge_refusal(&mut self) -> Option<String> {
        self.charge.refused.take()
    }

    pub(super) fn has_charge_refusal(&self) -> bool {
        self.charge.refused.is_some()
    }

    /// Fold an own driver's contribution `passed` to a `trireg` bit into
    /// `own`. A three-state gate or MOS switch whose enable is `x` or `z`
    /// drives `L`, `H` or `x`: what it drives on, or nothing (section 7.10.1).
    /// So does a driver whose drive is an `L` or `H` itself, as an `x` at a
    /// `highz` strength is. Either may be off; any other driver is surely on.
    pub(super) fn note_own_drive(
        &self,
        own: &mut OwnDrive,
        slot: usize,
        (value, offset): (&FourStateValue, u32),
        passed: StrengthBit,
        free: bool,
    ) {
        if passed == StrengthBit::HIGH_Z {
            return;
        }
        let gate = self.slot_gate(slot);
        let switch = matches!(gate, Some(SlotGate::Mos { .. }));
        let unknown_enable = (switch || gate == Some(SlotGate::Tristate))
            && matches!(
                value.bit(value.width() - 1),
                FourStateBit::Unknown | FourStateBit::HighImpedance
            );
        let on = if unknown_enable {
            self.gated_bit(slot, value, offset, free, Some(FourStateBit::One))
        } else {
            passed
        };
        if unknown_enable || on.reaches_high_z() {
            // A switch passes a net's strength, which may be an `L` or `H` that
            // several drivers made.
            own.maybe.push(on.driving_part(!switch));
        } else {
            own.sure = true;
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

    /// Which bits of `signal` its drivers alone leave, or may leave, at high
    /// impedance: a bit only drivers that may be off drive, with nothing
    /// from its island, may be holding, and so may decay. A forced bit is
    /// driven by its force.
    fn holding_bits(&self, signal: DigitalSignalId) -> Vec<bool> {
        let (free, own) = self.fold_strengths(signal, true);
        (0u32..)
            .zip(free.into_iter().zip(own))
            .map(|(bit, (free, own))| {
                (free == StrengthBit::HIGH_Z
                    || (own.may_release() && own.island == StrengthBit::HIGH_Z))
                    && self.forced_bit(signal, bit).is_none()
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
