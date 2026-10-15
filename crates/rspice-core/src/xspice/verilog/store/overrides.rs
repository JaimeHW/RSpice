//! Procedural continuous assignments (IEEE 1364-2005 section 9.3): the
//! override layer of the store.
//!
//! An overridden signal has one record: the `force` that owns each of its
//! bits, the forced bits, the procedural `assign` active on a variable, and
//! the value the signal holds beneath the override. The published value is
//! always the effective one — `force` over `assign` over the base — so every
//! reader, waiter, observer and boundary converter sees the override without
//! a rule of its own.
//!
//! What the base is differs by class, and the difference is section 9.3.2's
//! release rule:
//!
//! * **a net** keeps resolving beneath a force. Its base follows its drivers,
//!   so releasing it publishes the resolved value at once;
//! * **a variable** discards its procedural writes while it is forced or
//!   assigned (sections 9.3.1 and 9.3.2), so its base is the value the
//!   override leaves behind. Releasing it keeps that value until the next
//!   procedural write — unless an `assign` is active, which then takes effect.
//!
//! A right-hand side is continuous. The site's function runs when its
//! statement executes and again whenever one of its operands changes, for as
//! long as the site owns a bit or an `assign`; a change only queues the site,
//! and [`DigitalSignalStore::settle_overrides`] evaluates the queue at the
//! next dispatch, the way a continuous assignment's update is an event rather
//! than a write inside the statement that changed its operand.

use super::*;
use rspice_veriloga::ast::ProceduralContinuousKind;
use rspice_veriloga::canonical_ir::digital::{DigitalOverride, DigitalOverrideId};
use rspice_veriloga::canonical_ir::digital_eval::{DigitalScalar, evaluate_override};
use serde::{Deserialize, Serialize};

/// Re-evaluations one settlement may run before the design is called
/// oscillating: a `force a = ~a` never settles, and this ends it by name.
const OVERRIDE_SETTLE_LIMIT: usize = 1 << 20;

/// One overridden value: four-state bits at the signal's width, or a real.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OverrideValue {
    FourState(FourStateValue),
    Real(f64),
}

/// The override layer of one signal.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SignalOverride {
    /// The `force` site owning each bit position (one position for a real).
    owners: Vec<Option<u32>>,
    /// The forced value at the owned positions.
    forced: OverrideValue,
    /// The active procedural `assign` of a variable, and its value.
    assign: Option<(u32, OverrideValue)>,
    /// The value beneath the override.
    base: OverrideValue,
}

impl SignalOverride {
    fn is_empty(&self) -> bool {
        self.assign.is_none() && self.owners.iter().all(Option::is_none)
    }

    fn owns(&self, site: u32) -> bool {
        self.owners.contains(&Some(site)) || self.assign.as_ref().is_some_and(|a| a.0 == site)
    }

    /// Force over assign over base.
    fn effective(&self) -> OverrideValue {
        let under = self.assign.as_ref().map_or(&self.base, |(_, value)| value);
        match (&self.forced, under) {
            (OverrideValue::FourState(forced), OverrideValue::FourState(under)) => {
                let mut value = under.clone();
                for (position, owner) in self.owners.iter().enumerate() {
                    if owner.is_some() {
                        value.set_bit(position as u32, forced.bit(position as u32));
                    }
                }
                OverrideValue::FourState(value)
            }
            (OverrideValue::Real(forced), under) => {
                if self.owners.first().copied().flatten().is_some() {
                    OverrideValue::Real(*forced)
                } else {
                    under.clone()
                }
            }
            (forced, _) => forced.clone(),
        }
    }
}

/// Every override in effect, with the bookkeeping that keeps them continuous.
#[derive(Debug, Clone, Default)]
pub(crate) struct Overrides {
    signals: BTreeMap<DigitalSignalId, SignalOverride>,
    /// Active sites by operand.
    inputs: BTreeMap<DigitalSignalId, BTreeSet<u32>>,
    /// Sites that own a bit or an `assign` somewhere.
    active: BTreeSet<u32>,
    /// Active sites whose operands changed since the last settlement.
    pending: BTreeSet<u32>,
    error: Option<DigitalEvalError>,
    scratch: DigitalEvalScratch,
}

impl Overrides {
    pub(crate) fn is_empty(&self) -> bool {
        self.signals.is_empty() && self.active.is_empty()
    }
}

/// The override layer at a drained boundary: the host image's `forces`
/// section (design P2 section 5.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OverridesImage {
    signals: Vec<SignalOverrideImage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignalOverrideImage {
    signal: u32,
    owners: Vec<Option<u32>>,
    forced: ValueImage,
    assign: Option<(u32, ValueImage)>,
    base: ValueImage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ValueImage {
    FourState(FourStateValue),
    Real(u64),
}

impl ValueImage {
    fn capture(value: &OverrideValue) -> Self {
        match value {
            OverrideValue::FourState(value) => Self::FourState(value.clone()),
            OverrideValue::Real(value) => Self::Real(value.to_bits()),
        }
    }

    fn restore(&self, real: bool, width: u32) -> Result<OverrideValue, String> {
        match self {
            Self::FourState(value) if !real && value.width() == width => {
                Ok(OverrideValue::FourState(value.clone()))
            }
            Self::Real(bits) if real => Ok(OverrideValue::Real(f64::from_bits(*bits))),
            _ => Err("override value domain or width differs from its signal".into()),
        }
    }
}

impl DigitalSignalStore {
    fn override_site(&self, site: DigitalOverrideId) -> Result<&DigitalOverride, DigitalEvalError> {
        self.plan
            .overrides
            .get(site.index())
            .filter(|entry| entry.id == site)
            .ok_or_else(|| DigitalEvalError::OverrideUnavailable {
                site,
                detail: "the plan does not declare it".into(),
            })
    }

    /// Execute the statement `site` names: install its override or remove
    /// the ones it names.
    pub(crate) fn apply_override(
        &mut self,
        site: DigitalOverrideId,
    ) -> Result<(), DigitalEvalError> {
        self.override_site(site)?;
        let plan = Arc::clone(&self.plan);
        let entry = &plan.overrides[site.index()];
        match entry.kind {
            ProceduralContinuousKind::Force | ProceduralContinuousKind::Assign => {
                let value = self.evaluate_site(site)?;
                // Subscribed before the install publishes, so a right-hand
                // side that reads its own target is re-evaluated like any
                // other operand change.
                self.overrides.active.insert(site.0);
                if let Some(value) = &entry.value {
                    for signal in &value.dependencies {
                        self.overrides
                            .inputs
                            .entry(*signal)
                            .or_default()
                            .insert(site.0);
                    }
                }
                self.install(entry, value, true)?;
            }
            ProceduralContinuousKind::Release | ProceduralContinuousKind::Deassign => {
                self.remove(entry);
            }
        }
        self.prune_overrides();
        Ok(())
    }

    fn evaluate_site(
        &mut self,
        site: DigitalOverrideId,
    ) -> Result<DigitalScalar, DigitalEvalError> {
        let plan = Arc::clone(&self.plan);
        let mut scratch = std::mem::take(&mut self.overrides.scratch);
        let value = evaluate_override(&plan, site, self, &mut scratch);
        self.overrides.scratch = scratch;
        value
    }

    /// Lay `value` over the site's targets, most significant first. A
    /// statement takes the bits it names; a re-evaluation (`claim` false)
    /// updates only the bits the site still owns.
    fn install(
        &mut self,
        entry: &DigitalOverride,
        value: DigitalScalar,
        claim: bool,
    ) -> Result<(), DigitalEvalError> {
        let site = entry.id.0;
        let total = entry.width();
        let mut offset = total;
        for target in &entry.targets {
            let signal = target.target.signal;
            let index = usize::from(signal);
            let real = self.is_real(signal);
            let width = self.widths[index];
            let slice = match (&value, real) {
                (DigitalScalar::Real(value), true) => OverrideValue::Real(*value),
                (DigitalScalar::FourState(value), false) => {
                    offset -= target.width;
                    let mut slice = FourStateValue::splat(target.width, FourStateBit::Unknown);
                    for bit in 0..target.width {
                        slice.set_bit(bit, value.bit(offset + bit));
                    }
                    OverrideValue::FourState(slice)
                }
                _ => {
                    return Err(DigitalEvalError::OverrideUnavailable {
                        site: entry.id,
                        detail: "its right-hand side has the wrong value domain".into(),
                    });
                }
            };
            let current = if real {
                OverrideValue::Real(self.reals[index])
            } else {
                OverrideValue::FourState(self.values[index].clone())
            };
            let record = self
                .overrides
                .signals
                .entry(signal)
                .or_insert_with(|| SignalOverride {
                    owners: vec![None; if real { 1 } else { width as usize }],
                    forced: current.clone(),
                    assign: None,
                    base: current,
                });
            match entry.kind {
                ProceduralContinuousKind::Assign => {
                    if claim || record.assign.as_ref().is_some_and(|a| a.0 == site) {
                        record.assign = Some((site, slice));
                    }
                }
                _ => match (&mut record.forced, slice) {
                    (OverrideValue::Real(forced), OverrideValue::Real(slice)) => {
                        if claim || record.owners[0] == Some(site) {
                            record.owners[0] = Some(site);
                            *forced = slice;
                        }
                    }
                    (OverrideValue::FourState(forced), OverrideValue::FourState(slice)) => {
                        let range = self.plan.signals[index].declared_range();
                        let low = target.target.select.low_position(range);
                        for bit in 0..target.width {
                            let position = low + i64::from(bit);
                            if position < 0 || position >= i64::from(width) {
                                continue;
                            }
                            let position = position as usize;
                            if claim || record.owners[position] == Some(site) {
                                record.owners[position] = Some(site);
                                forced.set_bit(position as u32, slice.bit(bit));
                            }
                        }
                    }
                    _ => unreachable!("domain matched above"),
                },
            }
            self.refresh_override(signal);
        }
        Ok(())
    }

    /// `release` or `deassign` every target of `entry`.
    fn remove(&mut self, entry: &DigitalOverride) {
        for target in &entry.targets {
            let signal = target.target.signal;
            let index = usize::from(signal);
            let variable = self.variables[index];
            let real = self.is_real(signal);
            let width = self.widths[index];
            let current = if real {
                OverrideValue::Real(self.reals[index])
            } else {
                OverrideValue::FourState(self.values[index].clone())
            };
            let Some(record) = self.overrides.signals.get_mut(&signal) else {
                continue;
            };
            match entry.kind {
                ProceduralContinuousKind::Deassign => {
                    // Section 9.3.1: the variable keeps its value until the
                    // next procedural assignment.
                    if record.assign.take().is_some() {
                        record.base = current;
                    }
                }
                _ => {
                    let range = self.plan.signals[index].declared_range();
                    let (low, count) = if real {
                        (0, 1)
                    } else {
                        (target.target.select.low_position(range), target.width)
                    };
                    let mut released = false;
                    for bit in 0..count {
                        let position = low + i64::from(bit);
                        if position < 0 || position >= i64::from(width.max(1)) {
                            continue;
                        }
                        released |= record.owners[position as usize].take().is_some();
                    }
                    // Section 9.3.2: a released variable keeps the forced
                    // value until the next procedural assignment.
                    if released && variable {
                        record.base = current;
                    }
                }
            }
            self.refresh_override(signal);
            let record = &self.overrides.signals[&signal];
            if record.is_empty() {
                self.overrides.signals.remove(&signal);
            }
        }
    }

    /// Drop every site that no longer owns anything.
    fn prune_overrides(&mut self) {
        let plan = Arc::clone(&self.plan);
        let stale: Vec<u32> = self
            .overrides
            .active
            .iter()
            .copied()
            .filter(|site| {
                let entry = &plan.overrides[*site as usize];
                !entry.targets.iter().any(|target| {
                    self.overrides
                        .signals
                        .get(&target.target.signal)
                        .is_some_and(|record| record.owns(*site))
                })
            })
            .collect();
        for site in stale {
            self.overrides.active.remove(&site);
            self.overrides.pending.remove(&site);
            if let Some(value) = &plan.overrides[site as usize].value {
                for signal in &value.dependencies {
                    if let Some(sites) = self.overrides.inputs.get_mut(signal) {
                        sites.remove(&site);
                        if sites.is_empty() {
                            self.overrides.inputs.remove(signal);
                        }
                    }
                }
            }
        }
    }

    /// Publish a signal's effective value, beneath any admission.
    fn refresh_override(&mut self, signal: DigitalSignalId) {
        let Some(record) = self.overrides.signals.get(&signal) else {
            return;
        };
        match record.effective() {
            OverrideValue::FourState(value) => self.publish_effective(signal, value),
            OverrideValue::Real(value) => self.publish_real_member_unadmitted(signal, value),
        }
    }

    /// A write is arriving at an overridden signal: record it beneath the
    /// override and answer what to publish instead. A variable discards it.
    pub(super) fn admit(
        &mut self,
        signal: DigitalSignalId,
        value: FourStateValue,
    ) -> FourStateValue {
        let index = usize::from(signal);
        let variable = self.variables[index];
        let Some(record) = self.overrides.signals.get_mut(&signal) else {
            return value;
        };
        if variable {
            return self.values[index].clone();
        }
        record.base = OverrideValue::FourState(value);
        match record.effective() {
            OverrideValue::FourState(value) => value,
            OverrideValue::Real(_) => unreachable!("four-state record"),
        }
    }

    /// The real twin of [`Self::admit`].
    pub(super) fn admit_real(&mut self, signal: DigitalSignalId, value: f64) -> f64 {
        let index = usize::from(signal);
        let variable = self.variables[index];
        let Some(record) = self.overrides.signals.get_mut(&signal) else {
            return value;
        };
        if variable {
            return self.reals[index];
        }
        record.base = OverrideValue::Real(value);
        match record.effective() {
            OverrideValue::Real(value) => value,
            OverrideValue::FourState(_) => unreachable!("real record"),
        }
    }

    /// The forced bit of one signal bit, if a `force` owns it.
    pub(super) fn forced_bit(&self, signal: DigitalSignalId, bit: u32) -> Option<FourStateBit> {
        let record = self.overrides.signals.get(&signal)?;
        record.owners.get(bit as usize).copied().flatten()?;
        match &record.forced {
            OverrideValue::FourState(value) => Some(value.bit(bit)),
            OverrideValue::Real(_) => None,
        }
    }

    /// The value a net's driver resolution gives once its forced bits are
    /// laid over it.
    pub(super) fn overlay_forced(
        &self,
        signal: DigitalSignalId,
        mut value: FourStateValue,
    ) -> FourStateValue {
        if let Some(record) = self.overrides.signals.get(&signal)
            && let OverrideValue::FourState(forced) = &record.forced
        {
            for (position, owner) in record.owners.iter().enumerate() {
                if owner.is_some() && (position as u32) < value.width() {
                    value.set_bit(position as u32, forced.bit(position as u32));
                }
            }
        }
        value
    }

    /// The forced real value of a real signal, if a `force` owns it.
    pub(super) fn forced_real(&self, signal: DigitalSignalId) -> Option<f64> {
        let record = self.overrides.signals.get(&signal)?;
        match (&record.forced, record.owners.first().copied().flatten()) {
            (OverrideValue::Real(value), Some(_)) => Some(*value),
            _ => None,
        }
    }

    /// An operand of some active site changed: queue the sites.
    pub(super) fn note_override_input(&mut self, signal: DigitalSignalId) {
        if let Some(sites) = self.overrides.inputs.get(&signal) {
            self.overrides.pending.extend(sites.iter().copied());
        }
    }

    /// Re-evaluate every queued right-hand side until none is queued.
    pub(crate) fn settle_overrides(&mut self) -> Result<(), DigitalEvalError> {
        if let Some(error) = self.overrides.error.take() {
            return Err(error);
        }
        let plan = Arc::clone(&self.plan);
        let mut runs = 0usize;
        while let Some(site) = self.overrides.pending.pop_first() {
            if !self.overrides.active.contains(&site) {
                continue;
            }
            runs += 1;
            let id = DigitalOverrideId(site);
            if runs > OVERRIDE_SETTLE_LIMIT {
                self.overrides.pending.clear();
                return Err(DigitalEvalError::OverrideUnavailable {
                    site: id,
                    detail: format!(
                        "its right-hand side did not settle after {OVERRIDE_SETTLE_LIMIT} \
                         re-evaluations; it depends on what it overrides"
                    ),
                });
            }
            let value = self.evaluate_site(id)?;
            self.install(&plan.overrides[site as usize], value, false)?;
        }
        Ok(())
    }

    /// The override layer for the host image's `forces` section; `None`
    /// when nothing is overridden.
    pub(crate) fn checkpoint_overrides(&self) -> Result<Option<OverridesImage>, String> {
        if !self.overrides.pending.is_empty() || self.overrides.error.is_some() {
            return Err("procedural continuous assignments must be settled".into());
        }
        if self.overrides.is_empty() {
            return Ok(None);
        }
        Ok(Some(OverridesImage {
            signals: self
                .overrides
                .signals
                .iter()
                .map(|(signal, record)| SignalOverrideImage {
                    signal: signal.index(),
                    owners: record.owners.clone(),
                    forced: ValueImage::capture(&record.forced),
                    assign: record
                        .assign
                        .as_ref()
                        .map(|(site, value)| (*site, ValueImage::capture(value))),
                    base: ValueImage::capture(&record.base),
                })
                .collect(),
        }))
    }

    /// Install a `forces` section into a store being restored, before its
    /// resolution is validated. Every owner must be a site of the plan that
    /// names the signal with the kind it is recorded as.
    pub(super) fn restore_overrides(&mut self, image: &OverridesImage) -> Result<(), String> {
        let plan = Arc::clone(&self.plan);
        let mut last = None;
        for record in &image.signals {
            let signal = DigitalSignalId::new(record.signal);
            let declared = plan
                .signal(signal)
                .ok_or("override names an undeclared signal")?;
            if last.is_some_and(|previous| previous >= record.signal) {
                return Err("override records are unordered".into());
            }
            last = Some(record.signal);
            let real = declared.kind.is_real();
            let width = declared.width;
            if record.owners.len() != if real { 1 } else { width as usize } {
                return Err("override ownership width differs from its signal".into());
            }
            let names = |site: u32, kinds: &[ProceduralContinuousKind]| {
                plan.overrides.get(site as usize).is_some_and(|entry| {
                    kinds.contains(&entry.kind)
                        && entry
                            .targets
                            .iter()
                            .any(|target| target.target.signal == signal)
                })
            };
            if record
                .owners
                .iter()
                .flatten()
                .any(|site| !names(*site, &[ProceduralContinuousKind::Force]))
                || record.assign.as_ref().is_some_and(|(site, _)| {
                    !declared.procedurally_assignable
                        || !names(*site, &[ProceduralContinuousKind::Assign])
                })
            {
                return Err("override owner is not a site naming its signal".into());
            }
            let restored = SignalOverride {
                owners: record.owners.clone(),
                forced: record.forced.restore(real, width)?,
                assign: record
                    .assign
                    .as_ref()
                    .map(|(site, value)| value.restore(real, width).map(|value| (*site, value)))
                    .transpose()?,
                base: record.base.restore(real, width)?,
            };
            if restored.is_empty() {
                return Err("an empty override record was saved".into());
            }
            let effective = restored.effective();
            let index = usize::from(signal);
            let matches = match &effective {
                OverrideValue::FourState(value) => !real && self.values[index] == *value,
                OverrideValue::Real(value) => {
                    real && self.reals[index].to_bits() == value.to_bits()
                }
            };
            if !matches {
                return Err("overridden signal value differs from its override".into());
            }
            for site in restored
                .owners
                .iter()
                .flatten()
                .chain(restored.assign.iter().map(|a| &a.0))
            {
                self.overrides.active.insert(*site);
            }
            self.overrides.signals.insert(signal, restored);
        }
        for site in self.overrides.active.clone() {
            if let Some(value) = &plan.overrides[site as usize].value {
                for signal in &value.dependencies {
                    self.overrides
                        .inputs
                        .entry(*signal)
                        .or_default()
                        .insert(site);
                }
            }
        }
        Ok(())
    }
}
