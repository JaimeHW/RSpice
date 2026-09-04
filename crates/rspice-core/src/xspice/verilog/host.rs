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

use std::{fmt, sync::Arc};

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalProcessKind, DigitalSchedulingRegion, DigitalSensitivityTerm,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalEvalError, DigitalProcessOutcome, DigitalResumeState, DigitalWaitRequest,
    any_real_term_is_satisfied, any_term_is_satisfied, apply_deferred as apply_deferred_update,
    resume as resume_process, start as start_process,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;

use super::store::{DigitalSignalStore, StoreError, TransitionValues, signal_name};
use crate::xspice::EventValue;
use crate::xspice::digital::DigitalValue;
use crate::xspice::event_scheduler::{
    EventScheduler, EventTarget, SchedulerError, SchedulerLimits, SchedulerRegion, TargetId,
    TimeResolution,
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
    /// The vector-only [`super::run_digital_verilog`] call has no analog node
    /// mapping or Newton callbacks. Mixed modules execute through
    /// [`super::MixedSignalHost`]; this variant prevents accidentally using
    /// the digital-only convenience API for one.
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
    /// A stimulus offered a value in the wrong domain for the port.
    ///
    /// Verilog-AMS LRM 2.4 section 3.7 makes a `wreal` carry a real and a
    /// `wire` carry bits, and converts between them only through the explicit
    /// `$realtobits`/`$bitstoreal`. A harness that offers the wrong one is
    /// refused rather than converted for, so a stimulus that has drifted away
    /// from the design says so instead of running.
    StimulusValueDomain {
        /// The port.
        name: String,
        /// Whether the *design* declares the port real.
        port_is_real: bool,
    },
    /// A vector column that is not a real number, for a real-valued port.
    RealSpelling {
        /// The port the column drives.
        port: String,
        /// What the column said.
        spelling: String,
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
    /// A stimulus names a different module than the design was compiled for.
    ///
    /// Only reachable through [`super::CompiledDigitalDesign::run`], because
    /// [`super::run_digital_verilog`] compiles the module its own stimulus
    /// names. A compiled design outlives the stimulus that produced it, so the
    /// two can drift apart; running one against the other would produce a trace
    /// of the wrong design, and every port name the stimulus uses might well
    /// resolve in it.
    StimulusModule {
        /// The module the design was compiled from.
        compiled: String,
        /// The module the stimulus asked for.
        requested: String,
    },
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
                 processes; run it through MixedSignalHost so its equations are stamped during \
                 transient Newton evaluation"
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
            Self::StimulusValueDomain { name, port_is_real } => {
                if *port_is_real {
                    write!(
                        f,
                        "`{name}` is a real-valued (`wreal`) port and the stimulus offered a \
                         four-state value; Verilog-AMS LRM 2.4 section 3.7 makes it carry a real"
                    )
                } else {
                    write!(
                        f,
                        "`{name}` is a four-state port and the stimulus offered a real value"
                    )
                }
            }
            Self::RealSpelling { port, spelling } => write!(
                f,
                "the column driving `{port}` reads `{spelling}`, which is not a real number"
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
            Self::StimulusModule {
                compiled,
                requested,
            } => write!(
                f,
                "the design was compiled from module `{compiled}` and the stimulus asks for \
                 `{requested}`; compile the module the stimulus names rather than running it \
                 against another"
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
            StoreError::RealPortDrivenWithBits { name, .. } => Self::StimulusValueDomain {
                name,
                port_is_real: true,
            },
            StoreError::FourStatePortDrivenWithAReal { name, .. } => Self::StimulusValueDomain {
                name,
                port_is_real: false,
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
#[derive(Clone)]
pub(crate) struct DigitalHost {
    plan: Arc<CanonicalDigitalPlan>,
    store: DigitalSignalStore,
    scheduler: EventScheduler,
    slots: Vec<ProcessSlot>,
    /// Net to waiting-process index, ascending within each net.
    waiters: Vec<Vec<usize>>,
    /// Processes deferred to the inactive region, in the order they deferred.
    inactive: Vec<usize>,
    /// The kernel's id for each process's driver, interned once at
    /// construction. The [`EventTarget`] behind it — the strings an
    /// oscillation diagnostic prints — stays in the kernel, and an activation
    /// never touches it.
    targets: Vec<TargetId>,
    /// The process each interned driver belongs to, which is what a drained
    /// activation names.
    process_of_target: Vec<usize>,
    /// The drivers one drain of the kernel reported, reused across delta
    /// cycles so that settling a tick does not allocate per pass.
    fired: Vec<TargetId>,
}

impl DigitalHost {
    /// Build a host for one plan at one time resolution.
    ///
    /// Nothing runs yet: [`Self::start`] is what places every process's first
    /// activation at tick zero.
    pub(crate) fn new(
        plan: &CanonicalDigitalPlan,
        resolution: TimeResolution,
        limits: SchedulerLimits,
    ) -> Self {
        Self::from_plan(Arc::new(plan.clone()), resolution, limits)
    }

    /// Build a host over a plan that is already shared.
    ///
    /// The seam a compile-once caller needs. [`Self::new`] deep-copies the plan
    /// so that a caller holding a borrow does not have to give it up; a caller
    /// that compiled once and runs many times has nothing to copy, because the
    /// plan is immutable for the whole of a host's life — every field this
    /// builds is per-run state, and the `Arc` is only ever read through.
    ///
    /// So two hosts over one plan share the compiled design and share no
    /// running state, which is exactly the property
    /// [`super::CompiledDigitalDesign::run`] rests on.
    pub(crate) fn from_plan(
        plan: Arc<CanonicalDigitalPlan>,
        resolution: TimeResolution,
        limits: SchedulerLimits,
    ) -> Self {
        let mut scheduler = EventScheduler::new(resolution, limits);
        // Interned in process order, once, so that queueing an activation is
        // an index rather than two `String` allocations and a string-keyed map
        // probe. The reverse map is what a drained driver is turned back into
        // a process by; it is built rather than assumed so that the identity
        // survives a kernel that interns anything else first.
        let mut targets = Vec::with_capacity(plan.processes.len());
        let mut process_of_target = vec![0usize; plan.processes.len()];
        for (index, process) in plan.processes.iter().enumerate() {
            let id = scheduler.intern_target(EventTarget {
                node_id: index,
                port_name: driven_signal_name(&plan, index),
                driver_index: 0,
                instance: format!("{}#{}", process.kind.keyword(), usize::from(process.id)),
            });
            process_of_target[usize::from(id)] = index;
            targets.push(id);
        }
        Self {
            store: DigitalSignalStore::new(&plan),
            scheduler,
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
            process_of_target,
            fired: Vec::new(),
            plan,
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

    /// The value a real net holds right now.
    pub(crate) fn read_real(&self, signal: DigitalSignalId) -> Option<f64> {
        self.store.real_value(signal)
    }

    /// Whether the compiled design declares this signal real.
    ///
    /// The *design* is the authority on a port's value domain, not the
    /// stimulus: the net-type keyword its author wrote is what decides, and a
    /// harness that disagrees is refused rather than believed.
    pub(crate) fn is_real(&self, signal: DigitalSignalId) -> bool {
        self.store.is_real(signal)
    }

    /// Earliest scheduled activation, used by the analog transient driver as
    /// an exact breakpoint.
    pub(crate) fn next_tick(&self) -> Option<u64> {
        self.scheduler.next_tick()
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
        self.force_many(&[(signal, value)], tick)
    }

    /// Publish a set of co-timed external drives as one event boundary.
    ///
    /// All values enter the store before sensitivity is dispatched. This is
    /// essential for a bank of A/D bridges sampled from one converged Newton
    /// solution: no process may observe a half-updated bridge bank.
    ///
    /// Every drive is checked before any is published, so a bank containing
    /// one unacceptable drive publishes none of it. This used to be done by
    /// deep-copying the whole running host on the way in and restoring it on
    /// the way out of a refusal — a copy of the plan handle, the store, the
    /// scheduler, every process slot and the whole sensitivity index, paid on
    /// every publish so that the rare refusal had somewhere to go back to.
    /// [`DigitalSignalStore::check_force`] answers the same question without
    /// writing, and the refusals it can return are the only ones `force` has.
    pub(crate) fn force_many(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        for (signal, value) in drives {
            self.store.check_force(*signal, value, &self.plan)?;
        }
        for (signal, value) in drives {
            self.store.force(*signal, value.clone(), &self.plan)?;
        }
        self.dispatch(tick)?;
        self.settle(tick)
    }

    /// Publish one converged analog solution's probe values into the store.
    ///
    /// Call this immediately before anything that can run a process, so that
    /// every process activated by what follows reads the same analog
    /// solution — Verilog-AMS LRM 2.4 section 7.3.6.3 fixes a probe's value by
    /// the *time* the expression is evaluated, and a bank refreshed halfway
    /// through a settle would give two processes in one slot two answers.
    pub(crate) fn sample_analog_potentials(&mut self, values: &[f64]) {
        self.store.sample_analog_potentials(values);
    }

    /// Write a real net from outside the design and settle the consequences.
    pub(crate) fn force_real(
        &mut self,
        signal: DigitalSignalId,
        value: f64,
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        self.store.force_real(signal, value, &self.plan)?;
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
        // Taken out of `self` so that running a process — which needs the
        // whole host — cannot hold a borrow of the drain buffer, and put back
        // on the way out so the next tick reuses its capacity.
        let mut fired = std::mem::take(&mut self.fired);
        let outcome = self.settle_into(tick, &mut fired);
        fired.clear();
        self.fired = fired;
        outcome
    }

    fn settle_into(&mut self, tick: u64, fired: &mut Vec<TargetId>) -> Result<(), DigitalRunError> {
        loop {
            fired.clear();
            self.scheduler.run_due_event_targets(tick, fired)?;

            if fired.is_empty() {
                if !self.promote_region(tick)? {
                    return Ok(());
                }
            } else {
                // Indexed rather than drained: `run_process` needs `&mut
                // self`, and the buffer is the caller's.
                for position in 0..fired.len() {
                    let index = self.process_of_target[usize::from(fired[position])];
                    self.run_process(index, tick)?;
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
                    apply_deferred_update(&self.plan, &mut self.store, update).map_err(
                        |error| DigitalRunError::Evaluation {
                            process: format!("a {} update", region.name()),
                            error,
                        },
                    )?;
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
        // The plan is immutable for the whole of a host's life and already
        // shared, so an activation borrows its process through a refcount bump
        // on the handle. Reading it through `self.plan` instead would put that
        // borrow and `&mut self.store` on the same `self`, which is the borrow
        // the deep copy this replaces was paying to avoid: a whole
        // `CfgFunction` — blocks, instructions, values, params — plus the
        // static sensitivity list, copied and dropped per activation.
        let plan = Arc::clone(&self.plan);
        let Some(process) = plan.processes.get(index) else {
            return Ok(());
        };
        let resume = self.slots[index].resume.take();
        let outcome = match &resume {
            Some(state) => resume_process(&plan, process, state, &mut self.store),
            None => start_process(&plan, process, &mut self.store),
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
                    let satisfied = match (&self.slots[index].status, &transition.values) {
                        (
                            ProcessStatus::AwaitingEvent(terms),
                            TransitionValues::FourState { previous, next },
                        ) => any_term_is_satisfied(terms, transition.signal, previous, next),
                        // Verilog-AMS LRM 2.4 section 3.7's event on a real net
                        // is a change of value, and the front end has already
                        // refused `posedge` on one. The rule is asked of the
                        // canonical IR rather than restated here, for the same
                        // reason the four-state arm asks it there.
                        (
                            ProcessStatus::AwaitingEvent(terms),
                            TransitionValues::Real { previous, next },
                        ) => any_real_term_is_satisfied(terms, transition.signal, *previous, *next),
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
        self.scheduler.schedule_id_superseding_at(
            tick,
            SchedulerRegion::Active,
            self.targets[index],
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
