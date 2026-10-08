//! Ordered, speculative analog assignment notifications.
//! Numerical evaluation records counter changes; observation never records them.

use crate::{canonical_ir::CanonicalIrArtifact, codegen::CompiledModel, vm::VmError};
use std::collections::BTreeMap;

/// Compiler identity of one aggregate assignment and its declaration-order member.
/// Repeated execution starts another group when the member order restarts.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AnalogOccurrenceSource {
    pub group: u32,
    pub member: u32,
}

#[cfg(any(feature = "native", feature = "wasm-jit", test))]
impl AnalogOccurrenceSource {
    pub(crate) fn operands(source: Option<Self>) -> [f64; 2] {
        source.map_or([0.0, 0.0], |source| {
            [f64::from(source.group) + 1.0, f64::from(source.member)]
        })
    }

    pub(crate) fn from_operands(group: f64, member: f64) -> Result<Option<Self>, VmError> {
        if !group.is_finite()
            || group.fract() != 0.0
            || !(0.0..=f64::from(u32::MAX) + 1.0).contains(&group)
            || !member.is_finite()
            || member.fract() != 0.0
            || !(0.0..MAX_OCCURRENCES as f64).contains(&member)
            || (group == 0.0 && member != 0.0)
        {
            return Err(VmError::InvalidRuntimeOperation(
                "analog occurrence group is invalid".into(),
            ));
        }
        Ok((group != 0.0).then_some(Self {
            group: (group - 1.0) as u32,
            member: member as u32,
        }))
    }
}

/// One executed assignment, identified by the compiled counter storage slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AnalogAssignmentOccurrence {
    pub variable: usize,
    pub counter: u32,
    pub group: u32,
}

pub(crate) const COUNTER_MASK: u32 = i32::MAX as u32;
const MAX_OCCURRENCES: usize = 65_536;

pub(crate) fn counter_slots(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> Result<Vec<usize>, VmError> {
    use crate::canonical_ir::digital::DigitalAnalogProbeTarget;
    if !artifact
        .digital
        .analog_probes
        .iter()
        .any(|probe| probe.event_signal.is_some())
    {
        return Ok(Vec::new());
    }
    let names: BTreeMap<_, _> = model
        .variable_names
        .iter()
        .enumerate()
        .map(|(slot, name)| (name.as_str(), slot))
        .collect();
    let mut slots = Vec::new();
    for probe in &artifact.digital.analog_probes {
        if probe.event_signal.is_none() {
            continue;
        }
        let DigitalAnalogProbeTarget::Variable { name } = &probe.target else {
            return Err(VmError::InvalidModel(
                "analog event probe has no counter variable".into(),
            ));
        };
        let slot = names.get(name.as_str()).copied().ok_or_else(|| {
            VmError::InvalidModel(format!("analog event counter {name} has no runtime slot"))
        })?;
        slots.push(slot);
    }
    slots.sort_unstable();
    slots.dedup();
    Ok(slots)
}

/// Internal journal type exposed only for the native evaluation-frame ABI.
#[derive(Debug, Clone, Default)]
#[doc(hidden)]
pub struct AnalogOccurrenceJournal {
    counters: BTreeMap<usize, u32>,
    records: Vec<AnalogAssignmentOccurrence>,
    failure: Option<&'static str>,
    last_source: Option<AnalogOccurrenceSource>,
}

impl AnalogOccurrenceJournal {
    pub(crate) fn new(slots: &[usize]) -> Self {
        Self {
            counters: slots.iter().map(|&slot| (slot, 0)).collect(),
            ..Self::default()
        }
    }

    pub(crate) fn invalidate(&mut self) {
        self.failure
            .get_or_insert("analog occurrence evaluation failed");
    }

    pub(crate) fn clear(&mut self) {
        self.records.clear();
        self.failure = None;
        self.last_source = None;
    }

    pub(crate) fn reset(&mut self, variables: &[f64]) {
        self.clear();
        for (&slot, counter) in &mut self.counters {
            match variables
                .get(slot)
                .and_then(|&value| checked_counter(value))
            {
                Some(value) => *counter = value,
                None => {
                    self.failure = Some("analog occurrence origin is not a valid counter");
                }
            }
        }
    }

    pub(crate) fn record(
        &mut self,
        slot: usize,
        value: f64,
        source: Option<AnalogOccurrenceSource>,
    ) -> Result<(), VmError> {
        if let Some(error) = self.failure {
            return Err(VmError::InvalidRuntimeOperation(error.into()));
        }
        let Some(previous) = self.counters.get_mut(&slot) else {
            return Ok(());
        };
        let result = (|| {
            let counter = checked_counter(value).ok_or("analog occurrence counter is invalid")?;
            if counter == *previous {
                return Ok(());
            }
            if counter != previous.wrapping_add(1) & COUNTER_MASK {
                return Err("analog occurrence counter skipped an assignment");
            }
            if self.records.len() == MAX_OCCURRENCES {
                return Err("analog evaluation exceeded the assignment-occurrence limit");
            }
            self.records
                .try_reserve(1)
                .map_err(|_| "could not allocate analog occurrence storage")?;
            let same_group = source
                .zip(self.last_source)
                .is_some_and(|(next, previous)| {
                    next.group == previous.group && next.member > previous.member
                });
            let group = self
                .records
                .last()
                .map_or(0, |last| last.group + u32::from(!same_group));
            self.records.push(AnalogAssignmentOccurrence {
                variable: slot,
                counter,
                group,
            });
            self.last_source = source;
            *previous = counter;
            Ok(())
        })();
        if let Err(error) = result {
            self.failure = Some(error);
            return Err(VmError::InvalidRuntimeOperation(error.into()));
        }
        Ok(())
    }

    pub(crate) fn records(&self) -> Result<&[AnalogAssignmentOccurrence], VmError> {
        if let Some(error) = self.failure {
            return Err(VmError::InvalidRuntimeOperation(error.into()));
        }
        Ok(&self.records)
    }
}

fn checked_counter(value: f64) -> Option<u32> {
    (value.is_finite() && value.fract() == 0.0 && (0.0..=f64::from(COUNTER_MASK)).contains(&value))
        .then_some(value as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_wrap_replay_failure_and_resource_bounds_preserve_the_journal_contract() {
        let mut journal = AnalogOccurrenceJournal::new(&[1, 3]);
        let variables = [0.0, f64::from(COUNTER_MASK), 0.0, 7.0];
        journal.reset(&variables);
        journal.record(1, 0.0, None).unwrap();
        journal.record(3, 8.0, None).unwrap();
        journal.record(1, 0.0, None).unwrap(); // inactive guard, no new assignment
        assert_eq!(
            journal.records().unwrap(),
            &[
                AnalogAssignmentOccurrence {
                    variable: 1,
                    counter: 0,
                    group: 0,
                },
                AnalogAssignmentOccurrence {
                    variable: 3,
                    counter: 8,
                    group: 1,
                },
            ]
        );
        let checkpoint = journal.clone();
        assert!(journal.record(1, 2.0, None).is_err());
        assert!(journal.records().is_err());
        assert_eq!(checkpoint.records().unwrap().len(), 2);
        journal.reset(&variables);
        assert!(journal.records().unwrap().is_empty());
        for count in 0..MAX_OCCURRENCES {
            journal.record(1, count as f64, None).unwrap();
        }
        assert!(journal.record(1, MAX_OCCURRENCES as f64, None).is_err());
        assert!(journal.records().is_err());
        journal.clear();
        assert!(journal.records().unwrap().is_empty());
    }

    #[test]
    fn aggregate_members_restart_on_reexecution_and_scalar_interleaving() {
        let mut journal = AnalogOccurrenceJournal::new(&[0, 1]);
        let source = |group, member| Some(AnalogOccurrenceSource { group, member });
        journal.reset(&[0.0, 0.0]);
        // An unobserved member and an inactive write cannot split an aggregate.
        journal.record(0, 1.0, source(7, 0)).unwrap();
        journal.record(1, 0.0, source(8, 0)).unwrap();
        journal.record(1, 1.0, source(7, 3)).unwrap();
        journal.record(0, 2.0, source(7, 0)).unwrap();
        journal.record(1, 2.0, source(7, 3)).unwrap();
        journal.record(0, 3.0, None).unwrap();
        journal.record(1, 3.0, source(7, 3)).unwrap();
        assert_eq!(
            journal
                .records()
                .unwrap()
                .iter()
                .map(|r| r.group)
                .collect::<Vec<_>>(),
            [0, 0, 1, 1, 2, 3]
        );
        for value in [None, source(0, 0), source(u32::MAX, 65_535)] {
            let [group, member] = AnalogOccurrenceSource::operands(value);
            assert_eq!(
                AnalogOccurrenceSource::from_operands(group, member).unwrap(),
                value
            );
        }
        for (group, member) in [
            (0.0, 1.0),
            (-1.0, 0.0),
            (1.5, 0.0),
            (1.0, 65_536.0),
            (1.0, f64::NAN),
            (f64::INFINITY, 0.0),
        ] {
            assert!(AnalogOccurrenceSource::from_operands(group, member).is_err());
        }
    }
}
