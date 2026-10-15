//! Atomic persistence of an initialized, settled HDL host. The enclosing mixed
//! circuit must also authenticate physical state, participants and source inputs.
use super::super::store::checkpoint::StoreCheckpoint;
use super::*;
use crate::xspice::event_scheduler::{SchedulerCheckpoint, SchedulerCheckpointLimits};
use rspice_veriloga::canonical_ir::digital_eval::checkpoint::{
    DigitalCheckpointLimits, DigitalCheckpointReader, DigitalDeferredCheckpoint,
    DigitalEventCountCheckpoint, DigitalResumeCheckpoint, DigitalWaitCheckpoint,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub(crate) struct HostCheckpointLimits {
    pub scheduler: SchedulerCheckpointLimits,
    pub digital: DigitalCheckpointLimits,
    pub max_processes: usize,
    pub max_updates: usize,
}
impl Default for HostCheckpointLimits {
    fn default() -> Self {
        Self {
            scheduler: Default::default(),
            digital: Default::default(),
            max_processes: 1_048_576,
            max_updates: 4_194_304,
        }
    }
}

/// The host image version this build writes (design P2 section 5.4).
///
/// Version 2 saves threads rather than process slots, and carries
/// self-tagged sections later lanes fill without a further version step.
/// Version 1 images upcast losslessly: one thread with one frame per process,
/// and every section empty.
const HOST_CHECKPOINT_VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HostCheckpoint {
    version: u32,
    plan: [u8; 32],
    tick_seconds: u64,
    scheduler: SchedulerCheckpoint,
    store: StoreCheckpoint,
    /// Version 1's per-process slots; empty in a version 2 image.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    slots: Vec<SlotImage>,
    /// Version 2's threads in slot order: one per process, then the threads
    /// `fork` started, with a free slot kept as a finished thread with no
    /// parent so that slot numbers survive.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    threads: Vec<ThreadImage>,
    delayed: Vec<(u64, Vec<DigitalDeferredCheckpoint>)>,
    events: Vec<EventImage>,
    expressions: Vec<(u64, DigitalDeferredCheckpoint)>,
    /// Randomness, `$timeformat`, the active `$monitor`, severity counts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effects: Option<EffectsSection>,
    /// The `force` and procedural `assign` overrides in effect; absent when
    /// none is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    forces: Option<ForcesSection>,
}

/// The forces section, tagged with its own version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ForcesSection {
    version: u32,
    image: super::super::store::OverridesImage,
}

const FORCES_SECTION_VERSION: u32 = 1;

/// The effects section, tagged with its own version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectsSection {
    version: u32,
    image: super::super::effects::checkpoint::EffectsImage,
}

const EFFECTS_SECTION_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SlotImage {
    status: StatusImage,
    resume: Option<DigitalResumeCheckpoint>,
    after: u64,
    remaining: DigitalEventCountCheckpoint,
}

/// One thread of execution: its process, the thread that forked it, its
/// schedule status, and its frames, outermost first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ThreadImage {
    process: u32,
    parent: Option<u32>,
    status: StatusImage,
    frames: Vec<FrameImage>,
    after: u64,
    remaining: DigitalEventCountCheckpoint,
}

/// What a frame executes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum FrameRoutine {
    Process(u32),
}

/// One activation record: the routine, where it resumes, and its frame
/// slots (none for a process body). A thread inside tasks keeps one frame:
/// its continuation carries the task-enable frames beneath it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FrameImage {
    routine: FrameRoutine,
    resume: Option<DigitalResumeCheckpoint>,
}

impl ThreadImage {
    /// A version 1 slot as the single-frame thread it always was.
    fn from_slot(process: usize, slot: SlotImage) -> Self {
        Self {
            process: process as u32,
            parent: None,
            status: slot.status,
            frames: vec![FrameImage {
                routine: FrameRoutine::Process(process as u32),
                resume: slot.resume,
            }],
            after: slot.after,
            remaining: slot.remaining,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum StatusImage {
    Queued,
    Event(DigitalWaitCheckpoint),
    Expression(u64),
    Finished,
    /// At a `join`, waiting for the threads it forked.
    Join,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EventImage {
    sequence: u64,
    remaining: DigitalEventCountCheckpoint,
    wait: DigitalWaitCheckpoint,
    update: DigitalDeferredCheckpoint,
}

fn check_size(
    processes: usize,
    mut updates: impl Iterator<Item = usize>,
    limits: HostCheckpointLimits,
) -> Result<(), String> {
    if processes > limits.max_processes {
        return Err("host process count exceeds checkpoint limit".into());
    }
    updates
        .try_fold(0usize, |total, count| {
            total
                .checked_add(count)
                .filter(|total| *total <= limits.max_updates)
        })
        .ok_or_else(|| "host update count exceeds checkpoint limit".to_string())?;
    Ok(())
}

impl DigitalHost {
    pub(crate) fn validate_checkpoint_time(&self, accepted: f64) -> Result<(), String> {
        use rspice_veriloga::canonical_ir::digital_eval::DigitalEnvironment;
        if self
            .store
            .read_clock()
            .is_some_and(|clock| clock.absolute_seconds > accepted)
        {
            return Err("digital activation is later than the accepted circuit time".into());
        }
        if let Some(tick) = self.next_tick()
            && self
                .resolution
                .ticks_to_seconds(tick)
                .map_err(|e| e.to_string())?
                <= accepted
        {
            return Err("accepted circuit time has an undelivered digital event".into());
        }
        Ok(())
    }

    fn checkpoint_boundary(&self) -> Result<(), String> {
        if !self.elaboration_closed || self.checkpoint_failed {
            return Err("host must be initialized and successfully settled before capture".into());
        }
        if self.effects.finish().is_some() {
            return Err(
                "a run the design ended with `$finish`, `$stop` or `$fatal` cannot be checkpointed"
                    .into(),
            );
        }
        if !self.inactive.is_empty()
            || !self.analog_waiters.is_empty()
            || self.analog_ready.is_some()
            || self.analog_activation_seconds.is_some()
            || self.analog_exchange_seconds.is_some()
            || self.slots.iter().any(|slot| {
                matches!(
                    slot.status,
                    ProcessStatus::Inactive | ProcessStatus::AwaitingAnalog(_)
                )
            })
        {
            return Err("host has unfinished same-time execution".into());
        }
        Ok(())
    }

    pub(crate) fn checkpoint(
        &self,
        limits: HostCheckpointLimits,
    ) -> Result<HostCheckpoint, String> {
        self.checkpoint_boundary()?;
        check_size(
            self.slots.len(),
            self.delayed_updates
                .values()
                .map(Vec::len)
                .chain([self.event_updates.len(), self.expression_updates.len()]),
            limits,
        )?;
        let scheduler = self
            .scheduler
            .checkpoint(limits.scheduler)
            .map_err(|e| e.to_string())?;
        let store = self.store.checkpoint()?;
        let reader =
            DigitalCheckpointReader::new(&self.plan, limits.digital).map_err(|e| e.to_string())?;
        self.validate_checkpoint_links(&scheduler, &reader)?;
        let threads = self
            .slots
            .iter()
            .map(|slot| ThreadImage {
                process: slot.process as u32,
                parent: slot.parent.map(|parent| parent as u32),
                status: match &slot.status {
                    ProcessStatus::Queued => StatusImage::Queued,
                    ProcessStatus::AwaitingEvent(terms) => {
                        StatusImage::Event(DigitalWaitCheckpoint::capture(
                            &self.plan,
                            &DigitalWaitRequest::Event(terms.clone()),
                        ))
                    }
                    ProcessStatus::AwaitingExpression(token) => StatusImage::Expression(*token),
                    ProcessStatus::AwaitingJoin => StatusImage::Join,
                    ProcessStatus::Finished => StatusImage::Finished,
                    _ => unreachable!("checked drained boundary"),
                },
                frames: vec![FrameImage {
                    routine: FrameRoutine::Process(slot.process as u32),
                    resume: slot.resume.as_ref().map(DigitalResumeCheckpoint::capture),
                }],
                after: slot.wait_after_sequence,
                remaining: DigitalEventCountCheckpoint::capture(&slot.remaining_events),
            })
            .collect();
        let effects = super::super::effects::checkpoint::EffectsImage::capture(&self.effects)?;
        Ok(HostCheckpoint {
            version: HOST_CHECKPOINT_VERSION,
            plan: self.plan.content_identity,
            tick_seconds: self.resolution.seconds_per_tick().to_bits(),
            scheduler,
            store,
            slots: Vec::new(),
            threads,
            effects: Some(EffectsSection {
                version: EFFECTS_SECTION_VERSION,
                image: effects,
            }),
            forces: self
                .store
                .checkpoint_overrides()?
                .map(|image| ForcesSection {
                    version: FORCES_SECTION_VERSION,
                    image,
                }),
            delayed: self
                .delayed_updates
                .iter()
                .map(|(tick, updates)| {
                    (
                        *tick,
                        updates
                            .iter()
                            .map(|u| DigitalDeferredCheckpoint::capture(&self.plan, u))
                            .collect(),
                    )
                })
                .collect(),
            events: self
                .event_updates
                .values()
                .map(|event| EventImage {
                    sequence: event.sequence,
                    remaining: DigitalEventCountCheckpoint::capture(&event.remaining),
                    wait: DigitalWaitCheckpoint::capture(
                        &self.plan,
                        &DigitalWaitRequest::Event(event.terms.clone()),
                    ),
                    update: DigitalDeferredCheckpoint::capture(&self.plan, &event.update),
                })
                .collect(),
            expressions: self
                .expression_updates
                .iter()
                .map(|(token, update)| {
                    (
                        *token,
                        DigitalDeferredCheckpoint::capture(&self.plan, update),
                    )
                })
                .collect(),
        })
    }

    /// All decoding, ownership and scheduling checks precede the only live swap.
    /// Input bytes must be bounded by the enclosing circuit file reader.
    #[cfg(test)]
    pub(crate) fn restore_checkpoint(
        &mut self,
        image: &HostCheckpoint,
        limits: HostCheckpointLimits,
    ) -> Result<(), String> {
        let restored = self.restored_checkpoint(image, limits)?;
        *self = restored;
        Ok(())
    }

    pub(crate) fn restored_checkpoint(
        &self,
        image: &HostCheckpoint,
        limits: HostCheckpointLimits,
    ) -> Result<Self, String> {
        if !(1..=HOST_CHECKPOINT_VERSION).contains(&image.version)
            || image.plan != self.plan.content_identity
            || image.tick_seconds != self.resolution.seconds_per_tick().to_bits()
        {
            return Err("host checkpoint schema, plan or timing differs".into());
        }
        // A version 1 image is a version 2 image with one single-frame
        // thread per process and no sections.
        let threads: Vec<ThreadImage> = match image.version {
            1 => {
                if !image.threads.is_empty() || image.effects.is_some() {
                    return Err("a version 1 host image carries version 2 fields".into());
                }
                image
                    .slots
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, slot)| ThreadImage::from_slot(index, slot))
                    .collect()
            }
            _ => {
                if !image.slots.is_empty() {
                    return Err("a version 2 host image carries version 1 slots".into());
                }
                image.threads.clone()
            }
        };
        check_size(
            threads.len(),
            image
                .delayed
                .iter()
                .map(|(_, updates)| updates.len())
                .chain([image.events.len(), image.expressions.len()]),
            limits,
        )?;
        let processes = self.plan.processes.len();
        if threads.len() < processes {
            return Err("host process dimensions differ".into());
        }
        let mut slots = Vec::with_capacity(threads.len());
        let mut lineage = Vec::with_capacity(threads.len());
        for (index, thread) in threads.into_iter().enumerate() {
            // Every thread has one frame, running its process; the frames of
            // the tasks it is inside live in that frame's continuation. A
            // process's own thread sits in its process's slot with no parent.
            let [frame] = thread.frames.as_slice() else {
                return Err("host thread does not have exactly one frame".into());
            };
            let process = thread.process as usize;
            let root = index < processes;
            if frame.routine != FrameRoutine::Process(thread.process)
                || process >= processes
                || (root && (process != index || thread.parent.is_some()))
            {
                return Err("host thread belongs to another process or routine".into());
            }
            lineage.push((process, thread.parent.map(|parent| parent as usize)));
            slots.push(SlotImage {
                status: thread.status,
                resume: frame.resume.clone(),
                after: thread.after,
                remaining: thread.remaining,
            });
        }
        let mut reader =
            DigitalCheckpointReader::new(&self.plan, limits.digital).map_err(|e| e.to_string())?;
        let mut restored = self.fresh();
        // Preserve the receiving registration order, including interleaved real
        // and bit participants. No old queued event or accounting is copied.
        // Forked threads' targets come after every other, so the receiver's
        // own are left behind and the image's are interned again in slot
        // order: the kernel then refuses an image whose targets differ.
        let base_targets = self.targets[..processes]
            .iter()
            .chain([&self.nba_target])
            .chain(&self.external_targets)
            .chain(&self.external_real_targets)
            .map(|target| usize::from(*target) + 1)
            .max()
            .unwrap_or(0);
        restored.scheduler = self.scheduler.checkpoint_template(base_targets);
        restored.targets = self.targets[..processes].to_vec();
        restored.nba_target = self.nba_target;
        restored.external_targets.clone_from(&self.external_targets);
        restored
            .external_real_targets
            .clone_from(&self.external_real_targets);
        restored.process_of_target = self
            .process_of_target
            .iter()
            .copied()
            .take(base_targets)
            .collect();
        for (index, (process, _)) in lineage.iter().enumerate().skip(processes) {
            restored.intern_thread_target(*process, index);
        }
        restored
            .scheduler
            .restore_checkpoint(&image.scheduler, limits.scheduler)
            .map_err(|e| e.to_string())?;
        let forces = match &image.forces {
            Some(section) if section.version != FORCES_SECTION_VERSION => {
                return Err("host forces section version differs".into());
            }
            Some(section) => Some(&section.image),
            None => None,
        };
        restored.store = self
            .store
            .restored_checkpoint(&image.store, forces, &mut reader)?;
        restored.elaboration_closed = true;
        if let Some(section) = &image.effects {
            if section.version != EFFECTS_SECTION_VERSION {
                return Err("host effects section version differs".into());
            }
            if section
                .image
                .monitor
                .as_ref()
                .is_some_and(|monitor| monitor.observer as usize >= self.plan.observers.len())
            {
                return Err("host effects name an undeclared `$monitor`".into());
            }
            restored.effects.restore_state(&self.plan, &section.image)?;
        }
        for (index, image) in slots.iter().enumerate() {
            let (process, parent) = lineage[index];
            let resume = image
                .resume
                .as_ref()
                .map(|image| reader.restore_thread_resume(image, parent.is_some()))
                .transpose()
                .map_err(|e| e.to_string())?;
            let remaining_events = reader
                .restore_count(&image.remaining)
                .map_err(|e| e.to_string())?;
            let status = match &image.status {
                StatusImage::Queued => ProcessStatus::Queued,
                StatusImage::Finished => ProcessStatus::Finished,
                StatusImage::Event(image) => {
                    let DigitalWaitRequest::Event(terms) =
                        reader.restore_wait(image).map_err(|e| e.to_string())?
                    else {
                        return Err("process event subscription has the wrong domain".into());
                    };
                    restored.subscribe(index, &terms);
                    ProcessStatus::AwaitingEvent(terms)
                }
                StatusImage::Expression(token) => {
                    if restored
                        .expression_processes
                        .insert(*token, index)
                        .is_some()
                    {
                        return Err("expression subscription has multiple processes".into());
                    }
                    ProcessStatus::AwaitingExpression(*token)
                }
                StatusImage::Join => ProcessStatus::AwaitingJoin,
            };
            let slot = ProcessSlot {
                status,
                resume,
                parent,
                remaining_events,
                wait_after_sequence: image.after,
                ..ProcessSlot::start(process)
            };
            if index < processes {
                restored.slots[index] = slot;
            } else {
                restored.slots.push(slot);
            }
        }
        // A thread's count of live children follows from their parents; a
        // forked slot with no parent is free.
        for index in processes..restored.slots.len() {
            match restored.slots[index].parent {
                Some(parent) => {
                    let Some(forker) = restored.slots.get_mut(parent) else {
                        return Err("forked thread names an unknown parent".into());
                    };
                    forker.children = forker
                        .children
                        .checked_add(1)
                        .ok_or_else(|| "forked thread count overflows".to_string())?;
                }
                None => {
                    restored.free_threads.insert(index);
                }
            }
        }
        let mut previous = None;
        for (tick, images) in &image.delayed {
            if previous.is_some_and(|previous| previous >= *tick) || images.is_empty() {
                return Err("unordered or empty delayed update bucket".into());
            }
            restored.instant_of(*tick).map_err(|e| e.to_string())?;
            let mut updates = Vec::with_capacity(images.len());
            for image in images {
                updates.push(reader.restore_deferred(image).map_err(|e| e.to_string())?);
            }
            restored.delayed_updates.insert(*tick, updates);
            previous = Some(*tick);
        }
        let mut previous = None;
        for image in &image.events {
            if previous.is_some_and(|previous| previous >= image.sequence) {
                return Err("unordered event capture identities".into());
            }
            let DigitalWaitRequest::Event(terms) = reader
                .restore_wait(&image.wait)
                .map_err(|e| e.to_string())?
            else {
                return Err("event capture has the wrong subscription domain".into());
            };
            let remaining = reader
                .restore_count(&image.remaining)
                .map_err(|e| e.to_string())?;
            let update = reader
                .restore_deferred(&image.update)
                .map_err(|e| e.to_string())?;
            for term in &terms {
                restored.event_waiters[usize::from(term.signal)].insert(image.sequence);
            }
            restored.event_updates.insert(
                image.sequence,
                EventCapture {
                    sequence: image.sequence,
                    remaining,
                    terms,
                    update,
                },
            );
            previous = Some(image.sequence);
        }
        let mut previous = None;
        for (token, image) in &image.expressions {
            if previous.is_some_and(|previous| previous >= *token) {
                return Err("unordered expression update identities".into());
            }
            let update = reader.restore_deferred(image).map_err(|e| e.to_string())?;
            restored.expression_updates.insert(*token, update);
            previous = Some(*token);
        }
        restored.validate_checkpoint_links(&image.scheduler, &reader)?;
        Ok(restored)
    }

    fn validate_checkpoint_links(
        &self,
        scheduler: &SchedulerCheckpoint,
        reader: &DigitalCheckpointReader<'_>,
    ) -> Result<(), String> {
        let sequence = self
            .store
            .current_sequence()
            .ok_or_else(|| "exhausted store sequence".to_string())?;
        self.validate_thread_lineage()?;
        let mut expression_owners = BTreeSet::new();
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.wait_after_sequence > sequence
                || slot
                    .resume
                    .as_ref()
                    .is_some_and(|resume| usize::from(resume.process()) != slot.process)
            {
                return Err("process frame or event barrier belongs to another execution".into());
            }
            let require_frame = || {
                slot.resume
                    .as_ref()
                    .ok_or_else(|| "suspended process has no continuation".to_string())
            };
            match &slot.status {
                ProcessStatus::Queued => reader
                    .validate_timed_resume(require_frame()?)
                    .map_err(|e| e.to_string())?,
                ProcessStatus::AwaitingEvent(terms) => {
                    let repeated = reader
                        .validate_event_resume(
                            require_frame()?,
                            &DigitalWaitRequest::Event(terms.clone()),
                        )
                        .map_err(|e| e.to_string())?;
                    check_remaining(&slot.remaining_events, repeated)?;
                    if self.plan.processes[slot.process].kind
                        == DigitalProcessKind::ContinuousAssign
                        && slot.wait_after_sequence != 0
                    {
                        return Err("continuous driver has a procedural event barrier".into());
                    }
                }
                ProcessStatus::AwaitingExpression(token) => {
                    let (wait, remaining) = self
                        .store
                        .checkpoint_expression(*token)
                        .ok_or_else(|| "process subscription is absent from store".to_string())?;
                    if self.expression_processes.get(token) != Some(&index)
                        || !expression_owners.insert(*token)
                    {
                        return Err("process expression ownership differs".into());
                    }
                    let repeated = reader
                        .validate_event_resume(
                            require_frame()?,
                            &DigitalWaitRequest::Expressions(wait.clone()),
                        )
                        .map_err(|e| e.to_string())?;
                    check_remaining(remaining, repeated)?;
                }
                ProcessStatus::AwaitingJoin => reader
                    .validate_join_resume(require_frame()?)
                    .map_err(|e| e.to_string())?,
                ProcessStatus::Finished => {
                    if slot.parent.is_none()
                        && index < self.plan.processes.len()
                        && self.plan.processes[index].kind == DigitalProcessKind::Always
                    {
                        return Err("always process is marked finished".into());
                    }
                    if slot.resume.is_some() {
                        return Err("finished thread keeps a continuation".into());
                    }
                }
                _ => return Err("host has unfinished same-time execution".into()),
            }
        }
        if expression_owners.len() != self.expression_processes.len() {
            return Err("orphan process expression ownership".into());
        }
        for (token, update) in &self.expression_updates {
            if self.store.checkpoint_expression(*token).is_none()
                || !expression_owners.insert(*token)
                || update.wait.is_some()
            {
                return Err("expression update ownership or capture differs".into());
            }
        }
        if !self
            .store
            .checkpoint_expression_tokens()
            .eq(expression_owners.iter().copied())
        {
            return Err("orphan store expression subscription".into());
        }
        for (token, event) in &self.event_updates {
            if *token == 0
                || *token != event.sequence
                || *token > sequence
                || expression_owners.contains(token)
                || event.remaining.is_zero()
                || event.update.wait.is_some()
            {
                return Err("event capture identity, count or ownership differs".into());
            }
        }
        for updates in self.delayed_updates.values() {
            if updates.is_empty() || updates.iter().any(|update| !matches!(update.wait, Some(DigitalWaitRequest::Delay(delay)) if delay > 0)) {
                return Err("delayed bucket contains an invalid timed capture".into());
            }
        }

        let processes: BTreeMap<_, _> = self
            .targets
            .iter()
            .copied()
            .enumerate()
            .map(|(index, target)| (target, index))
            .collect();
        let known: BTreeSet<_> = self
            .targets
            .iter()
            .chain([&self.nba_target])
            .chain(&self.external_targets)
            .chain(&self.external_real_targets)
            .copied()
            .collect();
        if known.len() != scheduler.target_count()
            || processes.len() != self.slots.len()
            || processes.contains_key(&self.nba_target)
        {
            return Err("scheduler target registration differs from the host".into());
        }
        let mut activations = vec![0usize; self.slots.len()];
        let mut buckets = BTreeSet::new();
        for (at, region, target, value) in scheduler.live_events() {
            if region != SchedulerRegion::Active
                || value != EventValue::Digital(DigitalValue::default())
            {
                return Err("host wakeup region or payload differs".into());
            }
            let tick = self.tick_of(at).map_err(|e| e.to_string())?;
            if self.instant_of(tick).map_err(|e| e.to_string())? != at {
                return Err("host wakeup is not on its design grid".into());
            }
            if target == self.nba_target {
                if !self.delayed_updates.contains_key(&tick) || !buckets.insert(tick) {
                    return Err("NBA wakeup has no unique delayed bucket".into());
                }
            } else if let Some(index) = processes.get(&target) {
                activations[*index] += 1;
                if activations[*index] != 1 {
                    return Err("process has multiple pending wakeups".into());
                }
            } else {
                return Err("host scheduler contains an unowned wakeup".into());
            }
        }
        if buckets.len() != self.delayed_updates.len() {
            return Err("delayed update bucket has no wakeup".into());
        }
        for (slot, count) in self.slots.iter().zip(activations) {
            if count != usize::from(matches!(slot.status, ProcessStatus::Queued)) {
                return Err("queued process and scheduled wakeup disagree".into());
            }
        }
        Ok(())
    }
}

impl DigitalHost {
    /// A forked thread hangs off a live thread of its own process that waits
    /// at a `join` for exactly its live children, and every chain of parents
    /// ends at a process's own thread. A free slot is a finished forked slot
    /// with no parent, and nothing else is.
    fn validate_thread_lineage(&self) -> Result<(), String> {
        let processes = self.plan.processes.len();
        if self.slots.len() < processes
            || self
                .free_threads
                .iter()
                .any(|index| *index < processes || *index >= self.slots.len())
        {
            return Err("thread slots and free slots differ".into());
        }
        let mut children = vec![0u32; self.slots.len()];
        for (index, slot) in self.slots.iter().enumerate() {
            let Some(parent) = slot.parent else {
                let free = index >= processes;
                if free != self.free_threads.contains(&index)
                    || (free
                        && (slot.status != ProcessStatus::Finished
                            || slot.resume.is_some()
                            || slot.children != 0))
                    || (!free && slot.process != index)
                {
                    return Err("free thread slots differ".into());
                }
                continue;
            };
            let Some(forker) = self
                .slots
                .get(parent)
                .filter(|_| index >= processes && parent != index)
            else {
                return Err("forked thread names an unknown parent".into());
            };
            if forker.process != slot.process
                || forker.status != ProcessStatus::AwaitingJoin
                || slot.status == ProcessStatus::Finished
                || self.free_threads.contains(&index)
            {
                return Err("forked thread and its parent disagree".into());
            }
            children[parent] = children[parent].saturating_add(1);
            let mut ancestor = parent;
            let mut steps = 0;
            while let Some(next) = self.slots[ancestor].parent {
                steps += 1;
                if steps > self.slots.len() {
                    return Err("forked threads form a cycle".into());
                }
                ancestor = next;
            }
            if ancestor >= processes {
                return Err("forked thread descends from no process".into());
            }
        }
        for (slot, count) in self.slots.iter().zip(children) {
            if slot.children != count || (slot.status == ProcessStatus::AwaitingJoin) != (count > 0)
            {
                return Err("thread waits at a join for other threads than it forked".into());
            }
        }
        Ok(())
    }
}

fn check_remaining(count: &DigitalEventCount, repeated: bool) -> Result<(), String> {
    if count.is_zero() || (!repeated && count != &DigitalEventCount::one()) {
        return Err("process event count differs from its source wait".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    const SOURCE: &str = r#"
`timescale 1ns/1ns
module resumed;
reg clk,a,b,expr_done; reg [7:0] data,timed,event_q,expr_q,out,prefix;
wire [7:0] bus; real rv,rout;
assign bus=out;
initial begin
  clk=0; a=0; b=1; data=8'h42; prefix=1; out=0; rv=-0.0;
  timed<=#5 data;
  event_q<=repeat(3) @(posedge clk) data;
  expr_q<=@(posedge (a & b)) data;
  out = #7 data;
  #1; rout=rv;
end
initial begin expr_done=0; @(a ^ b) expr_done=1; end
endmodule
"#;
    fn host(source: &str) -> DigitalHost {
        let plan = rspice_veriloga::VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap()
            .digital;
        DigitalHost::new(
            &plan,
            TimeResolution::new(-9).unwrap(),
            SchedulerLimits::default(),
        )
    }
    fn signal(host: &DigitalHost, name: &str) -> DigitalSignalId {
        host.signal(name).unwrap()
    }
    fn force(host: &mut DigitalHost, name: &str, width: u32, value: u64, tick: u64) {
        host.force(
            signal(host, name),
            FourStateValue::from_u64(width, value),
            tick,
        )
        .unwrap();
    }
    fn image(host: &DigitalHost) -> HostCheckpoint {
        host.checkpoint(HostCheckpointLimits::default()).unwrap()
    }
    fn transport(image: &HostCheckpoint) -> HostCheckpoint {
        serde_json::from_slice(&serde_json::to_vec(image).unwrap()).unwrap()
    }
    fn prepared() -> DigitalHost {
        let mut host = host(SOURCE);
        host.start().unwrap();
        force(&mut host, "clk", 1, 1, 1);
        force(&mut host, "clk", 1, 0, 1);
        force(&mut host, "data", 8, 0x99, 1);
        force(&mut host, "prefix", 8, 100, 1);
        host
    }
    fn finish(host: &mut DigitalHost) {
        force(host, "clk", 1, 1, 2);
        force(host, "clk", 1, 0, 2);
        force(host, "a", 1, 1, 3);
        force(host, "clk", 1, 1, 4);
        host.advance_to(8).unwrap();
        for name in ["timed", "event_q", "expr_q", "out", "bus"] {
            assert_eq!(
                host.read(signal(host, name)).unwrap().to_u64(),
                Some(0x42),
                "{name}"
            );
        }
        assert_eq!(
            host.read(signal(host, "prefix")).unwrap().to_u64(),
            Some(100)
        );
        assert_eq!(
            host.read(signal(host, "expr_done")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(
            host.read_real(signal(host, "rout")).unwrap().to_bits(),
            (-0.0f64).to_bits()
        );
    }

    #[test]
    fn host_checkpoint_resumes_timers_captured_updates_and_repeated_expression_events() {
        let mut original = prepared();
        let saved = transport(&image(&original));
        let mut restored = host(SOURCE);
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        finish(&mut original);
        finish(&mut restored);
        assert_eq!(image(&restored), image(&original));
    }

    /// Design P2 section 5.4: a version 1 image (per-process slots, no
    /// sections) restores as the version 2 image of the same state.
    #[test]
    fn a_version_one_host_image_upcasts_to_single_frame_threads() {
        let original = prepared();
        let saved = image(&original);
        let mut value = serde_json::to_value(&saved).unwrap();
        let threads = value["threads"].as_array().unwrap().clone();
        let slots: Vec<Value> = threads
            .iter()
            .map(|thread| {
                json!({
                    "status": thread["status"],
                    "resume": thread["frames"][0]["resume"],
                    "after": thread["after"],
                    "remaining": thread["remaining"],
                })
            })
            .collect();
        let object = value.as_object_mut().unwrap();
        object.remove("threads");
        object.remove("effects");
        object.insert("slots".into(), Value::Array(slots));
        object.insert("version".into(), json!(1));
        let version_one: HostCheckpoint = serde_json::from_value(value).unwrap();
        let mut restored = host(SOURCE);
        restored
            .restore_checkpoint(&version_one, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
    }

    const THREADS: &str = r#"`timescale 1ns/1ns
module th;
  reg [7:0] a, b, c, t, u, w;
  reg go;
  event e;
  task automatic bump(inout [7:0] v, input [7:0] by);
    begin #2 v = v + by; end
  endtask
  task stat;
    begin
      fork
        #3 u = u + 1;
        begin @(e) u = u + 8'h10; end
      join
      #1 u = u + 2;
    end
  endtask
  initial begin
    a = 0; b = 0; c = 0; t = 0; u = 0; w = 0; go = 0;
    fork
      begin #2 a = 1; #4 a = 2; end
      begin @(posedge go) b = 5; #1 b = 6; end
      begin
        fork
          #3 c = 7;
          #5 c = c + 1;
        join
        c = c + 10;
      end
    join
    w = 1;
  end
  initial begin
    bump(t, 3);
    bump(t, 4);
  end
  initial begin
    #1 stat;
  end
  initial begin
    #4 go = 1;
    #1 -> e;
  end
endmodule
"#;

    /// Design P2 section 5.4: threads checkpoint mid-fork and mid-task. At
    /// tick 1 the first process waits at a join for three children, one of
    /// them at a join of its own; the second sleeps inside an automatic task;
    /// the third waits at a join inside a static task, whose two children run
    /// in that task's frame. A host restored at any tick resumes every thread
    /// exactly: Icarus 13 ends at a=2 b=6 c=18 t=7 u=19 w=1.
    #[test]
    fn host_checkpoint_resumes_threads_mid_fork_and_mid_task() {
        let mut original = host(THREADS);
        original.start().unwrap();
        for tick in [1, 3, 4, 5] {
            original.advance_to(tick).unwrap();
            let saved = transport(&image(&original));
            let mut restored = host(THREADS);
            restored
                .restore_checkpoint(&saved, HostCheckpointLimits::default())
                .unwrap();
            assert_eq!(image(&restored), saved, "tick {tick}");
            let mut reference = original.clone();
            reference.advance_to(20).unwrap();
            restored.advance_to(20).unwrap();
            assert_eq!(image(&restored), image(&reference), "tick {tick}");
            for (name, value) in [("a", 2), ("b", 6), ("c", 18), ("t", 7), ("u", 19), ("w", 1)] {
                assert_eq!(
                    restored.read(signal(&restored, name)).unwrap().to_u64(),
                    Some(value),
                    "{name} after a restore at tick {tick}"
                );
            }
        }
        // The first image really held forked threads, joins and task frames.
        let mut early = host(THREADS);
        early.start().unwrap();
        early.advance_to(1).unwrap();
        let value = serde_json::to_value(image(&early)).unwrap();
        let threads = value["threads"].as_array().unwrap();
        assert!(threads.len() > early.plan.processes.len() + 4);
        assert_eq!(
            threads
                .iter()
                .filter(|thread| thread["status"] == "Join")
                .count(),
            3
        );
        let in_task = |thread: &Value| {
            !thread["frames"][0]["resume"]["callers"]
                .as_array()
                .unwrap()
                .is_empty()
        };
        assert!(in_task(&threads[1]) && in_task(&threads[2]));
    }

    /// A forked thread whose parent, process, status or kernel target does
    /// not fit the rest of the image is refused, and the receiver is left as
    /// it was.
    #[test]
    fn host_checkpoint_rejects_threads_that_do_not_fit_their_lineage() {
        let mut receiver = host(THREADS);
        receiver.start().unwrap();
        receiver.advance_to(1).unwrap();
        let before = image(&receiver);
        let base = serde_json::to_value(&before).unwrap();
        let processes = receiver.plan.processes.len();
        let child = processes;
        let mut bad: Vec<Value> = Vec::new();
        let mut value = base.clone();
        value["threads"][child]["parent"] = json!(3);
        bad.push(value);
        let mut value = base.clone();
        value["threads"][child]["process"] = json!(1);
        bad.push(value);
        let mut value = base.clone();
        value["threads"][child]["status"] = json!("Finished");
        value["threads"][child]["frames"][0]["resume"] = Value::Null;
        bad.push(value);
        let mut value = base.clone();
        value["threads"].as_array_mut().unwrap().pop();
        bad.push(value);
        let mut value = base.clone();
        value["threads"][child]["parent"] = json!(child);
        bad.push(value);
        let mut value = base.clone();
        let targets = value["scheduler"]["targets"].as_array_mut().unwrap();
        let last = targets.len() - 1;
        targets[last]["instance"] = json!("initial#0 thread 99");
        bad.push(value);
        for (case, value) in bad.into_iter().enumerate() {
            let bad: HostCheckpoint = serde_json::from_value(value).unwrap();
            assert!(
                receiver
                    .restore_checkpoint(&bad, HostCheckpointLimits::default())
                    .is_err(),
                "case {case}"
            );
            assert_eq!(image(&receiver), before, "case {case} changed receiver");
        }
    }

    /// `$random`'s global seed and a process's `$urandom` stream are design
    /// state: a resumed run continues them exactly.
    #[test]
    fn host_checkpoint_continues_random_streams() {
        let source = "`timescale 1ns/1ns\nmodule rng; integer a, b; initial begin a = $random; b = $urandom; #5; a = $random; b = $urandom; end endmodule";
        let mut original = host(source);
        original.start().unwrap();
        let saved = transport(&image(&original));
        let mut restored = host(source);
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        original.advance_to(5).unwrap();
        restored.advance_to(5).unwrap();
        for name in ["a", "b"] {
            assert_eq!(
                restored.read(signal(&restored, name)),
                original.read(signal(&original, name)),
                "{name}"
            );
        }
    }

    #[test]
    fn host_checkpoint_preserves_an_analog_clock_with_an_undelivered_same_tick_timer() {
        let source = "`timescale 1ns/1ns\nmodule causal; reg a,q; real observed; initial begin a=0; q=0; #1; q=1; end always @(a) observed=$abstime; endmodule";
        let mut original = host(source);
        original.start().unwrap();
        original
            .force_many_from_analog_ordered(
                &[(signal(&original, "a"), FourStateValue::from_u64(1, 1))],
                1,
                0.4e-9,
                0.6e-9,
            )
            .unwrap();
        assert_eq!(
            original.read(signal(&original, "q")).unwrap().to_u64(),
            Some(0)
        );
        assert_eq!(
            original.read_real(signal(&original, "observed")),
            Some(0.4e-9)
        );
        let saved = transport(&image(&original));
        let mut restored = host(source);
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        original.advance_to(1).unwrap();
        restored.advance_to(1).unwrap();
        assert_eq!(
            restored.read(signal(&restored, "q")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(image(&restored), image(&original));
    }

    #[test]
    fn host_checkpoint_rejects_orphaned_frames_wakeups_and_subscriptions_atomically() {
        let mut receiver = prepared();
        let before = image(&receiver);
        let base = serde_json::to_value(&before).unwrap();
        let queued = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::Queued))
            .unwrap();
        let event = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::AwaitingEvent(_)))
            .unwrap();
        let expression = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::AwaitingExpression(_)))
            .unwrap();
        let mut bad: Vec<Value> = Vec::new();
        let mut value = base.clone();
        value["scheduler"]["events"].as_array_mut().unwrap().pop();
        bad.push(value);
        let mut value = base.clone();
        value["threads"][queued]["status"] = json!("Finished");
        bad.push(value);
        let mut value = base.clone();
        value["threads"][queued]["frames"][0]["resume"] = Value::Null;
        bad.push(value);
        let mut value = base.clone();
        value["threads"][event]["status"]["Event"]["base"]["Event"][0]["signal"] =
            json!(signal(&receiver, "data").index());
        bad.push(value);
        let mut value = base.clone();
        value["threads"][queued]["status"] = value["threads"][expression]["status"].clone();
        bad.push(value);
        // A process's own thread that names another process or a parent, and
        // a second frame (task frames live in the continuation), are refused.
        let mut value = base.clone();
        value["threads"][queued]["process"] = json!(event);
        bad.push(value);
        let mut value = base.clone();
        value["threads"][queued]["parent"] = json!(0);
        bad.push(value);
        let mut value = base.clone();
        let frame = value["threads"][queued]["frames"][0].clone();
        value["threads"][queued]["frames"]
            .as_array_mut()
            .unwrap()
            .push(frame);
        bad.push(value);
        let mut value = base.clone();
        value["store"]["expressions"].as_array_mut().unwrap().pop();
        bad.push(value);
        let mut value = base.clone();
        value["delayed"] = json!([]);
        bad.push(value);
        let mut value = base.clone();
        value["threads"][event]["after"] = json!(u64::MAX);
        bad.push(value);
        let mut value = base.clone();
        value["tick_seconds"] = json!(0);
        bad.push(value);
        let mut value = base.clone();
        value["events"][0]["remaining"]["aval"] = json!([0]);
        bad.push(value);
        for (case, value) in bad.into_iter().enumerate() {
            let bad: HostCheckpoint = serde_json::from_value(value).unwrap();
            assert!(
                receiver
                    .restore_checkpoint(&bad, HostCheckpointLimits::default())
                    .is_err(),
                "case {case}"
            );
            assert_eq!(image(&receiver), before, "case {case} changed receiver");
        }
        assert!(
            receiver
                .restore_checkpoint(
                    &before,
                    HostCheckpointLimits {
                        max_updates: 0,
                        ..Default::default()
                    }
                )
                .is_err()
        );
        assert_eq!(image(&receiver), before);
    }

    #[test]
    fn host_checkpoint_refuses_failed_settlement_and_recovers_only_through_valid_restore() {
        let mut receiver = host("module done; reg q; initial q=1; endmodule");
        assert!(
            receiver
                .checkpoint(HostCheckpointLimits::default())
                .is_err()
        );
        receiver.start().unwrap();
        let saved = image(&receiver);
        struct Fail;
        impl DigitalActiveParticipant for Fail {
            fn settle_active(
                &mut self,
                _: &mut DigitalActiveExchange<'_>,
            ) -> Result<bool, DigitalRunError> {
                Err(DigitalRunError::ExternalExecution {
                    detail: "fixture participant failure".into(),
                })
            }
        }
        assert!(receiver.settle_with(1, &mut Fail).is_err());
        assert!(
            receiver
                .checkpoint(HostCheckpointLimits::default())
                .is_err()
        );
        receiver
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&receiver), saved);
    }

    #[test]
    fn host_checkpoint_retains_interleaved_external_registration_and_driver_accounting() {
        use crate::xspice::verilog::store::{
            DigitalBitConnection, ExternalBitDriverId, ExternalRealDriverId,
        };
        fn enrolled() -> (DigitalHost, ExternalBitDriverId, ExternalRealDriverId) {
            let mut host = host(
                "module enrolled; wire d; wreal r; reg q; real captured; initial @(posedge d) q=1; always @(r) captured=r; endmodule",
            );
            let d = signal(&host, "d");
            let r = signal(&host, "r");
            host.connect_bits(&[vec![DigitalBitConnection { signal: d, bit: 0 }]])
                .unwrap();
            let target = |name: &str| EventTarget {
                node_id: usize::MAX,
                port_name: "out".into(),
                driver_index: 0,
                instance: name.into(),
            };
            // Deliberately opposite to fresh()'s registration order.
            let real = host
                .attach_external_reals(&[r], &[(r, target("real"))])
                .unwrap()[0];
            let bit = host
                .attach_external_bits(&[0], &[(0, target("bit"))])
                .unwrap()[0];
            (host, bit, real)
        }
        struct Driver {
            bits: Vec<(ExternalBitDriverId, DigitalValue)>,
            reals: Vec<(ExternalRealDriverId, f64)>,
        }
        impl DigitalActiveParticipant for Driver {
            fn settle_active(
                &mut self,
                exchange: &mut DigitalActiveExchange<'_>,
            ) -> Result<bool, DigitalRunError> {
                let mut changed = !exchange.take_event_changes().is_empty();
                if !self.bits.is_empty() || !self.reals.is_empty() {
                    exchange.drive_bank(&self.bits, &self.reals)?;
                    self.bits.clear();
                    self.reals.clear();
                    changed = true;
                }
                Ok(changed)
            }
        }
        let (mut original, bit, real) = enrolled();
        original.prepare_start().unwrap();
        original
            .settle_with(
                0,
                &mut Driver {
                    bits: vec![(bit, DigitalValue::zero())],
                    reals: vec![(real, 1.25)],
                },
            )
            .unwrap();
        let saved = transport(&image(&original));
        let (mut restored, restored_bit, _) = enrolled();
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        original
            .settle_with(
                2,
                &mut Driver {
                    bits: vec![(bit, DigitalValue::one())],
                    reals: vec![],
                },
            )
            .unwrap();
        restored
            .settle_with(
                2,
                &mut Driver {
                    bits: vec![(restored_bit, DigitalValue::one())],
                    reals: vec![],
                },
            )
            .unwrap();
        assert_eq!(
            restored.read(signal(&restored, "q")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(
            restored.read_real(signal(&restored, "captured")),
            Some(1.25)
        );
        assert_eq!(image(&restored), image(&original));
    }
}
