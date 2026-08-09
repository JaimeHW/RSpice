//! Simplification passes over a [`CfgFunction`].
//!
//! Constant folding, algebraic identities, constant-exponent powers, common
//! subexpression elimination, and dead code. Individually unremarkable; the
//! reason they are here rather than left to the Rust compiler is source size.
//! Every value that survives to the emitter is a line of generated Rust, and
//! the models this backend exists for produce enough of them to hit file-size
//! limits before `rustc` ever sees them.
//!
//! ## What is deliberately not done
//!
//! `x * 0 → 0` and `x - x → 0` are not applied. They are wrong when `x` is NaN
//! or infinite, which in a compact model is not hypothetical: a Newton step can
//! walk a model into an overflow, and a simplification that quietly turns that
//! into a finite zero converts a diagnosable failure into a wrong answer.
//! Constant-only versions of the same expressions fold anyway.
//!
//! ## Powers
//!
//! `pow(x, k)` for a constant `k` becomes multiplication or `sqrt` where it can.
//! The corpus emits `.powf()` for exponents like 2 and 0.5 thousands of times
//! over, and a `powf` call is roughly twenty times a multiply.
//!
//! ## Elimination is dominator-scoped
//!
//! Two occurrences of the same expression may only be merged if one dominates
//! the other; merging across sibling arms of a conditional would hoist work into
//! a path that does not need it, which is the very thing the CFG level exists to
//! stop. Dominance is computed by the Cooper-Harvey-Kennedy iteration, which is
//! a few dozen lines and needs no dominator tree of its own.

use std::collections::HashMap;

use crate::metrics::{NoPipelineControl, PipelineCancelled, PipelineControl, PipelinePhase};

use super::cfg::{
    CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue,
    CfgValueKind, CfgValueType, is_leaf_kind,
};
use super::cfg_eval::{apply_binary, apply_unary};
use super::{BlockId, ShapeId, ValueId};

/// Keep the entries `kept` marks, positionally.
///
/// A position past the end of the mask is kept. The two are parallel by
/// construction — an edge carries one argument per parameter of the block it
/// enters — and `CfgFunction::validate` rejects a function where they are not,
/// so silently dropping a trailing argument here would turn a violation of that
/// invariant into a differently-shaped one.
fn retain_by<T>(kept: &[bool], entries: &mut Vec<T>) {
    let mut position = 0;
    entries.retain(|_| {
        let keep = kept.get(position).copied().unwrap_or(true);
        position += 1;
        keep
    });
}

/// How many times the passes are re-run before giving up on further gains.
///
/// Each pass exposes work for the others — folding creates constants that
/// enable identities, which create duplicates for elimination — but the gains
/// converge quickly, and an unbounded loop over a rewriting pass is a hang
/// waiting for the one input that oscillates.
const MAX_ROUNDS: usize = 8;

/// Simplify `function`, reporting where `outputs` ended up.
///
/// Values are renumbered, so ids taken beforehand do not survive; the returned
/// vector is the only correct way to keep hold of one.
pub fn optimize(function: &CfgFunction, outputs: &[ValueId]) -> (CfgFunction, Vec<ValueId>) {
    optimize_with_control(function, outputs, &NoPipelineControl)
        .expect("the no-op pipeline control cannot cancel")
}

pub(crate) fn optimize_with_control(
    function: &CfgFunction,
    outputs: &[ValueId],
    control: &dyn PipelineControl,
) -> Result<(CfgFunction, Vec<ValueId>), PipelineCancelled> {
    let (function, outputs, _) =
        optimize_with_control_and_tracking(function, outputs, &[], control)?;
    Ok((function, outputs))
}

pub(crate) fn optimize_with_tracking(
    function: &CfgFunction,
    outputs: &[ValueId],
    tracked: &[ValueId],
) -> (CfgFunction, Vec<ValueId>, Vec<Option<ValueId>>) {
    optimize_with_control_and_tracking(function, outputs, tracked, &NoPipelineControl)
        .expect("the no-op pipeline control cannot cancel")
}

/// Simplify `function` while following values that are not liveness roots.
///
/// A tracked value returns `None` when optimizing `outputs` made it dead, or
/// the surviving compacted id after folding and CSE redirected it. Tracking
/// must not keep candidates alive: doing so would defeat the source-size
/// optimization this analysis is intended to support.
pub(crate) fn optimize_with_control_and_tracking(
    function: &CfgFunction,
    outputs: &[ValueId],
    tracked: &[ValueId],
    control: &dyn PipelineControl,
) -> Result<(CfgFunction, Vec<ValueId>, Vec<Option<ValueId>>), PipelineCancelled> {
    check_cancelled(control)?;
    let mut optimizer = Optimizer {
        entry: function.entry,
        values: function.values.clone(),
        blocks: function.blocks.clone(),
        shapes: function.shapes.clone(),
        outputs: outputs.to_vec(),
        tracked: tracked.iter().copied().map(Some).collect(),
        replacement: vec![None; function.values.len()],
    };

    for _ in 0..MAX_ROUNDS {
        check_cancelled(control)?;
        let mut changed = optimizer.simplify(control)?;
        optimizer.apply_replacements();
        check_cancelled(control)?;
        changed |= optimizer.eliminate_common_subexpressions(control)?;
        optimizer.apply_replacements();
        if !changed {
            break;
        }
    }
    check_cancelled(control)?;
    optimizer.eliminate_dead_code();
    check_cancelled(control)?;

    Ok((
        CfgFunction {
            entry: optimizer.entry,
            blocks: optimizer.blocks,
            values: optimizer.values,
            shapes: optimizer.shapes,
        },
        optimizer.outputs,
        optimizer.tracked,
    ))
}

fn check_cancelled(control: &dyn PipelineControl) -> Result<(), PipelineCancelled> {
    if control.is_cancelled() {
        Err(PipelineCancelled {
            phase: PipelinePhase::CfgOptimization,
        })
    } else {
        Ok(())
    }
}

struct Optimizer {
    entry: BlockId,
    values: Vec<CfgValue>,
    blocks: Vec<CfgBlock>,
    /// Carried through unchanged: shape ids are referenced by value types, and
    /// renumbering values does not renumber shapes.
    shapes: Vec<Vec<u32>>,
    outputs: Vec<ValueId>,
    tracked: Vec<Option<ValueId>>,
    /// Where each value has been redirected, as a union-find parent list.
    replacement: Vec<Option<ValueId>>,
}

impl Optimizer {
    /// Fold constants, apply identities, and lower constant-exponent powers.
    ///
    /// Rebuilds each block's instruction list because a power lowering may need
    /// temporaries, which have to land before the instruction that reads them.
    fn simplify(&mut self, control: &dyn PipelineControl) -> Result<bool, PipelineCancelled> {
        let mut changed = false;
        for index in 0..self.blocks.len() {
            check_cancelled(control)?;
            let instructions = std::mem::take(&mut self.blocks[index].instructions);
            let mut rewritten = Vec::with_capacity(instructions.len());
            for (ordinal, instruction) in instructions.into_iter().enumerate() {
                if ordinal.is_multiple_of(1024) {
                    check_cancelled(control)?;
                }
                let result = instruction.result;
                if self.replacement[usize::from(result)].is_some() {
                    // Already redirected by an earlier round; the sweep drops it.
                    rewritten.push(instruction);
                    continue;
                }
                if let Some(folded) = self.fold(result) {
                    self.values[usize::from(result)].kind = folded;
                    changed = true;
                    rewritten.push(instruction);
                    continue;
                }
                if let Some(identical) = self.identity(result) {
                    self.replacement[usize::from(result)] = Some(identical);
                    changed = true;
                    rewritten.push(instruction);
                    continue;
                }
                if self.lower_power(result, &mut rewritten) {
                    changed = true;
                }
                rewritten.push(instruction);
            }
            self.blocks[index].instructions = rewritten;
        }

        // A value that simplified to a constant is a leaf now, and a leaf
        // belongs to no block.
        //
        // Two passes rewrite a value's kind in place — folding, and `powf(x,0)`
        // — and leaving the instruction where it was is what HiSIM-HV caught.
        // Common subexpression elimination treats a constant as available
        // everywhere, which is right, so a second occurrence of `151.0` in a
        // sibling arm gets merged onto the first. But the first is still
        // lexically inside a guarded region, so the emitted Rust names a
        // binding Rust cannot see from there. Swept here rather than at each
        // site so a future pass that folds something cannot reintroduce it.
        let Self { blocks, values, .. } = self;
        for block in blocks.iter_mut() {
            block
                .instructions
                .retain(|instruction| !is_leaf_kind(&values[usize::from(instruction.result)].kind));
        }
        Ok(changed)
    }

    /// The constant a value evaluates to, when every operand is one.
    fn fold(&self, value: ValueId) -> Option<CfgValueKind> {
        let boolean = self.values[usize::from(value)].value_type == CfgValueType::Boolean;
        let folded = match &self.values[usize::from(value)].kind {
            CfgValueKind::Unary { op, input } => apply_unary(*op, self.constant(*input)?),
            CfgValueKind::Binary { op, left, right } => {
                apply_binary(*op, self.constant(*left)?, self.constant(*right)?)
            }
            _ => return None,
        };
        // A folded predicate stays a predicate, so a later pass reading its type
        // does not see a real where a branch condition belongs.
        Some(if boolean {
            CfgValueKind::BooleanConstant(folded != 0.0)
        } else {
            CfgValueKind::RealConstant(folded)
        })
    }

    fn constant(&self, value: ValueId) -> Option<f64> {
        match self.values[usize::from(value)].kind {
            CfgValueKind::RealConstant(constant) => Some(constant),
            CfgValueKind::BooleanConstant(constant) => Some(f64::from(u8::from(constant))),
            _ => None,
        }
    }

    fn shape(&self, value: ValueId) -> Option<ShapeId> {
        self.values[usize::from(value)].value_type.shape()
    }

    fn is_splat(&self, value: ValueId, constant: f64) -> bool {
        matches!(
            self.values[usize::from(value)].kind,
            CfgValueKind::LaneSplat(held) if held == constant
        )
    }

    /// The operand a value is equal to, by an identity that holds for every
    /// input including NaN and infinity.
    fn identity(&self, value: ValueId) -> Option<ValueId> {
        let (op, left, right) = match self.values[usize::from(value)].kind {
            CfgValueKind::Binary { op, left, right } => (op, left, right),
            // A widen that changes nothing, which replacements can produce even
            // though the derivative pass never emits one.
            CfgValueKind::LaneWiden { input } => {
                return (self.shape(input) == self.shape(value)).then_some(input);
            }
            // The packed forms of the identities below. `x + 0` holds lane by
            // lane for exactly the reasons it holds for a scalar.
            CfgValueKind::LaneBinary { op, left, right } => {
                return match op {
                    CfgBinaryOp::Add if self.is_splat(left, 0.0) => Some(right),
                    CfgBinaryOp::Add | CfgBinaryOp::Sub if self.is_splat(right, 0.0) => Some(left),
                    _ => None,
                };
            }
            CfgValueKind::LaneScalar { op, input, scalar } => {
                let unit = self.constant(scalar) == Some(1.0)
                    && matches!(op, CfgBinaryOp::Mul | CfgBinaryOp::Div);
                return unit.then_some(input);
            }
            _ => return None,
        };
        let left_constant = self.constant(left);
        let right_constant = self.constant(right);
        match op {
            CfgBinaryOp::Add if right_constant == Some(0.0) => Some(left),
            CfgBinaryOp::Add if left_constant == Some(0.0) => Some(right),
            CfgBinaryOp::Sub if right_constant == Some(0.0) => Some(left),
            CfgBinaryOp::Mul if right_constant == Some(1.0) => Some(left),
            CfgBinaryOp::Mul if left_constant == Some(1.0) => Some(right),
            CfgBinaryOp::Div if right_constant == Some(1.0) => Some(left),
            CfgBinaryOp::Pow if right_constant == Some(1.0) => Some(left),
            _ => None,
        }
    }

    /// Rewrite `pow(x, k)` for a constant `k` into cheaper operations.
    ///
    /// The result keeps its own id, so nothing referring to it has to change;
    /// only the intermediate products are new, and they are appended before it.
    fn lower_power(&mut self, value: ValueId, emitted: &mut Vec<CfgInstruction>) -> bool {
        let CfgValueKind::Binary {
            op: CfgBinaryOp::Pow,
            left: base,
            right: exponent,
        } = self.values[usize::from(value)].kind
        else {
            return false;
        };
        let Some(exponent) = self.constant(exponent) else {
            return false;
        };

        // `powf(x, 0)` is 1 for every x, NaN included, so this is exact.
        if exponent == 0.0 {
            self.values[usize::from(value)].kind = CfgValueKind::RealConstant(1.0);
            return true;
        }
        if exponent == 0.5 {
            self.values[usize::from(value)].kind = CfgValueKind::Unary {
                op: CfgUnaryOp::Sqrt,
                input: base,
            };
            return true;
        }
        if exponent == -1.0 {
            let one = self.real_constant(1.0);
            self.values[usize::from(value)].kind = CfgValueKind::Binary {
                op: CfgBinaryOp::Div,
                left: one,
                right: base,
            };
            return true;
        }
        if exponent == 1.5 {
            let root = self.push_unary(emitted, CfgUnaryOp::Sqrt, base);
            self.values[usize::from(value)].kind = CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: base,
                right: root,
            };
            return true;
        }

        // Multiply chains only while they stay shorter than the call they
        // replace; past that a `powf` is both smaller and more accurate.
        let integral = exponent.round();
        if integral != exponent || !(2.0..=8.0).contains(&integral) {
            return false;
        }
        let count = integral as usize;
        let mut product = base;
        for _ in 1..count - 1 {
            product = self.push(emitted, CfgBinaryOp::Mul, product, base);
        }
        self.values[usize::from(value)].kind = CfgValueKind::Binary {
            op: CfgBinaryOp::Mul,
            left: product,
            right: base,
        };
        true
    }

    /// Merge values that compute the same thing, where one dominates the other.
    fn eliminate_common_subexpressions(
        &mut self,
        control: &dyn PipelineControl,
    ) -> Result<bool, PipelineCancelled> {
        let dominators = self.dominators();
        let order = reverse_postorder(&self.blocks, self.entry);

        let mut seen: HashMap<CseKey, Vec<(BlockId, ValueId)>> = HashMap::new();
        let mut changed = false;
        for block in order {
            check_cancelled(control)?;
            for (ordinal, instruction) in self.blocks[usize::from(block)]
                .instructions
                .iter()
                .enumerate()
            {
                if ordinal.is_multiple_of(1024) {
                    check_cancelled(control)?;
                }
                let result = instruction.result;
                if self.replacement[usize::from(result)].is_some() {
                    continue;
                }
                let Some(key) = self.cse_key(result) else {
                    continue;
                };
                let candidates = seen.entry(key).or_default();
                match candidates
                    .iter()
                    .find(|(defined_in, _)| dominates(&dominators, *defined_in, block))
                {
                    Some((_, existing)) => {
                        self.replacement[usize::from(result)] = Some(*existing);
                        changed = true;
                    }
                    None => candidates.push((block, result)),
                }
            }
        }
        Ok(changed)
    }

    /// A value's identity for elimination, or `None` if it must not be merged.
    ///
    /// Stateful kinds are excluded: two `ddt`s that look alike own different
    /// history slots, and merging them would make one of them read the other's
    /// past.
    fn cse_key(&self, value: ValueId) -> Option<CseKey> {
        // Packed keys carry the shape: two values built the same way over
        // different lane sets are different values, and merging them would move
        // a partial into another unknown's slot.
        let shape = self.shape(value);
        Some(match self.values[usize::from(value)].kind {
            CfgValueKind::RealConstant(constant) => CseKey::RealConstant(constant.to_bits()),
            CfgValueKind::BooleanConstant(constant) => CseKey::BooleanConstant(constant),
            CfgValueKind::Unary { op, input } => CseKey::Unary(op, input),
            CfgValueKind::Binary { op, left, right } => {
                // Commutative operands are ordered so `a + b` and `b + a` share
                // a key.
                if is_commutative(op) && usize::from(left) > usize::from(right) {
                    CseKey::Binary(op, right, left)
                } else {
                    CseKey::Binary(op, left, right)
                }
            }
            CfgValueKind::LaneSplat(constant) => CseKey::LaneSplat(constant.to_bits(), shape?),
            CfgValueKind::LaneWiden { input } => CseKey::LaneWiden(input, shape?),
            CfgValueKind::LaneBinary { op, left, right } => {
                if is_commutative(op) && usize::from(left) > usize::from(right) {
                    CseKey::LaneBinary(op, right, left, shape?)
                } else {
                    CseKey::LaneBinary(op, left, right, shape?)
                }
            }
            CfgValueKind::LaneScalar { op, input, scalar } => {
                CseKey::LaneScalar(op, input, scalar, shape?)
            }
            CfgValueKind::LaneExtract { input, lane } => CseKey::LaneExtract(input, lane),
            _ => return None,
        })
    }

    /// Immediate dominators, by the Cooper-Harvey-Kennedy iteration.
    fn dominators(&self) -> Vec<Option<BlockId>> {
        let order = reverse_postorder(&self.blocks, self.entry);
        let mut position = vec![usize::MAX; self.blocks.len()];
        for (index, block) in order.iter().enumerate() {
            position[usize::from(*block)] = index;
        }

        let mut predecessors: Vec<Vec<BlockId>> = vec![Vec::new(); self.blocks.len()];
        for block in &self.blocks {
            for successor in block.successors() {
                predecessors[usize::from(successor)].push(block.id);
            }
        }

        let mut idom: Vec<Option<BlockId>> = vec![None; self.blocks.len()];
        idom[usize::from(self.entry)] = Some(self.entry);
        loop {
            let mut changed = false;
            for block in &order {
                if *block == self.entry {
                    continue;
                }
                let mut new_idom: Option<BlockId> = None;
                for predecessor in &predecessors[usize::from(*block)] {
                    if idom[usize::from(*predecessor)].is_none() {
                        continue;
                    }
                    new_idom = Some(match new_idom {
                        Some(current) => {
                            intersect(&idom, &position, *predecessor, current, self.entry)
                        }
                        None => *predecessor,
                    });
                }
                if new_idom.is_some() && idom[usize::from(*block)] != new_idom {
                    idom[usize::from(*block)] = new_idom;
                    changed = true;
                }
            }
            if !changed {
                return idom;
            }
        }
    }

    /// Point every use at the value it was redirected to.
    fn apply_replacements(&mut self) {
        if self.replacement.iter().all(Option::is_none) {
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
        for tracked in self.tracked.iter_mut().flatten() {
            *tracked = translate(*tracked);
        }
        // Instructions whose result was redirected no longer define anything.
        for block in &mut self.blocks {
            block.instructions.retain(|instruction| {
                resolved[usize::from(instruction.result)] == instruction.result
            });
        }
        self.replacement = vec![None; self.values.len()];
    }

    fn resolve(&self, value: ValueId) -> ValueId {
        let mut current = value;
        // Bounded rather than recursive: a cycle in the replacement map would be
        // a bug, and hanging is a worse way to report it than settling.
        for _ in 0..self.values.len() {
            match self.replacement[usize::from(current)] {
                Some(next) if next != current => current = next,
                _ => return current,
            }
        }
        current
    }

    /// Drop values nothing reads — merges included — then renumber what is left.
    fn eliminate_dead_code(&mut self) {
        let mut live = vec![false; self.values.len()];
        let mut worklist: Vec<ValueId> = Vec::new();
        let mark = |id: ValueId, live: &mut Vec<bool>, worklist: &mut Vec<ValueId>| {
            if !live[usize::from(id)] {
                live[usize::from(id)] = true;
                worklist.push(id);
            }
        };

        for output in &self.outputs {
            mark(*output, &mut live, &mut worklist);
        }
        // A branch condition decides which edge is taken, so it is read whether
        // or not any value downstream of it is. Nothing else is live by being
        // written: a merge is live only when something reads it, and the
        // arguments on the edges into it are live only then too.
        for block in &self.blocks {
            if let CfgTerminator::Branch { condition, .. } = &block.terminator {
                mark(*condition, &mut live, &mut worklist);
            }
        }

        // Which merge a value is, so that reaching one can reach back through
        // the edges that supply it.
        let mut merges: HashMap<ValueId, (usize, usize)> = HashMap::new();
        for (block, data) in self.blocks.iter().enumerate() {
            for (position, param) in data.params.iter().enumerate() {
                merges.insert(*param, (block, position));
            }
        }

        let mut incoming = Vec::new();
        while let Some(id) = worklist.pop() {
            for operand in self.values[usize::from(id)].kind.operands() {
                mark(operand, &mut live, &mut worklist);
            }
            let Some((block, position)) = merges.get(&id).copied() else {
                continue;
            };
            incoming.clear();
            for source in &self.blocks {
                match &source.terminator {
                    CfgTerminator::Jump { target, args } if usize::from(*target) == block => {
                        incoming.extend(args.get(position).copied());
                    }
                    CfgTerminator::Branch {
                        then_target,
                        then_args,
                        else_target,
                        else_args,
                        ..
                    } => {
                        if usize::from(*then_target) == block {
                            incoming.extend(then_args.get(position).copied());
                        }
                        if usize::from(*else_target) == block {
                            incoming.extend(else_args.get(position).copied());
                        }
                    }
                    _ => {}
                }
            }
            for argument in incoming.drain(..) {
                mark(argument, &mut live, &mut worklist);
            }
        }

        // A merge and the edges into it are one thing: dropping the parameter
        // means dropping the argument at the same position on every edge, and
        // they are dropped together or not at all. Keeping them instead — which
        // is what this pass used to do, to avoid exactly this rewrite — holds a
        // residual alive in every slice cut from the function, along with
        // everything that feeds it.
        let kept: Vec<Vec<bool>> = self
            .blocks
            .iter()
            .map(|block| {
                block
                    .params
                    .iter()
                    .map(|param| live[usize::from(*param)])
                    .collect()
            })
            .collect();
        for (index, block) in self.blocks.iter_mut().enumerate() {
            retain_by(&kept[index], &mut block.params);
            match &mut block.terminator {
                CfgTerminator::Jump { target, args } => {
                    retain_by(&kept[usize::from(*target)], args)
                }
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => {
                    retain_by(&kept[usize::from(*then_target)], then_args);
                    retain_by(&kept[usize::from(*else_target)], else_args);
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }

        for block in &mut self.blocks {
            block
                .instructions
                .retain(|instruction| live[usize::from(instruction.result)]);
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
        let translate =
            |id: ValueId| remap[usize::from(id)].expect("a live value must survive compaction");

        for value in &mut compacted {
            value.kind.map_operands(translate);
        }
        for output in &mut self.outputs {
            *output = translate(*output);
        }
        for tracked in &mut self.tracked {
            *tracked = tracked.and_then(|value| remap[usize::from(value)]);
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

    fn real_constant(&mut self, constant: f64) -> ValueId {
        self.new_value(CfgValueType::Real, CfgValueKind::RealConstant(constant))
    }

    fn push(
        &mut self,
        emitted: &mut Vec<CfgInstruction>,
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> ValueId {
        let id = self.new_value(CfgValueType::Real, CfgValueKind::Binary { op, left, right });
        emitted.push(CfgInstruction { result: id });
        id
    }

    fn push_unary(
        &mut self,
        emitted: &mut Vec<CfgInstruction>,
        op: CfgUnaryOp,
        input: ValueId,
    ) -> ValueId {
        let id = self.new_value(CfgValueType::Real, CfgValueKind::Unary { op, input });
        emitted.push(CfgInstruction { result: id });
        id
    }

    fn new_value(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.values.len());
        self.values.push(CfgValue {
            id,
            value_type,
            kind,
        });
        self.replacement.push(None);
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CseKey {
    RealConstant(u64),
    BooleanConstant(bool),
    Unary(CfgUnaryOp, ValueId),
    Binary(CfgBinaryOp, ValueId, ValueId),
    LaneSplat(u64, ShapeId),
    LaneWiden(ValueId, ShapeId),
    LaneBinary(CfgBinaryOp, ValueId, ValueId, ShapeId),
    LaneScalar(CfgBinaryOp, ValueId, ValueId, ShapeId),
    LaneExtract(ValueId, u32),
}

fn is_commutative(op: CfgBinaryOp) -> bool {
    matches!(
        op,
        CfgBinaryOp::Add
            | CfgBinaryOp::Mul
            | CfgBinaryOp::Min
            | CfgBinaryOp::Max
            | CfgBinaryOp::Eq
            | CfgBinaryOp::Ne
            | CfgBinaryOp::And
            | CfgBinaryOp::Or
    )
}

fn reverse_postorder(blocks: &[CfgBlock], entry: BlockId) -> Vec<BlockId> {
    let mut visited = vec![false; blocks.len()];
    let mut postorder = Vec::with_capacity(blocks.len());
    // An explicit stack rather than recursion: a deeply nested model would
    // otherwise be limited by the host's stack rather than by anything real.
    let mut stack = vec![(entry, 0usize)];
    visited[usize::from(entry)] = true;
    while let Some((block, index)) = stack.pop() {
        let successors = blocks[usize::from(block)].successors();
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
    postorder
}

/// The nearest block that dominates both, walking up the partial tree.
fn intersect(
    idom: &[Option<BlockId>],
    position: &[usize],
    mut left: BlockId,
    mut right: BlockId,
    entry: BlockId,
) -> BlockId {
    while left != right {
        while position[usize::from(left)] > position[usize::from(right)] {
            match idom[usize::from(left)] {
                Some(next) if next != left => left = next,
                _ => return entry,
            }
        }
        while position[usize::from(right)] > position[usize::from(left)] {
            match idom[usize::from(right)] {
                Some(next) if next != right => right = next,
                _ => return entry,
            }
        }
    }
    left
}

fn dominates(idom: &[Option<BlockId>], candidate: BlockId, block: BlockId) -> bool {
    let mut current = block;
    loop {
        if current == candidate {
            return true;
        }
        match idom[usize::from(current)] {
            Some(next) if next != current => current = next,
            _ => return false,
        }
    }
}
