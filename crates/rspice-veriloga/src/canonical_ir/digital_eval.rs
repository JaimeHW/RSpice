//! Running a digital process function.
//!
//! [`cfg_eval`](super::cfg_eval) is the reference interpreter for the analog
//! body: it walks a [`CfgFunction`] to completion and hands back every value it
//! computed. A process function cannot be evaluated that way, because it does
//! not run to completion — it suspends, and the thing that decides when it
//! resumes is a simulation kernel that does not exist yet.
//!
//! So this is a second interpreter, and the difference between the two is the
//! whole of its design. It walks the same graph over a different value domain
//! (four-state, not `f64`), and instead of returning a value table it returns
//! *what happened*: the process finished, or it suspended on something, and if
//! it suspended, everything needed to start it again.
//!
//! # What the caller supplies
//!
//! Two things, kept apart because they have different lifetimes:
//!
//! - the [`CanonicalDigitalPlan`], which is compiled data — signal widths, the
//!   process functions themselves — and never changes while a simulation runs;
//! - a [`DigitalEnvironment`], which is the live signal store and the sink for
//!   deferred updates, and changes constantly.
//!
//! The interpreter owns neither. It owns no clock either: a `#delay` reports
//! the number of time units it wants in [`DigitalWaitRequest::Delay`] and stops
//! there, because the interpreter has no basis for an opinion about when that
//! is. Time is the kernel's.
//!
//! # The execution contract
//!
//! Stated in [`digital`](super::digital)'s module documentation, which is the
//! specification this implements. Restated here only where honouring it
//! required a decision the contract did not make:
//!
//! - **A `Wait` resumes as a `Jump`.** So the value table does *not* survive a
//!   suspension: [`resume`] starts with an empty table and binds only the
//!   resume block's parameters from the [`DigitalResumeState`]. Anything a
//!   process needs across a suspension travels in those parameters, which is
//!   exactly what the contract says, and starting from an empty table is how
//!   this interpreter proves the lowering got it right rather than accidentally
//!   papering over a missing block argument.
//! - **Section 5.2.1 resizing happens at the write**, in [`apply_write`], and
//!   nowhere else. The lowering emits no resize node — a right-hand side
//!   arrives at its natural width and the target's declared width is applied
//!   here. This is assignment-context resizing, which zero-fills; it is not the
//!   literal padding rule of section 3.5.1, where a leading `x` extends with
//!   itself.
//! - **A branch on an ambiguous condition takes the false path**, IEEE
//!   1364-2005 section 9.4: a condition that is `x` or `z` "shall not be
//!   executed" as true, and the `else` runs if there is one. The lowering has
//!   already reduced a wide condition to one bit, but [`truth`] is applied
//!   regardless so a width-one `x` and a wide `x` reach the same answer.
//!
//! # What is not here
//!
//! Region ordering, time-slot iteration, and the decision to resume a process
//! at all. Those need every process at once and belong to the kernel; this
//! interpreter runs exactly one process for exactly as long as it will run,
//! then reports. [`classify_edge`] and [`term_is_satisfied`] are provided
//! because the classification is a semantic rule of the same standard rather
//! than a scheduling policy — the kernel decides *which* signals changed, and
//! these decide whether such a change means anything to a given process.

use std::borrow::Cow;

use super::cfg::{CfgFunction, CfgTerminator, CfgValueKind, DigitalWait, is_leaf_kind};
use super::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalDriverId, DigitalEdge, DigitalSchedulingRegion,
    DigitalSensitivityTerm, DigitalSignal, DigitalWriteSelect, DigitalWriteTarget,
};
use super::digital_value::{self, FourStateValue, truth};
use super::ids::{BlockId, DigitalAnalogProbeId, DigitalProcessId, DigitalSignalId, ValueId};
use crate::four_state::FourStateBit;

// ============================================================================
// The environment
// ============================================================================

/// The live half of a running digital simulation, as the interpreter sees it.
///
/// Deliberately small, and deliberately free of anything structural. Widths,
/// bounds, and process bodies come from the [`CanonicalDigitalPlan`], which the
/// interpreter reads directly; an environment that also had to answer "how wide
/// is this signal" would be a second authority on a question the compiler has
/// already settled, and the two could disagree.
///
/// Every signal a plan declares must be readable. An environment that returns
/// `None` from [`read_signal`](Self::read_signal) is reporting a store that was
/// not built from this plan, and the interpreter refuses rather than inventing
/// a value.
///
/// # What an implementation owes the kernel
///
/// [`write_signal`](Self::write_signal) is the only notification a blocking
/// write produces. An event kernel that must wake other processes on a change
/// observes it there, by comparing against what the store already held — the
/// interpreter does not report the old value, because a store that cannot see
/// its own previous contents cannot implement the trait usefully anyway.
pub trait DigitalEnvironment {
    /// The signal's value right now, at its declared width.
    ///
    /// Called once per [`CfgValueKind::DigitalSignalRead`] node, at the moment
    /// that node runs. Two reads of one signal on either side of a suspension
    /// are two calls and may return different values; that is the point of the
    /// node not being common-subexpressioned.
    fn read_signal(&self, signal: DigitalSignalId) -> Option<FourStateValue>;

    /// Replace the signal's whole value.
    ///
    /// Whole, not a slice: bit and part selects are resolved into a
    /// read-modify-write by [`apply_write`] before reaching here, so an
    /// implementation stores what it is given and nothing more. The value is
    /// always the signal's declared width.
    fn write_signal(&mut self, signal: DigitalSignalId, value: FourStateValue);

    /// Accept a nonblocking update for a later scheduling region.
    ///
    /// The right-hand side has already been evaluated and resized — the update
    /// carries a value, not an expression, which is what makes `a <= b; b <= a;`
    /// a swap. The target and region ride along so the kernel can apply it with
    /// [`apply_deferred`] when the region it names drains.
    fn defer_update(&mut self, update: DigitalDeferredUpdate);

    /// Replace a real variable's whole value.
    ///
    /// The real twin of [`write_signal`](Self::write_signal), and deliberately
    /// not [`drive_real_signal`](Self::drive_real_signal): a procedural write
    /// is IEEE 1364-2005 section 6.2's write of a *variable*, which has no
    /// drivers and therefore no resolution. Routing one through the driver
    /// table would give it a contribution slot to be folded against, and a
    /// `wrealsum` variable is not a thing the language has.
    ///
    /// No select rides along, for the reason [`DigitalRealDrive`] carries none:
    /// a real has no bits, so a partial write of one does not exist.
    fn write_real_signal(&mut self, signal: DigitalSignalId, value: f64);

    /// The real net's value right now (Verilog-AMS LRM 2.4 section 3.7).
    ///
    /// A second method rather than a widened return type on
    /// [`read_signal`](Self::read_signal), because the caller always knows
    /// which it wants: the node it is evaluating is either a
    /// `DigitalSignalRead` or a `DigitalRealSignalRead`, and the plan settled
    /// which at compile time. A union return would make every four-state
    /// consumer unwrap a case the compiler already ruled out.
    fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64>;

    /// The continuous-domain potential this probe names, right now.
    ///
    /// Verilog-AMS LRM 2.4 section 7.3.3 makes the read legal; section 7.3.6.3
    /// says which value it is — "the analog value calculated for the time
    /// corresponding to a real promotion of the digital time at which the
    /// expression is evaluated". Choosing that value is the environment's
    /// whole job here, because only the environment knows what the analog
    /// solver has settled: the interpreter has no clock, no solution vector,
    /// and no way to interpolate one.
    ///
    /// `None` is a probe the environment was not built for, and the
    /// interpreter refuses rather than reading zero volts — a missing node is
    /// not a grounded one.
    ///
    /// A default implementation returning `None` is deliberately *not*
    /// provided. An environment that silently answered "no such probe" for
    /// every probe would turn a wiring mistake into a refusal at simulation
    /// time on a design that compiled, which is exactly the shape of failure
    /// the plan's probe table exists to make impossible.
    fn read_analog_potential(&self, probe: DigitalAnalogProbeId) -> Option<f64>;

    /// Accept one driver's contribution to a real net.
    ///
    /// The real twin of [`drive_signal`](Self::drive_signal), and it carries no
    /// select: section 3.7 makes a `wreal` a real-valued connection with no
    /// bits, so there is no partial drive of one to represent.
    ///
    /// How several contributions combine is the kernel's, exactly as IEEE
    /// 1364-2005 table 4-1 is for a `wire`. The net's declaration says *which*
    /// rule — the compiler's half — and the store applies it.
    fn drive_real_signal(&mut self, drive: DigitalRealDrive);

    /// Accept one driver's contribution to a net.
    ///
    /// Deliberately *not* [`write_signal`](Self::write_signal), and deliberately
    /// without a default implementation that forwards to it. A net with two
    /// drivers has one value per driver and a resolution between them (IEEE
    /// 1364-2005 section 7.9); an implementation that stored a drive into the
    /// net would resolve it by last-write-wins, which is the exact bug the
    /// driver identity exists to prevent, and it would do so silently. An
    /// implementation that has no resolver yet should store the contribution
    /// per driver and resolve a single-driver net as that driver's value —
    /// which is correct, and stays correct when the resolver arrives.
    ///
    /// The value is already resized to the target's width per section 5.2.1.
    fn drive_signal(&mut self, drive: DigitalDrive);
}

/// One driver's contribution to a net, evaluated.
///
/// The counterpart of [`DigitalDeferredUpdate`] for a continuous driver. It
/// carries both identities because they answer different questions: the driver
/// id says *whose* contribution this is, and the target says *which bits of the
/// net* it covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalDrive {
    pub driver: DigitalDriverId,
    pub target: DigitalWriteTarget,
    /// Already resized to the target's width, per IEEE 1364-2005 section 5.2.1.
    pub value: FourStateValue,
}

/// One driver's contribution to a real net, evaluated.
///
/// No target and no width: a `wreal` has no bits, so a driver of one drives all
/// of it or none of it, and the driver identity is the whole of what the
/// resolution needs to tell two contributions apart.
///
/// `PartialEq` and not `Eq`, because an `f64` is not `Eq` — and the missing
/// reflexivity is a real property of the value rather than an inconvenience:
/// resolution and change detection both compare these, and a NaN that compared
/// equal to itself would be lying about which of the two it is.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalRealDrive {
    pub driver: DigitalDriverId,
    pub value: f64,
}

/// A nonblocking write, evaluated but not yet applied.
///
/// `PartialEq` and not `Eq` since [`DigitalUpdate::Real`] arrived, for the
/// reason [`DigitalRealDrive`] gives: an `f64` is not `Eq`.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalDeferredUpdate {
    pub target: DigitalWriteTarget,
    /// The right-hand side, evaluated.
    pub value: DigitalUpdate,
    pub region: DigitalSchedulingRegion,
}

/// The value half of a deferred update.
///
/// Two cases because the two domains defer differently in one respect that
/// matters: a four-state update was resized to the target's width when the
/// process ran (IEEE 1364-2005 section 5.2.1 makes the width a property of the
/// assignment, and by the time the update lands the statement is over), and a
/// real update has no width to have been resized to.
///
/// The alternative — one `FourStateValue` field and a real tunnelled through
/// its bits — would put a `$realtobits` in the nonblocking queue that the
/// author did not write, and section 3.7 is explicit that the conversion is a
/// call rather than a coincidence of storage.
#[derive(Debug, Clone, PartialEq)]
pub enum DigitalUpdate {
    /// Already resized to the target's width per section 5.2.1.
    FourState(FourStateValue),
    /// A real variable's next value. No width, no select.
    Real(f64),
}

// ============================================================================
// Values
// ============================================================================

/// A value a process function computes.
///
/// Four cases and no more, because a process function's type system has four:
/// [`CfgValueType::FourState`](super::cfg::CfgValueType::FourState),
/// [`Integer`](super::cfg::CfgValueType::Integer),
/// [`Real`](super::cfg::CfgValueType::Real) — which Verilog-AMS LRM 2.4 section
/// 3.7's real net brought into the discrete domain — and
/// [`Effect`](super::cfg::CfgValueType::Effect). The last is not data — it is a
/// write's position in the instruction stream — so reading one is an error
/// rather than a value.
///
/// `Eq` is gone with the arrival of [`Self::Real`], and deliberately not
/// worked around: an `f64` is not `Eq`, and every comparison this type is used
/// in — a resume state against a recompiled function, one drive against the
/// last — wants IEEE 754's answer rather than a bit pattern's.
#[derive(Debug, Clone, PartialEq)]
pub enum DigitalScalar {
    FourState(FourStateValue),
    Integer(i32),
    /// A real, as a `wreal` net or a process-local `real` holds one.
    Real(f64),
    /// A write's ordering token. Carries no data and may not be read.
    Effect,
}

// ============================================================================
// Outcomes
// ============================================================================

/// What running a process produced.
#[derive(Debug, Clone, PartialEq)]
pub enum DigitalProcessOutcome {
    /// The process reached a [`CfgTerminator::Wait`] and stopped there.
    Suspended(DigitalSuspension),
    /// The process reached a [`CfgTerminator::Return`] and is over.
    ///
    /// Only an `initial` process can produce this: IEEE 1364-2005 section 9.9.2
    /// makes an `always` process restart, and the lowering spells that as a back
    /// edge, so its function has no `Return` to reach. The interpreter does not
    /// consult [`CfgDigitalProcess::kind`] to decide — the graph decides, which
    /// is what makes the kind a description of the graph rather than a flag the
    /// runtime has to agree with.
    Finished,
}

/// A suspended process: what it is waiting for, and how to start it again.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalSuspension {
    wait: DigitalWaitRequest,
    resume: DigitalResumeState,
}

impl DigitalSuspension {
    pub fn wait(&self) -> &DigitalWaitRequest {
        &self.wait
    }

    pub fn resume_state(&self) -> &DigitalResumeState {
        &self.resume
    }

    pub fn into_parts(self) -> (DigitalWaitRequest, DigitalResumeState) {
        (self.wait, self.resume)
    }
}

/// What a suspended process is waiting for, with its operands evaluated.
///
/// The counterpart of [`DigitalWait`], which is the compiled form. The
/// difference is the delay: the IR carries a [`ValueId`] because the operand is
/// evaluated when the wait is *reached*, and by the time a kernel sees this the
/// evaluation has happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitalWaitRequest {
    /// `@(...)`: resume when one of these terms is satisfied.
    Event(Vec<DigitalSensitivityTerm>),
    /// `#n`: resume after this many time units, counted from the suspension.
    ///
    /// Time units, not seconds: converting one to the other needs a
    /// `timescale`, which the kernel owns.
    Delay(i64),
}

/// Everything needed to start a suspended process again.
///
/// Opaque on purpose. A kernel stores one of these and hands it back to
/// [`resume`]; it cannot build one, because a state that named the wrong block
/// or carried the wrong number of arguments would be a silently wrong
/// resumption rather than a refused one. [`resume`] validates what it is given
/// anyway — a state can outlive a recompilation.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalResumeState {
    process: DigitalProcessId,
    block: BlockId,
    arguments: Vec<DigitalScalar>,
}

impl DigitalResumeState {
    /// The process this state belongs to.
    pub fn process(&self) -> DigitalProcessId {
        self.process
    }

    /// The block execution will re-enter.
    pub fn block(&self) -> BlockId {
        self.block
    }

    /// The values that will bind to that block's parameters.
    pub fn arguments(&self) -> &[DigitalScalar] {
        &self.arguments
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Why a process could not be run.
///
/// Every one of these is a refusal rather than a panic, including the ones that
/// can only be reached by a malformed graph. A kernel that hits one has a bug
/// to report, and a process interpreter that aborts the simulator instead is
/// harder to debug and impossible to test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitalEvalError {
    /// A value was read before anything defined it.
    ///
    /// An unbound block parameter reaches here, which is how a resumption that
    /// lost an argument presents.
    UndefinedValue(ValueId),
    /// An analog-domain value appeared in a process function.
    AnalogValueInProcess(ValueId),
    /// A write's ordering token was read as data.
    EffectValueRead(ValueId),
    /// The plan does not declare a signal a node names.
    UndeclaredSignal(DigitalSignalId),
    /// The environment has no value for a declared signal.
    SignalUnavailable(DigitalSignalId),
    /// The plan does not declare an analog probe a node names.
    UndeclaredAnalogProbe(DigitalAnalogProbeId),
    /// The environment has no analog solution for a declared probe.
    ///
    /// Refused rather than read as zero volts, for the reason
    /// [`Self::SignalUnavailable`] is: an unbound net is not a grounded one,
    /// and a process that computed against a fabricated 0 V would produce a
    /// plausible waveform for a circuit nobody described.
    AnalogProbeUnavailable(DigitalAnalogProbeId),
    /// A block ended with [`CfgTerminator::Unset`].
    UnterminatedBlock(BlockId),
    /// A block's entry arguments do not match its parameter list.
    ArgumentArityMismatch {
        target: BlockId,
        expected: usize,
        found: usize,
    },
    /// The entry block of a process takes parameters, which nothing can bind.
    EntryBlockHasParameters(BlockId),
    /// A resume state built for a different process.
    ResumeProcessMismatch {
        expected: DigitalProcessId,
        found: DigitalProcessId,
    },
    /// A resume state naming a block this function does not have.
    ResumeBlockOutOfRange { block: BlockId, blocks: usize },
    /// A `#delay` operand that is not an integer.
    NonIntegerDelay(ValueId),
    /// A four-state value reached an operator that computes on reals, or the
    /// reverse.
    ///
    /// Refused rather than converted, for the reason Verilog-AMS LRM 2.4
    /// section 3.7 gives: the standard's own conversion between a real and bits
    /// is the explicit `$realtobits`/`$bitstoreal`, and a value holding `x` has
    /// no real to become. The lowering refuses every mix it can see, so
    /// reaching this means the plan and this interpreter disagree about a
    /// node's type.
    MixedValueDomains(ValueId),
    /// `$bitstoreal` was given a value with an `x` or a `z` in it.
    ///
    /// Neither standard rules on this case, so RSpice does, and it refuses.
    /// Verilog-AMS LRM 2.4 section 3.7 offers `$bitstoreal` as the conversion
    /// of a *bit pattern* into the real it encodes, and an unknown bit is the
    /// absence of a bit rather than a third value the IEEE 754 format has a
    /// place for. Substituting a `0` or a `1` would produce a specific, wrong,
    /// perfectly plausible number — the same reason
    /// [`FourStateValue::to_u64`](super::digital_value::FourStateValue::to_u64)
    /// answers `None` instead of guessing.
    ///
    /// A runtime refusal and not a compile-time one because the operand is a
    /// runtime value: a `reg` holds `x` until something writes it, and whether
    /// it still does when the conversion runs is a fact about the simulation.
    UnknownBitsToReal(ValueId),
    /// The process ran this many blocks without suspending or returning.
    StepLimitExceeded(usize),
}

impl std::fmt::Display for DigitalEvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedValue(value) => {
                write!(
                    f,
                    "value {} was read before it was defined",
                    usize::from(*value)
                )
            }
            Self::AnalogValueInProcess(value) => write!(
                f,
                "value {} is an analog-domain value and cannot appear in a digital process",
                usize::from(*value)
            ),
            Self::EffectValueRead(value) => write!(
                f,
                "value {} is a write's ordering token and carries no readable data",
                usize::from(*value)
            ),
            Self::UndeclaredSignal(signal) => write!(
                f,
                "signal {} is not declared by this digital plan",
                usize::from(*signal)
            ),
            Self::SignalUnavailable(signal) => write!(
                f,
                "the environment has no value for signal {}",
                usize::from(*signal)
            ),
            Self::UndeclaredAnalogProbe(probe) => write!(
                f,
                "analog probe {} is not declared by this digital plan",
                usize::from(*probe)
            ),
            Self::AnalogProbeUnavailable(probe) => write!(
                f,
                "the environment has no analog solution for probe {}",
                usize::from(*probe)
            ),
            Self::UnterminatedBlock(block) => {
                write!(f, "block {} has no terminator", usize::from(*block))
            }
            Self::ArgumentArityMismatch {
                target,
                expected,
                found,
            } => write!(
                f,
                "block {} takes {expected} parameters but {found} arguments were passed",
                usize::from(*target)
            ),
            Self::EntryBlockHasParameters(block) => write!(
                f,
                "entry block {} takes parameters, which no caller can bind",
                usize::from(*block)
            ),
            Self::ResumeProcessMismatch { expected, found } => write!(
                f,
                "resume state belongs to process {} and cannot resume process {}",
                usize::from(*found),
                usize::from(*expected)
            ),
            Self::ResumeBlockOutOfRange { block, blocks } => write!(
                f,
                "resume state names block {} of a function with {blocks} blocks",
                usize::from(*block)
            ),
            Self::NonIntegerDelay(value) => write!(
                f,
                "value {} is a delay operand but is not an integer",
                usize::from(*value)
            ),
            Self::MixedValueDomains(value) => write!(
                f,
                "value {} mixes a real and a four-state operand in one operator, which \
                 Verilog-AMS LRM 2.4 section 3.7 converts between only with an explicit \
                 `$realtobits` or `$bitstoreal`",
                usize::from(*value)
            ),
            Self::UnknownBitsToReal(value) => write!(
                f,
                "value {} reached `$bitstoreal` with an `x` or `z` bit; the conversion is \
                 defined over an IEEE 754 bit pattern and an unknown bit is not one, so there \
                 is no real to produce — drive every bit of the operand before converting it",
                usize::from(*value)
            ),
            Self::StepLimitExceeded(limit) => write!(
                f,
                "the process entered {limit} blocks without suspending or returning"
            ),
        }
    }
}

impl std::error::Error for DigitalEvalError {}

/// How many blocks one run of a process may enter.
///
/// Finite so that a `while` loop with a condition the body never falsifies, or
/// an `always` process whose back edge carries no `Wait`, reports instead of
/// hanging the simulator. IEEE 1364-2005 section 9.9.2 calls the latter a hang;
/// a refusal naming the process is more use than one.
pub const DEFAULT_PROCESS_STEP_LIMIT: usize = 100_000;

// ============================================================================
// Edge classification
// ============================================================================

/// Which edge, if any, a scalar transition is.
///
/// Rows are the previous bit, columns the new one, both in the truth tables'
/// `0 1 x z` order. Transcribed from IEEE 1364-2005 table 5-2: a `posedge` is a
/// transition *towards* 1 and a `negedge` one towards 0, where `x` and `z`
/// count as "away from" whichever of 0 and 1 the value used to be. So `0`→`x`
/// is a posedge and `1`→`x` is a negedge, while `x`→`z` is neither — nothing
/// about it moved towards either rail.
///
/// A table rather than a rule for the reason section 4.1's tables are tables:
/// it can be read against the document, and the alternative is a chain of
/// conditions nobody can check by eye.
pub const EDGE_TABLE: [[Option<DigitalEdge>; 4]; 4] = {
    const P: Option<DigitalEdge> = Some(DigitalEdge::Posedge);
    const N: Option<DigitalEdge> = Some(DigitalEdge::Negedge);
    const NONE: Option<DigitalEdge> = None;
    [
        //           0     1     x     z
        /* 0 */ [NONE, P, P, P],
        /* 1 */ [N, NONE, N, N],
        /* x */ [N, P, NONE, NONE],
        /* z */ [N, P, NONE, NONE],
    ]
};

/// Where a bit sits in [`EDGE_TABLE`].
///
/// The same order [`digital_value::TABLE_ORDER`] uses, and
/// [`tests::the_edge_table_is_indexed_like_the_truth_tables`] pins that it stays
/// so: a reordering of [`FourStateBit`] that permuted one set of tables and not
/// the other would be very hard to see.
const fn edge_index(bit: FourStateBit) -> usize {
    match bit {
        FourStateBit::Zero => 0,
        FourStateBit::One => 1,
        FourStateBit::Unknown => 2,
        FourStateBit::HighImpedance => 3,
    }
}

/// Classify a scalar transition, IEEE 1364-2005 table 5-2.
pub const fn classify_edge(previous: FourStateBit, next: FourStateBit) -> Option<DigitalEdge> {
    EDGE_TABLE[edge_index(previous)][edge_index(next)]
}

/// Whether a change of one signal satisfies one sensitivity term.
///
/// The caller has already established that the term names the signal that
/// changed; this decides whether the change is the kind the term asked for.
///
/// - An edge-qualified term looks at the least significant bit only, per IEEE
///   1364-2005 section 9.7.2: an edge on a vector is an edge on its LSB.
/// - A level-sensitive term fires on any change of value, and on nothing else —
///   a write that stores what the signal already held is not an event, which is
///   what keeps a combinational `@*` process from re-triggering itself forever.
pub fn term_is_satisfied(
    term: &DigitalSensitivityTerm,
    previous: &FourStateValue,
    next: &FourStateValue,
) -> bool {
    match term.edge {
        Some(edge) => classify_edge(previous.bit(0), next.bit(0)) == Some(edge),
        None => previous != next,
    }
}

/// Whether a change of one real net satisfies one sensitivity term.
///
/// The real counterpart of [`term_is_satisfied`], and it answers a shorter
/// question because the standard asks a shorter one. Verilog-AMS LRM 2.4
/// section 3.7 gives a `wreal` a value and no bits; the event a `@(net)` term
/// waits for is that value changing, and there is no edge to classify — the
/// front end refuses `posedge` on a real rather than inventing a threshold, so
/// an edge-qualified term here is a disagreement and is reported as
/// unsatisfied rather than guessed at.
///
/// **Exact inequality, with no tolerance.** The event is a change of value, not
/// a change large enough to matter: a model that ramps by 1e-18 per step has
/// changed, and an epsilon here would silently stop waking the processes that
/// said they wanted to know. Which also means a NaN never wakes anything by
/// staying NaN, and always wakes on the transition into or out of one, both of
/// which are what `!=` says.
pub fn real_term_is_satisfied(term: &DigitalSensitivityTerm, previous: f64, next: f64) -> bool {
    term.edge.is_none() && previous != next
}

/// Whether a change of one real net satisfies any term of a sensitivity list.
pub fn any_real_term_is_satisfied(
    terms: &[DigitalSensitivityTerm],
    signal: DigitalSignalId,
    previous: f64,
    next: f64,
) -> bool {
    terms
        .iter()
        .filter(|term| term.signal == signal)
        .any(|term| real_term_is_satisfied(term, previous, next))
}

/// Whether a change of `signal` satisfies any term of a sensitivity list.
pub fn any_term_is_satisfied(
    terms: &[DigitalSensitivityTerm],
    signal: DigitalSignalId,
    previous: &FourStateValue,
    next: &FourStateValue,
) -> bool {
    terms
        .iter()
        .filter(|term| term.signal == signal)
        .any(|term| term_is_satisfied(term, previous, next))
}

// ============================================================================
// Writes
// ============================================================================

/// The width one write drives.
///
/// Not the signal's width unless the target is the whole signal: assigning to
/// `q[3]` resizes the right-hand side to one bit, not to `q`'s.
fn target_width(signal: &DigitalSignal, select: &DigitalWriteSelect) -> u32 {
    match select {
        DigitalWriteSelect::Whole => signal.width,
        DigitalWriteSelect::Bit(_) => 1,
        DigitalWriteSelect::Part { msb, lsb } => msb.abs_diff(*lsb) as u32 + 1,
    }
}

/// Perform one write against the environment, resizing per section 5.2.1.
///
/// The single place assignment-context resizing happens. A bit or part select
/// is a read-modify-write of the whole signal, so the environment only ever
/// sees whole values; bits the select does not name keep what they held, and
/// bits the select names that lie outside the signal are dropped, which is IEEE
/// 1364-2005 section 4.2.1's rule for an out-of-range select on the left-hand
/// side. It is not symmetric with the read side, where an out-of-range select
/// yields `x` — reading a bit that does not exist has to produce *something*,
/// and writing one does not.
pub fn apply_write<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    environment: &mut E,
    target: &DigitalWriteTarget,
    value: &FourStateValue,
) -> Result<(), DigitalEvalError> {
    let signal = plan
        .signal(target.signal)
        .ok_or(DigitalEvalError::UndeclaredSignal(target.signal))?;
    let width = target_width(signal, &target.select);
    let value = value.resized(width);

    let next = match &target.select {
        DigitalWriteSelect::Whole => value,
        // A declared index is a name, so where the write lands is
        // `DigitalWriteSelect::low_position`'s answer against the signal's own
        // declaration rather than the index itself.
        select => {
            let mut current = read_signal(environment, signal)?;
            patch(
                &mut current,
                select.low_position(signal.declared_range()),
                &value,
            );
            current
        }
    };
    environment.write_signal(target.signal, next);
    Ok(())
}

/// Apply a deferred nonblocking update.
///
/// What a kernel calls when the region an update named drains. The value was
/// resized when the process ran; the *target* is resolved now, against whatever
/// the signal holds at this moment, so two nonblocking writes to different bits
/// of one signal in one time slot both survive.
pub fn apply_deferred<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    environment: &mut E,
    update: &DigitalDeferredUpdate,
) -> Result<(), DigitalEvalError> {
    match &update.value {
        DigitalUpdate::FourState(value) => apply_write(plan, environment, &update.target, value),
        // A real variable is written whole, so there is no target to resolve
        // against what the signal holds now — the read-modify-write the
        // four-state path performs has no partial write to perform.
        DigitalUpdate::Real(value) => {
            let signal = plan
                .signal(update.target.signal)
                .ok_or(DigitalEvalError::UndeclaredSignal(update.target.signal))?;
            environment.write_real_signal(signal.id, *value);
            Ok(())
        }
    }
}

/// Overwrite the bits of `current` starting at *position* `low`, counting from
/// the least significant end.
///
/// A position and not a declared index: the caller has already resolved the
/// select against the signal's declaration through
/// [`DigitalWriteSelect::low_position`], the same rule
/// [`digital_value::part_select`] reads back against. A position outside
/// `current` writes nothing, IEEE 1364-2005 section 5.2.1's rule for an
/// out-of-range left-hand side, and that is also how a declared index the
/// signal does not name comes to write nothing.
fn patch(current: &mut FourStateValue, low: i64, value: &FourStateValue) {
    for offset in 0..value.width() {
        let position = low + i64::from(offset);
        if position < 0 || position >= i64::from(current.width()) {
            continue;
        }
        current.set_bit(position as u32, value.bit(offset));
    }
}

fn read_signal<E: DigitalEnvironment + ?Sized>(
    environment: &E,
    signal: &DigitalSignal,
) -> Result<FourStateValue, DigitalEvalError> {
    let value = environment
        .read_signal(signal.id)
        .ok_or(DigitalEvalError::SignalUnavailable(signal.id))?;
    // Normalised rather than trusted. A store built from this plan already
    // holds the declared width; one that does not would otherwise make every
    // downstream width silently wrong, and section 5.2.1's rule is the right
    // answer to the question anyway.
    Ok(if value.width() == signal.width {
        value
    } else {
        value.resized(signal.width)
    })
}

// ============================================================================
// Running a process
// ============================================================================

/// One activation's SSA value table.
///
/// A slot holds whatever was last written to it, and a *generation stamp* says
/// which activation wrote it: the slot is defined now exactly when its stamp
/// equals [`current`](Self::current). Entering a function bumps the generation,
/// which empties the whole table in constant time.
///
/// That is the whole reason the table is not a `Vec<Option<DigitalScalar>>`
/// cleared at every entry. Clearing is O(|values|) per activation and drops
/// every value the previous activation left, one at a time; a gate-level
/// process runs about five instructions per activation over a table sized for
/// the *whole* function, so the emptying cost has no relation to the work done.
///
/// The execution contract is unchanged, and the generation is how it is
/// implemented rather than something it is traded against: every slot is
/// undefined at every entry, so a `Wait` still resumes as a `Jump` into an
/// empty table and a resumed block may read only its own parameters and what it
/// computes.
#[derive(Debug, Default, Clone)]
struct ValueTable {
    /// One slot per SSA value.
    slots: Vec<ValueSlot>,
    /// The generation being written now. Never zero once a run has started,
    /// which is what makes a never-written slot's zero stamp mean "undefined".
    current: u32,
}

/// One slot of a [`ValueTable`]: what was written, and when.
///
/// The stamp sits beside the value rather than in a table of its own, because
/// every read wants both — a separate stamp array would make each operand read
/// touch two cache lines instead of one, and would make the table two
/// allocations where a mixed-signal trial copies the whole scratch per trial.
#[derive(Debug, Clone)]
struct ValueSlot {
    /// The generation this slot was last written in.
    stamp: u32,
    /// Meaningful only when the stamp is the current generation; otherwise it
    /// is whatever some earlier activation left there.
    value: DigitalScalar,
}

impl ValueTable {
    /// Empty the table for a function with `len` values.
    ///
    /// Grows to the longest function it has been asked for and never shrinks:
    /// one scratch is shared by every process a host runs, so the table settles
    /// at the widest of them instead of being resized per activation. A slot
    /// beyond the current function's length is unreachable — every id comes
    /// from that function's own value list — and a slot within it carries an
    /// older generation's stamp, which is exactly "undefined".
    fn enter(&mut self, len: usize) {
        if self.slots.len() < len {
            self.slots.resize(
                len,
                ValueSlot {
                    stamp: 0,
                    value: DigitalScalar::Effect,
                },
            );
        }
        self.current = match self.current.checked_add(1) {
            Some(next) => next,
            // Four billion activations later. Every stamp goes back to the
            // never-written value and the generation restarts from one, so no
            // slot written before the wrap can be mistaken for one written
            // after it. The reset is O(|values|), once per 2^32 entries.
            None => {
                for slot in &mut self.slots {
                    slot.stamp = 0;
                }
                1
            }
        };
    }

    /// Record `value` as this activation's value for `id`.
    ///
    /// Overwriting is what releases whatever the slot held before, so a wide
    /// value from an earlier activation lives until the slot is used again
    /// rather than until the next entry.
    fn define(&mut self, id: ValueId, value: DigitalScalar) {
        let current = self.current;
        let slot = &mut self.slots[usize::from(id)];
        slot.value = value;
        slot.stamp = current;
    }

    /// This activation's value for `id`, if this activation gave it one.
    fn defined(&self, id: ValueId) -> Option<&DigitalScalar> {
        let slot = &self.slots[usize::from(id)];
        (slot.stamp == self.current).then_some(&slot.value)
    }

    /// The operand `id` names, borrowed.
    ///
    /// A miss is not by itself an error. Constants belong to no block — a
    /// `#delay` operand is pushed as a leaf because the `Wait` that consumes it
    /// is a terminator — so a value the table does not hold is read straight
    /// out of the function, which outlives the activation. Nothing is written
    /// back: a constant reference costs one match to rebuild, and caching one
    /// would cost a clone and a stamp write per entry to save that.
    fn get<'v>(
        &'v self,
        function: &'v CfgFunction,
        id: ValueId,
    ) -> Result<ScalarRef<'v>, DigitalEvalError> {
        if let Some(value) = self.defined(id) {
            return Ok(ScalarRef::of(value));
        }
        match &function.value(id).kind {
            CfgValueKind::FourStateConstant(value) => Ok(ScalarRef::FourState(value)),
            CfgValueKind::IntegerConstant(value) => Ok(ScalarRef::Integer(*value)),
            CfgValueKind::RealConstant(value) => Ok(ScalarRef::Real(*value)),
            // Every other leaf is an analog-domain one, and reporting it as
            // undefined would hide what it actually is.
            kind if is_leaf_kind(kind) => Err(DigitalEvalError::AnalogValueInProcess(id)),
            _ => Err(DigitalEvalError::UndefinedValue(id)),
        }
    }
}

/// One operand, borrowed out of wherever it lives.
///
/// The four-state case is a reference because it is the one with a payload
/// worth not copying — sixty-four bits inline, and a pair of heap planes above
/// that. The other three are one word each and are copied, which is why this is
/// not simply a `&DigitalScalar`: a constant read straight out of the function
/// has a `FourStateValue` to borrow but no `DigitalScalar` around it.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ScalarRef<'v> {
    FourState(&'v FourStateValue),
    Integer(i32),
    Real(f64),
    Effect,
}

impl<'v> ScalarRef<'v> {
    fn of(scalar: &'v DigitalScalar) -> Self {
        match scalar {
            DigitalScalar::FourState(value) => Self::FourState(value),
            DigitalScalar::Integer(value) => Self::Integer(*value),
            DigitalScalar::Real(value) => Self::Real(*value),
            DigitalScalar::Effect => Self::Effect,
        }
    }

    /// The value, owned.
    ///
    /// Called only where a value has to outlive the borrow it came from: an
    /// edge argument, which is read before the binding that overwrites its
    /// slot, and a resume state's, which outlives the whole activation.
    fn into_owned(self) -> DigitalScalar {
        match self {
            Self::FourState(value) => DigitalScalar::FourState(value.clone()),
            Self::Integer(value) => DigitalScalar::Integer(value),
            Self::Real(value) => DigitalScalar::Real(value),
            Self::Effect => DigitalScalar::Effect,
        }
    }
}

/// Read an operand as four-state, borrowing it where there is one to borrow.
///
/// An integer widens to its 32 bits, which is the width IEEE 1364-2005 section
/// 3.2.1 gives one; that is the only case with no value to borrow, and the only
/// reason this answers a [`Cow`]. The lowering produces integers only as
/// `#delay` operands, so it is a defensive conversion rather than a path the
/// current front end takes.
///
/// A free function rather than a method so that a caller which has already
/// borrowed the interpreter's fields apart — a write, which needs the
/// environment at the same time — can still read its operand.
fn four_state_in<'v>(
    function: &'v CfgFunction,
    table: &'v ValueTable,
    id: ValueId,
) -> Result<Cow<'v, FourStateValue>, DigitalEvalError> {
    match table.get(function, id)? {
        ScalarRef::FourState(value) => Ok(Cow::Borrowed(value)),
        ScalarRef::Integer(value) => Ok(Cow::Owned(FourStateValue::from_u64(
            32,
            u64::from(value as u32),
        ))),
        // Not converted. Section 3.7's conversion is an explicit system task,
        // and the lowering refuses every mix it can see, so a real arriving
        // here is a disagreement rather than a program.
        ScalarRef::Real(_) => Err(DigitalEvalError::MixedValueDomains(id)),
        ScalarRef::Effect => Err(DigitalEvalError::EffectValueRead(id)),
    }
}

/// The working set one activation needs, kept across activations.
///
/// Everything here is *storage*, never state. The execution contract is
/// unchanged: a `Wait` resumes as a `Jump` into an empty value table, and
/// [`ValueTable::enter`] empties the table at every entry so that a resumed
/// block still reads nothing but its own parameters. What the contract does
/// not require is that the storage be *re-obtained* from the allocator each
/// time, and a kernel running a million activations of a five-instruction
/// process should pay for it once.
///
/// A caller with no scratch of its own can keep using [`start`] and [`resume`],
/// which make a temporary one; a kernel should own one per host and hand it to
/// [`start_in`] and [`resume_in`].
#[derive(Debug, Default, Clone)]
pub struct DigitalEvalScratch {
    /// One slot per SSA value, emptied at every entry into a function.
    table: ValueTable,
    /// One control-flow edge's arguments, refilled per edge.
    arguments: Vec<DigitalScalar>,
    /// One concatenation's operands, refilled per node.
    operands: Vec<FourStateValue>,
    /// A spare sensitivity list for the next suspension to fill.
    terms: Vec<DigitalSensitivityTerm>,
    /// A spare argument list for the next suspension's resume state.
    resume: Vec<DigitalScalar>,
}

impl DigitalEvalScratch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Take back a sensitivity list a kernel has finished with.
    ///
    /// The list a suspension produced is the list the kernel subscribes with
    /// and then drops when the process wakes. Handed back here it becomes the
    /// buffer the *next* suspension fills, so a process that waits a million
    /// times allocates one list rather than a million.
    ///
    /// Keeps whichever buffer is larger, so the capacity settles at the
    /// widest sensitivity list the host has seen rather than oscillating.
    pub fn recycle_terms(&mut self, mut terms: Vec<DigitalSensitivityTerm>) {
        terms.clear();
        if terms.capacity() > self.terms.capacity() {
            self.terms = terms;
        }
    }

    /// Take back the argument list a resumption displaced.
    ///
    /// A resumption's own list arrives from the state and becomes the edge
    /// buffer; the buffer it displaces becomes the spare the *next* suspension
    /// fills, so the two lists trade places every activation and an `always`
    /// process allocates neither after its first pass. Keeps whichever is
    /// larger, for the reason [`recycle_terms`](Self::recycle_terms) does.
    fn recycle_resume(&mut self, mut arguments: Vec<DigitalScalar>) {
        arguments.clear();
        if arguments.capacity() > self.resume.capacity() {
            self.resume = arguments;
        }
    }
}

/// Run a process from its entry block.
pub fn start<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    environment: &mut E,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    start_with_limit(plan, process, environment, DEFAULT_PROCESS_STEP_LIMIT)
}

/// Run a process from its entry block, bounding how far it may go.
pub fn start_with_limit<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    environment: &mut E,
    step_limit: usize,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    start_in(
        plan,
        process,
        environment,
        &mut DigitalEvalScratch::new(),
        step_limit,
    )
}

/// Run a process from its entry block against a caller-owned working set.
pub fn start_in<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    environment: &mut E,
    scratch: &mut DigitalEvalScratch,
    step_limit: usize,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    let entry = process.function.entry;
    if !process.function.block(entry).params.is_empty() {
        return Err(DigitalEvalError::EntryBlockHasParameters(entry));
    }
    // An entry block takes no parameters, so there is nothing to bind and the
    // edge buffer starts empty.
    scratch.arguments.clear();
    Interpreter::new(plan, process, environment, scratch).run(entry, step_limit)
}

/// Resume a suspended process from a state a previous run produced.
pub fn resume<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    state: &DigitalResumeState,
    environment: &mut E,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    resume_with_limit(
        plan,
        process,
        state,
        environment,
        DEFAULT_PROCESS_STEP_LIMIT,
    )
}

/// Resume a suspended process, bounding how far it may go.
///
/// The state is validated against the function rather than trusted: a kernel
/// that stored a state, recompiled, and resumed into the new function would
/// otherwise re-enter an unrelated block with plausible-looking arguments.
pub fn resume_with_limit<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    state: &DigitalResumeState,
    environment: &mut E,
    step_limit: usize,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    resume_in(
        plan,
        process,
        state.clone(),
        environment,
        &mut DigitalEvalScratch::new(),
        step_limit,
    )
}

/// Resume a suspended process against a caller-owned working set.
///
/// The state arrives **by value**, which is the whole point: its argument list
/// is the one the previous suspension built, and taking ownership lets the same
/// allocation carry the next suspension's arguments back out. A kernel that
/// holds the state in a slot already has it by value — it takes it out of the
/// slot to run the process and puts a new one back.
pub fn resume_in<E: DigitalEnvironment + ?Sized>(
    plan: &CanonicalDigitalPlan,
    process: &CfgDigitalProcess,
    state: DigitalResumeState,
    environment: &mut E,
    scratch: &mut DigitalEvalScratch,
    step_limit: usize,
) -> Result<DigitalProcessOutcome, DigitalEvalError> {
    if state.process != process.id {
        return Err(DigitalEvalError::ResumeProcessMismatch {
            expected: process.id,
            found: state.process,
        });
    }
    let blocks = process.function.blocks.len();
    if usize::from(state.block) >= blocks {
        return Err(DigitalEvalError::ResumeBlockOutOfRange {
            block: state.block,
            blocks,
        });
    }
    let block = state.block;
    // The state's list becomes this entry's edge buffer, and the buffer it
    // displaces becomes the spare the next suspension fills.
    let displaced = std::mem::replace(&mut scratch.arguments, state.arguments);
    scratch.recycle_resume(displaced);
    Interpreter::new(plan, process, environment, scratch).run(block, step_limit)
}

struct Interpreter<'a, 's, E: ?Sized> {
    plan: &'a CanonicalDigitalPlan,
    process: &'a CfgDigitalProcess,
    environment: &'a mut E,
    /// The reused working set. Its value table is one slot per SSA value and is
    /// empty at every entry into the function — the generation bump in
    /// [`ValueTable::enter`] is what empties it.
    ///
    /// Emptied rather than carried across a suspension because a `Wait` resumes
    /// as a `Jump`: the only values a resumed block may read are its own
    /// parameters and whatever it computes. A table that survived would let a
    /// process read a value the lowering never routed through a parameter, and
    /// the bug would surface as a backend that disagreed.
    scratch: &'s mut DigitalEvalScratch,
}

impl<'a, 's, E: DigitalEnvironment + ?Sized> Interpreter<'a, 's, E> {
    fn new(
        plan: &'a CanonicalDigitalPlan,
        process: &'a CfgDigitalProcess,
        environment: &'a mut E,
        scratch: &'s mut DigitalEvalScratch,
    ) -> Self {
        // A generation bump rather than a clear: the observable state is the
        // same table an allocation would have produced — every slot empty —
        // and it costs one increment instead of one pass over the function's
        // whole value list.
        scratch.table.enter(process.function.values.len());
        Self {
            plan,
            process,
            environment,
            scratch,
        }
    }

    fn function(&self) -> &'a CfgFunction {
        &self.process.function
    }

    fn run(
        mut self,
        entry: BlockId,
        step_limit: usize,
    ) -> Result<DigitalProcessOutcome, DigitalEvalError> {
        let function = self.function();
        // The entry arguments are already in the edge buffer: empty for a
        // start, and the resumed state's own list for a resumption.
        self.bind(entry)?;

        let mut block = entry;
        for _ in 0..step_limit {
            for instruction in &function.block(block).instructions {
                let value = self.compute(instruction.result)?;
                self.scratch.table.define(instruction.result, value);
            }

            match &function.block(block).terminator {
                CfgTerminator::Return => return Ok(DigitalProcessOutcome::Finished),
                CfgTerminator::Unset => return Err(DigitalEvalError::UnterminatedBlock(block)),
                CfgTerminator::Jump { target, args } => {
                    self.cross(*target, args)?;
                    block = *target;
                }
                CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                } => {
                    // IEEE 1364-2005 section 9.4: only a known-true condition
                    // takes the first branch. `x` and `z` take the else, the
                    // same as a plain zero.
                    let taken = truth(&*self.four_state(*condition)?) == FourStateBit::One;
                    let (target, args) = if taken {
                        (*then_target, then_args)
                    } else {
                        (*else_target, else_args)
                    };
                    self.cross(target, args)?;
                    block = target;
                }
                CfgTerminator::Wait {
                    wait,
                    resume,
                    resume_args,
                } => {
                    let wait = match wait {
                        // Copied into the buffer a previous suspension's list
                        // was handed back as, rather than into a fresh one.
                        // `clone_from` reuses that capacity.
                        DigitalWait::Event(terms) => {
                            let mut spare = std::mem::take(&mut self.scratch.terms);
                            spare.clone_from(terms);
                            DigitalWaitRequest::Event(spare)
                        }
                        DigitalWait::Delay(delay) => {
                            DigitalWaitRequest::Delay(self.integer(*delay)?)
                        }
                    };
                    // Evaluated here, not at resumption: these are the values
                    // the process had when it suspended, and the signals they
                    // came from may have moved by the time it wakes.
                    //
                    // Filled into the spare list the last resumption put back,
                    // which leaves with the suspension and returns as the edge
                    // buffer when the process wakes.
                    let mut arguments = std::mem::take(&mut self.scratch.resume);
                    arguments.clear();
                    for arg in resume_args {
                        // Cloned rather than borrowed: these outlive the
                        // activation that computed them.
                        arguments.push(self.scalar(*arg)?.into_owned());
                    }
                    let params = function.block(*resume).params.len();
                    if arguments.len() != params {
                        let found = arguments.len();
                        // Handed back rather than dropped, so a refusal does
                        // not also cost the spare its capacity.
                        self.scratch.recycle_resume(arguments);
                        return Err(DigitalEvalError::ArgumentArityMismatch {
                            target: *resume,
                            expected: params,
                            found,
                        });
                    }
                    return Ok(DigitalProcessOutcome::Suspended(DigitalSuspension {
                        wait,
                        resume: DigitalResumeState {
                            process: self.process.id,
                            block: *resume,
                            arguments,
                        },
                    }));
                }
            }
        }
        Err(DigitalEvalError::StepLimitExceeded(step_limit))
    }

    /// Evaluate one edge's arguments into the reused buffer and bind them.
    ///
    /// Read every argument before writing any parameter: a back edge that
    /// carries two variables past each other passes each one's value from
    /// before the edge, and binding in place would feed the first write into
    /// the second read.
    ///
    /// One pass, and the buffer never leaves the scratch: the value table and
    /// the edge buffer are different fields, so destructuring the scratch once
    /// borrows them apart and the fill reads the table it is not writing.
    fn cross(&mut self, target: BlockId, args: &[ValueId]) -> Result<(), DigitalEvalError> {
        let function = self.function();
        let DigitalEvalScratch {
            table, arguments, ..
        } = &mut *self.scratch;
        arguments.clear();
        for arg in args {
            match table.get(function, *arg) {
                // Cloned because the buffer has to hold the value across the
                // binding that is about to overwrite the slot it came from.
                Ok(value) => arguments.push(value.into_owned()),
                Err(error) => {
                    // Nothing has been bound, so the refusal leaves the table
                    // exactly as it found it; the buffer is emptied so no
                    // argument outlives the edge that read it.
                    arguments.clear();
                    return Err(error);
                }
            }
        }
        self.bind(target)
    }

    /// Bind the edge buffer to a block's parameters.
    ///
    /// The arity is checked before any parameter is written, so a mismatch is
    /// refused with the table untouched.
    fn bind(&mut self, target: BlockId) -> Result<(), DigitalEvalError> {
        let params = &self.function().block(target).params;
        let DigitalEvalScratch {
            table, arguments, ..
        } = &mut *self.scratch;
        if params.len() != arguments.len() {
            let found = arguments.len();
            arguments.clear();
            return Err(DigitalEvalError::ArgumentArityMismatch {
                target,
                expected: params.len(),
                found,
            });
        }
        for (param, value) in params.iter().zip(arguments.drain(..)) {
            table.define(*param, value);
        }
        Ok(())
    }

    /// Read a value, borrowed.
    ///
    /// Shared rather than exclusive access, which is what lets an operator read
    /// both its operands at once without copying either.
    fn scalar(&self, id: ValueId) -> Result<ScalarRef<'_>, DigitalEvalError> {
        self.scratch.table.get(self.function(), id)
    }

    /// Read a value as four-state.
    fn four_state(&self, id: ValueId) -> Result<Cow<'_, FourStateValue>, DigitalEvalError> {
        four_state_in(self.function(), &self.scratch.table, id)
    }

    /// Read a value as a real, and only as one.
    fn real(&self, id: ValueId) -> Result<f64, DigitalEvalError> {
        match self.scalar(id)? {
            ScalarRef::Real(value) => Ok(value),
            ScalarRef::FourState(_) | ScalarRef::Integer(_) => {
                Err(DigitalEvalError::MixedValueDomains(id))
            }
            ScalarRef::Effect => Err(DigitalEvalError::EffectValueRead(id)),
        }
    }

    fn integer(&self, id: ValueId) -> Result<i64, DigitalEvalError> {
        match self.scalar(id)? {
            ScalarRef::Integer(value) => Ok(i64::from(value)),
            // A four-state delay operand with an unknown bit has no number to
            // wait for, and picking one would be inventing a schedule.
            ScalarRef::FourState(value) => value
                .to_u64()
                .and_then(|bits| i64::try_from(bits).ok())
                .ok_or(DigitalEvalError::NonIntegerDelay(id)),
            // A `#r` with a real operand would need section 3.9.2's rounding
            // and a ruling on what a fractional time unit is; neither is this
            // wave's, and a rounded delay is a schedule nobody wrote.
            ScalarRef::Real(_) => Err(DigitalEvalError::NonIntegerDelay(id)),
            ScalarRef::Effect => Err(DigitalEvalError::EffectValueRead(id)),
        }
    }

    fn signal(&self, id: DigitalSignalId) -> Result<&'a DigitalSignal, DigitalEvalError> {
        self.plan
            .signal(id)
            .ok_or(DigitalEvalError::UndeclaredSignal(id))
    }

    fn compute(&mut self, id: ValueId) -> Result<DigitalScalar, DigitalEvalError> {
        let kind = &self.function().value(id).kind;
        match kind {
            CfgValueKind::FourStateConstant(value) => Ok(DigitalScalar::FourState(value.clone())),
            CfgValueKind::IntegerConstant(value) => Ok(DigitalScalar::Integer(*value)),
            CfgValueKind::DigitalSignalRead { signal } => {
                let signal = self.signal(*signal)?;
                Ok(DigitalScalar::FourState(read_signal(
                    self.environment,
                    signal,
                )?))
            }
            // A real constant is the analog body's leaf too, and is shared
            // rather than duplicated: it reads nothing and means the same
            // number in both halves of the language. A block parameter is
            // shared for the same reason.
            CfgValueKind::RealConstant(value) => Ok(DigitalScalar::Real(*value)),
            CfgValueKind::DigitalRealSignalRead { signal } => {
                let id = *signal;
                // Asked of the plan first, so an id the plan does not declare
                // reports as that rather than as an environment miss.
                self.signal(id)?;
                let value = self
                    .environment
                    .read_real_signal(id)
                    .ok_or(DigitalEvalError::SignalUnavailable(id))?;
                Ok(DigitalScalar::Real(value))
            }
            // Verilog-AMS LRM 2.4 section 7.3.3's probe of a continuous net
            // from a discrete context. Asked of the plan first, for the reason
            // a signal read is: a probe the plan does not declare reports as
            // that rather than as an environment miss.
            //
            // Which value the environment hands back is section 7.3.6.3's
            // question, and it is deliberately not asked here — the
            // interpreter has no clock to compare the two domains' against.
            CfgValueKind::DigitalAnalogPotential { probe } => {
                let id = *probe;
                if self.plan.analog_probe(id).is_none() {
                    return Err(DigitalEvalError::UndeclaredAnalogProbe(id));
                }
                let value = self
                    .environment
                    .read_analog_potential(id)
                    .ok_or(DigitalEvalError::AnalogProbeUnavailable(id))?;
                Ok(DigitalScalar::Real(value))
            }
            CfgValueKind::DigitalRealArithmetic { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                let left = self.real(left)?;
                let right = self.real(right)?;
                Ok(DigitalScalar::Real(digital_value::real_arithmetic(
                    op, left, right,
                )))
            }
            CfgValueKind::DigitalRealCompare { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                let left = self.real(left)?;
                let right = self.real(right)?;
                Ok(DigitalScalar::FourState(digital_value::real_compare(
                    op, left, right,
                )))
            }
            // The two crossings the standard defines, and the only two nodes
            // in this interpreter whose operand and result are in different
            // value domains.
            CfgValueKind::DigitalRealToBits { input } => {
                let value = self.real(*input)?;
                Ok(DigitalScalar::FourState(FourStateValue::from_u64(
                    64,
                    value.to_bits(),
                )))
            }
            CfgValueKind::DigitalBitsToReal { input } => {
                let input = *input;
                let bits = self.four_state(input)?;
                // `to_u64` refuses a value with an unknown bit, which is
                // exactly the case this node has no answer for. Its `None` is
                // the refusal, reported under this node's own name rather than
                // turned into a number.
                let Some(bits) = bits.resized(64).to_u64() else {
                    return Err(DigitalEvalError::UnknownBitsToReal(input));
                };
                Ok(DigitalScalar::Real(f64::from_bits(bits)))
            }
            CfgValueKind::DigitalRealSelect {
                condition,
                then_value,
                else_value,
            } => {
                let (condition, then_value, else_value) = (*condition, *then_value, *else_value);
                // IEEE 1364-2005 section 9.4, not section 5.1.13. A real has no
                // bits for section 5.1.13's ambiguous-condition merge to
                // combine, so an `x` or `z` condition takes the `else` arm —
                // the rule this interpreter already applies at a `Branch`,
                // which is what keeps `c ? a : b` and the `if` it stands for
                // from disagreeing.
                let taken = truth(&*self.four_state(condition)?) == FourStateBit::One;
                // Both arms are still evaluated, so a refusal inside the arm
                // not taken is reported rather than hidden by the condition.
                let then_value = self.real(then_value)?;
                let else_value = self.real(else_value)?;
                Ok(DigitalScalar::Real(if taken {
                    then_value
                } else {
                    else_value
                }))
            }
            CfgValueKind::DigitalBitwise { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                let left = self.four_state(left)?;
                let right = self.four_state(right)?;
                Ok(DigitalScalar::FourState(digital_value::bitwise(
                    op, &left, &right,
                )))
            }
            CfgValueKind::DigitalBitwiseNot { input } => {
                let input = self.four_state(*input)?;
                Ok(DigitalScalar::FourState(digital_value::bitwise_not(&input)))
            }
            CfgValueKind::DigitalLogical { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                let left = self.four_state(left)?;
                let right = self.four_state(right)?;
                Ok(DigitalScalar::FourState(digital_value::logical(
                    op, &left, &right,
                )))
            }
            CfgValueKind::DigitalLogicalNot { input } => {
                let input = self.four_state(*input)?;
                Ok(DigitalScalar::FourState(digital_value::logical_not(&input)))
            }
            CfgValueKind::DigitalEquality {
                left,
                right,
                negate,
                signed,
            } => {
                let (left, right, negate, signed) = (*left, *right, *negate, *signed);
                let left = self.four_state(left)?;
                let right = self.four_state(right)?;
                Ok(DigitalScalar::FourState(digital_value::equality(
                    &left, &right, negate, signed,
                )))
            }
            CfgValueKind::DigitalCaseMatch {
                selector,
                label,
                kind,
                signed,
            } => {
                let (selector, label, kind, signed) = (*selector, *label, *kind, *signed);
                let selector = self.four_state(selector)?;
                let label = self.four_state(label)?;
                Ok(DigitalScalar::FourState(digital_value::case_match(
                    kind, &selector, &label, signed,
                )))
            }
            CfgValueKind::DigitalRelational {
                op,
                left,
                right,
                signed,
            } => {
                let (op, left, right, signed) = (*op, *left, *right, *signed);
                let left = self.four_state(left)?;
                let right = self.four_state(right)?;
                Ok(DigitalScalar::FourState(digital_value::relational(
                    op, &left, &right, signed,
                )))
            }
            CfgValueKind::DigitalArithmetic {
                op,
                left,
                right,
                signed,
            } => {
                let (op, left, right, signed) = (*op, *left, *right, *signed);
                let left = self.four_state(left)?;
                let right = self.four_state(right)?;
                Ok(DigitalScalar::FourState(digital_value::arithmetic(
                    op, &left, &right, signed,
                )))
            }
            CfgValueKind::DigitalShift { op, value, count } => {
                let (op, value, count) = (*op, *value, *count);
                let value = self.four_state(value)?;
                let count = self.four_state(count)?;
                Ok(DigitalScalar::FourState(digital_value::shift(
                    op, &value, &count,
                )))
            }
            CfgValueKind::DigitalPartSelect { input, msb, lsb } => {
                let (input, msb, lsb) = (*input, *msb, *lsb);
                let input = self.four_state(input)?;
                Ok(DigitalScalar::FourState(digital_value::part_select(
                    &input, msb, lsb,
                )))
            }
            CfgValueKind::DigitalConcat { parts } => {
                // The operand list is read straight out of the function, which
                // outlives this call, and gathered into the reused buffer
                // rather than a fresh one. The buffer and the value table are
                // different fields of the scratch, so borrowing them apart is
                // what lets the gather read operands while it fills.
                let function = self.function();
                let DigitalEvalScratch {
                    table, operands, ..
                } = &mut *self.scratch;
                operands.clear();
                let mut failure = None;
                for part in parts {
                    match four_state_in(function, table, *part) {
                        Ok(value) => operands.push(value.into_owned()),
                        Err(error) => {
                            failure = Some(error);
                            break;
                        }
                    }
                }
                let outcome = match failure {
                    Some(error) => Err(error),
                    None => Ok(DigitalScalar::FourState(digital_value::concat(
                        operands.as_slice(),
                    ))),
                };
                operands.clear();
                outcome
            }
            CfgValueKind::DigitalSelect {
                condition,
                then_value,
                else_value,
            } => {
                let (condition, then_value, else_value) = (*condition, *then_value, *else_value);
                let condition = self.four_state(condition)?;
                let then_value = self.four_state(then_value)?;
                let else_value = self.four_state(else_value)?;
                Ok(DigitalScalar::FourState(digital_value::conditional(
                    &condition,
                    &then_value,
                    &else_value,
                )))
            }
            CfgValueKind::DigitalBlockingWrite { target, value } => {
                let value = *value;
                // One write node for both domains, the way
                // [`CfgValueKind::DigitalDriverWrite`] is: what the written
                // name carries is a property of the declaration, and the plan
                // recorded it.
                if self.signal(target.signal)?.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment.write_real_signal(target.signal, value);
                    return Ok(DigitalScalar::Effect);
                }
                // The plan, the environment and the value table are three
                // fields, so the write borrows them apart rather than taking a
                // copy of the value to hand the store.
                let Interpreter {
                    plan,
                    process,
                    environment,
                    scratch,
                } = self;
                let value = four_state_in(&process.function, &scratch.table, value)?;
                apply_write(plan, &mut **environment, target, &value)?;
                Ok(DigitalScalar::Effect)
            }
            CfgValueKind::DigitalDriverWrite {
                driver,
                target,
                value,
            } => {
                let (driver, value) = (*driver, *value);
                // One write node for both domains. What the driven net carries
                // is a property of the net, and the plan already recorded it —
                // reading it here is what keeps `assign` one construct rather
                // than two that could drift apart.
                let signal = self.signal(target.signal)?;
                if signal.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment
                        .drive_real_signal(DigitalRealDrive { driver, value });
                    return Ok(DigitalScalar::Effect);
                }
                // Resized here for the same reason a nonblocking update is:
                // section 5.2.1's width belongs to the assignment, and the
                // assignment is here. What the kernel later does with the
                // contribution cannot recover a width it was not given.
                let width = target_width(signal, &target.select);
                let Interpreter {
                    process,
                    environment,
                    scratch,
                    ..
                } = self;
                let value = four_state_in(&process.function, &scratch.table, value)?;
                environment.drive_signal(DigitalDrive {
                    driver,
                    target: target.clone(),
                    value: value.resized(width),
                });
                Ok(DigitalScalar::Effect)
            }
            CfgValueKind::DigitalNonblockingWrite {
                target,
                value,
                region,
            } => {
                let (value, region) = (*value, *region);
                let signal = self.signal(target.signal)?;
                if signal.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment.defer_update(DigitalDeferredUpdate {
                        target: target.clone(),
                        value: DigitalUpdate::Real(value),
                        region,
                    });
                    return Ok(DigitalScalar::Effect);
                }
                // Resized here, where the assignment is, rather than at the
                // flush: section 5.2.1's width is the target's, and the target
                // is known now.
                let width = target_width(signal, &target.select);
                let Interpreter {
                    process,
                    environment,
                    scratch,
                    ..
                } = self;
                let value = four_state_in(&process.function, &scratch.table, value)?;
                environment.defer_update(DigitalDeferredUpdate {
                    target: target.clone(),
                    value: DigitalUpdate::FourState(value.resized(width)),
                    region,
                });
                Ok(DigitalScalar::Effect)
            }
            // A block parameter is defined by the edge that entered the block,
            // so reaching it here means nothing bound it.
            CfgValueKind::BlockParameter => Err(DigitalEvalError::UndefinedValue(id)),
            _ => Err(DigitalEvalError::AnalogValueInProcess(id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::cfg::{CfgBlock, CfgInstruction, CfgValue, CfgValueType};
    use super::super::diagnostic::SourceSpanRef;
    use super::super::digital::{DigitalProcessKind, DigitalSignalKind};
    use super::*;
    use crate::four_state::FourStateBit::{
        HighImpedance as Z, One as ONE, Unknown as X, Zero as ZERO,
    };

    /// A store with nothing in it, for the process tests that read no signal.
    struct NoEnvironment;

    impl DigitalEnvironment for NoEnvironment {
        fn read_signal(&self, _signal: DigitalSignalId) -> Option<FourStateValue> {
            None
        }

        fn write_signal(&mut self, _signal: DigitalSignalId, _value: FourStateValue) {}

        fn defer_update(&mut self, _update: DigitalDeferredUpdate) {}

        fn write_real_signal(&mut self, _signal: DigitalSignalId, _value: f64) {}

        fn read_real_signal(&self, _signal: DigitalSignalId) -> Option<f64> {
            None
        }

        fn read_analog_potential(&self, _probe: DigitalAnalogProbeId) -> Option<f64> {
            None
        }

        fn drive_real_signal(&mut self, _drive: DigitalRealDrive) {}

        fn drive_signal(&mut self, _drive: DigitalDrive) {}
    }

    /// A process that computes a value, suspends, and then reads that value in
    /// the block it resumes into.
    ///
    /// The lowering never emits this — a value that has to survive a suspension
    /// travels as a resume argument, bound to a block parameter — which is the
    /// point: it is the shape the execution contract forbids, built by hand so
    /// the interpreter can be asked what it does with one.
    fn process_reading_across_a_suspension() -> CfgDigitalProcess {
        let bit = CfgValueType::FourState { width: 1 };
        let scalar = |index: usize, kind| CfgValue {
            id: ValueId::from(index),
            value_type: bit,
            kind,
        };
        let values = vec![
            scalar(0, CfgValueKind::FourStateConstant(value("0"))),
            scalar(
                1,
                CfgValueKind::DigitalBitwiseNot {
                    input: ValueId::from(0usize),
                },
            ),
            scalar(
                2,
                CfgValueKind::DigitalBitwiseNot {
                    input: ValueId::from(1usize),
                },
            ),
        ];
        let blocks = vec![
            CfgBlock {
                id: BlockId::from(0usize),
                params: Vec::new(),
                instructions: vec![CfgInstruction {
                    result: ValueId::from(1usize),
                }],
                terminator: CfgTerminator::Wait {
                    wait: DigitalWait::Event(vec![DigitalSensitivityTerm {
                        signal: DigitalSignalId::from(0usize),
                        edge: None,
                    }]),
                    resume: BlockId::from(1usize),
                    resume_args: Vec::new(),
                },
            },
            CfgBlock {
                id: BlockId::from(1usize),
                params: Vec::new(),
                // Reads the value the block before the suspension computed,
                // and nothing bound it as a parameter.
                instructions: vec![CfgInstruction {
                    result: ValueId::from(2usize),
                }],
                terminator: CfgTerminator::Return,
            },
        ];
        CfgDigitalProcess {
            id: DigitalProcessId::from(0usize),
            kind: DigitalProcessKind::Always,
            function: CfgFunction {
                entry: BlockId::from(0usize),
                blocks,
                values,
                shapes: Vec::new(),
            },
            static_sensitivity: None,
            span: SourceSpanRef {
                source_file_id: 0,
                start: 0,
                end: 0,
            },
        }
    }

    /// The execution contract, pinned: a `Wait` resumes as a `Jump` into an
    /// *empty* value table, so a value computed before the suspension is
    /// undefined after it.
    ///
    /// It is pinned because the table no longer empties itself by dropping what
    /// it holds — a generation stamp decides what is readable, and the previous
    /// activation's values stay in their slots. A stamp that survived the entry
    /// would let this process run to completion, and a lowering that lost a
    /// block argument would pass here and disagree with every other backend.
    #[test]
    fn a_value_computed_before_a_suspension_is_undefined_after_it() {
        let plan = CanonicalDigitalPlan::default();
        let process = process_reading_across_a_suspension();
        let mut environment = NoEnvironment;
        let mut scratch = DigitalEvalScratch::new();

        let outcome = start_in(
            &plan,
            &process,
            &mut environment,
            &mut scratch,
            DEFAULT_PROCESS_STEP_LIMIT,
        )
        .expect("the process suspends at its first wait");
        let DigitalProcessOutcome::Suspended(suspension) = outcome else {
            panic!("an event wait suspends");
        };

        let (_, state) = suspension.into_parts();
        let error = resume_in(
            &plan,
            &process,
            state,
            &mut environment,
            &mut scratch,
            DEFAULT_PROCESS_STEP_LIMIT,
        )
        .expect_err("the resumed block may not read what the suspended one computed");
        assert_eq!(
            error,
            DigitalEvalError::UndefinedValue(ValueId::from(1usize))
        );
    }

    /// The generation is a `u32`, so an interpreter that only ever bumped it
    /// would, after four billion entries, hand a fresh activation a number some
    /// slot already carries and revive that slot. It wraps by clearing every
    /// stamp and restarting at one instead.
    #[test]
    fn a_wrapped_generation_revives_no_slot() {
        let process = process_reading_across_a_suspension();
        let slot = ValueId::from(1usize);
        let mut table = ValueTable::default();

        table.enter(process.function.values.len());
        assert_eq!(table.current, 1);
        table.define(slot, DigitalScalar::Integer(7));
        assert_eq!(table.defined(slot), Some(&DigitalScalar::Integer(7)));

        // Four billion entries later, on the last generation a `u32` counts.
        table.current = u32::MAX;
        table.enter(process.function.values.len());

        // Back to one — and the slot stamped with the *first* generation one is
        // not readable in the second, which is what the reset buys.
        assert_eq!(table.current, 1);
        assert_eq!(table.defined(slot), None);
    }

    fn value(spelling: &str) -> FourStateValue {
        let bits: Vec<FourStateBit> = spelling
            .chars()
            .map(|character| match character {
                '0' => ZERO,
                '1' => ONE,
                'x' => X,
                'z' => Z,
                other => panic!("not a four-state digit: {other}"),
            })
            .collect();
        FourStateValue::from_bits_msb_first(&bits)
    }

    /// The edge table and the section 4.1 truth tables must agree about which
    /// column is which bit, or a reordering of [`FourStateBit`] permutes one and
    /// not the other.
    #[test]
    fn the_edge_table_is_indexed_like_the_truth_tables() {
        for (index, bit) in digital_value::TABLE_ORDER.into_iter().enumerate() {
            assert_eq!(edge_index(bit), index);
        }
    }

    /// All sixteen transitions, against IEEE 1364-2005 table 5-2. Written out
    /// rather than generated: the table is the specification, and a test that
    /// re-derived it would agree with a wrong transcription.
    #[test]
    fn every_transition_classifies_as_the_standard_says() {
        let posedge = Some(DigitalEdge::Posedge);
        let negedge = Some(DigitalEdge::Negedge);

        // Towards 1 is a posedge, including from an ambiguous value.
        assert_eq!(classify_edge(ZERO, ONE), posedge);
        assert_eq!(classify_edge(ZERO, X), posedge);
        assert_eq!(classify_edge(ZERO, Z), posedge);
        assert_eq!(classify_edge(X, ONE), posedge);
        assert_eq!(classify_edge(Z, ONE), posedge);

        // Towards 0 is a negedge, symmetrically.
        assert_eq!(classify_edge(ONE, ZERO), negedge);
        assert_eq!(classify_edge(ONE, X), negedge);
        assert_eq!(classify_edge(ONE, Z), negedge);
        assert_eq!(classify_edge(X, ZERO), negedge);
        assert_eq!(classify_edge(Z, ZERO), negedge);

        // No movement, and movement between two ambiguous values, are neither.
        assert_eq!(classify_edge(ZERO, ZERO), None);
        assert_eq!(classify_edge(ONE, ONE), None);
        assert_eq!(classify_edge(X, X), None);
        assert_eq!(classify_edge(Z, Z), None);
        assert_eq!(classify_edge(X, Z), None);
        assert_eq!(classify_edge(Z, X), None);
    }

    /// Every entry is covered by the case analysis above: five posedges, five
    /// negedges, six that are neither.
    #[test]
    fn the_edge_table_has_the_expected_census() {
        let mut posedges = 0;
        let mut negedges = 0;
        let mut neither = 0;
        for previous in digital_value::TABLE_ORDER {
            for next in digital_value::TABLE_ORDER {
                match classify_edge(previous, next) {
                    Some(DigitalEdge::Posedge) => posedges += 1,
                    Some(DigitalEdge::Negedge) => negedges += 1,
                    None => neither += 1,
                }
            }
        }
        assert_eq!((posedges, negedges, neither), (5, 5, 6));
    }

    /// An edge on a vector is an edge on its least significant bit, IEEE
    /// 1364-2005 section 9.7.2 — the other bits may do anything.
    #[test]
    fn an_edge_term_watches_the_least_significant_bit() {
        let term = DigitalSensitivityTerm {
            signal: DigitalSignalId::from(0usize),
            edge: Some(DigitalEdge::Posedge),
        };
        assert!(term_is_satisfied(&term, &value("0000"), &value("1111")));
        assert!(term_is_satisfied(&term, &value("1110"), &value("0001")));
        // The LSB did not move, however much the rest of the word did.
        assert!(!term_is_satisfied(&term, &value("0000"), &value("1110")));
    }

    /// A level-sensitive term fires on a change of value and on nothing else,
    /// which is what stops a rewrite of the same value from re-triggering a
    /// combinational process.
    #[test]
    fn a_level_term_fires_on_any_change_and_not_on_a_rewrite() {
        let term = DigitalSensitivityTerm {
            signal: DigitalSignalId::from(0usize),
            edge: None,
        };
        assert!(term_is_satisfied(&term, &value("00"), &value("01")));
        assert!(term_is_satisfied(&term, &value("01"), &value("0x")));
        assert!(term_is_satisfied(&term, &value("0x"), &value("0z")));
        assert!(!term_is_satisfied(&term, &value("01"), &value("01")));
        assert!(!term_is_satisfied(&term, &value("0x"), &value("0x")));
    }

    /// A list only answers for the signal that changed.
    #[test]
    fn a_sensitivity_list_ignores_terms_for_other_signals() {
        let clock = DigitalSignalId::from(0usize);
        let reset = DigitalSignalId::from(1usize);
        let terms = [
            DigitalSensitivityTerm {
                signal: clock,
                edge: Some(DigitalEdge::Posedge),
            },
            DigitalSensitivityTerm {
                signal: reset,
                edge: Some(DigitalEdge::Negedge),
            },
        ];
        assert!(any_term_is_satisfied(
            &terms,
            clock,
            &value("0"),
            &value("1")
        ));
        // A rising edge on the signal the list watches for a falling one.
        assert!(!any_term_is_satisfied(
            &terms,
            reset,
            &value("0"),
            &value("1")
        ));
        assert!(any_term_is_satisfied(
            &terms,
            reset,
            &value("1"),
            &value("0")
        ));
        // A signal no term names.
        assert!(!any_term_is_satisfied(
            &terms,
            DigitalSignalId::from(2usize),
            &value("0"),
            &value("1")
        ));
    }

    /// A plan with one four-state signal declared over `bounds`.
    fn one_signal_plan(bounds: Option<(i64, i64)>, width: u32) -> CanonicalDigitalPlan {
        CanonicalDigitalPlan {
            signals: vec![DigitalSignal {
                id: DigitalSignalId::from(0usize),
                name: "q".into(),
                kind: DigitalSignalKind::FourState,
                width,
                bounds,
                signed: false,
                procedurally_assignable: true,
                span: SourceSpanRef {
                    source_file_id: 0,
                    start: 0,
                    end: 0,
                },
            }],
            ..CanonicalDigitalPlan::default()
        }
    }

    /// One signal's value, the smallest environment `apply_write` needs.
    struct OneSignal(FourStateValue);

    impl DigitalEnvironment for OneSignal {
        fn read_signal(&self, _signal: DigitalSignalId) -> Option<FourStateValue> {
            Some(self.0.clone())
        }

        fn write_signal(&mut self, _signal: DigitalSignalId, value: FourStateValue) {
            self.0 = value;
        }

        fn defer_update(&mut self, _update: DigitalDeferredUpdate) {}

        fn write_real_signal(&mut self, _signal: DigitalSignalId, _value: f64) {}

        fn read_real_signal(&self, _signal: DigitalSignalId) -> Option<f64> {
            None
        }

        fn read_analog_potential(&self, _probe: DigitalAnalogProbeId) -> Option<f64> {
            None
        }

        fn drive_real_signal(&mut self, _drive: DigitalRealDrive) {}

        fn drive_signal(&mut self, _drive: DigitalDrive) {}
    }

    fn write(
        plan: &CanonicalDigitalPlan,
        start: &str,
        select: DigitalWriteSelect,
        written: &str,
    ) -> String {
        let mut environment = OneSignal(value(start));
        apply_write(
            plan,
            &mut environment,
            &DigitalWriteTarget {
                signal: DigitalSignalId::from(0usize),
                select,
            },
            &value(written),
        )
        .expect("the signal is declared");
        environment.0.spelling()
    }

    /// A partial write lands on the bit the declaration *names*, IEEE
    /// 1364-2005 section 3.3.1 read together with section 5.2.1.
    ///
    /// `reg [7:4] q` is four bits called 7, 6, 5 and 4. Before this rule
    /// existed, `q[7] = 1` wrote position 7 of a four-bit value and did
    /// nothing at all, and `q[4] = 1` wrote position 4 and did nothing
    /// either — a whole declaration whose every bit was unwritable.
    #[test]
    fn a_partial_write_lands_on_the_bit_the_declaration_names() {
        let high = one_signal_plan(Some((7, 4)), 4);
        assert_eq!(
            write(&high, "0000", DigitalWriteSelect::Bit(7), "1"),
            "1000"
        );
        assert_eq!(
            write(&high, "0000", DigitalWriteSelect::Bit(4), "1"),
            "0001"
        );
        assert_eq!(
            write(
                &high,
                "0000",
                DigitalWriteSelect::Part { msb: 6, lsb: 5 },
                "11"
            ),
            "0110"
        );

        // Ascending, where the *left* bound is still the most significant bit.
        let ascending = one_signal_plan(Some((4, 7)), 4);
        assert_eq!(
            write(&ascending, "0000", DigitalWriteSelect::Bit(4), "1"),
            "1000"
        );
        assert_eq!(
            write(&ascending, "0000", DigitalWriteSelect::Bit(7), "1"),
            "0001"
        );
        assert_eq!(
            write(
                &ascending,
                "0000",
                DigitalWriteSelect::Part { msb: 5, lsb: 6 },
                "11"
            ),
            "0110"
        );

        // A range anchored at zero is the identity, in both directions.
        let descending = one_signal_plan(Some((3, 0)), 4);
        assert_eq!(
            write(&descending, "0000", DigitalWriteSelect::Bit(3), "1"),
            "1000"
        );
        let zero_ascending = one_signal_plan(Some((0, 3)), 4);
        assert_eq!(
            write(&zero_ascending, "0000", DigitalWriteSelect::Bit(0), "1"),
            "1000"
        );

        // A one-bit vector, and a scalar with no range at all.
        let single = one_signal_plan(Some((3, 3)), 1);
        assert_eq!(write(&single, "0", DigitalWriteSelect::Bit(3), "1"), "1");
        let scalar = one_signal_plan(None, 1);
        assert_eq!(write(&scalar, "0", DigitalWriteSelect::Bit(0), "1"), "1");
    }

    /// A write to a bit the declaration does not name changes nothing, and a
    /// part select that only partly overlaps writes only the bits it names.
    ///
    /// Section 5.2.1's rule for an out-of-range left-hand side. The front end
    /// refuses a constant select outside the declared bounds before it ever
    /// reaches here, so this is what the kernel does with a plan that was not
    /// built by that front end — and it is the reason the position map is an
    /// affine one rather than an absolute distance, which would have folded
    /// `q[3]` of a `[7:4]` reg onto `q[5]`.
    #[test]
    fn a_write_outside_the_declared_range_changes_nothing() {
        let high = one_signal_plan(Some((7, 4)), 4);
        assert_eq!(
            write(&high, "0000", DigitalWriteSelect::Bit(3), "1"),
            "0000"
        );
        assert_eq!(
            write(&high, "0000", DigitalWriteSelect::Bit(8), "1"),
            "0000"
        );

        // `q[5:2]`: two named bits and two that are not, so the value's top
        // half lands at positions 1 and 0 and its bottom half is dropped.
        assert_eq!(
            write(
                &high,
                "0000",
                DigitalWriteSelect::Part { msb: 5, lsb: 2 },
                "1100"
            ),
            "0011"
        );
    }
}
