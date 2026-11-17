//! Transactional delivery of analog system-task invocations.
//!
//! An evaluation records calls in source execution order. Only acceptance
//! publishes them to the host; another Newton evaluation replaces the candidate
//! calls. `$debug` and `$fdebug` are the exception: every evaluation's calls
//! are kept, a rejected point's included, and the next acceptance publishes
//! them. Tasks whose semantics require immediate delivery, including `$fatal`,
//! must use the host's immediate diagnostic path instead of this journal.

use std::fmt;

/// The analog system task associated with a compiled call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnalogTaskKind {
    Finish,
    Stop,
    Fatal,
    Error,
    Warning,
    Info,
    Display,
    Write,
    Strobe,
    Monitor,
    /// `$debug`: written at every evaluation, a rejected point's included
    /// (LRM 2.4 section 9.4.6).
    Debug,
    /// `$fdisplay`, `$fwrite`, `$fstrobe` and `$fmonitor`: the first operand
    /// is the file or multichannel descriptor.
    FileDisplay,
    FileWrite,
    FileStrobe,
    FileMonitor,
    /// `$fclose(descriptor)`.
    FileClose,
    /// `$fdebug`: `$debug` to the descriptor its first operand names, written
    /// even for a rejected point (LRM 2.4 section 9.5.9).
    FileDebug,
}

impl AnalogTaskKind {
    const ALL: [Self; 17] = [
        Self::Finish,
        Self::Stop,
        Self::Fatal,
        Self::Error,
        Self::Warning,
        Self::Info,
        Self::Display,
        Self::Write,
        Self::Strobe,
        Self::Monitor,
        Self::Debug,
        Self::FileDisplay,
        Self::FileWrite,
        Self::FileStrobe,
        Self::FileMonitor,
        Self::FileClose,
        Self::FileDebug,
    ];

    /// A stable small code, for machine-code descriptors that carry the kind.
    pub fn code(self) -> u8 {
        Self::ALL
            .iter()
            .position(|kind| *kind == self)
            .expect("every kind is listed") as u8
    }

    pub fn from_code(code: u64) -> Option<Self> {
        usize::try_from(code)
            .ok()
            .and_then(|code| Self::ALL.get(code))
            .copied()
    }

    /// Whether the task writes formatted text to the run's output: `$display`
    /// and its kin, `$debug`, and the severity tasks that do not stop the run.
    pub fn is_output(self) -> bool {
        matches!(
            self,
            Self::Display
                | Self::Write
                | Self::Strobe
                | Self::Monitor
                | Self::Debug
                | Self::Info
                | Self::Warning
                | Self::Error
        )
    }

    /// Whether the task writes to a descriptor its first operand names.
    pub fn writes_file(self) -> bool {
        matches!(
            self,
            Self::FileDisplay
                | Self::FileWrite
                | Self::FileStrobe
                | Self::FileMonitor
                | Self::FileClose
                | Self::FileDebug
        )
    }

    /// Whether a backend journals the call's numeric operands in source order
    /// through [`AnalogEffectJournal::record_values`]: every task but
    /// `$finish`, which journals its level alone.
    pub fn records_operands(self) -> bool {
        self != Self::Finish
    }

    /// Whether every evaluation writes, a rejected point's included, rather
    /// than only the evaluation a point is accepted with: `$debug` and
    /// `$fdebug`, which show the solver's iterations (LRM 2.4 sections 9.4.6
    /// and 9.5.9). Every other file task writes as its console twin does
    /// (section 9.5.2): `$fdisplay` and `$fwrite` once per accepted point, as
    /// `$display` and `$write` do.
    pub fn writes_every_evaluation(self) -> bool {
        matches!(self, Self::Debug | Self::FileDebug)
    }
}

/// Argument values captured when the call executes, before a later statement
/// can change a variable used by the call.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalogTaskArgument {
    Real(f64),
    Integer(i64),
    String(String),
}

/// One call to a site in the model's immutable task table. Repeated calls to
/// the same site remain separate records, including calls inside a loop.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogTaskInvocation {
    pub kind: AnalogTaskKind,
    pub site: u32,
    /// Simulation time when the call executed, independent of when the host
    /// drains the accepted delivery queue.
    pub time: f64,
    pub arguments: Box<[AnalogTaskArgument]>,
}

/// One accepted call together with the instance that executed it.
///
/// Names are borrowed for synchronous host delivery. The call owns its
/// argument snapshot, so retaining output never requires cloning a model or
/// evaluating its expressions again. `call.site` identifies the authored
/// call within this model; it is not a file line number.
#[derive(Debug)]
pub struct AnalogTaskEvent<'a> {
    pub instance: &'a str,
    pub model: &'a str,
    pub call: AnalogTaskInvocation,
    /// Where the call was written, when the model knows.
    pub source: Option<AnalogTaskSource<'a>>,
    /// The instance's open files, for a file task's delivery.
    pub files: Option<&'a mut dyn crate::AnalogFileTable>,
}

/// The file and one-based line of an authored task call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalogTaskSource<'a> {
    pub file: &'a str,
    pub line: u32,
}

impl AnalogTaskInvocation {
    fn payload_bytes(&self) -> Option<usize> {
        let fixed = self
            .arguments
            .len()
            .checked_mul(std::mem::size_of::<AnalogTaskArgument>())?;
        self.arguments.iter().try_fold(fixed, |bytes, argument| {
            bytes.checked_add(match argument {
                AnalogTaskArgument::String(value) => value.capacity(),
                _ => 0,
            })
        })
    }
}

/// Resource limits cover both speculative calls and accepted calls waiting for
/// delivery. Hosts should drain accepted calls at each public analysis point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalogEffectLimits {
    pub calls: usize,
    pub argument_bytes: usize,
}

impl Default for AnalogEffectLimits {
    fn default() -> Self {
        Self {
            calls: 65_536,
            argument_bytes: 4 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalogEffectError {
    InvalidTime,
    InvalidArgument,
    EvaluationFailed,
    CallLimit,
    ArgumentLimit,
    AllocationFailed,
}

impl fmt::Display for AnalogEffectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidTime => "analog system-task execution time must be finite",
            Self::InvalidArgument => "analog system-task argument is invalid",
            Self::EvaluationFailed => "analog system-task candidate evaluation failed",
            Self::CallLimit => "analog system tasks exceeded the pending-call limit",
            Self::ArgumentLimit => "analog system tasks exceeded the argument-memory limit",
            Self::AllocationFailed => "could not allocate analog system-task delivery storage",
        })
    }
}

impl std::error::Error for AnalogEffectError {}

/// Per-instance task journal. Cloning preserves an exact solver rollback image;
/// resetting an analysis retires both candidate and undelivered accepted calls.
///
/// A task that writes on every evaluation
/// ([`AnalogTaskKind::writes_every_evaluation`]) is kept from every
/// evaluation, a rejected point's and a withdrawn trial's included, rather
/// than replaced by the next one. The next accepted point publishes those
/// calls, in the order they ran, ahead of its own evaluation's other calls.
#[derive(Debug, Default, PartialEq)]
pub struct AnalogEffectJournal {
    limits: AnalogEffectLimits,
    candidate: Vec<AnalogTaskInvocation>,
    accepted: Vec<AnalogTaskInvocation>,
    /// Every-evaluation calls not yet published.
    debug: Vec<AnalogTaskInvocation>,
    candidate_bytes: usize,
    accepted_bytes: usize,
    debug_bytes: usize,
    failure: Option<AnalogEffectError>,
    evaluation_in_progress: bool,
}

impl Clone for AnalogEffectJournal {
    fn clone(&self) -> Self {
        // Vec::clone retains elements but need not retain spare capacity.
        // A rollback clone must keep acceptance allocation-free as well.
        let mut accepted =
            Vec::with_capacity(self.accepted.len() + self.candidate.len() + self.debug.len());
        accepted.extend(self.accepted.iter().cloned());
        Self {
            limits: self.limits,
            candidate: self.candidate.clone(),
            accepted,
            debug: self.debug.clone(),
            candidate_bytes: self.candidate_bytes,
            accepted_bytes: self.accepted_bytes,
            debug_bytes: self.debug_bytes,
            failure: self.failure,
            evaluation_in_progress: self.evaluation_in_progress,
        }
    }
}

impl AnalogEffectJournal {
    /// Start an evaluation whose early return must prevent acceptance. The
    /// generated Rust evaluator completes it only after its whole body succeeds.
    pub fn begin_evaluation(&mut self) {
        self.discard_candidate();
        self.evaluation_in_progress = true;
    }

    pub fn complete_evaluation(&mut self) {
        self.evaluation_in_progress = false;
    }

    /// A task operand or surrounding numerical evaluation failed. Preserve the
    /// first failure and prevent acceptance of an incomplete invocation stream.
    pub fn invalidate_candidate(&mut self, error: AnalogEffectError) {
        self.failure.get_or_insert(error);
    }

    /// Capture `$finish` using the Verilog real-to-integer conversion rule.
    /// Every backend uses this entry point so diagnostic levels agree.
    pub fn record_finish(
        &mut self,
        site: u32,
        time: f64,
        diagnostic: f64,
    ) -> Result<(), AnalogEffectError> {
        let diagnostic = diagnostic.round();
        if !diagnostic.is_finite() || !(0.0..=2.0).contains(&diagnostic) {
            self.invalidate_candidate(AnalogEffectError::InvalidArgument);
            return self.validate_candidate();
        }
        self.record(AnalogTaskInvocation {
            kind: AnalogTaskKind::Finish,
            site,
            time,
            arguments: Box::new([AnalogTaskArgument::Integer(diagnostic as i64)]),
        })
    }

    /// Capture a task's numeric arguments in source order. The model's task
    /// table supplies the string literals and the integer typing when the
    /// host formats the accepted call.
    pub fn record_values(
        &mut self,
        kind: AnalogTaskKind,
        site: u32,
        time: f64,
        values: &[f64],
    ) -> Result<(), AnalogEffectError> {
        if !kind.records_operands() {
            self.invalidate_candidate(AnalogEffectError::InvalidArgument);
            return self.validate_candidate();
        }
        let mut arguments = Vec::new();
        if arguments.try_reserve_exact(values.len()).is_err() {
            self.invalidate_candidate(AnalogEffectError::AllocationFailed);
            return self.validate_candidate();
        }
        arguments.extend(values.iter().map(|value| AnalogTaskArgument::Real(*value)));
        self.record(AnalogTaskInvocation {
            kind,
            site,
            time,
            arguments: arguments.into_boxed_slice(),
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn with_limits(limits: AnalogEffectLimits) -> Self {
        Self {
            limits,
            ..Self::default()
        }
    }

    /// Start a new candidate evaluation or reject the current candidate.
    pub fn discard_candidate(&mut self) {
        self.candidate.clear();
        self.candidate_bytes = 0;
        self.failure = None;
        self.evaluation_in_progress = false;
    }

    /// A new analysis starts: undelivered accepted calls are retired, but the
    /// every-evaluation calls of evaluations that ran stay for the next
    /// acceptance to publish.
    pub fn reset_analysis(&mut self) {
        self.discard_candidate();
        self.accepted.clear();
        self.accepted_bytes = 0;
    }

    /// Record a call atomically. A resource failure preserves prior records but
    /// invalidates this candidate; it cannot publish a truncated message stream.
    pub fn record(&mut self, invocation: AnalogTaskInvocation) -> Result<(), AnalogEffectError> {
        if let Some(error) = self.failure {
            return Err(error);
        }
        let result = self.record_checked(invocation);
        if let Err(error) = result {
            self.failure = Some(error);
        }
        result
    }

    fn record_checked(
        &mut self,
        invocation: AnalogTaskInvocation,
    ) -> Result<(), AnalogEffectError> {
        if !invocation.time.is_finite() {
            return Err(AnalogEffectError::InvalidTime);
        }
        let calls = self
            .candidate
            .len()
            .checked_add(self.accepted.len())
            .and_then(|count| count.checked_add(self.debug.len()))
            .and_then(|count| count.checked_add(1))
            .ok_or(AnalogEffectError::CallLimit)?;
        if calls > self.limits.calls {
            return Err(AnalogEffectError::CallLimit);
        }
        let payload = invocation
            .payload_bytes()
            .ok_or(AnalogEffectError::ArgumentLimit)?;
        let total_bytes = payload
            .checked_add(self.candidate_bytes)
            .and_then(|bytes| bytes.checked_add(self.debug_bytes))
            .and_then(|bytes| bytes.checked_add(self.accepted_bytes))
            .ok_or(AnalogEffectError::ArgumentLimit)?;
        if total_bytes > self.limits.argument_bytes {
            return Err(AnalogEffectError::ArgumentLimit);
        }
        // Reserve delivery capacity now, while errors still belong to candidate
        // evaluation. Circuit-wide accepted-state application is infallible.
        self.accepted
            .try_reserve(self.candidate.len() + self.debug.len() + 1)
            .map_err(|_| AnalogEffectError::AllocationFailed)?;
        let (buffer, bytes) = if invocation.kind.writes_every_evaluation() {
            (&mut self.debug, &mut self.debug_bytes)
        } else {
            (&mut self.candidate, &mut self.candidate_bytes)
        };
        buffer
            .try_reserve(1)
            .map_err(|_| AnalogEffectError::AllocationFailed)?;
        buffer.push(invocation);
        *bytes += payload;
        Ok(())
    }

    /// Publish the latest candidate after every device has passed acceptance
    /// validation. No messages are printed and no process exits in this method.
    #[cfg(test)]
    pub fn accept_candidate(&mut self) -> Result<(), AnalogEffectError> {
        self.validate_candidate()?;
        self.apply_validated_acceptance();
        Ok(())
    }

    pub fn validate_candidate(&self) -> Result<(), AnalogEffectError> {
        self.failure
            .or(self
                .evaluation_in_progress
                .then_some(AnalogEffectError::EvaluationFailed))
            .map_or(Ok(()), Err)
    }

    /// Circuit-wide commit after validation of every participating instance.
    pub fn apply_validated_acceptance(&mut self) {
        assert!(
            self.validate_candidate().is_ok(),
            "task acceptance requires a valid candidate"
        );
        self.accepted.append(&mut self.debug);
        self.accepted.append(&mut self.candidate);
        self.accepted_bytes += self.candidate_bytes + self.debug_bytes;
        self.candidate_bytes = 0;
        self.debug_bytes = 0;
    }

    pub fn accepted(&self) -> &[AnalogTaskInvocation] {
        &self.accepted
    }

    /// Inspect a completed candidate without publishing or consuming effects.
    /// The solver may use an accepted candidate's control requests to prepare
    /// its final solution before committing any model history.
    pub fn candidate(&self) -> Result<&[AnalogTaskInvocation], AnalogEffectError> {
        self.validate_candidate()?;
        Ok(&self.candidate)
    }

    pub fn has_candidate(&self) -> bool {
        !self.candidate.is_empty()
            || !self.debug.is_empty()
            || self.failure.is_some()
            || self.evaluation_in_progress
    }

    /// Keep the every-evaluation calls `rejected` holds in place of this
    /// journal's: a rollback that restores this older image after a rejected
    /// attempt must not unprint what that attempt's evaluations wrote, which
    /// begins with every call this image already held.
    pub fn keep_debug_of(&mut self, rejected: &mut Self) {
        std::mem::swap(&mut self.debug, &mut rejected.debug);
        std::mem::swap(&mut self.debug_bytes, &mut rejected.debug_bytes);
    }

    /// Transfer accepted calls to the analysis host, retaining journal capacity
    /// for subsequent points. Dropping the iterator discards its remaining
    /// records, so a host must process the whole delivery before accepting more.
    pub fn drain_accepted(&mut self) -> std::vec::Drain<'_, AnalogTaskInvocation> {
        self.accepted_bytes = 0;
        self.accepted.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_inspection_is_validated_and_does_not_publish_or_consume_calls() {
        let mut journal = AnalogEffectJournal::default();
        journal.begin_evaluation();
        journal.record_finish(7, 1.0, 0.0).unwrap();
        assert_eq!(
            journal.candidate(),
            Err(AnalogEffectError::EvaluationFailed)
        );
        journal.complete_evaluation();
        assert_eq!(journal.candidate().unwrap()[0].site, 7);
        assert!(journal.accepted().is_empty());
        assert_eq!(journal.candidate().unwrap().len(), 1);
        journal.discard_candidate();
        assert!(journal.candidate().unwrap().is_empty());
        journal.record_finish(8, 2.0, 1.0).unwrap();
        journal.accept_candidate().unwrap();
        assert!(journal.candidate().unwrap().is_empty());
        assert_eq!(journal.accepted()[0].site, 8);
    }

    #[test]
    fn incomplete_evaluation_cannot_be_accepted_or_lost_during_rollback() {
        let mut journal = AnalogEffectJournal::default();
        journal.begin_evaluation();
        journal.record_finish(0, 1.0, 0.0).unwrap();
        assert_eq!(
            journal.accept_candidate(),
            Err(AnalogEffectError::EvaluationFailed)
        );
        let mut restored = journal.clone();
        assert_eq!(restored, journal);
        assert_eq!(
            restored.validate_candidate(),
            Err(AnalogEffectError::EvaluationFailed)
        );
        restored.complete_evaluation();
        restored.accept_candidate().unwrap();
        assert_eq!(restored.drain_accepted().count(), 1);
        assert!(!restored.has_candidate());
        journal.begin_evaluation();
        journal.complete_evaluation();
        journal.accept_candidate().unwrap();
        assert!(journal.accepted().is_empty());
    }

    #[test]
    fn invalid_task_arguments_preserve_the_first_failure_until_rejection() {
        let mut journal = AnalogEffectJournal::default();
        journal.record_finish(0, 0.0, 0.5).unwrap();
        assert_eq!(
            journal.record_finish(1, 0.0, f64::NAN),
            Err(AnalogEffectError::InvalidArgument)
        );
        journal.invalidate_candidate(AnalogEffectError::EvaluationFailed);
        assert_eq!(
            journal.accept_candidate(),
            Err(AnalogEffectError::InvalidArgument)
        );
        assert!(journal.accepted().is_empty());
        journal.discard_candidate();
        journal.record_finish(2, 1.0, 1.5).unwrap();
        journal.accept_candidate().unwrap();
        assert_eq!(
            journal.accepted()[0].arguments.as_ref(),
            &[AnalogTaskArgument::Integer(2)]
        );
    }

    fn call(site: u32, value: i64) -> AnalogTaskInvocation {
        AnalogTaskInvocation {
            kind: AnalogTaskKind::Display,
            site,
            time: 0.0,
            arguments: Box::new([AnalogTaskArgument::Integer(value)]),
        }
    }

    #[test]
    fn invalid_execution_time_invalidates_the_candidate() {
        for time in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut journal = AnalogEffectJournal::default();
            let mut invocation = call(0, 1);
            invocation.time = time;
            assert_eq!(
                journal.record(invocation),
                Err(AnalogEffectError::InvalidTime)
            );
            assert_eq!(
                journal.accept_candidate(),
                Err(AnalogEffectError::InvalidTime)
            );
            assert!(journal.accepted().is_empty());
        }
    }

    #[test]
    fn only_the_last_accepted_iteration_is_delivered() {
        let mut journal = AnalogEffectJournal::default();
        journal.record(call(0, 1)).unwrap();
        assert!(journal.accepted().is_empty());
        journal.discard_candidate();
        journal.record(call(1, 2)).unwrap();
        journal.accept_candidate().unwrap();
        journal.record(call(2, 3)).unwrap();
        journal.discard_candidate();
        assert_eq!(journal.drain_accepted().collect::<Vec<_>>(), [call(1, 2)]);
        journal.accept_candidate().unwrap();
        assert!(journal.accepted().is_empty());
    }

    /// `$fdisplay` and `$fwrite` write once per accepted point, as `$display`
    /// and `$write` do: two Newton evaluations of one point leave the second
    /// one's call alone.
    #[test]
    fn a_file_task_writes_as_its_console_twin_does() {
        let file = |kind: AnalogTaskKind, value: i64| AnalogTaskInvocation {
            kind,
            site: 3,
            time: 0.0,
            arguments: Box::new([
                AnalogTaskArgument::Integer(0x8000_0003),
                AnalogTaskArgument::Integer(value),
            ]),
        };
        for kind in [AnalogTaskKind::FileDisplay, AnalogTaskKind::FileWrite] {
            assert!(!kind.writes_every_evaluation());
            let mut journal = AnalogEffectJournal::default();
            for value in [1, 2] {
                journal.begin_evaluation();
                journal.record(file(kind, value)).unwrap();
                journal.complete_evaluation();
            }
            journal.accept_candidate().unwrap();
            assert_eq!(
                journal.drain_accepted().collect::<Vec<_>>(),
                [file(kind, 2)],
                "{kind:?}"
            );
        }
    }

    fn debug(value: i64, time: f64) -> AnalogTaskInvocation {
        AnalogTaskInvocation {
            kind: AnalogTaskKind::Debug,
            site: 9,
            time,
            arguments: Box::new([AnalogTaskArgument::Integer(value)]),
        }
    }

    /// `$debug` writes at every evaluation, a rejected point's included
    /// (LRM 2.4 section 9.4.6): the next accepted point publishes every one,
    /// in the order they ran, ahead of its own evaluation's other calls.
    #[test]
    fn every_evaluation_calls_of_a_rejected_point_are_published_with_the_next_accepted_one() {
        let mut journal = AnalogEffectJournal::default();
        // Two evaluations of a point the solver then rejects.
        for value in [1, 2] {
            journal.begin_evaluation();
            journal.record(debug(value, 1.0)).unwrap();
            journal.record(call(0, value)).unwrap();
            journal.complete_evaluation();
        }
        journal.discard_candidate();
        // The earlier point it tried instead, accepted.
        journal.begin_evaluation();
        journal.record(debug(3, 0.5)).unwrap();
        journal.record(call(0, 3)).unwrap();
        journal.complete_evaluation();
        assert!(journal.has_candidate());
        let rollback = journal.clone();
        journal.accept_candidate().unwrap();
        assert_eq!(
            journal.drain_accepted().collect::<Vec<_>>(),
            [debug(1, 1.0), debug(2, 1.0), debug(3, 0.5), call(0, 3)]
        );
        assert!(!journal.has_candidate());
        // A withdrawn candidate keeps the debug calls its evaluations made,
        // and a new analysis still publishes them.
        let mut withdrawn = rollback;
        withdrawn.discard_candidate();
        withdrawn.reset_analysis();
        withdrawn.accept_candidate().unwrap();
        assert_eq!(
            withdrawn.drain_accepted().collect::<Vec<_>>(),
            [debug(1, 1.0), debug(2, 1.0), debug(3, 0.5)]
        );
        // A rollback to an image older than the rejected attempt keeps the
        // attempt's calls.
        let mut image = AnalogEffectJournal::default();
        let mut attempt = image.clone();
        attempt.record(debug(4, 2.0)).unwrap();
        image.keep_debug_of(&mut attempt);
        image.accept_candidate().unwrap();
        assert_eq!(image.drain_accepted().collect::<Vec<_>>(), [debug(4, 2.0)]);
        // `$fdebug` writes as `$debug` does.
        assert!(AnalogTaskKind::FileDebug.writes_every_evaluation());
        assert!(AnalogTaskKind::FileDebug.writes_file());
    }

    #[test]
    fn repeated_sites_keep_argument_snapshots_and_execution_order() {
        let mut journal = AnalogEffectJournal::default();
        for value in 0..4 {
            journal.record(call(7, value)).unwrap();
        }
        journal.accept_candidate().unwrap();
        assert_eq!(
            journal.accepted(),
            (0..4).map(|n| call(7, n)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn rollback_and_new_analysis_do_not_leak_messages() {
        let mut journal = AnalogEffectJournal::default();
        journal.record(call(0, 1)).unwrap();
        journal.accept_candidate().unwrap();
        let saved = journal.clone();
        journal.record(call(1, 2)).unwrap();
        journal.accept_candidate().unwrap();
        journal = saved;
        assert_eq!(journal.accepted(), [call(0, 1)]);
        journal.reset_analysis();
        assert!(journal.accepted().is_empty());
        journal.accept_candidate().unwrap();
        assert!(journal.accepted().is_empty());
    }

    #[test]
    fn accepting_a_rollback_clone_reuses_reserved_storage() {
        let mut journal = AnalogEffectJournal::default();
        journal.record(call(0, 1)).unwrap();
        journal.accept_candidate().unwrap();
        journal.record(call(1, 2)).unwrap();
        let mut restored = journal.clone();
        let pointer = restored.accepted.as_ptr();
        let capacity = restored.accepted.capacity();
        restored.accept_candidate().unwrap();
        assert_eq!(restored.accepted(), [call(0, 1), call(1, 2)]);
        assert_eq!(restored.accepted.as_ptr(), pointer);
        assert_eq!(restored.accepted.capacity(), capacity);
    }

    #[test]
    fn undelivered_calls_share_the_resource_budget() {
        let mut journal = AnalogEffectJournal::with_limits(AnalogEffectLimits {
            calls: 2,
            argument_bytes: usize::MAX,
        });
        journal.record(call(0, 1)).unwrap();
        journal.accept_candidate().unwrap();
        journal.record(call(1, 2)).unwrap();
        assert_eq!(
            journal.record(call(2, 3)),
            Err(AnalogEffectError::CallLimit)
        );
        assert_eq!(
            journal.accept_candidate(),
            Err(AnalogEffectError::CallLimit)
        );
        assert_eq!(journal.accepted(), [call(0, 1)]);
        journal.discard_candidate();
        journal.drain_accepted().for_each(drop);
        journal.record(call(2, 3)).unwrap();
    }

    #[test]
    fn string_payloads_are_bounded_without_partial_publication() {
        let mut journal = AnalogEffectJournal::with_limits(AnalogEffectLimits {
            calls: 8,
            argument_bytes: std::mem::size_of::<AnalogTaskArgument>() + 3,
        });
        let invocation = AnalogTaskInvocation {
            kind: AnalogTaskKind::Display,
            site: 0,
            time: 0.0,
            arguments: Box::new([AnalogTaskArgument::String("abcd".into())]),
        };
        assert_eq!(
            journal.record(invocation),
            Err(AnalogEffectError::ArgumentLimit)
        );
        assert_eq!(
            journal.accept_candidate(),
            Err(AnalogEffectError::ArgumentLimit)
        );
        assert!(journal.accepted().is_empty());
        journal.discard_candidate();
        journal.record(call(0, 1)).unwrap();
        journal.accept_candidate().unwrap();
        assert_eq!(journal.accepted(), [call(0, 1)]);
    }
}
