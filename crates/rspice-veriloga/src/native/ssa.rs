//! Typed, effect-annotated SSA consumed by every native machine backend.
//!
//! Architecture-neutral lowering currently delivers postfix `NativeOp`s.  The
//! native backends immediately lift that stream into explicit values so machine
//! code generation never has to trust an implicit operand stack.  Expressions
//! are single-block today; keeping the block and terminator explicit makes
//! control-flow extension possible without changing the value model.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

use crate::jit::expr::{
    IntegerBinaryOp, NativeOp, NativeProgram, UnaryMathOp, native_op_stack_effect,
};
use crate::jit::value_cache::{native_op_hash, native_ops_are_codegen_identical};
use crate::jit::{JitError, JitResult};
use std::collections::HashMap;

const MODEL: &str = "native-ssa";
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Terminator {
    Return(ValueId),
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
    instruction_start: usize,
    instruction_end: usize,
    terminator: Terminator,
}

impl BasicBlock {
    #[cfg(test)]
    fn terminator(&self) -> Terminator {
        self.terminator
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Program {
    entry: BlockId,
    block: BasicBlock,
    instructions: Vec<Instruction>,
    maximum_stack_depth: usize,
}

impl Program {
    pub(crate) fn lower(program: &NativeProgram) -> JitResult<Self> {
        let mut lowerer = ProgramLowerer::with_capacity(program.ops().len());
        let (result, maximum_stack_depth) = lowerer.append(program)?;
        eliminate_dead_instructions(lowerer.finish(result, maximum_stack_depth)?, &mut [])
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
        let result = ValueId::new(result)?;
        let entry = BlockId(0);
        let program = Self {
            entry,
            block: BasicBlock {
                id: entry,
                instruction_start: 0,
                instruction_end: instructions.len(),
                terminator: Terminator::Return(result),
            },
            instructions,
            maximum_stack_depth: 1,
        };
        program.validate()?;
        Ok(program)
    }

    pub(crate) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(crate) fn result(&self) -> ValueId {
        match self.block.terminator {
            Terminator::Return(value) => value,
        }
    }

    pub(crate) fn maximum_stack_depth(&self) -> usize {
        self.maximum_stack_depth
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

    fn validate(&self) -> JitResult<()> {
        if self.entry != self.block.id
            || self.block.instruction_start != 0
            || self.block.instruction_end != self.instructions.len()
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA entry block does not cover the declared instruction range".into(),
            });
        }
        for (index, instruction) in self.instructions.iter().enumerate() {
            if instruction.result != ValueId::new(index)? {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!("SSA instruction {index} has non-canonical result identity")
                        .into(),
                });
            }
            if instruction
                .operands
                .iter()
                .any(|operand| usize::try_from(operand.0).map_or(true, |value| value >= index))
            {
                return Err(JitError::Verifier {
                    model: MODEL.into(),
                    detail: format!("SSA instruction {index} uses a non-dominating value").into(),
                });
            }
        }
        if self
            .instructions
            .get(self.result().0 as usize)
            .is_none_or(|instruction| instruction.value_type != ValueType::F64)
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "SSA return does not reference a defined f64 value".into(),
            });
        }
        Ok(())
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
        let entry = BlockId(0);
        let program = Program {
            entry,
            block: BasicBlock {
                id: entry,
                instruction_start: 0,
                instruction_end: self.instructions.len(),
                terminator: Terminator::Return(result),
            },
            instructions: self.instructions,
            maximum_stack_depth,
        };
        program.validate()?;
        Ok(program)
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

fn eliminate_dead_instructions(
    program: Program,
    outputs: &mut [AssignmentOutput],
) -> JitResult<Program> {
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
    let entry = BlockId(0);
    let compact = Program {
        entry,
        block: BasicBlock {
            id: entry,
            instruction_start: 0,
            instruction_end: instructions.len(),
            terminator: Terminator::Return(result),
        },
        instructions,
        maximum_stack_depth,
    };
    compact.validate()?;
    Ok(compact)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisterAllocation {
    instructions: Vec<AllocatedInstruction>,
    result: ValueLocation,
    value_locations: Vec<ValueLocation>,
    spill_slot_count: usize,
    used_register_count: usize,
    required_register_count: usize,
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
        // Positions before/after one instruction are distinct. An ordinary
        // operand use is at 2*i; a source-order assignment publication is at
        // 2*i+1. This prevents the allocator from overwriting a value with an
        // instruction result before that value has been stored.
        let mut last_use_position: Vec<usize> =
            (0..instruction_count).map(|index| index * 2).collect();
        for (instruction_index, instruction) in program.instructions.iter().enumerate() {
            for operand in instruction.operands.iter().copied() {
                let operand_index = operand.0 as usize;
                last_use_position[operand_index] =
                    last_use_position[operand_index].max(instruction_index * 2);
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
        last_use_position[program.result().0 as usize] = instruction_count
            .checked_mul(2)
            .ok_or_else(|| JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: "SSA return liveness position overflow".into(),
            })?;

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
        let crosses_call = |definition: usize| {
            let position = last_use_position[definition];
            let end_instruction = (position / 2).min(instruction_count);
            let end_exclusive = end_instruction
                .saturating_add(usize::from(position & 1 != 0))
                .min(instruction_count);
            end_exclusive > definition + 1
                && call_prefix[end_exclusive] > call_prefix[definition + 1]
        };

        let mut locations = vec![None; instruction_count];
        let mut register_owners = vec![None; allocatable_register_count];
        let mut spill_owners: Vec<Option<ValueId>> = Vec::new();
        let mut instructions = Vec::with_capacity(instruction_count);
        let mut used_register_count = 0;

        for (instruction_index, instruction) in program.instructions.iter().enumerate() {
            let instruction_position = instruction_index * 2;
            for owner in &mut register_owners {
                if owner.is_some_and(|value: ValueId| {
                    last_use_position[value.index()] < instruction_position
                }) {
                    *owner = None;
                }
            }
            for owner in &mut spill_owners {
                if owner.is_some_and(|value: ValueId| {
                    last_use_position[value.index()] < instruction_position
                }) {
                    *owner = None;
                }
            }
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

            let result = if crosses_call(instruction_index) {
                // A caller-saved register does not survive the call this value
                // is live across; a callee-saved one does, and costs the
                // function one shared prologue save rather than a store here
                // and a reload at every use. See [`RegisterBank`]. Reusing a
                // dying operand's register is still preferable when that
                // register is itself preserved.
                reusable_first
                    .filter(|register| bank.is_callee_saved(*register))
                    .or_else(|| {
                        register_owners
                            .iter()
                            .enumerate()
                            .skip(bank.callee_saved_base)
                            .find_map(|(register, owner)| owner.is_none().then_some(register))
                    })
                    .map_or_else(
                        || {
                            ValueLocation::Spill(allocate_spill_slot(
                                &mut spill_owners,
                                instruction.result,
                            ))
                        },
                        ValueLocation::Register,
                    )
            } else if let Some(register) = reusable_first {
                ValueLocation::Register(register)
            } else if let Some(register) = register_owners.iter().position(Option::is_none) {
                used_register_count = used_register_count.max(register + 1);
                ValueLocation::Register(register)
            } else {
                ValueLocation::Spill(allocate_spill_slot(&mut spill_owners, instruction.result))
            };

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
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::SlewState(_)
            | NativeOp::AbsDelayState(_)
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
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::SlewState(_)
            | NativeOp::AbsDelayState(_)
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
            | NativeOp::ZiState(_)
            | NativeOp::ZiStateDerivative(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::SlewState(_)
            | NativeOp::AbsDelayState(_)
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
        ALLOCATABLE_VALUE_REGISTERS, AllocatedInstruction, AssignmentProgram, BasicBlock, BlockId,
        Effects, Instruction, Program, RegisterAllocation, RegisterBank, Terminator, ValueId,
        ValueLocation, ValueType, plan_shared_outputs, required_register_count,
    };
    use crate::jit::expr::{NativeOp, NativeProgram, native_op_stack_effect};

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
        assert_eq!(lowered.block.terminator(), Terminator::Return(ValueId(4)));
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
        let shared = Program {
            entry: BlockId(0),
            block: BasicBlock {
                id: BlockId(0),
                instruction_start: 0,
                instruction_end: instructions.len(),
                terminator: Terminator::Return(ValueId(2)),
            },
            instructions,
            maximum_stack_depth: 2,
        };
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

        Program {
            entry: BlockId(0),
            block: BasicBlock {
                id: BlockId(0),
                instruction_start: 0,
                instruction_end: instructions.len(),
                terminator: Terminator::Return(accumulator),
            },
            instructions,
            maximum_stack_depth: 2,
        }
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
}
