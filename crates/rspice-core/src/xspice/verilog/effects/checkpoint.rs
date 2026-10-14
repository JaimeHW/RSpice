//! The effects section of a host checkpoint image (design P2 section 5.4).
//!
//! Randomness, `$timeformat`, the active `$monitor` and the severity counts
//! are design state like any signal, and a resumed run must continue them
//! exactly. The journal and the monitor region's queue are not saved: a
//! checkpoint is taken at an accepted, settled boundary, where the journal
//! has been released and the queue has drained, and capture refuses
//! otherwise. A finished run cannot be checkpointed at all.
//!
//! # Files
//!
//! The file table rides in the same section: every open handle, the active
//! `$fmonitor`s and the dump. A resumed run reopens each written file and
//! cuts it back to what it held at the checkpoint, so whatever the first run
//! wrote after it is discarded; each read file is read again, and must be
//! what the handle was reading.

use super::dump::{Dump, DumpPhase};
use super::files::HandleImage;
use super::{
    ActiveMonitor, FileMonitor, HdlEffects, SeverityCounts, format::TimeFormat, random::Pcg32,
};
use serde::{Deserialize, Serialize};

/// Saved effects state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EffectsImage {
    pub random_seed: i32,
    /// Per process; `None` for a process that has not drawn yet.
    pub urandom: Vec<Option<Pcg32>>,
    pub time_format: TimeFormat,
    pub monitor: Option<ActiveMonitor>,
    pub severities: SeverityCounts,
    /// Open files, `$fmonitor`s and the dump; absent when there are none.
    #[serde(default, skip_serializing_if = "FilesImage::is_empty")]
    pub files: FilesImage,
}

/// The file table.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FilesImage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handles: Vec<HandleImage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitors: Vec<FileMonitor>,
    #[serde(default)]
    pub next_monitor: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dump: Option<Dump>,
}

impl FilesImage {
    fn is_empty(&self) -> bool {
        self.handles.is_empty()
            && self.monitors.is_empty()
            && self.next_monitor == 0
            && self.dump.is_none()
    }
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
        if matches!(effects.dump.phase, DumpPhase::Selecting(_)) {
            return Err("a `$dumpvars` selection has not been written".into());
        }
        Ok(Self {
            random_seed: effects.random_seed,
            urandom: effects.urandom.clone(),
            time_format: effects.time_format.clone(),
            monitor: effects.monitor.clone(),
            severities: effects.severities,
            files: FilesImage {
                handles: effects.files.capture()?,
                monitors: effects.file_monitors.clone(),
                next_monitor: effects.next_monitor_serial,
                dump: (effects.dump.phase != DumpPhase::Idle || effects.dump.name.is_some())
                    .then(|| effects.dump.clone()),
            },
        })
    }

    pub(crate) fn restore_into(
        &self,
        plan: &rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan,
        effects: &mut HdlEffects,
    ) -> Result<(), String> {
        if self
            .files
            .monitors
            .iter()
            .any(|monitor| monitor.observer as usize >= plan.observers.len())
        {
            return Err("host effects name an undeclared `$fmonitor`".into());
        }
        if self.files.dump.as_ref().is_some_and(|dump| {
            dump.vars
                .iter()
                .any(|var| var.signal as usize >= plan.signals.len())
        }) {
            return Err("the checkpointed dump names an undeclared signal".into());
        }
        if self.urandom.len() > effects.urandom.len() {
            return Err(
                "checkpoint has more `$urandom` streams than the design has processes".into(),
            );
        }
        if !(-15..=0).contains(&self.time_format.units) {
            return Err("checkpoint `$timeformat` units are out of range".into());
        }
        let options = effects.config.files.clone();
        let mut files = super::files::HdlFiles::default();
        files.restore(&options, &self.files.handles)?;
        let mut dump = self.files.dump.clone().unwrap_or_default();
        if let Some(path) = &dump.path {
            if !options.unrestricted && options.resolve_write(&path.to_string_lossy()).is_err() {
                return Err(format!(
                    "the checkpointed dump file {} is outside this run's directory",
                    path.display()
                ));
            }
            files.journal(super::files::FileOp::Truncate {
                path: path.clone(),
                length: dump.written,
            });
        }
        if dump.last.len() != dump.vars.len() && dump.phase == DumpPhase::Dumping {
            return Err("the checkpointed dump's values do not match its variables".into());
        }
        dump.reindex();
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
        effects.files = files;
        effects.file_monitors = self.files.monitors.clone();
        effects.next_monitor_serial = self.files.next_monitor;
        effects.evaluating = None;
        effects.dump = dump;
        Ok(())
    }
}
