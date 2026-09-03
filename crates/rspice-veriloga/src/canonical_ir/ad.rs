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

use crate::metrics::{NoPipelineControl, PipelineCancelled, PipelineControl, PipelinePhase};

use super::cfg::{
    CfgBinaryOp, CfgBlock, CfgDdxAxis, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp,
    CfgValidationError, CfgValue, CfgValueKind, CfgValueType,
};
use super::{BlockId, BranchUnknownId, NodeId, ShapeId, ValueId};

/// An unknown the Jacobian is taken with respect to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdSeed {
    NodePotential(NodeId),
    BranchUnknownFlow(BranchUnknownId),
    /// One independent syntactic noise process. Labels intentionally do not
    /// participate in this identity.
    NoiseProcess(u32),
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
    lane_liveness_with_control(function, lanes, &NoPipelineControl)
        .expect("the no-op pipeline control cannot cancel")
        .into_hash_sets()
}

/// Compact storage for the sparse lane sets used by the liveness fixed point.
///
/// The old representation allocated one `HashSet` per value and another
/// temporary set for every value on every fixed-point sweep. Compact models
/// routinely have hundreds of thousands of values but only a few dozen
/// derivative lanes, so a contiguous bit matrix is both materially smaller and
/// turns propagation into straight-line word unions. Pathological lane counts
/// fall back to sparse sets rather than being allowed to allocate an unbounded
/// dense matrix from untrusted model input.
struct LaneLiveness {
    value_count: usize,
    lane_count: usize,
    words_per_value: usize,
    storage: LaneLivenessStorage,
}

enum LaneLivenessStorage {
    Dense(Vec<u64>),
    Sparse(Vec<HashSet<usize>>),
}

impl LaneLiveness {
    const MAX_DENSE_BYTES: usize = 64 * 1024 * 1024;

    fn new(value_count: usize, lane_count: usize) -> Self {
        let words_per_value = lane_count.div_ceil(u64::BITS as usize);
        let dense_words = value_count.checked_mul(words_per_value);
        let dense_bytes = dense_words.and_then(|words| words.checked_mul(size_of::<u64>()));
        let storage = match (dense_words, dense_bytes) {
            (Some(words), Some(bytes)) if bytes <= Self::MAX_DENSE_BYTES => {
                LaneLivenessStorage::Dense(vec![0; words])
            }
            _ => LaneLivenessStorage::Sparse(vec![HashSet::new(); value_count]),
        };
        Self {
            value_count,
            lane_count,
            words_per_value,
            storage,
        }
    }

    fn range(&self, value: ValueId) -> std::ops::Range<usize> {
        let start = usize::from(value) * self.words_per_value;
        start..start + self.words_per_value
    }

    fn insert(&mut self, value: ValueId, lane: usize) -> bool {
        debug_assert!(lane < self.lane_count);
        let value = usize::from(value);
        match &mut self.storage {
            LaneLivenessStorage::Dense(bits) => {
                let index = value * self.words_per_value + lane / u64::BITS as usize;
                let bit = 1_u64 << (lane % u64::BITS as usize);
                let changed = bits[index] & bit == 0;
                bits[index] |= bit;
                changed
            }
            LaneLivenessStorage::Sparse(sets) => sets[value].insert(lane),
        }
    }

    fn union_from(&mut self, target: ValueId, source: ValueId) -> bool {
        let target_index = usize::from(target);
        let source_index = usize::from(source);
        if target_index == source_index {
            return false;
        }
        match &mut self.storage {
            LaneLivenessStorage::Dense(bits) => {
                let target_start = target_index * self.words_per_value;
                let source_start = source_index * self.words_per_value;
                let mut changed = false;
                for offset in 0..self.words_per_value {
                    let previous = bits[target_start + offset];
                    let merged = previous | bits[source_start + offset];
                    bits[target_start + offset] = merged;
                    changed |= merged != previous;
                }
                changed
            }
            LaneLivenessStorage::Sparse(sets) => {
                let (target, source) = if target_index < source_index {
                    let (before_source, from_source) = sets.split_at_mut(source_index);
                    (&mut before_source[target_index], &from_source[0])
                } else {
                    let (before_target, from_target) = sets.split_at_mut(target_index);
                    (&mut from_target[0], &before_target[source_index])
                };
                let previous_len = target.len();
                target.extend(source.iter().copied());
                target.len() != previous_len
            }
        }
    }

    fn is_empty(&self, value: ValueId) -> bool {
        match &self.storage {
            LaneLivenessStorage::Dense(bits) => {
                bits[self.range(value)].iter().all(|word| *word == 0)
            }
            LaneLivenessStorage::Sparse(sets) => sets[usize::from(value)].is_empty(),
        }
    }

    fn contains(&self, value: ValueId, lane: usize) -> bool {
        match &self.storage {
            LaneLivenessStorage::Dense(bits) => {
                let index = usize::from(value) * self.words_per_value + lane / u64::BITS as usize;
                bits[index] & (1_u64 << (lane % u64::BITS as usize)) != 0
            }
            LaneLivenessStorage::Sparse(sets) => sets[usize::from(value)].contains(&lane),
        }
    }

    fn lanes(&self, value: ValueId) -> Vec<usize> {
        match &self.storage {
            LaneLivenessStorage::Dense(bits) => {
                let mut lanes = Vec::new();
                for (word_offset, word) in bits[self.range(value)].iter().copied().enumerate() {
                    let mut word = word;
                    while word != 0 {
                        let bit = word.trailing_zeros() as usize;
                        let lane = word_offset * u64::BITS as usize + bit;
                        if lane < self.lane_count {
                            lanes.push(lane);
                        }
                        word &= word - 1;
                    }
                }
                lanes
            }
            LaneLivenessStorage::Sparse(sets) => {
                let mut lanes: Vec<usize> = sets[usize::from(value)].iter().copied().collect();
                lanes.sort_unstable();
                lanes
            }
        }
    }

    fn into_hash_sets(self) -> Vec<HashSet<usize>> {
        match self.storage {
            LaneLivenessStorage::Sparse(sets) => sets,
            LaneLivenessStorage::Dense(bits) => {
                let words_per_value = self.words_per_value;
                let lane_count = self.lane_count;
                (0..self.value_count)
                    .map(|value| {
                        let start = value * words_per_value;
                        let mut lanes = HashSet::new();
                        for (word_offset, word) in bits[start..start + words_per_value]
                            .iter()
                            .copied()
                            .enumerate()
                        {
                            let mut word = word;
                            while word != 0 {
                                let bit = word.trailing_zeros() as usize;
                                let lane = word_offset * u64::BITS as usize + bit;
                                if lane < lane_count {
                                    lanes.insert(lane);
                                }
                                word &= word - 1;
                            }
                        }
                        lanes
                    })
                    .collect()
            }
        }
    }
}

fn lane_liveness_with_control(
    function: &CfgFunction,
    lanes: &[AdSeed],
    control: &dyn PipelineControl,
) -> Result<LaneLiveness, PipelineCancelled> {
    check_cancelled(control)?;
    let seeds = seed_lanes(function, lanes);
    let mut live = LaneLiveness::new(function.values.len(), lanes.len());
    for (value, lane) in &seeds {
        live.insert(*value, *lane);
    }

    // Every `$limit` carries the correction lane whether or not what it limits
    // depends on an unknown: the displacement it introduces is a fact about the
    // iterate, not about the model's dependence on the solution.
    let correction = correction_lane(lanes);
    if let Some(correction) = correction {
        for (index, value) in function.values.iter().enumerate() {
            if index.is_multiple_of(1024) {
                check_cancelled(control)?;
            }
            if matches!(value.kind, CfgValueKind::Limit { .. }) {
                live.insert(value.id, correction);
            }
        }
    }

    // Which edge arguments feed which block parameter, computed once.
    let incoming = incoming_arguments(function);
    check_cancelled(control)?;

    loop {
        let mut changed = false;
        for (ordinal, value) in function.values.iter().enumerate() {
            if ordinal.is_multiple_of(1024) {
                check_cancelled(control)?;
            }
            match &value.kind {
                CfgValueKind::BlockParameter => {
                    if let Some(arguments) = incoming.get(&value.id) {
                        for argument in arguments {
                            changed |= live.union_from(value.id, *argument);
                        }
                    }
                }
                // Only `proposed`, not the limiter body: the rule chains through
                // the value that was offered, so a lane only the body can reach
                // would be a structural zero taking up a slot.
                CfgValueKind::Limit { proposed, .. } => {
                    changed |= live.union_from(value.id, *proposed);
                }
                kind if differentiable(kind) => match kind {
                    CfgValueKind::Unary { input, .. } | CfgValueKind::Ddt { input, .. } => {
                        changed |= live.union_from(value.id, *input);
                    }
                    CfgValueKind::Binary { left, right, .. } => {
                        changed |= live.union_from(value.id, *left);
                        changed |= live.union_from(value.id, *right);
                    }
                    CfgValueKind::Idt { input, ic, .. } => {
                        changed |= live.union_from(value.id, *input);
                        changed |= live.union_from(value.id, *ic);
                    }
                    // The modulus and the offset place the fold, which is a
                    // translation the derivative does not see, so they carry no
                    // lanes any more than a `%`'s divisor does.
                    CfgValueKind::IdtMod { input, ic, .. } => {
                        changed |= live.union_from(value.id, *input);
                        changed |= live.union_from(value.id, *ic);
                    }
                    // A delay that depends on an unknown moves the sample
                    // point, so it carries lanes.
                    CfgValueKind::AbsDelay { input, delay, .. } => {
                        changed |= live.union_from(value.id, *input);
                        changed |= live.union_from(value.id, *delay);
                    }
                    CfgValueKind::AbsDelayDerivative {
                        input_derivative,
                        delay_derivative,
                        ..
                    } => {
                        changed |= live.union_from(value.id, *input_derivative);
                        changed |= live.union_from(value.id, *delay_derivative);
                    }
                    // A rate that depends on an unknown moves the clamp, so the
                    // rates carry lanes for the reason the delay above does.
                    CfgValueKind::Slew {
                        input,
                        max_rise,
                        max_fall,
                        ..
                    } => {
                        changed |= live.union_from(value.id, *input);
                        changed |= live.union_from(value.id, *max_rise);
                        if let Some(max_fall) = max_fall {
                            changed |= live.union_from(value.id, *max_fall);
                        }
                    }
                    CfgValueKind::SlewDerivative {
                        input_derivative,
                        max_rise_derivative,
                        max_fall_derivative,
                        ..
                    } => {
                        changed |= live.union_from(value.id, *input_derivative);
                        changed |= live.union_from(value.id, *max_rise_derivative);
                        changed |= live.union_from(value.id, *max_fall_derivative);
                    }
                    // Only the input. The coefficients are constants of the
                    // solve, so a lane through one would be a lane through a
                    // number.
                    CfgValueKind::Laplace { input, .. } | CfgValueKind::Zi { input, .. } => {
                        changed |= live.union_from(value.id, *input);
                    }
                    CfgValueKind::LaplaceDerivative {
                        input_derivative, ..
                    }
                    | CfgValueKind::ZiDerivative {
                        input_derivative, ..
                    } => {
                        changed |= live.union_from(value.id, *input_derivative);
                    }
                    _ => unreachable!("every differentiable value kind is covered"),
                },
                _ => {}
            }
        }
        if !changed {
            return Ok(live);
        }
        check_cancelled(control)?;
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
        CfgValueKind::Ddt { .. }
        | CfgValueKind::Idt { .. }
        // The wrapped integral takes the unwrapped one's rule: the fold is a
        // translation by a whole number of periods, and a constant offset has
        // no derivative. It is discontinuous exactly at the wrap, which is a
        // measure-zero set the companion form does not linearise across.
        | CfgValueKind::IdtMod { .. }
        | CfgValueKind::AbsDelay { .. }
        | CfgValueKind::AbsDelayDerivative { .. }
        | CfgValueKind::Slew { .. }
        | CfgValueKind::SlewDerivative { .. }
        // A filter is linear in its input, so its action is the same filter
        // driven by the input's derivative. Its coefficients are not operands
        // of the solve either way: a Laplace coefficient is a compile-time
        // constant, and a `zi_*` coefficient is frozen per instance before the
        // first Newton pass.
        | CfgValueKind::Laplace { .. }
        | CfgValueKind::LaplaceDerivative { .. }
        | CfgValueKind::Zi { .. }
        | CfgValueKind::ZiDerivative { .. }
        | CfgValueKind::Limit { .. } => true,
        // The previous iterate is a constant as far as this iteration's Newton
        // step is concerned; that is what makes limiting a damping and not a
        // change of equations.
        CfgValueKind::LimitPrevious { .. } => false,
        // A crossing time is piecewise constant in the bias: between two
        // crossings it does not move at all, and at one it jumps. The JIT
        // answers `0.0` for its derivative and this is the same contract, held
        // where the enum can be checked against it.
        CfgValueKind::LastCrossing { .. } => false,
        // Bitwise and shift results are piecewise constant on the reals, so
        // they get a comparison's answer for a comparison's reason.
        CfgValueKind::IntegerBitwise { .. } | CfgValueKind::IntegerBitwiseNot { .. } => false,
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
    lanes
        .iter()
        .position(|seed| *seed == AdSeed::LimiterCorrection)
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
                CfgValueKind::NoiseProcess(process) => AdSeed::NoiseProcess(process),
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
        // Not reachable from an analog body, which never suspends, but the
        // edge is reported correctly rather than as "no successors" — a graph
        // utility that lies about its edges is worse than one that refuses.
        CfgTerminator::Wait {
            resume,
            resume_args,
            ..
        } => vec![(*resume, resume_args.clone())],
        CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
    }
}

/// Resolve every first-order `ddx` readback into ordinary scalar CFG before
/// the packed Jacobian pass runs.
///
/// This preliminary directional pass is what makes a solution-dependent
/// `ddx` safe in an equation. Once the readback is ordinary scalar arithmetic,
/// the packed pass differentiates it normally and therefore stamps the second
/// derivative instead of a plausible-looking zero. A `ddx` whose operand
/// already depends on another `ddx` would require a higher-order jet; reject
/// that bounded case rather than silently truncating it.
fn resolve_ddx_primal_with_control(
    function: &CfgFunction,
    second_order_roots: Option<&[ValueId]>,
    control: &dyn PipelineControl,
) -> Result<CfgFunction, DifferentiationError> {
    let second_order = match second_order_roots {
        Some(roots) => ancestors_of(function, roots),
        None => vec![true; function.values.len()],
    };
    let mut seeds = Vec::new();
    for value in &function.values {
        if !second_order[usize::from(value.id)] {
            continue;
        }
        let CfgValueKind::Ddx { axis, .. } = value.kind else {
            continue;
        };
        for seed in ddx_axis_seeds(axis) {
            if !seeds.contains(&seed) {
                seeds.push(seed);
            }
        }
    }
    if seeds.is_empty() {
        return Ok(function.clone());
    }

    reject_nested_ddx(function).map_err(DifferentiationError::Validation)?;
    let forward = lane_liveness_with_control(function, &seeds, control)
        .map_err(DifferentiationError::Cancelled)?;
    let requested = ddx_direction_liveness(function, &seeds, &second_order, control)
        .map_err(DifferentiationError::Cancelled)?;
    let mut active = LaneLiveness::new(function.values.len(), seeds.len());
    for value in &function.values {
        for lane in requested.lanes(value.id) {
            if forward.contains(value.id, lane) {
                active.insert(value.id, lane);
            }
        }
    }
    let mut builder = ScalarDdxBuilder::new(function, seeds, active, second_order);
    builder.add_block_parameters();
    builder
        .rewrite_blocks(control)
        .map_err(DifferentiationError::Cancelled)?;
    let resolved = CfgFunction {
        entry: function.entry,
        blocks: builder.blocks,
        values: builder.values,
        shapes: function.shapes.clone(),
    };
    resolved
        .validate()
        .map_err(DifferentiationError::Validation)?;
    Ok(resolved)
}

/// Values whose primals can reach one of `roots`. Merge inputs participate in
/// the walk, so a `ddx` selected by a conditional equation is still recognized
/// as stamp-relevant.
fn ancestors_of(function: &CfgFunction, roots: &[ValueId]) -> Vec<bool> {
    let incoming = incoming_arguments(function);
    let mut ancestors = vec![false; function.values.len()];
    for root in roots {
        if let Some(slot) = ancestors.get_mut(usize::from(*root)) {
            *slot = true;
        }
    }
    loop {
        let mut changed = false;
        for value in function.values.iter().rev() {
            if !ancestors[usize::from(value.id)] {
                continue;
            }
            let dependencies = match &value.kind {
                CfgValueKind::BlockParameter => {
                    incoming.get(&value.id).cloned().unwrap_or_default()
                }
                kind => kind.operands(),
            };
            for dependency in dependencies {
                let slot = &mut ancestors[usize::from(dependency)];
                if !*slot {
                    *slot = true;
                    changed = true;
                }
            }
        }
        if !changed {
            return ancestors;
        }
    }
}

/// Directional shadows are needed only on paths feeding a `ddx` operand. A
/// forward solution-dependence pass alone reaches every later value in a large
/// compact model and used to materialize model-wide shadows for a readback near
/// the end of one equation. This reverse pass keeps the higher-order work to
/// the actual operand slice and to the particular axes each readback names.
fn ddx_direction_liveness(
    function: &CfgFunction,
    seeds: &[AdSeed],
    second_order: &[bool],
    control: &dyn PipelineControl,
) -> Result<LaneLiveness, PipelineCancelled> {
    let seed_indices: HashMap<AdSeed, usize> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| (*seed, index))
        .collect();
    let incoming = incoming_arguments(function);
    let mut needed = LaneLiveness::new(function.values.len(), seeds.len());
    for value in &function.values {
        if !second_order[usize::from(value.id)] {
            continue;
        }
        if let CfgValueKind::Ddx {
            value: operand,
            axis,
        } = value.kind
        {
            for seed in ddx_axis_seeds(axis) {
                if let Some(lane) = seed_indices.get(&seed) {
                    needed.insert(operand, *lane);
                }
            }
        }
    }

    loop {
        let mut changed = false;
        for (ordinal, value) in function.values.iter().enumerate().rev() {
            if ordinal.is_multiple_of(1024) {
                check_cancelled(control)?;
            }
            match &value.kind {
                CfgValueKind::BlockParameter => {
                    if let Some(arguments) = incoming.get(&value.id) {
                        for argument in arguments {
                            changed |= needed.union_from(*argument, value.id);
                        }
                    }
                }
                CfgValueKind::Limit { proposed, .. } => {
                    changed |= needed.union_from(*proposed, value.id);
                }
                CfgValueKind::Unary { input, .. } if differentiable(&value.kind) => {
                    changed |= needed.union_from(*input, value.id);
                }
                CfgValueKind::Ddt { input, .. }
                | CfgValueKind::Idt { input, .. }
                | CfgValueKind::IdtMod { input, .. } => {
                    changed |= needed.union_from(*input, value.id);
                }
                CfgValueKind::AbsDelay { input, delay, .. } => {
                    changed |= needed.union_from(*input, value.id);
                    changed |= needed.union_from(*delay, value.id);
                }
                CfgValueKind::AbsDelayDerivative {
                    input_derivative,
                    delay_derivative,
                    ..
                } => {
                    changed |= needed.union_from(*input_derivative, value.id);
                    changed |= needed.union_from(*delay_derivative, value.id);
                }
                CfgValueKind::Slew {
                    input,
                    max_rise,
                    max_fall,
                    ..
                } => {
                    changed |= needed.union_from(*input, value.id);
                    changed |= needed.union_from(*max_rise, value.id);
                    if let Some(max_fall) = max_fall {
                        changed |= needed.union_from(*max_fall, value.id);
                    }
                }
                CfgValueKind::SlewDerivative {
                    input_derivative,
                    max_rise_derivative,
                    max_fall_derivative,
                    ..
                } => {
                    changed |= needed.union_from(*input_derivative, value.id);
                    changed |= needed.union_from(*max_rise_derivative, value.id);
                    changed |= needed.union_from(*max_fall_derivative, value.id);
                }
                CfgValueKind::Laplace { input, .. } | CfgValueKind::Zi { input, .. } => {
                    changed |= needed.union_from(*input, value.id);
                }
                CfgValueKind::LaplaceDerivative {
                    input_derivative, ..
                }
                | CfgValueKind::ZiDerivative {
                    input_derivative, ..
                } => {
                    changed |= needed.union_from(*input_derivative, value.id);
                }
                CfgValueKind::Binary {
                    op: CfgBinaryOp::Mod,
                    left,
                    ..
                } => {
                    changed |= needed.union_from(*left, value.id);
                }
                CfgValueKind::Binary { left, right, op } if !is_predicate(*op) => {
                    changed |= needed.union_from(*left, value.id);
                    changed |= needed.union_from(*right, value.id);
                }
                _ => {}
            }
        }
        if !changed {
            return Ok(needed);
        }
        check_cancelled(control)?;
    }
}

fn ddx_axis_seeds(axis: CfgDdxAxis) -> Vec<AdSeed> {
    match axis {
        CfgDdxAxis::Potential { pos_node, neg_node } => {
            let mut seeds = Vec::with_capacity(2);
            if let Some(node) = pos_node {
                seeds.push(AdSeed::NodePotential(node));
            }
            if let Some(node) = neg_node
                && !seeds.contains(&AdSeed::NodePotential(node))
            {
                seeds.push(AdSeed::NodePotential(node));
            }
            seeds
        }
        CfgDdxAxis::BranchFlow { unknown, .. } => {
            vec![AdSeed::BranchUnknownFlow(unknown)]
        }
    }
}

fn reject_nested_ddx(function: &CfgFunction) -> Result<(), CfgValidationError> {
    let incoming = incoming_arguments(function);
    let mut reaches_ddx = vec![false; function.values.len()];
    loop {
        let mut changed = false;
        for value in &function.values {
            let reaches = match &value.kind {
                CfgValueKind::Ddx { .. } => true,
                CfgValueKind::BlockParameter => incoming.get(&value.id).is_some_and(|arguments| {
                    arguments
                        .iter()
                        .any(|argument| reaches_ddx[usize::from(*argument)])
                }),
                kind => kind
                    .operands()
                    .iter()
                    .any(|operand| reaches_ddx[usize::from(*operand)]),
            };
            let slot = &mut reaches_ddx[usize::from(value.id)];
            if reaches && !*slot {
                *slot = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for value in &function.values {
        if let CfgValueKind::Ddx { value: operand, .. } = value.kind
            && reaches_ddx[usize::from(operand)]
        {
            return Err(CfgValidationError::NestedDdx(value.id));
        }
    }
    Ok(())
}

/// Sparse scalar forward mode used only to materialize `ddx` primals. The
/// ordinary packed pass follows it and differentiates these expressions again.
struct ScalarDdxBuilder<'a> {
    source: &'a CfgFunction,
    seeds: Vec<AdSeed>,
    seed_indices: HashMap<AdSeed, usize>,
    live: LaneLiveness,
    second_order: Vec<bool>,
    values: Vec<CfgValue>,
    blocks: Vec<CfgBlock>,
    derivatives: Vec<Option<ValueId>>,
    constants: HashMap<u64, ValueId>,
    one: ValueId,
    ddt_scale: Option<ValueId>,
    idt_scale: Option<ValueId>,
    added_params: HashMap<BlockId, Vec<(usize, usize)>>,
    emitted: Vec<CfgInstruction>,
}

impl<'a> ScalarDdxBuilder<'a> {
    fn new(
        source: &'a CfgFunction,
        seeds: Vec<AdSeed>,
        live: LaneLiveness,
        second_order: Vec<bool>,
    ) -> Self {
        let mut values = source.values.clone();
        let mut constants: HashMap<u64, ValueId> = values
            .iter()
            .filter_map(|value| match value.kind {
                CfgValueKind::RealConstant(constant) => Some((constant.to_bits(), value.id)),
                _ => None,
            })
            .collect();
        let one = constants
            .get(&1.0_f64.to_bits())
            .copied()
            .unwrap_or_else(|| {
                let id = ValueId::from(values.len());
                values.push(CfgValue {
                    id,
                    value_type: CfgValueType::Real,
                    kind: CfgValueKind::RealConstant(1.0),
                });
                constants.insert(1.0_f64.to_bits(), id);
                id
            });
        let source_value_count = source.values.len();
        let lane_count = seeds.len();
        let seed_indices = seeds
            .iter()
            .enumerate()
            .map(|(index, seed)| (*seed, index))
            .collect();
        let mut builder = Self {
            source,
            seeds,
            seed_indices,
            live,
            second_order,
            values,
            blocks: source.blocks.clone(),
            derivatives: vec![None; source_value_count * lane_count],
            constants,
            one,
            ddt_scale: None,
            idt_scale: None,
            added_params: HashMap::new(),
            emitted: Vec::new(),
        };
        for (value, lane) in seed_lanes(source, &builder.seeds) {
            builder.set_derivative(value, lane, Some(one));
        }
        builder
    }

    fn derivative_index(&self, value: ValueId, lane: usize) -> usize {
        usize::from(value) * self.seeds.len() + lane
    }

    fn derivative(&self, value: ValueId, lane: usize) -> Option<ValueId> {
        self.derivatives[self.derivative_index(value, lane)]
    }

    fn set_derivative(&mut self, value: ValueId, lane: usize, derivative: Option<ValueId>) {
        let index = self.derivative_index(value, lane);
        self.derivatives[index] = derivative;
    }

    fn add_block_parameters(&mut self) {
        for block in &self.source.blocks {
            let mut added = Vec::new();
            for (position, param) in block.params.iter().enumerate() {
                for lane in self.live.lanes(*param) {
                    let derivative =
                        self.new_value(CfgValueType::Real, CfgValueKind::BlockParameter);
                    self.blocks[usize::from(block.id)].params.push(derivative);
                    self.set_derivative(*param, lane, Some(derivative));
                    added.push((position, lane));
                }
            }
            self.added_params.insert(block.id, added);
        }
    }

    fn rewrite_blocks(&mut self, control: &dyn PipelineControl) -> Result<(), PipelineCancelled> {
        for block in reverse_postorder(self.source) {
            check_cancelled(control)?;
            self.emitted = Vec::new();
            for (ordinal, instruction) in self.source.block(block).instructions.iter().enumerate() {
                if ordinal.is_multiple_of(1024) {
                    check_cancelled(control)?;
                }
                let result = instruction.result;
                let original = self.source.value(result).kind.clone();
                if self.second_order[usize::from(result)]
                    && let CfgValueKind::Ddx { value, axis } = original
                {
                    let resolved = self.resolve_ddx_kind(value, axis);
                    self.values[usize::from(result)].kind = resolved;
                    self.emitted.push(CfgInstruction { result });
                    continue;
                }
                self.emitted.push(CfgInstruction { result });
                for lane in self.live.lanes(result) {
                    let derivative = self.scalar_rule(&original, lane);
                    self.set_derivative(result, lane, derivative);
                }
            }
            let terminator = self.rewrite_terminator(block);
            self.blocks[usize::from(block)].instructions = std::mem::take(&mut self.emitted);
            self.blocks[usize::from(block)].terminator = terminator;
        }
        Ok(())
    }

    fn resolve_ddx_kind(&mut self, value: ValueId, axis: CfgDdxAxis) -> CfgValueKind {
        let components: Vec<(AdSeed, f64)> = match axis {
            CfgDdxAxis::Potential {
                pos_node: Some(pos),
                neg_node: Some(neg),
            } if pos != neg => vec![
                (AdSeed::NodePotential(pos), 0.5),
                (AdSeed::NodePotential(neg), -0.5),
            ],
            CfgDdxAxis::Potential {
                pos_node: Some(pos),
                neg_node: _,
            } => vec![(AdSeed::NodePotential(pos), 1.0)],
            CfgDdxAxis::Potential {
                pos_node: None,
                neg_node: Some(neg),
            } => vec![(AdSeed::NodePotential(neg), -1.0)],
            CfgDdxAxis::Potential {
                pos_node: None,
                neg_node: None,
            } => Vec::new(),
            CfgDdxAxis::BranchFlow { unknown, reversed } => vec![(
                AdSeed::BranchUnknownFlow(unknown),
                if reversed { -1.0 } else { 1.0 },
            )],
        };
        let mut terms = Vec::with_capacity(components.len());
        for (seed, coefficient) in components {
            let Some(lane) = self.seed_indices.get(&seed).copied() else {
                continue;
            };
            let Some(derivative) = self.derivative(value, lane) else {
                continue;
            };
            let term = match coefficient {
                1.0 => derivative,
                -1.0 => self.push_unary(CfgUnaryOp::Neg, derivative),
                coefficient => {
                    let coefficient = self.constant(coefficient);
                    self.push_binary(CfgBinaryOp::Mul, coefficient, derivative)
                }
            };
            terms.push(term);
        }
        match terms.as_slice() {
            [] => CfgValueKind::RealConstant(0.0),
            [only] => CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: self.one,
                right: *only,
            },
            [left, right] => CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: *left,
                right: *right,
            },
            _ => unreachable!("a ddx axis has at most two components"),
        }
    }

    fn rewrite_terminator(&mut self, block: BlockId) -> CfgTerminator {
        match self.source.block(block).terminator.clone() {
            CfgTerminator::Jump { target, mut args } => {
                args.extend(self.derivative_arguments(target, &args));
                CfgTerminator::Jump { target, args }
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                mut then_args,
                else_target,
                mut else_args,
            } => {
                then_args.extend(self.derivative_arguments(then_target, &then_args));
                else_args.extend(self.derivative_arguments(else_target, &else_args));
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

    fn derivative_arguments(&mut self, target: BlockId, args: &[ValueId]) -> Vec<ValueId> {
        let added = self.added_params.get(&target).cloned().unwrap_or_default();
        added
            .into_iter()
            .map(|(position, lane)| {
                args.get(position)
                    .and_then(|argument| self.derivative(*argument, lane))
                    .unwrap_or_else(|| self.constant(0.0))
            })
            .collect()
    }

    fn scalar_rule(&mut self, kind: &CfgValueKind, lane: usize) -> Option<ValueId> {
        match kind {
            CfgValueKind::Unary { op, input } => {
                let derivative = self.derivative(*input, lane)?;
                let factor = self.unary_factor(*op, *input);
                Some(self.push_binary(CfgBinaryOp::Mul, derivative, factor))
            }
            CfgValueKind::Binary { op, left, right } => self.binary_rule(*op, *left, *right, lane),
            CfgValueKind::Ddt { input, .. } => {
                let derivative = self.derivative(*input, lane)?;
                let scale = self.ddt_scale();
                Some(self.push_binary(CfgBinaryOp::Mul, derivative, scale))
            }
            CfgValueKind::Idt { input, .. } | CfgValueKind::IdtMod { input, .. } => {
                let derivative = self.derivative(*input, lane)?;
                let scale = self.idt_scale();
                Some(self.push_binary(CfgBinaryOp::Mul, derivative, scale))
            }
            CfgValueKind::AbsDelay {
                operator,
                input,
                delay,
                max_delay,
            } => {
                let (input_derivative, delay_derivative) =
                    self.delayed_derivatives(*input, *delay, lane)?;
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::AbsDelayDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        delay: *delay,
                        delay_derivative,
                        max_delay: *max_delay,
                        order: 1,
                    },
                ))
            }
            CfgValueKind::AbsDelayDerivative {
                operator,
                input,
                input_derivative,
                delay,
                delay_derivative,
                max_delay,
                order,
            } => {
                let (input_derivative, delay_derivative) =
                    self.delayed_derivatives(*input_derivative, *delay_derivative, lane)?;
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::AbsDelayDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        delay: *delay,
                        delay_derivative,
                        max_delay: *max_delay,
                        order: order.saturating_add(1),
                    },
                ))
            }
            CfgValueKind::Slew {
                operator,
                input,
                max_rise,
                max_fall,
            } => {
                let (max_fall, max_fall_negated) = self.slew_falling_rate(*max_rise, *max_fall);
                let input_derivative = self.derivative(*input, lane);
                let max_rise_derivative = self.derivative(*max_rise, lane);
                let max_fall_derivative = if max_fall_negated {
                    max_rise_derivative.map(|value| self.push_unary(CfgUnaryOp::Neg, value))
                } else {
                    self.derivative(max_fall, lane)
                };
                if input_derivative.is_none()
                    && max_rise_derivative.is_none()
                    && max_fall_derivative.is_none()
                {
                    return None;
                }
                let input_derivative = self.or_zero(input_derivative);
                let max_rise_derivative = self.or_zero(max_rise_derivative);
                let max_fall_derivative = self.or_zero(max_fall_derivative);
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::SlewDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        max_rise: *max_rise,
                        max_rise_derivative,
                        max_fall,
                        max_fall_derivative,
                    },
                ))
            }
            CfgValueKind::SlewDerivative {
                operator,
                input,
                input_derivative,
                max_rise,
                max_rise_derivative,
                max_fall,
                max_fall_derivative,
            } => {
                let next_input = self.derivative(*input_derivative, lane);
                let next_rise = self.derivative(*max_rise_derivative, lane);
                let next_fall = self.derivative(*max_fall_derivative, lane);
                if next_input.is_none() && next_rise.is_none() && next_fall.is_none() {
                    return None;
                }
                let input_derivative = self.or_zero(next_input);
                let max_rise_derivative = self.or_zero(next_rise);
                let max_fall_derivative = self.or_zero(next_fall);
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::SlewDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        max_rise: *max_rise,
                        max_rise_derivative,
                        max_fall: *max_fall,
                        max_fall_derivative,
                    },
                ))
            }
            // A filter is linear, so differentiating it twice is the same node
            // again with the next derivative in it. There is no order to track
            // the way `absdelay` tracks one: the second derivative of a linear
            // map is not a Hessian term the runtime has to refuse, it is the
            // filter applied to a second derivative that is usually zero.
            CfgValueKind::Laplace {
                operator,
                input,
                transfer,
            } => {
                let input_derivative = self.derivative(*input, lane)?;
                let transfer = transfer.clone();
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::LaplaceDerivative {
                        operator: *operator,
                        input_derivative,
                        transfer,
                    },
                ))
            }
            CfgValueKind::LaplaceDerivative {
                operator,
                input_derivative,
                transfer,
            } => {
                let next = self.derivative(*input_derivative, lane)?;
                let transfer = transfer.clone();
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::LaplaceDerivative {
                        operator: *operator,
                        input_derivative: next,
                        transfer,
                    },
                ))
            }
            CfgValueKind::Zi {
                operator,
                input,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let input_derivative = self.derivative(*input, lane)?;
                let (numerator, denominator) = (numerator.clone(), denominator.clone());
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::ZiDerivative {
                        operator: *operator,
                        input_derivative,
                        numerator,
                        denominator,
                        period: *period,
                        transition: *transition,
                        first_transition: *first_transition,
                        direct_assignment: *direct_assignment,
                    },
                ))
            }
            CfgValueKind::ZiDerivative {
                operator,
                input_derivative,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let next = self.derivative(*input_derivative, lane)?;
                let (numerator, denominator) = (numerator.clone(), denominator.clone());
                Some(self.push(
                    CfgValueType::Real,
                    CfgValueKind::ZiDerivative {
                        operator: *operator,
                        input_derivative: next,
                        numerator,
                        denominator,
                        period: *period,
                        transition: *transition,
                        first_transition: *first_transition,
                        direct_assignment: *direct_assignment,
                    },
                ))
            }
            CfgValueKind::Limit { proposed, .. } => self.derivative(*proposed, lane),
            _ => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn binary_rule(
        &mut self,
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
        lane: usize,
    ) -> Option<ValueId> {
        let d_left = self.derivative(left, lane);
        let d_right = self.derivative(right, lane);
        match op {
            CfgBinaryOp::Add => match (d_left, d_right) {
                (Some(a), Some(b)) => Some(self.push_binary(CfgBinaryOp::Add, a, b)),
                (Some(only), None) | (None, Some(only)) => Some(only),
                (None, None) => None,
            },
            CfgBinaryOp::Sub => match (d_left, d_right) {
                (Some(a), Some(b)) => Some(self.push_binary(CfgBinaryOp::Sub, a, b)),
                (Some(only), None) => Some(only),
                (None, Some(only)) => Some(self.push_unary(CfgUnaryOp::Neg, only)),
                (None, None) => None,
            },
            CfgBinaryOp::Mul => match (d_left, d_right) {
                (Some(a), Some(b)) => {
                    let first = self.push_binary(CfgBinaryOp::Mul, a, right);
                    let second = self.push_binary(CfgBinaryOp::Mul, b, left);
                    Some(self.push_binary(CfgBinaryOp::Add, first, second))
                }
                (Some(a), None) => Some(self.push_binary(CfgBinaryOp::Mul, a, right)),
                (None, Some(b)) => Some(self.push_binary(CfgBinaryOp::Mul, b, left)),
                (None, None) => None,
            },
            CfgBinaryOp::Div => {
                let numerator = match (d_left, d_right) {
                    (Some(a), Some(b)) => {
                        let quotient = self.push_binary(CfgBinaryOp::Div, left, right);
                        let scaled = self.push_binary(CfgBinaryOp::Mul, quotient, b);
                        self.push_binary(CfgBinaryOp::Sub, a, scaled)
                    }
                    (Some(a), None) => a,
                    (None, Some(b)) => {
                        let quotient = self.push_binary(CfgBinaryOp::Div, left, right);
                        let scaled = self.push_binary(CfgBinaryOp::Mul, quotient, b);
                        self.push_unary(CfgUnaryOp::Neg, scaled)
                    }
                    (None, None) => return None,
                };
                Some(self.push_binary(CfgBinaryOp::Div, numerator, right))
            }
            CfgBinaryOp::Pow => {
                let from_base = d_left.map(|derivative| {
                    let exponent = self.push_binary(CfgBinaryOp::Sub, right, self.one);
                    let reduced = self.push_binary(CfgBinaryOp::Pow, left, exponent);
                    let factor = self.push_binary(CfgBinaryOp::Mul, right, reduced);
                    self.push_binary(CfgBinaryOp::Mul, derivative, factor)
                });
                let from_exponent = d_right.map(|derivative| {
                    let power = self.push_binary(CfgBinaryOp::Pow, left, right);
                    let logarithm = self.push_unary(CfgUnaryOp::Ln, left);
                    let factor = self.push_binary(CfgBinaryOp::Mul, power, logarithm);
                    self.push_binary(CfgBinaryOp::Mul, derivative, factor)
                });
                match (from_base, from_exponent) {
                    (Some(a), Some(b)) => Some(self.push_binary(CfgBinaryOp::Add, a, b)),
                    (Some(only), None) | (None, Some(only)) => Some(only),
                    (None, None) => None,
                }
            }
            CfgBinaryOp::Mod => d_left,
            // A selection, written as a mask over both arms.
            //
            // `db + (da - db)*c` is the same algebra in three operations rather
            // than four, and it is not the same number: with `c = 1` it
            // evaluates `db + fl(da - db)`, whose error is `2*u*|db|` against a
            // result of `|da|`. Where the losing arm's derivative is orders
            // above the winner's — which is what a `max` guarding a compact
            // model's floor is for — that is not rounding but the whole answer:
            // `da = 1`, `db = 1e17` returns zero. The masked form below is
            // exact for `c` in `{0, 1}`, because `x*1` and `x + 0` are exact.
            CfgBinaryOp::Min | CfgBinaryOp::Max => {
                let comparison = if op == CfgBinaryOp::Min {
                    CfgBinaryOp::Le
                } else {
                    CfgBinaryOp::Ge
                };
                let takes_left = self.push_typed(CfgValueType::Boolean, comparison, left, right);
                match (d_left, d_right) {
                    (Some(a), Some(b)) => {
                        let takes_right = self.push_binary(CfgBinaryOp::Sub, self.one, takes_left);
                        let from_left = self.push_binary(CfgBinaryOp::Mul, a, takes_left);
                        let from_right = self.push_binary(CfgBinaryOp::Mul, b, takes_right);
                        Some(self.push_binary(CfgBinaryOp::Add, from_left, from_right))
                    }
                    (Some(a), None) => Some(self.push_binary(CfgBinaryOp::Mul, a, takes_left)),
                    (None, Some(b)) => {
                        let takes_right = self.push_binary(CfgBinaryOp::Sub, self.one, takes_left);
                        Some(self.push_binary(CfgBinaryOp::Mul, b, takes_right))
                    }
                    (None, None) => None,
                }
            }
            other => {
                debug_assert!(is_predicate(other), "unhandled scalar derivative rule");
                None
            }
        }
    }

    fn unary_factor(&mut self, op: CfgUnaryOp, input: ValueId) -> ValueId {
        match op {
            CfgUnaryOp::Neg => self.constant(-1.0),
            CfgUnaryOp::Exp => self.push_unary(CfgUnaryOp::Exp, input),
            CfgUnaryOp::LimExp => self.push_unary(CfgUnaryOp::LimExp, input),
            CfgUnaryOp::LimitedExp => self.push_unary(CfgUnaryOp::LimitedExpDerivative, input),
            CfgUnaryOp::LimitedExpDerivative => self.constant(0.0),
            CfgUnaryOp::Ln => self.push_binary(CfgBinaryOp::Div, self.one, input),
            CfgUnaryOp::Log10 => {
                let ln_ten = self.constant(std::f64::consts::LN_10);
                let denominator = self.push_binary(CfgBinaryOp::Mul, input, ln_ten);
                self.push_binary(CfgBinaryOp::Div, self.one, denominator)
            }
            CfgUnaryOp::Sqrt => {
                let root = self.push_unary(CfgUnaryOp::Sqrt, input);
                let two = self.constant(2.0);
                let denominator = self.push_binary(CfgBinaryOp::Mul, two, root);
                self.push_binary(CfgBinaryOp::Div, self.one, denominator)
            }
            CfgUnaryOp::Abs => {
                let zero = self.constant(0.0);
                let positive = self.push_typed(CfgValueType::Boolean, CfgBinaryOp::Ge, input, zero);
                let two = self.constant(2.0);
                let doubled = self.push_binary(CfgBinaryOp::Mul, two, positive);
                self.push_binary(CfgBinaryOp::Sub, doubled, self.one)
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
                self.push_binary(CfgBinaryOp::Div, self.one, squared)
            }
            CfgUnaryOp::Sinh => self.push_unary(CfgUnaryOp::Cosh, input),
            CfgUnaryOp::Cosh => self.push_unary(CfgUnaryOp::Sinh, input),
            CfgUnaryOp::Tanh => {
                let tangent = self.push_unary(CfgUnaryOp::Tanh, input);
                let squared = self.push_binary(CfgBinaryOp::Mul, tangent, tangent);
                self.push_binary(CfgBinaryOp::Sub, self.one, squared)
            }
            CfgUnaryOp::Asin | CfgUnaryOp::Acos => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let difference = self.push_binary(CfgBinaryOp::Sub, self.one, squared);
                let root = self.push_unary(CfgUnaryOp::Sqrt, difference);
                let numerator = if matches!(op, CfgUnaryOp::Acos) {
                    self.constant(-1.0)
                } else {
                    self.one
                };
                self.push_binary(CfgBinaryOp::Div, numerator, root)
            }
            CfgUnaryOp::Atan => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let denominator = self.push_binary(CfgBinaryOp::Add, self.one, squared);
                self.push_binary(CfgBinaryOp::Div, self.one, denominator)
            }
            CfgUnaryOp::Asinh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let sum = self.push_binary(CfgBinaryOp::Add, self.one, squared);
                let root = self.push_unary(CfgUnaryOp::Sqrt, sum);
                self.push_binary(CfgBinaryOp::Div, self.one, root)
            }
            CfgUnaryOp::Acosh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let difference = self.push_binary(CfgBinaryOp::Sub, squared, self.one);
                let root = self.push_unary(CfgUnaryOp::Sqrt, difference);
                self.push_binary(CfgBinaryOp::Div, self.one, root)
            }
            CfgUnaryOp::Atanh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let denominator = self.push_binary(CfgBinaryOp::Sub, self.one, squared);
                self.push_binary(CfgBinaryOp::Div, self.one, denominator)
            }
            CfgUnaryOp::Not | CfgUnaryOp::Floor | CfgUnaryOp::Ceil => self.constant(0.0),
        }
    }

    fn ddt_scale(&mut self) -> ValueId {
        if let Some(value) = self.ddt_scale {
            return value;
        }
        let value = self.new_value(CfgValueType::Real, CfgValueKind::DdtScale);
        self.ddt_scale = Some(value);
        value
    }

    fn idt_scale(&mut self) -> ValueId {
        if let Some(value) = self.idt_scale {
            return value;
        }
        let value = self.new_value(CfgValueType::Real, CfgValueKind::IdtScale);
        self.idt_scale = Some(value);
        value
    }

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

    /// A derivative or an explicit zero.
    ///
    /// A [`CfgValueKind::AbsDelayDerivative`] or
    /// [`CfgValueKind::SlewDerivative`] names every operand it multiplies, so a
    /// missing lane has to become a value rather than an absence: the runtime
    /// multiplies whatever the node points at.
    fn or_zero(&mut self, derivative: Option<ValueId>) -> ValueId {
        match derivative {
            Some(value) => value,
            None => self.constant(0.0),
        }
    }

    /// The input and delay derivatives of one `absdelay`, or `None` when
    /// neither operand moves with this lane.
    fn delayed_derivatives(
        &mut self,
        input: ValueId,
        delay: ValueId,
        lane: usize,
    ) -> Option<(ValueId, ValueId)> {
        let input_derivative = self.derivative(input, lane);
        let delay_derivative = self.derivative(delay, lane);
        if input_derivative.is_none() && delay_derivative.is_none() {
            return None;
        }
        let input_derivative = self.or_zero(input_derivative);
        let delay_derivative = self.or_zero(delay_derivative);
        Some((input_derivative, delay_derivative))
    }

    /// The falling rate a `slew` node stands for, and whether it had to be
    /// built by negating the rising one.
    ///
    /// Verilog-AMS gives an omitted `max_fall` the rising rate's magnitude in
    /// the falling direction. Materialising it here rather than leaving the
    /// node's operand absent is what lets the derivative carry `-d(max_rise)`
    /// as an ordinary lane instead of a special case at every consumer.
    fn slew_falling_rate(
        &mut self,
        max_rise: ValueId,
        max_fall: Option<ValueId>,
    ) -> (ValueId, bool) {
        match max_fall {
            Some(max_fall) => (max_fall, false),
            None => (self.push_unary(CfgUnaryOp::Neg, max_rise), true),
        }
    }
}

/// Differentiate `function` with respect to `lanes`.
pub fn differentiate(
    function: &CfgFunction,
    lanes: &[AdSeed],
) -> Result<AdFunction, CfgValidationError> {
    match differentiate_with_control(function, lanes, &NoPipelineControl) {
        Ok(function) => Ok(function),
        Err(DifferentiationError::Validation(error)) => Err(error),
        Err(DifferentiationError::Cancelled(_)) => {
            unreachable!("the no-op pipeline control cannot cancel")
        }
    }
}

pub(crate) enum DifferentiationError {
    Validation(CfgValidationError),
    Cancelled(PipelineCancelled),
}

pub(crate) fn differentiate_with_control(
    function: &CfgFunction,
    lanes: &[AdSeed],
    control: &dyn PipelineControl,
) -> Result<AdFunction, DifferentiationError> {
    differentiate_with_control_for_optional_roots(function, lanes, None, control)
}

/// Differentiate a generated stamp while expanding second-order `ddx` work
/// only for values that can reach the residual or charge rows the backend will
/// extract. Other `ddx` readbacks (notably noise/reporting expressions) are
/// still resolved by the packed first-order pass, but do not inflate the stamp
/// with second derivatives no caller observes.
pub(crate) fn differentiate_with_control_for_roots(
    function: &CfgFunction,
    lanes: &[AdSeed],
    roots: &[ValueId],
    control: &dyn PipelineControl,
) -> Result<AdFunction, DifferentiationError> {
    differentiate_with_control_for_optional_roots(function, lanes, Some(roots), control)
}

fn differentiate_with_control_for_optional_roots(
    function: &CfgFunction,
    lanes: &[AdSeed],
    roots: Option<&[ValueId]>,
    control: &dyn PipelineControl,
) -> Result<AdFunction, DifferentiationError> {
    // Refuse a discrete-domain value up front rather than letting one fall
    // through `differentiable`'s catch-all, which would report `false` and
    // leave a silent zero where a derivative should be. The check is one pass
    // over the value table and runs before anything is allocated.
    //
    // The *kind* is asked as well as the type, and the two do not answer the
    // same question. Every four-state kind carries a four-state type, so the
    // type alone caught them; a real net's read (Verilog-AMS LRM 2.4 section
    // 3.7) carries `CfgValueType::Real`, which is exactly the type an analog
    // quantity has, and would sail through a type-only guard to be
    // differentiated as though a `wreal` were a node voltage.
    if let Some(value) = function
        .values
        .iter()
        .find(|value| value.value_type.is_digital() || value.kind.is_digital())
    {
        return Err(DifferentiationError::Validation(
            CfgValidationError::DigitalValueInDerivative(value.id),
        ));
    }
    let resolved = resolve_ddx_primal_with_control(function, roots, control)?;
    let live = lane_liveness_with_control(&resolved, lanes, control)
        .map_err(DifferentiationError::Cancelled)?;
    let mut builder = AdBuilder::new(&resolved, lanes, &live, control)
        .map_err(DifferentiationError::Cancelled)?;
    builder
        .add_block_parameters(control)
        .map_err(DifferentiationError::Cancelled)?;
    builder
        .rewrite_blocks(control)
        .map_err(DifferentiationError::Cancelled)?;

    let differentiated = CfgFunction {
        entry: function.entry,
        blocks: builder.blocks,
        values: builder.values,
        shapes: builder.shapes,
    };
    check_cancelled(control).map_err(DifferentiationError::Cancelled)?;
    differentiated
        .validate()
        .map_err(DifferentiationError::Validation)?;
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

fn check_cancelled(control: &dyn PipelineControl) -> Result<(), PipelineCancelled> {
    if control.is_cancelled() {
        Err(PipelineCancelled {
            phase: PipelinePhase::Differentiation,
        })
    } else {
        Ok(())
    }
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
    fn new(
        source: &'a CfgFunction,
        lanes: &[AdSeed],
        live: &LaneLiveness,
        control: &dyn PipelineControl,
    ) -> Result<Self, PipelineCancelled> {
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

        for index in 0..source.values.len() {
            if index.is_multiple_of(1024) {
                check_cancelled(control)?;
            }
            let value = ValueId::from(index);
            if live.is_empty(value) {
                continue;
            }
            let lanes: Vec<u32> = live
                .lanes(value)
                .into_iter()
                .map(|lane| u32::try_from(lane).expect("lane count fits a u32"))
                .collect();
            builder.target[index] = Some(builder.intern(lanes));
        }

        // The seeds. A node potential's derivative is one in its own lane, and
        // liveness gives it exactly that lane, so the packed form is a one-wide
        // value holding 1.0.
        for (ordinal, (value, _)) in seed_lanes(source, lanes).into_iter().enumerate() {
            if ordinal.is_multiple_of(1024) {
                check_cancelled(control)?;
            }
            let shape = builder.target[usize::from(value)].expect("a seed carries its own lane");
            let seed = builder.splat(1.0, shape);
            builder.derivatives[usize::from(value)] = Some(seed);
        }

        Ok(builder)
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
    fn add_block_parameters(
        &mut self,
        control: &dyn PipelineControl,
    ) -> Result<(), PipelineCancelled> {
        for (ordinal, block) in self.source.blocks.clone().iter().enumerate() {
            if ordinal.is_multiple_of(64) {
                check_cancelled(control)?;
            }
            let mut added = Vec::new();
            for (position, param) in block.params.iter().enumerate() {
                let Some(shape) = self.target[usize::from(*param)] else {
                    continue;
                };
                let derivative =
                    self.new_value(CfgValueType::Lanes(shape), CfgValueKind::BlockParameter);
                self.blocks[usize::from(block.id)].params.push(derivative);
                self.derivatives[usize::from(*param)] = Some(derivative);
                added.push((position, shape));
            }
            self.added_params.insert(block.id, added);
        }
        Ok(())
    }

    fn rewrite_blocks(&mut self, control: &dyn PipelineControl) -> Result<(), PipelineCancelled> {
        // Reverse postorder, so every operand's derivative exists before a rule
        // names it. Values crossing a back edge do not need the ordering: they
        // arrive as block parameters, which were allocated up front.
        for block in reverse_postorder(self.source) {
            check_cancelled(control)?;
            self.emitted = Vec::with_capacity(self.source.block(block).instructions.len());
            for (ordinal, instruction) in self
                .source
                .block(block)
                .instructions
                .clone()
                .iter()
                .enumerate()
            {
                if ordinal.is_multiple_of(1024) {
                    check_cancelled(control)?;
                }
                let result = instruction.result;
                // Read-outs have to land before the instruction that reads them.
                if let CfgValueKind::Ddx { value, axis } = self.source.value(result).kind {
                    self.resolve_ddx(result, value, axis);
                }
                self.emitted.push(CfgInstruction { result });
                let derivative = self.rule(result);
                self.derivatives[usize::from(result)] = derivative;
            }
            let terminator = self.rewrite_terminator(block);
            self.blocks[usize::from(block)].instructions = std::mem::take(&mut self.emitted);
            self.blocks[usize::from(block)].terminator = terminator;
        }
        Ok(())
    }

    /// Turn a `ddx` into the lane it names.
    ///
    /// `ddx(f, V(p, n))` is `(∂f/∂p − ∂f/∂n)/2`: changing the differential
    /// voltage holds common mode fixed, so each endpoint moves by half the
    /// requested change. A branch-flow probe selects its solver-owned branch
    /// lane directly, with orientation applied.
    /// Reading it back rather than lowering it separately is what lets a model
    /// asking for its own transconductance get the entry the backend already
    /// produces instead of a second, separately maintained one.
    ///
    /// A lane the value does not carry is a structural zero and folds away here.
    fn resolve_ddx(&mut self, result: ValueId, value: ValueId, axis: CfgDdxAxis) {
        let kind = match axis {
            CfgDdxAxis::Potential { pos_node, neg_node } => {
                let positive = self.extract_lane(value, pos_node.map(AdSeed::NodePotential));
                let negative = self.extract_lane(value, neg_node.map(AdSeed::NodePotential));
                if neg_node.is_some() {
                    match (positive, negative) {
                        (None, None) => CfgValueKind::RealConstant(0.0),
                        (positive, negative) => {
                            let positive = positive.unwrap_or_else(|| self.constant(0.0));
                            let negative = negative.unwrap_or_else(|| self.constant(0.0));
                            let difference = self.push(
                                CfgValueType::Real,
                                CfgValueKind::Binary {
                                    op: CfgBinaryOp::Sub,
                                    left: positive,
                                    right: negative,
                                },
                            );
                            CfgValueKind::Binary {
                                op: CfgBinaryOp::Mul,
                                left: self.constant(0.5),
                                right: difference,
                            }
                        }
                    }
                } else {
                    match positive {
                        Some(positive) => CfgValueKind::Binary {
                            op: CfgBinaryOp::Mul,
                            left: self.one,
                            right: positive,
                        },
                        None => CfgValueKind::RealConstant(0.0),
                    }
                }
            }
            CfgDdxAxis::BranchFlow { unknown, reversed } => {
                match self.extract_lane(value, Some(AdSeed::BranchUnknownFlow(unknown))) {
                    Some(derivative) if reversed => CfgValueKind::Unary {
                        op: CfgUnaryOp::Neg,
                        input: derivative,
                    },
                    Some(derivative) => CfgValueKind::Binary {
                        op: CfgBinaryOp::Mul,
                        left: self.one,
                        right: derivative,
                    },
                    None => CfgValueKind::RealConstant(0.0),
                }
            }
        };
        self.values[usize::from(result)].kind = kind;
    }

    /// The scalar partial of `value` with respect to one solver unknown.
    fn extract_lane(&mut self, value: ValueId, seed: Option<AdSeed>) -> Option<ValueId> {
        let seed = seed?;
        let lane = self.lanes.iter().position(|candidate| *candidate == seed)?;
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
            // The fold is a translation by a whole number of periods, so it
            // drops out of the derivative and the companion coefficient is the
            // unwrapped integral's. The modulus and offset place the branch;
            // they do not scale what crosses it.
            CfgValueKind::IdtMod { input, .. } => {
                let derivative = self.derivatives[usize::from(*input)]?;
                let scale = self.idt_scale();
                Some(self.scale(derivative, scale))
            }
            CfgValueKind::AbsDelay {
                operator,
                input,
                delay,
                max_delay,
            } => {
                let (input_derivative, delay_derivative) =
                    self.delayed_lane_derivatives(*input, *delay, target)?;
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::AbsDelayDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        delay: *delay,
                        delay_derivative,
                        max_delay: *max_delay,
                        order: 1,
                    },
                ))
            }
            CfgValueKind::AbsDelayDerivative {
                operator,
                input,
                input_derivative,
                delay,
                delay_derivative,
                max_delay,
                order,
            } => {
                let (input_derivative, delay_derivative) =
                    self.delayed_lane_derivatives(*input_derivative, *delay_derivative, target)?;
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::AbsDelayDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        delay: *delay,
                        delay_derivative,
                        max_delay: *max_delay,
                        order: order.saturating_add(1),
                    },
                ))
            }
            CfgValueKind::Slew {
                operator,
                input,
                max_rise,
                max_fall,
            } => {
                let (max_fall, negated) = self.slew_falling_rate(*max_rise, *max_fall);
                let input_derivative = self.derivatives[usize::from(*input)];
                let max_rise_derivative = self.derivatives[usize::from(*max_rise)];
                let max_fall_derivative = if negated {
                    max_rise_derivative.map(|value| self.negate_lanes(value, target))
                } else {
                    self.derivatives[usize::from(max_fall)]
                };
                if input_derivative.is_none()
                    && max_rise_derivative.is_none()
                    && max_fall_derivative.is_none()
                {
                    return None;
                }
                let input_derivative = self.or_zero_lanes(input_derivative, target);
                let max_rise_derivative = self.or_zero_lanes(max_rise_derivative, target);
                let max_fall_derivative = self.or_zero_lanes(max_fall_derivative, target);
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::SlewDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        max_rise: *max_rise,
                        max_rise_derivative,
                        max_fall,
                        max_fall_derivative,
                    },
                ))
            }
            CfgValueKind::SlewDerivative {
                operator,
                input,
                input_derivative,
                max_rise,
                max_rise_derivative,
                max_fall,
                max_fall_derivative,
            } => {
                let next_input = self.derivatives[usize::from(*input_derivative)];
                let next_rise = self.derivatives[usize::from(*max_rise_derivative)];
                let next_fall = self.derivatives[usize::from(*max_fall_derivative)];
                if next_input.is_none() && next_rise.is_none() && next_fall.is_none() {
                    return None;
                }
                let input_derivative = self.or_zero_lanes(next_input, target);
                let max_rise_derivative = self.or_zero_lanes(next_rise, target);
                let max_fall_derivative = self.or_zero_lanes(next_fall, target);
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::SlewDerivative {
                        operator: *operator,
                        input: *input,
                        input_derivative,
                        max_rise: *max_rise,
                        max_rise_derivative,
                        max_fall: *max_fall,
                        max_fall_derivative,
                    },
                ))
            }
            CfgValueKind::Laplace {
                operator,
                input,
                transfer,
            } => {
                let input_derivative = self.derivatives[usize::from(*input)]?;
                let transfer = transfer.clone();
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::LaplaceDerivative {
                        operator: *operator,
                        input_derivative,
                        transfer,
                    },
                ))
            }
            CfgValueKind::LaplaceDerivative {
                operator,
                input_derivative,
                transfer,
            } => {
                let next = self.derivatives[usize::from(*input_derivative)]?;
                let transfer = transfer.clone();
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::LaplaceDerivative {
                        operator: *operator,
                        input_derivative: next,
                        transfer,
                    },
                ))
            }
            CfgValueKind::Zi {
                operator,
                input,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let input_derivative = self.derivatives[usize::from(*input)]?;
                let (numerator, denominator) = (numerator.clone(), denominator.clone());
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::ZiDerivative {
                        operator: *operator,
                        input_derivative,
                        numerator,
                        denominator,
                        period: *period,
                        transition: *transition,
                        first_transition: *first_transition,
                        direct_assignment: *direct_assignment,
                    },
                ))
            }
            CfgValueKind::ZiDerivative {
                operator,
                input_derivative,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let next = self.derivatives[usize::from(*input_derivative)]?;
                let (numerator, denominator) = (numerator.clone(), denominator.clone());
                Some(self.push(
                    CfgValueType::Lanes(target),
                    CfgValueKind::ZiDerivative {
                        operator: *operator,
                        input_derivative: next,
                        numerator,
                        denominator,
                        period: *period,
                        transition: *transition,
                        first_transition: *first_transition,
                        direct_assignment: *direct_assignment,
                    },
                ))
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
                        let quotient = self.push_binary(CfgBinaryOp::Div, left, right);
                        let scaled = self.scale(d_right, quotient);
                        self.lane_binary(CfgBinaryOp::Sub, d_left, scaled, target)
                    }
                    (Some(d_left), None) => d_left,
                    (None, Some(d_right)) => {
                        let quotient = self.push_binary(CfgBinaryOp::Div, left, right);
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
                    (Some(from_base), Some(from_exponent)) => {
                        Some(self.lane_binary(CfgBinaryOp::Add, from_base, from_exponent, target))
                    }
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
                let takes_left = self.push_typed(CfgValueType::Boolean, comparison, left, right);
                match (d_left, d_right) {
                    // Masked over both arms rather than blended. The scalar
                    // rule above carries the reason: `db + (da - db)*c` loses
                    // the winner's derivative whenever the loser's is orders
                    // larger, and a `max` against a floor is exactly that
                    // shape.
                    (Some(d_left), Some(d_right)) => {
                        let one = self.one;
                        let takes_right = self.push_binary(CfgBinaryOp::Sub, one, takes_left);
                        let from_left = self.scale(d_left, takes_left);
                        let from_right = self.scale(d_right, takes_right);
                        Some(self.lane_binary(CfgBinaryOp::Add, from_left, from_right, target))
                    }
                    (Some(d_left), None) => Some(self.scale(d_left, takes_left)),
                    // db + c*(0 - db) is db*(1 - c).
                    (None, Some(d_right)) => {
                        let one = self.one;
                        let takes_right = self.push_binary(CfgBinaryOp::Sub, one, takes_left);
                        Some(self.scale(d_right, takes_right))
                    }
                    (None, None) => None,
                }
            }
            // `(a da + b db) / hypot(a, b)`: the gradient is the unit vector
            // along the operands, written with one division rather than two in
            // the same shape as the quotient rule above.
            //
            // It does not inherit `hypot`'s headroom, and nothing here could:
            // the products are formed before the divide, so operands large
            // enough that `hypot` earns its keep overflow this. The alternative
            // costs a division per lane to move the same limit a few orders,
            // which is not a trade worth making on every entry of every
            // Jacobian.
            CfgBinaryOp::Hypot => {
                let numerator = match (d_left, d_right) {
                    (Some(d_left), Some(d_right)) => {
                        let first = self.scale(d_left, left);
                        let second = self.scale(d_right, right);
                        self.lane_binary(CfgBinaryOp::Add, first, second, target)
                    }
                    (Some(d_left), None) => self.scale(d_left, left),
                    (None, Some(d_right)) => self.scale(d_right, right),
                    (None, None) => return None,
                };
                let magnitude = self.push_binary(CfgBinaryOp::Hypot, left, right);
                Some(self.lane_scalar(CfgBinaryOp::Div, numerator, magnitude))
            }
            // `(x dy - y dx) / (x² + y²)` for `atan2(y, x)`. The quadrant offset
            // the operation carries is piecewise constant, so it differentiates
            // to nothing and the ordinary arctangent rule is the whole answer
            // away from the branch cut.
            CfgBinaryOp::Atan2 => {
                let numerator = match (d_left, d_right) {
                    (Some(d_left), Some(d_right)) => {
                        let from_ordinate = self.scale(d_left, right);
                        let from_abscissa = self.scale(d_right, left);
                        self.lane_binary(CfgBinaryOp::Sub, from_ordinate, from_abscissa, target)
                    }
                    (Some(d_left), None) => self.scale(d_left, right),
                    (None, Some(d_right)) => {
                        let from_abscissa = self.scale(d_right, left);
                        self.negate(from_abscissa)
                    }
                    (None, None) => return None,
                };
                let ordinate = self.push_binary(CfgBinaryOp::Mul, left, left);
                let abscissa = self.push_binary(CfgBinaryOp::Mul, right, right);
                let denominator = self.push_binary(CfgBinaryOp::Add, ordinate, abscissa);
                Some(self.lane_scalar(CfgBinaryOp::Div, numerator, denominator))
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
            // `1 / (x ln 10)`, with the constant folded once rather than
            // written as `log10(e) / x`: both are one division, and this one
            // needs no second transcendental constant to be named.
            CfgUnaryOp::Log10 => {
                let ln_ten = self.constant(std::f64::consts::LN_10);
                let denominator = self.push_binary(CfgBinaryOp::Mul, input, ln_ten);
                let one = self.one;
                self.push_binary(CfgBinaryOp::Div, one, denominator)
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
                let positive = self.push_typed(CfgValueType::Boolean, CfgBinaryOp::Ge, input, zero);
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
            // `1 / sqrt(1 - x²)`, shared with `acos` up to the sign.
            CfgUnaryOp::Asin | CfgUnaryOp::Acos => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let one = self.one;
                let difference = self.push_binary(CfgBinaryOp::Sub, one, squared);
                let root = self.push_unary(CfgUnaryOp::Sqrt, difference);
                let numerator = if matches!(op, CfgUnaryOp::Acos) {
                    self.constant(-1.0)
                } else {
                    one
                };
                self.push_binary(CfgBinaryOp::Div, numerator, root)
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
            // `1 / sqrt(x² - 1)`, the same shape with the subtraction the other
            // way round — which is also where its domain differs.
            CfgUnaryOp::Acosh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let one = self.one;
                let difference = self.push_binary(CfgBinaryOp::Sub, squared, one);
                let root = self.push_unary(CfgUnaryOp::Sqrt, difference);
                self.push_binary(CfgBinaryOp::Div, one, root)
            }
            CfgUnaryOp::Atanh => {
                let squared = self.push_binary(CfgBinaryOp::Mul, input, input);
                let one = self.one;
                let denominator = self.push_binary(CfgBinaryOp::Sub, one, squared);
                self.push_binary(CfgBinaryOp::Div, one, denominator)
            }
            // Piecewise constant, so zero away from the steps. `differentiable`
            // keeps these out, and this arm exists so adding a unary op cannot
            // silently take the wrong branch.
            CfgUnaryOp::Not | CfgUnaryOp::Floor | CfgUnaryOp::Ceil => self.constant(0.0),
        }
    }

    // ---- stateful-operator operands ----------------------------------------

    /// A packed derivative laid out in `target`, or a zero of that shape.
    ///
    /// The runtime multiplies every operand an `absdelay` or `slew` derivative
    /// names, so a lane the operand cannot reach has to be a value rather than
    /// an absence.
    fn or_zero_lanes(&mut self, derivative: Option<ValueId>, target: ShapeId) -> ValueId {
        match derivative {
            Some(value) => self.widen(value, target),
            None => self.splat(0.0, target),
        }
    }

    fn negate_lanes(&mut self, value: ValueId, target: ShapeId) -> ValueId {
        let widened = self.widen(value, target);
        let zero = self.splat(0.0, target);
        self.push(
            CfgValueType::Lanes(target),
            CfgValueKind::LaneBinary {
                op: CfgBinaryOp::Sub,
                left: zero,
                right: widened,
            },
        )
    }

    /// The input and delay derivatives of one `absdelay`, or `None` when
    /// neither operand moves with any lane of `target`.
    fn delayed_lane_derivatives(
        &mut self,
        input: ValueId,
        delay: ValueId,
        target: ShapeId,
    ) -> Option<(ValueId, ValueId)> {
        let input_derivative = self.derivatives[usize::from(input)];
        let delay_derivative = self.derivatives[usize::from(delay)];
        if input_derivative.is_none() && delay_derivative.is_none() {
            return None;
        }
        let input_derivative = self.or_zero_lanes(input_derivative, target);
        let delay_derivative = self.or_zero_lanes(delay_derivative, target);
        Some((input_derivative, delay_derivative))
    }

    /// The falling rate a `slew` node stands for, and whether it was built by
    /// negating the rising one.
    fn slew_falling_rate(
        &mut self,
        max_rise: ValueId,
        max_fall: Option<ValueId>,
    ) -> (ValueId, bool) {
        match max_fall {
            Some(max_fall) => (max_fall, false),
            None => (self.push_unary(CfgUnaryOp::Neg, max_rise), true),
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

#[cfg(test)]
mod tests {
    use super::{LaneLiveness, LaneLivenessStorage};
    use crate::canonical_ir::ValueId;
    use std::collections::HashSet;

    #[test]
    fn compact_lane_liveness_crosses_word_boundaries() {
        let first = ValueId::from(0);
        let second = ValueId::from(1);
        let third = ValueId::from(2);
        let mut live = LaneLiveness::new(3, 130);

        for lane in [0, 63, 64, 65, 127, 129] {
            assert!(live.insert(first, lane));
            assert!(!live.insert(first, lane));
        }
        for lane in [1, 64, 128] {
            live.insert(second, lane);
        }
        assert!(live.union_from(third, first));
        assert!(live.union_from(third, second));
        assert!(!live.union_from(third, first));

        assert_eq!(live.lanes(first), [0, 63, 64, 65, 127, 129]);
        assert_eq!(live.lanes(third), [0, 1, 63, 64, 65, 127, 128, 129]);
    }

    #[test]
    fn empty_lane_census_preserves_one_result_per_value() {
        let live = LaneLiveness::new(3, 0).into_hash_sets();
        assert_eq!(live, vec![HashSet::new(), HashSet::new(), HashSet::new()]);
    }

    #[test]
    fn pathological_dense_matrix_uses_sparse_storage() {
        let mut live = LaneLiveness::new(100, 10_000_000);
        assert!(matches!(live.storage, LaneLivenessStorage::Sparse(_)));
        let first = ValueId::from(0);
        let second = ValueId::from(1);
        assert!(live.insert(first, 9_999_999));
        assert!(live.union_from(second, first));
        assert_eq!(live.lanes(second), [9_999_999]);
    }
}
