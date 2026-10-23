//! Pending updates of delayed drivers and nets (IEEE 1364-2005 sections
//! 6.1.3 and 7.14).
//!
//! Each delayed driver and each delayed net has one pending slot: the value
//! due and the tick it is due at. The store holds a delayed value back
//! (`store/delays.rs`), and the inertial rule decides what becomes of it:
//!
//! - a value the same as the pending one keeps the pending update and its
//!   original due tick;
//! - any other cancels the pending update, and is scheduled at now plus its
//!   transition's delay when it differs from what the driver or net has now.
//!
//! So a pulse shorter than the delay never arrives. The slots share one
//! wakeup identity kept at the earliest due tick, so a cancelled update
//! simply stops being the next tick, and an analog stepper reading the next
//! tick never steps to it.
//!
//! A module path destination's change made while one is pending is a pulse
//! (IEEE 1364-2005 section 14.6) as wide as the time from the pending
//! update's due tick to the new one's:
//!
//! - narrower than the path's `PATHPULSE$` reject limit (by default its
//!   delay, so every pulse) it is rejected, as above;
//! - narrower than the error limit it shows `x`, from the leading edge's
//!   due tick or, `pulsestyle_ondetect`, from now, to the trailing edge's;
//! - otherwise both edges arrive;
//! - a trailing edge due before the leading one cancels both, or under
//!   `showcancelled` shows `x` from the trailing edge's due tick (or now)
//!   to the leading edge's, then takes the new value.
//!
//! So such a destination may hold several pending updates, in due order.

use super::super::store::DelayRequest;
use super::*;
use rspice_veriloga::four_state::FourStateBit;
use serde::{Deserialize, Serialize};

/// A pending slot's owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(super) enum PendingOwner {
    Driver(rspice_veriloga::canonical_ir::digital::DigitalDriverId),
    Net(DigitalSignalId),
}

/// The pending updates, by due tick and by owner, and the module paths that
/// choose a path destination's delay.
#[derive(Debug, Clone)]
pub(super) struct PendingDrives {
    pub(super) target: TargetId,
    due: BTreeMap<u64, BTreeSet<PendingOwner>>,
    /// Each owner's pending updates in due order, at most one per tick: one
    /// at most but for a module path destination's pulse.
    slots: BTreeMap<PendingOwner, Vec<(u64, FourStateValue)>>,
    /// `None` for a plan with no module path, or a run that leaves them out
    /// (`.options hdl_path_delays=no`).
    pub(super) paths: Option<super::paths::PathDelays>,
}

/// One pending update as the host image's `pending_drives` section stores it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PendingDriveImage {
    due: u64,
    owner: PendingOwner,
    value: FourStateValue,
}

impl PendingDrives {
    /// Slots for a plan with a delayed driver or net; `None` for any other,
    /// which interns no wakeup.
    pub(super) fn for_plan(
        plan: &CanonicalDigitalPlan,
        scheduler: &mut EventScheduler,
    ) -> Option<Self> {
        (!plan.delays.is_empty()).then(|| Self {
            target: scheduler.intern_target(EventTarget {
                node_id: usize::MAX,
                port_name: "delayed drives".to_string(),
                driver_index: 0,
                instance: "delay".to_string(),
            }),
            due: BTreeMap::new(),
            slots: BTreeMap::new(),
            paths: super::paths::PathDelays::for_plan(plan),
        })
    }

    /// The earliest pending update.
    pub(super) fn next(&self) -> Option<u64> {
        self.due.first_key_value().map(|(tick, _)| *tick)
    }

    /// The owner's last pending update.
    fn last(&self, owner: PendingOwner) -> Option<&(u64, FourStateValue)> {
        self.slots.get(&owner).and_then(|queue| queue.last())
    }

    /// Queue an update after the owner's others, cancelling any due later
    /// than it; one due at the tick of the last replaces it. An update due
    /// past the last representable tick lands after the run ends.
    fn push(&mut self, due: u64, owner: PendingOwner, value: FourStateValue) {
        if due > TimeResolution::MAX_EXACT_TICKS {
            return;
        }
        while self.last(owner).is_some_and(|(at, _)| *at > due) {
            self.pop(owner);
        }
        let queue = self.slots.entry(owner).or_default();
        match queue.last_mut() {
            Some(last) if last.0 == due => last.1 = value,
            _ => {
                queue.push((due, value));
                self.due.entry(due).or_default().insert(owner);
            }
        }
    }

    /// Take back the owner's last pending update.
    fn pop(&mut self, owner: PendingOwner) -> Option<(u64, FourStateValue)> {
        let queue = self.slots.get_mut(&owner)?;
        let last = queue.pop()?;
        if queue.is_empty() {
            self.slots.remove(&owner);
        }
        self.unindex(last.0, owner);
        Some(last)
    }

    fn cancel(&mut self, owner: PendingOwner) {
        for (due, _) in self.slots.remove(&owner).unwrap_or_default() {
            self.unindex(due, owner);
        }
    }

    fn unindex(&mut self, due: u64, owner: PendingOwner) {
        if let Some(bucket) = self.due.get_mut(&due) {
            bucket.remove(&owner);
            if bucket.is_empty() {
                self.due.remove(&due);
            }
        }
    }

    /// Schedule a module path destination's change to `value` at `tick`
    /// (sections 14.4 and 14.6), as the module documentation describes;
    /// `current` is what it has now.
    fn schedule_path(
        &mut self,
        store: &mut DigitalSignalStore,
        (signal, value): (DigitalSignalId, FourStateValue),
        current: FourStateValue,
        tick: u64,
    ) {
        let owner = PendingOwner::Net(signal);
        let Some(style) = self.paths.as_ref().map(|paths| paths.style(signal)) else {
            return;
        };
        loop {
            let queue = self.slots.get(&owner).map_or(&[][..], Vec::as_slice);
            let lead = queue.last().cloned();
            let before = match queue {
                [.., (_, before), _] => before.clone(),
                _ => current.clone(),
            };
            let from = lead.as_ref().map_or(&before, |(_, value)| value);
            if *from == value {
                // The pending update, or what the destination has, stands.
                return;
            }
            let Some(paths) = &self.paths else {
                return;
            };
            let delay = paths.delay(store, signal, (from, &value), tick);
            let due = tick.saturating_add(delay.ticks);
            let Some((lead_due, lead_value)) = lead else {
                if due == tick {
                    apply(store, DelayRequest::Net { signal, value });
                } else {
                    self.push(due, owner, value);
                }
                return;
            };
            let shown = shown_as_x(&before, &lead_value);
            let width = due.checked_sub(lead_due);
            match (width, delay.limits) {
                // Section 14.6.4.2: a trailing edge due first.
                (None, _) if style.show_cancelled => {
                    if style.on_detect {
                        self.cancel(owner);
                        apply(
                            store,
                            DelayRequest::Net {
                                signal,
                                value: shown,
                            },
                        );
                    } else {
                        self.pop(owner);
                        self.push(due, owner, shown);
                    }
                    self.push(lead_due, owner, value);
                    return;
                }
                (Some(width), Some([reject, error])) if width >= reject => {
                    if width < error {
                        // Section 14.6.4.1: the pulse shows `x`.
                        if style.on_detect {
                            self.cancel(owner);
                            apply(
                                store,
                                DelayRequest::Net {
                                    signal,
                                    value: shown,
                                },
                            );
                        } else {
                            self.pop(owner);
                            self.push(lead_due, owner, shown);
                        }
                    }
                    self.push(due, owner, value);
                    return;
                }
                // Rejected: the change is measured again from the value
                // before the leading edge.
                _ => {
                    self.pop(owner);
                }
            }
        }
    }

    /// The pending updates in due order, as the host image stores them.
    pub(super) fn image(&self) -> Vec<PendingDriveImage> {
        self.due
            .iter()
            .flat_map(|(due, owners)| {
                owners.iter().filter_map(move |owner| {
                    let (_, value) = self.slots.get(owner)?.iter().find(|(at, _)| at == due)?;
                    Some(PendingDriveImage {
                        due: *due,
                        owner: *owner,
                        value: value.clone(),
                    })
                })
            })
            .collect()
    }
}

/// A pulse shown as `x` (section 14.6): the leading edge's value with each
/// bit it changes made `x`.
fn shown_as_x(before: &FourStateValue, lead: &FourStateValue) -> FourStateValue {
    if before.width() != lead.width() {
        return FourStateValue::splat(lead.width(), FourStateBit::Unknown);
    }
    let mut shown = lead.clone();
    for bit in 0..lead.width() {
        if before.bit(bit) != lead.bit(bit) {
            shown.set_bit(bit, FourStateBit::Unknown);
        }
    }
    shown
}

fn owner_of(request: &DelayRequest) -> (PendingOwner, &FourStateValue) {
    match request {
        DelayRequest::Drive { driver, value } => (PendingOwner::Driver(*driver), value),
        DelayRequest::Net { signal, value } => (PendingOwner::Net(*signal), value),
    }
}

fn request_of(owner: PendingOwner, value: FourStateValue) -> DelayRequest {
    match owner {
        PendingOwner::Driver(driver) => DelayRequest::Drive { driver, value },
        PendingOwner::Net(signal) => DelayRequest::Net { signal, value },
    }
}

impl DigitalHost {
    /// Run every module path destination undelayed (`.options
    /// hdl_path_delays=no`), before the design starts.
    pub(crate) fn drop_path_delays(&mut self) {
        if let Some(pending) = &mut self.pending
            && pending.paths.take().is_some()
        {
            self.store.drop_path_delays();
        }
    }

    /// Whether [`Self::drop_path_delays`] left the plan's module paths out,
    /// which a fresh or restored host of the same run keeps.
    pub(super) fn path_delays_dropped(&self) -> bool {
        !self.plan.delays.paths.is_empty()
            && self
                .pending
                .as_ref()
                .is_some_and(|pending| pending.paths.is_none())
    }

    /// Note on each module path the changes of its source among `drained`,
    /// the round's transitions at `tick`.
    pub(super) fn note_path_changes(&mut self, drained: &[SignalTransition], tick: u64) {
        if let Some(paths) = self
            .pending
            .as_mut()
            .and_then(|pending| pending.paths.as_mut())
        {
            for transition in drained {
                paths.note_change(transition, tick);
            }
        }
    }

    /// Apply the inertial rule to every value the store held back at `tick`.
    pub(super) fn schedule_pending_drives(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        if !self.store.has_delay_requests() {
            return Ok(());
        }
        let requests = self.store.take_delay_requests();
        let Some(mut pending) = self.pending.take() else {
            return Ok(());
        };
        let before = pending.next();
        for request in requests {
            let (owner, value) = owner_of(&request);
            let current = self
                .store
                .delayed_current(&request)
                .unwrap_or_else(|| FourStateValue::splat(value.width(), FourStateBit::Unknown));
            if let DelayRequest::Net { signal, .. } = &request
                && pending.paths.is_some()
                && self.store.path_destination(*signal)
            {
                let target = (*signal, value.clone());
                pending.schedule_path(&mut self.store, target, current, tick);
                continue;
            }
            if let Some((_, waiting)) = pending.last(owner)
                && self
                    .store
                    .transition_ticks(&request, waiting, value)
                    .is_none()
            {
                continue;
            }
            pending.cancel(owner);
            match self.store.transition_ticks(&request, &current, value) {
                None => {}
                Some(0) => apply(&mut self.store, request),
                Some(delay) => pending.push(tick.saturating_add(delay), owner, value.clone()),
            }
        }
        self.pending = Some(pending);
        self.rearm_pending(before)
    }

    /// Apply every update due by `tick`; the wakeup that brought the host
    /// here was the earliest.
    pub(super) fn fire_pending_drives(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        let Some(pending) = self.pending.as_mut() else {
            return Ok(());
        };
        let before = pending.next();
        let mut due = Vec::new();
        while let Some(entry) = pending.due.first_entry()
            && *entry.key() <= tick
        {
            for owner in entry.remove() {
                // The owner's first update is the one due now.
                if let Some(queue) = pending.slots.get_mut(&owner)
                    && !queue.is_empty()
                {
                    let (_, value) = queue.remove(0);
                    if queue.is_empty() {
                        pending.slots.remove(&owner);
                    }
                    due.push(request_of(owner, value));
                }
            }
        }
        for request in due {
            apply(&mut self.store, request);
        }
        self.rearm_pending(before)
    }

    /// Keep the one wakeup at the earliest pending update.
    fn rearm_pending(&mut self, before: Option<u64>) -> Result<(), DigitalRunError> {
        let Some(pending) = &self.pending else {
            return Ok(());
        };
        let (target, next) = (pending.target, pending.next());
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

    /// Install the updates a host image's `pending_drives` section names.
    /// Each must belong to a delayed driver or net, in due order, and only
    /// a module path destination may have more than one.
    pub(super) fn restore_pending_drives(
        &mut self,
        image: &[PendingDriveImage],
    ) -> Result<(), String> {
        if image.is_empty() {
            return Ok(());
        }
        let mut pending = self.pending.take().ok_or_else(|| {
            "host pending_drives section names updates the design has no delay for".to_string()
        })?;
        let mut previous = 0;
        for update in image {
            let delayed = match update.owner {
                PendingOwner::Driver(driver) => self
                    .plan
                    .delays
                    .drivers
                    .iter()
                    .any(|delay| delay.driver == driver),
                PendingOwner::Net(signal) => self.store.delays_net(signal),
            };
            let current = self
                .store
                .delayed_current(&request_of(update.owner, update.value.clone()));
            let queues = matches!(update.owner, PendingOwner::Net(signal)
                if pending.paths.is_some() && self.store.path_destination(signal));
            if !delayed
                || update.due < previous
                || pending
                    .last(update.owner)
                    .is_some_and(|(due, _)| !queues || *due >= update.due)
                || current.is_none_or(|current| current.width() != update.value.width())
            {
                return Err("pending drive names an undelayed owner, twice or out of order".into());
            }
            self.instant_of(update.due).map_err(|e| e.to_string())?;
            pending.push(update.due, update.owner, update.value.clone());
            previous = update.due;
        }
        self.pending = Some(pending);
        Ok(())
    }
}

fn apply(store: &mut DigitalSignalStore, request: DelayRequest) {
    match request {
        DelayRequest::Drive { driver, value } => store.apply_drive(driver, value),
        DelayRequest::Net { signal, value } => store.apply_net_value(signal, value),
    }
}
