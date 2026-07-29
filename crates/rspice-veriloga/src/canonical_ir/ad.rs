//! Forward-mode differentiation of a [`CfgFunction`].
//!
//! Every unknown the solver owns — each node potential, each branch unknown —
//! gets a *lane*. Differentiating a value produces one more SSA value holding
//! its partial with respect to every lane it can reach. The result is one
//! function computing both the residual and its Jacobian row, which is what a
//! hand-written model does and what the level being replaced could not do
//! without folding control flow away first.
//!
//! ## Sparsity is structural, and is the point
//!
//! A lane belongs to a value only if some chain of operations connects that
//! value to that unknown. [`lane_liveness`] computes this as a fixed point over
//! the graph, so a wide model — ASM-HEMT has 23 nodes — does not pay 23 lanes on
//! every intermediate. Measured over the corpus, values that carry any lane at
//! all carry between two and five of them against thirty-odd unknowns; dense
//! forward mode on those models is why the current backend's largest outputs are
//! the size they are.
//!
//! ## One value per derivative, not one per lane
//!
//! The lanes of a derivative are held together, in a value whose *shape* is that
//! reachable set. Both halves of that matter and they pull against each other:
//!
//! - Packed, because scalarising costs one emitted line per lane. The rules are
//!   elementwise, so a whole derivative is one instruction whatever its width,
//!   and the emitted source stops scaling with the lane count.
//! - Shaped rather than a uniform per-model width, because the corpus's mean
//!   live width is a small fraction of its unknown count. Laying every
//!   derivative out over all the unknowns would be simpler and would multiply
//!   the arithmetic by that ratio — six-fold on HiSIM-HV.
//!
//! An operand narrower than its consumer is re-laid-out by a
//! [`CfgValueKind::LaneWiden`], which is the only place a shape changes.
//!
//! ## Structural zeros are dropped, not computed
//!
//! `d(a*b) = da*b + a*db`, but where `b` provably cannot reach an unknown the
//! second term is not a value that happens to be zero — it does not exist. The
//! rules below emit only the terms whose operands carry the lane, so a product
//! with a temperature-dependent coefficient costs one elementwise scale rather
//! than two and an add. This is also why it would be wrong to recover the same
//! saving downstream with `x * 0 -> 0`: that identity is false for NaN, whereas
//! the term's absence here is a fact about the graph.
//!
//! ## Merges differentiate to merges
//!
//! A block parameter is a merge of primals; its derivative is a merge of
//! derivatives, so the block simply gains more parameters and each predecessor
//! passes one more argument. A predecessor whose incoming value carries fewer
//! lanes widens; one that carries none passes a zero. No dominance reasoning is
//! needed and no rule is special-cased for loops: a back edge is an edge.
//!
//! ## `min` and `max`
//!
//! Their derivative follows whichever operand won, which is a run-time choice.
//! Rather than split the block — introducing control flow inside a pass that
//! should not be rewriting the graph's shape — it is written arithmetically as
//! `db + c*(da - db)` with `c` the comparison the primal already implies.

use std::collections::{HashMap, HashSet};

use super::cfg::{
    CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp,
    CfgValidationError, CfgValue, CfgValueKind, CfgValueType,
};
use super::{BlockId, BranchUnknownId, NodeId, ShapeId, ValueId};

/// An unknown the Jacobian is taken with respect to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdSeed {
    NodePotential(NodeId),
    BranchUnknownFlow(BranchUnknownId),
    /// Not an unknown at all, and it must not reach the matrix.
    ///
    /// It is the directional affine correction stateful Newton limiting
    /// introduces: the displacement `$limit` moved the operating point by,
    /// carried through the same chain rule as the real lanes so that a residual
    /// built from a limited value knows how much of itself is limiter and not
    /// physics. The `Limit` arm of the derivative rules says why the convention
    /// needs it.
    ///
    /// Nothing seeds it. Every `$limit` *injects* into it, which is why it is
    /// the one lane whose liveness does not follow from the value graph.
    LimiterCorrection,
}

/// A function extended with the derivatives of its values.
#[derive(Debug, Clone, PartialEq)]
pub struct AdFunction {
    pub function: CfgFunction,
    /// The unknowns, in lane order.
    pub lanes: Vec<AdSeed>,
    /// The packed derivative of each value, `None` where nothing it computes
    /// can reach any unknown. Indexed by the *original* value ids, which the
    /// pass preserves.
    derivatives: Vec<Option<ValueId>>,
    /// Lanes already read out, so asking twice returns the same value rather
    /// than a second identical instruction.
    extracted: HashMap<(ValueId, u32), ValueId>,
    /// Where a read-out lands. See [`AdFunction::derivative`].
    return_block: BlockId,
}

impl AdFunction {
    /// The packed derivative of `value`.
    pub fn packed(&self, value: ValueId) -> Option<ValueId> {
        *self.derivatives.get(usize::from(value))?
    }

    /// Every lane of `value`'s derivative, in lane order.
    ///
    /// The form worth using: a caller wants a Jacobian row, and asking for it
    /// in one call means the function stops growing before anything reads it.
    pub fn derivative_row(&mut self, value: ValueId) -> Vec<Option<ValueId>> {
        (0..self.lanes.len())
            .map(|lane| self.derivative(value, lane))
            .collect()
    }

    /// One lane of `value`'s derivative, as a scalar.
    ///
    /// The read-out is appended to the block that returns, which is where a
    /// caller wanting a Jacobian entry — the only caller there is — needs it.
    /// A lane the value does not carry is structurally zero and reports `None`
    /// rather than a zero constant, so a caller can tell "no dependence" from
    /// "zero at this bias".
    ///
    /// Appending means this *changes the function*, so take every lane wanted
    /// before evaluating or emitting it. [`Self::derivative_row`] is the shape
    /// that makes that hard to get wrong.
    pub fn derivative(&mut self, value: ValueId, lane: usize) -> Option<ValueId> {
        let array = self.packed(value)?;
        let lane = u32::try_from(lane).ok()?;
        self.function.lane_position(array, lane)?;
        if let Some(existing) = self.extracted.get(&(array, lane)) {
            return Some(*existing);
        }
        let id = ValueId::from(self.function.values.len());
        self.function.values.push(CfgValue {
            id,
            value_type: CfgValueType::Real,
            kind: CfgValueKind::LaneExtract { input: array, lane },
        });
        self.function.blocks[usize::from(self.return_block)]
            .instructions
            .push(CfgInstruction { result: id });
        self.extracted.insert((array, lane), id);
        Some(id)
    }
}

/// Which lanes each value can depend on.
///
/// A fixed point rather than one pass, because a loop-carried block parameter
/// depends on values computed from itself: the first visit of a header sees
/// only the entry edge.
pub fn lane_liveness(function: &CfgFunction, lanes: &[AdSeed]) -> Vec<HashSet<usize>> {
    let seeds = seed_lanes(function, lanes);
    let mut live: Vec<HashSet<usize>> = vec![HashSet::new(); function.values.len()];
    for (value, lane) in &seeds {
        live[usize::from(*value)].insert(*lane);
    }

    // Every `$limit` carries the correction lane whether or not what it limits
    // depends on an unknown: the displacement it introduces is a fact about the
    // iterate, not about the model's dependence on the solution.
    let correction = correction_lane(lanes);
    if let Some(correction) = correction {
        for value in &function.values {
            if matches!(value.kind, CfgValueKind::Limit { .. }) {
                live[usize::from(value.id)].insert(correction);
            }
        }
    }

    // Which edge arguments feed which block parameter, computed once.
    let incoming = incoming_arguments(function);

    loop {
        let mut changed = false;
        for value in &function.values {
            let index = usize::from(value.id);
            let incoming_lanes: HashSet<usize> = match &value.kind {
                CfgValueKind::BlockParameter => match incoming.get(&value.id) {
                    Some(arguments) => arguments
                        .iter()
                        .flat_map(|argument| live[usize::from(*argument)].iter().copied())
                        .collect(),
                    None => HashSet::new(),
                },
                // Only `proposed`, not the limiter body: the rule chains through
                // the value that was offered, so a lane only the body can reach
                // would be a structural zero taking up a slot.
                CfgValueKind::Limit { proposed, .. } => {
                    live[usize::from(*proposed)].iter().copied().collect()
                }
                kind if differentiable(kind) => kind
                    .operands()
                    .into_iter()
                    .flat_map(|operand| live[usize::from(operand)].iter().copied())
                    .collect(),
                _ => HashSet::new(),
            };
            for lane in incoming_lanes {
                if live[index].insert(lane) {
                    changed = true;
                }
            }
        }
        if !changed {
            return live;
        }
    }
}

/// Whether a value's derivative can be nonzero given nonzero operands.
///
/// Predicates are not: a comparison is piecewise constant, and treating it
/// otherwise would propagate lanes into every guard in the model.
fn differentiable(kind: &CfgValueKind) -> bool {
    match kind {
        CfgValueKind::Unary { op, .. } => !matches!(
            op,
            CfgUnaryOp::Not
                | CfgUnaryOp::Floor
                | CfgUnaryOp::Ceil
                | CfgUnaryOp::LimitedExpDerivative
        ),
        CfgValueKind::Binary { op, .. } => !is_predicate(*op),
        // `$limit` is differentiable, but [`lane_liveness`] answers it ahead of
        // this rather than through it: its lanes are `proposed`'s plus the
        // correction lane, not every operand's.
        CfgValueKind::Ddt { .. } | CfgValueKind::Idt { .. } | CfgValueKind::Limit { .. } => true,
        // The previous iterate is a constant as far as this iteration's Newton
        // step is concerned; that is what makes limiting a damping and not a
        // change of equations.
        CfgValueKind::LimitPrevious { .. } => false,
        // `ddx` is a first-order readback. Differentiating through it would
        // mean carrying second derivatives everywhere a model reports a
        // transconductance, for a term no solver reads.
        CfgValueKind::Ddx { .. } => false,
        _ => false,
    }
}

fn is_predicate(op: CfgBinaryOp) -> bool {
    matches!(
        op,
        CfgBinaryOp::Eq
            | CfgBinaryOp::Ne
            | CfgBinaryOp::Lt
            | CfgBinaryOp::Le
            | CfgBinaryOp::Gt
            | CfgBinaryOp::Ge
            | CfgBinaryOp::And
            | CfgBinaryOp::Or
    )
}

/// Which lane, if any, the caller reserved for the limiter correction.
fn correction_lane(lanes: &[AdSeed]) -> Option<usize> {
    lanes.iter().position(|seed| *seed == AdSeed::LimiterCorrection)
}

fn seed_lanes(function: &CfgFunction, lanes: &[AdSeed]) -> Vec<(ValueId, usize)> {
    let index_of: HashMap<AdSeed, usize> = lanes
        .iter()
        .enumerate()
        .map(|(index, seed)| (*seed, index))
        .collect();
    function
        .values
        .iter()
        .filter_map(|value| {
            let seed = match value.kind {
                CfgValueKind::NodePotential(node) => AdSeed::NodePotential(node),
                CfgValueKind::BranchUnknownFlow(unknown) => AdSeed::BranchUnknownFlow(unknown),
                _ => return None,
            };
            index_of.get(&seed).map(|lane| (value.id, *lane))
        })
        .collect()
}

/// Every argument passed to each block parameter, over all incoming edges.
fn incoming_arguments(function: &CfgFunction) -> HashMap<ValueId, Vec<ValueId>> {
    let mut incoming: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
    for block in &function.blocks {
        for (target, args) in outgoing(block) {
            for (param, argument) in function.block(target).params.iter().zip(args) {
                incoming.entry(*param).or_default().push(argument);
            }
        }
    }
    incoming
}

fn outgoing(block: &CfgBlock) -> Vec<(BlockId, Vec<ValueId>)> {
    match &block.terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args.clone())],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![
            (*then_target, then_args.clone()),
            (*else_target, else_args.clone()),
        ],
        CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
    }
}

/// Differentiate `function` with respect to `lanes`.
pub fn differentiate(
    function: &CfgFunction,
    lanes: &[AdSeed],
) -> Result<AdFunction, CfgValidationError> {
    let live = lane_liveness(function, lanes);
    let mut builder = AdBuilder::new(function, lanes, &live);
    builder.add_block_parameters();
    builder.rewrite_blocks();

    let differentiated = CfgFunction {
        entry: function.entry,
        blocks: builder.blocks,
        values: builder.values,
        shapes: builder.shapes,
    };
    differentiated.validate()?;
    let return_block = differentiated
        .blocks
        .iter()
        .find(|block| matches!(block.terminator, CfgTerminator::Return))
        .map_or(differentiated.entry, |block| block.id);

    Ok(AdFunction {
        function: differentiated,
        lanes: lanes.to_vec(),
        derivatives: builder.derivatives,
        extracted: HashMap::new(),
        return_block,
    })
}

struct AdBuilder<'a> {
    source: &'a CfgFunction,
    lanes: Vec<AdSeed>,
    /// The shape each source value's derivative must end up with, from
    /// liveness. `None` where the value reaches no unknown at all.
    target: Vec<Option<ShapeId>>,
    values: Vec<CfgValue>,
    blocks: Vec<CfgBlock>,
    derivatives: Vec<Option<ValueId>>,
    shapes: Vec<Vec<u32>>,
    interned: HashMap<Vec<u32>, ShapeId>,
    /// Uniform packed constants, shared per (value, shape).
    splats: HashMap<(u64, ShapeId), ValueId>,
    /// Scalar constants the rules reach for, shared so a corpus-scale model
    /// does not carry tens of thousands of copies of `-1.0` into the optimiser.
    constants: HashMap<u64, ValueId>,
    one: ValueId,
    /// The shared `d/dt` coefficient, created on first use so a purely
    /// resistive model carries no reference to it.
    ddt_scale: Option<ValueId>,
    idt_scale: Option<ValueId>,
    /// The lane reserved for [`AdSeed::LimiterCorrection`], if the caller asked
    /// for one. Without it a `$limit` differentiates to its proposed value's
    /// row and the displacement is dropped, which is what a consumer that does
    /// not apply the correction should get.
    correction_lane: Option<u32>,
    /// Derivative parameters each block gained: the position of the primal
    /// parameter they follow, and the shape they merge into.
    added_params: HashMap<BlockId, Vec<(usize, ShapeId)>>,
    /// Instructions accumulated for the block being rewritten.
    emitted: Vec<CfgInstruction>,
}

impl<'a> AdBuilder<'a> {
    fn new(source: &'a CfgFunction, lanes: &[AdSeed], live: &[HashSet<usize>]) -> Self {
        let mut values = source.values.clone();
        let one = ValueId::from(values.len());
        values.push(CfgValue {
            id: one,
            value_type: CfgValueType::Real,
            kind: CfgValueKind::RealConstant(1.0),
        });

        let mut builder = Self {
            source,
            lanes: lanes.to_vec(),
            target: vec![None; source.values.len()],
            derivatives: vec![None; values.len()],
            values,
            blocks: source.blocks.clone(),
            shapes: Vec::new(),
            interned: HashMap::new(),
            splats: HashMap::new(),
            constants: HashMap::from([(1.0f64.to_bits(), one)]),
            one,
            ddt_scale: None,
            idt_scale: None,
            correction_lane: correction_lane(lanes)
                .map(|lane| u32::try_from(lane).expect("lane count fits a u32")),
            added_params: HashMap::new(),
            emitted: Vec::new(),
        };

        for (index, set) in live.iter().enumerate() {
            if set.is_empty() {
                continue;
            }
            let mut lanes: Vec<u32> = set
                .iter()
                .map(|lane| u32::try_from(*lane).expect("lane count fits a u32"))
                .collect();
            lanes.sort_unstable();
            builder.target[index] = Some(builder.intern(lanes));
        }

        // The seeds. A node potential's derivative is one in its own lane, and
        // liveness gives it exactly that lane, so the packed form is a one-wide
        // value holding 1.0.
        for (value, _) in seed_lanes(source, lanes) {
            let shape = builder.target[usize::from(value)].expect("a seed carries its own lane");
            let seed = builder.splat(1.0, shape);
            builder.derivatives[usize::from(value)] = Some(seed);
        }

        builder
    }

    // ---- shapes and constants ---------------------------------------------

    fn intern(&mut self, lanes: Vec<u32>) -> ShapeId {
        if let Some(existing) = self.interned.get(&lanes) {
            return *existing;
        }
        let id = ShapeId::from(self.shapes.len());
        self.interned.insert(lanes.clone(), id);
        self.shapes.push(lanes);
        id
    }

    fn shape_of(&self, value: ValueId) -> Option<ShapeId> {
        self.values[usize::from(value)].value_type.shape()
    }

    fn splat(&mut self, constant: f64, shape: ShapeId) -> ValueId {
        let key = (constant.to_bits(), shape);
        if let Some(existing) = self.splats.get(&key) {
            return *existing;
        }
        let value = self.new_value(
            CfgValueType::Lanes(shape),
            CfgValueKind::LaneSplat(constant),
        );
        self.splats.insert(key, value);
        value
    }

    fn ddt_scale(&mut self) -> ValueId {
        match self.ddt_scale {
            Some(value) => value,
            None => {
                let value = self.new_value(CfgValueType::Real, CfgValueKind::DdtScale);
                self.ddt_scale = Some(value);
                value
            }
        }
    }

    fn idt_scale(&mut self) -> ValueId {
        match self.idt_scale {
            Some(value) => value,
            None => {
                let value = self.new_value(CfgValueType::Real, CfgValueKind::IdtScale);
                self.idt_scale = Some(value);
                value
            }
        }
    }

    // ---- packed builders ---------------------------------------------------

    /// `value` re-laid-out over `target`, or itself when it already is.
    fn widen(&mut self, value: ValueId, target: ShapeId) -> ValueId {
        if self.shape_of(value) == Some(target) {
            return value;
        }
        self.push(
            CfgValueType::Lanes(target),
            CfgValueKind::LaneWiden { input: value },
        )
    }

    /// Elementwise `op` of two packed values, widening both to `target`.
    fn lane_binary(
        &mut self,
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
        target: ShapeId,
    ) -> ValueId {
        let left = self.widen(left, target);
        let right = self.widen(right, target);
        self.push(
            CfgValueType::Lanes(target),
            CfgValueKind::LaneBinary { op, left, right },
        )
    }

    /// Every lane of `input` combined with one scalar.
    fn lane_scalar(&mut self, op: CfgBinaryOp, input: ValueId, scalar: ValueId) -> ValueId {
        let shape = self
            .shape_of(input)
            .expect("a packed operand carries a shape");
        self.push(
            CfgValueType::Lanes(shape),
            CfgValueKind::LaneScalar { op, input, scalar },
        )
    }

    fn scale(&mut self, input: ValueId, scalar: ValueId) -> ValueId {
        self.lane_scalar(CfgBinaryOp::Mul, input, scalar)
    }

    fn negate(&mut self, input: ValueId) -> ValueId {
        let minus_one = self.constant(-1.0);
        self.scale(input, minus_one)
    }

    // ---- the walk ----------------------------------------------------------

    /// Give every block one derivative parameter per merged value that carries
    /// lanes, so an edge knows how many arguments it owes before any terminator
    /// is rewritten.
    fn add_block_parameters(&mut self) {
        for block in &self.source.blocks.clone() {
            let mut added = Vec::new();
            for (position, param) in block.params.iter().enumerate() {
                let Some(shape) = self.target[usize::from(*param)] else {
                    continue;
                };
                let derivative = self.new_value(
                    CfgValueType::Lanes(shape),
                    CfgValueKind::BlockParameter,
                );
                self.blocks[usize::from(block.id)].params.push(derivative);
                self.derivatives[usize::from(*param)] = Some(derivative);
                added.push((position, shape));
            }
            self.added_params.insert(block.id, added);
        }
    }

    fn rewrite_blocks(&mut self) {
        // Reverse postorder, so every operand's derivative exists before a rule
        // names it. Values crossing a back edge do not need the ordering: they
        // arrive as block parameters, which were allocated up front.
        for block in reverse_postorder(self.source) {
            self.emitted = Vec::with_capacity(self.source.block(block).instructions.len());
            for instruction in &self.source.block(block).instructions.clone() {
                let result = instruction.result;
                // Read-outs have to land before the instruction that reads them.
                if let CfgValueKind::Ddx {
                    value,
                    pos_node,
                    neg_node,
                } = self.source.value(result).kind
                {
                    self.resolve_ddx(result, value, pos_node, neg_node);
                }
                self.emitted.push(CfgInstruction { result });
                let derivative = self.rule(result);
                self.derivatives[usize::from(result)] = derivative;
            }
            let terminator = self.rewrite_terminator(block);
            self.blocks[usize::from(block)].instructions = std::mem::take(&mut self.emitted);
            self.blocks[usize::from(block)].terminator = terminator;
        }
    }

    /// Turn a `ddx` into the lane it names.
    ///
    /// `ddx(f, V(p, n))` is `∂f/∂p − ∂f/∂n`, which this pass has just computed.
    /// Reading it back rather than lowering it separately is what lets a model
    /// asking for its own transconductance get the entry the backend already
    /// produces instead of a second, separately maintained one.
    ///
    /// A lane the value does not carry is a structural zero and folds away here.
    fn resolve_ddx(
        &mut self,
        result: ValueId,
        value: ValueId,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    ) {
        let positive = self.extract_node_lane(value, pos_node);
        let negative = self.extract_node_lane(value, neg_node);
        let kind = match (positive, negative) {
            (Some(positive), Some(negative)) => CfgValueKind::Binary {
                op: CfgBinaryOp::Sub,
                left: positive,
                right: negative,
            },
            // The instruction has to define something, so the identity is
            // written out rather than aliased; simplification collapses it.
            (Some(positive), None) => CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: self.one,
                right: positive,
            },
            (None, Some(negative)) => CfgValueKind::Unary {
                op: CfgUnaryOp::Neg,
                input: negative,
            },
            (None, None) => CfgValueKind::RealConstant(0.0),
        };
        self.values[usize::from(result)].kind = kind;
    }

    /// The scalar partial of `value` with respect to a node's potential.
    fn extract_node_lane(&mut self, value: ValueId, node: Option<NodeId>) -> Option<ValueId> {
        let node = node?;
        let lane = self
            .lanes
            .iter()
            .position(|seed| *seed == AdSeed::NodePotential(node))?;
        let lane = u32::try_from(lane).ok()?;
        let array = self.derivatives[usize::from(value)]?;
        let shape = self.shape_of(array)?;
        if !self.shapes[usize::from(shape)].contains(&lane) {
            return None;
        }
        Some(self.push(
            CfgValueType::Real,
            CfgValueKind::LaneExtract { input: array, lane },
        ))
    }

    /// Append the derivative arguments this block owes each successor.
    fn rewrite_terminator(&mut self, block: BlockId) -> CfgTerminator {
        match self.source.block(block).terminator.clone() {
            CfgTerminator::Jump { target, mut args } => {
                let derivative_args = self.derivative_arguments(target, &args);
                args.extend(derivative_args);
                CfgTerminator::Jump { target, args }
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                mut then_args,
                else_target,
                mut else_args,
            } => {
                let then_derivatives = self.derivative_arguments(then_target, &then_args);
                let else_derivatives = self.derivative_arguments(else_target, &else_args);
                then_args.extend(then_derivatives);
                else_args.extend(else_derivatives);
                CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                }
            }
            other => other,
        }
    }

    /// The derivative of each argument, in the order the target added its
    /// derivative parameters.
    ///
    /// A predecessor whose incoming value carries fewer lanes than the merge
    /// widens; one that carries none passes a zero. That is what makes "this arm
    /// cannot depend on that unknown" a correct statement rather than a missing
    /// edge.
    fn derivative_arguments(&mut self, target: BlockId, args: &[ValueId]) -> Vec<ValueId> {
        let Some(added) = self.added_params.get(&target).cloned() else {
            return Vec::new();
        };
        let mut arguments: Vec<ValueId> = Vec::with_capacity(added.len());
        for (position, shape) in added {
            let incoming = args
                .get(position)
                .and_then(|argument| self.derivatives[usize::from(*argument)]);
            arguments.push(match incoming {
                Some(derivative) => self.widen(derivative, shape),
                None => self.splat(0.0, shape),
            });
        }
        arguments
    }

    // ---- the rules ---------------------------------------------------------

    /// The packed derivative of `result`, laid out over the shape liveness
    /// proved it needs.
    fn rule(&mut self, result: ValueId) -> Option<ValueId> {
        let target = self.target[usize::from(result)]?;
        let kind = self.source.value(result).kind.clone();
        let natural = self.natural_rule(&kind, target);
        // A shape wider than the rule produced means an operand the derivative
        // does not flow through carried lanes anyway — `$limit`'s proposed
        // value, `%`'s divisor. Those lanes are genuinely zero here.
        let natural = natural.unwrap_or_else(|| self.splat(0.0, target));
        Some(self.widen(natural, target))
    }

    fn natural_rule(&mut self, kind: &CfgValueKind, target: ShapeId) -> Option<ValueId> {
        match kind {
            CfgValueKind::Unary { op, input } => {
                let derivative = self.derivatives[usize::from(*input)]?;
                let factor = self.unary_factor(*op, *input);
                Some(self.scale(derivative, factor))
            }
            CfgValueKind::Binary { op, left, right } => {
                self.binary_rule(*op, *left, *right, target)
            }
            // Not another `ddt`: a second one would claim a second state slot
            // for a quantity with no history of its own. The companion form's
            // coefficient multiplies the input's derivative instead.
            CfgValueKind::Ddt { input, .. } => {
                let derivative = self.derivatives[usize::from(*input)]?;
                let scale = self.ddt_scale();
                Some(self.scale(derivative, scale))
            }
            // The same companion-form argument, integrated rather than
            // differentiated: the running total contributes `dt` times this
            // step's input, so that is what the unknowns see. The initial
            // condition is not differentiated — it is where the integral starts,
            // not something the solve moves.
            CfgValueKind::Idt { input, .. } => {
                let derivative = self.derivatives[usize::from(*input)]?;
                let scale = self.idt_scale();
                Some(self.scale(derivative, scale))
            }
            // The classic SPICE convention pinned by the compiler contract: what
            // `$limit` returned takes the *proposed* value's slope. `dL/dv := 1`
            // rather than the limiter body's own derivative.
            //
            // Chaining through the body reads better and is wrong for a solver.
            // A saturating limiter has slope zero wherever its clamp is active,
            // so the device would linearise as disconnected exactly on the
            // iterations that needed help, and the equations Newton sees would
            // change with the iterate. Limiting is damping applied to a step,
            // not a change to the equations being solved.
            //
            // The price of that pretence is here: the returned value really is
            // `proposed + (candidate - proposed)`, and the second term is a
            // displacement the residual has to be corrected by. It goes into its
            // own lane rather than into the matrix, because it is not a partial
            // with respect to any unknown — the stamp subtracts it once, at the
            // linearisation point, and only while limiting is enabled.
            CfgValueKind::Limit {
                proposed,
                candidate,
                ..
            } => {
                let base = self.derivatives[usize::from(*proposed)];
                let Some(lane) = self.correction_lane else {
                    return base;
                };
                let displacement = self.push_binary(CfgBinaryOp::Sub, *candidate, *proposed);
                let shape = self.intern(vec![lane]);
                // One-lane packed value holding the displacement. `1.0 *` is
                // exact and this is the only way to lift a scalar into a lane
                // without a kind that exists for one construct.
                let unit = self.splat(1.0, shape);
                let injected = self.scale(unit, displacement);
                Some(match base {
                    // Nested limiters accumulate: an outer displacement adds to
                    // whatever an inner one already put in the lane, which is
                    // what makes the correction the total offset rather than the
                    // last one applied.
                    Some(base) => self.lane_binary(CfgBinaryOp::Add, base, injected, target),
                    None => injected,
                })
            }
            _ => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn binary_rule(
        &mut self,
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
        target: ShapeId,
    ) -> Option<ValueId> {
        let d_left = self.derivatives[usize::from(left)];
        let d_right = self.derivatives[usize::from(right)];
        match op {
            CfgBinaryOp::Add => match (d_left, d_right) {
                (Some(d_left), Some(d_right)) => {
                    Some(self.lane_binary(CfgBinaryOp::Add, d_left, d_right, target))
                }
                (Some(only), None) | (None, Some(only)) => Some(only),
                (None, None) => None,
            },
            CfgBinaryOp::Sub => match (d_left, d_right) {
                (Some(d_left), Some(d_right)) => {
                    Some(self.lane_binary(CfgBinaryOp::Sub, d_left, d_right, target))
                }
                (Some(d_left), None) => Some(d_left),
                (None, Some(d_right)) => Some(self.negate(d_right)),
                (None, None) => None,
            },
            CfgBinaryOp::Mul => match (d_left, d_right) {
                (Some(d_left), Some(d_right)) => {
                    let first = self.scale(d_left, right);
                    let second = self.scale(d_right, left);
                    Some(self.lane_binary(CfgBinaryOp::Add, first, second, target))
                }
                (Some(d_left), None) => Some(self.scale(d_left, right)),
                (None, Some(d_right)) => Some(self.scale(d_right, left)),
                (None, None) => None,
            },
            // (da - (a/b) db) / b, which is the quotient rule with one division
            // rather than two and no b squared to overflow.
            CfgBinaryOp::Div => {
                let numerator = match (d_left, d_right) {
                    (Some(d_left), Some(d_right)) => {
                        let quotient =
                            self.push_binary(CfgBinaryOp::Div, left, right);
                        let scaled = self.scale(d_right, quotient);
                        self.lane_binary(CfgBinaryOp::Sub, d_left, scaled, target)
                    }
                    (Some(d_left), None) => d_left,
                    (None, Some(d_right)) => {
                        let quotient =
                            self.push_binary(CfgBinaryOp::Div, left, right);
                        let scaled = self.scale(d_right, quotient);
                        self.negate(scaled)
                    }
                    (None, None) => return None,
                };
                Some(self.lane_scalar(CfgBinaryOp::Div, numerator, right))
            }
            // d(a^b) = b*a^(b-1)*da + a^b*ln(a)*db. Written as two terms so a
            // constant exponent — which is almost all of them — costs one power
            // and no logarithm.
            CfgBinaryOp::Pow => {
                let from_base = d_left.map(|d_left| {
                    let one = self.one;
                    let exponent = self.push_binary(CfgBinaryOp::Sub, right, one);
                    let reduced = self.push_binary(CfgBinaryOp::Pow, left, exponent);
                    let scaled = self.push_binary(CfgBinaryOp::Mul, right, reduced);
                    self.scale(d_left, scaled)
                });
                let from_exponent = d_right.map(|d_right| {
                    let power = self.push_binary(CfgBinaryOp::Pow, left, right);
                    let logarithm = self.push_unary(CfgUnaryOp::Ln, left);
                    let scaled = self.push_binary(CfgBinaryOp::Mul, power, logarithm);
                    self.scale(d_right, scaled)
                });
                match (from_base, from_exponent) {
                    (Some(from_base), Some(from_exponent)) => Some(self.lane_binary(
                        CfgBinaryOp::Add,
                        from_base,
                        from_exponent,
                        target,
                    )),
                    (Some(only), None) | (None, Some(only)) => Some(only),
                    (None, None) => None,
                }
            }
            // `a % b` moves with `a` between the discontinuities.
            CfgBinaryOp::Mod => d_left,
            CfgBinaryOp::Min | CfgBinaryOp::Max => {
                let comparison = if matches!(op, CfgBinaryOp::Min) {
                    CfgBinaryOp::Le
                } else {
                    CfgBinaryOp::Ge
                };
                let takes_left =
                    self.push_typed(CfgValueType::Boolean, comparison, left, right);
                match (d_left, d_right) {
                    (Some(d_left), Some(d_right)) => {
                        let difference =
                            self.lane_binary(CfgBinaryOp::Sub, d_left, d_right, target);
                        let selected = self.scale(difference, takes_left);
                        Some(self.lane_binary(CfgBinaryOp::Add, d_right, selected, target))
                    }
                    (Some(d_left), None) => Some(self.scale(d_left, takes_left)),
                    // db + c*(0 - db) is db*(1 - c).
                    (None, Some(d_right)) => {
                        let one = self.one;
                        let takes_right =
                            self.push_binary(CfgBinaryOp::Sub, one, takes_left);
                        Some(self.scale(d_right, takes_right))
                    }
                    (None, None) => None,
                }
            }
            other => {
                debug_assert!(is_predicate(other), "unhandled binary derivative rule");
                None
            }
        }
    }

    /// `d(f(x)) = f'(x) * dx`; this returns `f'(x)`.
    fn unary_factor(&mut self, op: CfgUnaryOp, input: ValueId) -> ValueId {
        match op {
            CfgUnaryOp::Neg => self.constant(-1.0),
            CfgUnaryOp::Exp => self.push_unary(CfgUnaryOp::Exp, input),
            // Beyond the clamp `limexp` is affine, so its slope is the value at
            // the clamp — which is what `limexp` returns there.
            CfgUnaryOp::LimExp => self.push_unary(CfgUnaryOp::LimExp, input),
            CfgUnaryOp::LimitedExp => self.push_unary(CfgUnaryOp::LimitedExpDerivative, input),
            // Its own derivative is a step, so second derivatives are zero.
            CfgUnaryOp::LimitedExpDerivative => self.constant(0.0),
            CfgUnaryOp::Ln => {
                let one = self.one;
                self.push_binary(CfgBinaryOp::Div, one, input)
            }
            CfgUnaryOp::Sqrt => {
                let root = self.push_unary(CfgUnaryOp::Sqrt, input);
                let two = self.constant(2.0);
                let denominator = self.push_binary(CfgBinaryOp::Mul, two, root);
                let one = self.one;
                self.push_binary(CfgBinaryOp::Div, one, denominator)
            }
            // sign(x), written so the derivative is +1 or -1 rather than a
            // branch: 2*(x >= 0) - 1.
            CfgUnaryOp::Abs => {
                let zero = self.constant(0.0);
                let positive =
                    self.push_typed(CfgValueType::Boolean, CfgBinaryOp::Ge, input, zero);
                let two = self.constant(2.0);
                let doubled = self.push_binary(CfgBinaryOp::Mul, two, positive);
                let one = self.one;
                self.push_binary(CfgBinaryOp::Sub, doubled, one)
            }
            CfgUnaryOp::Sin => self.push_unary(CfgUnaryOp::Cos, input),
            CfgUnaryOp::Cos => {
                let sine = self.push_unary(CfgUnaryOp::Sin, input);
                let minus_one = self.constant(-1.0);
                self.push_binary(CfgBinaryOp::Mul, minus_one, sine)
            }
            CfgUnaryOp::Tan => {
                let cosine = self.push_unary(CfgUnaryOp::Cos, input);
                let squared = self.push_binary(CfgBinaryOp::Mul, cosine, cosine);
                let one = self.one;
                self.push_binary(CfgBinaryOp::Div, one, squared)
            }
            CfgUnaryOp::Sinh => self.push_unary(CfgUnaryOp::Cosh, input),
            CfgUnaryOp::Cosh => self.push_unary(CfgUnaryOp::Sinh, input),
            CfgUnaryOp::Tanh => {
                let tangent = self.push_unary(CfgUnaryOp::Tanh, input);
                let squared = self.push_binary(CfgBinaryOp::Mul, tangent, tangent);
                let one = self.one;
                self.push_binary(CfgBinaryOp::Sub, one, squared)
            }
            CfgUnaryOp::Atan => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let one = self.one;
                let denominator = self.push_binary(CfgBinaryOp::Add, one, squared);
                self.push_binary(CfgBinaryOp::Div, one, denominator)
            }
            CfgUnaryOp::Asinh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let one = self.one;
                let sum = self.push_binary(CfgBinaryOp::Add, one, squared);
                let root = self.push_unary(CfgUnaryOp::Sqrt, sum);
                self.push_binary(CfgBinaryOp::Div, one, root)
            }
            // Piecewise constant, so zero away from the steps. `differentiable`
            // keeps these out, and this arm exists so adding a unary op cannot
            // silently take the wrong branch.
            CfgUnaryOp::Not | CfgUnaryOp::Floor | CfgUnaryOp::Ceil => self.constant(0.0),
        }
    }

    // ---- value construction ------------------------------------------------

    fn constant(&mut self, value: f64) -> ValueId {
        if let Some(existing) = self.constants.get(&value.to_bits()) {
            return *existing;
        }
        let id = self.new_value(CfgValueType::Real, CfgValueKind::RealConstant(value));
        self.constants.insert(value.to_bits(), id);
        id
    }

    fn new_value(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.values.len());
        self.values.push(CfgValue {
            id,
            value_type,
            kind,
        });
        self.derivatives.push(None);
        id
    }

    fn push(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        let id = self.new_value(value_type, kind);
        self.emitted.push(CfgInstruction { result: id });
        id
    }

    fn push_binary(&mut self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        self.push_typed(CfgValueType::Real, op, left, right)
    }

    fn push_typed(
        &mut self,
        value_type: CfgValueType,
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> ValueId {
        self.push(value_type, CfgValueKind::Binary { op, left, right })
    }

    fn push_unary(&mut self, op: CfgUnaryOp, input: ValueId) -> ValueId {
        self.push(CfgValueType::Real, CfgValueKind::Unary { op, input })
    }
}

/// Blocks in reverse postorder, which is an order in which every definition
/// precedes the uses that are not carried by a block parameter.
fn reverse_postorder(function: &CfgFunction) -> Vec<BlockId> {
    let mut visited = vec![false; function.blocks.len()];
    let mut postorder = Vec::with_capacity(function.blocks.len());
    let mut stack = vec![(function.entry, 0usize)];
    visited[usize::from(function.entry)] = true;
    while let Some((block, index)) = stack.pop() {
        let successors = function.block(block).successors();
        if index < successors.len() {
            stack.push((block, index + 1));
            let successor = successors[index];
            if !visited[usize::from(successor)] {
                visited[usize::from(successor)] = true;
                stack.push((successor, 0));
            }
        } else {
            postorder.push(block);
        }
    }
    postorder.reverse();
    // An unreachable block still has to keep its instructions, and it has no
    // derivative to compute because nothing reachable reads what it defines.
    postorder.extend(
        function
            .blocks
            .iter()
            .map(|block| block.id)
            .filter(|block| !visited[usize::from(*block)]),
    );
    postorder
}
