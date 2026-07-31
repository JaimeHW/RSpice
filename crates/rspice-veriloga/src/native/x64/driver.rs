//! Whole-pass native entry-point drivers.
//!
//! Scalar expression entry points remain useful for diagnostics and narrowly
//! scoped evaluation APIs. The hot stamping path instead enters one driver,
//! which invokes the assignment, contribution, and Jacobian entries inside
//! the executable image and writes into caller-owned, preallocated buffers.

use super::encoder::{ConditionCode, Gpr, X64Encoder, Xmm};
use super::{CompiledX64Function, WindowsX64UnwindInfo, WindowsX64UnwindOperation};
use crate::native::abi::NativeRuntimeStatus;
use crate::native::model::{CodeOffset, NativeStampKernelIo};
use crate::native::{EvalContext, JitError, JitResult};

const WORD_BYTES: usize = std::mem::size_of::<f64>();
const BRANCH_CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, branch_currents) as i32;
const CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, currents) as i32;
const ACTIVE_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, program_active) as i32;
const JACOBIANS_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, jacobians) as i32;

#[cfg(windows)]
const DRIVER_FRAME_BYTES: i32 = 32;
#[cfg(not(windows))]
const DRIVER_FRAME_BYTES: i32 = 0;

/// Compiles a driver whose address will be `driver_image_offset` in the final
/// executable image.
#[allow(dead_code)]
pub(crate) fn compile_stamp_kernel(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    jacobians: &[Vec<CodeOffset>],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    Ok(compile_stamp_kernel_artifact(
        driver_image_offset,
        assignment,
        stamp_values,
        jacobians,
        published_current_pairs,
    )?
    .bytes)
}

#[allow(dead_code)]
pub(crate) fn compile_evaluation_kernel(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    Ok(compile_evaluation_kernel_artifact(
        driver_image_offset,
        assignment,
        stamp_values,
        published_current_pairs,
    )?
    .bytes)
}

pub(super) fn compile_evaluation_kernel_artifact(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<CompiledX64Function> {
    compile_kernel_artifact(
        driver_image_offset,
        assignment,
        stamp_values,
        None,
        published_current_pairs,
        "evaluation",
    )
}

pub(super) fn compile_stamp_kernel_artifact(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    jacobians: &[Vec<CodeOffset>],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<CompiledX64Function> {
    compile_kernel_artifact(
        driver_image_offset,
        assignment,
        stamp_values,
        Some(jacobians),
        published_current_pairs,
        "stamp",
    )
}

fn compile_kernel_artifact(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    jacobians: Option<&[Vec<CodeOffset>]>,
    published_current_pairs: &[Option<(usize, usize)>],
    driver_name: &str,
) -> JitResult<CompiledX64Function> {
    if stamp_values.len() != published_current_pairs.len()
        || jacobians.is_some_and(|entries| stamp_values.len() != entries.len())
    {
        return Err(internal_error(
            "native kernel value, Jacobian, and publication entry shapes differ",
        ));
    }
    let runtime_failed_offset = std::mem::offset_of!(EvalContext, runtime_status)
        .checked_add(NativeRuntimeStatus::failed_offset())
        .and_then(|offset| i32::try_from(offset).ok())
        .ok_or_else(|| internal_error("native runtime status offset exceeds x64 disp32 range"))?;

    let mut encoder = X64Encoder::new();
    let mut abort_branches = Vec::new();
    let mut windows_unwind_operations = Vec::new();
    encoder.push_r64(Gpr::R12);
    record_windows_unwind_push(&mut windows_unwind_operations, &encoder, 12);
    encoder.push_r64(Gpr::R13);
    record_windows_unwind_push(&mut windows_unwind_operations, &encoder, 13);
    encoder.push_r64(Gpr::R14);
    record_windows_unwind_push(&mut windows_unwind_operations, &encoder, 14);
    encoder.mov_r64_r64(Gpr::R12, host_ctx_arg_reg());
    encoder.mov_r64_r64(Gpr::R13, host_vars_arg_reg());
    encoder.mov_r64_r64(Gpr::R14, host_io_arg_reg());
    if DRIVER_FRAME_BYTES > 0 {
        encoder.sub_rsp_imm32(DRIVER_FRAME_BYTES);
        record_windows_unwind_stack_allocation(
            &mut windows_unwind_operations,
            &encoder,
            DRIVER_FRAME_BYTES as u32,
        );
    }
    let windows_unwind_prologue_size = current_windows_unwind_code_offset(&encoder);

    emit_entry_call(&mut encoder, driver_image_offset, assignment)?;
    abort_branches.push(emit_abort_if_failed(&mut encoder, runtime_failed_offset));

    let mut jacobian_index = 0usize;
    for (stamp_index, (value_entry, current_pair)) in
        stamp_values.iter().zip(published_current_pairs).enumerate()
    {
        let active_disp = byte_offset(stamp_index, "stamp active-mask")?;
        encoder.mov_r64_m64_base_disp32(Gpr::R10, Gpr::R14, ACTIVE_OFFSET);
        encoder.movzx_r32_m8_base_disp32(Gpr::Rax, Gpr::R10, active_disp);
        encoder.test_r8_r8(Gpr::Rax, Gpr::Rax);
        let skip_stamp = encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        emit_entry_call(&mut encoder, driver_image_offset, *value_entry)?;
        abort_branches.push(emit_abort_if_failed(&mut encoder, runtime_failed_offset));
        let finite_value = emit_branch_if_finite(&mut encoder, stamp_index, &mut abort_branches);
        patch_local_branch(&mut encoder, finite_value)?;
        let current_disp = word_offset(stamp_index, "stamp value")?;
        encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::R12, CURRENTS_OFFSET);
        encoder.movsd_m64_base_disp32_xmm(Gpr::R11, current_disp, Xmm::Xmm0);

        if let Some((forward, reverse)) = current_pair {
            encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::R12, BRANCH_CURRENTS_OFFSET);
            encoder.movsd_m64_base_disp32_xmm(
                Gpr::R11,
                word_offset(*forward, "forward terminal-pair current")?,
                Xmm::Xmm0,
            );
            if forward != reverse {
                encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
                encoder.btc_r64_imm8(Gpr::R10, 63);
                encoder.movq_xmm_r64(Xmm::Xmm1, Gpr::R10);
                encoder.movsd_m64_base_disp32_xmm(
                    Gpr::R11,
                    word_offset(*reverse, "reverse terminal-pair current")?,
                    Xmm::Xmm1,
                );
            }
        }

        if let Some(stamp_jacobians) = jacobians.map(|entries| &entries[stamp_index]) {
            for entry in stamp_jacobians {
                emit_entry_call(&mut encoder, driver_image_offset, *entry)?;
                abort_branches.push(emit_abort_if_failed(&mut encoder, runtime_failed_offset));
                let jacobian_disp = word_offset(jacobian_index, "Jacobian value")?;
                encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::R14, JACOBIANS_OFFSET);
                encoder.movsd_m64_base_disp32_xmm(Gpr::R11, jacobian_disp, Xmm::Xmm0);
                jacobian_index += 1;
            }
        }

        patch_local_branch(&mut encoder, skip_stamp)?;
    }

    for abort_branch in abort_branches {
        patch_local_branch(&mut encoder, abort_branch)?;
    }
    if DRIVER_FRAME_BYTES > 0 {
        encoder.add_rsp_imm32(DRIVER_FRAME_BYTES);
    }
    encoder.pop_r64(Gpr::R14);
    encoder.pop_r64(Gpr::R13);
    encoder.pop_r64(Gpr::R12);
    encoder.ret();
    let windows_unwind =
        windows_unwind_info(windows_unwind_prologue_size, windows_unwind_operations);
    let bytes = encoder.into_bytes();
    super::verify_x64_function_code(&bytes, &format!("{driver_name} driver"))?;
    Ok(CompiledX64Function {
        bytes,
        windows_unwind,
    })
}

fn current_windows_unwind_code_offset(encoder: &X64Encoder) -> u8 {
    let offset = encoder.position();
    debug_assert!(
        u8::try_from(offset).is_ok(),
        "fixed stamp-driver prologue must fit the Windows unwind u8 offset"
    );
    offset as u8
}

fn record_windows_unwind_push(
    operations: &mut Vec<WindowsX64UnwindOperation>,
    encoder: &X64Encoder,
    register: u8,
) {
    #[cfg(windows)]
    operations.push(WindowsX64UnwindOperation::PushNonvolatile {
        code_offset: current_windows_unwind_code_offset(encoder),
        register,
    });
    #[cfg(not(windows))]
    let _ = (operations, encoder, register);
}

fn record_windows_unwind_stack_allocation(
    operations: &mut Vec<WindowsX64UnwindOperation>,
    encoder: &X64Encoder,
    size: u32,
) {
    #[cfg(windows)]
    operations.push(WindowsX64UnwindOperation::AllocateStack {
        code_offset: current_windows_unwind_code_offset(encoder),
        size,
    });
    #[cfg(not(windows))]
    let _ = (operations, encoder, size);
}

fn windows_unwind_info(
    prologue_size: u8,
    operations: Vec<WindowsX64UnwindOperation>,
) -> Option<WindowsX64UnwindInfo> {
    #[cfg(windows)]
    {
        Some(WindowsX64UnwindInfo {
            prologue_size,
            frame_register: 0,
            operations,
        })
    }
    #[cfg(not(windows))]
    {
        let _ = (prologue_size, operations);
        None
    }
}

fn emit_abort_if_failed(encoder: &mut X64Encoder, failed_offset: i32) -> usize {
    encoder.movzx_r32_m8_base_disp32(Gpr::Rax, Gpr::R12, failed_offset);
    encoder.test_r8_r8(Gpr::Rax, Gpr::Rax);
    encoder.jcc_rel32_placeholder(ConditionCode::NotEqual)
}

fn emit_branch_if_finite(
    encoder: &mut X64Encoder,
    stamp_index: usize,
    abort_branches: &mut Vec<usize>,
) -> usize {
    const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;

    encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
    encoder.movabs_r64_imm64(Gpr::R11, EXPONENT_MASK);
    encoder.and_r64_r64(Gpr::R10, Gpr::R11);
    encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
    let finite_value = encoder.jcc_rel32_placeholder(ConditionCode::NotEqual);

    encoder.mov_r64_r64(host_ctx_arg_reg(), Gpr::R12);
    encoder.movabs_r64_imm64(host_vars_arg_reg(), stamp_index as u64);
    encoder.movabs_r64_imm64(
        Gpr::R11,
        crate::native::abi::rspice_native_non_finite_contribution_error as *const () as usize
            as u64,
    );
    encoder.call_r64(Gpr::R11);
    abort_branches.push(encoder.jmp_rel32_placeholder());

    finite_value
}

fn emit_entry_call(
    encoder: &mut X64Encoder,
    driver_image_offset: usize,
    target: CodeOffset,
) -> JitResult<()> {
    encoder.mov_r64_r64(host_ctx_arg_reg(), Gpr::R12);
    encoder.mov_r64_r64(host_vars_arg_reg(), Gpr::R13);
    let displacement_offset = encoder.call_rel32_placeholder();
    let next_instruction = driver_image_offset
        .checked_add(displacement_offset)
        .and_then(|offset| offset.checked_add(std::mem::size_of::<i32>()))
        .ok_or_else(|| internal_error("stamp-kernel call address overflow"))?;
    let displacement = i64::try_from(target.as_usize())
        .ok()
        .and_then(|target| {
            i64::try_from(next_instruction)
                .ok()
                .and_then(|next| target.checked_sub(next))
        })
        .and_then(|displacement| i32::try_from(displacement).ok())
        .ok_or_else(|| {
            internal_error("stamp-kernel call target is outside the x64 rel32 address range")
        })?;
    encoder.patch_i32(displacement_offset, displacement);
    Ok(())
}

fn patch_local_branch(encoder: &mut X64Encoder, displacement_offset: usize) -> JitResult<()> {
    let target = encoder.position();
    let next_instruction = displacement_offset
        .checked_add(std::mem::size_of::<i32>())
        .ok_or_else(|| internal_error("stamp-kernel branch address overflow"))?;
    let displacement = target
        .checked_sub(next_instruction)
        .and_then(|distance| i32::try_from(distance).ok())
        .ok_or_else(|| internal_error("stamp-kernel branch target is outside rel32 range"))?;
    encoder.patch_i32(displacement_offset, displacement);
    Ok(())
}

fn byte_offset(index: usize, label: &str) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| internal_error(&format!("{label} offset exceeds i32")))
}

fn word_offset(index: usize, label: &str) -> JitResult<i32> {
    index
        .checked_mul(WORD_BYTES)
        .and_then(|offset| i32::try_from(offset).ok())
        .ok_or_else(|| internal_error(&format!("{label} offset exceeds x64 disp32 range")))
}

fn internal_error(detail: &str) -> JitError {
    JitError::InternalCompilerError {
        model: "native-x64".into(),
        detail: detail.into(),
    }
}

#[cfg(windows)]
fn host_ctx_arg_reg() -> Gpr {
    Gpr::Rcx
}

#[cfg(windows)]
fn host_vars_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn host_io_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(not(windows))]
fn host_ctx_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn host_vars_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(not(windows))]
fn host_io_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(test)]
mod tests {
    use super::{compile_evaluation_kernel, compile_stamp_kernel};
    use crate::native::model::CodeOffset;

    #[test]
    fn rejects_mismatched_entry_shapes() {
        let error = compile_stamp_kernel(
            128,
            CodeOffset::new(0),
            &[CodeOffset::new(16)],
            &[],
            &[None],
        )
        .expect_err("shape mismatch must fail");
        assert!(error.to_string().contains("entry shapes differ"));
    }

    #[test]
    fn guards_every_direct_call_with_a_runtime_failure_abort() {
        let bytes = compile_stamp_kernel(
            128,
            CodeOffset::new(0),
            &[CodeOffset::new(16), CodeOffset::new(32)],
            &[
                vec![CodeOffset::new(48), CodeOffset::new(64)],
                vec![CodeOffset::new(80)],
            ],
            &[None, None],
        )
        .expect("compile driver");

        let failure_aborts = bytes
            .windows(2)
            .filter(|window| *window == [0x0F, 0x85])
            .count();
        // Absolute helper addresses are embedded in the image and can
        // coincidentally contain this byte pair, so this is a lower bound.
        assert!(failure_aborts >= 6);
        assert_eq!(bytes.last(), Some(&0xC3));
    }

    #[test]
    fn evaluation_driver_omits_jacobian_calls_and_guards_every_entry() {
        let bytes = compile_evaluation_kernel(
            128,
            CodeOffset::new(0),
            &[CodeOffset::new(16), CodeOffset::new(32)],
            &[None, Some((0, 1))],
        )
        .expect("compile evaluation driver");

        let failure_aborts = bytes
            .windows(2)
            .filter(|window| *window == [0x0F, 0x85])
            .count();
        assert!(failure_aborts >= 3);
        assert_eq!(bytes.last(), Some(&0xC3));

        let stamp_bytes = compile_stamp_kernel(
            128,
            CodeOffset::new(0),
            &[CodeOffset::new(16), CodeOffset::new(32)],
            &[vec![CodeOffset::new(48)], vec![CodeOffset::new(64)]],
            &[None, Some((0, 1))],
        )
        .expect("compile stamp driver");
        assert!(bytes.len() < stamp_bytes.len());
    }
}
