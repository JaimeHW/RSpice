//! Opening a segment, recovering one a crash cut short, and decoding rows on demand.
//!
//! A complete segment opens from the superblock's `END` pointer and the index chain. Without a
//! valid `END`, the reader walks back from the last durable index the superblock names, then scans
//! forward frame by frame, accepting a frame while its magic, length and CRC check out and it
//! continues its stream (the next chunk index and row, the next index in the chain). A provisional
//! frame is accepted as the last data; nothing real may follow it. What a crash leaves readable is
//! therefore always a prefix of the rows written.

use crate::codec::{self, Codec};
use crate::digest::{Digest, LogicalDigest};
use crate::format::{
    Cursor, FORMAT_VERSION, FRAME_HEADER_LEN, FrameHeader, FrameKind, LEVEL1_ROWS, POINTERS_AT,
    SEGMENT_MAGIC, SUPERBLOCK_LEN, fold_max, fold_min, frame_crc,
};
use crate::writer::{AnalogColumn, INDEX_ENTRY_LEN, RunStatus};
use crate::{RunMeta, Schema, SegmentIo, StoreError};

/// How a segment ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentState {
    /// `END` was written: the run's status and the logical digest the writer computed.
    Complete { status: RunStatus, digest: Digest },
    /// No `END`: the readable rows are what survived up to the first damaged or missing frame.
    Recovered,
}

/// Where one chunk's frame is and what it covers.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChunkRef {
    pub(crate) offset: u64,
    pub(crate) first_row: u64,
    pub(crate) rows: u32,
    pub(crate) x_min: f64,
    pub(crate) x_max: f64,
}

impl ChunkRef {
    pub(crate) fn end_row(&self) -> u64 {
        self.first_row + u64::from(self.rows)
    }
}

/// Every frame the reader found, in stream order.
#[derive(Default)]
struct Catalogue {
    chunks: Vec<ChunkRef>,
    rows: u64,
    last_index: Option<u64>,
    /// A provisional frame was accepted: only provisional frames may follow it.
    partial: bool,
}

impl Catalogue {
    /// Accept a chunk only when it continues the stream.
    fn chunk(&mut self, index: u64, chunk: ChunkRef) -> bool {
        if index != self.chunks.len() as u64 || chunk.first_row != self.rows || chunk.rows == 0 {
            return false;
        }
        self.rows = chunk.end_row();
        self.chunks.push(chunk);
        true
    }
}

/// What an `END` frame says.
struct EndRecord {
    status: RunStatus,
    rows: u64,
    chunks: u64,
    last_index: u64,
    digest: Digest,
}

/// Reads one segment.
pub struct SegmentReader<I: SegmentIo> {
    io: I,
    size: u64,
    schema: Schema,
    meta: RunMeta,
    slots: usize,
    analog: Vec<AnalogColumn>,
    catalogue: Catalogue,
    state: SegmentState,
    planes: Vec<u8>,
    block: Vec<u8>,
}

fn u64_at(bytes: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap_or_default())
}

impl<I: SegmentIo> SegmentReader<I> {
    /// Open a segment, recovering it when it has no valid `END`.
    pub fn open(io: I) -> Result<Self, StoreError> {
        let size = io.size()?;
        if size < SUPERBLOCK_LEN {
            return Err(StoreError::corrupt(
                0,
                "the segment is shorter than its superblock",
            ));
        }
        let mut superblock = [0u8; SUPERBLOCK_LEN as usize];
        io.read_at(0, &mut superblock)?;
        if superblock[..8] != SEGMENT_MAGIC {
            return Err(StoreError::corrupt(0, "not a results segment"));
        }
        let version = u32::from_le_bytes(superblock[8..12].try_into().unwrap_or_default());
        if version != FORMAT_VERSION {
            return Err(StoreError::corrupt(
                8,
                format!("format version {version} is not {FORMAT_VERSION}"),
            ));
        }
        let pointers = POINTERS_AT as usize;
        let durable_index = u64_at(&superblock, pointers);
        let end_at = u64_at(&superblock, pointers + 16);

        let mut reader = Self {
            io,
            size,
            schema: Schema::default(),
            meta: RunMeta::default(),
            slots: 0,
            analog: Vec::new(),
            catalogue: Catalogue::default(),
            state: SegmentState::Recovered,
            planes: Vec::new(),
            block: Vec::new(),
        };
        let (header, payload) = reader.read_frame(SUPERBLOCK_LEN)?;
        if header.kind != FrameKind::Schema {
            return Err(StoreError::corrupt(
                SUPERBLOCK_LEN,
                "the first frame is not the schema",
            ));
        }
        reader.schema = Schema::decode(&mut Cursor::new(&payload, SUPERBLOCK_LEN))?;
        let meta_at = header.next(SUPERBLOCK_LEN);
        let (header, payload) = reader.read_frame(meta_at)?;
        if header.kind != FrameKind::RunMeta {
            return Err(StoreError::corrupt(
                meta_at,
                "the second frame is not the run metadata",
            ));
        }
        reader.meta = RunMeta::decode(&mut Cursor::new(&payload, meta_at))?;
        let body = header.next(meta_at);
        let mut slot = 0;
        for signal in &reader.schema.signals {
            let slots = signal.kind.slots();
            if slots > 0 {
                reader.analog.push(AnalogColumn {
                    slot,
                    complex: slots == 2,
                });
            }
            slot += slots;
        }
        reader.slots = slot;

        if end_at >= body && reader.open_complete(end_at, body).is_ok() {
            return Ok(reader);
        }
        reader.recover(durable_index, body);
        Ok(reader)
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn run_meta(&self) -> &RunMeta {
        &self.meta
    }

    pub fn state(&self) -> &SegmentState {
        &self.state
    }

    /// Readable rows.
    pub fn rows(&self) -> u64 {
        self.catalogue.rows
    }

    /// Value slots per row.
    pub fn slots(&self) -> usize {
        self.slots
    }

    /// The smallest and largest x of the readable rows, ignoring NaN; `None` without rows.
    pub fn x_range(&self) -> Option<(f64, f64)> {
        let (mut lo, mut hi) = (f64::NAN, f64::NAN);
        for chunk in &self.catalogue.chunks {
            lo = fold_min(lo, chunk.x_min);
            hi = fold_max(hi, chunk.x_max);
        }
        (!lo.is_nan()).then_some((lo, hi))
    }

    /// Read and check the frame at `offset`.
    fn read_frame(&self, offset: u64) -> Result<(FrameHeader, Vec<u8>), StoreError> {
        let mut header_bytes = [0u8; FRAME_HEADER_LEN];
        if !offset.is_multiple_of(8) || offset + FRAME_HEADER_LEN as u64 > self.size {
            return Err(StoreError::corrupt(offset, "no frame header here"));
        }
        self.io.read_at(offset, &mut header_bytes)?;
        let header = FrameHeader::parse(&header_bytes)
            .ok_or_else(|| StoreError::corrupt(offset, "bad frame header"))?;
        let payload_at = offset + FRAME_HEADER_LEN as u64;
        if payload_at + u64::from(header.payload_len) > self.size {
            return Err(StoreError::corrupt(
                offset,
                "the frame runs past the end of the segment",
            ));
        }
        let mut payload = vec![0u8; header.payload_len as usize];
        self.io.read_at(payload_at, &mut payload)?;
        if frame_crc(&header_bytes, &payload) != header.crc {
            return Err(StoreError::corrupt(
                offset,
                "the frame's checksum does not match",
            ));
        }
        Ok((header, payload))
    }

    fn open_complete(&mut self, end_at: u64, body: u64) -> Result<(), StoreError> {
        let (header, payload) = self.read_frame(end_at)?;
        if header.kind != FrameKind::End {
            return Err(StoreError::corrupt(
                end_at,
                "the END pointer does not name END",
            ));
        }
        let end = Self::parse_end(&payload, end_at)?;
        let (catalogue, _) = self.load_chain(end.last_index, body)?;
        if catalogue.rows != end.rows || catalogue.chunks.len() as u64 != end.chunks {
            return Err(StoreError::corrupt(
                end_at,
                "END's totals disagree with its index chain",
            ));
        }
        self.catalogue = catalogue;
        self.state = SegmentState::Complete {
            status: end.status,
            digest: end.digest,
        };
        Ok(())
    }

    fn parse_end(payload: &[u8], offset: u64) -> Result<EndRecord, StoreError> {
        let mut cursor = Cursor::new(payload, offset + FRAME_HEADER_LEN as u64);
        let code = cursor.u8()?;
        let reason = cursor.u8()?;
        cursor.u16()?;
        let message = cursor.string()?;
        let status = RunStatus::decode(code, reason, message)
            .ok_or_else(|| cursor.error("unknown run status"))?;
        let rows = cursor.u64()?;
        let chunks = cursor.u64()?;
        let last_index = cursor.u64()?;
        let mut digest = [0u8; 32];
        digest.copy_from_slice(cursor.take(32)?);
        Ok(EndRecord {
            status,
            rows,
            chunks,
            last_index,
            digest: Digest(digest),
        })
    }

    /// Walk the index chain back from `last`, then replay it forwards into a catalogue. Returns the
    /// catalogue and the offset just past the last index frame.
    fn load_chain(&self, last: u64, body: u64) -> Result<(Catalogue, u64), StoreError> {
        let mut frames = Vec::new();
        let mut at = last;
        let mut after_last = None;
        loop {
            let (header, payload) = self.read_frame(at)?;
            if header.kind != FrameKind::Index || header.partial() {
                return Err(StoreError::corrupt(
                    at,
                    "the index chain names a frame that is not an index",
                ));
            }
            after_last.get_or_insert(header.next(at));
            let previous = u64_at(&payload, 0);
            frames.push((at, payload));
            if previous == u64::MAX {
                break;
            }
            if previous >= at || previous < body {
                return Err(StoreError::corrupt(
                    at,
                    "the index chain does not run backwards",
                ));
            }
            at = previous;
        }
        let mut catalogue = Catalogue::default();
        for (offset, payload) in frames.into_iter().rev() {
            self.replay_index(&mut catalogue, offset, &payload)?;
        }
        Ok((catalogue, after_last.unwrap_or(body)))
    }

    fn replay_index(
        &self,
        catalogue: &mut Catalogue,
        offset: u64,
        payload: &[u8],
    ) -> Result<(), StoreError> {
        let mut cursor = Cursor::new(payload, offset + FRAME_HEADER_LEN as u64);
        let previous = cursor.u64()?;
        if previous != catalogue.last_index.unwrap_or(u64::MAX) {
            return Err(cursor.error("an index does not chain to the one before it"));
        }
        let _ordinal = cursor.u64()?;
        let rows = cursor.u64()?;
        let count = cursor.count(INDEX_ENTRY_LEN)?;
        cursor.u32()?;
        for _ in 0..count {
            let kind = FrameKind::from_u8(cursor.u8()?)
                .ok_or_else(|| cursor.error("unknown frame kind"))?;
            let _level = cursor.u8()?;
            cursor.u16()?;
            let _signal = cursor.u32()?;
            let frame = cursor.u64()?;
            let first = cursor.u64()?;
            let count = cursor.u64()?;
            let x_min = cursor.f64()?;
            let x_max = cursor.f64()?;
            if kind == FrameKind::Chunk {
                let rows = u32::try_from(count)
                    .map_err(|_| cursor.error("a chunk of more than 2^32 rows"))?;
                let chunk = ChunkRef {
                    offset: frame,
                    first_row: first,
                    rows,
                    x_min,
                    x_max,
                };
                if frame >= offset || !catalogue.chunk(catalogue.chunks.len() as u64, chunk) {
                    return Err(cursor.error("an index lists a chunk out of order"));
                }
            }
        }
        if rows != catalogue.rows {
            return Err(cursor.error("an index's row count disagrees with its chunks"));
        }
        catalogue.last_index = Some(offset);
        Ok(())
    }

    /// Rebuild what a crash left: the durable index chain, then a forward scan.
    fn recover(&mut self, durable_index: u64, body: u64) {
        let (mut catalogue, mut at) = (Catalogue::default(), body);
        if durable_index >= body
            && let Ok((chain, after)) = self.load_chain(durable_index, body)
        {
            catalogue = chain;
            at = after;
        }
        let end = self.scan(&mut catalogue, at);
        self.catalogue = catalogue;
        self.state = match end {
            Some(end) => SegmentState::Complete {
                status: end.status,
                digest: end.digest,
            },
            None => SegmentState::Recovered,
        };
    }

    /// Accept frames from `at` while they check out and continue their streams.
    fn scan(&self, catalogue: &mut Catalogue, mut at: u64) -> Option<EndRecord> {
        while let Ok((header, payload)) = self.read_frame(at) {
            if catalogue.partial && !header.partial() {
                break;
            }
            let accepted = match header.kind {
                FrameKind::Chunk => self.scan_chunk(catalogue, at, &payload).unwrap_or(false),
                FrameKind::Index if !header.partial() => {
                    let fits = payload.len() >= 24
                        && u64_at(&payload, 0) == catalogue.last_index.unwrap_or(u64::MAX)
                        && u64_at(&payload, 16) == catalogue.rows;
                    if fits {
                        catalogue.last_index = Some(at);
                    }
                    fits
                }
                FrameKind::End if !header.partial() => {
                    return Self::parse_end(&payload, at).ok().filter(|end| {
                        end.rows == catalogue.rows && end.chunks == catalogue.chunks.len() as u64
                    });
                }
                _ => false,
            };
            if !accepted {
                break;
            }
            catalogue.partial |= header.partial();
            at = header.next(at);
        }
        None
    }

    fn scan_chunk(
        &self,
        catalogue: &mut Catalogue,
        offset: u64,
        payload: &[u8],
    ) -> Result<bool, StoreError> {
        let mut cursor = Cursor::new(payload, offset + FRAME_HEADER_LEN as u64);
        let index = cursor.u64()?;
        let first_row = cursor.u64()?;
        let rows = cursor.u32()?;
        let blocks = cursor.u32()? as usize;
        if blocks != self.slots + 1 {
            return Ok(false);
        }
        cursor.take(12 * blocks)?;
        let (mut x_min, mut x_max) = (f64::NAN, f64::NAN);
        for _ in 0..(rows as usize).div_ceil(LEVEL1_ROWS) {
            x_min = fold_min(x_min, cursor.f64()?);
            x_max = fold_max(x_max, cursor.f64()?);
        }
        Ok(catalogue.chunk(
            index,
            ChunkRef {
                offset,
                first_row,
                rows,
                x_min,
                x_max,
            },
        ))
    }

    /// Decode block `block` (0 is x, `1 + slot` a value slot) of `chunk` into `out`.
    pub(crate) fn decode_block(
        &mut self,
        chunk: &ChunkRef,
        block: usize,
        out: &mut [f64],
    ) -> Result<(), StoreError> {
        let payload = chunk.offset + FRAME_HEADER_LEN as u64;
        let mut entry = [0u8; 12];
        self.io
            .read_at(payload + 24 + 12 * block as u64, &mut entry)?;
        let at = u32::from_le_bytes(entry[..4].try_into().unwrap_or_default());
        let len = u32::from_le_bytes(entry[4..8].try_into().unwrap_or_default()) as usize;
        let codec = Codec::from_u8(entry[8])
            .ok_or_else(|| StoreError::corrupt(payload, "unknown block codec"))?;
        let block_at = payload + u64::from(at);
        if block_at + len as u64 > self.size {
            return Err(StoreError::corrupt(
                payload,
                "a block runs past the end of the segment",
            ));
        }
        self.block.resize(len, 0);
        self.io.read_at(block_at, &mut self.block)?;
        codec::decode_f64(codec, &self.block, out, &mut self.planes, block_at)
    }

    pub(crate) fn chunk_containing(&self, row: u64) -> usize {
        self.catalogue
            .chunks
            .partition_point(|chunk| chunk.end_row() <= row)
    }

    fn read_block_range(
        &mut self,
        block: usize,
        first: u64,
        count: usize,
    ) -> Result<Vec<f64>, StoreError> {
        let end = first
            .checked_add(count as u64)
            .filter(|&end| end <= self.rows());
        if end.is_none() {
            return Err(StoreError::OutOfRange(format!(
                "rows {first}..{} of a segment with {} rows",
                first.saturating_add(count as u64),
                self.rows()
            )));
        }
        let mut out = Vec::with_capacity(count);
        let mut decoded = Vec::new();
        let mut index = self.chunk_containing(first);
        while out.len() < count {
            let chunk = self.catalogue.chunks[index];
            decoded.resize(chunk.rows as usize, 0.0);
            self.decode_block(&chunk, block, &mut decoded)?;
            let from = (first + out.len() as u64 - chunk.first_row) as usize;
            let take = (count - out.len()).min(chunk.rows as usize - from);
            out.extend_from_slice(&decoded[from..from + take]);
            index += 1;
        }
        Ok(out)
    }

    /// x of rows `first .. first + count`.
    pub fn read_x(&mut self, first: u64, count: usize) -> Result<Vec<f64>, StoreError> {
        self.read_block_range(0, first, count)
    }

    /// One value slot of rows `first .. first + count`.
    pub fn read_slot(
        &mut self,
        slot: usize,
        first: u64,
        count: usize,
    ) -> Result<Vec<f64>, StoreError> {
        if slot >= self.slots {
            return Err(StoreError::OutOfRange(format!(
                "slot {slot} of a {}-slot schema",
                self.slots
            )));
        }
        self.read_block_range(slot + 1, first, count)
    }

    /// Recompute the logical digest from the readable data; equal to `END`'s for a complete
    /// segment.
    pub fn digest(&mut self) -> Result<Digest, StoreError> {
        let mut digest = LogicalDigest::new(std::iter::empty());
        let mut x = Vec::new();
        let mut columns = vec![Vec::new(); self.slots];
        for index in 0..self.catalogue.chunks.len() {
            let chunk = self.catalogue.chunks[index];
            x.resize(chunk.rows as usize, 0.0);
            self.decode_block(&chunk, 0, &mut x)?;
            for (slot, column) in columns.iter_mut().enumerate() {
                column.resize(chunk.rows as usize, 0.0);
                self.decode_block(&chunk, slot + 1, column)?;
            }
            digest.columns(&x, &columns);
        }
        let mut schema_bytes = Vec::new();
        self.schema.encode(&mut schema_bytes);
        Ok(digest.finish(&schema_bytes))
    }
}
