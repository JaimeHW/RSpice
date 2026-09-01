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

use super::cfg::{CfgFunction, CfgTerminator, CfgValueKind, DigitalWait, is_leaf_kind};
use super::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalDriverId, DigitalEdge, DigitalSchedulingRegion,
    DigitalSensitivityTerm, DigitalSignal, DigitalWriteSelect, DigitalWriteTarget,
};
use super::digital_value::{self, FourStateValue, truth};
use super::ids::{BlockId, DigitalProcessId, DigitalSignalId, ValueId};
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
        DigitalWriteSelect::Bit(index) => {
            let mut current = read_signal(environment, signal)?;
            patch(&mut current, *index, &value);
            current
        }
        DigitalWriteSelect::Part { msb, lsb } => {
            let mut current = read_signal(environment, signal)?;
            patch(&mut current, *msb.min(lsb), &value);
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

/// Overwrite the bits of `current` starting at declared index `low`.
///
/// Indices are positions from the least significant bit, which is what the
/// lowering emits and what [`digital_value::part_select`] reads back. A signal
/// declared with ascending bounds is therefore written the same way it is read;
/// see this module's note in the crate's digital documentation.
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
    let entry = process.function.entry;
    if !process.function.block(entry).params.is_empty() {
        return Err(DigitalEvalError::EntryBlockHasParameters(entry));
    }
    Interpreter::new(plan, process, environment).run(entry, Vec::new(), step_limit)
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
    Interpreter::new(plan, process, environment).run(
        state.block,
        state.arguments.clone(),
        step_limit,
    )
}

struct Interpreter<'a, E: ?Sized> {
    plan: &'a CanonicalDigitalPlan,
    process: &'a CfgDigitalProcess,
    environment: &'a mut E,
    /// One slot per SSA value, empty at every entry into the function.
    ///
    /// Emptied rather than carried across a suspension because a `Wait` resumes
    /// as a `Jump`: the only values a resumed block may read are its own
    /// parameters and whatever it computes. A table that survived would let a
    /// process read a value the lowering never routed through a parameter, and
    /// the bug would surface as a backend that disagreed.
    values: Vec<Option<DigitalScalar>>,
}

impl<'a, E: DigitalEnvironment + ?Sized> Interpreter<'a, E> {
    fn new(
        plan: &'a CanonicalDigitalPlan,
        process: &'a CfgDigitalProcess,
        environment: &'a mut E,
    ) -> Self {
        Self {
            plan,
            process,
            environment,
            values: vec![None; process.function.values.len()],
        }
    }

    fn function(&self) -> &'a CfgFunction {
        &self.process.function
    }

    fn run(
        mut self,
        entry: BlockId,
        arguments: Vec<DigitalScalar>,
        step_limit: usize,
    ) -> Result<DigitalProcessOutcome, DigitalEvalError> {
        let function = self.function();
        self.bind(entry, arguments)?;

        let mut block = entry;
        for _ in 0..step_limit {
            for instruction in &function.block(block).instructions {
                let value = self.compute(instruction.result)?;
                self.values[usize::from(instruction.result)] = Some(value);
            }

            match &function.block(block).terminator {
                CfgTerminator::Return => return Ok(DigitalProcessOutcome::Finished),
                CfgTerminator::Unset => return Err(DigitalEvalError::UnterminatedBlock(block)),
                CfgTerminator::Jump { target, args } => {
                    let arguments = self.evaluate_arguments(args)?;
                    self.bind(*target, arguments)?;
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
                    let taken = truth(&self.four_state(*condition)?) == FourStateBit::One;
                    let (target, args) = if taken {
                        (*then_target, then_args)
                    } else {
                        (*else_target, else_args)
                    };
                    let arguments = self.evaluate_arguments(args)?;
                    self.bind(target, arguments)?;
                    block = target;
                }
                CfgTerminator::Wait {
                    wait,
                    resume,
                    resume_args,
                } => {
                    let wait = match wait {
                        DigitalWait::Event(terms) => DigitalWaitRequest::Event(terms.clone()),
                        DigitalWait::Delay(delay) => {
                            DigitalWaitRequest::Delay(self.integer(*delay)?)
                        }
                    };
                    // Evaluated here, not at resumption: these are the values
                    // the process had when it suspended, and the signals they
                    // came from may have moved by the time it wakes.
                    let arguments = self.evaluate_arguments(resume_args)?;
                    let params = function.block(*resume).params.len();
                    if arguments.len() != params {
                        return Err(DigitalEvalError::ArgumentArityMismatch {
                            target: *resume,
                            expected: params,
                            found: arguments.len(),
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

    /// Read every argument before writing any parameter.
    ///
    /// A back edge that carries two variables past each other passes each one's
    /// value from before the edge, and binding in place would feed the first
    /// write into the second read.
    fn evaluate_arguments(
        &mut self,
        args: &[ValueId],
    ) -> Result<Vec<DigitalScalar>, DigitalEvalError> {
        args.iter().map(|arg| self.read(*arg)).collect()
    }

    fn bind(
        &mut self,
        target: BlockId,
        arguments: Vec<DigitalScalar>,
    ) -> Result<(), DigitalEvalError> {
        let params = &self.function().block(target).params;
        if params.len() != arguments.len() {
            return Err(DigitalEvalError::ArgumentArityMismatch {
                target,
                expected: params.len(),
                found: arguments.len(),
            });
        }
        for (param, value) in params.iter().zip(arguments) {
            self.values[usize::from(*param)] = Some(value);
        }
        Ok(())
    }

    fn read(&mut self, id: ValueId) -> Result<DigitalScalar, DigitalEvalError> {
        if let Some(value) = &self.values[usize::from(id)] {
            return Ok(value.clone());
        }
        // Constants belong to no block — a `#delay` operand is pushed as a leaf
        // because the `Wait` that consumes it is a terminator — so a miss asks
        // whether the value is one before reporting it undefined.
        if !is_leaf_kind(&self.function().value(id).kind) {
            return Err(DigitalEvalError::UndefinedValue(id));
        }
        let value = self.compute(id)?;
        self.values[usize::from(id)] = Some(value.clone());
        Ok(value)
    }

    /// Read a value as four-state.
    ///
    /// An integer widens to its 32 bits, which is the width IEEE 1364-2005
    /// section 3.2.1 gives one. The lowering produces integers only as `#delay`
    /// operands, so this is a defensive conversion rather than a path the
    /// current front end takes.
    fn four_state(&mut self, id: ValueId) -> Result<FourStateValue, DigitalEvalError> {
        match self.read(id)? {
            DigitalScalar::FourState(value) => Ok(value),
            DigitalScalar::Integer(value) => {
                Ok(FourStateValue::from_u64(32, u64::from(value as u32)))
            }
            // Not converted. Section 3.7's conversion is an explicit system
            // task, and the lowering refuses every mix it can see, so a real
            // arriving here is a disagreement rather than a program.
            DigitalScalar::Real(_) => Err(DigitalEvalError::MixedValueDomains(id)),
            DigitalScalar::Effect => Err(DigitalEvalError::EffectValueRead(id)),
        }
    }

    /// Read a value as a real, and only as one.
    fn real(&mut self, id: ValueId) -> Result<f64, DigitalEvalError> {
        match self.read(id)? {
            DigitalScalar::Real(value) => Ok(value),
            DigitalScalar::FourState(_) | DigitalScalar::Integer(_) => {
                Err(DigitalEvalError::MixedValueDomains(id))
            }
            DigitalScalar::Effect => Err(DigitalEvalError::EffectValueRead(id)),
        }
    }

    fn integer(&mut self, id: ValueId) -> Result<i64, DigitalEvalError> {
        match self.read(id)? {
            DigitalScalar::Integer(value) => Ok(i64::from(value)),
            // A four-state delay operand with an unknown bit has no number to
            // wait for, and picking one would be inventing a schedule.
            DigitalScalar::FourState(value) => value
                .to_u64()
                .and_then(|bits| i64::try_from(bits).ok())
                .ok_or(DigitalEvalError::NonIntegerDelay(id)),
            // A `#r` with a real operand would need section 3.9.2's rounding
            // and a ruling on what a fractional time unit is; neither is this
            // wave's, and a rounded delay is a schedule nobody wrote.
            DigitalScalar::Real(_) => Err(DigitalEvalError::NonIntegerDelay(id)),
            DigitalScalar::Effect => Err(DigitalEvalError::EffectValueRead(id)),
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
                let taken = truth(&self.four_state(condition)?) == FourStateBit::One;
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
                let parts = parts.clone();
                let mut values = Vec::with_capacity(parts.len());
                for part in parts {
                    values.push(self.four_state(part)?);
                }
                Ok(DigitalScalar::FourState(digital_value::concat(&values)))
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
                let (target, value) = (target.clone(), *value);
                // One write node for both domains, the way
                // [`CfgValueKind::DigitalDriverWrite`] is: what the written
                // name carries is a property of the declaration, and the plan
                // recorded it.
                if self.signal(target.signal)?.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment.write_real_signal(target.signal, value);
                    return Ok(DigitalScalar::Effect);
                }
                let value = self.four_state(value)?;
                apply_write(self.plan, self.environment, &target, &value)?;
                Ok(DigitalScalar::Effect)
            }
            CfgValueKind::DigitalDriverWrite {
                driver,
                target,
                value,
            } => {
                let (driver, target, value) = (*driver, target.clone(), *value);
                // One write node for both domains. What the driven net carries
                // is a property of the net, and the plan already recorded it —
                // reading it here is what keeps `assign` one construct rather
                // than two that could drift apart.
                if self.signal(target.signal)?.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment
                        .drive_real_signal(DigitalRealDrive { driver, value });
                    return Ok(DigitalScalar::Effect);
                }
                let value = self.four_state(value)?;
                // Resized here for the same reason a nonblocking update is:
                // section 5.2.1's width belongs to the assignment, and the
                // assignment is here. What the kernel later does with the
                // contribution cannot recover a width it was not given.
                let signal = self.signal(target.signal)?;
                let width = target_width(signal, &target.select);
                self.environment.drive_signal(DigitalDrive {
                    driver,
                    target,
                    value: value.resized(width),
                });
                Ok(DigitalScalar::Effect)
            }
            CfgValueKind::DigitalNonblockingWrite {
                target,
                value,
                region,
            } => {
                let (target, value, region) = (target.clone(), *value, *region);
                if self.signal(target.signal)?.kind.is_real() {
                    let value = self.real(value)?;
                    self.environment.defer_update(DigitalDeferredUpdate {
                        target,
                        value: DigitalUpdate::Real(value),
                        region,
                    });
                    return Ok(DigitalScalar::Effect);
                }
                let value = self.four_state(value)?;
                // Resized here, where the assignment is, rather than at the
                // flush: section 5.2.1's width is the target's, and the target
                // is known now.
                let signal = self.signal(target.signal)?;
                let width = target_width(signal, &target.select);
                self.environment.defer_update(DigitalDeferredUpdate {
                    target,
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
    use super::*;
    use crate::four_state::FourStateBit::{
        HighImpedance as Z, One as ONE, Unknown as X, Zero as ZERO,
    };

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
}
