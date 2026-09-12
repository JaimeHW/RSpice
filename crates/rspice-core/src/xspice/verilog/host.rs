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
//! evaluated. Timed captures are held by due tick, with one stable kernel
//! wakeup identity; when due, they join the store's region queue. The
//! stratification after the active region is [`DigitalHost::promote_region`]'s, and the kernel
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
//! One tick is the compiled plan's finest elaborated precision. Delay operands
//! already contain integer design ticks after module-local rounding. The same
//! precision maps digital events to analog breakpoints.
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

#[cfg(test)]
mod analog_sample_tests;
mod coupled;
use coupled::NoActiveParticipant;
pub(crate) use coupled::{DigitalActiveExchange, DigitalActiveParticipant};

use rspice_veriloga::canonical_ir::digital_value::DigitalEventCount;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    sync::Arc,
};

use rspice_veriloga::canonical_ir::VectorBounds;
use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalProcessKind, DigitalSchedulingRegion, DigitalSensitivityTerm,
    DigitalSignal,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DEFAULT_PROCESS_STEP_LIMIT, DigitalDeferredUpdate, DigitalEvalError, DigitalEvalScratch,
    DigitalProcessOutcome, DigitalResumeState, DigitalWaitRequest, any_real_term_is_satisfied,
    any_term_is_satisfied, apply_deferred as apply_deferred_update, resume_in as resume_process,
    start_in as start_process,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::{DigitalAnalogProbeId, DigitalSignalId};

use super::store::{
    DigitalSignalStore, EventCapture, SignalTransition, StoreError, TransitionValues, signal_name,
};
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
    /// An enrolled event participant could not complete shared execution.
    ExternalExecution {
        /// The participant's concrete failure or a connection protocol error.
        detail: String,
    },
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
        /// The delay it asked for, in design ticks.
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
    /// The logical event-order counter cannot represent another operation.
    EventSequenceOverflow,
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
            Self::ExternalExecution { detail } => {
                write!(f, "shared digital execution failed: {detail}")
            }
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
                "process {process} asked to resume {delay} design ticks after it suspended, \
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
            Self::EventSequenceOverflow => write!(f, "digital event ordering sequence exhausted"),
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
            StoreError::LinkedNetRequiresDriver { name } => Self::Compile {
                detail: format!(
                    "external drive of linked event net {name} requires a declared driver identity"
                ),
            },
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
    AwaitingExpression(u64),
    /// Paused within an expression until the circuit evaluates its producer.
    AwaitingAnalog(DigitalAnalogProbeId),
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
    /// The store sequence this process's wait was armed at, so that a change
    /// older than the wait cannot satisfy it. Zero for a standing `always`
    /// list, which was armed before anything happened — see the arming barrier
    /// in `run_process`.
    wait_after_sequence: u64,
    remaining_events: DigitalEventCount,
}

/// A compiled digital plan, running.
#[derive(Clone)]
pub(crate) struct DigitalHost {
    plan: Arc<CanonicalDigitalPlan>,
    store: DigitalSignalStore,
    scheduler: EventScheduler,
    /// One stable wakeup identity, with captured payloads owned by their due tick.
    /// No process slot or driver identity is allocated per delayed assignment.
    nba_target: TargetId,
    external_targets: Vec<TargetId>,
    elaboration_closed: bool,
    delayed_updates: BTreeMap<u64, Vec<DigitalDeferredUpdate>>,
    event_updates: BTreeMap<u64, EventCapture>,
    expression_updates: BTreeMap<u64, DigitalDeferredUpdate>,
    expression_processes: BTreeMap<u64, usize>,
    event_waiters: Vec<BTreeSet<u64>>,
    event_candidates: Vec<u64>,
    slots: Vec<ProcessSlot>,
    /// Net to waiting-process index, ascending within each net.
    waiters: Vec<Vec<usize>>,
    /// Processes deferred to the inactive region, in the order they deferred.
    inactive: Vec<usize>,
    /// Read barriers in encounter order, serviced before later HDL regions.
    analog_waiters: Vec<usize>,
    /// Ready activations caused by an analog event. Its reported tick may be
    /// ahead of physical analog time; unrelated timers at that tick stay queued.
    analog_ready: Option<Vec<TargetId>>,
    /// Physical time for causal analog activations whose reporting tick differs.
    ///
    /// This is what a process woken by the publication reads as `$abstime`, so
    /// it is the instant the *event* happened — the interpolated threshold
    /// crossing for an A/D transition, not the endpoint of the analog step
    /// that discovered it.
    analog_activation_seconds: Option<f64>,
    /// The analog candidate instant the continuous half is being evaluated at,
    /// which is what an external Active participant is handed.
    ///
    /// Separate from [`Self::analog_activation_seconds`] because the two are
    /// different quantities whenever an event is interior to an analog step: a
    /// crossing has an exact sub-step instant, while the analog solution — and
    /// so an XSPICE wave assembled against it — exists only at the candidate
    /// endpoint, and cannot be rewound inside one step. They are equal for
    /// every publication dated at the endpoint itself.
    analog_exchange_seconds: Option<f64>,
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
    /// The transitions one dispatch round consumed, reused for the reason
    /// `fired` is: a drain that handed back a fresh `Vec` allocated one per
    /// round, and a round happens whenever anything moves.
    drained: Vec<SignalTransition>,
    /// The interpreter's working set — its value table, its edge arguments,
    /// its concatenation operands, and the two lists a suspension hands out.
    ///
    /// Owned by the host rather than by an activation, because an activation
    /// is where the cost was: a five-instruction process that runs a million
    /// times asked the allocator for the same five buffers a million times.
    /// Nothing of one activation reaches the next through it — the interpreter
    /// clears the value table at every entry, which is the execution
    /// contract's "a `Wait` resumes as a `Jump`".
    scratch: DigitalEvalScratch,
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
        let nba_target = scheduler.intern_target(EventTarget {
            node_id: usize::MAX,
            port_name: "nonblocking updates".to_string(),
            driver_index: 0,
            instance: "NBA".to_string(),
        });
        Self {
            nba_target,
            external_targets: Vec::new(),
            elaboration_closed: false,
            delayed_updates: BTreeMap::new(),
            event_updates: BTreeMap::new(),
            expression_updates: BTreeMap::new(),
            expression_processes: BTreeMap::new(),
            event_waiters: vec![BTreeSet::new(); plan.signals.len()],
            event_candidates: Vec::new(),
            store: DigitalSignalStore::from_plan(Arc::clone(&plan)),
            scheduler,
            slots: vec![
                ProcessSlot {
                    status: ProcessStatus::Queued,
                    resume: None,
                    wait_after_sequence: 0,
                    remaining_events: DigitalEventCount::one(),
                };
                plan.processes.len()
            ],
            waiters: vec![Vec::new(); plan.signals.len()],
            inactive: Vec::new(),
            analog_ready: None,
            analog_waiters: Vec::new(),
            analog_activation_seconds: None,
            analog_exchange_seconds: None,
            targets,
            process_of_target,
            fired: Vec::new(),
            drained: Vec::new(),
            scratch: DigitalEvalScratch::new(),
            plan,
        }
    }

    pub(crate) fn scheduler_limits(&self) -> SchedulerLimits {
        self.scheduler.limits()
    }

    pub(crate) fn connect_bits(
        &mut self,
        nets: &[Vec<super::store::DigitalBitConnection>],
    ) -> Result<(), DigitalRunError> {
        if self.elaboration_closed {
            return Err(DigitalRunError::ExternalExecution {
                detail: "event topology cannot change after digital execution starts".into(),
            });
        }
        self.store
            .connect_bits(nets)
            .map_err(|detail| DigitalRunError::Compile { detail })
    }

    pub(crate) fn connected_value(&self, net: usize) -> Option<DigitalValue> {
        self.store.connected_value(net)
    }

    pub(crate) fn plan(&self) -> &Arc<CanonicalDigitalPlan> {
        &self.plan
    }

    /// A fresh run sharing the immutable design and retaining scheduler limits.
    pub(crate) fn fresh(&self) -> Self {
        let mut fresh = Self::from_plan(
            Arc::clone(&self.plan),
            self.scheduler.resolution(),
            self.scheduler.limits(),
        );
        fresh.store.inherit_bit_connections(&self.store);
        fresh.store.inherit_analog_variable_inputs(&self.store);
        for (_, target) in fresh.store.external_sources() {
            fresh
                .external_targets
                .push(fresh.scheduler.intern_target(target.clone()));
        }
        fresh
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

    /// The range a signal's bits are named over, from the plan.
    ///
    /// Asked when something has to *say* which bit it means. A bridge holds the
    /// position it reads, and a `[7:4]` net's position 3 is the bit its module
    /// calls 7; naming it 3 would name a bit the author cannot find.
    pub(crate) fn declared_range(&self, signal: DigitalSignalId) -> VectorBounds {
        self.plan
            .signal(signal)
            .map_or(VectorBounds::SCALAR, DigitalSignal::declared_range)
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

    /// [`Self::next_tick`], with the process whose activation it is.
    ///
    /// The process is itself optional: the kernel also holds the host's
    /// nonblocking-update wakeup and any external bit driver, and neither
    /// belongs to a process of the design. A caller that wants to *name* the
    /// module behind a tick gets `None` for those rather than a process index
    /// that happens to sit at the same interned slot.
    pub(crate) fn next_tick_process(&self) -> Option<(u64, Option<usize>)> {
        let (tick, target) = self.scheduler.next_tick_target()?;
        Some((
            tick,
            self.process_of_target.get(usize::from(target)).copied(),
        ))
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
        self.require_standalone_execution()?;
        self.prepare_start()?;
        self.settle(0)
    }

    pub(crate) fn prepare_start(&mut self) -> Result<(), DigitalRunError> {
        self.elaboration_closed = true;
        for index in 0..self.slots.len() {
            self.queue(index, 0)?;
        }
        Ok(())
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
        self.prepare_forces(drives, tick)?;
        self.settle(tick)
    }

    /// Publish a co-timed input bank and enqueue its consequences. A mixed
    /// trial may need its candidate solution before those processes can run.
    pub(crate) fn prepare_forces(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        self.require_standalone_execution()?;
        for (signal, value) in drives {
            self.store.check_force(*signal, value, &self.plan)?;
        }
        self.set_event_clock(tick, None)?;
        for (signal, value) in drives {
            self.store.force(*signal, value.clone(), &self.plan)?;
        }
        self.dispatch(tick)?;
        Ok(())
    }

    /// [`Self::force_many_from_analog_at`] for a host that owns its own
    /// execution, so there is no participant to hand the candidate to.
    pub(crate) fn force_many_from_analog_ordered(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
        clock_seconds: f64,
        candidate_seconds: f64,
    ) -> Result<(), DigitalRunError> {
        self.require_standalone_execution()?;
        self.force_many_from_analog_at(
            drives,
            tick,
            clock_seconds,
            candidate_seconds,
            &mut NoActiveParticipant,
        )
    }

    /// Publish an analog-caused bank whose event instant *is* the analog
    /// candidate instant — [`Self::force_many_from_analog_at`] with the same
    /// value passed twice.
    ///
    /// Its one production caller is the causal lane in `mixed/shared.rs`,
    /// which hands the HDL a wake from the other event kernel at the trial's
    /// own endpoint: that event happened there, so there is no interior
    /// instant to carry apart from the candidate.
    pub(crate) fn force_many_from_analog_with(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
        physical_seconds: f64,
        participant: &mut (impl DigitalActiveParticipant + ?Sized),
    ) -> Result<(), DigitalRunError> {
        self.force_many_from_analog_at(
            drives,
            tick,
            physical_seconds,
            physical_seconds,
            participant,
        )
    }

    /// Publish an analog-caused bank and settle only the zero-delay causal
    /// consequences of it. Positive delays are still scheduled relative to
    /// `tick`, but are consumed only when analog time reaches their tick.
    ///
    /// The event instant and the analog candidate instant are two arguments
    /// because they are two quantities.
    ///
    /// `clock_seconds` is when the event happened, and is what every process
    /// this publication wakes reads as `$abstime`. `candidate_seconds` is the
    /// analog timepoint the continuous half has a solution at, and is what an
    /// external Active participant assembles its wave against. A crossing
    /// interior to an analog step separates the two: the digital world learns
    /// the exact instant, while the analog world still only knows the
    /// endpoint. Pass the same value twice for a publication dated at the
    /// endpoint, which is what [`Self::force_many_from_analog_with`] does.
    pub(crate) fn force_many_from_analog_at(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
        tick: u64,
        clock_seconds: f64,
        candidate_seconds: f64,
        participant: &mut (impl DigitalActiveParticipant + ?Sized),
    ) -> Result<(), DigitalRunError> {
        for (signal, value) in drives {
            self.store.check_force(*signal, value, &self.plan)?;
        }
        self.set_event_clock(tick, Some(clock_seconds))?;
        for (signal, value) in drives {
            self.store.force(*signal, value.clone(), &self.plan)?;
        }
        self.analog_ready = Some(Vec::new());
        self.analog_activation_seconds = Some(clock_seconds);
        self.analog_exchange_seconds = Some(candidate_seconds);
        let result = self
            .dispatch(tick)
            .and_then(|()| self.settle_with(tick, participant));
        self.analog_ready = None;
        self.analog_activation_seconds = None;
        self.analog_exchange_seconds = None;
        result
    }

    /// Publish one converged analog solution's probe values into the store.
    ///
    /// Call this immediately before anything that can run a process, so that
    /// every process activated by what follows reads the same analog
    /// solution — Verilog-AMS LRM 2.4 section 7.3.6.3 fixes a probe's value by
    /// the *time* the expression is evaluated, and a bank refreshed halfway
    /// through a settle would give two processes in one slot two answers.
    pub(crate) fn sample_analog_probes(&mut self, values: &[Option<f64>]) {
        self.store.sample_analog_probes(values);
    }

    /// Write a real net from outside the design and settle the consequences.
    pub(crate) fn force_real(
        &mut self,
        signal: DigitalSignalId,
        value: f64,
        tick: u64,
    ) -> Result<(), DigitalRunError> {
        self.require_standalone_execution()?;
        self.store.check_force_real(signal, &self.plan)?;
        self.set_event_clock(tick, None)?;
        self.store.force_real(signal, value, &self.plan)?;
        self.dispatch(tick)?;
        self.settle(tick)
    }

    /// Run every event dated at or before `tick`.
    ///
    /// Ticks with nothing scheduled cost nothing: the kernel jumps to the next
    /// tick that has an event rather than stepping through empty ones.
    pub(crate) fn advance_to(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        self.require_standalone_execution()?;
        self.advance_to_with(tick, &mut NoActiveParticipant)
    }

    pub(crate) fn advance_to_with(
        &mut self,
        tick: u64,
        participant: &mut (impl DigitalActiveParticipant + ?Sized),
    ) -> Result<(), DigitalRunError> {
        while let Some(next) = self.scheduler.next_tick() {
            if next > tick {
                break;
            }
            self.settle_with(next, participant)?;
        }
        Ok(())
    }

    /// Iterate one tick's slot until it is quiet.
    fn settle(&mut self, tick: u64) -> Result<(), DigitalRunError> {
        self.settle_with(tick, &mut NoActiveParticipant)
    }

    pub(crate) fn settle_with(
        &mut self,
        tick: u64,
        participant: &mut (impl DigitalActiveParticipant + ?Sized),
    ) -> Result<(), DigitalRunError> {
        self.elaboration_closed = true;
        // Taken out of `self` so that running a process — which needs the
        // whole host — cannot hold a borrow of the drain buffer, and put back
        // on the way out so the next tick reuses its capacity.
        let mut fired = std::mem::take(&mut self.fired);
        let outcome = self.settle_into(tick, &mut fired, participant);
        fired.clear();
        self.fired = fired;
        outcome
    }

    fn settle_into(
        &mut self,
        tick: u64,
        fired: &mut Vec<TargetId>,
        participant: &mut (impl DigitalActiveParticipant + ?Sized),
    ) -> Result<(), DigitalRunError> {
        loop {
            fired.clear();
            if let Some(ready) = self.analog_ready.as_mut() {
                fired.append(ready);
                for target in fired.iter() {
                    self.scheduler.note_external_activation(tick, *target)?;
                }
            } else {
                self.scheduler.run_due_event_targets(tick, fired)?;
            }

            if !fired.is_empty() {
                // Read rather than drained: the buffer belongs to the caller,
                // which is what lets it be reused across delta cycles, and it
                // is reachable from neither `self` nor the kernel while a
                // process runs.
                for target in fired.iter() {
                    if *target == self.nba_target {
                        while self
                            .delayed_updates
                            .first_key_value()
                            .is_some_and(|(due, _)| *due <= tick)
                        {
                            let (_, updates) = self.delayed_updates.pop_first().unwrap();
                            for update in updates {
                                self.store.release_delayed_update(update);
                            }
                        }
                        continue;
                    }
                    let index = self.process_of_target[usize::from(*target)];
                    self.run_process(index, tick)?;
                    self.dispatch(tick)?;
                }
            }

            // External Active work and its HDL consequences must quiet before
            // inactive or nonblocking work may advance. A participant returns
            // after one wave, so feedback through HDL can interrupt its settle.
            self.set_event_clock(tick, None)?;
            // The participant's own time base, not the event clock's: it is
            // assembling a continuous candidate, and the only instant it has a
            // solution for is the analog endpoint.
            let physical_seconds = self
                .analog_exchange_seconds
                .unwrap_or(self.scheduler.resolution().ticks_to_seconds(tick)?);
            let more = participant.settle_active(&mut DigitalActiveExchange {
                host: self,
                tick,
                physical_seconds,
            })?;
            self.dispatch(tick)?;
            let active = match &self.analog_ready {
                Some(ready) => !ready.is_empty(),
                None => self.scheduler.next_tick().is_some_and(|next| next <= tick),
            };
            if fired.is_empty() && !more && !active {
                if !self.analog_waiters.is_empty() {
                    participant.sample_analog(&mut DigitalActiveExchange {
                        host: self,
                        tick,
                        physical_seconds,
                    })?;
                    self.resume_analog_waiters(tick)?;
                } else if !self.promote_region(tick)? {
                    return Ok(());
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
        self.set_event_clock(tick, None)?;
        for region in DigitalSchedulingRegion::ORDERED {
            let mut promoted = false;

            if region == DigitalSchedulingRegion::Inactive && !self.inactive.is_empty() {
                for index in std::mem::take(&mut self.inactive) {
                    self.queue_ready(index, tick)?;
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
                    self.scheduler
                        .note_external_activation(tick, self.nba_target)?;
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

    fn set_event_clock(
        &mut self,
        tick: u64,
        physical_seconds: Option<f64>,
    ) -> Result<(), DigitalRunError> {
        let absolute_seconds = match physical_seconds.or(self.analog_activation_seconds) {
            Some(seconds) => seconds,
            None => self.scheduler.resolution().ticks_to_seconds(tick)?,
        };
        self.store
            .set_activation_clock(rspice_veriloga::canonical_ir::DigitalClock {
                tick,
                absolute_seconds,
            });
        Ok(())
    }

    /// Run one process from wherever it stopped, and record where it stops
    /// next.
    fn run_process(&mut self, index: usize, tick: u64) -> Result<(), DigitalRunError> {
        self.set_event_clock(tick, None)?;
        // The plan, the store and the scratch are three different fields, so
        // an activation borrows all three at once and pays for none of them:
        // the process is read straight out of `self.plan` rather than through
        // a refcount bump on a cloned handle, and the scratch is lent rather
        // than moved out of `self` and back. Both detours existed to keep the
        // three borrows apart on one `self`, which a field-wise borrow does
        // for free. What they replaced is still the point: a whole
        // `CfgFunction` — blocks, instructions, values, params — plus the
        // static sensitivity list, copied and dropped per activation.
        let Some(process) = self.plan.processes.get(index) else {
            return Ok(());
        };
        // The resume state goes in **by value**: its argument list is the one
        // the previous suspension built, and handing it over is what lets the
        // same allocation carry the next suspension's arguments back out.
        let resume = self.slots[index].resume.take();
        let outcome = match resume {
            Some(state) => resume_process(
                &self.plan,
                process,
                state,
                &mut self.store,
                &mut self.scratch,
                DEFAULT_PROCESS_STEP_LIMIT,
            ),
            None => start_process(
                &self.plan,
                process,
                &mut self.store,
                &mut self.scratch,
                DEFAULT_PROCESS_STEP_LIMIT,
            ),
        };
        let outcome = outcome.map_err(|error| DigitalRunError::Evaluation {
            process: self.describe(index),
            error,
        })?;

        self.store
            .current_sequence()
            .ok_or(DigitalRunError::EventSequenceOverflow)?;
        let process_kind = process.kind;
        self.check_expression_error()?;
        self.expression_updates
            .extend(self.store.take_expression_captures());
        for capture in self.store.take_event_captures() {
            for term in &capture.terms {
                self.event_waiters[usize::from(term.signal)].insert(capture.sequence);
            }
            self.event_updates.insert(capture.sequence, capture);
        }

        // Capture requests were emitted at their statement, and retain that
        // order even if the process has completed or now suspends elsewhere.
        for update in self.store.take_delayed_updates() {
            let Some(DigitalWaitRequest::Delay(delay)) = update.wait else {
                unreachable!("timed capture")
            };
            let delay = u64::try_from(delay).map_err(|_| DigitalRunError::NegativeDelay {
                process: self.describe(index),
                delay,
            })?;
            let due = tick
                .checked_add(delay)
                .filter(|tick| *tick <= TimeResolution::MAX_EXACT_TICKS)
                .ok_or(DigitalRunError::TickOverflow)?;
            if !self.delayed_updates.contains_key(&due) {
                self.scheduler.schedule_id_at(
                    due,
                    SchedulerRegion::Active,
                    self.nba_target,
                    EventValue::Digital(DigitalValue::default()),
                )?;
            }
            self.delayed_updates.entry(due).or_default().push(update);
        }

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
                if process_kind == DigitalProcessKind::Always {
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
                let (remaining, wait) = match wait {
                    DigitalWaitRequest::Repeated { count, event } => (count, *event),
                    wait => (DigitalEventCount::one(), wait),
                };
                match wait {
                    DigitalWaitRequest::AnalogSample(probe) => {
                        self.slots[index].status = ProcessStatus::AwaitingAnalog(probe);
                        self.analog_waiters.push(index);
                        Ok(())
                    }
                    DigitalWaitRequest::Repeated { .. } => {
                        unreachable!("interpreter emits a single repeat control")
                    }
                    DigitalWaitRequest::Expressions(wait) => {
                        let token = self.store.register_expression_wait(wait, remaining);
                        self.expression_processes.insert(token, index);
                        self.slots[index].status = ProcessStatus::AwaitingExpression(token);
                        Ok(())
                    }
                    DigitalWaitRequest::Event(terms) => {
                        self.slots[index].remaining_events = remaining;
                        // An `always` block's own list is the process's
                        // standing sensitivity, not a wait armed where it is
                        // reached: IEEE 1364-2005 section 9.9.2 restarts the
                        // process at its event control, and that control has
                        // been watching since before the process first ran. So
                        // a write the body made on its way back to the top is
                        // a change the list was already subscribed to, and
                        // `always @(q or seed) q = ~q;` is the zero-delay loop
                        // the kernel's delta ceiling exists to bound — not a
                        // process that deadlocks on its own event.
                        //
                        // A `@(...)` reached inside a body is the other case
                        // and keeps the barrier, because it starts watching
                        // where it is written: `clk = 1; @(posedge clk) ...`
                        // waits for the *next* edge, not the one it just made.
                        //
                        // Zero is the barrier that stops nothing: sequences
                        // are handed out from one, so no transition can carry
                        // it.
                        let barrier = if self.is_standing_sensitivity(index, &terms) {
                            0
                        } else {
                            self.store
                                .current_sequence()
                                .ok_or(DigitalRunError::EventSequenceOverflow)?
                        };
                        self.slots[index].wait_after_sequence = barrier;
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
        // Taken out of `self` for the duration of the round, so that waking a
        // process — which needs the whole host — cannot hold a borrow of the
        // buffer, and put back with its capacity for the next round.
        let mut drained = std::mem::take(&mut self.drained);
        let outcome = self.dispatch_into(tick, &mut drained);
        drained.clear();
        self.drained = drained;
        outcome
    }

    fn dispatch_into(
        &mut self,
        tick: u64,
        drained: &mut Vec<SignalTransition>,
    ) -> Result<(), DigitalRunError> {
        loop {
            self.store
                .current_sequence()
                .ok_or(DigitalRunError::EventSequenceOverflow)?;
            self.check_expression_error()?;
            self.store.drain_transitions_into(drained);
            if drained.is_empty() {
                return Ok(());
            }
            for transition in drained.iter() {
                for token in &transition.expressions {
                    if let Some(update) = self.expression_updates.remove(token) {
                        self.store.release_expression_capture(*token, update);
                    } else if let Some(index) = self.expression_processes.remove(token) {
                        debug_assert_eq!(
                            self.slots[index].status,
                            ProcessStatus::AwaitingExpression(*token)
                        );
                        self.queue_ready(index, tick)?;
                    }
                }
                self.dispatch_event_captures(transition);
                let net = usize::from(transition.signal);
                let mut position = 0usize;
                while position < self.waiters[net].len() {
                    let index = self.waiters[net][position];
                    if transition.sequence <= self.slots[index].wait_after_sequence {
                        position += 1;
                        continue;
                    }
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
                    if satisfied && self.slots[index].remaining_events.consume() {
                        self.unsubscribe(index);
                        self.queue_ready(index, tick)?;
                        // `unsubscribe` removed this entry, so the next
                        // candidate has slid into `position`.
                        continue;
                    }
                    position += 1;
                }
            }
        }
    }

    fn check_expression_error(&mut self) -> Result<(), DigitalRunError> {
        if let Some((index, error)) = self.store.take_expression_error() {
            return Err(DigitalRunError::Evaluation {
                process: self.describe(index),
                error,
            });
        }
        Ok(())
    }

    /// Only captures subscribed to the changed signal are visited. A capture
    /// sees changes after registration, including later writes in its creator's
    /// current activation; earlier transitions in the same batch are excluded.
    fn dispatch_event_captures(&mut self, transition: &SignalTransition) {
        let mut candidates = std::mem::take(&mut self.event_candidates);
        candidates.extend(
            self.event_waiters[usize::from(transition.signal)]
                .range(..transition.sequence)
                .copied(),
        );
        for id in &candidates {
            let capture = self
                .event_updates
                .get_mut(id)
                .expect("indexed event capture");
            let satisfied = match &transition.values {
                TransitionValues::FourState { previous, next } => {
                    any_term_is_satisfied(&capture.terms, transition.signal, previous, next)
                }
                TransitionValues::Real { previous, next } => {
                    any_real_term_is_satisfied(&capture.terms, transition.signal, *previous, *next)
                }
            };
            if satisfied && capture.remaining.consume() {
                let capture = self.event_updates.remove(id).unwrap();
                for term in &capture.terms {
                    self.event_waiters[usize::from(term.signal)].remove(id);
                }
                self.store.release_event_capture(capture);
            }
        }
        candidates.clear();
        self.event_candidates = candidates;
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

    fn queue_ready(&mut self, index: usize, tick: u64) -> Result<(), DigitalRunError> {
        if let Some(ready) = self.analog_ready.as_mut() {
            self.slots[index].status = ProcessStatus::Queued;
            let target = self.targets[index];
            if !ready.contains(&target) {
                ready.push(target);
            }
            Ok(())
        } else {
            self.queue(index, tick)
        }
    }

    /// Whether this suspension is the process's standing `always @(...)` list.
    ///
    /// `CfgDigitalProcess::static_sensitivity` is present only for a process
    /// that opens with an event control, and is the list such a process comes
    /// back to on every pass. Matching the suspension against it is how the
    /// host tells the standing list from a `@(...)` written inside a body —
    /// the front end already decided which shape this process has, and the
    /// answer is asked of it rather than rediscovered here.
    fn is_standing_sensitivity(&self, index: usize, terms: &[DigitalSensitivityTerm]) -> bool {
        self.plan
            .processes
            .get(index)
            .and_then(|process| process.static_sensitivity.as_ref())
            .is_some_and(|standing| standing.terms.as_slice() == terms)
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
    ///
    /// The terms are moved out of the slot rather than copied from it. The
    /// only caller wakes the process immediately after, which overwrites the
    /// status anyway, so the list a wake cloned and dropped was pure cost.
    ///
    /// And the list is not dropped either: it goes back to the interpreter's
    /// scratch, which is where the *next* suspension's list comes from. A
    /// process that waits a million times allocates one list.
    fn unsubscribe(&mut self, index: usize) {
        if !matches!(self.slots[index].status, ProcessStatus::AwaitingEvent(_)) {
            return;
        }
        let ProcessStatus::AwaitingEvent(terms) =
            std::mem::replace(&mut self.slots[index].status, ProcessStatus::Queued)
        else {
            unreachable!("the status was just matched as awaiting an event");
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
        self.scratch.recycle_terms(terms);
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
