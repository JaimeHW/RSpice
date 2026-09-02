//! Emitting x86-64 machine code for a lowered native program.
//!
//! Compiles each lowered `NativeProgram` into a
//! function body: SSE2 scalar double arithmetic in XMM registers, spills to
//! the stack frame when registers run out, and `call`s into the
//! [`abi`](crate::native::abi) helpers for transcendentals and stateful
//! operators.
//!
//! Constants cannot be materialized until the code's final address is known,
//! so they are emitted as placeholders and recorded as patches to be applied
//! once the image is laid out. Every function follows the platform calling
//! convention exactly — the compiled code is entered by ordinary Rust
//! `extern "C"` pointers, with no shim to absorb a mistake.

use super::calling_convention::HOST_ABI;
use super::encoder::{ConditionCode, Gpr, Rel32Patch, X64Encoder, Xmm};
use super::ir::{
    ALLOCATABLE_VALUE_REGISTERS, AllocatedInstruction, AssignmentProgram,
    BasicBlock as X64BasicBlock, BlockId as X64BlockId, Effects as X64Effects,
    Program as X64SsaProgram, RegisterAllocation, RegisterBank, Terminator as X64Terminator,
    ValueLocation as X64ValueLocation, ValueType as X64ValueType,
    dynamic_variable_inline_supported, plan_shared_outputs,
};
use super::{
    CompiledX64Function, WindowsX64UnwindInfo, WindowsX64UnwindOperation, X64DataKind,
    X64DataRange, X64FunctionBody, X64RipRelativeRelocation,
};
use crate::jit::plan_program::PlanProgram;
use crate::native::abi::NativeRuntimeStatus;
use crate::native::abi::{
    INTEGER_CAST_DESCRIPTOR, integer_binary_const_descriptor, integer_binary_descriptor,
    integer_shift_const_descriptor, rspice_above_state_native,
    rspice_absdelay_derivative_max_native, rspice_absdelay_derivative_native,
    rspice_absdelay_state_max_native, rspice_absdelay_state_native, rspice_acos, rspice_acosh,
    rspice_asin, rspice_asinh, rspice_atan, rspice_atan2, rspice_atanh, rspice_ceil, rspice_cos,
    rspice_cosh, rspice_cross_state_native, rspice_ddt_jacobian_native, rspice_ddt_state_native,
    rspice_dynamic_variable_slot_native, rspice_exp, rspice_floor, rspice_hypot,
    rspice_idt_jacobian_native, rspice_idt_state_native, rspice_idtmod_state_native,
    rspice_integer_operation_native, rspice_laplace_derivative_native, rspice_laplace_step_native,
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
pub(crate) use crate::native::assignment::NativeAssignment;
use crate::native::assignment::shareable_batch_ranges;
use crate::native::expr::{BinaryMathOp, IntegerBinaryOp, UnaryMathOp, runtime_integer_operation};
use crate::native::expr::{CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram, VoltageNode};
use crate::native::{EvalContext, JitError, JitResult, NativeStampKernelIo};

const MODEL: &str = "native-x64";
const VOLTAGES_OFFSET: i32 = std::mem::offset_of!(EvalContext, voltages) as i32;
const INTERNAL_VOLTAGES_OFFSET: i32 = std::mem::offset_of!(EvalContext, internal_voltages) as i32;
const PARAMS_OFFSET: i32 = std::mem::offset_of!(EvalContext, params) as i32;
const BRANCH_CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, branch_currents) as i32;
const BRANCH_CURRENTS_LEN_OFFSET: i32 =
    std::mem::offset_of!(EvalContext, branch_currents_len) as i32;
const CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, currents) as i32;
const CURRENTS_LEN_OFFSET: i32 = std::mem::offset_of!(EvalContext, currents_len) as i32;
const PORT_CONNECTED_OFFSET: i32 = std::mem::offset_of!(EvalContext, port_connected) as i32;
const PORT_CONNECTED_LEN_OFFSET: i32 = std::mem::offset_of!(EvalContext, port_connected_len) as i32;
const TEMPERATURE_OFFSET: i32 = std::mem::offset_of!(EvalContext, temperature) as i32;
const TIME_OFFSET: i32 = std::mem::offset_of!(EvalContext, time) as i32;
const STATE_VALUES_OFFSET: i32 = std::mem::offset_of!(EvalContext, state_values) as i32;
const STATE_INITIALIZED_OFFSET: i32 = std::mem::offset_of!(EvalContext, state_initialized) as i32;
const STATE_INITIALIZED_LEN_OFFSET: i32 =
    std::mem::offset_of!(EvalContext, state_initialized_len) as i32;
const PARAM_GIVEN_OFFSET: i32 = std::mem::offset_of!(EvalContext, param_given) as i32;
const PARAM_GIVEN_LEN_OFFSET: i32 = std::mem::offset_of!(EvalContext, param_given_len) as i32;
const BRANCH_UNKNOWNS_OFFSET: i32 = std::mem::offset_of!(EvalContext, branch_unknowns) as i32;
const ANALYSIS_TYPE_OFFSET: i32 = std::mem::offset_of!(EvalContext, analysis_type) as i32;
const MFACTOR_OFFSET: i32 = std::mem::offset_of!(EvalContext, multiplicity) as i32;
const KERNEL_ACTIVE_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, program_active) as i32;
const KERNEL_JACOBIANS_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, jacobians) as i32;
const STATE_VALUES_LEN_OFFSET: i32 = std::mem::offset_of!(EvalContext, state_values_len) as i32;
const ANALYSIS_INITIAL_STEP_OFFSET: i32 =
    std::mem::offset_of!(EvalContext, analysis_initial_step) as i32;
const ANALYSIS_FINAL_STEP_OFFSET: i32 =
    std::mem::offset_of!(EvalContext, analysis_final_step) as i32;
const WORD_BYTES: usize = std::mem::size_of::<f64>();
const LITERAL_POOL_ALIGNMENT: usize = WORD_BYTES;
const VECTOR_LITERAL_ALIGNMENT: usize = 16;
const LITERAL_POOL_PADDING_BYTE: u8 = 0x90;
const ABS_VALUE_MASK_LOW: u64 = 0x7fff_ffff_ffff_ffff;
const ABS_VALUE_MASK_HIGH: u64 = 0;
const NEG_VALUE_MASK_LOW: u64 = 0x8000_0000_0000_0000;
const NEG_VALUE_MASK_HIGH: u64 = 0;
const K_BOLTZMANN: f64 = 1.380649e-23;
const Q_ELECTRON: f64 = 1.602176634e-19;
const THERMAL_VOLTAGE_PER_K: f64 = K_BOLTZMANN / Q_ELECTRON;
const F64_EXACT_INTEGER_LIMIT_ABS_BITS: u64 = 0x4330_0000_0000_0000;
#[cfg(all(test, target_arch = "x86_64"))]
const I64_MAX_EXCLUSIVE_AS_F64: f64 = 9_223_372_036_854_775_808.0;
#[cfg(all(test, target_arch = "x86_64"))]
const I64_MIN_AS_F64: f64 = -9_223_372_036_854_775_808.0;
#[cfg(all(test, target_arch = "x86_64"))]
const INLINE_DYNAMIC_LOWER_ABS_LIMIT: i64 = super::ir::INLINE_DYNAMIC_LOWER_ABS_LIMIT;
const DYNAMIC_READ_FRAME_BYTES: i32 = 16;
const ROUND_TEMP_FRAME_BYTES: i32 = 16;
const CALLEE_SAVED_XMM_BYTES: i32 = 16;
#[cfg(all(test, target_arch = "x86_64"))]
const CALL_RESULT_SLOT: usize = 6;
const CALL_SHADOW_BYTES: i32 = HOST_ABI.call_shadow_bytes;
const LOCAL_SLOT_BYTES: i32 = 8;
const LOCAL_FRAME_ALIGN_BYTES: i32 = 16;
/// Maximum distance between stack touches while reserving a generated frame.
///
/// Windows grows stacks through guard pages and requires large frames to probe
/// every intervening page before moving RSP.  Applying the same discipline on
/// every x64 host also prevents generated functions from creating a stack-clash
/// gap on System V targets.
const STACK_PROBE_INTERVAL_BYTES: i32 = 4096;
const INDEXED_ASSIGNMENT_SLOT_PTR_DISP: i32 = 0;
const MAX_RUNTIME_LOOP_ITERATIONS: i32 = 100_000;
const MAX_EXPRESSION_STACK_DEPTH: usize = 4096;
const CALLER_SAVED_XMM_COUNT: usize = HOST_ABI.caller_saved_xmm_count;
/// XMM0-XMM9 hold allocated values, and everything from
/// [`CALLER_SAVED_XMM_COUNT`] up is nonvolatile on the host ABI: XMM6-XMM9 on
/// Win64, nothing at all on System V, whose first nonvolatile index sits past
/// the end of the bank. Touching one obliges the prologue to preserve it,
/// which [`callee_saved_xmm_count_for_allocation`] already derives from the
/// highest register the allocation reaches.
const X64_VALUE_BANK: RegisterBank =
    RegisterBank::with_callee_saved_from(ALLOCATABLE_VALUE_REGISTERS, CALLER_SAVED_XMM_COUNT);
const XMM_STACK: [Xmm; 16] = [
    Xmm::Xmm0,
    Xmm::Xmm1,
    Xmm::Xmm2,
    Xmm::Xmm3,
    Xmm::Xmm4,
    Xmm::Xmm5,
    Xmm::Xmm6,
    Xmm::Xmm7,
    Xmm::Xmm8,
    Xmm::Xmm9,
    Xmm::Xmm10,
    Xmm::Xmm11,
    Xmm::Xmm12,
    Xmm::Xmm13,
    Xmm::Xmm14,
    Xmm::Xmm15,
];
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
pub(crate) fn compile_value_function(program: &NativeProgram) -> JitResult<Vec<u8>> {
    Ok(compile_value_function_artifact(program)?.bytes)
}

pub(crate) fn compile_value_function_artifact(
    program: &NativeProgram,
) -> JitResult<CompiledX64Function> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    compile_value_function_artifact_from_ssa(&X64SsaProgram::lower(program)?)
}

/// Compile one already-lowered value entry.
///
/// The postfix lift is the shipped route into this; taking the SSA directly is
/// what lets the branch form of a conditional be compiled through exactly the
/// same emitter, allocator, and verifier as the select form.
pub(crate) fn compile_value_function_artifact_from_ssa(
    ssa: &X64SsaProgram,
) -> JitResult<CompiledX64Function> {
    validate_expression_stack_depth(ssa.maximum_stack_depth())?;
    let allocation = RegisterAllocation::build(&ssa, X64_VALUE_BANK)?;
    let callee_saved_xmm_count = callee_saved_xmm_count_for_allocation(&allocation);
    let expression_spill_bytes = allocation_spill_frame_bytes(&allocation)?;
    let local_frame_bytes = checked_local_frame_bytes(&[
        expression_spill_bytes,
        callee_saved_xmm_frame_bytes(callee_saved_xmm_count),
    ])?;
    let mut compiler = FunctionCompiler::new(
        ssa.uses_helper_calls(),
        ssa.needs_saved_entry_args(),
        ssa.uses_helper_calls() || expression_spill_bytes > 0,
        local_frame_bytes,
        None,
        (expression_spill_bytes > 0).then_some(0),
        callee_saved_xmm_count,
    )?;
    compiler.emit_allocated_program(&ssa, &allocation)?;
    let windows_unwind = compiler.windows_unwind_info();
    let body = compiler.finish_value_function()?;
    let artifact = CompiledX64Function::new(body, windows_unwind);
    super::verify_x64_function_artifact(&artifact, "generated value function")?;
    Ok(artifact)
}

#[allow(dead_code)]
pub(crate) fn compile_assignment_function(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    Ok(compile_assignment_function_artifact(var_index, program)?.bytes)
}

fn compile_assignment_function_artifact(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<CompiledX64Function> {
    validate_expression_stack_depth(program.max_stack_depth())?;
    let ssa = X64SsaProgram::lower(program)?;
    validate_expression_stack_depth(ssa.maximum_stack_depth())?;
    let allocation = RegisterAllocation::build(&ssa, X64_VALUE_BANK)?;
    let callee_saved_xmm_count = callee_saved_xmm_count_for_allocation(&allocation);
    let expression_spill_bytes = allocation_spill_frame_bytes(&allocation)?;
    let local_frame_bytes = checked_local_frame_bytes(&[
        expression_spill_bytes,
        callee_saved_xmm_frame_bytes(callee_saved_xmm_count),
    ])?;
    let mut compiler = FunctionCompiler::new(
        ssa.uses_helper_calls(),
        ssa.uses_helper_calls(),
        ssa.uses_helper_calls() || expression_spill_bytes > 0,
        local_frame_bytes,
        None,
        (expression_spill_bytes > 0).then_some(0),
        callee_saved_xmm_count,
    )?;
    compiler.emit_allocated_program(&ssa, &allocation)?;
    let windows_unwind = compiler.windows_unwind_info();
    let body = compiler.finish_assignment_function(var_index)?;
    let artifact = CompiledX64Function::new(body, windows_unwind);
    super::verify_x64_function_artifact(&artifact, "generated assignment function")?;
    Ok(artifact)
}

pub(super) fn compile_fused_stamp_kernel_artifact(
    kernel_image_offset: usize,
    assignment: crate::native::model::CodeOffset,
    stamp_values: &[PlanProgram],
    jacobians: &[Vec<PlanProgram>],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<CompiledX64Function> {
    compile_fused_kernel_artifact(
        kernel_image_offset,
        assignment,
        stamp_values,
        Some(jacobians),
        published_current_pairs,
        "fused stamp",
    )
}

fn compile_fused_kernel_artifact(
    kernel_image_offset: usize,
    assignment: crate::native::model::CodeOffset,
    stamp_values: &[PlanProgram],
    jacobians: Option<&[Vec<PlanProgram>]>,
    published_current_pairs: &[Option<(usize, usize)>],
    kernel_name: &str,
) -> JitResult<CompiledX64Function> {
    if stamp_values.len() != published_current_pairs.len()
        || jacobians.is_some_and(|entries| entries.len() != stamp_values.len())
    {
        return Err(JitError::InternalCompilerError {
            model: MODEL.into(),
            detail: "fused kernel value, Jacobian, and publication shapes differ".into(),
        });
    }

    // The kernel inlines its entries rather than calling them, so it needs each
    // one in block form. A postfix entry is lifted exactly as it always was; a
    // block entry is already there. `emit_allocated_program` emits real
    // branches, so a multi-block entry inlines like any other.
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
        .map(|program| RegisterAllocation::build(program, X64_VALUE_BANK))
        .collect::<JitResult<Vec<_>>>()?;
    let jacobian_allocations = jacobian_ssa
        .as_ref()
        .map(|entries| {
            entries
                .iter()
                .map(|stamp| {
                    stamp
                        .iter()
                        .map(|program| RegisterAllocation::build(program, X64_VALUE_BANK))
                        .collect::<JitResult<Vec<_>>>()
                })
                .collect::<JitResult<Vec<_>>>()
        })
        .transpose()?;
    let maximum_stack_depth = value_ssa
        .iter()
        .chain(
            jacobian_ssa
                .iter()
                .flat_map(|entries| entries.iter().flatten()),
        )
        .map(X64SsaProgram::maximum_stack_depth)
        .max()
        .unwrap_or(0);
    validate_expression_stack_depth(maximum_stack_depth)?;
    let maximum_spill_slots = value_allocations
        .iter()
        .chain(
            jacobian_allocations
                .iter()
                .flat_map(|entries| entries.iter().flatten()),
        )
        .map(RegisterAllocation::spill_slot_count)
        .max()
        .unwrap_or(0);
    let maximum_required_registers = value_allocations
        .iter()
        .chain(
            jacobian_allocations
                .iter()
                .flat_map(|entries| entries.iter().flatten()),
        )
        .map(RegisterAllocation::required_register_count)
        .max()
        .unwrap_or(0);
    let expression_spill_bytes = maximum_spill_slots
        .checked_mul(WORD_BYTES)
        .and_then(|bytes| i32::try_from(bytes).ok())
        .ok_or_else(|| {
            register_allocation_error(
                "fused-kernel liveness spill frame exceeds the x64 displacement range".into(),
            )
        })?;
    let callee_saved_xmm_count = maximum_required_registers
        .min(XMM_STACK.len())
        .saturating_sub(CALLER_SAVED_XMM_COUNT);
    let local_frame_bytes = checked_local_frame_bytes(&[
        expression_spill_bytes,
        callee_saved_xmm_frame_bytes(callee_saved_xmm_count),
    ])?;
    let mut compiler = FunctionCompiler::new_kernel(
        local_frame_bytes,
        (expression_spill_bytes > 0).then_some(0),
        callee_saved_xmm_count,
    )?;

    compiler.emit_image_entry_call(kernel_image_offset, assignment)?;
    compiler.emit_kernel_abort_if_failed()?;

    let mut jacobian_index = 0_usize;
    for (stamp_index, ((program, allocation), current_pair)) in value_ssa
        .iter()
        .zip(&value_allocations)
        .zip(published_current_pairs)
        .enumerate()
    {
        let skip_stamp = compiler.emit_kernel_skip_if_inactive(stamp_index)?;
        compiler.emit_allocated_program(program, allocation)?;
        compiler.emit_kernel_non_finite_guard(stamp_index)?;
        compiler.emit_kernel_stamp_value_store(stamp_index, *current_pair)?;
        compiler.reset_expression_state();

        if let Some(stamp_jacobians) = jacobian_ssa.as_ref().map(|entries| &entries[stamp_index]) {
            let stamp_allocations = &jacobian_allocations
                .as_ref()
                .expect("Jacobian SSA and allocations are built together")[stamp_index];
            for group in plan_shared_outputs(stamp_jacobians) {
                let representative = group.representative();
                compiler.emit_allocated_program(
                    &stamp_jacobians[representative],
                    &stamp_allocations[representative],
                )?;
                for output in group.outputs() {
                    let output_index =
                        jacobian_index
                            .checked_add(*output)
                            .ok_or_else(|| JitError::Encoding {
                                model: MODEL.into(),
                                detail: "fused kernel Jacobian output index overflow".into(),
                            })?;
                    compiler.emit_kernel_jacobian_store(output_index)?;
                }
                compiler.reset_expression_state();
            }
            jacobian_index = jacobian_index
                .checked_add(stamp_jacobians.len())
                .ok_or_else(|| JitError::Encoding {
                    model: MODEL.into(),
                    detail: "fused kernel Jacobian index overflow".into(),
                })?;
        }
        compiler.patch_rel32_to_current(skip_stamp)?;
    }

    let windows_unwind = compiler.windows_unwind_info();
    let body = compiler.finish_assignment_pass_function()?;
    let artifact = CompiledX64Function::new(body, windows_unwind);
    super::verify_x64_function_artifact(&artifact, kernel_name)?;
    Ok(artifact)
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
pub(crate) fn compile_assignment_pass_function(
    assignments: &[NativeAssignment],
) -> JitResult<Vec<u8>> {
    Ok(compile_assignment_pass_function_artifact(assignments)?.bytes)
}

pub(super) fn compile_assignment_pass_function_artifact(
    assignments: &[NativeAssignment],
) -> JitResult<CompiledX64Function> {
    let has_indexed_assignment = assignments.iter().any(assignment_has_indexed);
    let loop_depth = assignment_loop_depth(assignments)?;
    let uses_helper_calls = assignments.iter().any(assignment_uses_helper_calls);
    let max_stack_depth = assignment_max_stack_depth(assignments);
    validate_expression_stack_depth(max_stack_depth)?;
    let (maximum_spill_slots, maximum_required_registers) =
        assignment_allocation_requirements(assignments)?;
    let callee_saved_xmm_count = maximum_required_registers
        .min(XMM_STACK.len())
        .saturating_sub(CALLER_SAVED_XMM_COUNT);
    let indexed_slot_bytes = if has_indexed_assignment {
        LOCAL_SLOT_BYTES
    } else {
        0
    };
    let loop_counter_base_disp = (loop_depth > 0).then_some(indexed_slot_bytes);
    let loop_slot_bytes = loop_depth.checked_mul(LOCAL_SLOT_BYTES).ok_or_else(|| {
        register_allocation_error("assignment-loop frame exceeds the x64 displacement range".into())
    })?;
    let expression_spill_base_disp =
        indexed_slot_bytes
            .checked_add(loop_slot_bytes)
            .ok_or_else(|| {
                register_allocation_error(
                    "indexed-assignment frame exceeds the x64 displacement range".into(),
                )
            })?;
    let expression_spill_bytes = maximum_spill_slots
        .checked_mul(WORD_BYTES)
        .and_then(|bytes| i32::try_from(bytes).ok())
        .ok_or_else(|| {
            register_allocation_error(
                "assignment liveness spill frame exceeds the x64 displacement range".into(),
            )
        })?;
    let local_frame_bytes = checked_local_frame_bytes(&[
        expression_spill_base_disp,
        expression_spill_bytes,
        callee_saved_xmm_frame_bytes(callee_saved_xmm_count),
    ])?;

    let mut compiler = FunctionCompiler::new(
        uses_helper_calls,
        uses_helper_calls,
        uses_helper_calls || expression_spill_bytes > 0,
        local_frame_bytes,
        loop_counter_base_disp,
        (expression_spill_bytes > 0).then_some(expression_spill_base_disp),
        callee_saved_xmm_count,
    )?;
    compiler.emit_assignment_steps(assignments, 0)?;
    let windows_unwind = compiler.windows_unwind_info();
    let body = compiler.finish_assignment_pass_function()?;
    let artifact = CompiledX64Function::new(body, windows_unwind);
    super::verify_x64_function_artifact(&artifact, "generated assignment-pass function")?;
    Ok(artifact)
}

pub(super) fn compile_assignment_dispatch_function_artifact(
    function_image_offset: usize,
    chunks: &[crate::native::model::CodeOffset],
) -> JitResult<CompiledX64Function> {
    if chunks.is_empty() {
        return Err(JitError::Encoding {
            model: MODEL.into(),
            detail: "assignment dispatcher requires at least one chunk".into(),
        });
    }

    // The dispatcher is itself an assignment entry: it preserves the context
    // and variable pointers, calls chunks in source order, and stops at the
    // first helper-reported runtime error.
    let mut compiler = FunctionCompiler::new(true, true, true, 0, None, None, 0)?;
    for chunk in chunks {
        compiler.emit_image_entry_call(function_image_offset, *chunk)?;
        compiler.emit_kernel_abort_if_failed()?;
    }
    let windows_unwind = compiler.windows_unwind_info();
    let body = compiler.finish_assignment_pass_function()?;
    let artifact = CompiledX64Function::new(body, windows_unwind);
    super::verify_x64_function_artifact(&artifact, "generated assignment dispatcher")?;
    Ok(artifact)
}

#[derive(Debug)]
struct FunctionCompiler {
    encoder: X64Encoder,
    /// Physical registers presented to the instruction emitters as their
    /// logical operand stack.  Liveness allocation can permute this window
    /// without making individual instruction encoders allocation-aware.
    register_stack: [Xmm; XMM_STACK.len()],
    /// Number of live expression values currently resident in `register_stack`.
    depth: usize,
    /// Number of older live expression values in the fixed local spill area.
    spilled_depth: usize,
    literals: Vec<LiteralPatch>,
    vector_literals: Vec<VectorLiteralPatch>,
    uses_helper_calls: bool,
    saves_entry_args: bool,
    uses_frame_pointer: bool,
    saves_kernel_io: bool,
    local_frame_bytes: i32,
    loop_counter_base_disp: Option<i32>,
    expression_spill_base_disp: Option<i32>,
    callee_saved_xmm_count: usize,
    early_return_jumps: Vec<Rel32Patch>,
    #[cfg_attr(not(windows), allow(dead_code))]
    windows_unwind_operations: Vec<WindowsX64UnwindOperation>,
    windows_unwind_prologue_size: u8,
}

#[derive(Debug)]
struct LiteralPatch {
    displacement_offset: usize,
    value: f64,
}

#[derive(Debug)]
struct VectorLiteralPatch {
    displacement_offset: usize,
    lanes: [u64; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionLiteralLayout {
    scalar_start: usize,
    scalar_bits: Vec<u64>,
    vector_start: usize,
    vector_bits: Vec<[u64; 2]>,
    final_len: usize,
}

impl FunctionLiteralLayout {
    fn build(
        code_len: usize,
        scalars: &[LiteralPatch],
        vectors: &[VectorLiteralPatch],
    ) -> JitResult<Self> {
        let mut scalar_bits = Vec::new();
        for literal in scalars {
            let bits = literal.value.to_bits();
            if !scalar_bits.contains(&bits) {
                scalar_bits.push(bits);
            }
        }
        let scalar_start = if scalar_bits.is_empty() {
            code_len
        } else {
            checked_align_up(code_len, LITERAL_POOL_ALIGNMENT)?
        };
        let scalar_end = scalar_bits
            .len()
            .checked_mul(WORD_BYTES)
            .and_then(|bytes| scalar_start.checked_add(bytes))
            .ok_or_else(|| JitError::Relocation {
                model: MODEL.into(),
                detail: "scalar literal layout overflow".into(),
            })?;

        let mut vector_bits = Vec::new();
        for literal in vectors {
            if !vector_bits.contains(&literal.lanes) {
                vector_bits.push(literal.lanes);
            }
        }
        let vector_start = if vector_bits.is_empty() {
            scalar_end
        } else {
            checked_align_up(scalar_end, VECTOR_LITERAL_ALIGNMENT)?
        };
        let final_len = vector_bits
            .len()
            .checked_mul(VECTOR_LITERAL_ALIGNMENT)
            .and_then(|bytes| vector_start.checked_add(bytes))
            .ok_or_else(|| JitError::Relocation {
                model: MODEL.into(),
                detail: "vector literal layout overflow".into(),
            })?;
        Ok(Self {
            scalar_start,
            scalar_bits,
            vector_start,
            vector_bits,
            final_len,
        })
    }

    fn scalar_offset(&self, bits: u64) -> usize {
        self.scalar_start
            + self
                .scalar_bits
                .iter()
                .position(|candidate| *candidate == bits)
                .expect("layout contains every scalar relocation")
                * WORD_BYTES
    }

    fn vector_offset(&self, bits: [u64; 2]) -> usize {
        self.vector_start
            + self
                .vector_bits
                .iter()
                .position(|candidate| *candidate == bits)
                .expect("layout contains every vector relocation")
                * VECTOR_LITERAL_ALIGNMENT
    }
}

fn checked_align_up(value: usize, alignment: usize) -> JitResult<usize> {
    debug_assert!(alignment.is_power_of_two());
    value
        .checked_add(alignment - 1)
        .map(|rounded| rounded & !(alignment - 1))
        .ok_or_else(|| JitError::Relocation {
            model: MODEL.into(),
            detail: "x64 function layout alignment overflow".into(),
        })
}

impl FunctionCompiler {
    fn new(
        uses_helper_calls: bool,
        saves_entry_args: bool,
        uses_frame_pointer: bool,
        local_frame_bytes: i32,
        loop_counter_base_disp: Option<i32>,
        expression_spill_base_disp: Option<i32>,
        callee_saved_xmm_count: usize,
    ) -> JitResult<Self> {
        Self::new_with_kernel_io(
            uses_helper_calls,
            saves_entry_args,
            uses_frame_pointer,
            false,
            local_frame_bytes,
            loop_counter_base_disp,
            expression_spill_base_disp,
            callee_saved_xmm_count,
        )
    }

    fn new_kernel(
        local_frame_bytes: i32,
        expression_spill_base_disp: Option<i32>,
        callee_saved_xmm_count: usize,
    ) -> JitResult<Self> {
        Self::new_with_kernel_io(
            true,
            true,
            true,
            true,
            local_frame_bytes,
            None,
            expression_spill_base_disp,
            callee_saved_xmm_count,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_with_kernel_io(
        uses_helper_calls: bool,
        saves_entry_args: bool,
        uses_frame_pointer: bool,
        saves_kernel_io: bool,
        local_frame_bytes: i32,
        loop_counter_base_disp: Option<i32>,
        expression_spill_base_disp: Option<i32>,
        callee_saved_xmm_count: usize,
    ) -> JitResult<Self> {
        if saves_kernel_io && !uses_frame_pointer {
            return Err(JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: "kernel I/O requires a preserved R14 frame anchor".into(),
            });
        }
        if local_frame_bytes < 0 || local_frame_bytes % LOCAL_FRAME_ALIGN_BYTES != 0 {
            return Err(JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: format!(
                    "generated local frame size {local_frame_bytes} is negative or not {LOCAL_FRAME_ALIGN_BYTES}-byte aligned"
                )
                .into(),
            });
        }
        let maximum_callee_saved_xmm_count = XMM_STACK.len() - CALLER_SAVED_XMM_COUNT;
        if callee_saved_xmm_count > maximum_callee_saved_xmm_count {
            return Err(JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: format!(
                    "generated frame requests {callee_saved_xmm_count} callee-saved XMM slots, but this ABI exposes only {maximum_callee_saved_xmm_count}"
                )
                .into(),
            });
        }
        let callee_saved_xmm_bytes = callee_saved_xmm_frame_bytes(callee_saved_xmm_count);
        if callee_saved_xmm_bytes > local_frame_bytes {
            return Err(JitError::RegisterAllocation {
                model: MODEL.into(),
                detail: format!(
                    "generated {local_frame_bytes}-byte frame cannot hold {callee_saved_xmm_bytes} bytes of callee-saved XMM state"
                )
                .into(),
            });
        }
        for (name, displacement) in [
            ("loop counter", loop_counter_base_disp),
            ("expression spill", expression_spill_base_disp),
        ] {
            let Some(displacement) = displacement else {
                continue;
            };
            let slot_end = displacement.checked_add(LOCAL_SLOT_BYTES);
            if displacement < 0 || slot_end.is_none_or(|end| end > local_frame_bytes) {
                return Err(JitError::RegisterAllocation {
                    model: MODEL.into(),
                    detail: format!(
                        "generated {name} slot at byte {displacement} falls outside the {local_frame_bytes}-byte local frame"
                    )
                    .into(),
                });
            }
        }
        let mut compiler = Self {
            encoder: X64Encoder::new(),
            register_stack: XMM_STACK,
            depth: 0,
            spilled_depth: 0,
            literals: Vec::new(),
            vector_literals: Vec::new(),
            uses_helper_calls,
            saves_entry_args,
            uses_frame_pointer,
            saves_kernel_io,
            local_frame_bytes,
            loop_counter_base_disp,
            expression_spill_base_disp,
            callee_saved_xmm_count,
            early_return_jumps: Vec::new(),
            windows_unwind_operations: Vec::new(),
            windows_unwind_prologue_size: 0,
        };
        if compiler.has_stack_setup() {
            compiler.emit_prologue()?;
        }
        Ok(compiler)
    }

    fn has_stack_setup(&self) -> bool {
        self.uses_frame_pointer || self.saves_entry_args || self.local_frame_bytes > 0
    }

    fn saves_entry_args(&self) -> bool {
        self.saves_entry_args
    }

    fn emit_native_program(&mut self, program: &NativeProgram) -> JitResult<()> {
        let ssa = X64SsaProgram::lower(program)?;
        self.emit_program(&ssa)
    }

    fn emit_program(&mut self, ssa: &X64SsaProgram) -> JitResult<()> {
        validate_expression_stack_depth(ssa.maximum_stack_depth())?;
        let allocation = RegisterAllocation::build(ssa, X64_VALUE_BANK)?;
        self.emit_allocated_program(ssa, &allocation)
    }

    fn emit_allocated_program(
        &mut self,
        ssa: &X64SsaProgram,
        allocation: &RegisterAllocation,
    ) -> JitResult<()> {
        self.emit_allocated_program_body(ssa, allocation, None)?;
        self.materialize_allocated_result(allocation.result())
    }

    fn emit_allocated_program_body(
        &mut self,
        ssa: &X64SsaProgram,
        allocation: &RegisterAllocation,
        assignment: Option<&AssignmentProgram>,
    ) -> JitResult<()> {
        if ssa.instructions().len() != allocation.instructions().len() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "x64 SSA and register-allocation instruction counts differ".into(),
            });
        }
        if assignment.is_some() && !ssa.is_single_block() {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: "x64 direct-assignment batches publish in one straight-line block".into(),
            });
        }

        self.depth = 0;
        self.spilled_depth = 0;
        let mut output_index = 0_usize;
        let mut block_offsets: Vec<Option<usize>> = vec![None; ssa.blocks().len()];
        let mut pending_jumps: Vec<(usize, Rel32Patch)> = Vec::new();
        for block in ssa.blocks() {
            block_offsets[block.id().index()] = Some(self.encoder.position());
            // A block reached from more than one predecessor cannot inherit what
            // any single one of them left cached in RAX.
            let mut context_pointer_cache = None;
            let range = block.instruction_start()..block.instruction_end();
            for (instruction_index, (instruction, allocated)) in ssa.instructions()[range.clone()]
                .iter()
                .zip(&allocation.instructions()[range])
                .enumerate()
                .map(|(offset, pair)| (block.instruction_start() + offset, pair))
            {
                if instruction.value_type() != X64ValueType::F64 {
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "x64 codegen cannot legalize SSA value type {:?}",
                            instruction.value_type()
                        )
                        .into(),
                    });
                }
                let result_register = self.prepare_allocated_instruction(allocated)?;

                let op = instruction.op();
                match op {
                    NativeOp::Const(value) => {
                        let dst = self.push_register()?;
                        self.emit_constant_load(dst, value);
                    }
                    NativeOp::LoadParam(index) => {
                        let dst = self.push_register()?;
                        self.emit_context_pointer_load_cached(
                            PARAMS_OFFSET,
                            &mut context_pointer_cache,
                        );
                        self.encoder
                            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                    }
                    NativeOp::LoadParamGiven(index) => {
                        self.emit_param_given_load(index, &mut context_pointer_cache)?;
                    }
                    NativeOp::LoadPortConnected(index) => {
                        self.emit_port_connected_load(index, &mut context_pointer_cache)?;
                    }
                    NativeOp::LoadVoltage { pos, neg } => {
                        self.emit_voltage_load(pos, neg, &mut context_pointer_cache)?;
                    }
                    NativeOp::LoadCurrent(pair_index) => {
                        self.emit_current_load(pair_index, &mut context_pointer_cache)?;
                    }
                    NativeOp::LoadPriorCurrent(current_index) => {
                        self.emit_prior_current_load(current_index, &mut context_pointer_cache)?;
                    }
                    NativeOp::LoadInternalVoltage(index) => {
                        let dst = self.push_register()?;
                        self.emit_context_pointer_load_cached(
                            INTERNAL_VOLTAGES_OFFSET,
                            &mut context_pointer_cache,
                        );
                        self.encoder
                            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                    }
                    NativeOp::LoadVariable(index) => {
                        let dst = self.push_register()?;
                        self.encoder.movsd_xmm_m64_base_disp32(
                            dst,
                            self.vars_arg_reg(),
                            byte_disp(index)?,
                        );
                    }
                    NativeOp::LoadVariableDyn { base, len, lower } => {
                        self.emit_dynamic_variable_load(base, len, lower)?;
                    }
                    NativeOp::LoadBranchUnknown(index) => {
                        let dst = self.push_register()?;
                        self.emit_context_pointer_load_cached(
                            BRANCH_UNKNOWNS_OFFSET,
                            &mut context_pointer_cache,
                        );
                        self.encoder
                            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                    }
                    NativeOp::LoadTemperature => {
                        self.emit_context_f64_load(TEMPERATURE_OFFSET)?;
                    }
                    NativeOp::LoadThermalVoltage => {
                        self.emit_thermal_voltage_load()?;
                    }
                    NativeOp::LoadTime => {
                        self.emit_context_f64_load(TIME_OFFSET)?;
                    }
                    NativeOp::Analysis(analysis_id) => {
                        self.emit_analysis_check(analysis_id)?;
                    }
                    NativeOp::LoadMfactor => {
                        self.emit_context_f64_load(MFACTOR_OFFSET)?;
                    }
                    NativeOp::Add => self.emit_binary_op(BinaryOp::Add)?,
                    NativeOp::Sub => self.emit_binary_op(BinaryOp::Sub)?,
                    NativeOp::Mul => self.emit_binary_op(BinaryOp::Mul)?,
                    NativeOp::Div => self.emit_binary_op(BinaryOp::Div)?,
                    NativeOp::AddConst(value) => {
                        self.emit_literal_rhs_binary_op(value, BinaryOp::Add)?
                    }
                    NativeOp::SubConst(value) => {
                        self.emit_literal_rhs_binary_op(value, BinaryOp::Sub)?
                    }
                    NativeOp::MulConst(value) => {
                        self.emit_literal_rhs_binary_op(value, BinaryOp::Mul)?
                    }
                    NativeOp::DivConst(value) => {
                        self.emit_literal_rhs_binary_op(value, BinaryOp::Div)?
                    }
                    NativeOp::SubFromConst(value) => {
                        self.emit_literal_lhs_binary_op(value, BinaryOp::Sub)?
                    }
                    NativeOp::DivFromConst(value) => {
                        self.emit_literal_lhs_binary_op(value, BinaryOp::Div)?
                    }
                    NativeOp::Neg => self.emit_neg()?,
                    NativeOp::Abs => self.emit_abs()?,
                    NativeOp::Square => self.emit_square()?,
                    NativeOp::Sqrt => self.emit_sqrt()?,
                    NativeOp::Compare(op) => self.emit_compare(op)?,
                    NativeOp::CompareConst(op, value) => self.emit_compare_const(op, value)?,
                    NativeOp::Logical(op) => self.emit_logical(op)?,
                    NativeOp::LogicalConst(op, value) => self.emit_logical_const(op, value)?,
                    NativeOp::IfElse => self.emit_ifelse()?,
                    NativeOp::Extremum(op) => self.emit_extremum(op)?,
                    NativeOp::ExtremumConst(op, value) => self.emit_extremum_const(op, value)?,
                    NativeOp::ExtremumConstLhs(op, value) => {
                        self.emit_extremum_const_lhs(op, value)?
                    }
                    NativeOp::UnaryMath(op) => self.emit_unary_math(op)?,
                    NativeOp::BinaryMath(op) => self.emit_binary_math(op)?,
                    NativeOp::IntegerCast => self.emit_integer_cast()?,
                    NativeOp::IntegerBinary(op) => self.emit_integer_binary(op)?,
                    NativeOp::IntegerShiftConst(op, count) => {
                        self.emit_integer_shift_const(op, count)?
                    }
                    NativeOp::IntegerBinaryConst(op, value) => {
                        self.emit_integer_bitwise_const(op, value)?
                    }
                    NativeOp::TableLookup(table_id) => {
                        self.emit_table_helper_call(table_id, rspice_table_lookup_native)?
                    }
                    NativeOp::TableDerivative(table_id) => {
                        self.emit_table_helper_call(table_id, rspice_table_derivative_native)?
                    }
                    NativeOp::LimitState(index) => self.emit_limit_state(index)?,
                    NativeOp::LimiterPrevious(index) => {
                        self.emit_limiter_state_helper(index, rspice_limiter_previous_native)?
                    }
                    NativeOp::LimiterStore(index) => self.emit_limiter_store(index)?,
                    NativeOp::LaplaceState(filter_id) => self.emit_laplace_state(filter_id)?,
                    NativeOp::LaplaceStateDerivative(filter_id) => {
                        self.emit_laplace_derivative(filter_id)?
                    }
                    NativeOp::ZiState(layout) => self.emit_zi_state(layout)?,
                    NativeOp::ZiStateDerivative(layout) => self.emit_zi_derivative_state(layout)?,
                    NativeOp::TimerState(timer_id) => self.emit_timer_state(timer_id)?,
                    NativeOp::TransitionState(filter_id) => {
                        self.emit_transition_state(filter_id)?
                    }
                    NativeOp::TransitionStateDerivative(filter_id) => {
                        self.emit_transition_derivative_state(filter_id)?
                    }
                    NativeOp::SlewState(filter_id) => self.emit_slew_state(filter_id)?,
                    NativeOp::SlewStateDerivative(filter_id) => {
                        self.emit_slew_derivative_state(filter_id)?
                    }
                    NativeOp::AbsDelayState(buffer_id) => self.emit_absdelay_state(buffer_id)?,
                    NativeOp::AbsDelayStateMax(buffer_id) => {
                        self.emit_absdelay_helper(buffer_id, 3, rspice_absdelay_state_max_native)?
                    }
                    NativeOp::AbsDelayStateDerivative(buffer_id) => {
                        self.emit_absdelay_helper(buffer_id, 4, rspice_absdelay_derivative_native)?
                    }
                    NativeOp::AbsDelayStateDerivativeMax(buffer_id) => self.emit_absdelay_helper(
                        buffer_id,
                        5,
                        rspice_absdelay_derivative_max_native,
                    )?,
                    NativeOp::CrossState(detector_id) => self.emit_cross_state(detector_id)?,
                    NativeOp::AboveState(detector_id) => self.emit_above_state(detector_id)?,
                    NativeOp::LastCrossingState(detector_id) => {
                        self.emit_last_crossing_state(detector_id)?
                    }
                    NativeOp::WhiteNoise => self.emit_white_noise()?,
                    NativeOp::FlickerNoise => self.emit_flicker_noise()?,
                    NativeOp::DdtState(index) => self.emit_ddt_state(index)?,
                    NativeOp::DdtJacobian => self.emit_ddt_jacobian()?,
                    NativeOp::IdtState(index) => self.emit_idt_state(index)?,
                    NativeOp::IdtJacobian => self.emit_idt_jacobian()?,
                    NativeOp::IdtModState(index) => self.emit_idtmod_state(index)?,
                }
                if self.logical_depth() != 1 {
                    return Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: format!(
                            "x64 emitter depth {} is not one result after allocated {op:?}",
                            self.logical_depth(),
                        )
                        .into(),
                    });
                }
                if let X64ValueLocation::Spill(slot) = allocated.result() {
                    let displacement = self.expression_spill_disp(slot)?;
                    self.encoder
                        .movsd_m64_base_disp32_xmm(Gpr::Rsp, displacement, result_register);
                }
                self.reset_expression_state();
                if instruction.effects().clobbers_context_pointer_cache() {
                    context_pointer_cache = None;
                }
                if let Some(assignment) = assignment {
                    let instruction_end = instruction_index + 1;
                    while let Some(output) = assignment.outputs().get(output_index).copied() {
                        if output.instruction_end() != instruction_end {
                            break;
                        }
                        self.emit_assignment_location_store(
                            allocation.location(output.value())?,
                            output.variable_index(),
                        )?;
                        output_index += 1;
                    }
                }
            }
            self.emit_terminator(allocation, block, &mut pending_jumps)?;
        }
        if let Some(assignment) = assignment
            && output_index != assignment.outputs().len()
        {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!(
                    "x64 emitted {output_index} of {} assignment outputs",
                    assignment.outputs().len()
                )
                .into(),
            });
        }
        for (target, patch) in pending_jumps {
            let offset = block_offsets[target].ok_or_else(|| JitError::Verifier {
                model: MODEL.into(),
                detail: format!("x64 branch targets block {target}, which was never emitted")
                    .into(),
            })?;
            self.patch_rel32_to_offset(patch, offset)?;
        }
        Ok(())
    }

    /// Emit one block's terminator.
    ///
    /// `Return` emits nothing: the caller either materializes the result into
    /// XMM0 or publishes assignment outputs. The exit block is always the last
    /// block in layout order, so nothing follows it here.
    fn emit_terminator(
        &mut self,
        allocation: &RegisterAllocation,
        block: &X64BasicBlock,
        pending_jumps: &mut Vec<(usize, Rel32Patch)>,
    ) -> JitResult<()> {
        let fallthrough = block.id().index() + 1;
        match block.terminator() {
            X64Terminator::Return(_) => Ok(()),
            X64Terminator::Jump(edge) => {
                self.emit_edge_moves(allocation, block.id(), 0)?;
                if edge.target().index() != fallthrough {
                    pending_jumps
                        .push((edge.target().index(), self.encoder.jmp_rel32_placeholder()));
                }
                Ok(())
            }
            X64Terminator::Branch {
                condition,
                then_edge,
                else_edge,
            } => {
                // Verilog-A truthiness is "not exactly zero", NaN included.
                // Clearing the sign bit and testing the remaining payload
                // decides that in one integer compare: only +0.0 and -0.0 are
                // left at zero, and every NaN keeps a nonzero exponent. This
                // is the same predicate the select form spells as UCOMISD
                // against zero plus CMOVNE and CMOVP, and RAX is volatile
                // under both supported ABIs and holds nothing at a block
                // boundary.
                match allocation.location(*condition)? {
                    X64ValueLocation::Register(register) => {
                        self.encoder
                            .movq_r64_xmm(Gpr::Rax, allocated_xmm(register)?);
                    }
                    X64ValueLocation::Spill(slot) => {
                        let displacement = self.expression_spill_disp(slot)?;
                        self.encoder
                            .mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rsp, displacement);
                    }
                }
                self.encoder.btr_r64_imm8(Gpr::Rax, 63);
                self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
                let taken = self.encoder.jcc_rel32_placeholder(ConditionCode::NotEqual);

                self.emit_edge_moves(allocation, block.id(), 1)?;
                // The taken arm's moves are laid out next, so the untaken edge
                // always needs its own jump even when its target follows.
                pending_jumps.push((
                    else_edge.target().index(),
                    self.encoder.jmp_rel32_placeholder(),
                ));

                self.patch_rel32_to_current(taken)?;
                self.emit_edge_moves(allocation, block.id(), 0)?;
                if then_edge.target().index() != fallthrough {
                    pending_jumps.push((
                        then_edge.target().index(),
                        self.encoder.jmp_rel32_placeholder(),
                    ));
                }
                Ok(())
            }
        }
    }

    fn emit_edge_moves(
        &mut self,
        allocation: &RegisterAllocation,
        block: X64BlockId,
        edge: usize,
    ) -> JitResult<()> {
        for step in allocation.edge_moves(block, edge)?.to_vec() {
            self.emit_location_move(step.from(), step.to())?;
        }
        Ok(())
    }

    fn emit_location_move(
        &mut self,
        from: X64ValueLocation,
        to: X64ValueLocation,
    ) -> JitResult<()> {
        match (from, to) {
            (X64ValueLocation::Register(source), X64ValueLocation::Register(destination)) => {
                let source = allocated_xmm(source)?;
                let destination = allocated_xmm(destination)?;
                if source != destination {
                    self.encoder.movsd_xmm_xmm(destination, source);
                }
            }
            (X64ValueLocation::Register(source), X64ValueLocation::Spill(slot)) => {
                let source = allocated_xmm(source)?;
                let displacement = self.expression_spill_disp(slot)?;
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::Rsp, displacement, source);
            }
            (X64ValueLocation::Spill(slot), X64ValueLocation::Register(destination)) => {
                let destination = allocated_xmm(destination)?;
                let displacement = self.expression_spill_disp(slot)?;
                self.encoder
                    .movsd_xmm_m64_base_disp32(destination, Gpr::Rsp, displacement);
            }
            (X64ValueLocation::Spill(source), X64ValueLocation::Spill(destination)) => {
                if source == destination {
                    return Ok(());
                }
                // RAX is volatile under both supported ABIs and holds nothing
                // at a block boundary, and a 64-bit integer move reproduces
                // the payload bit-for-bit without touching the SIMD bank,
                // which on Win64 would oblige the prologue to preserve it.
                let source = self.expression_spill_disp(source)?;
                let destination = self.expression_spill_disp(destination)?;
                self.encoder
                    .mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rsp, source);
                self.encoder
                    .mov_m64_base_disp32_r64(Gpr::Rsp, destination, Gpr::Rax);
            }
        }
        Ok(())
    }

    fn prepare_allocated_instruction(
        &mut self,
        allocated: &AllocatedInstruction,
    ) -> JitResult<Xmm> {
        if allocated.operands().len() > XMM_STACK.len() {
            return Err(register_allocation_error(format!(
                "allocated instruction requires {} simultaneous operands, but x64 exposes {} XMM registers",
                allocated.operands().len(),
                XMM_STACK.len()
            )));
        }

        self.reset_expression_state();
        let mut used_register_mask = allocated.live_register_mask();
        if let X64ValueLocation::Register(register) = allocated.result() {
            used_register_mask |= xmm_mask(allocated_xmm(register)?);
        }
        let mut operands = Vec::with_capacity(allocated.operands().len());
        for location in allocated.operands() {
            let register = match *location {
                X64ValueLocation::Register(register) => allocated_xmm(register)?,
                X64ValueLocation::Spill(slot) => {
                    let register = take_temporary_xmm(&mut used_register_mask)?;
                    self.encoder.movsd_xmm_m64_base_disp32(
                        register,
                        Gpr::Rsp,
                        self.expression_spill_disp(slot)?,
                    );
                    register
                }
            };
            used_register_mask |= xmm_mask(register);
            operands.push(register);
        }

        let result_register = match allocated.result() {
            X64ValueLocation::Register(register) => allocated_xmm(register)?,
            X64ValueLocation::Spill(_) => match (allocated.operands().first(), operands.first()) {
                (Some(X64ValueLocation::Spill(_)), Some(register)) => *register,
                _ => take_temporary_xmm(&mut used_register_mask)?,
            },
        };
        used_register_mask |= xmm_mask(result_register);

        if let Some(first_operand) = operands.first()
            && *first_operand != result_register
        {
            self.encoder.movsd_xmm_xmm(result_register, *first_operand);
        }

        self.register_stack = XMM_STACK;
        self.register_stack[0] = result_register;
        for (position, register) in operands.iter().copied().enumerate().skip(1) {
            self.register_stack[position] = register;
        }

        let operand_count = operands.len();
        let mut scratch_position = operand_count.max(1);
        for register in XMM_STACK {
            if scratch_position == XMM_STACK.len() {
                break;
            }
            if used_register_mask & xmm_mask(register) == 0 {
                self.register_stack[scratch_position] = register;
                scratch_position += 1;
            }
        }
        self.depth = operand_count;
        Ok(result_register)
    }

    fn materialize_allocated_result(&mut self, result: X64ValueLocation) -> JitResult<()> {
        match result {
            X64ValueLocation::Register(register) => {
                let source = allocated_xmm(register)?;
                if source != Xmm::Xmm0 {
                    self.encoder.movsd_xmm_xmm(Xmm::Xmm0, source);
                }
            }
            X64ValueLocation::Spill(slot) => {
                self.encoder.movsd_xmm_m64_base_disp32(
                    Xmm::Xmm0,
                    Gpr::Rsp,
                    self.expression_spill_disp(slot)?,
                );
            }
        }
        self.register_stack = XMM_STACK;
        self.depth = 1;
        self.spilled_depth = 0;
        Ok(())
    }

    fn reset_expression_state(&mut self) {
        self.depth = 0;
        self.spilled_depth = 0;
    }

    fn register_stack_slot(&self, register: Xmm) -> usize {
        self.register_stack[..self.depth]
            .iter()
            .position(|candidate| *candidate == register)
            .expect("active register belongs to the allocated operand stack")
    }

    fn emit_image_entry_call(
        &mut self,
        function_image_offset: usize,
        target: crate::native::model::CodeOffset,
    ) -> JitResult<()> {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(entry_ctx_arg_reg(), saved_ctx_arg_reg());
        self.encoder
            .mov_r64_r64(entry_vars_arg_reg(), saved_vars_arg_reg());
        let call_patch = self.encoder.call_rel32_placeholder();
        let next_instruction = function_image_offset
            .checked_add(call_patch.next_instruction_offset())
            .ok_or_else(|| JitError::Relocation {
                model: MODEL.into(),
                detail: "fused kernel call address overflow".into(),
            })?;
        let displacement = i64::try_from(target.as_usize())
            .ok()
            .and_then(|target| {
                i64::try_from(next_instruction)
                    .ok()
                    .and_then(|next| target.checked_sub(next))
            })
            .and_then(|displacement| i32::try_from(displacement).ok())
            .ok_or_else(|| JitError::Relocation {
                model: MODEL.into(),
                detail: "fused kernel call target is outside the x64 rel32 range".into(),
            })?;
        self.encoder.patch_rel32(call_patch, displacement);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_kernel_abort_if_failed(&mut self) -> JitResult<()> {
        let failed_offset = std::mem::offset_of!(EvalContext, runtime_status)
            .checked_add(NativeRuntimeStatus::failed_offset())
            .and_then(|offset| i32::try_from(offset).ok())
            .ok_or_else(|| JitError::Encoding {
                model: MODEL.into(),
                detail: "native runtime status offset exceeds x64 disp32 range".into(),
            })?;
        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::Rax, saved_ctx_arg_reg(), failed_offset);
        self.encoder.test_r8_r8(Gpr::Rax, Gpr::Rax);
        let abort = self.encoder.jcc_rel32_placeholder(ConditionCode::NotEqual);
        self.early_return_jumps.push(abort);
        Ok(())
    }

    fn emit_kernel_skip_if_inactive(&mut self, stamp_index: usize) -> JitResult<Rel32Patch> {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, Gpr::R14, KERNEL_ACTIVE_OFFSET);
        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::Rax, Gpr::R10, byte_disp_u8(stamp_index)?);
        self.encoder.test_r8_r8(Gpr::Rax, Gpr::Rax);
        Ok(self.encoder.jcc_rel32_placeholder(ConditionCode::Equal))
    }

    fn emit_kernel_non_finite_guard(&mut self, stamp_index: usize) -> JitResult<()> {
        const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;

        self.encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
        self.encoder.movabs_r64_imm64(Gpr::R11, EXPONENT_MASK);
        self.encoder.and_r64_r64(Gpr::R10, Gpr::R11);
        self.encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
        let finite = self.encoder.jcc_rel32_placeholder(ConditionCode::NotEqual);

        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(entry_ctx_arg_reg(), saved_ctx_arg_reg());
        self.emit_usize_arg(entry_vars_arg_reg(), stamp_index);
        self.encoder.movabs_r64_imm64(
            Gpr::R11,
            rspice_native_non_finite_contribution_error as *const () as usize as u64,
        );
        self.encoder.call_r64(Gpr::R11);
        self.encoder.add_rsp_imm32(frame_bytes);
        let abort = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(abort);
        self.patch_rel32_to_current(finite)
    }

    fn emit_kernel_stamp_value_store(
        &mut self,
        stamp_index: usize,
        current_pair: Option<(usize, usize)>,
    ) -> JitResult<()> {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R11, saved_ctx_arg_reg(), CURRENTS_OFFSET);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, byte_disp(stamp_index)?, Xmm::Xmm0);

        if let Some((forward, reverse)) = current_pair {
            self.encoder.mov_r64_m64_base_disp32(
                Gpr::R11,
                saved_ctx_arg_reg(),
                BRANCH_CURRENTS_OFFSET,
            );
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::R11, byte_disp(forward)?, Xmm::Xmm0);
            if forward != reverse {
                self.encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
                self.encoder.btc_r64_imm8(Gpr::R10, 63);
                self.encoder.movq_xmm_r64(Xmm::Xmm1, Gpr::R10);
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, byte_disp(reverse)?, Xmm::Xmm1);
            }
        }
        Ok(())
    }

    fn emit_kernel_jacobian_store(&mut self, jacobian_index: usize) -> JitResult<()> {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R11, Gpr::R14, KERNEL_JACOBIANS_OFFSET);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, byte_disp(jacobian_index)?, Xmm::Xmm0);
        Ok(())
    }

    fn finish_value_function(mut self) -> JitResult<X64FunctionBody> {
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_function(mut self, var_index: usize) -> JitResult<X64FunctionBody> {
        self.emit_assignment_store(var_index)?;
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_pass_function(mut self) -> JitResult<X64FunctionBody> {
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn emit_prologue(&mut self) -> JitResult<()> {
        if self.uses_frame_pointer {
            self.encoder.push_r64(Gpr::Rbp);
            self.record_windows_unwind_push(5);
            // Preserve the pre-call stack parity expected by the existing
            // temporary helper/spill frames while giving the unwinder a
            // stable nonvolatile anchor.
            self.encoder.push_r64(Gpr::R14);
            self.record_windows_unwind_push(14);
        }
        if self.saves_entry_args() {
            self.encoder.push_r64(Gpr::R12);
            self.record_windows_unwind_push(12);
            self.encoder.push_r64(Gpr::R13);
            self.record_windows_unwind_push(13);
            self.encoder
                .mov_r64_r64(saved_ctx_arg_reg(), entry_ctx_arg_reg());
            self.encoder
                .mov_r64_r64(saved_vars_arg_reg(), entry_vars_arg_reg());
        }
        if self.saves_kernel_io {
            self.encoder
                .mov_r64_r64(Gpr::R14, entry_kernel_io_arg_reg());
        }
        if self.local_frame_bytes > 0 {
            self.emit_stack_allocation()?;
        }
        self.emit_callee_saved_xmm_stores();
        if self.uses_frame_pointer {
            self.encoder.mov_r64_r64(Gpr::Rbp, Gpr::Rsp);
            self.record_windows_unwind_set_frame_pointer();
        }
        self.windows_unwind_prologue_size = self.current_windows_unwind_code_offset();
        Ok(())
    }

    /// Probe every page crossed by a generated frame before committing RSP to
    /// the allocation.  RAX, R10, and R11 are volatile under both supported
    /// x64 ABIs, so this sequence cannot disturb entry arguments or nonvolatile
    /// caller state.
    fn emit_stack_allocation(&mut self) -> JitResult<()> {
        let frame_bytes = self.local_frame_bytes;
        debug_assert!(frame_bytes > 0);

        if frame_bytes >= STACK_PROBE_INTERVAL_BYTES {
            self.encoder.mov_r64_r64(Gpr::R10, Gpr::Rsp);
            self.encoder
                .mov_r32_imm32(Gpr::R11, u32::try_from(frame_bytes).map_err(|_| {
                    JitError::RegisterAllocation {
                        model: MODEL.into(),
                        detail: format!(
                            "generated local frame size {frame_bytes} cannot be represented by the x64 stack prober"
                        )
                        .into(),
                    }
                })?);

            let probe_loop = self.encoder.position();
            self.encoder
                .cmp_r64_imm32(Gpr::R11, STACK_PROBE_INTERVAL_BYTES);
            let final_probe = self
                .encoder
                .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

            self.encoder
                .sub_r64_imm32(Gpr::R10, STACK_PROBE_INTERVAL_BYTES);
            self.encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::R10, 0);
            self.encoder
                .sub_r64_imm32(Gpr::R11, STACK_PROBE_INTERVAL_BYTES);
            self.emit_jmp_to_offset(probe_loop)?;

            self.patch_rel32_to_current(final_probe)?;
            self.encoder.sub_r64_r64(Gpr::R10, Gpr::R11);
            self.encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::R10, 0);
        }

        self.encoder.sub_rsp_imm32(frame_bytes);
        self.record_windows_unwind_stack_allocation(frame_bytes as u32);
        Ok(())
    }

    fn emit_return(&mut self) {
        self.emit_callee_saved_xmm_loads();
        if self.local_frame_bytes > 0 {
            self.encoder.add_rsp_imm32(self.local_frame_bytes);
        }
        if self.saves_entry_args() {
            self.encoder.pop_r64(Gpr::R13);
            self.encoder.pop_r64(Gpr::R12);
        }
        if self.uses_frame_pointer {
            self.encoder.pop_r64(Gpr::R14);
            self.encoder.pop_r64(Gpr::Rbp);
        }
        self.encoder.ret();
    }

    fn emit_callee_saved_xmm_stores(&mut self) {
        for index in 0..self.callee_saved_xmm_count {
            let register = callee_saved_xmm_register(index);
            let disp = self.callee_saved_xmm_disp(index);
            self.encoder
                .movdqu_m128_base_disp32_xmm(Gpr::Rsp, disp, register);
            self.record_windows_unwind_xmm_save(xmm_unwind_register(register), disp as u32);
        }
    }

    fn windows_unwind_info(&self) -> Option<WindowsX64UnwindInfo> {
        #[cfg(windows)]
        {
            (!self.windows_unwind_operations.is_empty()).then(|| WindowsX64UnwindInfo {
                prologue_size: self.windows_unwind_prologue_size,
                frame_register: if self.uses_frame_pointer { 5 } else { 0 },
                operations: self.windows_unwind_operations.clone(),
            })
        }
        #[cfg(not(windows))]
        {
            None
        }
    }

    fn current_windows_unwind_code_offset(&self) -> u8 {
        let offset = self.encoder.position();
        debug_assert!(
            u8::try_from(offset).is_ok(),
            "fixed native x64 prologue must fit the Windows unwind u8 offset"
        );
        offset as u8
    }

    fn record_windows_unwind_push(&mut self, register: u8) {
        #[cfg(windows)]
        self.windows_unwind_operations
            .push(WindowsX64UnwindOperation::PushNonvolatile {
                code_offset: self.current_windows_unwind_code_offset(),
                register,
            });
        #[cfg(not(windows))]
        let _ = register;
    }

    fn record_windows_unwind_stack_allocation(&mut self, size: u32) {
        #[cfg(windows)]
        self.windows_unwind_operations
            .push(WindowsX64UnwindOperation::AllocateStack {
                code_offset: self.current_windows_unwind_code_offset(),
                size,
            });
        #[cfg(not(windows))]
        let _ = size;
    }

    fn record_windows_unwind_xmm_save(&mut self, register: u8, stack_offset: u32) {
        #[cfg(windows)]
        self.windows_unwind_operations
            .push(WindowsX64UnwindOperation::SaveXmm128 {
                code_offset: self.current_windows_unwind_code_offset(),
                register,
                stack_offset,
            });
        #[cfg(not(windows))]
        let _ = (register, stack_offset);
    }

    fn record_windows_unwind_set_frame_pointer(&mut self) {
        #[cfg(windows)]
        self.windows_unwind_operations
            .push(WindowsX64UnwindOperation::SetFramePointer {
                code_offset: self.current_windows_unwind_code_offset(),
            });
    }

    fn emit_callee_saved_xmm_loads(&mut self) {
        for index in (0..self.callee_saved_xmm_count).rev() {
            let register = callee_saved_xmm_register(index);
            let disp = self.callee_saved_xmm_disp(index);
            self.encoder
                .movdqu_xmm_m128_base_disp32(register, Gpr::Rsp, disp);
        }
    }

    fn callee_saved_xmm_disp(&self, index: usize) -> i32 {
        debug_assert!(index < self.callee_saved_xmm_count);
        self.local_frame_bytes - callee_saved_xmm_frame_bytes(self.callee_saved_xmm_count)
            + (index as i32 * CALLEE_SAVED_XMM_BYTES)
    }

    fn ctx_arg_reg(&self) -> Gpr {
        if self.saves_entry_args() {
            saved_ctx_arg_reg()
        } else {
            entry_ctx_arg_reg()
        }
    }

    fn vars_arg_reg(&self) -> Gpr {
        if self.saves_entry_args() {
            saved_vars_arg_reg()
        } else {
            entry_vars_arg_reg()
        }
    }

    fn emit_assignment_store(&mut self, var_index: usize) -> JitResult<()> {
        if self.logical_depth() != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "assignment expression stack depth {}, expected 1",
                    self.logical_depth()
                )
                .into(),
            });
        }
        self.encoder.movsd_m64_base_disp32_xmm(
            self.vars_arg_reg(),
            byte_disp(var_index)?,
            Xmm::Xmm0,
        );
        self.depth = 0;
        self.spilled_depth = 0;
        Ok(())
    }

    fn emit_assignment_location_store(
        &mut self,
        location: X64ValueLocation,
        var_index: usize,
    ) -> JitResult<()> {
        match location {
            X64ValueLocation::Register(register) => {
                self.encoder.movsd_m64_base_disp32_xmm(
                    self.vars_arg_reg(),
                    byte_disp(var_index)?,
                    allocated_xmm(register)?,
                );
            }
            X64ValueLocation::Spill(slot) => {
                self.encoder.mov_r64_m64_base_disp32(
                    Gpr::R10,
                    Gpr::Rsp,
                    self.expression_spill_disp(slot)?,
                );
                self.encoder.mov_m64_base_disp32_r64(
                    self.vars_arg_reg(),
                    byte_disp(var_index)?,
                    Gpr::R10,
                );
            }
        }
        Ok(())
    }

    fn emit_assignment_step(
        &mut self,
        assignment: &NativeAssignment,
        loop_depth: i32,
    ) -> JitResult<()> {
        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                self.emit_native_program(program)?;
                self.emit_assignment_store(*var_index)
            }
            NativeAssignment::Indexed {
                base,
                len,
                lower,
                index,
                value,
            } => self.emit_indexed_assignment(*base, *len, *lower, index, value),
            NativeAssignment::Loop { condition, body } => {
                self.emit_loop_assignment(condition, body, loop_depth)
            }
        }
    }

    fn emit_assignment_steps(
        &mut self,
        assignments: &[NativeAssignment],
        loop_depth: i32,
    ) -> JitResult<()> {
        for range in shareable_batch_ranges(assignments) {
            let batch = &assignments[range];
            if matches!(batch.first(), Some(NativeAssignment::Direct { .. })) {
                self.emit_direct_assignment_batch(batch)?;
            } else {
                debug_assert_eq!(batch.len(), 1);
                self.emit_assignment_step(&batch[0], loop_depth)?;
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
                    Err(JitError::Verifier {
                        model: MODEL.into(),
                        detail: "x64 direct-assignment batch contains a control-flow assignment"
                            .into(),
                    })
                }
            })
            .collect::<JitResult<Vec<_>>>()?;
        let assignment = AssignmentProgram::lower(&direct)?;
        let allocation = RegisterAllocation::build_for_assignments(&assignment, X64_VALUE_BANK)?;
        self.emit_allocated_program_body(assignment.program(), &allocation, Some(&assignment))
    }

    fn emit_loop_assignment(
        &mut self,
        condition: &NativeProgram,
        body: &[NativeAssignment],
        loop_depth: i32,
    ) -> JitResult<()> {
        let counter_disp = self.loop_counter_disp(loop_depth)?;
        self.encoder.xor_r64_r64(Gpr::R10, Gpr::R10);
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, counter_disp, Gpr::R10);

        let loop_start = self.encoder.position();
        self.emit_native_program(condition)?;
        let loop_exit = self.emit_loop_exit_if_zero()?;

        self.emit_assignment_steps(body, loop_depth + 1)?;

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, Gpr::Rsp, counter_disp);
        self.encoder.add_r64_imm32(Gpr::R10, 1);
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, counter_disp, Gpr::R10);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, MAX_RUNTIME_LOOP_ITERATIONS);
        let limit_reached = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);
        self.emit_jmp_to_offset(loop_start)?;
        self.patch_rel32_to_current(limit_reached)?;
        self.emit_runtime_loop_limit_error_call();
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
        self.patch_rel32_to_current(loop_exit)?;
        Ok(())
    }

    fn loop_counter_disp(&self, loop_depth: i32) -> JitResult<i32> {
        let Some(base_disp) = self.loop_counter_base_disp else {
            return Err(JitError::InternalCompilerError {
                model: MODEL.into(),
                detail: "loop assignment emitted without loop-counter frame slot".into(),
            });
        };
        Ok(base_disp + loop_depth * LOCAL_SLOT_BYTES)
    }

    fn emit_loop_exit_if_zero(&mut self) -> JitResult<Rel32Patch> {
        if self.logical_depth() != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "loop condition stack depth {}, expected 1",
                    self.logical_depth()
                )
                .into(),
            });
        }

        self.encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
        self.encoder.btr_r64_imm8(Gpr::R10, 63);
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        let loop_exit = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.depth = 0;
        self.spilled_depth = 0;
        Ok(loop_exit)
    }

    fn emit_indexed_assignment(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
        index: &NativeProgram,
        value: &NativeProgram,
    ) -> JitResult<()> {
        debug_assert!(self.local_frame_bytes >= LOCAL_SLOT_BYTES);

        self.emit_native_program(index)?;
        if dynamic_variable_inline_supported(len, lower) {
            self.emit_dynamic_variable_slot_inline(base, len, lower)?;
        } else {
            self.emit_dynamic_variable_slot_call(base, len, lower)?;
            self.encoder.mov_m64_base_disp32_r64(
                Gpr::Rsp,
                INDEXED_ASSIGNMENT_SLOT_PTR_DISP,
                Gpr::Rax,
            );
        }

        self.emit_native_program(value)?;
        if self.logical_depth() != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "indexed assignment value stack depth {}, expected 1",
                    self.logical_depth()
                )
                .into(),
            });
        }
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, 0, Xmm::Xmm0);
        self.depth = 0;
        self.spilled_depth = 0;
        Ok(())
    }

    fn push_register(&mut self) -> JitResult<Xmm> {
        if self.depth >= XMM_STACK.len() {
            let spill_disp = self.expression_spill_disp(self.spilled_depth)?;
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, spill_disp, self.register_stack[0]);
            for index in 0..XMM_STACK.len() - 1 {
                self.encoder
                    .movsd_xmm_xmm(self.register_stack[index], self.register_stack[index + 1]);
            }
            self.depth -= 1;
            self.spilled_depth += 1;
        }

        let register = self.register_stack[self.depth];
        self.depth += 1;
        Ok(register)
    }

    fn logical_depth(&self) -> usize {
        self.spilled_depth + self.depth
    }

    fn expression_spill_disp(&self, index: usize) -> JitResult<i32> {
        let base = self.expression_spill_base_disp.ok_or_else(|| {
            register_allocation_error(
                "expression spill requested without reserved local-frame storage".to_string(),
            )
        })?;
        index
            .checked_mul(WORD_BYTES)
            .and_then(|offset| i32::try_from(offset).ok())
            .and_then(|offset| base.checked_add(offset))
            .ok_or_else(|| {
                register_allocation_error(
                    "expression spill slot exceeds the x64 frame displacement range".to_string(),
                )
            })
    }

    /// Drops values already consumed by an operation, then refills the bottom
    /// of the register window from the fixed spill area. The top of the
    /// logical expression stack therefore remains in XMM registers.
    fn drop_stack_values(&mut self, count: usize) -> JitResult<()> {
        if count > self.depth {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "expression stack drop {count} exceeds resident depth {}",
                    self.depth
                )
                .into(),
            });
        }
        self.depth -= count;
        while self.spilled_depth > 0 && self.depth < XMM_STACK.len() {
            for index in (0..self.depth).rev() {
                self.encoder
                    .movsd_xmm_xmm(self.register_stack[index + 1], self.register_stack[index]);
            }
            let spill_index = self.spilled_depth - 1;
            let spill_disp = self.expression_spill_disp(spill_index)?;
            self.encoder
                .movsd_xmm_m64_base_disp32(self.register_stack[0], Gpr::Rsp, spill_disp);
            self.spilled_depth -= 1;
            self.depth += 1;
        }
        Ok(())
    }

    fn scratch_register(&self) -> JitResult<Xmm> {
        if self.depth >= XMM_STACK.len() {
            return Err(register_allocation_error(
                "operation requires a scratch XMM register but all are live".to_string(),
            ));
        }

        Ok(self.register_stack[self.depth])
    }

    fn emit_binary_op(&mut self, op: BinaryOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("binary op requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        let right = self.register_stack[self.depth - 1];
        match op {
            BinaryOp::Add => self.encoder.addsd_xmm_xmm(left, right),
            BinaryOp::Sub => self.encoder.subsd_xmm_xmm(left, right),
            BinaryOp::Mul => self.encoder.mulsd_xmm_xmm(left, right),
            BinaryOp::Div => self.encoder.divsd_xmm_xmm(left, right),
        }
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_literal_rhs_binary_op(&mut self, value: f64, op: BinaryOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS binary op requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_literal_binary_op(target, value, op);
        Ok(())
    }

    fn emit_literal_lhs_binary_op(&mut self, value: f64, op: BinaryOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal LHS binary op requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        match op {
            BinaryOp::Sub | BinaryOp::Div => {
                if self.depth < XMM_STACK.len() {
                    let scratch = self.scratch_register()?;
                    self.emit_literal_lhs_scratch_binary_op(target, scratch, value, op);
                } else {
                    self.emit_literal_lhs_stack_binary_op(target, value, op);
                }
            }
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS binary lowering only accepts sub/div")
            }
        }
        Ok(())
    }

    fn emit_literal_lhs_scratch_binary_op(
        &mut self,
        target: Xmm,
        scratch: Xmm,
        value: f64,
        op: BinaryOp,
    ) {
        self.emit_literal_load(scratch, value);
        match op {
            BinaryOp::Sub => self.encoder.subsd_xmm_xmm(scratch, target),
            BinaryOp::Div => self.encoder.divsd_xmm_xmm(scratch, target),
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS scratch lowering only accepts sub/div")
            }
        }
        self.encoder.movsd_xmm_xmm(target, scratch);
    }

    fn emit_literal_lhs_stack_binary_op(&mut self, target: Xmm, value: f64, op: BinaryOp) {
        self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
        self.emit_literal_load(target, value);
        match op {
            BinaryOp::Sub => self.encoder.subsd_xmm_m64_base_disp32(target, Gpr::Rsp, 0),
            BinaryOp::Div => self.encoder.divsd_xmm_m64_base_disp32(target, Gpr::Rsp, 0),
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS stack lowering only accepts sub/div")
            }
        }
        self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
    }

    fn emit_neg(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "neg requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        let displacement_offset = self.encoder.xorpd_xmm_m128_rip_disp32(target, 0);
        self.vector_literals.push(VectorLiteralPatch {
            displacement_offset,
            lanes: [NEG_VALUE_MASK_LOW, NEG_VALUE_MASK_HIGH],
        });
        Ok(())
    }

    fn emit_abs(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "abs requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_abs_register(target);
        Ok(())
    }

    fn emit_square(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "square requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.encoder.mulsd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_abs_register(&mut self, target: Xmm) {
        let displacement_offset = self.encoder.andpd_xmm_m128_rip_disp32(target, 0);
        self.vector_literals.push(VectorLiteralPatch {
            displacement_offset,
            lanes: [ABS_VALUE_MASK_LOW, ABS_VALUE_MASK_HIGH],
        });
    }

    fn emit_sqrt(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "sqrt requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.encoder.sqrtsd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_unary_math(&mut self, op: UnaryMathOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "unary math requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        match op {
            UnaryMathOp::Floor => self.emit_floor_or_ceil(target, RoundDirection::Floor),
            UnaryMathOp::Ceil => self.emit_floor_or_ceil(target, RoundDirection::Ceil),
            UnaryMathOp::Limexp => self.emit_limexp(target),
            UnaryMathOp::LimitedExp => self.emit_limited_exp(target),
            _ => {
                self.emit_unary_helper_call(target, unary_math_helper(op));
                Ok(())
            }
        }
    }

    fn emit_limexp(&mut self, target: Xmm) -> JitResult<()> {
        self.emit_literal_compare(target, 40.0);
        let high_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);

        self.emit_literal_compare(target, -40.0);
        let exp_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        let low_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Below);

        self.patch_rel32_to_current(exp_jump)?;
        self.emit_unary_helper_call(target, rspice_exp);
        let exp_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(low_jump)?;
        self.emit_literal_load(target, (-40.0_f64).exp());
        let low_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(high_jump)?;
        self.emit_literal_binary_op(target, 1.0, BinaryOp::Add);
        self.emit_literal_binary_op(target, 40.0, BinaryOp::Sub);
        self.emit_literal_binary_op(target, 40.0_f64.exp(), BinaryOp::Mul);

        self.patch_rel32_to_current(exp_done)?;
        self.patch_rel32_to_current(low_done)?;
        Ok(())
    }

    fn emit_limited_exp(&mut self, target: Xmm) -> JitResult<()> {
        self.emit_literal_compare(target, 80.0);
        let high_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);

        self.emit_literal_compare(target, -80.0);
        let exp_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        let low_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Below);

        self.patch_rel32_to_current(exp_jump)?;
        self.emit_unary_helper_call(target, rspice_exp);
        let exp_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(low_jump)?;
        self.emit_literal_load(target, 1.804851387e-35);
        let low_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(high_jump)?;
        self.emit_literal_binary_op(target, 1.0, BinaryOp::Add);
        self.emit_literal_binary_op(target, 80.0, BinaryOp::Sub);
        self.emit_literal_binary_op(target, 80.0_f64.exp(), BinaryOp::Mul);

        self.patch_rel32_to_current(exp_done)?;
        self.patch_rel32_to_current(low_done)?;
        Ok(())
    }

    fn emit_floor_or_ceil(&mut self, target: Xmm, direction: RoundDirection) -> JitResult<()> {
        self.encoder.movq_r64_xmm(Gpr::R11, target);
        self.encoder.btr_r64_imm8(Gpr::R11, 63);
        self.encoder.cmp_r64_imm32(Gpr::R11, 0);
        let zero = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movabs_r64_imm64(Gpr::R10, F64_EXACT_INTEGER_LIMIT_ABS_BITS);
        self.encoder.cmp_r64_r64(Gpr::R11, Gpr::R10);
        let already_integral_or_unordered = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);

        if self.depth < XMM_STACK.len() {
            let original = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(original, target);
            self.emit_floor_or_ceil_adjust(target, direction, |compiler, target| {
                compiler.encoder.ucomisd_xmm_xmm(target, original);
            })?;
        } else {
            self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
            self.emit_floor_or_ceil_adjust(target, direction, |compiler, target| {
                compiler
                    .encoder
                    .ucomisd_xmm_m64_base_disp32(target, Gpr::Rsp, 0);
            })?;
            self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        }
        self.patch_rel32_to_current(zero)?;
        self.patch_rel32_to_current(already_integral_or_unordered)?;
        Ok(())
    }

    fn emit_floor_or_ceil_adjust(
        &mut self,
        target: Xmm,
        direction: RoundDirection,
        compare_original: impl FnOnce(&mut Self, Xmm),
    ) -> JitResult<()> {
        self.encoder.cvttsd2si_r64_xmm(Gpr::R10, target);
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        compare_original(self, target);

        let skip_adjust = match direction {
            RoundDirection::Floor => self
                .encoder
                .jcc_rel32_placeholder(ConditionCode::BelowOrEqual),
            RoundDirection::Ceil => self
                .encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        };
        let adjustment = match direction {
            RoundDirection::Floor => -1,
            RoundDirection::Ceil => 1,
        };
        self.encoder.add_r64_imm32(Gpr::R10, adjustment);
        self.patch_rel32_to_current(skip_adjust)?;
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        Ok(())
    }

    fn emit_unary_helper_call(&mut self, target: Xmm, helper: UnaryHelper) {
        debug_assert!(self.uses_helper_calls);
        let frame_bytes = call_frame_bytes_for_slots(call_frame_spill_slot_count(
            &self.register_stack[..self.depth],
            |_, register| register != target,
        ));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_helper_result_to_target_and_restore(
        &mut self,
        target: Xmm,
        restore_depth: usize,
        mut should_restore: impl FnMut(usize, Xmm) -> bool,
    ) {
        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(target, Xmm::Xmm0);
        }
        for (index, register) in self
            .register_stack
            .iter()
            .copied()
            .take(restore_depth)
            .enumerate()
        {
            if register != target && should_restore(index, register) {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::Rsp, call_spill_disp(index));
            }
        }
    }

    fn emit_call_frame_spills(
        &mut self,
        spill_depth: usize,
        mut should_spill: impl FnMut(usize, Xmm) -> bool,
    ) {
        for (index, register) in self
            .register_stack
            .iter()
            .copied()
            .take(spill_depth)
            .enumerate()
        {
            if should_spill(index, register) {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::Rsp, call_spill_disp(index), register);
            }
        }
    }

    fn emit_dynamic_variable_load(&mut self, base: usize, len: usize, lower: i64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "dynamic variable load requires stack depth 1, found 0".into(),
            });
        }

        if !dynamic_variable_inline_supported(len, lower) {
            let target = self.register_stack[self.depth - 1];
            return self.emit_dynamic_variable_helper_call(target, base, len, lower);
        }

        let target = self.register_stack[self.depth - 1];
        let base_disp = byte_disp(base)?;

        if self.depth < XMM_STACK.len() {
            let raw_index = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(raw_index, target);
            let slow_jumps =
                self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
            self.encoder.movsd_xmm_m64_base_disp32(target, Gpr::R11, 0);
            let fast_done = self.encoder.jmp_rel32_placeholder();

            for slow_jump in slow_jumps {
                self.patch_rel32_to_current(slow_jump)?;
            }
            self.emit_dynamic_variable_load_slow_return_from_register(raw_index, len, lower);
            self.patch_rel32_to_current(fast_done)?;
            return Ok(());
        }

        self.encoder.sub_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);

        let slow_jumps =
            self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
        self.encoder.movsd_xmm_m64_base_disp32(target, Gpr::R11, 0);
        self.encoder.add_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        let fast_done = self.encoder.jmp_rel32_placeholder();

        for slow_jump in slow_jumps {
            self.patch_rel32_to_current(slow_jump)?;
        }
        self.emit_dynamic_variable_load_slow_return(len, lower);
        self.patch_rel32_to_current(fast_done)?;
        Ok(())
    }

    fn emit_dynamic_variable_load_slow_return(&mut self, len: usize, lower: i64) {
        self.encoder
            .movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rsp, 0);
        self.emit_dynamic_variable_error_call_from_xmm0(len, lower);
        self.encoder.add_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_load_slow_return_from_register(
        &mut self,
        raw_index: Xmm,
        len: usize,
        lower: i64,
    ) {
        if raw_index != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, raw_index);
        }
        self.emit_dynamic_variable_error_call_from_xmm0(len, lower);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_error_call_from_xmm0(&mut self, len: usize, lower: i64) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(dynamic_variable_ptr_arg_reg(), self.ctx_arg_reg());
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableErrorHelper = rspice_native_dynamic_variable_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_dynamic_variable_slot_inline(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "dynamic variable slot inline path requires stack depth 1, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        let base_disp = byte_disp(base)?;
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP, target);

        let slow_jumps =
            self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP, Gpr::R11);
        let fast_done = self.encoder.jmp_rel32_placeholder();

        for slow_jump in slow_jumps {
            self.patch_rel32_to_current(slow_jump)?;
        }
        self.emit_dynamic_variable_slot_slow_return(len, lower);
        self.patch_rel32_to_current(fast_done)?;
        self.depth = 0;
        self.spilled_depth = 0;
        Ok(())
    }

    fn emit_dynamic_variable_slot_slow_return(&mut self, len: usize, lower: i64) {
        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::Rsp,
            INDEXED_ASSIGNMENT_SLOT_PTR_DISP,
        );
        self.emit_dynamic_variable_error_call_from_xmm0(len, lower);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_address_inline(
        &mut self,
        index: Xmm,
        base_disp: i32,
        len: usize,
        lower: i64,
    ) -> JitResult<Vec<Rel32Patch>> {
        let mut slow_jumps = Vec::new();

        self.encoder.movq_r64_xmm(Gpr::R8, index);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::R8);
        self.encoder.btr_r64_imm8(Gpr::R11, 63);
        self.encoder
            .movabs_r64_imm64(Gpr::R10, F64_EXACT_INTEGER_LIMIT_ABS_BITS);
        self.encoder.cmp_r64_r64(Gpr::R11, Gpr::R10);
        slow_jumps.push(
            self.encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        );

        self.encoder.test_r64_r64(Gpr::R8, Gpr::R8);
        let non_negative = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::NotNegative);
        self.emit_literal_binary_op(index, 0.5, BinaryOp::Sub);
        let rounded = self.encoder.jmp_rel32_placeholder();
        self.patch_rel32_to_current(non_negative)?;
        self.emit_literal_binary_op(index, 0.5, BinaryOp::Add);
        self.patch_rel32_to_current(rounded)?;

        self.encoder.cvttsd2si_r64_xmm(Gpr::R10, index);
        self.emit_i64_subtract(Gpr::R10, lower);
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        slow_jumps.push(self.encoder.jcc_rel32_placeholder(ConditionCode::Negative));
        self.emit_usize_compare(Gpr::R10, len);
        slow_jumps.push(
            self.encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        );

        self.encoder.lea_r64_base_index_scale8_disp32(
            Gpr::R11,
            self.vars_arg_reg(),
            Gpr::R10,
            base_disp,
        );
        Ok(slow_jumps)
    }

    fn emit_dynamic_variable_helper_call(
        &mut self,
        target: Xmm,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        debug_assert!(self.uses_helper_calls);
        debug_assert!(self.register_stack_slot(target) < self.depth);
        let base_disp = byte_disp(base)?;
        let context_slot = self.depth;

        let frame_bytes = call_frame_bytes_for_slots(context_slot + 1);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, _| true);
        self.encoder.mov_m64_base_disp32_r64(
            Gpr::Rsp,
            call_spill_disp(context_slot),
            self.ctx_arg_reg(),
        );

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(dynamic_variable_ptr_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_ptr_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableSlotHelper = rspice_dynamic_variable_slot_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let invalid_slot = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder.movsd_xmm_m64_base_disp32(target, Gpr::Rax, 0);
        for (index, register) in self
            .register_stack
            .iter()
            .copied()
            .take(self.depth)
            .enumerate()
        {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::Rsp, call_spill_disp(index));
            }
        }
        self.encoder.add_rsp_imm32(frame_bytes);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(invalid_slot)?;
        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::Rsp,
            call_spill_disp(self.register_stack_slot(target)),
        );
        self.encoder.mov_r64_m64_base_disp32(
            dynamic_variable_ptr_arg_reg(),
            Gpr::Rsp,
            call_spill_disp(context_slot),
        );
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let error_helper: DynamicVariableErrorHelper = rspice_native_dynamic_variable_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, error_helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);

        self.patch_rel32_to_current(done)?;
        Ok(())
    }

    fn emit_dynamic_variable_slot_call(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "dynamic variable slot helper requires stack depth 1, found {}",
                    self.depth
                )
                .into(),
            });
        }

        debug_assert!(self.uses_helper_calls);
        let base_disp = byte_disp(base)?;
        let target = self.register_stack[0];

        let frame_bytes = call_frame_bytes_for_slots(1);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rsp, call_spill_disp(0), target);
        self.encoder
            .mov_r64_r64(dynamic_variable_ptr_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_ptr_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableSlotHelper = rspice_dynamic_variable_slot_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let invalid_slot = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder.add_rsp_imm32(frame_bytes);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(invalid_slot)?;
        self.encoder
            .movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rsp, call_spill_disp(0));
        self.encoder
            .mov_r64_r64(dynamic_variable_ptr_arg_reg(), self.ctx_arg_reg());
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let error_helper: DynamicVariableErrorHelper = rspice_native_dynamic_variable_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, error_helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);

        self.patch_rel32_to_current(done)?;
        self.depth = 0;
        self.spilled_depth = 0;
        Ok(())
    }

    fn emit_runtime_loop_limit_error_call(&mut self) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(entry_ctx_arg_reg(), self.ctx_arg_reg());
        let helper: VoidHelper = rspice_native_loop_limit_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_binary_math(&mut self, op: BinaryMathOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("binary math requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        let right = self.register_stack[self.depth - 1];
        self.emit_binary_helper_call(left, right, binary_math_helper(op));
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_integer_binary(&mut self, op: IntegerBinaryOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "integer binary op requires stack depth 2, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            left,
            2,
            integer_binary_descriptor(runtime_integer_operation(op)),
            rspice_integer_operation_native,
        );
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_integer_cast(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "integer cast requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(
            target,
            1,
            INTEGER_CAST_DESCRIPTOR,
            rspice_integer_operation_native,
        );
        Ok(())
    }

    fn emit_integer_shift_const(&mut self, op: IntegerBinaryOp, count: u8) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "constant integer shift requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(
            target,
            1,
            integer_shift_const_descriptor(runtime_integer_operation(op), count),
            rspice_integer_operation_native,
        );
        Ok(())
    }

    fn emit_integer_bitwise_const(&mut self, op: IntegerBinaryOp, value: i64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "constant integer bitwise op requires stack depth 1, found 0".into(),
            });
        }

        let descriptor = integer_binary_const_descriptor(runtime_integer_operation(op), value)
            .ok_or_else(|| JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "constant Verilog-AMS integer operand {value} is outside signed 32-bit range"
                )
                .into(),
            })?;
        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(
            target,
            1,
            descriptor,
            rspice_integer_operation_native,
        );
        Ok(())
    }

    fn emit_current_load(
        &mut self,
        pair_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_f64_slice_load(
            BRANCH_CURRENTS_OFFSET,
            BRANCH_CURRENTS_LEN_OFFSET,
            pair_index,
            rspice_native_current_probe_error,
            context_pointer_cache,
        )
    }

    fn emit_prior_current_load(
        &mut self,
        current_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_f64_slice_load(
            CURRENTS_OFFSET,
            CURRENTS_LEN_OFFSET,
            current_index,
            rspice_native_prior_current_error,
            context_pointer_cache,
        )
    }

    fn emit_guarded_context_f64_slice_load(
        &mut self,
        pointer_field_offset: i32,
        len_field_offset: i32,
        index: usize,
        helper: VoidHelper,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;
        let value_disp = byte_disp(index)?;

        self.emit_context_pointer_load_cached(pointer_field_offset, context_pointer_cache);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let missing_storage = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), len_field_offset);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, slice_index_imm32(index)?);
        let index_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, value_disp);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(missing_storage)?;
        self.patch_rel32_to_current(index_out_of_range)?;
        self.emit_void_error_return(helper);

        self.patch_rel32_to_current(done)
    }

    fn emit_table_helper_call(&mut self, table_id: usize, helper: TableHelper) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "table model helper requires stack depth 1, found 0".into(),
            });
        }

        debug_assert!(self.uses_helper_calls);
        let target = self.register_stack[self.depth - 1];
        let frame_bytes = call_frame_bytes_for_slots(call_frame_spill_slot_count(
            &self.register_stack[..self.depth],
            |_, register| register != target,
        ));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(context_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.emit_usize_arg(context_filter_id_arg_reg(), table_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_context_filter_helper_call(
        &mut self,
        target: Xmm,
        filter_id: usize,
        helper: ContextFilterHelper,
    ) -> JitResult<()> {
        debug_assert!(self.uses_helper_calls);
        debug_assert!(self.register_stack_slot(target) < self.depth);
        let frame_bytes = call_frame_bytes_for_slots(call_frame_spill_slot_count(
            &self.register_stack[..self.depth],
            |_, register| register != target,
        ));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(context_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.emit_usize_arg(context_filter_id_arg_reg(), filter_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_limit_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("limit state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let value = self.register_stack[self.depth - 2];
        let step = self.register_stack[self.depth - 1];
        let state_disp = byte_disp(state_index)?;
        let initialized_disp = byte_disp_u8(state_index)?;
        let state_index_i32 = i32::try_from(state_index).map_err(|_| JitError::Encoding {
            model: MODEL.into(),
            detail: format!("state index {state_index} exceeds x64 imm32 range").into(),
        })?;

        self.emit_context_pointer_load(STATE_VALUES_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let no_state = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R11, self.ctx_arg_reg(), STATE_VALUES_LEN_OFFSET);
        self.encoder.cmp_r64_imm32(Gpr::R11, state_index_i32);
        let state_values_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder.mov_r64_m64_base_disp32(
            Gpr::R10,
            self.ctx_arg_reg(),
            STATE_INITIALIZED_OFFSET,
        );
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        let no_initialized_flags = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder.mov_r64_m64_base_disp32(
            Gpr::R11,
            self.ctx_arg_reg(),
            STATE_INITIALIZED_LEN_OFFSET,
        );
        self.encoder.cmp_r64_imm32(Gpr::R11, state_index_i32);
        let initialized_flags_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R11, Gpr::R10, initialized_disp);
        self.encoder.test_r8_r8(Gpr::R11, Gpr::R11);
        let first_evaluation = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        if self.depth < XMM_STACK.len() {
            let positive_step = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(positive_step, step);
            self.emit_limit_state_clamp_delta(value, step, state_disp, |compiler, value| {
                compiler.encoder.minsd_xmm_xmm(value, positive_step);
            })?;
        } else {
            self.encoder.sub_rsp_imm32(WORD_BYTES as i32);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, step);
            self.emit_limit_state_clamp_delta(value, step, state_disp, |compiler, value| {
                compiler
                    .encoder
                    .minsd_xmm_m64_base_disp32(value, Gpr::Rsp, 0);
            })?;
            self.encoder.add_rsp_imm32(WORD_BYTES as i32);
        }

        self.patch_rel32_to_current(first_evaluation)?;
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, state_disp, value);
        self.encoder
            .mov_m8_base_disp32_imm8(Gpr::R10, initialized_disp, 1);
        let done_after_initialized_store = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(no_initialized_flags)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_initialized_error);

        self.patch_rel32_to_current(initialized_flags_out_of_range)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_bounds_error);

        self.patch_rel32_to_current(state_values_out_of_range)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_values_bounds_error);

        self.patch_rel32_to_current(no_state)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_values_error);

        self.patch_rel32_to_current(done_after_initialized_store)?;
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_limiter_state_helper(
        &mut self,
        state_index: usize,
        helper: ContextFilterHelper,
    ) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "named limiter state helper requires stack depth 1, found 0".into(),
            });
        }
        self.emit_context_filter_helper_call(
            self.register_stack[self.depth - 1],
            state_index,
            helper,
        )
    }

    fn emit_limiter_store(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "named limiter candidate publish requires stack depth 2, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let proposed = self.register_stack[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            proposed,
            2,
            state_index,
            rspice_limiter_store_native,
        );
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_limit_state_clamp_delta(
        &mut self,
        value: Xmm,
        step: Xmm,
        state_disp: i32,
        emit_upper_clamp: impl FnOnce(&mut Self, Xmm),
    ) -> JitResult<()> {
        self.encoder
            .subsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.encoder.ucomisd_xmm_xmm(value, value);
        let unordered_delta = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        self.encoder.movq_r64_xmm(Gpr::R11, step);
        self.encoder.btc_r64_imm8(Gpr::R11, 63);
        self.encoder.movq_xmm_r64(step, Gpr::R11);
        self.encoder.maxsd_xmm_xmm(value, step);
        emit_upper_clamp(self, value);
        self.encoder
            .addsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(unordered_delta)?;
        Ok(())
    }

    fn emit_limit_state_error_return(&mut self, helper: VoidHelper) {
        self.emit_void_error_return(helper);
    }

    fn emit_white_noise(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "white noise requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.encoder.xorpd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_flicker_noise(&mut self) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("flicker noise requires stack depth 2, found {}", self.depth)
                    .into(),
            });
        }

        let target = self.register_stack[self.depth - 2];
        self.encoder.xorpd_xmm_xmm(target, target);
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_laplace_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "laplace state requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_context_filter_helper_call(target, filter_id, rspice_laplace_step_native)
    }

    fn emit_laplace_derivative(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "laplace derivative requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_context_filter_helper_call(target, filter_id, rspice_laplace_derivative_native)
    }

    fn emit_zi_state(&mut self, layout: crate::codegen::ZiRuntimeLayout) -> JitResult<()> {
        let operand_count =
            layout
                .validate_operand_budget()
                .map_err(|error| JitError::Encoding {
                    model: MODEL.into(),
                    detail: error.to_string().into(),
                })?;
        if self.depth < operand_count {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "zi state requires stack depth {operand_count}, found {}",
                    self.depth
                )
                .into(),
            });
        }
        let descriptor = layout
            .native_descriptor()
            .ok_or_else(|| JitError::Encoding {
                model: MODEL.into(),
                detail: "Zi runtime layout exceeds the native descriptor limits".into(),
            })?;
        let target = self.register_stack[self.depth - operand_count];
        self.emit_operand_context_filter_helper_call(
            target,
            operand_count,
            descriptor,
            rspice_zi_step_native,
        );
        self.drop_stack_values(operand_count - 1)
    }

    fn emit_zi_derivative_state(
        &mut self,
        layout: crate::codegen::ZiRuntimeLayout,
    ) -> JitResult<()> {
        let operand_count =
            layout
                .validate_operand_budget()
                .map_err(|error| JitError::Encoding {
                    model: MODEL.into(),
                    detail: error.to_string().into(),
                })?;
        if self.depth < operand_count {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "zi derivative state requires stack depth {operand_count}, found {}",
                    self.depth
                )
                .into(),
            });
        }
        let descriptor = layout
            .native_descriptor()
            .ok_or_else(|| JitError::Encoding {
                model: MODEL.into(),
                detail: "Zi derivative runtime layout exceeds the native descriptor limits".into(),
            })?;
        let target = self.register_stack[self.depth - operand_count];
        self.emit_operand_context_filter_helper_call(
            target,
            operand_count,
            descriptor,
            rspice_zi_derivative_native,
        );
        self.drop_stack_values(operand_count - 1)
    }

    fn emit_timer_state(&mut self, timer_id: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("timer state requires stack depth 4, found {}", self.depth).into(),
            });
        }

        let start = self.register_stack[self.depth - 4];
        self.emit_operand_context_filter_helper_call(start, 4, timer_id, rspice_timer_state_native);
        self.drop_stack_values(3)?;
        Ok(())
    }

    fn emit_transition_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "transition state requires stack depth 4, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = self.register_stack[self.depth - 4];
        self.emit_operand_context_filter_helper_call(
            input,
            4,
            filter_id,
            rspice_transition_state_native,
        );
        self.drop_stack_values(3)?;
        Ok(())
    }

    fn emit_transition_derivative_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 5 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "transition derivative state requires stack depth 5, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = self.register_stack[self.depth - 5];
        self.emit_operand_context_filter_helper_call(
            input,
            5,
            filter_id,
            rspice_transition_derivative_native,
        );
        self.drop_stack_values(4)?;
        Ok(())
    }

    fn emit_slew_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 3 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("slew state requires stack depth 3, found {}", self.depth).into(),
            });
        }

        let input = self.register_stack[self.depth - 3];
        self.emit_operand_context_filter_helper_call(input, 3, filter_id, rspice_slew_state_native);
        self.drop_stack_values(2)?;
        Ok(())
    }

    fn emit_slew_derivative_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 6 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "slew derivative state requires stack depth 6, found {}",
                    self.depth
                )
                .into(),
            });
        }
        let input = self.register_stack[self.depth - 6];
        self.emit_operand_context_filter_helper_call(
            input,
            6,
            filter_id,
            rspice_slew_derivative_native,
        );
        self.drop_stack_values(5)?;
        Ok(())
    }

    fn emit_absdelay_state(&mut self, buffer_id: usize) -> JitResult<()> {
        self.emit_absdelay_helper(buffer_id, 2, rspice_absdelay_state_native)
    }

    fn emit_absdelay_helper(
        &mut self,
        buffer_id: usize,
        operand_count: usize,
        helper: unsafe extern "C" fn(*const f64, *const crate::native::EvalContext, usize) -> f64,
    ) -> JitResult<()> {
        if self.depth < operand_count {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "absdelay state requires stack depth {operand_count}, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = self.register_stack[self.depth - operand_count];
        self.emit_operand_context_filter_helper_call(input, operand_count, buffer_id, helper);
        self.drop_stack_values(operand_count - 1)?;
        Ok(())
    }

    fn emit_cross_state(&mut self, detector_id: usize) -> JitResult<()> {
        if self.depth < 5 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("cross state requires stack depth 5, found {}", self.depth).into(),
            });
        }

        let input = self.register_stack[self.depth - 5];
        self.emit_operand_context_filter_helper_call(
            input,
            5,
            detector_id,
            rspice_cross_state_native,
        );
        self.drop_stack_values(4)?;
        Ok(())
    }

    fn emit_above_state(&mut self, detector_id: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("above state requires stack depth 4, found {}", self.depth).into(),
            });
        }

        let input = self.register_stack[self.depth - 4];
        self.emit_operand_context_filter_helper_call(
            input,
            4,
            detector_id,
            rspice_above_state_native,
        );
        self.drop_stack_values(3)?;
        Ok(())
    }

    fn emit_last_crossing_state(&mut self, detector_id: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "last_crossing state requires stack depth 2, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = self.register_stack[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            input,
            2,
            detector_id,
            rspice_last_crossing_state_native,
        );
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_ddt_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "ddt state requires stack depth 1, found 0".into(),
            });
        }
        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(
            target,
            1,
            state_index,
            rspice_ddt_state_native,
        );
        Ok(())
    }

    fn emit_ddt_jacobian(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "ddt jacobian requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(target, 1, 0, rspice_ddt_jacobian_native);
        Ok(())
    }

    fn emit_idt_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("idt state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let value = self.register_stack[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            value,
            2,
            state_index,
            rspice_idt_state_native,
        );
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_idt_jacobian(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "idt jacobian requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_operand_context_filter_helper_call(target, 1, 0, rspice_idt_jacobian_native);
        Ok(())
    }

    fn emit_idtmod_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("idtmod state requires stack depth 4, found {}", self.depth).into(),
            });
        }

        let value = self.register_stack[self.depth - 4];
        self.emit_operand_context_filter_helper_call(
            value,
            4,
            state_index,
            rspice_idtmod_state_native,
        );
        self.drop_stack_values(3)?;
        Ok(())
    }

    fn emit_void_error_return(&mut self, helper: VoidHelper) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(entry_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
        self.encoder.xorpd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm0);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_binary_helper_call(&mut self, left: Xmm, right: Xmm, helper: BinaryHelper) {
        debug_assert!(self.uses_helper_calls);
        let argument_swap = left == Xmm::Xmm1 && right == Xmm::Xmm0;
        let mut spill_slots =
            call_frame_spill_slot_count(&self.register_stack[..self.depth], |_, register| {
                register != left && register != right
            });
        if argument_swap {
            // XMM1 -> XMM0 and XMM0 -> XMM1 is a parallel move. Reserve a
            // non-overlapping call-frame slot so neither source is destroyed.
            spill_slots = spill_slots.max(self.depth + 1);
        }
        let frame_bytes = call_frame_bytes_for_slots(spill_slots);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| {
            register != left && register != right
        });

        if argument_swap {
            let swap_disp = call_spill_disp(self.depth);
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, swap_disp, left);
            self.encoder.movsd_xmm_xmm(Xmm::Xmm1, right);
            self.encoder
                .movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rsp, swap_disp);
        } else if left == Xmm::Xmm1 {
            // Preserve the first argument before assigning the second one.
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, left);
            if right != Xmm::Xmm1 {
                self.encoder.movsd_xmm_xmm(Xmm::Xmm1, right);
            }
        } else {
            // Preserve an XMM0 second argument before assigning XMM0.
            if right != Xmm::Xmm1 {
                self.encoder.movsd_xmm_xmm(Xmm::Xmm1, right);
            }
            if left != Xmm::Xmm0 {
                self.encoder.movsd_xmm_xmm(Xmm::Xmm0, left);
            }
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(left, self.depth, |_, register| {
            register != right
        });
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_operand_context_filter_helper_call(
        &mut self,
        input: Xmm,
        operand_count: usize,
        filter_id: usize,
        helper: OperandContextFilterHelper,
    ) {
        debug_assert!(self.uses_helper_calls);
        let input_slot = self.register_stack_slot(input);
        debug_assert!(operand_count > 0);
        debug_assert!(input_slot + operand_count <= self.depth);

        let frame_bytes = call_frame_bytes_for_slots(self.depth);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in self
            .register_stack
            .iter()
            .copied()
            .take(self.depth)
            .enumerate()
        {
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
        }

        self.encoder
            .mov_r64_r64(operand_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .mov_r64_r64(operand_filter_operands_arg_reg(), Gpr::R11);
        let operands_disp = call_spill_disp(input_slot);
        if operands_disp != 0 {
            self.encoder
                .add_r64_imm32(operand_filter_operands_arg_reg(), operands_disp);
        }
        self.emit_usize_arg(operand_filter_id_arg_reg(), filter_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(input, input_slot, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_compare(&mut self, op: CompareOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("comparison requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        let right = self.register_stack[self.depth - 1];
        match op {
            CompareOp::Gt => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                self.emit_condition_result(left, ConditionCode::Above);
            }
            CompareOp::Ge => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                self.emit_condition_result(left, ConditionCode::AboveOrEqual);
            }
            CompareOp::Lt => {
                self.encoder.ucomisd_xmm_xmm(right, left);
                self.emit_condition_result(left, ConditionCode::Above);
            }
            CompareOp::Le => {
                self.encoder.ucomisd_xmm_xmm(right, left);
                self.emit_condition_result(left, ConditionCode::AboveOrEqual);
            }
            CompareOp::Eq => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                self.emit_ordered_condition_result(left, ConditionCode::Equal);
            }
            CompareOp::Ne => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                self.emit_unordered_or_condition_result(left, ConditionCode::NotEqual);
            }
        }
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_compare_const(&mut self, op: CompareOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS comparison requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        match op {
            CompareOp::Gt => {
                self.emit_literal_compare(target, value);
                self.emit_condition_result(target, ConditionCode::Above);
            }
            CompareOp::Ge => {
                self.emit_literal_compare(target, value);
                self.emit_condition_result(target, ConditionCode::AboveOrEqual);
            }
            CompareOp::Lt => {
                self.emit_literal_compare(target, value);
                self.emit_ordered_condition_result(target, ConditionCode::Below);
            }
            CompareOp::Le => {
                self.emit_literal_compare(target, value);
                self.emit_ordered_condition_result(target, ConditionCode::BelowOrEqual);
            }
            CompareOp::Eq => {
                self.emit_literal_compare(target, value);
                self.emit_ordered_condition_result(target, ConditionCode::Equal);
            }
            CompareOp::Ne => {
                self.emit_literal_compare(target, value);
                self.emit_unordered_or_condition_result(target, ConditionCode::NotEqual);
            }
        }
        Ok(())
    }

    fn emit_condition_result(&mut self, dst: Xmm, condition: ConditionCode) {
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    }

    fn emit_ordered_condition_result(&mut self, dst: Xmm, condition: ConditionCode) {
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.setcc_r8(ConditionCode::NotParity, Gpr::R11);
        self.encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    }

    fn emit_unordered_or_condition_result(&mut self, dst: Xmm, condition: ConditionCode) {
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    }

    fn emit_logical(&mut self, op: LogicalOp) -> JitResult<()> {
        match op {
            LogicalOp::And | LogicalOp::Or => self.emit_logical_binary(op),
            LogicalOp::Not => self.emit_logical_not(),
        }
    }

    fn emit_logical_binary(&mut self, op: LogicalOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("logical op requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        let right = self.register_stack[self.depth - 1];
        self.emit_truthy_to_gpr(right, Gpr::R11);
        self.emit_truthy_to_gpr(left, Gpr::R10);
        match op {
            LogicalOp::And => self.encoder.and_r8_r8(Gpr::R10, Gpr::R11),
            LogicalOp::Or => self.encoder.or_r8_r8(Gpr::R10, Gpr::R11),
            LogicalOp::Not => unreachable!("logical binary lowering only accepts and/or"),
        }
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(left, Gpr::R10);
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_logical_const(&mut self, op: LogicalOp, rhs_truthy: bool) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS logical op requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        match (op, rhs_truthy) {
            (LogicalOp::And, true) | (LogicalOp::Or, false) => {
                self.emit_truthy_to_gpr(target, Gpr::R10);
                self.emit_gpr_bool_result(target, Gpr::R10);
            }
            (LogicalOp::And, false) => self.emit_bool_result(target, false),
            (LogicalOp::Or, true) => self.emit_bool_result(target, true),
            (LogicalOp::Not, _) => unreachable!("logical constant RHS only accepts and/or"),
        }
        Ok(())
    }

    fn emit_logical_not(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "logical not requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_falsy_to_gpr(target, Gpr::R10);
        self.emit_gpr_bool_result(target, Gpr::R10);
        Ok(())
    }

    fn emit_truthy_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.encoder.movq_r64_xmm(dst, value);
        self.encoder.btr_r64_imm8(dst, 63);
        self.encoder.test_r64_r64(dst, dst);
        self.encoder.setcc_r8(ConditionCode::NotEqual, dst);
    }

    fn emit_bool_result(&mut self, dst: Xmm, value: bool) {
        if value {
            self.encoder.mov_r32_imm32(Gpr::R10, 1);
            self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        } else {
            self.encoder.xorpd_xmm_xmm(dst, dst);
        }
    }

    fn emit_gpr_bool_result(&mut self, dst: Xmm, src: Gpr) {
        self.encoder.movzx_r32_r8(src, src);
        self.encoder.cvtsi2sd_xmm_r32(dst, src);
    }

    fn emit_falsy_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.encoder.movq_r64_xmm(dst, value);
        self.encoder.btr_r64_imm8(dst, 63);
        self.encoder.test_r64_r64(dst, dst);
        self.encoder.setcc_r8(ConditionCode::Equal, dst);
    }

    fn emit_ifelse(&mut self) -> JitResult<()> {
        if self.depth < 3 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("ifelse requires stack depth 3, found {}", self.depth).into(),
            });
        }

        let cond = self.register_stack[self.depth - 3];
        let then_value = self.register_stack[self.depth - 2];
        let else_value = self.register_stack[self.depth - 1];
        self.emit_literal_compare(cond, 0.0);
        self.encoder.movq_r64_xmm(Gpr::R8, else_value);
        self.encoder.movq_r64_xmm(Gpr::R11, then_value);
        self.encoder.cmovne_r64_r64(Gpr::R8, Gpr::R11);
        // UCOMISD reports unordered values with PF=1 and ZF=1. Verilog-A
        // truthiness treats every nonzero bit pattern, including NaN, as true.
        self.encoder.cmovp_r64_r64(Gpr::R8, Gpr::R11);
        self.encoder.movq_xmm_r64(cond, Gpr::R8);
        self.drop_stack_values(2)?;
        Ok(())
    }

    fn emit_extremum(&mut self, op: ExtremumOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("min/max requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = self.register_stack[self.depth - 2];
        let right = self.register_stack[self.depth - 1];
        self.emit_extremum_left_prelude(left);
        self.emit_extremum_register_op(left, right, op);
        self.emit_extremum_select_left_fixup(left, right);
        self.drop_stack_values(1)?;
        Ok(())
    }

    fn emit_extremum_const(&mut self, op: ExtremumOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS min/max requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        self.emit_extremum_left_prelude(target);
        self.emit_literal_extremum_op(target, value, op);
        self.emit_extremum_select_left_fixup_from_result(target);
        Ok(())
    }

    fn emit_extremum_const_lhs(&mut self, op: ExtremumOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal LHS min/max requires stack depth 1, found 0".into(),
            });
        }

        let target = self.register_stack[self.depth - 1];
        if self.depth < XMM_STACK.len() {
            let left = self.scratch_register()?;
            self.emit_literal_load(left, value);
            self.emit_extremum_left_prelude(left);
            self.emit_extremum_register_op(left, target, op);
            self.emit_extremum_select_left_fixup(left, target);
            self.encoder.movsd_xmm_xmm(target, left);
        } else {
            self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
            self.emit_literal_load(target, value);
            self.emit_extremum_left_prelude(target);
            self.emit_extremum_memory_op(target, Gpr::Rsp, 0, op);
            self.emit_extremum_select_left_fixup_from_result(target);
            self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        }

        Ok(())
    }

    fn emit_extremum_left_prelude(&mut self, left: Xmm) {
        self.encoder.movq_r64_xmm(Gpr::R8, left);
        self.encoder.ucomisd_xmm_xmm(left, left);
        self.encoder.setcc_r8(ConditionCode::NotParity, Gpr::R10);
        self.emit_abs_zero_from_bits_to_gpr(Gpr::R8, Gpr::R9);
    }

    fn emit_extremum_register_op(&mut self, left: Xmm, right: Xmm, op: ExtremumOp) {
        match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_xmm(left, right),
            ExtremumOp::Max => self.encoder.maxsd_xmm_xmm(left, right),
        }
    }

    fn emit_extremum_memory_op(&mut self, left: Xmm, base: Gpr, disp: i32, op: ExtremumOp) {
        match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_m64_base_disp32(left, base, disp),
            ExtremumOp::Max => self.encoder.maxsd_xmm_m64_base_disp32(left, base, disp),
        }
    }

    fn emit_extremum_select_left_fixup(&mut self, result: Xmm, right: Xmm) {
        self.encoder.ucomisd_xmm_xmm(right, right);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.emit_extremum_select_left_fixup_after_right_check(result);
    }

    fn emit_extremum_select_left_fixup_from_result(&mut self, result: Xmm) {
        self.encoder.ucomisd_xmm_xmm(result, result);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.emit_extremum_select_left_fixup_after_right_check(result);
    }

    fn emit_extremum_select_left_fixup_after_right_check(&mut self, result: Xmm) {
        self.encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        self.emit_abs_zero_to_gpr(result, Gpr::R11);
        self.encoder.and_r8_r8(Gpr::R9, Gpr::R11);
        self.encoder.or_r8_r8(Gpr::R10, Gpr::R9);
        self.encoder.movq_r64_xmm(Gpr::R11, result);
        self.encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        self.encoder.cmovne_r64_r64(Gpr::R11, Gpr::R8);
        self.encoder.movq_xmm_r64(result, Gpr::R11);
    }

    fn emit_abs_zero_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.encoder.movq_r64_xmm(dst, value);
        self.emit_abs_zero_bits_in_place(dst);
    }

    fn emit_abs_zero_from_bits_to_gpr(&mut self, bits: Gpr, dst: Gpr) {
        debug_assert_ne!(bits, dst);
        self.encoder.mov_r64_r64(dst, bits);
        self.emit_abs_zero_bits_in_place(dst);
    }

    fn emit_abs_zero_bits_in_place(&mut self, dst: Gpr) {
        self.encoder.btr_r64_imm8(dst, 63);
        self.encoder.test_r64_r64(dst, dst);
        self.encoder.setcc_r8(ConditionCode::Equal, dst);
    }

    fn emit_voltage_load(
        &mut self,
        pos: VoltageNode,
        neg: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;

        match (pos, neg) {
            (VoltageNode::Ground, VoltageNode::Ground) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
            }
            (pos, VoltageNode::Ground) => {
                self.emit_node_voltage_load(dst, pos, context_pointer_cache)?;
            }
            (VoltageNode::Ground, neg) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
                self.emit_node_voltage_subtract(dst, neg, context_pointer_cache)?;
            }
            (VoltageNode::Terminal(pos), VoltageNode::Terminal(neg)) => {
                self.emit_same_storage_voltage_difference(
                    dst,
                    VOLTAGES_OFFSET,
                    pos,
                    neg,
                    context_pointer_cache,
                )?;
            }
            (VoltageNode::Internal(pos), VoltageNode::Internal(neg)) => {
                self.emit_same_storage_voltage_difference(
                    dst,
                    INTERNAL_VOLTAGES_OFFSET,
                    pos,
                    neg,
                    context_pointer_cache,
                )?;
            }
            (pos, neg) => {
                self.emit_node_voltage_load(dst, pos, context_pointer_cache)?;
                self.emit_node_voltage_subtract(dst, neg, context_pointer_cache)?;
            }
        }

        Ok(())
    }

    fn emit_same_storage_voltage_difference(
        &mut self,
        dst: Xmm,
        ctx_field_offset: i32,
        pos_index: usize,
        neg_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(ctx_field_offset, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(pos_index)?);
        if pos_index == neg_index {
            self.encoder.subsd_xmm_xmm(dst, dst);
        } else {
            self.encoder
                .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(neg_index)?);
        }
        Ok(())
    }

    fn emit_thermal_voltage_load(&mut self) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), TEMPERATURE_OFFSET);
        self.emit_literal_binary_op(dst, THERMAL_VOLTAGE_PER_K, BinaryOp::Mul);
        Ok(())
    }

    fn emit_node_voltage_load(
        &mut self,
        dst: Xmm,
        node: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => {
                self.emit_terminal_voltage_load(dst, index, context_pointer_cache)
            }
            VoltageNode::Internal(index) => {
                self.emit_internal_voltage_load(dst, index, context_pointer_cache)
            }
            VoltageNode::Ground => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
                Ok(())
            }
        }
    }

    fn emit_node_voltage_subtract(
        &mut self,
        dst: Xmm,
        node: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => {
                self.emit_context_pointer_load_cached(VOLTAGES_OFFSET, context_pointer_cache);
                self.encoder
                    .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                Ok(())
            }
            VoltageNode::Internal(index) => {
                self.emit_context_pointer_load_cached(
                    INTERNAL_VOLTAGES_OFFSET,
                    context_pointer_cache,
                );
                self.encoder
                    .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                Ok(())
            }
            VoltageNode::Ground => Ok(()),
        }
    }

    fn emit_terminal_voltage_load(
        &mut self,
        dst: Xmm,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(VOLTAGES_OFFSET, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_internal_voltage_load(
        &mut self,
        dst: Xmm,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(INTERNAL_VOLTAGES_OFFSET, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_context_pointer_load(&mut self, ctx_field_offset: i32) {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, self.ctx_arg_reg(), ctx_field_offset);
    }

    fn emit_context_pointer_load_cached(
        &mut self,
        ctx_field_offset: i32,
        context_pointer_cache: &mut Option<i32>,
    ) {
        if *context_pointer_cache != Some(ctx_field_offset) {
            self.emit_context_pointer_load(ctx_field_offset);
            *context_pointer_cache = Some(ctx_field_offset);
        }
    }

    fn emit_context_f64_load(&mut self, ctx_field_offset: i32) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), ctx_field_offset);
        Ok(())
    }

    fn emit_param_given_load(
        &mut self,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_u8_slice_load(
            PARAM_GIVEN_OFFSET,
            PARAM_GIVEN_LEN_OFFSET,
            index,
            rspice_native_param_given_error,
            context_pointer_cache,
        )
    }

    fn emit_port_connected_load(
        &mut self,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_u8_slice_load(
            PORT_CONNECTED_OFFSET,
            PORT_CONNECTED_LEN_OFFSET,
            index,
            rspice_native_port_connected_error,
            context_pointer_cache,
        )
    }

    fn emit_guarded_context_u8_slice_load(
        &mut self,
        pointer_field_offset: i32,
        len_field_offset: i32,
        index: usize,
        helper: VoidHelper,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;
        let value_disp = byte_disp_u8(index)?;

        self.emit_context_pointer_load_cached(pointer_field_offset, context_pointer_cache);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let missing_storage = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), len_field_offset);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, slice_index_imm32(index)?);
        let index_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, value_disp);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(missing_storage)?;
        self.patch_rel32_to_current(index_out_of_range)?;
        self.emit_void_error_return(helper);

        self.patch_rel32_to_current(done)
    }

    fn emit_analysis_check(&mut self, analysis_id: u8) -> JitResult<()> {
        let dst = self.push_register()?;
        if matches!(analysis_id, 7 | 8) {
            let offset = if analysis_id == 7 {
                ANALYSIS_INITIAL_STEP_OFFSET
            } else {
                ANALYSIS_FINAL_STEP_OFFSET
            };
            self.encoder
                .movzx_r32_m8_base_disp32(Gpr::R10, self.ctx_arg_reg(), offset);
            self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
            return Ok(());
        }
        if analysis_id > 8 {
            self.encoder.xorpd_xmm_xmm(dst, dst);
            return Ok(());
        }

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, self.ctx_arg_reg(), ANALYSIS_TYPE_OFFSET);
        match analysis_id {
            5 => {
                self.encoder.cmp_r32_imm8(Gpr::R10, 0);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R11);
                self.encoder.cmp_r32_imm8(Gpr::R10, 4);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
                self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
            }
            6 => {
                self.encoder.cmp_r32_imm8(Gpr::R10, 1);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R11);
                self.encoder.cmp_r32_imm8(Gpr::R10, 3);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
                self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
            }
            _ => {
                self.encoder.cmp_r32_imm8(Gpr::R10, analysis_id);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
            }
        }
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        Ok(())
    }

    fn emit_literal_load(&mut self, dst: Xmm, value: f64) {
        let displacement_offset = self.encoder.movsd_xmm_m64_rip_disp32(dst, 0);
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_constant_load(&mut self, dst: Xmm, value: f64) {
        if value.to_bits() == 0 {
            self.encoder.xorpd_xmm_xmm(dst, dst);
        } else {
            self.emit_literal_load(dst, value);
        }
    }

    fn emit_literal_binary_op(&mut self, dst: Xmm, value: f64, op: BinaryOp) {
        if matches!(op, BinaryOp::Mul) && value.to_bits() == 2.0_f64.to_bits() {
            self.encoder.addsd_xmm_xmm(dst, dst);
            return;
        }

        let displacement_offset = match op {
            BinaryOp::Add => self.encoder.addsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Sub => self.encoder.subsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Mul => self.encoder.mulsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Div => self.encoder.divsd_xmm_m64_rip_disp32(dst, 0),
        };
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_literal_compare(&mut self, left: Xmm, value: f64) {
        let displacement_offset = self.encoder.ucomisd_xmm_m64_rip_disp32(left, 0);
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_literal_extremum_op(&mut self, dst: Xmm, value: f64, op: ExtremumOp) {
        let displacement_offset = match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_m64_rip_disp32(dst, 0),
            ExtremumOp::Max => self.encoder.maxsd_xmm_m64_rip_disp32(dst, 0),
        };
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_usize_arg(&mut self, dst: Gpr, value: usize) {
        if value == 0 {
            self.encoder.xor_r64_r64(dst, dst);
        } else if let Ok(value) = u32::try_from(value) {
            self.encoder.mov_r32_imm32(dst, value);
        } else {
            self.encoder.movabs_r64_imm64(dst, value as u64);
        }
    }

    fn emit_i64_arg(&mut self, dst: Gpr, value: i64) {
        if value == 0 {
            self.encoder.xor_r64_r64(dst, dst);
        } else if let Ok(value) = u32::try_from(value) {
            self.encoder.mov_r32_imm32(dst, value);
        } else if let Ok(value) = i32::try_from(value) {
            self.encoder.mov_r64_imm32(dst, value);
        } else {
            self.encoder.movabs_r64_imm64(dst, value as u64);
        }
    }

    fn emit_usize_compare(&mut self, left: Gpr, value: usize) {
        if let Ok(value) = i32::try_from(value) {
            self.encoder.cmp_r64_imm32(left, value);
        } else {
            self.encoder.movabs_r64_imm64(Gpr::R11, value as u64);
            self.encoder.cmp_r64_r64(left, Gpr::R11);
        }
    }

    fn emit_i64_subtract(&mut self, target: Gpr, value: i64) {
        if value == 0 {
            return;
        }
        if let Ok(value) = i32::try_from(value) {
            self.encoder.sub_r64_imm32(target, value);
        } else {
            self.encoder.movabs_r64_imm64(Gpr::R11, value as u64);
            self.encoder.sub_r64_r64(target, Gpr::R11);
        }
    }

    fn finish_with_literals(self) -> JitResult<X64FunctionBody> {
        let mut bytes = self.encoder.into_bytes();
        let code_len = bytes.len();
        super::verify_x64_function_code(&bytes, "generated function")?;
        let layout = FunctionLiteralLayout::build(code_len, &self.literals, &self.vector_literals)?;
        bytes.resize(layout.final_len, LITERAL_POOL_PADDING_BYTE);
        let mut data_ranges = Vec::new();
        let mut rip_relative_relocations = Vec::with_capacity(
            self.literals
                .len()
                .checked_add(self.vector_literals.len())
                .ok_or_else(|| JitError::Relocation {
                    model: MODEL.into(),
                    detail: "literal relocation count overflow".into(),
                })?,
        );
        for (index, bits) in layout.scalar_bits.iter().copied().enumerate() {
            let offset = layout.scalar_start + index * WORD_BYTES;
            bytes[offset..offset + WORD_BYTES].copy_from_slice(&bits.to_le_bytes());
        }
        for literal in &self.literals {
            let bits = literal.value.to_bits();
            let literal_offset = layout.scalar_offset(bits);
            let next_instruction_offset = literal.displacement_offset + std::mem::size_of::<i32>();
            let displacement = i32::try_from(
                literal_offset as isize - next_instruction_offset as isize,
            )
            .map_err(|_| JitError::Relocation {
                model: MODEL.into(),
                detail: "literal pool displacement does not fit in i32".into(),
            })?;

            bytes[literal.displacement_offset..literal.displacement_offset + 4]
                .copy_from_slice(&displacement.to_le_bytes());
            rip_relative_relocations.push(X64RipRelativeRelocation {
                displacement_offset: literal.displacement_offset,
                target_offset: literal_offset,
                kind: X64DataKind::ScalarF64,
            });
        }
        if !layout.scalar_bits.is_empty() {
            data_ranges.push(X64DataRange {
                start: layout.scalar_start,
                end: layout.scalar_start + layout.scalar_bits.len() * WORD_BYTES,
                alignment: LITERAL_POOL_ALIGNMENT,
                kind: X64DataKind::ScalarF64,
            });
        }

        for (index, lanes) in layout.vector_bits.iter().copied().enumerate() {
            let offset = layout.vector_start + index * VECTOR_LITERAL_ALIGNMENT;
            bytes[offset..offset + WORD_BYTES].copy_from_slice(&lanes[0].to_le_bytes());
            bytes[offset + WORD_BYTES..offset + VECTOR_LITERAL_ALIGNMENT]
                .copy_from_slice(&lanes[1].to_le_bytes());
        }
        for literal in &self.vector_literals {
            let literal_offset = layout.vector_offset(literal.lanes);
            let next_instruction_offset = literal.displacement_offset + std::mem::size_of::<i32>();
            let displacement = i32::try_from(
                literal_offset as isize - next_instruction_offset as isize,
            )
            .map_err(|_| JitError::Relocation {
                model: MODEL.into(),
                detail: "vector literal pool displacement does not fit in i32".into(),
            })?;

            bytes[literal.displacement_offset..literal.displacement_offset + 4]
                .copy_from_slice(&displacement.to_le_bytes());
            rip_relative_relocations.push(X64RipRelativeRelocation {
                displacement_offset: literal.displacement_offset,
                target_offset: literal_offset,
                kind: X64DataKind::Vector128,
            });
        }
        if !layout.vector_bits.is_empty() {
            data_ranges.push(X64DataRange {
                start: layout.vector_start,
                end: layout.vector_start + layout.vector_bits.len() * VECTOR_LITERAL_ALIGNMENT,
                alignment: VECTOR_LITERAL_ALIGNMENT,
                kind: X64DataKind::Vector128,
            });
        }

        Ok(X64FunctionBody {
            bytes,
            code_len,
            data_ranges,
            rip_relative_relocations,
        })
    }

    fn emit_jmp_to_offset(&mut self, target_offset: usize) -> JitResult<()> {
        let displacement_offset = self.encoder.jmp_rel32_placeholder();
        self.patch_rel32_to_offset(displacement_offset, target_offset)
    }

    fn patch_rel32_to_current(&mut self, patch: Rel32Patch) -> JitResult<()> {
        self.patch_rel32_to_offset(patch, self.encoder.position())
    }

    fn patch_rel32_to_offset(&mut self, patch: Rel32Patch, target_offset: usize) -> JitResult<()> {
        let next_instruction_offset = patch.next_instruction_offset();
        let displacement = i64::try_from(target_offset)
            .ok()
            .and_then(|target| {
                i64::try_from(next_instruction_offset)
                    .ok()
                    .and_then(|next| target.checked_sub(next))
            })
            .and_then(|displacement| i32::try_from(displacement).ok())
            .ok_or_else(|| JitError::Relocation {
                model: MODEL.into(),
                detail: "branch displacement does not fit in i32".into(),
            })?;
        self.encoder.patch_rel32(patch, displacement);
        Ok(())
    }

    fn patch_early_returns_to_current(&mut self) -> JitResult<()> {
        let jumps = std::mem::take(&mut self.early_return_jumps);
        for patch in jumps {
            self.patch_rel32_to_current(patch)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum RoundDirection {
    Floor,
    Ceil,
}

type UnaryHelper = extern "C" fn(f64) -> f64;
type BinaryHelper = extern "C" fn(f64, f64) -> f64;
type VoidHelper = extern "C" fn(*const crate::native::EvalContext);
type TableHelper = unsafe extern "C" fn(f64, *const crate::native::EvalContext, usize) -> f64;
type ContextFilterHelper =
    unsafe extern "C" fn(f64, *const crate::native::EvalContext, usize) -> f64;
type OperandContextFilterHelper =
    unsafe extern "C" fn(*const f64, *const crate::native::EvalContext, usize) -> f64;
type DynamicVariableErrorHelper =
    extern "C" fn(f64, *const crate::native::EvalContext, usize, i64) -> f64;
type DynamicVariableSlotHelper = unsafe extern "C" fn(f64, *mut f64, usize, i64) -> *mut f64;

fn assignment_uses_helper_calls(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { program, .. } => program_uses_helper_calls(program),
        NativeAssignment::Indexed {
            len,
            lower,
            index,
            value,
            ..
        } => {
            !dynamic_variable_inline_supported(*len, *lower)
                || program_uses_helper_calls(index)
                || program_uses_helper_calls(value)
        }
        NativeAssignment::Loop { condition, body } => {
            program_uses_helper_calls(condition) || body.iter().any(assignment_uses_helper_calls)
        }
    }
}

fn assignment_max_stack_depth(assignments: &[NativeAssignment]) -> usize {
    assignments
        .iter()
        .map(|assignment| match assignment {
            NativeAssignment::Direct { program, .. } => program.max_stack_depth(),
            NativeAssignment::Indexed { index, value, .. } => {
                index.max_stack_depth().max(value.max_stack_depth())
            }
            NativeAssignment::Loop { condition, body } => condition
                .max_stack_depth()
                .max(assignment_max_stack_depth(body)),
        })
        .max()
        .unwrap_or(0)
}

fn assignment_allocation_requirements(
    assignments: &[NativeAssignment],
) -> JitResult<(usize, usize)> {
    let mut maximum_spill_slots = 0;
    let mut maximum_required_registers = 0;
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
                            Err(JitError::Verifier {
                                model: MODEL.into(),
                                detail: "x64 requirement batch contains a control-flow assignment"
                                    .into(),
                            })
                        }
                    })
                    .collect::<JitResult<Vec<_>>>()?;
                let assignment = AssignmentProgram::lower(&direct)?;
                let allocation =
                    RegisterAllocation::build_for_assignments(&assignment, X64_VALUE_BANK)?;
                maximum_spill_slots = maximum_spill_slots.max(allocation.spill_slot_count());
                maximum_required_registers =
                    maximum_required_registers.max(allocation.required_register_count());
            }
            NativeAssignment::Indexed { index, value, .. } => {
                debug_assert_eq!(batch.len(), 1);
                for program in [index, value] {
                    let ssa = X64SsaProgram::lower(program)?;
                    let allocation = RegisterAllocation::build(&ssa, X64_VALUE_BANK)?;
                    maximum_spill_slots = maximum_spill_slots.max(allocation.spill_slot_count());
                    maximum_required_registers =
                        maximum_required_registers.max(allocation.required_register_count());
                }
            }
            NativeAssignment::Loop { condition, body } => {
                debug_assert_eq!(batch.len(), 1);
                let (body_spills, body_registers) = assignment_allocation_requirements(body)?;
                maximum_spill_slots = maximum_spill_slots.max(body_spills);
                maximum_required_registers = maximum_required_registers.max(body_registers);
                let ssa = X64SsaProgram::lower(condition)?;
                let allocation = RegisterAllocation::build(&ssa, X64_VALUE_BANK)?;
                maximum_spill_slots = maximum_spill_slots.max(allocation.spill_slot_count());
                maximum_required_registers =
                    maximum_required_registers.max(allocation.required_register_count());
            }
        }
    }
    Ok((maximum_spill_slots, maximum_required_registers))
}

fn assignment_has_indexed(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { .. } => false,
        NativeAssignment::Indexed { .. } => true,
        NativeAssignment::Loop { body, .. } => body.iter().any(assignment_has_indexed),
    }
}

fn assignment_loop_depth(assignments: &[NativeAssignment]) -> JitResult<i32> {
    let mut maximum_depth = 0_i32;
    for assignment in assignments {
        let depth = match assignment {
            NativeAssignment::Direct { .. } | NativeAssignment::Indexed { .. } => 0,
            NativeAssignment::Loop { body, .. } => {
                assignment_loop_depth(body)?.checked_add(1).ok_or_else(|| {
                    register_allocation_error(
                        "assignment-loop nesting exceeds the x64 frame displacement range".into(),
                    )
                })?
            }
        };
        maximum_depth = maximum_depth.max(depth);
    }
    Ok(maximum_depth)
}

fn checked_local_frame_bytes(components: &[i32]) -> JitResult<i32> {
    let unaligned_bytes = components.iter().try_fold(0_i32, |total, component| {
        if *component < 0 {
            return Err(register_allocation_error(format!(
                "generated frame component has negative size {component}"
            )));
        }
        total.checked_add(*component).ok_or_else(|| {
            register_allocation_error("generated frame exceeds the x64 displacement range".into())
        })
    })?;
    if unaligned_bytes == 0 {
        return Ok(0);
    }
    let rounded_bytes = unaligned_bytes
        .checked_add(LOCAL_FRAME_ALIGN_BYTES - 1)
        .ok_or_else(|| {
            register_allocation_error(
                "aligned generated frame exceeds the x64 displacement range".into(),
            )
        })?;
    Ok((rounded_bytes / LOCAL_FRAME_ALIGN_BYTES) * LOCAL_FRAME_ALIGN_BYTES)
}

fn validate_expression_stack_depth(max_stack_depth: usize) -> JitResult<()> {
    if max_stack_depth > MAX_EXPRESSION_STACK_DEPTH {
        return Err(register_allocation_error(format!(
            "expression stack depth {max_stack_depth} exceeds the {MAX_EXPRESSION_STACK_DEPTH}-value safety limit"
        )));
    }
    Ok(())
}

fn allocation_spill_frame_bytes(allocation: &RegisterAllocation) -> JitResult<i32> {
    allocation
        .spill_slot_count()
        .checked_mul(WORD_BYTES)
        .and_then(|bytes| i32::try_from(bytes).ok())
        .ok_or_else(|| {
            register_allocation_error(
                "liveness spill frame exceeds the x64 frame displacement range".to_string(),
            )
        })
}

fn callee_saved_xmm_count_for_allocation(allocation: &RegisterAllocation) -> usize {
    allocation
        .required_register_count()
        .min(XMM_STACK.len())
        .saturating_sub(CALLER_SAVED_XMM_COUNT)
}

fn callee_saved_xmm_frame_bytes(count: usize) -> i32 {
    count as i32 * CALLEE_SAVED_XMM_BYTES
}

fn allocated_xmm(index: usize) -> JitResult<Xmm> {
    if index >= ALLOCATABLE_VALUE_REGISTERS {
        return Err(register_allocation_error(format!(
            "allocated XMM register {index} exceeds the {ALLOCATABLE_VALUE_REGISTERS}-register value set"
        )));
    }
    Ok(XMM_STACK[index])
}

fn xmm_mask(register: Xmm) -> u32 {
    1_u32 << xmm_unwind_register(register)
}

fn take_temporary_xmm(used_register_mask: &mut u32) -> JitResult<Xmm> {
    let register = XMM_STACK
        .into_iter()
        .find(|register| *used_register_mask & xmm_mask(*register) == 0)
        .ok_or_else(|| {
            register_allocation_error(
                "allocated instruction has no XMM register available for a spilled operand"
                    .to_string(),
            )
        })?;
    *used_register_mask |= xmm_mask(register);
    Ok(register)
}

fn callee_saved_xmm_register(index: usize) -> Xmm {
    XMM_STACK[CALLER_SAVED_XMM_COUNT + index]
}

fn xmm_unwind_register(register: Xmm) -> u8 {
    match register {
        Xmm::Xmm0 => 0,
        Xmm::Xmm1 => 1,
        Xmm::Xmm2 => 2,
        Xmm::Xmm3 => 3,
        Xmm::Xmm4 => 4,
        Xmm::Xmm5 => 5,
        Xmm::Xmm6 => 6,
        Xmm::Xmm7 => 7,
        Xmm::Xmm8 => 8,
        Xmm::Xmm9 => 9,
        Xmm::Xmm10 => 10,
        Xmm::Xmm11 => 11,
        Xmm::Xmm12 => 12,
        Xmm::Xmm13 => 13,
        Xmm::Xmm14 => 14,
        Xmm::Xmm15 => 15,
    }
}

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
        UnaryMathOp::Floor => rspice_floor,
        UnaryMathOp::Ceil => rspice_ceil,
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

fn call_spill_disp(index: usize) -> i32 {
    CALL_SHADOW_BYTES + (index * WORD_BYTES) as i32
}

fn call_frame_bytes_for_slots(slot_count: usize) -> i32 {
    let spill_bytes = (slot_count * WORD_BYTES) as i32;
    let mut frame_bytes = CALL_SHADOW_BYTES + spill_bytes;
    while frame_bytes % LOCAL_FRAME_ALIGN_BYTES != LOCAL_SLOT_BYTES {
        frame_bytes += LOCAL_SLOT_BYTES;
    }
    frame_bytes
}

fn call_frame_spill_slot_count(
    registers: &[Xmm],
    mut should_spill: impl FnMut(usize, Xmm) -> bool,
) -> usize {
    registers
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, register)| should_spill(index, register).then_some(index + 1))
        .max()
        .unwrap_or(0)
}

#[cfg(all(test, target_arch = "x86_64"))]
fn call_result_disp() -> i32 {
    call_spill_disp(CALL_RESULT_SLOT)
}

fn program_uses_helper_calls(program: &NativeProgram) -> bool {
    program.ops().iter().any(native_op_uses_helper_call)
}

#[cfg(all(test, target_arch = "x86_64"))]
fn value_program_needs_saved_entry_args(program: &NativeProgram) -> bool {
    let mut helper_seen = false;
    for op in program.ops() {
        if native_op_needs_saved_entry_args_for_internal_helper_continuation(*op) {
            return true;
        }
        if helper_seen && native_op_reads_entry_args(*op) {
            return true;
        }
        helper_seen |= native_op_uses_helper_call(op);
    }
    false
}

#[cfg(all(test, target_arch = "x86_64"))]
fn native_op_needs_saved_entry_args_for_internal_helper_continuation(op: NativeOp) -> bool {
    X64Effects::for_op(op).needs_saved_entry_args_for_internal_continuation()
}

fn native_op_uses_helper_call(op: &NativeOp) -> bool {
    X64Effects::for_op(*op).may_call()
}

#[cfg(all(test, target_arch = "x86_64"))]
fn native_op_reads_entry_args(op: NativeOp) -> bool {
    X64Effects::for_op(op).reads_entry_args()
}

fn byte_disp(index: usize) -> JitResult<i32> {
    let byte_offset = index
        .checked_mul(WORD_BYTES)
        .ok_or_else(|| JitError::Encoding {
            model: MODEL.into(),
            detail: format!("index {index} byte offset overflow").into(),
        })?;

    i32::try_from(byte_offset).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("index {index} byte offset exceeds x64 disp32 range").into(),
    })
}

fn byte_disp_u8(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("u8 flag index {index} exceeds x64 disp32 range").into(),
    })
}

fn slice_index_imm32(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("slice index {index} exceeds x64 imm32 range").into(),
    })
}

fn register_allocation_error(detail: String) -> JitError {
    JitError::RegisterAllocation {
        model: MODEL.into(),
        detail: detail.into(),
    }
}

fn saved_ctx_arg_reg() -> Gpr {
    HOST_ABI.saved_context
}

fn saved_vars_arg_reg() -> Gpr {
    HOST_ABI.saved_variables
}

fn entry_ctx_arg_reg() -> Gpr {
    HOST_ABI.entry_context
}

fn entry_vars_arg_reg() -> Gpr {
    HOST_ABI.entry_variables
}

fn entry_kernel_io_arg_reg() -> Gpr {
    HOST_ABI.entry_kernel_io
}

fn context_filter_ctx_arg_reg() -> Gpr {
    HOST_ABI.context_filter[0]
}

fn context_filter_id_arg_reg() -> Gpr {
    HOST_ABI.context_filter[1]
}

fn operand_filter_operands_arg_reg() -> Gpr {
    HOST_ABI.operand_filter[0]
}

fn operand_filter_ctx_arg_reg() -> Gpr {
    HOST_ABI.operand_filter[1]
}

fn operand_filter_id_arg_reg() -> Gpr {
    HOST_ABI.operand_filter[2]
}

fn dynamic_variable_ptr_arg_reg() -> Gpr {
    HOST_ABI.dynamic_variable[0]
}

fn dynamic_variable_len_arg_reg() -> Gpr {
    HOST_ABI.dynamic_variable[1]
}

fn dynamic_variable_lower_arg_reg() -> Gpr {
    HOST_ABI.dynamic_variable[2]
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        ABS_VALUE_MASK_HIGH, ABS_VALUE_MASK_LOW, BRANCH_CURRENTS_OFFSET, ConditionCode,
        ContextFilterHelper, DYNAMIC_READ_FRAME_BYTES, FunctionCompiler, Gpr,
        I64_MAX_EXCLUSIVE_AS_F64, I64_MIN_AS_F64, INTERNAL_VOLTAGES_OFFSET, K_BOLTZMANN,
        LITERAL_POOL_ALIGNMENT, NativeAssignment, OperandContextFilterHelper, PARAMS_OFFSET,
        Q_ELECTRON, ROUND_TEMP_FRAME_BYTES, STACK_PROBE_INTERVAL_BYTES, THERMAL_VOLTAGE_PER_K,
        TableHelper, VECTOR_LITERAL_ALIGNMENT, VOLTAGES_OFFSET, X64Encoder, X64SsaProgram,
        XMM_STACK, Xmm, assignment_uses_helper_calls, call_result_disp,
        compile_assignment_function, compile_assignment_pass_function, compile_value_function,
        compile_value_function_artifact_from_ssa, entry_ctx_arg_reg, entry_vars_arg_reg,
        native_op_reads_entry_args, native_op_uses_helper_call, program_uses_helper_calls,
        rspice_exp, value_program_needs_saved_entry_args,
    };
    use crate::codegen::{BytecodeProgram, Instruction, LookupTable};
    use crate::jit::plan_program::PlanProgram;
    use crate::laplace::StateSpaceFilter;
    use crate::native::expr::{
        BinaryMathOp, CompareOp, EntryKind, ExtremumOp, IntegerBinaryOp, LogicalOp,
        NativeLoweringLimits, NativeOp, NativeProgram, UnaryMathOp,
    };
    use crate::native::model::CodeOffset;
    use crate::native::runtime::ExecutableMemory;
    use crate::native::{EvalContext, rspice_limexp, rspice_limited_exp};
    use crate::vm::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
    use crate::zfilter::ZiFilter;

    #[test]
    fn literal_layout_is_deduplicated_aligned_and_stable_before_emission() {
        let scalars = [
            super::LiteralPatch {
                displacement_offset: 0,
                value: 2.0,
            },
            super::LiteralPatch {
                displacement_offset: 4,
                value: 2.0,
            },
        ];
        let vectors = [super::VectorLiteralPatch {
            displacement_offset: 8,
            lanes: [1, 2],
        }];
        let first =
            super::FunctionLiteralLayout::build(7, &scalars, &vectors).expect("literal layout");
        let second = super::FunctionLiteralLayout::build(7, &scalars, &vectors)
            .expect("repeat literal layout");

        assert_eq!(first, second, "layout must converge deterministically");
        assert_eq!(first.scalar_start, 8);
        assert_eq!(first.scalar_bits.len(), 1);
        assert_eq!(first.vector_start, 16);
        assert_eq!(first.final_len, 32);
    }

    #[test]
    fn literal_pool_reuses_identical_bit_patterns() {
        let mut compiler =
            FunctionCompiler::new(false, false, false, 0, None, None, 0).expect("test frame");
        compiler.emit_literal_load(Xmm::Xmm0, 2.0);
        compiler.emit_literal_load(Xmm::Xmm1, 2.0);
        compiler.emit_literal_load(Xmm::Xmm2, 3.0);
        compiler.encoder.ret();

        let bytes = compiler
            .finish_with_literals()
            .expect("literal pool relocation succeeds")
            .bytes;

        assert_eq!(
            count_bytes(&bytes, &2.0_f64.to_le_bytes()),
            1,
            "duplicate exact literals should share one pool slot"
        );
        assert_eq!(
            count_bytes(&bytes, &3.0_f64.to_le_bytes()),
            1,
            "distinct literals should keep distinct pool slots"
        );
        let two_offset =
            find_bytes(&bytes, &2.0_f64.to_le_bytes()).expect("2.0 literal is present");
        let three_offset =
            find_bytes(&bytes, &3.0_f64.to_le_bytes()).expect("3.0 literal is present");
        assert_eq!(
            two_offset % LITERAL_POOL_ALIGNMENT,
            0,
            "first literal should be naturally aligned"
        );
        assert_eq!(
            three_offset % LITERAL_POOL_ALIGNMENT,
            0,
            "subsequent literal should stay naturally aligned"
        );
    }

    #[test]
    fn vector_literal_pool_reuses_abs_masks_and_aligns_to_16_bytes() {
        let mut compiler =
            FunctionCompiler::new(false, false, false, 0, None, None, 0).expect("test frame");
        compiler.emit_abs_register(Xmm::Xmm0);
        compiler.emit_abs_register(Xmm::Xmm1);
        compiler.encoder.ret();

        let bytes = compiler
            .finish_with_literals()
            .expect("vector literal pool relocation succeeds")
            .bytes;
        let mask = abs_value_mask_bytes();
        let mask_offset = find_bytes(&bytes, &mask).expect("abs mask literal is present");

        assert_eq!(
            count_bytes(&bytes, &mask),
            1,
            "duplicate vector masks should share one pool slot"
        );
        assert_eq!(
            mask_offset % VECTOR_LITERAL_ALIGNMENT,
            0,
            "vector literal should be naturally aligned"
        );
    }

    #[test]
    fn generated_frame_rejects_negative_or_misaligned_sizes() {
        for frame_bytes in [-16, 1, STACK_PROBE_INTERVAL_BYTES - 1] {
            assert!(
                FunctionCompiler::new(false, false, false, frame_bytes, None, None, 0).is_err(),
                "invalid generated frame size {frame_bytes} must be rejected"
            );
        }
        assert!(
            FunctionCompiler::new(false, false, false, 16, Some(16), None, 0).is_err(),
            "local slots must fit completely inside the generated frame"
        );
        assert!(
            FunctionCompiler::new(
                false,
                false,
                false,
                16,
                None,
                None,
                XMM_STACK.len() - super::CALLER_SAVED_XMM_COUNT + 1,
            )
            .is_err(),
            "callee-saved XMM reservations must be valid for the target ABI"
        );
    }

    #[test]
    fn generated_frame_probes_each_crossed_stack_page_with_a_compact_loop() {
        let small_frame_bytes = STACK_PROBE_INTERVAL_BYTES - 16;
        let small_bytes =
            FunctionCompiler::new(false, false, false, small_frame_bytes, None, None, 0)
                .expect("small aligned frame")
                .finish_assignment_pass_function()
                .expect("finish small-frame function")
                .bytes;
        assert!(contains_bytes(
            &small_bytes,
            &sub_rsp_bytes(small_frame_bytes)
        ));
        assert!(
            !contains_bytes(
                &small_bytes,
                &cmp_r64_imm32_bytes(Gpr::R11, STACK_PROBE_INTERVAL_BYTES)
            ),
            "sub-page frames should not pay for the probing loop"
        );

        let large_frame_bytes = STACK_PROBE_INTERVAL_BYTES * 3;
        let large_bytes =
            FunctionCompiler::new(false, false, false, large_frame_bytes, None, None, 0)
                .expect("large aligned frame")
                .finish_assignment_pass_function()
                .expect("finish large-frame function")
                .bytes;

        assert!(contains_bytes(
            &large_bytes,
            &cmp_r64_imm32_bytes(Gpr::R11, STACK_PROBE_INTERVAL_BYTES)
        ));
        assert_eq!(
            count_bytes(
                &large_bytes,
                &sub_r64_imm32_bytes(Gpr::R10, STACK_PROBE_INTERVAL_BYTES)
            ),
            1,
            "page probing should be encoded as a loop, not code-size-linear unrolling"
        );
        assert!(contains_bytes(&large_bytes, &stack_probe_touch_bytes()));
        assert!(contains_bytes(
            &large_bytes,
            &sub_rsp_bytes(large_frame_bytes)
        ));
        assert!(contains_bytes(
            &large_bytes,
            &add_rsp_bytes(large_frame_bytes)
        ));

        let memory = ExecutableMemory::allocate(&large_bytes).expect("allocate probed frame");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let function: extern "C" fn(*const EvalContext, *mut f64) =
            unsafe { std::mem::transmute(entry) };
        let context = eval_context(&[], &[], &[], &[]);
        function(&context, std::ptr::null_mut());
    }

    #[test]
    fn fused_stamp_kernel_inlines_value_and_jacobian_programs() {
        let value = PlanProgram::Postfix(NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadVariable(0)],
            1,
            Vec::new(),
            Vec::new(),
        ));
        let artifact = super::compile_fused_stamp_kernel_artifact(
            1024,
            CodeOffset::new(0),
            std::slice::from_ref(&value),
            &[vec![value.clone(), value.clone()]],
            &[None],
        )
        .expect("compile fused stamp kernel");
        let verified = crate::native::x64::verifier::verify_exact_function(
            &artifact.bytes[..artifact.code_len],
            "fused stamp kernel test",
        )
        .expect("verify fused stamp kernel");

        assert_eq!(
            verified.direct_call_targets.len(),
            1,
            "the assignment pass should be the fused kernel's only sibling entry call"
        );

        let mut load = X64Encoder::new();
        load.movsd_xmm_m64_base_disp32(Xmm::Xmm0, super::saved_vars_arg_reg(), 0);
        assert_eq!(
            count_bytes(&artifact.bytes[..artifact.code_len], &load.into_bytes()),
            2,
            "one value load plus one shared Jacobian load should be emitted"
        );
    }

    #[test]
    fn allocated_codegen_preserves_a_shared_ssa_operand() {
        let ssa = crate::native::x64::ir::Program::from_ssa_for_test(
            vec![
                (NativeOp::LoadVariable(0), vec![]),
                (NativeOp::Square, vec![0]),
                (NativeOp::Add, vec![0, 1]),
            ],
            2,
        )
        .expect("shared SSA program");
        let allocation =
            crate::native::x64::ir::RegisterAllocation::build(&ssa, super::X64_VALUE_BANK)
                .expect("allocate shared SSA program");
        let spill_bytes =
            super::allocation_spill_frame_bytes(&allocation).expect("shared SSA spill frame");
        let callee_saved = super::callee_saved_xmm_count_for_allocation(&allocation);
        let frame_bytes = super::checked_local_frame_bytes(&[
            spill_bytes,
            super::callee_saved_xmm_frame_bytes(callee_saved),
        ])
        .expect("shared SSA frame");
        let mut compiler = FunctionCompiler::new(
            false,
            false,
            spill_bytes > 0,
            frame_bytes,
            None,
            (spill_bytes > 0).then_some(0),
            callee_saved,
        )
        .expect("shared SSA compiler");
        compiler
            .emit_allocated_program(&ssa, &allocation)
            .expect("emit shared SSA");
        let bytes = compiler
            .finish_value_function()
            .expect("finish shared SSA")
            .bytes;

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate shared SSA leaf");
        let entry = memory.ptr_at(0).expect("shared SSA entry");
        let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let context = eval_context(&[], &[], &[], &[]);
        let variables = [3.0_f64];
        assert_eq!(
            function(&context, variables.as_ptr()).to_bits(),
            12.0_f64.to_bits()
        );
    }

    #[test]
    fn binary_helper_marshalling_preserves_swapped_xmm_arguments() {
        // The second pow is deliberately allocated with its left operand in
        // XMM1 and right operand in XMM0. Helper argument setup must implement
        // that exchange as a parallel move.
        let input = 1.42_f64;
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::Const(input),
                NativeOp::Const(1.0),
                NativeOp::Const(input),
                NativeOp::DivConst(1.45),
                NativeOp::Const(3.0),
                NativeOp::BinaryMath(BinaryMathOp::Pow),
                NativeOp::Add,
                NativeOp::Const(0.33),
                NativeOp::BinaryMath(BinaryMathOp::Pow),
                NativeOp::Div,
            ],
            4,
            Vec::new(),
            Vec::new(),
        );
        let bytes = compile_value_function(&program).expect("compile swapped-argument pow leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate swapped-argument leaf");
        let entry = memory.ptr_at(0).expect("swapped-argument entry");
        let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let context = eval_context(&[], &[], &[], &[]);

        let expected = input / (1.0 + (input / 1.45).powf(3.0)).powf(0.33);
        let actual = function(&context, std::ptr::null());
        assert!(
            (actual - expected).abs() <= 1.0e-14,
            "swapped helper arguments changed the result: expected={expected:.17e} actual={actual:.17e}"
        );
    }

    /// A loop with a genuine permutation on its back edge, executed.
    ///
    /// Two things can only be checked by running it. The back edge exchanges
    /// two loop-carried parameters, which the allocator has to sequence
    /// through its reserved scratch slot rather than overwrite in place; and a
    /// constant defined before the loop is read in the middle of the body, so
    /// its register has to survive to the next iteration even though a linear
    /// scan has passed its last use. Either mistake produces a number, not a
    /// crash, which is why the assertion is on the value.
    #[test]
    fn a_loop_with_a_swapping_back_edge_computes_what_it_says() {
        for (limit, scale, first, second) in [
            (20.0_f64, 3.0_f64, 1.0_f64, 2.0_f64),
            (100.0, 0.5, -4.0, 7.5),
            (1.0, 1.0, 0.25, 0.75),
            // Zero trips: the body never runs and the exit reads the header
            // parameters the preheader bound.
            (-1.0, 3.0, 6.0, 9.0),
        ] {
            let ssa =
                X64SsaProgram::loop_fixture_for_test(limit, scale).expect("build the loop program");
            let artifact =
                compile_value_function_artifact_from_ssa(&ssa).expect("compile the loop program");
            let memory = ExecutableMemory::allocate(artifact.bytes()).expect("publish the loop");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [first, second];
            let context = eval_context(&params, &[], &[], &[]);
            let expected = X64SsaProgram::loop_fixture_expectation(limit, scale, first, second);
            assert_eq!(
                function(&context, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "loop with limit={limit} scale={scale} first={first} second={second}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_evaluates_native_expression() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVoltage(0, 1),
                Instruction::Mul,
                Instruction::PushVariable(1),
                Instruction::PushConst(4.0),
                Instruction::Div,
                Instruction::Add,
            ],
            2,
        );
        let bytes = compile_value_function(&program).expect("compile value function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let vars = [0.0_f64, 8.0_f64];
        let ctx = eval_context(&params, &voltages, &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()), 10.0);
    }

    #[test]
    fn generated_value_leaf_uses_compact_base_displacements_for_param_load() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushParam(0)], 0);

        let bytes = compile_value_function(&program).expect("compile param value function");

        assert!(
            contains_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            "param base pointer should use the compact context field load"
        );
        assert!(
            contains_bytes(&bytes, &[0xF2, 0x0F, 0x10, 0x00]),
            "param value load at index 0 should use the no-displacement memory form"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &old_disp32_context_pointer_load_bytes(PARAMS_OFFSET)
            ),
            "param base pointer should not use the old forced disp32 context field load"
        );
        assert!(
            !contains_bytes(&bytes, &[0xF2, 0x0F, 0x10, 0x80, 0, 0, 0, 0]),
            "param value load at index 0 should not use the old forced disp32 memory form"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate param value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [7.25_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 7.25_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_for_adjacent_param_loads() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushParam(1),
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile adjacent param value leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "adjacent param loads should materialize the params base pointer once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate adjacent param leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 4.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_pure_arithmetic() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariable(0),
                Instruction::Mul,
                Instruction::PushParam(1),
                Instruction::PushVariable(1),
                Instruction::Mul,
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile fused param product leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across direct variable loads and pure arithmetic"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate fused param product leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let vars = [7.0_f64, 10.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 43.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_context_scalar_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadTemperature,
                NativeOp::Add,
                NativeOp::LoadThermalVoltage,
                NativeOp::Add,
                NativeOp::LoadTime,
                NativeOp::Add,
                NativeOp::Analysis(1),
                NativeOp::Add,
                NativeOp::LoadMfactor,
                NativeOp::Add,
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile scalar context param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across scalar context loads"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate scalar context leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let mut ctx = eval_context(&params, &[], &[], &[]);
        ctx.temperature = 300.0;
        ctx.time = 2.0;
        ctx.analysis_type = 1;
        ctx.multiplicity = 4.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.25_f64 + 300.0 + thermal_voltage(300.0) + 2.0 + 1.0 + 4.0 + 3.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_large_signal_noise_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::Const(1.0),
                NativeOp::WhiteNoise,
                NativeOp::Add,
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::FlickerNoise,
                NativeOp::Add,
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile noise param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across large-signal noise ops"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate noise param leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 4.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_inline_dynamic_variable_load() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 3,
                    lower: 1,
                },
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile dynamic variable param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across inline dynamic variable fast path"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [2.0_f64, 3.5_f64];
        let vars = [5.0_f64, 7.0_f64, 9.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_boolean_select_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::Compare(CompareOp::Eq),
                NativeOp::LoadParam(2),
                NativeOp::Logical(LogicalOp::And),
                NativeOp::LoadParam(3),
                NativeOp::LoadParam(4),
                NativeOp::IfElse,
                NativeOp::LoadParam(5),
                NativeOp::Add,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile boolean select param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across compare/logical/ifelse ops"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate boolean select leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [2.0_f64, 2.0_f64, 7.0_f64, 10.0_f64, 20.0_f64, 5.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 15.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_preserves_values_across_checked_integer_helpers() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::Neg,
                NativeOp::Abs,
                NativeOp::UnaryMath(UnaryMathOp::Floor),
                NativeOp::LoadParam(1),
                NativeOp::Extremum(ExtremumOp::Max),
                NativeOp::LoadParam(2),
                NativeOp::IntegerBinary(IntegerBinaryOp::BitAnd),
                NativeOp::ExtremumConst(ExtremumOp::Max, 2.0),
                NativeOp::LoadParam(3),
                NativeOp::Add,
                NativeOp::LoadParam(4),
                NativeOp::IntegerBinary(IntegerBinaryOp::Shl),
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile checked integer param leaf");

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate misc pure leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [6.75_f64, 4.0_f64, 3.0_f64, 5.0_f64, 1.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 14.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_without_helper_call_omits_saved_arg_prologue() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0);

        let bytes = compile_value_function(&program).expect("compile literal value function");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free native leaves should not pay callee-saved prologue cost"
        );
    }

    #[test]
    fn generated_value_leaf_uses_register_zero_for_positive_zero_constant() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(0.0)], 0);

        let bytes = compile_value_function(&program).expect("compile zero value function");

        assert!(
            contains_bytes(&bytes, &xorpd_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "positive zero constants should use a register zero idiom"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_preserves_negative_zero_constant_bits() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(-0.0)], 0);

        let bytes = compile_value_function(&program).expect("compile negative zero value function");

        assert!(
            !contains_bytes(&bytes, &xorpd_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "negative zero constants must remain literal loads to preserve sign bits"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate negative zero leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn generated_value_leaf_terminal_helper_call_omits_saved_arg_prologue() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Exp],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile helper-call value function");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "terminal pure helper-call leaves should not save unused context and vars pointers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate terminal helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_exp(0.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_terminal_idtmod_preserves_entry_args() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );

        let bytes =
            compile_value_function(&program).expect("compile terminal idtmod value function");

        assert!(
            contains_bytes(&bytes, &[0x41, 0x54, 0x41, 0x55]),
            "terminal idtmod must preserve context and vars pointers across its internal helper call"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate terminal idtmod leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let previous_state = [0.0_f64, 0.9_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();
        set_backward_euler(&mut ctx, 0.25);

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 0.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );
        assert!((state_older_candidate[1] + 0.1).abs() < 1.0e-12);
    }

    #[test]
    fn generated_value_leaf_helper_before_context_load_preserves_entry_args() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::Exp,
                Instruction::PushParam(0),
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile helper-call value function");

        assert!(
            contains_bytes(&bytes, &[0x41, 0x54, 0x41, 0x55]),
            "helper calls before later context loads must preserve context and vars pointers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate preserved helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[2.0], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (runtime_exp(0.5) + 2.0).to_bits()
        );
    }

    #[test]
    fn helper_call_metadata_preserves_entry_args_for_all_continuing_helpers() {
        let helper_ops = [
            ("unary-math", NativeOp::UnaryMath(UnaryMathOp::Exp), false),
            (
                "binary-math",
                NativeOp::BinaryMath(BinaryMathOp::Pow),
                false,
            ),
            ("table-lookup", NativeOp::TableLookup(0), false),
            ("table-derivative", NativeOp::TableDerivative(0), false),
            ("laplace", NativeOp::LaplaceState(0), false),
            (
                "laplace-derivative",
                NativeOp::LaplaceStateDerivative(0),
                false,
            ),
            (
                "zi",
                NativeOp::ZiState(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
                false,
            ),
            ("timer", NativeOp::TimerState(0), false),
            ("transition", NativeOp::TransitionState(0), false),
            (
                "transition-derivative",
                NativeOp::TransitionStateDerivative(0),
                false,
            ),
            ("slew", NativeOp::SlewState(0), false),
            ("absdelay", NativeOp::AbsDelayState(0), false),
            ("cross", NativeOp::CrossState(0), false),
            ("above", NativeOp::AboveState(0), false),
            ("last-crossing", NativeOp::LastCrossingState(0), false),
            ("idtmod", NativeOp::IdtModState(0), true),
            (
                "dynamic-variable-slow-path",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: super::INLINE_DYNAMIC_LOWER_ABS_LIMIT + 1,
                },
                false,
            ),
        ];

        for (name, op, terminal_needs_saved_entry_args) in helper_ops {
            assert!(
                native_op_uses_helper_call(&op),
                "{name} must be classified as a continuing helper call"
            );

            let terminal_program =
                NativeProgram::from_ops_for_test(vec![op], 1, Vec::new(), Vec::new());
            assert_eq!(
                value_program_needs_saved_entry_args(&terminal_program),
                terminal_needs_saved_entry_args,
                "{name}: terminal helper entry-arg preservation mismatch"
            );

            let later_entry_read = NativeProgram::from_ops_for_test(
                vec![op, NativeOp::LoadParam(0)],
                1,
                Vec::new(),
                Vec::new(),
            );
            assert!(
                value_program_needs_saved_entry_args(&later_entry_read),
                "{name} must preserve entry args when a later op reads EvalContext"
            );
        }
    }

    #[test]
    fn helper_call_metadata_preserves_entry_args_for_context_helpers_after_prior_helper() {
        let pure_helper_prefixes = [
            (
                "unary-math",
                vec![NativeOp::Const(0.25), NativeOp::UnaryMath(UnaryMathOp::Exp)],
            ),
            (
                "binary-math",
                vec![
                    NativeOp::Const(2.0),
                    NativeOp::Const(3.0),
                    NativeOp::BinaryMath(BinaryMathOp::Pow),
                ],
            ),
        ];
        let entry_arg_ops = [
            ("load-param", NativeOp::LoadParam(0)),
            ("load-var", NativeOp::LoadVariable(0)),
            ("table-lookup", NativeOp::TableLookup(0)),
            ("table-derivative", NativeOp::TableDerivative(0)),
            ("laplace", NativeOp::LaplaceState(0)),
            ("laplace-derivative", NativeOp::LaplaceStateDerivative(0)),
            (
                "zi",
                NativeOp::ZiState(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
            ),
            ("timer", NativeOp::TimerState(0)),
            ("transition", NativeOp::TransitionState(0)),
            (
                "transition-derivative",
                NativeOp::TransitionStateDerivative(0),
            ),
            ("slew", NativeOp::SlewState(0)),
            ("absdelay", NativeOp::AbsDelayState(0)),
            ("cross", NativeOp::CrossState(0)),
            ("above", NativeOp::AboveState(0)),
            ("last-crossing", NativeOp::LastCrossingState(0)),
            ("idtmod", NativeOp::IdtModState(0)),
            (
                "dynamic-variable-slow-path",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: super::INLINE_DYNAMIC_LOWER_ABS_LIMIT + 1,
                },
            ),
        ];

        for (prefix_name, prefix_ops) in pure_helper_prefixes {
            for (op_name, op) in entry_arg_ops {
                let mut ops = prefix_ops.clone();
                ops.push(op);
                let program = NativeProgram::from_ops_for_test(ops, 2, Vec::new(), Vec::new());
                assert!(
                    value_program_needs_saved_entry_args(&program),
                    "{prefix_name} before {op_name} must preserve ctx/vars across the first helper"
                );
            }
        }
    }

    #[test]
    fn generated_value_leaf_helper_before_table_helper_preserves_entry_args() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadTemperature,
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::TableLookup(0),
            ],
            1,
            Vec::new(),
            Vec::new(),
        );

        let bytes =
            compile_value_function(&program).expect("compile pure-helper before table-helper leaf");
        assert!(
            contains_bytes(&bytes, &[0x41, 0x54, 0x41, 0x55]),
            "table helper after an earlier helper must use saved ctx/vars registers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper table leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let table = [LookupTable::from_data(
            vec![0.0, 1.0, 2.0],
            vec![10.0, 20.0, 40.0],
        )];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.0;
        ctx.lookup_tables = table.as_ptr();
        ctx.lookup_tables_len = table.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 20.0_f64.to_bits());
    }

    #[test]
    fn helper_call_metadata_keeps_error_return_loads_out_of_helper_class() {
        let entry_arg_loads = [
            ("param-given", NativeOp::LoadParamGiven(0)),
            ("port-connected", NativeOp::LoadPortConnected(0)),
            ("current", NativeOp::LoadCurrent(0)),
            ("prior-current", NativeOp::LoadPriorCurrent(0)),
            (
                "dynamic-variable-inline",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: 0,
                },
            ),
        ];

        for (name, op) in entry_arg_loads {
            assert!(
                !native_op_uses_helper_call(&op),
                "{name} may hard-return on error but must not be classified as a continuing helper call"
            );
            assert!(
                native_op_reads_entry_args(op),
                "{name} must still force saved entry args after an earlier continuing helper"
            );
        }
    }

    #[test]
    fn generated_helper_call_abi_sentinels_receive_mixed_arguments() {
        let table = [
            LookupTable::from_data(vec![0.0, 1.0], vec![2.0, 3.0]),
            LookupTable::from_data(vec![0.0, 1.0], vec![4.0, 5.0]),
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.lookup_tables = table.as_ptr();
        ctx.lookup_tables_len = table.len();
        ctx.time = 12.0;
        ctx.temperature = 315.0;
        ctx.multiplicity = 3.0;

        assert_f64_matches(
            run_table_helper_sentinel(abi_sentinel_table_helper, &ctx),
            101.0,
            "table helper ABI sentinel",
        );
        assert_f64_matches(
            run_context_filter_helper_sentinel(abi_sentinel_context_filter_helper, &ctx),
            202.0,
            "context filter helper ABI sentinel",
        );
        assert_f64_matches(
            run_timer_helper_sentinel(abi_sentinel_timer_helper, &ctx),
            303.0,
            "timer helper ABI sentinel",
        );
        assert_f64_matches(
            run_operand_context_filter_helper_sentinel(
                abi_sentinel_operand_context_filter_helper,
                &ctx,
            ),
            404.0,
            "operand context filter helper ABI sentinel",
        );
    }

    #[test]
    fn generated_helper_call_abi_sentinels_enter_helpers_with_aligned_stack() {
        let alignment_helper_memory = stack_alignment_helper_memory();
        let helper_ptr = alignment_helper_memory
            .ptr_at(0)
            .expect("alignment helper entry point");
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.time = 12.0;
        ctx.temperature = 315.0;
        ctx.multiplicity = 3.0;

        let table_helper: TableHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_table_helper_sentinel(table_helper, &ctx),
            1.0,
            "table helper stack alignment",
        );

        let context_filter_helper: ContextFilterHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_context_filter_helper_sentinel(context_filter_helper, &ctx),
            1.0,
            "context filter helper stack alignment",
        );

        let timer_helper: OperandContextFilterHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_timer_helper_sentinel(timer_helper, &ctx),
            1.0,
            "timer helper stack alignment",
        );

        let operand_context_filter_helper: OperandContextFilterHelper =
            unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_operand_context_filter_helper_sentinel(operand_context_filter_helper, &ctx),
            1.0,
            "operand context filter helper stack alignment",
        );
    }

    #[test]
    fn generated_value_leaf_no_spill_unary_helper_omits_call_frame_base_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Exp],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile unary helper value function");
        let frame_bytes = call_frame_bytes(0);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "no-spill helper calls must reserve the minimum ABI call frame"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "no-spill helper calls must release the minimum ABI call frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "unary helper calls without preserved XMM values should not materialize a call-frame base"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate unary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_exp(0.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_no_spill_binary_helper_omits_call_frame_base_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::PushTime,
                Instruction::Pow,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile binary helper value function");
        let frame_bytes = call_frame_bytes(0);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "no-spill helper calls must reserve the minimum ABI call frame"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "no-spill helper calls must release the minimum ABI call frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "binary helper calls without preserved XMM values should not materialize a call-frame base"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate binary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 2.0;
        ctx.time = 3.0;

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 8.0_f64.to_bits());
    }

    #[cfg(windows)]
    #[test]
    fn generated_value_leaf_preserves_win64_callee_saved_xmm_stack_slots() {
        let prefix = variable_prefix(0, XMM_STACK.len());
        let mut instructions = prefix.clone();
        instructions.extend(add_reductions(prefix.len() - 1));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes = compile_value_function(&program).expect("compile full-depth value function");
        let ssa = crate::native::x64::ir::Program::lower(&program).expect("lower full-depth SSA");
        let allocation =
            crate::native::x64::ir::RegisterAllocation::build(&ssa, super::X64_VALUE_BANK)
                .expect("allocate full-depth SSA");
        let callee_saved_count = super::callee_saved_xmm_count_for_allocation(&allocation);
        let frame_bytes = super::checked_local_frame_bytes(&[
            super::allocation_spill_frame_bytes(&allocation).expect("liveness spill frame fits"),
            super::callee_saved_xmm_frame_bytes(callee_saved_count),
        ])
        .expect("allocated XMM frame fits");
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "full-depth Win64 value leaf should reserve callee-saved XMM spill space"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "full-depth Win64 value leaf should release callee-saved XMM spill space"
        );

        for index in 0..callee_saved_count {
            let register = super::callee_saved_xmm_register(index);
            let disp = frame_bytes - super::callee_saved_xmm_frame_bytes(callee_saved_count)
                + (index as i32 * super::CALLEE_SAVED_XMM_BYTES);
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_store_bytes(register, disp)),
                1,
                "{register:?} should be saved exactly once in the prologue"
            );
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_load_bytes(register, disp)),
                1,
                "{register:?} should be restored exactly once in the epilogue"
            );
        }

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-depth value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        let vars = (0..prefix.len())
            .map(|index| (index + 1) as f64)
            .collect::<Vec<_>>();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            constant_prefix_sum(prefix.len()).to_bits()
        );
    }

    /// Values live across a helper call belong in the preserved registers, and
    /// a function that puts them there owes the prologue a save.
    ///
    /// The two halves are asserted together on purpose, because each one is
    /// what makes the other safe. The allocator may park a value in XMM6-XMM9
    /// only because Win64 obliges `exp` to give those registers back; and the
    /// same rule obliges this generated function, which is itself somebody's
    /// callee, to hand them back to whoever called it. Occupying one without
    /// the matching prologue save silently corrupts the caller's register --
    /// which is why the save count is asserted directly rather than left to a
    /// loop that checks nothing when it happens to be zero.
    #[cfg(windows)]
    #[test]
    fn win64_values_live_across_a_helper_call_occupy_registers_the_prologue_preserves() {
        let mut instructions = variable_prefix(0, 5);
        instructions.push(Instruction::Exp);
        instructions.extend(add_reductions(4));
        let program = native_program(EntryKind::StampValue, instructions, 0);

        let ssa = crate::native::x64::ir::Program::lower(&program).expect("lower cross-call SSA");
        let allocation =
            crate::native::x64::ir::RegisterAllocation::build(&ssa, super::X64_VALUE_BANK)
                .expect("allocate cross-call SSA");
        let live_across_call: Vec<crate::native::x64::ir::ValueLocation> = allocation
            .instructions()[..4]
            .iter()
            .map(crate::native::x64::ir::AllocatedInstruction::result)
            .collect();
        assert_eq!(
            live_across_call,
            vec![
                crate::native::x64::ir::ValueLocation::Register(6),
                crate::native::x64::ir::ValueLocation::Register(7),
                crate::native::x64::ir::ValueLocation::Register(8),
                crate::native::x64::ir::ValueLocation::Register(9),
            ],
            "the four variables the exp() call sits between belong in XMM6-XMM9"
        );
        assert_eq!(
            allocation.spill_slot_count(),
            0,
            "nothing needs a spill slot while a preserved register is free"
        );

        let bytes = compile_value_function(&program).expect("compile cross-call value function");
        let callee_saved_count = super::callee_saved_xmm_count_for_allocation(&allocation);
        assert_eq!(
            callee_saved_count, 4,
            "XMM6-XMM9 carry values here, so the prologue owes exactly four saves"
        );
        let frame_bytes = super::checked_local_frame_bytes(&[
            super::allocation_spill_frame_bytes(&allocation).expect("liveness spill frame fits"),
            super::callee_saved_xmm_frame_bytes(callee_saved_count),
        ])
        .expect("allocated XMM frame fits");
        for index in 0..callee_saved_count {
            let register = super::callee_saved_xmm_register(index);
            let disp = frame_bytes - super::callee_saved_xmm_frame_bytes(callee_saved_count)
                + (index as i32 * super::CALLEE_SAVED_XMM_BYTES);
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_store_bytes(register, disp)),
                1,
                "{register:?} holds a value across the call and must be saved exactly once"
            );
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_load_bytes(register, disp)),
                1,
                "{register:?} must be restored exactly once"
            );
        }

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate cross-call value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        // exp(0) is exactly 1.0, so the whole sum is exact in binary floating
        // point and a survivor that came back subtly wrong still fails here.
        let vars = [1.0, 2.0, 3.0, 4.0, 0.0];

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 11.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_helper_result_stays_in_target_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushParam(0),
                Instruction::Exp,
                Instruction::Add,
            ],
            0,
        );

        assert_eq!(
            program.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::LoadParam(0),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
                NativeOp::Add,
            ],
            "test fixture must force a unary helper with xmm0 live and xmm1 as target"
        );

        let bytes = compile_value_function(&program).expect("compile helper result leaf");
        let frame_bytes = call_frame_bytes(0);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "a prefix already assigned a liveness home should not consume a call-frame slot"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "a prefix already assigned a liveness home should use the minimum call frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "single-prefix helper call should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "single-prefix helper call should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "helper calls should address spills directly from rsp instead of materializing a call-frame base"
        );
        let first_call_spill = call_frame_spill_bytes(0, Xmm::Xmm0);
        #[cfg(windows)]
        assert!(
            !contains_bytes(&bytes, &first_call_spill),
            "live-across-call values should use their reusable liveness home"
        );
        #[cfg(not(windows))]
        assert_eq!(
            count_bytes(&bytes, &first_call_spill),
            1,
            "System V encodes the local liveness-home store exactly like call spill slot zero; one store is the home, a second would be a duplicate call spill"
        );
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm0, 0)),
            "the helper call should not duplicate a value already held in its liveness home"
        );

        let mut old_result_store = X64Encoder::new();
        old_result_store.movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        assert!(
            !contains_bytes(&bytes, &old_result_store.into_bytes()),
            "helper result should not be spilled to the call frame"
        );

        let mut old_result_reload = X64Encoder::new();
        old_result_reload.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::R11, call_result_disp());
        assert!(
            !contains_bytes(&bytes, &old_result_reload.into_bytes()),
            "helper result should move directly into the target register"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper result leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[2.0], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0_f64 + runtime_exp(2.0)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Add, 12.0_f64),
            (Instruction::Sub, 4.0_f64),
            (Instruction::Mul, 32.0_f64),
            (Instruction::Div, 2.0_f64),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal RHS instruction, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal RHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS arithmetic should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 8.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_doubles_constant_rhs_mul_without_literal_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushConst(2.0),
                Instruction::Mul,
            ],
            0,
        );
        assert_eq!(
            program.ops(),
            &[NativeOp::LoadParam(0), NativeOp::MulConst(2.0)],
            "x * 2.0 should stay a literal RHS multiply in native IR"
        );
        assert_eq!(
            program.max_stack_depth(),
            1,
            "x * 2.0 should not allocate an RHS stack slot"
        );

        let bytes = compile_value_function(&program).expect("compile constant double leaf");
        assert!(
            contains_bytes(&bytes, &addsd_xmm_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "x * 2.0 should emit an in-place addsd"
        );
        assert!(
            !contains_bytes(&bytes, &mulsd_xmm_m64_rip_prefix_bytes(Xmm::Xmm0)),
            "x * 2.0 should not emit a literal-pool mulsd"
        );
        assert!(
            !contains_bytes(&bytes, &2.0_f64.to_le_bytes()),
            "x * 2.0 should not append a 2.0 literal"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate constant double leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let cases = [
            ("finite", 3.25_f64),
            ("negative-zero", -0.0_f64),
            ("positive-infinity", f64::INFINITY),
            ("overflow", f64::MAX),
            ("unordered", f64::from_bits(0x7ff0_0000_0000_1111)),
        ];

        for (name, input) in cases {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_f64_matches(f(&ctx, std::ptr::null()), input * 2.0, name);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_commutative_arithmetic_without_extra_stack_slot() {
        let cases = [
            (
                "add-finite",
                Instruction::Add,
                10.0_f64,
                3.25_f64,
                10.0_f64 + 3.25_f64,
            ),
            (
                "mul-finite",
                Instruction::Mul,
                -3.5_f64,
                4.0_f64,
                -3.5_f64 * 4.0_f64,
            ),
            (
                "add-snan-rhs",
                Instruction::Add,
                10.0_f64,
                f64::from_bits(0x7ff0_0000_0000_1111),
                10.0_f64 + f64::from_bits(0x7ff0_0000_0000_1111),
            ),
        ];

        for (name, instruction, lhs, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS commutative op, not a second stack slot"
            );

            let bytes = compile_value_function(&program)
                .expect("compile literal LHS commutative arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS commutative arithmetic should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate literal LHS commutative arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, name);
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_binary_arithmetic_to_literal() {
        let cases = [
            (
                "add finite",
                Instruction::Add,
                3.25_f64,
                4.5_f64,
                3.25_f64 + 4.5_f64,
            ),
            (
                "sub finite",
                Instruction::Sub,
                3.25_f64,
                4.5_f64,
                3.25_f64 - 4.5_f64,
            ),
            (
                "mul signed zero",
                Instruction::Mul,
                -0.0_f64,
                4.5_f64,
                -0.0_f64 * 4.5_f64,
            ),
            (
                "div negative zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                10.0_f64 / -0.0_f64,
            ),
            (
                "mul unordered",
                Instruction::Mul,
                f64::from_bits(0x7ff8_0000_0000_0003),
                4.5_f64,
                f64::from_bits(0x7ff8_0000_0000_0003) * 4.5_f64,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            match program.ops() {
                [NativeOp::Const(value)] => {
                    assert_f64_matches(*value, expected, case);
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }

            let bytes =
                compile_value_function(&program).expect("compile folded arithmetic literal leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded arithmetic literal should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate folded arithmetic literal leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, case);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_sub_div_without_extra_stack_slot() {
        let temp_sub_rsp = sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let temp_add_rsp = add_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let cases = [
            (
                "sub-finite",
                Instruction::Sub,
                10.0_f64,
                3.0_f64,
                10.0_f64 - 3.0_f64,
            ),
            (
                "sub-signed-zero",
                Instruction::Sub,
                -0.0_f64,
                0.0_f64,
                -0.0_f64 - 0.0_f64,
            ),
            (
                "sub-unordered",
                Instruction::Sub,
                10.0_f64,
                f64::from_bits(0x7ff8_0000_0000_0001),
                10.0_f64 - f64::from_bits(0x7ff8_0000_0000_0001),
            ),
            (
                "div-finite",
                Instruction::Div,
                10.0_f64,
                4.0_f64,
                10.0_f64 / 4.0_f64,
            ),
            (
                "div-negative-zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                10.0_f64 / -0.0_f64,
            ),
            (
                "div-unordered",
                Instruction::Div,
                10.0_f64,
                f64::from_bits(0x7ff8_0000_0000_0002),
                10.0_f64 / f64::from_bits(0x7ff8_0000_0000_0002),
            ),
        ];

        for (name, instruction, lhs, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS arithmetic op, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal LHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS arithmetic should stay helper-free"
            );
            assert!(
                !contains_bytes(&bytes, &temp_sub_rsp),
                "{name} should use an XMM scratch instead of an RSP temp slot"
            );
            assert!(
                !contains_bytes(&bytes, &temp_add_rsp),
                "{name} should not restore an unused RSP temp slot"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, name);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_arithmetic_at_full_xmm_stack_depth() {
        let temp_sub_rsp = sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let temp_add_rsp = add_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let cases = [
            ("sub", Instruction::Sub, 4.0_f64, 10.0_f64 - 4.0_f64),
            ("div", Instruction::Div, 4.0_f64, 10.0_f64 / 4.0_f64),
        ];

        for (name, instruction, input, folded_value) in cases {
            let prefix = constant_prefix(XMM_STACK.len() - 1);
            let mut instructions = prefix.clone();
            instructions.extend([
                Instruction::PushConst(10.0),
                Instruction::PushTemperature,
                instruction,
            ]);
            instructions.extend(add_reductions(prefix.len()));
            let program = native_program(EntryKind::StampValue, instructions, 0);

            assert_eq!(program.max_stack_depth(), XMM_STACK.len(), "{name}");

            let bytes = compile_value_function(&program)
                .expect("compile full-stack literal LHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS arithmetic should stay helper-free at full XMM stack depth"
            );
            assert!(
                !contains_bytes(&bytes, &temp_sub_rsp),
                "{name} should use an allocator-provided scratch register"
            );
            assert!(
                !contains_bytes(&bytes, &temp_add_rsp),
                "{name} should not create an instruction-local spill frame"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate full-stack literal LHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            let expected = constant_prefix_sum(prefix.len()) + folded_value;
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, 8.0_f64, 1.0_f64),
            (Instruction::Ge, 4.0_f64, 1.0_f64),
            (Instruction::Lt, 2.0_f64, 1.0_f64),
            (Instruction::Le, 4.0_f64, 1.0_f64),
            (Instruction::Eq, 4.0_f64, 1.0_f64),
            (Instruction::Ne, 8.0_f64, 1.0_f64),
            (Instruction::Lt, f64::NAN, 0.0_f64),
            (Instruction::Le, f64::NAN, 0.0_f64),
            (Instruction::Eq, f64::NAN, 0.0_f64),
            (Instruction::Ne, f64::NAN, 1.0_f64),
        ];

        for (instruction, input, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal RHS compare, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal RHS comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_compares_zero_rhs_without_literal_subtract() {
        let cases = [
            ("eq-positive-zero", Instruction::Eq, CompareOp::Eq, 0.0),
            ("eq-negative-zero", Instruction::Eq, CompareOp::Eq, -0.0),
            ("ne-positive-zero", Instruction::Ne, CompareOp::Ne, 0.0),
            ("ne-negative-zero", Instruction::Ne, CompareOp::Ne, -0.0),
        ];
        let inputs = [
            ("positive-zero", 0.0_f64),
            ("negative-zero", -0.0_f64),
            ("tiny-nonzero", 0.5e-15_f64),
            ("larger-nonzero", 1.0e-15_f64),
            ("infinity", f64::INFINITY),
            ("unordered", f64::NAN),
        ];

        for (name, instruction, expected_op, literal) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(literal),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::CompareConst(expected_op, literal)
                ],
                "{name} should lower to a literal RHS comparison"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS compare, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile zero literal comparison leaf");
            assert!(
                !contains_bytes(&bytes, &subsd_xmm_m64_rip_prefix_bytes(Xmm::Xmm0)),
                "{name} should compare directly against zero"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            for (input_name, input) in inputs {
                let params = [input];
                let ctx = eval_context(&params, &[], &[], &[]);
                let expected: f64 = match expected_op {
                    CompareOp::Eq => {
                        if input == literal {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    CompareOp::Ne => {
                        if input != literal {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    CompareOp::Gt | CompareOp::Ge | CompareOp::Lt | CompareOp::Le => {
                        unreachable!("zero comparison test only covers equality operators")
                    }
                };

                assert_eq!(
                    f(&ctx, std::ptr::null()).to_bits(),
                    expected.to_bits(),
                    "{name}:{input_name}"
                );
            }
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Lt, 2.0_f64, 1.0_f64),
            (Instruction::Ge, CompareOp::Le, 4.0_f64, 1.0_f64),
            (Instruction::Lt, CompareOp::Gt, 8.0_f64, 1.0_f64),
            (Instruction::Le, CompareOp::Ge, 4.0_f64, 1.0_f64),
            (Instruction::Eq, CompareOp::Eq, 4.0_f64, 1.0_f64),
            (Instruction::Ne, CompareOp::Ne, 8.0_f64, 1.0_f64),
            (Instruction::Gt, CompareOp::Lt, f64::NAN, 0.0_f64),
            (Instruction::Ge, CompareOp::Le, f64::NAN, 0.0_f64),
            (Instruction::Eq, CompareOp::Eq, f64::NAN, 0.0_f64),
            (Instruction::Ne, CompareOp::Ne, f64::NAN, 1.0_f64),
        ];

        for (instruction, expected_op, input, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(4.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal LHS compare, not a second stack slot"
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected_op, 4.0)
                ],
                "{instruction_name} should flip the comparison around the literal LHS"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal LHS comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_comparisons_to_literals() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 4.0, 1.0),
            ("gt-unordered", Instruction::Gt, f64::NAN, 4.0, 0.0),
            ("ge-equal", Instruction::Ge, 4.0, 4.0, 1.0),
            ("lt-true", Instruction::Lt, 3.0, 4.0, 1.0),
            ("le-equal", Instruction::Le, 4.0, 4.0, 1.0),
            ("le-unordered", Instruction::Le, 4.0, f64::NAN, 0.0),
            ("eq-tiny-nonzero", Instruction::Eq, 0.0, 0.5e-15, 0.0),
            ("eq-larger-nonzero", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-unordered", Instruction::Eq, f64::NAN, 0.0, 0.0),
            ("ne-tiny-nonzero", Instruction::Ne, 0.0, 0.5e-15, 1.0),
            ("ne-larger-nonzero", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-unordered", Instruction::Ne, 0.0, f64::NAN, 1.0),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_squares_constant_power_without_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile square power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-square power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate square power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -3.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 9.0_f64.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_elides_identity_power_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile identity power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-one power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate identity power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -0.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), (-0.0_f64).to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_elides_reciprocal_power_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(-1.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile reciprocal power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-minus-one power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate reciprocal power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -0.0;

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                f64::NEG_INFINITY.to_bits()
            );
        }
    }

    #[test]
    fn generated_value_leaf_loads_dynamic_variable_and_preserves_stack() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic read fast path should not pay helper-call prologue cost"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "dynamic read with a spare XMM register should not spill the index before the fast path"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "dynamic read with a spare XMM register should not restore a fast-path spill frame"
        );
        assert!(
            contains_bytes(&bytes, &dynamic_variable_scaled_address_bytes(1)),
            "dynamic read fast path should use one scaled indexed LEA for variable address formation"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_shift_add_address_bytes(1)),
            "dynamic read fast path should not use the old shift/add address sequence"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 1)),
            "dynamic read fast path should subtract small lower bounds as an imm32"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 3)),
            "dynamic read fast path should compare small lengths as an imm32"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(1)),
            "dynamic read fast path should not materialize small lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_cmp_len_bytes(3)),
            "dynamic read fast path should not materialize small lengths in a GPR"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::dynamic_variable_len_arg_reg(), 3)
            ),
            "dynamic read helper slow path should materialize small lengths with a compact imm32 move"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::dynamic_variable_lower_arg_reg(), 1)
            ),
            "dynamic read helper slow path should materialize small positive lower bounds with a compact imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_len_arg_reg(), 3)
            ),
            "dynamic read helper slow path should not materialize small lengths with movabs"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_lower_arg_reg(), 1)
            ),
            "dynamic read helper slow path should not materialize small positive lower bounds with movabs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        ctx.clear_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 5.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_helper_free_dynamic_variable_read_hard_fails_bounds() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic read slow path should not require the helper-call prologue"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "out-of-range dynamic read with a spare XMM register should still avoid the old spill frame"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[4.0], &[], &[], &[]);
        ctx.clear_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("out-of-range dynamic read must hard-fail in its dispatch");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve array bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_value_leaf_zero_based_dynamic_variable_read_skips_lower_subtract() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 0,
                    len: 4,
                    lower: 0,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile zero-based dynamic read");
        assert!(
            !contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 0)),
            "zero-based dynamic read fast path should not emit a no-op lower-bound subtract"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 4)),
            "zero-based dynamic read fast path should still bounds-check the normalized index"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero-based dynamic read");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [2.0_f64, 4.0, 8.0, 16.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        ctx.clear_runtime_error();

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_dynamic_variable_read_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushParam(0),
            Instruction::PushVariableDyn {
                base: 1,
                len: 3,
                lower: 1,
            },
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes =
            compile_value_function(&program).expect("compile full-stack dynamic variable leaf");
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "liveness allocation should avoid the instruction-local dynamic-read spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "dynamic-read operands should use their allocated homes"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        ctx.clear_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(
            loaded.to_bits(),
            (constant_prefix_sum(prefix.len()) + 4.0).to_bits()
        );
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_keeps_huge_dynamic_variable_ranges_on_helper_path() {
        let huge_len = (1_usize << 52) + 1;
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushParam(0),
                    Instruction::PushVariableDyn {
                        base: 0,
                        len: huge_len,
                        lower: 0,
                    },
                ],
            },
            NativeLoweringLimits::new(0, 0, 1, huge_len, 0),
        )
        .expect("huge dynamic range is valid IR before x64 lowering");
        let bytes = compile_value_function(&program).expect("compile huge dynamic range leaf");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "terminal huge dynamic helper leaves should not save unused context and vars pointers"
        );
        assert!(
            contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_len_arg_reg(), huge_len as u64)
            ),
            "huge dynamic ranges must still use the helper-backed continuation path"
        );
    }

    #[test]
    fn generated_value_leaf_folds_constant_dynamic_variable_read_to_direct_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.49),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
                Instruction::Add,
            ],
            0,
        );

        assert_eq!(
            program.ops(),
            &[NativeOp::LoadVariable(2), NativeOp::AddConst(1.0)],
            "finite in-range literal dynamic index should lower to direct variable load"
        );

        let bytes = compile_value_function(&program).expect("compile folded dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "folded dynamic read should be helper-free"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate folded dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 5.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_hard_fails_dynamic_variable_bounds_errors() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(4.0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("out-of-range native array read must hard-fail in its dispatch");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_assignment_leaf_stores_native_expression_result() {
        let program = native_program(
            EntryKind::Assignment,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVoltage(1, 0),
                Instruction::Add,
            ],
            2,
        );
        let bytes = compile_assignment_function(2, &program).expect("compile assignment function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate assignment leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [3.0_f64];
        let voltages = [1.0_f64, 6.0_f64];
        let mut vars = [0.0_f64; 3];
        let ctx = eval_context(&params, &voltages, &[], &[]);

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars[2], 8.0);
    }

    #[test]
    fn generated_assignment_pass_stores_indexed_value_and_preserves_slot_across_helper_call() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushConst(2.49)], 0),
                value: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(2.0), Instruction::Exp],
                    0,
                ),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![
                        Instruction::PushVariable(2),
                        Instruction::PushConst(1.0),
                        Instruction::Add,
                    ],
                    0,
                ),
            },
        ];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile indexed assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate indexed assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        let expected = runtime_exp(2.0);
        assert_eq!(vars[2].to_bits(), expected.to_bits());
        assert_eq!(vars[0].to_bits(), (expected + 1.0).to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_indexed_assignment_stores_runtime_slot() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
                value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(123.0)],
                    0,
                ),
            },
        ];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "supported runtime indexed writes should not force the helper-call prologue"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile helper-free indexed assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free indexed writes should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 1)),
            "indexed write fast path should subtract small lower bounds as an imm32"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 3)),
            "indexed write fast path should compare small lengths as an imm32"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(1)),
            "indexed write fast path should not materialize small lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_cmp_len_bytes(3)),
            "indexed write fast path should not materialize small lengths in a GPR"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate helper-free indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [2.49_f64, 11.0];
        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [123.0, 2.0, 11.0, 8.0]);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_indexed_assignment_handles_negative_lower_bound() {
        let assignments = [NativeAssignment::Indexed {
            base: 1,
            len: 3,
            lower: -2,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
        }];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "supported negative lower-bound indexed writes should not require the helper"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile negative lower-bound indexed assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free indexed writes should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, -2)),
            "indexed write fast path should subtract negative lower bounds with a sign-extended imm32"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r64_imm32_bytes(super::dynamic_variable_lower_arg_reg(), -2)
            ),
            "indexed write helper slow path should materialize small negative lower bounds with a sign-extended imm32 move"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(-2)),
            "indexed write fast path should not materialize small negative lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_lower_arg_reg(), (-2_i64) as u64)
            ),
            "indexed write helper slow path should not materialize small negative lower bounds with movabs"
        );
        let memory = ExecutableMemory::allocate(&bytes)
            .expect("allocate negative lower-bound indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [-1.5_f64, 12.0];
        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0, 12.0, 4.0, 8.0]);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_zero_based_indexed_assignment_skips_lower_subtract() {
        let assignments = [NativeAssignment::Indexed {
            base: 0,
            len: 4,
            lower: 0,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
        }];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile zero-based indexed assignment pass");
        assert!(
            !contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 0)),
            "zero-based indexed write fast path should not emit a no-op lower-bound subtract"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 4)),
            "zero-based indexed write fast path should still bounds-check the normalized index"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate zero-based indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [2.49_f64, 13.0];
        let mut vars = [2.0_f64, 4.0, 8.0, 16.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [2.0, 4.0, 13.0, 16.0]);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_keeps_huge_indexed_variable_ranges_on_helper_path() {
        let huge_len = (1_usize << 52) + 1;
        let assignments = [NativeAssignment::Indexed {
            base: 0,
            len: huge_len,
            lower: 0,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushConst(11.0)], 0),
        }];
        assert!(
            assignment_uses_helper_calls(&assignments[0]),
            "huge indexed ranges must keep helper-backed continuation semantics"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile huge indexed range assignment pass");

        assert!(
            contains_bytes(&bytes, &[0x41, 0x54, 0x41, 0x55]),
            "huge indexed ranges must keep the helper-call prologue"
        );
    }

    #[test]
    fn generated_assignment_pass_hard_fails_indexed_assignment_bounds_errors() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushConst(4.0)], 0),
                value: native_program(EntryKind::Assignment, vec![Instruction::PushConst(11.0)], 0),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(123.0)],
                    0,
                ),
            },
        ];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile indexed assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate indexed assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0, 2.0, 4.0, 8.0]);
        let error = ctx
            .take_runtime_error()
            .expect("out-of-range native indexed write must hard-fail in its dispatch");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve array bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_assignment_pass_executes_nested_loops_with_indexed_body() {
        let assignments = [
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(0.0)],
                    0,
                ),
            },
            NativeAssignment::Direct {
                var_index: 2,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(0.0)],
                    0,
                ),
            },
            NativeAssignment::Loop {
                condition: native_program(
                    EntryKind::Assignment,
                    vec![
                        Instruction::PushVariable(0),
                        Instruction::PushConst(2.0),
                        Instruction::Lt,
                    ],
                    0,
                ),
                body: vec![
                    NativeAssignment::Direct {
                        var_index: 1,
                        program: native_program(
                            EntryKind::Assignment,
                            vec![Instruction::PushConst(0.0)],
                            0,
                        ),
                    },
                    NativeAssignment::Loop {
                        condition: native_program(
                            EntryKind::Assignment,
                            vec![
                                Instruction::PushVariable(1),
                                Instruction::PushConst(2.0),
                                Instruction::Lt,
                            ],
                            0,
                        ),
                        body: vec![
                            NativeAssignment::Indexed {
                                base: 3,
                                len: 2,
                                lower: 1,
                                index: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                                value: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(0),
                                        Instruction::PushConst(10.0),
                                        Instruction::Mul,
                                        Instruction::PushVariable(1),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                            NativeAssignment::Direct {
                                var_index: 2,
                                program: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(2),
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                        Instruction::PushVariableDyn {
                                            base: 3,
                                            len: 2,
                                            lower: 1,
                                        },
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                            NativeAssignment::Direct {
                                var_index: 1,
                                program: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                        ],
                    },
                    NativeAssignment::Direct {
                        var_index: 0,
                        program: native_program(
                            EntryKind::Assignment,
                            vec![
                                Instruction::PushVariable(0),
                                Instruction::PushConst(1.0),
                                Instruction::Add,
                            ],
                            0,
                        ),
                    },
                ],
            },
        ];
        let bytes =
            compile_assignment_pass_function(&assignments).expect("compile loop assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate loop assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64; 5];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [2.0, 2.0, 22.0, 10.0, 11.0]);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_loop_omits_saved_arg_prologue_and_hard_fails_limit() {
        let assignments = [
            NativeAssignment::Loop {
                condition: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(1.0)],
                    0,
                ),
                body: Vec::new(),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(99.0)],
                    0,
                ),
            },
        ];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "helper-free runtime loops should not force the helper-call prologue"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile helper-free infinite loop assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free runtime loops should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &xor_r64_bytes(Gpr::R10, Gpr::R10)),
            "runtime loop counter should be zeroed with a zero idiom"
        );
        assert!(
            !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R10, 0)),
            "runtime loop counter should not materialize zero as a 64-bit immediate"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate infinite loop assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64];
        let ctx = eval_context(&[], &[], &[], &[]);
        ctx.clear_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0]);
        let error = ctx.take_runtime_error().expect("loop limit must hard-fail");
        assert!(
            error.contains("native runtime loop iteration limit exceeded"),
            "error must preserve loop-limit diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_value_leaf_handles_ground_voltage_without_memory_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushVoltage(usize::MAX, 0)],
            1,
        );
        let bytes = compile_value_function(&program).expect("compile ground voltage function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ground voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let voltages = [6.25_f64];
        let ctx = eval_context(&[], &voltages, &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), -6.25);
    }

    #[test]
    fn generated_value_leaf_handles_unified_internal_voltage_pairs() {
        let terminals = [10.0_f64, 4.0_f64];
        let internals = [7.0_f64, 3.0_f64];
        let cases = [
            ("terminal-internal", Instruction::PushVoltage(0, 2), 3.0_f64),
            (
                "ground-internal",
                Instruction::PushVoltage(usize::MAX, 2),
                -7.0_f64,
            ),
            (
                "internal-ground",
                Instruction::PushVoltage(3, usize::MAX),
                3.0_f64,
            ),
            ("internal-internal", Instruction::PushVoltage(2, 3), 4.0_f64),
        ];

        for (name, instruction, expected) in cases {
            let program =
                native_program_with_internals(EntryKind::StampValue, vec![instruction], 2, 2);
            let bytes =
                compile_value_function(&program).expect("compile unified internal voltage leaf");
            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate unified internal voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &terminals, &internals, &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_reuses_voltage_base_for_same_storage_pairs() {
        let cases = [
            (
                "terminal-terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(0, 1)],
                    2,
                ),
                eval_context(&[], &[9.0, 4.0], &[], &[]),
                VOLTAGES_OFFSET,
                INTERNAL_VOLTAGES_OFFSET,
                5.0_f64,
            ),
            (
                "internal-internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(2, 3)],
                    2,
                    2,
                ),
                eval_context(&[], &[0.0, 0.0], &[8.0, 3.0], &[]),
                INTERNAL_VOLTAGES_OFFSET,
                VOLTAGES_OFFSET,
                5.0_f64,
            ),
        ];

        for (name, program, ctx, reused_offset, unused_offset, expected) in cases {
            let bytes =
                compile_value_function(&program).expect("compile same-storage voltage leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name} should stay helper-free"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} should load its voltage base pointer once"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(unused_offset)),
                0,
                "{name} should not touch the other voltage storage"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate same-storage voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_reuses_voltage_bases_across_pure_arithmetic() {
        let cases = [
            (
                "terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(0, 1),
                        Instruction::PushVoltage(2, 3),
                        Instruction::Add,
                    ],
                    4,
                ),
                eval_context(&[], &[9.0, 4.0, 12.0, 2.0], &[], &[]),
                VOLTAGES_OFFSET,
                INTERNAL_VOLTAGES_OFFSET,
                15.0_f64,
            ),
            (
                "internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(4, 5),
                        Instruction::PushVoltage(6, 7),
                        Instruction::Add,
                    ],
                    4,
                    4,
                ),
                eval_context(&[], &[0.0, 0.0, 0.0, 0.0], &[8.0, 3.0, 11.0, 2.0], &[]),
                INTERNAL_VOLTAGES_OFFSET,
                VOLTAGES_OFFSET,
                14.0_f64,
            ),
        ];

        for (name, program, ctx, reused_offset, unused_offset, expected) in cases {
            let bytes =
                compile_value_function(&program).expect("compile repeated voltage expression leaf");
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} repeated voltage loads should materialize their base pointer once"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(unused_offset)),
                0,
                "{name} repeated voltage loads should not touch the other voltage storage"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate repeated voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_keeps_mixed_storage_voltage_base_loads_separate() {
        let program = native_program_with_internals(
            EntryKind::StampValue,
            vec![Instruction::PushVoltage(0, 2)],
            2,
            1,
        );

        let bytes = compile_value_function(&program).expect("compile mixed-storage voltage leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "mixed-storage voltage load should stay helper-free"
        );
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(VOLTAGES_OFFSET)),
            1,
            "mixed-storage voltage load should read terminal voltage storage once"
        );
        assert_eq!(
            count_bytes(
                &bytes,
                &context_pointer_load_bytes(INTERNAL_VOLTAGES_OFFSET)
            ),
            1,
            "mixed-storage voltage load should read internal voltage storage once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate mixed voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[11.0, 0.0], &[2.5], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 8.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_same_node_voltage_subtracts_in_register() {
        let cases = [
            (
                "terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(0, 0)],
                    1,
                ),
                VOLTAGES_OFFSET,
                0,
                vec![
                    (6.25_f64, 0.0_f64),
                    (-0.0_f64, 0.0_f64),
                    (f64::INFINITY, f64::NAN),
                    (f64::from_bits(0x7ff8_0000_0000_0001), f64::NAN),
                ],
                false,
            ),
            (
                "internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(2, 2)],
                    2,
                    1,
                ),
                INTERNAL_VOLTAGES_OFFSET,
                0,
                vec![
                    (-3.0_f64, 0.0_f64),
                    (f64::NEG_INFINITY, f64::NAN),
                    (f64::from_bits(0xfff8_0000_0000_0002), f64::NAN),
                ],
                true,
            ),
        ];

        for (name, program, reused_offset, index, values, uses_internal) in cases {
            let bytes = compile_value_function(&program).expect("compile same-node voltage leaf");
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} same-node voltage should still load its base pointer once"
            );
            assert!(
                !contains_bytes(&bytes, &same_storage_voltage_memory_subtract_bytes(index)),
                "{name} same-node voltage should not reread the same slot for subtraction"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate same-node voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            for (input, expected) in values {
                let got = if uses_internal {
                    let voltages = [0.0_f64, 0.0_f64];
                    let internal_voltages = [input];
                    let ctx = eval_context(&[], &voltages, &internal_voltages, &[]);
                    f(&ctx, std::ptr::null())
                } else {
                    let voltages = [input];
                    let ctx = eval_context(&[], &voltages, &[], &[]);
                    f(&ctx, std::ptr::null())
                };
                if expected.is_nan() {
                    assert!(got.is_nan(), "{name} {input:?}");
                } else {
                    assert_eq!(got.to_bits(), expected.to_bits(), "{name} {input:?}");
                }
            }
        }
    }

    #[test]
    fn generated_value_leaf_handles_internal_branch_sub_and_neg() {
        let program = native_program_with_internals(
            EntryKind::StampValue,
            vec![
                Instruction::PushInternalVoltage(1),
                Instruction::PushBranchCurrent(0),
                Instruction::Sub,
                Instruction::Neg,
            ],
            0,
            2,
        );
        let bytes = compile_value_function(&program).expect("compile negated internal expression");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate negated internal leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let internal_voltages = [0.0_f64, 9.0_f64];
        let branch_unknowns = [4.0_f64];
        let ctx = eval_context(&[], &[], &internal_voltages, &branch_unknowns);

        assert_eq!(f(&ctx, std::ptr::null()), -5.0);
    }

    #[test]
    fn generated_value_leaf_loads_terminal_pair_current_probe() {
        let available_current_pairs = [3];
        let program = native_program_with_available_current_pairs(
            EntryKind::StampValue,
            vec![
                Instruction::PushCurrent(1, 0),
                Instruction::PushConst(0.25),
                Instruction::Mul,
            ],
            2,
            &available_current_pairs,
        );
        let bytes = compile_value_function(&program).expect("compile current probe leaf");
        assert!(
            contains_bytes(&bytes, &guarded_slice_index_cmp_imm32_bytes(3)),
            "current probe load should compare storage length against an imm32 index"
        );
        assert!(
            !contains_bytes(&bytes, &guarded_slice_index_cmp_register_bytes(3)),
            "current probe load should not materialize the constant index in a GPR"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate current probe leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [
            f64::NAN,
            4.0_f64,
            f64::NAN,
            -4.0_f64,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.num_terminals = 2;

        assert_eq!(f(&ctx, std::ptr::null()), -1.0);

        ctx.branch_currents = std::ptr::null();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing current probe must hard-fail");
        assert_current_probe_error(&error);

        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = 3;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("out-of-range current probe must hard-fail");
        assert_current_probe_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_terminal_to_ground_current_probe() {
        let available_current_pairs = [2];
        let program = native_program_with_available_current_pairs(
            EntryKind::StampValue,
            vec![
                Instruction::PushCurrent(0, usize::MAX),
                Instruction::PushConst(0.25),
                Instruction::Mul,
            ],
            2,
            &available_current_pairs,
        );
        let bytes = compile_value_function(&program).expect("compile current probe leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate current probe leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [
            f64::NAN,
            f64::NAN,
            8.0_f64,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -8.0_f64,
            f64::NAN,
            f64::NAN,
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.num_terminals = 2;

        assert_eq!(f(&ctx, std::ptr::null()), 2.0);
    }

    #[test]
    fn generated_value_leaf_reuses_current_base_for_adjacent_current_loads() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadCurrent(1),
                NativeOp::LoadCurrent(2),
                NativeOp::Add,
            ],
            2,
            vec![1, 2],
            Vec::new(),
        );
        let bytes = compile_value_function(&program).expect("compile adjacent current leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(BRANCH_CURRENTS_OFFSET)),
            1,
            "adjacent guarded current loads should materialize the current base pointer once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate adjacent current leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [f64::NAN, 4.0_f64, 6.5_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_loads_prior_current_probe() {
        let program = NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadPriorCurrent(1)],
            1,
            Vec::new(),
            vec![1],
        );
        let bytes = compile_value_function(&program).expect("compile prior current leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate prior current leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let currents = [2.0_f64, 7.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.currents = currents.as_ptr();
        ctx.currents_len = currents.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 7.0_f64.to_bits());

        ctx.currents = std::ptr::null();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing prior current must hard-fail");
        assert_prior_current_error(&error);

        ctx.currents = currents.as_ptr();
        ctx.currents_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short prior current must hard-fail");
        assert_prior_current_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_param_given_flag() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParamGiven(1)],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile param_given leaf");
        assert!(
            contains_bytes(&bytes, &guarded_slice_index_cmp_imm32_bytes(1)),
            "param_given load should compare storage length against an imm32 index"
        );
        assert!(
            !contains_bytes(&bytes, &guarded_slice_index_cmp_register_bytes(1)),
            "param_given load should not materialize the constant index in a GPR"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate param_given leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let param_given = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.param_given = param_given.as_ptr();
        ctx.param_given_len = param_given.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.param_given = std::ptr::null();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing param_given must hard-fail");
        assert_param_given_error(&error);

        ctx.param_given = param_given.as_ptr();
        ctx.param_given_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short param_given must hard-fail");
        assert_param_given_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_port_connected_flag() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushPortConnected(1)],
            2,
        );
        let bytes = compile_value_function(&program).expect("compile port_connected leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate port_connected leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let port_connected = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.port_connected = port_connected.as_ptr();
        ctx.port_connected_len = port_connected.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.port_connected = std::ptr::null();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing port_connected must hard-fail");
        assert_port_connected_error(&error);

        ctx.port_connected = port_connected.as_ptr();
        ctx.port_connected_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short port_connected must hard-fail");
        assert_port_connected_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_thermal_voltage_from_context_temperature() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushVt], 0);
        let bytes = compile_value_function(&program).expect("compile thermal voltage leaf");
        assert!(
            contains_bytes(&bytes, &THERMAL_VOLTAGE_PER_K.to_le_bytes()),
            "thermal voltage should use one precomputed k/q literal"
        );
        assert!(
            !contains_bytes(&bytes, &K_BOLTZMANN.to_le_bytes()),
            "thermal voltage should not emit a separate Boltzmann literal"
        );
        assert!(
            !contains_bytes(&bytes, &Q_ELECTRON.to_le_bytes()),
            "thermal voltage should not emit a separate electron-charge literal"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate thermal voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 315.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            thermal_voltage(315.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_computes_ddt_state_and_records_operand() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushVariable(0), Instruction::DdtState(1)],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile ddt state leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ddt state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 1.5_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();
        ctx.integration_derivative_scale = 4.0;
        ctx.integration_previous_value_scale = 4.0;
        ctx.integration_active = 1;

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 2.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        state_values[1] = f64::NAN;
        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        set_backward_euler(&mut ctx, 0.25);
        ctx.state_values = std::ptr::null_mut();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing ddt state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short ddt state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short ddt prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_ddt_jacobian_from_timestep() {
        let program = native_program(
            EntryKind::Jacobian,
            vec![Instruction::PushVariable(0), Instruction::DdtJacobian],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile ddt jacobian leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ddt jacobian leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);

        set_backward_euler(&mut ctx, 0.25);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());

        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_idt_state_and_records_integral() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::IdtState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idt state leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idt state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 1.5_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 2.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        state_values[1] = f64::NAN;
        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        set_backward_euler(&mut ctx, 1.0e-20);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        set_backward_euler(&mut ctx, -0.25);
        ctx.state_prev = previous_state.as_ptr();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 1.0_f64.to_bits());

        state_values[1] = f64::NAN;
        set_backward_euler(&mut ctx, f64::NAN);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        ctx.state_values = std::ptr::null_mut();
        set_backward_euler(&mut ctx, 0.0);
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing idt state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short idt state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short idt prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_idt_jacobian_from_timestep() {
        let program = native_program(
            EntryKind::Jacobian,
            vec![Instruction::PushVariable(0), Instruction::IdtJacobian],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idt jacobian leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idt jacobian leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);

        set_backward_euler(&mut ctx, 0.25);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());

        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());

        set_backward_euler(&mut ctx, 1.0e-20);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());

        set_backward_euler(&mut ctx, -0.25);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), (-0.5_f64).to_bits());

        set_backward_euler(&mut ctx, f64::NAN);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_idtmod_state_and_records_common_branch_history() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idtmod state leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idtmod state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 0.9_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 0.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );
        assert!((state_older_candidate[1] + 0.1).abs() < 1.0e-12);

        state_values[1] = f64::NAN;
        state_older_candidate[1] = f64::NAN;
        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_older_candidate[1].to_bits(), 0.9_f64.to_bits());

        state_values[1] = f64::NAN;
        state_older_candidate[1] = f64::NAN;
        set_backward_euler(&mut ctx, 1.0e-20);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_older_candidate[1].to_bits(), 0.9_f64.to_bits());

        state_values[1] = f64::NAN;
        state_older_candidate[1] = f64::NAN;
        set_backward_euler(&mut ctx, f64::NAN);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_older_candidate[1].to_bits(), 0.9_f64.to_bits());

        let invalid_modulus_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(0.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );
        let invalid_modulus_bytes = compile_value_function(&invalid_modulus_program)
            .expect("compile invalid-modulus idtmod leaf");
        let invalid_modulus_memory = ExecutableMemory::allocate(&invalid_modulus_bytes)
            .expect("allocate invalid-modulus idtmod leaf");
        let invalid_modulus_entry = invalid_modulus_memory
            .ptr_at(0)
            .expect("entry point inside invalid-modulus image");
        let invalid_modulus: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(invalid_modulus_entry) };

        state_values[1] = 7.0;
        state_older_candidate[1] = 8.0;
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.clear_runtime_error();
        assert_eq!(
            invalid_modulus(&ctx, vars.as_ptr()).to_bits(),
            0.0_f64.to_bits()
        );
        let error = ctx
            .take_runtime_error()
            .expect("nonpositive idtmod modulus must hard-fail");
        assert!(error.contains("modulus must be finite and greater than zero"));
        assert_eq!(state_values[1].to_bits(), 7.0_f64.to_bits());
        assert_eq!(state_older_candidate[1].to_bits(), 8.0_f64.to_bits());

        ctx.state_older_candidate = std::ptr::null_mut();
        ctx.state_older_candidate_len = 0;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing integration older-candidate storage must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values = std::ptr::null_mut();
        set_backward_euler(&mut ctx, 0.0);
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing idtmod state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short idtmod state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short idtmod prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_omits_stateful_scratch_when_full_stack_occurs_after_stateful_op() {
        let prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut instructions = vec![Instruction::PushVariable(0), Instruction::DdtState(1)];
        instructions.extend(prefix.clone());
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile spare-depth stateful leaf");

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate spare-depth stateful leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = vars_with_prefix(2.0, prefix.len());
        let previous_state = [0.0_f64, 1.5_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (2.0 + constant_prefix_sum(prefix.len())).to_bits()
        );
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_supports_full_stack_stateful_ops() {
        let run = |program: NativeProgram, vars: &[f64], expected: f64, state_expected: f64| {
            assert_eq!(program.max_stack_depth(), XMM_STACK.len());
            let bytes = compile_value_function(&program).expect("compile full-stack stateful leaf");

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate full-stack stateful leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            let previous_state = [0.0_f64, 1.5_f64];
            let older_state = previous_state;
            let mut state_values = [0.0_f64, 0.0_f64];
            let mut state_derivatives = [0.0_f64; 2];
            let previous_derivatives = [0.0_f64; 2];
            let mut state_initialized = [1_u8; 2];
            let mut state_older_candidate = [0.0_f64; 2];
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.timestep = 0.25;
            ctx.state_prev = previous_state.as_ptr();
            ctx.state_older = older_state.as_ptr();
            ctx.state_older_len = older_state.len();
            ctx.state_prev_len = previous_state.len();
            ctx.state_values = state_values.as_mut_ptr();
            ctx.state_derivatives = state_derivatives.as_mut_ptr();
            ctx.state_derivatives_len = state_derivatives.len();
            ctx.state_derivatives_prev = previous_derivatives.as_ptr();
            ctx.state_derivatives_prev_len = previous_derivatives.len();
            ctx.state_initialized = state_initialized.as_mut_ptr();
            ctx.state_initialized_len = state_initialized.len();
            ctx.state_candidate_valid = state_initialized.as_mut_ptr();
            ctx.state_candidate_valid_len = state_initialized.len();
            ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
            ctx.state_older_candidate_len = state_older_candidate.len();
            ctx.state_values_len = state_values.len();
            ctx.integration_derivative_scale = 4.0;
            ctx.integration_previous_value_scale = 4.0;
            ctx.integration_active = 1;

            assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), expected.to_bits());
            assert_eq!(state_values[1].to_bits(), state_expected.to_bits());
        };

        let ddt_prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut ddt_instructions = ddt_prefix.clone();
        ddt_instructions.extend([Instruction::PushVariable(0), Instruction::DdtState(1)]);
        ddt_instructions.extend(add_reductions(ddt_prefix.len()));
        let ddt_vars = vars_with_prefix(2.0, ddt_prefix.len());
        run(
            native_program(EntryKind::StampValue, ddt_instructions, 0),
            &ddt_vars,
            constant_prefix_sum(ddt_prefix.len()) + 2.0,
            2.0,
        );

        let idt_prefix = variable_prefix(1, XMM_STACK.len() - 2);
        let mut idt_instructions = idt_prefix.clone();
        idt_instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::IdtState(1),
        ]);
        idt_instructions.extend(add_reductions(idt_prefix.len()));
        let idt_vars = vars_with_prefix(2.0, idt_prefix.len());
        run(
            native_program(EntryKind::StampValue, idt_instructions, 0),
            &idt_vars,
            constant_prefix_sum(idt_prefix.len()) + 2.0,
            2.0,
        );

        let idtmod_prefix = variable_prefix(1, XMM_STACK.len() - 4);
        let mut idtmod_instructions = idtmod_prefix.clone();
        idtmod_instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::PushConst(1.0),
            Instruction::PushConst(0.25),
            Instruction::IdtModState(1),
        ]);
        idtmod_instructions.extend(add_reductions(idtmod_prefix.len()));
        let idtmod_vars = vars_with_prefix(2.0, idtmod_prefix.len());
        run(
            native_program(EntryKind::StampValue, idtmod_instructions, 0),
            &idtmod_vars,
            constant_prefix_sum(idtmod_prefix.len()) + 1.0,
            1.0,
        );

        let jacobian_prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut ddt_jacobian = jacobian_prefix.clone();
        ddt_jacobian.extend([Instruction::PushVariable(0), Instruction::DdtJacobian]);
        ddt_jacobian.extend(add_reductions(jacobian_prefix.len()));
        let mut idt_jacobian = jacobian_prefix.clone();
        idt_jacobian.extend([Instruction::PushVariable(0), Instruction::IdtJacobian]);
        idt_jacobian.extend(add_reductions(jacobian_prefix.len()));
        let jacobian_cases = [
            (
                constant_prefix_sum(jacobian_prefix.len()) + 8.0,
                ddt_jacobian,
            ),
            (
                constant_prefix_sum(jacobian_prefix.len()) + 0.5,
                idt_jacobian,
            ),
        ];
        for (expected, instructions) in jacobian_cases {
            let program = native_program(EntryKind::Jacobian, instructions, 0);
            assert_eq!(program.max_stack_depth(), XMM_STACK.len());
            let bytes =
                compile_value_function(&program).expect("compile full-stack stateful jacobian");

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate full-stack jacobian leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.timestep = 0.25;
            ctx.integration_derivative_scale = 4.0;
            ctx.integration_previous_value_scale = 4.0;
            ctx.integration_active = 1;

            let vars = vars_with_prefix(2.0, jacobian_prefix.len());
            assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_preserves_prefix_across_idtmod_helper_call() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(10.0),
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
                Instruction::Add,
            ],
            0,
        );
        assert_eq!(program.max_stack_depth(), 5);
        let bytes = compile_value_function(&program).expect("compile prefixed idtmod state leaf");

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate prefixed idtmod leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 0.9_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64; 2];
        let previous_derivatives = [0.0_f64; 2];
        let mut state_initialized = [1_u8; 2];
        let mut state_older_candidate = [0.0_f64; 2];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        set_backward_euler(&mut ctx, 0.25);
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_older = older_state.as_ptr();
        ctx.state_older_len = older_state.len();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_derivatives = state_derivatives.as_mut_ptr();
        ctx.state_derivatives_len = state_derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        ctx.state_candidate_valid_len = state_initialized.len();
        ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = state_older_candidate.len();
        ctx.state_values_len = state_values.len();

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 10.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );
        assert!((state_older_candidate[1] + 0.1).abs() < 1.0e-12);

        state_values[1] = f64::NAN;
        state_older_candidate[1] = f64::NAN;
        set_backward_euler(&mut ctx, 0.0);
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_older_candidate[1].to_bits(), 0.9_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_limit_state_and_records_iteration_value() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::LimitState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile limit state leaf");
        assert!(
            !contains_bytes(&bytes, &stack_spill_store_bytes(Xmm::Xmm1)),
            "limit state with a spare XMM register should not spill the positive step to the stack"
        );
        assert!(
            !contains_bytes(&bytes, &stack_spill_minsd_bytes(Xmm::Xmm0)),
            "limit state with a spare XMM register should clamp from the XMM scratch copy"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate limit state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_initialized = [0_u8, 0_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();

        let vars = [10.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_initialized[1], 1);

        let vars = [11.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_initialized[1], 1);

        let vars = [0.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.0_f64.to_bits());

        state_values[1] = 0.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [10.0_f64];
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            0.5_f64.to_bits(),
            "initialized zero state must clamp instead of behaving like first evaluation"
        );
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        ctx.state_initialized_len = 1;
        state_values[1] = 10.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [20.0_f64];
        ctx.clear_runtime_error();
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            0.0_f64.to_bits(),
            "native limit must return through the hard-fail path"
        );
        let error = ctx
            .take_runtime_error()
            .expect("out-of-range limit metadata must hard-fail");
        assert!(
            error.contains("state index outside initialization flag storage"),
            "error must identify invalid limit metadata, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
        assert_eq!(
            state_values[1].to_bits(),
            10.0_f64.to_bits(),
            "out-of-range state flag metadata must leave native state untouched"
        );

        ctx.state_initialized_len = state_initialized.len();
        ctx.state_values_len = 1;
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("short limit state must hard-fail");
        assert_limit_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.state_values = std::ptr::null_mut();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing limit state must hard-fail");
        assert!(
            error.contains("missing state storage"),
            "error must identify missing limit state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_initialized = std::ptr::null_mut();
        ctx.clear_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing limit initialization flags must hard-fail");
        assert!(
            error.contains("missing initialization flag storage"),
            "error must identify missing limit initialization storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        ctx.state_initialized = state_initialized.as_mut_ptr();
        state_values[1] = 1.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [f64::NAN];
        let result = f(&ctx, vars.as_ptr());
        assert!(result.is_nan(), "initialized limit must propagate NaN");
        assert!(
            state_values[1].is_nan(),
            "native state should record propagated NaN"
        );
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_limit_state_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 2);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::LimitState(1),
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile full-stack limit state leaf");
        assert!(
            !contains_bytes(
                &bytes,
                &stack_spill_store_bytes(XMM_STACK[XMM_STACK.len() - 1])
            ),
            "the positive step should remain in its allocated register; stack adjustment alone is not diagnostic because System V uses an eight-byte helper-call alignment frame"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &stack_spill_minsd_bytes(XMM_STACK[XMM_STACK.len() - 2])
            ),
            "limit-state clamping should not reload an instruction-local spill"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack limit state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 10.0_f64];
        let mut state_initialized = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();

        let vars = [11.0_f64];
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 10.5).to_bits()
        );
        assert_eq!(state_values[1].to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_named_limiter_reads_previous_and_publishes_candidate() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(0),
                NativeOp::LimiterPrevious(1),
                NativeOp::Const(0.5),
                NativeOp::Add,
                NativeOp::LimiterStore(1),
            ],
            3,
            Vec::new(),
            Vec::new(),
        );
        let bytes = compile_value_function(&program).expect("compile named limiter value leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate named limiter value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_initialized = [0_u8, 0_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        let mut limiter_active = 0_u8;
        ctx.limiter_active = &mut limiter_active;
        ctx.limiting_enabled = 1;

        let first_vars = [10.0_f64];
        assert_eq!(f(&ctx, first_vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_initialized[1], 1);
        assert_eq!(limiter_active, 1);

        limiter_active = 0;
        let second_vars = [20.0_f64];
        assert_eq!(f(&ctx, second_vars.as_ptr()).to_bits(), 11.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 11.0_f64.to_bits());
        assert_eq!(state_initialized[1], 1);
        assert_eq!(limiter_active, 1);

        limiter_active = 0;
        ctx.limiting_enabled = 0;
        let state_before_probe = state_values;
        let probe_vars = [30.0_f64];
        assert_eq!(f(&ctx, probe_vars.as_ptr()).to_bits(), 30.0_f64.to_bits());
        assert_eq!(state_values, state_before_probe);
        assert_eq!(state_initialized[1], 1);
        assert_eq!(limiter_active, 0);
        assert!(ctx.take_runtime_error().is_none());

        let converged_program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::LoadVariable(0),
                NativeOp::LimiterStore(1),
            ],
            2,
            Vec::new(),
            Vec::new(),
        );
        let converged_bytes =
            compile_value_function(&converged_program).expect("compile converged limiter leaf");
        let converged_memory =
            ExecutableMemory::allocate(&converged_bytes).expect("allocate converged limiter leaf");
        let converged_entry = converged_memory.ptr_at(0).expect("converged limiter entry");
        let converged: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(converged_entry) };
        ctx.limiting_enabled = 1;
        limiter_active = 0;
        let converged_vars = [4.0_f64];
        assert_eq!(
            converged(&ctx, converged_vars.as_ptr()).to_bits(),
            4.0_f64.to_bits()
        );
        assert_eq!(state_values[1].to_bits(), 4.0_f64.to_bits());
        assert_eq!(limiter_active, 0);
    }

    #[test]
    fn generated_value_leaf_computes_sqrt_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Sqrt],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile sqrt leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate sqrt leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 49.0;

        assert_eq!(f(&ctx, std::ptr::null()), 7.0);
    }

    #[test]
    fn generated_value_leaf_computes_abs_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Abs],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile abs leaf");
        assert!(
            contains_bytes(&bytes, &andpd_rip_prefix_bytes(Xmm::Xmm0)),
            "abs should clear the sign bit with a RIP-relative vector mask"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &scalar_abs_integer_bit_clear_bytes(Xmm::Xmm0, Gpr::Rax)
            ),
            "abs should not roundtrip through a GPR"
        );
        let mask_offset =
            find_bytes(&bytes, &abs_value_mask_bytes()).expect("abs mask literal is present");
        assert_eq!(
            mask_offset % VECTOR_LITERAL_ALIGNMENT,
            0,
            "abs mask literal should be 16-byte aligned"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate abs leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.temperature = -7.5;
        assert_eq!(f(&ctx, std::ptr::null()), 7.5);
        ctx.temperature = -0.0;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        ctx.temperature = f64::from_bits(0xfff8_0000_0000_0001);
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0x7ff8_0000_0000_0001);
    }

    #[test]
    fn generated_value_leaf_folds_constant_unary_ops_to_literals() {
        let cases = [
            (
                "neg negative literal",
                Instruction::Neg,
                (-7.5_f64).to_bits(),
                7.5_f64.to_bits(),
            ),
            (
                "neg negative zero",
                Instruction::Neg,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "neg positive nan",
                Instruction::Neg,
                0x7ff8_0000_0000_0001,
                0xfff8_0000_0000_0001,
            ),
            (
                "abs negative literal",
                Instruction::Abs,
                (-7.5_f64).to_bits(),
                7.5_f64.to_bits(),
            ),
            (
                "abs negative zero",
                Instruction::Abs,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "abs negative nan",
                Instruction::Abs,
                0xfff8_0000_0000_0001,
                0x7ff8_0000_0000_0001,
            ),
        ];

        for (case, instruction, input_bits, expected_bits) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(f64::from_bits(input_bits)),
                    instruction,
                ],
                0,
            );

            match program.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded unary leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded constant unary op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded unary leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected_bits, "{case}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_large_signal_noise_as_zero() {
        let cases = [
            (
                "white-standalone",
                vec![Instruction::PushConst(5.0), Instruction::WhiteNoise],
                0.0_f64,
            ),
            (
                "flicker-standalone",
                vec![
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                ],
                0.0_f64,
            ),
            (
                "white-composed",
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(5.0),
                    Instruction::WhiteNoise,
                    Instruction::Add,
                ],
                2.0_f64,
            ),
            (
                "flicker-composed",
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                    Instruction::Add,
                ],
                2.0_f64,
            ),
            (
                "flicker-overwrite-dead-register",
                vec![
                    Instruction::PushConst(7.0),
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                    Instruction::PushConst(3.0),
                    Instruction::Add,
                    Instruction::Add,
                ],
                10.0_f64,
            ),
        ];

        for (name, instructions, expected) in cases {
            let program = native_program(EntryKind::StampValue, instructions, 0);
            let bytes = compile_value_function(&program).expect("compile noise leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate noise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_laplace_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(4.0),
                    Instruction::LaplaceState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0).with_laplace_filter_count(1),
        )
        .expect("lower Laplace helper program");
        let bytes = compile_value_function(&program).expect("compile Laplace helper leaf");
        assert!(
            contains_bytes(
                &bytes,
                &xor_r64_bytes(
                    super::context_filter_id_arg_reg(),
                    super::context_filter_id_arg_reg()
                )
            ),
            "Laplace helper should zero filter ID 0 with a compact dependency-breaking xor"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::context_filter_id_arg_reg(), 0)
            ),
            "Laplace helper should not materialize filter ID 0 with a wider imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::context_filter_id_arg_reg(), 0)
            ),
            "Laplace helper should not use movabs for small filter IDs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate Laplace helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [
            StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 1.0])
                .expect("valid first-order Laplace filter"),
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            6.0_f64.to_bits(),
            "non-transient Laplace evaluation uses DC output"
        );

        ctx.analysis_type = 2;
        ctx.timestep = 0.5;
        ctx.integration_active = 1;
        ctx.integration_derivative_scale = 2.0;
        ctx.integration_previous_value_scale = 2.0;
        let transient = f(&ctx, std::ptr::null());
        let repeated = f(&ctx, std::ptr::null());
        assert!(
            (transient - (2.0 + 4.0 / 3.0)).abs() < 1.0e-12,
            "transient Laplace value: {transient}"
        );
        assert_eq!(
            transient.to_bits(),
            repeated.to_bits(),
            "native Laplace evaluation must be Newton-idempotent"
        );
        filters[0].commit();

        let next = f(&ctx, std::ptr::null());
        assert!(
            (next - (2.0 + 20.0 / 9.0)).abs() < 1.0e-12,
            "accepted Laplace state must advance exactly once: {next}"
        );
    }

    #[test]
    fn generated_laplace_derivative_leaf_is_exact_and_read_only() {
        let program = NativeProgram::from_bytecode(
            "x64-laplace-derivative-test",
            EntryKind::Jacobian,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(2.0),
                    Instruction::LaplaceStateDerivative(0),
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0).with_laplace_filter_count(1),
        )
        .expect("lower Laplace derivative helper program");
        let bytes = compile_value_function(&program).expect("compile Laplace derivative leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate Laplace derivative leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [
            StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 1.0])
                .expect("valid first-order Laplace filter"),
        ];
        let accepted = filters[0].checkpoint();
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.analysis_type = 2;
        ctx.timestep = 0.5;
        ctx.integration_active = 1;
        ctx.integration_derivative_scale = 2.0;
        ctx.integration_previous_value_scale = 2.0;
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        let derivative = f(&ctx, std::ptr::null());
        assert!((derivative - (2.0 / 3.0)).abs() <= 1.0e-15);
        assert_eq!(filters[0].checkpoint(), accepted);
        assert!(ctx.take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_calls_zi_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::ZiState(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0).with_zi_filter_count(1),
        )
        .expect("lower zi helper program");
        let bytes = compile_value_function(&program).expect("compile zi helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zi helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters =
            [ZiFilter::new(vec![0.25], vec![1.0, -0.75], 1.0e-6).expect("valid zi test filter")];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.zi_filters = filters.as_mut_ptr();
        ctx.zi_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            3.0_f64.to_bits(),
            "non-transient zi evaluation uses DC steady state"
        );

        ctx.analysis_type = 2;
        ctx.time = 0.0;
        let first = f(&ctx, std::ptr::null());
        let repeated = f(&ctx, std::ptr::null());
        assert_eq!(
            first.to_bits(),
            repeated.to_bits(),
            "native zi helper must preserve Newton re-evaluation idempotence"
        );
        assert!((first - 2.25).abs() < 1.0e-12, "first zi sample: {first}");
        filters[0].commit(ctx.time).expect("commit first sample");

        ctx.time = 0.5e-6;
        let held = f(&ctx, std::ptr::null());
        assert!((held - 2.25).abs() < 1.0e-12, "held zi output: {held}");
        filters[0].commit(ctx.time).expect("commit held point");

        ctx.time = 1.0e-6;
        let next = f(&ctx, std::ptr::null());
        assert!((next - 2.4375).abs() < 1.0e-12, "second zi sample: {next}");
    }

    #[test]
    fn generated_value_leaf_calls_transition_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.2),
                    Instruction::PushConst(0.4),
                    Instruction::PushConst(0.4),
                    Instruction::TransitionState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("lower transition helper program");
        let bytes = compile_value_function(&program).expect("compile transition helper leaf");
        assert!(
            contains_bytes(
                &bytes,
                &xor_r64_bytes(
                    super::operand_filter_id_arg_reg(),
                    super::operand_filter_id_arg_reg()
                )
            ),
            "transition helper should zero filter ID 0 with a compact dependency-breaking xor"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::operand_filter_id_arg_reg(), 0)
            ),
            "transition helper should not materialize filter ID 0 with a wider imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::operand_filter_id_arg_reg(), 0)
            ),
            "transition helper should not use movabs for small filter IDs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate transition helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [TransitionFilter::default()];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.analysis_type = 2;
        ctx.temperature = 310.0;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            311.0_f64.to_bits(),
            "non-transient transition evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 1.0;
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 311.0_f64.to_bits());
        filters[0].commit();

        ctx.time = 1.4;
        let mid = f(&ctx, std::ptr::null());
        assert!((mid - 311.0).abs() < 1.0e-12, "mid transition: {mid}");
        filters[0].commit();

        ctx.time = 1.6;
        let done = f(&ctx, std::ptr::null());
        assert!((done - 311.0).abs() < 1.0e-12, "done transition: {done}");
    }

    #[test]
    fn generated_value_leaf_routes_transition_derivative_with_five_operands() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-transition-derivative-test",
            EntryKind::Jacobian,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(3.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(0.0),
                    Instruction::TransitionStateDerivative(0),
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("lower transition derivative helper program");
        let bytes =
            compile_value_function(&program).expect("compile transition derivative helper leaf");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate transition derivative leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [TransitionFilter::default()];
        filters[0]
            .eval(1.0, 0.0, 0.0, 0.0, 0.0)
            .expect("seed direct transition");
        filters[0].commit();
        let accepted = filters[0].checkpoint();
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.analysis_type = 2;
        ctx.time = 1.0;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();

        for _ in 0..2 {
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                3.0_f64.to_bits(),
                "unchanged instantaneous transition has unity local Jacobian"
            );
            assert_eq!(
                filters[0].checkpoint(),
                accepted,
                "generated native derivative must not change transition history"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_slew_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(10.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(-2.0),
                    Instruction::SlewState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("lower slew helper program");
        let bytes = compile_value_function(&program).expect("compile slew helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate slew helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [SlewFilter::default()];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 310.0;
        ctx.slew_filters = filters.as_mut_ptr();
        ctx.slew_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            320.0_f64.to_bits(),
            "non-transient slew evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 320.0_f64.to_bits());
        filters[0].commit();

        ctx.time = 0.5;
        let mid = f(&ctx, std::ptr::null());
        assert!((mid - 320.0).abs() < 1.0e-12, "mid slew: {mid}");
        filters[0].commit();

        ctx.time = 1.0;
        let done = f(&ctx, std::ptr::null());
        assert!((done - 320.0).abs() < 1.0e-12, "done slew: {done}");
    }

    #[test]
    fn generated_value_leaf_calls_absdelay_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVoltage(0, usize::MAX),
                    Instruction::PushConst(0.5),
                    Instruction::AbsDelayState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(1, 0, 0, 0, 0),
        )
        .expect("lower absdelay helper program");
        let bytes = compile_value_function(&program).expect("compile absdelay helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate absdelay helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut voltages = [7.0_f64];
        let mut buffers = [DelayBuffer::default()];
        let mut ctx = eval_context(&[], &voltages, &[], &[]);
        ctx.temperature = 310.0;
        ctx.delay_buffers = buffers.as_mut_ptr();
        ctx.delay_buffers_len = buffers.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            317.0_f64.to_bits(),
            "non-transient absdelay evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        voltages[0] = 0.0;
        std::hint::black_box(voltages[0]);
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());
        buffers[0].commit().unwrap();

        ctx.time = 0.5;
        voltages[0] = 1.0;
        std::hint::black_box(voltages[0]);
        let delayed_start = f(&ctx, std::ptr::null());
        assert_eq!(delayed_start.to_bits(), 310.0_f64.to_bits());
        buffers[0].commit().unwrap();

        ctx.time = 1.0;
        voltages[0] = 3.0;
        std::hint::black_box(voltages[0]);
        let delayed = f(&ctx, std::ptr::null());
        assert!((delayed - 311.0).abs() < 1.0e-12, "delayed: {delayed}");
        buffers[0].commit().unwrap();

        ctx.time = 1.25;
        voltages[0] = 5.0;
        std::hint::black_box(voltages[0]);
        let interpolated = f(&ctx, std::ptr::null());
        assert!(
            (interpolated - 312.0).abs() < 1.0e-12,
            "interpolated: {interpolated}"
        );
    }

    #[test]
    fn generated_value_leaf_calls_cross_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVoltage(0, usize::MAX),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(1.0),
                    Instruction::CrossState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(1, 0, 0, 0, 0),
        )
        .expect("lower cross helper program");
        let bytes = compile_value_function(&program).expect("compile cross helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate cross helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut voltages = [7.0_f64];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = eval_context(&[], &voltages, &[], &[]);
        ctx.temperature = 310.0;
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();

        ctx.time = -0.5;
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            310.0_f64.to_bits(),
            "non-transient cross evaluation reports zero while preserving the stack"
        );
        detectors[0].commit();

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        voltages[0] = -1.0;
        std::hint::black_box(voltages[0]);
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());
        detectors[0].commit();

        ctx.time = 0.5;
        voltages[0] = 1.0;
        std::hint::black_box(voltages[0]);
        let crossing = f(&ctx, std::ptr::null());
        assert_eq!(crossing.to_bits(), 311.0_f64.to_bits());
        detectors[0].commit();

        ctx.time = 1.0;
        voltages[0] = 2.0;
        std::hint::black_box(voltages[0]);
        let steady = f(&ctx, std::ptr::null());
        assert_eq!(steady.to_bits(), 310.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_ordered_comparisons() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 3.0, 1.0),
            ("gt-false", Instruction::Gt, 3.0, 5.0, 0.0),
            ("lt-true", Instruction::Lt, 3.0, 5.0, 1.0),
            ("lt-false", Instruction::Lt, 5.0, 3.0, 0.0),
            ("ge-true", Instruction::Ge, 5.0, 3.0, 1.0),
            ("ge-false", Instruction::Ge, 3.0, 5.0, 0.0),
            ("ge-equal", Instruction::Ge, 3.0, 3.0, 1.0),
            ("le-true", Instruction::Le, 3.0, 5.0, 1.0),
            ("le-false", Instruction::Le, 5.0, 3.0, 0.0),
            ("le-equal", Instruction::Le, 3.0, 3.0, 1.0),
            ("gt-left-unordered", Instruction::Gt, f64::NAN, 3.0, 0.0),
            ("gt-right-unordered", Instruction::Gt, 3.0, f64::NAN, 0.0),
            ("lt-left-unordered", Instruction::Lt, f64::NAN, 3.0, 0.0),
            ("lt-right-unordered", Instruction::Lt, 3.0, f64::NAN, 0.0),
            ("ge-left-unordered", Instruction::Ge, f64::NAN, 3.0, 0.0),
            ("ge-right-unordered", Instruction::Ge, 3.0, f64::NAN, 0.0),
            ("le-left-unordered", Instruction::Le, f64::NAN, 3.0, 0.0),
            ("le-right-unordered", Instruction::Le, 3.0, f64::NAN, 0.0),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile comparison leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_equality_comparisons() {
        let cases = [
            ("eq-exact", Instruction::Eq, 1.0, 1.0, 1.0),
            ("eq-tiny-nonzero", Instruction::Eq, 0.0, 0.5e-15, 0.0),
            ("eq-larger-nonzero", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-nonzero", Instruction::Eq, 0.0, 1.5e-15, 0.0),
            ("ne-exact", Instruction::Ne, 1.0, 1.0, 0.0),
            ("ne-tiny-nonzero", Instruction::Ne, 0.0, 0.5e-15, 1.0),
            ("ne-larger-nonzero", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-nonzero", Instruction::Ne, 0.0, 1.5e-15, 1.0),
            ("eq-left-unordered", Instruction::Eq, f64::NAN, 1.0, 0.0),
            ("eq-right-unordered", Instruction::Eq, 1.0, f64::NAN, 0.0),
            ("ne-left-unordered", Instruction::Ne, f64::NAN, 1.0, 1.0),
            ("ne-right-unordered", Instruction::Ne, 1.0, f64::NAN, 1.0),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile equality leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate equality leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_logical_ops() {
        let cases = [
            ("and-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "and-left-unordered",
                Instruction::And,
                f64::NAN,
                2.0e-15,
                1.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                1.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-nonzero", Instruction::Or, 1.0e-15, 0.5e-15, 1.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 1.0),
            (
                "or-right-unordered",
                Instruction::Or,
                0.5e-15,
                f64::NAN,
                1.0,
            ),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile logical leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let const_rhs_cases = [
            (
                "and-rhs-true-left-true",
                Instruction::And,
                2.0e-15,
                -2.0e-15,
                1.0,
            ),
            (
                "and-rhs-true-left-false",
                Instruction::And,
                0.5e-15,
                -2.0e-15,
                1.0,
            ),
            ("and-rhs-nonzero", Instruction::And, 2.0e-15, 1.0e-15, 1.0),
            (
                "and-rhs-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                1.0,
            ),
            ("or-rhs-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            (
                "or-rhs-false-left-true",
                Instruction::Or,
                2.0e-15,
                1.0e-15,
                1.0,
            ),
            (
                "or-rhs-false-left-false",
                Instruction::Or,
                0.5e-15,
                1.0e-15,
                1.0,
            ),
            ("or-rhs-unordered", Instruction::Or, 0.5e-15, f64::NAN, 1.0),
        ];

        for (name, op, input, rhs, expected) in const_rhs_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    op,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS logical op, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal RHS logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS logical op should stay helper-free"
            );
            if name == "or-rhs-true" {
                assert!(
                    contains_bytes(&bytes, &mov_r32_imm32_bytes(Gpr::R10, 1)),
                    "literal true logical result should use compact imm32 materialization"
                );
                assert!(
                    !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R10, 1)),
                    "literal true logical result should not use a 64-bit immediate"
                );
            }

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let const_lhs_cases = [
            (
                "and-lhs-true-right-true",
                Instruction::And,
                -2.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "and-lhs-true-right-false",
                Instruction::And,
                -2.0e-15,
                0.5e-15,
                1.0,
            ),
            ("and-lhs-nonzero", Instruction::And, 1.0e-15, 2.0e-15, 1.0),
            (
                "and-lhs-unordered",
                Instruction::And,
                f64::NAN,
                2.0e-15,
                1.0,
            ),
            ("or-lhs-true", Instruction::Or, -2.0e-15, 0.5e-15, 1.0),
            (
                "or-lhs-false-right-true",
                Instruction::Or,
                1.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "or-lhs-false-right-false",
                Instruction::Or,
                1.0e-15,
                0.5e-15,
                1.0,
            ),
            ("or-lhs-unordered", Instruction::Or, f64::NAN, 0.5e-15, 1.0),
        ];

        for (name, op, lhs, input, expected) in const_lhs_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    op,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS logical op, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal LHS logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS logical op should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let not_cases = [
            ("not-tiny-nonzero", 0.5e-15, 0.0),
            ("not-at-epsilon", 1.0e-15, 0.0),
            ("not-outside-epsilon", 2.0e-15, 0.0),
            ("not-unordered", f64::NAN, 0.0),
        ];

        for (name, value, expected) in not_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushConst(value), Instruction::Not],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile logical-not leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate logical-not leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_logical_ops_to_literals() {
        let cases = [
            ("and-both-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                1.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-nonzero", Instruction::Or, 1.0e-15, 0.5e-15, 1.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 1.0),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded logical op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_constant_rhs_shifts_with_shared_contract() {
        let cases = [
            (
                "shl",
                Instruction::Shl,
                IntegerBinaryOp::Shl,
                3.0,
                2.0,
                runtime_shl(3.0, 2.0),
                2,
            ),
            (
                "shr-negative",
                Instruction::Shr,
                IntegerBinaryOp::Shr,
                -16.0,
                2.75,
                runtime_shr(-16.0, 2.75),
                3,
            ),
        ];

        for (name, instruction, expected_op, left, right, integer_expected, expected_count) in cases
        {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerShiftConst(expected_op, expected_count),
                ],
                "{name}: valid constant count should lower to an immediate shift"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: immediate shift should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile constant shift leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate constant shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                integer_expected.to_bits(),
                "{name}"
            );
            assert!(ctx.take_runtime_error().is_none(), "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_zero_count_shift_preserves_checked_conversion() {
        let cases = [
            (
                "shl-zero-fractional",
                Instruction::Shl,
                IntegerBinaryOp::Shl,
                13.75,
                runtime_shl(13.75, 0.0),
            ),
            (
                "shr-zero-nan",
                Instruction::Shr,
                IntegerBinaryOp::Shr,
                f64::NAN,
                runtime_shr(f64::NAN, 0.0),
            ),
        ];

        for (name, instruction, expected_op, left, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(0.0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerShiftConst(expected_op, 0)
                ],
                "{name}: zero count should still lower to the integer-conversion shift op"
            );

            let bytes = compile_value_function(&program).expect("compile zero-count shift leaf");

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate zero-count shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}: zero-count shift must still perform checked Verilog-AMS conversion"
            );
            assert_eq!(
                ctx.take_runtime_error().is_some(),
                !left.is_finite(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_runtime_shifts_with_shared_contract() {
        let cases = [
            ("shl", Instruction::Shl, 3.0, 2.0, runtime_shl(3.0, 2.0)),
            (
                "shr-negative",
                Instruction::Shr,
                -16.0,
                2.0,
                runtime_shr(-16.0, 2.0),
            ),
        ];

        for (name, op, left, right, integer_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushParam(0),
                    Instruction::PushParam(1),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile integer shift leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let mut ctx = eval_context(&params, &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            ctx.clear_runtime_error();
            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + integer_expected) + 2.0)).to_bits(),
                "{name}"
            );
            assert!(
                ctx.take_runtime_error().is_none(),
                "{name}: valid runtime shift count should not report a native runtime error"
            );
        }
    }

    #[test]
    fn generated_value_leaf_unsigned_out_of_width_shift_counts_produce_zero() {
        let cases = [
            ("shl-negative-count", Instruction::Shl, 3.0, -1.0),
            ("shr-too-large-count", Instruction::Shr, -16.0, 64.0),
        ];

        for (name, op, left, right) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), Instruction::PushParam(1), op],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile integer shift leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let ctx = eval_context(&params, &[], &[], &[]);

            ctx.clear_runtime_error();
            let result = f(&ctx, std::ptr::null());

            assert_eq!(result.to_bits(), 0.0_f64.to_bits(), "{name}");
            assert!(
                ctx.take_runtime_error().is_none(),
                "{name}: unsigned out-of-width counts are defined, not runtime errors"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_runtime_bitwise_integer_ops_with_shared_contract() {
        let cases = [
            (
                "bitand-rounds",
                Instruction::BitAnd,
                13.75,
                6.25,
                runtime_bitand(13.75, 6.25),
            ),
            (
                "bitor-negative",
                Instruction::BitOr,
                -16.0,
                3.0,
                runtime_bitor(-16.0, 3.0),
            ),
            (
                "bitxor-nan",
                Instruction::BitXor,
                f64::NAN,
                7.0,
                runtime_bitxor(f64::NAN, 7.0),
            ),
            (
                "bitand-positive-infinity",
                Instruction::BitAnd,
                f64::INFINITY,
                -1.0,
                runtime_bitand(f64::INFINITY, -1.0),
            ),
            (
                "bitand-positive-saturation",
                Instruction::BitAnd,
                I64_MAX_EXCLUSIVE_AS_F64,
                -1.0,
                runtime_bitand(I64_MAX_EXCLUSIVE_AS_F64, -1.0),
            ),
            (
                "bitor-negative-saturation",
                Instruction::BitOr,
                I64_MIN_AS_F64,
                7.0,
                runtime_bitor(I64_MIN_AS_F64, 7.0),
            ),
            (
                "bitxor-negative-infinity",
                Instruction::BitXor,
                f64::NEG_INFINITY,
                7.0,
                runtime_bitxor(f64::NEG_INFINITY, 7.0),
            ),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), Instruction::PushParam(1), op],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile bitwise leaf");

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
            let expected_error = crate::integer_runtime::real_to_integer(left).is_err()
                || crate::integer_runtime::real_to_integer(right).is_err();
            assert_eq!(ctx.take_runtime_error().is_some(), expected_error, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_constant_rhs_bitwise_with_shared_contract() {
        let cases = [
            (
                "bitand-imm",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                13.75,
                6.25,
                6,
                runtime_bitand(13.75, 6.25),
            ),
            (
                "bitor-negative-imm",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                -16.0,
                -1.0,
                -1,
                runtime_bitor(-16.0, -1.0),
            ),
            (
                "bitxor-nonzero-imm",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                f64::NAN,
                7.0,
                7,
                runtime_bitxor(f64::NAN, 7.0),
            ),
        ];

        for (name, instruction, expected_op, left, right, rhs_i64, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerBinaryConst(expected_op, rhs_i64),
                ],
                "{name}: constant RHS should lower to an immediate bitwise op"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: constant RHS bitwise should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile constant bitwise leaf");

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate constant bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
            assert_eq!(
                ctx.take_runtime_error().is_some(),
                crate::integer_runtime::real_to_integer(left).is_err(),
                "{name}"
            );
        }

        let wide_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushConst(f64::INFINITY),
                Instruction::BitAnd,
            ],
            0,
        );
        let wide_bytes =
            compile_value_function(&wide_program).expect("compile wide constant bitwise leaf");
        let memory = ExecutableMemory::allocate(&wide_bytes).expect("allocate wide bitwise leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [13.75];
        let ctx = eval_context(&params, &[], &[], &[]);
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_bitand(13.75, f64::INFINITY).to_bits()
        );
        assert!(ctx.take_runtime_error().is_some());
    }

    #[test]
    fn generated_value_leaf_computes_constant_lhs_bitwise_with_shared_contract() {
        let cases = [
            (
                "bitand-lhs-imm",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                6.25,
                13.75,
                6,
                runtime_bitand(6.25, 13.75),
            ),
            (
                "bitor-negative-lhs-imm",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                -1.0,
                -16.0,
                -1,
                runtime_bitor(-1.0, -16.0),
            ),
            (
                "bitxor-lhs-imm",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                7.0,
                f64::NAN,
                7,
                runtime_bitxor(7.0, f64::NAN),
            ),
        ];

        for (name, instruction, expected_op, left, right, lhs_i64, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushParam(0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerBinaryConst(expected_op, lhs_i64),
                ],
                "{name}: constant LHS should commute to an immediate bitwise op"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: constant LHS bitwise should not allocate an LHS stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile constant-LHS bitwise leaf");

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate constant-LHS bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
            assert_eq!(
                ctx.take_runtime_error().is_some(),
                crate::integer_runtime::real_to_integer(right).is_err(),
                "{name}"
            );
        }

        let wide_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(f64::INFINITY),
                Instruction::PushParam(0),
                Instruction::BitAnd,
            ],
            0,
        );
        let wide_bytes =
            compile_value_function(&wide_program).expect("compile wide constant-LHS bitwise leaf");
        let memory = ExecutableMemory::allocate(&wide_bytes)
            .expect("allocate wide constant-LHS bitwise leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [13.75];
        let ctx = eval_context(&params, &[], &[], &[]);
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_bitand(f64::INFINITY, 13.75).to_bits()
        );
        assert!(ctx.take_runtime_error().is_some());
    }

    #[test]
    fn generated_value_leaf_integerizes_constant_rhs_bitwise_identities() {
        let cases = [
            (
                "bitor-zero-fraction",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                13.75,
                0.0,
                runtime_bitor(13.75, 0.0),
            ),
            (
                "bitxor-negative-zero-fraction",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                -16.75,
                -0.0,
                runtime_bitxor(-16.75, -0.0),
            ),
            (
                "bitand-all-ones-nan",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                f64::NAN,
                -1.0,
                runtime_bitand(f64::NAN, -1.0),
            ),
            (
                "bitand-all-ones-positive-infinity",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                f64::INFINITY,
                -1.0,
                runtime_bitand(f64::INFINITY, -1.0),
            ),
        ];

        for (name, instruction, _elided_op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[NativeOp::LoadParam(0), NativeOp::IntegerCast],
                "{name}: identity bitwise op should lower to integer conversion"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: identity bitwise op should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile integer-cast leaf");

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer-cast leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
            assert_eq!(
                ctx.take_runtime_error().is_some(),
                crate::integer_runtime::real_to_integer(left).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_integerizes_constant_lhs_bitwise_identities() {
        let cases = [
            (
                "bitor-zero-fraction",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                0.0,
                13.75,
                runtime_bitor(0.0, 13.75),
            ),
            (
                "bitxor-negative-zero-fraction",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                -0.0,
                -16.75,
                runtime_bitxor(-0.0, -16.75),
            ),
            (
                "bitand-all-ones-nan",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                -1.0,
                f64::NAN,
                runtime_bitand(-1.0, f64::NAN),
            ),
            (
                "bitand-all-ones-positive-infinity",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                -1.0,
                f64::INFINITY,
                runtime_bitand(-1.0, f64::INFINITY),
            ),
        ];

        for (name, instruction, _elided_op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushParam(0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[NativeOp::LoadParam(0), NativeOp::IntegerCast],
                "{name}: identity bitwise op should lower to integer conversion"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: identity bitwise op should not allocate an LHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile integer-cast leaf");

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer-cast leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
            assert_eq!(
                ctx.take_runtime_error().is_some(),
                crate::integer_runtime::real_to_integer(right).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_safe_constant_integer_binary_to_literals() {
        let cases = [
            ("shl", Instruction::Shl, 3.0, 2.0, runtime_shl(3.0, 2.0)),
            (
                "shr-negative",
                Instruction::Shr,
                -16.0,
                2.0,
                runtime_shr(-16.0, 2.0),
            ),
            (
                "bitand",
                Instruction::BitAnd,
                13.0,
                6.0,
                runtime_bitand(13.0, 6.0),
            ),
            (
                "bitor",
                Instruction::BitOr,
                8.0,
                3.0,
                runtime_bitor(8.0, 3.0),
            ),
            (
                "bitxor",
                Instruction::BitXor,
                15.0,
                6.0,
                runtime_bitxor(15.0, 6.0),
            ),
            (
                "rounds-operands",
                Instruction::BitAnd,
                13.75,
                6.25,
                runtime_bitand(13.75, 6.25),
            ),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded helper-equivalent literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded integer leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded integer binary op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded integer leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_table_helpers_and_preserves_state() {
        let cases = [
            (
                "lookup-interpolate",
                Instruction::TableLookup(0),
                1.5,
                5.0_f64,
            ),
            (
                "lookup-extrapolate",
                Instruction::TableLookup(0),
                -0.5,
                -1.0_f64,
            ),
            (
                "lookup-second-table",
                Instruction::TableLookup(1),
                1.5,
                13.0_f64,
            ),
            (
                "derivative-interpolate",
                Instruction::TableDerivative(0),
                1.5,
                6.0_f64,
            ),
            (
                "derivative-extrapolate",
                Instruction::TableDerivative(0),
                -0.5,
                2.0_f64,
            ),
            (
                "derivative-second-table",
                Instruction::TableDerivative(1),
                1.5,
                4.0_f64,
            ),
        ];

        for (name, op, input, table_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushConst(input),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile table helper leaf");
            if name.ends_with("second-table") {
                assert!(
                    contains_bytes(
                        &bytes,
                        &mov_r32_imm32_bytes(super::context_filter_id_arg_reg(), 1)
                    ),
                    "table helper should materialize small table IDs with a compact imm32 move"
                );
                assert!(
                    !contains_bytes(
                        &bytes,
                        &movabs_imm64_bytes(super::context_filter_id_arg_reg(), 1)
                    ),
                    "table helper should not use movabs for small table IDs"
                );
            }
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate table helper leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let table = [
                LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![0.0, 2.0, 8.0]),
                LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 15.0]),
            ];
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            ctx.lookup_tables = table.as_ptr();
            ctx.lookup_tables_len = table.len();
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + table_expected) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_ifelse() {
        let then_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let else_neg_zero = -0.0_f64;
        let cases = [
            ("true", 2.0e-15, 7.0, 3.0, 7.0_f64.to_bits()),
            ("tiny-nonzero", 0.5e-15, 7.0, 3.0, 7.0_f64.to_bits()),
            ("larger-nonzero", 1.0e-15, 7.0, 3.0, 7.0_f64.to_bits()),
            ("unordered", f64::NAN, 7.0, 3.0, 7.0_f64.to_bits()),
            (
                "selected-then-bits",
                2.0e-15,
                then_nan,
                3.0,
                then_nan.to_bits(),
            ),
            (
                "selected-else-bits",
                0.0,
                7.0,
                else_neg_zero,
                else_neg_zero.to_bits(),
            ),
        ];

        for (name, cond, then_value, else_value, expected_bits) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(cond),
                    Instruction::PushConst(then_value),
                    Instruction::PushConst(else_value),
                    Instruction::IfElse,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile ifelse leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate ifelse leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected_bits, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_variable_condition_ifelse_with_literal_else() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(9),
                    Instruction::PushVariable(3),
                    Instruction::PushVariable(0),
                    Instruction::Mul,
                    Instruction::PushConst(0.0),
                    Instruction::IfElse,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 34, 0),
        )
        .expect("lower variable ifelse leaf");
        let bytes = compile_value_function(&program).expect("compile variable ifelse leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate variable ifelse leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        let mut vars = vec![0.0; 34];

        vars[0] = 2.0;
        vars[3] = 4.0;
        vars[9] = 1.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());

        vars[9] = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_analysis_checks() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::Analysis(2)], 0);
        let bytes = compile_value_function(&program).expect("compile analysis leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate analysis leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.analysis_type = 2;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.analysis_type = 0;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());

        let static_program =
            native_program(EntryKind::StampValue, vec![Instruction::Analysis(5)], 0);
        let static_bytes =
            compile_value_function(&static_program).expect("compile static analysis leaf");
        let static_memory =
            ExecutableMemory::allocate(&static_bytes).expect("allocate static analysis leaf");
        let static_entry = static_memory.ptr_at(0).expect("entry point inside image");
        let static_check: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(static_entry) };

        for analysis_type in [0, 4] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                static_check(&ctx, std::ptr::null()).to_bits(),
                1.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
        for analysis_type in [1, 2, 3, 5] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                static_check(&ctx, std::ptr::null()).to_bits(),
                0.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }

        let smallsig_program =
            native_program(EntryKind::StampValue, vec![Instruction::Analysis(6)], 0);
        let smallsig_bytes =
            compile_value_function(&smallsig_program).expect("compile smallsig analysis leaf");
        let smallsig_memory =
            ExecutableMemory::allocate(&smallsig_bytes).expect("allocate smallsig analysis leaf");
        let smallsig_entry = smallsig_memory.ptr_at(0).expect("entry point inside image");
        let smallsig_check: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(smallsig_entry) };

        for analysis_type in [1, 3] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                smallsig_check(&ctx, std::ptr::null()).to_bits(),
                1.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
        for analysis_type in [0, 2, 4, 5] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                smallsig_check(&ctx, std::ptr::null()).to_bits(),
                0.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_executes_timer_state_and_preserves_stack() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(0.0),
                Instruction::PushConst(1.0),
                Instruction::TimerState(0),
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile timer leaf");
        let frame_bytes = call_frame_bytes(5);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "timer helper should reserve its live operands and lower prefix"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "timer helper should release its live operand frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "timer helper should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "timer helper should not release the old maximum spill frame"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate timer leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.analysis_type = 2;
        ctx.temperature = 310.0;
        ctx.timestep = 0.01;

        ctx.time = 1.25;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 310.0_f64.to_bits());

        ctx.time = 1.5;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 311.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_min_max() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, op, left, right) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    op.clone(),
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile min/max leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = left;
            let vars = [right];
            let expected = match op {
                Instruction::Min => runtime_min(left, right),
                Instruction::Max => runtime_max(left, right),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_min_max_without_extra_stack_slot() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, instruction, input, rhs) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    instruction.clone(),
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS min/max, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal RHS min/max leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS min/max should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;
            let expected = match instruction {
                Instruction::Min => runtime_min(input, rhs),
                Instruction::Max => runtime_max(input, rhs),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_min_max_without_extra_stack_slot() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, instruction, lhs, input) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction.clone(),
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS min/max, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal LHS min/max leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS min/max should stay helper-free"
            );
            assert!(
                !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: constant LHS min/max with a spare XMM register should not spill the RHS"
            );
            assert!(
                !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: constant LHS min/max with a spare XMM register should not restore a spill frame"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;
            let expected = match instruction {
                Instruction::Min => runtime_min(lhs, input),
                Instruction::Max => runtime_max(lhs, input),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_min_max_at_full_xmm_stack_depth() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushConst(-2.0),
            Instruction::PushTemperature,
            Instruction::Min,
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes =
            compile_value_function(&program).expect("compile full-stack literal LHS min/max leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "full-stack constant LHS min/max should remain helper-free"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "allocated scratch should avoid saving the dynamic RHS in an instruction-local frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "allocated scratch should avoid an instruction-local min/max restore"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack literal LHS min/max");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 5.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) + runtime_min(-2.0, 5.0)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_calls_unary_math_helpers_and_preserves_state() {
        let cases = [
            ("exp", Instruction::Exp, 0.5, runtime_exp(0.5)),
            ("log", Instruction::Log, 2.5, runtime_log(2.5)),
            ("log10", Instruction::Log10, 100.0, runtime_log10(100.0)),
            ("sin", Instruction::Sin, 0.5, runtime_sin(0.5)),
            ("cos", Instruction::Cos, 0.5, runtime_cos(0.5)),
            ("tan", Instruction::Tan, 0.25, runtime_tan(0.25)),
            ("sinh", Instruction::Sinh, 0.25, runtime_sinh(0.25)),
            ("cosh", Instruction::Cosh, 0.25, runtime_cosh(0.25)),
            ("tanh", Instruction::Tanh, 0.25, runtime_tanh(0.25)),
            (
                "limexp-linear",
                Instruction::Limexp,
                45.0,
                runtime_limexp(45.0),
            ),
            (
                "limexp-negative",
                Instruction::Limexp,
                -50.0,
                runtime_limexp(-50.0),
            ),
            (
                "limited-exp-linear",
                Instruction::LimitedExp,
                85.0,
                runtime_limited_exp(85.0),
            ),
            (
                "limited-exp-negative",
                Instruction::LimitedExp,
                -85.0,
                runtime_limited_exp(-85.0),
            ),
            ("asin", Instruction::Asin, 0.25, runtime_asin(0.25)),
            ("acos", Instruction::Acos, 0.25, runtime_acos(0.25)),
            ("atan", Instruction::Atan, 0.25, runtime_atan(0.25)),
            ("asinh", Instruction::Asinh, 0.25, runtime_asinh(0.25)),
            ("acosh", Instruction::Acosh, 1.25, runtime_acosh(1.25)),
            ("atanh", Instruction::Atanh, 0.25, runtime_atanh(0.25)),
            ("floor", Instruction::Floor, 3.75, runtime_floor(3.75)),
            (
                "floor-negative",
                Instruction::Floor,
                -3.25,
                runtime_floor(-3.25),
            ),
            ("ceil", Instruction::Ceil, 3.25, runtime_ceil(3.25)),
            (
                "ceil-negative",
                Instruction::Ceil,
                -3.75,
                runtime_ceil(-3.75),
            ),
        ];

        for (name, op, input, unary_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(input),
                    op,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile helper-call leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper-call leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((unary_expected + 7.0) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_floor_ceil_without_helper_call() {
        let cases = [
            ("floor-positive", Instruction::Floor, 3.75, 3.0_f64),
            ("floor-negative", Instruction::Floor, -3.25, -4.0_f64),
            ("floor-integral", Instruction::Floor, -4.0, -4.0_f64),
            ("floor-negative-zero", Instruction::Floor, -0.0, -0.0_f64),
            (
                "floor-huge",
                Instruction::Floor,
                4_503_599_627_370_496.0,
                4_503_599_627_370_496.0,
            ),
            ("ceil-positive", Instruction::Ceil, 3.25, 4.0_f64),
            ("ceil-negative", Instruction::Ceil, -3.75, -3.0_f64),
            ("ceil-integral", Instruction::Ceil, 4.0, 4.0_f64),
            ("ceil-negative-zero", Instruction::Ceil, -0.0, -0.0_f64),
            (
                "ceil-huge",
                Instruction::Ceil,
                -4_503_599_627_370_496.0,
                -4_503_599_627_370_496.0,
            ),
        ];

        for (name, instruction, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), instruction],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile helper-free floor/ceil");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: floor/ceil should not pay helper-call prologue cost"
            );
            assert!(
                !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: floor/ceil with a spare XMM register should not spill the original value"
            );
            assert!(
                !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: floor/ceil with a spare XMM register should not restore a spill frame"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate floor/ceil leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_floor_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([Instruction::PushParam(0), Instruction::Floor]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile full-stack floor leaf");
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "liveness allocation should keep floor scratch in a register"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "floor should not create an instruction-local spill restore"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-stack floor leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[3.75], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 3.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_inlines_limexp_clamped_regions() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParam(0), Instruction::Limexp],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile inline limexp leaf");
        assert!(
            !contains_bytes(
                &bytes,
                &(rspice_limexp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limexp should not call the limexp helper"
        );
        assert!(
            contains_bytes(
                &bytes,
                &(rspice_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limexp should only call exp for the middle region"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate inline limexp leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        for (name, input) in [
            ("middle", 0.5),
            ("upper-threshold", 40.0),
            ("lower-threshold", -40.0),
            ("high-linear", 45.0),
            ("low-clamped", -50.0),
            ("nan", f64::NAN),
        ] {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                runtime_limexp(input).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_inlines_limited_exp_clamped_regions() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParam(0), Instruction::LimitedExp],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile inline limited-exp leaf");
        assert!(
            !contains_bytes(
                &bytes,
                &(rspice_limited_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limited exp should not call the limited-exp helper"
        );
        assert!(
            contains_bytes(
                &bytes,
                &(rspice_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limited exp should only call exp for the middle region"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate inline limited-exp leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        for (name, input) in [
            ("middle", 0.5),
            ("upper-threshold", 80.0),
            ("lower-threshold", -80.0),
            ("high-linear", 85.0),
            ("low-clamped", -85.0),
            ("nan", f64::NAN),
        ] {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                runtime_limited_exp(input).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn native_x64_hot_path_codegen_stays_compact_and_avoids_unneeded_calls() {
        let cases = [
            (
                "constant_leaf",
                native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0),
                32,
                false,
            ),
            (
                "arithmetic_context",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(0),
                        Instruction::PushParam(1),
                        Instruction::Mul,
                        Instruction::PushVariable(0),
                        Instruction::Add,
                        Instruction::PushVoltage(0, 1),
                        Instruction::Add,
                        Instruction::PushTemperature,
                        Instruction::PushConst(0.01),
                        Instruction::Mul,
                        Instruction::Add,
                    ],
                    2,
                ),
                192,
                false,
            ),
            (
                "param_variable_dot8",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(0),
                        Instruction::PushVariable(0),
                        Instruction::Mul,
                        Instruction::PushParam(1),
                        Instruction::PushVariable(1),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(2),
                        Instruction::PushVariable(2),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(3),
                        Instruction::PushVariable(3),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(4),
                        Instruction::PushVariable(4),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(5),
                        Instruction::PushVariable(5),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(6),
                        Instruction::PushVariable(6),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(7),
                        Instruction::PushVariable(7),
                        Instruction::Mul,
                        Instruction::Add,
                    ],
                    0,
                ),
                384,
                false,
            ),
            (
                "same_storage_voltage_pair",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(0, 1),
                        Instruction::PushVoltage(2, 3),
                        Instruction::Add,
                    ],
                    4,
                ),
                96,
                false,
            ),
            (
                "dynamic_index_inline",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVariable(0),
                        Instruction::PushVariableDyn {
                            base: 1,
                            len: 4,
                            lower: 0,
                        },
                        Instruction::PushParam(0),
                        Instruction::Add,
                    ],
                    0,
                ),
                320,
                true,
            ),
            (
                "integer_bitwise",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(2),
                        Instruction::PushParam(3),
                        Instruction::BitAnd,
                        Instruction::PushVariable(0),
                        Instruction::Add,
                    ],
                    0,
                ),
                384,
                false,
            ),
        ];

        for (name, program, max_bytes, allow_cold_error_call) in cases {
            let expected_helper = name == "integer_bitwise";
            assert_eq!(
                program_uses_helper_calls(&program),
                expected_helper,
                "{name}: lowering helper classification"
            );

            let bytes = compile_value_function(&program).expect("compile hot path");
            if !allow_cold_error_call && !expected_helper {
                assert!(
                    !contains_bytes(&bytes, &call_rax_bytes()),
                    "{name}: helper-free hot path emitted an indirect helper call"
                );
            }
            assert!(
                bytes.len() <= max_bytes,
                "{name}: generated {} bytes, budget {max_bytes}",
                bytes.len()
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_unary_math_to_literals() {
        let cases = [
            ("exp", Instruction::Exp, 0.5, runtime_exp(0.5)),
            ("log", Instruction::Log, 2.5, runtime_log(2.5)),
            ("log10", Instruction::Log10, 100.0, runtime_log10(100.0)),
            ("sin", Instruction::Sin, 0.5, runtime_sin(0.5)),
            ("cos", Instruction::Cos, 0.5, runtime_cos(0.5)),
            ("tan", Instruction::Tan, 0.25, runtime_tan(0.25)),
            ("sinh", Instruction::Sinh, 0.25, runtime_sinh(0.25)),
            ("cosh", Instruction::Cosh, 0.25, runtime_cosh(0.25)),
            ("tanh", Instruction::Tanh, 0.25, runtime_tanh(0.25)),
            (
                "limexp-linear",
                Instruction::Limexp,
                45.0,
                runtime_limexp(45.0),
            ),
            (
                "limexp-negative",
                Instruction::Limexp,
                -50.0,
                runtime_limexp(-50.0),
            ),
            (
                "limited-exp-linear",
                Instruction::LimitedExp,
                85.0,
                runtime_limited_exp(85.0),
            ),
            (
                "limited-exp-negative",
                Instruction::LimitedExp,
                -85.0,
                runtime_limited_exp(-85.0),
            ),
            ("asin", Instruction::Asin, 0.25, runtime_asin(0.25)),
            ("asin-domain-nan", Instruction::Asin, 2.0, runtime_asin(2.0)),
            ("acos", Instruction::Acos, 0.25, runtime_acos(0.25)),
            ("atan", Instruction::Atan, 0.25, runtime_atan(0.25)),
            ("asinh", Instruction::Asinh, 0.25, runtime_asinh(0.25)),
            ("acosh", Instruction::Acosh, 1.25, runtime_acosh(1.25)),
            ("atanh", Instruction::Atanh, 0.25, runtime_atanh(0.25)),
            ("floor", Instruction::Floor, 3.75, runtime_floor(3.75)),
            (
                "floor-negative",
                Instruction::Floor,
                -3.25,
                runtime_floor(-3.25),
            ),
            ("ceil", Instruction::Ceil, 3.25, runtime_ceil(3.25)),
            (
                "ceil-negative",
                Instruction::Ceil,
                -3.75,
                runtime_ceil(-3.75),
            ),
        ];

        for (name, instruction, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushConst(input), instruction],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            match program.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{name} should compile as the helper-equivalent folded literal"
                ),
                other => panic!("{name} should compile as one folded literal, got {other:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded unary math leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded unary math should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded unary math leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_binary_math_helpers_and_preserves_state() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                runtime_pow(2.0, 3.0),
            ),
            (
                "fn-pow",
                Instruction::FnPow,
                4.0,
                0.5,
                runtime_pow(4.0, 0.5),
            ),
            (
                "atan2",
                Instruction::Atan2,
                0.5,
                0.25,
                runtime_atan2(0.5, 0.25),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, runtime_mod(5.25, 2.0)),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                runtime_mod(-5.25, 2.0),
            ),
            (
                "mod-negative-divisor",
                Instruction::Mod,
                5.25,
                -2.0,
                runtime_mod(5.25, -2.0),
            ),
        ];

        for (name, op, left, right, binary_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile binary helper-call leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate binary helper leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + binary_expected) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_binary_math_to_literals() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                runtime_pow(2.0, 3.0),
            ),
            (
                "fn-pow",
                Instruction::FnPow,
                4.0,
                0.5,
                runtime_pow(4.0, 0.5),
            ),
            (
                "pow-domain-nan",
                Instruction::Pow,
                -4.0,
                0.5,
                runtime_pow(-4.0, 0.5),
            ),
            (
                "atan2",
                Instruction::Atan2,
                0.5,
                0.25,
                runtime_atan2(0.5, 0.25),
            ),
            (
                "atan2-signed-zero",
                Instruction::Atan2,
                -0.0,
                -0.0,
                runtime_atan2(-0.0, -0.0),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, runtime_mod(5.25, 2.0)),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                runtime_mod(-5.25, 2.0),
            ),
            (
                "mod-zero-divisor",
                Instruction::Mod,
                5.25,
                0.0,
                runtime_mod(5.25, 0.0),
            ),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            match program.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{name} should compile as the helper-equivalent folded literal"
                ),
                other => panic!("{name} should compile as one folded literal, got {other:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded binary math leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded binary math should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded binary math leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_spills_max_depth_across_helper_call() {
        let input = 0.25;
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(5.0),
                Instruction::PushConst(input),
                Instruction::Exp,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile max-depth helper-call leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate max-depth helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + runtime_exp(input)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_spills_max_depth_across_binary_helper_call() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(5.0),
                Instruction::PushConst(2.0),
                Instruction::Pow,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
            ],
            0,
        );
        let bytes =
            compile_value_function(&program).expect("compile max-depth binary helper-call leaf");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate max-depth binary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0 + (2.0 + (3.0 + (4.0 + runtime_pow(5.0, 2.0))))).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_spills_deep_expression_across_helper_call() {
        let stack_depth = 64;
        let mut ops = (0..stack_depth)
            .map(NativeOp::LoadVariable)
            .collect::<Vec<_>>();
        ops.push(NativeOp::UnaryMath(UnaryMathOp::Exp));
        ops.extend(std::iter::repeat_n(NativeOp::Add, stack_depth - 1));
        let program = NativeProgram::from_ops_for_test(ops, stack_depth, Vec::new(), Vec::new());
        let bytes =
            compile_value_function(&program).expect("compile deep expression-spilling leaf");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate deep expression-spilling leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        let mut variables = (1..=stack_depth)
            .map(|value| value as f64)
            .collect::<Vec<_>>();
        variables[stack_depth - 1] = 0.25;
        let mut expected = runtime_exp(variables[stack_depth - 1]);
        for value in variables[..stack_depth - 1].iter().rev() {
            expected += *value;
        }

        assert_eq!(f(&ctx, variables.as_ptr()).to_bits(), expected.to_bits());
    }

    #[test]
    fn generated_value_leaf_rejects_unbounded_expression_stack() {
        let program = NativeProgram::from_ops_for_test(
            vec![NativeOp::Const(1.0)],
            super::MAX_EXPRESSION_STACK_DEPTH + 1,
            Vec::new(),
            Vec::new(),
        );
        let error =
            compile_value_function(&program).expect_err("unbounded expression stack must fail");
        assert!(error.to_string().contains("4096-value safety limit"));
    }

    #[test]
    fn generated_value_leaf_runs_from_nonzero_concatenated_image_offset() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(12.0),
                Instruction::PushConst(0.5),
                Instruction::Add,
            ],
            0,
        );
        let function = compile_value_function(&program).expect("compile literal value function");
        let prefix = [0xC3_u8];
        let mut image = prefix.to_vec();
        image.extend_from_slice(&function);

        let memory = ExecutableMemory::allocate(&image).expect("allocate concatenated image");
        let entry = memory
            .ptr_at(prefix.len())
            .expect("entry point inside concatenated image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), 12.5);
    }

    #[test]
    fn rejects_variable_index_that_exceeds_disp32_range() {
        let too_large_index = (i32::MAX as usize / std::mem::size_of::<f64>()) + 1;
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![Instruction::PushVariable(too_large_index)],
            },
            NativeLoweringLimits::new(0, 0, 0, too_large_index + 1, 0),
        )
        .expect("large variable index is valid IR before x64 disp32 lowering");

        let error = compile_value_function(&program)
            .expect_err("large variable index must not truncate displacement");

        assert!(matches!(error, crate::native::JitError::Encoding { .. }));
        assert!(error.to_string().contains("disp32"));
    }

    #[test]
    fn generated_value_leaf_negates_at_full_xmm_stack_depth() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([Instruction::PushTemperature, Instruction::Neg]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes = compile_value_function(&program).expect("compile full-stack dynamic neg leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic neg should remain helper-free at full XMM stack depth"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack dynamic neg leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 6.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) - 6.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_loads_differential_voltage_at_full_xmm_stack_depth_without_scratch() {
        let prefix = variable_prefix(0, XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.push(Instruction::PushVoltage(0, 1));
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 2);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes =
            compile_value_function(&program).expect("compile full-stack differential voltage leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "differential voltage load should stay helper-free at full XMM stack depth"
        );
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(VOLTAGES_OFFSET)),
            1,
            "full-stack differential voltage load should reuse one terminal voltage base pointer"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-stack voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[9.0, 4.0], &[], &[]);
        let vars: Vec<f64> = (0..prefix.len()).map(|index| (index + 1) as f64).collect();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 5.0).to_bits()
        );
    }

    #[test]
    #[ignore = "release-only direct native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_microbench_reports_direct_call_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 microbench is release-only; rerun with --release"
        );
        let iterations = native_microbench_iterations();
        let samples = native_microbench_samples();
        eprintln!("native-x64-microbench iterations={iterations} samples={samples}");

        let params = [1.25_f64, 3.5, 13.75, 6.25, 0.5, 1.5, 2.5, 3.5];
        let vars = [7.0_f64, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0];
        let dyn_vars = [2.0_f64, 10.0, 20.0, 30.0, 40.0];
        let voltages = [9.0_f64, 4.0, 12.0, 2.0];
        let branch_currents = [f64::NAN, 4.0_f64, 6.5_f64];
        let mut ctx = eval_context(&params, &voltages, &[], &[]);
        ctx.temperature = 300.0;
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();

        run_native_value_microbench(
            "constant_leaf",
            native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            1.0,
        );
        run_native_value_microbench(
            "arithmetic_context",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushParam(1),
                    Instruction::Mul,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                    Instruction::PushVoltage(0, 1),
                    Instruction::Add,
                    Instruction::PushTemperature,
                    Instruction::PushConst(0.01),
                    Instruction::Mul,
                    Instruction::Add,
                ],
                2,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            19.375,
        );
        run_native_value_microbench(
            "param_variable_dot8",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushVariable(0),
                    Instruction::Mul,
                    Instruction::PushParam(1),
                    Instruction::PushVariable(1),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(2),
                    Instruction::PushVariable(2),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(3),
                    Instruction::PushVariable(3),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(4),
                    Instruction::PushVariable(4),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(5),
                    Instruction::PushVariable(5),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(6),
                    Instruction::PushVariable(6),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(7),
                    Instruction::PushVariable(7),
                    Instruction::Mul,
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            996.25,
        );
        run_native_value_microbench(
            "same_storage_voltage_pair",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushVoltage(0, 1),
                    Instruction::PushVoltage(2, 3),
                    Instruction::Add,
                ],
                4,
            ),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            15.0,
        );
        run_native_value_microbench(
            "dynamic_index_inline",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushVariable(0),
                    Instruction::PushVariableDyn {
                        base: 1,
                        len: 4,
                        lower: 0,
                    },
                    Instruction::PushParam(0),
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            dyn_vars.as_ptr(),
            iterations,
            samples,
            31.25,
        );
        run_native_value_microbench(
            "integer_bitwise",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(2),
                    Instruction::PushParam(3),
                    Instruction::BitAnd,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            // Integer conversion rounds 13.75 to 14 and 6.25 to 6, so
            // `(14 & 6) + 7 == 13` under the shared runtime contract.
            13.0,
        );
        run_native_value_microbench(
            "guarded_current_pair",
            NativeProgram::from_ops_for_test(
                vec![
                    NativeOp::LoadCurrent(1),
                    NativeOp::LoadCurrent(2),
                    NativeOp::Add,
                ],
                2,
                vec![1, 2],
                Vec::new(),
            ),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            10.5,
        );

        let state_vars = [2.0_f64];
        let previous_state = [0.0_f64, 1.5_f64];
        let older_state = previous_state;
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_derivatives = [0.0_f64, 0.0_f64];
        let previous_derivatives = [0.0_f64, 0.0_f64];
        let mut state_initialized = [1_u8, 1_u8];
        let mut state_older_candidate = [0.0_f64, 0.0_f64];
        let mut state_ctx = eval_context(&[], &[], &[], &[]);
        set_backward_euler(&mut state_ctx, 0.25);
        state_ctx.state_prev = previous_state.as_ptr();
        state_ctx.state_prev_len = previous_state.len();
        state_ctx.state_older = older_state.as_ptr();
        state_ctx.state_older_len = older_state.len();
        state_ctx.state_values = state_values.as_mut_ptr();
        state_ctx.state_values_len = state_values.len();
        state_ctx.state_derivatives = state_derivatives.as_mut_ptr();
        state_ctx.state_derivatives_len = state_derivatives.len();
        state_ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        state_ctx.state_derivatives_prev_len = previous_derivatives.len();
        state_ctx.state_initialized = state_initialized.as_mut_ptr();
        state_ctx.state_initialized_len = state_initialized.len();
        state_ctx.state_candidate_valid = state_initialized.as_mut_ptr();
        state_ctx.state_candidate_valid_len = state_initialized.len();
        state_ctx.state_older_candidate = state_older_candidate.as_mut_ptr();
        state_ctx.state_older_candidate_len = state_older_candidate.len();
        run_native_value_microbench(
            "stateful_ddt",
            native_program(
                EntryKind::StampValue,
                vec![Instruction::PushVariable(0), Instruction::DdtState(1)],
                0,
            ),
            &state_ctx,
            state_vars.as_ptr(),
            iterations,
            samples,
            2.0,
        );

        let laplace_vars = [2.0_f64];
        let mut laplace_filters = [
            StateSpaceFilter::integrator(1.0).expect("first-order Laplace microbenchmark filter")
        ];
        let mut laplace_ctx = eval_context(&[], &[], &[], &[]);
        laplace_ctx.analysis_type = 2;
        laplace_ctx.integration_active = 1;
        laplace_ctx.integration_derivative_scale = 4.0;
        laplace_ctx.integration_previous_value_scale = 4.0;
        laplace_ctx.integration_older_value_scale = 0.0;
        laplace_ctx.integration_previous_derivative_scale = 1.0;
        laplace_ctx.laplace_filters = laplace_filters.as_mut_ptr();
        laplace_ctx.laplace_filters_len = laplace_filters.len();
        run_native_value_microbench(
            "laplace_trapezoidal",
            NativeProgram::from_bytecode(
                "native-x64-laplace-microbench",
                EntryKind::StampValue,
                &BytecodeProgram {
                    instructions: vec![Instruction::PushVariable(0), Instruction::LaplaceState(0)],
                },
                NativeLoweringLimits::new(0, 0, 0, 1, 0).with_laplace_filter_count(1),
            )
            .expect("lower Laplace microbenchmark program"),
            &laplace_ctx,
            laplace_vars.as_ptr(),
            iterations,
            samples,
            0.4,
        );
    }

    fn run_native_value_microbench(
        name: &str,
        program: NativeProgram,
        ctx: &EvalContext,
        vars: *const f64,
        iterations: usize,
        samples: usize,
        expected: f64,
    ) {
        let bytes = compile_value_function(&program).expect("compile native microbench leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate native microbench leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        assert_f64_matches(f(ctx, vars), expected, name);

        let warmup_iterations = iterations / 10;
        let warmup_checksum = run_native_microbench_sample(f, ctx, vars, warmup_iterations.max(1));
        assert!(
            warmup_checksum.is_finite(),
            "{name}: native microbench warmup checksum must stay finite"
        );

        let mut sample_ns_per_eval = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = web_time::Instant::now();
            checksum += run_native_microbench_sample(f, ctx, vars, iterations);
            let elapsed = start.elapsed();
            sample_ns_per_eval.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_eval.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_eval = sample_ns_per_eval[0];
        let median_ns_per_eval = sample_ns_per_eval[sample_ns_per_eval.len() / 2];
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{name}: native microbench checksum must stay finite"
        );
        eprintln!(
            "native-x64-microbench case={name} code_bytes={} min_ns_per_eval={min_ns_per_eval:.3} median_ns_per_eval={median_ns_per_eval:.3} checksum={checksum:.17e}",
            bytes.len(),
        );

        std::hint::black_box(memory);
    }

    fn run_native_microbench_sample(
        f: extern "C" fn(*const EvalContext, *const f64) -> f64,
        ctx: &EvalContext,
        vars: *const f64,
        iterations: usize,
    ) -> f64 {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(f(
                std::hint::black_box(ctx as *const EvalContext),
                std::hint::black_box(vars),
            ));
        }
        std::hint::black_box(checksum)
    }

    fn native_microbench_iterations() -> usize {
        5_000_000
    }

    fn native_microbench_samples() -> usize {
        5
    }

    fn native_program(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
    ) -> NativeProgram {
        native_program_with_internals(entry_kind, instructions, terminal_count, 0)
    }

    fn native_program_with_internals(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
        internal_node_count: usize,
    ) -> NativeProgram {
        let storage_limit = test_storage_limit();
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(
                terminal_count,
                internal_node_count,
                storage_limit,
                storage_limit,
                storage_limit,
            )
            .with_lookup_table_count(8),
        )
        .expect("lower bytecode to native program")
    }

    fn native_program_with_available_current_pairs(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
        available_current_pairs: &[usize],
    ) -> NativeProgram {
        let storage_limit = test_storage_limit();
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(
                terminal_count,
                0,
                storage_limit,
                storage_limit,
                storage_limit,
            )
            .with_lookup_table_count(8)
            .with_available_current_pairs(available_current_pairs),
        )
        .expect("lower bytecode to native program")
    }

    fn test_storage_limit() -> usize {
        XMM_STACK.len().max(8) + 1
    }

    fn run_table_helper_sentinel(helper: TableHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        push_abi_sentinel_const(&mut compiler, 3.25);
        compiler
            .emit_table_helper_call(7, helper)
            .expect("emit table helper ABI sentinel");
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_context_filter_helper_sentinel(helper: ContextFilterHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        let target = push_abi_sentinel_const(&mut compiler, 4.5);
        compiler
            .emit_context_filter_helper_call(target, 9, helper)
            .expect("emit context filter helper ABI sentinel");
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_timer_helper_sentinel(helper: OperandContextFilterHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        let start = push_abi_sentinel_const(&mut compiler, 1.25);
        push_abi_sentinel_const(&mut compiler, 0.75);
        push_abi_sentinel_const(&mut compiler, 0.125);
        push_abi_sentinel_const(&mut compiler, 1.0);
        compiler.emit_operand_context_filter_helper_call(start, 4, 17, helper);
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_operand_context_filter_helper_sentinel(
        helper: OperandContextFilterHelper,
        ctx: &EvalContext,
    ) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        push_abi_sentinel_const(&mut compiler, 99.0);
        let input = push_abi_sentinel_const(&mut compiler, 2.0);
        push_abi_sentinel_const(&mut compiler, 3.0);
        push_abi_sentinel_const(&mut compiler, 4.0);
        compiler.emit_operand_context_filter_helper_call(input, 3, 11, helper);
        compiler.encoder.movsd_xmm_xmm(Xmm::Xmm0, input);
        run_abi_sentinel_value(compiler, ctx)
    }

    fn abi_sentinel_compiler() -> FunctionCompiler {
        FunctionCompiler::new(true, false, true, 0, None, None, 0).expect("ABI sentinel frame")
    }

    fn push_abi_sentinel_const(compiler: &mut FunctionCompiler, value: f64) -> Xmm {
        let register = compiler.push_register().expect("ABI sentinel stack slot");
        compiler.emit_constant_load(register, value);
        register
    }

    fn run_abi_sentinel_value(compiler: FunctionCompiler, ctx: &EvalContext) -> f64 {
        let bytes = compiler
            .finish_value_function()
            .expect("finish ABI sentinel value function")
            .bytes;
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ABI sentinel function");
        let entry = memory.ptr_at(0).expect("ABI sentinel entry point");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        f(ctx, std::ptr::null())
    }

    unsafe extern "C" fn abi_sentinel_table_helper(
        input: f64,
        ctx: *const EvalContext,
        table_id: usize,
    ) -> f64 {
        if input.to_bits() != 3.25_f64.to_bits() {
            return -1.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -2.0;
        };
        if ctx.lookup_tables.is_null() {
            return -3.0;
        }
        if ctx.lookup_tables_len != 2 {
            return -4.0;
        }
        if table_id != 7 {
            return -5.0;
        }
        101.0
    }

    unsafe extern "C" fn abi_sentinel_context_filter_helper(
        input: f64,
        ctx: *const EvalContext,
        filter_id: usize,
    ) -> f64 {
        if input.to_bits() != 4.5_f64.to_bits() {
            return -10.0;
        }
        if filter_id != 9 {
            return -11.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -12.0;
        };
        if ctx.time.to_bits() != 12.0_f64.to_bits()
            || ctx.temperature.to_bits() != 315.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -13.0;
        }
        202.0
    }

    unsafe extern "C" fn abi_sentinel_timer_helper(
        operands: *const f64,
        ctx: *const EvalContext,
        timer_id: usize,
    ) -> f64 {
        if timer_id != 17 || operands.is_null() {
            return -20.0;
        }
        let operands = unsafe { std::slice::from_raw_parts(operands, 4) };
        if operands[0].to_bits() != 1.25_f64.to_bits()
            || operands[1].to_bits() != 0.75_f64.to_bits()
            || operands[2].to_bits() != 0.125_f64.to_bits()
            || operands[3].to_bits() != 1.0_f64.to_bits()
        {
            return -21.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -22.0;
        };
        if ctx.time.to_bits() != 12.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -23.0;
        }
        303.0
    }

    unsafe extern "C" fn abi_sentinel_operand_context_filter_helper(
        operands: *const f64,
        ctx: *const EvalContext,
        filter_id: usize,
    ) -> f64 {
        if filter_id != 11 {
            return -30.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -31.0;
        };
        if ctx.temperature.to_bits() != 315.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -32.0;
        }
        if operands.is_null() {
            return -33.0;
        }
        let operands = unsafe { std::slice::from_raw_parts(operands, 3) };
        if operands[0].to_bits() != 2.0_f64.to_bits()
            || operands[1].to_bits() != 3.0_f64.to_bits()
            || operands[2].to_bits() != 4.0_f64.to_bits()
        {
            return -34.0;
        }
        404.0
    }

    fn stack_alignment_helper_memory() -> ExecutableMemory {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::Rax, Gpr::Rsp);
        encoder.mov_r64_imm32(Gpr::R11, 15);
        encoder.and_r64_r64(Gpr::Rax, Gpr::R11);
        encoder.cmp_r64_imm32(Gpr::Rax, 8);
        encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
        encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        encoder.cvtsi2sd_xmm_r32(Xmm::Xmm0, Gpr::R10);
        encoder.ret();
        ExecutableMemory::allocate(&encoder.into_bytes()).expect("allocate stack helper sentinel")
    }

    fn eval_context(
        params: &[f64],
        voltages: &[f64],
        internal_voltages: &[f64],
        branch_unknowns: &[f64],
    ) -> EvalContext {
        EvalContext {
            voltages: voltages.as_ptr(),
            internal_voltages: internal_voltages.as_ptr(),
            params: params.as_ptr(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: branch_unknowns.as_ptr(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
            timer_event_bound: std::ptr::null_mut(),
            analysis_initial_step: 0,
            analysis_final_step: 0,
            state_older: std::ptr::null(),
            state_older_len: 0,
            state_derivatives: std::ptr::null_mut(),
            state_derivatives_len: 0,
            state_derivatives_prev: std::ptr::null(),
            state_derivatives_prev_len: 0,
            integration_derivative_scale: 0.0,
            integration_previous_value_scale: 0.0,
            integration_older_value_scale: 0.0,
            integration_previous_derivative_scale: 0.0,
            integration_active: 0,
            limiter_active: std::ptr::null_mut(),
            limiting_enabled: 0,
            runtime_status: Default::default(),
            state_candidate_valid: std::ptr::null_mut(),
            state_candidate_valid_len: 0,
            state_older_candidate: std::ptr::null_mut(),
            state_older_candidate_len: 0,
        }
    }

    fn set_backward_euler(ctx: &mut EvalContext, timestep: f64) {
        ctx.timestep = timestep;
        if timestep.is_finite() && timestep.abs() > 1.0e-20 {
            let inverse = 1.0 / timestep;
            ctx.integration_derivative_scale = inverse;
            ctx.integration_previous_value_scale = inverse;
            ctx.integration_older_value_scale = 0.0;
            ctx.integration_previous_derivative_scale = 0.0;
            ctx.integration_active = 1;
        } else {
            ctx.integration_derivative_scale = 0.0;
            ctx.integration_previous_value_scale = 0.0;
            ctx.integration_older_value_scale = 0.0;
            ctx.integration_previous_derivative_scale = 0.0;
            ctx.integration_active = 0;
        }
    }

    fn contains_bytes(bytes: &[u8], needle: &[u8]) -> bool {
        bytes.windows(needle.len()).any(|window| window == needle)
    }

    fn find_bytes(bytes: &[u8], needle: &[u8]) -> Option<usize> {
        bytes
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn assert_f64_matches(actual: f64, expected: f64, context: &str) {
        if expected.is_nan() {
            assert!(
                actual.is_nan(),
                "{context}: expected NaN, got {actual:?} ({:#x})",
                actual.to_bits()
            );
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits(), "{context}");
        }
    }

    fn sub_rsp_bytes(value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.sub_rsp_imm32(value);
        encoder.into_bytes()
    }

    fn add_rsp_bytes(value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.add_rsp_imm32(value);
        encoder.into_bytes()
    }

    fn addsd_xmm_xmm_bytes(dst: Xmm, src: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.addsd_xmm_xmm(dst, src);
        encoder.into_bytes()
    }

    fn mulsd_xmm_m64_rip_prefix_bytes(dst: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mulsd_xmm_m64_rip_disp32(dst, 0);
        let mut bytes = encoder.into_bytes();
        bytes.truncate(bytes.len() - std::mem::size_of::<i32>());
        bytes
    }

    fn subsd_xmm_m64_rip_prefix_bytes(dst: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.subsd_xmm_m64_rip_disp32(dst, 0);
        let mut bytes = encoder.into_bytes();
        bytes.truncate(bytes.len() - std::mem::size_of::<i32>());
        bytes
    }

    fn stack_spill_store_bytes(register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, register);
        encoder.into_bytes()
    }

    fn stack_spill_minsd_bytes(register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.minsd_xmm_m64_base_disp32(register, Gpr::Rsp, 0);
        encoder.into_bytes()
    }

    fn call_rax_bytes() -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.call_r64(Gpr::Rax);
        encoder.into_bytes()
    }

    fn call_frame_bytes(slot_count: usize) -> i32 {
        super::call_frame_bytes_for_slots(slot_count)
    }

    fn old_fixed_call_frame_bytes() -> i32 {
        call_frame_bytes(XMM_STACK.len() + 1)
    }

    fn constant_prefix(count: usize) -> Vec<Instruction> {
        (0..count)
            .map(|index| Instruction::PushConst((index + 1) as f64))
            .collect()
    }

    fn variable_prefix(start: usize, count: usize) -> Vec<Instruction> {
        (0..count)
            .map(|index| Instruction::PushVariable(start + index))
            .collect()
    }

    fn vars_with_prefix(first: f64, count: usize) -> Vec<f64> {
        let mut vars = Vec::with_capacity(count + 1);
        vars.push(first);
        vars.extend((0..count).map(|index| (index + 1) as f64));
        vars
    }

    fn add_reductions(count: usize) -> Vec<Instruction> {
        (0..count).map(|_| Instruction::Add).collect()
    }

    fn constant_prefix_sum(count: usize) -> f64 {
        (count * (count + 1) / 2) as f64
    }

    fn count_bytes(bytes: &[u8], needle: &[u8]) -> usize {
        bytes
            .windows(needle.len())
            .filter(|window| *window == needle)
            .count()
    }

    fn context_pointer_load_bytes(ctx_field_offset: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, entry_ctx_arg_reg(), ctx_field_offset);
        encoder.into_bytes()
    }

    fn old_disp32_context_pointer_load_bytes(ctx_field_offset: i32) -> Vec<u8> {
        let modrm = match entry_ctx_arg_reg() {
            Gpr::Rcx => 0x81,
            Gpr::Rdi => 0x87,
            _ => unreachable!("native entry context arg register is fixed by the host ABI"),
        };
        let mut bytes = vec![0x48, 0x8B, modrm];
        bytes.extend_from_slice(&ctx_field_offset.to_le_bytes());
        bytes
    }

    fn dynamic_variable_scaled_address_bytes(base: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.lea_r64_base_index_scale8_disp32(
            Gpr::R11,
            entry_vars_arg_reg(),
            Gpr::R10,
            super::byte_disp(base).expect("dynamic address test index fits disp32"),
        );
        encoder.into_bytes()
    }

    fn dynamic_variable_shift_add_address_bytes(base: usize) -> Vec<u8> {
        let base_disp = super::byte_disp(base).expect("dynamic address test index fits disp32");
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::Rax, Gpr::R10);
        encoder.shl_r64_imm8(Gpr::Rax, 3);
        encoder.mov_r64_r64(Gpr::R11, entry_vars_arg_reg());
        if base_disp != 0 {
            encoder.add_r64_imm32(Gpr::R11, base_disp);
        }
        encoder.add_r64_r64(Gpr::Rax, Gpr::R11);
        encoder.into_bytes()
    }

    fn sub_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.sub_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn cmp_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.cmp_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn stack_probe_touch_bytes() -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::R10, 0);
        encoder.into_bytes()
    }

    fn dynamic_variable_movabs_sub_lower_bytes(lower: i64) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, lower as u64);
        encoder.sub_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn dynamic_variable_movabs_cmp_len_bytes(len: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, len as u64);
        encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn guarded_slice_index_cmp_imm32_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.cmp_r64_imm32(
            Gpr::R10,
            super::slice_index_imm32(index).expect("guarded slice test index fits imm32"),
        );
        encoder.into_bytes()
    }

    fn guarded_slice_index_cmp_register_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, index as u64);
        encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn mov_r32_imm32_bytes(register: Gpr, value: u32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r32_imm32(register, value);
        encoder.into_bytes()
    }

    fn mov_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn movabs_imm64_bytes(register: Gpr, value: u64) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(register, value);
        encoder.into_bytes()
    }

    fn xor_r64_bytes(dst: Gpr, src: Gpr) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.xor_r64_r64(dst, src);
        encoder.into_bytes()
    }

    fn mov_r11_rsp_bytes() -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        encoder.into_bytes()
    }

    fn xorpd_xmm_bytes(dst: Xmm, src: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.xorpd_xmm_xmm(dst, src);
        encoder.into_bytes()
    }

    fn andpd_rip_prefix_bytes(target: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.andpd_xmm_m128_rip_disp32(target, 0);
        encoder.into_bytes()[..4].to_vec()
    }

    fn abs_value_mask_bytes() -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&ABS_VALUE_MASK_LOW.to_le_bytes());
        bytes.extend_from_slice(&ABS_VALUE_MASK_HIGH.to_le_bytes());
        bytes
    }

    fn scalar_abs_integer_bit_clear_bytes(target: Xmm, scratch: Gpr) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movq_r64_xmm(scratch, target);
        encoder.btr_r64_imm8(scratch, 63);
        encoder.movq_xmm_r64(target, scratch);
        encoder.into_bytes()
    }

    fn call_frame_spill_bytes(index: usize, register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, super::call_spill_disp(index), register);
        encoder.into_bytes()
    }

    fn call_frame_load_bytes(register: Xmm, index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_xmm_m64_base_disp32(register, Gpr::Rsp, super::call_spill_disp(index));
        encoder.into_bytes()
    }

    #[cfg(windows)]
    fn callee_saved_xmm_store_bytes(register: Xmm, disp: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movdqu_m128_base_disp32_xmm(Gpr::Rsp, disp, register);
        encoder.into_bytes()
    }

    #[cfg(windows)]
    fn callee_saved_xmm_load_bytes(register: Xmm, disp: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movdqu_xmm_m128_base_disp32(register, Gpr::Rsp, disp);
        encoder.into_bytes()
    }

    fn same_storage_voltage_memory_subtract_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.subsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::Rax,
            super::byte_disp(index).expect("same-node test index fits disp32"),
        );
        encoder.into_bytes()
    }

    fn assert_missing_state_storage_error(error: &str) {
        assert!(
            error.contains("invalid state storage"),
            "error must identify missing native state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("invalid state storage"),
            "error must identify out-of-range native state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_limit_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("limit state index outside state storage"),
            "error must identify out-of-range limit state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_prior_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("invalid state storage"),
            "error must identify out-of-range native prior-state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_current_probe_error(error: &str) {
        assert!(
            error.contains("missing terminal-pair current storage"),
            "error must identify missing native current-probe storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_prior_current_error(error: &str) {
        assert!(
            error.contains("missing contribution current storage"),
            "error must identify missing native prior-current storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_param_given_error(error: &str) {
        assert!(
            error.contains("missing parameter-given storage"),
            "error must identify missing native param_given storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_port_connected_error(error: &str) {
        assert!(
            error.contains("missing connection-flag storage"),
            "error must identify missing native port_connected storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn runtime_min(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).min(std::hint::black_box(right))
    }

    fn runtime_max(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).max(std::hint::black_box(right))
    }

    fn runtime_exp(value: f64) -> f64 {
        std::hint::black_box(value).exp()
    }

    fn runtime_log(value: f64) -> f64 {
        std::hint::black_box(value).ln()
    }

    fn runtime_log10(value: f64) -> f64 {
        std::hint::black_box(value).log10()
    }

    fn runtime_sin(value: f64) -> f64 {
        std::hint::black_box(value).sin()
    }

    fn runtime_cos(value: f64) -> f64 {
        std::hint::black_box(value).cos()
    }

    fn runtime_tan(value: f64) -> f64 {
        std::hint::black_box(value).tan()
    }

    fn runtime_sinh(value: f64) -> f64 {
        std::hint::black_box(value).sinh()
    }

    fn runtime_cosh(value: f64) -> f64 {
        std::hint::black_box(value).cosh()
    }

    fn runtime_tanh(value: f64) -> f64 {
        std::hint::black_box(value).tanh()
    }

    fn runtime_limexp(value: f64) -> f64 {
        const LIMIT: f64 = 40.0;
        let value = std::hint::black_box(value);
        if value > LIMIT {
            let exp_limit = LIMIT.exp();
            exp_limit * (1.0 + value - LIMIT)
        } else if value < -LIMIT {
            (-LIMIT).exp()
        } else {
            value.exp()
        }
    }

    fn runtime_limited_exp(value: f64) -> f64 {
        const LIMIT: f64 = 80.0;
        const LOW_VALUE: f64 = 1.804851387e-35;
        let value = std::hint::black_box(value);
        if value > LIMIT {
            LIMIT.exp() * (1.0 + value - LIMIT)
        } else if value < -LIMIT {
            LOW_VALUE
        } else {
            value.exp()
        }
    }

    fn runtime_asin(value: f64) -> f64 {
        std::hint::black_box(value).asin()
    }

    fn runtime_acos(value: f64) -> f64 {
        std::hint::black_box(value).acos()
    }

    fn runtime_atan(value: f64) -> f64 {
        std::hint::black_box(value).atan()
    }

    fn runtime_asinh(value: f64) -> f64 {
        std::hint::black_box(value).asinh()
    }

    fn runtime_acosh(value: f64) -> f64 {
        std::hint::black_box(value).acosh()
    }

    fn runtime_atanh(value: f64) -> f64 {
        std::hint::black_box(value).atanh()
    }

    fn runtime_floor(value: f64) -> f64 {
        std::hint::black_box(value).floor()
    }

    fn runtime_ceil(value: f64) -> f64 {
        std::hint::black_box(value).ceil()
    }

    fn runtime_pow(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).powf(std::hint::black_box(right))
    }

    fn runtime_atan2(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).atan2(std::hint::black_box(right))
    }

    fn runtime_mod(left: f64, right: f64) -> f64 {
        std::hint::black_box(left) % std::hint::black_box(right)
    }

    fn runtime_shl(left: f64, right: f64) -> f64 {
        crate::integer_runtime::integer_binary(
            crate::integer_runtime::IntegerBinaryOperation::Shl,
            std::hint::black_box(left),
            std::hint::black_box(right),
        )
        .unwrap_or(0.0)
    }

    fn runtime_shr(left: f64, right: f64) -> f64 {
        crate::integer_runtime::integer_binary(
            crate::integer_runtime::IntegerBinaryOperation::Shr,
            std::hint::black_box(left),
            std::hint::black_box(right),
        )
        .unwrap_or(0.0)
    }

    fn runtime_bitand(left: f64, right: f64) -> f64 {
        crate::integer_runtime::integer_binary(
            crate::integer_runtime::IntegerBinaryOperation::BitAnd,
            std::hint::black_box(left),
            std::hint::black_box(right),
        )
        .unwrap_or(0.0)
    }

    fn runtime_bitor(left: f64, right: f64) -> f64 {
        crate::integer_runtime::integer_binary(
            crate::integer_runtime::IntegerBinaryOperation::BitOr,
            std::hint::black_box(left),
            std::hint::black_box(right),
        )
        .unwrap_or(0.0)
    }

    fn runtime_bitxor(left: f64, right: f64) -> f64 {
        crate::integer_runtime::integer_binary(
            crate::integer_runtime::IntegerBinaryOperation::BitXor,
            std::hint::black_box(left),
            std::hint::black_box(right),
        )
        .unwrap_or(0.0)
    }

    fn thermal_voltage(temperature: f64) -> f64 {
        temperature * THERMAL_VOLTAGE_PER_K
    }

    // ----------------------------------------------- branch-lowered conditionals

    fn branching_ssa(program: &NativeProgram) -> super::X64SsaProgram {
        super::X64SsaProgram::lower(program)
            .expect("lower the postfix program")
            .with_branching_conditionals()
            .expect("re-express conditionals as branches")
    }

    fn compile_branching_value_function(program: &NativeProgram) -> Vec<u8> {
        super::compile_value_function_artifact_from_ssa(&branching_ssa(program))
            .expect("compile the branch-form value function")
            .bytes
    }

    fn value_entry(
        memory: &ExecutableMemory,
    ) -> extern "C" fn(*const EvalContext, *const f64) -> f64 {
        let entry = memory.ptr_at(0).expect("entry point inside image");
        unsafe { std::mem::transmute(entry) }
    }

    /// Nested conditionals whose arms own real work, including a helper call
    /// that only one arm should reach.
    fn nested_conditional_program() -> NativeProgram {
        NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::LoadParam(2),
                NativeOp::LoadParam(3),
                NativeOp::Mul,
                NativeOp::LoadParam(4),
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::IfElse,
                NativeOp::LoadParam(5),
                NativeOp::Sqrt,
                NativeOp::IfElse,
                NativeOp::LoadParam(6),
                NativeOp::Add,
            ],
            4,
            Vec::new(),
            Vec::new(),
        )
    }

    #[test]
    fn branch_lowered_conditionals_agree_with_the_select_form_bit_for_bit() {
        let program = nested_conditional_program();
        let split = branching_ssa(&program);
        assert!(
            split.blocks().len() == 7,
            "two conditionals lay out two nested diamonds, found {} blocks",
            split.blocks().len()
        );

        let select = ExecutableMemory::allocate(
            &compile_value_function(&program).expect("compile the select form"),
        )
        .expect("allocate select leaf");
        let branch = ExecutableMemory::allocate(&compile_branching_value_function(&program))
            .expect("allocate branch leaf");
        let select = value_entry(&select);
        let branch = value_entry(&branch);

        // Every truthiness class the two lowerings have to agree on, on both
        // conditions: exact zero, negative zero, NaN, infinities, ordinary.
        let truthiness = [
            0.0_f64,
            -0.0,
            f64::NAN,
            -f64::NAN,
            1.0,
            -1.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE,
        ];
        for outer in truthiness {
            for inner in truthiness {
                let params = [outer, inner, 3.0_f64, 0.5, 2.0, 9.0, -4.5];
                let ctx = eval_context(&params, &[], &[], &[]);
                let expected = select(&ctx, std::ptr::null());
                let actual = branch(&ctx, std::ptr::null());
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "branch and select disagree at outer={outer} inner={inner}"
                );
                assert!(ctx.take_runtime_error().is_none());
            }
        }
    }

    #[test]
    fn a_branch_lowered_arm_does_not_run_the_untaken_arms_failing_load() {
        // `condition ? variables[index] : 0.0` with an out-of-range index.
        // The select form evaluates both arms, so the bounds check fires and
        // hard-fails the entry even when the constant arm is the one selected.
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 3,
                    lower: 1,
                },
                NativeOp::Const(7.0),
                NativeOp::IfElse,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );
        let select = ExecutableMemory::allocate(
            &compile_value_function(&program).expect("compile the select form"),
        )
        .expect("allocate select leaf");
        let branch = ExecutableMemory::allocate(&compile_branching_value_function(&program))
            .expect("allocate branch leaf");
        let select = value_entry(&select);
        let branch = value_entry(&branch);
        let variables = [11.0_f64, 22.0, 33.0, 44.0];

        let out_of_range = [0.0_f64, 9.0];
        let ctx = eval_context(&out_of_range, &[], &[], &[]);
        ctx.clear_runtime_error();
        let selected = select(&ctx, variables.as_ptr());
        let select_error = ctx
            .take_runtime_error()
            .expect("the select form evaluates the arm it does not choose");
        assert!(select_error.contains("array index 9 outside declared bounds"));
        assert_eq!(selected.to_bits(), 0.0_f64.to_bits());

        ctx.clear_runtime_error();
        let branched = branch(&ctx, variables.as_ptr());
        assert!(
            ctx.take_runtime_error().is_none(),
            "the branch form never reaches the untaken arm's bounds check"
        );
        assert_eq!(branched.to_bits(), 7.0_f64.to_bits());

        // The taken arm still fails, and both forms still agree when the
        // index is in range.
        let taken = [1.0_f64, 9.0];
        let ctx = eval_context(&taken, &[], &[], &[]);
        ctx.clear_runtime_error();
        branch(&ctx, variables.as_ptr());
        assert!(
            ctx.take_runtime_error().is_some(),
            "a failing load on the taken path must still hard-fail"
        );
        for condition in [0.0_f64, 1.0, f64::NAN] {
            let params = [condition, 2.0_f64];
            let ctx = eval_context(&params, &[], &[], &[]);
            ctx.clear_runtime_error();
            let expected = select(&ctx, variables.as_ptr());
            let actual = branch(&ctx, variables.as_ptr());
            assert_eq!(actual.to_bits(), expected.to_bits());
            assert!(ctx.take_runtime_error().is_none());
        }
    }

    #[test]
    fn branch_lowering_moves_block_parameters_between_spill_slots() {
        // Thirteen simultaneously live values leave the ten-register bank
        // exhausted, so the conditional's condition, its argument, and the
        // join block's parameter all land in spill slots: the edge move is
        // memory to memory.
        let mut ops = (0..13).map(NativeOp::LoadParam).collect::<Vec<_>>();
        ops.push(NativeOp::IfElse);
        ops.extend(vec![NativeOp::Add; 10]);
        let program = NativeProgram::from_ops_for_test(ops, 13, Vec::new(), Vec::new());

        let split = branching_ssa(&program);
        let allocation = super::RegisterAllocation::build(&split, super::X64_VALUE_BANK)
            .expect("register allocation");
        let spill_to_spill = split
            .blocks()
            .iter()
            .flat_map(|block| {
                (0..block.terminator().edge_count())
                    .filter_map(|edge| allocation.edge_moves(block.id(), edge).ok())
                    .flatten()
            })
            .any(|step| {
                matches!(step.from(), super::X64ValueLocation::Spill(_))
                    && matches!(step.to(), super::X64ValueLocation::Spill(_))
            });
        assert!(
            spill_to_spill,
            "this program is the fixture for the memory-to-memory edge move"
        );

        let select = ExecutableMemory::allocate(
            &compile_value_function(&program).expect("compile the select form"),
        )
        .expect("allocate select leaf");
        let branch = ExecutableMemory::allocate(&compile_branching_value_function(&program))
            .expect("allocate branch leaf");
        let select = value_entry(&select);
        let branch = value_entry(&branch);
        for condition in [0.0_f64, -0.0, 1.0, f64::NAN] {
            let mut params = [1.0_f64; 13];
            for (index, param) in params.iter_mut().enumerate() {
                *param = index as f64 * 0.25 - 1.0;
            }
            params[10] = condition;
            let ctx = eval_context(&params, &[], &[], &[]);
            let expected = select(&ctx, std::ptr::null());
            let actual = branch(&ctx, std::ptr::null());
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "spilled-parameter edge move disagrees at condition={condition}"
            );
        }
    }
}
