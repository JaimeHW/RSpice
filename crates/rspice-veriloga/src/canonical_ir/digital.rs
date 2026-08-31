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

/// Which procedural process this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalProcessKind {
    /// Restarts as soon as it finishes (IEEE 1364-2005 section 9.9.2).
    Always,
    /// Runs once (IEEE 1364-2005 section 9.9.1).
    Initial,
}

impl DigitalProcessKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Initial => "initial",
        }
    }

    /// Whether the lowered function ends by looping back to its entry.
    pub const fn restarts(self) -> bool {
        matches!(self, Self::Always)
    }
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
}

impl CanonicalDigitalPlan {
    pub fn is_empty(&self) -> bool {
        self.signals.is_empty() && self.processes.is_empty()
    }

    pub fn signal(&self, id: DigitalSignalId) -> Option<&DigitalSignal> {
        self.signals.get(usize::from(id))
    }

    pub fn process(&self, id: DigitalProcessId) -> Option<&CfgDigitalProcess> {
        self.processes
            .iter()
            .find(|process| process.id == id)
    }
}
