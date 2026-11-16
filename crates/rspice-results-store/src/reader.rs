//! Opening a segment, recovering one a crash cut short, and decoding rows on demand.
//!
//! A complete segment opens from the superblock's `END` pointer alone: `END` carries a copy of
//! every index entry, so no index frame is read however long the run. Without a valid `END`, the
//! reader walks back from the last durable index the superblock names, checks every frame that
//! chain names whole against its CRC (writes before `END` reach the device in no order, so an
//! index can outlive the chunks it names), then scans forward frame by frame, accepting a frame
//! while its magic, length and CRC check out and it continues its stream (the next chunk index and
//! row, the next index in the chain). A provisional frame is accepted as the last data; nothing
//! real may follow it. What a crash leaves readable is therefore always a prefix of the rows
//! written.

use crate::codec::{self, Codec};
use crate::digest::{Digest, LogicalDigest};
use crate::events::{EventChange, EventKind, decode_events, hash_change, parse_events_header};
use crate::format::{
    Cursor, FORMAT_VERSION, FRAME_HEADER_LEN, FrameHeader, FrameKind, LEVEL1_ROWS, POINTERS_AT,
    SEGMENT_MAGIC, SUPERBLOCK_LEN, bucket_rows, fold_max, fold_min, frame_crc, summary_frame_rows,
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

/// Where one `SUMMARY` frame is and what it covers.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SummaryRef {
    pub(crate) offset: u64,
    pub(crate) first_row: u64,
    pub(crate) rows: u64,
    pub(crate) buckets: u32,
    pub(crate) x_min: f64,
    pub(crate) x_max: f64,
}

/// Where one `EVENTS` frame is and what it covers.
#[derive(Debug, Clone, Copy)]
pub(crate) struct EventsRef {
    pub(crate) offset: u64,
    pub(crate) first: u64,
    pub(crate) count: u32,
    pub(crate) t_min: f64,
    pub(crate) t_max: f64,
}

/// Every frame the reader found, in stream order.
#[derive(Default)]
struct Catalogue {
    chunks: Vec<ChunkRef>,
    /// `summaries[0]` is level 2.
    summaries: Vec<Vec<SummaryRef>>,
    /// Per event signal (in id order), its `EVENTS` frames.
    events: Vec<Vec<EventsRef>>,
    /// Per event signal, the changes in those frames.
    event_counts: Vec<u64>,
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

    /// Accept a summary frame only when it is the next of its level, follows a complete frame,
    /// and covers rows already read.
    /// Accept an `EVENTS` frame only when it continues its signal's changes.
    fn events(&mut self, position: usize, frame: EventsRef) -> bool {
        if self.events.len() <= position {
            self.events.resize_with(position + 1, Vec::new);
            self.event_counts.resize(position + 1, 0);
        }
        if frame.first != self.event_counts[position] || frame.count == 0 {
            return false;
        }
        self.event_counts[position] += u64::from(frame.count);
        self.events[position].push(frame);
        true
    }

    fn summary(&mut self, level: u8, summary: SummaryRef) -> bool {
        if level < 2 {
            return false;
        }
        let index = usize::from(level - 2);
        if index > self.summaries.len() {
            return false;
        }
        if index == self.summaries.len() {
            self.summaries.push(Vec::new());
        }
        let frames = &mut self.summaries[index];
        let frame_rows = summary_frame_rows(level);
        let fits = summary.first_row == frames.len() as u64 * frame_rows
            && summary.rows > 0
            && summary.rows <= frame_rows
            && summary.first_row + summary.rows <= self.rows
            && u64::from(summary.buckets) == summary.rows.div_ceil(bucket_rows(level))
            && frames.last().is_none_or(|last| last.rows == frame_rows);
        if fits {
            frames.push(summary);
        }
        fits
    }

    /// Drop every frame at `offset` or after it: what is left is the prefix of the stream before
    /// that point.
    fn truncate_at(&mut self, offset: u64) {
        self.chunks.retain(|chunk| chunk.offset < offset);
        self.rows = self.chunks.last().map_or(0, ChunkRef::end_row);
        let rows = self.rows;
        for frames in &mut self.summaries {
            frames.retain(|summary| {
                summary.offset < offset && summary.first_row + summary.rows <= rows
            });
        }
        while self.summaries.last().is_some_and(Vec::is_empty) {
            self.summaries.pop();
        }
        for (frames, count) in self.events.iter_mut().zip(&mut self.event_counts) {
            frames.retain(|frame| frame.offset < offset);
            *count = frames.iter().map(|frame| u64::from(frame.count)).sum();
        }
        self.last_index = self.last_index.filter(|&index| index < offset);
    }
}

/// What an `END` frame says.
struct EndRecord {
    status: RunStatus,
    rows: u64,
    chunks: u64,
    last_index: u64,
    digest: Digest,
    /// Per event signal: (signal id, changes).
    event_totals: Vec<(u32, u64)>,
    /// What the run said of itself, by key.
    annotations: Vec<(String, Vec<u8>)>,
}

/// Reads one segment.
pub struct SegmentReader<I: SegmentIo> {
    io: I,
    size: u64,
    schema: Schema,
    meta: RunMeta,
    slots: usize,
    analog: Vec<AnalogColumn>,
    /// Event signals in id order: (signal id, kind).
    event_signals: Vec<(u32, EventKind)>,
    catalogue: Catalogue,
    state: SegmentState,
    /// What the run said of itself in `END`; none for a segment recovered without it.
    annotations: Vec<(String, Vec<u8>)>,
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
            event_signals: Vec::new(),
            catalogue: Catalogue::default(),
            state: SegmentState::Recovered,
            annotations: Vec::new(),
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
        for (id, signal) in reader.schema.signals.iter().enumerate() {
            if let Some(kind) = EventKind::of(signal.kind) {
                reader.event_signals.push((id as u32, kind));
            }
        }

        if end_at >= body && reader.open_complete(end_at, body).is_ok() {
            return Ok(reader);
        }
        reader.recover(durable_index, body);
        Ok(reader)
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    #[cfg(test)]
    pub(crate) fn io(&self) -> &I {
        &self.io
    }

    pub fn run_meta(&self) -> &RunMeta {
        &self.meta
    }

    /// The bytes the run kept under `key` in `END`, if it ended with any.
    pub fn annotation(&self, key: &str) -> Option<&[u8]> {
        self.annotations
            .iter()
            .find(|(kept, _)| kept == key)
            .map(|(_, bytes)| bytes.as_slice())
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

    /// The chunks of the readable rows, in row order.
    pub(crate) fn chunk_refs(&self) -> &[ChunkRef] {
        &self.catalogue.chunks
    }

    /// The `SUMMARY` frames read, per level from level 2.
    pub(crate) fn summary_refs(&self) -> &[Vec<SummaryRef>] {
        &self.catalogue.summaries
    }

    /// The `EVENTS` frames of the event signal at `position` among the event signals.
    pub(crate) fn event_refs(&self, position: usize) -> &[EventsRef] {
        self.catalogue
            .events
            .get(position)
            .map_or(&[], Vec::as_slice)
    }

    /// The event signals in id order: (signal id, kind).
    pub(crate) fn event_kinds(&self) -> &[(u32, EventKind)] {
        &self.event_signals
    }

    /// The last `INDEX` frame read.
    pub(crate) fn last_index_offset(&self) -> Option<u64> {
        self.catalogue.last_index
    }

    /// The segment's I/O.
    pub(crate) fn into_io(self) -> I {
        self.io
    }

    /// Read and check the frame at `offset`.
    pub(crate) fn read_frame(&self, offset: u64) -> Result<(FrameHeader, Vec<u8>), StoreError> {
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
        let (end, directory) = Self::parse_end(&payload, end_at)?;
        if end.last_index < body || end.last_index >= end_at {
            return Err(StoreError::corrupt(end_at, "END names no index before it"));
        }
        let mut cursor = Cursor::new(
            &payload[directory..],
            end_at + (FRAME_HEADER_LEN + directory) as u64,
        );
        let count = cursor.count(INDEX_ENTRY_LEN)?;
        let mut catalogue = Catalogue::default();
        self.replay_entries(&mut catalogue, &mut cursor, count, end_at)?;
        catalogue.last_index = Some(end.last_index);
        if cursor.remaining() != 0 || !self.end_matches(&end, &catalogue) {
            return Err(StoreError::corrupt(
                end_at,
                "END's totals disagree with its frame directory",
            ));
        }
        self.catalogue = catalogue;
        self.state = SegmentState::Complete {
            status: end.status,
            digest: end.digest,
        };
        self.annotations = end.annotations;
        Ok(())
    }

    /// Parse `END` up to its frame directory; returns the record and where the directory starts.
    fn parse_end(payload: &[u8], offset: u64) -> Result<(EndRecord, usize), StoreError> {
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
        let count = cursor.count(16)?;
        let mut event_totals = Vec::with_capacity(count);
        for _ in 0..count {
            let signal = cursor.u32()?;
            cursor.u32()?;
            event_totals.push((signal, cursor.u64()?));
        }
        let count = cursor.count(8)?;
        let mut annotations = Vec::with_capacity(count);
        for _ in 0..count {
            let key = cursor.string()?;
            let len = cursor.u32()? as usize;
            annotations.push((key, cursor.take(len)?.to_vec()));
        }
        let record = EndRecord {
            status,
            rows,
            chunks,
            last_index,
            digest: Digest(digest),
            event_totals,
            annotations,
        };
        Ok((record, cursor.pos()))
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
        self.replay_entries(catalogue, &mut cursor, count, offset)?;
        if rows != catalogue.rows {
            return Err(cursor.error("an index's row count disagrees with its chunks"));
        }
        catalogue.last_index = Some(offset);
        Ok(())
    }

    /// Add `count` index entries, each naming a frame before offset `before`, to `catalogue`.
    fn replay_entries(
        &self,
        catalogue: &mut Catalogue,
        cursor: &mut Cursor<'_>,
        count: usize,
        before: u64,
    ) -> Result<(), StoreError> {
        for _ in 0..count {
            let kind = FrameKind::from_u8(cursor.u8()?)
                .ok_or_else(|| cursor.error("unknown frame kind"))?;
            let level = cursor.u8()?;
            cursor.u16()?;
            let signal = cursor.u32()?;
            let frame = cursor.u64()?;
            let first = cursor.u64()?;
            let count = cursor.u64()?;
            let x_min = cursor.f64()?;
            let x_max = cursor.f64()?;
            if kind == FrameKind::Events {
                let position = self.event_position(signal);
                let count = u32::try_from(count)
                    .map_err(|_| cursor.error("an EVENTS frame of more than 2^32 changes"))?;
                let events = EventsRef {
                    offset: frame,
                    first,
                    count,
                    t_min: x_min,
                    t_max: x_max,
                };
                if frame >= before
                    || position.is_none_or(|position| !catalogue.events(position, events))
                {
                    return Err(cursor.error("an index lists event changes out of order"));
                }
            }
            if kind == FrameKind::Summary {
                let buckets = count.div_ceil(bucket_rows(level.max(2))) as u32;
                let summary = SummaryRef {
                    offset: frame,
                    first_row: first,
                    rows: count,
                    buckets,
                    x_min,
                    x_max,
                };
                if frame >= before || !catalogue.summary(level, summary) {
                    return Err(cursor.error("an index lists a summary out of order"));
                }
            }
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
                if frame >= before || !catalogue.chunk(catalogue.chunks.len() as u64, chunk) {
                    return Err(cursor.error("an index lists a chunk out of order"));
                }
            }
        }
        Ok(())
    }

    /// Rebuild what a crash left: the durable index chain, then a forward scan.
    ///
    /// Writes before `END` reach the device in no particular order, so after a power loss an index
    /// can name frames that never did. Every frame the chain names is read whole and checked
    /// against its CRC here, before any row is served from it; the first that fails ends the
    /// readable prefix, and nothing after it is scanned.
    fn recover(&mut self, durable_index: u64, body: u64) {
        let (mut catalogue, mut at) = (Catalogue::default(), body);
        if durable_index >= body
            && let Ok((chain, after)) = self.load_chain(durable_index, body)
        {
            catalogue = chain;
            at = after;
            if let Some(unsound) = self.first_unsound_frame(&catalogue) {
                catalogue.truncate_at(unsound);
                self.catalogue = catalogue;
                self.state = SegmentState::Recovered;
                return;
            }
        }
        let end = self.scan(&mut catalogue, at);
        self.catalogue = catalogue;
        self.state = match end {
            Some(end) => {
                self.annotations = end.annotations;
                SegmentState::Complete {
                    status: end.status,
                    digest: end.digest,
                }
            }
            None => SegmentState::Recovered,
        };
    }

    /// The offset of the first frame `catalogue` names that is not, whole and checked, the frame
    /// its index entry describes; `None` when every one is.
    fn first_unsound_frame(&self, catalogue: &Catalogue) -> Option<u64> {
        let mut named: Vec<(u64, FrameKind, u64, u64)> = Vec::new();
        named.extend(catalogue.chunks.iter().map(|chunk| {
            (
                chunk.offset,
                FrameKind::Chunk,
                chunk.first_row,
                u64::from(chunk.rows),
            )
        }));
        for summary in catalogue.summaries.iter().flatten() {
            named.push((
                summary.offset,
                FrameKind::Summary,
                summary.first_row,
                summary.rows,
            ));
        }
        for frame in catalogue.events.iter().flatten() {
            named.push((
                frame.offset,
                FrameKind::Events,
                frame.first,
                u64::from(frame.count),
            ));
        }
        named.sort_unstable_by_key(|&(offset, ..)| offset);
        named.into_iter().find_map(|(offset, kind, first, count)| {
            let sound = self.read_frame(offset).is_ok_and(|(header, payload)| {
                header.kind == kind
                    && !header.partial()
                    && Self::frame_covers(kind, &payload, first, count)
            });
            (!sound).then_some(offset)
        })
    }

    /// Whether a checked frame's own header covers `first` and `count` (rows, or changes).
    fn frame_covers(kind: FrameKind, payload: &[u8], first: u64, count: u64) -> bool {
        let u32_at = |at: usize| {
            payload
                .get(at..at + 4)
                .map(|b| u64::from(u32::from_le_bytes(b.try_into().unwrap_or_default())))
        };
        let u64_at = |at: usize| {
            payload
                .get(at..at + 8)
                .map(|b| u64::from_le_bytes(b.try_into().unwrap_or_default()))
        };
        let (at_first, at_count) = match kind {
            FrameKind::Chunk => (u64_at(8), u32_at(16)),
            FrameKind::Summary => (u64_at(16), u64_at(24)),
            FrameKind::Events => (u64_at(8), u32_at(16)),
            _ => return false,
        };
        at_first == Some(first) && at_count == Some(count)
    }

    /// Accept frames from `at` while they check out and continue their streams.
    fn scan(&self, catalogue: &mut Catalogue, mut at: u64) -> Option<EndRecord> {
        while let Ok((header, payload)) = self.read_frame(at) {
            if catalogue.partial && !header.partial() {
                break;
            }
            let accepted = match header.kind {
                FrameKind::Chunk => self.scan_chunk(catalogue, at, &payload).unwrap_or(false),
                FrameKind::Summary if !header.partial() => {
                    self.scan_summary(catalogue, at, &payload).unwrap_or(false)
                }
                FrameKind::Events => self.scan_events(catalogue, at, &payload).unwrap_or(false),
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
                    return Self::parse_end(&payload, at)
                        .ok()
                        .map(|(end, _)| end)
                        .filter(|end| self.end_matches(end, catalogue));
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

    fn scan_summary(
        &self,
        catalogue: &mut Catalogue,
        offset: u64,
        payload: &[u8],
    ) -> Result<bool, StoreError> {
        let mut cursor = Cursor::new(payload, offset + FRAME_HEADER_LEN as u64);
        let level = cursor.u8()?;
        cursor.take(3)?;
        let buckets = cursor.u32()?;
        let _ordinal = cursor.u64()?;
        let first_row = cursor.u64()?;
        let rows = cursor.u64()?;
        if payload.len() != 32 + 16 * buckets as usize * (1 + self.analog.len()) {
            return Ok(false);
        }
        let (mut x_min, mut x_max) = (f64::NAN, f64::NAN);
        for _ in 0..buckets {
            x_min = fold_min(x_min, cursor.f64()?);
            x_max = fold_max(x_max, cursor.f64()?);
        }
        let summary = SummaryRef {
            offset,
            first_row,
            rows,
            buckets,
            x_min,
            x_max,
        };
        Ok(catalogue.summary(level, summary))
    }

    pub(crate) fn chunks(&self) -> &[ChunkRef] {
        &self.catalogue.chunks
    }

    /// The `SUMMARY` frames of `level` (2 and up), in row order.
    pub(crate) fn summaries(&self, level: u8) -> &[SummaryRef] {
        self.catalogue
            .summaries
            .get(usize::from(level).wrapping_sub(2))
            .map_or(&[], Vec::as_slice)
    }

    /// The highest summary level with frames, or 1 (the chunks' own) when there are none.
    pub(crate) fn top_level(&self) -> u8 {
        self.catalogue.summaries.len() as u8 + 1
    }

    pub(crate) fn analog(&self) -> &[AnalogColumn] {
        &self.analog
    }

    /// Read `count` little-endian f64 at `at`.
    pub(crate) fn read_f64s(
        &self,
        at: u64,
        count: usize,
        out: &mut Vec<f64>,
    ) -> Result<(), StoreError> {
        let mut bytes = vec![0u8; count * 8];
        if at + bytes.len() as u64 > self.size {
            return Err(StoreError::corrupt(
                at,
                "a summary runs past the end of the segment",
            ));
        }
        self.io.read_at(at, &mut bytes)?;
        out.clear();
        out.extend(
            bytes
                .chunks_exact(8)
                .map(|b| f64::from_le_bytes(b.try_into().unwrap_or_default())),
        );
        Ok(())
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
        let mut out = Vec::new();
        crate::error::reserve(&mut out, count, "the rows read back from a results segment")?;
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

    /// The chunks of the readable rows in row order: each one's first row and row count.
    ///
    /// A reader that streams a whole run reads it chunk by chunk with [`Self::read_chunk_x`] and
    /// [`Self::read_chunk_slot`], which decode each block once into the caller's buffer.
    pub fn chunk_spans(&self) -> impl ExactSizeIterator<Item = (u64, usize)> + '_ {
        self.catalogue
            .chunks
            .iter()
            .map(|chunk| (chunk.first_row, chunk.rows as usize))
    }

    /// x of every row of chunk `chunk`, into `out`, resized to the chunk's rows.
    pub fn read_chunk_x(&mut self, chunk: usize, out: &mut Vec<f64>) -> Result<(), StoreError> {
        self.read_chunk_block(chunk, 0, out)
    }

    /// Value slot `slot` of every row of chunk `chunk`, into `out`, resized to the chunk's rows.
    pub fn read_chunk_slot(
        &mut self,
        chunk: usize,
        slot: usize,
        out: &mut Vec<f64>,
    ) -> Result<(), StoreError> {
        if slot >= self.slots {
            return Err(StoreError::OutOfRange(format!(
                "slot {slot} of a {}-slot schema",
                self.slots
            )));
        }
        self.read_chunk_block(chunk, slot + 1, out)
    }

    fn read_chunk_block(
        &mut self,
        chunk: usize,
        block: usize,
        out: &mut Vec<f64>,
    ) -> Result<(), StoreError> {
        let Some(&chunk_ref) = self.catalogue.chunks.get(chunk) else {
            return Err(StoreError::OutOfRange(format!(
                "chunk {chunk} of a segment with {} chunks",
                self.catalogue.chunks.len()
            )));
        };
        out.resize(chunk_ref.rows as usize, 0.0);
        self.decode_block(&chunk_ref, block, out)
    }

    /// Recompute the logical digest from the readable data; equal to `END`'s for a complete
    /// segment.
    pub fn digest(&mut self) -> Result<Digest, StoreError> {
        let mut digest = LogicalDigest::new(self.event_signals.iter().map(|&(id, _)| id));
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
        for position in 0..self.event_signals.len() {
            let (id, kind) = self.event_signals[position];
            let changes = self.events(id as usize, f64::NEG_INFINITY, f64::INFINITY)?;
            let stream = digest.event_stream(position, changes.len() as u64);
            let mut value = Vec::new();
            for change in &changes {
                value.clear();
                change.value.put(&mut value);
                hash_change(
                    stream,
                    kind,
                    change.time,
                    change.sequence,
                    &value,
                    change.strength,
                );
            }
        }
        let mut schema_bytes = Vec::new();
        self.schema.encode(&mut schema_bytes);
        Ok(digest.finish(&schema_bytes))
    }

    /// Changes of event signal `signal` read so far.
    pub fn event_count(&self, signal: usize) -> Result<u64, StoreError> {
        let position = self
            .event_position(signal as u32)
            .ok_or_else(|| Self::not_event(signal))?;
        Ok(self
            .catalogue
            .event_counts
            .get(position)
            .copied()
            .unwrap_or(0))
    }

    fn not_event(signal: usize) -> StoreError {
        StoreError::OutOfRange(format!("signal {signal} is not an event signal"))
    }

    /// How many `EVENTS` frames event signal `signal` has. Its changes are those of frame 0,
    /// then frame 1, and so on ([`Self::read_event_frame`]), so a reader can walk a signal's
    /// whole history holding one frame of it.
    pub fn event_frames(&self, signal: usize) -> Result<usize, StoreError> {
        let position = self
            .event_position(signal as u32)
            .ok_or_else(|| Self::not_event(signal))?;
        Ok(self.catalogue.events.get(position).map_or(0, Vec::len))
    }

    /// The changes of event signal `signal`'s `frame`-th `EVENTS` frame, in commit order, into
    /// `out`, which is emptied first.
    pub fn read_event_frame(
        &self,
        signal: usize,
        frame: usize,
        out: &mut Vec<EventChange>,
    ) -> Result<(), StoreError> {
        let position = self
            .event_position(signal as u32)
            .ok_or_else(|| Self::not_event(signal))?;
        let kind = self.event_signals[position].1;
        let frame = self
            .catalogue
            .events
            .get(position)
            .and_then(|frames| frames.get(frame))
            .ok_or_else(|| {
                StoreError::OutOfRange(format!("event signal {signal} has no frame {frame}"))
            })?;
        let (_, payload) = self.read_frame(frame.offset)?;
        out.clear();
        decode_events(&payload, frame.offset, kind, out)
    }

    /// The changes of event signal `signal` with time in `t0 ..= t1`, in commit order.
    pub fn events(
        &mut self,
        signal: usize,
        t0: f64,
        t1: f64,
    ) -> Result<Vec<EventChange>, StoreError> {
        let position = self
            .event_position(signal as u32)
            .ok_or_else(|| Self::not_event(signal))?;
        let kind = self.event_signals[position].1;
        let mut changes = Vec::new();
        let frames = self
            .catalogue
            .events
            .get(position)
            .cloned()
            .unwrap_or_default();
        for frame in frames {
            if !(frame.t_max >= t0 && frame.t_min <= t1) {
                continue;
            }
            let (_, payload) = self.read_frame(frame.offset)?;
            let start = changes.len();
            decode_events(&payload, frame.offset, kind, &mut changes)?;
            let mut keep = start;
            for at in start..changes.len() {
                if changes[at].time >= t0 && changes[at].time <= t1 {
                    changes.swap(keep, at);
                    keep += 1;
                }
            }
            changes.truncate(keep);
        }
        Ok(changes)
    }

    /// The position of signal id `signal` among the event signals.
    fn event_position(&self, signal: u32) -> Option<usize> {
        self.event_signals.iter().position(|&(id, _)| id == signal)
    }

    fn end_matches(&self, end: &EndRecord, catalogue: &Catalogue) -> bool {
        end.rows == catalogue.rows
            && end.chunks == catalogue.chunks.len() as u64
            && end.event_totals.len() == self.event_signals.len()
            && end
                .event_totals
                .iter()
                .enumerate()
                .all(|(position, &(id, count))| {
                    id == self.event_signals[position].0
                        && catalogue.event_counts.get(position).copied().unwrap_or(0) == count
                })
    }

    fn scan_events(
        &self,
        catalogue: &mut Catalogue,
        offset: u64,
        payload: &[u8],
    ) -> Result<bool, StoreError> {
        let header = parse_events_header(payload, offset)?;
        let Some(position) = self.event_position(header.signal) else {
            return Ok(false);
        };
        let events = EventsRef {
            offset,
            first: header.first,
            count: header.count,
            t_min: header.t_min,
            t_max: header.t_max,
        };
        Ok(catalogue.events(position, events))
    }
}
