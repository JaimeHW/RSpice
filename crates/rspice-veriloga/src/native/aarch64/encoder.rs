//! Checked encoding for the A64 subset emitted by RSpice.
//!
//! A64 instructions are fixed-width little-endian words. Every immediate and
//! patch is range-checked before it reaches the image; callers never truncate
//! an address or displacement implicitly.

use crate::native::{JitError, JitResult};

const MODEL: &str = "native-aarch64";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)] // Complete architectural register set keeps encoding APIs typed.
pub(crate) enum XReg {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    Sp = 31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)] // Complete architectural register set keeps encoding APIs typed.
pub(crate) enum DReg {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    D10 = 10,
    D11 = 11,
    D12 = 12,
    D13 = 13,
    D14 = 14,
    D15 = 15,
    D16 = 16,
    D17 = 17,
    D18 = 18,
    D19 = 19,
    D20 = 20,
    D21 = 21,
    D22 = 22,
    D23 = 23,
    D24 = 24,
    D25 = 25,
    D26 = 26,
    D27 = 27,
    D28 = 28,
    D29 = 29,
    D30 = 30,
    D31 = 31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)] // Conditions are kept numerically complete for checked encodings.
pub(crate) enum Condition {
    Equal = 0,
    NotEqual = 1,
    CarrySet = 2,
    CarryClear = 3,
    Minus = 4,
    Plus = 5,
    Overflow = 6,
    NoOverflow = 7,
    UnsignedHigher = 8,
    UnsignedLowerOrSame = 9,
    SignedGreaterOrEqual = 10,
    SignedLess = 11,
    SignedGreater = 12,
    SignedLessOrEqual = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BranchPatch {
    instruction_offset: usize,
    kind: BranchKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LiteralPatch {
    instruction_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BranchKind {
    Unconditional,
    #[cfg(test)]
    Link,
}

#[derive(Debug, Default)]
pub(crate) struct A64Encoder {
    bytes: Vec<u8>,
}

impl A64Encoder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn position(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn nop(&mut self) {
        self.emit(0xD503_201F);
    }

    pub(crate) fn bti_c(&mut self) {
        self.emit(0xD503_245F);
    }

    pub(crate) fn ret(&mut self) {
        self.ret_reg(XReg::X30);
    }

    pub(crate) fn ret_reg(&mut self, register: XReg) {
        self.emit(0xD65F_0000 | (reg(register) << 5));
    }

    pub(crate) fn blr(&mut self, register: XReg) {
        self.emit(0xD63F_0000 | (reg(register) << 5));
    }

    pub(crate) fn image_call(
        &mut self,
        function_image_offset: usize,
        target_image_offset: usize,
        scratch: XReg,
    ) -> JitResult<()> {
        require_general_register(scratch, "image-call scratch register")?;
        let instruction_image_offset = function_image_offset
            .checked_add(self.position())
            .ok_or_else(|| encoding_error("A64 image-call source overflow"))?;
        let source_page = instruction_image_offset & !0xfff;
        let target_page = target_image_offset & !0xfff;
        let page_delta = i64::try_from(target_page)
            .ok()
            .and_then(|target| {
                i64::try_from(source_page)
                    .ok()
                    .and_then(|source| target.checked_sub(source))
            })
            .map(|delta| delta / 4096)
            .ok_or_else(|| encoding_error("A64 image-call page displacement overflow"))?;
        let immediate = encode_signed(page_delta, 21, "A64 ADRP page displacement")?;
        let immediate_low = immediate & 0x3;
        let immediate_high = (immediate >> 2) & 0x7_ffff;
        self.emit(0x9000_0000 | (immediate_low << 29) | (immediate_high << 5) | reg(scratch));
        let page_offset = u16::try_from(target_image_offset & 0xfff)
            .map_err(|_| encoding_error("A64 image-call page offset overflow"))?;
        self.add_x_imm(scratch, scratch, page_offset)?;
        self.blr(scratch);
        Ok(())
    }

    pub(crate) fn movz_x(&mut self, destination: XReg, immediate: u16, shift: u8) -> JitResult<()> {
        let halfword = checked_halfword_shift(shift)?;
        require_general_register(destination, "MOVZ destination")?;
        self.emit(
            0xD280_0000
                | (u32::from(halfword) << 21)
                | (u32::from(immediate) << 5)
                | reg(destination),
        );
        Ok(())
    }

    pub(crate) fn movk_x(&mut self, destination: XReg, immediate: u16, shift: u8) -> JitResult<()> {
        let halfword = checked_halfword_shift(shift)?;
        require_general_register(destination, "MOVK destination")?;
        self.emit(
            0xF280_0000
                | (u32::from(halfword) << 21)
                | (u32::from(immediate) << 5)
                | reg(destination),
        );
        Ok(())
    }

    pub(crate) fn mov_u64(&mut self, destination: XReg, value: u64) -> JitResult<()> {
        self.movz_x(destination, value as u16, 0)?;
        for shift in [16_u8, 32, 48] {
            let halfword = ((value >> shift) & 0xffff) as u16;
            if halfword != 0 {
                self.movk_x(destination, halfword, shift)?;
            }
        }
        Ok(())
    }

    pub(crate) fn mov_x(&mut self, destination: XReg, source: XReg) -> JitResult<()> {
        require_general_register(destination, "MOV destination")?;
        require_general_register(source, "MOV source")?;
        self.emit(0xAA00_03E0 | (reg(source) << 16) | reg(destination));
        Ok(())
    }

    pub(crate) fn add_x(&mut self, destination: XReg, left: XReg, right: XReg) -> JitResult<()> {
        require_general_register(destination, "ADD destination")?;
        require_general_register(left, "ADD left operand")?;
        require_general_register(right, "ADD right operand")?;
        self.emit(0x8B00_0000 | (reg(right) << 16) | (reg(left) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn sub_x(&mut self, destination: XReg, left: XReg, right: XReg) -> JitResult<()> {
        require_general_register(destination, "SUB destination")?;
        require_general_register(left, "SUB left operand")?;
        require_general_register(right, "SUB right operand")?;
        self.emit(0xCB00_0000 | (reg(right) << 16) | (reg(left) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn and_x(&mut self, destination: XReg, left: XReg, right: XReg) -> JitResult<()> {
        self.logical_x(0x8A00_0000, destination, left, right)
    }

    pub(crate) fn orr_x(&mut self, destination: XReg, left: XReg, right: XReg) -> JitResult<()> {
        self.logical_x(0xAA00_0000, destination, left, right)
    }

    pub(crate) fn eor_x(&mut self, destination: XReg, left: XReg, right: XReg) -> JitResult<()> {
        self.logical_x(0xCA00_0000, destination, left, right)
    }

    fn logical_x(
        &mut self,
        opcode: u32,
        destination: XReg,
        left: XReg,
        right: XReg,
    ) -> JitResult<()> {
        require_general_register(destination, "logical destination")?;
        require_general_register(left, "logical left operand")?;
        require_general_register(right, "logical right operand")?;
        self.emit(opcode | (reg(right) << 16) | (reg(left) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn lslv_x(&mut self, destination: XReg, value: XReg, count: XReg) -> JitResult<()> {
        self.variable_shift_x(0x9AC0_2000, destination, value, count)
    }

    pub(crate) fn asrv_x(&mut self, destination: XReg, value: XReg, count: XReg) -> JitResult<()> {
        self.variable_shift_x(0x9AC0_2800, destination, value, count)
    }

    fn variable_shift_x(
        &mut self,
        opcode: u32,
        destination: XReg,
        value: XReg,
        count: XReg,
    ) -> JitResult<()> {
        require_general_register(destination, "shift destination")?;
        require_general_register(value, "shift value")?;
        require_general_register(count, "shift count")?;
        self.emit(opcode | (reg(count) << 16) | (reg(value) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn add_x_imm(
        &mut self,
        destination: XReg,
        source: XReg,
        immediate: u16,
    ) -> JitResult<()> {
        self.add_sub_imm(0x9100_0000, destination, source, immediate)
    }

    pub(crate) fn sub_x_imm(
        &mut self,
        destination: XReg,
        source: XReg,
        immediate: u16,
    ) -> JitResult<()> {
        self.add_sub_imm(0xD100_0000, destination, source, immediate)
    }

    fn add_sub_imm(
        &mut self,
        opcode: u32,
        destination: XReg,
        source: XReg,
        immediate: u16,
    ) -> JitResult<()> {
        if immediate > 4095 {
            return Err(encoding_error(format!(
                "A64 add/sub immediate {immediate} exceeds imm12"
            )));
        }
        self.emit(opcode | (u32::from(immediate) << 10) | (reg(source) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn add_x_imm_shift12(
        &mut self,
        destination: XReg,
        source: XReg,
        immediate: u16,
    ) -> JitResult<()> {
        if immediate > 4095 {
            return Err(encoding_error(format!(
                "A64 shifted ADD immediate {immediate} exceeds imm12"
            )));
        }
        require_general_register(destination, "shifted ADD destination")?;
        self.emit(
            0x9140_0000 | (u32::from(immediate) << 10) | (reg(source) << 5) | reg(destination),
        );
        Ok(())
    }

    pub(crate) fn ldr_x_unsigned(
        &mut self,
        destination: XReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        self.load_store_unsigned(0xF940_0000, reg(destination), base, byte_offset, 8)
    }

    pub(crate) fn str_x_unsigned(
        &mut self,
        source: XReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        self.load_store_unsigned(0xF900_0000, reg(source), base, byte_offset, 8)
    }

    pub(crate) fn ldr_d_unsigned(
        &mut self,
        destination: DReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        self.load_store_unsigned(0xFD40_0000, dreg(destination), base, byte_offset, 8)
    }

    pub(crate) fn str_d_unsigned(
        &mut self,
        source: DReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        self.load_store_unsigned(0xFD00_0000, dreg(source), base, byte_offset, 8)
    }

    pub(crate) fn ldrb_w_unsigned(
        &mut self,
        destination: XReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        require_general_register(destination, "LDRB destination")?;
        self.load_store_unsigned(0x3940_0000, reg(destination), base, byte_offset, 1)
    }

    pub(crate) fn strb_w_unsigned(
        &mut self,
        source: XReg,
        base: XReg,
        byte_offset: usize,
    ) -> JitResult<()> {
        require_general_register(source, "STRB source")?;
        self.load_store_unsigned(0x3900_0000, reg(source), base, byte_offset, 1)
    }

    fn load_store_unsigned(
        &mut self,
        opcode: u32,
        transfer_register: u32,
        base: XReg,
        byte_offset: usize,
        scale: usize,
    ) -> JitResult<()> {
        if byte_offset % scale != 0 || byte_offset / scale > 4095 {
            return Err(encoding_error(format!(
                "A64 unsigned load/store byte offset {byte_offset} is not representable at scale {scale}"
            )));
        }
        let immediate = u32::try_from(byte_offset / scale)
            .map_err(|_| encoding_error("A64 unsigned load/store offset exceeds u32"))?;
        self.emit(opcode | (immediate << 10) | (reg(base) << 5) | transfer_register);
        Ok(())
    }

    pub(crate) fn stp_x_pre(
        &mut self,
        first: XReg,
        second: XReg,
        base: XReg,
        byte_offset: i16,
    ) -> JitResult<()> {
        self.pair(0xA980_0000, first, second, base, byte_offset)
    }

    pub(crate) fn ldp_x_post(
        &mut self,
        first: XReg,
        second: XReg,
        base: XReg,
        byte_offset: i16,
    ) -> JitResult<()> {
        self.pair(0xA8C0_0000, first, second, base, byte_offset)
    }

    fn pair(
        &mut self,
        opcode: u32,
        first: XReg,
        second: XReg,
        base: XReg,
        byte_offset: i16,
    ) -> JitResult<()> {
        if byte_offset % 8 != 0 || !(-512..=504).contains(&byte_offset) {
            return Err(encoding_error(format!(
                "A64 pair byte offset {byte_offset} is outside the signed scaled imm7 range"
            )));
        }
        require_general_register(first, "pair first register")?;
        require_general_register(second, "pair second register")?;
        let immediate = u32::from((byte_offset / 8) as u8) & 0x7f;
        self.emit(opcode | (immediate << 15) | (reg(second) << 10) | (reg(base) << 5) | reg(first));
        Ok(())
    }

    pub(crate) fn fmov_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E60_4000 | (dreg(source) << 5) | dreg(destination));
    }

    /// Emit the architectural scalar floating-point immediate form when the
    /// exact IEEE-754 bit pattern belongs to A64's 256-value immediate set.
    pub(crate) fn fmov_d_imm(&mut self, destination: DReg, value: f64) -> bool {
        let Some(immediate) = (0_u16..=u16::from(u8::MAX))
            .map(|value| value as u8)
            .find(|immediate| expand_fmov_imm8(*immediate) == value.to_bits())
        else {
            return false;
        };
        self.emit(0x1E60_1000 | (u32::from(immediate) << 13) | dreg(destination));
        true
    }

    pub(crate) fn fmov_d_positive_zero(&mut self, destination: DReg) {
        // Register encoding 31 denotes XZR for FMOV (general), even though the
        // same architectural field denotes SP in add/sub addressing forms.
        self.emit(0x9E67_0000 | (31 << 5) | dreg(destination));
    }

    pub(crate) fn ldr_d_literal_placeholder(&mut self, destination: DReg) -> LiteralPatch {
        let patch = LiteralPatch {
            instruction_offset: self.position(),
        };
        self.emit(0x5C00_0000 | dreg(destination));
        patch
    }

    pub(crate) fn patch_ldr_d_literal(
        &mut self,
        patch: LiteralPatch,
        target: usize,
    ) -> JitResult<()> {
        if target % 4 != 0 {
            return Err(encoding_error(format!(
                "A64 LDR literal target byte {target} is not word aligned"
            )));
        }
        let delta = i64::try_from(target)
            .ok()
            .and_then(|target| {
                i64::try_from(patch.instruction_offset)
                    .ok()
                    .and_then(|source| target.checked_sub(source))
            })
            .ok_or_else(|| encoding_error("A64 LDR literal displacement overflow"))?;
        if delta % 4 != 0 {
            return Err(encoding_error(
                "A64 LDR literal displacement is not word aligned",
            ));
        }
        let immediate = encode_signed(delta / 4, 19, "A64 LDR literal displacement")?;
        let instruction = self.read_word(patch.instruction_offset)?;
        self.write_word(
            patch.instruction_offset,
            (instruction & 0xFF00_001F) | (immediate << 5),
        )
    }

    pub(crate) fn append_u64_data(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(crate) fn fmov_d_x(&mut self, destination: DReg, source: XReg) -> JitResult<()> {
        require_general_register(source, "FMOV integer source")?;
        self.emit(0x9E67_0000 | (reg(source) << 5) | dreg(destination));
        Ok(())
    }

    pub(crate) fn fmov_x_d(&mut self, destination: XReg, source: DReg) -> JitResult<()> {
        require_general_register(destination, "FMOV integer destination")?;
        self.emit(0x9E66_0000 | (dreg(source) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn fadd_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_2800, destination, left, right);
    }

    pub(crate) fn fsub_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_3800, destination, left, right);
    }

    pub(crate) fn fmul_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_0800, destination, left, right);
    }

    pub(crate) fn fdiv_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_1800, destination, left, right);
    }

    pub(crate) fn fmax_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_4800, destination, left, right);
    }

    pub(crate) fn fmin_d(&mut self, destination: DReg, left: DReg, right: DReg) {
        self.fp_binary(0x1E60_5800, destination, left, right);
    }

    fn fp_binary(&mut self, opcode: u32, destination: DReg, left: DReg, right: DReg) {
        self.emit(opcode | (dreg(right) << 16) | (dreg(left) << 5) | dreg(destination));
    }

    pub(crate) fn fsqrt_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E61_C000 | (dreg(source) << 5) | dreg(destination));
    }

    pub(crate) fn fabs_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E60_C000 | (dreg(source) << 5) | dreg(destination));
    }

    pub(crate) fn fneg_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E61_4000 | (dreg(source) << 5) | dreg(destination));
    }

    pub(crate) fn frintm_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E65_4000 | (dreg(source) << 5) | dreg(destination));
    }

    pub(crate) fn frintp_d(&mut self, destination: DReg, source: DReg) {
        self.emit(0x1E64_C000 | (dreg(source) << 5) | dreg(destination));
    }

    pub(crate) fn scvtf_d_x(&mut self, destination: DReg, source: XReg) -> JitResult<()> {
        require_general_register(source, "SCVTF source")?;
        self.emit(0x9E62_0000 | (reg(source) << 5) | dreg(destination));
        Ok(())
    }

    pub(crate) fn fcvtzs_x_d(&mut self, destination: XReg, source: DReg) -> JitResult<()> {
        require_general_register(destination, "FCVTZS destination")?;
        self.emit(0x9E78_0000 | (dreg(source) << 5) | reg(destination));
        Ok(())
    }

    pub(crate) fn cmp_x(&mut self, left: XReg, right: XReg) -> JitResult<()> {
        require_general_register(left, "CMP left operand")?;
        require_general_register(right, "CMP right operand")?;
        self.emit(0xEB00_001F | (reg(right) << 16) | (reg(left) << 5));
        Ok(())
    }

    pub(crate) fn cmp_x_imm(&mut self, source: XReg, immediate: u16) -> JitResult<()> {
        require_general_register(source, "CMP source")?;
        if immediate > 4095 {
            return Err(encoding_error(format!(
                "A64 CMP immediate {immediate} exceeds imm12"
            )));
        }
        self.emit(0xF100_001F | (u32::from(immediate) << 10) | (reg(source) << 5));
        Ok(())
    }

    pub(crate) fn fcmp_d(&mut self, left: DReg, right: DReg) {
        self.emit(0x1E60_2000 | (dreg(right) << 16) | (dreg(left) << 5));
    }

    pub(crate) fn fcmp_d_zero(&mut self, source: DReg) {
        self.emit(0x1E60_2008 | (dreg(source) << 5));
    }

    pub(crate) fn cset_x(&mut self, destination: XReg, condition: Condition) -> JitResult<()> {
        require_general_register(destination, "CSET destination")?;
        let inverse = (condition as u8) ^ 1;
        self.emit(0x9A9F_07E0 | (u32::from(inverse) << 12) | reg(destination));
        Ok(())
    }

    pub(crate) fn fcsel_d(
        &mut self,
        destination: DReg,
        when_true: DReg,
        when_false: DReg,
        condition: Condition,
    ) {
        self.emit(
            0x1E60_0C00
                | (dreg(when_false) << 16)
                | (u32::from(condition as u8) << 12)
                | (dreg(when_true) << 5)
                | dreg(destination),
        );
    }

    pub(crate) fn b_placeholder(&mut self) -> BranchPatch {
        self.branch_placeholder(0x1400_0000, BranchKind::Unconditional)
    }

    #[cfg(test)]
    pub(crate) fn bl_placeholder(&mut self) -> BranchPatch {
        self.branch_placeholder(0x9400_0000, BranchKind::Link)
    }

    pub(crate) fn b_cond_placeholder(&mut self, condition: Condition) -> BranchPatch {
        // AArch64 conditional branches reach only +/-1 MiB. Reserve a
        // fixed-size long form: branch on the inverse condition over an
        // unconditional B, whose imm26 reaches +/-128 MiB. Fixed reservation
        // keeps every later label and patch offset stable.
        let inverse = u32::from((condition as u8) ^ 1);
        self.emit(0x5400_0000 | (2 << 5) | inverse);
        self.b_placeholder()
    }

    pub(crate) fn cbz_placeholder(&mut self, register: XReg) -> JitResult<BranchPatch> {
        self.long_compare_branch_placeholder(0xB500_0000, register)
    }

    pub(crate) fn cbnz_placeholder(&mut self, register: XReg) -> JitResult<BranchPatch> {
        self.long_compare_branch_placeholder(0xB400_0000, register)
    }

    fn branch_placeholder(&mut self, opcode: u32, kind: BranchKind) -> BranchPatch {
        let instruction_offset = self.position();
        self.emit(opcode);
        BranchPatch {
            instruction_offset,
            kind,
        }
    }

    fn long_compare_branch_placeholder(
        &mut self,
        inverse_opcode: u32,
        register: XReg,
    ) -> JitResult<BranchPatch> {
        require_general_register(register, "compare-and-branch register")?;
        self.emit(inverse_opcode | (2 << 5) | reg(register));
        Ok(self.b_placeholder())
    }

    pub(crate) fn patch_branch(&mut self, patch: BranchPatch, target: usize) -> JitResult<()> {
        if target % 4 != 0 || patch.instruction_offset % 4 != 0 {
            return Err(encoding_error(format!(
                "A64 branch from byte {} to byte {target} is not instruction aligned",
                patch.instruction_offset
            )));
        }
        let delta = i64::try_from(target)
            .ok()
            .and_then(|target| {
                i64::try_from(patch.instruction_offset)
                    .ok()
                    .and_then(|source| target.checked_sub(source))
            })
            .ok_or_else(|| encoding_error("A64 branch displacement overflow"))?;
        self.patch_branch_delta(patch, delta)
    }

    #[cfg(test)]
    pub(crate) fn patch_branch_to_image_offset(
        &mut self,
        patch: BranchPatch,
        function_image_offset: usize,
        target_image_offset: usize,
    ) -> JitResult<()> {
        if function_image_offset % 4 != 0 || target_image_offset % 4 != 0 {
            return Err(encoding_error(format!(
                "A64 image branch from function byte {function_image_offset} to image byte {target_image_offset} is not instruction aligned"
            )));
        }
        let source = function_image_offset
            .checked_add(patch.instruction_offset)
            .ok_or_else(|| encoding_error("A64 image branch source overflow"))?;
        let delta = i64::try_from(target_image_offset)
            .ok()
            .and_then(|target| {
                i64::try_from(source)
                    .ok()
                    .and_then(|source| target.checked_sub(source))
            })
            .ok_or_else(|| encoding_error("A64 image branch displacement overflow"))?;
        self.patch_branch_delta(patch, delta)
    }

    fn patch_branch_delta(&mut self, patch: BranchPatch, delta: i64) -> JitResult<()> {
        if delta % 4 != 0 {
            return Err(encoding_error(format!(
                "A64 branch displacement {delta} is not instruction aligned"
            )));
        }
        let word_delta = delta / 4;
        let instruction = self.read_word(patch.instruction_offset)?;
        let patched = match patch.kind {
            BranchKind::Unconditional => {
                let immediate = encode_signed(word_delta, 26, "A64 B/BL displacement")?;
                (instruction & 0xFC00_0000) | immediate
            }
            #[cfg(test)]
            BranchKind::Link => {
                let immediate = encode_signed(word_delta, 26, "A64 B/BL displacement")?;
                (instruction & 0xFC00_0000) | immediate
            }
        };
        self.write_word(patch.instruction_offset, patched)
    }

    fn emit(&mut self, instruction: u32) {
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
    }

    fn read_word(&self, offset: usize) -> JitResult<u32> {
        let bytes = self
            .bytes
            .get(offset..offset.saturating_add(4))
            .ok_or_else(|| encoding_error(format!("A64 patch byte {offset} is outside image")))?;
        Ok(u32::from_le_bytes(
            bytes
                .try_into()
                .expect("checked four-byte instruction slice"),
        ))
    }

    fn write_word(&mut self, offset: usize, instruction: u32) -> JitResult<()> {
        let bytes = self
            .bytes
            .get_mut(offset..offset.saturating_add(4))
            .ok_or_else(|| encoding_error(format!("A64 patch byte {offset} is outside image")))?;
        bytes.copy_from_slice(&instruction.to_le_bytes());
        Ok(())
    }
}

fn expand_fmov_imm8(immediate: u8) -> u64 {
    let sign = u64::from(immediate >> 7) << 63;
    let exponent_selector = u64::from((immediate >> 6) & 1);
    let exponent = ((exponent_selector ^ 1) << 10)
        | (exponent_selector * 0xff << 2)
        | u64::from((immediate >> 4) & 0x3);
    let fraction = u64::from(immediate & 0xf) << 48;
    sign | (exponent << 52) | fraction
}

fn reg(register: XReg) -> u32 {
    u32::from(register as u8)
}

fn dreg(register: DReg) -> u32 {
    u32::from(register as u8)
}

fn require_general_register(register: XReg, role: &str) -> JitResult<()> {
    if register == XReg::Sp {
        return Err(encoding_error(format!(
            "{role} cannot use register 31/SP in this encoding"
        )));
    }
    Ok(())
}

fn checked_halfword_shift(shift: u8) -> JitResult<u8> {
    match shift {
        0 | 16 | 32 | 48 => Ok(shift / 16),
        _ => Err(encoding_error(format!(
            "A64 wide-immediate shift {shift} is not one of 0, 16, 32, or 48"
        ))),
    }
}

fn encode_signed(value: i64, bits: u8, role: &str) -> JitResult<u32> {
    let minimum = -(1_i64 << (bits - 1));
    let maximum = (1_i64 << (bits - 1)) - 1;
    if !(minimum..=maximum).contains(&value) {
        return Err(encoding_error(format!(
            "{role} {value} is outside signed {bits}-bit range"
        )));
    }
    Ok((value as u64 & ((1_u64 << bits) - 1)) as u32)
}

fn encoding_error(detail: impl Into<String>) -> JitError {
    JitError::Encoding {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{A64Encoder, Condition, DReg, XReg};

    fn words(bytes: &[u8]) -> Vec<u32> {
        bytes
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes(word.try_into().expect("four-byte word")))
            .collect()
    }

    #[test]
    fn encodes_known_a64_instruction_words() {
        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        encoder
            .movz_x(XReg::X0, 42, 0)
            .expect("encode MOVZ immediate");
        encoder
            .stp_x_pre(XReg::X29, XReg::X30, XReg::Sp, -16)
            .expect("encode frame push");
        encoder
            .ldp_x_post(XReg::X29, XReg::X30, XReg::Sp, 16)
            .expect("encode frame pop");
        encoder.ret();

        assert_eq!(
            words(&encoder.into_bytes()),
            [
                0xD503_245F,
                0xD280_0540,
                0xA9BF_7BFD,
                0xA8C1_7BFD,
                0xD65F_03C0
            ]
        );
    }

    #[test]
    fn encodes_exact_scalar_floating_immediates_and_positive_zero() {
        let mut encoder = A64Encoder::new();
        assert!(encoder.fmov_d_imm(DReg::D0, 1.0));
        assert!(encoder.fmov_d_imm(DReg::D3, -2.0));
        assert!(!encoder.fmov_d_imm(DReg::D4, std::f64::consts::PI));
        encoder.fmov_d_positive_zero(DReg::D5);
        assert_eq!(
            words(&encoder.into_bytes()),
            [0x1E6E_1000, 0x1E70_1003, 0x9E67_03E5]
        );
    }

    #[test]
    fn encodes_shifted_page_add_for_compact_large_offsets() {
        let mut encoder = A64Encoder::new();
        encoder
            .add_x_imm_shift12(XReg::X0, XReg::X1, 3)
            .expect("shifted page add");
        assert_eq!(words(&encoder.into_bytes()), [0x9140_0C20]);
    }

    #[test]
    fn encodes_zero_compare_and_condition_materialization() {
        let mut encoder = A64Encoder::new();
        encoder.fcmp_d_zero(DReg::D3);
        encoder
            .cset_x(XReg::X9, Condition::NotEqual)
            .expect("CSET NE");
        encoder
            .cset_x(XReg::X10, Condition::Equal)
            .expect("CSET EQ");
        assert_eq!(
            words(&encoder.into_bytes()),
            [0x1E60_2068, 0x9A9F_07E9, 0x9A9F_17EA]
        );
    }

    #[test]
    fn patches_forward_and_backward_branches() {
        let mut encoder = A64Encoder::new();
        let forward = encoder.b_cond_placeholder(Condition::Equal);
        encoder.nop();
        encoder
            .patch_branch(forward, encoder.position())
            .expect("patch forward");
        let backward = encoder.b_placeholder();
        encoder.patch_branch(backward, 0).expect("patch backward");

        assert_eq!(
            words(&encoder.into_bytes()),
            [0x5400_0041, 0x1400_0002, 0xD503_201F, 0x17FF_FFFD]
        );
    }

    #[test]
    fn long_conditional_branch_reaches_beyond_imm19() {
        let mut encoder = A64Encoder::new();
        let branch = encoder.b_cond_placeholder(Condition::Equal);
        for _ in 0..262_144 {
            encoder.nop();
        }
        let target = encoder.position();
        encoder
            .patch_branch(branch, target)
            .expect("long conditional must use imm26 reach");
        let encoded = words(&encoder.into_bytes());
        assert_eq!(encoded[0], 0x5400_0041);
        assert_eq!(encoded[1], 0x1404_0001);
    }

    #[test]
    fn patches_checked_image_relative_calls() {
        let mut encoder = A64Encoder::new();
        let call = encoder.bl_placeholder();
        encoder
            .patch_branch_to_image_offset(call, 0x100, 0x80)
            .expect("patch backward image call");
        assert_eq!(words(&encoder.into_bytes()), [0x97FF_FFE0]);

        let mut out_of_range = A64Encoder::new();
        let call = out_of_range.bl_placeholder();
        assert!(
            out_of_range
                .patch_branch_to_image_offset(call, 0, 128 * 1024 * 1024)
                .is_err()
        );
    }

    #[test]
    fn image_calls_reach_the_full_executable_image_limit() {
        let function_offset = 0x1000;
        let target_offset = 0x0ff0_4320;
        let mut encoder = A64Encoder::new();
        encoder
            .image_call(function_offset, target_offset, XReg::X16)
            .expect("encode far image call");
        let encoded = words(&encoder.into_bytes());
        assert_eq!(encoded.len(), 3);
        assert_eq!(encoded[1], 0x910C_8210);
        assert_eq!(encoded[2], 0xD63F_0200);
    }

    #[test]
    fn rejects_truncated_wide_immediates_and_unaligned_offsets() {
        let mut encoder = A64Encoder::new();
        assert!(encoder.movz_x(XReg::X0, 1, 8).is_err());
        assert!(encoder.ldr_x_unsigned(XReg::X0, XReg::X1, 3).is_err());
        assert!(encoder.add_x_imm(XReg::X0, XReg::X1, 4096).is_err());
    }

    #[test]
    fn encodes_scalar_floating_point_subset() {
        let mut encoder = A64Encoder::new();
        encoder.fadd_d(DReg::D0, DReg::D0, DReg::D1);
        encoder.fsqrt_d(DReg::D2, DReg::D3);
        encoder.fcmp_d(DReg::D4, DReg::D5);
        encoder.fcsel_d(DReg::D6, DReg::D7, DReg::D8, Condition::SignedGreater);
        encoder.frintm_d(DReg::D9, DReg::D10);
        encoder.frintp_d(DReg::D11, DReg::D12);
        encoder
            .ldrb_w_unsigned(XReg::X2, XReg::X3, 5)
            .expect("encode LDRB");
        encoder
            .cmp_x(XReg::X4, XReg::X5)
            .expect("encode register CMP");
        encoder
            .cmp_x_imm(XReg::X6, 7)
            .expect("encode immediate CMP");
        encoder.scvtf_d_x(DReg::D7, XReg::X8).expect("encode SCVTF");
        encoder
            .and_x(XReg::X0, XReg::X1, XReg::X2)
            .expect("encode AND");
        encoder
            .orr_x(XReg::X3, XReg::X4, XReg::X5)
            .expect("encode ORR");
        encoder
            .eor_x(XReg::X6, XReg::X7, XReg::X8)
            .expect("encode EOR");
        encoder
            .lslv_x(XReg::X9, XReg::X10, XReg::X11)
            .expect("encode LSLV");
        encoder
            .asrv_x(XReg::X12, XReg::X13, XReg::X14)
            .expect("encode ASRV");
        encoder
            .fcvtzs_x_d(XReg::X15, DReg::D16)
            .expect("encode FCVTZS");
        encoder.fmax_d(DReg::D0, DReg::D1, DReg::D2);
        encoder.fmin_d(DReg::D3, DReg::D4, DReg::D5);
        encoder
            .strb_w_unsigned(XReg::X6, XReg::X7, 8)
            .expect("encode STRB");

        assert_eq!(
            words(&encoder.into_bytes()),
            [
                0x1E61_2800,
                0x1E61_C062,
                0x1E65_2080,
                0x1E68_CCE6,
                0x1E65_4149,
                0x1E64_C18B,
                0x3940_1462,
                0xEB05_009F,
                0xF100_1CDF,
                0x9E62_0107,
                0x8A02_0020,
                0xAA05_0083,
                0xCA08_00E6,
                0x9ACB_2149,
                0x9ACE_29AC,
                0x9E78_020F,
                0x1E62_4820,
                0x1E65_5883,
                0x3900_20E6,
            ]
        );
    }

    #[cfg(all(feature = "native", target_arch = "aarch64"))]
    #[test]
    fn emitted_integer_and_floating_abi_entries_execute() {
        use crate::native::runtime::ExecutableMemory;

        let mut integer = A64Encoder::new();
        integer
            .add_x(XReg::X0, XReg::X0, XReg::X1)
            .expect("encode integer add");
        integer.ret();
        let memory =
            ExecutableMemory::allocate(&integer.into_bytes()).expect("publish A64 integer entry");
        let entry: extern "C" fn(u64, u64) -> u64 =
            unsafe { std::mem::transmute(memory.ptr_at(0).expect("integer entry pointer")) };
        assert_eq!(entry(19, 23), 42);

        let mut floating = A64Encoder::new();
        floating.fadd_d(DReg::D0, DReg::D0, DReg::D1);
        floating.ret();
        let memory =
            ExecutableMemory::allocate(&floating.into_bytes()).expect("publish A64 floating entry");
        let entry: extern "C" fn(f64, f64) -> f64 =
            unsafe { std::mem::transmute(memory.ptr_at(0).expect("floating entry pointer")) };
        assert_eq!(entry(19.5, 22.5), 42.0);
    }
}
