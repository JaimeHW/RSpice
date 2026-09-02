//! Typed, effect-annotated SSA consumed by every native machine backend.
//!
//! Architecture-neutral lowering currently delivers postfix `NativeOp`s.  The
//! native backends immediately lift that stream into explicit values so machine
//! code generation never has to trust an implicit operand stack.  Expressions
//! are single-block today; keeping the block and terminator explicit makes
//! control-flow extension possible without changing the value model.
//!
//! The block model here is complete — typed block parameters, `Jump` and
//! `Branch` terminators, a verifier, and three backends that emit real
//! branches for it — but nothing outside the tests that qualify it builds a
//! multi-block program yet. The shipped route is still the postfix lift and
//! its select form of a conditional, so the items marked
//! `cfg_attr(not(test), allow(dead_code))` below are the ones whose only
//! constructor today is a test. W-C3 (digital process control flow) and
//! W-D/W-F (the flip) are what give them a shipped one.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

#[cfg(test)]
use crate::jit::expr::CompareOp;
use crate::jit::expr::{
    IntegerBinaryOp, NativeOp, NativeProgram, UnaryMathOp, native_op_stack_effect,
};
use crate::jit::value_cache::{native_op_hash, native_ops_are_codegen_identical};
use crate::jit::{JitError, JitResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

const MODEL: &str = "native-ssa";

/// Hash a value through its `Debug` rendering.
///
/// Used for the few key components that are exhaustively enumerated but not
/// `Hash` — effect sets, value types, terminators. Their `Debug` output is
/// derived and therefore total: two values render the same string exactly when
/// they are equal.
fn hash_debug<H: Hasher>(value: &impl Debug, hasher: &mut H) {
    use std::fmt::Write as _;

    struct HashWriter<'a, H: Hasher>(&'a mut H);

    impl<H: Hasher> std::fmt::Write for HashWriter<'_, H> {
        fn write_str(&mut self, value: &str) -> std::fmt::Result {
            self.0.write(value.as_bytes());
            Ok(())
        }
    }

    write!(&mut HashWriter(hasher), "{value:?};").expect("hash writer cannot fail");
}
pub(crate) const INLINE_DYNAMIC_LOWER_ABS_LIMIT: i64 = 1_i64 << 51;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ValueId(u32);

impl ValueId {
    fn new(index: usize) -> JitResult<Self> {
        u32::try_from(index)
            .map(Self)
            .map_err(|_| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA value count {index} exceeds the u32 identity space").into(),
            })
    }

    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValueType {
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct BlockId(u32);

impl BlockId {
    pub(crate) fn new(index: usize) -> JitResult<Self> {
        u32::try_from(index)
            .map(Self)
            .map_err(|_| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA block count {index} exceeds the u32 identity space").into(),
            })
    }

    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}

/// One typed value a block receives from each of its predecessors.
///
/// Block parameters, not phi nodes: the CFG level already carries typed SSA
/// block parameters, and an edge that names its arguments keeps the merge
/// explicit at the one place a backend has to realize it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockParameter {
    value: ValueId,
    value_type: ValueType,
}

#[cfg_attr(not(test), allow(dead_code))]
impl BlockParameter {
    pub(crate) fn value(self) -> ValueId {
        self.value
    }
}

/// One control-flow edge and the arguments it binds to its target's parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Edge {
    target: BlockId,
    arguments: Box<[ValueId]>,
}

#[cfg_attr(not(test), allow(dead_code))]
impl Edge {
    pub(crate) fn new(target: BlockId, arguments: impl Into<Box<[ValueId]>>) -> Self {
        Self {
            target,
            arguments: arguments.into(),
        }
    }

    pub(crate) fn target(&self) -> BlockId {
        self.target
    }

    /// Read by the WebAssembly backend, which binds an edge structurally.
    #[cfg_attr(not(feature = "wasm-jit"), allow(dead_code))]
    pub(crate) fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum Terminator {
    Return(ValueId),
    Jump(Edge),
    Branch {
        condition: ValueId,
        then_edge: Edge,
        else_edge: Edge,
    },
}

impl Terminator {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn edge_count(&self) -> usize {
        match self {
            Self::Return(_) => 0,
            Self::Jump(_) => 1,
            Self::Branch { .. } => 2,
        }
    }

    fn edges(&self) -> impl Iterator<Item = &Edge> {
        match self {
            Self::Return(_) => [None, None],
            Self::Jump(edge) => [Some(edge), None],
            Self::Branch {
                then_edge,
                else_edge,
                ..
            } => [Some(then_edge), Some(else_edge)],
        }
        .into_iter()
        .flatten()
    }

    /// Every value this terminator reads, in a fixed order.
    fn uses(&self) -> Vec<ValueId> {
        match self {
            Self::Return(value) => vec![*value],
            Self::Jump(edge) => edge.arguments.to_vec(),
            Self::Branch {
                condition,
                then_edge,
                else_edge,
            } => {
                let mut uses = vec![*condition];
                uses.extend(then_edge.arguments.iter().copied());
                uses.extend(else_edge.arguments.iter().copied());
                uses
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Effects(u16);

impl Effects {
    const READ_CONTEXT: u16 = 1 << 0;
    const READ_STATE: u16 = 1 << 1;
    const WRITE_STATE: u16 = 1 << 2;
    const MAY_CALL: u16 = 1 << 3;
    const MAY_FAIL: u16 = 1 << 4;
    const READ_ENTRY_ARGS: u16 = 1 << 5;
    const CLOBBER_CONTEXT_CACHE: u16 = 1 << 6;
    const INTERNAL_CALL_CONTINUATION: u16 = 1 << 7;

    pub(crate) fn for_op(op: NativeOp) -> Self {
        let mut bits = 0;
        if op_reads_context(op) {
            bits |= Self::READ_CONTEXT;
        }
        if op_reads_state(op) {
            bits |= Self::READ_STATE;
        }
        if op_writes_state(op) {
            bits |= Self::WRITE_STATE;
        }
        if op_may_call(op) {
            bits |= Self::MAY_CALL;
        }
        if op_may_fail(op) {
            bits |= Self::MAY_FAIL;
        }
        if op_reads_entry_args(op) {
            bits |= Self::READ_ENTRY_ARGS;
        }
        if !op_preserves_context_pointer_cache(op) {
            bits |= Self::CLOBBER_CONTEXT_CACHE;
        }
        if matches!(op, NativeOp::IdtModState(_)) {
            bits |= Self::INTERNAL_CALL_CONTINUATION;
        }
        Self(bits)
    }

    pub(crate) fn may_call(self) -> bool {
        self.contains(Self::MAY_CALL)
    }

    pub(crate) fn may_fail(self) -> bool {
        self.contains(Self::MAY_FAIL)
    }

    pub(crate) fn writes_state(self) -> bool {
        self.contains(Self::WRITE_STATE)
    }

    pub(crate) fn reads_entry_args(self) -> bool {
        self.contains(Self::READ_ENTRY_ARGS)
    }

    pub(crate) fn clobbers_context_pointer_cache(self) -> bool {
        self.contains(Self::CLOBBER_CONTEXT_CACHE)
    }

    pub(crate) fn needs_saved_entry_args_for_internal_continuation(self) -> bool {
        self.contains(Self::INTERNAL_CALL_CONTINUATION)
    }

    /// Whether evaluating this instruction once may replace identical repeated
    /// evaluations at the same program point.
    ///
    /// Context reads are deliberately allowed: the shared-output planner only
    /// coalesces adjacent Jacobian programs, and publishing the first result
    /// cannot mutate the evaluation context.  Calls, failures, and state writes
    /// remain barriers even when a particular helper is expected to be pure;
    /// that keeps sharing correct as runtime helper contracts evolve.
    fn permits_result_sharing(self) -> bool {
        self.0 & (Self::WRITE_STATE | Self::MAY_CALL | Self::MAY_FAIL) == 0
    }

    #[cfg(test)]
    fn is_semantically_pure(self) -> bool {
        self.0 & (Self::READ_CONTEXT | Self::READ_STATE | Self::WRITE_STATE | Self::MAY_FAIL) == 0
    }

    fn contains(self, flag: u16) -> bool {
        self.0 & flag != 0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Instruction {
    result: ValueId,
    value_type: ValueType,
    op: NativeOp,
    operands: Box<[ValueId]>,
    effects: Effects,
}

impl Instruction {
    pub(crate) fn result(&self) -> ValueId {
        self.result
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub(crate) fn op(&self) -> NativeOp {
        self.op
    }

    pub(crate) fn operands(&self) -> &[ValueId] {
        &self.operands
    }

    pub(crate) fn effects(&self) -> Effects {
        self.effects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BasicBlock {
    id: BlockId,
    parameter_start: usize,
    parameter_end: usize,
    instruction_start: usize,
    instruction_end: usize,
    terminator: Terminator,
}

impl BasicBlock {
    pub(crate) fn id(&self) -> BlockId {
        self.id
    }

    pub(crate) fn instruction_start(&self) -> usize {
        self.instruction_start
    }

    pub(crate) fn instruction_end(&self) -> usize {
        self.instruction_end
    }

    pub(crate) fn terminator(&self) -> &Terminator {
        &self.terminator
    }
}

/// A typed SSA program over one or more basic blocks.
///
/// Two identity rules make every consumer's indexing total and let a
/// single-block program stay byte-for-byte what the postfix lift always
/// produced:
///
/// * an instruction's [`ValueId`] is its index in the flat `instructions`
///   vector, and every block owns a contiguous, increasing instruction range,
///   so the flat vector is also the block layout order;
/// * a block parameter's [`ValueId`] is `instructions.len()` plus its index in
///   the flat `block_parameters` vector, so a program with no parameters
///   numbers values exactly as it did before block parameters existed.
///
/// The verifier ([`Program::validate`]) is the authority on the rest: exactly
/// one `Return`, a reachable acyclic CFG, edge arity and type agreement,
/// dominance of every use by its definition, and the rule that keeps state
/// writes unconditional.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Program {
    entry: BlockId,
    exit: BlockId,
    blocks: Vec<BasicBlock>,
    block_parameters: Vec<BlockParameter>,
    instructions: Vec<Instruction>,
    maximum_stack_depth: usize,
}

impl Program {
    pub(crate) fn lower(program: &NativeProgram) -> JitResult<Self> {
        let mut lowerer = ProgramLowerer::with_capacity(program.ops().len());
        let (result, maximum_stack_depth) = lowerer.append(program)?;
        eliminate_dead_instructions(lowerer.finish(result, maximum_stack_depth)?, &mut [])
    }

    fn single_block(
        instructions: Vec<Instruction>,
        result: ValueId,
        maximum_stack_depth: usize,
    ) -> JitResult<Self> {
        let entry = BlockId(0);
        let program = Self {
            entry,
            exit: entry,
            blocks: vec![BasicBlock {
                id: entry,
                parameter_start: 0,
                parameter_end: 0,
                instruction_start: 0,
                instruction_end: instructions.len(),
                terminator: Terminator::Return(result),
            }],
            block_parameters: Vec::new(),
            instructions,
            maximum_stack_depth,
        };
        program.validate()?;
        Ok(program)
    }

    /// The loop fixture all three backends' tests share.
    ///
    /// One program that exercises every part of loop support at once, so a
    /// backend that gets any of it wrong produces a wrong number rather than
    /// merely a different encoding:
    ///
    /// ```text
    /// k = 3.0; i = 0; a = param0; b = param1;
    /// while (i < limit) { i, a, b = i + (a + b) * k, b, a }
    /// return a * i;
    /// ```
    ///
    /// * the back edge itself, and a header whose three parameters are all
    ///   loop-carried;
    /// * a genuine permutation cycle on that edge — `a` and `b` exchange
    ///   locations, which no forward edge can produce and which a naive
    ///   sequence of moves loses;
    /// * `k`, defined before the loop and read in the middle of the body,
    ///   whose location must survive to the next iteration even though a
    ///   linear scan sees its last use pass;
    /// * header parameters read *after* the loop, in the exit block.
    #[cfg(test)]
    pub(crate) fn loop_fixture_for_test(limit: f64, k: f64) -> JitResult<Self> {
        let mut builder =
            ProgramBuilder::new(&[Vec::new(), vec![ValueType::F64; 3], Vec::new(), Vec::new()])?;
        let entry = BlockId::new(0)?;
        let header = BlockId::new(1)?;
        let body = BlockId::new(2)?;
        let exit = BlockId::new(3)?;

        builder.begin_block(entry)?;
        let scale = builder.push(NativeOp::Const(k), &[], ValueType::F64)?;
        let zero = builder.push(NativeOp::Const(0.0), &[], ValueType::F64)?;
        let first = builder.push(NativeOp::LoadParam(0), &[], ValueType::F64)?;
        let second = builder.push(NativeOp::LoadParam(1), &[], ValueType::F64)?;
        builder.end_block(BuilderTerminator::Jump {
            target: header,
            arguments: vec![zero, first, second],
        })?;

        let counter = builder.parameter(header, 0)?;
        let left = builder.parameter(header, 1)?;
        let right = builder.parameter(header, 2)?;
        builder.begin_block(header)?;
        let test = builder.push(
            NativeOp::CompareConst(CompareOp::Lt, limit),
            &[counter],
            ValueType::F64,
        )?;
        builder.end_block(BuilderTerminator::Branch {
            condition: test,
            then_target: body,
            then_arguments: Vec::new(),
            else_target: exit,
            else_arguments: Vec::new(),
        })?;

        builder.begin_block(body)?;
        let sum = builder.push(NativeOp::Add, &[left, right], ValueType::F64)?;
        let step = builder.push(NativeOp::Mul, &[sum, scale], ValueType::F64)?;
        let advanced = builder.push(NativeOp::Add, &[counter, step], ValueType::F64)?;
        builder.end_block(BuilderTerminator::Jump {
            target: header,
            arguments: vec![advanced, right, left],
        })?;

        builder.begin_block(exit)?;
        let result = builder.push(NativeOp::Mul, &[left, counter], ValueType::F64)?;
        builder.end_block(BuilderTerminator::Return(result))?;
        builder.finish(entry, exit)
    }

    /// What [`Self::loop_fixture_for_test`] must compute.
    #[cfg(test)]
    pub(crate) fn loop_fixture_expectation(limit: f64, k: f64, first: f64, second: f64) -> f64 {
        let mut counter = 0.0_f64;
        let mut left = first;
        let mut right = second;
        while counter < limit {
            let advanced = counter + (left + right) * k;
            let (next_left, next_right) = (right, left);
            counter = advanced;
            left = next_left;
            right = next_right;
        }
        left * counter
    }

    #[cfg(all(test, target_arch = "x86_64"))]
    pub(crate) fn from_ssa_for_test(
        operations: Vec<(NativeOp, Vec<usize>)>,
        result: usize,
    ) -> JitResult<Self> {
        let instructions = operations
            .into_iter()
            .enumerate()
            .map(|(index, (op, operands))| {
                Ok(Instruction {
                    result: ValueId::new(index)?,
                    value_type: ValueType::F64,
                    op,
                    operands: operands
                        .into_iter()
                        .map(ValueId::new)
                        .collect::<JitResult<Vec<_>>>()?
                        .into_boxed_slice(),
                    effects: Effects::for_op(op),
                })
            })
            .collect::<JitResult<Vec<_>>>()?;
        Self::single_block(instructions, ValueId::new(result)?, 1)
    }

    pub(crate) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(crate) fn blocks(&self) -> &[BasicBlock] {
        &self.blocks
    }

    #[cfg_attr(not(feature = "wasm-jit"), allow(dead_code))]
    pub(crate) fn entry(&self) -> BlockId {
        self.entry
    }

    pub(crate) fn is_single_block(&self) -> bool {
        self.blocks.len() == 1
    }

    /// The natural loops of a program the verifier has already admitted.
    ///
    /// Recomputed rather than cached on the program: it is one dominator pass
    /// over the block list, the two consumers each need it once, and a cached
    /// field would be a second authority on a question `validate` answers.
    pub(crate) fn loop_ranges(&self) -> JitResult<Vec<LoopRange>> {
        if self.is_single_block() {
            return Ok(Vec::new());
        }
        let predecessors = self.predecessors();
        let idom = self.immediate_dominators(&predecessors);
        self.natural_loops(&DominatorTree::new(&idom, self.entry), &predecessors)
    }

    #[cfg_attr(not(feature = "wasm-jit"), allow(dead_code))]
    pub(crate) fn block(&self, id: BlockId) -> JitResult<&BasicBlock> {
        self.blocks
            .get(id.index())
            .ok_or_else(|| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA block {} is not defined", id.index()).into(),
            })
    }

    pub(crate) fn parameters(&self, block: &BasicBlock) -> &[BlockParameter] {
        &self.block_parameters[block.parameter_start..block.parameter_end]
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn block_parameter_count(&self) -> usize {
        self.block_parameters.len()
    }

    /// Total number of distinct SSA values: instruction results then block
    /// parameters, in that identity order.
    pub(crate) fn value_count(&self) -> usize {
        self.instructions.len() + self.block_parameters.len()
    }

    pub(crate) fn result(&self) -> ValueId {
        match &self.blocks[self.exit.index()].terminator {
            Terminator::Return(value) => *value,
            // `validate` admits a program only when `exit` names its single
            // `Return`, and every constructor validates before publishing.
            Terminator::Jump(_) | Terminator::Branch { .. } => {
                unreachable!("validated SSA exit block terminates with Return")
            }
        }
    }

    pub(crate) fn maximum_stack_depth(&self) -> usize {
        self.maximum_stack_depth
    }

    /// Feed this program's complete code-generation identity to `hasher`.
    ///
    /// A flat postfix program is keyed by its operation sequence because that
    /// sequence is the whole of it. A block program is not: two programs can
    /// carry the same instructions in the same order and still compile to
    /// different code because a terminator sends control somewhere else, or
    /// because a merge binds different arguments to its block parameters. So
    /// the key walks every field the emitters read — blocks in layout order,
    /// each block's parameters, its instructions, and its terminator — and
    /// hashes operations through [`native_op_hash`], which is what keeps
    /// `+0.0` and `-0.0` apart.
    ///
    /// Paired with [`Self::is_codegen_identical_to`]: the hash selects a
    /// bucket and that comparison decides membership, so the hash may collide
    /// but must never separate two programs the comparison calls identical.
    pub(crate) fn codegen_identity_hash<H: Hasher>(&self, hasher: &mut H) {
        self.entry.hash(hasher);
        self.exit.hash(hasher);
        self.maximum_stack_depth.hash(hasher);
        self.blocks.len().hash(hasher);
        for block in &self.blocks {
            block.id.hash(hasher);
            for parameter in &self.block_parameters[block.parameter_start..block.parameter_end] {
                parameter.value.hash(hasher);
                hash_debug(&parameter.value_type, hasher);
            }
            hash_debug(&block.terminator, hasher);
            for instruction in &self.instructions[block.instruction_start..block.instruction_end] {
                instruction.result.hash(hasher);
                hash_debug(&instruction.value_type, hasher);
                hash_debug(&instruction.effects, hasher);
                native_op_hash(
                    instruction.op,
                    instruction.operands.iter().map(|operand| operand.index()),
                )
                .hash(hasher);
            }
        }
    }

    /// Whether two block programs compile to the same code.
    ///
    /// Structural equality with one deliberate departure from the derived
    /// `PartialEq`: operations compare through
    /// [`native_ops_are_codegen_identical`], so two programs that differ only
    /// in the *bits* of a constant are different here even where `f64`
    /// equality would call them the same, and two carrying the same NaN
    /// payload are the same even where it would not.
    pub(crate) fn is_codegen_identical_to(&self, other: &Self) -> bool {
        if self.entry != other.entry
            || self.exit != other.exit
            || self.maximum_stack_depth != other.maximum_stack_depth
            || self.blocks.len() != other.blocks.len()
            || self.block_parameters != other.block_parameters
            || self.instructions.len() != other.instructions.len()
        {
            return false;
        }
        if self
            .blocks
            .iter()
            .zip(&other.blocks)
            .any(|(left, right)| left != right)
        {
            return false;
        }
        self.instructions
            .iter()
            .zip(&other.instructions)
            .all(|(left, right)| {
                left.result == right.result
                    && left.value_type == right.value_type
                    && left.effects == right.effects
                    && left.operands == right.operands
                    && native_ops_are_codegen_identical(left.op, right.op)
            })
    }

    pub(crate) fn uses_helper_calls(&self) -> bool {
        self.instructions
            .iter()
            .any(|instruction| instruction.effects.may_call())
    }

    pub(crate) fn requires_call_frame(&self) -> bool {
        self.instructions
            .iter()
            .any(|instruction| instruction.effects.may_call() || instruction.effects.may_fail())
    }

    pub(crate) fn needs_saved_entry_args(&self) -> bool {
        let mut helper_seen = false;
        for instruction in &self.instructions {
            let effects = instruction.effects;
            if effects.needs_saved_entry_args_for_internal_continuation()
                || (helper_seen && effects.reads_entry_args())
            {
                return true;
            }
            helper_seen |= effects.may_call();
        }
        false
    }

    fn permits_result_sharing(&self) -> bool {
        self.instructions
            .iter()
            .all(|instruction| instruction.effects.permits_result_sharing())
    }

    /// For every block parameter, the instruction boundary at which a backend
    /// first writes it: the earliest predecessor terminator in layout order.
    fn parameter_definition_positions(&self) -> JitResult<Vec<usize>> {
        let mut positions = vec![None; self.block_parameters.len()];
        for block in &self.blocks {
            for edge in block.terminator.edges() {
                let target = &self.blocks[edge.target.index()];
                for position in &mut positions[target.parameter_start..target.parameter_end] {
                    *position = Some(
                        position
                            .unwrap_or(block.instruction_end)
                            .min(block.instruction_end),
                    );
                }
            }
        }
        positions
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                position.ok_or_else(|| JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!("SSA block parameter {index} has no binding predecessor")
                        .into(),
                })
            })
            .collect()
    }

    fn value_type_of(&self, value: ValueId) -> JitResult<ValueType> {
        if let Some(instruction) = self.instructions.get(value.index()) {
            return Ok(instruction.value_type);
        }
        self.block_parameters
            .get(value.index() - self.instructions.len())
            .map(|parameter| parameter.value_type)
            .ok_or_else(|| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA value {} is not defined", value.index()).into(),
            })
    }

    fn predecessors(&self) -> Vec<Vec<BlockId>> {
        let mut predecessors = vec![Vec::new(); self.blocks.len()];
        for block in &self.blocks {
            for edge in block.terminator.edges() {
                if let Some(entry) = predecessors.get_mut(edge.target.index())
                    && !entry.contains(&block.id)
                {
                    entry.push(block.id);
                }
            }
        }
        predecessors
    }

    /// Immediate dominators, indexed by block, computed over the layout order.
    ///
    /// Layout order is a topological order of the acyclic CFG the verifier
    /// admits, so one forward pass reaches the fixed point.
    fn immediate_dominators(&self, predecessors: &[Vec<BlockId>]) -> Vec<Option<BlockId>> {
        let mut idom: Vec<Option<BlockId>> = vec![None; self.blocks.len()];
        let mut processed = vec![false; self.blocks.len()];
        processed[self.entry.index()] = true;
        for index in 0..self.blocks.len() {
            if index == self.entry.index() {
                continue;
            }
            let mut candidate: Option<BlockId> = None;
            for predecessor in predecessors[index].iter().copied() {
                if !processed[predecessor.index()] {
                    continue;
                }
                candidate = Some(match candidate {
                    None => predecessor,
                    Some(current) => {
                        Self::common_dominator(&idom, self.entry, current, predecessor)
                    }
                });
            }
            idom[index] = candidate;
            processed[index] = candidate.is_some();
        }
        idom
    }

    fn common_dominator(
        idom: &[Option<BlockId>],
        entry: BlockId,
        left: BlockId,
        right: BlockId,
    ) -> BlockId {
        let mut left = left;
        let mut right = right;
        while left != right {
            while left > right {
                match idom[left.index()] {
                    Some(parent) => left = parent,
                    None => return entry,
                }
            }
            while right > left {
                match idom[right.index()] {
                    Some(parent) => right = parent,
                    None => return entry,
                }
            }
        }
        left
    }

    fn validate(&self) -> JitResult<()> {
        self.validate_layout()?;
        if self.is_single_block() {
            // One block is its own entry and exit, has no edges and no
            // parameters, and dominates everything, so every rule below is
            // vacuous on it. What remains is the operand index rule the
            // postfix lift has always been held to, and keeping this path
            // allocation-free keeps the shipped route's verification cost
            // exactly what it was.
            return self.validate_straight_line();
        }
        let predecessors = self.predecessors_checked()?;
        self.validate_edges()?;
        let idom = self.immediate_dominators(&predecessors);
        let dominance = DominatorTree::new(&idom, self.entry);
        let loops = self.natural_loops(&dominance, &predecessors)?;
        self.validate_reaches_exit(&predecessors)?;
        self.validate_dominance(&dominance)?;
        self.validate_structured_regions(&dominance)?;
        self.validate_effect_discipline(&dominance, &loops)
    }

    /// The natural loops of this program, one per back edge, as contiguous
    /// layout ranges.
    ///
    /// A back edge is an edge whose target does not follow its source in
    /// layout order. Admitting one is the whole of loop support, and it is
    /// admitted on exactly two conditions.
    ///
    /// *Reducibility.* The target must dominate the source, which is what
    /// makes the edge the latch of a natural loop rather than a second entry
    /// into the middle of one. Verilog-A's `for`, `while` and `repeat` are
    /// reducible by construction — the language has no `goto` and no `break` —
    /// so an irreducible graph reaching here is a lowering bug, and it is
    /// refused by name rather than structurized.
    ///
    /// *Contiguity.* The loop's blocks — the header, plus everything that
    /// reaches the latch without passing back through the header — must be
    /// exactly the layout range from header to latch, with nothing else inside
    /// it. Two consumers depend on that: the allocator extends live intervals
    /// across a range, and the WebAssembly structurizer emits one `loop`
    /// around a range and decides which of the header's two edges leaves the
    /// loop by asking whether its target is inside. A layout that interleaves
    /// a loop with blocks outside it is refused here rather than silently
    /// mis-structured downstream.
    fn natural_loops(
        &self,
        dominance: &DominatorTree,
        predecessors: &[Vec<BlockId>],
    ) -> JitResult<Vec<LoopRange>> {
        let verifier = |detail: String| JitError::Verifier {
            model: MODEL.into(),
            detail: detail.into(),
        };
        let mut loops: Vec<LoopRange> = Vec::new();
        for block in &self.blocks {
            for edge in block.terminator.edges() {
                if edge.target > block.id {
                    continue;
                }
                if !dominance.dominates(edge.target, block.id) {
                    return Err(verifier(format!(
                        "SSA block {} has a back edge to block {}, which does not dominate it; the control-flow graph is irreducible",
                        block.id.index(),
                        edge.target.index()
                    )));
                }
                let body = self.natural_loop_body(edge.target, block.id, predecessors);
                let range = edge.target.index()..=block.id.index();
                if let Some(stranger) = body.iter().enumerate().find_map(|(index, member)| {
                    (*member != range.contains(&index)).then_some(index)
                }) {
                    return Err(verifier(format!(
                        "SSA block {stranger} puts the loop headed by block {} out of step with its layout range, which ends at its latch, block {}; the loop is not laid out contiguously",
                        edge.target.index(),
                        block.id.index()
                    )));
                }
                loops.push(LoopRange {
                    header: edge.target,
                    latch: block.id,
                });
            }
        }
        Ok(loops)
    }

    /// Membership in the natural loop closed by `latch -> header`.
    fn natural_loop_body(
        &self,
        header: BlockId,
        latch: BlockId,
        predecessors: &[Vec<BlockId>],
    ) -> Vec<bool> {
        let mut member = vec![false; self.blocks.len()];
        member[header.index()] = true;
        let mut work = Vec::new();
        if latch != header {
            member[latch.index()] = true;
            work.push(latch);
        }
        while let Some(block) = work.pop() {
            for predecessor in predecessors[block.index()].iter().copied() {
                if !member[predecessor.index()] {
                    member[predecessor.index()] = true;
                    work.push(predecessor);
                }
            }
        }
        member
    }

    /// The whole verification for a program with one block.
    fn validate_straight_line(&self) -> JitResult<()> {
        for (index, instruction) in self.instructions.iter().enumerate() {
            if instruction
                .operands
                .iter()
                .any(|operand| operand.index() >= index)
            {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!("SSA instruction {index} uses a non-dominating value").into(),
                });
            }
        }
        if self
            .instructions
            .get(self.result().index())
            .is_none_or(|instruction| instruction.value_type != ValueType::F64)
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA return does not reference a defined f64 value".into(),
            });
        }
        Ok(())
    }

    /// Which block defines each value, indexed by [`ValueId`].
    ///
    /// Built once per verification: walking the block list for every operand
    /// turns a program with one block per conditional arm into a quadratic
    /// check, and a compact model splits thousands of conditionals.
    fn definition_blocks(&self) -> Vec<BlockId> {
        let mut blocks = vec![BlockId(0); self.value_count()];
        let parameter_base = self.instructions.len();
        for block in &self.blocks {
            blocks[block.instruction_start..block.instruction_end].fill(block.id);
            blocks[parameter_base + block.parameter_start..parameter_base + block.parameter_end]
                .fill(block.id);
        }
        blocks
    }

    /// The block at which a two-way branch's arms reconverge.
    ///
    /// Every edge runs forward in layout order, so the reconvergence point is
    /// the first block index past which nothing inside the region still
    /// branches: start at the furthest successor and extend the watermark
    /// while any block before it reaches further.
    pub(crate) fn branch_join(&self, block: &BasicBlock) -> JitResult<BlockId> {
        let furthest = |block: &BasicBlock| {
            block
                .terminator
                .edges()
                .map(|edge| edge.target.index())
                .max()
        };
        let mut watermark = furthest(block).ok_or_else(|| JitError::Verifier {
            model: MODEL.into(),
            detail: format!("SSA block {} has no successor to join", block.id.index()).into(),
        })?;
        let mut cursor = block.id.index() + 1;
        while cursor < watermark {
            if let Some(target) = furthest(&self.blocks[cursor]) {
                watermark = watermark.max(target);
            } else {
                // A `Return` inside the region: the arms cannot reconverge.
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!(
                        "SSA block {} returns inside the region branching from block {}",
                        cursor,
                        block.id.index()
                    )
                    .into(),
                });
            }
            cursor += 1;
        }
        BlockId::new(watermark)
    }

    /// Branch arms must form single-entry regions that reconverge.
    ///
    /// WebAssembly has no `goto`: its only control flow is nested blocks,
    /// loops and `br_if`. Requiring the CFG to be structured is what lets the
    /// WebAssembly backend emit `if`/`else`/`end` directly instead of carrying
    /// a relooper, and it costs nothing, because structured Verilog-A control
    /// flow is the only thing that lowers here. The check is that nothing
    /// outside a branch's region enters it, which is exactly that every block
    /// the branch reaches before its join is dominated by the branch: a block
    /// with a predecessor outside the region could not be dominated by the
    /// block the region starts at.
    fn validate_structured_regions(&self, dominance: &DominatorTree) -> JitResult<()> {
        // The region is what the branch *reaches* before its join, not what
        // the layout happens to place between the two. The distinction is
        // real: the CFG level splices out empty blocks, which collapses an
        // inner conditional's join into the enclosing one, and the enclosing
        // conditional's other arm then sits between the inner branch and the
        // join they now share. That arm is not in the inner branch's region,
        // and the structural walk never enters it from there.
        let mut visited = vec![u32::MAX; self.blocks.len()];
        let mut stack: Vec<BlockId> = Vec::new();
        for (stamp, block) in self.blocks.iter().enumerate() {
            if !matches!(block.terminator, Terminator::Branch { .. }) {
                continue;
            }
            let stamp = u32::try_from(stamp).map_err(|_| JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA block count exceeds the region-walk stamp space".into(),
            })?;
            let join = self.branch_join(block)?;
            stack.clear();
            stack.extend(block.terminator.edges().map(|edge| edge.target));
            while let Some(inside) = stack.pop() {
                if inside == join || visited[inside.index()] == stamp {
                    continue;
                }
                visited[inside.index()] = stamp;
                if !dominance.dominates(block.id, inside) {
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "SSA block {} is entered from outside the region branching from block {}",
                            inside.index(),
                            block.id.index()
                        )
                        .into(),
                    });
                }
                stack.extend(
                    self.blocks[inside.index()]
                        .terminator
                        .edges()
                        .map(|edge| edge.target),
                );
            }
        }
        Ok(())
    }

    /// Structural identity rules: block ranges partition the instruction and
    /// parameter vectors in layout order, values are canonically numbered, the
    /// entry block takes no parameters, and exactly one block returns.
    fn validate_layout(&self) -> JitResult<()> {
        let verifier = |detail: String| JitError::Verifier {
            model: MODEL.into(),
            detail: detail.into(),
        };
        if self.blocks.is_empty() {
            return Err(verifier("SSA program declares no blocks".into()));
        }
        if self.entry != BlockId(0) {
            return Err(verifier("SSA entry block is not the first block".into()));
        }
        let mut instruction_cursor = 0;
        let mut parameter_cursor = 0;
        let mut return_blocks = 0;
        for (index, block) in self.blocks.iter().enumerate() {
            if block.id != BlockId::new(index)? {
                return Err(verifier(format!(
                    "SSA block {index} has non-canonical identity"
                )));
            }
            if block.instruction_start != instruction_cursor
                || block.instruction_end < block.instruction_start
                || block.instruction_end > self.instructions.len()
            {
                return Err(verifier(format!(
                    "SSA block {index} does not continue the instruction layout"
                )));
            }
            if block.parameter_start != parameter_cursor
                || block.parameter_end < block.parameter_start
                || block.parameter_end > self.block_parameters.len()
            {
                return Err(verifier(format!(
                    "SSA block {index} does not continue the parameter layout"
                )));
            }
            if index == self.entry.index() && block.parameter_end != block.parameter_start {
                return Err(verifier(
                    "SSA entry block cannot declare block parameters".into(),
                ));
            }
            if matches!(block.terminator, Terminator::Return(_)) {
                return_blocks += 1;
            }
            instruction_cursor = block.instruction_end;
            parameter_cursor = block.parameter_end;
        }
        if instruction_cursor != self.instructions.len()
            || parameter_cursor != self.block_parameters.len()
        {
            return Err(verifier(
                "SSA blocks do not cover every instruction and parameter".into(),
            ));
        }
        if return_blocks != 1 {
            return Err(verifier(format!(
                "SSA program has {return_blocks} return terminators, expected exactly one"
            )));
        }
        if !matches!(
            self.blocks[self.exit.index()].terminator,
            Terminator::Return(_)
        ) {
            return Err(verifier(
                "SSA exit block does not terminate with Return".into(),
            ));
        }
        for (index, instruction) in self.instructions.iter().enumerate() {
            if instruction.result != ValueId::new(index)? {
                return Err(verifier(format!(
                    "SSA instruction {index} has non-canonical result identity"
                )));
            }
        }
        for (index, parameter) in self.block_parameters.iter().enumerate() {
            if parameter.value != ValueId::new(self.instructions.len() + index)? {
                return Err(verifier(format!(
                    "SSA block parameter {index} has non-canonical identity"
                )));
            }
        }
        Ok(())
    }

    /// Every edge names a defined block, every block is reachable from the
    /// entry, and every block either returns or has a successor.
    ///
    /// Back edges are admitted here and classified in [`Self::natural_loops`],
    /// which needs dominance and therefore cannot run before this. What this
    /// pass still relies on is that every *forward* edge runs forward in
    /// layout order, which is what makes the one-pass reachability sweep and
    /// the one-pass dominator computation total.
    fn predecessors_checked(&self) -> JitResult<Vec<Vec<BlockId>>> {
        let verifier = |detail: String| JitError::Verifier {
            model: MODEL.into(),
            detail: detail.into(),
        };
        for block in &self.blocks {
            for edge in block.terminator.edges() {
                if edge.target.index() >= self.blocks.len() {
                    return Err(verifier(format!(
                        "SSA block {} branches to undefined block {}",
                        block.id.index(),
                        edge.target.index()
                    )));
                }
            }
        }
        let mut reachable = vec![false; self.blocks.len()];
        reachable[self.entry.index()] = true;
        for block in &self.blocks {
            if !reachable[block.id.index()] {
                return Err(verifier(format!(
                    "SSA block {} is unreachable from the entry block",
                    block.id.index()
                )));
            }
            if block.terminator.edges().next().is_none()
                && !matches!(block.terminator, Terminator::Return(_))
            {
                return Err(verifier(format!(
                    "SSA block {} has no successor and does not return",
                    block.id.index()
                )));
            }
            for edge in block.terminator.edges() {
                reachable[edge.target.index()] = true;
            }
        }
        Ok(self.predecessors())
    }

    /// Every block reaches the single exit.
    ///
    /// A backwards sweep from the exit over the predecessor map rather than
    /// one reverse pass over the layout: a latch's only successor is its
    /// header, which the layout has already passed, so a single reverse pass
    /// would report every loop as unable to reach the exit.
    fn validate_reaches_exit(&self, predecessors: &[Vec<BlockId>]) -> JitResult<()> {
        let mut reaches_exit = vec![false; self.blocks.len()];
        reaches_exit[self.exit.index()] = true;
        let mut work = vec![self.exit];
        while let Some(block) = work.pop() {
            for predecessor in predecessors[block.index()].iter().copied() {
                if !reaches_exit[predecessor.index()] {
                    reaches_exit[predecessor.index()] = true;
                    work.push(predecessor);
                }
            }
        }
        if let Some(block) = self
            .blocks
            .iter()
            .find(|block| !reaches_exit[block.id.index()])
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA block {} cannot reach the exit block", block.id.index())
                    .into(),
            });
        }
        Ok(())
    }

    fn validate_edges(&self) -> JitResult<()> {
        for block in &self.blocks {
            for edge in block.terminator.edges() {
                let target = &self.blocks[edge.target.index()];
                let parameters = self.parameters(target);
                if edge.arguments.len() != parameters.len() {
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "SSA edge {}->{} passes {} argument(s) to {} parameter(s)",
                            block.id.index(),
                            edge.target.index(),
                            edge.arguments.len(),
                            parameters.len()
                        )
                        .into(),
                    });
                }
                for (argument, parameter) in edge.arguments.iter().copied().zip(parameters) {
                    if self.value_type_of(argument)? != parameter.value_type {
                        return Err(JitError::Verifier {
                            model: MODEL.into(),
                            detail: format!(
                                "SSA edge {}->{} binds value {} to a parameter of a different type",
                                block.id.index(),
                                edge.target.index(),
                                argument.index()
                            )
                            .into(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Every use is dominated by its definition, and the returned value is a
    /// defined f64.
    fn validate_dominance(&self, dominance: &DominatorTree) -> JitResult<()> {
        let verifier = |detail: String| JitError::Verifier {
            model: MODEL.into(),
            detail: detail.into(),
        };
        let definitions = self.definition_blocks();
        let defining_block = |value: ValueId| -> JitResult<BlockId> {
            definitions
                .get(value.index())
                .copied()
                .ok_or_else(|| verifier(format!("SSA value {} is not defined", value.index())))
        };
        for (index, instruction) in self.instructions.iter().enumerate() {
            let using_block = defining_block(instruction.result)?;
            for operand in instruction.operands.iter().copied() {
                if operand.index() < self.instructions.len() && operand.index() >= index {
                    return Err(verifier(format!(
                        "SSA instruction {index} uses instruction value {} defined no earlier",
                        operand.index()
                    )));
                }
                if !dominance.dominates(defining_block(operand)?, using_block) {
                    return Err(verifier(format!(
                        "SSA instruction {index} uses value {} from a non-dominating block",
                        operand.index()
                    )));
                }
            }
        }
        for block in &self.blocks {
            for use_value in block.terminator.uses() {
                if !dominance.dominates(defining_block(use_value)?, block.id) {
                    return Err(verifier(format!(
                        "SSA block {} terminator uses value {} from a non-dominating block",
                        block.id.index(),
                        use_value.index()
                    )));
                }
                if use_value.index() < self.instructions.len()
                    && use_value.index() >= block.instruction_end
                {
                    return Err(verifier(format!(
                        "SSA block {} terminator uses instruction value {} defined after it",
                        block.id.index(),
                        use_value.index()
                    )));
                }
            }
        }
        if self.value_type_of(self.result())? != ValueType::F64 {
            return Err(verifier(
                "SSA return does not reference a defined f64 value".into(),
            ));
        }
        Ok(())
    }

    /// A state write must execute exactly once per evaluation.
    ///
    /// Verilog-A analog operators carry per-evaluation state whose continuity
    /// the solver depends on, so sinking one into a conditional arm silently
    /// desynchronizes it. Requiring its block to dominate the exit block is
    /// exactly the condition that it runs *at least* once, and it is the rule
    /// that keeps the conditional-splitting transform sound.
    ///
    /// Loops make "at least once" insufficient, so a state write inside one is
    /// refused as well — including in a loop header, which dominates the exit
    /// and still runs once per iteration. The reason is the state layout, not
    /// this verifier: `CanonicalStateLayout` gives one record to each analog
    /// operator *site*, so a `ddt` inside a loop is one operator holding one
    /// history for every trip the loop takes, which is the right number of
    /// records for exactly one trip and the wrong number for every other. The
    /// language agrees — Verilog-AMS LRM 2.4 section 4.4.1 admits an analog
    /// operator inside a loop only where the iteration count is a constant
    /// expression, whose elaborated form is unrolled into distinct operator
    /// sites. A front end that unrolls such a loop before this level therefore
    /// reaches here with distinct sites and no loop, and is admitted; anything
    /// else is refused by name rather than silently sharing one history.
    fn validate_effect_discipline(
        &self,
        dominance: &DominatorTree,
        loops: &[LoopRange],
    ) -> JitResult<()> {
        for block in &self.blocks {
            let inside_loop = loops
                .iter()
                .find(|range| range.contains(block.id))
                .map(|range| range.header);
            if inside_loop.is_none() && dominance.dominates(block.id, self.exit) {
                continue;
            }
            for instruction in &self.instructions[block.instruction_start..block.instruction_end] {
                if instruction.effects.writes_state() {
                    let detail = match inside_loop {
                        Some(header) => format!(
                            "SSA block {} runs once per iteration of the loop headed by block {} but writes state with {:?}",
                            block.id.index(),
                            header.index(),
                            instruction.op
                        ),
                        None => format!(
                            "SSA block {} executes conditionally but writes state with {:?}",
                            block.id.index(),
                            instruction.op
                        ),
                    };
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: detail.into(),
                    });
                }
            }
        }
        Ok(())
    }
}

/// One natural loop, as the contiguous layout range its blocks occupy.
///
/// Header and latch rather than a block set: [`Program::natural_loops`] has
/// already proved every block between them belongs to the loop, so the range
/// *is* the membership test, and both consumers — the allocator's interval
/// extension and the WebAssembly structurizer — want a range rather than a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopRange {
    header: BlockId,
    latch: BlockId,
}

impl LoopRange {
    pub(crate) fn header(self) -> BlockId {
        self.header
    }

    /// The block whose back edge closes the loop.
    #[cfg_attr(not(feature = "wasm-jit"), allow(dead_code))]
    pub(crate) fn latch(self) -> BlockId {
        self.latch
    }

    /// Whether `block` lies in the loop's layout range.
    ///
    /// The range *is* the membership test: [`Program::natural_loops`] admits
    /// a back edge only after proving every block between the header and the
    /// latch belongs to the loop.
    pub(crate) fn contains_layout(self, block: BlockId) -> bool {
        self.header <= block && block <= self.latch
    }

    fn contains(self, block: BlockId) -> bool {
        self.contains_layout(block)
    }
}

/// Constant-time dominance queries over the immediate-dominator forest.
///
/// A depth-first numbering of the dominator tree turns "does `ancestor`
/// dominate `block`" into one interval containment test. Walking parent links
/// instead is quadratic on a program whose blocks chain one conditional after
/// another, which is exactly what a compact model produces.
struct DominatorTree {
    enter: Vec<usize>,
    exit: Vec<usize>,
}

impl DominatorTree {
    fn new(idom: &[Option<BlockId>], entry: BlockId) -> Self {
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); idom.len()];
        for (index, parent) in idom.iter().enumerate() {
            if index == entry.index() {
                continue;
            }
            if let Some(parent) = parent {
                children[parent.index()].push(index);
            }
        }
        let mut enter = vec![usize::MAX; idom.len()];
        let mut exit = vec![usize::MAX; idom.len()];
        let mut clock = 0;
        let mut stack = vec![(entry.index(), false)];
        while let Some((block, finished)) = stack.pop() {
            if finished {
                exit[block] = clock;
                clock += 1;
                continue;
            }
            enter[block] = clock;
            clock += 1;
            stack.push((block, true));
            for child in children[block].iter().rev() {
                stack.push((*child, false));
            }
        }
        Self { enter, exit }
    }

    fn dominates(&self, ancestor: BlockId, block: BlockId) -> bool {
        let (Some(ancestor_enter), Some(ancestor_exit)) = (
            self.enter.get(ancestor.index()).copied(),
            self.exit.get(ancestor.index()).copied(),
        ) else {
            return false;
        };
        let Some(block_enter) = self.enter.get(block.index()).copied() else {
            return false;
        };
        // A block the depth-first walk never reached has no numbering, and
        // `predecessors_checked` has already rejected that program.
        ancestor_enter != usize::MAX
            && block_enter != usize::MAX
            && ancestor_enter <= block_enter
            && block_enter < ancestor_exit
    }
}

/// Which conditional arm exclusively consumes an instruction's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(test), allow(dead_code))]
struct ArmOwner {
    conditional: usize,
    taken: bool,
}

impl Program {
    /// Re-express every `NativeOp::IfElse` as a real two-way branch over
    /// blocks, with the conditional's result arriving as a block parameter.
    ///
    /// The postfix stream evaluates both arms before selecting between them,
    /// so this is the inverse of that if-conversion: an instruction whose
    /// every consumer sits inside one arm sinks into that arm's block and
    /// therefore executes only when the arm is taken. A `MAY_FAIL` operand —
    /// a bounds-checked dynamic array read, a table lookup — then raises a
    /// runtime error only on the path that actually reads it, which the select
    /// form cannot express. State writes never sink: the verifier's effect
    /// discipline requires them to stay on a block that dominates the exit,
    /// because analog operator state must advance on every evaluation whatever
    /// the condition does.
    ///
    /// Values agree bit-for-bit with the select form whenever both arms are
    /// pure, and the conditional's own truthiness (nonzero, NaN included) is
    /// unchanged; the backends realize it with the same comparison.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_branching_conditionals(&self) -> JitResult<Self> {
        if !self.is_single_block() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "conditional splitting consumes the single-block postfix lift".into(),
            });
        }
        if !self
            .instructions
            .iter()
            .any(|instruction| matches!(instruction.op, NativeOp::IfElse))
        {
            return Ok(self.clone());
        }
        let owners = self.conditional_arm_owners();
        ConditionalSplitter::new(self, &owners)?.run()
    }

    /// For each instruction, the conditional arm that exclusively consumes it.
    ///
    /// Uses always carry a larger index than their definition, so one reverse
    /// pass reaches the fixed point: an instruction belongs to an arm exactly
    /// when every one of its use sites is either that conditional's arm
    /// operand or another instruction already attributed to the same arm.
    #[cfg_attr(not(test), allow(dead_code))]
    fn conditional_arm_owners(&self) -> Vec<Option<ArmOwner>> {
        enum UseSite {
            /// The arm operand of a conditional: the use happens only when
            /// that arm is taken.
            Arm(ArmOwner),
            /// An ordinary operand of another instruction — a conditional's
            /// own condition included, since that is evaluated exactly when
            /// the conditional is. The use inherits whatever arm the using
            /// instruction turns out to belong to.
            Operand(usize),
            /// The program result: the use happens unconditionally.
            Unconditional,
        }

        let mut use_sites: Vec<Vec<UseSite>> = Vec::with_capacity(self.instructions.len());
        use_sites.resize_with(self.instructions.len(), Vec::new);
        for (index, instruction) in self.instructions.iter().enumerate() {
            let conditional = matches!(instruction.op, NativeOp::IfElse);
            for (position, operand) in instruction.operands.iter().copied().enumerate() {
                let site = if conditional && (position == 1 || position == 2) {
                    UseSite::Arm(ArmOwner {
                        conditional: index,
                        taken: position == 1,
                    })
                } else {
                    UseSite::Operand(index)
                };
                use_sites[operand.index()].push(site);
            }
        }
        use_sites[self.result().index()].push(UseSite::Unconditional);

        let mut owners: Vec<Option<ArmOwner>> = vec![None; self.instructions.len()];
        for index in (0..self.instructions.len()).rev() {
            if self.instructions[index].effects.writes_state() {
                continue;
            }
            let mut owner: Option<ArmOwner> = None;
            let mut exclusive = !use_sites[index].is_empty();
            for site in &use_sites[index] {
                let candidate = match site {
                    UseSite::Arm(arm) => Some(*arm),
                    UseSite::Operand(user) => owners[*user],
                    UseSite::Unconditional => None,
                };
                let Some(candidate) = candidate else {
                    exclusive = false;
                    break;
                };
                match owner {
                    None => owner = Some(candidate),
                    Some(existing) if existing == candidate => {}
                    Some(_) => {
                        exclusive = false;
                        break;
                    }
                }
            }
            if exclusive {
                owners[index] = owner;
            }
        }
        owners
    }
}

/// Rebuilds a single-block program as a diamond-structured block program.
///
/// Blocks are emitted in layout order, so a block's instruction range is
/// closed before the next block opens. A branch's terminator is written after
/// its arms are laid out, which is the only point at which the join block's
/// identity is known.
#[cfg_attr(not(test), allow(dead_code))]
struct ConditionalSplitter<'a> {
    source: &'a Program,
    /// Every `IfElse` becomes a terminator rather than an instruction, so the
    /// rebuilt program is shorter and block parameters number from here.
    instruction_count: usize,
    regions: HashMap<ArmOwner, Vec<usize>>,
    outer: Vec<usize>,
    remap: Vec<Option<ValueId>>,
    instructions: Vec<Instruction>,
    parameters: Vec<BlockParameter>,
    blocks: Vec<BasicBlock>,
    current: BlockId,
}

#[cfg_attr(not(test), allow(dead_code))]
impl<'a> ConditionalSplitter<'a> {
    fn new(source: &'a Program, owners: &[Option<ArmOwner>]) -> JitResult<Self> {
        let mut regions: HashMap<ArmOwner, Vec<usize>> = HashMap::new();
        let mut outer = Vec::new();
        for (index, owner) in owners.iter().enumerate() {
            match owner {
                Some(arm) => regions.entry(*arm).or_default().push(index),
                None => outer.push(index),
            }
        }
        let conditionals = source
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction.op, NativeOp::IfElse))
            .count();
        let mut splitter = Self {
            source,
            instruction_count: source.instructions.len() - conditionals,
            regions,
            outer,
            remap: vec![None; source.instructions.len()],
            instructions: Vec::with_capacity(source.instructions.len()),
            parameters: Vec::new(),
            blocks: Vec::new(),
            current: BlockId(0),
        };
        splitter.current = splitter.open_block(0)?;
        Ok(splitter)
    }

    fn open_block(&mut self, parameter_count: usize) -> JitResult<BlockId> {
        let id = BlockId::new(self.blocks.len())?;
        let parameter_start = self.parameters.len();
        for offset in 0..parameter_count {
            self.parameters.push(BlockParameter {
                value: ValueId::new(self.instruction_count + parameter_start + offset)?,
                value_type: ValueType::F64,
            });
        }
        self.blocks.push(BasicBlock {
            id,
            parameter_start,
            parameter_end: self.parameters.len(),
            instruction_start: self.instructions.len(),
            instruction_end: self.instructions.len(),
            // Every block created here is closed with its real terminator
            // before `finish`; a leaked placeholder produces a second `Return`
            // and the verifier rejects the program.
            terminator: Terminator::Return(ValueId(0)),
        });
        Ok(id)
    }

    fn close_range(&mut self, id: BlockId) {
        self.blocks[id.index()].instruction_end = self.instructions.len();
    }

    fn take_region(&mut self, conditional: usize, taken: bool) -> Vec<usize> {
        self.regions
            .remove(&ArmOwner { conditional, taken })
            .unwrap_or_default()
    }

    fn mapped(&self, value: ValueId) -> JitResult<ValueId> {
        self.remap[value.index()].ok_or_else(|| JitError::Verifier {
            model: MODEL.into(),
            detail: format!(
                "conditional splitting reached value {} before its definition",
                value.index()
            )
            .into(),
        })
    }

    fn emit(&mut self, region: &[usize]) -> JitResult<()> {
        let source = self.source;
        for index in region.iter().copied() {
            let instruction = &source.instructions[index];
            if !matches!(instruction.op, NativeOp::IfElse) {
                let operands = instruction
                    .operands
                    .iter()
                    .copied()
                    .map(|operand| self.mapped(operand))
                    .collect::<JitResult<Vec<_>>>()?
                    .into_boxed_slice();
                let result = ValueId::new(self.instructions.len())?;
                self.instructions.push(Instruction {
                    result,
                    value_type: instruction.value_type,
                    op: instruction.op,
                    operands,
                    effects: instruction.effects,
                });
                self.remap[index] = Some(result);
                continue;
            }

            let [condition, then_value, else_value] = *instruction.operands else {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: "conditional splitting requires three IfElse operands".into(),
                });
            };
            let condition = self.mapped(condition)?;
            let branch_block = self.current;
            self.close_range(branch_block);

            let then_entry = self.open_block(0)?;
            self.current = then_entry;
            let then_members = self.take_region(index, true);
            self.emit(&then_members)?;
            let then_tail = self.current;
            let then_argument = self.mapped(then_value)?;
            self.close_range(then_tail);

            let else_entry = self.open_block(0)?;
            self.current = else_entry;
            let else_members = self.take_region(index, false);
            self.emit(&else_members)?;
            let else_tail = self.current;
            let else_argument = self.mapped(else_value)?;
            self.close_range(else_tail);

            let join = self.open_block(1)?;
            let merged = self.parameters[self.blocks[join.index()].parameter_start].value;
            self.blocks[branch_block.index()].terminator = Terminator::Branch {
                condition,
                then_edge: Edge::new(then_entry, Vec::new()),
                else_edge: Edge::new(else_entry, Vec::new()),
            };
            self.blocks[then_tail.index()].terminator =
                Terminator::Jump(Edge::new(join, vec![then_argument]));
            self.blocks[else_tail.index()].terminator =
                Terminator::Jump(Edge::new(join, vec![else_argument]));
            self.current = join;
            self.remap[index] = Some(merged);
        }
        Ok(())
    }

    fn run(mut self) -> JitResult<Program> {
        let outer = std::mem::take(&mut self.outer);
        self.emit(&outer)?;
        let result = self.mapped(self.source.result())?;
        let exit = self.current;
        self.close_range(exit);
        self.blocks[exit.index()].terminator = Terminator::Return(result);
        let program = Program {
            entry: BlockId(0),
            exit,
            blocks: self.blocks,
            block_parameters: self.parameters,
            instructions: self.instructions,
            maximum_stack_depth: self.source.maximum_stack_depth,
        };
        program.validate()?;
        Ok(program)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExpressionId(u32);

impl ExpressionId {
    fn new(index: usize) -> JitResult<Self> {
        u32::try_from(index)
            .map(Self)
            .map_err(|_| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA expression count {index} exceeds the u32 identity space")
                    .into(),
            })
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
struct InternedExpression {
    op: NativeOp,
    operands: Box<[ExpressionId]>,
}

#[derive(Default)]
struct ExpressionInterner {
    expressions: Vec<InternedExpression>,
    buckets: HashMap<u64, Vec<ExpressionId>>,
}

impl ExpressionInterner {
    fn intern(&mut self, op: NativeOp, operands: Box<[ExpressionId]>) -> JitResult<ExpressionId> {
        let hash = native_op_hash(op, operands.iter().map(|operand| operand.index()));
        if let Some(existing) = self.buckets.get(&hash).and_then(|entries| {
            entries.iter().copied().find(|candidate| {
                let expression = &self.expressions[candidate.index()];
                native_ops_are_codegen_identical(expression.op, op)
                    && expression.operands.as_ref() == operands.as_ref()
            })
        }) {
            return Ok(existing);
        }
        let expression = ExpressionId::new(self.expressions.len())?;
        self.expressions.push(InternedExpression { op, operands });
        self.buckets.entry(hash).or_default().push(expression);
        Ok(expression)
    }

    fn unique(&mut self, op: NativeOp, operands: Box<[ExpressionId]>) -> JitResult<ExpressionId> {
        let expression = ExpressionId::new(self.expressions.len())?;
        self.expressions.push(InternedExpression { op, operands });
        Ok(expression)
    }
}

/// Stateful builder used for both one-result entries and source-ordered
/// assignment batches. Structural expression identities make equivalent DAG
/// nodes reusable even when their postfix programs are lowered separately.
struct ProgramLowerer {
    instructions: Vec<Instruction>,
    value_expressions: Vec<ExpressionId>,
    expressions: ExpressionInterner,
    reusable_values: HashMap<ExpressionId, ValueId>,
}

impl ProgramLowerer {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            instructions: Vec::with_capacity(capacity),
            value_expressions: Vec::with_capacity(capacity),
            expressions: ExpressionInterner::default(),
            reusable_values: HashMap::new(),
        }
    }

    fn append(&mut self, program: &NativeProgram) -> JitResult<(ValueId, usize)> {
        program.validate_dependency_metadata()?;

        let mut stack: Vec<ValueId> = Vec::with_capacity(program.max_stack_depth());
        let mut observed_maximum_depth = 0;
        for op in program.ops().iter().copied() {
            let (pop_count, push_count) = native_op_stack_effect(&op);
            if push_count != 1 {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!(
                        "native op {op:?} declares unsupported SSA result count {push_count}"
                    )
                    .into(),
                });
            }
            let operand_start =
                stack
                    .len()
                    .checked_sub(pop_count)
                    .ok_or_else(|| JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "native op {op:?} consumes {pop_count} value(s) at SSA depth {}",
                            stack.len()
                        )
                        .into(),
                    })?;
            let operands = stack.split_off(operand_start).into_boxed_slice();
            let operand_expressions = operands
                .iter()
                .map(|operand| {
                    self.value_expressions
                        .get(operand.index())
                        .copied()
                        .ok_or_else(|| JitError::Verifier {
                            model: MODEL.into(),
                            detail: format!(
                                "SSA expression identity is missing for operand {}",
                                operand.index()
                            )
                            .into(),
                        })
                })
                .collect::<JitResult<Vec<_>>>()?
                .into_boxed_slice();
            if matches!(op, NativeOp::IfElse)
                && let [_, then_value, else_value] = operands.as_ref()
                && then_value == else_value
            {
                stack.push(*then_value);
                observed_maximum_depth = observed_maximum_depth.max(stack.len());
                continue;
            }
            let effects = Effects::for_op(op);
            let expression = if effects.permits_result_sharing() {
                let expression = self.expressions.intern(op, operand_expressions)?;
                if let Some(result) = self.reusable_values.get(&expression).copied() {
                    stack.push(result);
                    observed_maximum_depth = observed_maximum_depth.max(stack.len());
                    continue;
                }
                expression
            } else {
                self.reusable_values.clear();
                self.expressions.unique(op, operand_expressions)?
            };
            let result = ValueId::new(self.instructions.len())?;
            self.instructions.push(Instruction {
                result,
                value_type: ValueType::F64,
                op,
                operands,
                effects,
            });
            self.value_expressions.push(expression);
            if effects.permits_result_sharing() {
                self.reusable_values.insert(expression, result);
            }
            stack.push(result);
            observed_maximum_depth = observed_maximum_depth.max(stack.len());
        }

        let [result] = stack.as_slice() else {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!(
                    "SSA lowering ends with {} live values, expected exactly one",
                    stack.len()
                )
                .into(),
            });
        };
        if observed_maximum_depth != program.max_stack_depth() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!(
                    "SSA observed stack depth {observed_maximum_depth}, native metadata records {}",
                    program.max_stack_depth()
                )
                .into(),
            });
        }
        Ok((*result, observed_maximum_depth))
    }

    fn finish(self, result: ValueId, maximum_stack_depth: usize) -> JitResult<Program> {
        Program::single_block(self.instructions, result, maximum_stack_depth)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AssignmentOutput {
    variable_index: usize,
    instruction_end: usize,
    value: ValueId,
}

impl AssignmentOutput {
    pub(crate) fn variable_index(self) -> usize {
        self.variable_index
    }

    pub(crate) fn instruction_end(self) -> usize {
        self.instruction_end
    }

    pub(crate) fn value(self) -> ValueId {
        self.value
    }
}

/// One SSA DAG with source-order publication points for a safe batch of direct
/// assignments. Stores are deliberately not represented as movable SSA ops:
/// each output boundary remains observable before any later failing program.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AssignmentProgram {
    program: Program,
    outputs: Box<[AssignmentOutput]>,
}

impl AssignmentProgram {
    pub(crate) fn lower(assignments: &[(usize, &NativeProgram)]) -> JitResult<Self> {
        if assignments.is_empty() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "cannot lower an empty direct-assignment batch".into(),
            });
        }
        let capacity = assignments
            .iter()
            .map(|(_, program)| program.ops().len())
            .fold(0_usize, usize::saturating_add);
        let mut lowerer = ProgramLowerer::with_capacity(capacity);
        let mut maximum_stack_depth = 0;
        let mut outputs = Vec::with_capacity(assignments.len());
        let mut final_result = None;
        for (variable_index, native) in assignments.iter().copied() {
            let (result, stack_depth) = lowerer.append(native)?;
            maximum_stack_depth = maximum_stack_depth.max(stack_depth);
            let instruction_end = lowerer.instructions.len();
            if instruction_end == 0 || result.index() >= instruction_end {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: "direct-assignment output is not dominated by its publication boundary"
                        .into(),
                });
            }
            outputs.push(AssignmentOutput {
                variable_index,
                instruction_end,
                value: result,
            });
            final_result = Some(result);
        }
        // Outputs at the same instruction boundary have no computation,
        // failure point, or variable read between them. Targets are distinct
        // by the assignment batch contract, so address order is equivalent
        // and substantially improves large-array store locality.
        outputs.sort_by_key(|output| (output.instruction_end, output.variable_index));
        let program = lowerer.finish(
            final_result.expect("non-empty assignment batch has a final result"),
            maximum_stack_depth,
        )?;
        let program = eliminate_dead_instructions(program, &mut outputs)?;
        Ok(Self {
            program,
            outputs: outputs.into_boxed_slice(),
        })
    }

    pub(crate) fn program(&self) -> &Program {
        &self.program
    }

    pub(crate) fn outputs(&self) -> &[AssignmentOutput] {
        &self.outputs
    }
}

/// One value while a [`ProgramBuilder`] is still open.
///
/// A block parameter's final [`ValueId`] is `instructions.len() + k`, which is
/// not known until the last instruction is emitted, so a builder cannot hand
/// out final identities as it goes. Naming the two spaces separately and
/// resolving both in [`ProgramBuilder::finish`] keeps that rule in the one
/// module that owns it, rather than obliging every producer to reproduce it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum BuilderValue {
    Instruction(usize),
    Parameter(usize),
}

/// How a block under construction ends.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum BuilderTerminator {
    Return(BuilderValue),
    Jump {
        target: BlockId,
        arguments: Vec<BuilderValue>,
    },
    Branch {
        condition: BuilderValue,
        then_target: BlockId,
        then_arguments: Vec<BuilderValue>,
        else_target: BlockId,
        else_arguments: Vec<BuilderValue>,
    },
}

#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
struct BuilderBlock {
    parameter_start: usize,
    parameter_end: usize,
    instruction_start: usize,
    instruction_end: usize,
    terminator: Option<BuilderTerminator>,
}

#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
struct BuilderInstruction {
    op: NativeOp,
    operands: Box<[BuilderValue]>,
    value_type: ValueType,
}

/// Construct a multi-block [`Program`] directly, rather than by lifting a
/// postfix stream.
///
/// The block list and every block's parameters are fixed when the builder is
/// created, because the identity rules make both positional: a block owns a
/// contiguous instruction range in layout order, and the flat parameter vector
/// is laid out in the same order. Filling the blocks afterwards, in order, is
/// what keeps those two facts true by construction instead of by assertion.
#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct ProgramBuilder {
    blocks: Vec<BuilderBlock>,
    parameter_types: Vec<ValueType>,
    instructions: Vec<BuilderInstruction>,
    open: Option<usize>,
}

#[cfg_attr(not(test), allow(dead_code))]
impl ProgramBuilder {
    /// Open a builder over `blocks` blocks, the n-th taking the parameter
    /// types in `parameters[n]`.
    pub(crate) fn new(parameters: &[Vec<ValueType>]) -> JitResult<Self> {
        if parameters.is_empty() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA program builder declares no blocks".into(),
            });
        }
        let mut parameter_types = Vec::new();
        let blocks = parameters
            .iter()
            .map(|types| {
                let parameter_start = parameter_types.len();
                parameter_types.extend(types.iter().copied());
                BuilderBlock {
                    parameter_start,
                    parameter_end: parameter_types.len(),
                    instruction_start: 0,
                    instruction_end: 0,
                    terminator: None,
                }
            })
            .collect();
        Ok(Self {
            blocks,
            parameter_types,
            instructions: Vec::new(),
            open: None,
        })
    }

    /// The `index`-th parameter of `block`.
    pub(crate) fn parameter(&self, block: BlockId, index: usize) -> JitResult<BuilderValue> {
        let block = self
            .blocks
            .get(block.index())
            .ok_or_else(|| self.verifier(format!("SSA builder has no block {}", block.index())))?;
        let flat = block.parameter_start + index;
        if flat >= block.parameter_end {
            return Err(self.verifier(format!(
                "SSA builder block declares {} parameters, not {}",
                block.parameter_end - block.parameter_start,
                index + 1
            )));
        }
        Ok(BuilderValue::Parameter(flat))
    }

    /// Start filling `block`. Blocks must be filled in layout order.
    pub(crate) fn begin_block(&mut self, block: BlockId) -> JitResult<()> {
        if self.open.is_some() {
            return Err(self.verifier(format!(
                "SSA builder block {} is still open",
                self.open.unwrap_or_default()
            )));
        }
        let expected = self
            .blocks
            .iter()
            .position(|candidate| candidate.terminator.is_none())
            .unwrap_or(self.blocks.len());
        if block.index() != expected {
            return Err(self.verifier(format!(
                "SSA builder block {} was filled out of layout order; block {expected} is next",
                block.index()
            )));
        }
        let start = self.instructions.len();
        let entry = &mut self.blocks[block.index()];
        entry.instruction_start = start;
        entry.instruction_end = start;
        self.open = Some(block.index());
        Ok(())
    }

    /// Append one instruction to the open block.
    pub(crate) fn push(
        &mut self,
        op: NativeOp,
        operands: &[BuilderValue],
        value_type: ValueType,
    ) -> JitResult<BuilderValue> {
        if self.open.is_none() {
            return Err(self.verifier("SSA builder has no open block".to_string()));
        }
        let index = self.instructions.len();
        self.instructions.push(BuilderInstruction {
            op,
            operands: operands.to_vec().into_boxed_slice(),
            value_type,
        });
        Ok(BuilderValue::Instruction(index))
    }

    /// Close the open block with its terminator.
    pub(crate) fn end_block(&mut self, terminator: BuilderTerminator) -> JitResult<()> {
        let open = self
            .open
            .take()
            .ok_or_else(|| self.verifier("SSA builder has no open block to end".to_string()))?;
        let end = self.instructions.len();
        let block = &mut self.blocks[open];
        block.instruction_end = end;
        block.terminator = Some(terminator);
        Ok(())
    }

    pub(crate) fn finish(self, entry: BlockId, exit: BlockId) -> JitResult<Program> {
        if self.open.is_some() {
            return Err(self.verifier("SSA builder finished with an open block".to_string()));
        }
        let instruction_count = self.instructions.len();
        let resolve = |value: BuilderValue| -> JitResult<ValueId> {
            match value {
                BuilderValue::Instruction(index) => ValueId::new(index),
                BuilderValue::Parameter(index) => ValueId::new(instruction_count + index),
            }
        };
        let resolve_all = |values: &[BuilderValue]| -> JitResult<Vec<ValueId>> {
            values.iter().copied().map(resolve).collect()
        };
        let maximum_stack_depth = self
            .instructions
            .iter()
            .map(|instruction| instruction.operands.len().max(1))
            .max()
            .unwrap_or(1);
        let instructions = self
            .instructions
            .iter()
            .enumerate()
            .map(|(index, instruction)| {
                Ok(Instruction {
                    result: ValueId::new(index)?,
                    value_type: instruction.value_type,
                    op: instruction.op,
                    operands: resolve_all(&instruction.operands)?.into_boxed_slice(),
                    effects: Effects::for_op(instruction.op),
                })
            })
            .collect::<JitResult<Vec<_>>>()?;
        let block_parameters = self
            .parameter_types
            .iter()
            .enumerate()
            .map(|(index, value_type)| {
                Ok(BlockParameter {
                    value: ValueId::new(instruction_count + index)?,
                    value_type: *value_type,
                })
            })
            .collect::<JitResult<Vec<_>>>()?;
        let blocks = self
            .blocks
            .iter()
            .enumerate()
            .map(|(index, block)| {
                let terminator = block
                    .terminator
                    .as_ref()
                    .ok_or_else(|| JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!("SSA builder block {index} was never terminated").into(),
                    })?;
                Ok(BasicBlock {
                    id: BlockId::new(index)?,
                    parameter_start: block.parameter_start,
                    parameter_end: block.parameter_end,
                    instruction_start: block.instruction_start,
                    instruction_end: block.instruction_end,
                    terminator: match terminator {
                        BuilderTerminator::Return(value) => Terminator::Return(resolve(*value)?),
                        BuilderTerminator::Jump { target, arguments } => {
                            Terminator::Jump(Edge::new(*target, resolve_all(arguments)?))
                        }
                        BuilderTerminator::Branch {
                            condition,
                            then_target,
                            then_arguments,
                            else_target,
                            else_arguments,
                        } => Terminator::Branch {
                            condition: resolve(*condition)?,
                            then_edge: Edge::new(*then_target, resolve_all(then_arguments)?),
                            else_edge: Edge::new(*else_target, resolve_all(else_arguments)?),
                        },
                    },
                })
            })
            .collect::<JitResult<Vec<_>>>()?;
        let program = Program {
            entry,
            exit,
            blocks,
            block_parameters,
            instructions,
            maximum_stack_depth,
        };
        program.validate()?;
        Ok(program)
    }

    fn verifier(&self, detail: String) -> JitError {
        JitError::Verifier {
            model: MODEL.into(),
            detail: detail.into(),
        }
    }
}

fn eliminate_dead_instructions(
    program: Program,
    outputs: &mut [AssignmentOutput],
) -> JitResult<Program> {
    if !program.is_single_block() {
        return Err(JitError::Verifier {
            model: MODEL.into(),
            detail: "dead-instruction elimination runs before conditional splitting and requires a single-block program".into(),
        });
    }
    let instruction_count = program.instructions.len();
    let mut live = vec![false; instruction_count];
    let mut work = vec![program.result()];
    work.extend(outputs.iter().map(|output| output.value));
    work.extend(
        program
            .instructions
            .iter()
            .filter(|instruction| !instruction.effects.permits_result_sharing())
            .map(|instruction| instruction.result),
    );
    while let Some(value) = work.pop() {
        let Some(marked) = live.get_mut(value.index()) else {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA liveness references undefined value {}", value.index()).into(),
            });
        };
        if *marked {
            continue;
        }
        *marked = true;
        work.extend(program.instructions[value.index()].operands.iter().copied());
    }
    if live.iter().all(|value| *value) {
        return Ok(program);
    }

    let mut prefix = vec![0_usize; instruction_count + 1];
    for (index, is_live) in live.iter().copied().enumerate() {
        prefix[index + 1] = prefix[index] + usize::from(is_live);
    }
    let old_result = program.result();
    let maximum_stack_depth = program.maximum_stack_depth;
    let mut remap = vec![None; instruction_count];
    let mut instructions = Vec::with_capacity(prefix[instruction_count]);
    for (old_index, instruction) in program.instructions.into_iter().enumerate() {
        if !live[old_index] {
            continue;
        }
        let result = ValueId::new(instructions.len())?;
        remap[old_index] = Some(result);
        let operands = instruction
            .operands
            .iter()
            .map(|operand| {
                remap[operand.index()].ok_or_else(|| JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!(
                        "SSA live value {} references removed operand {}",
                        old_index,
                        operand.index()
                    )
                    .into(),
                })
            })
            .collect::<JitResult<Vec<_>>>()?
            .into_boxed_slice();
        instructions.push(Instruction {
            result,
            value_type: instruction.value_type,
            op: instruction.op,
            operands,
            effects: instruction.effects,
        });
    }
    let remap_value = |value: ValueId| {
        remap[value.index()].ok_or_else(|| JitError::Verifier {
            model: MODEL.into(),
            detail: format!("SSA required value {} was removed", value.index()).into(),
        })
    };
    for output in outputs {
        output.value = remap_value(output.value)?;
        output.instruction_end =
            prefix
                .get(output.instruction_end)
                .copied()
                .ok_or_else(|| JitError::Verifier {
                    model: MODEL.into(),
                    detail: "SSA output boundary exceeds the instruction range".into(),
                })?;
    }
    let result = remap_value(old_result)?;
    Program::single_block(instructions, result, maximum_stack_depth)
}

/// One computation and every output slot that consumes its result.
///
/// Sparse Jacobian construction already omits structurally absent axes.  This
/// plan handles the other common source of redundant work: a current
/// contribution stamps the same derivative into positive and negative KCL
/// rows, so the two matrix entries carry byte-for-byte identical programs.
/// Exact SSA equality plus the formal effect gate makes that one computation
/// with two stores without changing numerical semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SharedOutputGroup {
    representative: usize,
    outputs: Box<[usize]>,
}

impl SharedOutputGroup {
    pub(crate) fn representative(&self) -> usize {
        self.representative
    }

    pub(crate) fn outputs(&self) -> &[usize] {
        &self.outputs
    }
}

pub(crate) fn plan_shared_outputs(programs: &[Program]) -> Vec<SharedOutputGroup> {
    let mut groups: Vec<SharedOutputGroup> = Vec::with_capacity(programs.len());
    for (output, program) in programs.iter().enumerate() {
        let reusable = program.permits_result_sharing();
        let existing = reusable.then(|| {
            groups.iter().position(|group| {
                programs[group.representative].permits_result_sharing()
                    && programs[group.representative] == *program
            })
        });
        if let Some(Some(group)) = existing {
            let mut outputs = groups[group].outputs.to_vec();
            outputs.push(output);
            groups[group].outputs = outputs.into_boxed_slice();
        } else {
            groups.push(SharedOutputGroup {
                representative: output,
                outputs: vec![output].into_boxed_slice(),
            });
        }
    }
    groups
}

/// Logical registers 0-9 hold allocated SSA values. Backends map this bank to
/// caller- or callee-saved machine registers according to their host ABI. Six
/// logical registers remain available for materializing the widest native
/// operation's five spilled operands plus one instruction-specific scratch.
pub(crate) const ALLOCATABLE_VALUE_REGISTERS: usize = 10;
pub(crate) const LOGICAL_VALUE_REGISTER_COUNT: usize = 21;

/// How one backend's logical register bank maps onto its ABI's preservation
/// rules.
///
/// The allocator needs the second fact as much as the first. A value that is
/// live across a helper call cannot stay in a caller-saved register, because
/// the emitters preserve only the operand stack of the instruction making the
/// call; anything else in a volatile register is gone when the helper returns.
/// A callee-saved register is preserved by the helper itself, so parking the
/// value there costs one prologue save that the whole function shares, instead
/// of a store at the definition and a reload at every later use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RegisterBank {
    count: usize,
    callee_saved_base: usize,
}

impl RegisterBank {
    /// A bank in which every value live across a call has to spill.
    ///
    /// This is AArch64, whose bank deliberately excludes the nonvolatile
    /// D8-D15 so that leaf arithmetic needs no SIMD save area at all.
    pub(crate) const fn all_caller_saved(count: usize) -> Self {
        Self {
            count,
            callee_saved_base: count,
        }
    }

    /// A bank whose logical registers at or above `callee_saved_base` map to
    /// machine registers the ABI requires a callee to preserve.
    ///
    /// A base at or past the end of the bank means no register in it is
    /// preserved, which is how System V x64 arrives here: it preserves no XMM
    /// register, so its first nonvolatile index is past every allocatable one.
    pub(crate) const fn with_callee_saved_from(count: usize, callee_saved_base: usize) -> Self {
        Self {
            count,
            callee_saved_base: if callee_saved_base < count {
                callee_saved_base
            } else {
                count
            },
        }
    }

    pub(crate) const fn count(&self) -> usize {
        self.count
    }

    const fn is_callee_saved(&self, register: usize) -> bool {
        register >= self.callee_saved_base
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ValueLocation {
    Register(usize),
    Spill(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AllocatedInstruction {
    operands: Box<[ValueLocation]>,
    result: ValueLocation,
    live_register_mask: u32,
}

impl AllocatedInstruction {
    pub(crate) fn operands(&self) -> &[ValueLocation] {
        &self.operands
    }

    pub(crate) fn result(&self) -> ValueLocation {
        self.result
    }

    pub(crate) fn live_register_mask(&self) -> u32 {
        self.live_register_mask
    }
}

/// One step of an edge's parallel move.
///
/// A naive sequential move loses a value whenever two edge arguments swap
/// locations, so the sequencer orders every move whose destination is not
/// still somebody's source and breaks the remaining cycles through one
/// reserved scratch slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MoveStep {
    Move {
        from: ValueLocation,
        to: ValueLocation,
    },
    SaveToScratch {
        from: ValueLocation,
    },
    RestoreFromScratch {
        to: ValueLocation,
    },
}

/// One sequenced move a backend performs on a control-flow edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EdgeMove {
    from: ValueLocation,
    to: ValueLocation,
}

impl EdgeMove {
    pub(crate) fn from(self) -> ValueLocation {
        self.from
    }

    pub(crate) fn to(self) -> ValueLocation {
        self.to
    }
}

/// Reduce a sequenced parallel move to plain moves, realizing the
/// cycle-breaking scratch as one reserved spill slot.
///
/// On a forward edge no cycle can form: a block parameter takes its location
/// at the earliest predecessor terminator that binds it, while every argument
/// bound on that edge is still live and so still owns its own location, so no
/// argument can already sit in a parameter's location. A loop back edge breaks
/// that premise — a header parameter's location was fixed at the preheader and
/// the latch's arguments are the parameters themselves — and `x, y = y, x`
/// across one is a genuine permutation cycle.
///
/// The scratch is a spill slot rather than a register because a spill slot is
/// the one storage class all three backends already move to and from: x64 and
/// AArch64 each have a `Spill`/`Register` move for every combination, and the
/// WebAssembly encoder gives every value its own local. Reserving a register
/// instead would take one from a bank the widest operation already sizes
/// exactly. The slot is allocated once for the whole function and only when
/// some edge needs it, so a program without a cycle keeps the frame it had.
fn realize_edge_moves(steps: &[MoveStep], scratch: Option<usize>) -> JitResult<Box<[EdgeMove]>> {
    steps
        .iter()
        .map(|step| {
            let scratch = || {
                scratch
                    .map(ValueLocation::Spill)
                    .ok_or(JitError::RegisterAllocation {
                        model: MODEL.into(),
                        detail:
                            "SSA edge move needs a cycle break but no scratch slot was reserved"
                                .into(),
                    })
            };
            match *step {
                MoveStep::Move { from, to } => Ok(EdgeMove { from, to }),
                MoveStep::SaveToScratch { from } => Ok(EdgeMove {
                    from,
                    to: scratch()?,
                }),
                MoveStep::RestoreFromScratch { to } => Ok(EdgeMove {
                    from: scratch()?,
                    to,
                }),
            }
        })
        .collect::<JitResult<Vec<_>>>()
        .map(Vec::into_boxed_slice)
}

/// Whether a sequenced parallel move needs the cycle-breaking scratch.
fn needs_scratch(steps: &[MoveStep]) -> bool {
    steps
        .iter()
        .any(|step| matches!(step, MoveStep::SaveToScratch { .. }))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisterAllocation {
    instructions: Vec<AllocatedInstruction>,
    result: ValueLocation,
    value_locations: Vec<ValueLocation>,
    spill_slot_count: usize,
    used_register_count: usize,
    required_register_count: usize,
    edge_moves: Vec<Box<[Box<[EdgeMove]>]>>,
}

fn expire_owners(
    register_owners: &mut [Option<ValueId>],
    spill_owners: &mut [Option<ValueId>],
    last_use_position: &[usize],
    position: usize,
) {
    for owner in register_owners.iter_mut().chain(spill_owners.iter_mut()) {
        if owner.is_some_and(|value: ValueId| last_use_position[value.index()] < position) {
            *owner = None;
        }
    }
}

fn select_location(
    value: ValueId,
    reusable_first: Option<usize>,
    crosses_call: bool,
    bank: RegisterBank,
    register_owners: &[Option<ValueId>],
    spill_owners: &mut Vec<Option<ValueId>>,
) -> ValueLocation {
    if crosses_call {
        // A caller-saved register does not survive the call this value is live
        // across; a callee-saved one does, and costs the function one shared
        // prologue save rather than a store here and a reload at every use.
        // See [`RegisterBank`]. Reusing a dying operand's register is still
        // preferable when that register is itself preserved.
        return reusable_first
            .filter(|register| bank.is_callee_saved(*register))
            .or_else(|| {
                register_owners
                    .iter()
                    .enumerate()
                    .skip(bank.callee_saved_base)
                    .find_map(|(register, owner)| owner.is_none().then_some(register))
            })
            .map_or_else(
                || ValueLocation::Spill(allocate_spill_slot(spill_owners, value)),
                ValueLocation::Register,
            );
    }
    if let Some(register) = reusable_first {
        return ValueLocation::Register(register);
    }
    register_owners
        .iter()
        .position(Option::is_none)
        .map_or_else(
            || ValueLocation::Spill(allocate_spill_slot(spill_owners, value)),
            ValueLocation::Register,
        )
}

/// Order one edge's simultaneous location assignments into machine moves.
///
/// Sources may repeat; destinations may not. Any move whose destination is no
/// longer read can be emitted immediately, and what remains once nothing is
/// emittable is a set of disjoint permutation cycles. Breaking one cycle
/// through the scratch slot unblocks exactly that cycle and no other, so at
/// most one saved value is ever outstanding.
fn sequence_parallel_move(pairs: &[(ValueLocation, ValueLocation)]) -> JitResult<Vec<MoveStep>> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Source {
        Location(ValueLocation),
        Scratch,
    }

    for (index, (_, destination)) in pairs.iter().enumerate() {
        if pairs[..index]
            .iter()
            .any(|(_, earlier)| earlier == destination)
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA edge binds two block parameters to one location".into(),
            });
        }
    }

    let mut pending: Vec<(Source, ValueLocation)> = pairs
        .iter()
        .filter(|(from, to)| from != to)
        .map(|(from, to)| (Source::Location(*from), *to))
        .collect();
    let mut steps = Vec::with_capacity(pending.len());
    let budget = pending.len().saturating_mul(2).saturating_add(1);
    for _ in 0..budget {
        if pending.is_empty() {
            return Ok(steps);
        }
        let ready = pending.iter().position(|(_, destination)| {
            !pending
                .iter()
                .any(|(source, _)| *source == Source::Location(*destination))
        });
        match ready {
            Some(index) => {
                let (source, destination) = pending.remove(index);
                steps.push(match source {
                    Source::Location(from) => MoveStep::Move {
                        from,
                        to: destination,
                    },
                    Source::Scratch => MoveStep::RestoreFromScratch { to: destination },
                });
            }
            None => {
                let (Source::Location(cycle_source), _) = pending[0] else {
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: "SSA parallel move reached a second scratch cycle".into(),
                    });
                };
                steps.push(MoveStep::SaveToScratch { from: cycle_source });
                for entry in &mut pending {
                    if entry.0 == Source::Location(cycle_source) {
                        entry.0 = Source::Scratch;
                    }
                }
            }
        }
    }
    Err(JitError::Verifier {
        model: MODEL.into(),
        detail: "SSA parallel move did not terminate within its step budget".into(),
    })
}

impl RegisterAllocation {
    pub(crate) fn build(program: &Program, bank: RegisterBank) -> JitResult<Self> {
        Self::build_with_output_uses(program, bank, &[])
    }

    pub(crate) fn build_for_assignments(
        program: &AssignmentProgram,
        bank: RegisterBank,
    ) -> JitResult<Self> {
        let output_uses = program
            .outputs()
            .iter()
            .map(|output| (output.instruction_end(), output.value()))
            .collect::<Vec<_>>();
        Self::build_with_output_uses(program.program(), bank, &output_uses)
    }

    fn build_with_output_uses(
        program: &Program,
        bank: RegisterBank,
        output_uses: &[(usize, ValueId)],
    ) -> JitResult<Self> {
        program.validate()?;
        let allocatable_register_count = bank.count();
        if !(1..=LOGICAL_VALUE_REGISTER_COUNT).contains(&allocatable_register_count) {
            return Err(JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: format!(
                    "logical value-register count {allocatable_register_count} is outside 1..={LOGICAL_VALUE_REGISTER_COUNT}"
                )
                .into(),
            });
        }
        let instruction_count = program.instructions.len();
        let value_count = program.value_count();
        // A block parameter is available from the earliest predecessor
        // terminator that binds it, because that is where the backend performs
        // the edge's parallel move. Holding its location from that point keeps
        // an intervening sibling block from clobbering it.
        let parameter_definition = program.parameter_definition_positions()?;
        // Positions before/after one instruction are distinct. An ordinary
        // operand use is at 2*i; a source-order assignment publication is at
        // 2*i+1. This prevents the allocator from overwriting a value with an
        // instruction result before that value has been stored.
        let mut last_use_position: Vec<usize> = (0..value_count)
            .map(|index| {
                if index < instruction_count {
                    index * 2
                } else {
                    parameter_definition[index - instruction_count] * 2
                }
            })
            .collect();
        for (instruction_index, instruction) in program.instructions.iter().enumerate() {
            for operand in instruction.operands.iter().copied() {
                let operand_index = operand.0 as usize;
                last_use_position[operand_index] =
                    last_use_position[operand_index].max(instruction_index * 2);
            }
        }
        for block in program.blocks() {
            let terminator_position = block.instruction_end.checked_mul(2).ok_or_else(|| {
                JitError::RegisterAllocation {
                    model: MODEL.into(),
                    detail: "SSA terminator liveness position overflow".into(),
                }
            })?;
            for use_value in block.terminator.uses() {
                last_use_position[use_value.index()] =
                    last_use_position[use_value.index()].max(terminator_position);
            }
        }
        for &(instruction_end, value) in output_uses {
            if instruction_end == 0
                || instruction_end > instruction_count
                || value.index() >= instruction_end
            {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!(
                        "SSA output value {} is invalid at instruction boundary {instruction_end}",
                        value.index()
                    )
                    .into(),
                });
            }
            let use_position = (instruction_end - 1)
                .checked_mul(2)
                .and_then(|position| position.checked_add(1))
                .ok_or_else(|| JitError::RegisterAllocation {
                    model: MODEL.into(),
                    detail: "SSA assignment-output liveness position overflow".into(),
                })?;
            last_use_position[value.index()] = last_use_position[value.index()].max(use_position);
        }

        // A linear scan expires a value after its last use in *layout* order,
        // which is its last use in execution order only while control runs
        // forward. Inside a loop it does not: a value defined before the loop
        // and read at its top is read again on the next iteration, after the
        // scan has already handed its register to something defined lower in
        // the body. Extending every interval that reaches into a loop to the
        // loop's end restores the invariant the scan needs — the standard
        // treatment of loops in a linear-scan allocator — at the cost of
        // holding a location across the whole loop, which is exactly what a
        // value live around the back edge occupies anyway.
        let loops = program.loop_ranges()?;
        if !loops.is_empty() {
            // Where each value first holds its definition, on the same scale
            // as `last_use_position`. A value defined inside a loop is
            // redefined on every iteration and needs no extension; only one
            // that enters the loop already defined can be read after the scan
            // has released it.
            let definition_position: Vec<usize> = (0..value_count)
                .map(|index| {
                    if index < instruction_count {
                        index * 2
                    } else {
                        parameter_definition[index - instruction_count] * 2
                    }
                })
                .collect();
            let extent = |range: &LoopRange| {
                let blocks = program.blocks();
                (
                    blocks[range.header().index()].instruction_start * 2,
                    blocks[range.latch().index()].instruction_end * 2,
                )
            };
            // Extending one value across an inner loop can carry it into an
            // enclosing one, so the sweep repeats until it is stable. Loops
            // nest, so the number of rounds is bounded by the nesting depth
            // and the bound below is only a backstop.
            for _ in 0..=loops.len() {
                let mut changed = false;
                for range in &loops {
                    let (start, end) = extent(range);
                    for (value, position) in last_use_position.iter_mut().enumerate() {
                        if definition_position[value] < start
                            && *position >= start
                            && *position < end
                        {
                            *position = end;
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        let mut call_prefix = Vec::with_capacity(instruction_count + 1);
        call_prefix.push(0_usize);
        for instruction in &program.instructions {
            let next = call_prefix
                .last()
                .copied()
                .unwrap_or(0)
                .saturating_add(usize::from(instruction.effects.may_call()));
            call_prefix.push(next);
        }
        // `bound` is the first instruction index at which the value already
        // holds its definition: `i + 1` for instruction `i`, and the binding
        // terminator's boundary for a block parameter.
        let crosses_call = |value: ValueId| {
            let position = last_use_position[value.index()];
            let end_instruction = (position / 2).min(instruction_count);
            let end_exclusive = end_instruction
                .saturating_add(usize::from(position & 1 != 0))
                .min(instruction_count);
            let bound = if value.index() < instruction_count {
                value.index() + 1
            } else {
                parameter_definition[value.index() - instruction_count]
            };
            end_exclusive > bound && call_prefix[end_exclusive] > call_prefix[bound]
        };

        let mut locations = vec![None; value_count];
        let mut register_owners = vec![None; allocatable_register_count];
        let mut spill_owners: Vec<Option<ValueId>> = Vec::new();
        let mut instructions = Vec::with_capacity(instruction_count);
        let mut used_register_count = 0;
        let mut move_plans: Vec<Vec<Vec<MoveStep>>> = Vec::with_capacity(program.blocks().len());

        for block in program.blocks() {
            expire_owners(
                &mut register_owners,
                &mut spill_owners,
                &last_use_position,
                block.instruction_start * 2,
            );
            for (instruction_index, instruction) in program.instructions
                [block.instruction_start..block.instruction_end]
                .iter()
                .enumerate()
                .map(|(offset, instruction)| (block.instruction_start + offset, instruction))
            {
                let instruction_position = instruction_index * 2;
                expire_owners(
                    &mut register_owners,
                    &mut spill_owners,
                    &last_use_position,
                    instruction_position,
                );
                let live_register_mask = register_owners
                    .iter()
                    .enumerate()
                    .fold(0_u32, |mask, (register, owner)| {
                        mask | (u32::from(owner.is_some()) << register)
                    });
                let operands: Box<[ValueLocation]> = instruction
                .operands
                .iter()
                .map(|operand| {
                    locations[operand.0 as usize].ok_or_else(|| JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "SSA allocator reached undefined operand {operand:?} at instruction {instruction_index}"
                        )
                        .into(),
                    })
                })
                .collect::<JitResult<Vec<_>>>()?
                .into_boxed_slice();

                let reusable_first = instruction
                    .operands
                    .first()
                    .copied()
                    .filter(|operand| last_use_position[operand.0 as usize] == instruction_position)
                    .and_then(|operand| locations[operand.0 as usize])
                    .and_then(|location| match location {
                        ValueLocation::Register(register) => Some(register),
                        ValueLocation::Spill(_) => None,
                    });

                let result = select_location(
                    instruction.result,
                    reusable_first,
                    crosses_call(instruction.result),
                    bank,
                    &register_owners,
                    &mut spill_owners,
                );

                for (operand, location) in instruction
                    .operands
                    .iter()
                    .copied()
                    .zip(operands.iter().copied())
                {
                    if last_use_position[operand.0 as usize] != instruction_position {
                        continue;
                    }
                    match location {
                        ValueLocation::Register(register)
                            if result != ValueLocation::Register(register) =>
                        {
                            register_owners[register] = None;
                        }
                        ValueLocation::Spill(slot) if result != ValueLocation::Spill(slot) => {
                            spill_owners[slot] = None;
                        }
                        ValueLocation::Register(_) | ValueLocation::Spill(_) => {}
                    }
                }

                match result {
                    ValueLocation::Register(register) => {
                        register_owners[register] = Some(instruction.result);
                        used_register_count = used_register_count.max(register + 1);
                    }
                    ValueLocation::Spill(slot) => spill_owners[slot] = Some(instruction.result),
                }
                locations[instruction.result.0 as usize] = Some(result);
                instructions.push(AllocatedInstruction {
                    operands,
                    result,
                    live_register_mask,
                });
            }

            let terminator_position = block.instruction_end * 2;
            expire_owners(
                &mut register_owners,
                &mut spill_owners,
                &last_use_position,
                terminator_position,
            );
            // The first predecessor in layout order owns the target's
            // parameter locations; every later predecessor reuses them.
            for edge in block.terminator.edges() {
                let target = &program.blocks()[edge.target.index()];
                for parameter in program.parameters(target) {
                    if locations[parameter.value.index()].is_some() {
                        continue;
                    }
                    let location = select_location(
                        parameter.value,
                        None,
                        crosses_call(parameter.value),
                        bank,
                        &register_owners,
                        &mut spill_owners,
                    );
                    match location {
                        ValueLocation::Register(register) => {
                            register_owners[register] = Some(parameter.value);
                            used_register_count = used_register_count.max(register + 1);
                        }
                        ValueLocation::Spill(slot) => spill_owners[slot] = Some(parameter.value),
                    }
                    locations[parameter.value.index()] = Some(location);
                }
            }
            let mut block_edge_moves = Vec::new();
            for edge in block.terminator.edges() {
                let target = &program.blocks()[edge.target.index()];
                let parameters = program.parameters(target);
                let mut pairs = Vec::with_capacity(parameters.len());
                for (argument, parameter) in edge.arguments.iter().copied().zip(parameters) {
                    let from = locations[argument.index()].ok_or_else(|| JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "SSA allocator reached unassigned edge argument {}",
                            argument.index()
                        )
                        .into(),
                    })?;
                    let to =
                        locations[parameter.value.index()].ok_or_else(|| JitError::Verifier {
                            model: MODEL.into(),
                            detail: format!(
                                "SSA allocator reached unassigned block parameter {}",
                                parameter.value.index()
                            )
                            .into(),
                        })?;
                    pairs.push((from, to));
                }
                block_edge_moves.push(sequence_parallel_move(&pairs)?);
            }
            move_plans.push(block_edge_moves);
        }

        // One reserved slot serves every cycle in the function: the sequencer
        // keeps at most one value outstanding, and each edge's plan restores
        // it before the branch. Reserving it only when some plan asks keeps a
        // program without a back edge on exactly the frame it had.
        let scratch_slot = move_plans
            .iter()
            .flatten()
            .any(|steps| needs_scratch(steps))
            .then(|| {
                spill_owners.push(None);
                spill_owners.len() - 1
            });
        let edge_moves = move_plans
            .iter()
            .map(|block| {
                block
                    .iter()
                    .map(|steps| realize_edge_moves(steps, scratch_slot))
                    .collect::<JitResult<Vec<_>>>()
                    .map(Vec::into_boxed_slice)
            })
            .collect::<JitResult<Vec<_>>>()?;

        let required_register_count = instructions
            .iter()
            .map(required_register_count)
            .max()
            .unwrap_or(0)
            .max(used_register_count);
        let result = locations[program.result().0 as usize].ok_or_else(|| JitError::Verifier {
            model: MODEL.into(),
            detail: "SSA allocator did not assign the validated return value".into(),
        })?;

        Ok(Self {
            instructions,
            result,
            value_locations: locations
                .into_iter()
                .enumerate()
                .map(|(index, location)| {
                    location.ok_or_else(|| JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!("SSA allocator did not assign value {index}").into(),
                    })
                })
                .collect::<JitResult<Vec<_>>>()?,
            spill_slot_count: spill_owners.len(),
            used_register_count,
            required_register_count,
            edge_moves,
        })
    }

    /// The parallel move a backend must perform on one outgoing edge before
    /// branching to its target.
    pub(crate) fn edge_moves(&self, block: BlockId, edge: usize) -> JitResult<&[EdgeMove]> {
        self.edge_moves
            .get(block.index())
            .and_then(|edges| edges.get(edge))
            .map(|steps| steps.as_ref())
            .ok_or_else(|| JitError::Verifier {
                model: MODEL.into(),
                detail: format!(
                    "SSA allocation has no move plan for edge {edge} of block {}",
                    block.index()
                )
                .into(),
            })
    }

    pub(crate) fn instructions(&self) -> &[AllocatedInstruction] {
        &self.instructions
    }

    pub(crate) fn result(&self) -> ValueLocation {
        self.result
    }

    pub(crate) fn location(&self, value: ValueId) -> JitResult<ValueLocation> {
        self.value_locations
            .get(value.index())
            .copied()
            .ok_or_else(|| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA allocation has no location for value {}", value.index())
                    .into(),
            })
    }

    pub(crate) fn spill_slot_count(&self) -> usize {
        self.spill_slot_count
    }

    #[cfg(test)]
    pub(crate) fn used_register_count(&self) -> usize {
        self.used_register_count
    }

    pub(crate) fn required_register_count(&self) -> usize {
        self.required_register_count
    }
}

fn allocate_spill_slot(owners: &mut Vec<Option<ValueId>>, value: ValueId) -> usize {
    if let Some(slot) = owners.iter().position(Option::is_none) {
        owners[slot] = Some(value);
        slot
    } else {
        owners.push(Some(value));
        owners.len() - 1
    }
}

fn required_register_count(instruction: &AllocatedInstruction) -> usize {
    let mut used = [false; LOGICAL_VALUE_REGISTER_COUNT];
    for (register, occupied) in used.iter_mut().enumerate() {
        *occupied = instruction.live_register_mask & (1_u32 << register) != 0;
    }
    if let ValueLocation::Register(register) = instruction.result {
        used[register] = true;
    }
    for operand in &instruction.operands {
        let register = match *operand {
            ValueLocation::Register(register) => Some(register),
            ValueLocation::Spill(_) => take_free_register(&mut used),
        };
        if let Some(register) = register {
            used[register] = true;
        }
    }
    if matches!(instruction.result, ValueLocation::Spill(_)) {
        let reuses_spilled_first =
            matches!(instruction.operands.first(), Some(ValueLocation::Spill(_)));
        if !reuses_spilled_first && let Some(register) = take_free_register(&mut used) {
            used[register] = true;
        }
    }
    // Legalization can request one scratch register immediately above the
    // logical operands. Count the lowest free register even when a particular
    // opcode does not use it: on Win64, merely touching XMM6-XMM15 requires
    // matching prologue and unwind preservation.
    if let Some(scratch) = used.iter().position(|occupied| !occupied) {
        used[scratch] = true;
    }
    used.iter()
        .rposition(|occupied| *occupied)
        .map_or(0, |last| last + 1)
}

fn take_free_register(used: &mut [bool; LOGICAL_VALUE_REGISTER_COUNT]) -> Option<usize> {
    used.iter().position(|occupied| !occupied)
}

pub(crate) fn dynamic_variable_inline_supported(len: usize, lower: i64) -> bool {
    let Ok(len_i64) = i64::try_from(len) else {
        return false;
    };
    let Some(upper) = lower
        .checked_add(len_i64)
        .and_then(|exclusive| exclusive.checked_sub(1))
    else {
        return false;
    };
    let supported = -INLINE_DYNAMIC_LOWER_ABS_LIMIT..=INLINE_DYNAMIC_LOWER_ABS_LIMIT;
    supported.contains(&lower) && supported.contains(&upper)
}

fn op_may_call(op: NativeOp) -> bool {
    matches!(
        op,
        NativeOp::BinaryMath(_)
            | NativeOp::IntegerCast
            | NativeOp::IntegerBinary(_)
            | NativeOp::IntegerShiftConst(_, _)
            | NativeOp::IntegerBinaryConst(_, _)
            | NativeOp::TableLookup(_)
            | NativeOp::TableDerivative(_)
            | NativeOp::LimiterPrevious(_)
            | NativeOp::LimiterStore(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::LaplaceStateDerivative(_)
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::TransitionStateDerivative(_)
            | NativeOp::SlewState(_)
            | NativeOp::SlewStateDerivative(_)
            | NativeOp::AbsDelayState(_)
            | NativeOp::AbsDelayStateMax(_)
            | NativeOp::AbsDelayStateDerivative(_)
            | NativeOp::AbsDelayStateDerivativeMax(_)
            | NativeOp::CrossState(_)
            | NativeOp::AboveState(_)
            | NativeOp::LastCrossingState(_)
            | NativeOp::DdtState(_)
            | NativeOp::DdtJacobian
            | NativeOp::IdtState(_)
            | NativeOp::IdtJacobian
            | NativeOp::IdtModState(_)
    ) || matches!(op, NativeOp::UnaryMath(op) if unary_math_uses_helper(op))
        || matches!(
            op,
            NativeOp::LoadVariableDyn { len, lower, .. }
                if !dynamic_variable_inline_supported(len, lower)
        )
}

fn op_reads_entry_args(op: NativeOp) -> bool {
    !matches!(
        op,
        NativeOp::Const(_)
            | NativeOp::Add
            | NativeOp::Sub
            | NativeOp::Mul
            | NativeOp::Div
            | NativeOp::AddConst(_)
            | NativeOp::SubConst(_)
            | NativeOp::MulConst(_)
            | NativeOp::DivConst(_)
            | NativeOp::SubFromConst(_)
            | NativeOp::DivFromConst(_)
            | NativeOp::Neg
            | NativeOp::Abs
            | NativeOp::Square
            | NativeOp::Sqrt
            | NativeOp::Compare(_)
            | NativeOp::CompareConst(_, _)
            | NativeOp::Logical(_)
            | NativeOp::LogicalConst(_, _)
            | NativeOp::IfElse
            | NativeOp::Extremum(_)
            | NativeOp::ExtremumConst(_, _)
            | NativeOp::ExtremumConstLhs(_, _)
            | NativeOp::UnaryMath(_)
            | NativeOp::BinaryMath(_)
            | NativeOp::WhiteNoise
            | NativeOp::FlickerNoise
    )
}

fn op_preserves_context_pointer_cache(op: NativeOp) -> bool {
    match op {
        NativeOp::LoadVariableDyn { len, lower, .. } => {
            dynamic_variable_inline_supported(len, lower)
        }
        _ => matches!(
            op,
            NativeOp::Const(_)
                | NativeOp::LoadParam(_)
                | NativeOp::LoadParamGiven(_)
                | NativeOp::LoadPortConnected(_)
                | NativeOp::LoadVoltage { .. }
                | NativeOp::LoadTemperature
                | NativeOp::LoadThermalVoltage
                | NativeOp::LoadTime
                | NativeOp::Analysis(_)
                | NativeOp::LoadMfactor
                | NativeOp::LoadCurrent(_)
                | NativeOp::LoadPriorCurrent(_)
                | NativeOp::LoadInternalVoltage(_)
                | NativeOp::LoadBranchUnknown(_)
                | NativeOp::LoadVariable(_)
                | NativeOp::Add
                | NativeOp::Sub
                | NativeOp::Mul
                | NativeOp::Div
                | NativeOp::AddConst(_)
                | NativeOp::SubConst(_)
                | NativeOp::MulConst(_)
                | NativeOp::DivConst(_)
                | NativeOp::SubFromConst(_)
                | NativeOp::DivFromConst(_)
                | NativeOp::Neg
                | NativeOp::Abs
                | NativeOp::Square
                | NativeOp::Sqrt
                | NativeOp::Compare(_)
                | NativeOp::CompareConst(_, _)
                | NativeOp::Logical(_)
                | NativeOp::LogicalConst(_, _)
                | NativeOp::IfElse
                | NativeOp::Extremum(_)
                | NativeOp::ExtremumConst(_, _)
                | NativeOp::ExtremumConstLhs(_, _)
                | NativeOp::UnaryMath(UnaryMathOp::Floor | UnaryMathOp::Ceil)
                | NativeOp::WhiteNoise
                | NativeOp::FlickerNoise
        ),
    }
}

fn op_reads_context(op: NativeOp) -> bool {
    op_reads_entry_args(op)
}

fn op_reads_state(op: NativeOp) -> bool {
    matches!(
        op,
        NativeOp::LimiterPrevious(_)
            | NativeOp::LimiterStore(_)
            | NativeOp::LimitState(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::LaplaceStateDerivative(_)
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::TransitionStateDerivative(_)
            | NativeOp::SlewState(_)
            | NativeOp::SlewStateDerivative(_)
            | NativeOp::AbsDelayState(_)
            | NativeOp::AbsDelayStateMax(_)
            | NativeOp::AbsDelayStateDerivative(_)
            | NativeOp::AbsDelayStateDerivativeMax(_)
            | NativeOp::CrossState(_)
            | NativeOp::AboveState(_)
            | NativeOp::LastCrossingState(_)
            | NativeOp::DdtState(_)
            | NativeOp::DdtJacobian
            | NativeOp::IdtState(_)
            | NativeOp::IdtJacobian
            | NativeOp::IdtModState(_)
    )
}

fn op_writes_state(op: NativeOp) -> bool {
    matches!(
        op,
        NativeOp::LimiterStore(_)
            | NativeOp::LimitState(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::SlewState(_)
            | NativeOp::AbsDelayState(_)
            | NativeOp::AbsDelayStateMax(_)
            | NativeOp::AbsDelayStateDerivative(_)
            | NativeOp::AbsDelayStateDerivativeMax(_)
            | NativeOp::CrossState(_)
            | NativeOp::AboveState(_)
            | NativeOp::DdtState(_)
            | NativeOp::IdtState(_)
            | NativeOp::IdtModState(_)
    )
}

fn op_may_fail(op: NativeOp) -> bool {
    matches!(
        op,
        NativeOp::LoadParamGiven(_)
            | NativeOp::LoadPortConnected(_)
            | NativeOp::LoadCurrent(_)
            | NativeOp::LoadPriorCurrent(_)
            | NativeOp::LoadVariableDyn { .. }
            | NativeOp::TableLookup(_)
            | NativeOp::TableDerivative(_)
            | NativeOp::LimiterPrevious(_)
            | NativeOp::LimiterStore(_)
            | NativeOp::LimitState(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::LaplaceStateDerivative(_)
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::TransitionStateDerivative(_)
            | NativeOp::SlewState(_)
            | NativeOp::SlewStateDerivative(_)
            | NativeOp::AbsDelayState(_)
            | NativeOp::AbsDelayStateMax(_)
            | NativeOp::AbsDelayStateDerivative(_)
            | NativeOp::AbsDelayStateDerivativeMax(_)
            | NativeOp::CrossState(_)
            | NativeOp::AboveState(_)
            | NativeOp::LastCrossingState(_)
            | NativeOp::DdtState(_)
            | NativeOp::DdtJacobian
            | NativeOp::IdtState(_)
            | NativeOp::IdtJacobian
            | NativeOp::IdtModState(_)
            | NativeOp::IntegerBinary(IntegerBinaryOp::Shl | IntegerBinaryOp::Shr)
    )
}

fn unary_math_uses_helper(op: UnaryMathOp) -> bool {
    !matches!(op, UnaryMathOp::Floor | UnaryMathOp::Ceil)
}

#[cfg(test)]
mod tests {
    use super::{
        ALLOCATABLE_VALUE_REGISTERS, AllocatedInstruction, AssignmentProgram, BlockId,
        BuilderTerminator, Edge, Effects, Instruction, MoveStep, Program, ProgramBuilder,
        RegisterAllocation, RegisterBank, Terminator, ValueId, ValueLocation, ValueType,
        plan_shared_outputs, required_register_count, sequence_parallel_move,
    };
    use crate::jit::expr::{NativeOp, NativeProgram, native_op_stack_effect};
    use std::collections::HashMap;

    /// The bank as a backend that preserves no floating-point register sees
    /// it: AArch64 and System V x64.
    const CALLER_SAVED_BANK: RegisterBank =
        RegisterBank::all_caller_saved(ALLOCATABLE_VALUE_REGISTERS);
    /// The same bank as Win64 sees it, where XMM6-XMM9 survive a helper call.
    const WIN64_BANK: RegisterBank =
        RegisterBank::with_callee_saved_from(ALLOCATABLE_VALUE_REGISTERS, 6);

    fn program(ops: Vec<NativeOp>, max_stack_depth: usize) -> NativeProgram {
        NativeProgram::from_ops_for_test(ops, max_stack_depth, Vec::new(), Vec::new())
    }

    fn single_block(
        instructions: Vec<Instruction>,
        result: ValueId,
        maximum_stack_depth: usize,
    ) -> Program {
        Program::single_block(instructions, result, maximum_stack_depth)
            .expect("hand-built single-block SSA")
    }

    #[test]
    fn lifts_postfix_values_to_explicit_topological_operands() {
        let lowered = Program::lower(&program(
            vec![
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::Mul,
                NativeOp::Const(4.0),
                NativeOp::Add,
            ],
            2,
        ))
        .expect("valid SSA");

        assert_eq!(
            lowered.instructions()[2].operands(),
            &[ValueId(0), ValueId(1)]
        );
        assert_eq!(
            lowered.instructions()[4].operands(),
            &[ValueId(2), ValueId(3)]
        );
        assert_eq!(lowered.blocks().len(), 1);
        assert_eq!(
            lowered.blocks()[0].terminator(),
            &Terminator::Return(ValueId(4))
        );
        assert_eq!(lowered.maximum_stack_depth(), 2);
    }

    #[test]
    fn formal_effects_distinguish_pure_math_from_state_and_calls() {
        assert!(Effects::for_op(NativeOp::Mul).is_semantically_pure());
        assert!(
            Effects::for_op(NativeOp::UnaryMath(crate::jit::expr::UnaryMathOp::Exp)).may_call()
        );
        let state = Effects::for_op(NativeOp::IdtState(0));
        assert!(state.may_call());
        assert!(!state.is_semantically_pure());
        assert!(state.clobbers_context_pointer_cache());

        let laplace_derivative = Effects::for_op(NativeOp::LaplaceStateDerivative(0));
        assert!(laplace_derivative.contains(Effects::READ_STATE));
        assert!(!laplace_derivative.contains(Effects::WRITE_STATE));
        assert!(laplace_derivative.may_call());
        assert!(laplace_derivative.may_fail());

        let transition_derivative = Effects::for_op(NativeOp::TransitionStateDerivative(0));
        assert!(transition_derivative.contains(Effects::READ_STATE));
        assert!(!transition_derivative.contains(Effects::WRITE_STATE));
        assert!(transition_derivative.may_call());
        assert!(transition_derivative.may_fail());
    }

    #[test]
    fn rejects_stack_metadata_that_disagrees_with_explicit_ssa() {
        let invalid = program(vec![NativeOp::Const(1.0)], 2);
        let error = Program::lower(&invalid).expect_err("stale depth metadata must fail");
        assert!(error.to_string().contains("SSA observed stack depth 1"));
    }

    #[test]
    fn local_cse_shares_exact_pure_subexpressions_but_stops_at_effect_barriers() {
        let shared = Program::lower(&program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::Add,
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::Add,
                NativeOp::Mul,
            ],
            3,
        ))
        .expect("pure CSE program");
        assert_eq!(shared.instructions().len(), 4);

        let barrier = Program::lower(&program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::Const(1.0),
                NativeOp::LimiterStore(0),
                NativeOp::LoadVariable(0),
                NativeOp::Add,
            ],
            2,
        ))
        .expect("state barrier program");
        assert_eq!(barrier.instructions().len(), 5);
    }

    #[test]
    fn identical_if_else_arms_alias_exactly_and_dead_pure_conditions_disappear() {
        let pure = Program::lower(&program(
            vec![
                NativeOp::Const(1.0),
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(0),
                NativeOp::IfElse,
            ],
            3,
        ))
        .expect("identical-arm conditional");
        assert_eq!(pure.instructions().len(), 1);
        assert_eq!(pure.instructions()[0].op(), NativeOp::LoadVariable(0));

        let failing_condition = Program::lower(&program(
            vec![
                NativeOp::LoadParamGiven(0),
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(0),
                NativeOp::IfElse,
            ],
            3,
        ))
        .expect("effectful identical-arm conditional");
        assert_eq!(failing_condition.instructions().len(), 2);
        assert_eq!(
            failing_condition.instructions()[0].op(),
            NativeOp::LoadParamGiven(0)
        );
    }

    #[test]
    fn assignment_ssa_shares_across_outputs_and_keeps_publication_liveness() {
        let first = program(vec![NativeOp::LoadVariable(7), NativeOp::Square], 1);
        let second = program(
            vec![
                NativeOp::LoadVariable(7),
                NativeOp::Square,
                NativeOp::Const(1.0),
                NativeOp::Add,
            ],
            2,
        );
        let batch =
            AssignmentProgram::lower(&[(0, &first), (1, &second)]).expect("shared assignment SSA");
        assert_eq!(batch.program().instructions().len(), 4);
        assert_eq!(batch.outputs()[0].instruction_end(), 2);
        assert_eq!(batch.outputs()[1].instruction_end(), 4);

        let allocation = RegisterAllocation::build_for_assignments(&batch, CALLER_SAVED_BANK)
            .expect("batch allocation");
        let first_location = allocation
            .location(batch.outputs()[0].value())
            .expect("first output location");
        assert_eq!(
            first_location,
            allocation.instructions()[1].result(),
            "the first output must still occupy its assigned location at publication"
        );
    }

    #[test]
    fn equivalent_same_boundary_outputs_are_grouped_by_target_address() {
        let constant = program(vec![NativeOp::Const(2.0)], 1);
        let batch =
            AssignmentProgram::lower(&[(5000, &constant), (2, &constant), (300, &constant)])
                .expect("constant assignment batch");
        assert_eq!(batch.program().instructions().len(), 1);
        assert_eq!(
            batch
                .outputs()
                .iter()
                .map(|output| output.variable_index())
                .collect::<Vec<_>>(),
            [2, 300, 5000]
        );
    }

    #[test]
    fn shares_identical_effect_safe_jacobian_outputs() {
        let first = Program::lower(&program(
            vec![NativeOp::LoadVariable(0), NativeOp::MulConst(2.0)],
            1,
        ))
        .expect("first SSA program");
        let second = first.clone();
        let distinct = Program::lower(&program(vec![NativeOp::LoadVariable(1)], 1))
            .expect("distinct SSA program");

        let groups = plan_shared_outputs(&[first, second, distinct]);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].representative(), 0);
        assert_eq!(groups[0].outputs(), &[0, 1]);
        assert_eq!(groups[1].representative(), 2);
        assert_eq!(groups[1].outputs(), &[2]);
    }

    #[test]
    fn never_shares_calls_failures_or_state_writes() {
        for op in [
            NativeOp::LoadParamGiven(0),
            NativeOp::UnaryMath(crate::jit::expr::UnaryMathOp::Exp),
            NativeOp::LimitState(0),
        ] {
            let operand_count = native_op_stack_effect(&op).0;
            let mut ops = vec![NativeOp::Const(1.0); operand_count];
            ops.push(op);
            let first =
                Program::lower(&program(ops, operand_count.max(1))).expect("effectful SSA program");
            let groups = plan_shared_outputs(&[first.clone(), first]);
            assert_eq!(groups.len(), 2, "{op:?} must remain a sharing barrier");
        }
    }

    #[test]
    fn liveness_allocation_reclaims_registers_at_last_use() {
        let lowered = Program::lower(&program(
            vec![
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::Add,
                NativeOp::Const(4.0),
                NativeOp::Mul,
            ],
            2,
        ))
        .expect("valid SSA");
        let allocation =
            RegisterAllocation::build(&lowered, CALLER_SAVED_BANK).expect("register allocation");

        assert_eq!(allocation.used_register_count(), 2);
        assert_eq!(allocation.spill_slot_count(), 0);
        assert_eq!(
            allocation.instructions()[2].result(),
            allocation.instructions()[0].result(),
            "a binary result should reuse its dead left operand register"
        );
        assert_eq!(
            allocation.instructions()[4].result(),
            allocation.instructions()[2].result()
        );
    }

    #[test]
    fn liveness_allocation_preserves_shared_values_until_their_last_use() {
        let instructions = vec![
            Instruction {
                result: ValueId(0),
                value_type: ValueType::F64,
                op: NativeOp::LoadVariable(0),
                operands: Box::new([]),
                effects: Effects::for_op(NativeOp::LoadVariable(0)),
            },
            Instruction {
                result: ValueId(1),
                value_type: ValueType::F64,
                op: NativeOp::Square,
                operands: Box::new([ValueId(0)]),
                effects: Effects::for_op(NativeOp::Square),
            },
            Instruction {
                result: ValueId(2),
                value_type: ValueType::F64,
                op: NativeOp::Add,
                operands: Box::new([ValueId(0), ValueId(1)]),
                effects: Effects::for_op(NativeOp::Add),
            },
        ];
        let shared = single_block(instructions, ValueId(2), 2);
        let allocation =
            RegisterAllocation::build(&shared, CALLER_SAVED_BANK).expect("register allocation");

        assert_ne!(
            allocation.instructions()[0].result(),
            allocation.instructions()[1].result(),
            "the first value is still live when the square is evaluated"
        );
        assert_eq!(
            allocation.instructions()[2].operands(),
            &[
                allocation.instructions()[0].result(),
                allocation.instructions()[1].result(),
            ]
        );
    }

    /// `live` values defined before one `exp()` and folded in after it.
    ///
    /// This is the shape every compact model produces: bias-dependent terms
    /// computed up front, a transcendental in the middle, and the terms
    /// combined once it returns. The call's own operand is loaded last so that
    /// it is consumed before the call rather than living across it.
    fn program_with_values_live_across_a_call(live: usize) -> Program {
        let value = |index: usize| ValueId(u32::try_from(index).expect("test value index"));
        let mut instructions = Vec::new();
        for index in 0..=live {
            instructions.push(Instruction {
                result: value(index),
                value_type: ValueType::F64,
                op: NativeOp::LoadVariable(index),
                operands: Box::new([]),
                effects: Effects::for_op(NativeOp::LoadVariable(index)),
            });
        }
        let exp = NativeOp::UnaryMath(crate::jit::expr::UnaryMathOp::Exp);
        instructions.push(Instruction {
            result: value(live + 1),
            value_type: ValueType::F64,
            op: exp,
            operands: Box::new([value(live)]),
            effects: Effects::for_op(exp),
        });
        let mut accumulator = value(live + 1);
        for index in 0..live {
            let sum = value(live + 2 + index);
            instructions.push(Instruction {
                result: sum,
                value_type: ValueType::F64,
                op: NativeOp::Add,
                operands: Box::new([accumulator, value(index)]),
                effects: Effects::for_op(NativeOp::Add),
            });
            accumulator = sum;
        }

        single_block(instructions, accumulator, 2)
    }

    #[test]
    fn values_live_across_a_call_spill_when_the_bank_preserves_nothing() {
        let across_call = program_with_values_live_across_a_call(1);
        let allocation = RegisterAllocation::build(&across_call, CALLER_SAVED_BANK)
            .expect("register allocation");

        assert!(matches!(
            allocation.instructions()[0].result(),
            ValueLocation::Spill(_)
        ));
        assert_eq!(allocation.spill_slot_count(), 1);
    }

    #[test]
    fn values_live_across_a_call_take_a_preserved_register_when_the_bank_has_one() {
        let across_call = program_with_values_live_across_a_call(1);
        let allocation =
            RegisterAllocation::build(&across_call, WIN64_BANK).expect("register allocation");

        assert_eq!(
            allocation.instructions()[0].result(),
            ValueLocation::Register(6),
            "the first preserved register is where a value live across the call belongs"
        );
        assert_eq!(allocation.spill_slot_count(), 0);
        assert!(
            allocation.required_register_count() > 6,
            "reaching a preserved register has to oblige the prologue to save it"
        );
    }

    #[test]
    fn cross_call_values_spill_once_every_preserved_register_is_taken() {
        let across_call = program_with_values_live_across_a_call(5);
        let allocation =
            RegisterAllocation::build(&across_call, WIN64_BANK).expect("register allocation");

        let results: Vec<ValueLocation> = allocation.instructions()[..5]
            .iter()
            .map(AllocatedInstruction::result)
            .collect();
        assert_eq!(
            results,
            vec![
                ValueLocation::Register(6),
                ValueLocation::Register(7),
                ValueLocation::Register(8),
                ValueLocation::Register(9),
                ValueLocation::Spill(0),
            ],
            "four values fit the preserved registers and the fifth still has to spill"
        );
        assert_eq!(allocation.spill_slot_count(), 1);
    }

    #[test]
    fn a_bank_whose_preserved_registers_start_past_its_end_allocates_as_all_caller_saved() {
        // System V x64 arrives here: it preserves no XMM register at all, so
        // its first nonvolatile index sits past every allocatable one and the
        // allocation it gets has to stay exactly what it was.
        let system_v = RegisterBank::with_callee_saved_from(ALLOCATABLE_VALUE_REGISTERS, 16);
        assert_eq!(
            system_v,
            RegisterBank::all_caller_saved(ALLOCATABLE_VALUE_REGISTERS)
        );

        let across_call = program_with_values_live_across_a_call(3);
        assert_eq!(
            RegisterAllocation::build(&across_call, system_v).expect("System V allocation"),
            RegisterAllocation::build(&across_call, CALLER_SAVED_BANK)
                .expect("caller-saved allocation"),
        );
    }

    #[test]
    fn legalization_reserves_a_register_result_before_loading_spilled_operands() {
        let instruction = AllocatedInstruction {
            operands: Box::new([ValueLocation::Register(0), ValueLocation::Spill(0)]),
            result: ValueLocation::Register(1),
            live_register_mask: 1,
        };

        assert_eq!(
            required_register_count(&instruction),
            4,
            "register 1 is the future result, register 2 materializes the spill, and register 3 remains available for opcode legalization"
        );
    }

    // ---------------------------------------------------------------- blocks

    fn register(index: usize) -> ValueLocation {
        ValueLocation::Register(index)
    }

    fn moves(pairs: &[(ValueLocation, ValueLocation)]) -> Vec<MoveStep> {
        sequence_parallel_move(pairs).expect("sequenced parallel move")
    }

    /// Replay a move plan against a model of the machine's locations and
    /// report where each destination ends up.
    fn replay(plan: &[MoveStep], initial: &[(ValueLocation, u32)]) -> HashMap<ValueLocation, u32> {
        let mut state: HashMap<ValueLocation, u32> = initial.iter().copied().collect();
        let mut scratch = None;
        for step in plan {
            match *step {
                MoveStep::Move { from, to } => {
                    let value = state
                        .get(&from)
                        .copied()
                        .expect("move reads a live location");
                    state.insert(to, value);
                }
                MoveStep::SaveToScratch { from } => {
                    assert!(scratch.is_none(), "only one value may occupy the scratch");
                    scratch = state.get(&from).copied();
                    assert!(scratch.is_some(), "scratch save reads a live location");
                }
                MoveStep::RestoreFromScratch { to } => {
                    let value = scratch.take().expect("scratch restore follows a save");
                    state.insert(to, value);
                }
            }
        }
        state
    }

    #[test]
    fn parallel_move_orders_chains_and_needs_no_scratch() {
        // r1 <- r0 and r2 <- r1 must run in the order that preserves r1.
        let plan = moves(&[(register(0), register(1)), (register(1), register(2))]);
        assert_eq!(
            plan,
            vec![
                MoveStep::Move {
                    from: register(1),
                    to: register(2)
                },
                MoveStep::Move {
                    from: register(0),
                    to: register(1)
                },
            ]
        );
        let final_state = replay(&plan, &[(register(0), 10), (register(1), 11)]);
        assert_eq!(final_state[&register(1)], 10);
        assert_eq!(final_state[&register(2)], 11);
    }

    #[test]
    fn parallel_move_breaks_a_swap_through_the_scratch_slot() {
        let plan = moves(&[(register(0), register(1)), (register(1), register(0))]);
        assert!(
            plan.iter()
                .any(|step| matches!(step, MoveStep::SaveToScratch { .. })),
            "a two-cycle cannot be realized by sequential moves alone"
        );
        let final_state = replay(&plan, &[(register(0), 10), (register(1), 11)]);
        assert_eq!(final_state[&register(0)], 11);
        assert_eq!(final_state[&register(1)], 10);
    }

    #[test]
    fn parallel_move_rotates_a_three_cycle_and_mixed_locations() {
        let cycle = [
            (register(0), register(1)),
            (register(1), ValueLocation::Spill(0)),
            (ValueLocation::Spill(0), register(0)),
        ];
        let plan = moves(&cycle);
        let final_state = replay(
            &plan,
            &[
                (register(0), 10),
                (register(1), 11),
                (ValueLocation::Spill(0), 12),
            ],
        );
        assert_eq!(final_state[&register(1)], 10);
        assert_eq!(final_state[&ValueLocation::Spill(0)], 11);
        assert_eq!(final_state[&register(0)], 12);
    }

    #[test]
    fn parallel_move_fans_one_source_out_and_drops_identities() {
        let plan = moves(&[
            (register(0), register(1)),
            (register(0), register(2)),
            (register(3), register(3)),
        ]);
        assert_eq!(plan.len(), 2, "a location moved onto itself emits nothing");
        let final_state = replay(&plan, &[(register(0), 7), (register(3), 9)]);
        assert_eq!(final_state[&register(1)], 7);
        assert_eq!(final_state[&register(2)], 7);
    }

    #[test]
    fn parallel_move_rejects_two_parameters_bound_to_one_location() {
        let error =
            sequence_parallel_move(&[(register(0), register(2)), (register(1), register(2))])
                .expect_err("aliased destinations are not a parallel move");
        assert!(error.to_string().contains("two block parameters"));
    }

    /// `variable(0) ? load_dyn : constant`, whose taken arm owns the failing
    /// dynamic load.
    fn conditional_with_a_failing_arm() -> Program {
        Program::lower(&program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::LoadVariableDyn {
                    base: 4,
                    len: 2,
                    lower: 0,
                },
                NativeOp::Const(0.0),
                NativeOp::IfElse,
            ],
            3,
        ))
        .expect("valid SSA")
    }

    #[test]
    fn conditional_splitting_sinks_an_arm_only_operand_into_its_block() {
        let flat = conditional_with_a_failing_arm();
        assert!(flat.is_single_block());
        let split = flat
            .with_branching_conditionals()
            .expect("split conditional");

        assert_eq!(split.blocks().len(), 4, "branch, then, else, join");
        assert_eq!(split.instructions().len(), flat.instructions().len() - 1);
        assert_eq!(split.block_parameter_count(), 1);

        let then_block = &split.blocks()[1];
        let sunk =
            &split.instructions()[then_block.instruction_start()..then_block.instruction_end()];
        assert_eq!(
            sunk.len(),
            2,
            "the dynamic load and the index it alone consumes both sink"
        );
        assert!(matches!(sunk[0].op(), NativeOp::LoadVariable(1)));
        assert!(matches!(sunk[1].op(), NativeOp::LoadVariableDyn { .. }));
        assert!(
            sunk[1].effects().may_fail(),
            "the point of the branch form is that this runs only when taken"
        );

        let else_block = &split.blocks()[2];
        assert_eq!(
            else_block.instruction_end() - else_block.instruction_start(),
            1,
            "the constant arm keeps its own constant"
        );
        assert!(matches!(
            split.blocks()[0].terminator(),
            Terminator::Branch { .. }
        ));
        assert!(matches!(
            split.blocks()[3].terminator(),
            Terminator::Return(_)
        ));
    }

    #[test]
    fn conditional_splitting_keeps_a_shared_operand_unconditional() {
        // The dynamic load feeds both the arm and the final sum, so it escapes
        // the arm and must stay in the entry block.
        let flat = Program::lower(&program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::Square,
                NativeOp::Const(0.0),
                NativeOp::IfElse,
                NativeOp::LoadVariable(1),
                NativeOp::Square,
                NativeOp::Add,
            ],
            3,
        ))
        .expect("valid SSA");
        let split = flat
            .with_branching_conditionals()
            .expect("split conditional");
        let entry = &split.blocks()[0];
        assert!(
            split.instructions()[entry.instruction_start()..entry.instruction_end()]
                .iter()
                .any(|instruction| matches!(instruction.op(), NativeOp::Square)),
            "a value read on both paths stays unconditional"
        );
        assert_eq!(
            split.blocks()[1].instruction_end() - split.blocks()[1].instruction_start(),
            0,
            "the taken arm owns nothing of its own"
        );
    }

    #[test]
    fn conditional_splitting_never_sinks_a_state_write() {
        let flat = Program::lower(&program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::DdtState(0),
                NativeOp::Const(0.0),
                NativeOp::IfElse,
            ],
            3,
        ))
        .expect("valid SSA");
        let split = flat
            .with_branching_conditionals()
            .expect("split conditional");
        let entry = &split.blocks()[0];
        assert!(
            split.instructions()[entry.instruction_start()..entry.instruction_end()]
                .iter()
                .any(|instruction| instruction.effects().writes_state()),
            "analog state must advance whichever arm the condition selects"
        );
    }

    #[test]
    fn verifier_rejects_a_state_write_on_a_conditional_block() {
        let flat = conditional_with_a_failing_arm();
        let mut split = flat
            .with_branching_conditionals()
            .expect("split conditional");
        let sunk = split.blocks()[1].instruction_start();
        split.instructions[sunk].op = NativeOp::DdtState(0);
        split.instructions[sunk].effects = Effects::for_op(NativeOp::DdtState(0));
        let error = split
            .validate()
            .expect_err("a conditional state write must not verify");
        assert!(error.to_string().contains("writes state"));
    }

    #[test]
    fn verifier_rejects_unstructured_back_edges_unreachable_blocks_and_edge_arity() {
        let split = conditional_with_a_failing_arm()
            .with_branching_conditionals()
            .expect("split conditional");

        // The *second* arm of a diamond jumping back to the block that
        // branched into it is a back edge to a block that dominates it, so it
        // closes a natural loop — but the first arm then sits inside that
        // loop's layout range without belonging to it, which is the shape the
        // contiguity rule exists to refuse. (The first arm jumping back would
        // be an ordinary `while`, and is admitted.)
        let mut back_edge = split.clone();
        back_edge.blocks[2].terminator = Terminator::Jump(Edge::new(BlockId(0), Vec::new()));
        assert!(
            back_edge
                .validate()
                .expect_err("a loop that is not laid out contiguously is refused")
                .to_string()
                .contains("not laid out contiguously")
        );

        let mut arity = split.clone();
        arity.blocks[1].terminator = Terminator::Jump(Edge::new(BlockId(3), Vec::new()));
        assert!(
            arity
                .validate()
                .expect_err("an edge must bind every parameter")
                .to_string()
                .contains("argument(s) to 1 parameter(s)")
        );

        let mut unreachable = split.clone();
        let Terminator::Branch {
            condition,
            else_edge,
            ..
        } = unreachable.blocks[0].terminator.clone()
        else {
            panic!("entry block branches");
        };
        unreachable.blocks[0].terminator = Terminator::Branch {
            condition,
            then_edge: else_edge.clone(),
            else_edge,
        };
        assert!(
            unreachable
                .validate()
                .expect_err("a block nothing reaches is dead code")
                .to_string()
                .contains("unreachable")
        );
    }

    #[test]
    fn block_parameters_and_arguments_take_distinct_live_locations() {
        let split = conditional_with_a_failing_arm()
            .with_branching_conditionals()
            .expect("split conditional");
        let allocation =
            RegisterAllocation::build(&split, CALLER_SAVED_BANK).expect("register allocation");

        let parameter = split.parameters(&split.blocks()[3])[0];
        let parameter_location = allocation
            .location(parameter.value())
            .expect("allocated block parameter");
        let then_plan = allocation
            .edge_moves(BlockId(1), 0)
            .expect("then edge move plan");
        assert_eq!(then_plan.len(), 1);
        assert_eq!(
            then_plan[0].from(),
            allocation
                .location(ValueId(1))
                .expect("allocated arm value")
        );
        assert_eq!(then_plan[0].to(), parameter_location);
        assert_ne!(
            then_plan[0].from(),
            then_plan[0].to(),
            "the parameter takes a location no live argument occupies"
        );
        assert!(
            allocation
                .edge_moves(BlockId(0), 0)
                .expect("branch edges carry no arguments")
                .is_empty()
        );
    }

    #[test]
    fn single_block_allocation_is_unchanged_by_the_block_model() {
        // Every quantity the emitters read is derived from a position axis
        // that collapses to the old instruction indices when there is one
        // block, so a one-block program allocates exactly as it always did.
        let across_call = program_with_values_live_across_a_call(3);
        let allocation =
            RegisterAllocation::build(&across_call, WIN64_BANK).expect("register allocation");
        assert_eq!(
            allocation.instructions().len(),
            across_call.instructions().len()
        );
        assert_eq!(allocation.edge_moves.len(), 1);
        assert!(allocation.edge_moves[0].is_empty());
    }

    #[test]
    fn a_natural_loop_validates_and_reports_its_layout_range() {
        let program = Program::loop_fixture_for_test(20.0, 3.0).expect("validated loop program");
        let loops = program.loop_ranges().expect("loop ranges");
        assert_eq!(loops.len(), 1);
        assert_eq!(loops[0].header(), BlockId::new(1).expect("header"));
        assert_eq!(loops[0].latch(), BlockId::new(2).expect("latch"));
    }

    #[test]
    fn a_loop_back_edge_is_where_the_allocator_reserves_the_cycle_scratch() {
        let program = Program::loop_fixture_for_test(20.0, 3.0).expect("validated loop program");
        let allocation =
            RegisterAllocation::build(&program, CALLER_SAVED_BANK).expect("allocated loop");
        let latch = BlockId::new(2).expect("latch");
        let moves = allocation.edge_moves(latch, 0).expect("back-edge plan");
        let scratch = allocation.spill_slot_count();
        assert!(scratch > 0, "the cycle reserves one spill slot");
        assert!(
            moves
                .iter()
                .any(|step| step.to() == ValueLocation::Spill(scratch - 1))
                && moves
                    .iter()
                    .any(|step| step.from() == ValueLocation::Spill(scratch - 1)),
            "the back edge stages one value through the reserved slot: {moves:?}"
        );
    }

    #[test]
    fn a_value_read_inside_a_loop_keeps_its_location_across_the_whole_loop() {
        // `k` is defined before the loop and last read in the middle of the
        // body. Everything the body defines after that read would be free to
        // take its register under a plain linear scan, and would then be
        // reading its own result on the next iteration.
        let program = Program::loop_fixture_for_test(20.0, 3.0).expect("validated loop program");
        let allocation =
            RegisterAllocation::build(&program, CALLER_SAVED_BANK).expect("allocated loop");
        let scale = allocation
            .location(ValueId::new(0).expect("k"))
            .expect("k location");
        for value in [1_usize, 5, 6, 7] {
            let other = allocation
                .location(ValueId::new(value).expect("value"))
                .expect("value location");
            assert_ne!(
                other, scale,
                "value {value} took the location a loop-carried read still needs"
            );
        }
    }

    #[test]
    fn an_irreducible_back_edge_is_refused_by_name() {
        // Two entries into one cycle: block 1 and block 2 each branch into the
        // other, and neither dominates the other.
        let mut builder = ProgramBuilder::new(&[Vec::new(), Vec::new(), Vec::new(), Vec::new()])
            .expect("builder");
        let entry = BlockId::new(0).expect("entry");
        let left = BlockId::new(1).expect("left");
        let right = BlockId::new(2).expect("right");
        let exit = BlockId::new(3).expect("exit");
        builder.begin_block(entry).expect("open entry");
        let condition = builder
            .push(NativeOp::Const(1.0), &[], ValueType::F64)
            .expect("condition");
        builder
            .end_block(BuilderTerminator::Branch {
                condition,
                then_target: left,
                then_arguments: Vec::new(),
                else_target: right,
                else_arguments: Vec::new(),
            })
            .expect("close entry");
        builder.begin_block(left).expect("open left");
        builder
            .end_block(BuilderTerminator::Jump {
                target: right,
                arguments: Vec::new(),
            })
            .expect("close left");
        builder.begin_block(right).expect("open right");
        builder
            .end_block(BuilderTerminator::Branch {
                condition,
                then_target: left,
                then_arguments: Vec::new(),
                else_target: exit,
                else_arguments: Vec::new(),
            })
            .expect("close right");
        builder.begin_block(exit).expect("open exit");
        builder
            .end_block(BuilderTerminator::Return(condition))
            .expect("close exit");
        let error = builder
            .finish(entry, exit)
            .expect_err("an irreducible graph is not admitted");
        assert!(
            error.to_string().contains("irreducible"),
            "unexpected refusal: {error}"
        );
    }

    #[test]
    fn a_state_write_inside_a_loop_is_refused_by_name() {
        let mut builder = ProgramBuilder::new(&[Vec::new(), Vec::new(), Vec::new(), Vec::new()])
            .expect("builder");
        let entry = BlockId::new(0).expect("entry");
        let header = BlockId::new(1).expect("header");
        let body = BlockId::new(2).expect("body");
        let exit = BlockId::new(3).expect("exit");
        builder.begin_block(entry).expect("open entry");
        let one = builder
            .push(NativeOp::Const(1.0), &[], ValueType::F64)
            .expect("constant");
        builder
            .end_block(BuilderTerminator::Jump {
                target: header,
                arguments: Vec::new(),
            })
            .expect("close entry");
        builder.begin_block(header).expect("open header");
        builder
            .end_block(BuilderTerminator::Branch {
                condition: one,
                then_target: body,
                then_arguments: Vec::new(),
                else_target: exit,
                else_arguments: Vec::new(),
            })
            .expect("close header");
        builder.begin_block(body).expect("open body");
        builder
            .push(NativeOp::DdtState(0), &[one], ValueType::F64)
            .expect("ddt inside the loop");
        builder
            .end_block(BuilderTerminator::Jump {
                target: header,
                arguments: Vec::new(),
            })
            .expect("close body");
        builder.begin_block(exit).expect("open exit");
        builder
            .end_block(BuilderTerminator::Return(one))
            .expect("close exit");
        let error = builder
            .finish(entry, exit)
            .expect_err("an analog operator's record cannot advance once per iteration");
        assert!(
            error.to_string().contains("once per iteration"),
            "unexpected refusal: {error}"
        );
    }

    #[test]
    fn an_edge_move_plan_that_needs_the_scratch_realizes_it_as_the_reserved_spill_slot() {
        let cycle =
            sequence_parallel_move(&[(register(0), register(1)), (register(1), register(0))])
                .expect("sequenced swap");
        assert!(super::needs_scratch(&cycle));
        let moves = super::realize_edge_moves(&cycle, Some(7)).expect("realized swap");
        assert_eq!(
            moves
                .iter()
                .map(|step| (step.from(), step.to()))
                .collect::<Vec<_>>(),
            vec![
                (register(0), ValueLocation::Spill(7)),
                (register(1), register(0)),
                (ValueLocation::Spill(7), register(1)),
            ],
        );
    }

    #[test]
    fn realizing_a_cycle_without_a_reserved_slot_is_refused_by_name() {
        let cycle =
            sequence_parallel_move(&[(register(0), register(1)), (register(1), register(0))])
                .expect("sequenced swap");
        let error = super::realize_edge_moves(&cycle, None)
            .expect_err("a cycle cannot be realized without the reserved slot");
        assert!(error.to_string().contains("no scratch slot was reserved"));
    }
}
