//! What a complete segment refuses to read: a damaged block, a damaged summary, a damaged `END`,
//! a segment of an older format. Each refusal names the byte it was refused at, and nothing
//! damaged is ever decoded as a value or drawn as an overview.

use super::support::{u64_at, walk};
use crate::format::{
    BLOCK_ENTRY_LEN, CHUNK_HEADER_LEN, FRAME_HEADER_LEN, FrameKind, POINTERS_AT,
    SUMMARY_HEADER_LEN, chunk_crcs_at, chunk_summaries_at, frame_crc, summary_buckets_at,
};
use crate::{
    Axis, MemoryIo, RunMeta, RunStatus, Schema, SegmentReader, SegmentState, SegmentWriter, Signal,
    SignalKind, StoreError, Tile,
};

const ROWS: usize = 20_000;
const CHUNK_ROWS: usize = 256;

/// A 1 V, 100 kHz sine (no sample at its peak) over 20,000 rows in 256-row chunks: level-1 summaries in every chunk and
/// one level-2 `SUMMARY` frame over the whole run.
fn sine_segment() -> Vec<u8> {
    let schema = Schema {
        x: Axis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals: vec![Signal::new("v(in)", "V", SignalKind::Real)],
    };
    let mut writer =
        SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &RunMeta::default(), CHUNK_ROWS)
            .unwrap();
    for row in 0..ROWS {
        let t = row as f64 * 1e-8;
        writer
            .push_row(t, &[(2.0 * std::f64::consts::PI * 1e5 * t + 0.1).sin()])
            .unwrap();
    }
    writer.finish(RunStatus::Completed).unwrap().1.into_bytes()
}

fn open(bytes: Vec<u8>) -> SegmentReader<MemoryIo> {
    SegmentReader::open(MemoryIo::from_bytes(bytes)).unwrap()
}

fn damaged_at(error: StoreError) -> u64 {
    match error {
        StoreError::Corrupt { offset, .. } => offset,
        other => panic!("refused as damaged, not {other:?}"),
    }
}

fn u32_at(bytes: &[u8], at: usize) -> usize {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize
}

/// The `n`-th frame of `kind`, as (frame offset, payload offset).
fn frame(bytes: &[u8], kind: FrameKind, n: usize) -> (usize, usize) {
    let frame = walk(bytes)
        .into_iter()
        .filter(|frame| frame.kind == kind)
        .nth(n)
        .unwrap();
    (
        frame.offset as usize,
        frame.offset as usize + FRAME_HEADER_LEN,
    )
}

/// Rewrite the CRC of the frame at `offset`, as a writer that meant the bytes would have.
fn reseal(bytes: &mut [u8], offset: usize) {
    let len = u32_at(bytes, offset + 8);
    let header: [u8; FRAME_HEADER_LEN] =
        bytes[offset..offset + FRAME_HEADER_LEN].try_into().unwrap();
    let crc = frame_crc(
        &header,
        &bytes[offset + FRAME_HEADER_LEN..offset + FRAME_HEADER_LEN + len],
    );
    bytes[offset + 12..offset + 16].copy_from_slice(&crc.to_le_bytes());
}

#[test]
fn a_flipped_block_byte_is_refused_at_its_block_and_never_read_as_numbers() {
    let clean = sine_segment();
    let (_, payload) = frame(&clean, FrameKind::Chunk, 3);
    for block in [0, 1] {
        let entry = payload + CHUNK_HEADER_LEN + BLOCK_ENTRY_LEN * block;
        let block_at = payload + u32_at(&clean, entry);
        let len = u32_at(&clean, entry + 4);
        let mut bytes = clean.clone();
        bytes[block_at + len / 2] ^= 0x01;
        let reader = open(bytes);
        assert!(
            matches!(reader.state(), SegmentState::Complete { .. }),
            "END is intact"
        );
        let refused = if block == 0 {
            reader.read_x(0, ROWS).unwrap_err()
        } else {
            reader.read_slot(0, 0, ROWS).unwrap_err()
        };
        assert_eq!(damaged_at(refused), block_at as u64, "block {block}");
        let mut out = Vec::new();
        let refused = reader.read_chunk_slot(3, 0, &mut out);
        if block == 1 {
            assert_eq!(damaged_at(refused.unwrap_err()), block_at as u64);
        } else {
            assert_eq!(refused, Ok(()), "the x block is not the value block");
        }
        assert_eq!(damaged_at(reader.digest().unwrap_err()), block_at as u64);
        // Every other chunk still reads.
        assert_eq!(reader.read_slot(0, 0, 3 * CHUNK_ROWS).unwrap().len(), 768);
    }
}

#[test]
fn a_flipped_summary_bit_is_refused_and_never_drawn_as_a_peak() {
    let clean = sine_segment();
    let analog = 1;
    // The maximum of chunk 0's level-1 bucket 3 (rows 192..256, over the sine's first peak),
    // and a level-2 maximum in the run's `SUMMARY` frame.
    let (_, chunk) = frame(&clean, FrameKind::Chunk, 0);
    let level1 = chunk + chunk_summaries_at(2, analog) + 16 * (CHUNK_ROWS / 64) + 16 * 3 + 8;
    let (_, summary) = frame(&clean, FrameKind::Summary, 0);
    let buckets = u32_at(&clean, summary + 4);
    let level2 = summary + summary_buckets_at(analog) + 16 * buckets + 16 * 5 + 8;
    let mut peaks = Vec::new();
    for (at, pixels, column_start) in [
        (
            level1,
            100,
            chunk + chunk_summaries_at(2, analog) + 16 * (CHUNK_ROWS / 64),
        ),
        (
            level2,
            5,
            summary + summary_buckets_at(analog) + 16 * buckets,
        ),
    ] {
        let mut bytes = clean.clone();
        // An exponent bit: 0.99... becomes 1.99...
        bytes[at + 6] ^= 0x10;
        let reader = open(bytes.clone());
        let refused = reader.tile(0, 0.0, ROWS as f64 * 1e-8, pixels).unwrap_err();
        assert_eq!(damaged_at(refused), column_start as u64);
        assert_eq!(
            damaged_at(reader.digest().unwrap_err()),
            column_start as u64
        );
        peaks.push(f64::from_le_bytes(bytes[at..at + 8].try_into().unwrap()));
    }
    assert!(peaks.iter().all(|&peak| peak > 1.5), "{peaks:?}");
    // The clean segment draws the sine's envelope at both levels.
    let reader = open(clean);
    for pixels in [100, 5] {
        let Tile::Envelope { max, .. } = reader.tile(0, 0.0, ROWS as f64 * 1e-8, pixels).unwrap()
        else {
            panic!("an envelope at {pixels} pixels");
        };
        assert!(max.iter().all(|&max| max <= 1.0), "{max:?}");
    }
}

#[test]
fn the_digest_covers_the_summaries_so_a_rewritten_one_fails_verification() {
    let clean = sine_segment();
    let mut bytes = clean.clone();
    let (frame_at, payload) = frame(&bytes, FrameKind::Summary, 0);
    let buckets = u32_at(&bytes, payload + 4);
    let column = payload + summary_buckets_at(1) + 16 * buckets;
    bytes[column + 8..column + 16].copy_from_slice(&1.85_f64.to_le_bytes());
    // Sealed again as a writer that meant it would have: the column's CRC and the frame's.
    let crc = crc32fast::hash(&bytes[column..column + 16 * buckets]);
    let table = payload + SUMMARY_HEADER_LEN + 4;
    bytes[table..table + 4].copy_from_slice(&crc.to_le_bytes());
    reseal(&mut bytes, frame_at);

    let reader = open(bytes);
    let SegmentState::Complete { digest, .. } = reader.state().clone() else {
        panic!("END is intact");
    };
    // The rows are untouched; only the overview changed.
    assert_eq!(open(clean).digest().unwrap(), digest);
    assert_ne!(reader.digest().unwrap(), digest);
}

#[test]
fn a_flipped_end_byte_opens_the_rows_as_recovered_and_names_the_byte() {
    let clean = sine_segment();
    let end_at = u64_at(&clean, POINTERS_AT as usize + 16) as usize;
    let mut bytes = clean.clone();
    bytes[end_at + FRAME_HEADER_LEN + 40] ^= 0x01;
    let reader = open(bytes);
    assert_eq!(reader.state(), &SegmentState::Recovered);
    assert_eq!(reader.rows(), ROWS as u64);
    let damage = reader.end_damage().cloned().expect("the END is damaged");
    assert_eq!(damaged_at(damage), end_at as u64);
    // A clean segment and a crash-cut one have no damaged END.
    assert_eq!(open(clean.clone()).end_damage(), None);
    let cut = open(clean[..end_at].to_vec());
    assert_eq!(cut.state(), &SegmentState::Recovered);
    assert!(cut.end_damage().is_some(), "the END it names is gone");
    let mut crashed = clean[..end_at].to_vec();
    crashed[POINTERS_AT as usize + 16..POINTERS_AT as usize + 24].fill(0);
    assert_eq!(open(crashed).end_damage(), None);
}

#[test]
fn a_segment_of_an_older_format_is_refused_plainly() {
    let mut bytes = sine_segment();
    bytes[8..12].copy_from_slice(&3_u32.to_le_bytes());
    let refused = SegmentReader::open(MemoryIo::from_bytes(bytes))
        .err()
        .expect("refused");
    assert_eq!(damaged_at(refused.clone()), 8);
    assert!(
        refused.to_string().contains("format 3"),
        "{}",
        refused.to_string()
    );
}

#[test]
fn a_block_checksum_covers_exactly_its_block() {
    let bytes = sine_segment();
    let (_, payload) = frame(&bytes, FrameKind::Chunk, 0);
    let blocks = u32_at(&bytes, payload + 20);
    assert_eq!(blocks, 2);
    for block in 0..blocks {
        let entry = payload + CHUNK_HEADER_LEN + BLOCK_ENTRY_LEN * block;
        let at = payload + u32_at(&bytes, entry);
        let len = u32_at(&bytes, entry + 4);
        let crc = u32::from_le_bytes(bytes[entry + 12..entry + 16].try_into().unwrap());
        assert_eq!(crc32fast::hash(&bytes[at..at + len]), crc);
    }
    assert_eq!(
        chunk_summaries_at(2, 1) - chunk_crcs_at(2),
        8,
        "two summary CRCs, padded to eight bytes"
    );
}
