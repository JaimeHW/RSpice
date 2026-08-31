//! Digital processes as canonical-IR functions.
//!
//! A module lowers to a set of functions. The analog block is one, as it has
//! always been. Each `always` or `initial` process is another, and this module
//! defines what one of those is: a [`CfgFunction`] plus the metadata a
//! scheduler needs to decide when to run it.
//!
//! # What the process function does *not* desugar
//!
//! The suspension points stay where the author wrote them. `always @(posedge
//! clk) q <= d;` lowers to a function whose entry block ends in
//! [`CfgTerminator::Wait`](super::cfg::CfgTerminator::Wait), not to a function
//! that is somehow called when `clk` rises. The difference matters as soon as
//! a process has two suspension points, because then there is no single
//! "sensitivity list" to hoist and no single place to call it from — the
//! process is a coroutine, and the graph has to say so.
//!
//! # Where the two process kinds differ, structurally
//!
//! IEEE 1364-2005 section 9.9.2 says an `always` process restarts as soon as
//! it finishes; section 9.9.1 says an `initial` process runs once. That is a
//! difference in the graph, not a flag a runtime consults:
//!
//! - an `initial` process ends in `Return`;
//! - an `always` process ends in a back edge to its own entry block, so
//!   `Return` never appears in one.
//!
//! Making the restart an edge means the loop is visible to every pass that
//! walks the CFG, and means a process that can complete without suspending —
//! which section 9.9.2 makes a hang — is a cycle with no `Wait` on it, a
//! property that can be checked on the graph.
//!
//! # Execution contract for the process interpreter
//!
//! This wave produces the IR and nothing executes it. What an interpreter must
//! honour, stated here because it is a property of the representation rather
//! than of any one backend:
//!
//! 1. **A `Wait` suspends the process** and names the block to resume into.
//!    Resuming means entering `resume` with `resume_args` bound to its block
//!    parameters, exactly as a `Jump` would. A process's live state across a
//!    suspension is therefore carried in block parameters, not in a side
//!    table.
//! 2. **Signals are not SSA values.** A [`CfgValueKind::DigitalSignalRead`]
//!    reads the signal's current value at the moment the node runs; a write
//!    node updates it. Two reads of one signal on either side of a `Wait` are
//!    two nodes and may differ, which is the whole point.
//! 3. **A nonblocking write does not take effect where it appears.** It
//!    evaluates its right-hand side in the region it is written in and defers
//!    the update to [`DigitalSchedulingRegion::NonBlockingAssign`]. Every
//!    deferred update in a time slot is applied after every process in the
//!    slot has run, which is what makes `a <= b; b <= a;` a swap.
//! 4. **A blocking write takes effect immediately**, before the next
//!    instruction in the same block.
//!
//! # The plan is flat, and hierarchy is gone from it
//!
//! A Verilog design is a tree of module instances; this is not. A
//! [`CanonicalDigitalPlan`] is one signal table, one process list, and one
//! driver list for the whole compiled design, with every instantiated module
//! already elaborated into it by
//! [`digital_elaborate`](crate::semantic::digital_elaborate). Nothing here or
//! downstream — not the interpreter, not the event kernel that follows it —
//! ever asks which instance something came from.
//!
//! What survives the flattening is identity, in three places:
//!
//! * a [`DigitalSignal`] of an instance is named by its instance path
//!   (`u1.g2.n1`, the IEEE 1364-2005 section 12.4 hierarchical name minus the
//!   top module), which no author-written identifier can collide with because
//!   an identifier cannot contain a `.`;
//! * each instance's processes get their own [`DigitalProcessId`]s, so two
//!   instances of one module are two things a scheduler resumes rather than
//!   one resumed twice;
//! * each instance's drivers get their own [`DigitalDriverId`]s on whatever
//!   net they end up driving.
//!
//! ## What a port connection became
//!
//! IEEE 1364-2005 section 12.3.9 reads a port connection two ways, and the
//! elaboration chooses between them **by the port's declared class**:
//!
//! * **A port declared as a net collapsed.** The port and the net it was
//!   connected to are *one* entry in [`CanonicalDigitalPlan::signals`].
//!   Section 12.3.9.3 makes an inout connection precisely this join, and
//!   section 12.3.10 — which asks what net type results from connecting
//!   dissimilar nets — is a question that only exists because the two nets
//!   become one. For an input or an output the join and the assignment
//!   readings agree whenever both sides are plain nets with no delay, and the
//!   cases where they would not agree are refused by the elaboration rather
//!   than decided here.
//! * **A variable output port did not.** Section 12.3.9.2 permits `output q;
//!   reg q;`, and a variable cannot be joined with a net: it keeps its own
//!   signal and the connection appears in [`CanonicalDigitalPlan::drivers`] as
//!   an ordinary continuous driver of the connected net.
//!
//! Collapsing does not merge drivers. Two instances driving one net through
//! collapsed output ports are two entries in
//! [`CanonicalDigitalPlan::drivers`] with indices 0 and 1 on that net, which
//! is what [`DigitalDriverId`] exists to keep apart — a resolver sees both
//! contributions, exactly as it would for two `assign` statements written side
//! by side.
//!
//! [`CfgValueKind::DigitalSignalRead`]: super::cfg::CfgValueKind::DigitalSignalRead

use super::cfg::CfgFunction;
use super::diagnostic::SourceSpanRef;
use super::ids::{DigitalProcessId, DigitalSignalId};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Scheduling region of one simulation time slot.
///
/// IEEE 1364-2005 section 11 stratifies a time slot so that a nonblocking
/// assignment reads the values its right-hand sides had before any of them
/// updated.
///
/// This mirrors the event kernel's own region vocabulary rather than importing
/// it. The compiler does not depend on the kernel — a front end that could not
/// be built without the simulator it feeds is a front end that cannot be
/// tested on its own — so the two enums are kept identical by name and order
/// and mapped one-for-one where they meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DigitalSchedulingRegion {
    /// Blocking assignments, evaluation, and process continuation.
    Active,
    /// Explicitly deferred to after the active region drains (`#0`).
    Inactive,
    /// Nonblocking assignment updates. The right-hand side was evaluated in an
    /// earlier region; only the update lands here.
    NonBlockingAssign,
    /// Passive observation of a settled slot.
    Monitor,
}

impl DigitalSchedulingRegion {
    /// Every region, in execution order.
    pub const ORDERED: [Self; 4] = [
        Self::Active,
        Self::Inactive,
        Self::NonBlockingAssign,
        Self::Monitor,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::NonBlockingAssign => "nonblocking-assign",
            Self::Monitor => "monitor",
        }
    }
}

/// Edge qualifier on a sensitivity term.
///
/// Restated here rather than reused from the syntax tree: the canonical IR is
/// serialized and must not inherit a parser type's layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalEdge {
    Posedge,
    Negedge,
}

impl DigitalEdge {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Posedge => "posedge",
            Self::Negedge => "negedge",
        }
    }
}

/// One term of a sensitivity list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DigitalSensitivityTerm {
    pub signal: DigitalSignalId,
    /// `None` is level-sensitive: any value change triggers it.
    pub edge: Option<DigitalEdge>,
}

/// How a sensitivity list came to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalSensitivityOrigin {
    /// The author wrote the terms.
    Explicit,
    /// The author wrote `@*` or `@(*)` and the terms were computed from the
    /// guarded statement's read set, per IEEE 1364-2005 section 9.7.5.
    Implicit,
}

/// A declared discrete-domain net or variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSignal {
    pub id: DigitalSignalId,
    pub name: SmolStr,
    /// Declared width in bits. A scalar is one.
    pub width: u32,
    /// Left and right bounds exactly as written, `None` for a scalar. Retained
    /// because IEEE 1364-2005 section 4.2.1 makes `[7:0]` and `[0:7]` different
    /// declarations, and a bit select has to agree with the one written.
    pub bounds: Option<(i64, i64)>,
    pub signed: bool,
    /// Whether a procedural assignment may drive it (`reg` can, `wire` cannot).
    pub procedurally_assignable: bool,
    pub span: SourceSpanRef,
}

/// Which bits of a signal an assignment drives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalWriteSelect {
    /// The whole signal.
    Whole,
    /// One bit, by declared index.
    Bit(i64),
    /// A constant part select, bounds as written.
    Part { msb: i64, lsb: i64 },
}

/// The destination of one write.
///
/// A concatenation target is not represented here. `{carry, sum} = ...` lowers
/// to one write per element over slices of the right-hand side, because that
/// is what it means; keeping the concatenation would put a second, different
/// notion of "target" into every consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalWriteTarget {
    pub signal: DigitalSignalId,
    pub select: DigitalWriteSelect,
}

/// One `always` or `initial` process, lowered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgDigitalProcess {
    /// Declaration-ordered identity, the same number the front end assigned.
    pub id: DigitalProcessId,
    pub kind: DigitalProcessKind,
    pub function: CfgFunction,
    /// The list the process suspends on every pass, when it has one.
    ///
    /// Present only when the process opens with an event control, which is the
    /// one shape from which a *static* list is meaningful. A process that
    /// suspends somewhere other than its top has no single list and reports
    /// `None` — it is not that its sensitivity is unknown, it is that the
    /// question does not have one answer.
    pub static_sensitivity: Option<DigitalStaticSensitivity>,
    pub span: SourceSpanRef,
}

/// A process's static sensitivity list and where it came from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalStaticSensitivity {
    pub terms: Vec<DigitalSensitivityTerm>,
    pub origin: DigitalSensitivityOrigin,
}

/// Which process this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalProcessKind {
    /// Restarts as soon as it finishes (IEEE 1364-2005 section 9.9.2).
    Always,
    /// Runs once (IEEE 1364-2005 section 9.9.1).
    Initial,
    /// A continuous assignment: a permanent driver on a net (section 6.1).
    ///
    /// A process kind rather than a fourth kind of thing, because what a
    /// continuous assignment *does* is what an `always @(operands)` does —
    /// evaluate an expression whenever an operand changes. What it does not do
    /// is write a variable, and that is the whole difference: its write is a
    /// [`CfgValueKind::DigitalDriverWrite`] carrying a driver identity, so the
    /// contribution of this driver stays separable from the contributions of
    /// every other driver on the same net.
    ///
    /// The evaluation is in the *entry* block and the suspension follows it,
    /// which is the other structural difference from an `always @*`. A driver
    /// is active from the start of the simulation rather than from the first
    /// change of an operand, so it evaluates once before it ever waits.
    ///
    /// [`CfgValueKind::DigitalDriverWrite`]: super::cfg::CfgValueKind::DigitalDriverWrite
    ContinuousAssign,
}

impl DigitalProcessKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Initial => "initial",
            Self::ContinuousAssign => "assign",
        }
    }

    /// Whether the lowered function ends by looping back to its entry.
    pub const fn restarts(self) -> bool {
        matches!(self, Self::Always | Self::ContinuousAssign)
    }
}

/// Stable identity of one driver of one net.
///
/// The compiler's half of the identity the event kernel completes. A kernel
/// names a driven point by instance, node, port, and driver index; a plan names
/// a *device*'s driven point, so it fixes the part that is the device's — which
/// net, and which of that net's drivers this is — and the kernel supplies the
/// instance the device itself was instantiated as. Nothing here depends on the
/// kernel's own type: the two are kept parallel by shape and mapped where they
/// meet, for the same reason [`DigitalSchedulingRegion`] is.
///
/// The index is declaration order among the drivers of that one net, and says
/// nothing about any other net's drivers. Elaborating a hierarchy does not
/// weaken that: the order is the elaboration order — the compiled module's own
/// assignments, then each instance in depth-first declaration order with its
/// assignments and then its implicit port drivers — which is a function of the
/// source, so a driver keeps its index across recompilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DigitalDriverId {
    pub signal: DigitalSignalId,
    pub index: u32,
}

/// One continuous driver of a net, as the plan declares it.
///
/// Listed on the plan rather than discovered by walking every process, because
/// a kernel has to know how many drivers a net has *before* it runs any of
/// them: resolving a multi-driver net means combining one contribution per
/// driver, and a driver that has not run yet still contributes `z`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalDriver {
    pub id: DigitalDriverId,
    /// Which bits of the net this driver drives.
    ///
    /// `assign bus[3:0] = ...` drives four of them and leaves the rest to
    /// whatever else drives the net, so resolution is per bit and not per net.
    pub target: DigitalWriteTarget,
    /// The process whose function computes it.
    pub process: DigitalProcessId,
    pub span: SourceSpanRef,
}

/// The discrete-domain half of a module, lowered.
///
/// Lifted out beside the analog body rather than folded into it, for the same
/// reason [`CanonicalNoiseSourcePlan`] is: the two are evaluated by different
/// machinery at different times, and a consumer of one should not have to walk
/// past the other. A module with no digital content carries an empty plan and
/// serializes to nothing.
///
/// [`CanonicalNoiseSourcePlan`]: super::noise::CanonicalNoiseSourcePlan
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanonicalDigitalPlan {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<DigitalSignal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<CfgDigitalProcess>,
    /// Every continuous driver in the module, in declaration order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drivers: Vec<DigitalDriver>,
}

impl CanonicalDigitalPlan {
    pub fn is_empty(&self) -> bool {
        self.signals.is_empty() && self.processes.is_empty() && self.drivers.is_empty()
    }

    /// Every driver of one net, in declaration order.
    ///
    /// What a resolver iterates: a net with two of these needs both
    /// contributions combined, and a net with one needs no resolution at all.
    pub fn drivers_of(&self, signal: DigitalSignalId) -> impl Iterator<Item = &DigitalDriver> {
        self.drivers
            .iter()
            .filter(move |driver| driver.id.signal == signal)
    }

    pub fn signal(&self, id: DigitalSignalId) -> Option<&DigitalSignal> {
        self.signals.get(usize::from(id))
    }

    pub fn process(&self, id: DigitalProcessId) -> Option<&CfgDigitalProcess> {
        self.processes.iter().find(|process| process.id == id)
    }
}
