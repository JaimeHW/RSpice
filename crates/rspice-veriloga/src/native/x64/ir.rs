//! Typed, effect-annotated SSA consumed by the x64 backend.
//!
//! Architecture-neutral lowering currently delivers postfix `NativeOp`s.  The
//! x64 backend immediately lifts that stream into explicit values so machine
//! code generation never has to trust an implicit operand stack.  Expressions
//! are single-block today; keeping the block and terminator explicit makes
//! control-flow extension possible without changing the value model.

use crate::native::expr::{NativeOp, NativeProgram, UnaryMathOp, native_op_stack_effect};
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-x64-ssa";
pub(super) const INLINE_DYNAMIC_LOWER_ABS_LIMIT: i64 = 1_i64 << 51;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct ValueId(u32);

impl ValueId {
    fn new(index: usize) -> JitResult<Self> {
        u32::try_from(index)
            .map(Self)
            .map_err(|_| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("SSA value count {index} exceeds the u32 identity space").into(),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ValueType {
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BlockId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Terminator {
    Return(ValueId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Effects(u16);

impl Effects {
    const READ_CONTEXT: u16 = 1 << 0;
    const READ_STATE: u16 = 1 << 1;
    const WRITE_STATE: u16 = 1 << 2;
    const MAY_CALL: u16 = 1 << 3;
    const MAY_FAIL: u16 = 1 << 4;
    const READ_ENTRY_ARGS: u16 = 1 << 5;
    const CLOBBER_CONTEXT_CACHE: u16 = 1 << 6;
    const INTERNAL_CALL_CONTINUATION: u16 = 1 << 7;

    pub(super) fn for_op(op: NativeOp) -> Self {
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

    pub(super) fn may_call(self) -> bool {
        self.contains(Self::MAY_CALL)
    }

    pub(super) fn reads_entry_args(self) -> bool {
        self.contains(Self::READ_ENTRY_ARGS)
    }

    pub(super) fn clobbers_context_pointer_cache(self) -> bool {
        self.contains(Self::CLOBBER_CONTEXT_CACHE)
    }

    pub(super) fn needs_saved_entry_args_for_internal_continuation(self) -> bool {
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
pub(super) struct Instruction {
    result: ValueId,
    value_type: ValueType,
    op: NativeOp,
    operands: Box<[ValueId]>,
    effects: Effects,
}

impl Instruction {
    pub(super) fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub(super) fn op(&self) -> NativeOp {
        self.op
    }

    #[cfg(test)]
    pub(super) fn operands(&self) -> &[ValueId] {
        &self.operands
    }

    pub(super) fn effects(&self) -> Effects {
        self.effects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct BasicBlock {
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
pub(super) struct Program {
    entry: BlockId,
    block: BasicBlock,
    instructions: Vec<Instruction>,
    maximum_stack_depth: usize,
}

impl Program {
    pub(super) fn lower(program: &NativeProgram) -> JitResult<Self> {
        program.validate_dependency_metadata()?;

        let mut stack = Vec::with_capacity(program.max_stack_depth());
        let mut instructions = Vec::with_capacity(program.ops().len());
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
            let result = ValueId::new(instructions.len())?;
            instructions.push(Instruction {
                result,
                value_type: ValueType::F64,
                op,
                operands,
                effects: Effects::for_op(op),
            });
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

        let entry = BlockId(0);
        let block = BasicBlock {
            id: entry,
            instruction_start: 0,
            instruction_end: instructions.len(),
            terminator: Terminator::Return(*result),
        };
        let lowered = Self {
            entry,
            block,
            instructions,
            maximum_stack_depth: observed_maximum_depth,
        };
        lowered.validate()?;
        Ok(lowered)
    }

    #[cfg(test)]
    pub(super) fn from_ssa_for_test(
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

    pub(super) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(super) fn result(&self) -> ValueId {
        match self.block.terminator {
            Terminator::Return(value) => value,
        }
    }

    pub(super) fn maximum_stack_depth(&self) -> usize {
        self.maximum_stack_depth
    }

    pub(super) fn uses_helper_calls(&self) -> bool {
        self.instructions
            .iter()
            .any(|instruction| instruction.effects.may_call())
    }

    pub(super) fn needs_saved_entry_args(&self) -> bool {
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

/// One computation and every output slot that consumes its result.
///
/// Sparse Jacobian construction already omits structurally absent axes.  This
/// plan handles the other common source of redundant work: a current
/// contribution stamps the same derivative into positive and negative KCL
/// rows, so the two matrix entries carry byte-for-byte identical programs.
/// Exact SSA equality plus the formal effect gate makes that one computation
/// with two stores without changing numerical semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SharedOutputGroup {
    representative: usize,
    outputs: Box<[usize]>,
}

impl SharedOutputGroup {
    pub(super) fn representative(&self) -> usize {
        self.representative
    }

    pub(super) fn outputs(&self) -> &[usize] {
        &self.outputs
    }
}

pub(super) fn plan_shared_outputs(programs: &[Program]) -> Vec<SharedOutputGroup> {
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

/// XMM0-XMM9 hold allocated SSA values. Six registers remain available for
/// materializing the widest native operation's five spilled operands plus one
/// instruction-specific scratch value.
pub(super) const ALLOCATABLE_XMM_REGISTERS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ValueLocation {
    Register(usize),
    Spill(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AllocatedInstruction {
    operands: Box<[ValueLocation]>,
    result: ValueLocation,
    live_register_mask: u16,
}

impl AllocatedInstruction {
    pub(super) fn operands(&self) -> &[ValueLocation] {
        &self.operands
    }

    pub(super) fn result(&self) -> ValueLocation {
        self.result
    }

    pub(super) fn live_register_mask(&self) -> u16 {
        self.live_register_mask
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RegisterAllocation {
    instructions: Vec<AllocatedInstruction>,
    result: ValueLocation,
    spill_slot_count: usize,
    used_register_count: usize,
    required_register_count: usize,
}

impl RegisterAllocation {
    pub(super) fn build(program: &Program) -> JitResult<Self> {
        program.validate()?;
        let instruction_count = program.instructions.len();
        let mut last_use: Vec<usize> = (0..instruction_count).collect();
        for (instruction_index, instruction) in program.instructions.iter().enumerate() {
            for operand in instruction.operands.iter().copied() {
                let operand_index = operand.0 as usize;
                last_use[operand_index] = last_use[operand_index].max(instruction_index);
            }
        }
        last_use[program.result().0 as usize] = instruction_count;

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
            let end = last_use[definition];
            end > definition + 1 && call_prefix[end] > call_prefix[definition + 1]
        };

        let mut locations = vec![None; instruction_count];
        let mut register_owners = [None; ALLOCATABLE_XMM_REGISTERS];
        let mut spill_owners: Vec<Option<ValueId>> = Vec::new();
        let mut instructions = Vec::with_capacity(instruction_count);
        let mut used_register_count = 0;

        for (instruction_index, instruction) in program.instructions.iter().enumerate() {
            let live_register_mask = register_owners
                .iter()
                .enumerate()
                .fold(0_u16, |mask, (register, owner)| {
                    mask | (u16::from(owner.is_some()) << register)
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
                .filter(|operand| last_use[operand.0 as usize] == instruction_index)
                .and_then(|operand| locations[operand.0 as usize])
                .and_then(|location| match location {
                    ValueLocation::Register(register) => Some(register),
                    ValueLocation::Spill(_) => None,
                });

            let result = if crosses_call(instruction_index) {
                ValueLocation::Spill(allocate_spill_slot(&mut spill_owners, instruction.result))
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
                if last_use[operand.0 as usize] != instruction_index {
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
            spill_slot_count: spill_owners.len(),
            used_register_count,
            required_register_count,
        })
    }

    pub(super) fn instructions(&self) -> &[AllocatedInstruction] {
        &self.instructions
    }

    pub(super) fn result(&self) -> ValueLocation {
        self.result
    }

    pub(super) fn spill_slot_count(&self) -> usize {
        self.spill_slot_count
    }

    #[cfg(test)]
    pub(super) fn used_register_count(&self) -> usize {
        self.used_register_count
    }

    pub(super) fn required_register_count(&self) -> usize {
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
    const XMM_REGISTER_COUNT: usize = 16;

    let mut used = [false; XMM_REGISTER_COUNT];
    for (register, occupied) in used.iter_mut().enumerate() {
        *occupied = instruction.live_register_mask & (1_u16 << register) != 0;
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
    match instruction.result {
        ValueLocation::Register(register) => used[register] = true,
        ValueLocation::Spill(_) => {
            let reuses_spilled_first =
                matches!(instruction.operands.first(), Some(ValueLocation::Spill(_)));
            if !reuses_spilled_first && let Some(register) = take_free_register(&mut used) {
                used[register] = true;
            }
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

fn take_free_register(used: &mut [bool; 16]) -> Option<usize> {
    used.iter().position(|occupied| !occupied)
}

pub(super) fn dynamic_variable_inline_supported(len: usize, lower: i64) -> bool {
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
            | NativeOp::TableLookup(_)
            | NativeOp::TableDerivative(_)
            | NativeOp::LimiterPrevious(_)
            | NativeOp::LimiterStore(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::ZiState(_)
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
            | NativeOp::IntegerCast
            | NativeOp::IntegerBinary(_)
            | NativeOp::IntegerShiftConst(_, _)
            | NativeOp::IntegerBinaryConst(_, _)
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
                | NativeOp::IntegerCast
                | NativeOp::IntegerBinary(_)
                | NativeOp::IntegerShiftConst(_, _)
                | NativeOp::IntegerBinaryConst(_, _)
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

fn unary_math_uses_helper(op: UnaryMathOp) -> bool {
    !matches!(op, UnaryMathOp::Floor | UnaryMathOp::Ceil)
}

#[cfg(test)]
mod tests {
    use super::{
        BasicBlock, BlockId, Effects, Instruction, Program, RegisterAllocation, Terminator,
        ValueId, ValueLocation, ValueType, plan_shared_outputs,
    };
    use crate::native::expr::{NativeOp, NativeProgram, native_op_stack_effect};

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
            Effects::for_op(NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp)).may_call()
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
            NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
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
        let allocation = RegisterAllocation::build(&lowered).expect("register allocation");

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
        let allocation = RegisterAllocation::build(&shared).expect("register allocation");

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

    #[test]
    fn liveness_allocation_spills_values_live_across_helper_calls() {
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
                op: NativeOp::LoadVariable(1),
                operands: Box::new([]),
                effects: Effects::for_op(NativeOp::LoadVariable(1)),
            },
            Instruction {
                result: ValueId(2),
                value_type: ValueType::F64,
                op: NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
                operands: Box::new([ValueId(1)]),
                effects: Effects::for_op(NativeOp::UnaryMath(
                    crate::native::expr::UnaryMathOp::Exp,
                )),
            },
            Instruction {
                result: ValueId(3),
                value_type: ValueType::F64,
                op: NativeOp::Add,
                operands: Box::new([ValueId(0), ValueId(2)]),
                effects: Effects::for_op(NativeOp::Add),
            },
        ];
        let across_call = Program {
            entry: BlockId(0),
            block: BasicBlock {
                id: BlockId(0),
                instruction_start: 0,
                instruction_end: instructions.len(),
                terminator: Terminator::Return(ValueId(3)),
            },
            instructions,
            maximum_stack_depth: 2,
        };
        let allocation = RegisterAllocation::build(&across_call).expect("register allocation");

        assert!(matches!(
            allocation.instructions()[0].result(),
            ValueLocation::Spill(_)
        ));
        assert_eq!(allocation.spill_slot_count(), 1);
    }
}
