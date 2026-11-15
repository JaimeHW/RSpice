//! The segment writer: rows are staged row-major into the open chunk, and each full chunk is
//! transposed, summarised, encoded, checksummed and appended.
//!
//! Frame order depends only on how many rows and events arrived, never on wall clock:
//! 1. an `EVENTS` frame is written the moment an event signal's buffer reaches 4,096 changes;
//! 2. when a chunk fills: its `CHUNK`, then each event signal's partial buffer in signal-id order,
//!    then the `SUMMARY` frames due at its last row in ascending level, then an `INDEX` every 64
//!    chunks (or once 4,096 frames wait for one);
//! 3. at the end: the last partial chunk and event buffers, every pending summary, a final `INDEX`
//!    and `END`.
//!
//! A durability flush writes the open chunk (and open event buffers) as provisional frames at the
//! tail without advancing it; the next real frame overwrites them and the file is truncated to the
//! real tail, so the final bytes do not depend on when, or whether, flushes happened.

use crate::digest::{Digest, LogicalDigest};
use crate::format::{
    FLAG_PARTIAL, FORMAT_VERSION, FrameBuf, FrameKind, INDEX_CHUNKS, INDEX_ENTRY_LIMIT,
    LEVEL1_ROWS, POINTERS_AT, POINTERS_LEN, SEGMENT_MAGIC, SUPERBLOCK_LEN, canonical, chunk_rows,
    fold_max, fold_min,
};
use crate::{RunMeta, Schema, SegmentIo, StoreError, codec};

/// Why a run stopped before completing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopReason {
    DiskFull,
    MemoryFull,
    Cancelled,
    Timeout,
    Failed(String),
}

/// How a run ended, recorded in the segment's `END` frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    Completed,
    Stopped(StopReason),
}

impl RunStatus {
    pub(crate) fn encode(&self) -> (u8, u8, &str) {
        match self {
            Self::Completed => (0, 0, ""),
            Self::Stopped(StopReason::DiskFull) => (1, 1, ""),
            Self::Stopped(StopReason::MemoryFull) => (1, 2, ""),
            Self::Stopped(StopReason::Cancelled) => (1, 3, ""),
            Self::Stopped(StopReason::Timeout) => (1, 4, ""),
            Self::Stopped(StopReason::Failed(message)) => (1, 5, message),
        }
    }
}

/// What a finished segment holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreReceipt {
    pub rows: u64,
    pub chunks: u64,
    pub events: u64,
    /// The segment's length in bytes.
    pub bytes: u64,
    pub digest: Digest,
}

/// The open chunk: x and the value slots of up to `chunk_rows` rows, row-major as the engine
/// produces them. Two of these exist per writer; neither is ever reallocated.
pub(crate) struct Batch {
    pub(crate) first_row: u64,
    pub(crate) rows: usize,
    pub(crate) slots: usize,
    pub(crate) x: Vec<f64>,
    pub(crate) values: Vec<f64>,
}

impl Batch {
    pub(crate) fn new(chunk_rows: usize, slots: usize) -> Self {
        Self {
            first_row: 0,
            rows: 0,
            slots,
            x: vec![0.0; chunk_rows],
            values: vec![0.0; chunk_rows * slots],
        }
    }

    #[inline(always)]
    pub(crate) fn push(&mut self, x: f64, values: &[f64]) {
        let row = self.rows;
        let slots = self.slots;
        self.x[row] = x;
        self.values[row * slots..row * slots + slots].copy_from_slice(values);
        self.rows = row + 1;
    }

    /// Start the next chunk after this one was handed off.
    pub(crate) fn reset(&mut self, first_row: u64) {
        self.first_row = first_row;
        self.rows = 0;
    }
}

/// One analog signal's place in the row.
#[derive(Debug, Clone, Copy)]
pub(crate) struct AnalogColumn {
    pub(crate) slot: usize,
    pub(crate) complex: bool,
}

/// What the encoder derives once from the schema.
pub(crate) struct Layout {
    pub(crate) slots: usize,
    pub(crate) chunk_rows: usize,
    pub(crate) analog: Vec<AnalogColumn>,
    pub(crate) schema_bytes: Vec<u8>,
}

impl Layout {
    fn new(schema: &Schema, chunk_rows: usize) -> Self {
        let mut analog = Vec::new();
        let mut slot = 0;
        for signal in &schema.signals {
            let slots = signal.kind.slots();
            if slots > 0 {
                analog.push(AnalogColumn {
                    slot,
                    complex: slots == 2,
                });
            }
            slot += slots;
        }
        let mut schema_bytes = Vec::new();
        schema.encode(&mut schema_bytes);
        Self {
            slots: slot,
            chunk_rows,
            analog,
            schema_bytes,
        }
    }
}

/// One frame an `INDEX` lists: 48 bytes on disk.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IndexEntry {
    pub(crate) kind: FrameKind,
    /// Summary level for `SUMMARY`; zero otherwise.
    pub(crate) level: u8,
    /// Signal id for `EVENTS`; zero otherwise.
    pub(crate) signal: u32,
    pub(crate) offset: u64,
    /// First row (`CHUNK`, `SUMMARY`) or the signal's first event ordinal (`EVENTS`).
    pub(crate) first: u64,
    /// Rows (`CHUNK`, `SUMMARY`) or events (`EVENTS`).
    pub(crate) count: u64,
    pub(crate) x_min: f64,
    pub(crate) x_max: f64,
}

impl IndexEntry {
    fn put(&self, frame: &mut FrameBuf) {
        frame.put_u8(self.kind as u8);
        frame.put_u8(self.level);
        frame.put(&[0, 0]);
        frame.put_u32(self.signal);
        frame.put_u64(self.offset);
        frame.put_u64(self.first);
        frame.put_u64(self.count);
        frame.put_f64(self.x_min);
        frame.put_f64(self.x_max);
    }
}

/// An `INDEX` frame as `END` lists it, so a reader can find any index without walking the chain.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IndexDirEntry {
    pub(crate) offset: u64,
    pub(crate) first_row: u64,
    pub(crate) rows: u64,
    pub(crate) x_min: f64,
    pub(crate) x_max: f64,
    pub(crate) entries: u32,
}

/// Transpose `rows` row-major rows of `slots` values into columns `stride` apart, in tiles so both
/// sides stay in cache.
fn transpose(values: &[f64], rows: usize, slots: usize, out: &mut [f64], stride: usize) {
    const TILE: usize = 16;
    for r0 in (0..rows).step_by(TILE) {
        let r1 = (r0 + TILE).min(rows);
        for s0 in (0..slots).step_by(TILE) {
            let s1 = (s0 + TILE).min(slots);
            for row in r0..r1 {
                let source = &values[row * slots + s0..row * slots + s1];
                for (k, &value) in source.iter().enumerate() {
                    out[(s0 + k) * stride + row] = value;
                }
            }
        }
    }
}

pub(crate) fn value_range(values: &[f64]) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::NAN, f64::NAN);
    for &value in values {
        lo = fold_min(lo, value);
        hi = fold_max(hi, value);
    }
    (canonical(lo), canonical(hi))
}

/// |z| of one complex row, with only correctly rounded operations so every machine agrees.
#[inline(always)]
pub(crate) fn magnitude(re: f64, im: f64) -> f64 {
    (re * re + im * im).sqrt()
}

/// The single-threaded half of the writer: turns staged chunks into frames on its `SegmentIo`.
pub(crate) struct Encoder<I: SegmentIo> {
    io: I,
    layout: Layout,
    frame: FrameBuf,
    /// The chunk transposed: `slots` columns of `chunk_rows` values.
    columns: Vec<f64>,
    /// Shuffle scratch, `chunk_rows * 8` bytes.
    planes: Vec<u8>,
    /// |z| scratch for complex columns' level-1 summaries.
    scratch: Vec<f64>,
    /// The last chunk's level-1 bucket x ranges, `(x_min, x_max)` per bucket.
    level1_x: Vec<f64>,
    /// The last chunk's level-1 summaries, signal-major `(min, max)` per bucket.
    level1: Vec<f64>,
    /// End of the real frames: where the next real frame goes.
    tail: u64,
    /// The segment's length: past `tail` while a provisional tail is on disk.
    file_end: u64,
    next_chunk: u64,
    rows: u64,
    entries: Vec<IndexEntry>,
    chunks_since_index: usize,
    index_first_row: u64,
    last_index: Option<u64>,
    index_dir: Vec<IndexDirEntry>,
    digest: LogicalDigest,
}

impl<I: SegmentIo> Encoder<I> {
    pub(crate) fn create(
        mut io: I,
        schema: &Schema,
        meta: &RunMeta,
        chunk_rows: usize,
    ) -> Result<Self, StoreError> {
        schema.validate()?;
        let layout = Layout::new(schema, chunk_rows);
        let block_bytes = codec::max_encoded_len(chunk_rows * 8);
        let buckets = chunk_rows.div_ceil(LEVEL1_ROWS);
        let chunk_frame = 64
            + 12 * (layout.slots + 1)
            + 16 * buckets * (layout.analog.len() + 1)
            + block_bytes * (layout.slots + 1);
        let mut encoder = Self {
            frame: FrameBuf::with_capacity(chunk_frame),
            columns: vec![0.0; layout.slots * chunk_rows],
            planes: vec![0; chunk_rows * 8],
            scratch: vec![0.0; chunk_rows],
            level1_x: Vec::with_capacity(2 * buckets),
            level1: Vec::with_capacity(2 * buckets * layout.analog.len()),
            tail: SUPERBLOCK_LEN,
            file_end: 0,
            next_chunk: 0,
            rows: 0,
            entries: Vec::with_capacity(INDEX_ENTRY_LIMIT),
            chunks_since_index: 0,
            index_first_row: 0,
            last_index: None,
            index_dir: Vec::new(),
            digest: LogicalDigest::new(std::iter::empty()),
            layout,
            io: {
                io.truncate(0)?;
                io
            },
        };
        let mut superblock = [0u8; SUPERBLOCK_LEN as usize];
        superblock[..8].copy_from_slice(&SEGMENT_MAGIC);
        superblock[8..12].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
        encoder.io.write_at(0, &superblock)?;
        encoder.file_end = SUPERBLOCK_LEN;

        encoder.frame.begin(FrameKind::Schema, 0);
        let schema_bytes = std::mem::take(&mut encoder.layout.schema_bytes);
        encoder.frame.put(&schema_bytes);
        encoder.layout.schema_bytes = schema_bytes;
        encoder.emit_real()?;

        let mut meta_bytes = Vec::new();
        meta.encode(&mut meta_bytes);
        encoder.frame.begin(FrameKind::RunMeta, 0);
        encoder.frame.put(&meta_bytes);
        encoder.emit_real()?;
        encoder.io.flush()?;
        Ok(encoder)
    }

    pub(crate) fn layout(&self) -> &Layout {
        &self.layout
    }

    #[cfg(test)]
    pub(crate) fn io(&self) -> &I {
        &self.io
    }

    /// Append the frame in `self.frame` at the real tail.
    fn emit_real(&mut self) -> Result<u64, StoreError> {
        let at = self.tail;
        let bytes = self.frame.finish()?;
        self.io.write_at(at, bytes)?;
        self.tail = at + bytes.len() as u64;
        self.file_end = self.file_end.max(self.tail);
        Ok(at)
    }

    /// Drop whatever provisional tail the real frames did not overwrite.
    fn settle(&mut self) -> Result<(), StoreError> {
        if self.file_end > self.tail {
            self.io.truncate(self.tail)?;
            self.file_end = self.tail;
        }
        Ok(())
    }

    fn write_pointers(&mut self, end: u64) -> Result<(), StoreError> {
        let mut pointers = [0u8; POINTERS_LEN];
        pointers[..8].copy_from_slice(&self.last_index.unwrap_or(0).to_le_bytes());
        pointers[8..16].copy_from_slice(&self.index_first_row.to_le_bytes());
        pointers[16..24].copy_from_slice(&end.to_le_bytes());
        self.io.write_at(POINTERS_AT, &pointers)?;
        Ok(())
    }

    /// Encode `batch` as a `CHUNK` frame into `self.frame`, leaving its level-1 summaries in
    /// `self.level1_x` / `self.level1`. Returns the chunk's x range.
    fn encode_chunk(&mut self, batch: &Batch, chunk_index: u64, flags: u8) -> (f64, f64) {
        let rows = batch.rows;
        let slots = self.layout.slots;
        let stride = self.layout.chunk_rows;
        transpose(&batch.values, rows, slots, &mut self.columns, stride);
        let buckets = rows.div_ceil(LEVEL1_ROWS);

        self.level1_x.clear();
        let (mut x_min, mut x_max) = (f64::NAN, f64::NAN);
        for bucket in batch.x[..rows].chunks(LEVEL1_ROWS) {
            let (lo, hi) = value_range(bucket);
            x_min = fold_min(x_min, lo);
            x_max = fold_max(x_max, hi);
            self.level1_x.push(lo);
            self.level1_x.push(hi);
        }
        self.level1.clear();
        for column in &self.layout.analog {
            let base = column.slot * stride;
            let values: &[f64] = if column.complex {
                let (re, im) = (
                    &self.columns[base..base + rows],
                    &self.columns[base + stride..base + stride + rows],
                );
                for ((out, &re), &im) in self.scratch[..rows].iter_mut().zip(re).zip(im) {
                    *out = magnitude(re, im);
                }
                &self.scratch[..rows]
            } else {
                &self.columns[base..base + rows]
            };
            for bucket in values.chunks(LEVEL1_ROWS) {
                let (lo, hi) = value_range(bucket);
                self.level1.push(lo);
                self.level1.push(hi);
            }
        }

        let blocks = slots + 1;
        self.frame.begin(FrameKind::Chunk, flags);
        self.frame.put_u64(chunk_index);
        self.frame.put_u64(batch.first_row);
        self.frame.put_u32(rows as u32);
        self.frame.put_u32(blocks as u32);
        let table_at = self.frame.payload_len();
        self.frame.put_zeros(12 * blocks);
        for &value in self.level1_x.iter().chain(&self.level1) {
            self.frame.put_f64(value);
        }
        debug_assert_eq!(self.level1_x.len(), 2 * buckets);
        let max_block = codec::max_encoded_len(rows * 8);
        for block in 0..blocks {
            let values: &[f64] = if block == 0 {
                &batch.x[..rows]
            } else {
                let base = (block - 1) * stride;
                &self.columns[base..base + rows]
            };
            let at = self.frame.payload_len();
            let (codec, len) =
                codec::encode_f64(values, &mut self.planes, self.frame.spare(max_block));
            self.frame.advance(len);
            let entry = table_at + 12 * block;
            self.frame.patch_u32(entry, at as u32);
            self.frame.patch_u32(entry + 4, len as u32);
            self.frame.patch_u8(entry + 8, codec as u8);
        }
        (x_min, x_max)
    }

    /// Write a staged chunk (full, or the last partial one) as a real `CHUNK` and what follows it.
    pub(crate) fn write_batch(&mut self, batch: &Batch) -> Result<(), StoreError> {
        debug_assert_eq!(batch.first_row, self.rows);
        if batch.rows == 0 {
            return Ok(());
        }
        let chunk_index = self.next_chunk;
        let (x_min, x_max) = self.encode_chunk(batch, chunk_index, 0);
        let offset = self.emit_real()?;
        self.entries.push(IndexEntry {
            kind: FrameKind::Chunk,
            level: 0,
            signal: 0,
            offset,
            first: batch.first_row,
            count: batch.rows as u64,
            x_min,
            x_max,
        });
        self.digest
            .rows(&batch.x, &batch.values, self.layout.slots, batch.rows);
        self.next_chunk += 1;
        self.rows += batch.rows as u64;
        self.chunks_since_index += 1;
        if self.chunks_since_index >= INDEX_CHUNKS || self.entries.len() >= INDEX_ENTRY_LIMIT {
            self.write_index()?;
        }
        self.settle()
    }

    fn write_index(&mut self) -> Result<(), StoreError> {
        let count = self.entries.len();
        let (mut x_min, mut x_max) = (f64::NAN, f64::NAN);
        self.frame.begin(FrameKind::Index, 0);
        self.frame.put_u64(self.last_index.unwrap_or(u64::MAX));
        self.frame.put_u64(self.index_dir.len() as u64);
        self.frame.put_u64(self.rows);
        self.frame.put_u32(count as u32);
        self.frame.put_u32(0);
        for entry in &self.entries {
            if entry.kind == FrameKind::Chunk {
                x_min = fold_min(x_min, entry.x_min);
                x_max = fold_max(x_max, entry.x_max);
            }
            entry.put(&mut self.frame);
        }
        let offset = self.emit_real()?;
        self.index_dir.push(IndexDirEntry {
            offset,
            first_row: self.index_first_row,
            rows: self.rows - self.index_first_row,
            x_min: canonical(x_min),
            x_max: canonical(x_max),
            entries: count as u32,
        });
        self.entries.clear();
        self.chunks_since_index = 0;
        self.index_first_row = self.rows;
        self.last_index = Some(offset);
        self.io.flush()?;
        self.write_pointers(0)
    }

    /// Write the open chunk as provisional frames at the tail, durably, without advancing the tail.
    pub(crate) fn write_provisional(&mut self, batch: &Batch) -> Result<(), StoreError> {
        let mut at = self.tail;
        if batch.rows > 0 {
            self.encode_chunk(batch, self.next_chunk, FLAG_PARTIAL);
            let bytes = self.frame.finish()?;
            self.io.write_at(at, bytes)?;
            at += bytes.len() as u64;
        }
        if self.file_end > at {
            self.io.truncate(at)?;
        }
        self.file_end = at;
        self.io.flush()?;
        Ok(())
    }

    /// Write the last frames and `END`; returns the receipt and the segment's I/O.
    pub(crate) fn finish(mut self, status: &RunStatus) -> Result<(StoreReceipt, I), StoreError> {
        if !self.entries.is_empty() || self.last_index.is_none() {
            self.write_index()?;
        }
        let digest = std::mem::replace(&mut self.digest, LogicalDigest::new(std::iter::empty()))
            .finish(&self.layout.schema_bytes);
        let (code, reason, message) = status.encode();
        self.frame.begin(FrameKind::End, 0);
        self.frame.put_u8(code);
        self.frame.put_u8(reason);
        self.frame.put(&[0, 0]);
        self.frame.put_u32(message.len() as u32);
        self.frame.put(message.as_bytes());
        self.frame.put_u64(self.rows);
        self.frame.put_u64(self.next_chunk);
        self.frame.put_u64(self.last_index.unwrap_or(u64::MAX));
        self.frame.put(&digest.0);
        // Event totals and the summary directory follow; both are empty without event signals or
        // summary frames.
        self.frame.put_u32(0);
        self.frame.put_u32(self.index_dir.len() as u32);
        for entry in &self.index_dir {
            self.frame.put_u64(entry.offset);
            self.frame.put_u64(entry.first_row);
            self.frame.put_u64(entry.rows);
            self.frame.put_f64(entry.x_min);
            self.frame.put_f64(entry.x_max);
            self.frame.put_u32(entry.entries);
        }
        self.frame.put_u32(0);
        let end = self.emit_real()?;
        self.settle()?;
        self.io.flush()?;
        self.write_pointers(end)?;
        self.io.flush()?;
        let receipt = StoreReceipt {
            rows: self.rows,
            chunks: self.next_chunk,
            events: 0,
            bytes: self.tail,
            digest,
        };
        Ok((receipt, self.io))
    }
}

/// Writes one run's segment.
///
/// `push_row` copies the row into the open chunk; a full chunk is encoded and appended. Memory is
/// the open chunk, the encoder's buffers for one chunk and one index's entries, whatever the run's
/// length.
pub struct SegmentWriter<I: SegmentIo> {
    open: Batch,
    encoder: Encoder<I>,
    stopped: bool,
}

impl<I: SegmentIo> SegmentWriter<I> {
    /// Start a segment on `io` (emptied first) for rows of `schema`.
    pub fn create(io: I, schema: &Schema, meta: &RunMeta) -> Result<Self, StoreError> {
        Self::with_chunk_rows(io, schema, meta, chunk_rows(schema.slots()))
    }

    pub(crate) fn with_chunk_rows(
        io: I,
        schema: &Schema,
        meta: &RunMeta,
        chunk_rows: usize,
    ) -> Result<Self, StoreError> {
        debug_assert!(chunk_rows.is_power_of_two() && chunk_rows >= LEVEL1_ROWS);
        let encoder = Encoder::create(io, schema, meta, chunk_rows)?;
        Ok(Self {
            open: Batch::new(chunk_rows, encoder.layout().slots),
            encoder,
            stopped: false,
        })
    }

    /// Rows accepted so far.
    pub fn rows(&self) -> u64 {
        self.open.first_row + self.open.rows as u64
    }

    fn guard<T>(&mut self, result: Result<T, StoreError>) -> Result<T, StoreError> {
        if result.is_err() {
            self.stopped = true;
        }
        result
    }

    /// Stage one row: x and the schema's value slots in order.
    #[inline]
    pub fn push_row(&mut self, x: f64, values: &[f64]) -> Result<(), StoreError> {
        if values.len() != self.open.slots || self.stopped {
            return Err(if self.stopped {
                StoreError::WriterStopped
            } else {
                StoreError::RowWidth {
                    expected: self.open.slots,
                    found: values.len(),
                }
            });
        }
        self.open.push(x, values);
        if self.open.rows == self.open.x.len() {
            let result = self.encoder.write_batch(&self.open);
            self.guard(result)?;
            let next = self.open.first_row + self.open.rows as u64;
            self.open.reset(next);
        }
        Ok(())
    }

    /// Stage several rows: `x` and their value slots, row-major. The bytes written are the same as
    /// for one `push_row` per row.
    pub fn push_rows(&mut self, x: &[f64], values: &[f64]) -> Result<(), StoreError> {
        let slots = self.open.slots;
        if values.len() != x.len() * slots {
            return Err(StoreError::RowWidth {
                expected: x.len() * slots,
                found: values.len(),
            });
        }
        for (row, &x) in x.iter().enumerate() {
            self.push_row(x, &values[row * slots..row * slots + slots])?;
        }
        Ok(())
    }

    /// Overwrite one value of a row still in the open chunk (the engine's backfill of row 0).
    pub fn amend_open_row(&mut self, row: u64, slot: usize, value: f64) -> Result<(), StoreError> {
        let open = &mut self.open;
        if row < open.first_row || row >= open.first_row + open.rows as u64 || slot >= open.slots {
            return Err(StoreError::RowNotOpen { row, slot });
        }
        open.values[(row - open.first_row) as usize * open.slots + slot] = value;
        Ok(())
    }

    /// Make every row pushed so far durable by writing the open chunk provisionally.
    pub fn sync(&mut self) -> Result<(), StoreError> {
        if self.stopped {
            return Err(StoreError::WriterStopped);
        }
        let result = self.encoder.write_provisional(&self.open);
        self.guard(result)
    }

    /// Write the last chunk and `END` with `status`; returns the receipt and the segment's I/O.
    pub fn finish(mut self, status: RunStatus) -> Result<(StoreReceipt, I), StoreError> {
        if self.stopped {
            return Err(StoreError::WriterStopped);
        }
        self.encoder.write_batch(&self.open)?;
        self.encoder.finish(&status)
    }

    #[cfg(test)]
    pub(crate) fn io(&self) -> &I {
        self.encoder.io()
    }
}
