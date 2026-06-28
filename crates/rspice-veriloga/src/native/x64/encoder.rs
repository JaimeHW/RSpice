#[derive(Debug, Default, Clone)]
pub struct X64Encoder {
    bytes: Vec<u8>,
}

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

    pub fn mov_eax_imm32(&mut self, value: u32) {
        self.emit_u8(0xB8);
        self.emit_all(&value.to_le_bytes());
    }

    pub fn addsd_xmm0_xmm1(&mut self) {
        self.emit_all(&[0xF2, 0x0F, 0x58, 0xC1]);
    }

    pub fn subsd_xmm0_xmm1(&mut self) {
        self.emit_all(&[0xF2, 0x0F, 0x5C, 0xC1]);
    }

    pub fn mulsd_xmm0_xmm1(&mut self) {
        self.emit_all(&[0xF2, 0x0F, 0x59, 0xC1]);
    }

    pub fn divsd_xmm0_xmm1(&mut self) {
        self.emit_all(&[0xF2, 0x0F, 0x5E, 0xC1]);
    }

    pub fn ret(&mut self) {
        self.emit_u8(0xC3);
    }
}

#[cfg(test)]
mod tests {
    use super::X64Encoder;

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
                0xF2, 0x0F, 0x58, 0xC1, 0xF2, 0x0F, 0x5C, 0xC1, 0xF2, 0x0F, 0x59, 0xC1,
                0xF2, 0x0F, 0x5E, 0xC1, 0xC3,
            ]
        );
    }
}
