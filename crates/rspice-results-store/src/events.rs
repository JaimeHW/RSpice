//! Event signals: digital nets, real-valued event nets and buses, stored as `EVENTS` frames.
//!
//! Each event signal buffers its changes; a buffer becomes a frame when it reaches 4,096 changes, or
//! at the end of the chunk in which it filled. A frame holds one signal's changes as four columns
//! (time, commit sequence within one time, value, strength), the value before its first change,
//! and a summary of every 1,024 changes (time span, count, values seen).

use crate::codec::{self, Codec};
use crate::digest::{ValueHasher, canonical_bits};
use crate::format::{Cursor, FRAME_HEADER_LEN, FrameBuf, FrameKind, canonical, fold_max, fold_min};
use crate::{SignalKind, StoreError};

/// Changes per `EVENTS` frame at most.
pub(crate) const EVENTS_PER_FRAME: usize = 4096;
/// Changes per event summary bucket.
pub(crate) const EVENTS_PER_BUCKET: usize = 1024;

/// A four-state logic value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Logic {
    Zero,
    One,
    X,
    Z,
}

impl Logic {
    fn code(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::X => 2,
            Self::Z => 3,
        }
    }

    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::X,
            3 => Self::Z,
            _ => return None,
        })
    }
}

/// The value an event signal changes to.
#[derive(Debug, Clone, PartialEq)]
pub enum EventValue {
    Logic(Logic),
    Real(f64),
    /// A bus of any width, as three bit strings of `width.div_ceil(8)` bytes each, least
    /// significant byte first (bit `i` is bit `i % 8` of byte `i / 8`): bit `i` is X when `x` has
    /// it set, else Z when `z` has it set, else the bit of `value`.
    Bus {
        value: Vec<u8>,
        x: Vec<u8>,
        z: Vec<u8>,
    },
}

impl EventValue {
    /// Append the value as stored: the logic code, the real's bits, or the bus's value, X and Z.
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        match self {
            Self::Logic(logic) => out.push(logic.code()),
            Self::Real(value) => out.extend_from_slice(&value.to_bits().to_le_bytes()),
            Self::Bus { value, x, z } => {
                out.extend_from_slice(value);
                out.extend_from_slice(x);
                out.extend_from_slice(z);
            }
        }
    }
}

/// One change of one event signal.
#[derive(Debug, Clone, PartialEq)]
pub struct EventChange {
    /// The signal's id: its position in the schema.
    pub signal: u32,
    pub time: f64,
    /// Commit order among changes at the same time.
    pub sequence: u32,
    pub value: EventValue,
    /// Drive strength: Verilog's eight levels (0 highz to 7 supply) or XSPICE's.
    pub strength: u8,
}

/// The value kinds an event signal stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EventKind {
    Logic,
    Real,
    Bus { width: u32 },
}

impl EventKind {
    pub(crate) fn of(kind: SignalKind) -> Option<Self> {
        match kind {
            SignalKind::Logic => Some(Self::Logic),
            SignalKind::RealEvent => Some(Self::Real),
            SignalKind::Bus { width } => Some(Self::Bus { width }),
            SignalKind::Real | SignalKind::Complex => None,
        }
    }

    /// Check that `value` is this kind's, and a bus value fits the width.
    pub(crate) fn admits(self, value: &EventValue) -> Result<(), &'static str> {
        match (self, value) {
            (Self::Logic, EventValue::Logic(_)) | (Self::Real, EventValue::Real(_)) => Ok(()),
            (Self::Bus { width }, EventValue::Bus { value, x, z }) => {
                let bytes = width.div_ceil(8) as usize;
                if value.len() != bytes || x.len() != bytes || z.len() != bytes {
                    return Err("a bus value is not the bus's width");
                }
                let above = if width % 8 == 0 {
                    0
                } else {
                    0xffu8 << (width % 8)
                };
                if (value[bytes - 1] | x[bytes - 1] | z[bytes - 1]) & above != 0 {
                    return Err("a bus value sets bits above the bus width");
                }
                Ok(())
            }
            _ => Err("the value's kind is not the signal's"),
        }
    }

    fn code(self) -> u8 {
        match self {
            Self::Logic => 0,
            Self::Real => 1,
            Self::Bus { .. } => 2,
        }
    }

    /// Bytes per stored value: three bit strings for a bus.
    fn value_bytes(self) -> usize {
        match self {
            Self::Logic => 1,
            Self::Real => 8,
            Self::Bus { width } => 3 * width.div_ceil(8) as usize,
        }
    }
}

/// Reusable encoding buffers for event frames; they grow to one full frame and stay.
pub(crate) struct EventScratch {
    /// Whether blocks may be compressed; without it every block is `plain`.
    pub(crate) compress: bool,
    planes: Vec<u8>,
    raw: Vec<u8>,
    reals: Vec<f64>,
}

impl EventScratch {
    pub(crate) fn new(compress: bool) -> Self {
        Self {
            compress,
            planes: Vec::new(),
            raw: Vec::new(),
            reals: Vec::new(),
        }
    }
}

/// One change's contribution to its signal's digest stream.
pub(crate) fn hash_change(
    hasher: &mut ValueHasher,
    kind: EventKind,
    time: f64,
    sequence: u32,
    value: &[u8],
    strength: u8,
) {
    hasher.value(time);
    hasher.bytes(&sequence.to_le_bytes());
    match kind {
        EventKind::Real => hasher.bytes(&canonical_bits(real_at(value, 0)).to_le_bytes()),
        EventKind::Logic | EventKind::Bus { .. } => hasher.bytes(value),
    }
    hasher.bytes(&[strength]);
}

/// The real stored at byte `at` of `bytes`.
fn real_at(bytes: &[u8], at: usize) -> f64 {
    f64::from_bits(u64::from_le_bytes(
        bytes[at..at + 8].try_into().unwrap_or_default(),
    ))
}

/// One event signal's open buffer in the writer.
pub(crate) struct EventBuffer {
    pub(crate) signal: u32,
    pub(crate) kind: EventKind,
    /// Changes written to frames so far: the ordinal of the buffer's first change.
    pub(crate) written: u64,
    times: Vec<f64>,
    sequences: Vec<u32>,
    /// Stored values, `kind.value_bytes()` apart.
    values: Vec<u8>,
    strengths: Vec<u8>,
    /// The last change written to a frame (value and strength), which the next frame records as
    /// its value before; empty before the first frame.
    before: Vec<u8>,
    before_strength: u8,
}

impl EventBuffer {
    pub(crate) fn new(signal: u32, kind: EventKind) -> Self {
        Self {
            signal,
            kind,
            written: 0,
            times: Vec::new(),
            sequences: Vec::new(),
            values: Vec::new(),
            strengths: Vec::new(),
            before: Vec::new(),
            before_strength: 0,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.times.len()
    }

    /// Append one change; returns whether the buffer is now full.
    pub(crate) fn push(&mut self, change: &EventChange) -> bool {
        self.times.push(change.time);
        self.sequences.push(change.sequence);
        change.value.put(&mut self.values);
        self.strengths.push(change.strength);
        self.times.len() == EVENTS_PER_FRAME
    }

    /// Hash every buffered change into the signal's digest stream.
    pub(crate) fn digest(&self, hasher: &mut ValueHasher) {
        let stride = self.kind.value_bytes();
        for at in 0..self.len() {
            hash_change(
                hasher,
                self.kind,
                self.times[at],
                self.sequences[at],
                &self.values[at * stride..(at + 1) * stride],
                self.strengths[at],
            );
        }
    }

    /// Encode the buffer as an `EVENTS` frame. Returns the frame's (first ordinal, count, t_min,
    /// t_max). A real (non-provisional) frame then empties the buffer with [`Self::clear`].
    pub(crate) fn encode(
        &self,
        flags: u8,
        frame: &mut FrameBuf,
        scratch: &mut EventScratch,
    ) -> (u64, u64, f64, f64) {
        let count = self.len();
        let buckets = count.div_ceil(EVENTS_PER_BUCKET);
        frame.begin(FrameKind::Events, flags);
        frame.put_u32(self.signal);
        frame.put_u8(self.kind.code());
        frame.put_u8(u8::from(!self.before.is_empty()));
        frame.put(&[0, 0]);
        frame.put_u64(self.written);
        frame.put_u32(count as u32);
        frame.put_u32(buckets as u32);
        let (t_min, t_max) = crate::writer::value_range(&self.times);
        frame.put_f64(t_min);
        frame.put_f64(t_max);
        frame.put_u8(self.before_strength);
        frame.put_zeros(7);
        let stride = self.kind.value_bytes();
        if self.before.is_empty() {
            frame.put_zeros(stride);
        } else {
            frame.put(&self.before);
        }
        frame.put_zeros(stride.next_multiple_of(8) - stride);
        for bucket in 0..buckets {
            let range = bucket * EVENTS_PER_BUCKET..((bucket + 1) * EVENTS_PER_BUCKET).min(count);
            let (lo_t, hi_t) = crate::writer::value_range(&self.times[range.clone()]);
            let (flags, lo, hi) = self.seen(range.clone());
            frame.put_f64(lo_t);
            frame.put_f64(hi_t);
            frame.put_u32(range.len() as u32);
            frame.put_u32(flags);
            frame.put_u64(lo);
            frame.put_u64(hi);
        }
        let table_at = frame.payload_len();
        frame.put_zeros(12 * 4);

        let block = |frame: &mut FrameBuf,
                     index: usize,
                     encode: &mut dyn FnMut(&mut [u8]) -> (Codec, usize),
                     raw: usize| {
            let at = frame.payload_len();
            let (codec, len) = encode(frame.spare(codec::max_encoded_len(raw)));
            frame.advance(len);
            frame.patch_u32(table_at + 12 * index, at as u32);
            frame.patch_u32(table_at + 12 * index + 4, len as u32);
            frame.patch_u8(table_at + 12 * index + 8, codec as u8);
        };
        let EventScratch {
            compress,
            planes,
            raw,
            reals,
        } = scratch;
        let compress = *compress;
        planes.resize(count * 8, 0);
        block(
            frame,
            0,
            &mut |out| codec::encode_f64(&self.times, planes, out, compress),
            count * 8,
        );
        raw.clear();
        raw.extend(
            self.sequences
                .iter()
                .flat_map(|sequence| sequence.to_le_bytes()),
        );
        block(
            frame,
            1,
            &mut |out| codec::encode_bytes(raw, out, compress),
            raw.len(),
        );
        match self.kind {
            EventKind::Real => {
                reals.clear();
                reals.extend((0..count).map(|at| real_at(&self.values, 8 * at)));
                block(
                    frame,
                    2,
                    &mut |out| codec::encode_f64(reals, planes, out, compress),
                    count * 8,
                );
            }
            EventKind::Logic => {
                block(
                    frame,
                    2,
                    &mut |out| codec::encode_bytes(&self.values, out, compress),
                    count,
                );
            }
            EventKind::Bus { .. } => {
                // Value, X and Z each as one run of bit strings, so like bytes sit together.
                raw.clear();
                let plane = stride / 3;
                for part in 0..3 {
                    for at in 0..count {
                        let start = at * stride + part * plane;
                        raw.extend_from_slice(&self.values[start..start + plane]);
                    }
                }
                block(
                    frame,
                    2,
                    &mut |out| codec::encode_bytes(raw, out, compress),
                    raw.len(),
                );
            }
        }
        block(
            frame,
            3,
            &mut |out| codec::encode_bytes(&self.strengths, out, compress),
            count,
        );
        (self.written, count as u64, t_min, t_max)
    }

    /// The values seen among `range`: flags, then low and high as raw words. Logic: one flag bit
    /// per level seen. Real: NaN seen, then the range. Bus: X seen, Z seen, then the range of the
    /// value's low 64 bits.
    fn seen(&self, range: std::ops::Range<usize>) -> (u32, u64, u64) {
        let stride = self.kind.value_bytes();
        match self.kind {
            EventKind::Logic => {
                let flags = self.values[range]
                    .iter()
                    .fold(0u32, |flags, &code| flags | 1 << code);
                (flags, 0, 0)
            }
            EventKind::Real => {
                let (mut lo, mut hi, mut nan) = (f64::NAN, f64::NAN, false);
                for at in range {
                    let value = real_at(&self.values, 8 * at);
                    nan |= value.is_nan();
                    lo = fold_min(lo, value);
                    hi = fold_max(hi, value);
                }
                (
                    u32::from(nan),
                    canonical(lo).to_bits(),
                    canonical(hi).to_bits(),
                )
            }
            EventKind::Bus { .. } => {
                let plane = stride / 3;
                let (mut flags, mut lo, mut hi) = (0u32, u64::MAX, 0u64);
                for at in range {
                    let bytes = &self.values[at * stride..(at + 1) * stride];
                    let set = |part: usize| {
                        bytes[part * plane..(part + 1) * plane]
                            .iter()
                            .any(|&b| b != 0)
                    };
                    flags |= u32::from(set(1)) | u32::from(set(2)) << 1;
                    let mut low = [0u8; 8];
                    let take = plane.min(8);
                    low[..take].copy_from_slice(&bytes[..take]);
                    let low = u64::from_le_bytes(low);
                    lo = lo.min(low);
                    hi = hi.max(low);
                }
                (flags, lo, hi)
            }
        }
    }

    /// Empty the buffer after a real frame took its changes.
    pub(crate) fn clear(&mut self) {
        if let Some(last) = self.len().checked_sub(1) {
            let stride = self.kind.value_bytes();
            self.before.clear();
            self.before
                .extend_from_slice(&self.values[last * stride..(last + 1) * stride]);
            self.before_strength = self.strengths[last];
        }
        self.written += self.len() as u64;
        self.times.clear();
        self.sequences.clear();
        self.values.clear();
        self.strengths.clear();
    }
}

/// The fixed part of an `EVENTS` payload; the value before the first change follows it, padded
/// to 8 bytes, then the bucket summaries.
const EVENTS_HEADER_LEN: usize = 48;
const EVENTS_BUCKET_LEN: usize = 40;

/// What an `EVENTS` frame's header says.
pub(crate) struct EventsHeader {
    pub(crate) signal: u32,
    pub(crate) first: u64,
    pub(crate) count: u32,
    pub(crate) t_min: f64,
    pub(crate) t_max: f64,
}

pub(crate) fn parse_events_header(payload: &[u8], offset: u64) -> Result<EventsHeader, StoreError> {
    let mut cursor = Cursor::new(payload, offset + FRAME_HEADER_LEN as u64);
    let signal = cursor.u32()?;
    cursor.take(4)?;
    let first = cursor.u64()?;
    let count = cursor.u32()?;
    let buckets = cursor.u32()? as usize;
    let t_min = cursor.f64()?;
    let t_max = cursor.f64()?;
    if count == 0
        || count as usize > EVENTS_PER_FRAME
        || buckets != (count as usize).div_ceil(EVENTS_PER_BUCKET)
    {
        return Err(cursor.error("an EVENTS frame's counts disagree"));
    }
    Ok(EventsHeader {
        signal,
        first,
        count,
        t_min,
        t_max,
    })
}

/// Decode every change of an `EVENTS` payload.
pub(crate) fn decode_events(
    payload: &[u8],
    offset: u64,
    kind: EventKind,
    out: &mut Vec<EventChange>,
) -> Result<(), StoreError> {
    let header = parse_events_header(payload, offset)?;
    let count = header.count as usize;
    let base = offset + FRAME_HEADER_LEN as u64;
    let mut cursor = Cursor::new(payload, base);
    let before = kind.value_bytes().next_multiple_of(8);
    cursor
        .take(EVENTS_HEADER_LEN + before + EVENTS_BUCKET_LEN * count.div_ceil(EVENTS_PER_BUCKET))?;
    let mut blocks = [(0usize, 0usize, Codec::Plain); 4];
    for block in &mut blocks {
        let at = cursor.u32()? as usize;
        let len = cursor.u32()? as usize;
        let codec = Codec::from_u8(cursor.u8()?)
            .ok_or_else(|| cursor.error("unknown event block codec"))?;
        cursor.take(3)?;
        if at.checked_add(len).is_none_or(|end| end > payload.len()) {
            return Err(cursor.error("an event block runs past its frame"));
        }
        *block = (at, len, codec);
    }
    let bytes = |(at, len, _): (usize, usize, Codec)| &payload[at..at + len];
    let mut planes = Vec::new();
    let mut times = vec![0.0; count];
    codec::decode_f64(blocks[0].2, bytes(blocks[0]), &mut times, &mut planes, base)?;
    let mut sequences = vec![0u8; count * 4];
    codec::decode_bytes(blocks[1].2, bytes(blocks[1]), &mut sequences, base)?;
    let mut values = vec![0u8; count * kind.value_bytes()];
    let mut reals = Vec::new();
    if kind == EventKind::Real {
        reals.resize(count, 0.0);
        codec::decode_f64(blocks[2].2, bytes(blocks[2]), &mut reals, &mut planes, base)?;
    } else {
        codec::decode_bytes(blocks[2].2, bytes(blocks[2]), &mut values, base)?;
    }
    let mut strengths = vec![0u8; count];
    codec::decode_bytes(blocks[3].2, bytes(blocks[3]), &mut strengths, base)?;
    let plane = kind.value_bytes() / 3;
    let bits = |part: usize, at: usize| {
        let start = (part * count + at) * plane;
        values[start..start + plane].to_vec()
    };
    for at in 0..count {
        let value = match kind {
            EventKind::Logic => {
                EventValue::Logic(Logic::from_code(values[at]).ok_or_else(|| {
                    StoreError::corrupt(base, "a logic value is not 0, 1, X or Z")
                })?)
            }
            EventKind::Real => EventValue::Real(reals[at]),
            EventKind::Bus { .. } => EventValue::Bus {
                value: bits(0, at),
                x: bits(1, at),
                z: bits(2, at),
            },
        };
        out.push(EventChange {
            signal: header.signal,
            time: times[at],
            sequence: u32::from_le_bytes(
                sequences[4 * at..4 * at + 4].try_into().unwrap_or_default(),
            ),
            value,
            strength: strengths[at],
        });
    }
    Ok(())
}
