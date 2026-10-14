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
//! # Where a declared index is stored
//!
//! **A [`FourStateValue`](super::digital_value::FourStateValue) holds a
//! signal's bits at positions counting up from the least significant end, and
//! the bit a declaration names `index` sits at
//! [`VectorBounds::position_of(index)`](VectorBounds::position_of) — its
//! distance from the declaration's *right* bound.** That is the whole storage
//! rule, and [`VectorBounds::position_of`] is the only place it is written
//! down: the lowering calls it to turn a source-level select into a positional
//! [`CfgValueKind::DigitalPartSelect`](super::cfg::CfgValueKind::DigitalPartSelect),
//! [`apply_write`](super::digital_eval::apply_write) calls it to place a
//! partial write, and the host's driver resolver calls it to place a partial
//! contribution.
//!
//! The distinction only shows itself on a range that is not anchored at zero.
//! IEEE 1364-2005 section 4.3.1 lets a vector be declared over any two bounds,
//! and makes the left one the most significant bit whatever its value, so
//! `reg [7:4] x` is a *four-bit* value whose bits are named 7, 6, 5 and 4 —
//! `x[4]` is its least significant bit and `x[7]` its most, and `x[3]` is not
//! one of its bits at all. A [`DigitalWriteSelect`] and the `msb`/`lsb` of a
//! source-level part select are therefore names, and only a
//! [`CfgValueKind::DigitalPartSelect`](super::cfg::CfgValueKind::DigitalPartSelect)
//! — which slices values that were never declared, such as a concatenation's
//! right-hand side — is positional.
//!
//! [`CfgValueKind::DigitalSignalRead`]: super::cfg::CfgValueKind::DigitalSignalRead

use super::ValueId;
use super::cfg::CfgFunction;
use super::diagnostic::SourceSpanRef;
use super::ids::{DigitalAnalogProbeId, DigitalProcessId, DigitalSignalId};
use crate::semantic::VectorBounds;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Simulator time functions evaluated at a process's current activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalTimeQuery {
    Time,
    ShortTime,
    RealTime,
    AbsoluteTime,
}

impl DigitalTimeQuery {
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        match name {
            "$time" => Some(Self::Time),
            "$stime" => Some(Self::ShortTime),
            "$realtime" => Some(Self::RealTime),
            "$abstime" => Some(Self::AbsoluteTime),
            _ => None,
        }
    }

    pub(crate) fn bit_width(self) -> Option<u32> {
        match self {
            Self::Time => Some(64),
            Self::ShortTime => Some(32),
            Self::RealTime | Self::AbsoluteTime => None,
        }
    }

    pub(crate) fn value_type(self) -> super::cfg::CfgValueType {
        self.bit_width()
            .map_or(super::cfg::CfgValueType::Real, |width| {
                super::cfg::CfgValueType::FourState { width }
            })
    }
}
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

/// One computed event expression, evaluated at registration and input changes.
/// Its root uses the ordinary typed digital expression graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DigitalEventExpression {
    pub value: super::ids::ValueId,
    pub edge: Option<DigitalEdge>,
    /// For an analog array assignment event, value computes the selected index.
    /// Index changes alone are not assignments and cannot trigger this term.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment: Option<DigitalAssignmentEventSelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DigitalAssignmentEventSelection {
    pub array: DigitalArrayRef,
    pub signed: bool,
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

/// How a real net combines the contributions of its drivers.
///
/// Restated here rather than reused from the syntax tree, for the reason
/// [`DigitalEdge`] is: the canonical IR is serialized and must not inherit a
/// parser type's layout.
///
/// [`Self::Single`] is Verilog-AMS LRM 2.4 section 6.5.3 — "there can be a
/// maximum of one driver of a real-valued net" — and is what a plain `wreal`
/// selects. The other four are the real-number-modelling extension the LRM does
/// not define, opted into by writing `wrealsum`, `wrealavg`, `wrealmin` or
/// `wrealmax` on the declaration.
///
/// The fold itself is the *kernel's*, exactly as IEEE 1364-2005 table 4-1's is:
/// which value a net takes from several drivers is a simulation rule over the
/// whole net, and the compiler's job is to say which rule, not to apply it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum DigitalRealResolution {
    /// One driver, and a second is refused.
    #[default]
    Single,
    Sum,
    Average,
    Minimum,
    Maximum,
}

impl DigitalRealResolution {
    /// The net-type keyword that selects this resolution.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Single => "wreal",
            Self::Sum => "wrealsum",
            Self::Average => "wrealavg",
            Self::Minimum => "wrealmin",
            Self::Maximum => "wrealmax",
        }
    }
}

/// What kind of value a signal carries.
///
/// A separate axis from `width` and from
/// [`DigitalSignal::procedurally_assignable`], because it is a separate
/// question: `reg`/`wire` decides *who may write it*, the width decides *how
/// many bits*, and this decides *what a bit even is*. A real net has no bits at
/// all (Verilog-AMS LRM 2.4 section 3.7), which is why the two cannot be folded
/// into one enum without one of them lying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum DigitalSignalKind {
    /// IEEE 1364-2005's four-state value, at the declared width.
    #[default]
    FourState,
    /// Verilog-AMS LRM 2.4 section 3.7's real-valued net, with the resolution
    /// its net-type keyword named.
    Real(DigitalRealResolution),
}

impl DigitalSignalKind {
    pub const fn is_real(self) -> bool {
        matches!(self, Self::Real(_))
    }

    pub const fn resolution(self) -> Option<DigitalRealResolution> {
        match self {
            Self::FourState => None,
            Self::Real(resolution) => Some(resolution),
        }
    }
}

/// Typed, constant initial storage for a numeric variable declaration.
/// This is installed before processes run, not replayed as a procedural driver.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DigitalInitialValue {
    FourState(super::digital_value::FourStateValue),
    Real(f64),
}

/// A checked, contiguous group of scalar signal cells for an unpacked array.
/// Storage follows increasing logical index, independently of declaration direction.
/// Keeping ordinary signal IDs preserves element-level events and checkpoint state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DigitalArrayRef {
    pub base: DigitalSignalId,
    pub lower: i64,
    pub len: u32,
}

impl DigitalArrayRef {
    /// The bounded storage range; malformed artifacts must not create huge walks.
    pub fn cell_range(self) -> Option<std::ops::Range<u32>> {
        if self.len == 0 || u64::from(self.len) > crate::semantic::MAX_PARAMETER_ARRAY_ELEMENTS {
            return None;
        }
        self.lower.checked_add(i64::from(self.len) - 1)?;
        Some(self.base.index()..self.base.index().checked_add(self.len)?)
    }

    /// Resolve in the integer domain, including indices beyond f64 precision.
    pub fn element(self, index: i64) -> Option<DigitalSignalId> {
        self.cell_range()?;
        let offset = rspice_veriloga_runtime::array_index::checked_integer_array_slot(
            index,
            0,
            self.len as usize,
            self.lower,
        )
        .ok()?;
        Some(DigitalSignalId::new(self.base.index() + offset as u32))
    }
}

/// An unpacked variable declaration over scalar element storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalArray {
    pub name: SmolStr,
    /// Authored bounds for a one-dimensional array; zero-based flat bounds for
    /// a multidimensional array, whose declaration order is in `dimensions`.
    pub bounds: (i64, i64),
    /// Authored multidimensional bounds. Empty retains the one-dimensional
    /// representation above; multiple dimensions use zero-based flat storage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<(i64, i64)>,
    pub storage: DigitalArrayRef,
}

impl DigitalArray {
    pub fn layout(
        &self,
    ) -> Result<crate::array_index::UnpackedArrayLayout, crate::array_index::ArrayShapeError> {
        let bounds = if self.dimensions.is_empty() {
            std::slice::from_ref(&self.bounds)
        } else {
            &self.dimensions
        };
        crate::array_index::UnpackedArrayLayout::new(
            bounds,
            crate::semantic::MAX_PARAMETER_ARRAY_ELEMENTS as usize,
        )
    }

    pub fn element_name(&self, offset: usize) -> Option<SmolStr> {
        self.element_name_with_layout(&self.layout().ok()?, offset)
    }

    pub(crate) fn element_name_with_layout(
        &self,
        layout: &crate::array_index::UnpackedArrayLayout,
        offset: usize,
    ) -> Option<SmolStr> {
        let mut name = self.name.to_string();
        for index in layout.indices(offset)? {
            use std::fmt::Write;
            write!(name, "[{index}]").expect("writing to a string");
        }
        Some(name.into())
    }
}

/// A static lexical declaration whose value needs scheduler-visible storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalLocalStorage {
    /// Logical unpacked index; None identifies a scalar declaration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<i64>,
    pub process: DigitalProcessId,
    pub declaration: super::ids::DigitalLocalId,
    pub name: SmolStr,
}

/// A declared discrete-domain net or variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSignal {
    /// Lexical identity for addressable process-local storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<DigitalLocalStorage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<DigitalInitialValue>,
    pub id: DigitalSignalId,
    pub name: SmolStr,
    /// What the signal carries.
    ///
    /// Defaulted on the wire format so that a plan serialized before real nets
    /// existed still decodes as the four-state design it was.
    #[serde(default, skip_serializing_if = "is_four_state")]
    pub kind: DigitalSignalKind,
    /// Declared width in bits. A scalar is one, and a real net is zero — it has
    /// no bits, and says so the same way a process-local `real` does.
    pub width: u32,
    /// Left and right bounds exactly as written, `None` for a scalar. Retained
    /// because IEEE 1364-2005 section 5.2.1 makes `[7:0]` and `[0:7]` different
    /// declarations, and a bit select has to agree with the one written.
    pub bounds: Option<(i64, i64)>,
    pub signed: bool,
    /// Authored integer variable, distinct from a signed packed reg. Cross-domain
    /// access preserves its signed 32-bit value; packed bits are zero-extended.
    #[serde(default)]
    pub integer: bool,
    /// Whether a procedural assignment may drive it (`reg` can, `wire` cannot).
    pub procedurally_assignable: bool,
    pub span: SourceSpanRef,
}

impl DigitalSignal {
    /// The range this signal's bits are named over.
    ///
    /// [`VectorBounds::SCALAR`] for a signal declared without one, which has
    /// the one bit every scalar has and numbers it zero. The bounds field
    /// stays a bare pair on the wire; this is the view of it every consumer of
    /// the storage rule asks for, so that "what does index `i` mean here" has
    /// one answer for a vector and a scalar alike.
    pub fn declared_range(&self) -> VectorBounds {
        match self.bounds {
            Some((msb, lsb)) => VectorBounds { msb, lsb },
            None => VectorBounds::SCALAR,
        }
    }
}

/// Whether a signal kind is the default, for the wire format.
fn is_four_state(kind: &DigitalSignalKind) -> bool {
    matches!(kind, DigitalSignalKind::FourState)
}

/// The authored net pair or named branch of a continuous-domain read.
///
/// Verilog-AMS LRM 2.4 section 7.3.3: "All continuous nets can be probed from
/// a discrete context using access functions. All probes which are legal in a
/// continuous context of a module are also legal in the discrete context of a
/// module." Section 7.3's opening paragraph is what makes that a *read* and
/// only a read — "Read operations of nets and variables in both domains are
/// allowed from both contexts. Write operations of nets and variables are only
/// allowed from the context of their domain" — so nothing here can be
/// contributed to.
///
/// # Why the nets are named rather than numbered
///
/// The analog levels are lowered from the analyzed module by `hir`/`mir`,
/// which allocate a [`NodeId`](super::ids::NodeId) per continuous net, and the
/// discrete plan is lowered from the same module by `digital_lower`, which
/// never sees that allocation. A probe carrying a `NodeId` would make one pass
/// depend on the other's numbering; a probe carrying the author's own net name
/// is resolvable by whichever side has the mapping — which is the host, the
/// same way it already resolves a bridge's signal by name.
///
/// # What the reader owes
///
/// A value, at the time section 7.3.6.3 fixes: "the analog value calculated
/// for the time corresponding to a real promotion of the digital time at which
/// the expression is evaluated". The plan cannot enforce that; it can only
/// make the probe a *leaf*, so that two reads on either side of a suspension
/// are two calls and may differ, exactly as two reads of a signal are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalAnalogProbeTarget {
    Nodes {
        positive: SmolStr,
        negative: Option<SmolStr>,
    },
    Branch {
        name: SmolStr,
    },
    Variable {
        name: SmolStr,
    },
}

/// The value domain of a continuous-domain read. Integer variables retain
/// their declared 32-bit signed domain rather than becoming real operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalAnalogQuantity {
    Potential,
    Flow,
    RealVariable,
    IntegerVariable,
}

impl From<crate::ast::AccessKind> for DigitalAnalogQuantity {
    fn from(kind: crate::ast::AccessKind) -> Self {
        match kind {
            crate::ast::AccessKind::Potential => Self::Potential,
            crate::ast::AccessKind::Flow => Self::Flow,
        }
    }
}

/// A typed solver sample requested by a discrete-domain expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalAnalogProbe {
    /// Private occurrence counter signal for an analog event subscription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_signal: Option<DigitalSignalId>,
    /// Read a constant-after-startup or event-assigned value from its last
    /// assignment, without a new numerical evaluation (VAMS 7.3.6.3).
    #[serde(default)]
    pub retained: bool,
    pub id: DigitalAnalogProbeId,
    /// Authored nature access function, resolved to its physical role.
    pub access: SmolStr,
    pub quantity: DigitalAnalogQuantity,
    pub target: DigitalAnalogProbeTarget,
    pub span: SourceSpanRef,
}

impl DigitalAnalogProbe {
    pub fn spelling(&self) -> String {
        match &self.target {
            DigitalAnalogProbeTarget::Nodes {
                positive,
                negative: Some(negative),
            } => format!("{}({}, {})", self.access, positive, negative),
            DigitalAnalogProbeTarget::Nodes {
                positive,
                negative: None,
            } => format!("{}({})", self.access, positive),
            DigitalAnalogProbeTarget::Branch { name } => format!("{}(<{}>)", self.access, name),
            DigitalAnalogProbeTarget::Variable { name } => name.to_string(),
        }
    }
}

/// Which bits of a signal an assignment drives.
///
/// The indices are the signal's own **names** for its bits, as written; where
/// those bits are stored is [`VectorBounds::position_of`]'s answer and
/// [`Self::low_position`] is how every consumer asks it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalWriteSelect {
    /// The whole signal.
    Whole,
    /// One bit, by declared index.
    Bit(i64),
    /// A constant part select, bounds as written.
    Part { msb: i64, lsb: i64 },
}

impl DigitalWriteSelect {
    /// Selected width and direction, checked before sizing or applying a write.
    /// Out-of-range endpoints remain valid; the runtime clips their bits.
    pub(crate) fn checked_width(&self, range: VectorBounds) -> Option<u32> {
        let width = match self {
            Self::Whole => u64::from(range.width()),
            Self::Bit(_) => 1,
            Self::Part { msb, lsb } => {
                if msb != lsb && (msb > lsb) != (range.msb >= range.lsb) {
                    return None;
                }
                msb.abs_diff(*lsb).checked_add(1)?
            }
        };
        (width <= u64::from(crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)).then_some(width as u32)
    }

    /// Where this select's least significant bit is stored on a signal
    /// declared over `range`, counting from the least significant end.
    ///
    /// The one thing a partial write needs and the one thing a partial
    /// contribution needs, so it is answered once: a value of the select's
    /// width is laid down from here upwards, and whatever falls outside the
    /// signal is dropped, which is IEEE 1364-2005 section 5.2.1's rule for an
    /// out-of-range left-hand side.
    ///
    /// Signed, because a select the range does not wholly name starts below
    /// the value — `x[5:2]` of a `reg [7:4]` starts at −2, and only its top
    /// two bits land.
    pub fn low_position(&self, range: VectorBounds) -> i64 {
        match self {
            Self::Whole => 0,
            Self::Bit(index) => range.position_of(*index),
            Self::Part { msb, lsb } => range.position_of(*msb).min(range.position_of(*lsb)),
        }
    }
}

/// Packed selection for an array write or a pure four-state value update.
/// Unlike a captured `DigitalWriteSelect`, a bit index here is an SSA value:
/// it is evaluated at the update and resolved before a store write is deferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalArrayWriteSelect {
    Whole,
    Bit {
        index: ValueId,
        signed: bool,
    },
    Part {
        msb: i64,
        lsb: i64,
    },
    /// `[base +: width]` (`ascending`) or `[base -: width]`, IEEE 1364-2005
    /// section 5.2.1: a constant width at a run-time base, resolved through
    /// [`indexed_part_bounds`] to a [`Self::Part`] when written.
    Indexed {
        base: ValueId,
        signed: bool,
        width: u32,
        ascending: bool,
    },
}

impl DigitalArrayWriteSelect {
    pub fn operand(self) -> Option<ValueId> {
        match self {
            Self::Bit { index, .. } => Some(index),
            Self::Indexed { base, .. } => Some(base),
            _ => None,
        }
    }

    pub fn map_operands(&mut self, map: &mut impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Bit { index, .. } => *index = map(*index),
            Self::Indexed { base, .. } => *base = map(*base),
            Self::Whole | Self::Part { .. } => {}
        }
    }
}

/// The part an indexed part-select names once its base is known, as `(msb,
/// lsb)` indices written in the direction `bounds` was declared in.
///
/// IEEE 1364-2005 section 5.2.1: `+:` names `width` indices counting up from
/// `base`, `-:` counting down, and the declared direction decides which end
/// is most significant (`v[0 +: 8]` is `v[7:0]` of a `[31:0]` vector and
/// `v[0:7]` of a `[0:31]` one). `None` when the indices are not representable,
/// which places every bit outside any vector.
pub(crate) fn indexed_part_bounds(
    bounds: VectorBounds,
    base: i64,
    width: u32,
    ascending: bool,
) -> Option<(i64, i64)> {
    let span = i64::from(width.checked_sub(1)?);
    let low = if ascending {
        base
    } else {
        base.checked_sub(span)?
    };
    let high = low.checked_add(span)?;
    Some(if bounds.msb >= bounds.lsb {
        (high, low)
    } else {
        (low, high)
    })
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
    /// Owning module's units and precision, retained after hierarchy flattening.
    pub time_scale: crate::time_scale::ModuleTimeScale,
    /// Dense declaration-ordered identity allocated after elaboration.
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

/// Exact elaboration value for a scalar parameter outside the analog numeric ABI.
/// Changing it requires source specialization, including all shapes and processes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalElaborationParameter {
    /// Explicit source assignment, including an assignment equal to the default.
    pub is_given: bool,
    pub name: SmolStr,
    pub aliases: Vec<SmolStr>,
    pub is_public: bool,
    pub scope: crate::semantic::ParameterScope,
    pub also_model: bool,
    pub value: super::digital_value::FourStateValue,
    pub signed: bool,
    #[serde(default)]
    pub bounds: Option<crate::semantic::VectorBounds>,
}

/// One bit in normalized storage order (zero is the least significant bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DigitalNetBit {
    pub signal: DigitalSignalId,
    pub bit: u32,
}

/// An electrical identity between wire bits, not a value-copying driver.
/// Transitive aliases retain every original driver's independent contribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalBitAlias {
    pub left: DigitalNetBit,
    pub right: DigitalNetBit,
    pub span: SourceSpanRef,
}

/// Two views of one real-valued net. Original drivers resolve together;
/// the connection does not add a continuous-assignment driver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalRealAlias {
    pub left: DigitalSignalId,
    pub right: DigitalSignalId,
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
    /// Delays in this plan are already rounded and expressed in these ticks.
    pub timing: crate::time_scale::DigitalTiming,
    /// BLAKE3 identity of the complete lowered plan. Validate after decoding or
    /// editing an artifact, then keep it immutable throughout execution.
    #[serde(default)]
    pub content_identity: [u8; 32],
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elaboration_parameters: Vec<DigitalElaborationParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<DigitalSignal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arrays: Vec<DigitalArray>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bit_aliases: Vec<DigitalBitAlias>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub real_aliases: Vec<DigitalRealAlias>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<CfgDigitalProcess>,
    /// Every continuous driver in the module, in declaration order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drivers: Vec<DigitalDriver>,
    /// Every continuous-domain potential or flow a process reads, in first-appearance
    /// order (Verilog-AMS LRM 2.4 section 7.3.3).
    ///
    /// Empty for a design with no cross-domain read, which is every design
    /// that existed before this table did, so an artifact serialized without
    /// it decodes unchanged.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub analog_probes: Vec<DigitalAnalogProbe>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub absdelta: Vec<DigitalAbsDeltaObserver>,
    /// Hierarchical scopes a system call can name (`%m`), root first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<DigitalScope>,
    /// One entry per system task or function call site (IEEE 1364-2005
    /// section 17), in lowering order. A call node names its entry here.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub system_calls: Vec<DigitalSystemCall>,
    /// The argument functions of `$strobe` and `$monitor` sites, which the
    /// host evaluates in the monitor region rather than at the call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observers: Vec<DigitalObserver>,
    /// Every task and function of the design, per instance, in elaboration
    /// order. A call names its entry here.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routines: Vec<DigitalRoutine>,
}

/// Dense identity of one system task or function call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DigitalSystemCallId(pub u32);

impl DigitalSystemCallId {
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for DigitalSystemCallId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site#{}", self.0)
    }
}

/// Dense identity of one hierarchical scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DigitalScopeId(pub u32);

impl DigitalScopeId {
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// What kind of declarative region a scope is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalScopeKind {
    /// A module instance (or the root module).
    Module,
    /// A named `begin : name` block (IEEE 1364-2005 section 9.8.3).
    Block,
    /// A task's body (section 10.2).
    Task,
    /// A function's body (section 10.4).
    Function,
}

/// One hierarchical scope, named the way `%m` prints it (IEEE 1364-2005
/// section 17.1.1.4): the full path including the root module name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalScope {
    pub id: DigitalScopeId,
    pub parent: Option<DigitalScopeId>,
    pub path: SmolStr,
    /// The definition name of the module instance this scope belongs to.
    pub module: SmolStr,
    pub kind: DigitalScopeKind,
    /// That module's `timescale` (`$printtimescale`, `%t`).
    pub time_scale: crate::time_scale::ModuleTimeScale,
}

/// The radix a display-family task formats an unformatted argument in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalRadix {
    Decimal,
    Binary,
    Octal,
    Hex,
}

impl DigitalRadix {
    const fn suffix(self) -> &'static str {
        match self {
            Self::Decimal => "",
            Self::Binary => "b",
            Self::Octal => "o",
            Self::Hex => "h",
        }
    }

    fn from_suffix(suffix: &str) -> Option<Self> {
        Some(match suffix {
            "" => Self::Decimal,
            "b" => Self::Binary,
            "o" => Self::Octal,
            "h" => Self::Hex,
            _ => return None,
        })
    }
}

/// The P2 lane that implements a system call kind's host behaviour.
///
/// Every kind of design P2 section 1.3 is registered and lowered now, so a
/// later lane changes only the host. A kind whose lane has not landed is
/// refused when it runs, naming the lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalSystemLane {
    /// Display, control, severity, random and plusargs (P2 lane L1).
    Effects,
    /// File I/O, `$sformat`, `$readmem*` and VCD tasks (P2 lane L5).
    Files,
}

/// How a system call uses one argument position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalArgumentUse {
    /// Read (a value, a string literal, or an empty argument).
    Input,
    /// Written: the position must be a variable target.
    Output,
    /// Read and written back (`$random(seed)`).
    InOut,
    /// A whole unpacked array the host writes (`$readmemh`, `$fread`).
    ArrayOutput,
    /// A scope or signal named rather than read (`$dumpvars`, `$printtimescale`).
    Reference,
}

/// Every system task and function the digital subset registers (design P2
/// section 1.3). Pure functions (`$signed`, `$clog2`, math, conversions) are
/// not here: they are ordinary operators of the expression graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalSystemCallKind {
    Display(DigitalRadix),
    Write(DigitalRadix),
    Strobe(DigitalRadix),
    Monitor(DigitalRadix),
    MonitorOn,
    MonitorOff,
    Finish,
    Stop,
    TimeFormat,
    PrintTimeScale,
    Info,
    Warning,
    Error,
    Fatal,
    Random,
    DistUniform,
    DistNormal,
    DistExponential,
    DistPoisson,
    DistChiSquare,
    DistT,
    DistErlang,
    Urandom,
    UrandomRange,
    TestPlusargs,
    ValuePlusargs,
    Fopen,
    Fclose,
    Fdisplay(DigitalRadix),
    Fwrite(DigitalRadix),
    Fstrobe(DigitalRadix),
    Fmonitor(DigitalRadix),
    Fflush,
    Fgetc,
    Ungetc,
    Fgets,
    Fscanf,
    Sscanf,
    Fread,
    Ftell,
    Fseek,
    Rewind,
    Feof,
    Ferror,
    Sformat,
    Swrite(DigitalRadix),
    Readmemb,
    Readmemh,
    Dumpfile,
    Dumpvars,
    Dumpon,
    Dumpoff,
    Dumpall,
    Dumplimit,
    Dumpflush,
}

impl DigitalSystemCallKind {
    /// The kind a `$name` spells, or `None` for a name outside the subset.
    pub fn from_name(name: &str) -> Option<Self> {
        let radix = |prefix: &str| {
            name.strip_prefix(prefix)
                .and_then(DigitalRadix::from_suffix)
        };
        if let Some(radix) = radix("$display") {
            return Some(Self::Display(radix));
        }
        if let Some(radix) = radix("$write") {
            return Some(Self::Write(radix));
        }
        if let Some(radix) = radix("$strobe") {
            return Some(Self::Strobe(radix));
        }
        if let Some(radix) = radix("$monitor") {
            return Some(Self::Monitor(radix));
        }
        if let Some(radix) = radix("$fdisplay") {
            return Some(Self::Fdisplay(radix));
        }
        if let Some(radix) = radix("$fwrite") {
            return Some(Self::Fwrite(radix));
        }
        if let Some(radix) = radix("$fstrobe") {
            return Some(Self::Fstrobe(radix));
        }
        if let Some(radix) = radix("$fmonitor") {
            return Some(Self::Fmonitor(radix));
        }
        if let Some(radix) = radix("$swrite") {
            return Some(Self::Swrite(radix));
        }
        Some(match name {
            "$monitoron" => Self::MonitorOn,
            "$monitoroff" => Self::MonitorOff,
            "$finish" => Self::Finish,
            "$stop" => Self::Stop,
            "$timeformat" => Self::TimeFormat,
            "$printtimescale" => Self::PrintTimeScale,
            "$info" => Self::Info,
            "$warning" => Self::Warning,
            "$error" => Self::Error,
            "$fatal" => Self::Fatal,
            "$random" => Self::Random,
            "$dist_uniform" => Self::DistUniform,
            "$dist_normal" => Self::DistNormal,
            "$dist_exponential" => Self::DistExponential,
            "$dist_poisson" => Self::DistPoisson,
            "$dist_chi_square" => Self::DistChiSquare,
            "$dist_t" => Self::DistT,
            "$dist_erlang" => Self::DistErlang,
            "$urandom" => Self::Urandom,
            "$urandom_range" => Self::UrandomRange,
            "$test$plusargs" => Self::TestPlusargs,
            "$value$plusargs" => Self::ValuePlusargs,
            "$fopen" => Self::Fopen,
            "$fclose" => Self::Fclose,
            "$fflush" => Self::Fflush,
            "$fgetc" => Self::Fgetc,
            "$ungetc" => Self::Ungetc,
            "$fgets" => Self::Fgets,
            "$fscanf" => Self::Fscanf,
            "$sscanf" => Self::Sscanf,
            "$fread" => Self::Fread,
            "$ftell" => Self::Ftell,
            "$fseek" => Self::Fseek,
            "$rewind" => Self::Rewind,
            "$feof" => Self::Feof,
            "$ferror" => Self::Ferror,
            "$sformat" => Self::Sformat,
            "$readmemb" => Self::Readmemb,
            "$readmemh" => Self::Readmemh,
            "$dumpfile" => Self::Dumpfile,
            "$dumpvars" => Self::Dumpvars,
            "$dumpon" => Self::Dumpon,
            "$dumpoff" => Self::Dumpoff,
            "$dumpall" => Self::Dumpall,
            "$dumplimit" => Self::Dumplimit,
            "$dumpflush" => Self::Dumpflush,
            _ => return None,
        })
    }

    /// The spelling a diagnostic names.
    pub fn name(self) -> String {
        let radix = |base: &str, radix: DigitalRadix| format!("{base}{}", radix.suffix());
        match self {
            Self::Display(r) => radix("$display", r),
            Self::Write(r) => radix("$write", r),
            Self::Strobe(r) => radix("$strobe", r),
            Self::Monitor(r) => radix("$monitor", r),
            Self::Fdisplay(r) => radix("$fdisplay", r),
            Self::Fwrite(r) => radix("$fwrite", r),
            Self::Fstrobe(r) => radix("$fstrobe", r),
            Self::Fmonitor(r) => radix("$fmonitor", r),
            Self::Swrite(r) => radix("$swrite", r),
            Self::MonitorOn => "$monitoron".into(),
            Self::MonitorOff => "$monitoroff".into(),
            Self::Finish => "$finish".into(),
            Self::Stop => "$stop".into(),
            Self::TimeFormat => "$timeformat".into(),
            Self::PrintTimeScale => "$printtimescale".into(),
            Self::Info => "$info".into(),
            Self::Warning => "$warning".into(),
            Self::Error => "$error".into(),
            Self::Fatal => "$fatal".into(),
            Self::Random => "$random".into(),
            Self::DistUniform => "$dist_uniform".into(),
            Self::DistNormal => "$dist_normal".into(),
            Self::DistExponential => "$dist_exponential".into(),
            Self::DistPoisson => "$dist_poisson".into(),
            Self::DistChiSquare => "$dist_chi_square".into(),
            Self::DistT => "$dist_t".into(),
            Self::DistErlang => "$dist_erlang".into(),
            Self::Urandom => "$urandom".into(),
            Self::UrandomRange => "$urandom_range".into(),
            Self::TestPlusargs => "$test$plusargs".into(),
            Self::ValuePlusargs => "$value$plusargs".into(),
            Self::Fopen => "$fopen".into(),
            Self::Fclose => "$fclose".into(),
            Self::Fflush => "$fflush".into(),
            Self::Fgetc => "$fgetc".into(),
            Self::Ungetc => "$ungetc".into(),
            Self::Fgets => "$fgets".into(),
            Self::Fscanf => "$fscanf".into(),
            Self::Sscanf => "$sscanf".into(),
            Self::Fread => "$fread".into(),
            Self::Ftell => "$ftell".into(),
            Self::Fseek => "$fseek".into(),
            Self::Rewind => "$rewind".into(),
            Self::Feof => "$feof".into(),
            Self::Ferror => "$ferror".into(),
            Self::Sformat => "$sformat".into(),
            Self::Readmemb => "$readmemb".into(),
            Self::Readmemh => "$readmemh".into(),
            Self::Dumpfile => "$dumpfile".into(),
            Self::Dumpvars => "$dumpvars".into(),
            Self::Dumpon => "$dumpon".into(),
            Self::Dumpoff => "$dumpoff".into(),
            Self::Dumpall => "$dumpall".into(),
            Self::Dumplimit => "$dumplimit".into(),
            Self::Dumpflush => "$dumpflush".into(),
        }
    }

    /// The lane whose host implements this kind.
    pub const fn lane(self) -> DigitalSystemLane {
        match self {
            Self::Fopen
            | Self::Fclose
            | Self::Fdisplay(_)
            | Self::Fwrite(_)
            | Self::Fstrobe(_)
            | Self::Fmonitor(_)
            | Self::Fflush
            | Self::Fgetc
            | Self::Ungetc
            | Self::Fgets
            | Self::Fscanf
            | Self::Sscanf
            | Self::Fread
            | Self::Ftell
            | Self::Fseek
            | Self::Rewind
            | Self::Feof
            | Self::Ferror
            | Self::Sformat
            | Self::Swrite(_)
            | Self::Readmemb
            | Self::Readmemh
            | Self::Dumpfile
            | Self::Dumpvars
            | Self::Dumpon
            | Self::Dumpoff
            | Self::Dumpall
            | Self::Dumplimit
            | Self::Dumpflush => DigitalSystemLane::Files,
            _ => DigitalSystemLane::Effects,
        }
    }

    /// Whether a call returns a value (IEEE 1364-2005 section 17's system
    /// *functions*). A function may also be enabled as a task, which discards
    /// its value but still performs its outputs.
    pub const fn is_function(self) -> bool {
        matches!(
            self,
            Self::Random
                | Self::DistUniform
                | Self::DistNormal
                | Self::DistExponential
                | Self::DistPoisson
                | Self::DistChiSquare
                | Self::DistT
                | Self::DistErlang
                | Self::Urandom
                | Self::UrandomRange
                | Self::TestPlusargs
                | Self::ValuePlusargs
                | Self::Fopen
                | Self::Fgetc
                | Self::Ungetc
                | Self::Fgets
                | Self::Fscanf
                | Self::Sscanf
                | Self::Fread
                | Self::Ftell
                | Self::Fseek
                | Self::Rewind
                | Self::Feof
                | Self::Ferror
        )
    }

    /// Whether the value a function returns is signed (IEEE 1364-2005 section
    /// 17.9.1 makes `$random` a signed integer; 1800 section 18.13 makes
    /// `$urandom` unsigned). Every function returns 32 bits.
    pub const fn result_signed(self) -> bool {
        !matches!(self, Self::Urandom | Self::UrandomRange | Self::Fopen)
    }

    /// Whether the call's arguments are evaluated when the monitor region
    /// runs rather than when the call executes (IEEE 1364-2005 sections
    /// 17.1.2 and 17.1.3).
    pub const fn is_observer(self) -> bool {
        matches!(
            self,
            Self::Strobe(_) | Self::Monitor(_) | Self::Fstrobe(_) | Self::Fmonitor(_)
        )
    }

    /// Whether the call formats its arguments the way `$display` does.
    pub const fn formats(self) -> bool {
        matches!(
            self,
            Self::Display(_)
                | Self::Write(_)
                | Self::Strobe(_)
                | Self::Monitor(_)
                | Self::Fdisplay(_)
                | Self::Fwrite(_)
                | Self::Fstrobe(_)
                | Self::Fmonitor(_)
                | Self::Swrite(_)
                | Self::Sformat
                | Self::Info
                | Self::Warning
                | Self::Error
                | Self::Fatal
        )
    }

    /// Ends the calling thread (IEEE 1364-2005 section 17.4; 1800 section
    /// 20.10 for `$fatal`).
    pub const fn halts(self) -> bool {
        matches!(self, Self::Finish | Self::Stop | Self::Fatal)
    }

    /// The fewest and most arguments a call may be written with. `None` is
    /// unbounded.
    pub const fn arity(self) -> (usize, Option<usize>) {
        match self {
            Self::MonitorOn
            | Self::MonitorOff
            | Self::Dumpon
            | Self::Dumpoff
            | Self::Dumpall
            | Self::Dumpflush => (0, Some(0)),
            Self::Finish | Self::Stop => (0, Some(1)),
            Self::TimeFormat => (0, Some(4)),
            Self::PrintTimeScale => (0, Some(1)),
            Self::Random | Self::Urandom => (0, Some(1)),
            Self::DistUniform | Self::DistNormal | Self::DistErlang => (3, Some(3)),
            Self::DistExponential | Self::DistPoisson | Self::DistChiSquare | Self::DistT => {
                (2, Some(2))
            }
            Self::UrandomRange => (1, Some(2)),
            Self::TestPlusargs => (1, Some(1)),
            Self::ValuePlusargs => (2, Some(2)),
            Self::Fopen => (1, Some(2)),
            Self::Fclose | Self::Fgetc | Self::Ftell | Self::Rewind | Self::Feof => (1, Some(1)),
            Self::Fflush => (0, Some(1)),
            Self::Ungetc | Self::Fgets | Self::Ferror => (2, Some(2)),
            Self::Fseek => (3, Some(3)),
            Self::Fscanf | Self::Sscanf | Self::Sformat => (2, None),
            Self::Fread => (2, Some(4)),
            Self::Readmemb | Self::Readmemh => (2, Some(4)),
            Self::Dumpfile | Self::Dumplimit => (1, Some(1)),
            Self::Fdisplay(_)
            | Self::Fwrite(_)
            | Self::Fstrobe(_)
            | Self::Fmonitor(_)
            | Self::Swrite(_) => (1, None),
            Self::Fatal => (0, None),
            Self::Display(_)
            | Self::Write(_)
            | Self::Strobe(_)
            | Self::Monitor(_)
            | Self::Info
            | Self::Warning
            | Self::Error
            | Self::Dumpvars => (0, None),
        }
    }

    /// How argument `index` is used.
    pub const fn argument_use(self, index: usize) -> DigitalArgumentUse {
        use DigitalArgumentUse::*;
        match (self, index) {
            (
                Self::Random
                | Self::DistUniform
                | Self::DistNormal
                | Self::DistExponential
                | Self::DistPoisson
                | Self::DistChiSquare
                | Self::DistT
                | Self::DistErlang,
                0,
            ) => InOut,
            (Self::ValuePlusargs, 1) => Output,
            (Self::Fgets | Self::Sformat | Self::Swrite(_), 0) => Output,
            (Self::Ferror, 1) => Output,
            (Self::Fscanf | Self::Sscanf, index) if index >= 2 => Output,
            (Self::Fread, 0) => ArrayOutput,
            (Self::Readmemb | Self::Readmemh, 1) => ArrayOutput,
            (Self::Dumpvars, index) if index >= 1 => Reference,
            (Self::PrintTimeScale, 0) => Reference,
            _ => Input,
        }
    }
}

/// The type an argument or result value carries across the call boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalValueClass {
    FourState { width: u32, signed: bool },
    Real,
}

/// One argument of a call site, as the host sees it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalSystemArgument {
    /// An empty argument: `$display(a,,b)` prints a space for it.
    Empty,
    /// A string literal, escapes already decoded (a format or text argument).
    String(SmolStr),
    /// An evaluated operand. `operand` indexes the call node's arguments.
    Value {
        operand: u32,
        class: DigitalValueClass,
    },
    /// A variable written back through `DigitalCallOutput { index: output }`.
    /// `operand` is the value read first, for an in-out argument.
    Output {
        output: u32,
        operand: Option<u32>,
        class: DigitalValueClass,
    },
    /// A whole unpacked array the host writes element by element.
    Array {
        array: DigitalArrayRef,
        class: DigitalValueClass,
    },
    /// A module instance or named block named by a reference argument.
    Scope(DigitalScopeId),
    /// A net or variable named by a reference argument.
    Signal(DigitalSignalId),
}

/// The source line a call was written on, for its diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitalSourceLine {
    pub file: SmolStr,
    pub line: u32,
}

/// One system task or function call site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSystemCall {
    pub id: DigitalSystemCallId,
    pub kind: DigitalSystemCallKind,
    pub arguments: Vec<DigitalSystemArgument>,
    /// How many `DigitalCallOutput` values the call produces.
    pub outputs: u32,
    /// The value a function call returns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<DigitalValueClass>,
    /// The innermost scope the call is written in (`%m`).
    pub scope: DigitalScopeId,
    /// The owning process's module time scale, which a `%t` argument and
    /// `$timeformat` read their operands in.
    pub time_scale: crate::time_scale::ModuleTimeScale,
    /// `$finish(n)`, `$stop(n)` and `$fatal(n, ...)`'s constant level; the
    /// default 1 when it is omitted. A `$fatal` level is not a display
    /// argument, so it is absent from `arguments`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    /// The `$strobe`/`$monitor` argument function, for an observer kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observer: Option<u32>,
    pub span: SourceSpanRef,
    /// Where the call was written, once a source map has resolved the span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<DigitalSourceLine>,
}

/// The argument function of a `$strobe` or `$monitor` site.
///
/// Its entry block evaluates the arguments and ends in the site's own
/// `DigitalSystemTask` node, then returns; it never suspends. Every value it
/// reads is a signal — a process-local the arguments name is promoted to
/// signal storage — so it can run outside its process's activation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalObserver {
    pub site: DigitalSystemCallId,
    pub process: DigitalProcessId,
    pub function: CfgFunction,
    /// Every signal the arguments read, ascending: a `$monitor` prints when
    /// one of them changes.
    pub dependencies: Vec<DigitalSignalId>,
}

/// An interpolated analog event site. Operands refer to normal analog evaluation
/// slots in expr, delta, time_tol, expr_tol, enable order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalAbsDeltaObserver {
    pub time_scale: crate::time_scale::ModuleTimeScale,
    pub signal: DigitalSignalId,
    pub operands: [DigitalAnalogProbeId; 5],
    pub span: SourceSpanRef,
}

impl CanonicalDigitalPlan {
    pub fn is_empty(&self) -> bool {
        self.elaboration_parameters.is_empty() && !self.has_executable_content()
    }

    /// Whether this plan needs a digital runtime. Retained constants still
    /// participate in serialization and identity even without executable work.
    pub fn has_executable_content(&self) -> bool {
        !self.signals.is_empty()
            || !self.arrays.is_empty()
            || !self.bit_aliases.is_empty()
            || !self.real_aliases.is_empty()
            || !self.processes.is_empty()
            || !self.routines.is_empty()
            || !self.drivers.is_empty()
            || !self.analog_probes.is_empty()
            || !self.absdelta.is_empty()
    }

    /// Stable representatives for real-net views, including transitive and
    /// repeated connections. Validate endpoint types before unioning them so
    /// decoded artifacts and the runtime use exactly the same identity rules.
    pub fn real_net_representatives(&self) -> Result<Vec<DigitalSignalId>, &'static str> {
        let mut parents: Vec<_> = (0..self.signals.len()).collect();
        fn root(parents: &mut [usize], mut index: usize) -> usize {
            while parents[index] != index {
                parents[index] = parents[parents[index]];
                index = parents[index];
            }
            index
        }
        for alias in &self.real_aliases {
            let left = self
                .signal(alias.left)
                .ok_or("real-net alias names an unknown signal")?;
            let right = self
                .signal(alias.right)
                .ok_or("real-net alias names an unknown signal")?;
            if !left.kind.is_real()
                || left.kind != right.kind
                || left.procedurally_assignable
                || right.procedurally_assignable
            {
                return Err("real-net aliases require nets with the same real resolution");
            }
            let a = root(&mut parents, usize::from(alias.left));
            let b = root(&mut parents, usize::from(alias.right));
            parents[a.max(b)] = a.min(b);
        }
        Ok((0..parents.len())
            .map(|index| DigitalSignalId::from(root(&mut parents, index)))
            .collect())
    }

    /// The probe an id names.
    pub fn analog_probe(&self, id: DigitalAnalogProbeId) -> Option<&DigitalAnalogProbe> {
        self.analog_probes.get(usize::from(id))
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
        self.processes
            .get(usize::from(id))
            .filter(|process| process.id == id)
    }

    pub fn routine(&self, id: DigitalRoutineId) -> Option<&DigitalRoutine> {
        self.routines
            .get(id.index())
            .filter(|routine| routine.id == id)
    }
}

/// Dense identity of one task or function of the elaborated design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DigitalRoutineId(pub u32);

impl DigitalRoutineId {
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for DigitalRoutineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "routine#{}", self.0)
    }
}

/// Whether a routine is a task or a function (IEEE 1364-2005 section 10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalRoutineKind {
    /// May suspend; enabled as a statement; returns nothing but its outputs.
    Task,
    /// Runs without suspending inside an expression and yields its result.
    Function,
}

/// Where one routine variable lives (IEEE 1364-2005 section 10.2.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalRoutineStorage {
    /// A static routine's one shared copy: an ordinary signal, which every
    /// concurrent activation of the routine reads and writes.
    Signal(DigitalSignalId),
    /// An automatic routine's per-activation frame slot.
    Slot(u32),
}

/// One argument of a routine, in declaration order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalRoutineParam {
    pub name: SmolStr,
    pub direction: crate::ast::PortDirection,
    pub storage: DigitalRoutineStorage,
    /// The value type the argument holds: four-state of its width, or real.
    pub value_type: super::cfg::CfgValueType,
    pub signed: bool,
}

impl DigitalRoutineParam {
    /// Copied in at the call (section 10.2.2).
    pub const fn is_input(&self) -> bool {
        matches!(
            self.direction,
            crate::ast::PortDirection::Input | crate::ast::PortDirection::Inout
        )
    }

    /// Copied back when the task completes (section 10.2.2).
    pub const fn is_output(&self) -> bool {
        matches!(
            self.direction,
            crate::ast::PortDirection::Output | crate::ast::PortDirection::Inout
        )
    }
}

/// A task or function, lowered (IEEE 1364-2005 section 10, design P2 §4.1).
///
/// A call enters [`Self::function`] at its entry block with the inputs copied
/// into their storage, and a `Return` ends the activation: a task's outputs
/// and a function's result are read back out of storage then. Static routines
/// keep every variable in signals, so concurrent activations share them;
/// automatic routines keep theirs in the activation's frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalRoutine {
    pub id: DigitalRoutineId,
    pub name: SmolStr,
    pub kind: DigitalRoutineKind,
    pub automatic: bool,
    pub function: CfgFunction,
    pub params: Vec<DigitalRoutineParam>,
    /// A function's result variable, named by the function (section 10.4.2).
    pub result: Option<DigitalRoutineParam>,
    /// The value types of an automatic activation's frame slots; each starts
    /// at its type's default (`x` bits, or 0.0).
    pub frame_slots: Vec<super::cfg::CfgValueType>,
    /// The scope `%m` names inside the routine and `disable` targets.
    pub scope: DigitalScopeId,
    pub time_scale: crate::time_scale::ModuleTimeScale,
    pub span: SourceSpanRef,
}

impl DigitalRoutine {
    /// The arguments copied back when a task completes, in declaration order.
    pub fn outputs(&self) -> impl Iterator<Item = &DigitalRoutineParam> {
        self.params.iter().filter(|param| param.is_output())
    }

    /// The arguments a call supplies, in declaration order.
    pub fn inputs(&self) -> impl Iterator<Item = &DigitalRoutineParam> {
        self.params.iter().filter(|param| param.is_input())
    }
}
