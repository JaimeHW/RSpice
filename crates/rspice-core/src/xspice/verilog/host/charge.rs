//! `trireg` charge decay timers (IEEE 1364-2005 section 7.14.2).
//!
//! The store says when a `trireg` bit starts or stops holding charge
//! (`store/charge.rs`). A bit that starts holding decays after its net's
//! charge decay time unless a driver takes it back first, which cancels the
//! decay. The pending decays share one wakeup identity, kept scheduled at the
//! earliest of them, so a cancelled decay simply stops being the next tick.

use super::*;
use rspice_veriloga::canonical_ir::digital::{DigitalNetBit, DigitalNetType};

/// The pending decays, by due tick and by bit.
#[derive(Debug, Clone)]
pub(super) struct ChargeTimers {
    pub(super) target: TargetId,
    due: BTreeMap<u64, BTreeSet<(DigitalSignalId, u32)>>,
    bits: BTreeMap<(DigitalSignalId, u32), u64>,
}

impl ChargeTimers {
    /// Timers for a plan with a `trireg` that decays; `None` for any other,
    /// which interns no wakeup.
    pub(super) fn for_plan(
        plan: &CanonicalDigitalPlan,
        scheduler: &mut EventScheduler,
    ) -> Option<Self> {
        plan.strength
            .nets
            .iter()
            .any(|net| matches!(net.net_type, DigitalNetType::Trireg { decay: Some(_), .. }))
            .then(|| Self {
                target: scheduler.intern_target(EventTarget {
                    node_id: usize::MAX,
                    port_name: "charge decay".to_string(),
                    driver_index: 0,
                    instance: "trireg".to_string(),
                }),
                due: BTreeMap::new(),
                bits: BTreeMap::new(),
            })
    }

    /// The earliest pending decay.
    pub(super) fn next(&self) -> Option<u64> {
        self.due.first_key_value().map(|(tick, _)| *tick)
    }

    fn insert(&mut self, due: u64, bit: (DigitalSignalId, u32)) {
        self.cancel(bit);
        self.due.entry(due).or_default().insert(bit);
        self.bits.insert(bit, due);
    }

    fn cancel(&mut self, bit: (DigitalSignalId, u32)) {
        let Some(due) = self.bits.remove(&bit) else {
            return;
        };
        if let Some(bucket) = self.due.get_mut(&due) {
            bucket.remove(&bit);
            if bucket.is_empty() {
                self.due.remove(&due);
            }
        }
    }

    /// The pending decays as the host image's `strength` section stores them.
    pub(super) fn image(&self) -> Vec<(u64, Vec<DigitalNetBit>)> {
        self.due
            .iter()
            .map(|(tick, bits)| {
                (
                    *tick,
                    bits.iter()
                        .map(|&(signal, bit)| DigitalNetBit { signal, bit })
                        .collect(),
                )
            })
            .collect()
    }
}

impl DigitalHost {
    /// Schedule a decay for every bit the store says started holding charge
    /// at `tick`, and cancel it for every bit that stopped.
    pub(super) fn schedule_charge_changes(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        let changes = self.store.take_charge_changes();
        let Some(timers) = self.charge.as_mut().filter(|_| !changes.is_empty()) else {
            return Ok(());
        };
        let before = timers.next();
        for change in changes {
            let bit = (change.signal, change.bit);
            timers.cancel(bit);
            // A decay due past the last representable tick happens after
            // the run ends.
            if let Some(due) = change
                .decay
                .and_then(|decay| tick.checked_add(decay))
                .filter(|due| *due <= TimeResolution::MAX_EXACT_TICKS)
            {
                timers.insert(due, bit);
            }
        }
        self.rearm_charge(before)
    }

    /// Decay every bit due by `tick`; the wakeup that brought the host here
    /// was the earliest.
    pub(super) fn fire_charge(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        let Some(timers) = self.charge.as_mut() else {
            return Ok(());
        };
        let before = timers.next();
        let mut due = Vec::new();
        while let Some(entry) = timers.due.first_entry()
            && *entry.key() <= tick
        {
            for bit in entry.remove() {
                timers.bits.remove(&bit);
                due.push(bit);
            }
        }
        for (signal, bit) in due {
            self.store.decay_charge(signal, bit);
        }
        self.rearm_charge(before)
    }

    /// Keep the one wakeup at the earliest pending decay.
    fn rearm_charge(&mut self, before: Option<u64>) -> Result<(), DigitalRunError> {
        let Some(timers) = &self.charge else {
            return Ok(());
        };
        let (target, next) = (timers.target, timers.next());
        if next == before {
            return Ok(());
        }
        self.scheduler.cancel_target(target);
        if let Some(next) = next {
            let at = self.instant_of(next)?;
            self.scheduler.schedule_id_at(
                at,
                SchedulerRegion::Active,
                target,
                EventValue::Digital(DigitalValue::default()),
            )?;
        }
        Ok(())
    }

    /// Install the decays a host image's `strength` section names. Each must
    /// be a bit of a decaying `trireg` that holds charge, at most once.
    pub(super) fn restore_charge_timers(
        &mut self,
        image: &[(u64, Vec<DigitalNetBit>)],
    ) -> Result<(), String> {
        if image.is_empty() {
            return Ok(());
        }
        let mut timers = self.charge.take().ok_or_else(|| {
            "host strength section names decays the design has none of".to_string()
        })?;
        let mut previous = None;
        for (tick, bits) in image {
            if previous.is_some_and(|previous| previous >= *tick) || bits.is_empty() {
                return Err("unordered or empty charge decay bucket".into());
            }
            self.instant_of(*tick).map_err(|e| e.to_string())?;
            for bit in bits {
                if !self.store.holds_charge(bit.signal, bit.bit)
                    || timers.bits.contains_key(&(bit.signal, bit.bit))
                {
                    return Err("charge decay names a bit that does not hold charge".into());
                }
                timers.insert(*tick, (bit.signal, bit.bit));
            }
            previous = Some(*tick);
        }
        self.charge = Some(timers);
        Ok(())
    }
}
