//! Whole-pass native entry-point drivers.
//!
//! Scalar expression entry points remain useful for diagnostics and narrowly
//! scoped evaluation APIs. The hot stamping path instead enters one driver,
//! which invokes the assignment, contribution, and Jacobian entries inside
//! the executable image and writes into caller-owned, preallocated buffers.

use super::encoder::{ConditionCode, Gpr, X64Encoder, Xmm};
use crate::native::model::{CodeOffset, NativeStampKernelIo};
use crate::native::{EvalContext, JitError, JitResult};

const WORD_BYTES: usize = std::mem::size_of::<f64>();
const BRANCH_CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, branch_currents) as i32;
const CURRENTS_OFFSET: i32 = std::mem::offset_of!(EvalContext, currents) as i32;
const ACTIVE_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, program_active) as i32;
const JACOBIANS_OFFSET: i32 = std::mem::offset_of!(NativeStampKernelIo, jacobians) as i32;

#[cfg(windows)]
const DRIVER_FRAME_BYTES: i32 = 40;
#[cfg(windows)]
const IO_STACK_OFFSET: i32 = 32;
#[cfg(not(windows))]
const DRIVER_FRAME_BYTES: i32 = 8;
#[cfg(not(windows))]
const IO_STACK_OFFSET: i32 = 0;

/// Compiles a driver whose address will be `driver_image_offset` in the final
/// executable image.
pub(crate) fn compile_stamp_kernel(
    driver_image_offset: usize,
    assignment: CodeOffset,
    stamp_values: &[CodeOffset],
    jacobians: &[Vec<CodeOffset>],
    published_current_pairs: &[Option<(usize, usize)>],
) -> JitResult<Vec<u8>> {
    if stamp_values.len() != jacobians.len() || stamp_values.len() != published_current_pairs.len()
    {
        return Err(internal_error(
            "stamp-kernel value and Jacobian entry shapes differ",
        ));
    }

    let mut encoder = X64Encoder::new();
    encoder.push_r64(Gpr::R12);
    encoder.push_r64(Gpr::R13);
    encoder.mov_r64_r64(Gpr::R12, host_ctx_arg_reg());
    encoder.mov_r64_r64(Gpr::R13, host_vars_arg_reg());
    encoder.sub_rsp_imm32(DRIVER_FRAME_BYTES);
    encoder.mov_m64_base_disp32_r64(Gpr::Rsp, IO_STACK_OFFSET, host_io_arg_reg());

    emit_entry_call(&mut encoder, driver_image_offset, assignment)?;

    let mut jacobian_index = 0usize;
    for (stamp_index, ((value_entry, stamp_jacobians), current_pair)) in stamp_values
        .iter()
        .zip(jacobians)
        .zip(published_current_pairs)
        .enumerate()
    {
        let active_disp = byte_offset(stamp_index, "stamp active-mask")?;
        encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::Rsp, IO_STACK_OFFSET);
        encoder.mov_r64_m64_base_disp32(Gpr::R10, Gpr::R11, ACTIVE_OFFSET);
        encoder.movzx_r32_m8_base_disp32(Gpr::Rax, Gpr::R10, active_disp);
        encoder.test_r8_r8(Gpr::Rax, Gpr::Rax);
        let skip_stamp = encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        emit_entry_call(&mut encoder, driver_image_offset, *value_entry)?;
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

        for entry in stamp_jacobians {
            emit_entry_call(&mut encoder, driver_image_offset, *entry)?;
            let jacobian_disp = word_offset(jacobian_index, "Jacobian value")?;
            encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::Rsp, IO_STACK_OFFSET);
            encoder.mov_r64_m64_base_disp32(Gpr::R11, Gpr::R11, JACOBIANS_OFFSET);
            encoder.movsd_m64_base_disp32_xmm(Gpr::R11, jacobian_disp, Xmm::Xmm0);
            jacobian_index += 1;
        }

        patch_local_branch(&mut encoder, skip_stamp)?;
    }

    encoder.add_rsp_imm32(DRIVER_FRAME_BYTES);
    encoder.pop_r64(Gpr::R13);
    encoder.pop_r64(Gpr::R12);
    encoder.ret();
    Ok(encoder.into_bytes())
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
    use super::compile_stamp_kernel;
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
    fn emits_one_direct_call_per_planned_entry() {
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

        assert_eq!(bytes.iter().filter(|byte| **byte == 0xE8).count(), 6);
        assert_eq!(bytes.last(), Some(&0xC3));
    }
}
