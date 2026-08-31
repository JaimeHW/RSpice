//! Hosting a compiled digital plan on the event kernel.
//!
//! [`DigitalSignalStore`] is the memory of a running design and the canonical
//! IR's interpreter is its arithmetic. This is the part that decides *when*
//! anything runs: which process is resumed, in what order, and at what time.
//!
//! # What the kernel supplies and what this supplies
//!
//! [`EventScheduler`] owns time and ordering. Everything this host wants to
//! happen — a process starting, a process resuming after a `#delay`, a process
//! woken by a signal it waits on — is scheduled as one kernel event in
//! [`SchedulerRegion::Active`], so the order two activations run in is the
//! kernel's `(tick, region, sequence)` total order and not this module's
//! opinion, not a hash iteration, and not the order a `Vec` happened to be
//! built in.
//!
//! What the kernel deliberately does not own is IEEE 1364-2005 section 11's
//! *later* regions. A nonblocking update is not an event with a target and a
//! value the kernel could deliver; it is a write the interpreter already
//! evaluated, held by the store until its region drains. So the stratification
//! after the active region is [`DigitalHost::promote_region`]'s, and the kernel
//! sees one region.
//!
//! # A delta cycle is one pass of this loop
//!
//! [`DigitalHost::settle`] repeats: drain every activation the kernel has at
//! this tick, run each one, subscribe or reschedule whatever it suspended on,
//! and look again. When nothing is left it promotes the earliest non-empty
//! later region and looks again. Each repetition is marked with
//! [`EventScheduler::note_delta_cycle`], which is what converts a combinational
//! loop — `assign a = ~a;` is the smallest — from a hang into
//! [`SchedulerError::Oscillation`] naming the process that kept firing.
//!
//! That is the reading the kernel's own documentation asks for: one call per
//! settle iteration, so the ceiling measures how deep the settling is rather
//! than how many processes the design has.
//!
//! # Time
//!
//! One tick is one Verilog *time unit*, and this host fixes the unit for the
//! whole run rather than reading one out of the source. See
//! [`super::TIME_UNIT_RULING`] for what that decides and why the alternative
//! was refused rather than guessed.
//!
//! # Waking a process
//!
//! Sensitivity is an index from net to waiting process, rebuilt as processes
//! suspend and resume — the same shape [`crate::circuit::xspice_dispatch`]
//! uses for the settle loop, and for the same reason: a design where one net
//! moves must not cost a pass over every process. Indices within one net's
//! list are kept ascending so that the subset a dispatch visits is always a
//! subsequence of the full pass.
//!
//! Whether a change *means* anything to a waiting process is not decided here.
//! [`any_term_is_satisfied`] is the canonical IR's, because table 5-2's edge
//! classification is a semantic rule of the standard rather than a scheduling
//! policy, and a second copy of it here could disagree with the interpreter's.

use std::fmt;

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalProcessKind, DigitalSchedulingRegion, DigitalSensitivityTerm,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalEvalError, DigitalProcessOutcome, DigitalResumeState, DigitalWaitRequest,
    any_term_is_satisfied, apply_deferred as apply_deferred_update, resume as resume_process,
    start as start_process,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;

use super::store::{DigitalSignalStore, StoreError, signal_name};
use crate::xspice::EventValue;
use crate::xspice::digital::DigitalValue;
use crate::xspice::event_scheduler::{
    EventScheduler, EventTarget, SchedulerError, SchedulerLimits, SchedulerRegion, TimeResolution,
};

/// Why a digital run stopped.
///
/// Every variant is a refusal. A construct this host cannot execute is named
/// here rather than approximated, because a digital simulator that guesses
/// produces a plausible waveform and no way to tell it is wrong.
#[derive(Debug, Clone, PartialEq)]
pub enum DigitalRunError {
    /// The front end could not compile the source.
    Compile {
        /// What the compiler reported.
        detail: String,
    },
    /// The module compiled, but has no digital content to run.
    NoDigitalContent {
        /// The module that was compiled.
        module: String,
    },
    /// The module has an analog block as well as digital processes.
    ///
    /// Interleaving the two domains is a separate piece of work. Running the
    /// digital half alone would answer the question asked, plausibly and
    /// wrongly, so this refuses instead.
    MixedSignalModule {
        /// The module that was compiled.
        module: String,
        /// How many analog equations it carries.
        equations: usize,
    },
    /// The source declares a `` `timescale ``.
    ///
    /// See [`super::TIME_UNIT_RULING`]: this host fixes one time unit for the
    /// whole run, and a directive that says otherwise cannot be honoured, so it
    /// is refused rather than read and ignored.
    TimescaleDirective {
        /// One-based line the directive appears on.
        line: usize,
    },
    /// The event kernel refused, most often because a tick did not settle.
    Scheduler(SchedulerError),
    /// The process interpreter refused.
    Evaluation {
        /// Which process was running.
        process: String,
        /// What the interpreter reported.
        error: DigitalEvalError,
    },
    /// A stimulus tried to drive a net the design itself drives.
    StimulusOnDrivenNet {
        /// The net.
        name: String,
        /// How many drivers the design has on it.
        drivers: usize,
    },
    /// A stimulus offered a value at a width the design did not declare.
    StimulusWidth {
        /// The port.
        name: String,
        /// Width the design declares.
        declared: u32,
        /// Width the stimulus offered.
        offered: u32,
    },
    /// A vector column that is not a four-state spelling.
    VectorSpelling {
        /// The port the column drives.
        port: String,
        /// What the column said.
        spelling: String,
    },
    /// A vector column of the wrong width for its port.
    VectorWidth {
        /// The port the column drives.
        port: String,
        /// Width the stimulus declares for the port.
        declared: u32,
        /// Width the column spells.
        offered: u32,
    },
    /// A `#delay` whose operand evaluated negative. There is no time before
    /// the suspension to resume at, and clamping it to zero would silently
    /// turn a bug into a delta cycle.
    NegativeDelay {
        /// Which process asked.
        process: String,
        /// The delay it asked for, in time units.
        delay: i64,
    },
    /// A process finished when its own graph says it cannot.
    ///
    /// IEEE 1364-2005 section 9.9.2 spells an `always` process's restart as a
    /// back edge, so its function has no `Return` to reach. Reaching one means
    /// the lowering and this host disagree about the graph.
    UnexpectedCompletion {
        /// Which process.
        process: String,
    },
    /// The design names no signal by this name.
    UnknownSignal {
        /// The name that was asked for.
        name: String,
    },
    /// The run needed more ticks than the decimal grid can represent exactly.
    TickOverflow,
}

impl fmt::Display for DigitalRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile { detail } => write!(f, "the digital source did not compile: {detail}"),
            Self::NoDigitalContent { module } => write!(
                f,
                "module `{module}` declares no digital nets, processes or drivers"
            ),
            Self::MixedSignalModule { module, equations } => write!(
                f,
                "module `{module}` has {equations} analog equation(s) as well as digital \
                 processes; mixed-signal interleave is not implemented and running the \
                 digital half alone would report a wrong answer as a right one"
            ),
            Self::TimescaleDirective { line } => write!(
                f,
                "line {line} declares a `timescale, which this host cannot honour: {}",
                super::TIME_UNIT_RULING
            ),
            Self::Scheduler(error) => write!(f, "{error}"),
            Self::Evaluation { process, error } => {
                write!(f, "process {process} could not run: {error}")
            }
            Self::StimulusOnDrivenNet { name, drivers } => write!(
                f,
                "`{name}` is driven by {drivers} driver(s) inside the design, so a stimulus \
                 cannot drive it too"
            ),
            Self::StimulusWidth {
                name,
                declared,
                offered,
            } => write!(
                f,
                "`{name}` is declared {declared} bit(s) wide but the stimulus offered \
                 {offered} bit(s)"
            ),
            Self::VectorSpelling { port, spelling } => write!(
                f,
                "the column driving `{port}` reads `{spelling}`, which is not a four-state \
                 spelling of 0, 1, x and z"
            ),
            Self::VectorWidth {
                port,
                declared,
                offered,
            } => write!(
                f,
                "the column driving `{port}` spells {offered} bit(s) for a {declared}-bit port"
            ),
            Self::NegativeDelay { process, delay } => write!(
                f,
                "process {process} asked to resume {delay} time units after it suspended, \
                 which is before it suspended"
            ),
            Self::UnexpectedCompletion { process } => write!(
                f,
                "process {process} returned, but IEEE 1364-2005 section 9.9.2 makes it restart; \
                 its lowered graph should carry a back edge rather than a return"
            ),
            Self::UnknownSignal { name } => {
                write!(f, "the compiled design declares no signal named `{name}`")
            }
            Self::TickOverflow => write!(
                f,
                "the run reached a time past the exactly representable tick range"
            ),
        }
    }
}

impl std::error::Error for DigitalRunError {}

impl From<SchedulerError> for DigitalRunError {
    fn from(error: SchedulerError) -> Self {
        Self::Scheduler(error)
    }
}

impl From<StoreError> for DigitalRunError {
    fn from(error: StoreError) -> Self {
        // Flattened rather than nested so the public error carries no
        // crate-private type, and so a caller reading a refusal sees one
        // vocabulary rather than two.
        match error {
            StoreError::UndeclaredSignal(signal) => Self::UnknownSignal {
                name: format!("signal#{}", usize::from(signal)),
            },
            StoreError::ExternallyDrivenNetHasDrivers { name, drivers, .. } => {
                Self::StimulusOnDrivenNet { name, drivers }
            }
            StoreError::WidthMismatch {
                name,
                declared,
                offered,
                ..
            } => Self::StimulusWidth {
                name,
                declared,
                offered,
            },
        }
    }
}

/// Where a process is, between runs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessStatus {
    /// An activation is queued in the kernel, at this tick or a later one.
    Queued,
    /// Suspended on an event control, subscribed to every signal its terms
    /// name.
    AwaitingEvent(Vec<DigitalSensitivityTerm>),
    /// Waiting in the inactive region for the active region to drain (`#0`).
    Inactive,
    /// Reached `Return`. An `initial` process ends here and never runs again.
    Finished,
}

/// One process's schedule state.
#[derive(Debug, Clone)]
struct ProcessSlot {
    status: ProcessStatus,
    /// How to enter the process next time. `None` enters at the entry block,
    /// which is what a process that has never run does.
    resume: Option<DigitalResumeState>,
}

/// A compiled digital plan, running.
pub(crate) struct DigitalHost<'plan> {
    plan: &'plan CanonicalDigitalPlan,
    store: DigitalSignalStore,
    scheduler: EventScheduler,
    slots: Vec<ProcessSlot>,
    /// Net to waiting-process index, ascending within each net.
    waiters: Vec<Vec<usize>>,
    /// Processes deferred to the inactive region, in the order they deferred.
    inactive: Vec<usize>,
    /// The kernel target naming each process, precomputed because it is used
    /// on every activation and its strings are what an oscillation diagnostic
    /// prints.
    targets: Vec<EventTarget>,
}

impl<'plan> DigitalHost<'plan> {
    /// Build a host for one plan at one time resolution.
    ///
    /// Nothing runs yet: [`Self::start`] is what places every process's first
    /// activation at tick zero.
    pub(crate) fn new(
        plan: &'plan CanonicalDigitalPlan,
        resolution: TimeResolution,
        limits: SchedulerLimits,
    ) -> Self {
        let targets = plan
            .processes
            .iter()
            .enumerate()
            .map(|(index, process)| EventTarget {
                node_id: index,
                port_name: driven_signal_name(plan, index),
                driver_index: 0,
                instance: format!("{}#{}", process.kind.keyword(), usize::from(process.id)),
            })
            .collect();
        Self {
            plan,
            store: DigitalSignalStore::new(plan),
            scheduler: EventScheduler::new(resolution, limits),
            slots: vec![
                ProcessSlot {
                    status: ProcessStatus::Queued,
                    resume: None,
                };
                plan.processes.len()
            ],
            waiters: vec![Vec::new(); plan.signals.len()],
            inactive: Vec::new(),
            targets,
        }
    }

    /// The signal a name refers to.
    pub(crate) fn signal(&self, name: &str) -> Result<DigitalSignalId, DigitalRunError> {
        self.plan
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .map(|signal| signal.id)
            .ok_or_else(|| DigitalRunError::UnknownSignal {
                name: name.to_string(),
            })
    }

    /// The value a signal holds right now.
    pub(crate) fn read(&self, signal: DigitalSignalId) -> Option<&FourStateValue> {
        self.store.value(signal)
    }

    /// Queue every process's first activation at tick zero and settle it.
    ///
    /// IEEE 1364-2005 section 9.9 starts every `always` and `initial` process
    /// at the beginning of simulation, and section 6.1 makes a continuous
    /// assignment a driver that is active from then too — which is why the
    /// lowering puts a `ContinuousAssign`'s evaluation in its entry block and
    /// the suspension after it. All three kinds therefore start the same way,
    /// and the host does not consult the kind to decide.
    pub(crate) fn start(&mut self) -> Result<(), DigitalRunError> {
        for index in 0..self.slots.len() {
            self.queue(index, 0)?;
        }
        self.settle(0)
    }

    /// Write a signal from outside the design and settle the consequences.
    pub(crate) fn force(
        &mut self,
        signal: DigitalSignalId,
        value: FourStateValue,
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        self.store.force(signal, value, self.plan)?;
        self.dispatch(tick)?;
        self.settle(tick)
    }

    /// Run every event dated at or before `tick`.
    ///
    /// Ticks with nothing scheduled cost nothing: the kernel jumps to the next
    /// tick that has an event rather than stepping through empty ones.
    pub(crate) fn advance_to(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        while let Some(next) = self.scheduler.next_tick() {
            if next > tick {
                break;
            }
            self.settle(next)?;
        }
        Ok(())
    }

    /// Iterate one tick's slot until it is quiet.
    fn settle(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        loop {
            let mut fired = Vec::new();
            self.scheduler
                .run_due_events(tick, |event, _| fired.push(event))?;

            if fired.is_empty() {
                if !self.promote_region(tick)? {
                    return Ok(());
                }
            } else {
                for event in fired {
                    self.run_process(event.target.node_id, tick)?;
                    self.dispatch(tick)?;
                }
            }

            // One call per settle iteration, which is the reading
            // `note_delta_cycle` documents: the ceiling measures the depth of
            // the settling, not the size of the design.
            self.scheduler.note_delta_cycle(tick)?;
        }
    }

    /// Promote the earliest non-empty region after the active one.
    ///
    /// Returns whether anything moved. Regions are walked in
    /// [`DigitalSchedulingRegion::ORDERED`] order and only the first non-empty
    /// one is promoted, so a nonblocking update scheduled by an inactive-region
    /// process still lands after every process in the slot has run.
    fn promote_region(&mut self, tick: u64) -> Result<bool, DigitalRunError> {
        for region in DigitalSchedulingRegion::ORDERED {
            let mut promoted = false;

            if region == DigitalSchedulingRegion::Inactive && !self.inactive.is_empty() {
                for index in std::mem::take(&mut self.inactive) {
                    self.queue(index, tick)?;
                }
                promoted = true;
            }

            // Applied in the order they were evaluated, which is what makes two
            // nonblocking writes to different bits of one signal in one slot
            // both survive: `apply_deferred` resolves the target now, against
            // whatever the signal holds at this moment.
            let due = self.store.take_deferred_in(region);
            if !due.is_empty() {
                for update in &due {
                    apply_deferred_update(self.plan, &mut self.store, update).map_err(|error| {
                        DigitalRunError::Evaluation {
                            process: format!("a {} update", region.name()),
                            error,
                        }
                    })?;
                }
                self.dispatch(tick)?;
                promoted = true;
            }

            if promoted {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Run one process from wherever it stopped, and record where it stops
    /// next.
    fn run_process(&mut self, index: usize, tick: u64) -> Result<(), DigitalRunError> {
        let Some(process) = self.plan.processes.get(index) else {
            return Ok(());
        };
        let resume = self.slots[index].resume.take();
        let outcome = match &resume {
            Some(state) => resume_process(self.plan, process, state, &mut self.store),
            None => start_process(self.plan, process, &mut self.store),
        }
        .map_err(|error| DigitalRunError::Evaluation {
            process: self.describe(index),
            error,
        })?;

        match outcome {
            DigitalProcessOutcome::Finished => {
                // An `always` process that returns is a lowering bug: section
                // 9.9.2 makes it restart, spelled as a back edge, so its graph
                // has no `Return` to reach.
                //
                // A continuous assignment that returns is not. `assign y =
                // 1'b0;` has an empty read set, so the lowering gives it no
                // list to wait on and lets it return — its value cannot change,
                // and a driver woken for it would have nothing to do. The graph
                // decides, and the host does not overrule it.
                if process.kind == DigitalProcessKind::Always {
                    return Err(DigitalRunError::UnexpectedCompletion {
                        process: self.describe(index),
                    });
                }
                self.slots[index].status = ProcessStatus::Finished;
                Ok(())
            }
            DigitalProcessOutcome::Suspended(suspension) => {
                let (wait, resume) = suspension.into_parts();
                self.slots[index].resume = Some(resume);
                match wait {
                    DigitalWaitRequest::Event(terms) => {
                        self.subscribe(index, &terms);
                        self.slots[index].status = ProcessStatus::AwaitingEvent(terms);
                        Ok(())
                    }
                    DigitalWaitRequest::Delay(delay) => {
                        if delay < 0 {
                            return Err(DigitalRunError::NegativeDelay {
                                process: self.describe(index),
                                delay,
                            });
                        }
                        if delay == 0 {
                            // IEEE 1364-2005 section 9.7.7: `#0` defers to the
                            // inactive region of the *same* time slot, which is
                            // not the active region and not the next tick.
                            self.slots[index].status = ProcessStatus::Inactive;
                            self.inactive.push(index);
                            return Ok(());
                        }
                        let target = tick
                            .checked_add(delay as u64)
                            .filter(|tick| *tick <= TimeResolution::MAX_EXACT_TICKS)
                            .ok_or(DigitalRunError::TickOverflow)?;
                        self.queue(index, target)
                    }
                }
            }
        }
    }

    /// Wake every process a value change satisfies.
    ///
    /// Transitions are consumed oldest first, and every change is offered to
    /// every process waiting on that net: a blocking write that moves a signal
    /// twice inside one process is two events, and a process watching for an
    /// edge sees both.
    fn dispatch(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        loop {
            let transitions = self.store.take_transitions();
            if transitions.is_empty() {
                return Ok(());
            }
            for transition in transitions {
                let net = usize::from(transition.signal);
                let mut position = 0usize;
                while position < self.waiters[net].len() {
                    let index = self.waiters[net][position];
                    let satisfied = match &self.slots[index].status {
                        ProcessStatus::AwaitingEvent(terms) => any_term_is_satisfied(
                            terms,
                            transition.signal,
                            &transition.previous,
                            &transition.next,
                        ),
                        _ => false,
                    };
                    if satisfied {
                        self.unsubscribe(index);
                        self.queue(index, tick)?;
                        // `unsubscribe` removed this entry, so the next
                        // candidate has slid into `position`.
                        continue;
                    }
                    position += 1;
                }
            }
        }
    }

    /// Place one activation for a process in the kernel.
    ///
    /// Scheduled as a superseding event so that a process which two changes
    /// make ready in one delta wakes once, which is what a sensitivity list
    /// means: the process runs because something it waits on moved, not once
    /// per thing that moved. Supersession is per driver and a process is its
    /// own driver, so this never touches another process's activation.
    fn queue(&mut self, index: usize, tick: u64) -> Result<(), DigitalRunError> {
        if matches!(self.slots[index].status, ProcessStatus::Finished) {
            return Ok(());
        }
        self.slots[index].status = ProcessStatus::Queued;
        self.scheduler.schedule_superseding_at(
            tick,
            SchedulerRegion::Active,
            self.targets[index].clone(),
            EventValue::Digital(DigitalValue::default()),
        );
        Ok(())
    }

    /// Subscribe a process to every net its sensitivity list names.
    fn subscribe(&mut self, index: usize, terms: &[DigitalSensitivityTerm]) {
        for term in terms {
            let net = usize::from(term.signal);
            let Some(list) = self.waiters.get_mut(net) else {
                continue;
            };
            if let Err(position) = list.binary_search(&index) {
                list.insert(position, index);
            }
        }
    }

    /// Remove a process from every net it waits on.
    fn unsubscribe(&mut self, index: usize) {
        let ProcessStatus::AwaitingEvent(terms) = self.slots[index].status.clone() else {
            return;
        };
        for term in &terms {
            let net = usize::from(term.signal);
            let Some(list) = self.waiters.get_mut(net) else {
                continue;
            };
            if let Ok(position) = list.binary_search(&index) {
                list.remove(position);
            }
        }
    }

    /// A process, as a diagnostic names it.
    fn describe(&self, index: usize) -> String {
        self.plan
            .processes
            .get(index)
            .map(|process| format!("{} #{}", process.kind.keyword(), usize::from(process.id)))
            .unwrap_or_else(|| format!("#{index}"))
    }
}

/// The name an oscillation diagnostic should print for one process.
///
/// A continuous assignment is named by the net it drives, which is what an
/// author looking at `assign a = ~a;` needs to see. A procedural process has no
/// single driven net, so it is named by its kind.
fn driven_signal_name(plan: &CanonicalDigitalPlan, index: usize) -> String {
    let Some(process) = plan.processes.get(index) else {
        return "process".to_string();
    };
    if process.kind != DigitalProcessKind::ContinuousAssign {
        return "process".to_string();
    }
    plan.drivers
        .iter()
        .find(|driver| driver.process == process.id)
        .map(|driver| signal_name(plan, driver.id.signal))
        .unwrap_or_else(|| "process".to_string())
}
