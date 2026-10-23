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
    slots: BTreeMap<PendingOwner, (u64, FourStateValue)>,
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

    /// How long `request`'s driver or net takes to go from `from` to `to`,
    /// or `None` when that is no change: a module path destination's delay
    /// from its paths, counted from `now`, and any other's from the store.
    fn transition_ticks(
        &self,
        store: &DigitalSignalStore,
        request: &DelayRequest,
        (from, to): (&FourStateValue, &FourStateValue),
        now: u64,
    ) -> Option<u64> {
        if let DelayRequest::Net { signal, .. } = request
            && let Some(paths) = &self.paths
            && store.path_destination(*signal)
        {
            return (from != to).then(|| paths.delay(store, *signal, (from, to), now));
        }
        store.transition_ticks(request, from, to)
    }

    /// The earliest pending update.
    pub(super) fn next(&self) -> Option<u64> {
        self.due.first_key_value().map(|(tick, _)| *tick)
    }

    fn insert(&mut self, due: u64, owner: PendingOwner, value: FourStateValue) {
        self.cancel(owner);
        self.due.entry(due).or_default().insert(owner);
        self.slots.insert(owner, (due, value));
    }

    fn cancel(&mut self, owner: PendingOwner) {
        let Some((due, _)) = self.slots.remove(&owner) else {
            return;
        };
        if let Some(bucket) = self.due.get_mut(&due) {
            bucket.remove(&owner);
            if bucket.is_empty() {
                self.due.remove(&due);
            }
        }
    }

    /// The pending updates in due order, as the host image stores them.
    pub(super) fn image(&self) -> Vec<PendingDriveImage> {
        self.due
            .iter()
            .flat_map(|(due, owners)| {
                owners.iter().map(|owner| PendingDriveImage {
                    due: *due,
                    owner: *owner,
                    value: self.slots[owner].1.clone(),
                })
            })
            .collect()
    }
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
            if let Some((_, waiting)) = pending.slots.get(&owner)
                && pending
                    .transition_ticks(&self.store, &request, (waiting, value), tick)
                    .is_none()
            {
                continue;
            }
            pending.cancel(owner);
            let current = self
                .store
                .delayed_current(&request)
                .unwrap_or_else(|| FourStateValue::splat(value.width(), FourStateBit::Unknown));
            match pending.transition_ticks(&self.store, &request, (&current, value), tick) {
                None => {}
                Some(0) => apply(&mut self.store, request),
                // An update due past the last representable tick lands after
                // the run ends.
                Some(delay) => {
                    if let Some(due) = tick
                        .checked_add(delay)
                        .filter(|due| *due <= TimeResolution::MAX_EXACT_TICKS)
                    {
                        pending.insert(due, owner, value.clone());
                    }
                }
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
                if let Some((_, value)) = pending.slots.remove(&owner) {
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
    /// Each must belong to a delayed driver or net, at most once, in due
    /// order.
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
            if !delayed
                || update.due < previous
                || pending.slots.contains_key(&update.owner)
                || current.is_none_or(|current| current.width() != update.value.width())
            {
                return Err("pending drive names an undelayed owner, twice or out of order".into());
            }
            self.instant_of(update.due).map_err(|e| e.to_string())?;
            pending.insert(update.due, update.owner, update.value.clone());
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
