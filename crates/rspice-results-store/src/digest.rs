//! SHA-256, the store's logical result digest, and the digest of one column.
//!
//! The segment's digest is logical: it hashes the decoded values' stored bits in canonical order
//! (each row's x then its value slots, rows in order; then each event signal's changes), and the
//! summaries a reader draws overviews from, each region's stored bytes in the order the segment
//! holds them. A kept segment is named by its digest, so the name follows every stored bit: two
//! runs that differ only in a signed zero or a NaN's payload are two segments. It does not change
//! when the codec, the chunk size or the way rows were pushed changes.
//!
//! A column's digest ([`ColumnDigest`]) identifies its values, as a result's digest identifies
//! the result: the SHA-256 of its rows as little-endian binary64 in row order, each signed zero
//! written as zero and each NaN as the one quiet NaN. A column held in memory digests the same as
//! the column a segment stores, so the same values have one identity wherever they are held.

use std::fmt;

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

const DIGEST_DOMAIN: &[u8] = b"rspice-results-store logical digest 2";

/// The bits a column digest writes for `value`: zero for either signed zero, and the one quiet
/// NaN for any NaN, as a result's digest writes a value.
fn canonical_bits(value: f64) -> u64 {
    if value == 0.0 {
        0
    } else if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

/// The digest of one column of a run: the SHA-256 of its rows as little-endian binary64 in row
/// order, each signed zero written as zero and each NaN as the one quiet NaN. A complete segment
/// states one for each of its columns ([`crate::SegmentReader::column_digest`]), and a column held
/// in memory digests the same, so the same values have the same digest wherever they are held.
#[derive(Clone, Default)]
pub struct ColumnDigest(Sha256);

impl ColumnDigest {
    pub fn new() -> Self {
        Self::default()
    }

    /// The digest of `values`, a whole column.
    pub fn of(values: &[f64]) -> Digest {
        let mut digest = Self::new();
        digest.values(values);
        digest.finish()
    }

    /// Hash the column's next rows.
    pub fn values(&mut self, values: &[f64]) {
        // Staged through the stack, so a column digest holds no buffer of its own: a segment
        // keeps one per column.
        let mut staged = [0u8; 4096];
        for block in values.chunks(staged.len() / 8) {
            let bytes = &mut staged[..8 * block.len()];
            for (out, &value) in bytes.chunks_exact_mut(8).zip(block) {
                out.copy_from_slice(&canonical_bits(value).to_le_bytes());
            }
            self.0.update(bytes);
        }
    }

    pub fn finish(self) -> Digest {
        self.0.finish()
    }

    /// The state reached so far, as [`Self::resumed`] continues it.
    pub(crate) fn state(&self) -> [u8; HASH_STATE_LEN] {
        use sha2::digest::common::hazmat::SerializableState;
        let mut state = [0u8; HASH_STATE_LEN];
        state.copy_from_slice(&self.0.0.serialize());
        state
    }

    /// A column digest continuing from `state`, as [`Self::state`] recorded it.
    pub(crate) fn resumed(state: &[u8]) -> Option<Self> {
        use sha2::digest::common::hazmat::SerializableState;
        let state = state.try_into().ok()?;
        let hasher = sha2::Sha256::deserialize(state).ok()?;
        Some(Self(Sha256(hasher)))
    }
}

/// The row stream, the column digests and the row count at a point a write may return to.
pub(crate) struct RowsMark {
    pub(crate) rows: ValueHasher,
    pub(crate) columns: Vec<ColumnDigest>,
    pub(crate) row_count: u64,
}

/// Values' bytes staged through a fixed buffer, so hashing a chunk allocates nothing.
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
    /// Hash a summary region bucket by bucket, each bucket's columns in turn, so the stream does
    /// not depend on how the run was cut into chunks: a region stores `columns` columns, each of
    /// the same number of 16-byte buckets.
    pub(crate) fn region(&mut self, region: &[u8], columns: usize) {
        let column_len = region.len() / columns;
        debug_assert_eq!(column_len * columns, region.len());
        debug_assert_eq!(column_len % 16, 0);
        for bucket in (0..column_len).step_by(16) {
            for column in 0..columns {
                let at = column * column_len + bucket;
                self.bytes(&region[at..at + 16]);
            }
        }
    }

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
        self.buffer[self.len..self.len + 8].copy_from_slice(&value.to_le_bytes());
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
                out.copy_from_slice(&value.to_le_bytes());
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

/// Accumulates the logical digest of a segment as its rows and events are written, and each
/// column's digest.
pub(crate) struct LogicalDigest {
    rows: ValueHasher,
    row_count: u64,
    /// One digest per column: the run's x, then each value slot.
    columns: Vec<ColumnDigest>,
    /// Every summary region, in the order the segment holds them: each chunk's level-1 buckets,
    /// and each `SUMMARY` frame's, as they follow the chunks, bucket by bucket. Chunk rows are a
    /// power of two, so a full `SUMMARY` frame always follows the chunk its rows end with and the
    /// stream does not depend on the chunk size.
    summaries: ValueHasher,
    /// One stream per event signal, in signal-id order.
    events: Vec<(u32, ValueHasher, u64)>,
}

impl LogicalDigest {
    /// The digest of a segment of `slots` value slots and the event signals `event_signals`.
    pub(crate) fn new(slots: usize, event_signals: impl Iterator<Item = u32>) -> Self {
        Self {
            rows: ValueHasher::default(),
            row_count: 0,
            columns: vec![ColumnDigest::default(); slots + 1],
            summaries: ValueHasher::default(),
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

    /// Hash rows given as x and one column per value slot: the row stream, and each column's
    /// digest.
    pub(crate) fn columns(&mut self, x: &[f64], columns: &[Vec<f64>]) {
        for (row, &x) in x.iter().enumerate() {
            self.rows.value(x);
            for column in columns {
                self.rows.value(column[row]);
            }
        }
        self.row_count += x.len() as u64;
        self.column_values(x, columns.iter().map(Vec::as_slice));
    }

    /// Hash a chunk's columns into each column's digest: its x, and one column per value slot,
    /// each as long as x.
    pub(crate) fn column_values<'a>(&mut self, x: &[f64], slots: impl Iterator<Item = &'a [f64]>) {
        self.columns[0].values(x);
        for (column, values) in self.columns[1..].iter_mut().zip(slots) {
            debug_assert_eq!(values.len(), x.len());
            column.values(values);
        }
    }

    /// The row stream's state after every row hashed so far.
    pub(crate) fn rows_state(&mut self) -> [u8; HASH_STATE_LEN] {
        self.rows.state()
    }

    /// Each column digest's state after every row hashed so far, the run's x first.
    pub(crate) fn column_states(&self) -> impl Iterator<Item = [u8; HASH_STATE_LEN]> + '_ {
        self.columns.iter().map(ColumnDigest::state)
    }

    /// Hash one summary region's stored bytes, `columns` columns of 16-byte buckets.
    pub(crate) fn summaries(&mut self, region: &[u8], columns: usize) {
        self.summaries.region(region, columns);
    }

    /// The summary stream's state after every region hashed so far.
    pub(crate) fn summaries_state(&mut self) -> [u8; HASH_STATE_LEN] {
        self.summaries.state()
    }

    /// The summary stream, to put back with [`Self::resume_summaries`] when the regions hashed
    /// after this point are not kept.
    pub(crate) fn summaries_mark(&self) -> ValueHasher {
        self.summaries.clone()
    }

    /// Continue the summary stream from `stream`.
    pub(crate) fn resume_summaries(&mut self, stream: ValueHasher) {
        self.summaries = stream;
    }

    /// The row stream, the column digests and the row count, to put back with
    /// [`Self::resume_rows`] when the rows hashed after this point are not kept.
    pub(crate) fn rows_mark(&self) -> RowsMark {
        RowsMark {
            rows: self.rows.clone(),
            columns: self.columns.clone(),
            row_count: self.row_count,
        }
    }

    /// The `index`-th event signal's stream and count, as [`Self::rows_mark`] keeps the rows'.
    pub(crate) fn events_mark(&self, index: usize) -> (ValueHasher, u64) {
        let (_, stream, count) = &self.events[index];
        (stream.clone(), *count)
    }

    /// Continue the row stream and the column digests from `mark`.
    pub(crate) fn resume_rows(&mut self, mark: RowsMark) {
        debug_assert_eq!(mark.columns.len(), self.columns.len());
        self.rows = mark.rows;
        self.columns = mark.columns;
        self.row_count = mark.row_count;
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

    /// The digest over everything written, bound to the schema's canonical bytes, and each
    /// column's digest, the run's x first.
    pub(crate) fn finish(self, schema_bytes: &[u8]) -> (Digest, Vec<Digest>) {
        let columns: Vec<Digest> = self.columns.into_iter().map(ColumnDigest::finish).collect();
        let mut hasher = Sha256::default();
        hasher.update(DIGEST_DOMAIN);
        hasher.update(&(schema_bytes.len() as u64).to_le_bytes());
        hasher.update(schema_bytes);
        hasher.update(&self.row_count.to_le_bytes());
        hasher.update(&self.rows.finish().0);
        hasher.update(&self.summaries.finish().0);
        for (id, stream, count) in self.events {
            hasher.update(&id.to_le_bytes());
            hasher.update(&count.to_le_bytes());
            hasher.update(&stream.finish().0);
        }
        (hasher.finish(), columns)
    }
}
