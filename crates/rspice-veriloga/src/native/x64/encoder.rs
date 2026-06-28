#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Gpr {
    Rax,
    Rcx,
    Rdx,
    Rsp,
    Rdi,
    Rsi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
}

#[allow(dead_code)]
impl Gpr {
    fn code(self) -> u8 {
        match self {
            Self::Rax => 0,
            Self::Rcx => 1,
            Self::Rdx => 2,
            Self::Rsp => 4,
            Self::Rsi => 6,
            Self::Rdi => 7,
            Self::R8 => 8,
            Self::R9 => 9,
            Self::R10 => 10,
            Self::R11 => 11,
            Self::R12 => 12,
            Self::R13 => 13,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Xmm {
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConditionCode {
    Above,
    AboveOrEqual,
    Below,
    BelowOrEqual,
    Equal,
    NotParity,
    Parity,
}

impl Xmm {
    fn code(self) -> u8 {
        match self {
            Self::Xmm0 => 0,
            Self::Xmm1 => 1,
            Self::Xmm2 => 2,
            Self::Xmm3 => 3,
            Self::Xmm4 => 4,
            Self::Xmm5 => 5,
        }
    }
}

impl ConditionCode {
    fn set_opcode(self) -> u8 {
        match self {
            Self::Above => 0x97,
            Self::AboveOrEqual => 0x93,
            Self::Below => 0x92,
            Self::BelowOrEqual => 0x96,
            Self::Equal => 0x94,
            Self::NotParity => 0x9B,
            Self::Parity => 0x9A,
        }
    }

    fn jump_opcode(self) -> u8 {
        match self {
            Self::Above => 0x87,
            Self::AboveOrEqual => 0x83,
            Self::Below => 0x82,
            Self::BelowOrEqual => 0x86,
            Self::Equal => 0x84,
            Self::NotParity => 0x8B,
            Self::Parity => 0x8A,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct X64Encoder {
    bytes: Vec<u8>,
}

#[allow(dead_code)]
impl X64Encoder {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn emit_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn emit_all(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub(crate) fn position(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn patch_i32(&mut self, offset: usize, value: i32) {
        self.bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub fn mov_eax_imm32(&mut self, value: u32) {
        self.emit_u8(0xB8);
        self.emit_all(&value.to_le_bytes());
    }

    pub(crate) fn push_r64(&mut self, reg: Gpr) {
        self.emit_rex(false, 0, 0, reg.code());
        self.emit_u8(0x50 + (reg.code() & 0b111));
    }

    pub(crate) fn pop_r64(&mut self, reg: Gpr) {
        self.emit_rex(false, 0, 0, reg.code());
        self.emit_u8(0x58 + (reg.code() & 0b111));
    }

    pub(crate) fn mov_r64_r64(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex(true, src.code(), 0, dst.code());
        self.emit_u8(0x89);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn movabs_r64_imm64(&mut self, dst: Gpr, value: u64) {
        self.emit_rex(true, 0, 0, dst.code());
        self.emit_u8(0xB8 + (dst.code() & 0b111));
        self.emit_all(&value.to_le_bytes());
    }

    pub(crate) fn call_r64(&mut self, target: Gpr) {
        self.emit_rex(false, 0, 0, target.code());
        self.emit_u8(0xFF);
        self.emit_modrm(0b11, 0b010, target.code());
    }

    pub(crate) fn sub_rsp_imm32(&mut self, value: i32) {
        self.emit_all(&[0x48, 0x81, 0xEC]);
        self.emit_i32(value);
    }

    pub(crate) fn add_rsp_imm32(&mut self, value: i32) {
        self.emit_all(&[0x48, 0x81, 0xC4]);
        self.emit_i32(value);
    }

    pub(crate) fn add_r64_imm32(&mut self, reg: Gpr, value: i32) {
        self.emit_rex(true, 0, 0, reg.code());
        self.emit_u8(0x81);
        self.emit_modrm(0b11, 0b000, reg.code());
        self.emit_i32(value);
    }

    pub(crate) fn mov_r64_m64_base_disp32(&mut self, dst: Gpr, base: Gpr, disp: i32) {
        self.emit_rex(true, dst.code(), 0, base.code());
        self.emit_u8(0x8B);
        self.emit_base_disp32_modrm(dst.code(), base.code(), disp);
    }

    pub(crate) fn mov_m64_base_disp32_r64(&mut self, base: Gpr, disp: i32, src: Gpr) {
        self.emit_rex(true, src.code(), 0, base.code());
        self.emit_u8(0x89);
        self.emit_base_disp32_modrm(src.code(), base.code(), disp);
    }

    pub(crate) fn movsd_xmm_m64_base_disp32(&mut self, dst: Xmm, base: Gpr, disp: i32) {
        self.emit_u8(0xF2);
        self.emit_rex(false, dst.code(), 0, base.code());
        self.emit_all(&[0x0F, 0x10]);
        self.emit_base_disp32_modrm(dst.code(), base.code(), disp);
    }

    pub(crate) fn movzx_r32_m8_base_disp32(&mut self, dst: Gpr, base: Gpr, disp: i32) {
        self.emit_rex(false, dst.code(), 0, base.code());
        self.emit_all(&[0x0F, 0xB6]);
        self.emit_base_disp32_modrm(dst.code(), base.code(), disp);
    }

    pub(crate) fn movzx_r32_r8(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex_for_byte_operands(false, dst.code(), 0, src.code(), false, true);
        self.emit_all(&[0x0F, 0xB6]);
        self.emit_modrm(0b11, dst.code(), src.code());
    }

    pub(crate) fn cvtsi2sd_xmm_r32(&mut self, dst: Xmm, src: Gpr) {
        self.emit_u8(0xF2);
        self.emit_rex(false, dst.code(), 0, src.code());
        self.emit_all(&[0x0F, 0x2A]);
        self.emit_modrm(0b11, dst.code(), src.code());
    }

    pub(crate) fn cmp_r32_imm8(&mut self, reg: Gpr, value: u8) {
        self.emit_rex(false, 0, 0, reg.code());
        self.emit_u8(0x83);
        self.emit_modrm(0b11, 0b111, reg.code());
        self.emit_u8(value);
    }

    pub(crate) fn cmp_r64_imm32(&mut self, reg: Gpr, value: i32) {
        self.emit_rex(true, 0, 0, reg.code());
        self.emit_u8(0x81);
        self.emit_modrm(0b11, 0b111, reg.code());
        self.emit_i32(value);
    }

    pub(crate) fn movsd_m64_base_disp32_xmm(&mut self, base: Gpr, disp: i32, src: Xmm) {
        self.emit_u8(0xF2);
        self.emit_rex(false, src.code(), 0, base.code());
        self.emit_all(&[0x0F, 0x11]);
        self.emit_base_disp32_modrm(src.code(), base.code(), disp);
    }

    pub(crate) fn mov_m8_base_disp32_imm8(&mut self, base: Gpr, disp: i32, value: u8) {
        self.emit_rex_for_byte_operands(false, 0, 0, base.code(), false, true);
        self.emit_u8(0xC6);
        self.emit_base_disp32_modrm(0, base.code(), disp);
        self.emit_u8(value);
    }

    pub(crate) fn movsd_xmm_m64_rip_disp32(&mut self, dst: Xmm, disp: i32) -> usize {
        self.emit_u8(0xF2);
        self.emit_rex(false, dst.code(), 0, 0);
        self.emit_all(&[0x0F, 0x10]);
        self.emit_modrm(0b00, dst.code(), 0b101);
        let offset = self.position();
        self.emit_i32(disp);
        offset
    }

    pub(crate) fn movsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x10, dst, src);
    }

    pub(crate) fn xorpd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0x66, 0x57, dst, src);
    }

    pub(crate) fn movq_r64_xmm(&mut self, dst: Gpr, src: Xmm) {
        self.emit_u8(0x66);
        self.emit_rex(true, src.code(), 0, dst.code());
        self.emit_all(&[0x0F, 0x7E]);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn movq_xmm_r64(&mut self, dst: Xmm, src: Gpr) {
        self.emit_u8(0x66);
        self.emit_rex(true, dst.code(), 0, src.code());
        self.emit_all(&[0x0F, 0x6E]);
        self.emit_modrm(0b11, dst.code(), src.code());
    }

    pub(crate) fn btr_r64_imm8(&mut self, reg: Gpr, bit: u8) {
        self.emit_rex(true, 0, 0, reg.code());
        self.emit_all(&[0x0F, 0xBA]);
        self.emit_modrm(0b11, 0b110, reg.code());
        self.emit_u8(bit);
    }

    pub(crate) fn btc_r64_imm8(&mut self, reg: Gpr, bit: u8) {
        self.emit_rex(true, 0, 0, reg.code());
        self.emit_all(&[0x0F, 0xBA]);
        self.emit_modrm(0b11, 0b111, reg.code());
        self.emit_u8(bit);
    }

    pub(crate) fn sqrtsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x51, dst, src);
    }

    pub(crate) fn ucomisd_xmm_xmm(&mut self, left: Xmm, right: Xmm) {
        self.emit_sse_reg_reg(0x66, 0x2E, left, right);
    }

    pub(crate) fn ucomisd_xmm_m64_rip_disp32(&mut self, left: Xmm, disp: i32) -> usize {
        self.emit_sse_reg_rip_disp32(0x66, 0x2E, left, disp)
    }

    pub(crate) fn setcc_r8(&mut self, condition: ConditionCode, dst: Gpr) {
        self.emit_rex_for_byte_operands(false, 0, 0, dst.code(), false, true);
        self.emit_all(&[0x0F, condition.set_opcode()]);
        self.emit_modrm(0b11, 0, dst.code());
    }

    pub(crate) fn and_r8_r8(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex_for_byte_operands(false, src.code(), 0, dst.code(), true, true);
        self.emit_u8(0x20);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn or_r8_r8(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex_for_byte_operands(false, src.code(), 0, dst.code(), true, true);
        self.emit_u8(0x08);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn test_r8_r8(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex_for_byte_operands(false, src.code(), 0, dst.code(), true, true);
        self.emit_u8(0x84);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn test_r64_r64(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex(true, src.code(), 0, dst.code());
        self.emit_u8(0x85);
        self.emit_modrm(0b11, src.code(), dst.code());
    }

    pub(crate) fn cmovne_r64_r64(&mut self, dst: Gpr, src: Gpr) {
        self.emit_rex(true, dst.code(), 0, src.code());
        self.emit_all(&[0x0F, 0x45]);
        self.emit_modrm(0b11, dst.code(), src.code());
    }

    pub(crate) fn jcc_rel32_placeholder(&mut self, condition: ConditionCode) -> usize {
        self.emit_all(&[0x0F, condition.jump_opcode()]);
        let offset = self.position();
        self.emit_i32(0);
        offset
    }

    pub(crate) fn jmp_rel32_placeholder(&mut self) -> usize {
        self.emit_u8(0xE9);
        let offset = self.position();
        self.emit_i32(0);
        offset
    }

    pub(crate) fn addsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x58, dst, src);
    }

    pub(crate) fn addsd_xmm_m64_rip_disp32(&mut self, dst: Xmm, disp: i32) -> usize {
        self.emit_sse_reg_rip_disp32(0xF2, 0x58, dst, disp)
    }

    pub(crate) fn addsd_xmm_m64_base_disp32(&mut self, dst: Xmm, base: Gpr, disp: i32) {
        self.emit_sse_reg_base_disp32(0xF2, 0x58, dst, base, disp);
    }

    pub fn addsd_xmm0_xmm1(&mut self) {
        self.addsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    }

    pub(crate) fn subsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x5C, dst, src);
    }

    pub(crate) fn subsd_xmm_m64_rip_disp32(&mut self, dst: Xmm, disp: i32) -> usize {
        self.emit_sse_reg_rip_disp32(0xF2, 0x5C, dst, disp)
    }

    pub(crate) fn subsd_xmm_m64_base_disp32(&mut self, dst: Xmm, base: Gpr, disp: i32) {
        self.emit_sse_reg_base_disp32(0xF2, 0x5C, dst, base, disp);
    }

    pub fn subsd_xmm0_xmm1(&mut self) {
        self.subsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    }

    pub(crate) fn mulsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x59, dst, src);
    }

    pub(crate) fn mulsd_xmm_m64_rip_disp32(&mut self, dst: Xmm, disp: i32) -> usize {
        self.emit_sse_reg_rip_disp32(0xF2, 0x59, dst, disp)
    }

    pub fn mulsd_xmm0_xmm1(&mut self) {
        self.mulsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    }

    pub(crate) fn divsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x5E, dst, src);
    }

    pub(crate) fn divsd_xmm_m64_rip_disp32(&mut self, dst: Xmm, disp: i32) -> usize {
        self.emit_sse_reg_rip_disp32(0xF2, 0x5E, dst, disp)
    }

    pub fn divsd_xmm0_xmm1(&mut self) {
        self.divsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    }

    pub(crate) fn minsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x5D, dst, src);
    }

    pub(crate) fn minsd_xmm_m64_base_disp32(&mut self, dst: Xmm, base: Gpr, disp: i32) {
        self.emit_sse_reg_base_disp32(0xF2, 0x5D, dst, base, disp);
    }

    pub(crate) fn maxsd_xmm_xmm(&mut self, dst: Xmm, src: Xmm) {
        self.emit_sse_reg_reg(0xF2, 0x5F, dst, src);
    }

    pub fn ret(&mut self) {
        self.emit_u8(0xC3);
    }

    fn emit_sse_reg_reg(&mut self, prefix: u8, opcode: u8, dst: Xmm, src: Xmm) {
        self.emit_u8(prefix);
        self.emit_rex(false, dst.code(), 0, src.code());
        self.emit_all(&[0x0F, opcode]);
        self.emit_modrm(0b11, dst.code(), src.code());
    }

    fn emit_sse_reg_rip_disp32(&mut self, prefix: u8, opcode: u8, dst: Xmm, disp: i32) -> usize {
        self.emit_u8(prefix);
        self.emit_rex(false, dst.code(), 0, 0);
        self.emit_all(&[0x0F, opcode]);
        self.emit_modrm(0b00, dst.code(), 0b101);
        let offset = self.position();
        self.emit_i32(disp);
        offset
    }

    fn emit_sse_reg_base_disp32(&mut self, prefix: u8, opcode: u8, dst: Xmm, base: Gpr, disp: i32) {
        self.emit_u8(prefix);
        self.emit_rex(false, dst.code(), 0, base.code());
        self.emit_all(&[0x0F, opcode]);
        self.emit_base_disp32_modrm(dst.code(), base.code(), disp);
    }

    fn emit_base_disp32_modrm(&mut self, reg: u8, base: u8, disp: i32) {
        self.emit_modrm(0b10, reg, base);
        if needs_sib_base(base) {
            self.emit_sib(0, 0b100, base);
        }
        self.emit_i32(disp);
    }

    fn emit_i32(&mut self, value: i32) {
        self.emit_all(&value.to_le_bytes());
    }

    fn emit_modrm(&mut self, mode: u8, reg: u8, rm: u8) {
        self.emit_u8(((mode & 0b11) << 6) | ((reg & 0b111) << 3) | (rm & 0b111));
    }

    fn emit_sib(&mut self, scale: u8, index: u8, base: u8) {
        self.emit_u8(((scale & 0b11) << 6) | ((index & 0b111) << 3) | (base & 0b111));
    }

    fn emit_rex(&mut self, w: bool, reg: u8, index: u8, base: u8) {
        let rex = 0x40
            | ((w as u8) << 3)
            | (((reg >> 3) & 1) << 2)
            | (((index >> 3) & 1) << 1)
            | ((base >> 3) & 1);
        if rex != 0x40 {
            self.emit_u8(rex);
        }
    }

    fn emit_rex_for_byte_operands(
        &mut self,
        w: bool,
        reg: u8,
        index: u8,
        base: u8,
        reg_is_byte: bool,
        base_is_byte: bool,
    ) {
        let rex = 0x40
            | ((w as u8) << 3)
            | (((reg >> 3) & 1) << 2)
            | (((index >> 3) & 1) << 1)
            | ((base >> 3) & 1);
        let byte_register_requires_rex = (reg_is_byte && requires_low_byte_rex(reg))
            || (base_is_byte && requires_low_byte_rex(base));
        if rex != 0x40 || byte_register_requires_rex {
            self.emit_u8(rex);
        }
    }
}

fn requires_low_byte_rex(reg: u8) -> bool {
    matches!(reg & 0b111, 0b100..=0b111)
}

fn needs_sib_base(base: u8) -> bool {
    base & 0b111 == 0b100
}

#[cfg(test)]
mod tests {
    use super::{ConditionCode, Gpr, X64Encoder, Xmm};

    #[cfg(all(feature = "native", target_arch = "x86_64"))]
    use crate::native::{EvalContext, runtime::ExecutableMemory};

    #[test]
    fn encodes_mov_eax_imm32_ret() {
        let mut encoder = X64Encoder::new();

        encoder.mov_eax_imm32(42);
        encoder.ret();

        assert_eq!(encoder.into_bytes(), [0xB8, 42, 0, 0, 0, 0xC3]);
    }

    #[test]
    fn encodes_scalar_f64_register_ops() {
        let mut encoder = X64Encoder::new();

        encoder.addsd_xmm0_xmm1();
        encoder.subsd_xmm0_xmm1();
        encoder.mulsd_xmm0_xmm1();
        encoder.divsd_xmm0_xmm1();
        encoder.ret();

        assert_eq!(
            encoder.into_bytes(),
            [
                0xF2, 0x0F, 0x58, 0xC1, 0xF2, 0x0F, 0x5C, 0xC1, 0xF2, 0x0F, 0x59, 0xC1, 0xF2, 0x0F,
                0x5E, 0xC1, 0xC3,
            ]
        );
    }

    #[test]
    fn encodes_windows_param_load_leaf() {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rcx, 16);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 24);
        encoder.ret();

        assert_eq!(
            encoder.into_bytes(),
            [
                0x48, 0x8B, 0x81, 16, 0, 0, 0, 0xF2, 0x0F, 0x10, 0x80, 24, 0, 0, 0, 0xC3,
            ]
        );
    }

    #[test]
    fn encodes_system_v_param_load_leaf() {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rdi, 16);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 24);
        encoder.ret();

        assert_eq!(
            encoder.into_bytes(),
            [
                0x48, 0x8B, 0x87, 16, 0, 0, 0, 0xF2, 0x0F, 0x10, 0x80, 24, 0, 0, 0, 0xC3,
            ]
        );
    }

    #[test]
    fn encodes_r64_imm32_adds() {
        let mut encoder = X64Encoder::new();

        encoder.add_r64_imm32(Gpr::Rdx, 24);
        encoder.add_r64_imm32(Gpr::R13, -8);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x48, 0x81, 0xC2, 24, 0, 0, 0, 0x49, 0x81, 0xC5, 0xF8, 0xFF, 0xFF, 0xFF,
            ]
        );
    }

    #[test]
    fn encodes_r12_disp32_memory_base_with_sib() {
        let mut encoder = X64Encoder::new();

        encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::R12, 16);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::R12, 80);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x49, 0x8B, 0x84, 0x24, 16, 0, 0, 0, 0xF2, 0x41, 0x0F, 0x10, 0x84, 0x24, 80, 0, 0,
                0,
            ]
        );
    }

    #[test]
    fn encodes_r64_store_to_disp32_memory_base() {
        let mut encoder = X64Encoder::new();

        encoder.mov_m64_base_disp32_r64(Gpr::Rsp, 0, Gpr::Rax);
        encoder.mov_m64_base_disp32_r64(Gpr::R12, 16, Gpr::R13);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x48, 0x89, 0x84, 0x24, 0, 0, 0, 0, 0x4D, 0x89, 0xAC, 0x24, 16, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn encodes_u8_flag_load_and_f64_conversion() {
        let mut encoder = X64Encoder::new();
        encoder.movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, 8);
        encoder.cmp_r32_imm8(Gpr::R10, 5);
        encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
        encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        encoder.cvtsi2sd_xmm_r32(Xmm::Xmm0, Gpr::R10);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x44, 0x0F, 0xB6, 0x90, 8, 0, 0, 0, 0x41, 0x83, 0xFA, 5, 0x41, 0x0F, 0x94, 0xC2,
                0x45, 0x0F, 0xB6, 0xD2, 0xF2, 0x41, 0x0F, 0x2A, 0xC2,
            ]
        );
    }

    #[test]
    fn encodes_scalar_register_and_memory_ops() {
        let mut encoder = X64Encoder::new();
        encoder.xorpd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm0);
        encoder.movsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
        encoder.addsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
        encoder.subsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
        encoder.mulsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
        encoder.divsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
        encoder.movsd_m64_base_disp32_xmm(Gpr::Rdx, 16, Xmm::Xmm1);
        encoder.ret();

        assert_eq!(
            encoder.into_bytes(),
            [
                0x66, 0x0F, 0x57, 0xC0, 0xF2, 0x0F, 0x10, 0xC8, 0xF2, 0x0F, 0x58, 0xC8, 0xF2, 0x0F,
                0x5C, 0xC8, 0xF2, 0x0F, 0x59, 0xC8, 0xF2, 0x0F, 0x5E, 0xC8, 0xF2, 0x0F, 0x11, 0x8A,
                16, 0, 0, 0, 0xC3,
            ]
        );
    }

    #[test]
    fn encodes_scalar_f64_rip_memory_ops() {
        let mut encoder = X64Encoder::new();

        let mul_disp = encoder.mulsd_xmm_m64_rip_disp32(Xmm::Xmm1, 16);
        let div_disp = encoder.divsd_xmm_m64_rip_disp32(Xmm::Xmm5, -8);

        assert_eq!(mul_disp, 4);
        assert_eq!(div_disp, 12);
        assert_eq!(
            encoder.into_bytes(),
            [
                0xF2, 0x0F, 0x59, 0x0D, 16, 0, 0, 0, 0xF2, 0x0F, 0x5E, 0x2D, 0xF8, 0xFF, 0xFF,
                0xFF,
            ]
        );
    }

    #[test]
    fn encodes_scalar_sqrt_register_op() {
        let mut encoder = X64Encoder::new();

        encoder.sqrtsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm1);

        assert_eq!(encoder.into_bytes(), [0xF2, 0x0F, 0x51, 0xC9]);
    }

    #[test]
    fn encodes_scalar_abs_integer_bit_clear_sequence() {
        let mut encoder = X64Encoder::new();

        encoder.movq_r64_xmm(Gpr::Rax, Xmm::Xmm2);
        encoder.btr_r64_imm8(Gpr::Rax, 63);
        encoder.movq_xmm_r64(Xmm::Xmm2, Gpr::Rax);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x66, 0x48, 0x0F, 0x7E, 0xD0, 0x48, 0x0F, 0xBA, 0xF0, 0x3F, 0x66, 0x48, 0x0F, 0x6E,
                0xD0,
            ]
        );
    }

    #[test]
    fn encodes_ordered_compare_to_f64_sequence() {
        let mut encoder = X64Encoder::new();

        encoder.ucomisd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm2);
        encoder.setcc_r8(ConditionCode::Above, Gpr::R10);
        encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        encoder.cvtsi2sd_xmm_r32(Xmm::Xmm1, Gpr::R10);
        encoder.ucomisd_xmm_xmm(Xmm::Xmm2, Xmm::Xmm1);
        encoder.setcc_r8(ConditionCode::AboveOrEqual, Gpr::R10);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x66, 0x0F, 0x2E, 0xCA, 0x41, 0x0F, 0x97, 0xC2, 0x45, 0x0F, 0xB6, 0xD2, 0xF2, 0x41,
                0x0F, 0x2A, 0xCA, 0x66, 0x0F, 0x2E, 0xD1, 0x41, 0x0F, 0x93, 0xC2,
            ]
        );
    }

    #[test]
    fn encodes_logical_truthiness_sequence_pieces() {
        let mut encoder = X64Encoder::new();

        let disp = encoder.ucomisd_xmm_m64_rip_disp32(Xmm::Xmm1, 16);
        encoder.setcc_r8(ConditionCode::Below, Gpr::R10);
        encoder.setcc_r8(ConditionCode::NotParity, Gpr::R11);
        encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        encoder.or_r8_r8(Gpr::R10, Gpr::R11);

        assert_eq!(disp, 4);
        assert_eq!(
            encoder.into_bytes(),
            [
                0x66, 0x0F, 0x2E, 0x0D, 16, 0, 0, 0, 0x41, 0x0F, 0x92, 0xC2, 0x41, 0x0F, 0x9B,
                0xC3, 0x45, 0x20, 0xDA, 0x45, 0x08, 0xDA,
            ]
        );
    }

    #[test]
    fn encodes_low_byte_registers_that_require_rex_prefix() {
        let mut encoder = X64Encoder::new();

        encoder.setcc_r8(ConditionCode::Above, Gpr::Rsi);
        encoder.setcc_r8(ConditionCode::AboveOrEqual, Gpr::Rdi);
        encoder.and_r8_r8(Gpr::Rsi, Gpr::Rdi);
        encoder.or_r8_r8(Gpr::Rdi, Gpr::Rsi);
        encoder.movzx_r32_r8(Gpr::Rsi, Gpr::Rdi);
        encoder.movzx_r32_r8(Gpr::Rdi, Gpr::Rsi);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x40, 0x0F, 0x97, 0xC6, 0x40, 0x0F, 0x93, 0xC7, 0x40, 0x20, 0xFE, 0x40, 0x08, 0xF7,
                0x40, 0x0F, 0xB6, 0xF7, 0x40, 0x0F, 0xB6, 0xFE,
            ]
        );
    }

    #[test]
    fn encodes_ifelse_select_sequence_pieces() {
        let mut encoder = X64Encoder::new();

        encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        encoder.cmovne_r64_r64(Gpr::Rax, Gpr::R11);

        assert_eq!(
            encoder.into_bytes(),
            [0x45, 0x84, 0xD2, 0x49, 0x0F, 0x45, 0xC3]
        );
    }

    #[test]
    fn encodes_call_preservation_primitives() {
        let mut encoder = X64Encoder::new();

        encoder.push_r64(Gpr::R12);
        encoder.push_r64(Gpr::R13);
        encoder.mov_r64_r64(Gpr::R12, Gpr::Rcx);
        encoder.mov_r64_r64(Gpr::R13, Gpr::Rdx);
        encoder.sub_rsp_imm32(88);
        encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        encoder.movabs_r64_imm64(Gpr::Rax, 0x1122_3344_5566_7788);
        encoder.call_r64(Gpr::Rax);
        encoder.add_rsp_imm32(88);
        encoder.pop_r64(Gpr::R13);
        encoder.pop_r64(Gpr::R12);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x41, 0x54, 0x41, 0x55, 0x49, 0x89, 0xCC, 0x49, 0x89, 0xD5, 0x48, 0x81, 0xEC, 0x58,
                0, 0, 0, 0x49, 0x89, 0xE3, 0x48, 0xB8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
                0x11, 0xFF, 0xD0, 0x48, 0x81, 0xC4, 0x58, 0, 0, 0, 0x41, 0x5D, 0x41, 0x5C,
            ]
        );
    }

    #[test]
    fn encodes_scalar_min_max_fixup_sequence_pieces() {
        let mut encoder = X64Encoder::new();

        encoder.ucomisd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm1);
        encoder.setcc_r8(ConditionCode::NotParity, Gpr::R10);
        encoder.movq_r64_xmm(Gpr::R8, Xmm::Xmm1);
        encoder.btr_r64_imm8(Gpr::R8, 63);
        encoder.test_r64_r64(Gpr::R8, Gpr::R8);
        encoder.setcc_r8(ConditionCode::Equal, Gpr::R8);
        encoder.ucomisd_xmm_xmm(Xmm::Xmm2, Xmm::Xmm2);
        encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        encoder.movq_r64_xmm(Gpr::R9, Xmm::Xmm0);
        encoder.btr_r64_imm8(Gpr::R9, 63);
        encoder.test_r64_r64(Gpr::R9, Gpr::R9);
        encoder.setcc_r8(ConditionCode::Equal, Gpr::R9);
        encoder.and_r8_r8(Gpr::R8, Gpr::R9);
        encoder.or_r8_r8(Gpr::R10, Gpr::R8);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x66, 0x0F, 0x2E, 0xC9, 0x41, 0x0F, 0x9B, 0xC2, 0x66, 0x49, 0x0F, 0x7E, 0xC8, 0x49,
                0x0F, 0xBA, 0xF0, 0x3F, 0x4D, 0x85, 0xC0, 0x41, 0x0F, 0x94, 0xC0, 0x66, 0x0F, 0x2E,
                0xD2, 0x41, 0x0F, 0x9A, 0xC3, 0x45, 0x20, 0xDA, 0x66, 0x49, 0x0F, 0x7E, 0xC1, 0x49,
                0x0F, 0xBA, 0xF1, 0x3F, 0x4D, 0x85, 0xC9, 0x41, 0x0F, 0x94, 0xC1, 0x45, 0x20, 0xC8,
                0x45, 0x08, 0xC2,
            ]
        );
    }

    #[test]
    fn encodes_scalar_min_max_register_ops() {
        let mut encoder = X64Encoder::new();

        encoder.minsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm2);
        encoder.maxsd_xmm_xmm(Xmm::Xmm3, Xmm::Xmm4);

        assert_eq!(
            encoder.into_bytes(),
            [0xF2, 0x0F, 0x5D, 0xCA, 0xF2, 0x0F, 0x5F, 0xDC]
        );
    }

    #[test]
    fn encodes_limit_state_sequence_primitives() {
        let mut encoder = X64Encoder::new();

        encoder.mov_m8_base_disp32_imm8(Gpr::R10, 16, 1);
        encoder.btc_r64_imm8(Gpr::R11, 63);
        encoder.cmp_r64_imm32(Gpr::R11, 1);
        let branch = encoder.jcc_rel32_placeholder(ConditionCode::BelowOrEqual);
        encoder.patch_i32(branch, 0);
        encoder.subsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 24);
        encoder.minsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rsp, 0);
        encoder.addsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 24);

        assert_eq!(
            encoder.into_bytes(),
            [
                0x41, 0xC6, 0x82, 16, 0, 0, 0, 1, 0x49, 0x0F, 0xBA, 0xFB, 63, 0x49, 0x81, 0xFB, 1,
                0, 0, 0, 0x0F, 0x86, 0, 0, 0, 0, 0xF2, 0x0F, 0x5C, 0x88, 24, 0, 0, 0, 0xF2, 0x0F,
                0x5D, 0x8C, 0x24, 0, 0, 0, 0, 0xF2, 0x0F, 0x58, 0x88, 24, 0, 0, 0,
            ]
        );
    }

    #[cfg(all(feature = "native", target_arch = "x86_64"))]
    #[test]
    fn encoded_leaf_loads_and_combines_context_values() {
        let params = [2.0_f64];
        let voltages = [5.0_f64];
        let internals = [1.0_f64];
        let vars = [0.0_f64, 8.0_f64];
        let branch_unknowns = [4.0_f64];
        let ctx = EvalContext {
            voltages: voltages.as_ptr(),
            internal_voltages: internals.as_ptr(),
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
            branch_unknowns: branch_unknowns.as_ptr(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
        };

        let mut encoder = X64Encoder::new();
        let ctx_reg = host_ctx_arg_reg();
        let vars_reg = host_vars_arg_reg();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 0);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 0);
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 8);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 0);
        encoder.subsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 16);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 0);
        encoder.mulsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, vars_reg, 8);
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 176);
        encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm2, Gpr::Rax, 0);
        encoder.divsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm2);
        encoder.addsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
        encoder.ret();

        let memory = ExecutableMemory::allocate(&encoder.into_bytes()).expect("allocate leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        assert_eq!(f(&ctx, vars.as_ptr()), 10.0);
    }

    #[cfg(all(windows, feature = "native", target_arch = "x86_64"))]
    fn host_ctx_arg_reg() -> Gpr {
        Gpr::Rcx
    }

    #[cfg(all(windows, feature = "native", target_arch = "x86_64"))]
    fn host_vars_arg_reg() -> Gpr {
        Gpr::Rdx
    }

    #[cfg(all(not(windows), feature = "native", target_arch = "x86_64"))]
    fn host_ctx_arg_reg() -> Gpr {
        Gpr::Rdi
    }

    #[cfg(all(not(windows), feature = "native", target_arch = "x86_64"))]
    fn host_vars_arg_reg() -> Gpr {
        Gpr::Rsi
    }
}
