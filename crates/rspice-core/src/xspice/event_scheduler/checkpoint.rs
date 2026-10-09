//! Portable event state. Queues, interning indices and cancellation tombstones
//! are rebuilt; only live events and semantic ordering/accounting are retained.
use super::super::digital::DigitalValue;
use super::*;
use serde::{Deserialize, Serialize};

const VERSION: u32 = 1;

// HDL wakeups use usize::MAX for "no circuit node". Its wire spelling must
// survive a 64-bit desktop / 32-bit Wasm boundary without becoming a real index.
pub(super) mod target_index {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(value: &usize, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(if *value == usize::MAX {
            u64::MAX
        } else {
            *value as u64
        })
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
        let value = u64::deserialize(deserializer)?;
        if value == u64::MAX {
            return Ok(usize::MAX);
        }
        usize::try_from(value)
            .ok()
            .filter(|value| *value != usize::MAX)
            .ok_or_else(|| {
                serde::de::Error::custom("target index is not representable on this platform")
            })
    }
}

/// Bounds on a decoded scheduler image and its reconstructed runtime storage.
/// The enclosing checkpoint reader must additionally bound input bytes before
/// deserialization. These limits do not replace the circuit's resource policy.
#[derive(Debug, Clone, Copy)]
pub struct SchedulerCheckpointLimits {
    pub max_targets: usize,
    pub max_events: usize,
    pub max_name_bytes: usize,
}

impl Default for SchedulerCheckpointLimits {
    fn default() -> Self {
        Self {
            max_targets: 1_048_576,
            max_events: 4_194_304,
            max_name_bytes: 64 * 1024 * 1024,
        }
    }
}

/// A versioned, address-independent event kernel image. This is one component
/// of a circuit checkpoint: model identities, process frames, resolved drivers
/// and analog state must be authenticated by the enclosing checkpoint.
///
/// Times and real payloads use IEEE-754 bits, retaining off-grid instants,
/// signed zero and nonfinite payloads without JSON floating-point conversion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulerCheckpoint {
    version: u32,
    limits: SchedulerLimits,
    current_bits: u64,
    started: bool,
    next_sequence: u64,
    delta_cycles: u32,
    events_executed: u64,
    targets: Vec<EventTarget>,
    activations: Vec<u64>,
    events: Vec<EventImage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EventImage {
    at_bits: u64,
    region: SchedulerRegion,
    sequence: u64,
    target: u32,
    value: ValueImage,
}

impl SchedulerCheckpoint {
    #[cfg(any(feature = "veriloga", test))]
    pub(crate) fn targets(&self) -> &[EventTarget] {
        &self.targets
    }
    /// Enclosing hosts inspect relationships only after kernel validation.
    #[cfg(feature = "veriloga")]
    pub(crate) fn target_count(&self) -> usize {
        self.targets.len()
    }
    #[cfg(any(feature = "veriloga", test))]
    pub(crate) fn live_events(
        &self,
    ) -> impl Iterator<Item = (Instant, SchedulerRegion, TargetId, EventValue)> + '_ {
        self.events.iter().map(|event| {
            (
                Instant(event.at_bits),
                event.region,
                TargetId(event.target),
                (&event.value).into(),
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ValueImage {
    Digital(DigitalValue),
    RealBits(u64),
}

impl From<EventValue> for ValueImage {
    fn from(value: EventValue) -> Self {
        match value {
            EventValue::Digital(value) => Self::Digital(value),
            EventValue::Real(value) => Self::RealBits(value.to_bits()),
        }
    }
}

impl From<&ValueImage> for EventValue {
    fn from(value: &ValueImage) -> Self {
        match value {
            ValueImage::Digital(value) => Self::Digital(*value),
            ValueImage::RealBits(bits) => Self::Real(f64::from_bits(*bits)),
        }
    }
}

/// Invalid, incompatible or over-budget event checkpoint. Restore is atomic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerCheckpointError(String);

impl fmt::Display for SchedulerCheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "event checkpoint: {}", self.0)
    }
}
impl std::error::Error for SchedulerCheckpointError {}

fn error(message: &str) -> SchedulerCheckpointError {
    SchedulerCheckpointError(message.into())
}

fn instant(bits: u64) -> Result<Instant, SchedulerCheckpointError> {
    Instant::from_seconds(f64::from_bits(bits))
        .filter(|at| at.0 == bits)
        .ok_or_else(|| error("time is not a canonical finite nonnegative instant"))
}

fn check_budget(
    targets: &[EventTarget],
    events: usize,
    budget: SchedulerCheckpointLimits,
) -> Result<(), SchedulerCheckpointError> {
    if targets.len() > budget.max_targets || targets.len() > u32::MAX as usize {
        return Err(error("target count exceeds the restore limit"));
    }
    if events > budget.max_events {
        return Err(error("event count exceeds the restore limit"));
    }
    let mut names = 0usize;
    for target in targets {
        names = names
            .checked_add(target.port_name.len())
            .and_then(|size| size.checked_add(target.instance.len()))
            .filter(|size| *size <= budget.max_name_bytes)
            .ok_or_else(|| error("target names exceed the restore limit"))?;
    }
    Ok(())
}

impl EventScheduler {
    /// Retain registration order and limits without cloning pending execution.
    #[cfg(feature = "veriloga")]
    pub(crate) fn checkpoint_template(&self) -> Self {
        let mut template = Self::new(self.limits);
        for target in &self.queues.targets {
            template.queues.intern(target.clone());
        }
        template
    }

    /// Capture a drained kernel between executions. The circuit owner must
    /// call this only after accepting all participants. A failed or partially
    /// executed region is rejected; future events, including causal timers
    /// deferred past a rounded reporting horizon, remain pending.
    pub fn checkpoint(
        &self,
        budget: SchedulerCheckpointLimits,
    ) -> Result<SchedulerCheckpoint, SchedulerCheckpointError> {
        if self.failed_slot || self.queues.sequence_exhausted || !self.queues.slot_is_empty() {
            return Err(error("cannot capture an interrupted or failed event slot"));
        }
        check_budget(&self.queues.targets, self.pending(), budget)?;
        let mut events: Vec<_> = self
            .queues
            .future
            .iter()
            .filter(|Reverse(queued)| !self.queues.cancelled.contains(&queued.0.sequence))
            .map(|Reverse(queued)| EventImage {
                at_bits: queued.0.at.0,
                region: queued.0.region,
                sequence: queued.0.sequence,
                target: queued.0.target.0,
                value: queued.0.value.into(),
            })
            .collect();
        events.sort_unstable_by_key(|event| (event.at_bits, event.region, event.sequence));
        Ok(SchedulerCheckpoint {
            version: VERSION,
            limits: self.limits,
            current_bits: self.current.0,
            started: self.started,
            next_sequence: self.queues.next_sequence,
            delta_cycles: self.slot_delta_cycles,
            events_executed: self.slot_events_executed,
            targets: self.queues.targets.clone(),
            activations: self.queues.activations.clone(),
            events,
        })
    }

    /// Validate and restore into this kernel, preserving its already-interned
    /// target identities and configured execution limits. Construct the entire
    /// replacement before installation; every refusal leaves `self` unchanged.
    pub fn restore_checkpoint(
        &mut self,
        image: &SchedulerCheckpoint,
        budget: SchedulerCheckpointLimits,
    ) -> Result<(), SchedulerCheckpointError> {
        if image.version != VERSION {
            return Err(error("unsupported schema version"));
        }
        if image.limits != self.limits {
            return Err(error("execution limits differ from the receiving kernel"));
        }
        check_budget(&image.targets, image.events.len(), budget)?;
        if !image.targets.starts_with(&self.queues.targets) {
            return Err(error(
                "interned target identities differ from the receiving kernel",
            ));
        }
        if image.activations.len() != image.targets.len() {
            return Err(error("activation counts do not match target identities"));
        }
        let current = instant(image.current_bits)?;
        let activation_count = image
            .activations
            .iter()
            .try_fold(0u64, |sum, count| sum.checked_add(*count))
            .ok_or_else(|| error("activation counts overflow"))?;
        if image.events_executed > self.limits.max_events_per_tick
            || image.delta_cycles > self.limits.max_delta_cycles_per_tick
        {
            return Err(error("slot accounting exceeds the execution limits"));
        }
        if !image.started
            && (current != Instant::ZERO
                || activation_count != 0
                || image.events_executed != 0
                || image.delta_cycles != 0)
        {
            return Err(error("an unstarted kernel has executed slot state"));
        }

        let mut restored = Self::new(self.limits);
        restored.current = current;
        restored.started = image.started;
        restored.slot_delta_cycles = image.delta_cycles;
        restored.slot_events_executed = image.events_executed;
        restored.queues.next_sequence = image.next_sequence;
        for (index, target) in image.targets.iter().enumerate() {
            if restored.queues.intern(target.clone()).index() != index {
                return Err(error("duplicate target identity"));
            }
            let count = image.activations[index];
            restored.queues.activations[index] = count;
            if count != 0 {
                restored.queues.activated.push(TargetId(index as u32));
            }
        }
        let mut sequences = HashSet::with_capacity(image.events.len());
        for event in &image.events {
            let at = instant(event.at_bits)?;
            if event.target as usize >= image.targets.len() {
                return Err(error("event references an undeclared target"));
            }
            if event.sequence >= image.next_sequence || !sequences.insert(event.sequence) {
                return Err(error(
                    "event sequence is duplicated or outside the allocated range",
                ));
            }
            let target = TargetId(event.target);
            restored.queues.driver_events[target.index()].push((at, event.sequence));
            restored.queues.future.push(Reverse(Queued(PendingEvent {
                at,
                region: event.region,
                sequence: event.sequence,
                target,
                value: (&event.value).into(),
            })));
        }
        *self = restored;
        Ok(())
    }
}
