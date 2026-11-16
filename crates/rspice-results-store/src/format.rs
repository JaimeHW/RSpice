//! The segment's byte layout: superblock, frame headers, design constants and little-endian helpers.
//!
//! Every integer is little-endian and every float IEEE-754 binary64 little-endian. Frames start at
//! 8-byte aligned offsets. A frame is a 16-byte header {magic u32, kind u8, flags u8, reserved u16,
//! payload length u32, CRC-32 u32} followed by its payload and zero padding to the next multiple of
//! eight. The CRC covers the first twelve header bytes and the payload, so a torn or flipped header
//! is caught as surely as a damaged payload.

use crate::StoreError;

/// Superblock: magic, format version, then the durable pointers the writer rewrites in place.
pub(crate) const SEGMENT_MAGIC: [u8; 8] = *b"RSPSEG\r\n";
pub(crate) const FORMAT_VERSION: u32 = 3;
pub(crate) const SUPERBLOCK_LEN: u64 = 64;
/// Offset of the pointer block: last durable index offset u64, rows it covers u64, END offset u64,
/// reserved u64. It is written as one aligned 32-byte write; zero means "none".
pub(crate) const POINTERS_AT: u64 = 16;
pub(crate) const POINTERS_LEN: usize = 32;

pub(crate) const FRAME_MAGIC: [u8; 4] = *b"RSFR";
pub(crate) const FRAME_HEADER_LEN: usize = 16;
/// A provisional frame: the open chunk or open event buffers, written for durability and overwritten
/// by the next real frame.
pub(crate) const FLAG_PARTIAL: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum FrameKind {
    Schema = 1,
    RunMeta = 2,
    Chunk = 3,
    Events = 4,
    Summary = 5,
    Index = 6,
    End = 7,
}

impl FrameKind {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        Some(match value {
            1 => Self::Schema,
            2 => Self::RunMeta,
            3 => Self::Chunk,
            4 => Self::Events,
            5 => Self::Summary,
            6 => Self::Index,
            7 => Self::End,
            _ => return None,
        })
    }
}

/// Rows per level-1 summary bucket; level-1 summaries live in each `CHUNK`.
pub(crate) const LEVEL1_ROWS: usize = 64;
/// Each summary level's bucket spans this many buckets of the level below.
pub(crate) const SUMMARY_FANOUT: u64 = 16;
/// Buckets per `SUMMARY` frame.
pub(crate) const SUMMARY_FRAME_BUCKETS: usize = 64;
/// An `INDEX` frame is written after this many chunks ...
pub(crate) const INDEX_CHUNKS: usize = 64;
/// ... or as soon as this many frames wait for one, so the writer's index memory stays bounded when
/// event frames outnumber chunks.
pub(crate) const INDEX_ENTRY_LIMIT: usize = 4096;

/// Rows in one bucket of summary `level` (level 1 is 64 rows; each level above is 16 times wider).
pub(crate) fn bucket_rows(level: u8) -> u64 {
    LEVEL1_ROWS as u64 * SUMMARY_FANOUT.pow(u32::from(level.saturating_sub(1)))
}

/// Rows one `SUMMARY` frame of `level` covers when it is complete.
pub(crate) fn summary_frame_rows(level: u8) -> u64 {
    bucket_rows(level) * SUMMARY_FRAME_BUCKETS as u64
}

/// Rows per chunk: `clamp(floor_pow2(1 Mi / slots), 256, 4096)`.
///
/// A design constant derived from the value count, so one chunk buffer stays near 1 Mi values
/// (8 MiB) however wide the row is. It is never a limit on what a run can save.
pub(crate) fn chunk_rows(slots: usize) -> usize {
    let per_chunk = ((1usize << 20) / slots.max(1)).max(1);
    let floor_pow2 = 1usize << (usize::BITS - 1 - per_chunk.leading_zeros());
    floor_pow2.clamp(256, 4096)
}

pub(crate) fn align8(offset: u64) -> u64 {
    (offset + 7) & !7
}

pub(crate) const CANONICAL_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;

/// `min` that ignores NaN, keeps the earlier operand on ties, and so gives the same bits on every
/// machine (`f64::min` may return either zero for `-0.0` against `+0.0`).
#[inline(always)]
pub(crate) fn fold_min(acc: f64, value: f64) -> f64 {
    if value < acc || acc.is_nan() {
        value
    } else {
        acc
    }
}

/// `max` with the same rules as [`fold_min`].
#[inline(always)]
pub(crate) fn fold_max(acc: f64, value: f64) -> f64 {
    if value > acc || acc.is_nan() {
        value
    } else {
        acc
    }
}

/// A NaN from a fold becomes the one canonical NaN, so summaries never carry payload bits.
#[inline(always)]
pub(crate) fn canonical(value: f64) -> f64 {
    if value.is_nan() {
        f64::from_bits(CANONICAL_NAN_BITS)
    } else {
        value
    }
}

/// A frame under construction in a reusable buffer. The buffer only grows; chunk frames are sized
/// once for the largest chunk, so steady-state writing never allocates.
pub(crate) struct FrameBuf {
    buf: Vec<u8>,
    len: usize,
}

impl FrameBuf {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity.max(FRAME_HEADER_LEN + 8)],
            len: 0,
        }
    }

    pub(crate) fn begin(&mut self, kind: FrameKind, flags: u8) {
        self.len = 0;
        self.reserve(FRAME_HEADER_LEN);
        self.buf[..4].copy_from_slice(&FRAME_MAGIC);
        self.buf[4] = kind as u8;
        self.buf[5] = flags;
        self.buf[6..FRAME_HEADER_LEN].fill(0);
        self.len = FRAME_HEADER_LEN;
    }

    #[inline]
    fn reserve(&mut self, extra: usize) {
        let need = self.len + extra;
        if need > self.buf.len() {
            self.buf.resize(need.max(self.buf.len() * 2), 0);
        }
    }

    /// Payload bytes written so far.
    pub(crate) fn payload_len(&self) -> usize {
        self.len - FRAME_HEADER_LEN
    }

    #[inline]
    pub(crate) fn put(&mut self, bytes: &[u8]) {
        self.reserve(bytes.len());
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
    }

    #[inline]
    pub(crate) fn put_u8(&mut self, value: u8) {
        self.put(&[value]);
    }
    #[inline]
    pub(crate) fn put_u32(&mut self, value: u32) {
        self.put(&value.to_le_bytes());
    }
    #[inline]
    pub(crate) fn put_u64(&mut self, value: u64) {
        self.put(&value.to_le_bytes());
    }
    #[inline]
    pub(crate) fn put_f64(&mut self, value: f64) {
        self.put(&value.to_le_bytes());
    }

    pub(crate) fn put_zeros(&mut self, count: usize) {
        self.reserve(count);
        self.buf[self.len..self.len + count].fill(0);
        self.len += count;
    }

    /// Writable space of `len` bytes at the end, for an encoder to fill; commit with [`Self::advance`].
    pub(crate) fn spare(&mut self, len: usize) -> &mut [u8] {
        self.reserve(len);
        &mut self.buf[self.len..self.len + len]
    }

    pub(crate) fn advance(&mut self, count: usize) {
        debug_assert!(self.len + count <= self.buf.len());
        self.len += count;
    }

    /// Overwrite four payload bytes at payload offset `at`.
    pub(crate) fn patch_u32(&mut self, at: usize, value: u32) {
        let at = FRAME_HEADER_LEN + at;
        self.buf[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub(crate) fn patch_u8(&mut self, at: usize, value: u8) {
        self.buf[FRAME_HEADER_LEN + at] = value;
    }

    /// Fill in length and CRC, pad to eight bytes, and return the frame's bytes.
    pub(crate) fn finish(&mut self) -> Result<&[u8], StoreError> {
        let payload = self.payload_len();
        let payload_len = u32::try_from(payload).map_err(|_| {
            StoreError::Schema(format!(
                "a {payload}-byte frame exceeds the 4 GiB frame limit"
            ))
        })?;
        self.buf[8..12].copy_from_slice(&payload_len.to_le_bytes());
        let crc = frame_crc(
            &self.buf[..FRAME_HEADER_LEN],
            &self.buf[FRAME_HEADER_LEN..self.len],
        );
        self.buf[12..16].copy_from_slice(&crc.to_le_bytes());
        let padded = align8(self.len as u64) as usize;
        let pad = padded - self.len;
        self.put_zeros(pad);
        Ok(&self.buf[..self.len])
    }
}

pub(crate) fn frame_crc(header: &[u8], payload: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&header[..12]);
    hasher.update(payload);
    hasher.finalize()
}

/// A decoded frame header.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FrameHeader {
    pub(crate) kind: FrameKind,
    pub(crate) flags: u8,
    pub(crate) payload_len: u32,
    pub(crate) crc: u32,
}

impl FrameHeader {
    pub(crate) fn parse(bytes: &[u8; FRAME_HEADER_LEN]) -> Option<Self> {
        if bytes[..4] != FRAME_MAGIC || bytes[6] != 0 || bytes[7] != 0 {
            return None;
        }
        Some(Self {
            kind: FrameKind::from_u8(bytes[4])?,
            flags: bytes[5],
            payload_len: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            crc: u32::from_le_bytes(bytes[12..16].try_into().ok()?),
        })
    }

    pub(crate) fn partial(&self) -> bool {
        self.flags & FLAG_PARTIAL != 0
    }

    /// Offset of the next frame after this one at `offset`.
    pub(crate) fn next(&self, offset: u64) -> u64 {
        align8(offset + FRAME_HEADER_LEN as u64 + u64::from(self.payload_len))
    }
}

/// A bounds-checked little-endian reader over a payload; every short read is a typed corruption.
pub(crate) struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
    /// Segment offset of `bytes[0]`, for error messages.
    base: u64,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(bytes: &'a [u8], base: u64) -> Self {
        Self {
            bytes,
            pos: 0,
            base,
        }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    pub(crate) fn error(&self, reason: &str) -> StoreError {
        StoreError::corrupt(self.base + self.pos as u64, reason)
    }

    pub(crate) fn take(&mut self, len: usize) -> Result<&'a [u8], StoreError> {
        let end = self
            .pos
            .checked_add(len)
            .filter(|&end| end <= self.bytes.len());
        let end = end.ok_or_else(|| self.error("a field runs past the end of its frame"))?;
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    pub(crate) fn u8(&mut self) -> Result<u8, StoreError> {
        Ok(self.take(1)?[0])
    }
    pub(crate) fn u16(&mut self) -> Result<u16, StoreError> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().unwrap_or_default(),
        ))
    }
    pub(crate) fn u32(&mut self) -> Result<u32, StoreError> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().unwrap_or_default(),
        ))
    }
    pub(crate) fn u64(&mut self) -> Result<u64, StoreError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().unwrap_or_default(),
        ))
    }
    pub(crate) fn f64(&mut self) -> Result<f64, StoreError> {
        Ok(f64::from_bits(self.u64()?))
    }
    pub(crate) fn string(&mut self) -> Result<String, StoreError> {
        let len = self.u32()? as usize;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| self.error("a name is not UTF-8"))
    }
    /// A count of items each at least `item_bytes` long, checked against what is left.
    pub(crate) fn count(&mut self, item_bytes: usize) -> Result<usize, StoreError> {
        let count = self.u32()? as usize;
        if count.saturating_mul(item_bytes.max(1)) > self.remaining() {
            return Err(self.error("a count exceeds the bytes left in its frame"));
        }
        Ok(count)
    }
}

/// Little-endian appends for building payloads in a plain `Vec<u8>`.
pub(crate) trait PutLe {
    fn put_u8(&mut self, value: u8);
    fn put_u32(&mut self, value: u32);
    fn put_u64(&mut self, value: u64);
    fn put_f64(&mut self, value: f64);
    fn put_str(&mut self, value: &str);
}

impl PutLe for Vec<u8> {
    fn put_u8(&mut self, value: u8) {
        self.push(value);
    }
    fn put_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }
    fn put_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }
    fn put_f64(&mut self, value: f64) {
        self.extend_from_slice(&value.to_le_bytes());
    }
    fn put_str(&mut self, value: &str) {
        self.put_u32(value.len() as u32);
        self.extend_from_slice(value.as_bytes());
    }
}
