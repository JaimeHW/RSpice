//! Unwind metadata derived independently from emitted A64 functions.
//!
//! Code generation does not declare its own frame shape. This module decodes
//! the fixed prologue and epilogue instructions after machine verification so
//! unwind data cannot silently drift away from the bytes that execute.

use crate::native::model::CodeOffset;
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-aarch64-unwind";

const BTI_C: u32 = 0xD503_245F;
const RET_LR: u32 = 0xD65F_03C0;
const STP_X19_X20_PRE_16: u32 = 0xA9BF_53F3;
const STP_X21_X22_PRE_16: u32 = 0xA9BF_5BF5;
const STP_X29_X30_PRE_16: u32 = 0xA9BF_7BFD;
const ADD_X29_SP_0: u32 = 0x9100_03FD;
const LDP_X29_X30_POST_16: u32 = 0xA8C1_7BFD;
const LDP_X21_X22_POST_16: u32 = 0xA8C1_5BF5;
const LDP_X19_X20_POST_16: u32 = 0xA8C1_53F3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum A64FrameKind {
    Leaf,
    Frame {
        saves_kernel_io: bool,
        epilogue_start: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct A64UnwindFunction {
    pub(super) start: CodeOffset,
    pub(super) end: CodeOffset,
    pub(super) frame: A64FrameKind,
}

impl A64UnwindFunction {
    pub(crate) fn len(self) -> usize {
        self.end.as_usize() - self.start.as_usize()
    }
}

pub(crate) fn analyze_function(
    start: CodeOffset,
    bytes: &[u8],
    entry_kind: &str,
) -> JitResult<A64UnwindFunction> {
    if bytes.len() < 8 || bytes.len() % 4 != 0 {
        return Err(unwind_error(format!(
            "{entry_kind} has invalid A64 function length {}",
            bytes.len()
        )));
    }
    if word(bytes, 0)? != BTI_C || word(bytes, bytes.len() - 4)? != RET_LR {
        return Err(unwind_error(format!(
            "{entry_kind} does not have the authenticated BTI/RET boundary"
        )));
    }

    let frame = if word(bytes, 4)? != STP_X19_X20_PRE_16 {
        A64FrameKind::Leaf
    } else {
        let saves_kernel_io = word(bytes, 8)? == STP_X21_X22_PRE_16;
        let frame_pointer_save = if saves_kernel_io { 12 } else { 8 };
        if word(bytes, frame_pointer_save)? != STP_X29_X30_PRE_16
            || word(bytes, frame_pointer_save + 4)? != ADD_X29_SP_0
        {
            return Err(unwind_error(format!(
                "{entry_kind} has an unrecognized nonvolatile A64 prologue"
            )));
        }

        let mut cursor = bytes.len() - 8;
        if word(bytes, cursor)? != LDP_X19_X20_POST_16 {
            return Err(unwind_error(format!(
                "{entry_kind} does not restore x19/x20 before RET"
            )));
        }
        if saves_kernel_io {
            cursor = cursor
                .checked_sub(4)
                .ok_or_else(|| unwind_error("A64 epilogue offset underflow"))?;
            if word(bytes, cursor)? != LDP_X21_X22_POST_16 {
                return Err(unwind_error(format!(
                    "{entry_kind} does not restore x21/x22 before x19/x20"
                )));
            }
        }
        cursor = cursor
            .checked_sub(4)
            .ok_or_else(|| unwind_error("A64 epilogue offset underflow"))?;
        if word(bytes, cursor)? != LDP_X29_X30_POST_16 {
            return Err(unwind_error(format!(
                "{entry_kind} does not restore x29/x30 at the epilogue boundary"
            )));
        }
        A64FrameKind::Frame {
            saves_kernel_io,
            epilogue_start: cursor,
        }
    };

    let end = start
        .as_usize()
        .checked_add(bytes.len())
        .map(CodeOffset::new)
        .ok_or_else(|| unwind_error("A64 unwind function range overflow"))?;
    Ok(A64UnwindFunction { start, end, frame })
}

#[cfg(unix)]
pub(crate) struct A64EhFrame {
    pub(crate) words: Box<[u64]>,
    #[cfg(target_os = "macos")]
    pub(crate) fde_offsets: Vec<usize>,
}

#[cfg(unix)]
pub(crate) fn encode_eh_frame(
    code_base: *const u8,
    functions: &[A64UnwindFunction],
) -> JitResult<A64EhFrame> {
    if code_base.is_null() || functions.is_empty() {
        return Err(unwind_error(
            "DWARF frame registration requires code and at least one function",
        ));
    }

    let mut bytes = Vec::new();
    let cie_start = bytes.len();
    let cie_length = reserve_u32(&mut bytes);
    push_u32(&mut bytes, 0);
    bytes.push(1); // CIE version
    bytes.extend_from_slice(b"zR\0");
    push_uleb(&mut bytes, 1); // code alignment
    push_sleb(&mut bytes, -8); // data alignment
    push_uleb(&mut bytes, 30); // return-address register (x30)
    push_uleb(&mut bytes, 1); // augmentation length
    bytes.push(0); // DW_EH_PE_absptr
    bytes.extend_from_slice(&[0x0c, 31, 0]); // DW_CFA_def_cfa sp, 0
    bytes.extend_from_slice(&[0x08, 30]); // DW_CFA_same_value x30
    align(&mut bytes, 8);
    fill_length(&mut bytes, cie_length)?;

    #[cfg(target_os = "macos")]
    let mut fde_offsets = Vec::with_capacity(functions.len());
    for function in functions {
        #[cfg(target_os = "macos")]
        fde_offsets.push(bytes.len());
        let fde_length = reserve_u32(&mut bytes);
        let cie_pointer_field = bytes.len();
        let cie_delta = cie_pointer_field
            .checked_sub(cie_start)
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(|| unwind_error("DWARF CIE pointer exceeds u32"))?;
        push_u32(&mut bytes, cie_delta);
        let address = unsafe { code_base.add(function.start.as_usize()) } as usize;
        push_u64(&mut bytes, address as u64);
        push_u64(&mut bytes, function.len() as u64);
        push_uleb(&mut bytes, 0); // FDE augmentation length
        encode_dwarf_cfi(&mut bytes, *function)?;
        align(&mut bytes, 8);
        fill_length(&mut bytes, fde_length)?;
    }
    push_u32(&mut bytes, 0); // end-of-section terminator
    align(&mut bytes, 8);

    let mut words = vec![0_u64; bytes.len() / 8];
    let word_bytes =
        unsafe { std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), bytes.len()) };
    word_bytes.copy_from_slice(&bytes);
    Ok(A64EhFrame {
        words: words.into_boxed_slice(),
        #[cfg(target_os = "macos")]
        fde_offsets,
    })
}

#[cfg(unix)]
fn encode_dwarf_cfi(bytes: &mut Vec<u8>, function: A64UnwindFunction) -> JitResult<()> {
    let A64FrameKind::Frame {
        saves_kernel_io,
        epilogue_start,
    } = function.frame
    else {
        return Ok(());
    };

    let saved_bytes = if saves_kernel_io { 48_u64 } else { 32_u64 };
    let mut location = 0_usize;
    advance(bytes, &mut location, 8)?; // BTI + save x19/x20
    def_cfa_offset(bytes, 16);
    offset(bytes, 19, 2);
    offset(bytes, 20, 1);
    if saves_kernel_io {
        advance(bytes, &mut location, 12)?;
        def_cfa_offset(bytes, 32);
        offset(bytes, 21, 4);
        offset(bytes, 22, 3);
    }
    advance(bytes, &mut location, if saves_kernel_io { 16 } else { 12 })?;
    def_cfa_offset(bytes, saved_bytes);
    offset(bytes, 29, saved_bytes / 8);
    offset(bytes, 30, saved_bytes / 8 - 1);
    advance(bytes, &mut location, if saves_kernel_io { 20 } else { 16 })?;
    bytes.extend_from_slice(&[0x0d, 29]); // DW_CFA_def_cfa_register x29

    advance(bytes, &mut location, epilogue_start + 4)?;
    bytes.extend_from_slice(&[0x0d, 31]); // CFA register becomes sp
    def_cfa_offset(bytes, saved_bytes - 16);
    restore(bytes, 29);
    restore(bytes, 30);
    if saves_kernel_io {
        advance(bytes, &mut location, epilogue_start + 8)?;
        def_cfa_offset(bytes, 16);
        restore(bytes, 21);
        restore(bytes, 22);
        advance(bytes, &mut location, epilogue_start + 12)?;
    } else {
        advance(bytes, &mut location, epilogue_start + 8)?;
    }
    def_cfa_offset(bytes, 0);
    restore(bytes, 19);
    restore(bytes, 20);
    Ok(())
}

#[cfg(any(windows, test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WindowsA64RuntimeFunction {
    pub(crate) begin_address: u32,
    pub(crate) unwind_data: u32,
}

#[cfg(any(windows, test))]
pub(crate) fn append_windows_unwind_data(
    image: &mut Vec<u8>,
    functions: &[A64UnwindFunction],
) -> JitResult<Vec<WindowsA64RuntimeFunction>> {
    let code_len = image.len();
    let mut table = Vec::new();
    let mut previous_end = 0_usize;
    for function in functions {
        let start = function.start.as_usize();
        let end = function.end.as_usize();
        if start >= end || end > code_len || start < previous_end {
            return Err(unwind_error(
                "Windows ARM64 functions must be ordered, nonoverlapping code-image ranges",
            ));
        }
        previous_end = end;
        let A64FrameKind::Frame {
            saves_kernel_io,
            epilogue_start,
        } = function.frame
        else {
            continue;
        };
        align(image, 4);
        let unwind_data = u32::try_from(image.len())
            .map_err(|_| unwind_error("Windows ARM64 xdata RVA exceeds u32"))?;
        let xdata = encode_windows_xdata(function.len(), epilogue_start, saves_kernel_io)?;
        image.extend_from_slice(&xdata);
        table.push(WindowsA64RuntimeFunction {
            begin_address: u32::try_from(function.start.as_usize())
                .map_err(|_| unwind_error("Windows ARM64 function RVA exceeds u32"))?,
            unwind_data,
        });
    }
    Ok(table)
}

#[cfg(any(windows, test))]
fn encode_windows_xdata(
    function_len: usize,
    epilogue_start: usize,
    saves_kernel_io: bool,
) -> JitResult<Vec<u8>> {
    if function_len % 4 != 0 || epilogue_start % 4 != 0 {
        return Err(unwind_error(
            "Windows ARM64 function offsets are not word aligned",
        ));
    }
    let function_words = function_len / 4;
    if function_words > 0x3ffff {
        return Err(unwind_error(
            "Windows ARM64 function exceeds the 1 MiB full-xdata limit",
        ));
    }
    if epilogue_start >= function_len || epilogue_start / 4 > 0x3ffff {
        return Err(unwind_error(
            "Windows ARM64 epilogue scope is outside the function",
        ));
    }

    // The epilogue is the prologue-code suffix beginning after `set_fp`:
    // save_fplr_x, optional save_regp_x x21/x22, save_regp_x x19/x20,
    // nop (BTI in the prologue, RET in the epilogue), end. Sharing that
    // suffix is the canonical encoding emitted by LLVM's Windows AArch64
    // assembler for these exact frames.
    let mut codes = vec![0xe1, 0x81]; // set_fp; save_fplr_x -16
    if saves_kernel_io {
        codes.extend_from_slice(&[0xcc, 0x81]); // save_regp_x x21/x22, -16
    }
    codes.extend_from_slice(&[0x22, 0xe3, 0xe4]); // save x19/x20; BTI/RET nop; end

    let epilogue_index = 1_usize;
    let code_words = codes.len().div_ceil(4);
    if code_words > 31 || epilogue_index > 0x3ff {
        return Err(unwind_error(
            "Windows ARM64 unwind-code pool exceeds compact header fields",
        ));
    }
    codes.resize(code_words * 4, 0xe3);

    let header = u32::try_from(function_words).expect("18-bit function length")
        | (1_u32 << 22) // one explicit epilogue scope
        | (u32::try_from(code_words).expect("five-bit code-word count") << 27);
    let scope = u32::try_from(epilogue_start / 4).expect("18-bit epilogue offset")
        | (u32::try_from(epilogue_index).expect("ten-bit epilogue index") << 22);

    let mut result = Vec::with_capacity(8 + codes.len());
    push_u32(&mut result, header);
    push_u32(&mut result, scope);
    result.extend_from_slice(&codes);
    Ok(result)
}

fn word(bytes: &[u8], offset: usize) -> JitResult<u32> {
    let encoded = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| unwind_error("A64 unwind decoder read outside function"))?;
    Ok(u32::from_le_bytes(
        encoded.try_into().expect("four-byte instruction slice"),
    ))
}

#[cfg(unix)]
fn advance(bytes: &mut Vec<u8>, location: &mut usize, target: usize) -> JitResult<()> {
    let delta = target
        .checked_sub(*location)
        .ok_or_else(|| unwind_error("DWARF CFI locations are not monotonic"))?;
    if delta <= 63 {
        bytes.push(0x40 | delta as u8);
    } else {
        bytes.push(0x04); // DW_CFA_advance_loc4
        push_u32(
            bytes,
            u32::try_from(delta).map_err(|_| unwind_error("DWARF CFI delta exceeds u32"))?,
        );
    }
    *location = target;
    Ok(())
}

#[cfg(unix)]
fn def_cfa_offset(bytes: &mut Vec<u8>, offset: u64) {
    bytes.push(0x0e);
    push_uleb(bytes, offset);
}

#[cfg(unix)]
fn offset(bytes: &mut Vec<u8>, register: u8, units: u64) {
    bytes.push(0x80 | register);
    push_uleb(bytes, units);
}

#[cfg(unix)]
fn restore(bytes: &mut Vec<u8>, register: u8) {
    bytes.push(0xc0 | register);
}

#[cfg(unix)]
fn reserve_u32(bytes: &mut Vec<u8>) -> usize {
    let offset = bytes.len();
    push_u32(bytes, 0);
    offset
}

#[cfg(unix)]
fn fill_length(bytes: &mut [u8], length_offset: usize) -> JitResult<()> {
    let length = bytes
        .len()
        .checked_sub(length_offset + 4)
        .and_then(|length| u32::try_from(length).ok())
        .ok_or_else(|| unwind_error("DWARF frame record length exceeds u32"))?;
    bytes[length_offset..length_offset + 4].copy_from_slice(&length.to_ne_bytes());
    Ok(())
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

#[cfg(unix)]
fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

#[cfg(unix)]
fn push_uleb(bytes: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
}

#[cfg(unix)]
fn push_sleb(bytes: &mut Vec<u8>, mut value: i64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        let done = (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0);
        bytes.push(if done { byte } else { byte | 0x80 });
        if done {
            break;
        }
    }
}

fn align(bytes: &mut Vec<u8>, alignment: usize) {
    let padding = (alignment - bytes.len() % alignment) % alignment;
    bytes.resize(bytes.len() + padding, 0);
}

fn unwind_error(detail: impl Into<String>) -> JitError {
    JitError::Encoding {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        A64FrameKind, A64UnwindFunction, ADD_X29_SP_0, LDP_X19_X20_POST_16, LDP_X21_X22_POST_16,
        LDP_X29_X30_POST_16, STP_X19_X20_PRE_16, STP_X21_X22_PRE_16, STP_X29_X30_PRE_16,
    };
    use crate::native::aarch64::encoder::{A64Encoder, XReg};
    use crate::native::model::CodeOffset;

    #[test]
    fn independently_named_frame_words_match_the_checked_encoder() {
        let mut encoder = A64Encoder::new();
        encoder
            .stp_x_pre(XReg::X19, XReg::X20, XReg::Sp, -16)
            .unwrap();
        encoder
            .stp_x_pre(XReg::X21, XReg::X22, XReg::Sp, -16)
            .unwrap();
        encoder
            .stp_x_pre(XReg::X29, XReg::X30, XReg::Sp, -16)
            .unwrap();
        encoder.add_x_imm(XReg::X29, XReg::Sp, 0).unwrap();
        encoder
            .ldp_x_post(XReg::X29, XReg::X30, XReg::Sp, 16)
            .unwrap();
        encoder
            .ldp_x_post(XReg::X21, XReg::X22, XReg::Sp, 16)
            .unwrap();
        encoder
            .ldp_x_post(XReg::X19, XReg::X20, XReg::Sp, 16)
            .unwrap();
        let words = encoder
            .into_bytes()
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(
            words,
            [
                STP_X19_X20_PRE_16,
                STP_X21_X22_PRE_16,
                STP_X29_X30_PRE_16,
                ADD_X29_SP_0,
                LDP_X29_X30_POST_16,
                LDP_X21_X22_POST_16,
                LDP_X19_X20_POST_16,
            ]
        );
    }

    #[test]
    fn windows_full_xdata_matches_llvm_shared_epilogue_suffixes() {
        let ordinary = super::encode_windows_xdata(32, 20, false).unwrap();
        assert_eq!(
            ordinary,
            [
                0x08, 0x00, 0x40, 0x10, // header: 8 words, 1 epilogue, 2 code words
                0x05, 0x00, 0x40, 0x00, // epilogue at word 5, code-byte index 1
                0xe1, 0x81, 0x22, 0xe3, 0xe4, 0xe3, 0xe3, 0xe3,
            ]
        );

        let kernel = super::encode_windows_xdata(48, 32, true).unwrap();
        assert_eq!(
            u32::from_le_bytes(kernel[0..4].try_into().unwrap()),
            0x1040_000c
        );
        assert_eq!(
            u32::from_le_bytes(kernel[4..8].try_into().unwrap()),
            0x0040_0008
        );
        assert_eq!(
            &kernel[8..],
            &[0xe1, 0x81, 0xcc, 0x81, 0x22, 0xe3, 0xe4, 0xe3]
        );

        let error = super::encode_windows_xdata(32, 32, false)
            .expect_err("epilogue scope outside the function must fail");
        assert!(error.to_string().contains("outside the function"));
    }

    #[test]
    fn windows_runtime_function_ranges_must_be_sorted_and_nonoverlapping() {
        let mut image = vec![0_u8; 32];
        let functions = [
            A64UnwindFunction {
                start: CodeOffset::new(16),
                end: CodeOffset::new(24),
                frame: A64FrameKind::Leaf,
            },
            A64UnwindFunction {
                start: CodeOffset::new(0),
                end: CodeOffset::new(8),
                frame: A64FrameKind::Leaf,
            },
        ];
        let error = super::append_windows_unwind_data(&mut image, &functions)
            .expect_err("unsorted Windows runtime functions must fail");
        assert!(error.to_string().contains("ordered, nonoverlapping"));
    }
}
