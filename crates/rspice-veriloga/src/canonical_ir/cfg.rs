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

use super::{
    BlockId, BranchId, BranchUnknownId, ContributionId, ExprId, NodeId, ParamId, ShapeId, ValueId,
    VariableId,
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
    Residual(ContributionId),
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
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Atan,
    Asinh,
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
    LaneWiden { input: ValueId },
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
    LaneExtract { input: ValueId, lane: u32 },

    /// A value a coarser invalidation stage computed and cached.
    ///
    /// What one slice of a split body reads from another. It is a leaf here on
    /// purpose: within a stage the value has no derivation, which is exactly
    /// what makes the stage a function of its own inputs and cacheable
    /// independently.
    Staged { slot: u32 },
}

impl CfgValueKind {
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
            Self::Limit {
                proposed,
                candidate,
                ..
            } => vec![*proposed, *candidate],
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
            Self::Limit {
                proposed,
                candidate,
                ..
            } => {
                *proposed = map(*proposed);
                *candidate = map(*candidate);
            }
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
}

impl CfgValueType {
    pub fn shape(self) -> Option<ShapeId> {
        match self {
            Self::Lanes(shape) => Some(shape),
            Self::Real | Self::Boolean => None,
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
    /// Placeholder while a block is under construction. Never present in a
    /// finished function; [`CfgFunction::validate`] rejects it.
    Unset,
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
        self.value(value).value_type.shape().map(|id| self.shape(id))
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
            outputs: Vec::new(),
            redirect: Vec::new(),
        }
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
            CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
        };
        for successor in successors {
            self.predecessors.entry(successor).or_default().push(block);
        }
        self.blocks[usize::from(block)].terminator = terminator;
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
            let value = self.new_value(CfgValueType::Real, CfgValueKind::BlockParameter);
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
                let value = self.new_value(CfgValueType::Real, CfgValueKind::BlockParameter);
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
        // parameter. The `None` is what tells the lowering to say so.
        let replacement = unique
            .unwrap_or_else(|| self.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0)));
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
