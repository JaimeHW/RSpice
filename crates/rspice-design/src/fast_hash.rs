//! A fast hash for the private tables a pass over the drawing builds.
//!
//! The connectivity extraction and the junction rule key their tables by
//! points, coordinates, ids and net numbers the drawing supplies, and each
//! table lives for one pass. The standard hasher's defence against keys
//! chosen to collide buys nothing there, and it costs several times this
//! hash: a large sheet's pass hashes tens of thousands of keys, and every
//! frame of a part drag runs one. This is the multiply-and-rotate hash rustc
//! keeps its own tables with.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

/// A map keyed by what the drawing supplies, hashed with [`FastHasher`].
pub(crate) type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<FastHasher>>;

/// A set of what the drawing supplies, hashed with [`FastHasher`].
pub(crate) type FastSet<T> = HashSet<T, BuildHasherDefault<FastHasher>>;

/// The hash state: each word is added, then the sum multiplied.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct FastHasher {
    hash: u64,
}

/// An odd multiplier that spreads every input bit into the high bits.
const MULTIPLIER: u64 = 0xf135_7aea_2e62_a9c5;

impl FastHasher {
    #[inline]
    fn add(&mut self, word: u64) {
        self.hash = self.hash.wrapping_add(word).wrapping_mul(MULTIPLIER);
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut words = bytes.chunks_exact(8);
        for word in &mut words {
            let mut buffer = [0; 8];
            buffer.copy_from_slice(word);
            self.add(u64::from_le_bytes(buffer));
        }
        let rest = words.remainder();
        if !rest.is_empty() {
            let mut buffer = [0; 8];
            buffer[..rest.len()].copy_from_slice(rest);
            self.add(u64::from_le_bytes(buffer));
        }
    }

    #[inline]
    fn write_u8(&mut self, value: u8) {
        self.add(u64::from(value));
    }

    #[inline]
    fn write_u16(&mut self, value: u16) {
        self.add(u64::from(value));
    }

    #[inline]
    fn write_u32(&mut self, value: u32) {
        self.add(u64::from(value));
    }

    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.add(value);
    }

    #[inline]
    fn write_usize(&mut self, value: usize) {
        self.add(value as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        // The multiply mixes upward, so the low bits, which pick the bucket,
        // are the weakest; the rotation brings the well-mixed high bits down.
        self.hash.rotate_left(26)
    }
}
