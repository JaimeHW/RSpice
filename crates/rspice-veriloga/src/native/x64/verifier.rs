//! Independent structural decoder for the x64 subset emitted by this backend.
//!
//! This deliberately does not reuse `X64Encoder`: accepting an instruction is
//! based on decoding the finished byte stream, so an encoder length/opcode bug
//! cannot validate itself through shared bookkeeping.

use crate::native::{JitError, JitResult};

#[cfg(windows)]
use super::{WindowsX64UnwindInfo, WindowsX64UnwindOperation};

#[derive(Debug)]
pub(super) struct VerifiedX64Code {
    pub(super) code_len: usize,
    pub(super) direct_call_targets: Vec<i64>,
    pub(super) rip_relative_references: Vec<VerifiedRipRelativeReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VerifiedRipRelativeReference {
    pub(super) displacement_offset: usize,
    pub(super) target_offset: i64,
}

#[derive(Debug, Clone, Copy)]
enum ControlFlow {
    None,
    Return,
    Jump { displacement_offset: usize },
    Call { displacement_offset: usize },
}

#[derive(Debug)]
struct DecodedInstruction {
    end: usize,
    control_flow: ControlFlow,
    rip_displacement_offset: Option<usize>,
}

pub(super) fn verify_exact_function(bytes: &[u8], entry_kind: &str) -> JitResult<VerifiedX64Code> {
    let verified = decode_through_return(bytes, entry_kind)?;
    if verified.code_len != bytes.len() {
        return Err(verifier_error(format!(
            "compiled {entry_kind} has {} trailing byte(s) after its terminal RET",
            bytes.len() - verified.code_len
        )));
    }
    Ok(verified)
}

pub(super) fn verify_function_prefix(bytes: &[u8], entry_kind: &str) -> JitResult<VerifiedX64Code> {
    decode_through_return(bytes, entry_kind)
}

/// Independently authenticate Windows unwind operations against the finalized
/// prologue bytes. Metadata is recorded while encoding, so checking only its
/// internal ranges would let an encoder/recorder drift validate itself.
#[cfg(windows)]
pub(super) fn verify_windows_unwind_prologue(
    code: &[u8],
    info: &WindowsX64UnwindInfo,
    entry_kind: &str,
) -> JitResult<()> {
    let prologue_end = usize::from(info.prologue_size);
    if prologue_end == 0 || prologue_end > code.len() {
        return Err(verifier_error(format!(
            "compiled {entry_kind} declares Windows prologue length {prologue_end} for {} code bytes",
            code.len()
        )));
    }

    let mut previous_offset = 0_u8;
    let mut has_frame_pointer = false;
    let mut allocation_size = None;
    for operation in &info.operations {
        let (code_offset, expected) = match *operation {
            WindowsX64UnwindOperation::PushNonvolatile {
                code_offset,
                register,
            } => {
                if !matches!(register, 5 | 12 | 13 | 14) {
                    return Err(verifier_error(format!(
                        "compiled {entry_kind} unwind metadata pushes volatile/unsupported GPR {register}"
                    )));
                }
                let mut bytes = Vec::with_capacity(2);
                if register >= 8 {
                    bytes.push(0x41);
                }
                bytes.push(0x50 + (register & 7));
                (code_offset, bytes)
            }
            WindowsX64UnwindOperation::AllocateStack { code_offset, size } => {
                let size = i32::try_from(size).map_err(|_| {
                    verifier_error(format!(
                        "compiled {entry_kind} unwind stack allocation exceeds i32"
                    ))
                })?;
                if size <= 0 || size % 16 != 0 || allocation_size.replace(size as u32).is_some() {
                    return Err(verifier_error(format!(
                        "compiled {entry_kind} has invalid/duplicate unwind stack allocation {size}"
                    )));
                }
                let mut bytes = vec![0x48];
                if i8::try_from(size).is_ok() {
                    bytes.extend_from_slice(&[0x83, 0xEC, size as i8 as u8]);
                } else {
                    bytes.extend_from_slice(&[0x81, 0xEC]);
                    bytes.extend_from_slice(&size.to_le_bytes());
                }
                (code_offset, bytes)
            }
            WindowsX64UnwindOperation::SaveXmm128 {
                code_offset,
                register,
                stack_offset,
            } => {
                let allocation = allocation_size.ok_or_else(|| {
                    verifier_error(format!(
                        "compiled {entry_kind} saves XMM{register} before declaring its stack allocation"
                    ))
                })?;
                if !(6..=15).contains(&register)
                    || stack_offset % 16 != 0
                    || stack_offset
                        .checked_add(16)
                        .is_none_or(|end| end > allocation)
                {
                    return Err(verifier_error(format!(
                        "compiled {entry_kind} has invalid XMM{register} unwind save at stack byte {stack_offset}"
                    )));
                }
                let mut bytes = vec![0xF3];
                if register >= 8 {
                    bytes.push(0x44);
                }
                bytes.extend_from_slice(&[0x0F, 0x7F]);
                append_rsp_memory_operand(&mut bytes, register & 7, stack_offset as i32);
                (code_offset, bytes)
            }
            WindowsX64UnwindOperation::SetFramePointer { code_offset } => {
                if has_frame_pointer {
                    return Err(verifier_error(format!(
                        "compiled {entry_kind} declares the frame pointer twice"
                    )));
                }
                has_frame_pointer = true;
                (code_offset, vec![0x48, 0x89, 0xE5])
            }
        };

        if code_offset <= previous_offset || code_offset > info.prologue_size {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has non-monotonic/out-of-range unwind code offset {code_offset}"
            )));
        }
        previous_offset = code_offset;
        let end = usize::from(code_offset);
        let start = end.checked_sub(expected.len()).ok_or_else(|| {
            verifier_error(format!(
                "compiled {entry_kind} unwind operation ending at {end} starts before the function"
            ))
        })?;
        if code.get(start..end) != Some(expected.as_slice()) {
            return Err(verifier_error(format!(
                "compiled {entry_kind} unwind operation ending at byte {end} does not match finalized prologue bytes"
            )));
        }
    }

    let expected_frame_register = if has_frame_pointer { 5 } else { 0 };
    if info.frame_register != expected_frame_register {
        return Err(verifier_error(format!(
            "compiled {entry_kind} declares frame register {}, expected {expected_frame_register}",
            info.frame_register
        )));
    }
    Ok(())
}

#[cfg(windows)]
fn append_rsp_memory_operand(bytes: &mut Vec<u8>, register: u8, displacement: i32) {
    let mode = if displacement == 0 {
        0b00
    } else if i8::try_from(displacement).is_ok() {
        0b01
    } else {
        0b10
    };
    bytes.push((mode << 6) | ((register & 7) << 3) | 0b100);
    bytes.push(0x24);
    match mode {
        0b00 => {}
        0b01 => bytes.push(displacement as i8 as u8),
        0b10 => bytes.extend_from_slice(&displacement.to_le_bytes()),
        _ => unreachable!(),
    }
}

fn decode_through_return(bytes: &[u8], entry_kind: &str) -> JitResult<VerifiedX64Code> {
    if bytes.is_empty() {
        return Err(verifier_error(format!(
            "compiled {entry_kind} has an empty code range"
        )));
    }

    let mut instruction_starts = Vec::new();
    let mut local_branch_targets = Vec::new();
    let mut direct_call_targets = Vec::new();
    let mut rip_relative_references = Vec::new();
    let mut offset = 0;
    let code_len = loop {
        instruction_starts.push(offset);
        let instruction = decode_instruction(bytes, offset).map_err(|detail| {
            verifier_error(format!(
                "compiled {entry_kind} contains invalid x64 at byte {offset}: {detail}"
            ))
        })?;
        let next = instruction.end;

        if let Some(displacement_offset) = instruction.rip_displacement_offset {
            let displacement = read_i32(bytes, displacement_offset).map_err(verifier_error)?;
            rip_relative_references.push(VerifiedRipRelativeReference {
                displacement_offset,
                target_offset: relative_target(next, displacement),
            });
        }

        match instruction.control_flow {
            ControlFlow::None => {}
            ControlFlow::Return => break next,
            ControlFlow::Jump {
                displacement_offset,
            } => {
                let displacement = read_i32(bytes, displacement_offset).map_err(verifier_error)?;
                local_branch_targets.push(relative_target(next, displacement));
            }
            ControlFlow::Call {
                displacement_offset,
            } => {
                let displacement = read_i32(bytes, displacement_offset).map_err(verifier_error)?;
                direct_call_targets.push(relative_target(next, displacement));
            }
        }
        offset = next;
        if offset == bytes.len() {
            return Err(verifier_error(format!(
                "compiled {entry_kind} reaches the end of its code range without RET"
            )));
        }
    };

    for target in local_branch_targets {
        let target = usize::try_from(target)
            .ok()
            .filter(|target| *target < code_len);
        if target.is_none_or(|target| instruction_starts.binary_search(&target).is_err()) {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has a branch to non-instruction byte {target:?}"
            )));
        }
    }
    for target in &direct_call_targets {
        if let Ok(local_target) = usize::try_from(*target)
            && local_target < code_len
            && instruction_starts.binary_search(&local_target).is_err()
        {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has a call to non-instruction byte {local_target}"
            )));
        }
    }

    Ok(VerifiedX64Code {
        code_len,
        direct_call_targets,
        rip_relative_references,
    })
}

fn decode_instruction(bytes: &[u8], start: usize) -> Result<DecodedInstruction, String> {
    let mut cursor = start;
    let mut legacy_prefix = None;
    if matches!(bytes.get(cursor), Some(0x66 | 0xF2 | 0xF3)) {
        legacy_prefix = bytes.get(cursor).copied();
        cursor += 1;
        if matches!(bytes.get(cursor), Some(0x66 | 0xF2 | 0xF3)) {
            return Err("multiple mandatory prefixes are outside the emitted subset".into());
        }
    }

    let mut rex_w = false;
    if bytes
        .get(cursor)
        .is_some_and(|byte| matches!(*byte, 0x40..=0x4F))
    {
        rex_w = bytes[cursor] & 0x08 != 0;
        cursor += 1;
    }

    let opcode = take_u8(bytes, &mut cursor, "opcode")?;
    let mut control_flow = ControlFlow::None;
    let mut rip_displacement_offset = None;
    match opcode {
        0x01 | 0x08 | 0x09 | 0x20 | 0x21 | 0x29 | 0x31 | 0x39 | 0x84 | 0x85 | 0x89 | 0x8B
        | 0x8D => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            rip_displacement_offset = decode_modrm(bytes, &mut cursor)?;
        }
        0x50..=0x5F => reject_legacy_prefix(legacy_prefix, opcode)?,
        0x81 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if !matches!(extension, 0 | 1 | 4 | 5 | 6 | 7) {
                return Err(format!("unsupported 81 /{extension} instruction"));
            }
            take(bytes, &mut cursor, 4, "imm32")?;
        }
        0x83 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if !matches!(extension, 0 | 1 | 4 | 5 | 6 | 7) {
                return Err(format!("unsupported 83 /{extension} instruction"));
            }
            take(bytes, &mut cursor, 1, "imm8")?;
        }
        0xC1 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if !matches!(extension, 4 | 7) {
                return Err(format!("unsupported C1 /{extension} instruction"));
            }
            take(bytes, &mut cursor, 1, "shift imm8")?;
        }
        0xC3 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            control_flow = ControlFlow::Return;
        }
        0xC6 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if extension != 0 {
                return Err(format!("unsupported C6 /{extension} instruction"));
            }
            take(bytes, &mut cursor, 1, "imm8")?;
        }
        0xC7 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if extension != 0 {
                return Err(format!("unsupported C7 /{extension} instruction"));
            }
            take(bytes, &mut cursor, 4, "imm32")?;
        }
        0xD3 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if !matches!(extension, 4 | 7) {
                return Err(format!("unsupported D3 /{extension} instruction"));
            }
        }
        0xE8 | 0xE9 => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let displacement_offset = cursor;
            take(bytes, &mut cursor, 4, "rel32")?;
            control_flow = if opcode == 0xE8 {
                ControlFlow::Call {
                    displacement_offset,
                }
            } else {
                ControlFlow::Jump {
                    displacement_offset,
                }
            };
        }
        0xFF => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            let extension =
                decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
            if extension != 2 {
                return Err(format!("unsupported FF /{extension} instruction"));
            }
        }
        0xB8..=0xBF => {
            reject_legacy_prefix(legacy_prefix, opcode)?;
            take(
                bytes,
                &mut cursor,
                if rex_w { 8 } else { 4 },
                if rex_w { "imm64" } else { "imm32" },
            )?;
        }
        0x0F => {
            let secondary = take_u8(bytes, &mut cursor, "two-byte opcode")?;
            match secondary {
                0x80..=0x8F => {
                    if legacy_prefix.is_some() {
                        return Err("conditional branch has an unexpected mandatory prefix".into());
                    }
                    let displacement_offset = cursor;
                    take(bytes, &mut cursor, 4, "conditional rel32")?;
                    control_flow = ControlFlow::Jump {
                        displacement_offset,
                    };
                }
                0x90..=0x9F => {
                    if legacy_prefix.is_some() {
                        return Err("SETcc has an unexpected mandatory prefix".into());
                    }
                    rip_displacement_offset = decode_modrm(bytes, &mut cursor)?;
                }
                0x10 | 0x11 | 0x2A | 0x2C | 0x2E | 0x45 | 0x4A | 0x51 | 0x54 | 0x57 | 0x58
                | 0x59 | 0x5C | 0x5D | 0x5E | 0x5F | 0x6E | 0x6F | 0x7E | 0x7F | 0xB6 => {
                    validate_two_byte_prefix(legacy_prefix, secondary)?;
                    rip_displacement_offset = decode_modrm(bytes, &mut cursor)?;
                }
                0xBA => {
                    if legacy_prefix.is_some() {
                        return Err(
                            "bit-test instruction has an unexpected mandatory prefix".into()
                        );
                    }
                    let extension =
                        decode_modrm_extension(bytes, &mut cursor, &mut rip_displacement_offset)?;
                    if !matches!(extension, 6 | 7) {
                        return Err(format!("unsupported 0F BA /{extension} instruction"));
                    }
                    take(bytes, &mut cursor, 1, "bit index")?;
                }
                _ => return Err(format!("unsupported two-byte opcode 0F {secondary:02X}")),
            }
        }
        _ => return Err(format!("unsupported opcode {opcode:02X}")),
    }

    let length = cursor - start;
    if length > 15 {
        return Err(format!(
            "instruction length {length} exceeds the architectural maximum"
        ));
    }
    Ok(DecodedInstruction {
        end: cursor,
        control_flow,
        rip_displacement_offset,
    })
}

fn decode_modrm_extension(
    bytes: &[u8],
    cursor: &mut usize,
    rip_displacement_offset: &mut Option<usize>,
) -> Result<u8, String> {
    let modrm = *bytes
        .get(*cursor)
        .ok_or_else(|| "truncated ModRM byte".to_string())?;
    let extension = (modrm >> 3) & 0b111;
    *rip_displacement_offset = decode_modrm(bytes, cursor)?;
    Ok(extension)
}

fn decode_modrm(bytes: &[u8], cursor: &mut usize) -> Result<Option<usize>, String> {
    let modrm = take_u8(bytes, cursor, "ModRM")?;
    let mode = modrm >> 6;
    let rm = modrm & 0b111;
    if mode == 0b11 {
        return Ok(None);
    }

    let mut displacement_bytes = match mode {
        0b00 => usize::from(rm == 0b101) * 4,
        0b01 => 1,
        0b10 => 4,
        _ => unreachable!(),
    };
    let rip_relative = mode == 0b00 && rm == 0b101;
    if rm == 0b100 {
        let sib = take_u8(bytes, cursor, "SIB")?;
        if mode == 0b00 && sib & 0b111 == 0b101 {
            displacement_bytes = 4;
        }
    }
    let displacement_offset = *cursor;
    take(bytes, cursor, displacement_bytes, "memory displacement")?;
    Ok(rip_relative.then_some(displacement_offset))
}

fn validate_two_byte_prefix(prefix: Option<u8>, opcode: u8) -> Result<(), String> {
    let valid = match opcode {
        0x45 | 0x4A | 0xB6 => prefix.is_none(),
        0x6F | 0x7F => prefix == Some(0xF3),
        0x10 | 0x11 | 0x2A | 0x2C | 0x51 | 0x58 | 0x59 | 0x5C | 0x5D | 0x5E | 0x5F => {
            prefix == Some(0xF2)
        }
        0x2E | 0x54 | 0x57 | 0x6E | 0x7E => prefix == Some(0x66),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(format!(
            "opcode 0F {opcode:02X} has unsupported mandatory prefix {prefix:?}"
        ))
    }
}

fn reject_legacy_prefix(prefix: Option<u8>, opcode: u8) -> Result<(), String> {
    if prefix.is_some() {
        Err(format!(
            "opcode {opcode:02X} has unexpected mandatory prefix {prefix:?}"
        ))
    } else {
        Ok(())
    }
}

fn relative_target(next_instruction: usize, displacement: i32) -> i64 {
    next_instruction as i64 + i64::from(displacement)
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, String> {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("truncated rel32/disp32 at byte {offset}"))?
        .try_into()
        .map_err(|_| format!("invalid rel32/disp32 at byte {offset}"))?;
    Ok(i32::from_le_bytes(raw))
}

fn take_u8(bytes: &[u8], cursor: &mut usize, what: &str) -> Result<u8, String> {
    let byte = *bytes
        .get(*cursor)
        .ok_or_else(|| format!("truncated {what} at byte {cursor}"))?;
    *cursor += 1;
    Ok(byte)
}

fn take(bytes: &[u8], cursor: &mut usize, count: usize, what: &str) -> Result<(), String> {
    let end = cursor
        .checked_add(count)
        .ok_or_else(|| format!("{what} range overflow at byte {cursor}"))?;
    if end > bytes.len() {
        return Err(format!("truncated {what} at byte {cursor}"));
    }
    *cursor = end;
    Ok(())
}

fn verifier_error(detail: impl Into<String>) -> JitError {
    JitError::Verifier {
        model: "native-x64".into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{verify_exact_function, verify_function_prefix};

    #[test]
    fn rejects_branch_into_the_middle_of_an_instruction() {
        let error = verify_exact_function(
            &[0xE9, 0x01, 0x00, 0x00, 0x00, 0xB8, 0, 0, 0, 0, 0xC3],
            "bad branch",
        )
        .expect_err("middle-of-instruction branch must fail");
        assert!(error.to_string().contains("non-instruction"));
    }

    #[test]
    fn separates_terminal_code_from_following_data() {
        let verified =
            verify_function_prefix(&[0xB8, 1, 0, 0, 0, 0xC3, 0xFF, 0xFF], "code with data")
                .expect("valid function prefix");
        assert_eq!(verified.code_len, 6);
    }

    #[test]
    fn rejects_unknown_or_truncated_encodings() {
        assert!(verify_exact_function(&[0x0F, 0x0B, 0xC3], "unknown").is_err());
        assert!(verify_exact_function(&[0xE9, 0, 0, 0], "truncated").is_err());
    }
}
