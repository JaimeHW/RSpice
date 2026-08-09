//! Independent structural verification for RSpice's emitted A64 subset.
//!
//! This decoder does not call into `A64Encoder`. It authenticates finalized
//! little-endian instruction words and their direct control-flow targets.

use crate::native::{JitError, JitResult};

const MODEL: &str = "native-aarch64-verifier";
const NOP_BYTES: [u8; 4] = 0xD503_201F_u32.to_le_bytes();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectBranchKind {
    Jump,
    Call,
    Conditional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectBranch {
    pub(crate) instruction_offset: usize,
    pub(crate) target_offset: i64,
    pub(crate) kind: DirectBranchKind,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedA64Code {
    pub(crate) direct_branches: Vec<DirectBranch>,
    pub(crate) code_bytes: usize,
}

pub(crate) fn verify_exact_function(bytes: &[u8], entry_kind: &str) -> JitResult<VerifiedA64Code> {
    verify_exact_function_at(bytes, entry_kind, 0)
}

pub(crate) fn verify_exact_function_at(
    bytes: &[u8],
    entry_kind: &str,
    function_image_offset: usize,
) -> JitResult<VerifiedA64Code> {
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Err(verifier_error(format!(
            "compiled {entry_kind} length {} is not a nonempty sequence of A64 words",
            bytes.len()
        )));
    }

    let mut direct_branches = Vec::new();
    let mut literal_targets = Vec::new();
    let mut code_bytes = None;
    for (index, chunk) in bytes.chunks_exact(4).enumerate() {
        let offset = index * 4;
        let instruction = u32::from_le_bytes(chunk.try_into().expect("four-byte A64 word"));
        match classify(instruction) {
            InstructionClass::Ordinary => {}
            InstructionClass::Return => {
                code_bytes = Some(offset + 4);
                break;
            }
            InstructionClass::LiteralLoad { displacement } => {
                let target = i64::try_from(offset)
                    .ok()
                    .and_then(|offset| offset.checked_add(displacement))
                    .and_then(|target| usize::try_from(target).ok())
                    .ok_or_else(|| {
                        verifier_error(format!(
                            "compiled {entry_kind} literal target overflows at byte {offset}"
                        ))
                    })?;
                literal_targets.push(target);
            }
            InstructionClass::DirectBranch { displacement, kind } => {
                let target_offset = i64::try_from(offset)
                    .ok()
                    .and_then(|offset| offset.checked_add(displacement))
                    .ok_or_else(|| {
                        verifier_error(format!(
                            "compiled {entry_kind} branch target overflows at byte {offset}"
                        ))
                    })?;
                direct_branches.push(DirectBranch {
                    instruction_offset: offset,
                    target_offset,
                    kind,
                });
            }
            InstructionClass::ImagePageAddress => {
                let target_offset = authenticate_image_call(
                    bytes,
                    offset,
                    function_image_offset,
                    instruction,
                    entry_kind,
                )?;
                direct_branches.push(DirectBranch {
                    instruction_offset: offset,
                    target_offset,
                    kind: DirectBranchKind::Call,
                });
            }
            InstructionClass::Unknown => {
                return Err(verifier_error(format!(
                    "compiled {entry_kind} contains unapproved A64 word {instruction:#010x} at byte {offset}"
                )));
            }
        }
    }

    let code_bytes = code_bytes.ok_or_else(|| {
        verifier_error(format!("compiled {entry_kind} contains no RET instruction"))
    })?;
    for branch in &direct_branches {
        if branch.kind != DirectBranchKind::Call
            && (branch.target_offset < 0
                || usize::try_from(branch.target_offset)
                    .ok()
                    .is_none_or(|target| target >= code_bytes || target % 4 != 0))
        {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has a {:?} branch at byte {} to invalid local byte {}",
                branch.kind, branch.instruction_offset, branch.target_offset
            )));
        }
    }

    literal_targets.sort_unstable();
    literal_targets.dedup();
    if literal_targets.is_empty() {
        if code_bytes != bytes.len() {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has {} unauthenticated trailing byte(s)",
                bytes.len() - code_bytes
            )));
        }
    } else {
        let pool_start = code_bytes
            .checked_add(7)
            .map(|value| value / 8 * 8)
            .ok_or_else(|| verifier_error("AArch64 literal-pool alignment overflow"))?;
        if literal_targets[0] != pool_start {
            return Err(verifier_error(format!(
                "compiled {entry_kind} literal pool does not begin at aligned byte {pool_start}"
            )));
        }
        if bytes.get(code_bytes..pool_start)
            != match pool_start - code_bytes {
                0 => Some([].as_slice()),
                4 => Some(NOP_BYTES.as_slice()),
                _ => None,
            }
        {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has invalid literal-pool alignment padding"
            )));
        }
        for (index, target) in literal_targets.iter().copied().enumerate() {
            let expected = pool_start
                .checked_add(index * 8)
                .ok_or_else(|| verifier_error("AArch64 literal-pool offset overflow"))?;
            if target != expected || target + 8 > bytes.len() {
                return Err(verifier_error(format!(
                    "compiled {entry_kind} has invalid literal target byte {target}, expected {expected}"
                )));
            }
        }
        let authenticated_end = pool_start
            .checked_add(literal_targets.len() * 8)
            .ok_or_else(|| verifier_error("AArch64 literal-pool length overflow"))?;
        if authenticated_end != bytes.len() {
            return Err(verifier_error(format!(
                "compiled {entry_kind} has unauthenticated bytes after its literal pool"
            )));
        }
    }

    Ok(VerifiedA64Code {
        direct_branches,
        code_bytes,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionClass {
    Ordinary,
    Return,
    LiteralLoad {
        displacement: i64,
    },
    DirectBranch {
        displacement: i64,
        kind: DirectBranchKind,
    },
    ImagePageAddress,
    Unknown,
}

fn classify(instruction: u32) -> InstructionClass {
    if instruction & 0x9F00_0000 == 0x9000_0000 {
        return InstructionClass::ImagePageAddress;
    }
    if instruction & 0xFF00_0000 == 0x5C00_0000 {
        return InstructionClass::LiteralLoad {
            displacement: sign_extend((instruction >> 5) & 0x7_FFFF, 19) * 4,
        };
    }
    if matches!(instruction, 0xD503_201F | 0xD503_245F)
        || instruction & 0xFFE0_001F == 0xD420_0000
        || instruction & 0xFFFF_FC1F == 0xD61F_0000
        || instruction & 0xFFFF_FC1F == 0xD63F_0000
        || instruction & 0xFF80_0000 == 0xD280_0000
        || instruction & 0xFF80_0000 == 0xF280_0000
        || instruction & 0xFFE0_FC00 == 0xAA00_0000
        || instruction & 0xFFE0_0000 == 0x8B00_0000
        || instruction & 0xFFE0_0000 == 0xCB00_0000
        || instruction & 0xFFE0_FC00 == 0x8A00_0000
        || instruction & 0xFFE0_FC00 == 0xAA00_0000
        || instruction & 0xFFE0_FC00 == 0xCA00_0000
        || instruction & 0xFFE0_FC00 == 0x9AC0_2000
        || instruction & 0xFFE0_FC00 == 0x9AC0_2800
        || instruction & 0xFFFF_0FE0 == 0x9A9F_07E0
        || instruction & 0xFF00_0000 == 0x9100_0000
        || instruction & 0xFF00_0000 == 0xD100_0000
        || instruction & 0xFFC0_0000 == 0xF940_0000
        || instruction & 0xFFC0_0000 == 0xF900_0000
        || instruction & 0xFFC0_0000 == 0xFD40_0000
        || instruction & 0xFFC0_0000 == 0xFD00_0000
        || instruction & 0xFFC0_0000 == 0x3940_0000
        || instruction & 0xFFC0_0000 == 0x3900_0000
        || instruction & 0xFFC0_0000 == 0xA980_0000
        || instruction & 0xFFC0_0000 == 0xA8C0_0000
        || instruction & 0xFFFF_FC00 == 0x1E60_4000
        || instruction & 0xFFE0_1FE0 == 0x1E60_1000
        || instruction & 0xFFFF_FC00 == 0x9E67_0000
        || instruction & 0xFFFF_FC00 == 0x9E66_0000
        || instruction & 0xFFE0_FC00 == 0x1E60_2800
        || instruction & 0xFFE0_FC00 == 0x1E60_3800
        || instruction & 0xFFE0_FC00 == 0x1E60_0800
        || instruction & 0xFFE0_FC00 == 0x1E60_1800
        || instruction & 0xFFE0_FC00 == 0x1E60_4800
        || instruction & 0xFFE0_FC00 == 0x1E60_5800
        || instruction & 0xFFFF_FC00 == 0x1E61_C000
        || instruction & 0xFFFF_FC00 == 0x1E60_C000
        || instruction & 0xFFFF_FC00 == 0x1E61_4000
        || instruction & 0xFFFF_FC00 == 0x1E65_4000
        || instruction & 0xFFFF_FC00 == 0x1E64_C000
        || instruction & 0xFFFF_FC00 == 0x9E62_0000
        || instruction & 0xFFFF_FC00 == 0x9E78_0000
        || instruction & 0xFFE0_FC1F == 0xEB00_001F
        || instruction & 0xFF00_001F == 0xF100_001F
        || instruction & 0xFFE0_FC1F == 0x1E60_2000
        || instruction & 0xFFFF_FC1F == 0x1E60_2008
        || instruction & 0xFFE0_0C00 == 0x1E60_0C00
    {
        return InstructionClass::Ordinary;
    }
    if instruction & 0xFFFF_FC1F == 0xD65F_0000 {
        return InstructionClass::Return;
    }
    if instruction & 0xFC00_0000 == 0x1400_0000 {
        return InstructionClass::DirectBranch {
            displacement: sign_extend(instruction & 0x03FF_FFFF, 26) * 4,
            kind: DirectBranchKind::Jump,
        };
    }
    if instruction & 0xFC00_0000 == 0x9400_0000 {
        return InstructionClass::DirectBranch {
            displacement: sign_extend(instruction & 0x03FF_FFFF, 26) * 4,
            kind: DirectBranchKind::Call,
        };
    }
    if instruction & 0xFF00_0010 == 0x5400_0000 {
        return InstructionClass::DirectBranch {
            displacement: sign_extend((instruction >> 5) & 0x7_FFFF, 19) * 4,
            kind: DirectBranchKind::Conditional,
        };
    }
    if instruction & 0x7E00_0000 == 0x3400_0000 {
        return InstructionClass::DirectBranch {
            displacement: sign_extend((instruction >> 5) & 0x7_FFFF, 19) * 4,
            kind: DirectBranchKind::Conditional,
        };
    }
    InstructionClass::Unknown
}

fn authenticate_image_call(
    bytes: &[u8],
    instruction_offset: usize,
    function_image_offset: usize,
    adrp: u32,
    entry_kind: &str,
) -> JitResult<i64> {
    const IMAGE_CALL_REGISTER: u32 = 16;
    const ADD_X16_X16_MASKED: u32 = 0x9100_0210;
    const BLR_X16: u32 = 0xD63F_0200;
    if adrp & 0x1f != IMAGE_CALL_REGISTER {
        return Err(verifier_error(format!(
            "compiled {entry_kind} uses ADRP outside the authenticated image-call register at byte {instruction_offset}"
        )));
    }
    let add_offset = instruction_offset
        .checked_add(4)
        .ok_or_else(|| verifier_error("AArch64 image-call ADD offset overflow"))?;
    let blr_offset = instruction_offset
        .checked_add(8)
        .ok_or_else(|| verifier_error("AArch64 image-call BLR offset overflow"))?;
    let add = read_word(bytes, add_offset).ok_or_else(|| {
        verifier_error(format!(
            "compiled {entry_kind} has a truncated image call at byte {instruction_offset}"
        ))
    })?;
    let blr = read_word(bytes, blr_offset).ok_or_else(|| {
        verifier_error(format!(
            "compiled {entry_kind} has a truncated image call at byte {instruction_offset}"
        ))
    })?;
    if add & 0xFFC0_03FF != ADD_X16_X16_MASKED || blr != BLR_X16 {
        return Err(verifier_error(format!(
            "compiled {entry_kind} ADRP at byte {instruction_offset} is not the approved ADRP/ADD/BLR image-call sequence"
        )));
    }

    let immediate_low = (adrp >> 29) & 0x3;
    let immediate_high = (adrp >> 5) & 0x7_ffff;
    let page_delta = sign_extend((immediate_high << 2) | immediate_low, 21)
        .checked_mul(4096)
        .ok_or_else(|| verifier_error("AArch64 image-call page displacement overflow"))?;
    let instruction_image_offset = function_image_offset
        .checked_add(instruction_offset)
        .ok_or_else(|| verifier_error("AArch64 image-call source offset overflow"))?;
    let source_page = instruction_image_offset & !0xfff;
    let target_page = i64::try_from(source_page)
        .ok()
        .and_then(|source| source.checked_add(page_delta))
        .ok_or_else(|| verifier_error("AArch64 image-call target page overflow"))?;
    let page_offset = i64::from((add >> 10) & 0xfff);
    let target_image_offset = target_page
        .checked_add(page_offset)
        .ok_or_else(|| verifier_error("AArch64 image-call target offset overflow"))?;
    target_image_offset
        .checked_sub(i64::try_from(function_image_offset).map_err(|_| {
            verifier_error("AArch64 function image offset does not fit signed displacement")
        })?)
        .ok_or_else(|| verifier_error("AArch64 image-call relative target overflow"))
}

fn read_word(bytes: &[u8], offset: usize) -> Option<u32> {
    let chunk: [u8; 4] = bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?;
    Some(u32::from_le_bytes(chunk))
}

fn sign_extend(value: u32, bits: u8) -> i64 {
    let shift = 64 - u32::from(bits);
    ((i64::from(value)) << shift) >> shift
}

fn verifier_error(detail: impl Into<String>) -> JitError {
    JitError::Verifier {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{DirectBranchKind, verify_exact_function, verify_exact_function_at};
    use crate::native::aarch64::encoder::{A64Encoder, Condition, DReg, XReg};

    #[test]
    fn accepts_encoder_output_and_authenticates_local_targets() {
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        let skip = encoder.b_cond_placeholder(Condition::Equal);
        encoder.nop();
        let target = encoder.position();
        encoder
            .patch_branch(skip, target)
            .expect("patch local branch");
        encoder.ret();

        let verified = verify_exact_function(&encoder.into_bytes(), "test function")
            .expect("verify approved A64 function");
        assert_eq!(verified.direct_branches.len(), 2);
        assert_eq!(
            verified.direct_branches[0].kind,
            DirectBranchKind::Conditional
        );
        assert_eq!(
            verified.direct_branches[0].target_offset,
            (verified.direct_branches[0].instruction_offset + 8) as i64
        );
        assert_eq!(verified.direct_branches[1].kind, DirectBranchKind::Jump);
        assert_eq!(verified.direct_branches[1].target_offset, target as i64);
    }

    #[test]
    fn accepts_compare_nonzero_branches() {
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        let branch = encoder
            .cbnz_placeholder(XReg::X9)
            .expect("encode CBNZ placeholder");
        encoder.nop();
        let target = encoder.position();
        encoder
            .patch_branch(branch, target)
            .expect("patch CBNZ target");
        encoder.ret();

        let verified = verify_exact_function(&encoder.into_bytes(), "CBNZ function")
            .expect("verify CBNZ function");
        assert_eq!(verified.direct_branches.len(), 2);
        assert_eq!(
            verified.direct_branches[0].target_offset,
            (verified.direct_branches[0].instruction_offset + 8) as i64
        );
        assert_eq!(verified.direct_branches[1].kind, DirectBranchKind::Jump);
        assert_eq!(verified.direct_branches[1].target_offset, target as i64);
    }

    #[test]
    fn authenticates_far_image_call_target_independently() {
        let function_offset = 0x2000;
        let target_offset = 0x0ff0_4320;
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        encoder
            .image_call(function_offset, target_offset, XReg::X16)
            .expect("encode far image call");
        encoder.ret();
        let verified = verify_exact_function_at(
            &encoder.into_bytes(),
            "far image-call test",
            function_offset,
        )
        .expect("authenticate far image call");
        assert_eq!(verified.direct_branches.len(), 1);
        assert_eq!(verified.direct_branches[0].kind, DirectBranchKind::Call);
        assert_eq!(
            verified.direct_branches[0].target_offset,
            (target_offset - function_offset) as i64
        );
    }

    #[test]
    fn rejects_unknown_words_and_unaligned_images() {
        assert!(verify_exact_function(&[0, 0, 0, 0], "unknown").is_err());
        assert!(verify_exact_function(&[0xC0, 0x03, 0x5F], "short").is_err());
    }

    #[test]
    fn rejects_local_branch_outside_function() {
        let bytes = [0x02, 0x00, 0x00, 0x14, 0xC0, 0x03, 0x5F, 0xD6];
        assert!(verify_exact_function(&bytes, "out-of-range branch").is_err());
    }

    #[test]
    fn authenticates_exact_aligned_literal_pool() {
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        let literal = encoder.ldr_d_literal_placeholder(DReg::D0);
        encoder.ret();
        encoder.nop();
        let pool_start = encoder.position();
        encoder
            .patch_ldr_d_literal(literal, pool_start)
            .expect("patch literal load");
        encoder.append_u64_data(2.5_f64.to_bits());

        let verified = verify_exact_function(&encoder.into_bytes(), "literal-pool function")
            .expect("authenticate exact literal pool");
        assert_eq!(verified.code_bytes, 12);
    }

    #[test]
    fn rejects_literal_targets_in_code_and_unreferenced_pool_bytes() {
        let mut into_code = A64Encoder::new();
        into_code.bti_c();
        let literal = into_code.ldr_d_literal_placeholder(DReg::D0);
        into_code.ret();
        into_code.nop();
        into_code
            .patch_ldr_d_literal(literal, 0)
            .expect("encode in-range malformed target");
        into_code.append_u64_data(1.0_f64.to_bits());
        assert!(verify_exact_function(&into_code.into_bytes(), "literal into code").is_err());

        let mut trailing = A64Encoder::new();
        trailing.bti_c();
        let literal = trailing.ldr_d_literal_placeholder(DReg::D0);
        trailing.ret();
        trailing.nop();
        let pool_start = trailing.position();
        trailing
            .patch_ldr_d_literal(literal, pool_start)
            .expect("patch valid literal load");
        trailing.append_u64_data(1.0_f64.to_bits());
        trailing.append_u64_data(2.0_f64.to_bits());
        assert!(
            verify_exact_function(&trailing.into_bytes(), "unreferenced literal bytes").is_err()
        );
    }
}
