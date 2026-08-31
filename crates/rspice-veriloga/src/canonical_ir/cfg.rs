//! CFG: the canonical level that keeps control flow.
//!
//! Every level below this one describes a Verilog-A module as a flat list of
//! values with conditionals folded into per-value selects. That representation
//! is what forces the rest of the backend into its current shape, and this
//! module is the replacement: basic blocks, real terminators, and SSA values
//! whose merges are block parameters.
//!
//! ## Why blocks
//!
//! Dissolving `if` into `guard ? value : previous` costs three things at once.
//! The "previous" value has to be recovered by searching backwards through the
//! assignment history, which is bounded heuristic work where SSA construction is
//! exact and linear. Emitted source then scales as operations x guard depth x
//! derivative lanes, which is what makes the largest models too big to emit
//! directly. And operands shared between guarded consumers get hoisted above
//! their guard, so a disabled model option still costs — a hand-written model
//! skips it with one branch.
//!
//! Blocks fix all three by not throwing the information away in the first place.
//!
//! ## Block parameters instead of phi instructions
//!
//! A merge is expressed by giving the successor block a parameter and having
//! each predecessor pass an argument on its terminator, rather than by a `phi`
//! pseudo-instruction that reads "the value from predecessor N". The two are
//! equivalent, but parameters keep every value definition inside exactly one
//! block, which means a pass never has to special-case instructions that are
//! positional with respect to their predecessors. Differentiating a merge then
//! becomes what it should be: a block that takes one more parameter.
//!
//! ## Construction
//!
//! [`SsaBuilder`] implements Braun et al., *Simple and Efficient Construction of
//! Static Single Assignment Form* (CC 2013). It builds pruned, minimal SSA
//! directly from the structured statements as they are walked, with no dominance
//! frontier and no separate renaming pass. A block is *sealed* once every
//! predecessor it will ever have is known; reads before that point leave an
//! incomplete parameter behind, which is filled in on sealing. Loops are the
//! only construct that needs the delay, because a loop header is reachable from
//! its own body.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::digital::{
    DigitalDriverId, DigitalSchedulingRegion, DigitalSensitivityTerm, DigitalWriteTarget,
};
use super::digital_value;
use super::{
    BlockId, BranchId, BranchUnknownId, ContributionId, DigitalLocalId, DigitalSignalId, ExprId,
    NodeId, ParamId, ShapeId, ValueId, VariableId,
};

/// What SSA tracks a definition for.
///
/// Module variables are the obvious case. Contribution residuals are here for
/// the same reason a hand-written model accumulates into a local: `I(a,b) <+ x`
/// inside an `if` is `acc = acc + x` on the taken path and nothing on the other,
/// so the merge at the join is the ordinary one the block-parameter machinery
/// already builds. Modelling it any other way would mean a second, parallel
/// merge mechanism for effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgVariable {
    Local(VariableId),
    /// A variable declared inside a digital process, which lives in SSA rather
    /// than in the signal store.
    ///
    /// Separate from [`Self::Local`] because the two are drawn from different
    /// numbering spaces — one indexes the module's analog variables, the other
    /// the declarations of one process — and a merge that confused them would
    /// read an analog `real` where a four-state counter belongs.
    DigitalLocal(DigitalLocalId),
    Residual(ContributionId),
    /// Whether a potential contribution's leading instance-static guard prefix
    /// enables its topology. This is not ordinary statement reachability: once
    /// a bias-, time-, or state-dependent guard is encountered, topology stays
    /// active even when runtime control does not take the contribution's path.
    /// It must remain separate from the residual because an inactive/open
    /// contribution and an active ideal zero-volt source both have a numeric
    /// residual of zero, but only the latter closes the branch topology.
    Activation(ContributionId),
}

/// Arithmetic on one operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgUnaryOp {
    Neg,
    Not,
    Exp,
    LimExp,
    /// The runtime's own bounded exponential, distinct from `limexp`: it is
    /// clamped from below as well as above, so it has its own derivative.
    LimitedExp,
    /// `d/dx` of [`Self::LimitedExp`], which is not itself expressible as one.
    LimitedExpDerivative,
    Ln,
    Log10,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Asinh,
    Acosh,
    Atanh,
    Floor,
    Ceil,
}

/// Arithmetic on two operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    /// Kept as an operation rather than lowered to a branch: a hand-written
    /// model writes `fmin`, and turning every `min` in a BSIM model into a
    /// diamond would cost more blocks than the whole rest of the module.
    Min,
    Max,
    /// `hypot(x, y)`, kept as one operation because it is the numerically
    /// careful form the library provides and squaring the operands to rebuild
    /// it would overflow exactly where it exists to not.
    Hypot,
    /// `atan2(y, x)`, left as its own operation for the same reason `hypot` is:
    /// the quadrant is part of the answer, and a quotient loses it.
    Atan2,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// What an SSA value is.
///
/// Deliberately without a `Select`: a conditional is a [`CfgTerminator::Branch`]
/// and a merge is a block parameter. Deliberately without loop constructs:
/// a loop is a back edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CfgValueKind {
    RealConstant(f64),
    BooleanConstant(bool),
    /// Defined by the enclosing block's parameter list, not by an instruction.
    BlockParameter,
    Parameter(ParamId),
    ParameterGiven(ParamId),
    /// Accepted procedural state for a variable written by an event-controlled
    /// body. The dense slot is stable within one generated model.
    EventState(u32),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis(smol_str::SmolStr),
    /// `$simparam("name", fallback)`.
    SimParam {
        name: smol_str::SmolStr,
        fallback: ValueId,
    },
    NodePotential(NodeId),
    BranchFlow(BranchId),
    BranchUnknownFlow(BranchUnknownId),
    /// Time derivative of `input`, keyed by the source operator so its state
    /// slot survives any reordering the passes do.
    Ddt {
        operator: ExprId,
        input: ValueId,
    },
    /// The integration rule's `d/dt` coefficient for the current step.
    ///
    /// `ddt` differentiates to this times the derivative of its input, rather
    /// than to another `ddt`: a second `ddt` would claim a second state slot for
    /// a quantity that has no history of its own.
    DdtScale,
    /// `idt(x, ic)` — the time integral of `x`, in companion form.
    ///
    /// Carries its initial condition rather than defaulting it, because a step
    /// with no history to integrate from returns `ic` and not zero, and the
    /// operator id because the running total is a per-instance slot keyed by the
    /// call — the same reason [`Self::Ddt`] carries one.
    Idt {
        operator: ExprId,
        input: ValueId,
        ic: ValueId,
    },
    /// The integration rule's `dt` for the current step, and zero where there is
    /// no step to integrate over.
    ///
    /// `idt` differentiates to this times the derivative of its input, for the
    /// reason [`Self::DdtScale`] exists: a second `idt` would claim a second
    /// slot for a quantity with no history of its own.
    IdtScale,
    /// `cross(expr, direction, time_tol, expr_tol, enable)`, evaluated from
    /// accepted detector history into a speculative candidate lane.
    Cross {
        operator: ExprId,
        input: ValueId,
        direction: ValueId,
        time_tol: ValueId,
        expr_tol: ValueId,
        enable: ValueId,
    },
    /// `above(expr, time_tol, expr_tol, enable)`, including its initial-positive
    /// equilibrium behavior.
    Above {
        operator: ExprId,
        input: ValueId,
        time_tol: ValueId,
        expr_tol: ValueId,
        enable: ValueId,
    },
    /// `timer(start, period, time_tol, enable)`, which also requests the next
    /// exact transient breakpoint.
    Timer {
        operator: ExprId,
        start: ValueId,
        period: ValueId,
        time_tol: ValueId,
        enable: ValueId,
    },
    /// Newton limiting; carries an affine correction lane in the derivative
    /// pass whether or not `proposed` depends on an unknown.
    ///
    /// `candidate` is the inlined limiter body, not an alternative value:
    /// evaluating it *is* running the limiter. Its implicit arguments reach it
    /// as [`Self::LimitPrevious`] and as `proposed` itself.
    Limit {
        operator: ExprId,
        proposed: ValueId,
        candidate: ValueId,
        /// Which limiting function the source named, e.g. `pnjlim`.
        selector: smol_str::SmolStr,
    },
    /// `ddx(value, V(pos, neg))` — one entry of the Jacobian, read back into
    /// the model.
    ///
    /// Left symbolic here because the lanes it names do not exist until the
    /// derivative pass runs; that pass replaces it with the lane itself. A
    /// model that reads its own transconductance is asking for a value the
    /// backend already computes, and this is how it gets the same one.
    Ddx {
        value: ValueId,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    },
    /// The value this `$limit` returned on the previous Newton iteration.
    ///
    /// Equal to `proposed` whenever there is no previous iterate to speak of,
    /// which is what makes a static probe of a limited model well defined.
    LimitPrevious {
        operator: ExprId,
        proposed: ValueId,
    },
    Unary {
        op: CfgUnaryOp,
        input: ValueId,
    },
    Binary {
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    },

    // ---- Packed derivative lanes -------------------------------------------
    //
    // A derivative is a partial with respect to each unknown it can reach, and
    // giving each of those its own SSA value costs one emitted line per lane.
    // These kinds carry the whole set as one value instead. The set — the
    // value's *shape* — is exactly what the liveness analysis proved reachable,
    // so packing costs no arithmetic on lanes that are structurally zero: the
    // corpus averages 3-5 live lanes against 30-odd unknowns, and a uniform
    // width would multiply the work by that ratio.
    /// Every lane of this value's shape set to one constant. The seed for an
    /// unknown is a one-lane shape holding `1.0`; a merge that one arm cannot
    /// reach passes a zero.
    LaneSplat(f64),
    /// `input` re-laid-out into this value's shape, zero in lanes it lacks.
    ///
    /// Its shape must be a superset of the input's, which is what an operand
    /// reaching a merge or a wider consumer needs.
    LaneWiden {
        input: ValueId,
    },
    /// Elementwise over two packed values that share this value's shape.
    /// Only [`CfgBinaryOp::Add`] and [`CfgBinaryOp::Sub`] appear here.
    LaneBinary {
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    /// Every lane of `input` combined with one scalar — the chain rule's
    /// multiplier. Only [`CfgBinaryOp::Mul`] and [`CfgBinaryOp::Div`] appear,
    /// and the scalar is always on the right so the division is the one the
    /// quotient rule wrote.
    LaneScalar {
        op: CfgBinaryOp,
        input: ValueId,
        scalar: ValueId,
    },
    /// One lane of a packed value, named by the unknown rather than by its
    /// position, so a reader does not have to know the shape's layout.
    LaneExtract {
        input: ValueId,
        lane: u32,
    },

    /// A value a coarser invalidation stage computed and cached.
    ///
    /// What one slice of a split body reads from another. It is a leaf here on
    /// purpose: within a stage the value has no derivation, which is exactly
    /// what makes the stage a function of its own inputs and cacheable
    /// independently.
    Staged {
        slot: u32,
    },

    // ---- Discrete domain ---------------------------------------------------
    //
    // These appear only in a digital process function. None of them is
    // differentiable, none is a function of a solver unknown, and none may
    // reach a residual — see `CfgValueType::is_digital`, which is what the
    // analog passes assert against.
    //
    // The operators are not folded into `Binary`. `CfgBinaryOp::Add` is
    // addition on the reals; four-state addition is a different function with
    // a different domain, whose answer to `1 + x` is `x` rather than a number.
    // One enum for both would put that distinction behind an operand type
    // check in every consumer.
    /// A four-state literal, already decoded to its planes.
    FourStateConstant(digital_value::FourStateValue),
    /// A signed 32-bit integer constant.
    IntegerConstant(i32),
    /// The current value of a declared net or variable.
    ///
    /// A leaf: within one process function a signal has no derivation. Two
    /// reads of the same signal on either side of a `Wait` are two of these
    /// and may differ, which is why they are not common-subexpressioned.
    DigitalSignalRead {
        signal: DigitalSignalId,
    },
    /// Elementwise bitwise operator over four-state values.
    DigitalBitwise {
        op: digital_value::BitwiseOp,
        left: ValueId,
        right: ValueId,
    },
    /// Bitwise negation (`~`).
    DigitalBitwiseNot {
        input: ValueId,
    },
    /// Logical operator (`&&`, `||`), yielding one bit.
    DigitalLogical {
        op: digital_value::LogicalOp,
        left: ValueId,
        right: ValueId,
    },
    /// Logical negation (`!`), yielding one bit.
    DigitalLogicalNot {
        input: ValueId,
    },
    /// Logical equality (`==`) or inequality (`!=`), yielding one bit, unknown
    /// if either operand has an `x`/`z` bit.
    DigitalEquality {
        left: ValueId,
        right: ValueId,
        negate: bool,
    },
    /// A `case` item match, yielding one bit and never an unknown one.
    ///
    /// A different operator from [`Self::DigitalEquality`], not a use of it.
    /// IEEE 1364-2005 section 9.5 compares a case item against the selector
    /// bit by bit *including* `x` and `z`, so a selector of `x0` matches the
    /// item `2'bx0` — while `==` yields `x` there and would send it to the
    /// default. Section 9.5.1's `casez` and `casex` are the same comparison
    /// with a set of positions ignored, which is why the three forms are one
    /// node with a kind rather than three lowerings.
    DigitalCaseMatch {
        selector: ValueId,
        label: ValueId,
        kind: digital_value::DigitalCaseMatch,
    },
    /// Relational comparison, yielding one bit.
    DigitalRelational {
        op: digital_value::RelationalOp,
        left: ValueId,
        right: ValueId,
    },
    /// Arithmetic on four-state values, all-unknown if any operand bit is.
    DigitalArithmetic {
        op: digital_value::ArithmeticOp,
        left: ValueId,
        right: ValueId,
    },
    /// Logical shift, keeping the shifted value's width.
    DigitalShift {
        op: digital_value::ShiftOp,
        value: ValueId,
        count: ValueId,
    },
    /// A constant bit or part select. Bounds are as written, so `[0:7]` and
    /// `[7:0]` select the same bits of differently declared signals.
    DigitalPartSelect {
        input: ValueId,
        msb: i64,
        lsb: i64,
    },
    /// Concatenation. The first part supplies the most significant bits.
    DigitalConcat {
        parts: Vec<ValueId>,
    },
    /// `condition ? then_value : else_value` as a value rather than a branch.
    ///
    /// A conditional *expression* does not suspend, so making it a branch
    /// would split a block for no reason and force the result through a block
    /// parameter. Conditional *statements* still lower to real branches.
    DigitalSelect {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    /// A blocking write (`=`), visible to the next instruction.
    DigitalBlockingWrite {
        target: DigitalWriteTarget,
        value: ValueId,
    },
    /// A nonblocking write (`<=`), deferred to a later scheduling region.
    ///
    /// Kept a separate kind from the blocking write rather than a flag on one.
    /// The two have different execution semantics and different visibility,
    /// and every consumer has to treat them differently; a shared kind would
    /// make "did you check the flag?" a review question on each one.
    DigitalNonblockingWrite {
        target: DigitalWriteTarget,
        value: ValueId,
        /// Where the update lands. Always
        /// [`DigitalSchedulingRegion::NonBlockingAssign`] for a `<=` written
        /// in source; carried explicitly so the region is read off the node
        /// rather than assumed from the kind.
        region: DigitalSchedulingRegion,
    },
    /// A continuous driver's contribution to a net (IEEE 1364-2005 section
    /// 6.1).
    ///
    /// A third write kind, and not a flag on either of the others, because it
    /// does not write the net: it publishes *this driver's* value for it. A net
    /// with two drivers has one value per driver and a resolution function
    /// between them (section 7.9), and a write node that stored into the net
    /// would have destroyed the other driver's contribution before the
    /// resolver ever saw it. The driver identity is what keeps them apart, and
    /// it is on the node so that no consumer has to recover it by asking which
    /// process it is standing in.
    ///
    /// Resolution itself is not the compiler's: which value wins when two
    /// drivers disagree is a simulation-kernel rule over the whole net, and
    /// the kernel owns the table.
    DigitalDriverWrite {
        driver: DigitalDriverId,
        /// Which bits this driver drives. `driver.signal` and `target.signal`
        /// name the same net — the id says *which driver of it*, the target
        /// says *which of its bits*.
        target: DigitalWriteTarget,
        value: ValueId,
    },
}

impl CfgValueKind {
    /// Whether this kind belongs to the discrete-domain half of the language.
    ///
    /// Deliberately an exhaustive match with no catch-all, and the only one
    /// over this enum that exists purely to be exhaustive. Every other place a
    /// new kind has to be considered — `operands`, `leaf_class`,
    /// `static_dependencies`, the emitter's inlining predicates — has a
    /// catch-all that will absorb it in silence. This one will not compile
    /// until somebody says which half of the language the new kind is in, and
    /// the answer is what the scheduling, derivative, and emitter refusals are
    /// all keyed on.
    pub fn is_digital(&self) -> bool {
        match self {
            Self::RealConstant(_)
            | Self::BooleanConstant(_)
            | Self::BlockParameter
            | Self::Parameter(_)
            | Self::ParameterGiven(_)
            | Self::EventState(_)
            | Self::Temperature
            | Self::ThermalVoltage
            | Self::Multiplicity
            | Self::Time
            | Self::Analysis(_)
            | Self::SimParam { .. }
            | Self::NodePotential(_)
            | Self::BranchFlow(_)
            | Self::BranchUnknownFlow(_)
            | Self::Ddt { .. }
            | Self::DdtScale
            | Self::Idt { .. }
            | Self::IdtScale
            | Self::Cross { .. }
            | Self::Above { .. }
            | Self::Timer { .. }
            | Self::Limit { .. }
            | Self::Ddx { .. }
            | Self::LimitPrevious { .. }
            | Self::Unary { .. }
            | Self::Binary { .. }
            | Self::LaneSplat(_)
            | Self::LaneWiden { .. }
            | Self::LaneBinary { .. }
            | Self::LaneScalar { .. }
            | Self::LaneExtract { .. }
            | Self::Staged { .. } => false,

            Self::FourStateConstant(_)
            | Self::IntegerConstant(_)
            | Self::DigitalSignalRead { .. }
            | Self::DigitalBitwise { .. }
            | Self::DigitalBitwiseNot { .. }
            | Self::DigitalLogical { .. }
            | Self::DigitalLogicalNot { .. }
            | Self::DigitalEquality { .. }
            | Self::DigitalCaseMatch { .. }
            | Self::DigitalRelational { .. }
            | Self::DigitalArithmetic { .. }
            | Self::DigitalShift { .. }
            | Self::DigitalPartSelect { .. }
            | Self::DigitalConcat { .. }
            | Self::DigitalSelect { .. }
            | Self::DigitalBlockingWrite { .. }
            | Self::DigitalNonblockingWrite { .. }
            | Self::DigitalDriverWrite { .. } => true,
        }
    }

    /// Values this one reads.
    ///
    /// Every pass that rewrites or walks the graph needs this, and having one
    /// definition of it is what keeps a newly added kind from being silently
    /// skipped by half of them.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Unary { input, .. }
            | Self::Ddt { input, .. }
            | Self::Ddx { value: input, .. }
            | Self::LaneWiden { input }
            | Self::LaneExtract { input, .. }
            | Self::LimitPrevious {
                proposed: input, ..
            } => vec![*input],
            Self::SimParam { fallback, .. } => vec![*fallback],
            Self::Binary { left, right, .. } | Self::LaneBinary { left, right, .. } => {
                vec![*left, *right]
            }
            Self::LaneScalar { input, scalar, .. } => vec![*input, *scalar],
            Self::Idt { input, ic, .. } => vec![*input, *ic],
            Self::Cross {
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => vec![*input, *direction, *time_tol, *expr_tol, *enable],
            Self::Above {
                input,
                time_tol,
                expr_tol,
                enable,
                ..
            } => vec![*input, *time_tol, *expr_tol, *enable],
            Self::Timer {
                start,
                period,
                time_tol,
                enable,
                ..
            } => vec![*start, *period, *time_tol, *enable],
            Self::Limit {
                proposed,
                candidate,
                ..
            } => vec![*proposed, *candidate],

            Self::DigitalBitwiseNot { input }
            | Self::DigitalLogicalNot { input }
            | Self::DigitalPartSelect { input, .. } => vec![*input],
            Self::DigitalBitwise { left, right, .. }
            | Self::DigitalLogical { left, right, .. }
            | Self::DigitalEquality { left, right, .. }
            | Self::DigitalRelational { left, right, .. }
            | Self::DigitalArithmetic { left, right, .. } => vec![*left, *right],
            Self::DigitalCaseMatch {
                selector, label, ..
            } => vec![*selector, *label],
            Self::DigitalShift { value, count, .. } => vec![*value, *count],
            Self::DigitalConcat { parts } => parts.clone(),
            Self::DigitalSelect {
                condition,
                then_value,
                else_value,
            } => vec![*condition, *then_value, *else_value],
            Self::DigitalBlockingWrite { value, .. }
            | Self::DigitalNonblockingWrite { value, .. }
            | Self::DigitalDriverWrite { value, .. } => vec![*value],

            _ => Vec::new(),
        }
    }

    pub(crate) fn map_operands(&mut self, mut map: impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Unary { input, .. }
            | Self::Ddt { input, .. }
            | Self::Ddx { value: input, .. }
            | Self::LaneWiden { input }
            | Self::LaneExtract { input, .. }
            | Self::LimitPrevious {
                proposed: input, ..
            } => *input = map(*input),
            Self::SimParam { fallback, .. } => *fallback = map(*fallback),
            Self::Binary { left, right, .. } | Self::LaneBinary { left, right, .. } => {
                *left = map(*left);
                *right = map(*right);
            }
            Self::LaneScalar { input, scalar, .. } => {
                *input = map(*input);
                *scalar = map(*scalar);
            }
            Self::Idt { input, ic, .. } => {
                *input = map(*input);
                *ic = map(*ic);
            }
            Self::Cross {
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                *input = map(*input);
                *direction = map(*direction);
                *time_tol = map(*time_tol);
                *expr_tol = map(*expr_tol);
                *enable = map(*enable);
            }
            Self::Above {
                input,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                *input = map(*input);
                *time_tol = map(*time_tol);
                *expr_tol = map(*expr_tol);
                *enable = map(*enable);
            }
            Self::Timer {
                start,
                period,
                time_tol,
                enable,
                ..
            } => {
                *start = map(*start);
                *period = map(*period);
                *time_tol = map(*time_tol);
                *enable = map(*enable);
            }
            Self::Limit {
                proposed,
                candidate,
                ..
            } => {
                *proposed = map(*proposed);
                *candidate = map(*candidate);
            }

            Self::DigitalBitwiseNot { input }
            | Self::DigitalLogicalNot { input }
            | Self::DigitalPartSelect { input, .. } => *input = map(*input),
            Self::DigitalBitwise { left, right, .. }
            | Self::DigitalLogical { left, right, .. }
            | Self::DigitalEquality { left, right, .. }
            | Self::DigitalRelational { left, right, .. }
            | Self::DigitalArithmetic { left, right, .. } => {
                *left = map(*left);
                *right = map(*right);
            }
            Self::DigitalCaseMatch {
                selector, label, ..
            } => {
                *selector = map(*selector);
                *label = map(*label);
            }
            Self::DigitalShift { value, count, .. } => {
                *value = map(*value);
                *count = map(*count);
            }
            Self::DigitalConcat { parts } => {
                for part in parts {
                    *part = map(*part);
                }
            }
            Self::DigitalSelect {
                condition,
                then_value,
                else_value,
            } => {
                *condition = map(*condition);
                *then_value = map(*then_value);
                *else_value = map(*else_value);
            }
            Self::DigitalBlockingWrite { value, .. }
            | Self::DigitalNonblockingWrite { value, .. }
            | Self::DigitalDriverWrite { value, .. } => *value = map(*value),

            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgValueType {
    Real,
    Boolean,
    /// A packed derivative, one `f64` per unknown in the named shape.
    Lanes(ShapeId),
    /// A signed 32-bit two's-complement integer.
    ///
    /// Distinct from [`Self::Real`] rather than tunnelled through it. The
    /// analog half does carry `integer` in an `f64` — that ABI is frozen and
    /// is not what this is — but a discrete-domain index, shift count, or
    /// delay is an integer whose wrapping and division behaviour is defined,
    /// and rounding it out of a float at each use is how those definitions get
    /// lost.
    Integer,
    /// A four-state value of a fixed width, held as `aval`/`bval` planes.
    ///
    /// The width is part of the type because almost every IEEE 1364-2005
    /// operator is width-sensitive: an assignment truncates or zero-extends to
    /// the target's width, and arithmetic that overflows the operand width
    /// wraps within it. A four-state value whose width is not known statically
    /// is not something this IR can represent, which is deliberate.
    FourState {
        width: u32,
    },
    /// Carries ordering, not data.
    ///
    /// The type of a signal write. A write has to sit in the instruction
    /// stream — its position is its execution order — but nothing may read
    /// what it "produces", and giving it the written value's type would invite
    /// exactly that. A nonblocking write in particular has no readable result
    /// at the point it appears: its update is not visible until the
    /// nonblocking region flushes.
    Effect,
}

impl CfgValueType {
    pub fn shape(self) -> Option<ShapeId> {
        match self {
            Self::Lanes(shape) => Some(shape),
            Self::Real | Self::Boolean | Self::Integer | Self::FourState { .. } | Self::Effect => {
                None
            }
        }
    }

    /// Whether this type belongs to the discrete-domain half of the language.
    ///
    /// The analog solver has no representation for any of these, so a value
    /// carrying one reaching a residual, a Jacobian lane, or a stage cache is
    /// a compiler bug rather than an unsupported model.
    pub const fn is_digital(self) -> bool {
        matches!(self, Self::Integer | Self::FourState { .. } | Self::Effect)
    }

    /// Declared width in bits, for the types that have one.
    pub const fn width(self) -> Option<u32> {
        match self {
            Self::FourState { width } => Some(width),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgValue {
    pub id: ValueId,
    pub value_type: CfgValueType,
    pub kind: CfgValueKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgInstruction {
    pub result: ValueId,
}

/// How a block ends. Every block has exactly one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgTerminator {
    Jump {
        target: BlockId,
        args: Vec<ValueId>,
    },
    Branch {
        condition: ValueId,
        then_target: BlockId,
        then_args: Vec<ValueId>,
        else_target: BlockId,
        else_args: Vec<ValueId>,
    },
    Return,
    /// Suspend a digital process here and resume into `resume` when the wait
    /// is satisfied.
    ///
    /// The suspension point the author wrote, in the position they wrote it.
    /// Resuming is a `Jump`: `resume_args` bind to `resume`'s block
    /// parameters, so whatever the process needs to survive the suspension
    /// travels through parameters like any other cross-block value, and no
    /// pass has to know that a process has state.
    ///
    /// Appears only in a digital process function. The analog body never
    /// suspends — a Newton iteration runs to completion or fails — so every
    /// analog pass treats this as unreachable rather than handling it.
    Wait {
        wait: DigitalWait,
        resume: BlockId,
        resume_args: Vec<ValueId>,
    },
    /// Placeholder while a block is under construction. Never present in a
    /// finished function; [`CfgFunction::validate`] rejects it.
    Unset,
}

/// What a suspended process is waiting for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalWait {
    /// `@(...)`: resume when any listed event occurs.
    ///
    /// An empty list would be a process that can never resume; the lowering
    /// refuses one rather than emitting it.
    Event(Vec<DigitalSensitivityTerm>),
    /// `#delay`: resume after this many time units have elapsed.
    ///
    /// The operand is an [`CfgValueType::Integer`] value, evaluated when the
    /// wait is reached rather than when the process starts — `#(n)` where `n`
    /// is a variable waits for its value at that moment.
    Delay(ValueId),
}

impl DigitalWait {
    /// Values this wait reads.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Event(_) => Vec::new(),
            Self::Delay(delay) => vec![*delay],
        }
    }

    /// Rewrite the values this wait reads.
    ///
    /// The mutable twin of [`Self::operands`]. Every renumbering pass has to
    /// call it or a delay operand survives a compaction pointing at whatever
    /// value took its old index.
    pub fn map_operands(&mut self, mut map: impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Event(_) => {}
            Self::Delay(delay) => *delay = map(*delay),
        }
    }

    /// The signals whose events can resume the process.
    pub fn sensitivity(&self) -> &[DigitalSensitivityTerm] {
        match self {
            Self::Event(terms) => terms,
            Self::Delay(_) => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgBlock {
    pub id: BlockId,
    /// Values this block defines by merging its predecessors' arguments.
    pub params: Vec<ValueId>,
    pub instructions: Vec<CfgInstruction>,
    pub terminator: CfgTerminator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgFunction {
    pub entry: BlockId,
    pub blocks: Vec<CfgBlock>,
    pub values: Vec<CfgValue>,
    /// The lane sets packed values are laid out over, each sorted and interned
    /// so a shape is compared by id. Empty until the derivative pass runs.
    #[serde(default)]
    pub shapes: Vec<Vec<u32>>,
}

impl CfgFunction {
    pub fn block(&self, id: BlockId) -> &CfgBlock {
        &self.blocks[usize::from(id)]
    }

    pub fn value(&self, id: ValueId) -> &CfgValue {
        &self.values[usize::from(id)]
    }

    /// The unknowns a shape carries, in layout order.
    pub fn shape(&self, id: ShapeId) -> &[u32] {
        &self.shapes[usize::from(id)]
    }

    /// The lane set of `value`, or `None` if it is an ordinary scalar.
    pub fn lanes_of(&self, value: ValueId) -> Option<&[u32]> {
        self.value(value)
            .value_type
            .shape()
            .map(|id| self.shape(id))
    }

    /// Where `lane` sits inside `value`'s shape.
    pub fn lane_position(&self, value: ValueId, lane: u32) -> Option<usize> {
        self.lanes_of(value)?
            .iter()
            .position(|candidate| *candidate == lane)
    }

    /// Blocks that jump or branch to `target`.
    pub fn predecessors(&self, target: BlockId) -> Vec<BlockId> {
        self.blocks
            .iter()
            .filter(|block| block.successors().contains(&target))
            .map(|block| block.id)
            .collect()
    }

    /// Structural checks a well-formed function must satisfy.
    ///
    /// Cheap enough to run after every construction, and worth it: an SSA bug
    /// that reaches the derivative pass presents as a wrong number rather than
    /// as a malformed graph, and is correspondingly harder to find.
    pub fn validate(&self) -> Result<(), CfgValidationError> {
        for block in &self.blocks {
            if matches!(block.terminator, CfgTerminator::Unset) {
                return Err(CfgValidationError::UnterminatedBlock(block.id));
            }
            for successor in block.successors() {
                let arg_count = block.arguments_to(successor).map_or(0, <[ValueId]>::len);
                let param_count = self.block(successor).params.len();
                if arg_count != param_count {
                    return Err(CfgValidationError::ArgumentCountMismatch {
                        from: block.id,
                        to: successor,
                        expected: param_count,
                        found: arg_count,
                    });
                }
            }
        }

        // Every value must be defined exactly once, either as a block parameter
        // or by an instruction.
        let mut defined = vec![0usize; self.values.len()];
        for block in &self.blocks {
            for param in &block.params {
                defined[usize::from(*param)] += 1;
            }
            for instruction in &block.instructions {
                defined[usize::from(instruction.result)] += 1;
            }
        }
        for (index, count) in defined.iter().enumerate() {
            let id = ValueId::from(index);
            match (count, &self.values[index].kind) {
                (1, _) => {}
                (0, CfgValueKind::BlockParameter) => {
                    return Err(CfgValidationError::UndefinedValue(id));
                }
                // Constants and leaves may be referenced without being placed in
                // a block; everything else must be scheduled somewhere.
                (0, kind) if is_leaf(kind) => {}
                (0, _) => return Err(CfgValidationError::UndefinedValue(id)),
                _ => return Err(CfgValidationError::MultiplyDefinedValue(id)),
            }
        }

        self.validate_shapes()
    }

    /// Packed values agree with their operands about which unknowns they carry.
    ///
    /// Worth checking because the failure it catches is otherwise silent: a
    /// derivative laid out over the wrong lane set still evaluates, and reports
    /// one unknown's partial in another's slot.
    fn validate_shapes(&self) -> Result<(), CfgValidationError> {
        for value in &self.values {
            let lanes = self.value_lanes(value.id);
            match &value.kind {
                CfgValueKind::LaneSplat(_) | CfgValueKind::BlockParameter => {}
                CfgValueKind::LaneWiden { input } => {
                    // A superset, not equality: widening is how a narrow operand
                    // reaches a merge that other arms made wider.
                    let source = self.value_lanes(*input);
                    if !source.iter().all(|lane| lanes.contains(lane)) {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneBinary { left, right, .. } => {
                    if self.value_lanes(*left) != lanes || self.value_lanes(*right) != lanes {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneScalar { input, scalar, .. } => {
                    if self.value_lanes(*input) != lanes
                        || self.value(*scalar).value_type.shape().is_some()
                    {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneExtract { input, lane } => {
                    if !lanes.is_empty() || !self.value_lanes(*input).contains(lane) {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                // Every other kind is scalar arithmetic, and a packed operand
                // reaching one is the mistake this catches.
                kind => {
                    if !lanes.is_empty()
                        || kind
                            .operands()
                            .into_iter()
                            .any(|operand| self.value(operand).value_type.shape().is_some())
                    {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
            }
        }
        Ok(())
    }

    fn value_lanes(&self, value: ValueId) -> &[u32] {
        self.lanes_of(value).unwrap_or(&[])
    }
}

/// Whether a value may be referenced without any block defining it.
///
/// Also what decides, when a body is split by invalidation class, whether a
/// value crossing a stage boundary needs a cache slot or is simply rebuilt: a
/// leaf reads nothing, so rebuilding it is free and costs no slot.
pub(super) fn is_leaf_kind(kind: &CfgValueKind) -> bool {
    is_leaf(kind)
}

fn is_leaf(kind: &CfgValueKind) -> bool {
    matches!(
        kind,
        CfgValueKind::RealConstant(_)
            | CfgValueKind::BooleanConstant(_)
            | CfgValueKind::Parameter(_)
            | CfgValueKind::ParameterGiven(_)
            | CfgValueKind::EventState(_)
            | CfgValueKind::Temperature
            | CfgValueKind::ThermalVoltage
            | CfgValueKind::Multiplicity
            | CfgValueKind::Time
            | CfgValueKind::Analysis(_)
            | CfgValueKind::DdtScale
            | CfgValueKind::IdtScale
            | CfgValueKind::NodePotential(_)
            | CfgValueKind::BranchFlow(_)
            | CfgValueKind::BranchUnknownFlow(_)
            | CfgValueKind::LaneSplat(_)
            | CfgValueKind::Staged { .. }
            // Digital constants are leaves for the same reason a real constant
            // is: they read nothing and rebuilding one is free.
            //
            // `DigitalSignalRead` is deliberately *not* here. It reads nothing
            // in the operand sense, but it is not a constant — its value is
            // whatever the signal holds when the node runs, and two reads on
            // either side of a `Wait` are meant to differ. It has to stay
            // pinned to a block so that ordering survives.
            | CfgValueKind::FourStateConstant(_)
            | CfgValueKind::IntegerConstant(_)
    )
}

impl CfgBlock {
    pub fn successors(&self) -> Vec<BlockId> {
        match &self.terminator {
            CfgTerminator::Jump { target, .. } => vec![*target],
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                if then_target == else_target {
                    vec![*then_target]
                } else {
                    vec![*then_target, *else_target]
                }
            }
            // A suspension is an edge like any other. The process resumes into
            // `resume`, and every pass that walks the graph has to see that or
            // it will call the resumed half unreachable and delete it.
            CfgTerminator::Wait { resume, .. } => vec![*resume],
            CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
        }
    }

    fn arguments_to(&self, successor: BlockId) -> Option<&[ValueId]> {
        match &self.terminator {
            CfgTerminator::Jump { target, args } if *target == successor => Some(args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                ..
            } if *then_target == successor => Some(then_args),
            CfgTerminator::Branch {
                else_target,
                else_args,
                ..
            } if *else_target == successor => Some(else_args),
            // Resuming passes arguments exactly as a jump does, so a block
            // parameter filled in on sealing finds them here.
            CfgTerminator::Wait {
                resume,
                resume_args,
                ..
            } if *resume == successor => Some(resume_args),
            _ => None,
        }
    }

    fn push_argument(&mut self, successor: BlockId, value: ValueId) {
        match &mut self.terminator {
            CfgTerminator::Jump { target, args } if *target == successor => args.push(value),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                if *then_target == successor {
                    then_args.push(value);
                }
                if *else_target == successor {
                    else_args.push(value);
                }
            }
            CfgTerminator::Wait {
                resume,
                resume_args,
                ..
            } if *resume == successor => resume_args.push(value),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgValidationError {
    UnterminatedBlock(BlockId),
    ArgumentCountMismatch {
        from: BlockId,
        to: BlockId,
        expected: usize,
        found: usize,
    },
    UndefinedValue(ValueId),
    MultiplyDefinedValue(ValueId),
    LaneShapeMismatch(ValueId),
    /// A discrete-domain value reached the derivative pass.
    ///
    /// Not an unsupported model — a compiler bug. Nothing in a four-state
    /// value is differentiable: it has no neighbourhood, and `x` is not a
    /// point you can perturb. The derivative pass runs on the analog body, so
    /// one of these arriving means a process function was handed to it, and
    /// the pass says so instead of quietly contributing a zero row to the
    /// Jacobian.
    DigitalValueInDerivative(ValueId),
}

impl std::fmt::Display for CfgValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedBlock(block) => write!(f, "{block} has no terminator"),
            Self::ArgumentCountMismatch {
                from,
                to,
                expected,
                found,
            } => write!(
                f,
                "{from} passes {found} arguments to {to}, which takes {expected}"
            ),
            Self::UndefinedValue(value) => write!(f, "{value} is never defined"),
            Self::MultiplyDefinedValue(value) => write!(f, "{value} is defined more than once"),
            Self::LaneShapeMismatch(value) => {
                write!(f, "{value} does not carry the lanes its operands do")
            }
            Self::DigitalValueInDerivative(value) => write!(
                f,
                "{value} is a discrete-domain value and cannot be differentiated"
            ),
        }
    }
}

impl std::error::Error for CfgValidationError {}

/// Incremental SSA construction over a CFG being built.
///
/// The caller drives it while walking structured statements: create blocks,
/// [`SsaBuilder::write_variable`] on assignment, [`SsaBuilder::read_variable`]
/// on use, and [`SsaBuilder::seal_block`] once a block's predecessors are all
/// known. Straight-line and `if`/`else` blocks can be sealed as soon as they are
/// created; a loop header cannot be sealed until its back edge exists.
pub struct SsaBuilder {
    blocks: Vec<CfgBlock>,
    values: Vec<CfgValue>,
    /// Latest definition of a variable within a block.
    definitions: HashMap<(CfgVariable, BlockId), ValueId>,
    /// Parameters created for a variable in a block that was not yet sealed.
    incomplete: Vec<(BlockId, CfgVariable, ValueId)>,
    sealed: HashSet<BlockId>,
    /// Predecessors recorded as edges are added, because a block under
    /// construction cannot be scanned for them.
    predecessors: HashMap<BlockId, Vec<BlockId>>,
    /// Values the caller needs after construction.
    ///
    /// They have to be held here rather than by the caller because finishing
    /// rewrites value ids twice — collapsing trivial merges, then compacting —
    /// and an id read out beforehand names something else afterwards.
    outputs: Vec<ValueId>,
    /// The type a variable's merges take, when it is not the default.
    ///
    /// Empty for the analog body, whose every variable is a `real` — an
    /// `integer` included, because that ABI carries one in an `f64`. A digital
    /// process declares each of its locals, and the merge the builder creates
    /// for one then carries the declared type instead of guessing.
    variable_types: HashMap<CfgVariable, CfgValueType>,
    /// Blocks a [`CfgTerminator::Wait`] resumes into.
    ///
    /// Their parameters are exempt from trivial-merge collapse. A resumption
    /// does not preserve the value table — the process suspended, and whatever
    /// it computed before the suspension is gone — so a parameter that
    /// "trivially" equals a value defined before the `Wait` is not trivial at
    /// all: collapsing it makes the resumed half read something nothing
    /// defines. This is the one place the analog and digital construction
    /// rules genuinely differ, and it is inert for the analog body because
    /// nothing there suspends.
    resume_targets: HashSet<BlockId>,
    /// Where a collapsed merge sends its readers, as a union-find parent list.
    ///
    /// Recorded rather than applied. Rewriting every use at the moment a merge
    /// collapses is a scan of the whole value table per collapse, and a model
    /// with thousands of merges spends nearly all of construction doing it —
    /// measured at 27 of 28 seconds on `DIODE_CMC`. Reads resolve through this,
    /// and one pass at the end fixes the operands that were recorded before the
    /// redirection existed.
    redirect: Vec<Option<ValueId>>,
}

impl Default for SsaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SsaBuilder {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            values: Vec::new(),
            definitions: HashMap::new(),
            incomplete: Vec::new(),
            sealed: HashSet::new(),
            predecessors: HashMap::new(),
            variable_types: HashMap::new(),
            resume_targets: HashSet::new(),
            outputs: Vec::new(),
            redirect: Vec::new(),
        }
    }

    /// Declare the type a variable's values carry.
    ///
    /// Every merge the builder creates for `variable` takes this type, and so
    /// does the constant a merge with no reaching definition collapses to. A
    /// variable that is never declared is a `real`, which is what the analog
    /// body's every variable is and why that half needs no declarations.
    pub fn declare_variable(&mut self, variable: CfgVariable, value_type: CfgValueType) {
        self.variable_types.insert(variable, value_type);
    }

    fn variable_type(&self, variable: CfgVariable) -> CfgValueType {
        self.variable_types
            .get(&variable)
            .copied()
            .unwrap_or(CfgValueType::Real)
    }

    /// The value a variable of this type has where nothing defined it.
    ///
    /// Zero for a `real`, because that is the Verilog-AMS initial value of an
    /// analog variable. `x` for a four-state value, because IEEE 1364-2005
    /// section 4.2.2 gives an unwritten variable exactly that — the two
    /// languages disagree about what "no value yet" means, and the type says
    /// which of them is being asked.
    fn undefined_value(&mut self, value_type: CfgValueType) -> ValueId {
        let kind = match value_type {
            CfgValueType::FourState { width } => {
                CfgValueKind::FourStateConstant(digital_value::FourStateValue::splat(
                    width,
                    crate::four_state::FourStateBit::Unknown,
                ))
            }
            CfgValueType::Integer => CfgValueKind::IntegerConstant(0),
            CfgValueType::Boolean => CfgValueKind::BooleanConstant(false),
            CfgValueType::Real | CfgValueType::Lanes(_) | CfgValueType::Effect => {
                CfgValueKind::RealConstant(0.0)
            }
        };
        let value_type = match kind {
            CfgValueKind::RealConstant(_) => CfgValueType::Real,
            _ => value_type,
        };
        self.push_leaf(value_type, kind)
    }

    /// Follow a value through any merges that collapsed since it was recorded.
    fn resolve(&self, value: ValueId) -> ValueId {
        let mut current = value;
        // Bounded rather than recursive: a cycle would be a bug, and settling is
        // a better way to report one than a stack overflow.
        for _ in 0..self.redirect.len() {
            match self.redirect.get(usize::from(current)).copied().flatten() {
                Some(next) if next != current => current = next,
                _ => break,
            }
        }
        current
    }

    pub fn create_block(&mut self) -> BlockId {
        let id = BlockId::from(self.blocks.len());
        self.blocks.push(CfgBlock {
            id,
            params: Vec::new(),
            instructions: Vec::new(),
            terminator: CfgTerminator::Unset,
        });
        id
    }

    /// Append an instruction defining a fresh value.
    pub fn push(
        &mut self,
        block: BlockId,
        value_type: CfgValueType,
        kind: CfgValueKind,
    ) -> ValueId {
        let id = self.new_value(value_type, kind);
        self.blocks[usize::from(block)]
            .instructions
            .push(CfgInstruction { result: id });
        id
    }

    /// The type of a value defined so far.
    ///
    /// A lowering that computes a result width from its operands' widths needs
    /// to read them back while the function is still under construction.
    pub fn value_type_of(&self, value: ValueId) -> Option<CfgValueType> {
        self.values
            .get(usize::from(value))
            .map(|value| value.value_type)
    }

    /// Define a value without placing it in a block.
    ///
    /// For leaves — constants, parameters, node potentials — which every block
    /// may reference and which no block owns.
    pub fn push_leaf(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        debug_assert!(is_leaf(&kind), "non-leaf value pushed outside a block");
        self.new_value(value_type, kind)
    }

    fn new_value(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.values.len());
        self.values.push(CfgValue {
            id,
            value_type,
            kind,
        });
        self.redirect.push(None);
        id
    }

    pub fn set_terminator(&mut self, block: BlockId, terminator: CfgTerminator) {
        let successors = match &terminator {
            CfgTerminator::Jump { target, .. } => vec![*target],
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => vec![*then_target, *else_target],
            CfgTerminator::Wait { resume, .. } => vec![*resume],
            CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
        };
        if let CfgTerminator::Wait { resume, .. } = &terminator {
            self.resume_targets.insert(*resume);
        }
        for successor in successors {
            self.predecessors.entry(successor).or_default().push(block);
        }
        self.blocks[usize::from(block)].terminator = terminator;
    }

    /// Route one value across an edge that cannot carry the value table.
    ///
    /// The mechanism a [`CfgTerminator::Wait`] needs and an ordinary jump does
    /// not: a suspension ends the run, so a value the resumed half reads has to
    /// arrive as a block parameter rather than be looked up. Adds the argument
    /// and the parameter together, which is what keeps the two lists aligned,
    /// and returns the parameter the resumed block reads instead.
    ///
    /// Call it immediately after the terminator that creates the edge and
    /// before anything reads a variable in `to`; a parameter added by the
    /// ordinary merge machinery in between would take its argument at sealing
    /// time and the positions would cross.
    pub fn carry_value(&mut self, value: ValueId, from: BlockId, to: BlockId) -> ValueId {
        debug_assert_eq!(
            self.blocks[usize::from(from)]
                .arguments_to(to)
                .map(<[ValueId]>::len),
            Some(self.blocks[usize::from(to)].params.len()),
            "carry_value must run before any other parameter is added to the target"
        );
        let value_type = self.values[usize::from(value)].value_type;
        let parameter = self.new_value(value_type, CfgValueKind::BlockParameter);
        self.blocks[usize::from(to)].params.push(parameter);
        self.blocks[usize::from(from)].push_argument(to, value);
        parameter
    }

    /// Route a variable's current value across such an edge.
    ///
    /// The variable reads as the new parameter in `to`, so everything after the
    /// suspension sees the value the process had when it suspended.
    pub fn carry_variable(
        &mut self,
        variable: CfgVariable,
        from: BlockId,
        to: BlockId,
    ) -> Option<ValueId> {
        let value = self.read_variable(variable, from)?;
        let parameter = self.carry_value(value, from, to);
        self.write_variable(variable, to, parameter);
        Some(parameter)
    }

    pub fn write_variable(&mut self, variable: CfgVariable, block: BlockId, value: ValueId) {
        self.definitions.insert((variable, block), value);
    }

    /// The value of `variable` on entry to `block`, creating merges as needed.
    pub fn read_variable(&mut self, variable: CfgVariable, block: BlockId) -> Option<ValueId> {
        if let Some(value) = self.definitions.get(&(variable, block)) {
            // Resolved on the way out rather than rewritten in place: the
            // definition table is large and a collapse would otherwise have to
            // sweep it.
            return Some(self.resolve(*value));
        }
        self.read_variable_recursive(variable, block)
    }

    fn read_variable_recursive(
        &mut self,
        variable: CfgVariable,
        block: BlockId,
    ) -> Option<ValueId> {
        if !self.sealed.contains(&block) {
            // Predecessors are still arriving. Reserve a parameter now and fill
            // its arguments in on sealing; this is the only reason construction
            // needs two passes over a loop.
            let value_type = self.variable_type(variable);
            let value = self.new_value(value_type, CfgValueKind::BlockParameter);
            self.blocks[usize::from(block)].params.push(value);
            self.incomplete.push((block, variable, value));
            self.write_variable(variable, block, value);
            return Some(value);
        }

        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        let value = match preds.as_slice() {
            // Unreachable or the entry block: the variable has no definition
            // on this path, which the caller reports as a use before assignment.
            [] => return None,
            // One predecessor needs no merge — this is what keeps the form
            // minimal without a separate cleanup pass.
            [single] => self.read_variable(variable, *single)?,
            _ => {
                let value_type = self.variable_type(variable);
                let value = self.new_value(value_type, CfgValueKind::BlockParameter);
                self.blocks[usize::from(block)].params.push(value);
                // Written before recursing so a cycle terminates here rather
                // than spinning through the loop body forever.
                self.write_variable(variable, block, value);
                self.add_parameter_arguments(variable, block, value);
                return self.try_remove_trivial_parameter(block, value);
            }
        };
        self.write_variable(variable, block, value);
        Some(value)
    }

    fn add_parameter_arguments(
        &mut self,
        variable: CfgVariable,
        block: BlockId,
        parameter: ValueId,
    ) {
        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        for pred in preds {
            // A predecessor with no definition contributes the parameter
            // itself, which keeps the edge well-formed while the rest of the
            // arguments arrive. If every predecessor does that, the merge has
            // no reaching definition at all, which is what
            // [`Self::try_remove_trivial_parameter`] then reports.
            let argument = self.read_variable(variable, pred).unwrap_or(parameter);
            self.blocks[usize::from(pred)].push_argument(block, argument);
        }
    }

    /// Drop a merge whose arguments are all the same value.
    ///
    /// Without this the form is correct but not minimal, and every redundant
    /// merge becomes a redundant derivative array downstream.
    ///
    /// The parameter may already have been handed out — a loop body reads its
    /// carried variable long before the header can be sealed — so removing it
    /// has to rewrite every use, not just the variable table. A reader that
    /// still names a removed parameter is a value defined nowhere, which
    /// [`CfgFunction::validate`] rejects but only after the fact.
    ///
    /// Returns `None` when *every* incoming argument is the parameter itself.
    /// A merge of nothing but itself is not a definition: the variable has no
    /// reaching definition on any path, and the caller reports a read before
    /// assignment. Handing the parameter back instead — which is what this did
    /// until a real model caught it — makes a block with two predecessors
    /// answer that question differently from one with a single predecessor,
    /// and the difference survives all the way to generated Rust that reads an
    /// uninitialised binding.
    ///
    /// The parameter is redirected to zero on the way out regardless, because
    /// by the time a loop header is sealed its body already holds the value and
    /// there is no longer anywhere to report it to.
    fn try_remove_trivial_parameter(
        &mut self,
        block: BlockId,
        parameter: ValueId,
    ) -> Option<ValueId> {
        // A resume block's parameters are its entire live-in state: the value
        // table does not survive the suspension that reaches it, so there is no
        // "the argument is already available here" to collapse into.
        if self.resume_targets.contains(&block) {
            return Some(parameter);
        }
        let mut unique: Option<ValueId> = None;
        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        let position = match self.blocks[usize::from(block)]
            .params
            .iter()
            .position(|param| *param == parameter)
        {
            Some(position) => position,
            None => return Some(parameter),
        };

        // An edge that supplies no argument at this position is either still
        // under construction or malformed. Either way this pass must not touch
        // the parameter: mid-construction the fixed point at `finish` will
        // revisit it, and a genuinely malformed edge is
        // [`CfgFunction::validate`]'s to report — collapsing it here would
        // quietly repair the graph instead.
        let mut incomplete_edge = false;
        for pred in &preds {
            let Some(args) = self.blocks[usize::from(*pred)].arguments_to(block) else {
                incomplete_edge = true;
                continue;
            };
            let Some(argument) = args.get(position).copied() else {
                incomplete_edge = true;
                continue;
            };
            // Through the redirections, because an argument recorded earlier may
            // name a merge that has since collapsed.
            let argument = self.resolve(argument);
            // Self-references are what make a loop-carried merge non-trivial
            // only when something else also flows in; ignoring them here is
            // what lets a loop variable that never changes collapse.
            if argument == parameter {
                continue;
            }
            match unique {
                Some(existing) if existing != argument => return Some(parameter),
                _ => unique = Some(argument),
            }
        }

        if incomplete_edge {
            return Some(parameter);
        }

        // Removed either way: trivial when something unique flows in, undefined
        // when nothing does. Both leave the parameter with no role.
        self.blocks[usize::from(block)].params.remove(position);
        for pred in &preds {
            let block_index = usize::from(*pred);
            match &mut self.blocks[block_index].terminator {
                CfgTerminator::Jump { target, args } if *target == block => {
                    if position < args.len() {
                        args.remove(position);
                    }
                }
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => {
                    if *then_target == block && position < then_args.len() {
                        then_args.remove(position);
                    }
                    if *else_target == block && position < else_args.len() {
                        else_args.remove(position);
                    }
                }
                _ => {}
            }
        }
        // Zero is the Verilog-AMS initial value of an analog variable, so an
        // undefined merge resolves to it for any reader that already holds the
        // parameter. The `None` is what tells the lowering to say so. The
        // parameter's own type chooses the constant, so a four-state merge
        // resolves to `x` rather than to a real zero nothing in a process can
        // read.
        let replacement = match unique {
            Some(value) => value,
            None => {
                let value_type = self.values[usize::from(parameter)].value_type;
                self.undefined_value(value_type)
            }
        };
        self.replace_all_uses(parameter, replacement);
        unique.map(|_| replacement)
    }

    /// Record that every reader of `from` means `to`.
    ///
    /// One entry, not a sweep. [`Self::apply_redirections`] settles the graph
    /// once construction is finished.
    fn replace_all_uses(&mut self, from: ValueId, to: ValueId) {
        if from == to {
            return;
        }
        self.redirect[usize::from(from)] = Some(to);
    }

    /// Rewrite every recorded operand through the redirections.
    fn apply_redirections(&mut self) {
        if self.redirect.iter().all(Option::is_none) {
            return;
        }
        let resolved: Vec<ValueId> = (0..self.values.len())
            .map(|index| self.resolve(ValueId::from(index)))
            .collect();
        let translate = |id: ValueId| resolved[usize::from(id)];

        for value in &mut self.values {
            value.kind.map_operands(translate);
        }
        for block in &mut self.blocks {
            match &mut block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    *condition = translate(*condition);
                    for arg in then_args.iter_mut().chain(else_args.iter_mut()) {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    wait.map_operands(translate);
                    for arg in resume_args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        for output in &mut self.outputs {
            *output = translate(*output);
        }
    }

    /// Collapse every merge that became trivial as a side effect of collapsing
    /// another, until none are left.
    ///
    /// Removing one parameter can make a second one's arguments agree, and that
    /// second one is not otherwise revisited — the incremental construction only
    /// examines a parameter when it is created or sealed.
    fn remove_trivial_parameters(&mut self) {
        loop {
            let mut removed = false;
            for block in 0..self.blocks.len() {
                let block = BlockId::from(block);
                let params = self.blocks[usize::from(block)].params.clone();
                for parameter in params {
                    if self.try_remove_trivial_parameter(block, parameter) != Some(parameter) {
                        removed = true;
                    }
                }
            }
            if !removed {
                return;
            }
        }
    }

    /// Declare that `block` will gain no further predecessors.
    pub fn seal_block(&mut self, block: BlockId) {
        if !self.sealed.insert(block) {
            return;
        }
        let pending: Vec<(BlockId, CfgVariable, ValueId)> = self
            .incomplete
            .iter()
            .filter(|(pending_block, _, _)| *pending_block == block)
            .copied()
            .collect();
        self.incomplete
            .retain(|(pending_block, _, _)| *pending_block != block);
        for (_, variable, parameter) in pending {
            self.add_parameter_arguments(variable, block, parameter);
            // Nothing to report to: the body that reads this variable was built
            // long before the header could be sealed, so an undefined result
            // has already been redirected to zero for it.
            let _ = self.try_remove_trivial_parameter(block, parameter);
        }
    }

    /// Seal every block that is still open.
    ///
    /// The backstop a lowering calls once it has finished creating edges. A
    /// construct that refused partway through leaves its blocks behind, and an
    /// unsealed block reaching [`Self::finish`] holds parameters whose
    /// arguments were never supplied.
    pub fn seal_all_blocks(&mut self) {
        for index in 0..self.blocks.len() {
            self.seal_block(BlockId::from(index));
        }
    }

    pub fn is_sealed(&self, block: BlockId) -> bool {
        self.sealed.contains(&block)
    }

    /// Finish construction.
    ///
    /// Every block must be sealed: an unsealed block has parameters whose
    /// arguments were never supplied, and shipping one would produce a function
    /// that validates today and reads uninitialised state later.
    ///
    /// Values are compacted on the way out. Collapsing a trivial merge leaves
    /// its parameter defined nowhere, and a dense value table that downstream
    /// passes can index without checking for holes is worth the one renumbering
    /// pass it costs.
    pub fn finish(self, entry: BlockId) -> Result<CfgFunction, CfgValidationError> {
        self.finish_with_outputs(entry, &[])
            .map(|(function, _)| function)
    }

    /// Finish, translating `outputs` into the ids they end up with.
    ///
    /// Anything the caller intends to read out of the finished function has to
    /// go through here. Ids taken before this point do not survive it: trivial
    /// merges collapse into their arguments, and the value table is renumbered.
    pub fn finish_with_outputs(
        mut self,
        entry: BlockId,
        outputs: &[ValueId],
    ) -> Result<(CfgFunction, Vec<ValueId>), CfgValidationError> {
        debug_assert!(
            self.incomplete.is_empty(),
            "finish() with {} unsealed block parameters",
            self.incomplete.len()
        );
        self.outputs = outputs.to_vec();
        self.remove_trivial_parameters();
        self.apply_redirections();
        self.compact();
        let outputs = std::mem::take(&mut self.outputs);
        let function = CfgFunction {
            entry,
            blocks: self.blocks,
            values: self.values,
            shapes: Vec::new(),
        };
        function.validate()?;
        Ok((function, outputs))
    }

    /// Drop values nothing defines or reads, and renumber what remains.
    fn compact(&mut self) {
        let mut live = vec![false; self.values.len()];

        // Anything a block defines is live, and so is anything a live value or
        // a terminator reads. Leaves are only live if something reads them.
        let mut worklist: Vec<ValueId> = Vec::new();
        let mark = |id: ValueId, live: &mut Vec<bool>, worklist: &mut Vec<ValueId>| {
            if !live[usize::from(id)] {
                live[usize::from(id)] = true;
                worklist.push(id);
            }
        };

        // An output may be a bare leaf — a residual is the zero constant when
        // nothing contributed — so it is seeded explicitly rather than relying
        // on some block naming it.
        for output in &self.outputs {
            mark(*output, &mut live, &mut worklist);
        }
        for block in &self.blocks {
            for param in &block.params {
                mark(*param, &mut live, &mut worklist);
            }
            for instruction in &block.instructions {
                mark(instruction.result, &mut live, &mut worklist);
            }
            match &block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    mark(*condition, &mut live, &mut worklist);
                    for arg in then_args.iter().chain(else_args) {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    for operand in wait.operands() {
                        mark(operand, &mut live, &mut worklist);
                    }
                    for arg in resume_args {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        while let Some(id) = worklist.pop() {
            for operand in self.values[usize::from(id)].kind.operands() {
                mark(operand, &mut live, &mut worklist);
            }
        }

        let mut remap = vec![None; self.values.len()];
        let mut compacted = Vec::with_capacity(self.values.len());
        for (index, value) in self.values.iter().enumerate() {
            if !live[index] {
                continue;
            }
            let id = ValueId::from(compacted.len());
            remap[index] = Some(id);
            compacted.push(CfgValue {
                id,
                value_type: value.value_type,
                kind: value.kind.clone(),
            });
        }
        let translate = |id: ValueId| -> ValueId {
            remap[usize::from(id)].expect("live value must survive compaction")
        };

        for value in &mut compacted {
            value.kind.map_operands(translate);
        }
        for output in &mut self.outputs {
            *output = translate(*output);
        }
        for block in &mut self.blocks {
            for param in &mut block.params {
                *param = translate(*param);
            }
            for instruction in &mut block.instructions {
                instruction.result = translate(instruction.result);
            }
            match &mut block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    *condition = translate(*condition);
                    for arg in then_args.iter_mut().chain(else_args.iter_mut()) {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    wait.map_operands(translate);
                    for arg in resume_args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        self.values = compacted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variable(index: usize) -> CfgVariable {
        CfgVariable::Local(VariableId::from(index))
    }

    fn constant(builder: &mut SsaBuilder, value: f64) -> ValueId {
        builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(value))
    }

    /// `x = 1; x = x + 1;` in one block needs no merges at all.
    #[test]
    fn straight_line_code_creates_no_block_parameters() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);

        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, one);
        let read = builder.read_variable(variable(0), entry).expect("defined");
        let sum = builder.push(
            entry,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: read,
                right: one,
            },
        );
        builder.write_variable(variable(0), entry, sum);
        builder.set_terminator(entry, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert!(function.blocks.iter().all(|block| block.params.is_empty()));
    }

    /// A diamond where both arms assign needs exactly one merge — which is the
    /// whole point: one parameter, not one select per downstream value.
    #[test]
    fn a_diamond_merges_with_one_block_parameter() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(then_block);
        builder.seal_block(else_block);

        let two = constant(&mut builder, 2.0);
        let three = constant(&mut builder, 3.0);
        builder.write_variable(variable(0), then_block, two);
        builder.write_variable(variable(0), else_block, three);
        builder.set_terminator(
            then_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.set_terminator(
            else_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let merged = builder.read_variable(variable(0), join).expect("merged");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(function.block(join).params, vec![merged]);
        assert_eq!(
            function.value(merged).kind,
            CfgValueKind::BlockParameter,
            "a merge is a block parameter, not an instruction"
        );
        // Both arms must supply exactly one argument.
        for arm in [then_block, else_block] {
            let CfgTerminator::Jump { args, .. } = &function.block(arm).terminator else {
                panic!("arm must jump to the join");
            };
            assert_eq!(args.len(), 1);
        }
    }

    /// Both arms assigning the *same* value must not create a merge; a
    /// redundant parameter becomes a redundant derivative array downstream.
    #[test]
    fn a_diamond_assigning_one_value_needs_no_parameter() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let shared = constant(&mut builder, 7.0);
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(false));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(then_block);
        builder.seal_block(else_block);
        builder.write_variable(variable(0), then_block, shared);
        builder.write_variable(variable(0), else_block, shared);
        builder.set_terminator(
            then_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.set_terminator(
            else_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let merged = builder.read_variable(variable(0), join).expect("defined");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(merged, shared, "identical arms must collapse to the value");
        assert!(function.block(join).params.is_empty());
    }

    /// A value assigned only before the branch flows through untouched.
    #[test]
    fn a_value_defined_before_a_branch_needs_no_merge() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let body = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let five = constant(&mut builder, 5.0);
        builder.write_variable(variable(0), entry, five);
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: join,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let read = builder.read_variable(variable(0), join).expect("defined");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(read, five);
        assert!(function.block(join).params.is_empty());
    }

    /// The case sealing exists for: a loop header is reachable from its own
    /// body, so its merge cannot be built when the header is first visited.
    #[test]
    fn a_loop_header_merges_its_back_edge() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let header = builder.create_block();
        let body = builder.create_block();
        let exit = builder.create_block();
        builder.seal_block(entry);

        let zero = constant(&mut builder, 0.0);
        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, zero);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        // Header is deliberately left unsealed: the back edge does not exist.
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);

        let carried = builder.read_variable(variable(0), header).expect("defined");
        let next = builder.push(
            body,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: carried,
                right: one,
            },
        );
        builder.write_variable(variable(0), body, next);
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        // The back edge now exists, so the header can be sealed and its
        // reserved parameter filled in.
        builder.seal_block(header);
        builder.seal_block(exit);
        builder.set_terminator(exit, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(
            function.block(header).params,
            vec![carried],
            "the loop-carried variable must be a header parameter"
        );

        let CfgTerminator::Jump { args, .. } = &function.block(entry).terminator else {
            panic!("entry jumps to the header");
        };
        assert_eq!(args, &vec![zero], "the initial value enters on the edge in");

        let CfgTerminator::Jump { args, .. } = &function.block(body).terminator else {
            panic!("body jumps back to the header");
        };
        assert_eq!(
            args,
            &vec![next],
            "the updated value returns on the back edge"
        );
    }

    /// A loop-carried variable the body never writes: the header parameter is
    /// handed to an instruction *before* the back edge exists, and then turns
    /// out to be trivial. Removing it has to rewrite that instruction, or the
    /// function names a value nothing defines.
    #[test]
    fn collapsing_a_header_parameter_rewrites_the_uses_that_already_read_it() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let header = builder.create_block();
        let body = builder.create_block();
        let exit = builder.create_block();
        builder.seal_block(entry);

        let seven = constant(&mut builder, 7.0);
        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, seven);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);

        // Read inside the body while the header is unsealed: this reserves a
        // parameter and immediately consumes it.
        let carried = builder.read_variable(variable(0), header).expect("defined");
        let doubled = builder.push(
            body,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: carried,
                right: one,
            },
        );
        // Deliberately not written back: the variable is loop-invariant, so the
        // parameter it reserved is trivial once the back edge arrives.
        let _ = doubled;
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        builder.seal_block(header);
        builder.seal_block(exit);
        builder.set_terminator(exit, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert!(
            function.block(header).params.is_empty(),
            "a loop-invariant variable needs no header parameter"
        );
        let addition = function
            .values
            .iter()
            .find(|value| matches!(value.kind, CfgValueKind::Binary { .. }))
            .expect("the addition survives");
        let CfgValueKind::Binary { left, .. } = addition.kind else {
            unreachable!("just matched")
        };
        assert_eq!(
            function.value(left).kind,
            CfgValueKind::RealConstant(7.0),
            "the use must have been rewritten to the value that flowed in"
        );
    }

    #[test]
    fn reading_an_unassigned_variable_reports_rather_than_inventing() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        builder.set_terminator(entry, CfgTerminator::Return);
        assert_eq!(builder.read_variable(variable(3), entry), None);
    }

    #[test]
    fn an_unterminated_block_fails_validation() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        assert_eq!(
            builder.finish(entry),
            Err(CfgValidationError::UnterminatedBlock(BlockId::from(0usize)))
        );
    }

    #[test]
    fn an_argument_count_mismatch_fails_validation() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let target = builder.create_block();
        builder.seal_block(entry);
        builder.seal_block(target);

        let stray = builder.new_value(CfgValueType::Real, CfgValueKind::BlockParameter);
        builder.blocks[usize::from(target)].params.push(stray);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target,
                args: Vec::new(),
            },
        );
        builder.set_terminator(target, CfgTerminator::Return);

        assert_eq!(
            builder.finish(entry),
            Err(CfgValidationError::ArgumentCountMismatch {
                from: BlockId::from(0usize),
                to: BlockId::from(1usize),
                expected: 1,
                found: 0,
            })
        );
    }

    #[test]
    fn predecessors_are_reported_from_terminators() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let target = builder.create_block();
        builder.seal_block(entry);
        builder.seal_block(target);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target,
                args: Vec::new(),
            },
        );
        builder.set_terminator(target, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(function.predecessors(target), vec![entry]);
        assert!(function.predecessors(entry).is_empty());
    }
}
