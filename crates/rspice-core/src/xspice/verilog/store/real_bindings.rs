//! External real drivers contribute to the authored HDL net resolver.
use super::*;
use crate::xspice::DigitalValue;
use crate::xspice::event_scheduler::EventTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExternalRealDriverId(pub(super) usize);
impl ExternalRealDriverId {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExternalNetChange {
    Bits(DigitalBitChange),
    DriverInput {
        driver: ExternalBitDriverId,
        previous: DigitalValue,
        value: DigitalValue,
        starts_publication: bool,
    },
    Real {
        signal: DigitalSignalId,
        previous: f64,
        value: f64,
        starts_publication: bool,
    },
    /// A receiving driver's net has started being driven by its own drivers:
    /// the receiver's sample no longer stands for it.
    RealDrive {
        driver: ExternalRealDriverId,
        driven: bool,
        starts_publication: bool,
    },
}
impl ExternalNetChange {
    pub(crate) fn starts_publication(self) -> bool {
        match self {
            Self::Bits(change) => change.starts_publication,
            Self::DriverInput {
                starts_publication, ..
            }
            | Self::Real {
                starts_publication, ..
            }
            | Self::RealDrive {
                starts_publication, ..
            } => starts_publication,
        }
    }
    pub(super) fn set_start(&mut self, start: bool) {
        match self {
            Self::Bits(change) => change.starts_publication = start,
            Self::DriverInput {
                starts_publication, ..
            }
            | Self::Real {
                starts_publication, ..
            }
            | Self::RealDrive {
                starts_publication, ..
            } => *starts_publication = start,
        }
    }
}

/// Local signal views retain array coordinates while sharing one resolved net.
/// This immutable topology is shared by trial clones and rebuilt from the plan
/// for reset/restart; driver values remain in the ordinary store.
#[derive(Clone)]
pub(super) struct RealConnections {
    representatives: Vec<DigitalSignalId>,
    members: Vec<Vec<DigitalSignalId>>,
}
impl RealConnections {
    pub(super) fn new(plan: &CanonicalDigitalPlan) -> Self {
        let representatives = plan
            .real_net_representatives()
            .expect("validated real-net aliases");
        let mut members = vec![Vec::new(); plan.signals.len()];
        for (index, representative) in representatives.iter().enumerate() {
            if index == usize::from(*representative) {
                continue;
            }
            let group = &mut members[usize::from(*representative)];
            if group.is_empty() {
                group.push(*representative);
            }
            group.push(DigitalSignalId::from(index));
        }
        Self {
            representatives,
            members,
        }
    }
    pub(super) fn members(&self, signal: DigitalSignalId) -> &[DigitalSignalId] {
        let representative = &self.representatives[usize::from(signal)];
        let members = &self.members[usize::from(*representative)];
        if members.is_empty() {
            // Unconnected signals do not allocate a one-element member vector.
            std::slice::from_ref(representative)
        } else {
            members
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub(super) struct RealTopology {
    pub(super) sources: Vec<(DigitalSignalId, EventTarget)>,
    pub(super) observed: BTreeSet<DigitalSignalId>,
    by_signal: Vec<Vec<usize>>,
    /// The receiving drivers, ascending: a converter's sample of its node,
    /// which stands for the net only while nothing else drives it. Empty for
    /// every net without one, which keeps their topology's fingerprint.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fallbacks: Vec<usize>,
}

impl RealTopology {
    fn is_fallback(&self, slot: usize) -> bool {
        self.fallbacks.binary_search(&slot).is_ok()
    }
}
pub(super) struct ExternalReals {
    pub(super) topology: Arc<RealTopology>,
    values: Vec<f64>,
}
/// Written out so a trial image refills the bank in place; see
/// `host/image.rs`. Both methods destructure, so a new field must be placed.
impl Clone for ExternalReals {
    fn clone(&self) -> Self {
        let Self { topology, values } = self;
        Self {
            topology: Arc::clone(topology),
            values: values.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let Self { topology, values } = self;
        let Self {
            topology: source_topology,
            values: source_values,
        } = source;
        topology.clone_from(source_topology);
        values.clone_from(source_values);
    }
}
impl ExternalReals {
    pub(super) fn checkpoint(&self) -> Result<RealCheckpoint, String> {
        Ok(RealCheckpoint {
            topology: super::checkpoint::fingerprint(self.topology.as_ref())?,
            values: self.values.iter().map(|value| value.to_bits()).collect(),
        })
    }

    pub(super) fn restore_checkpoint(&self, image: &RealCheckpoint) -> Result<Self, String> {
        if image.topology != super::checkpoint::fingerprint(self.topology.as_ref())?
            || image.values.len() != self.values.len()
        {
            return Err("real checkpoint topology or dimensions differ".into());
        }
        Ok(Self {
            topology: Arc::clone(&self.topology),
            values: image
                .values
                .iter()
                .map(|bits| f64::from_bits(*bits))
                .collect(),
        })
    }

    pub(super) fn fresh(&self) -> Self {
        Self {
            topology: Arc::clone(&self.topology),
            values: vec![0.0; self.values.len()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(super) struct RealCheckpoint {
    topology: [u8; 32],
    values: Vec<u64>,
}

impl DigitalSignalStore {
    pub(super) fn real_members<'a>(&'a self, signal: &'a DigitalSignalId) -> &'a [DigitalSignalId] {
        self.connected_reals.as_ref().map_or_else(
            || std::slice::from_ref(signal),
            |topology| topology.members(*signal),
        )
    }

    #[cfg(test)]
    pub(crate) fn attach_external_reals(
        &mut self,
        observed: &[DigitalSignalId],
        drivers: &[(DigitalSignalId, EventTarget)],
    ) -> Result<Vec<ExternalRealDriverId>, String> {
        self.attach_external_reals_with_fallbacks(observed, drivers, &[])
    }

    /// Attach external real drivers, `fallbacks` naming (by index into
    /// `drivers`) the receiving ones: each stands for its net only while no
    /// other driver of the net drives it, so it is never a second driver.
    pub(crate) fn attach_external_reals_with_fallbacks(
        &mut self,
        observed: &[DigitalSignalId],
        drivers: &[(DigitalSignalId, EventTarget)],
        fallbacks: &[usize],
    ) -> Result<Vec<ExternalRealDriverId>, String> {
        if self.external_reals.is_some() {
            return Err("external real participants are already attached".into());
        }
        let mut fallbacks = fallbacks.to_vec();
        fallbacks.sort_unstable();
        fallbacks.dedup();
        if fallbacks.last().is_some_and(|slot| *slot >= drivers.len()) {
            return Err("a receiving real driver is not one of the drivers".into());
        }
        let observed: BTreeSet<_> = observed
            .iter()
            .copied()
            .chain(drivers.iter().map(|(signal, _)| *signal))
            .collect();
        for &signal in &observed {
            let index = usize::from(signal);
            if index >= self.kinds.len() || !self.kinds[index].is_real() || self.variables[index] {
                return Err(format!(
                    "external real connection requires a declared real net, got signal {signal}"
                ));
            }
        }
        let mut identities = BTreeSet::new();
        let mut by_signal = vec![Vec::new(); self.kinds.len()];
        for (slot, (signal, target)) in drivers.iter().enumerate() {
            if !identities.insert(target) {
                return Err(format!(
                    "external real driver {}.{}[{}] is declared twice",
                    target.instance, target.port_name, target.driver_index
                ));
            }
            by_signal[usize::from(*signal)].push(slot);
        }
        for &signal in &observed {
            let index = usize::from(signal);
            let driver_count: usize = self
                .real_members(&signal)
                .iter()
                .map(|member| {
                    let member = usize::from(*member);
                    let external = by_signal[member]
                        .iter()
                        .filter(|slot| fallbacks.binary_search(slot).is_err())
                        .count();
                    self.spans[member].count + external
                })
                .sum();
            if self.kinds[index].resolution() == Some(DigitalRealResolution::Single)
                && driver_count > 1
            {
                return Err(format!(
                    "wreal net '{}' requires one driver; HDL and external outputs declare {}. \
                     Declare it with the resolution that combines them, or give every plain \
                     `wreal` one with `.options wreal_resolution=sum|avg|min|max`",
                    self.plan.signal(signal).unwrap().name,
                    driver_count
                ));
            }
            by_signal[index].sort_unstable_by(|a, b| drivers[*a].1.cmp(&drivers[*b].1));
        }
        self.external_reals = Some(ExternalReals {
            topology: Arc::new(RealTopology {
                sources: drivers.to_vec(),
                observed,
                by_signal,
                fallbacks,
            }),
            values: vec![0.0; drivers.len()],
        });
        Ok((0..drivers.len()).map(ExternalRealDriverId).collect())
    }

    pub(crate) fn external_real_sources(&self) -> &[(DigitalSignalId, EventTarget)] {
        self.external_reals
            .as_ref()
            .map_or(&[], |reals| reals.topology.sources.as_slice())
    }

    pub(crate) fn external_real_contributions(
        &self,
    ) -> impl Iterator<Item = (&EventTarget, f64)> + '_ {
        self.external_reals.iter().flat_map(|reals| {
            reals
                .topology
                .sources
                .iter()
                .zip(&reals.values)
                .map(|((_, target), value)| (target, *value))
        })
    }
    pub(super) fn external_real_values(
        &self,
        signal: DigitalSignalId,
    ) -> impl Iterator<Item = f64> + '_ {
        self.external_reals.iter().flat_map(move |reals| {
            reals.topology.by_signal[usize::from(signal)]
                .iter()
                .map(|&slot| reals.values[slot])
        })
    }

    /// The external values that resolve `signal`: the receiving drivers'
    /// when `receiving`, every other driver's otherwise.
    pub(super) fn resolving_external_reals(
        &self,
        signal: DigitalSignalId,
        receiving: bool,
    ) -> impl Iterator<Item = f64> + '_ {
        self.external_reals.iter().flat_map(move |reals| {
            reals.topology.by_signal[usize::from(signal)]
                .iter()
                .filter(move |&&slot| reals.topology.is_fallback(slot) == receiving)
                .map(|&slot| reals.values[slot])
        })
    }

    /// Whether the net `members` join resolves to its receiving drivers'
    /// sample: it has one, no other external driver, and none of its HDL
    /// drivers has produced a value (LRM 2.4 section 3.7: a driver that has
    /// not run contributes nothing).
    pub(super) fn receives_fallback(&self, members: &[DigitalSignalId]) -> bool {
        let Some(reals) = self.external_reals.as_ref() else {
            return false;
        };
        let topology = &reals.topology;
        if topology.fallbacks.is_empty() {
            return false;
        }
        let slots = || {
            members
                .iter()
                .flat_map(|member| &topology.by_signal[usize::from(*member)])
        };
        let ran = || {
            members.iter().any(|member| {
                let span = self.spans[usize::from(*member)];
                self.contributions[span.start..span.start + span.count]
                    .iter()
                    .any(|contribution| {
                        matches!(contribution.value, ContributionValue::Real(Some(_)))
                    })
            })
        };
        slots().any(|&slot| topology.is_fallback(slot))
            && !slots().any(|&slot| !topology.is_fallback(slot))
            && !ran()
    }

    /// Whether the net the receiving driver `driver` samples into is driven
    /// by its own drivers; `None` for a driver that is not a receiving one.
    pub(crate) fn real_fallback_driven(&self, driver: ExternalRealDriverId) -> Option<bool> {
        let topology = &self.external_reals.as_ref()?.topology;
        if !topology.is_fallback(driver.0) {
            return None;
        }
        let (signal, _) = topology.sources.get(driver.0)?;
        Some(!self.receives_fallback(self.real_members(signal)))
    }

    /// Tell each receiving driver of `signal`'s net that its net is now
    /// driven by its own drivers, after `was_receiving` said it was not.
    pub(super) fn note_real_drive(&mut self, signal: DigitalSignalId, was_receiving: bool) {
        if !was_receiving || self.receives_fallback(self.real_members(&signal)) {
            return;
        }
        let Some(reals) = self.external_reals.as_ref() else {
            return;
        };
        let topology = &reals.topology;
        let receivers: Vec<_> = self
            .real_members(&signal)
            .iter()
            .flat_map(|member| &topology.by_signal[usize::from(*member)])
            .copied()
            .filter(|&slot| topology.is_fallback(slot))
            .collect();
        for slot in receivers {
            self.external_changes.push(ExternalNetChange::RealDrive {
                driver: ExternalRealDriverId(slot),
                driven: true,
                starts_publication: true,
            });
        }
    }
    pub(crate) fn check_external_real_drives(
        &self,
        drives: &[(ExternalRealDriverId, f64)],
    ) -> Result<(), String> {
        let count = self
            .external_reals
            .as_ref()
            .map_or(0, |reals| reals.values.len());
        for (driver, _) in drives {
            if driver.0 >= count {
                return Err(format!("unknown external real driver {}", driver.0));
            }
        }
        Ok(())
    }
    pub(crate) fn take_external_changes(&mut self) -> Vec<ExternalNetChange> {
        std::mem::take(&mut self.external_changes)
    }

    /// Install both value domains before computed event expressions observe any
    /// changed signal. The caller validates identities and scheduler limits first.
    pub(crate) fn publish_external_bank(
        &mut self,
        bits: &[(ExternalBitDriverId, DigitalValue)],
        reals: &[(ExternalRealDriverId, f64)],
    ) {
        assert!(self.external_batch.is_none());
        self.external_batch = Some(Vec::new());
        let start = self.external_changes.len();
        self.publish_external_drives(bits);
        if !reals.is_empty() {
            let external = self
                .external_reals
                .as_mut()
                .expect("validated external real drivers");
            let mut signals = BTreeSet::new();
            for (driver, value) in reals {
                external.values[driver.0] = *value;
                signals.insert(external.topology.sources[driver.0].0);
            }
            for signal in signals {
                let value = self.resolve_real(signal);
                self.publish_real(signal, value);
            }
        }
        for (index, change) in self.external_changes[start..].iter_mut().enumerate() {
            change.set_start(index == 0);
        }
        for (signal, values) in self.external_batch.take().unwrap() {
            self.record_transition(signal, values);
        }
    }
}
