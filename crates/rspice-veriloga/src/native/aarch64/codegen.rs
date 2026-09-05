//! AArch64 lowering for scalar native expression entries.
//!
//! This compiler consumes the same typed SSA and liveness allocation as x64,
//! maps logical values only to volatile AAPCS64 SIMD registers, emits checked
//! A64 words, and authenticates the completed body with the independent
//! decoder before executable publication.

use super::calling_convention::HOST_ABI;
use super::encoder::{
    A64Encoder, BranchPatch, Condition, DReg, LiteralPatch, MAX_FORWARD_LITERAL_REACH_BYTES, XReg,
};
use super::verifier::{verify_exact_function, verify_exact_function_at};
use crate::jit::plan_program::PlanProgram;
use crate::native::FUSED_KERNEL_INLINE_LIMIT;
use crate::native::abi::NativeRuntimeStatus;
use crate::native::abi::{
    INTEGER_CAST_DESCRIPTOR, integer_binary_const_descriptor, integer_binary_descriptor,
    integer_shift_const_descriptor, rspice_above_state_native,
    rspice_absdelay_derivative_max_native, rspice_absdelay_derivative_native,
    rspice_absdelay_state_max_native, rspice_absdelay_state_native, rspice_acos, rspice_acosh,
    rspice_asin, rspice_asinh, rspice_atan, rspice_atan2, rspice_atanh, rspice_cos, rspice_cosh,
    rspice_cross_state_native, rspice_ddt_jacobian_native, rspice_ddt_state_native,
    rspice_dynamic_variable_slot_native, rspice_exp, rspice_hypot, rspice_idt_jacobian_native,
    rspice_idt_state_native, rspice_idtmod_state_native, rspice_integer_operation_native,
    rspice_laplace_derivative_native, rspice_laplace_step_native,
    rspice_last_crossing_state_native, rspice_limexp, rspice_limited_exp,
    rspice_limiter_previous_native, rspice_limiter_store_native, rspice_log, rspice_log10,
    rspice_mod, rspice_native_current_probe_error, rspice_native_dynamic_variable_error,
    rspice_native_limit_state_bounds_error, rspice_native_limit_state_initialized_error,
    rspice_native_limit_state_values_bounds_error, rspice_native_limit_state_values_error,
    rspice_native_loop_limit_error, rspice_native_non_finite_contribution_error,
    rspice_native_param_given_error, rspice_native_port_connected_error,
    rspice_native_prior_current_error, rspice_pow, rspice_sin, rspice_sinh,
    rspice_slew_derivative_native, rspice_slew_state_native, rspice_table_derivative_native,
    rspice_table_lookup_native, rspice_tan, rspice_tanh, rspice_timer_state_native,
    rspice_transition_derivative_native, rspice_transition_state_native,
    rspice_zi_derivative_native, rspice_zi_step_native,
};
use crate::native::assignment::{NativeAssignment, shareable_batch_ranges};
use crate::native::expr::{
    BinaryMathOp, CompareOp, ExtremumOp, IntegerBinaryOp, LogicalOp, NativeOp, NativeProgram,
    UnaryMathOp, VoltageNode, runtime_integer_operation,
};
use crate::native::model::{CodeOffset, NativeStampKernelIo};
use crate::native::ssa::{
    AllocatedInstruction, AssignmentProgram, BasicBlock, BlockId, Instruction,
    LOGICAL_VALUE_REGISTER_COUNT, Program, RegisterAllocation, RegisterBank, Terminator,
    ValueLocation, ValueType, dynamic_variable_inline_supported, plan_shared_outputs,
};
use crate::native::{EvalContext, JitError, JitResult};

const MODEL: &str = "native-aarch64";
const WORD_BYTES: usize = std::mem::size_of::<f64>();
const STACK_ALIGNMENT: usize = 16;
const STACK_PROBE_INTERVAL_BYTES: usize = 4080;
const MAX_EXPRESSION_STACK_DEPTH: usize = 4096;
const MAX_SEGMENT_INSTRUCTIONS: usize = 1024;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;
const MAX_RUNTIME_LOOP_ITERATIONS: u64 = 100_000;
const A64_ALLOCATABLE_VALUE_REGISTERS: usize = 15;
/// D0-D7 and D16-D22, every one of them volatile: the host convention keeps
/// the nonvolatile D8-D15 out of the bank entirely, so a value live across a
/// helper call has nowhere to sit but a spill slot.
pub(crate) const A64_VALUE_BANK: RegisterBank =
    RegisterBank::all_caller_saved(A64_ALLOCATABLE_VALUE_REGISTERS);
/// Every logical register the allocator can hand out, as a mask.
///
/// A block terminator runs between instructions, so anything above the bank is
/// free to materialize a spilled branch condition or relay a memory-to-memory
/// edge move.
const A64_ALLOCATED_REGISTER_MASK: u32 = (1_u32 << A64_ALLOCATABLE_VALUE_REGISTERS) - 1;

const VOLTAGES_OFFSET: usize = std::mem::offset_of!(EvalContext, voltages);
const INTERNAL_VOLTAGES_OFFSET: usize = std::mem::offset_of!(EvalContext, internal_voltages);
const PARAMS_OFFSET: usize = std::mem::offset_of!(EvalContext, params);
const PRELUDE_SLOTS_OFFSET: usize = std::mem::offset_of!(EvalContext, prelude_slots);
const BRANCH_CURRENTS_OFFSET: usize = std::mem::offset_of!(EvalContext, branch_currents);
const BRANCH_CURRENTS_LEN_OFFSET: usize = std::mem::offset_of!(EvalContext, branch_currents_len);
const CURRENTS_OFFSET: usize = std::mem::offset_of!(EvalContext, currents);
const CURRENTS_LEN_OFFSET: usize = std::mem::offset_of!(EvalContext, currents_len);
const PORT_CONNECTED_OFFSET: usize = std::mem::offset_of!(EvalContext, port_connected);
const PORT_CONNECTED_LEN_OFFSET: usize = std::mem::offset_of!(EvalContext, port_connected_len);
const PARAM_GIVEN_OFFSET: usize = std::mem::offset_of!(EvalContext, param_given);
const PARAM_GIVEN_LEN_OFFSET: usize = std::mem::offset_of!(EvalContext, param_given_len);
const STATE_VALUES_OFFSET: usize = std::mem::offset_of!(EvalContext, state_values);
const STATE_VALUES_LEN_OFFSET: usize = std::mem::offset_of!(EvalContext, state_values_len);
const STATE_INITIALIZED_OFFSET: usize = std::mem::offset_of!(EvalContext, state_initialized);
const STATE_INITIALIZED_LEN_OFFSET: usize =
    std::mem::offset_of!(EvalContext, state_initialized_len);
const BRANCH_UNKNOWNS_OFFSET: usize = std::mem::offset_of!(EvalContext, branch_unknowns);
const ANALYSIS_TYPE_OFFSET: usize = std::mem::offset_of!(EvalContext, analysis_type);
const ANALYSIS_INITIAL_STEP_OFFSET: usize =
    std::mem::offset_of!(EvalContext, analysis_initial_step);
const ANALYSIS_FINAL_STEP_OFFSET: usize = std::mem::offset_of!(EvalContext, analysis_final_step);
const TEMPERATURE_OFFSET: usize = std::mem::offset_of!(EvalContext, temperature);
const TIME_OFFSET: usize = std::mem::offset_of!(EvalContext, time);
const MFACTOR_OFFSET: usize = std::mem::offset_of!(EvalContext, multiplicity);
const KERNEL_ACTIVE_OFFSET: usize = std::mem::offset_of!(NativeStampKernelIo, program_active);
const KERNEL_JACOBIANS_OFFSET: usize = std::mem::offset_of!(NativeStampKernelIo, jacobians);

/// Compile one native `(context, variables) -> f64` entry.
///
/// The complete model compiler uses this entry for scalar parameters, static
/// conditions, stamps, Jacobians, and noise expressions.
pub(crate) fn compile_value_function(program: &NativeProgram) -> JitResult<Vec<u8>> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    compile_value_function_from_ssa(&Program::lower(program)?)
}

/// Compile one already-lowered value entry.
///
/// The postfix lift is the shipped route into this; taking the SSA directly is
/// what lets the branch form of a conditional be compiled through exactly the
/// same allocator, emitter, and independent A64 verifier as the select form.
pub(crate) fn compile_value_function_from_ssa(ssa: &Program) -> JitResult<Vec<u8>> {
    validate_expression_stack_depth(ssa.maximum_stack_depth())?;
    let allocation = RegisterAllocation::build(ssa, A64_VALUE_BANK)?;
    let frame_bytes = spill_frame_bytes(&allocation)?;
    let mut compiler = FunctionCompiler::new(frame_bytes, ssa.requires_call_frame())?;
    compiler.emit_program(&ssa, &allocation)?;
    let bytes = compiler.finish(allocation.result())?;
    verify_exact_function(&bytes, "scalar value function")?;
    Ok(bytes)
}

pub(crate) struct A64SegmentedProgram {
    pub(crate) functions: Vec<Vec<u8>>,
    pub(crate) value_count: usize,
    pub(crate) result_index: usize,
}

pub(crate) fn compile_segmented_program(program: &NativeProgram) -> JitResult<A64SegmentedProgram> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    let ssa = Program::lower(program)?;
    validate_expression_stack_depth(ssa.maximum_stack_depth())?;
    let mut functions =
        Vec::with_capacity(ssa.instructions().len().div_ceil(MAX_SEGMENT_INSTRUCTIONS));
    for instructions in ssa.instructions().chunks(MAX_SEGMENT_INSTRUCTIONS) {
        let mut compiler = FunctionCompiler::new_with_kernel_io(0, true, true)?;
        let scratch = compiler.kernel_io_register()?;
        for instruction in instructions {
            if instruction.operands().len() >= LOGICAL_VALUE_REGISTER_COUNT {
                return Err(register_allocation_error(format!(
                    "AArch64 segmented instruction requires {} operands",
                    instruction.operands().len()
                )));
            }
            let mut used_mask = 0_u32;
            let mut operands = Vec::with_capacity(instruction.operands().len());
            for (register_index, operand) in instruction.operands().iter().enumerate() {
                let register = logical_register(register_index)?;
                compiler.emit_array_load(register, scratch, operand.index())?;
                used_mask |= register_mask(register)?;
                operands.push(register);
            }
            let result = logical_register(instruction.operands().len())?;
            used_mask |= register_mask(result)?;
            let mut prepared = PreparedInstruction {
                operands,
                result,
                used_mask,
            };
            compiler.emit_op(instruction.op(), &mut prepared)?;
            compiler.emit_array_store(prepared.result, scratch, instruction.result().index())?;
        }
        let bytes = compiler.finish_assignment_pass()?;
        verify_exact_function(&bytes, "segmented expression function")?;
        functions.push(bytes);
    }
    if functions.is_empty() {
        return Err(verifier_error(
            "AArch64 segmented expression contains no instructions",
        ));
    }
    Ok(A64SegmentedProgram {
        functions,
        value_count: ssa.instructions().len(),
        result_index: ssa.result().index(),
    })
}

pub(crate) fn compile_segmented_value_driver(
    function_image_offset: usize,
    segments: &[CodeOffset],
    value_count: usize,
    result_index: usize,
) -> JitResult<Vec<u8>> {
    let frame_bytes = aligned_frame_bytes(value_count)?;
    let mut compiler = FunctionCompiler::new(frame_bytes, true)?;
    for segment in segments {
        compiler.emit_image_segment_call(function_image_offset, *segment)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    compiler.stack_load_d(DReg::D0, value_slot_offset(result_index)?)?;
    let bytes = compiler.finish(ValueLocation::Register(0))?;
    verify_exact_function_at(&bytes, "segmented value driver", function_image_offset)?;
    Ok(bytes)
}

pub(crate) fn compile_segmented_assignment_driver(
    function_image_offset: usize,
    segments: &[CodeOffset],
    value_count: usize,
    result_index: usize,
    variable_index: usize,
) -> JitResult<Vec<u8>> {
    let frame_bytes = aligned_frame_bytes(value_count)?;
    let mut compiler = FunctionCompiler::new(frame_bytes, true)?;
    for segment in segments {
        compiler.emit_image_segment_call(function_image_offset, *segment)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    compiler.stack_load_d(DReg::D0, value_slot_offset(result_index)?)?;
    compiler.emit_array_store(DReg::D0, compiler.variables_register(), variable_index)?;
    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(&bytes, "segmented assignment driver", function_image_offset)?;
    Ok(bytes)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_segmented_indexed_assignment_driver(
    function_image_offset: usize,
    index_segments: &[CodeOffset],
    index_value_count: usize,
    index_result_index: usize,
    value_segments: &[CodeOffset],
    value_value_count: usize,
    value_result_index: usize,
    base: usize,
    len: usize,
    lower: i64,
) -> JitResult<Vec<u8>> {
    if index_segments.is_empty() || value_segments.is_empty() {
        return Err(encoding_error(
            "AArch64 segmented indexed assignment requires index and value segments",
        ));
    }
    let scratch_value_count = index_value_count.max(value_value_count);
    let pointer_slot = scratch_value_count;
    let frame_slot_count = scratch_value_count.checked_add(1).ok_or_else(|| {
        register_allocation_error("AArch64 segmented indexed-assignment frame overflow")
    })?;
    let frame_bytes = aligned_frame_bytes(frame_slot_count)?;
    let pointer_offset = value_slot_offset(pointer_slot)?;
    let mut compiler = FunctionCompiler::new(frame_bytes, true)?;

    for segment in index_segments {
        compiler.emit_image_segment_call(function_image_offset, *segment)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    compiler.stack_load_d(DReg::D0, value_slot_offset(index_result_index)?)?;
    compiler.stack_store_d(DReg::D0, pointer_offset)?;
    compiler.emit_variable_base_pointer(XReg::X0, base)?;
    compiler.encoder.mov_u64(XReg::X1, len as u64)?;
    compiler.encoder.mov_u64(XReg::X2, lower as u64)?;
    compiler.encoder.mov_u64(
        HOST_ABI.indirect_call_scratch,
        rspice_dynamic_variable_slot_native as *const () as usize as u64,
    )?;
    compiler.encoder.blr(HOST_ABI.indirect_call_scratch);
    let invalid = compiler.encoder.cbz_placeholder(XReg::X0)?;
    compiler.stack_store_x(XReg::X0, pointer_offset)?;
    let slot_ready = compiler.encoder.b_placeholder();

    let invalid_target = compiler.encoder.position();
    compiler.encoder.patch_branch(invalid, invalid_target)?;
    compiler.stack_load_d(DReg::D0, pointer_offset)?;
    compiler.emit_dynamic_variable_error(len, lower)?;

    let slot_ready_target = compiler.encoder.position();
    compiler
        .encoder
        .patch_branch(slot_ready, slot_ready_target)?;
    for segment in value_segments {
        compiler.emit_image_segment_call(function_image_offset, *segment)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    compiler.stack_load_d(DReg::D0, value_slot_offset(value_result_index)?)?;
    compiler.stack_load_x(XReg::X9, pointer_offset)?;
    compiler.encoder.str_d_unsigned(DReg::D0, XReg::X9, 0)?;
    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(
        &bytes,
        "segmented indexed-assignment driver",
        function_image_offset,
    )?;
    Ok(bytes)
}

#[cfg(test)]
pub(crate) fn compile_assignment_function(
    variable_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    let ssa = Program::lower(program)?;
    validate_expression_stack_depth(ssa.maximum_stack_depth())?;
    let allocation = RegisterAllocation::build(&ssa, A64_VALUE_BANK)?;
    let frame_bytes = spill_frame_bytes(&allocation)?;
    let mut compiler = FunctionCompiler::new(frame_bytes, ssa.requires_call_frame())?;
    compiler.emit_program(&ssa, &allocation)?;
    let bytes = compiler.finish_assignment(allocation.result(), variable_index)?;
    verify_exact_function(&bytes, "assignment function")?;
    Ok(bytes)
}

pub(crate) fn compile_assignment_pass_function(
    assignments: &[NativeAssignment],
) -> JitResult<Vec<u8>> {
    let requirements = assignment_requirements(assignments)?;
    let layout = AssignmentFrameLayout::new(requirements)?;
    let mut compiler = FunctionCompiler::new(layout.frame_bytes, requirements.requires_call_frame)?;
    compiler.emit_assignment_steps(assignments, 0, layout)?;
    let bytes = compiler.finish_assignment_pass()?;
    if std::env::var_os("RSPICE_NATIVE_A64_IMAGE_TRACE").is_some() && bytes.len() >= 1_000_000 {
        eprintln!(
            "RSPICE A64 assignment function bytes={} ssa={} outputs={} spill_operands={} spill_results={} max_spill_slots={}",
            bytes.len(),
            requirements.trace_instructions,
            requirements.trace_outputs,
            requirements.trace_spill_operands,
            requirements.trace_spill_results,
            requirements.maximum_spill_slots,
        );
    }
    verify_exact_function(&bytes, "assignment-pass function")?;
    Ok(bytes)
}

pub(crate) fn compile_assignment_dispatch_function(
    function_image_offset: usize,
    chunks: &[CodeOffset],
) -> JitResult<Vec<u8>> {
    if chunks.is_empty() {
        return Err(encoding_error(
            "AArch64 assignment dispatcher requires at least one chunk",
        ));
    }
    let mut compiler = FunctionCompiler::new(0, true)?;
    for chunk in chunks {
        compiler.emit_image_entry_call(function_image_offset, *chunk)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(&bytes, "assignment dispatcher", function_image_offset)?;
    Ok(bytes)
}

pub(crate) fn compile_loop_dispatch_function(
    function_image_offset: usize,
    condition: &NativeProgram,
    body_chunks: &[CodeOffset],
) -> JitResult<Vec<u8>> {
    if body_chunks.is_empty() {
        return Err(encoding_error(
            "AArch64 loop dispatcher requires at least one body chunk",
        ));
    }
    let mut requirements = AssignmentRequirements {
        loop_depth: 1,
        requires_call_frame: true,
        ..AssignmentRequirements::default()
    };
    inspect_assignment_program(condition, &mut requirements)?;
    let layout = AssignmentFrameLayout::new(requirements)?;
    let mut compiler = FunctionCompiler::new(layout.frame_bytes, true)?;
    let counter_offset = layout.loop_counter_offset(0)?;
    compiler.encoder.mov_u64(XReg::X9, 0)?;
    compiler.stack_store_x(XReg::X9, counter_offset)?;
    let loop_start = compiler.encoder.position();

    let condition_result = compiler.emit_native_program_for_pass(condition)?;
    compiler.materialize_location(condition_result, DReg::D0)?;
    compiler.emit_literal(DReg::D31, 0.0)?;
    compiler.encoder.fcmp_d(DReg::D0, DReg::D31);
    let loop_exit = compiler.encoder.b_cond_placeholder(Condition::Equal);

    for chunk in body_chunks {
        compiler.emit_image_entry_call(function_image_offset, *chunk)?;
        compiler.emit_kernel_abort_if_failed()?;
    }

    compiler.stack_load_x(XReg::X9, counter_offset)?;
    compiler.encoder.add_x_imm(XReg::X9, XReg::X9, 1)?;
    compiler.stack_store_x(XReg::X9, counter_offset)?;
    compiler
        .encoder
        .mov_u64(XReg::X10, MAX_RUNTIME_LOOP_ITERATIONS)?;
    compiler.encoder.cmp_x(XReg::X9, XReg::X10)?;
    let limit_reached = compiler.encoder.b_cond_placeholder(Condition::CarrySet);
    let repeat = compiler.encoder.b_placeholder();
    compiler.encoder.patch_branch(repeat, loop_start)?;

    let limit_target = compiler.encoder.position();
    compiler.encoder.patch_branch(limit_reached, limit_target)?;
    compiler.emit_void_error_early_return(rspice_native_loop_limit_error as *const () as usize)?;

    let exit_target = compiler.encoder.position();
    compiler.encoder.patch_branch(loop_exit, exit_target)?;
    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(&bytes, "loop assignment dispatcher", function_image_offset)?;
    Ok(bytes)
}

/// Where in the image each entry a fused kernel emits already lives.
///
/// The kernel is assembled after the entries, so every program it is about to
/// inline is also a function it could call instead. Which of the two it does is
/// [`FUSED_KERNEL_INLINE_LIMIT`].
#[derive(Clone, Copy)]
pub(crate) struct A64FusedKernelEntries<'a> {
    pub(crate) stamp_values: &'a [CodeOffset],
    pub(crate) jacobians: &'a [Vec<CodeOffset>],
}

pub(crate) fn compile_fused_evaluation_kernel(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[PlanProgram],
    entries: A64FusedKernelEntries<'_>,
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    compile_fused_kernel(
        kernel_image_offset,
        assignment,
        prelude,
        stamp_values,
        None,
        entries,
        published_current_pairs,
        "fused evaluation kernel",
    )
}

pub(crate) fn compile_fused_stamp_kernel(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[PlanProgram],
    jacobians: &[Vec<PlanProgram>],
    entries: A64FusedKernelEntries<'_>,
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    compile_fused_kernel(
        kernel_image_offset,
        assignment,
        prelude,
        stamp_values,
        Some(jacobians),
        entries,
        published_current_pairs,
        "fused stamp kernel",
    )
}

pub(crate) fn compile_fused_evaluation_driver(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[CodeOffset],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    compile_fused_driver(
        kernel_image_offset,
        assignment,
        prelude,
        stamp_values,
        None,
        published_current_pairs,
        "fused evaluation call driver",
    )
}

pub(crate) fn compile_fused_stamp_driver(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[CodeOffset],
    jacobians: &[Vec<CodeOffset>],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    compile_fused_driver(
        kernel_image_offset,
        assignment,
        prelude,
        stamp_values,
        Some(jacobians),
        published_current_pairs,
        "fused stamp call driver",
    )
}

fn compile_fused_driver(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[CodeOffset],
    jacobians: Option<&[Vec<CodeOffset>]>,
    published_current_pairs: &[Option<(usize, usize)>],
    kernel_name: &str,
) -> JitResult<Vec<u8>> {
    if stamp_values.len() != published_current_pairs.len()
        || jacobians.is_some_and(|entries| entries.len() != stamp_values.len())
    {
        return Err(JitError::InternalCompilerError {
            model: MODEL.into(),
            detail: "AArch64 call-driver value, Jacobian, and publication shapes differ".into(),
        });
    }

    let mut compiler = FunctionCompiler::new_with_kernel_io(0, true, true)?;
    compiler.emit_image_entry_call(kernel_image_offset, assignment)?;
    compiler.emit_kernel_abort_if_failed()?;
    // The CFG route's assignment pass, once, before the first entry that reads
    // a slot it publishes. `None` for every postfix plan, so a shipped driver
    // emits exactly the bytes it did before.
    if let Some(prelude) = prelude {
        compiler.emit_image_entry_call(kernel_image_offset, prelude)?;
        compiler.emit_kernel_abort_if_failed()?;
    }

    let mut jacobian_index = 0_usize;
    for (stamp_index, (value_entry, current_pair)) in
        stamp_values.iter().zip(published_current_pairs).enumerate()
    {
        let skip_stamp = compiler.emit_kernel_skip_if_inactive(stamp_index)?;
        compiler.emit_image_entry_call(kernel_image_offset, *value_entry)?;
        compiler.emit_kernel_abort_if_failed()?;
        compiler.emit_kernel_non_finite_guard(stamp_index)?;
        compiler.emit_kernel_stamp_value_store(stamp_index, *current_pair)?;

        if let Some(stamp_jacobians) = jacobians.map(|entries| &entries[stamp_index]) {
            for entry in stamp_jacobians {
                compiler.emit_image_entry_call(kernel_image_offset, *entry)?;
                compiler.emit_kernel_abort_if_failed()?;
                compiler.emit_kernel_jacobian_store(jacobian_index)?;
                jacobian_index = jacobian_index.checked_add(1).ok_or_else(|| {
                    encoding_error("AArch64 call-driver Jacobian output index overflow")
                })?;
            }
        }
        let skip_target = compiler.encoder.position();
        compiler.encoder.patch_branch(skip_stamp, skip_target)?;
    }

    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(&bytes, kernel_name, kernel_image_offset)?;
    Ok(bytes)
}

fn compile_fused_kernel(
    kernel_image_offset: usize,
    assignment: CodeOffset,
    prelude: Option<CodeOffset>,
    stamp_values: &[PlanProgram],
    jacobians: Option<&[Vec<PlanProgram>]>,
    entries: A64FusedKernelEntries<'_>,
    published_current_pairs: &[Option<(usize, usize)>],
    kernel_name: &str,
) -> JitResult<Vec<u8>> {
    if stamp_values.len() != published_current_pairs.len()
        || jacobians.is_some_and(|rows| rows.len() != stamp_values.len())
        || entries.stamp_values.len() != stamp_values.len()
        || jacobians.is_some_and(|rows| {
            entries.jacobians.len() != rows.len()
                || rows
                    .iter()
                    .zip(entries.jacobians)
                    .any(|(row, offsets)| row.len() != offsets.len())
        })
    {
        return Err(JitError::InternalCompilerError {
            model: MODEL.into(),
            detail: "AArch64 fused-kernel value, Jacobian, and publication shapes differ".into(),
        });
    }

    // The kernel inlines its entries rather than calling them, so it needs each
    // one in block form. A postfix entry is lifted exactly as it always was; a
    // block entry is already there, and `emit_program` emits real branches for
    // it.
    let value_ssa = stamp_values
        .iter()
        .map(|program| program.borrow().lower_to_ssa())
        .collect::<JitResult<Vec<_>>>()?;
    let jacobian_ssa = jacobians
        .map(|entries| {
            entries
                .iter()
                .map(|stamp| {
                    stamp
                        .iter()
                        .map(|program| program.borrow().lower_to_ssa())
                        .collect::<JitResult<Vec<_>>>()
                })
                .collect::<JitResult<Vec<_>>>()
        })
        .transpose()?;
    let value_allocations = value_ssa
        .iter()
        .map(|program| RegisterAllocation::build(program, A64_VALUE_BANK))
        .collect::<JitResult<Vec<_>>>()?;
    let jacobian_allocations = jacobian_ssa
        .as_ref()
        .map(|entries| {
            entries
                .iter()
                .map(|stamp| {
                    stamp
                        .iter()
                        .map(|program| RegisterAllocation::build(program, A64_VALUE_BANK))
                        .collect::<JitResult<Vec<_>>>()
                })
                .collect::<JitResult<Vec<_>>>()
        })
        .transpose()?;

    // Whether the kernel emits a program's body or a branch to the copy the
    // image already holds. Only a plan with a prelude is eligible, so a postfix
    // kernel — the modules the CFG route refuses — emits the instructions it
    // emitted before this rule existed.
    let inlines = |program: &Program| {
        prelude.is_none() || program.instructions().len() <= FUSED_KERNEL_INLINE_LIMIT
    };
    // The frame is sized by the programs the kernel actually emits. A called
    // entry brings its own frame and its own spill slots, so it contributes
    // nothing here.
    let inlined_ssa = || {
        value_ssa
            .iter()
            .chain(jacobian_ssa.iter().flat_map(|rows| rows.iter().flatten()))
            .filter(|program| inlines(program))
    };
    let inlined_allocations = || {
        value_ssa
            .iter()
            .zip(&value_allocations)
            .chain(
                jacobian_ssa
                    .iter()
                    .flat_map(|rows| rows.iter().flatten())
                    .zip(
                        jacobian_allocations
                            .iter()
                            .flat_map(|rows| rows.iter().flatten()),
                    ),
            )
            .filter(|(program, _)| inlines(program))
            .map(|(_, allocation)| allocation)
    };
    let maximum_stack_depth = inlined_ssa()
        .map(|program| program.maximum_stack_depth())
        .max()
        .unwrap_or(0);
    validate_expression_stack_depth(maximum_stack_depth)?;
    let maximum_spill_slots = inlined_allocations()
        .map(|allocation| allocation.spill_slot_count())
        .max()
        .unwrap_or(0);
    let frame_bytes = aligned_frame_bytes(maximum_spill_slots)?;
    let mut compiler = FunctionCompiler::new_with_kernel_io(frame_bytes, true, true)?;
    compiler.emit_image_entry_call(kernel_image_offset, assignment)?;
    compiler.emit_kernel_abort_if_failed()?;
    // Called rather than inlined, like the assignment pass above it: the one
    // copy in the image serves this kernel and the per-entry path both, and
    // inlining it would put back the code size the prelude exists to remove.
    if let Some(prelude) = prelude {
        compiler.emit_image_entry_call(kernel_image_offset, prelude)?;
        compiler.emit_kernel_abort_if_failed()?;
    }

    let mut jacobian_index = 0_usize;
    for (stamp_index, ((program, allocation), current_pair)) in value_ssa
        .iter()
        .zip(&value_allocations)
        .zip(published_current_pairs)
        .enumerate()
    {
        let skip_stamp = compiler.emit_kernel_skip_if_inactive(stamp_index)?;
        if inlines(program) {
            compiler.emit_program(program, allocation)?;
            compiler.materialize_location(allocation.result(), DReg::D0)?;
        } else {
            compiler
                .emit_image_entry_call(kernel_image_offset, entries.stamp_values[stamp_index])?;
            compiler.emit_kernel_abort_if_failed()?;
        }
        compiler.emit_kernel_non_finite_guard(stamp_index)?;
        compiler.emit_kernel_stamp_value_store(stamp_index, *current_pair)?;

        if let Some(stamp_jacobians) = jacobian_ssa.as_ref().map(|rows| &rows[stamp_index]) {
            let stamp_allocations = &jacobian_allocations
                .as_ref()
                .expect("Jacobian SSA and allocations are built together")[stamp_index];
            for group in plan_shared_outputs(stamp_jacobians) {
                let representative = group.representative();
                if inlines(&stamp_jacobians[representative]) {
                    compiler.emit_program(
                        &stamp_jacobians[representative],
                        &stamp_allocations[representative],
                    )?;
                    compiler.materialize_location(
                        stamp_allocations[representative].result(),
                        DReg::D0,
                    )?;
                } else {
                    // One call for the whole group, exactly as one inlined body
                    // served it: the outputs share a program, so they share the
                    // image function the value-entry cache published for it.
                    compiler.emit_image_entry_call(
                        kernel_image_offset,
                        entries.jacobians[stamp_index][representative],
                    )?;
                    compiler.emit_kernel_abort_if_failed()?;
                }
                for output in group.outputs() {
                    let output_index = jacobian_index.checked_add(*output).ok_or_else(|| {
                        encoding_error("AArch64 fused-kernel Jacobian output index overflow")
                    })?;
                    compiler.emit_kernel_jacobian_store(output_index)?;
                }
            }
            jacobian_index = jacobian_index
                .checked_add(stamp_jacobians.len())
                .ok_or_else(|| encoding_error("AArch64 fused-kernel Jacobian index overflow"))?;
        }
        let skip_target = compiler.encoder.position();
        compiler.encoder.patch_branch(skip_stamp, skip_target)?;
    }

    let bytes = compiler.finish_assignment_pass()?;
    verify_exact_function_at(&bytes, kernel_name, kernel_image_offset)?;
    Ok(bytes)
}

struct FunctionCompiler {
    encoder: A64Encoder,
    frame_bytes: usize,
    saves_entry_args: bool,
    saves_kernel_io: bool,
    early_returns: Vec<BranchPatch>,
    literals: Vec<(LiteralPatch, u64)>,
    /// Byte offset of the oldest `LDR (literal)` still waiting for its
    /// constant. The reach that has to be respected is measured from here.
    literal_window_start: Option<usize>,
    variable_window_enabled: bool,
    variable_window_base: Option<usize>,
}

/// Bytes held back from [`MAX_FORWARD_LITERAL_REACH_BYTES`] when deciding to
/// flush an inline constant island.
///
/// The decision is taken at instruction boundaries, so the margin has to cover
/// everything one SSA instruction can emit — a spilled-operand reload, a
/// helper call sequence, its own new constants — before the next boundary is
/// reached. One page is three orders of magnitude more than any single
/// instruction emits and still leaves the flush point far beyond the largest
/// function the shipped route builds, so no function that encodes today
/// changes by a byte.
const LITERAL_ISLAND_MARGIN_BYTES: usize = 4096;

/// The most constants one island may hold, bounded by the marker's `imm16`.
const MAX_LITERAL_ISLAND_WORDS: usize = u16::MAX as usize;

fn align_up_8(offset: usize) -> usize {
    offset.div_ceil(8) * 8
}

impl FunctionCompiler {
    fn new(frame_bytes: usize, uses_helper_calls: bool) -> JitResult<Self> {
        Self::new_with_kernel_io(frame_bytes, uses_helper_calls, false)
    }

    fn new_with_kernel_io(
        frame_bytes: usize,
        saves_entry_args: bool,
        saves_kernel_io: bool,
    ) -> JitResult<Self> {
        if saves_kernel_io && !saves_entry_args {
            return Err(register_allocation_error(
                "AArch64 kernel I/O requires preserved entry arguments",
            ));
        }
        if frame_bytes % HOST_ABI.stack_alignment != 0 {
            return Err(register_allocation_error(format!(
                "AArch64 frame size {frame_bytes} is not {}-byte aligned",
                HOST_ABI.stack_alignment
            )));
        }
        let mut compiler = Self {
            encoder: A64Encoder::new(),
            frame_bytes,
            saves_entry_args,
            saves_kernel_io,
            early_returns: Vec::new(),
            literals: Vec::new(),
            literal_window_start: None,
            variable_window_enabled: !saves_entry_args,
            variable_window_base: None,
        };
        compiler.encoder.bti_c();
        if compiler.saves_entry_args {
            compiler.encoder.stp_x_pre(
                HOST_ABI.saved_context,
                HOST_ABI.saved_variables,
                XReg::Sp,
                -16,
            )?;
            if compiler.saves_kernel_io {
                compiler
                    .encoder
                    .stp_x_pre(HOST_ABI.saved_kernel_io, XReg::X22, XReg::Sp, -16)?;
            }
            compiler.encoder.stp_x_pre(
                HOST_ABI.frame_pointer,
                HOST_ABI.link_register,
                XReg::Sp,
                -16,
            )?;
            compiler
                .encoder
                .add_x_imm(HOST_ABI.frame_pointer, XReg::Sp, 0)?;
            compiler
                .encoder
                .mov_x(HOST_ABI.saved_context, HOST_ABI.entry_context)?;
            compiler
                .encoder
                .mov_x(HOST_ABI.saved_variables, HOST_ABI.entry_variables)?;
            if compiler.saves_kernel_io {
                compiler
                    .encoder
                    .mov_x(HOST_ABI.saved_kernel_io, HOST_ABI.entry_kernel_io)?;
            }
        }
        compiler.adjust_stack(frame_bytes, false)?;
        Ok(compiler)
    }

    fn emit_program(&mut self, ssa: &Program, allocation: &RegisterAllocation) -> JitResult<()> {
        if ssa.instructions().len() != allocation.instructions().len() {
            return Err(verifier_error(
                "AArch64 SSA and register-allocation instruction counts differ",
            ));
        }

        let mut block_offsets: Vec<Option<usize>> = vec![None; ssa.blocks().len()];
        let mut pending_branches: Vec<(usize, BranchPatch)> = Vec::new();
        for block in ssa.blocks() {
            self.flush_literal_island_if_needed()?;
            block_offsets[block.id().index()] = Some(self.encoder.position());
            let range = block.instruction_start()..block.instruction_end();
            for (instruction, allocated) in ssa.instructions()[range.clone()]
                .iter()
                .zip(&allocation.instructions()[range])
            {
                self.flush_literal_island_if_needed()?;
                self.emit_allocated_instruction(instruction, allocated)?;
            }
            self.emit_terminator(allocation, block, &mut pending_branches)?;
        }
        for (target, patch) in pending_branches {
            let offset = block_offsets[target].ok_or_else(|| {
                verifier_error(format!(
                    "AArch64 branch targets block {target}, which was never emitted"
                ))
            })?;
            self.encoder.patch_branch(patch, offset)?;
        }
        Ok(())
    }

    /// Emit one block's terminator.
    ///
    /// `Return` emits nothing: the caller decides what to do with the
    /// allocation's result location, and the exit block is always last in
    /// layout order.
    fn emit_terminator(
        &mut self,
        allocation: &RegisterAllocation,
        block: &BasicBlock,
        pending_branches: &mut Vec<(usize, BranchPatch)>,
    ) -> JitResult<()> {
        let fallthrough = block.id().index() + 1;
        match block.terminator() {
            Terminator::Return(_) => Ok(()),
            Terminator::Jump(edge) => {
                self.emit_edge_moves(allocation, block.id(), 0)?;
                if edge.target().index() != fallthrough {
                    pending_branches.push((edge.target().index(), self.encoder.b_placeholder()));
                }
                Ok(())
            }
            Terminator::Branch {
                condition,
                then_edge,
                else_edge,
            } => {
                // FCMP against zero leaves Z clear for every nonzero value and
                // for an unordered compare, so NE is exactly Verilog-A
                // truthiness: the same condition the select form hands to
                // FCSEL.
                let condition = match allocation.location(*condition)? {
                    ValueLocation::Register(index) => logical_register(index)?,
                    ValueLocation::Spill(slot) => {
                        let mut used = A64_ALLOCATED_REGISTER_MASK;
                        let register = take_temporary(&mut used)?;
                        self.encoder.ldr_d_unsigned(
                            register,
                            XReg::Sp,
                            spill_slot_offset(slot)?,
                        )?;
                        register
                    }
                };
                self.encoder.fcmp_d_zero(condition);
                let taken = self.encoder.b_cond_placeholder(Condition::NotEqual);

                self.emit_edge_moves(allocation, block.id(), 1)?;
                // The taken arm's moves are laid out next, so the untaken edge
                // always needs its own branch even when its target follows.
                pending_branches.push((else_edge.target().index(), self.encoder.b_placeholder()));

                let taken_offset = self.encoder.position();
                self.encoder.patch_branch(taken, taken_offset)?;
                self.emit_edge_moves(allocation, block.id(), 0)?;
                if then_edge.target().index() != fallthrough {
                    pending_branches
                        .push((then_edge.target().index(), self.encoder.b_placeholder()));
                }
                Ok(())
            }
        }
    }

    fn emit_edge_moves(
        &mut self,
        allocation: &RegisterAllocation,
        block: BlockId,
        edge: usize,
    ) -> JitResult<()> {
        for step in allocation.edge_moves(block, edge)?.to_vec() {
            self.emit_location_move(step.from(), step.to())?;
        }
        Ok(())
    }

    fn emit_location_move(&mut self, from: ValueLocation, to: ValueLocation) -> JitResult<()> {
        match (from, to) {
            (ValueLocation::Register(source), ValueLocation::Register(destination)) => {
                let source = logical_register(source)?;
                let destination = logical_register(destination)?;
                if source != destination {
                    self.encoder.fmov_d(destination, source);
                }
            }
            (ValueLocation::Register(source), ValueLocation::Spill(slot)) => {
                let source = logical_register(source)?;
                self.encoder
                    .str_d_unsigned(source, XReg::Sp, spill_slot_offset(slot)?)?;
            }
            (ValueLocation::Spill(slot), ValueLocation::Register(destination)) => {
                let destination = logical_register(destination)?;
                self.encoder
                    .ldr_d_unsigned(destination, XReg::Sp, spill_slot_offset(slot)?)?;
            }
            (ValueLocation::Spill(source), ValueLocation::Spill(destination)) => {
                if source == destination {
                    return Ok(());
                }
                // The bank stops short of D16-D22, which the host convention
                // leaves volatile, so a temporary here costs no prologue save.
                let mut used = A64_ALLOCATED_REGISTER_MASK;
                let scratch = take_temporary(&mut used)?;
                self.encoder
                    .ldr_d_unsigned(scratch, XReg::Sp, spill_slot_offset(source)?)?;
                self.encoder
                    .str_d_unsigned(scratch, XReg::Sp, spill_slot_offset(destination)?)?;
            }
        }
        Ok(())
    }

    fn emit_allocated_instruction(
        &mut self,
        instruction: &Instruction,
        allocated: &AllocatedInstruction,
    ) -> JitResult<()> {
        if instruction.value_type() != ValueType::F64 {
            return Err(verifier_error(format!(
                "AArch64 codegen cannot legalize SSA value type {:?}",
                instruction.value_type()
            )));
        }
        let mut prepared = self.prepare_instruction(allocated)?;
        self.emit_op(instruction.op(), &mut prepared)?;
        if let ValueLocation::Spill(slot) = allocated.result() {
            self.encoder
                .str_d_unsigned(prepared.result, XReg::Sp, spill_slot_offset(slot)?)?;
        }
        Ok(())
    }

    fn emit_native_program_for_pass(
        &mut self,
        program: &NativeProgram,
    ) -> JitResult<ValueLocation> {
        validate_expression_stack_depth(program.max_stack_depth())?;
        let ssa = Program::lower(program)?;
        let allocation = RegisterAllocation::build(&ssa, A64_VALUE_BANK)?;
        self.emit_program(&ssa, &allocation)?;
        Ok(allocation.result())
    }

    fn emit_assignment_step(
        &mut self,
        assignment: &NativeAssignment,
        loop_depth: usize,
        layout: AssignmentFrameLayout,
    ) -> JitResult<()> {
        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                let result = self.emit_native_program_for_pass(program)?;
                let source = self.materialize_location(result, DReg::D0)?;
                self.emit_array_store(source, self.variables_register(), *var_index)
            }
            NativeAssignment::Indexed {
                base,
                len,
                lower,
                index,
                value,
            } => self.emit_indexed_assignment(*base, *len, *lower, index, value, layout),
            NativeAssignment::Loop { condition, body } => {
                self.emit_loop_assignment(condition, body, loop_depth, layout)
            }
        }
    }

    fn emit_assignment_steps(
        &mut self,
        assignments: &[NativeAssignment],
        loop_depth: usize,
        layout: AssignmentFrameLayout,
    ) -> JitResult<()> {
        for range in shareable_batch_ranges(assignments) {
            let batch = &assignments[range];
            if matches!(batch.first(), Some(NativeAssignment::Direct { .. })) {
                self.emit_direct_assignment_batch(batch)?;
            } else {
                debug_assert_eq!(batch.len(), 1);
                self.emit_assignment_step(&batch[0], loop_depth, layout)?;
            }
        }
        Ok(())
    }

    fn emit_direct_assignment_batch(&mut self, assignments: &[NativeAssignment]) -> JitResult<()> {
        let direct = assignments
            .iter()
            .map(|assignment| match assignment {
                NativeAssignment::Direct { var_index, program } => Ok((*var_index, program)),
                NativeAssignment::Indexed { .. } | NativeAssignment::Loop { .. } => {
                    Err(verifier_error(
                        "AArch64 direct-assignment batch contains a control-flow assignment",
                    ))
                }
            })
            .collect::<JitResult<Vec<_>>>()?;
        let ssa = AssignmentProgram::lower(&direct)?;
        let allocation = RegisterAllocation::build_for_assignments(&ssa, A64_VALUE_BANK)?;
        if ssa.program().instructions().len() != allocation.instructions().len() {
            return Err(verifier_error(
                "AArch64 assignment SSA and register-allocation instruction counts differ",
            ));
        }

        let mut output_index = 0_usize;
        for (instruction_index, (instruction, allocated)) in ssa
            .program()
            .instructions()
            .iter()
            .zip(allocation.instructions())
            .enumerate()
        {
            self.flush_literal_island_if_needed()?;
            self.emit_allocated_instruction(instruction, allocated)?;
            let instruction_end = instruction_index + 1;
            while let Some(output) = ssa.outputs().get(output_index).copied() {
                if output.instruction_end() != instruction_end {
                    break;
                }
                let location = allocation.location(output.value())?;
                let source = self.materialize_location(location, DReg::D31)?;
                self.emit_array_store(source, self.variables_register(), output.variable_index())?;
                output_index += 1;
            }
        }
        if output_index != ssa.outputs().len() {
            return Err(verifier_error(format!(
                "AArch64 emitted {output_index} of {} assignment outputs",
                ssa.outputs().len()
            )));
        }
        Ok(())
    }

    fn emit_indexed_assignment(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
        index: &NativeProgram,
        value: &NativeProgram,
        layout: AssignmentFrameLayout,
    ) -> JitResult<()> {
        let slot_offset = layout
            .indexed_slot
            .ok_or_else(|| JitError::InternalCompilerError {
                model: MODEL.into(),
                detail: "AArch64 indexed assignment has no reserved pointer slot".into(),
            })?;
        let index_result = self.emit_native_program_for_pass(index)?;
        self.materialize_location(index_result, DReg::D0)?;
        self.stack_store_d(DReg::D0, slot_offset)?;
        self.emit_variable_base_pointer(XReg::X0, base)?;
        self.encoder.mov_u64(XReg::X1, len as u64)?;
        self.encoder.mov_u64(XReg::X2, lower as u64)?;
        self.encoder.mov_u64(
            HOST_ABI.indirect_call_scratch,
            rspice_dynamic_variable_slot_native as *const () as usize as u64,
        )?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        let invalid = self.encoder.cbz_placeholder(XReg::X0)?;
        self.stack_store_x(XReg::X0, slot_offset)?;
        let slot_ready = self.encoder.b_placeholder();

        let invalid_target = self.encoder.position();
        self.encoder.patch_branch(invalid, invalid_target)?;
        self.stack_load_d(DReg::D0, slot_offset)?;
        self.emit_dynamic_variable_error(len, lower)?;

        let slot_ready_target = self.encoder.position();
        self.encoder.patch_branch(slot_ready, slot_ready_target)?;
        let value_result = self.emit_native_program_for_pass(value)?;
        self.materialize_location(value_result, DReg::D0)?;
        self.stack_load_x(XReg::X9, slot_offset)?;
        self.encoder.str_d_unsigned(DReg::D0, XReg::X9, 0)
    }

    fn emit_loop_assignment(
        &mut self,
        condition: &NativeProgram,
        body: &[NativeAssignment],
        loop_depth: usize,
        layout: AssignmentFrameLayout,
    ) -> JitResult<()> {
        let counter_offset = layout.loop_counter_offset(loop_depth)?;
        self.encoder.mov_u64(XReg::X9, 0)?;
        self.stack_store_x(XReg::X9, counter_offset)?;
        let loop_start = self.encoder.position();

        let condition_result = self.emit_native_program_for_pass(condition)?;
        self.materialize_location(condition_result, DReg::D0)?;
        self.emit_literal(DReg::D31, 0.0)?;
        self.encoder.fcmp_d(DReg::D0, DReg::D31);
        let loop_exit = self.encoder.b_cond_placeholder(Condition::Equal);

        self.emit_assignment_steps(body, loop_depth + 1, layout)?;

        self.stack_load_x(XReg::X9, counter_offset)?;
        self.encoder.add_x_imm(XReg::X9, XReg::X9, 1)?;
        self.stack_store_x(XReg::X9, counter_offset)?;
        self.encoder
            .mov_u64(XReg::X10, MAX_RUNTIME_LOOP_ITERATIONS)?;
        self.encoder.cmp_x(XReg::X9, XReg::X10)?;
        let limit_reached = self.encoder.b_cond_placeholder(Condition::CarrySet);
        let repeat = self.encoder.b_placeholder();
        self.encoder.patch_branch(repeat, loop_start)?;

        let limit_target = self.encoder.position();
        self.encoder.patch_branch(limit_reached, limit_target)?;
        self.emit_void_error_early_return(rspice_native_loop_limit_error as *const () as usize)?;

        let exit_target = self.encoder.position();
        self.encoder.patch_branch(loop_exit, exit_target)
    }

    fn materialize_location(
        &mut self,
        location: ValueLocation,
        destination: DReg,
    ) -> JitResult<DReg> {
        match location {
            ValueLocation::Register(index) => {
                let source = logical_register(index)?;
                if source != destination {
                    self.encoder.fmov_d(destination, source);
                }
            }
            ValueLocation::Spill(slot) => {
                self.encoder
                    .ldr_d_unsigned(destination, XReg::Sp, spill_slot_offset(slot)?)?
            }
        }
        Ok(destination)
    }

    fn emit_image_entry_call(
        &mut self,
        function_image_offset: usize,
        target: CodeOffset,
    ) -> JitResult<()> {
        self.encoder
            .mov_x(HOST_ABI.entry_context, self.context_register())?;
        self.encoder
            .mov_x(HOST_ABI.entry_variables, self.variables_register())?;
        self.encoder.image_call(
            function_image_offset,
            target.as_usize(),
            HOST_ABI.indirect_call_scratch,
        )
    }

    fn emit_image_segment_call(
        &mut self,
        function_image_offset: usize,
        target: CodeOffset,
    ) -> JitResult<()> {
        self.encoder
            .mov_x(HOST_ABI.entry_context, self.context_register())?;
        self.encoder
            .mov_x(HOST_ABI.entry_variables, self.variables_register())?;
        self.encoder.add_x_imm(XReg::X2, XReg::Sp, 0)?;
        self.encoder.image_call(
            function_image_offset,
            target.as_usize(),
            HOST_ABI.indirect_call_scratch,
        )
    }

    fn emit_kernel_abort_if_failed(&mut self) -> JitResult<()> {
        let failed_offset = std::mem::offset_of!(EvalContext, runtime_status)
            .checked_add(NativeRuntimeStatus::failed_offset())
            .ok_or_else(|| encoding_error("AArch64 native runtime-status offset overflow"))?;
        self.encoder
            .ldrb_w_unsigned(XReg::X9, self.context_register(), failed_offset)?;
        let abort = self.encoder.cbnz_placeholder(XReg::X9)?;
        self.early_returns.push(abort);
        Ok(())
    }

    fn emit_kernel_skip_if_inactive(&mut self, stamp_index: usize) -> JitResult<BranchPatch> {
        self.encoder
            .ldr_x_unsigned(XReg::X9, self.kernel_io_register()?, KERNEL_ACTIVE_OFFSET)?;
        self.emit_u8_array_load(XReg::X10, XReg::X9, stamp_index)?;
        self.encoder.cbz_placeholder(XReg::X10)
    }

    fn emit_kernel_non_finite_guard(&mut self, stamp_index: usize) -> JitResult<()> {
        const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;

        self.encoder.fmov_x_d(XReg::X9, DReg::D0)?;
        self.encoder.mov_u64(XReg::X10, EXPONENT_MASK)?;
        self.encoder.and_x(XReg::X9, XReg::X9, XReg::X10)?;
        self.encoder.cmp_x(XReg::X9, XReg::X10)?;
        let finite = self.encoder.b_cond_placeholder(Condition::NotEqual);

        self.encoder
            .mov_x(HOST_ABI.entry_context, self.context_register())?;
        self.encoder
            .mov_u64(HOST_ABI.entry_variables, stamp_index as u64)?;
        self.encoder.mov_u64(
            HOST_ABI.indirect_call_scratch,
            rspice_native_non_finite_contribution_error as *const () as usize as u64,
        )?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        self.early_returns.push(self.encoder.b_placeholder());

        let finite_target = self.encoder.position();
        self.encoder.patch_branch(finite, finite_target)
    }

    fn emit_kernel_stamp_value_store(
        &mut self,
        stamp_index: usize,
        current_pair: Option<(usize, usize)>,
    ) -> JitResult<()> {
        self.encoder
            .ldr_x_unsigned(XReg::X9, self.context_register(), CURRENTS_OFFSET)?;
        self.emit_array_store(DReg::D0, XReg::X9, stamp_index)?;

        if let Some((forward, reverse)) = current_pair {
            self.encoder.ldr_x_unsigned(
                XReg::X9,
                self.context_register(),
                BRANCH_CURRENTS_OFFSET,
            )?;
            self.emit_array_store(DReg::D0, XReg::X9, forward)?;
            if forward != reverse {
                self.encoder.fmov_x_d(XReg::X10, DReg::D0)?;
                self.encoder.mov_u64(XReg::X11, (-0.0_f64).to_bits())?;
                self.encoder.eor_x(XReg::X10, XReg::X10, XReg::X11)?;
                self.encoder.fmov_d_x(DReg::D1, XReg::X10)?;
                self.emit_array_store(DReg::D1, XReg::X9, reverse)?;
            }
        }
        Ok(())
    }

    fn emit_kernel_jacobian_store(&mut self, jacobian_index: usize) -> JitResult<()> {
        self.encoder.ldr_x_unsigned(
            XReg::X9,
            self.kernel_io_register()?,
            KERNEL_JACOBIANS_OFFSET,
        )?;
        self.emit_array_store(DReg::D0, XReg::X9, jacobian_index)
    }

    fn prepare_instruction(
        &mut self,
        allocated: &AllocatedInstruction,
    ) -> JitResult<PreparedInstruction> {
        let mut used_mask = allocated.live_register_mask();
        if let ValueLocation::Register(index) = allocated.result() {
            used_mask |= register_mask(logical_register(index)?)?;
        }
        let mut operands = Vec::with_capacity(allocated.operands().len());
        for location in allocated.operands() {
            let register = match *location {
                ValueLocation::Register(index) => logical_register(index)?,
                ValueLocation::Spill(slot) => {
                    let register = take_temporary(&mut used_mask)?;
                    self.encoder
                        .ldr_d_unsigned(register, XReg::Sp, spill_slot_offset(slot)?)?;
                    register
                }
            };
            used_mask |= register_mask(register)?;
            operands.push(register);
        }

        let result = match allocated.result() {
            ValueLocation::Register(index) => logical_register(index)?,
            ValueLocation::Spill(_) => match (allocated.operands().first(), operands.first()) {
                (Some(ValueLocation::Spill(_)), Some(register)) => *register,
                _ => take_temporary(&mut used_mask)?,
            },
        };
        used_mask |= register_mask(result)?;
        if let Some(first) = operands.first()
            && *first != result
        {
            self.encoder.fmov_d(result, *first);
        }

        Ok(PreparedInstruction {
            operands,
            result,
            used_mask,
        })
    }

    fn emit_op(&mut self, op: NativeOp, prepared: &mut PreparedInstruction) -> JitResult<()> {
        let result = prepared.result;
        match op {
            NativeOp::Const(value) => self.emit_literal(result, value)?,
            NativeOp::LoadParam(index) => {
                self.emit_context_array_load(result, PARAMS_OFFSET, index)?
            }
            NativeOp::LoadParamGiven(index) => self.emit_guarded_context_u8_load(
                result,
                PARAM_GIVEN_OFFSET,
                PARAM_GIVEN_LEN_OFFSET,
                index,
                rspice_native_param_given_error as *const () as usize,
            )?,
            NativeOp::LoadPortConnected(index) => self.emit_guarded_context_u8_load(
                result,
                PORT_CONNECTED_OFFSET,
                PORT_CONNECTED_LEN_OFFSET,
                index,
                rspice_native_port_connected_error as *const () as usize,
            )?,
            NativeOp::LoadVoltage { pos, neg } => {
                self.emit_voltage_load(result, pos, neg, prepared)?
            }
            NativeOp::LoadCurrent(index) => self.emit_guarded_context_f64_load(
                result,
                BRANCH_CURRENTS_OFFSET,
                BRANCH_CURRENTS_LEN_OFFSET,
                index,
                rspice_native_current_probe_error as *const () as usize,
            )?,
            NativeOp::LoadPriorCurrent(index) => self.emit_guarded_context_f64_load(
                result,
                CURRENTS_OFFSET,
                CURRENTS_LEN_OFFSET,
                index,
                rspice_native_prior_current_error as *const () as usize,
            )?,
            NativeOp::LoadInternalVoltage(index) => {
                self.emit_context_array_load(result, INTERNAL_VOLTAGES_OFFSET, index)?
            }
            NativeOp::LoadPreludeSlot(index) => {
                self.emit_context_array_load(result, PRELUDE_SLOTS_OFFSET, index)?
            }
            // An identity on its operand. `prepare` has already moved operand
            // zero into `result`, so the value to publish is `result` itself and
            // the store is the whole of the operation.
            NativeOp::StorePreludeSlot(index) => {
                self.emit_context_array_store(result, PRELUDE_SLOTS_OFFSET, index)?
            }
            NativeOp::LoadVariable(index) => {
                self.emit_array_load(result, self.variables_register(), index)?
            }
            NativeOp::LoadVariableDyn { base, len, lower } => {
                self.emit_dynamic_variable_load(prepared, base, len, lower)?
            }
            NativeOp::LoadBranchUnknown(index) => {
                self.emit_context_array_load(result, BRANCH_UNKNOWNS_OFFSET, index)?
            }
            NativeOp::LoadTemperature => self.emit_context_f64_load(result, TEMPERATURE_OFFSET)?,
            NativeOp::LoadThermalVoltage => {
                self.emit_context_f64_load(result, TEMPERATURE_OFFSET)?;
                let literal = prepared.temporary()?;
                self.emit_literal(literal, THERMAL_VOLTAGE_PER_K)?;
                self.encoder.fmul_d(result, result, literal);
            }
            NativeOp::LoadTime => self.emit_context_f64_load(result, TIME_OFFSET)?,
            NativeOp::Analysis(analysis_id) => self.emit_analysis(result, analysis_id)?,
            NativeOp::LoadMfactor => self.emit_context_f64_load(result, MFACTOR_OFFSET)?,
            NativeOp::Add => self.emit_binary(prepared, BinaryOp::Add)?,
            NativeOp::Sub => self.emit_binary(prepared, BinaryOp::Sub)?,
            NativeOp::Mul => self.emit_binary(prepared, BinaryOp::Mul)?,
            NativeOp::Div => self.emit_binary(prepared, BinaryOp::Div)?,
            NativeOp::AddConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Add, false)?
            }
            NativeOp::SubConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Sub, false)?
            }
            NativeOp::MulConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Mul, false)?
            }
            NativeOp::DivConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Div, false)?
            }
            NativeOp::SubFromConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Sub, true)?
            }
            NativeOp::DivFromConst(value) => {
                self.emit_binary_const(prepared, value, BinaryOp::Div, true)?
            }
            NativeOp::Neg => self.encoder.fneg_d(result, unary_operand(prepared)?),
            NativeOp::Abs => self.encoder.fabs_d(result, unary_operand(prepared)?),
            NativeOp::Square => {
                let source = unary_operand(prepared)?;
                self.encoder.fmul_d(result, source, source);
            }
            NativeOp::Sqrt => self.encoder.fsqrt_d(result, unary_operand(prepared)?),
            NativeOp::Compare(compare) => self.emit_compare(prepared, compare, None)?,
            NativeOp::CompareConst(compare, value) => {
                self.emit_compare(prepared, compare, Some(value))?
            }
            NativeOp::Logical(logical) => self.emit_logical(prepared, logical)?,
            NativeOp::LogicalConst(logical, value) => {
                self.emit_logical_const(prepared, logical, value)?
            }
            NativeOp::IfElse => self.emit_if_else(prepared)?,
            NativeOp::Extremum(extremum) => self.emit_extremum(prepared, extremum, None, false)?,
            NativeOp::ExtremumConst(extremum, value) => {
                self.emit_extremum(prepared, extremum, Some(value), false)?
            }
            NativeOp::ExtremumConstLhs(extremum, value) => {
                self.emit_extremum(prepared, extremum, Some(value), true)?
            }
            NativeOp::UnaryMath(math) => self.emit_unary_math(prepared, math)?,
            NativeOp::BinaryMath(math) => self.emit_binary_math(prepared, math)?,
            NativeOp::IntegerCast => self.emit_integer_cast(prepared)?,
            NativeOp::IntegerBinary(integer) => self.emit_integer_binary(prepared, integer)?,
            NativeOp::IntegerShiftConst(integer, count) => {
                self.emit_integer_shift_const(prepared, integer, count)?
            }
            NativeOp::IntegerBinaryConst(integer, value) => {
                self.emit_integer_binary_const(prepared, integer, value)?
            }
            NativeOp::TableLookup(table_id) => self.emit_scalar_context_helper(
                prepared,
                table_id,
                rspice_table_lookup_native as *const () as usize,
            )?,
            NativeOp::TableDerivative(table_id) => self.emit_scalar_context_helper(
                prepared,
                table_id,
                rspice_table_derivative_native as *const () as usize,
            )?,
            NativeOp::LimitState(state_id) => self.emit_limit_state(prepared, state_id)?,
            NativeOp::LimiterPrevious(state_id) => self.emit_scalar_context_helper(
                prepared,
                state_id,
                rspice_limiter_previous_native as *const () as usize,
            )?,
            NativeOp::LimiterStore(state_id) => self.emit_operand_context_helper(
                prepared,
                2,
                state_id,
                rspice_limiter_store_native as *const () as usize,
            )?,
            NativeOp::LaplaceState(filter_id) => self.emit_scalar_context_helper(
                prepared,
                filter_id,
                rspice_laplace_step_native as *const () as usize,
            )?,
            NativeOp::LaplaceStateDerivative(filter_id) => self.emit_scalar_context_helper(
                prepared,
                filter_id,
                rspice_laplace_derivative_native as *const () as usize,
            )?,
            NativeOp::ZiState(layout) => {
                let operands =
                    layout
                        .validate_operand_budget()
                        .map_err(|error| JitError::Encoding {
                            model: "native-aarch64".into(),
                            detail: error.to_string().into(),
                        })?;
                self.emit_operand_context_helper(
                    prepared,
                    operands,
                    layout
                        .native_descriptor()
                        .ok_or_else(|| JitError::Encoding {
                            model: "native-aarch64".into(),
                            detail: "Zi runtime layout exceeds the native descriptor limits".into(),
                        })?,
                    rspice_zi_step_native as *const () as usize,
                )?;
            }
            NativeOp::ZiStateDerivative(layout) => {
                let operands =
                    layout
                        .validate_operand_budget()
                        .map_err(|error| JitError::Encoding {
                            model: "native-aarch64".into(),
                            detail: error.to_string().into(),
                        })?;
                self.emit_operand_context_helper(
                    prepared,
                    operands,
                    layout
                        .native_descriptor()
                        .ok_or_else(|| JitError::Encoding {
                            model: "native-aarch64".into(),
                            detail:
                                "Zi derivative runtime layout exceeds the native descriptor limits"
                                    .into(),
                        })?,
                    rspice_zi_derivative_native as *const () as usize,
                )?;
            }
            NativeOp::TimerState(timer_id) => self.emit_operand_context_helper(
                prepared,
                4,
                timer_id,
                rspice_timer_state_native as *const () as usize,
            )?,
            NativeOp::TransitionState(filter_id) => self.emit_operand_context_helper(
                prepared,
                4,
                filter_id,
                rspice_transition_state_native as *const () as usize,
            )?,
            NativeOp::TransitionStateDerivative(filter_id) => self.emit_operand_context_helper(
                prepared,
                5,
                filter_id,
                rspice_transition_derivative_native as *const () as usize,
            )?,
            NativeOp::SlewState(filter_id) => self.emit_operand_context_helper(
                prepared,
                3,
                filter_id,
                rspice_slew_state_native as *const () as usize,
            )?,
            NativeOp::SlewStateDerivative(filter_id) => self.emit_operand_context_helper(
                prepared,
                6,
                filter_id,
                rspice_slew_derivative_native as *const () as usize,
            )?,
            NativeOp::AbsDelayState(buffer_id) => self.emit_operand_context_helper(
                prepared,
                2,
                buffer_id,
                rspice_absdelay_state_native as *const () as usize,
            )?,
            NativeOp::AbsDelayStateMax(buffer_id) => self.emit_operand_context_helper(
                prepared,
                3,
                buffer_id,
                rspice_absdelay_state_max_native as *const () as usize,
            )?,
            NativeOp::AbsDelayStateDerivative(buffer_id) => self.emit_operand_context_helper(
                prepared,
                4,
                buffer_id,
                rspice_absdelay_derivative_native as *const () as usize,
            )?,
            NativeOp::AbsDelayStateDerivativeMax(buffer_id) => self.emit_operand_context_helper(
                prepared,
                5,
                buffer_id,
                rspice_absdelay_derivative_max_native as *const () as usize,
            )?,
            NativeOp::CrossState(detector_id) => self.emit_operand_context_helper(
                prepared,
                5,
                detector_id,
                rspice_cross_state_native as *const () as usize,
            )?,
            NativeOp::AboveState(detector_id) => self.emit_operand_context_helper(
                prepared,
                4,
                detector_id,
                rspice_above_state_native as *const () as usize,
            )?,
            NativeOp::LastCrossingState(detector_id) => self.emit_operand_context_helper(
                prepared,
                2,
                detector_id,
                rspice_last_crossing_state_native as *const () as usize,
            )?,
            NativeOp::DdtState(state_id) => self.emit_operand_context_helper(
                prepared,
                1,
                state_id,
                rspice_ddt_state_native as *const () as usize,
            )?,
            NativeOp::DdtJacobian => self.emit_operand_context_helper(
                prepared,
                1,
                0,
                rspice_ddt_jacobian_native as *const () as usize,
            )?,
            NativeOp::IdtState(state_id) => self.emit_operand_context_helper(
                prepared,
                2,
                state_id,
                rspice_idt_state_native as *const () as usize,
            )?,
            NativeOp::IdtJacobian => self.emit_operand_context_helper(
                prepared,
                1,
                0,
                rspice_idt_jacobian_native as *const () as usize,
            )?,
            NativeOp::IdtModState(state_id) => self.emit_operand_context_helper(
                prepared,
                4,
                state_id,
                rspice_idtmod_state_native as *const () as usize,
            )?,
            NativeOp::WhiteNoise | NativeOp::FlickerNoise => self.emit_literal(result, 0.0)?,
        }
        Ok(())
    }

    fn emit_binary(&mut self, prepared: &PreparedInstruction, op: BinaryOp) -> JitResult<()> {
        let [left, right] = prepared.operands.as_slice() else {
            return Err(encoding_error(format!(
                "AArch64 {op:?} requires two SSA operands"
            )));
        };
        self.emit_binary_registers(prepared.result, *left, *right, op);
        Ok(())
    }

    fn emit_binary_const(
        &mut self,
        prepared: &mut PreparedInstruction,
        value: f64,
        op: BinaryOp,
        literal_is_left: bool,
    ) -> JitResult<()> {
        let source = unary_operand(prepared)?;
        let literal = prepared.temporary()?;
        self.emit_literal(literal, value)?;
        let (left, right) = if literal_is_left {
            (literal, source)
        } else {
            (source, literal)
        };
        self.emit_binary_registers(prepared.result, left, right, op);
        Ok(())
    }

    fn emit_binary_registers(&mut self, result: DReg, left: DReg, right: DReg, op: BinaryOp) {
        match op {
            BinaryOp::Add => self.encoder.fadd_d(result, left, right),
            BinaryOp::Sub => self.encoder.fsub_d(result, left, right),
            BinaryOp::Mul => self.encoder.fmul_d(result, left, right),
            BinaryOp::Div => self.encoder.fdiv_d(result, left, right),
        }
    }

    fn emit_unary_math(
        &mut self,
        prepared: &PreparedInstruction,
        op: UnaryMathOp,
    ) -> JitResult<()> {
        let source = unary_operand(prepared)?;
        match op {
            UnaryMathOp::Floor => self.encoder.frintm_d(prepared.result, source),
            UnaryMathOp::Ceil => self.encoder.frintp_d(prepared.result, source),
            _ => self.emit_unary_helper_call(prepared.result, source, unary_math_helper(op))?,
        }
        Ok(())
    }

    fn emit_binary_math(
        &mut self,
        prepared: &PreparedInstruction,
        op: BinaryMathOp,
    ) -> JitResult<()> {
        let (left, right) = binary_operands(prepared)?;
        self.emit_binary_helper_call(prepared.result, left, right, binary_math_helper(op))
    }

    fn emit_unary_helper_call(
        &mut self,
        result: DReg,
        source: DReg,
        helper: UnaryHelper,
    ) -> JitResult<()> {
        if !self.saves_entry_args {
            return Err(verifier_error(
                "AArch64 helper call emitted without a nonvolatile entry frame",
            ));
        }
        if source != DReg::D0 {
            self.encoder.fmov_d(DReg::D0, source);
        }
        self.encoder
            .mov_u64(HOST_ABI.indirect_call_scratch, helper as usize as u64)?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        if result != DReg::D0 {
            self.encoder.fmov_d(result, DReg::D0);
        }
        Ok(())
    }

    fn emit_binary_helper_call(
        &mut self,
        result: DReg,
        left: DReg,
        right: DReg,
        helper: BinaryHelper,
    ) -> JitResult<()> {
        if !self.saves_entry_args {
            return Err(verifier_error(
                "AArch64 binary helper call emitted without a nonvolatile entry frame",
            ));
        }
        // Route the first argument through X17 as an exact bit-preserving
        // temporary. This is a total parallel move for every D0/D1 aliasing
        // arrangement, including swapped and identical operands.
        self.encoder.fmov_x_d(XReg::X17, left)?;
        if right != DReg::D1 {
            self.encoder.fmov_d(DReg::D1, right);
        }
        self.encoder.fmov_d_x(DReg::D0, XReg::X17)?;
        self.encoder
            .mov_u64(HOST_ABI.indirect_call_scratch, helper as usize as u64)?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        if result != DReg::D0 {
            self.encoder.fmov_d(result, DReg::D0);
        }
        Ok(())
    }

    fn emit_scalar_context_helper(
        &mut self,
        prepared: &PreparedInstruction,
        state_id: usize,
        helper: usize,
    ) -> JitResult<()> {
        if !self.saves_entry_args {
            return Err(verifier_error(
                "AArch64 context helper emitted without a nonvolatile entry frame",
            ));
        }
        let source = unary_operand(prepared)?;
        if source != DReg::D0 {
            self.encoder.fmov_d(DReg::D0, source);
        }
        self.encoder.mov_x(XReg::X0, self.context_register())?;
        self.encoder.mov_u64(XReg::X1, state_id as u64)?;
        self.encoder
            .mov_u64(HOST_ABI.indirect_call_scratch, helper as u64)?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        if prepared.result != DReg::D0 {
            self.encoder.fmov_d(prepared.result, DReg::D0);
        }
        Ok(())
    }

    fn emit_operand_context_helper(
        &mut self,
        prepared: &PreparedInstruction,
        operand_count: usize,
        state_id: usize,
        helper: usize,
    ) -> JitResult<()> {
        if !self.saves_entry_args {
            return Err(verifier_error(
                "AArch64 operand helper emitted without a nonvolatile entry frame",
            ));
        }
        if prepared.operands.len() != operand_count {
            return Err(encoding_error(format!(
                "AArch64 operand helper requires {operand_count} operands, received {}",
                prepared.operands.len()
            )));
        }
        let call_frame_bytes = operand_count
            .checked_mul(WORD_BYTES)
            .and_then(|bytes| bytes.checked_add(STACK_ALIGNMENT - 1))
            .map(|bytes| bytes / STACK_ALIGNMENT * STACK_ALIGNMENT)
            .ok_or_else(|| encoding_error("AArch64 operand-helper frame size overflow"))?;
        self.adjust_stack(call_frame_bytes, false)?;
        for (index, operand) in prepared.operands.iter().copied().enumerate() {
            self.encoder
                .str_d_unsigned(operand, XReg::Sp, index * WORD_BYTES)?;
        }
        self.encoder.add_x_imm(XReg::X0, XReg::Sp, 0)?;
        self.encoder.mov_x(XReg::X1, self.context_register())?;
        self.encoder.mov_u64(XReg::X2, state_id as u64)?;
        self.encoder
            .mov_u64(HOST_ABI.indirect_call_scratch, helper as u64)?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        self.adjust_stack(call_frame_bytes, true)?;
        if prepared.result != DReg::D0 {
            self.encoder.fmov_d(prepared.result, DReg::D0);
        }
        Ok(())
    }

    fn emit_integer_cast(&mut self, prepared: &PreparedInstruction) -> JitResult<()> {
        self.emit_operand_context_helper(
            prepared,
            1,
            INTEGER_CAST_DESCRIPTOR,
            rspice_integer_operation_native as *const () as usize,
        )
    }

    fn emit_integer_binary(
        &mut self,
        prepared: &PreparedInstruction,
        op: IntegerBinaryOp,
    ) -> JitResult<()> {
        self.emit_operand_context_helper(
            prepared,
            2,
            integer_binary_descriptor(runtime_integer_operation(op)),
            rspice_integer_operation_native as *const () as usize,
        )
    }

    fn emit_integer_shift_const(
        &mut self,
        prepared: &PreparedInstruction,
        op: IntegerBinaryOp,
        count: u8,
    ) -> JitResult<()> {
        self.emit_operand_context_helper(
            prepared,
            1,
            integer_shift_const_descriptor(runtime_integer_operation(op), count),
            rspice_integer_operation_native as *const () as usize,
        )
    }

    fn emit_integer_binary_const(
        &mut self,
        prepared: &PreparedInstruction,
        op: IntegerBinaryOp,
        value: i64,
    ) -> JitResult<()> {
        let descriptor = integer_binary_const_descriptor(runtime_integer_operation(op), value)
            .ok_or_else(|| {
                encoding_error(format!(
                    "constant Verilog-AMS integer operand {value} is outside signed 32-bit range"
                ))
            })?;
        self.emit_operand_context_helper(
            prepared,
            1,
            descriptor,
            rspice_integer_operation_native as *const () as usize,
        )
    }

    fn emit_compare(
        &mut self,
        prepared: &mut PreparedInstruction,
        op: CompareOp,
        rhs_literal: Option<f64>,
    ) -> JitResult<()> {
        let (left, right) = match rhs_literal {
            Some(value) => {
                let left = unary_operand(prepared)?;
                let right = prepared.temporary()?;
                self.emit_literal(right, value)?;
                (left, right)
            }
            None => binary_operands(prepared)?,
        };
        self.encoder.fcmp_d(left, right);
        self.emit_boolean_result(prepared.result, compare_condition(op))
    }

    fn emit_logical(&mut self, prepared: &mut PreparedInstruction, op: LogicalOp) -> JitResult<()> {
        match op {
            LogicalOp::Not => {
                let source = unary_operand(prepared)?;
                self.encoder.fcmp_d_zero(source);
                self.emit_boolean_result(prepared.result, Condition::Equal)?;
            }
            LogicalOp::And | LogicalOp::Or => {
                let (left, right) = binary_operands(prepared)?;
                self.encoder.fcmp_d_zero(left);
                self.encoder.cset_x(XReg::X9, Condition::NotEqual)?;
                self.encoder.fcmp_d_zero(right);
                self.encoder.cset_x(XReg::X10, Condition::NotEqual)?;
                match op {
                    LogicalOp::And => self.encoder.and_x(XReg::X9, XReg::X9, XReg::X10)?,
                    LogicalOp::Or => self.encoder.orr_x(XReg::X9, XReg::X9, XReg::X10)?,
                    LogicalOp::Not => unreachable!("handled above"),
                }
                self.encoder.scvtf_d_x(prepared.result, XReg::X9)?;
            }
        }
        Ok(())
    }

    fn emit_logical_const(
        &mut self,
        prepared: &mut PreparedInstruction,
        op: LogicalOp,
        rhs: bool,
    ) -> JitResult<()> {
        let source = unary_operand(prepared)?;
        match (op, rhs) {
            (LogicalOp::And, false) => {
                self.encoder.fmov_d_positive_zero(prepared.result);
            }
            (LogicalOp::Or, true) => {
                if !self.encoder.fmov_d_imm(prepared.result, 1.0) {
                    return Err(encoding_error("AArch64 cannot encode boolean one"));
                }
            }
            (LogicalOp::And, true) | (LogicalOp::Or, false) => {
                self.encoder.fcmp_d_zero(source);
                self.emit_boolean_result(prepared.result, Condition::NotEqual)?;
            }
            (LogicalOp::Not, _) => {
                return Err(encoding_error(
                    "logical constant lowering cannot carry a NOT operand",
                ));
            }
        }
        Ok(())
    }

    fn emit_if_else(&mut self, prepared: &mut PreparedInstruction) -> JitResult<()> {
        let [condition, then_value, else_value] = prepared.operands.as_slice() else {
            return Err(encoding_error(
                "AArch64 if/else requires three SSA operands",
            ));
        };
        let condition = *condition;
        let then_value = *then_value;
        let else_value = *else_value;
        self.encoder.fcmp_d_zero(condition);
        self.encoder
            .fcsel_d(prepared.result, then_value, else_value, Condition::NotEqual);
        Ok(())
    }

    fn emit_boolean_result(&mut self, result: DReg, condition: Condition) -> JitResult<()> {
        self.encoder.cset_x(XReg::X9, condition)?;
        self.encoder.scvtf_d_x(result, XReg::X9)
    }

    fn emit_extremum(
        &mut self,
        prepared: &mut PreparedInstruction,
        op: ExtremumOp,
        literal: Option<f64>,
        literal_is_left: bool,
    ) -> JitResult<()> {
        let (left, right) = match literal {
            None => binary_operands(prepared)?,
            Some(value) => {
                let source = unary_operand(prepared)?;
                let constant = prepared.temporary()?;
                self.emit_literal(constant, value)?;
                if literal_is_left {
                    (constant, source)
                } else {
                    (source, constant)
                }
            }
        };

        // IEEE-aware min/max: return the numeric operand if exactly one input
        // is NaN, retain the RHS if both are NaN, and retain the LHS for equal
        // values so signed zero matches the established x64 contract.
        self.encoder.fcmp_d(left, left);
        let left_nan = self.encoder.b_cond_placeholder(Condition::NotEqual);
        self.encoder.fcmp_d(right, right);
        let right_nan = self.encoder.b_cond_placeholder(Condition::NotEqual);
        self.encoder.fcmp_d(left, right);
        let select_left = match op {
            ExtremumOp::Min => Condition::UnsignedLowerOrSame,
            ExtremumOp::Max => Condition::SignedGreaterOrEqual,
        };
        self.encoder
            .fcsel_d(prepared.result, left, right, select_left);
        let done_ordered = self.encoder.b_placeholder();

        let left_nan_target = self.encoder.position();
        self.encoder.patch_branch(left_nan, left_nan_target)?;
        self.encoder.fmov_d(prepared.result, right);
        let done_left_nan = self.encoder.b_placeholder();

        let right_nan_target = self.encoder.position();
        self.encoder.patch_branch(right_nan, right_nan_target)?;
        self.encoder.fmov_d(prepared.result, left);

        let done = self.encoder.position();
        self.encoder.patch_branch(done_ordered, done)?;
        self.encoder.patch_branch(done_left_nan, done)?;
        Ok(())
    }

    fn emit_voltage_load(
        &mut self,
        result: DReg,
        pos: VoltageNode,
        neg: VoltageNode,
        prepared: &mut PreparedInstruction,
    ) -> JitResult<()> {
        match (pos, neg) {
            (VoltageNode::Ground, VoltageNode::Ground) => self.emit_literal(result, 0.0),
            (pos, VoltageNode::Ground) => self.emit_node_voltage_load(result, pos),
            (VoltageNode::Ground, neg) => {
                self.emit_node_voltage_load(result, neg)?;
                self.encoder.fneg_d(result, result);
                Ok(())
            }
            (pos, neg) => {
                self.emit_node_voltage_load(result, pos)?;
                let right = prepared.temporary()?;
                self.emit_node_voltage_load(right, neg)?;
                self.encoder.fsub_d(result, result, right);
                Ok(())
            }
        }
    }

    fn emit_dynamic_variable_load(
        &mut self,
        prepared: &PreparedInstruction,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        if dynamic_variable_inline_supported(len, lower) {
            self.emit_dynamic_variable_load_inline(prepared, base, len, lower)
        } else {
            self.emit_dynamic_variable_load_helper(prepared, base, len, lower)
        }
    }

    fn emit_dynamic_variable_load_inline(
        &mut self,
        prepared: &PreparedInstruction,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        let raw = unary_operand(prepared)?;
        self.encoder.fmov_d(DReg::D29, raw);
        self.encoder.fabs_d(DReg::D30, DReg::D29);
        self.emit_literal(DReg::D31, 4_503_599_627_370_496.0)?;
        self.encoder.fcmp_d(DReg::D30, DReg::D31);
        let unsafe_numeric = self.encoder.b_cond_placeholder(Condition::CarrySet);

        self.emit_literal(DReg::D31, 0.0)?;
        self.encoder.fcmp_d(DReg::D29, DReg::D31);
        let negative = self.encoder.b_cond_placeholder(Condition::Minus);
        self.emit_literal(DReg::D31, 0.5)?;
        self.encoder.fadd_d(prepared.result, DReg::D29, DReg::D31);
        let rounded = self.encoder.b_placeholder();
        let negative_target = self.encoder.position();
        self.encoder.patch_branch(negative, negative_target)?;
        self.emit_literal(DReg::D31, 0.5)?;
        self.encoder.fsub_d(prepared.result, DReg::D29, DReg::D31);
        let rounded_target = self.encoder.position();
        self.encoder.patch_branch(rounded, rounded_target)?;

        self.encoder.fcvtzs_x_d(XReg::X9, prepared.result)?;
        self.encoder.mov_u64(XReg::X10, lower as u64)?;
        self.encoder.sub_x(XReg::X9, XReg::X9, XReg::X10)?;
        self.encoder.cmp_x_imm(XReg::X9, 0)?;
        let below_range = self.encoder.b_cond_placeholder(Condition::SignedLess);
        self.encoder.mov_u64(XReg::X10, len as u64)?;
        self.encoder.cmp_x(XReg::X9, XReg::X10)?;
        let above_range = self.encoder.b_cond_placeholder(Condition::CarrySet);

        self.encoder.mov_u64(XReg::X10, 3)?;
        self.encoder.lslv_x(XReg::X9, XReg::X9, XReg::X10)?;
        self.emit_variable_base_pointer(XReg::X11, base)?;
        self.encoder.add_x(XReg::X11, XReg::X11, XReg::X9)?;
        self.encoder.ldr_d_unsigned(prepared.result, XReg::X11, 0)?;
        let done = self.encoder.b_placeholder();

        let error_target = self.encoder.position();
        for branch in [unsafe_numeric, below_range, above_range] {
            self.encoder.patch_branch(branch, error_target)?;
        }
        self.encoder.fmov_d(DReg::D0, DReg::D29);
        self.emit_dynamic_variable_error(len, lower)?;

        let done_target = self.encoder.position();
        self.encoder.patch_branch(done, done_target)
    }

    fn emit_dynamic_variable_load_helper(
        &mut self,
        prepared: &PreparedInstruction,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        let raw = unary_operand(prepared)?;
        let call_frame_bytes = STACK_ALIGNMENT;
        self.adjust_stack(call_frame_bytes, false)?;
        self.encoder.str_d_unsigned(raw, XReg::Sp, 0)?;
        if raw != DReg::D0 {
            self.encoder.fmov_d(DReg::D0, raw);
        }
        self.emit_variable_base_pointer(XReg::X0, base)?;
        self.encoder.mov_u64(XReg::X1, len as u64)?;
        self.encoder.mov_u64(XReg::X2, lower as u64)?;
        self.encoder.mov_u64(
            HOST_ABI.indirect_call_scratch,
            rspice_dynamic_variable_slot_native as *const () as usize as u64,
        )?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        let invalid = self.encoder.cbz_placeholder(XReg::X0)?;
        self.encoder.ldr_d_unsigned(prepared.result, XReg::X0, 0)?;
        self.adjust_stack(call_frame_bytes, true)?;
        let done = self.encoder.b_placeholder();

        let invalid_target = self.encoder.position();
        self.encoder.patch_branch(invalid, invalid_target)?;
        self.encoder.ldr_d_unsigned(DReg::D0, XReg::Sp, 0)?;
        self.adjust_stack(call_frame_bytes, true)?;
        self.emit_dynamic_variable_error(len, lower)?;

        let done_target = self.encoder.position();
        self.encoder.patch_branch(done, done_target)
    }

    fn emit_variable_base_pointer(&mut self, destination: XReg, base: usize) -> JitResult<()> {
        let base_bytes = base
            .checked_mul(WORD_BYTES)
            .ok_or_else(|| encoding_error("AArch64 dynamic-variable base offset overflow"))?;
        if base_bytes == 0 {
            self.encoder.mov_x(destination, self.variables_register())?;
        } else if base_bytes <= 4095 {
            self.encoder
                .add_x_imm(destination, self.variables_register(), base_bytes as u16)?;
        } else if base_bytes >> 12 <= 4095 {
            self.encoder.add_x_imm_shift12(
                destination,
                self.variables_register(),
                (base_bytes >> 12) as u16,
            )?;
            let remainder = (base_bytes & 0xfff) as u16;
            if remainder != 0 {
                self.encoder
                    .add_x_imm(destination, destination, remainder)?;
            }
        } else {
            self.encoder.mov_x(destination, self.variables_register())?;
            self.encoder.mov_u64(XReg::X10, base_bytes as u64)?;
            self.encoder.add_x(destination, destination, XReg::X10)?;
        }
        Ok(())
    }

    fn emit_dynamic_variable_error(&mut self, len: usize, lower: i64) -> JitResult<()> {
        self.encoder.mov_x(XReg::X0, self.context_register())?;
        self.encoder.mov_u64(XReg::X1, len as u64)?;
        self.encoder.mov_u64(XReg::X2, lower as u64)?;
        self.encoder.mov_u64(
            HOST_ABI.indirect_call_scratch,
            rspice_native_dynamic_variable_error as *const () as usize as u64,
        )?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        self.early_returns.push(self.encoder.b_placeholder());
        Ok(())
    }

    fn emit_limit_state(
        &mut self,
        prepared: &PreparedInstruction,
        state_index: usize,
    ) -> JitResult<()> {
        let (value, step) = binary_operands(prepared)?;
        self.encoder.fmov_d(prepared.result, value);

        self.encoder
            .ldr_x_unsigned(XReg::X16, self.context_register(), STATE_VALUES_OFFSET)?;
        let no_state = self.encoder.cbz_placeholder(XReg::X16)?;
        self.encoder
            .ldr_x_unsigned(XReg::X17, self.context_register(), STATE_VALUES_LEN_OFFSET)?;
        self.encoder.mov_u64(XReg::X15, state_index as u64)?;
        self.encoder.cmp_x(XReg::X17, XReg::X15)?;
        let state_out_of_range = self
            .encoder
            .b_cond_placeholder(Condition::UnsignedLowerOrSame);

        self.encoder.ldr_x_unsigned(
            XReg::X14,
            self.context_register(),
            STATE_INITIALIZED_OFFSET,
        )?;
        let no_initialized = self.encoder.cbz_placeholder(XReg::X14)?;
        self.encoder.ldr_x_unsigned(
            XReg::X13,
            self.context_register(),
            STATE_INITIALIZED_LEN_OFFSET,
        )?;
        self.encoder.cmp_x(XReg::X13, XReg::X15)?;
        let initialized_out_of_range = self
            .encoder
            .b_cond_placeholder(Condition::UnsignedLowerOrSame);
        self.emit_u8_array_load(XReg::X12, XReg::X14, state_index)?;
        let first_evaluation = self.encoder.cbz_placeholder(XReg::X12)?;

        self.emit_array_load(DReg::D29, XReg::X16, state_index)?;
        self.encoder
            .fsub_d(prepared.result, prepared.result, DReg::D29);
        self.encoder.fcmp_d(prepared.result, prepared.result);
        let unordered_delta = self.encoder.b_cond_placeholder(Condition::NotEqual);
        self.encoder.fneg_d(DReg::D30, step);
        self.encoder
            .fmax_d(prepared.result, prepared.result, DReg::D30);
        self.encoder.fmin_d(prepared.result, prepared.result, step);
        self.encoder
            .fadd_d(prepared.result, prepared.result, DReg::D29);

        let store_target = self.encoder.position();
        self.encoder.patch_branch(first_evaluation, store_target)?;
        self.encoder.patch_branch(unordered_delta, store_target)?;
        self.emit_array_store(prepared.result, XReg::X16, state_index)?;
        self.encoder.mov_u64(XReg::X12, 1)?;
        self.emit_u8_array_store(XReg::X12, XReg::X14, state_index)?;
        let done = self.encoder.b_placeholder();

        let no_initialized_target = self.encoder.position();
        self.encoder
            .patch_branch(no_initialized, no_initialized_target)?;
        self.emit_void_error_early_return(
            rspice_native_limit_state_initialized_error as *const () as usize,
        )?;

        let initialized_bounds_target = self.encoder.position();
        self.encoder
            .patch_branch(initialized_out_of_range, initialized_bounds_target)?;
        self.emit_void_error_early_return(
            rspice_native_limit_state_bounds_error as *const () as usize,
        )?;

        let state_bounds_target = self.encoder.position();
        self.encoder
            .patch_branch(state_out_of_range, state_bounds_target)?;
        self.emit_void_error_early_return(
            rspice_native_limit_state_values_bounds_error as *const () as usize,
        )?;

        let no_state_target = self.encoder.position();
        self.encoder.patch_branch(no_state, no_state_target)?;
        self.emit_void_error_early_return(
            rspice_native_limit_state_values_error as *const () as usize,
        )?;

        let done_target = self.encoder.position();
        self.encoder.patch_branch(done, done_target)
    }

    fn emit_node_voltage_load(&mut self, result: DReg, node: VoltageNode) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => {
                self.emit_context_array_load(result, VOLTAGES_OFFSET, index)
            }
            VoltageNode::Internal(index) => {
                self.emit_context_array_load(result, INTERNAL_VOLTAGES_OFFSET, index)
            }
            VoltageNode::Ground => self.emit_literal(result, 0.0),
        }
    }

    fn emit_guarded_context_f64_load(
        &mut self,
        result: DReg,
        pointer_offset: usize,
        length_offset: usize,
        index: usize,
        error_helper: usize,
    ) -> JitResult<()> {
        let (missing_storage, out_of_range) =
            self.emit_guarded_slice_prelude(pointer_offset, length_offset, index)?;
        self.emit_array_load(result, HOST_ABI.indirect_call_scratch, index)?;
        let done = self.encoder.b_placeholder();

        let error_target = self.encoder.position();
        self.encoder.patch_branch(missing_storage, error_target)?;
        self.encoder.patch_branch(out_of_range, error_target)?;
        self.emit_void_error_early_return(error_helper)?;

        let done_target = self.encoder.position();
        self.encoder.patch_branch(done, done_target)
    }

    fn emit_guarded_context_u8_load(
        &mut self,
        result: DReg,
        pointer_offset: usize,
        length_offset: usize,
        index: usize,
        error_helper: usize,
    ) -> JitResult<()> {
        let (missing_storage, out_of_range) =
            self.emit_guarded_slice_prelude(pointer_offset, length_offset, index)?;
        if index <= 4095 {
            self.encoder
                .ldrb_w_unsigned(XReg::X17, HOST_ABI.indirect_call_scratch, index)?;
        } else {
            self.encoder.mov_u64(XReg::X17, index as u64)?;
            self.encoder
                .add_x(XReg::X17, HOST_ABI.indirect_call_scratch, XReg::X17)?;
            self.encoder.ldrb_w_unsigned(XReg::X17, XReg::X17, 0)?;
        }
        self.encoder.scvtf_d_x(result, XReg::X17)?;
        let done = self.encoder.b_placeholder();

        let error_target = self.encoder.position();
        self.encoder.patch_branch(missing_storage, error_target)?;
        self.encoder.patch_branch(out_of_range, error_target)?;
        self.emit_void_error_early_return(error_helper)?;

        let done_target = self.encoder.position();
        self.encoder.patch_branch(done, done_target)
    }

    fn emit_guarded_slice_prelude(
        &mut self,
        pointer_offset: usize,
        length_offset: usize,
        index: usize,
    ) -> JitResult<(BranchPatch, BranchPatch)> {
        if !self.saves_entry_args {
            return Err(verifier_error(
                "AArch64 guarded load emitted without a nonvolatile call frame",
            ));
        }
        self.encoder.ldr_x_unsigned(
            HOST_ABI.indirect_call_scratch,
            self.context_register(),
            pointer_offset,
        )?;
        let missing_storage = self
            .encoder
            .cbz_placeholder(HOST_ABI.indirect_call_scratch)?;
        self.encoder
            .ldr_x_unsigned(XReg::X17, self.context_register(), length_offset)?;
        self.encoder.mov_u64(XReg::X15, index as u64)?;
        self.encoder.cmp_x(XReg::X17, XReg::X15)?;
        let out_of_range = self
            .encoder
            .b_cond_placeholder(Condition::UnsignedLowerOrSame);
        Ok((missing_storage, out_of_range))
    }

    fn emit_void_error_early_return(&mut self, helper: usize) -> JitResult<()> {
        self.encoder.mov_x(XReg::X0, self.context_register())?;
        self.encoder
            .mov_u64(HOST_ABI.indirect_call_scratch, helper as u64)?;
        self.encoder.blr(HOST_ABI.indirect_call_scratch);
        self.emit_literal(DReg::D0, 0.0)?;
        self.early_returns.push(self.encoder.b_placeholder());
        Ok(())
    }

    fn emit_context_array_load(
        &mut self,
        result: DReg,
        pointer_offset: usize,
        index: usize,
    ) -> JitResult<()> {
        self.encoder.ldr_x_unsigned(
            HOST_ABI.indirect_call_scratch,
            self.context_register(),
            pointer_offset,
        )?;
        self.emit_array_load(result, HOST_ABI.indirect_call_scratch, index)
    }

    fn emit_array_load(&mut self, result: DReg, base: XReg, index: usize) -> JitResult<()> {
        let byte_offset = index
            .checked_mul(WORD_BYTES)
            .ok_or_else(|| encoding_error("AArch64 array index byte offset overflow"))?;
        if base == self.variables_register()
            && let Some((window, offset)) = self.variable_window(byte_offset)?
        {
            return self.encoder.ldr_d_unsigned(result, window, offset);
        }
        if byte_offset / WORD_BYTES <= 4095 {
            self.encoder.ldr_d_unsigned(result, base, byte_offset)
        } else if byte_offset >> 12 <= 4095 {
            self.encoder.add_x_imm_shift12(
                XReg::X17,
                base,
                u16::try_from(byte_offset >> 12)
                    .map_err(|_| encoding_error("AArch64 array page offset overflow"))?,
            )?;
            self.encoder
                .ldr_d_unsigned(result, XReg::X17, byte_offset & 0xfff)
        } else {
            self.encoder.mov_u64(XReg::X17, byte_offset as u64)?;
            self.encoder.add_x(XReg::X17, base, XReg::X17)?;
            self.encoder.ldr_d_unsigned(result, XReg::X17, 0)
        }
    }

    fn emit_context_array_store(
        &mut self,
        source: DReg,
        pointer_offset: usize,
        index: usize,
    ) -> JitResult<()> {
        self.encoder.ldr_x_unsigned(
            HOST_ABI.indirect_call_scratch,
            self.context_register(),
            pointer_offset,
        )?;
        self.emit_array_store(source, HOST_ABI.indirect_call_scratch, index)
    }

    fn emit_array_store(&mut self, source: DReg, base: XReg, index: usize) -> JitResult<()> {
        let byte_offset = index
            .checked_mul(WORD_BYTES)
            .ok_or_else(|| encoding_error("AArch64 array store byte offset overflow"))?;
        if base == self.variables_register()
            && let Some((window, offset)) = self.variable_window(byte_offset)?
        {
            return self.encoder.str_d_unsigned(source, window, offset);
        }
        if byte_offset / WORD_BYTES <= 4095 {
            self.encoder.str_d_unsigned(source, base, byte_offset)
        } else if byte_offset >> 12 <= 4095 {
            self.encoder.add_x_imm_shift12(
                XReg::X11,
                base,
                u16::try_from(byte_offset >> 12)
                    .map_err(|_| encoding_error("AArch64 array page offset overflow"))?,
            )?;
            self.encoder
                .str_d_unsigned(source, XReg::X11, byte_offset & 0xfff)
        } else {
            self.encoder.mov_u64(XReg::X11, byte_offset as u64)?;
            self.encoder.add_x(XReg::X11, base, XReg::X11)?;
            self.encoder.str_d_unsigned(source, XReg::X11, 0)
        }
    }

    fn variable_window(&mut self, byte_offset: usize) -> JitResult<Option<(XReg, usize)>> {
        const WINDOW_BYTES: usize = 4096 * WORD_BYTES;
        if !self.variable_window_enabled {
            return Ok(None);
        }
        let window_base = byte_offset / WINDOW_BYTES * WINDOW_BYTES;
        let offset = byte_offset - window_base;
        if window_base == 0 {
            return Ok(Some((self.variables_register(), offset)));
        }
        let page_immediate = window_base >> 12;
        if page_immediate > 4095 {
            return Ok(None);
        }
        if self.variable_window_base != Some(window_base) {
            self.encoder.add_x_imm_shift12(
                XReg::X14,
                self.variables_register(),
                page_immediate as u16,
            )?;
            self.variable_window_base = Some(window_base);
        }
        Ok(Some((XReg::X14, offset)))
    }

    fn emit_u8_array_load(&mut self, destination: XReg, base: XReg, index: usize) -> JitResult<()> {
        if index <= 4095 {
            self.encoder.ldrb_w_unsigned(destination, base, index)
        } else {
            self.encoder.mov_u64(XReg::X11, index as u64)?;
            self.encoder.add_x(XReg::X11, base, XReg::X11)?;
            self.encoder.ldrb_w_unsigned(destination, XReg::X11, 0)
        }
    }

    fn emit_u8_array_store(&mut self, source: XReg, base: XReg, index: usize) -> JitResult<()> {
        if index <= 4095 {
            self.encoder.strb_w_unsigned(source, base, index)
        } else {
            self.encoder.mov_u64(XReg::X11, index as u64)?;
            self.encoder.add_x(XReg::X11, base, XReg::X11)?;
            self.encoder.strb_w_unsigned(source, XReg::X11, 0)
        }
    }

    fn emit_context_f64_load(&mut self, result: DReg, offset: usize) -> JitResult<()> {
        self.encoder
            .ldr_d_unsigned(result, self.context_register(), offset)
    }

    fn emit_analysis(&mut self, result: DReg, analysis_id: u8) -> JitResult<()> {
        if matches!(analysis_id, 7 | 8) {
            let offset = if analysis_id == 7 {
                ANALYSIS_INITIAL_STEP_OFFSET
            } else {
                ANALYSIS_FINAL_STEP_OFFSET
            };
            self.encoder
                .ldrb_w_unsigned(XReg::X16, self.context_register(), offset)?;
            return self.encoder.scvtf_d_x(result, XReg::X16);
        }
        if analysis_id > 8 {
            return self.emit_literal(result, 0.0);
        }

        self.encoder
            .ldrb_w_unsigned(XReg::X15, self.context_register(), ANALYSIS_TYPE_OFFSET)?;
        self.emit_literal(DReg::D30, 0.0)?;
        self.emit_literal(DReg::D31, 1.0)?;
        match analysis_id {
            0..=4 => {
                self.encoder.cmp_x_imm(XReg::X15, u16::from(analysis_id))?;
                self.encoder
                    .fcsel_d(result, DReg::D31, DReg::D30, Condition::Equal);
            }
            5 | 6 => {
                let (first, second) = if analysis_id == 5 { (0, 4) } else { (1, 3) };
                self.encoder.cmp_x_imm(XReg::X15, first)?;
                let first_matches = self.encoder.b_cond_placeholder(Condition::Equal);
                self.encoder.cmp_x_imm(XReg::X15, second)?;
                self.encoder
                    .fcsel_d(result, DReg::D31, DReg::D30, Condition::Equal);
                let done = self.encoder.b_placeholder();
                let first_target = self.encoder.position();
                self.encoder.patch_branch(first_matches, first_target)?;
                self.encoder.fmov_d(result, DReg::D31);
                let done_target = self.encoder.position();
                self.encoder.patch_branch(done, done_target)?;
            }
            7 | 8 => unreachable!("step flags handled above"),
            _ => unreachable!("analysis IDs above eight handled above"),
        }
        Ok(())
    }

    fn emit_literal(&mut self, result: DReg, value: f64) -> JitResult<()> {
        if value.to_bits() == 0 {
            self.encoder.fmov_d_positive_zero(result);
            return Ok(());
        }
        if self.encoder.fmov_d_imm(result, value) {
            return Ok(());
        }
        let patch = self.encoder.ldr_d_literal_placeholder(result);
        self.literal_window_start
            .get_or_insert(patch.instruction_offset());
        self.literals.push((patch, value.to_bits()));
        Ok(())
    }

    /// Place the pending constants inline when the function has grown too long
    /// for them to stay in the trailing pool.
    ///
    /// Call sites are instruction boundaries: every live value is in its
    /// allocated register or its spill slot there, and an island clobbers
    /// neither, so the flush is invisible to the program around it.
    fn flush_literal_island_if_needed(&mut self) -> JitResult<()> {
        let Some(window_start) = self.literal_window_start else {
            return Ok(());
        };
        let words = self.distinct_pending_literals();
        let projected_end = align_up_8(self.encoder.position() + 8) + words * 8;
        let reach = window_start.saturating_add(MAX_FORWARD_LITERAL_REACH_BYTES);
        if words < MAX_LITERAL_ISLAND_WORDS
            && projected_end.saturating_add(LITERAL_ISLAND_MARGIN_BYTES) <= reach
        {
            return Ok(());
        }
        self.flush_literal_island()
    }

    fn distinct_pending_literals(&self) -> usize {
        let mut seen = std::collections::HashSet::<u64>::with_capacity(self.literals.len());
        self.literals
            .iter()
            .filter(|(_, bits)| seen.insert(*bits))
            .count()
    }

    /// Emit `B over; BRK words; <constants>` and resolve every pending patch
    /// into the island.
    fn flush_literal_island(&mut self) -> JitResult<()> {
        if self.literals.is_empty() {
            return Ok(());
        }
        let words = self.distinct_pending_literals();
        let words = u16::try_from(words).map_err(|_| {
            encoding_error("AArch64 literal island holds more constants than its marker can name")
        })?;
        if self.encoder.position() % 8 != 0 {
            self.encoder.nop();
        }
        let over = self.encoder.b_placeholder();
        self.encoder.literal_island_marker(words);
        self.emit_pending_literal_pool()?;
        let resume = self.encoder.position();
        self.encoder.patch_branch(over, resume)?;
        self.literal_window_start = None;
        Ok(())
    }

    /// Append the pending constants at the current position and patch every
    /// load that named one. The caller owns alignment and reachability.
    fn emit_pending_literal_pool(&mut self) -> JitResult<()> {
        let mut offsets = std::collections::HashMap::<u64, usize>::new();
        for &(_, bits) in &self.literals {
            if offsets.contains_key(&bits) {
                continue;
            }
            let offset = self.encoder.position();
            if offset % 8 != 0 {
                return Err(encoding_error(
                    "AArch64 literal pool is not naturally aligned",
                ));
            }
            offsets.insert(bits, offset);
            self.encoder.append_u64_data(bits);
        }
        for (patch, bits) in std::mem::take(&mut self.literals) {
            let target = offsets
                .get(&bits)
                .copied()
                .ok_or_else(|| encoding_error("AArch64 literal patch has no pooled constant"))?;
            self.encoder.patch_ldr_d_literal(patch, target)?;
        }
        Ok(())
    }

    fn finish(mut self, result: ValueLocation) -> JitResult<Vec<u8>> {
        match result {
            ValueLocation::Register(index) => {
                let source = logical_register(index)?;
                if source != HOST_ABI.result {
                    self.encoder.fmov_d(HOST_ABI.result, source);
                }
            }
            ValueLocation::Spill(slot) => {
                self.encoder
                    .ldr_d_unsigned(HOST_ABI.result, XReg::Sp, spill_slot_offset(slot)?)?
            }
        }
        self.emit_epilogue()?;
        self.finish_bytes()
    }

    #[cfg(test)]
    fn finish_assignment(
        mut self,
        result: ValueLocation,
        variable_index: usize,
    ) -> JitResult<Vec<u8>> {
        let source = match result {
            ValueLocation::Register(index) => logical_register(index)?,
            ValueLocation::Spill(slot) => {
                self.encoder
                    .ldr_d_unsigned(DReg::D0, XReg::Sp, spill_slot_offset(slot)?)?;
                DReg::D0
            }
        };
        self.emit_array_store(source, self.variables_register(), variable_index)?;
        self.emit_epilogue()?;
        self.finish_bytes()
    }

    fn finish_assignment_pass(mut self) -> JitResult<Vec<u8>> {
        self.emit_epilogue()?;
        self.finish_bytes()
    }

    fn finish_bytes(mut self) -> JitResult<Vec<u8>> {
        if !self.literals.is_empty() && self.encoder.position() % 8 != 0 {
            self.encoder.nop();
        }
        self.emit_pending_literal_pool()?;
        Ok(self.encoder.into_bytes())
    }

    fn emit_epilogue(&mut self) -> JitResult<()> {
        let epilogue = self.encoder.position();
        for branch in std::mem::take(&mut self.early_returns) {
            self.encoder.patch_branch(branch, epilogue)?;
        }
        self.adjust_stack(self.frame_bytes, true)?;
        if self.saves_entry_args {
            self.encoder.ldp_x_post(
                HOST_ABI.frame_pointer,
                HOST_ABI.link_register,
                XReg::Sp,
                16,
            )?;
            if self.saves_kernel_io {
                self.encoder
                    .ldp_x_post(HOST_ABI.saved_kernel_io, XReg::X22, XReg::Sp, 16)?;
            }
            self.encoder.ldp_x_post(
                HOST_ABI.saved_context,
                HOST_ABI.saved_variables,
                XReg::Sp,
                16,
            )?;
        }
        self.encoder.ret();
        Ok(())
    }

    fn stack_store_d(&mut self, source: DReg, offset: usize) -> JitResult<()> {
        if offset % WORD_BYTES == 0 && offset / WORD_BYTES <= 4095 {
            self.encoder.str_d_unsigned(source, XReg::Sp, offset)
        } else {
            self.emit_stack_address(XReg::X11, offset)?;
            self.encoder.str_d_unsigned(source, XReg::X11, 0)
        }
    }

    fn stack_load_d(&mut self, destination: DReg, offset: usize) -> JitResult<()> {
        if offset % WORD_BYTES == 0 && offset / WORD_BYTES <= 4095 {
            self.encoder.ldr_d_unsigned(destination, XReg::Sp, offset)
        } else {
            self.emit_stack_address(XReg::X11, offset)?;
            self.encoder.ldr_d_unsigned(destination, XReg::X11, 0)
        }
    }

    fn stack_store_x(&mut self, source: XReg, offset: usize) -> JitResult<()> {
        if offset % WORD_BYTES == 0 && offset / WORD_BYTES <= 4095 {
            self.encoder.str_x_unsigned(source, XReg::Sp, offset)
        } else {
            self.emit_stack_address(XReg::X11, offset)?;
            self.encoder.str_x_unsigned(source, XReg::X11, 0)
        }
    }

    fn stack_load_x(&mut self, destination: XReg, offset: usize) -> JitResult<()> {
        if offset % WORD_BYTES == 0 && offset / WORD_BYTES <= 4095 {
            self.encoder.ldr_x_unsigned(destination, XReg::Sp, offset)
        } else {
            self.emit_stack_address(XReg::X11, offset)?;
            self.encoder.ldr_x_unsigned(destination, XReg::X11, 0)
        }
    }

    fn emit_stack_address(&mut self, destination: XReg, offset: usize) -> JitResult<()> {
        self.encoder.add_x_imm(destination, XReg::Sp, 0)?;
        if offset != 0 {
            self.encoder.mov_u64(XReg::X10, offset as u64)?;
            self.encoder.add_x(destination, destination, XReg::X10)?;
        }
        Ok(())
    }

    fn adjust_stack(&mut self, bytes: usize, restore: bool) -> JitResult<()> {
        let mut remaining = bytes;
        while remaining != 0 {
            let chunk = remaining.min(STACK_PROBE_INTERVAL_BYTES);
            let chunk = u16::try_from(chunk)
                .map_err(|_| encoding_error("AArch64 stack adjustment exceeds imm12"))?;
            if restore {
                self.encoder.add_x_imm(XReg::Sp, XReg::Sp, chunk)?;
            } else {
                self.encoder.sub_x_imm(XReg::Sp, XReg::Sp, chunk)?;
                // Touch every newly crossed page-sized interval. X16 is
                // caller-saved and is not live before expression emission.
                self.encoder
                    .str_x_unsigned(HOST_ABI.indirect_call_scratch, XReg::Sp, 0)?;
            }
            remaining -= usize::from(chunk);
        }
        Ok(())
    }

    fn context_register(&self) -> XReg {
        if self.saves_entry_args {
            HOST_ABI.saved_context
        } else {
            HOST_ABI.entry_context
        }
    }

    fn variables_register(&self) -> XReg {
        if self.saves_entry_args {
            HOST_ABI.saved_variables
        } else {
            HOST_ABI.entry_variables
        }
    }

    fn kernel_io_register(&self) -> JitResult<XReg> {
        if !self.saves_kernel_io {
            return Err(register_allocation_error(
                "AArch64 kernel I/O requested outside a fused kernel",
            ));
        }
        Ok(HOST_ABI.saved_kernel_io)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct AssignmentRequirements {
    maximum_spill_slots: usize,
    has_indexed_assignment: bool,
    loop_depth: usize,
    requires_call_frame: bool,
    trace_instructions: usize,
    trace_outputs: usize,
    trace_spill_operands: usize,
    trace_spill_results: usize,
}

fn assignment_requirements(assignments: &[NativeAssignment]) -> JitResult<AssignmentRequirements> {
    let mut requirements = AssignmentRequirements::default();
    inspect_assignment_requirements(assignments, 0, &mut requirements)?;
    Ok(requirements)
}

fn inspect_assignment_requirements(
    assignments: &[NativeAssignment],
    loop_depth: usize,
    requirements: &mut AssignmentRequirements,
) -> JitResult<()> {
    for range in shareable_batch_ranges(assignments) {
        let batch = &assignments[range];
        match &batch[0] {
            NativeAssignment::Direct { .. } => {
                let direct = batch
                    .iter()
                    .map(|assignment| match assignment {
                        NativeAssignment::Direct { var_index, program } => {
                            Ok((*var_index, program))
                        }
                        NativeAssignment::Indexed { .. } | NativeAssignment::Loop { .. } => {
                            Err(verifier_error(
                                "AArch64 requirement batch contains a control-flow assignment",
                            ))
                        }
                    })
                    .collect::<JitResult<Vec<_>>>()?;
                let ssa = AssignmentProgram::lower(&direct)?;
                validate_expression_stack_depth(ssa.program().maximum_stack_depth())?;
                let allocation = RegisterAllocation::build_for_assignments(&ssa, A64_VALUE_BANK)?;
                record_assignment_allocation_trace(
                    ssa.program(),
                    &allocation,
                    direct.len(),
                    requirements,
                );
                requirements.maximum_spill_slots = requirements
                    .maximum_spill_slots
                    .max(allocation.spill_slot_count());
                requirements.requires_call_frame |= ssa.program().requires_call_frame();
            }
            NativeAssignment::Indexed { index, value, .. } => {
                debug_assert_eq!(batch.len(), 1);
                requirements.has_indexed_assignment = true;
                requirements.requires_call_frame = true;
                inspect_assignment_program(index, requirements)?;
                inspect_assignment_program(value, requirements)?;
            }
            NativeAssignment::Loop { condition, body } => {
                debug_assert_eq!(batch.len(), 1);
                let nested_depth = loop_depth.checked_add(1).ok_or_else(|| {
                    register_allocation_error("AArch64 assignment-loop nesting overflow")
                })?;
                requirements.loop_depth = requirements.loop_depth.max(nested_depth);
                requirements.requires_call_frame = true;
                inspect_assignment_program(condition, requirements)?;
                inspect_assignment_requirements(body, nested_depth, requirements)?;
            }
        }
    }
    Ok(())
}

fn inspect_assignment_program(
    program: &NativeProgram,
    requirements: &mut AssignmentRequirements,
) -> JitResult<()> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    let ssa = Program::lower(program)?;
    let allocation = RegisterAllocation::build(&ssa, A64_VALUE_BANK)?;
    record_assignment_allocation_trace(&ssa, &allocation, 1, requirements);
    requirements.maximum_spill_slots = requirements
        .maximum_spill_slots
        .max(allocation.spill_slot_count());
    requirements.requires_call_frame |= ssa.requires_call_frame();
    Ok(())
}

fn record_assignment_allocation_trace(
    program: &Program,
    allocation: &RegisterAllocation,
    outputs: usize,
    requirements: &mut AssignmentRequirements,
) {
    requirements.trace_instructions = requirements
        .trace_instructions
        .saturating_add(program.instructions().len());
    requirements.trace_outputs = requirements.trace_outputs.saturating_add(outputs);
    for instruction in allocation.instructions() {
        requirements.trace_spill_operands = requirements.trace_spill_operands.saturating_add(
            instruction
                .operands()
                .iter()
                .filter(|location| matches!(location, ValueLocation::Spill(_)))
                .count(),
        );
        requirements.trace_spill_results =
            requirements
                .trace_spill_results
                .saturating_add(usize::from(matches!(
                    instruction.result(),
                    ValueLocation::Spill(_)
                )));
    }
}

#[derive(Debug, Clone, Copy)]
struct AssignmentFrameLayout {
    frame_bytes: usize,
    indexed_slot: Option<usize>,
    loop_counter_base: Option<usize>,
    loop_depth: usize,
}

impl AssignmentFrameLayout {
    fn new(requirements: AssignmentRequirements) -> JitResult<Self> {
        let spill_bytes = requirements
            .maximum_spill_slots
            .checked_mul(WORD_BYTES)
            .ok_or_else(|| register_allocation_error("AArch64 assignment spill-frame overflow"))?;
        let indexed_slot = requirements.has_indexed_assignment.then_some(spill_bytes);
        let after_indexed = spill_bytes
            .checked_add(usize::from(requirements.has_indexed_assignment) * WORD_BYTES)
            .ok_or_else(|| {
                register_allocation_error("AArch64 indexed-assignment frame overflow")
            })?;
        let loop_counter_base = (requirements.loop_depth != 0).then_some(after_indexed);
        let unaligned = requirements
            .loop_depth
            .checked_mul(WORD_BYTES)
            .and_then(|bytes| after_indexed.checked_add(bytes))
            .ok_or_else(|| register_allocation_error("AArch64 loop-counter frame overflow"))?;
        let frame_bytes = if unaligned == 0 {
            0
        } else {
            unaligned
                .checked_add(STACK_ALIGNMENT - 1)
                .map(|bytes| bytes / STACK_ALIGNMENT * STACK_ALIGNMENT)
                .ok_or_else(|| register_allocation_error("AArch64 assignment frame overflow"))?
        };
        Ok(Self {
            frame_bytes,
            indexed_slot,
            loop_counter_base,
            loop_depth: requirements.loop_depth,
        })
    }

    fn loop_counter_offset(self, depth: usize) -> JitResult<usize> {
        if depth >= self.loop_depth {
            return Err(JitError::InternalCompilerError {
                model: MODEL.into(),
                detail: format!(
                    "AArch64 loop depth {depth} exceeds {} reserved counter slots",
                    self.loop_depth
                )
                .into(),
            });
        }
        self.loop_counter_base
            .and_then(|base| {
                depth
                    .checked_mul(WORD_BYTES)
                    .and_then(|offset| base.checked_add(offset))
            })
            .ok_or_else(|| JitError::InternalCompilerError {
                model: MODEL.into(),
                detail: "AArch64 loop-counter offset overflow".into(),
            })
    }
}

struct PreparedInstruction {
    operands: Vec<DReg>,
    result: DReg,
    used_mask: u32,
}

impl PreparedInstruction {
    fn temporary(&mut self) -> JitResult<DReg> {
        take_temporary(&mut self.used_mask)
    }
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

type UnaryHelper = extern "C" fn(f64) -> f64;
type BinaryHelper = extern "C" fn(f64, f64) -> f64;

fn unary_math_helper(op: UnaryMathOp) -> UnaryHelper {
    match op {
        UnaryMathOp::Exp => rspice_exp,
        UnaryMathOp::Log => rspice_log,
        UnaryMathOp::Log10 => rspice_log10,
        UnaryMathOp::Sin => rspice_sin,
        UnaryMathOp::Cos => rspice_cos,
        UnaryMathOp::Tan => rspice_tan,
        UnaryMathOp::Sinh => rspice_sinh,
        UnaryMathOp::Cosh => rspice_cosh,
        UnaryMathOp::Tanh => rspice_tanh,
        UnaryMathOp::Asinh => rspice_asinh,
        UnaryMathOp::Acosh => rspice_acosh,
        UnaryMathOp::Atanh => rspice_atanh,
        UnaryMathOp::Limexp => rspice_limexp,
        UnaryMathOp::LimitedExp => rspice_limited_exp,
        UnaryMathOp::Asin => rspice_asin,
        UnaryMathOp::Acos => rspice_acos,
        UnaryMathOp::Atan => rspice_atan,
        UnaryMathOp::Floor | UnaryMathOp::Ceil => {
            unreachable!("floor and ceil are emitted as A64 instructions")
        }
    }
}

fn binary_math_helper(op: BinaryMathOp) -> BinaryHelper {
    match op {
        BinaryMathOp::Pow => rspice_pow,
        BinaryMathOp::Atan2 => rspice_atan2,
        BinaryMathOp::Hypot => rspice_hypot,
        BinaryMathOp::Mod => rspice_mod,
    }
}

fn unary_operand(prepared: &PreparedInstruction) -> JitResult<DReg> {
    let [source] = prepared.operands.as_slice() else {
        return Err(encoding_error(format!(
            "AArch64 unary operation received {} SSA operands",
            prepared.operands.len()
        )));
    };
    Ok(*source)
}

fn binary_operands(prepared: &PreparedInstruction) -> JitResult<(DReg, DReg)> {
    let [left, right] = prepared.operands.as_slice() else {
        return Err(encoding_error(format!(
            "AArch64 binary operation received {} SSA operands",
            prepared.operands.len()
        )));
    };
    Ok((*left, *right))
}

fn compare_condition(op: CompareOp) -> Condition {
    match op {
        CompareOp::Gt => Condition::SignedGreater,
        CompareOp::Lt => Condition::Minus,
        CompareOp::Ge => Condition::SignedGreaterOrEqual,
        CompareOp::Le => Condition::UnsignedLowerOrSame,
        CompareOp::Eq => Condition::Equal,
        // ARM's NE condition is set for unordered comparisons, matching the
        // Verilog-A/x64 contract that NaN is unequal to every value.
        CompareOp::Ne => Condition::NotEqual,
    }
}

fn logical_register(index: usize) -> JitResult<DReg> {
    if (A64_ALLOCATABLE_VALUE_REGISTERS..LOGICAL_VALUE_REGISTER_COUNT).contains(&index) {
        // Temporary logical registers are legal here, but allocator-owned
        // values must remain in the shared prefix. The caller-specific checks
        // distinguish those cases before reaching this physical map.
    }
    HOST_ABI
        .logical_f64_registers
        .get(index)
        .copied()
        .ok_or_else(|| {
            register_allocation_error(format!("logical AArch64 register {index} is out of range"))
        })
}

fn register_mask(register: DReg) -> JitResult<u32> {
    let logical = HOST_ABI
        .logical_f64_registers
        .iter()
        .position(|candidate| *candidate == register)
        .ok_or_else(|| {
            register_allocation_error(format!(
                "AArch64 register {register:?} is outside the logical value bank"
            ))
        })?;
    Ok(1_u32 << logical)
}

fn take_temporary(used_mask: &mut u32) -> JitResult<DReg> {
    let index = (0..LOGICAL_VALUE_REGISTER_COUNT)
        .find(|index| *used_mask & (1_u32 << index) == 0)
        .ok_or_else(|| {
            register_allocation_error(
                "AArch64 instruction has no volatile SIMD temporary available",
            )
        })?;
    *used_mask |= 1_u32 << index;
    logical_register(index)
}

fn spill_frame_bytes(allocation: &RegisterAllocation) -> JitResult<usize> {
    aligned_frame_bytes(allocation.spill_slot_count())
}

fn aligned_frame_bytes(spill_slot_count: usize) -> JitResult<usize> {
    let bytes = spill_slot_count
        .checked_mul(WORD_BYTES)
        .ok_or_else(|| register_allocation_error("AArch64 spill-frame byte count overflow"))?;
    if bytes == 0 {
        return Ok(0);
    }
    bytes
        .checked_add(STACK_ALIGNMENT - 1)
        .map(|rounded| rounded / STACK_ALIGNMENT * STACK_ALIGNMENT)
        .ok_or_else(|| register_allocation_error("AArch64 aligned spill-frame size overflow"))
}

fn spill_slot_offset(slot: usize) -> JitResult<usize> {
    let offset = slot
        .checked_mul(WORD_BYTES)
        .ok_or_else(|| register_allocation_error("AArch64 spill-slot byte offset overflow"))?;
    if offset / WORD_BYTES > 4095 {
        return Err(register_allocation_error(format!(
            "AArch64 spill slot {slot} exceeds the scaled imm12 frame range"
        )));
    }
    Ok(offset)
}

fn value_slot_offset(slot: usize) -> JitResult<usize> {
    slot.checked_mul(WORD_BYTES)
        .ok_or_else(|| register_allocation_error("AArch64 segmented value-slot offset overflow"))
}

fn validate_expression_stack_depth(max_stack_depth: usize) -> JitResult<()> {
    if max_stack_depth > MAX_EXPRESSION_STACK_DEPTH {
        return Err(register_allocation_error(format!(
            "expression stack depth {max_stack_depth} exceeds the {MAX_EXPRESSION_STACK_DEPTH}-value safety limit"
        )));
    }
    Ok(())
}

fn encoding_error(detail: impl Into<String>) -> JitError {
    JitError::Encoding {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

fn verifier_error(detail: impl Into<String>) -> JitError {
    JitError::Verifier {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

fn register_allocation_error(detail: impl Into<String>) -> JitError {
    JitError::RegisterAllocation {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod cross_target_contract_tests {
    use super::compile_value_function;
    use crate::native::aarch64::calling_convention::HOST_ABI;
    use crate::native::aarch64::encoder::{
        A64Encoder, DReg, MAX_FORWARD_LITERAL_REACH_BYTES, XReg,
    };
    use crate::native::aarch64::image::A64_SEGMENT_THRESHOLD_BYTES;
    use crate::native::aarch64::verifier::verify_exact_function;
    use crate::native::expr::{BinaryMathOp, NativeOp, NativeProgram};

    fn instruction_occurrences(bytes: &[u8], instruction: &[u8]) -> usize {
        bytes
            .chunks_exact(std::mem::size_of::<u32>())
            .filter(|candidate| *candidate == instruction)
            .count()
    }

    fn word_at(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("four-byte A64 word"),
        )
    }

    /// Walk the instruction stream the way the verifier does, reporting every
    /// inline constant island as `(marker offset, constant count)`.
    fn constant_islands(bytes: &[u8]) -> Vec<(usize, usize)> {
        let mut islands = Vec::new();
        let mut offset = 0;
        while offset < bytes.len() {
            let instruction = word_at(bytes, offset);
            if instruction & 0xFFE0_001F == 0xD420_0000 {
                let words = ((instruction >> 5) & 0xFFFF) as usize;
                islands.push((offset, words));
                offset += 4 + words * 8;
                continue;
            }
            if instruction & 0xFFFF_FC1F == 0xD65F_0000 {
                break;
            }
            offset += 4;
        }
        islands
    }

    /// One function whose constants all reach a trailing pool, and one long
    /// enough that they cannot.
    ///
    /// `LDR (literal)` carries a signed 19-bit word displacement, so a
    /// constant more than [`MAX_FORWARD_LITERAL_REACH_BYTES`] past the load
    /// that names it cannot be encoded at all. Before inline islands this was
    /// the reason an A64 function could not grow past a megabyte; the
    /// megabyte-sized refusal `A64_SEGMENT_THRESHOLD_BYTES` then named in the
    /// image builder hid it by never letting the encoder get there.
    #[test]
    #[ignore = "emits two megabytes of real code; run with --release --features native -- --ignored"]
    fn a_function_past_the_literal_reach_places_its_constants_inline() {
        // Roughly forty bytes of code per pair — one `LDR` literal and one
        // helper call — so this clears two megabytes and needs two islands.
        const PAIRS: usize = 60_000;
        let mut ops = Vec::with_capacity(PAIRS * 2 + 1);
        ops.push(NativeOp::Const(1.5));
        for index in 0..PAIRS {
            // Distinct, and none of them encodable as an `FMOV` immediate, so
            // every one takes a pooled constant of its own.
            ops.push(NativeOp::Const(2.0 + (index as f64) * 1.0e-6));
            ops.push(NativeOp::BinaryMath(BinaryMathOp::Hypot));
        }
        let program = NativeProgram::from_ops_for_test(ops, 2, Vec::new(), Vec::new());
        let bytes = compile_value_function(&program)
            .expect("a function past the literal reach must still encode");
        verify_exact_function(&bytes, "oversized value function")
            .expect("inline constant islands must verify");

        assert!(
            bytes.len() > A64_SEGMENT_THRESHOLD_BYTES,
            "the case is only interesting past the old ceiling, got {} bytes",
            bytes.len()
        );
        let islands = constant_islands(&bytes);
        assert!(
            islands.len() >= 2,
            "a function of {} bytes needs more than one island, got {islands:?}",
            bytes.len()
        );
        for (marker, words) in islands {
            let end = marker + 4 + words * 8;
            // The exact relaxed shape: an unconditional `B` over the data,
            // then the marker naming its length.
            let branch = word_at(&bytes, marker - 4);
            let displacement = ((branch & 0x03FF_FFFF) as i32) << 6 >> 6;
            assert_eq!(branch & 0xFC00_0000, 0x1400_0000, "island {marker} branch");
            assert_eq!(
                (marker - 4) as i64 + i64::from(displacement) * 4,
                end as i64,
                "island {marker} is not branched over"
            );
            assert_eq!(
                word_at(&bytes, marker),
                0xD420_0000 | ((words as u32) << 5),
                "island {marker} marker"
            );
            assert_eq!(marker % 8, 4, "island {marker} data is not eight-aligned");
        }
    }

    /// The relaxation is invisible to anything that already fit: no function
    /// under the reach gains an island, so its bytes cannot move.
    #[test]
    fn a_function_within_the_literal_reach_keeps_one_trailing_pool() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::Const(1.5),
                NativeOp::Const(2.5),
                NativeOp::BinaryMath(BinaryMathOp::Hypot),
            ],
            2,
            Vec::new(),
            Vec::new(),
        );
        let bytes = compile_value_function(&program).expect("encode a small value function");
        verify_exact_function(&bytes, "small value function").expect("verify");
        assert!(bytes.len() < MAX_FORWARD_LITERAL_REACH_BYTES);
        assert_eq!(constant_islands(&bytes), Vec::new());
    }

    /// Build `BTI; LDR d0, island; B over; BRK words; <constants>; RET`.
    fn hand_built_island(marker_words: u16, branch_target_delta: i64, load: bool) -> Vec<u8> {
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        let literal = load.then(|| encoder.ldr_d_literal_placeholder(DReg::D0));
        if !load {
            encoder.nop();
        }
        let over = encoder.b_placeholder();
        encoder.literal_island_marker(marker_words);
        let start = encoder.position();
        encoder.append_u64_data(2.0_f64.to_bits());
        let end = encoder.position();
        encoder
            .patch_branch(over, (end as i64 + branch_target_delta) as usize)
            .expect("patch the branch over the island");
        if let Some(literal) = literal {
            encoder
                .patch_ldr_d_literal(literal, start)
                .expect("patch the inline literal load");
        }
        encoder.ret();
        encoder.into_bytes()
    }

    /// The verifier authenticates an island rather than trusting it: the data
    /// has to be jumped over, its declared length has to match the branch, and
    /// every word of it has to be named by a load.
    #[test]
    fn the_verifier_authenticates_inline_constant_islands() {
        verify_exact_function(&hand_built_island(1, 0, true), "island")
            .expect("a well-formed island verifies");

        let unbranched = hand_built_island(1, -8, true);
        let error = verify_exact_function(&unbranched, "island")
            .expect_err("an island the code does not jump over must be refused");
        assert!(
            format!("{error}").contains("does not branch over"),
            "unexpected refusal: {error}"
        );

        let overdeclared = hand_built_island(2, 0, true);
        assert!(
            verify_exact_function(&overdeclared, "island").is_err(),
            "an island longer than the branch skips must be refused"
        );

        let unread = hand_built_island(1, 0, false);
        let error = verify_exact_function(&unread, "island")
            .expect_err("inline constants nothing loads must be refused");
        assert!(
            format!("{error}").contains("partly unread"),
            "unexpected refusal: {error}"
        );
    }

    #[test]
    fn slew_derivative_encodes_one_signed_six_operand_helper_call() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::Const(10.0),
                NativeOp::Const(0.0),
                NativeOp::Const(2.0),
                NativeOp::Const(-0.25),
                NativeOp::Const(-2.0),
                NativeOp::Const(0.5),
                NativeOp::SlewStateDerivative(0),
            ],
            6,
            Vec::new(),
            Vec::new(),
        );
        let bytes = compile_value_function(&program)
            .expect("encode AArch64 signed slew derivative helper call");
        verify_exact_function(&bytes, "signed slew derivative helper call")
            .expect("verify AArch64 signed slew derivative helper call");

        // Six f64 operands require one 48-byte, already 16-byte-aligned call
        // frame. Seeing exactly one allocate/call/restore sequence makes the
        // operand-count ABI executable on every host that can run the checked
        // AArch64 encoder, without requiring an AArch64 emulator.
        let mut signatures = A64Encoder::new();
        signatures
            .sub_x_imm(XReg::Sp, XReg::Sp, 48)
            .expect("encode expected call-frame allocation");
        signatures.blr(HOST_ABI.indirect_call_scratch);
        signatures
            .add_x_imm(XReg::Sp, XReg::Sp, 48)
            .expect("encode expected call-frame restoration");
        let signatures = signatures.into_bytes();
        for (label, instruction) in [
            ("48-byte allocation", &signatures[0..4]),
            ("indirect helper call", &signatures[4..8]),
            ("48-byte restoration", &signatures[8..12]),
        ] {
            assert_eq!(
                instruction_occurrences(&bytes, instruction),
                1,
                "AArch64 slew derivative must contain exactly one {label}"
            );
        }
    }

    #[test]
    fn transition_derivative_encodes_one_five_operand_helper_call() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::Const(1.5),
                NativeOp::Const(-2.0),
                NativeOp::Const(0.25),
                NativeOp::Const(0.5),
                NativeOp::Const(0.75),
                NativeOp::TransitionStateDerivative(0),
            ],
            5,
            Vec::new(),
            Vec::new(),
        );
        let bytes = compile_value_function(&program)
            .expect("encode AArch64 transition derivative helper call");
        verify_exact_function(&bytes, "transition derivative helper call")
            .expect("verify AArch64 transition derivative helper call");

        // Five f64 operands occupy 40 bytes and round to one 48-byte aligned
        // call frame. Pin the single allocation/call/restoration sequence so
        // the generated AAPCS64 ABI cannot silently drop an operand.
        let mut signatures = A64Encoder::new();
        signatures
            .sub_x_imm(XReg::Sp, XReg::Sp, 48)
            .expect("encode expected call-frame allocation");
        signatures.blr(HOST_ABI.indirect_call_scratch);
        signatures
            .add_x_imm(XReg::Sp, XReg::Sp, 48)
            .expect("encode expected call-frame restoration");
        let signatures = signatures.into_bytes();
        for (label, instruction) in [
            ("48-byte allocation", &signatures[0..4]),
            ("indirect helper call", &signatures[4..8]),
            ("48-byte restoration", &signatures[8..12]),
        ] {
            assert_eq!(
                instruction_occurrences(&bytes, instruction),
                1,
                "AArch64 transition derivative must contain exactly one {label}"
            );
        }
    }

    /// The branch form of a conditional encodes real A64 control flow.
    ///
    /// Nothing here executes A64: the checked encoder plus the independent
    /// decoder in [`verify_exact_function`] is the contract every non-AArch64
    /// host can hold the backend to, and it is what rejects a branch landing
    /// anywhere but an instruction boundary.
    #[test]
    fn branch_lowered_conditionals_encode_verified_a64_control_flow() {
        let source = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::LoadParam(2),
                NativeOp::Mul,
                NativeOp::LoadParam(3),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
                NativeOp::IfElse,
                NativeOp::LoadParam(4),
                NativeOp::Add,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );
        let select = compile_value_function(&source).expect("encode the select form");
        let ssa = crate::native::ssa::Program::lower(&source)
            .expect("lower the postfix program")
            .with_branching_conditionals()
            .expect("re-express conditionals as branches");
        assert_eq!(ssa.blocks().len(), 4, "branch, then, else, join");
        let branch = super::compile_value_function_from_ssa(&ssa)
            .expect("encode and verify the branch form");

        let words = |bytes: &[u8], mask: u32, expected: u32| {
            bytes
                .chunks_exact(4)
                .filter(|word| {
                    let word = u32::from_le_bytes((*word).try_into().expect("aligned word"));
                    word & mask == expected
                })
                .count()
        };
        // FCSEL is how the select form picks an arm.
        assert_eq!(
            words(&select, 0xFFE0_0C00, 0x1E60_0C00),
            1,
            "the select form folds the conditional into one FCSEL"
        );
        assert_eq!(
            words(&branch, 0xFFE0_0C00, 0x1E60_0C00),
            0,
            "the branch form selects nothing"
        );
        // A64 conditional branches reach only +/-1 MiB, so the encoder's
        // fixed-size long form is B.<inverse> over an unconditional B. NE
        // inverts to EQ, and the skip is exactly two instructions.
        assert_eq!(
            words(&branch, 0xFFFF_FFFF, 0x5400_0040),
            1,
            "the branch form takes its arm through exactly one conditional branch"
        );
        assert!(
            words(&branch, 0xFC00_0000, 0x1400_0000) >= 3,
            "the taken branch, the untaken edge, and the arm's jump to the join each need a B"
        );
        verify_exact_function(&branch, "branch-form scalar value function")
            .expect("the independent A64 decoder accepts the branch form");
    }

    /// A loop, encoded and decoded.
    ///
    /// The host running this is x86-64, so the assertion is on the image
    /// rather than on a number: an unconditional branch to an offset already
    /// emitted is the back edge, and the reserved spill slot the swapping
    /// edge needs has to appear as a store/load pair the independent decoder
    /// accepts. The value itself is checked where it can be executed, in the
    /// x64 and WebAssembly backends' own loop tests, over the same fixture.
    #[test]
    fn a_loop_encodes_a_backward_branch_and_its_cycle_scratch() {
        let ssa = crate::native::ssa::Program::loop_fixture_for_test(20.0, 3.0)
            .expect("build the loop program");
        let bytes =
            super::compile_value_function_from_ssa(&ssa).expect("encode and verify the loop");
        let backwards = bytes
            .chunks_exact(4)
            .enumerate()
            .filter(|(index, word)| {
                let word = u32::from_le_bytes((*word).try_into().expect("aligned word"));
                // B with a negative 26-bit signed word offset lands before the
                // instruction that made it, and only a back edge does that.
                let offset = ((word & 0x03FF_FFFF) << 6) as i32 >> 6;
                word & 0xFC00_0000 == 0x1400_0000
                    && offset < 0
                    && (*index as i64 + i64::from(offset)) >= 0
            })
            .count();
        assert_eq!(backwards, 1, "the latch branches back to the loop header");
        verify_exact_function(&bytes, "loop-form scalar value function")
            .expect("the independent A64 decoder accepts the loop form");
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::{FunctionCompiler, PreparedInstruction};
    use super::{
        compile_assignment_dispatch_function, compile_assignment_function,
        compile_assignment_pass_function, compile_fused_evaluation_kernel,
        compile_fused_stamp_kernel, compile_value_function,
    };
    use crate::native::EvalContext;
    use crate::native::aarch64::encoder::DReg;
    use crate::native::aarch64::verifier::verify_exact_function;
    use crate::native::assignment::NativeAssignment;
    use crate::native::expr::{CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram};
    use crate::native::model::{CodeOffset, NativeStampKernelIo};
    use crate::native::runtime::ExecutableMemory;
    use crate::native::ssa::ValueLocation;

    fn program(ops: Vec<NativeOp>, max_stack_depth: usize) -> NativeProgram {
        NativeProgram::from_ops_for_test(ops, max_stack_depth, Vec::new(), Vec::new())
    }

    fn execute(program: &NativeProgram, variables: &[f64]) -> f64 {
        execute_with_context(program, std::ptr::null(), variables)
    }

    #[cfg(target_arch = "aarch64")]
    fn execute_with_context(
        program: &NativeProgram,
        context: *const EvalContext,
        variables: &[f64],
    ) -> f64 {
        let bytes = compile_value_function(program).expect("compile AArch64 scalar function");
        let memory = ExecutableMemory::allocate(&bytes).expect("publish AArch64 scalar function");
        let entry: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(memory.ptr_at(0).expect("AArch64 scalar entry pointer")) };
        entry(context, variables.as_ptr())
    }

    #[cfg(target_arch = "aarch64")]
    fn execute_assignment(
        program: &NativeProgram,
        variable_index: usize,
        context: *const EvalContext,
        variables: &mut [f64],
    ) {
        let bytes = compile_assignment_function(variable_index, program)
            .expect("compile AArch64 assignment function");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("publish AArch64 assignment function");
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(memory.ptr_at(0).expect("AArch64 assignment entry pointer"))
        };
        entry(context, variables.as_mut_ptr());
    }

    #[cfg(target_arch = "aarch64")]
    fn execute_assignment_pass(
        assignments: &[NativeAssignment],
        context: *const EvalContext,
        variables: &mut [f64],
    ) {
        let bytes = compile_assignment_pass_function(assignments)
            .expect("compile AArch64 assignment-pass function");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("publish AArch64 assignment-pass function");
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(0)
                    .expect("AArch64 assignment-pass entry pointer"),
            )
        };
        entry(context, variables.as_mut_ptr());
    }

    #[cfg(target_arch = "aarch64")]
    fn align_test_image(image: &mut Vec<u8>) -> CodeOffset {
        while image.len() % 16 != 0 {
            image.extend_from_slice(&0xD503_201F_u32.to_le_bytes());
        }
        CodeOffset::new(image.len())
    }

    #[cfg(target_arch = "aarch64")]
    fn execute_fused_kernel(
        assignments: &[NativeAssignment],
        stamp_values: &[NativeProgram],
        jacobians: Option<&[Vec<NativeProgram>]>,
        published_current_pairs: &[Option<(usize, usize)>],
        context: *const EvalContext,
        variables: &mut [f64],
        io: &NativeStampKernelIo,
    ) {
        let assignment = compile_assignment_pass_function(assignments)
            .expect("compile AArch64 fused-kernel assignment");
        let mut image = assignment;
        let kernel_offset = align_test_image(&mut image).as_usize();
        // No prelude and no separately published entries: every program here is
        // a handful of instructions, so the kernel inlines all of them and the
        // offsets are only shape.
        let stamp_value_entries = vec![CodeOffset::new(0); stamp_values.len()];
        let jacobian_entries: Vec<Vec<CodeOffset>> = jacobians
            .map(|rows| {
                rows.iter()
                    .map(|row| vec![CodeOffset::new(0); row.len()])
                    .collect()
            })
            .unwrap_or_default();
        let entries = A64FusedKernelEntries {
            stamp_values: &stamp_value_entries,
            jacobians: &jacobian_entries,
        };
        let kernel = match jacobians {
            Some(jacobians) => compile_fused_stamp_kernel(
                kernel_offset,
                CodeOffset::new(0),
                None,
                stamp_values,
                jacobians,
                entries,
                published_current_pairs,
            )
            .expect("compile AArch64 fused stamp kernel"),
            None => compile_fused_evaluation_kernel(
                kernel_offset,
                CodeOffset::new(0),
                None,
                stamp_values,
                entries,
                published_current_pairs,
            )
            .expect("compile AArch64 fused evaluation kernel"),
        };
        image.extend_from_slice(&kernel);
        let memory = ExecutableMemory::allocate(&image).expect("publish AArch64 fused kernel");
        let entry: extern "C" fn(*const EvalContext, *mut f64, *const NativeStampKernelIo) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(kernel_offset)
                    .expect("AArch64 fused-kernel entry pointer"),
            )
        };
        entry(context, variables.as_mut_ptr(), io);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn executes_shared_ssa_arithmetic_and_variable_loads() {
        let expression = program(
            vec![
                NativeOp::Const(2.0),
                NativeOp::LoadVariable(0),
                NativeOp::Mul,
                NativeOp::Const(3.0),
                NativeOp::Add,
            ],
            2,
        );
        assert_eq!(execute(&expression, &[19.5]), 42.0);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn assignment_entries_store_results_and_skip_failed_stores() {
        let assignment = program(vec![NativeOp::LoadVariable(0), NativeOp::MulConst(2.0)], 1);
        let mut variables = [3.0_f64, 0.0];
        execute_assignment(&assignment, 1, std::ptr::null(), &mut variables);
        assert_eq!(variables, [3.0, 6.0]);

        let helper_assignment = program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
            ],
            1,
        );
        execute_assignment(&helper_assignment, 1, std::ptr::null(), &mut variables);
        assert!((variables[1] - 3.0_f64.exp()).abs() < 1.0e-14);

        let context = EvalContext::empty_for_test();
        variables[1] = 99.0;
        let failed = NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadPriorCurrent(0)],
            1,
            Vec::new(),
            vec![0],
        );
        execute_assignment(&failed, 1, &context, &mut variables);
        assert_eq!(variables[1], 99.0);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn assignment_pass_preserves_source_order() {
        let assignments = [
            NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::Const(1.0)], 1),
            },
            NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(2.0)], 1),
            },
        ];
        let mut variables = [100.0_f64, 200.0];
        execute_assignment_pass(&assignments, std::ptr::null(), &mut variables);
        assert_eq!(variables, [1.0, 3.0]);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn assignment_pass_handles_indexed_targets_and_failure_atomicity() {
        let valid = [NativeAssignment::Indexed {
            base: 0,
            len: 3,
            lower: 1,
            index: program(vec![NativeOp::Const(2.49)], 1),
            value: program(vec![NativeOp::Const(9.0)], 1),
        }];
        let mut variables = [1.0_f64, 2.0, 3.0];
        execute_assignment_pass(&valid, std::ptr::null(), &mut variables);
        assert_eq!(variables, [1.0, 9.0, 3.0]);

        let invalid = [NativeAssignment::Indexed {
            base: 0,
            len: 3,
            lower: 1,
            index: program(vec![NativeOp::Const(4.0)], 1),
            value: program(vec![NativeOp::Const(99.0)], 1),
        }];
        let context = EvalContext::empty_for_test();
        execute_assignment_pass(&invalid, &context, &mut variables);
        assert_eq!(variables, [1.0, 9.0, 3.0]);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn assignment_pass_executes_bounded_runtime_loops() {
        let loop_body = vec![
            NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(1), NativeOp::AddConst(1.0)], 1),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::SubConst(1.0)], 1),
            },
        ];
        let assignments = [NativeAssignment::Loop {
            condition: program(vec![NativeOp::LoadVariable(0)], 1),
            body: loop_body,
        }];
        let mut variables = [3.0_f64, 0.0];
        execute_assignment_pass(&assignments, std::ptr::null(), &mut variables);
        assert_eq!(variables, [0.0, 3.0]);

        let unbounded = [NativeAssignment::Loop {
            condition: program(vec![NativeOp::Const(1.0)], 1),
            body: Vec::new(),
        }];
        let context = EvalContext::empty_for_test();
        execute_assignment_pass(&unbounded, &context, &mut variables);
        assert_eq!(variables, [0.0, 3.0]);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn assignment_dispatch_runs_chunks_in_order_and_aborts_on_failure() {
        let ordered_chunks = [
            vec![NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::Const(1.0)], 1),
            }],
            vec![NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(2.0)], 1),
            }],
        ];
        let mut image = Vec::new();
        let mut chunk_offsets = Vec::new();
        for chunk in &ordered_chunks {
            let offset = align_test_image(&mut image);
            let bytes = compile_assignment_pass_function(chunk).expect("compile ordered chunk");
            image.extend_from_slice(&bytes);
            chunk_offsets.push(offset);
        }
        let dispatcher_offset = align_test_image(&mut image);
        let dispatcher =
            compile_assignment_dispatch_function(dispatcher_offset.as_usize(), &chunk_offsets)
                .expect("compile ordered dispatcher");
        image.extend_from_slice(&dispatcher);
        let memory = ExecutableMemory::allocate(&image).expect("publish ordered dispatcher");
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(dispatcher_offset.as_usize())
                    .expect("ordered dispatcher pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let mut variables = [100.0_f64, 200.0];
        entry(&context, variables.as_mut_ptr());
        assert_eq!(variables, [1.0, 3.0]);
        assert!(context.take_runtime_error().is_none());

        let failed_chunks = [
            vec![NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::Const(4.0)], 1),
            }],
            vec![NativeAssignment::Direct {
                var_index: 1,
                program: NativeProgram::from_ops_for_test(
                    vec![NativeOp::LoadPriorCurrent(0)],
                    1,
                    Vec::new(),
                    vec![0],
                ),
            }],
            vec![NativeAssignment::Direct {
                var_index: 2,
                program: program(vec![NativeOp::Const(9.0)], 1),
            }],
        ];
        let mut image = Vec::new();
        let mut chunk_offsets = Vec::new();
        for chunk in &failed_chunks {
            let offset = align_test_image(&mut image);
            let bytes = compile_assignment_pass_function(chunk).expect("compile failing chunk");
            image.extend_from_slice(&bytes);
            chunk_offsets.push(offset);
        }
        let dispatcher_offset = align_test_image(&mut image);
        let dispatcher =
            compile_assignment_dispatch_function(dispatcher_offset.as_usize(), &chunk_offsets)
                .expect("compile failing dispatcher");
        image.extend_from_slice(&dispatcher);
        let memory = ExecutableMemory::allocate(&image).expect("publish failing dispatcher");
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(dispatcher_offset.as_usize())
                    .expect("failing dispatcher pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let mut variables = [0.0_f64, 20.0, 30.0];
        entry(&context, variables.as_mut_ptr());
        assert_eq!(variables, [4.0, 20.0, 30.0]);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn fused_evaluation_kernel_assigns_filters_and_publishes_currents() {
        let assignments = [NativeAssignment::Direct {
            var_index: 0,
            program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(1.0)], 1),
        }];
        let stamp_values = [
            program(vec![NativeOp::LoadVariable(0), NativeOp::MulConst(2.0)], 1),
            program(vec![NativeOp::Const(7.0)], 1),
        ];
        let current_pairs = [Some((0, 1)), None];
        let active = [1_u8, 0];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: std::ptr::null_mut(),
        };
        let mut currents = [-1.0_f64, -2.0];
        let mut branch_currents = [-3.0_f64, -4.0];
        let mut context = EvalContext::empty_for_test();
        context.currents = currents.as_mut_ptr();
        context.currents_len = currents.len();
        context.branch_currents = branch_currents.as_mut_ptr();
        context.branch_currents_len = branch_currents.len();
        let mut variables = [1.0_f64];

        execute_fused_kernel(
            &assignments,
            &stamp_values,
            None,
            &current_pairs,
            &context,
            &mut variables,
            &io,
        );

        assert_eq!(variables, [2.0]);
        assert_eq!(currents, [4.0, -2.0]);
        assert_eq!(branch_currents, [4.0, -4.0]);
        assert!(context.take_runtime_error().is_none());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn fused_stamp_kernel_writes_shared_jacobians() {
        let stamp_values = [program(vec![NativeOp::Const(5.0)], 1)];
        let derivative = || program(vec![NativeOp::LoadVariable(0), NativeOp::MulConst(3.0)], 1);
        let jacobians = [vec![derivative(), derivative()]];
        let current_pairs = [None];
        let active = [1_u8];
        let mut jacobian_values = [-1.0_f64, -2.0];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: jacobian_values.as_mut_ptr(),
        };
        let mut currents = [-3.0_f64];
        let mut context = EvalContext::empty_for_test();
        context.currents = currents.as_mut_ptr();
        context.currents_len = currents.len();
        let mut variables = [4.0_f64];

        execute_fused_kernel(
            &[],
            &stamp_values,
            Some(&jacobians),
            &current_pairs,
            &context,
            &mut variables,
            &io,
        );

        assert_eq!(currents, [5.0]);
        assert_eq!(jacobian_values, [12.0, 12.0]);
        assert!(context.take_runtime_error().is_none());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn fused_kernels_abort_before_publishing_failed_or_non_finite_values() {
        let active = [1_u8, 1];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: std::ptr::null_mut(),
        };
        let current_pairs = [None, None];
        let mut currents = [-1.0_f64, -2.0];
        let mut context = EvalContext::empty_for_test();
        context.currents = currents.as_mut_ptr();
        context.currents_len = currents.len();
        let mut variables = [11.0_f64];
        let non_finite = [
            program(vec![NativeOp::Const(f64::INFINITY)], 1),
            program(vec![NativeOp::Const(2.0)], 1),
        ];
        execute_fused_kernel(
            &[],
            &non_finite,
            None,
            &current_pairs,
            &context,
            &mut variables,
            &io,
        );
        assert_eq!(currents, [-1.0, -2.0]);
        assert!(context.take_runtime_error().is_some());

        let failed_assignment = [NativeAssignment::Direct {
            var_index: 0,
            program: NativeProgram::from_ops_for_test(
                vec![NativeOp::LoadPriorCurrent(2)],
                1,
                Vec::new(),
                vec![2],
            ),
        }];
        let finite = [program(vec![NativeOp::Const(3.0)], 1)];
        let active = [1_u8];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: std::ptr::null_mut(),
        };
        execute_fused_kernel(
            &failed_assignment,
            &finite,
            None,
            &[None],
            &context,
            &mut variables,
            &io,
        );
        assert_eq!(variables, [11.0]);
        assert_eq!(currents, [-1.0, -2.0]);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn executes_spills_and_the_maximum_safe_frame_depth() {
        let count = 4096;
        let mut ops = vec![NativeOp::Const(1.0); count];
        ops.extend(std::iter::repeat_n(NativeOp::Add, count - 1));
        let expression = program(ops, count);
        assert_eq!(execute(&expression, &[]), count as f64);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn comparisons_and_logicals_match_nan_truthiness() {
        for (ops, expected) in [
            (
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::CompareConst(CompareOp::Eq, 0.0),
                ],
                0.0,
            ),
            (
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::CompareConst(CompareOp::Ne, 0.0),
                ],
                1.0,
            ),
            (
                vec![NativeOp::Const(f64::NAN), NativeOp::Logical(LogicalOp::Not)],
                0.0,
            ),
            (
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::LogicalConst(LogicalOp::And, true),
                ],
                1.0,
            ),
        ] {
            let expression = program(ops, 1);
            assert_eq!(execute(&expression, &[]), expected);
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn extremum_preserves_numeric_nan_and_signed_zero_contracts() {
        let numeric_over_nan = program(
            vec![
                NativeOp::Const(7.0),
                NativeOp::Const(f64::NAN),
                NativeOp::Extremum(ExtremumOp::Min),
            ],
            2,
        );
        assert_eq!(execute(&numeric_over_nan, &[]), 7.0);

        let signed_zero = program(
            vec![
                NativeOp::Const(-0.0),
                NativeOp::Const(0.0),
                NativeOp::Extremum(ExtremumOp::Max),
            ],
            2,
        );
        assert_eq!(execute(&signed_zero, &[]).to_bits(), (-0.0_f64).to_bits());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn helper_calls_preserve_spills_and_saved_entry_arguments() {
        let expression = program(
            vec![
                NativeOp::Const(2.0),
                NativeOp::LoadVariable(0),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
                NativeOp::Add,
                NativeOp::LoadVariable(1),
                NativeOp::Add,
            ],
            2,
        );
        let actual = execute(&expression, &[1.0, 4.0]);
        assert!((actual - (1.0_f64.exp() + 6.0)).abs() < 1.0e-14);

        let power = program(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(1),
                NativeOp::BinaryMath(crate::native::expr::BinaryMathOp::Pow),
            ],
            2,
        );
        assert_eq!(execute(&power, &[2.0, 3.0]), 8.0);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn floor_and_ceil_execute_without_helper_call_side_effects() {
        for (op, expected) in [
            (crate::native::expr::UnaryMathOp::Floor, -2.0),
            (crate::native::expr::UnaryMathOp::Ceil, -1.0),
        ] {
            let expression = program(vec![NativeOp::Const(-1.25), NativeOp::UnaryMath(op)], 1);
            assert_eq!(execute(&expression, &[]), expected);
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn scalar_and_operand_context_helpers_obey_aapcs64() {
        let table = program(vec![NativeOp::Const(1.0), NativeOp::TableLookup(0)], 1);
        assert_eq!(execute(&table, &[]), 0.0);

        let ddt = program(vec![NativeOp::Const(1.0), NativeOp::DdtState(0)], 1);
        assert_eq!(execute(&ddt, &[]), 0.0);
    }

    #[test]
    fn slew_derivative_helper_preserves_signed_rate_derivatives() {
        let mut filters = [crate::vm::SlewFilter::default()];
        let mut context = EvalContext::empty_for_test();
        context.analysis_type = 2;
        context.slew_filters = filters.as_mut_ptr();
        context.slew_filters_len = filters.len();

        let seed = program(
            vec![
                NativeOp::Const(0.0),
                NativeOp::Const(2.0),
                NativeOp::Const(-2.0),
                NativeOp::SlewState(0),
            ],
            3,
        );
        assert_eq!(execute_with_context(&seed, &context, &[]), 0.0);
        filters[0].commit();
        let accepted = filters[0].checkpoint();
        context.time = 1.0;

        let derivative = program(
            vec![
                NativeOp::Const(10.0),
                NativeOp::Const(0.0),
                NativeOp::Const(2.0),
                NativeOp::Const(-0.25),
                NativeOp::Const(-2.0),
                NativeOp::Const(0.5),
                NativeOp::SlewStateDerivative(0),
            ],
            6,
        );
        assert_eq!(
            execute_with_context(&derivative, &context, &[]).to_bits(),
            (-0.25_f64).to_bits(),
            "the rising branch must preserve the source-level derivative sign"
        );
        assert_eq!(filters[0].checkpoint(), accepted);
        assert!(context.take_runtime_error().is_none());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn mixed_and_pointer_helper_abi_sentinels_execute() {
        extern "C" fn mixed(input: f64, context: *const u64, id: usize) -> f64 {
            input + unsafe { *context } as f64 + id as f64
        }
        extern "C" fn pointer(operands: *const f64, context: *const u64, id: usize) -> f64 {
            unsafe { *operands + *operands.add(1) + *context as f64 + id as f64 }
        }

        let mut mixed_compiler = FunctionCompiler::new(0, true).expect("mixed ABI compiler");
        mixed_compiler
            .emit_literal(DReg::D2, 1.5)
            .expect("mixed input literal");
        let mixed_prepared = PreparedInstruction {
            operands: vec![DReg::D2],
            result: DReg::D3,
            used_mask: 0,
        };
        mixed_compiler
            .emit_scalar_context_helper(&mixed_prepared, 2, mixed as *const () as usize)
            .expect("mixed helper call");
        let mixed_bytes = mixed_compiler
            .finish(ValueLocation::Register(3))
            .expect("finish mixed sentinel");
        verify_exact_function(&mixed_bytes, "mixed helper ABI sentinel")
            .expect("verify mixed sentinel");
        let mixed_memory =
            ExecutableMemory::allocate(&mixed_bytes).expect("publish mixed sentinel");
        let mixed_entry: extern "C" fn(*const u64, *const f64) -> f64 =
            unsafe { std::mem::transmute(mixed_memory.ptr_at(0).expect("mixed sentinel pointer")) };
        let context = 40_u64;
        assert_eq!(mixed_entry(&context, std::ptr::null()), 43.5);

        let mut pointer_compiler = FunctionCompiler::new(0, true).expect("pointer ABI compiler");
        pointer_compiler
            .emit_literal(DReg::D2, 1.0)
            .expect("first pointer operand");
        pointer_compiler
            .emit_literal(DReg::D3, 2.0)
            .expect("second pointer operand");
        let pointer_prepared = PreparedInstruction {
            operands: vec![DReg::D2, DReg::D3],
            result: DReg::D4,
            used_mask: 0,
        };
        pointer_compiler
            .emit_operand_context_helper(&pointer_prepared, 2, 39, pointer as *const () as usize)
            .expect("pointer helper call");
        let pointer_bytes = pointer_compiler
            .finish(ValueLocation::Register(4))
            .expect("finish pointer sentinel");
        verify_exact_function(&pointer_bytes, "pointer helper ABI sentinel")
            .expect("verify pointer sentinel");
        let pointer_memory =
            ExecutableMemory::allocate(&pointer_bytes).expect("publish pointer sentinel");
        let pointer_entry: extern "C" fn(*const u64, *const f64) -> f64 = unsafe {
            std::mem::transmute(pointer_memory.ptr_at(0).expect("pointer sentinel pointer"))
        };
        assert_eq!(pointer_entry(&context, std::ptr::null()), 82.0);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn guarded_slice_loads_succeed_and_fail_with_early_return() {
        let branch_currents = [5.5_f64];
        let param_given = [1_u8];
        let mut context = EvalContext::empty_for_test();
        context.branch_currents = branch_currents.as_ptr();
        context.branch_currents_len = branch_currents.len();
        context.param_given = param_given.as_ptr();
        context.param_given_len = param_given.len();

        let current = NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadCurrent(0)],
            1,
            vec![0],
            Vec::new(),
        );
        assert_eq!(execute_with_context(&current, &context, &[]), 5.5);

        let given = program(vec![NativeOp::LoadParamGiven(0)], 1);
        assert_eq!(execute_with_context(&given, &context, &[]), 1.0);

        let missing_prior = NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadPriorCurrent(0), NativeOp::AddConst(100.0)],
            1,
            Vec::new(),
            vec![0],
        );
        assert_eq!(execute_with_context(&missing_prior, &context, &[]), 0.0);
        assert!(context.take_runtime_error().is_some());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn integer_and_analysis_ops_match_rust_and_verilog_contracts() {
        use crate::native::expr::IntegerBinaryOp;

        let bitwise = program(
            vec![
                NativeOp::Const(6.0),
                NativeOp::Const(3.0),
                NativeOp::IntegerBinary(IntegerBinaryOp::BitAnd),
            ],
            2,
        );
        assert_eq!(execute(&bitwise, &[]), 2.0);

        let shift = program(
            vec![
                NativeOp::Const(-8.0),
                NativeOp::IntegerShiftConst(IntegerBinaryOp::Shr, 2),
            ],
            1,
        );
        assert_eq!(execute(&shift, &[]), 1_073_741_822.0);

        let mut context = EvalContext::empty_for_test();
        let nan_cast = program(vec![NativeOp::Const(f64::NAN), NativeOp::IntegerCast], 1);
        assert_eq!(execute_with_context(&nan_cast, &context, &[]), 0.0);
        assert!(context.take_runtime_error().is_some());

        context.analysis_type = 4;
        context.analysis_initial_step = 1;
        for (analysis_id, expected) in [(0, 0.0), (5, 1.0), (7, 1.0)] {
            let analysis = program(vec![NativeOp::Analysis(analysis_id)], 1);
            assert_eq!(
                execute_with_context(&analysis, &context, &[]),
                expected,
                "analysis id {analysis_id}"
            );
        }

        let out_of_width_shift = program(
            vec![
                NativeOp::Const(1.0),
                NativeOp::Const(-1.0),
                NativeOp::IntegerBinary(IntegerBinaryOp::Shl),
            ],
            2,
        );
        assert_eq!(
            execute_with_context(&out_of_width_shift, &context, &[]),
            0.0
        );
        assert!(context.take_runtime_error().is_none());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn dynamic_variables_and_limit_state_execute_full_semantics() {
        let variables = [2.0_f64, 4.0, 8.0];
        let dynamic = program(
            vec![
                NativeOp::Const(2.49),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 3,
                    lower: 1,
                },
            ],
            1,
        );
        assert_eq!(execute(&dynamic, &variables), 4.0);

        let helper_dynamic = program(
            vec![
                NativeOp::Const((1_i64 << 52) as f64),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: 1_i64 << 52,
                },
            ],
            1,
        );
        assert_eq!(execute(&helper_dynamic, &variables), 2.0);

        let mut context = EvalContext::empty_for_test();
        let invalid_dynamic = program(
            vec![
                NativeOp::Const(4.0),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 3,
                    lower: 1,
                },
                NativeOp::AddConst(100.0),
            ],
            1,
        );
        assert_eq!(
            execute_with_context(&invalid_dynamic, &context, &variables),
            0.0
        );
        assert!(context.take_runtime_error().is_some());

        let mut state_values = [10.0_f64];
        let mut initialized = [1_u8];
        context.state_values = state_values.as_mut_ptr();
        context.state_values_len = state_values.len();
        context.state_initialized = initialized.as_mut_ptr();
        context.state_initialized_len = initialized.len();
        let limited = program(
            vec![
                NativeOp::Const(20.0),
                NativeOp::Const(3.0),
                NativeOp::LimitState(0),
            ],
            2,
        );
        assert_eq!(execute_with_context(&limited, &context, &[]), 13.0);
        assert_eq!(state_values[0], 13.0);

        initialized[0] = 0;
        assert_eq!(execute_with_context(&limited, &context, &[]), 20.0);
        assert_eq!(state_values[0], 20.0);
        assert_eq!(initialized[0], 1);
    }
}
