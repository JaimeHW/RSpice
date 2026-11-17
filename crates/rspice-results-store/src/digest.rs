//! SHA-256 and the store's logical result digest.
//!
//! The digest is logical: it hashes decoded values in canonical order (each row's x then its value
//! slots, rows in order; then each event signal's changes), after mapping `-0.0` to `+0.0` and every
//! NaN to one canonical NaN, as `rspice-results`' result digest does. It therefore does not change
//! when the codec, the chunk size or the way rows were pushed changes.

use std::fmt;

use crate::format::CANONICAL_NAN_BITS;

/// The file name of the kept segment whose logical digest is `digest`: a project, a recovery or
/// a session keeps each segment under the name of what it holds.
pub fn kept_segment_name(digest: &Digest) -> String {
    format!("{digest}.rsr")
}

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

/// The bytes of a [`ValueHasher`]'s state, as a frame's trailer records it for a reopened
/// segment to continue from.
pub(crate) const HASH_STATE_LEN: usize = {
    use sha2::digest::common::hazmat::SerializableState;
    use sha2::digest::typenum::Unsigned;
    <sha2::Sha256 as SerializableState>::SerializedStateSize::USIZE
};

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
#[derive(Clone)]
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

    /// Hash several values; the same bytes as one [`Self::value`] per value.
    #[inline]
    pub(crate) fn values(&mut self, mut values: &[f64]) {
        while !values.is_empty() {
            let room = (self.buffer.len() - self.len) / 8;
            if room == 0 {
                self.drain();
                continue;
            }
            let take = room.min(values.len());
            let out = &mut self.buffer[self.len..self.len + 8 * take];
            for (out, &value) in out.chunks_exact_mut(8).zip(&values[..take]) {
                out.copy_from_slice(&canonical_bits(value).to_le_bytes());
            }
            self.len += 8 * take;
            values = &values[take..];
        }
    }

    fn drain(&mut self) {
        self.hasher.update(&self.buffer[..self.len]);
        self.len = 0;
    }

    pub(crate) fn finish(mut self) -> Digest {
        self.drain();
        self.hasher.finish()
    }

    /// The state reached so far: everything hashed, as [`Self::resumed`] continues it.
    pub(crate) fn state(&mut self) -> [u8; HASH_STATE_LEN] {
        use sha2::digest::common::hazmat::SerializableState;
        self.drain();
        let mut state = [0u8; HASH_STATE_LEN];
        state.copy_from_slice(&self.hasher.0.serialize());
        state
    }

    /// A hasher continuing from `state`, as [`Self::state`] recorded it.
    pub(crate) fn resumed(state: &[u8]) -> Option<Self> {
        use sha2::digest::common::hazmat::SerializableState;
        let state = state.try_into().ok()?;
        let hasher = sha2::Sha256::deserialize(state).ok()?;
        Some(Self {
            hasher: Sha256(hasher),
            ..Self::default()
        })
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
            self.rows.values(&values[row * slots..row * slots + slots]);
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

    /// The row stream's state after every row hashed so far.
    pub(crate) fn rows_state(&mut self) -> [u8; HASH_STATE_LEN] {
        self.rows.state()
    }

    /// The row stream and its row count, to put back with [`Self::resume_rows`] when the rows
    /// hashed after this point are not kept.
    pub(crate) fn rows_mark(&self) -> (ValueHasher, u64) {
        (self.rows.clone(), self.row_count)
    }

    /// The `index`-th event signal's stream and count, as [`Self::rows_mark`] keeps the rows'.
    pub(crate) fn events_mark(&self, index: usize) -> (ValueHasher, u64) {
        let (_, stream, count) = &self.events[index];
        (stream.clone(), *count)
    }

    /// Continue the row stream from `state`, after `row_count` rows.
    pub(crate) fn resume_rows(&mut self, rows: ValueHasher, row_count: u64) {
        self.rows = rows;
        self.row_count = row_count;
    }

    /// The `index`-th event signal's stream state after every change hashed so far.
    pub(crate) fn event_state(&mut self, index: usize) -> [u8; HASH_STATE_LEN] {
        self.events[index].1.state()
    }

    /// Continue the `index`-th event signal's stream from `stream`, after `count` changes.
    pub(crate) fn resume_events(&mut self, index: usize, stream: ValueHasher, count: u64) {
        self.events[index].1 = stream;
        self.events[index].2 = count;
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
