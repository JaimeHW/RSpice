//! Shared fixtures: a deterministic generator, schemas, and a frame walker.

use crate::format::{FRAME_HEADER_LEN, FrameHeader, FrameKind, SUPERBLOCK_LEN, frame_crc};
use crate::{RunMeta, Schema, Signal, SignalKind};

/// SplitMix64: reproducible test data without a dependency.
pub(crate) struct Rng(pub(crate) u64);

impl Rng {
    pub(crate) fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub(crate) fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub(crate) fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

/// Two real signals and one complex: four value slots.
pub(crate) fn mixed_schema() -> Schema {
    Schema {
        x: crate::Axis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals: vec![
            Signal::new("v(out)", "V", SignalKind::Real),
            Signal::new("i(v1)", "A", SignalKind::Real),
            Signal::new("v(ac)", "V", SignalKind::Complex),
        ],
    }
}

pub(crate) fn meta() -> RunMeta {
    RunMeta {
        analysis: ".tran 1n 10u".into(),
        sweep: vec![("temp".into(), 27.0)],
        seed: Some(7),
        options: vec![("reltol".into(), "1e-3".into())],
    }
}

/// Rows a simulator might produce: a rising x, smooth waveforms, and the awkward values a store
/// must keep exactly (signed zeros, NaN gaps, infinities, a payload-carrying NaN).
pub(crate) fn rows(count: usize, slots: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let mut rng = Rng(seed);
    let mut x = Vec::with_capacity(count);
    let mut values = Vec::with_capacity(count * slots);
    let mut t = 0.0;
    for row in 0..count {
        t += 1e-9 * (0.5 + rng.unit());
        x.push(t);
        for slot in 0..slots {
            let smooth = (t * 1e6 * (slot as f64 + 1.0)).sin() * (1.0 + slot as f64);
            let value = match rng.below(97) {
                0 => f64::NAN,
                1 => -0.0,
                2 => 0.0,
                3 => f64::INFINITY,
                4 => f64::from_bits(0x7ff8_0000_dead_beef),
                _ if (row / 40) % 7 == 3 && slot == 1 => f64::NAN,
                _ => smooth + 1e-3 * rng.unit(),
            };
            values.push(value);
        }
    }
    (x, values)
}

/// A frame found by walking a segment from the superblock.
#[derive(Debug, Clone)]
pub(crate) struct Walked {
    pub(crate) offset: u64,
    pub(crate) kind: FrameKind,
    pub(crate) partial: bool,
    pub(crate) payload: Vec<u8>,
}

/// Every frame of a complete segment, each checked for magic, CRC and alignment.
pub(crate) fn walk(bytes: &[u8]) -> Vec<Walked> {
    let mut frames = Vec::new();
    let mut offset = SUPERBLOCK_LEN;
    while (offset as usize) < bytes.len() {
        assert_eq!(offset % 8, 0, "frames are 8-byte aligned");
        let at = offset as usize;
        let header: [u8; FRAME_HEADER_LEN] = bytes[at..at + FRAME_HEADER_LEN].try_into().unwrap();
        let parsed = FrameHeader::parse(&header).expect("a frame header");
        let payload =
            &bytes[at + FRAME_HEADER_LEN..at + FRAME_HEADER_LEN + parsed.payload_len as usize];
        assert_eq!(
            frame_crc(&header, payload),
            parsed.crc,
            "frame at {offset} checks out"
        );
        frames.push(Walked {
            offset,
            kind: parsed.kind,
            partial: parsed.partial(),
            payload: payload.to_vec(),
        });
        offset = parsed.next(offset);
    }
    assert_eq!(
        offset as usize,
        bytes.len(),
        "the last frame ends the segment"
    );
    frames
}

pub(crate) fn u64_at(bytes: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap())
}
