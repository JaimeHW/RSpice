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

/// The longest span one `.xdata` record can describe: 1,048,572 bytes.
///
/// The header's Function Length field is 18 bits of instruction words. A
/// function longer than that is not refused — it is described as several
/// *fragments*, each carrying its own `.pdata` record and, here, its own full
/// `.xdata` record. Microsoft, "ARM64 exception handling", *Large functions*:
/// "Fragments can be used to describe functions larger than the 1M limit
/// imposed by the bit fields in the `.xdata` header."
#[cfg(any(windows, test))]
const MAX_XDATA_FUNCTION_WORDS: usize = 0x3_ffff;

#[cfg(any(windows, test))]
const MAX_XDATA_FUNCTION_BYTES: usize = MAX_XDATA_FUNCTION_WORDS * 4;

/// One `.pdata`/`.xdata` pair: the span it describes, and which of the
/// function's prologue and epilogue lies inside that span.
///
/// A function within [`MAX_XDATA_FUNCTION_BYTES`] is a single fragment holding
/// both, which is the one record that has always been emitted. Past that size
/// the function is cut into several, and only the first carries the prologue:
/// "Only the first fragment of the function will contain a prolog; all other
/// fragments are marked as having no prolog."
#[cfg(any(windows, test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct A64Fragment {
    /// Byte offset of the fragment from the start of the function.
    start: usize,
    /// Byte length of the fragment.
    len: usize,
    /// Byte offset of the epilogue from the start of *this fragment*, when the
    /// epilogue lies inside it: "each epilog scope in a fragment specifies its
    /// starting offset relative to the start of the fragment, not the start of
    /// the function."
    epilogue_start: Option<usize>,
    /// Whether this fragment carries the function's real prologue.
    has_prologue: bool,
}

#[cfg(any(windows, test))]
pub(crate) fn append_windows_unwind_data(
    image: &mut Vec<u8>,
    functions: &[A64UnwindFunction],
) -> JitResult<Vec<WindowsA64RuntimeFunction>> {
    let code_len = image.len();
    // Plan before encoding. Planning reads the instruction stream each record
    // will describe, and appending `.xdata` to the same buffer moves it; doing
    // every read first also leaves the image untouched when a function is
    // rejected.
    let mut planned = Vec::new();
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
        let fragments = plan_windows_fragments(&image[start..end], epilogue_start)?;
        planned.push((start, saves_kernel_io, fragments));
    }

    let mut table = Vec::new();
    for (start, saves_kernel_io, fragments) in planned {
        for fragment in fragments {
            align(image, 4);
            let unwind_data = u32::try_from(image.len())
                .map_err(|_| unwind_error("Windows ARM64 xdata RVA exceeds u32"))?;
            let xdata = encode_windows_xdata(fragment, saves_kernel_io)?;
            image.extend_from_slice(&xdata);
            table.push(WindowsA64RuntimeFunction {
                begin_address: start
                    .checked_add(fragment.start)
                    .and_then(|rva| u32::try_from(rva).ok())
                    .ok_or_else(|| unwind_error("Windows ARM64 function RVA exceeds u32"))?,
                unwind_data,
            });
        }
    }
    Ok(table)
}

/// Cut one function into the fragments its `.xdata` records will describe.
///
/// A function that fits one record is one fragment, so nothing about the
/// common case moves. Past that the cut is greedy from the front, and obeys
/// two rules from *Large functions*: each fragment is "smaller than 1M", and
/// "each fragment should be adjusted so that it doesn't split an epilog into
/// multiple pieces" — the boundary is pulled back to the first epilogue
/// instruction rather than landing inside the return sequence. Only the first
/// fragment carries the prologue, and the epilogue lands in exactly one.
///
/// Boundaries sit on instruction boundaries outside any inline constant
/// island. Island data is never executed, so a boundary inside one would not
/// be wrong, but a `.pdata` record whose Function Start RVA names a `double`
/// is not something to leave in a shipped image.
#[cfg(any(windows, test))]
fn plan_windows_fragments(code: &[u8], epilogue_start: usize) -> JitResult<Vec<A64Fragment>> {
    if code.len() % 4 != 0 || epilogue_start % 4 != 0 {
        return Err(unwind_error(
            "Windows ARM64 function offsets are not word aligned",
        ));
    }
    if epilogue_start >= code.len() {
        return Err(unwind_error(
            "Windows ARM64 epilogue scope is outside the function",
        ));
    }
    if code.len() <= MAX_XDATA_FUNCTION_BYTES {
        return Ok(vec![A64Fragment {
            start: 0,
            len: code.len(),
            epilogue_start: Some(epilogue_start),
            has_prologue: true,
        }]);
    }

    let mut fragments = Vec::new();
    let mut start = 0_usize;
    while code.len() - start > MAX_XDATA_FUNCTION_BYTES {
        // `epilogue_start` is always ahead of `start` here: the epilogue is a
        // handful of words at the end of the function, so a `start` at or past
        // it leaves far less than one fragment to cover and the loop is over.
        let limit = (start + MAX_XDATA_FUNCTION_BYTES).min(epilogue_start);
        let boundary = last_instruction_boundary(code, start, limit)?;
        fragments.push(A64Fragment {
            start,
            len: boundary - start,
            epilogue_start: None,
            has_prologue: fragments.is_empty(),
        });
        start = boundary;
    }
    fragments.push(A64Fragment {
        start,
        len: code.len() - start,
        epilogue_start: Some(epilogue_start - start),
        has_prologue: fragments.is_empty(),
    });
    Ok(fragments)
}

/// The last instruction boundary at or before `limit`, never inside an
/// announced constant island.
#[cfg(any(windows, test))]
fn last_instruction_boundary(code: &[u8], start: usize, limit: usize) -> JitResult<usize> {
    let mut boundary = start;
    while boundary < limit {
        let next = instruction_unit_end(code, boundary)?;
        if next > limit {
            break;
        }
        boundary = next;
    }
    if boundary == start {
        // Unreachable by construction, and cheaper to keep than to argue: the
        // longest indivisible unit is a `B` over an island, and the island
        // marker's `imm16` caps the data at 65,535 constants — 4 + 4 +
        // 65,535 * 8 = 524,288 bytes, exactly half of what a fragment spans.
        return Err(unwind_error(format!(
            "Windows ARM64 fragment starting at function byte {start} has no instruction \
             boundary within {} bytes",
            limit - start
        )));
    }
    Ok(boundary)
}

/// One indivisible step of the instruction walk.
///
/// Ordinarily an A64 instruction. The exception is the `B over; BRK words;
/// <constants>` trio the emitter writes for an inline constant island: the
/// branch, the marker and the data it names step as one, so no boundary can
/// land on the island.
#[cfg(any(windows, test))]
fn instruction_unit_end(code: &[u8], offset: usize) -> JitResult<usize> {
    let instruction = word(code, offset)?;
    if instruction & 0xFC00_0000 == 0x1400_0000
        && let Ok(marker) = word(code, offset + 4)
        && marker & 0xFFE0_001F == 0xD420_0000
    {
        let data_words = ((marker >> 5) & 0xFFFF) as usize;
        return offset
            .checked_add(8 + data_words * 8)
            .filter(|end| *end <= code.len())
            .ok_or_else(|| {
                unwind_error(format!(
                    "A64 constant island at byte {offset} runs past its own function"
                ))
            });
    }
    Ok(offset + 4)
}

/// Encode one fragment's full `.xdata` record.
///
/// Header word, *`.xdata` records*: Function Length (18 bits, bytes / 4), Vers
/// (2 bits, 0), X (1 bit, no exception data), E (1 bit, 0 — an epilog scope is
/// never packed into the header here), Epilog Count (5 bits), Code Words
/// (5 bits). Code Words is always 2, so the two fields are never both zero and
/// the extension word is never required. Each epilog scope word is Epilog
/// Start Offset (18 bits, bytes / 4, relative to the fragment), Res (4 bits,
/// 0), Epilog Start Index (10 bits, a byte index into the unwind codes).
///
/// The unwind codes are the prologue read backwards, which is also the
/// epilogue read forwards:
///
/// ```text
/// set_fp        e1     add x29,sp,#0
/// save_fplr_x   81     stp x29,x30,[sp,#-16]!  / ldp x29,x30,[sp],#16
/// save_regp_x   cc 81  stp x21,x22,[sp,#-16]!  / ldp x21,x22,[sp],#16  (optional)
/// save_r19r20_x 22     stp x19,x20,[sp,#-16]!  / ldp x19,x20,[sp],#16
/// nop           e3     bti c                   / ret
/// end           e4
/// ```
///
/// A fragment that does not carry the prologue puts `end_c` (e5) in front of
/// that sequence. It is the phantom prolog of *Function fragments*: the
/// leading `end_c` "indicates the size of prolog is zero", and the codes
/// between it and the `end` "represent the prolog operations in the parent
/// region", which is how a fragment in the middle of a function still unwinds.
#[cfg(any(windows, test))]
fn encode_windows_xdata(fragment: A64Fragment, saves_kernel_io: bool) -> JitResult<Vec<u8>> {
    if fragment.len % 4 != 0 || fragment.epilogue_start.is_some_and(|start| start % 4 != 0) {
        return Err(unwind_error(
            "Windows ARM64 function offsets are not word aligned",
        ));
    }
    let fragment_words = fragment.len / 4;
    if fragment_words == 0 || fragment_words > MAX_XDATA_FUNCTION_WORDS {
        return Err(unwind_error(format!(
            "Windows ARM64 fragment of {} bytes does not fit the 18-bit Function Length field, \
             which spans {MAX_XDATA_FUNCTION_BYTES} bytes",
            fragment.len
        )));
    }
    if fragment
        .epilogue_start
        .is_some_and(|start| start >= fragment.len)
    {
        return Err(unwind_error(
            "Windows ARM64 epilogue scope is outside the function",
        ));
    }

    let mut codes = Vec::with_capacity(8);
    if !fragment.has_prologue {
        codes.push(0xe5); // end_c: this fragment's own prologue is empty
    }
    // Byte index of `set_fp`, which is where the host prologue's codes begin.
    let prologue_index = codes.len();
    codes.extend_from_slice(&[0xe1, 0x81]); // set_fp; save_fplr_x -16
    if saves_kernel_io {
        codes.extend_from_slice(&[0xcc, 0x81]); // save_regp_x x21/x22, -16
    }
    codes.extend_from_slice(&[0x22, 0xe3, 0xe4]); // save x19/x20; BTI/RET nop; end

    // The epilogue is the prologue-code suffix beginning after `set_fp`: this
    // frame never restores `sp` from `x29`, so the epilogue's first
    // instruction is the `ldp x29,x30` that `save_fplr_x` describes.
    let epilogue_index = prologue_index + 1;
    let (epilog_count, scope) = match (fragment.has_prologue, fragment.epilogue_start) {
        // Prologue and epilogue both inside: the whole function in one record.
        (true, Some(epilogue)) => (1_usize, Some((epilogue, epilogue_index))),
        // Prologue only — *Function fragments* region 1: "Only the prolog must
        // be described ... it can be represented by setting Epilog Count = 0."
        (true, None) => (0, None),
        // Epilogue only — region 2: a real scope, at its offset within this
        // fragment, whose index points past the `end_c` at the codes the
        // epilogue executes.
        (false, Some(epilogue)) => (1, Some((epilogue, epilogue_index))),
        // Neither — region 3: "Epilog Count = 1 ... but Epilog Start Index
        // also points to `end_c`." A code sequence that starts on `end_c` is
        // zero instructions long, so no address in the fragment ever falls in
        // this scope, which is the doc's "Partial unwind will never happen in
        // this region of code".
        (false, None) => (1, Some((0, 0))),
    };

    let code_words = codes.len().div_ceil(4);
    if code_words > 31 || epilogue_index > 0x3ff {
        return Err(unwind_error(
            "Windows ARM64 unwind-code pool exceeds compact header fields",
        ));
    }
    codes.resize(code_words * 4, 0xe3);

    let header = u32::try_from(fragment_words).expect("18-bit function length")
        | (u32::try_from(epilog_count).expect("five-bit epilogue count") << 22)
        | (u32::try_from(code_words).expect("five-bit code-word count") << 27);

    let mut result = Vec::with_capacity(8 + codes.len());
    push_u32(&mut result, header);
    if let Some((epilogue_start, index)) = scope {
        push_u32(
            &mut result,
            u32::try_from(epilogue_start / 4).expect("18-bit epilogue offset")
                | (u32::try_from(index).expect("ten-bit epilogue index") << 22),
        );
    }
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
        A64Fragment, A64FrameKind, A64UnwindFunction, ADD_X29_SP_0, BTI_C, LDP_X19_X20_POST_16,
        LDP_X21_X22_POST_16, LDP_X29_X30_POST_16, MAX_XDATA_FUNCTION_BYTES,
        MAX_XDATA_FUNCTION_WORDS, RET_LR, STP_X19_X20_PRE_16, STP_X21_X22_PRE_16,
        STP_X29_X30_PRE_16,
    };
    use crate::native::aarch64::encoder::{A64Encoder, XReg};
    use crate::native::model::CodeOffset;

    const A64_NOP: u32 = 0xD503_201F;

    /// The whole function in one fragment: what every function within the
    /// limit produces, and what shipped before fragmentation existed.
    fn whole(len: usize, epilogue_start: usize) -> A64Fragment {
        A64Fragment {
            start: 0,
            len,
            epilogue_start: Some(epilogue_start),
            has_prologue: true,
        }
    }

    /// One `.xdata` record read back through the bit layouts of "ARM64
    /// exception handling", *`.xdata` records*.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct DecodedXdata {
        function_words: u32,
        version: u32,
        exception_data: bool,
        packed_epilog: bool,
        epilog_count: u32,
        code_words: u32,
        /// `(Epilog Start Offset in words, Epilog Start Index in code bytes)`.
        scopes: Vec<(u32, u32)>,
        codes: Vec<u8>,
        /// What the doc's `ComputeXdataSize` says the record occupies.
        size: usize,
    }

    fn decode_xdata(xdata: &[u8]) -> DecodedXdata {
        let word = |index: usize| {
            u32::from_ne_bytes(
                xdata[index * 4..index * 4 + 4]
                    .try_into()
                    .expect("four-byte xdata word"),
            )
        };
        let header = word(0);

        // `ComputeXdataSize`, transcribed from the doc.
        let extended = (header >> 22) == 0;
        let (mut size, epilog_scopes, unwind_words) = if extended {
            let extension = word(1);
            (8_usize, extension & 0xffff, (extension >> 16) & 0xff)
        } else {
            (4_usize, (header >> 22) & 0x1f, (header >> 27) & 0x1f)
        };
        if header & (1 << 21) == 0 {
            size += 4 * epilog_scopes as usize;
        }
        size += 4 * unwind_words as usize;
        if header & (1 << 20) != 0 {
            size += 4; // Exception handler RVA
        }

        let header_words = if extended { 2 } else { 1 };
        let mut scopes = Vec::new();
        if header & (1 << 21) == 0 {
            for index in 0..epilog_scopes as usize {
                let scope = word(header_words + index);
                assert_eq!(scope & 0x003c_0000, 0, "epilog scope Res must be zero");
                scopes.push((scope & 0x0003_ffff, scope >> 22));
            }
        }
        let codes_at = (header_words + scopes.len()) * 4;
        DecodedXdata {
            function_words: header & 0x0003_ffff,
            version: (header >> 18) & 0x3,
            exception_data: header & (1 << 20) != 0,
            packed_epilog: header & (1 << 21) != 0,
            epilog_count: epilog_scopes,
            code_words: unwind_words,
            scopes,
            codes: xdata[codes_at..codes_at + unwind_words as usize * 4].to_vec(),
            size,
        }
    }

    fn put(bytes: &mut [u8], offset: usize, instruction: u32) {
        bytes[offset..offset + 4].copy_from_slice(&instruction.to_le_bytes());
    }

    /// A function of `len` bytes carrying the exact prologue and epilogue
    /// [`super::analyze_function`] recognizes, its body all `nop`, and
    /// optionally one inline constant island whose `B` sits at `branch`.
    fn synthetic_function(len: usize, island: Option<(usize, u16)>) -> Vec<u8> {
        assert_eq!(len % 4, 0, "an A64 function is whole words");
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len / 4 {
            bytes.extend_from_slice(&A64_NOP.to_le_bytes());
        }
        put(&mut bytes, 0, BTI_C);
        put(&mut bytes, 4, STP_X19_X20_PRE_16);
        put(&mut bytes, 8, STP_X29_X30_PRE_16);
        put(&mut bytes, 12, ADD_X29_SP_0);
        if let Some((branch, data_words)) = island {
            assert_eq!(branch % 8, 0, "island data is eight-aligned");
            let end = branch + 8 + usize::from(data_words) * 8;
            put(
                &mut bytes,
                branch,
                0x1400_0000 | ((end - branch) / 4) as u32,
            );
            put(
                &mut bytes,
                branch + 4,
                0xD420_0000 | (u32::from(data_words) << 5),
            );
        }
        put(&mut bytes, len - 12, LDP_X29_X30_POST_16);
        put(&mut bytes, len - 8, LDP_X19_X20_POST_16);
        put(&mut bytes, len - 4, RET_LR);
        bytes
    }

    /// One synthetic function's published unwind data: the image it was
    /// appended to, the `.pdata` table, every `.xdata` record decoded back,
    /// and the epilogue's offset in the function.
    struct Published {
        image: Vec<u8>,
        table: Vec<super::WindowsA64RuntimeFunction>,
        records: Vec<DecodedXdata>,
        epilogue_start: usize,
    }

    fn publish(bytes: &[u8]) -> Published {
        let function = super::analyze_function(CodeOffset::new(0), bytes, "synthetic")
            .expect("the synthetic function has the recognized frame");
        let A64FrameKind::Frame { epilogue_start, .. } = function.frame else {
            panic!("the synthetic function is not a leaf");
        };
        let mut image = bytes.to_vec();
        let table = super::append_windows_unwind_data(&mut image, &[function])
            .expect("publish the synthetic function's unwind data");

        // Records are laid out back to back after the code, so each one's own
        // `ComputeXdataSize` must account for exactly its span.
        let mut records = Vec::with_capacity(table.len());
        for (index, entry) in table.iter().enumerate() {
            let start = entry.unwind_data as usize;
            let end = table
                .get(index + 1)
                .map_or(image.len(), |next| next.unwind_data as usize);
            let record = decode_xdata(&image[start..end]);
            assert_eq!(
                record.size,
                end - start,
                "ComputeXdataSize for record {index}"
            );
            records.push(record);
        }
        Published {
            image,
            table,
            records,
            epilogue_start,
        }
    }

    /// Everything true of any fragmentation of any function.
    fn assert_fragments_tile(
        table: &[super::WindowsA64RuntimeFunction],
        records: &[DecodedXdata],
        len: usize,
        epilogue_start: usize,
    ) {
        let mut expected_start = 0_u32;
        for (index, (entry, record)) in table.iter().zip(records).enumerate() {
            assert_eq!(
                entry.begin_address, expected_start,
                "fragment {index} start"
            );
            assert_eq!(record.version, 0, "fragment {index} Vers must be 0");
            assert!(!record.exception_data, "fragment {index} X must be 0");
            assert!(!record.packed_epilog, "fragment {index} E must be 0");
            assert!(
                record.function_words as usize <= MAX_XDATA_FUNCTION_WORDS,
                "fragment {index} is {} words, past the 18-bit field",
                record.function_words
            );
            assert!(
                entry.begin_address as usize <= epilogue_start,
                "fragment {index} starts inside the epilogue at {epilogue_start}"
            );
            expected_start += record.function_words * 4;
        }
        assert_eq!(
            expected_start as usize, len,
            "the fragments must tile the function exactly"
        );
        assert!(
            table
                .windows(2)
                .all(|pair| pair[0].begin_address < pair[1].begin_address),
            "the runtime-function table must be sorted by start RVA"
        );
    }

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

    /// Byte identity for every function that fits one record.
    ///
    /// These are the exact bytes emitted before fragmentation existed, so a
    /// function within the limit cannot have moved.
    #[test]
    fn windows_full_xdata_matches_llvm_shared_epilogue_suffixes() {
        let ordinary = super::encode_windows_xdata(whole(32, 20), false).unwrap();
        assert_eq!(
            ordinary,
            [
                0x08, 0x00, 0x40, 0x10, // header: 8 words, 1 epilogue, 2 code words
                0x05, 0x00, 0x40, 0x00, // epilogue at word 5, code-byte index 1
                0xe1, 0x81, 0x22, 0xe3, 0xe4, 0xe3, 0xe3, 0xe3,
            ]
        );

        let kernel = super::encode_windows_xdata(whole(48, 32), true).unwrap();
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

        // The same record read back through the documented bit layouts.
        let decoded = decode_xdata(&ordinary);
        assert_eq!(decoded.function_words, 8);
        assert_eq!(decoded.version, 0);
        assert!(!decoded.exception_data);
        assert!(!decoded.packed_epilog);
        assert_eq!(decoded.epilog_count, 1);
        assert_eq!(decoded.code_words, 2);
        assert_eq!(decoded.scopes, [(5, 1)]);
        assert_eq!(
            decoded.codes,
            [0xe1, 0x81, 0x22, 0xe3, 0xe4, 0xe3, 0xe3, 0xe3]
        );
        assert_eq!(decoded.size, ordinary.len());

        let error = super::plan_windows_fragments(&[0_u8; 32], 32)
            .expect_err("epilogue scope outside the function must fail");
        assert!(error.to_string().contains("outside the function"));
    }

    /// One record can only name 18 bits of instruction words. The planner
    /// never hands the encoder more, but the field is guarded where it lives.
    #[test]
    fn windows_xdata_refuses_a_fragment_past_the_function_length_field() {
        let error = super::encode_windows_xdata(
            A64Fragment {
                start: 0,
                len: MAX_XDATA_FUNCTION_BYTES + 4,
                epilogue_start: None,
                has_prologue: false,
            },
            false,
        )
        .expect_err("a fragment past the 18-bit field must fail");
        assert!(error.to_string().contains("18-bit Function Length"));
    }

    /// A function past the megabyte one `.xdata` header can name, decoded back
    /// record by record.
    ///
    /// Two shapes, because the split rule has two ways to choose a boundary:
    /// the first function's natural boundary lands inside a constant island
    /// and has to be pulled back in front of the `B` that jumps over it; the
    /// second's lands inside the epilogue and has to be pulled back to the
    /// epilogue's first instruction.
    #[test]
    fn windows_unwind_data_fragments_a_function_past_the_xdata_limit() {
        // The `B` sits just under the limit and its constants straddle it.
        const ISLAND_BRANCH: usize = 1_048_560;
        const ISLAND_WORDS: u16 = 16;
        assert!(ISLAND_BRANCH < MAX_XDATA_FUNCTION_BYTES);
        assert!(ISLAND_BRANCH + 8 + ISLAND_WORDS as usize * 8 > MAX_XDATA_FUNCTION_BYTES);

        let prologue_codes = [0xe1_u8, 0x81, 0x22, 0xe3, 0xe4];
        let mut phantom = vec![0xe5_u8];
        phantom.extend_from_slice(&prologue_codes);
        phantom.extend_from_slice(&[0xe3, 0xe3]); // word padding
        let mut host = prologue_codes.to_vec();
        host.extend_from_slice(&[0xe3, 0xe3, 0xe3]);

        // The first fragment's record is the whole function's record with its
        // own length and no epilog scope: the same codes and the same header
        // fields otherwise.
        let unfragmented = decode_xdata(
            &super::encode_windows_xdata(whole(64, 52), false).expect("one-record encoding"),
        );

        for (len, island) in [
            (2_101_236_usize, Some((ISLAND_BRANCH, ISLAND_WORDS))),
            (2_097_148, None),
        ] {
            let bytes = synthetic_function(len, island);
            let Published {
                table,
                records,
                epilogue_start,
                ..
            } = publish(&bytes);
            assert_eq!(table.len(), 3, "expected three fragments for {len} bytes");
            assert_fragments_tile(&table, &records, len, epilogue_start);

            // Region 1: the prologue, and no epilog scope, because the
            // epilogue is in another fragment.
            assert_eq!(records[0].epilog_count, 0);
            assert!(records[0].scopes.is_empty());
            assert_eq!(records[0].codes, host);

            // Region 3: neither prologue nor epilogue. `end_c` in front of the
            // host prologue's codes, and the one scope points at the `end_c`.
            assert_eq!(records[1].epilog_count, 1);
            assert_eq!(records[1].scopes, [(0, 0)]);
            assert_eq!(records[1].codes, phantom);

            // Region 2: the epilogue, at its offset within this fragment, with
            // the index one byte past the `end_c`.
            let last_start = table[2].begin_address as usize;
            assert_eq!(records[2].epilog_count, 1);
            assert_eq!(
                records[2].scopes,
                [(((epilogue_start - last_start) / 4) as u32, 2)]
            );
            assert_eq!(records[2].codes, phantom);

            // The epilogue is whole inside the last fragment: no fragment
            // starts strictly inside the return sequence.
            for entry in &table {
                let start = entry.begin_address as usize;
                assert!(
                    start <= epilogue_start || start >= len,
                    "fragment at {start} splits the epilogue at {epilogue_start}"
                );
            }

            match island {
                // The boundary is the `B`, not the constants behind it.
                Some((branch, _)) => assert_eq!(table[1].begin_address as usize, branch),
                // The boundary is the epilogue's first instruction.
                None => assert_eq!(table[2].begin_address as usize, epilogue_start),
            }

            assert_eq!(records[0].codes, unfragmented.codes);
            assert_eq!(records[0].code_words, unfragmented.code_words);
            assert_eq!(records[0].version, unfragmented.version);
            assert_eq!(records[0].exception_data, unfragmented.exception_data);
            assert_eq!(records[0].packed_epilog, unfragmented.packed_epilog);
        }
    }

    /// A function inside the limit is still one record, one `.pdata` entry,
    /// and the same bytes it always was.
    #[test]
    fn windows_unwind_data_leaves_a_function_within_the_limit_in_one_fragment() {
        let bytes = synthetic_function(4096, None);
        let Published {
            image,
            table,
            records,
            epilogue_start,
        } = publish(&bytes);
        assert_eq!(table.len(), 1);
        assert_fragments_tile(&table, &records, 4096, epilogue_start);
        assert_eq!(records[0].function_words, 1024);
        assert_eq!(records[0].scopes, [((epilogue_start / 4) as u32, 1)]);
        assert_eq!(&image[..4096], &bytes[..], "the code must not have moved");
        assert_eq!(
            &image[table[0].unwind_data as usize..],
            super::encode_windows_xdata(whole(4096, epilogue_start), false)
                .unwrap()
                .as_slice()
        );
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
