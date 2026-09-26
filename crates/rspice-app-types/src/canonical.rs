//! Versioned canonical encoding for content identities.
//!
//! This encoding is deliberately independent of `Debug`, serde formats, map
//! iteration order, and host word size. Every aggregate starts with a domain
//! separator; variable-length values carry a big-endian `u64` length.

use crate::product::ContentDigest;
use sha2::{Digest as _, Sha256};

const CANONICAL_MAGIC: &[u8] = b"RSPICE-CANONICAL";
const CANONICAL_VERSION: u16 = 1;

pub struct CanonicalWriter {
    hasher: Sha256,
}

impl CanonicalWriter {
    pub fn new(domain: &str) -> Self {
        let mut writer = Self {
            hasher: Sha256::new(),
        };
        writer.raw(CANONICAL_MAGIC);
        writer.raw(&CANONICAL_VERSION.to_be_bytes());
        writer.domain(domain);
        writer
    }

    pub fn domain(&mut self, value: &str) {
        self.raw(&[0xd0]);
        self.length(value.len());
        self.raw(value.as_bytes());
    }

    pub fn bool(&mut self, value: bool) {
        self.raw(&[0x01, u8::from(value)]);
    }

    pub fn u8(&mut self, value: u8) {
        self.raw(&[0x02, value]);
    }

    pub fn u64(&mut self, value: u64) {
        self.raw(&[0x03]);
        self.raw(&value.to_be_bytes());
    }

    pub fn usize(&mut self, value: usize) {
        self.u64(u64::try_from(value).expect("supported Rust targets use at most 64-bit usize"));
    }

    pub fn i32(&mut self, value: i32) {
        self.raw(&[0x04]);
        self.raw(&value.to_be_bytes());
    }

    pub fn f64(&mut self, value: f64) {
        self.raw(&[0x06]);
        self.raw(&value.to_bits().to_be_bytes());
    }

    pub fn string(&mut self, value: &str) {
        self.raw(&[0x07]);
        self.length(value.len());
        self.raw(value.as_bytes());
    }

    pub fn bytes(&mut self, value: &[u8]) {
        self.raw(&[0x08]);
        self.length(value.len());
        self.raw(value);
    }

    pub fn digest(&mut self, value: ContentDigest) {
        self.raw(&[0x09]);
        self.raw(value.as_bytes());
    }

    pub fn uuid(&mut self, value: uuid::Uuid) {
        self.raw(&[0x0c]);
        self.raw(value.as_bytes());
    }

    pub fn sequence(&mut self, len: usize) {
        self.raw(&[0x0a]);
        self.length(len);
    }

    pub fn option<T: ?Sized>(&mut self, value: Option<&T>, encode: impl FnOnce(&mut Self, &T)) {
        match value {
            Some(value) => {
                self.raw(&[0x0b, 1]);
                encode(self, value);
            }
            None => self.raw(&[0x0b, 0]),
        }
    }

    pub fn finish(self) -> ContentDigest {
        ContentDigest::from_bytes(self.hasher.finalize().into())
    }

    fn length(&mut self, len: usize) {
        self.raw(
            &u64::try_from(len)
                .expect("supported Rust targets use at most 64-bit usize")
                .to_be_bytes(),
        );
    }

    fn raw(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}

pub fn content_digest(domain: &str, bytes: &[u8]) -> ContentDigest {
    let mut writer = CanonicalWriter::new(domain);
    writer.bytes(bytes);
    writer.finish()
}
