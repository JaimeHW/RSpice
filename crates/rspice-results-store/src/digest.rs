//! SHA-256 and the store's logical result digest.
//!
//! The digest is logical: it hashes decoded values in canonical order (each row's x then its value
//! slots, rows in order; then each event signal's changes), after mapping `-0.0` to `+0.0` and every
//! NaN to one canonical NaN, as `rspice-results`' result digest does. It therefore does not change
//! when the codec, the chunk size or the way rows were pushed changes.

use std::fmt;

use crate::format::CANONICAL_NAN_BITS;

/// A SHA-256 digest.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Digest(pub [u8; 32]);

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({self})")
    }
}

/// The bits a value contributes to the digest.
#[inline(always)]
pub(crate) fn canonical_bits(value: f64) -> u64 {
    if value == 0.0 {
        0
    } else if value.is_nan() {
        CANONICAL_NAN_BITS
    } else {
        value.to_bits()
    }
}

/// SHA-256 from the workspace's `sha2`, which uses the CPU's SHA instructions where it has them.
#[derive(Clone, Default)]
pub(crate) struct Sha256(sha2::Sha256);

impl Sha256 {
    pub(crate) fn update(&mut self, bytes: &[u8]) {
        sha2::Digest::update(&mut self.0, bytes);
    }

    pub(crate) fn finish(self) -> Digest {
        Digest(sha2::Digest::finalize(self.0).into())
    }
}

const DIGEST_DOMAIN: &[u8] = b"rspice-results-store logical digest 1";

/// Canonical bytes staged through a fixed buffer, so hashing a chunk allocates nothing.
pub(crate) struct ValueHasher {
    hasher: Sha256,
    buffer: Box<[u8; 8192]>,
    len: usize,
}

impl Default for ValueHasher {
    fn default() -> Self {
        Self {
            hasher: Sha256::default(),
            buffer: Box::new([0; 8192]),
            len: 0,
        }
    }
}

impl ValueHasher {
    #[inline(always)]
    pub(crate) fn bytes(&mut self, bytes: &[u8]) {
        if self.len + bytes.len() > self.buffer.len() {
            self.drain();
            if bytes.len() > self.buffer.len() {
                self.hasher.update(bytes);
                return;
            }
        }
        self.buffer[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
    }

    #[inline(always)]
    pub(crate) fn value(&mut self, value: f64) {
        if self.len + 8 > self.buffer.len() {
            self.drain();
        }
        self.buffer[self.len..self.len + 8].copy_from_slice(&canonical_bits(value).to_le_bytes());
        self.len += 8;
    }

    fn drain(&mut self) {
        self.hasher.update(&self.buffer[..self.len]);
        self.len = 0;
    }

    pub(crate) fn finish(mut self) -> Digest {
        self.drain();
        self.hasher.finish()
    }
}

/// Accumulates the logical digest of a segment as its rows and events are written.
pub(crate) struct LogicalDigest {
    rows: ValueHasher,
    row_count: u64,
    /// One stream per event signal, in signal-id order.
    events: Vec<(u32, ValueHasher, u64)>,
}

impl LogicalDigest {
    pub(crate) fn new(event_signals: impl Iterator<Item = u32>) -> Self {
        Self {
            rows: ValueHasher::default(),
            row_count: 0,
            events: event_signals
                .map(|id| (id, ValueHasher::default(), 0))
                .collect(),
        }
    }

    /// Hash `rows` rows of a row-major chunk: each row's x, then its slots.
    pub(crate) fn rows(&mut self, x: &[f64], values: &[f64], slots: usize, rows: usize) {
        for (row, &x) in x[..rows].iter().enumerate() {
            self.rows.value(x);
            for &value in &values[row * slots..row * slots + slots] {
                self.rows.value(value);
            }
        }
        self.row_count += rows as u64;
    }

    /// Hash rows given as x and one column per value slot.
    pub(crate) fn columns(&mut self, x: &[f64], columns: &[Vec<f64>]) {
        for (row, &x) in x.iter().enumerate() {
            self.rows.value(x);
            for column in columns {
                self.rows.value(column[row]);
            }
        }
        self.row_count += x.len() as u64;
    }

    /// The digest stream of the `index`-th event signal, which `count` more changes will enter.
    pub(crate) fn event_stream(&mut self, index: usize, count: u64) -> &mut ValueHasher {
        let stream = &mut self.events[index];
        stream.2 += count;
        &mut stream.1
    }

    /// The digest over everything written, bound to the schema's canonical bytes.
    pub(crate) fn finish(self, schema_bytes: &[u8]) -> Digest {
        let mut hasher = Sha256::default();
        hasher.update(DIGEST_DOMAIN);
        hasher.update(&(schema_bytes.len() as u64).to_le_bytes());
        hasher.update(schema_bytes);
        hasher.update(&self.row_count.to_le_bytes());
        hasher.update(&self.rows.finish().0);
        for (id, stream, count) in self.events {
            hasher.update(&id.to_le_bytes());
            hasher.update(&count.to_le_bytes());
            hasher.update(&stream.finish().0);
        }
        hasher.finish()
    }
}
