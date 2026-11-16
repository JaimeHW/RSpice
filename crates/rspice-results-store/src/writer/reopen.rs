//! Reopening a segment to append to it, as a resumed run does.
//!
//! A resume continues a run from a checkpoint taken at one of its rows. The segment keeps every row
//! and event change up to that row and drops the rest, which a run that went on past the
//! checkpoint (or died after it) wrote. The frames are cut in place at the chunk holding the row:
//! that chunk's rows up to the row, and the changes up to it in the frames cut, are read first
//! (one chunk, and the changes of about one chunk) and become the open chunk and the open event
//! buffers again. The writer's state at the cut is rebuilt from what the segment holds before it:
//! the digest streams from the trailers of the last chunk and of each signal's last `EVENTS`
//! frame, the summary pyramid from at most a frame's worth of buckets per level, and the index
//! from the frames since the last `INDEX`. Nothing before the cut is rewritten.
//!
//! The kept rows and changes go back as provisional frames written over the cut point before the
//! stale frames after them are truncated, and both steps are synced, so a crash at any point
//! leaves a segment that reads to its last whole chunk and still holds the resume's row, unless
//! the crash tears that one provisional write itself.

use super::{Batch, Encoder, IndexEntry, Layout};
use crate::digest::{HASH_STATE_LEN, ValueHasher};
use crate::events::{EventBuffer, EventChange, decode_events};
use crate::format::{FrameKind, LEVEL1_ROWS};
use crate::reader::{ChunkRef, EventsRef, SegmentReader};
use crate::summary::{Pyramid, StoredBucket};
use crate::{SegmentIo, StoreError};

/// The rows and changes a reopened segment keeps after its cut, which the caller stages again.
pub(crate) struct Kept {
    pub(crate) batch: Batch,
    pub(crate) changes: Vec<EventChange>,
}

impl<I: SegmentIo> Encoder<I> {
    /// Reopen the segment on `io` after its row at `through` (bit for bit): keep every row and
    /// event change up to it, drop every one after it, and continue from there with chunks of
    /// `chunk_rows` rows (the schema's own size without one). A segment with no row at `through`
    /// is left as it was and refused with [`StoreError::NoRowAt`].
    pub(crate) fn reopen(
        io: I,
        through: f64,
        chunk_rows: Option<usize>,
        compress: bool,
    ) -> Result<(Self, Kept), StoreError> {
        let mut reader = SegmentReader::open(io)?;
        let chunk_rows = chunk_rows.unwrap_or_else(|| super::chunk_rows(reader.slots()));
        let no_row = StoreError::NoRowAt {
            x_bits: through.to_bits(),
        };
        let chunks = reader.chunk_refs().to_vec();
        let Some(k) = chunks.iter().rposition(|chunk| chunk.x_min <= through) else {
            return Err(no_row);
        };
        let mut x = Vec::new();
        reader.read_chunk_x(k, &mut x)?;
        let Some(last) = x.iter().rposition(|&value| value <= through) else {
            return Err(no_row);
        };
        if x[last].to_bits() != through.to_bits() {
            return Err(no_row);
        }
        if chunks[..k]
            .iter()
            .any(|chunk| chunk.rows as usize != chunk_rows)
        {
            return Err(StoreError::Schema(format!(
                "the segment's chunks are not {chunk_rows} rows, so it cannot be reopened"
            )));
        }
        let schema = reader.schema().clone();
        let layout = Layout::new(&schema, chunk_rows);
        let rows = chunks[k].first_row;

        // The cut: the chunk holding the row, or an earlier `EVENTS` frame with a change after it
        // or written provisionally (a provisional frame ends the readable frames, so none of
        // them stays before the rows the writer appends).
        let mut cut = chunks[k].offset;
        let floor = k
            .checked_sub(1)
            .map_or(0, |previous| chunks[previous].offset);
        for position in 0..reader.event_kinds().len() {
            for frame in reader.event_refs(position) {
                if frame.offset <= floor && frame.t_max > through {
                    return Err(StoreError::corrupt(
                        frame.offset,
                        "an event frame before the resume's chunk holds changes after it",
                    ));
                }
                if frame.offset > floor
                    && frame.offset < cut
                    && (frame.t_max > through || reader.read_frame(frame.offset)?.0.partial())
                {
                    cut = frame.offset;
                }
            }
        }

        // What the cut drops but the segment keeps: the chunk's rows up to the row, and every
        // change up to it in the frames from the cut on.
        let mut batch = Batch::new(chunk_rows, layout.slots);
        batch.first_row = rows;
        let kept_rows = last + 1;
        let mut column = Vec::new();
        for (row, &value) in x[..kept_rows].iter().enumerate() {
            batch.x[row] = value;
        }
        for slot in 0..layout.slots {
            reader.read_chunk_slot(k, slot, &mut column)?;
            for (row, &value) in column[..kept_rows].iter().enumerate() {
                batch.values[row * layout.slots + slot] = value;
            }
        }
        batch.rows = kept_rows;
        let mut changes = Vec::new();
        let mut buffers = Vec::with_capacity(layout.events.len());
        let mut streams = Vec::with_capacity(layout.events.len());
        for (position, &(signal, kind)) in reader.event_kinds().to_vec().iter().enumerate() {
            let frames = reader.event_refs(position).to_vec();
            let (kept, dropped): (Vec<&EventsRef>, Vec<&EventsRef>) =
                frames.iter().partition(|frame| frame.offset < cut);
            for frame in dropped {
                let (_, payload) = reader.read_frame(frame.offset)?;
                let start = changes.len();
                decode_events(&payload, frame.offset, kind, &mut changes)?;
                let mut keep = start;
                for at in start..changes.len() {
                    if changes[at].time <= through {
                        changes.swap(keep, at);
                        keep += 1;
                    }
                }
                changes.truncate(keep);
            }
            let mut buffer = EventBuffer::new(signal, kind);
            let stream = match kept.last() {
                Some(frame) => {
                    let (_, payload) = reader.read_frame(frame.offset)?;
                    let mut last = Vec::new();
                    decode_events(&payload, frame.offset, kind, &mut last)?;
                    let written = kept.iter().map(|frame| u64::from(frame.count)).sum();
                    let change = last.last().ok_or_else(|| {
                        StoreError::corrupt(frame.offset, "an EVENTS frame without changes")
                    })?;
                    buffer.continue_after(written, change);
                    Some((trailer(&payload, frame.offset)?, written))
                }
                None => None,
            };
            buffers.push(buffer);
            streams.push(stream);
        }

        // The index: the chain of `INDEX` frames before the cut, and the frames since its last.
        let mut index_frames = Vec::new();
        let mut index_rows = 0;
        let mut at = reader.last_index_offset();
        while let Some(offset) = at {
            let (_, payload) = reader.read_frame(offset)?;
            let previous = u64_le(&payload, 0, offset)?;
            if offset < cut {
                if index_frames.is_empty() {
                    index_rows = u64_le(&payload, 16, offset)?;
                }
                let count =
                    u32::try_from(u64_le(&payload, 24, offset)? & 0xffff_ffff).unwrap_or(u32::MAX);
                index_frames.push((offset, count));
            }
            at = (previous != u64::MAX).then_some(previous);
        }
        index_frames.reverse();
        let last_index = index_frames.last().map(|&(offset, _)| offset);
        let since = last_index.unwrap_or(0);
        let mut entries = Vec::new();
        for chunk in &chunks[..k] {
            if chunk.offset > since {
                entries.push(chunk_entry(chunk));
            }
        }
        for (index, frames) in reader.summary_refs().iter().enumerate() {
            for frame in frames {
                if frame.offset > since && frame.offset < cut {
                    entries.push(IndexEntry {
                        kind: FrameKind::Summary,
                        level: index as u8 + 2,
                        signal: 0,
                        offset: frame.offset,
                        first: frame.first_row,
                        count: frame.rows,
                        x_min: frame.x_min,
                        x_max: frame.x_max,
                    });
                }
            }
        }
        for (position, &(signal, _)) in reader.event_kinds().iter().enumerate() {
            for frame in reader.event_refs(position) {
                if frame.offset > since && frame.offset < cut {
                    entries.push(IndexEntry {
                        kind: FrameKind::Events,
                        level: 0,
                        signal,
                        offset: frame.offset,
                        first: frame.first,
                        count: u64::from(frame.count),
                        x_min: frame.t_min,
                        x_max: frame.t_max,
                    });
                }
            }
        }
        entries.sort_unstable_by_key(|entry| entry.offset);

        // The summary pyramid at the cut, from the buckets the segment stores.
        let signals = layout.analog.len();
        let pyramid = {
            let reader = &reader;
            let chunks = &chunks[..k];
            Pyramid::rebuild(signals, rows, &mut |level, first, end| {
                stored_buckets(reader, chunks, signals, (level, first, end))
            })?
        };

        // The digest streams at the cut.
        let rows_stream = match k.checked_sub(1) {
            Some(previous) => {
                let (_, payload) = reader.read_frame(chunks[previous].offset)?;
                Some(trailer(&payload, chunks[previous].offset)?)
            }
            None => None,
        };

        let io = reader.into_io();
        let file_end = io.size()?;
        let chunks_since_index = entries
            .iter()
            .filter(|entry| entry.kind == FrameKind::Chunk)
            .count();
        let mut encoder = Self::with_layout(io, layout, compress);
        encoder.tail = cut;
        encoder.file_end = file_end;
        encoder.next_chunk = k as u64;
        encoder.rows = rows;
        encoder.entries = entries;
        encoder.chunks_since_index = chunks_since_index;
        encoder.index_first_row = index_rows;
        encoder.last_index = last_index;
        encoder.index_frames = index_frames;
        encoder.pyramid = pyramid;
        encoder.events = buffers;
        if let Some(stream) = rows_stream {
            encoder.digest.resume_rows(stream, rows);
        }
        for (position, stream) in streams.into_iter().enumerate() {
            if let Some((stream, count)) = stream {
                encoder.digest.resume_events(position, stream, count);
            }
        }
        // The superblock stops naming an `END` or an index past the cut before anything is cut.
        encoder.write_pointers(0)?;
        encoder.io.flush()?;
        Ok((encoder, Kept { batch, changes }))
    }

    /// Stage what [`Self::reopen`] kept after the cut, written over it as provisional frames, then
    /// drop the stale frames after them; both steps are synced. A chunk the kept rows fill is
    /// written whole, as the run would have.
    pub(crate) fn restage(
        &mut self,
        batch: &mut Batch,
        changes: &[EventChange],
    ) -> Result<(), StoreError> {
        // A run pushes the changes at a row's x after the row, so the changes at the last kept
        // row follow a chunk that row completes.
        let full = batch.rows == self.layout.chunk_rows;
        let last = batch.rows.checked_sub(1).map(|row| batch.x[row]);
        let (before, after): (Vec<&EventChange>, Vec<&EventChange>) = changes
            .iter()
            .partition(|change| !full || last.is_none_or(|last| change.time < last));
        for change in before {
            self.restage_change(change)?;
        }
        if full {
            self.write_batch(batch)?;
            batch.reset(self.rows);
        }
        for change in after {
            self.restage_change(change)?;
        }
        // `write_provisional` truncates only past its frames; the stale frames go once they are
        // on the device.
        let stale_end = self.file_end;
        self.file_end = self.tail;
        self.write_provisional(batch)?;
        self.io.flush()?;
        let end = self.file_end;
        if stale_end > end {
            self.io.truncate(end)?;
        }
        self.io.flush()?;
        Ok(())
    }
}

impl<I: SegmentIo> Encoder<I> {
    fn restage_change(&mut self, change: &EventChange) -> Result<(), StoreError> {
        let position = self.layout.event_index[change.signal as usize] as usize;
        if self.events[position].push(change) {
            self.write_events(position)?;
        }
        Ok(())
    }
}

/// The digest state a real `CHUNK` or `EVENTS` frame ends with.
fn trailer(payload: &[u8], offset: u64) -> Result<ValueHasher, StoreError> {
    payload
        .len()
        .checked_sub(HASH_STATE_LEN)
        .and_then(|at| ValueHasher::resumed(&payload[at..]))
        .ok_or_else(|| StoreError::corrupt(offset, "a frame's digest state does not decode"))
}

fn u64_le(payload: &[u8], at: usize, offset: u64) -> Result<u64, StoreError> {
    payload
        .get(at..at + 8)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or_else(|| StoreError::corrupt(offset, "an index frame is too short"))
}

fn chunk_entry(chunk: &ChunkRef) -> IndexEntry {
    IndexEntry {
        kind: FrameKind::Chunk,
        level: 0,
        signal: 0,
        offset: chunk.offset,
        first: chunk.first_row,
        count: u64::from(chunk.rows),
        x_min: chunk.x_min,
        x_max: chunk.x_max,
    }
}

/// Level `level`'s stored buckets over rows `first .. end`: the level-1 summaries of the chunks
/// there, or the buckets of the `SUMMARY` frames there.
fn stored_buckets<I: SegmentIo>(
    reader: &SegmentReader<I>,
    chunks: &[ChunkRef],
    signals: usize,
    (level, first, end): (u8, u64, u64),
) -> Result<Vec<StoredBucket>, StoreError> {
    let mut buckets = Vec::new();
    // Each frame holds `count` buckets: x pairs from `x_at`, then each signal's pairs.
    let mut take = |payload: &[u8], offset: u64, x_at: usize, count: usize| {
        let value = |at: usize| {
            payload
                .get(at..at + 8)
                .and_then(|bytes| bytes.try_into().ok())
                .map(f64::from_le_bytes)
                .ok_or_else(|| StoreError::corrupt(offset, "a summary runs past its frame"))
        };
        for bucket in 0..count {
            let x = x_at + 16 * bucket;
            let mut pairs = Vec::with_capacity(2 * signals);
            for signal in 0..signals {
                let at = x_at + 16 * count * (signal + 1) + 16 * bucket;
                pairs.push(value(at)?);
                pairs.push(value(at + 8)?);
            }
            buckets.push((value(x)?, value(x + 8)?, pairs));
        }
        Ok::<(), StoreError>(())
    };
    if level == 1 {
        for chunk in chunks
            .iter()
            .filter(|chunk| chunk.first_row >= first && chunk.end_row() <= end)
        {
            let (_, payload) = reader.read_frame(chunk.offset)?;
            let blocks = u32::from_le_bytes(
                payload
                    .get(20..24)
                    .and_then(|bytes| bytes.try_into().ok())
                    .ok_or_else(|| StoreError::corrupt(chunk.offset, "a chunk is too short"))?,
            ) as usize;
            let count = (chunk.rows as usize).div_ceil(LEVEL1_ROWS);
            take(&payload, chunk.offset, 24 + 12 * blocks, count)?;
        }
    } else {
        let frames = reader
            .summary_refs()
            .get(usize::from(level - 2))
            .map_or(&[][..], Vec::as_slice);
        for frame in frames
            .iter()
            .filter(|frame| frame.first_row >= first && frame.first_row + frame.rows <= end)
        {
            let (_, payload) = reader.read_frame(frame.offset)?;
            take(&payload, frame.offset, 32, frame.buckets as usize)?;
        }
    }
    Ok(buckets)
}
