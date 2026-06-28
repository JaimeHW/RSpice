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
}
