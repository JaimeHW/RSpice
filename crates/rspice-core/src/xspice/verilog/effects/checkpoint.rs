//! The effects section of a host checkpoint image (design P2 section 5.4).
//!
//! Randomness, `$timeformat`, the active `$monitor` and the severity counts
//! are design state like any signal, and a resumed run must continue them
//! exactly. The journal and the monitor region's queue are not saved: a
//! checkpoint is taken at an accepted, settled boundary, where the journal
//! has been released and the queue has drained, and capture refuses
//! otherwise. A finished run cannot be checkpointed at all.

use super::{ActiveMonitor, HdlEffects, SeverityCounts, format::TimeFormat, random::Pcg32};
use serde::{Deserialize, Serialize};

/// Saved effects state. Lane L5 adds its file table as a further field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EffectsImage {
    pub random_seed: i32,
    /// Per process; `None` for a process that has not drawn yet.
    pub urandom: Vec<Option<Pcg32>>,
    pub time_format: TimeFormat,
    pub monitor: Option<ActiveMonitor>,
    pub severities: SeverityCounts,
}

impl EffectsImage {
    pub(crate) fn capture(effects: &HdlEffects) -> Result<Self, String> {
        if effects.finish.is_some() {
            return Err(
                "a run that executed `$finish`, `$stop` or `$fatal` cannot be checkpointed".into(),
            );
        }
        if !effects.journal.is_empty() || !effects.queue.is_empty() {
            return Err("HDL output or the monitor region is not drained".into());
        }
        Ok(Self {
            random_seed: effects.random_seed,
            urandom: effects.urandom.clone(),
            time_format: effects.time_format.clone(),
            monitor: effects.monitor.clone(),
            severities: effects.severities,
        })
    }

    pub(crate) fn restore_into(&self, effects: &mut HdlEffects) -> Result<(), String> {
        if self.urandom.len() > effects.urandom.len() {
            return Err(
                "checkpoint has more `$urandom` streams than the design has processes".into(),
            );
        }
        if !(-15..=0).contains(&self.time_format.units) {
            return Err("checkpoint `$timeformat` units are out of range".into());
        }
        let processes = effects.urandom.len();
        effects.random_seed = self.random_seed;
        effects.urandom = self.urandom.clone();
        effects.urandom.resize(processes, None);
        effects.time_format = self.time_format.clone();
        effects.monitor = self.monitor.clone();
        effects.severities = self.severities;
        effects.journal.clear();
        effects.queue.clear();
        effects.monitor_queued = false;
        effects.finish = None;
        Ok(())
    }
}
