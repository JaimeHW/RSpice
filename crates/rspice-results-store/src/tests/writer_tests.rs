//! The writer's bytes: layout, determinism across flushes and push chunkings, and the codecs.

use super::support::{Rng, meta, mixed_schema, rows, u64_at, walk};
use crate::codec::{self, Codec};
use crate::format::{FrameKind, POINTERS_AT, chunk_rows};
use crate::{MemoryIo, RunStatus, SegmentWriter, StopReason, StoreError};

#[test]
fn chunk_rows_follow_the_column_count() {
    for (slots, rows) in [
        (0, 4096),
        (1, 4096),
        (100, 4096),
        (256, 4096),
        (257, 2048),
        (1000, 1024),
        (10_000, 256),
    ] {
        assert_eq!(chunk_rows(slots), rows, "{slots} slots");
    }
    assert_eq!(chunk_rows(5_000_000), 256);
}

#[test]
fn float_blocks_round_trip_bit_for_bit() {
    let (_, smooth) = rows(3000, 1, 11);
    let mut rng = Rng(5);
    let noise: Vec<f64> = (0..3000).map(|_| f64::from_bits(rng.next_u64())).collect();
    let mut planes = vec![0; 3000 * 8];
    for (values, expected) in [(&smooth, Codec::XorShuffleLz4), (&noise, Codec::Plain)] {
        let mut out = vec![0; codec::max_encoded_len(values.len() * 8)];
        let (codec, len) = codec::encode_f64(values, &mut planes, &mut out);
        assert_eq!(codec, expected);
        let mut decoded = vec![0.0; values.len()];
        let mut scratch = Vec::new();
        codec::decode_f64(codec, &out[..len], &mut decoded, &mut scratch, 0).unwrap();
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&decoded), bits(values));
    }
}

/// Write `count` rows with `push`, calling `sync` before the rows `syncs` names.
fn segment(
    count: usize,
    chunk: usize,
    pushes: &[usize],
    syncs: &[usize],
) -> (Vec<u8>, crate::StoreReceipt) {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(count, slots, 42);
    let mut writer =
        SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), chunk).unwrap();
    let mut row = 0;
    let mut splits = pushes.iter().copied().cycle();
    while row < count {
        let take = splits.next().unwrap_or(1).max(1).min(count - row);
        if syncs.iter().any(|&s| s >= row && s < row + take) {
            writer.sync().unwrap();
        }
        writer
            .push_rows(
                &x[row..row + take],
                &values[row * slots..(row + take) * slots],
            )
            .unwrap();
        row += take;
    }
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    (io.into_bytes(), receipt)
}

#[test]
fn the_same_rows_give_the_same_bytes_with_and_without_flushes_and_across_push_chunkings() {
    let (reference, receipt) = segment(1000, 64, &[1], &[]);
    assert_eq!(receipt.rows, 1000);
    assert_eq!(receipt.chunks, 16);
    assert_eq!(receipt.bytes, reference.len() as u64);
    for (pushes, syncs) in [
        (&[7usize, 64, 1, 200][..], &[][..]),
        (&[1000][..], &[0usize, 1, 63, 64, 65, 500, 999][..]),
        (&[3][..], &[10usize, 130, 131, 640][..]),
    ] {
        let (bytes, other) = segment(1000, 64, pushes, syncs);
        assert_eq!(other, receipt);
        assert!(
            bytes == reference,
            "pushes {pushes:?} syncs {syncs:?} changed the bytes"
        );
    }
}

#[test]
fn a_segment_is_superblock_schema_meta_chunks_indexes_and_end() {
    let (bytes, receipt) = segment(4200, 64, &[1], &[]);
    let frames = walk(&bytes);
    let kinds: Vec<FrameKind> = frames.iter().map(|f| f.kind).collect();
    assert_eq!(&kinds[..2], &[FrameKind::Schema, FrameKind::RunMeta]);
    assert_eq!(
        kinds.iter().filter(|&&k| k == FrameKind::Chunk).count() as u64,
        receipt.chunks
    );
    // 66 chunks: one index after the 64th, one at the end.
    assert_eq!(receipt.chunks, 66);
    let indexes: Vec<u64> = frames
        .iter()
        .filter(|f| f.kind == FrameKind::Index)
        .map(|f| f.offset)
        .collect();
    assert_eq!(indexes.len(), 2);
    assert_eq!(kinds[2 + 64], FrameKind::Index);
    assert_eq!(*kinds.last().unwrap(), FrameKind::End);
    assert!(frames.iter().all(|f| !f.partial));
    // The superblock names the last index, the rows it covers, and END.
    let pointers = POINTERS_AT as usize;
    assert_eq!(u64_at(&bytes, pointers), indexes[1]);
    assert_eq!(u64_at(&bytes, pointers + 8), 4200);
    assert_eq!(u64_at(&bytes, pointers + 16), frames.last().unwrap().offset);
    // The second index chains back to the first.
    let second = frames.iter().find(|f| f.offset == indexes[1]).unwrap();
    assert_eq!(u64_at(&second.payload, 0), indexes[0]);
}

#[test]
fn a_sync_leaves_a_provisional_chunk_that_the_next_real_frame_replaces() {
    let schema = mixed_schema();
    let slots = schema.slots();
    let (x, values) = rows(100, slots, 9);
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    writer.push_rows(&x[..70], &values[..70 * slots]).unwrap();
    let before = writer.io().bytes().len();
    writer.sync().unwrap();
    let synced = writer.io().bytes().to_vec();
    assert!(synced.len() > before);
    let frames = walk(&synced);
    let last = frames.last().unwrap();
    assert!(last.partial && last.kind == FrameKind::Chunk);
    assert_eq!(
        u64_at(&last.payload, 8),
        64,
        "the provisional chunk starts at the open row"
    );
    // A second sync rewrites the same tail; finishing replaces it with real frames.
    writer
        .push_rows(&x[70..80], &values[70 * slots..80 * slots])
        .unwrap();
    writer.sync().unwrap();
    assert_eq!(
        walk(writer.io().bytes())
            .iter()
            .filter(|f| f.partial)
            .count(),
        1
    );
    writer.push_rows(&x[80..], &values[80 * slots..]).unwrap();
    let (_, io) = writer
        .finish(RunStatus::Stopped(StopReason::Cancelled))
        .unwrap();
    assert!(walk(io.bytes()).iter().all(|f| !f.partial));
}

#[test]
fn rows_of_the_wrong_width_and_closed_rows_are_refused() {
    let schema = mixed_schema();
    let mut writer = SegmentWriter::with_chunk_rows(MemoryIo::new(), &schema, &meta(), 64).unwrap();
    assert_eq!(
        writer.push_row(0.0, &[1.0]),
        Err(StoreError::RowWidth {
            expected: 4,
            found: 1
        })
    );
    for row in 0..65 {
        writer.push_row(row as f64, &[0.0; 4]).unwrap();
    }
    assert_eq!(
        writer.amend_open_row(3, 0, 1.0),
        Err(StoreError::RowNotOpen { row: 3, slot: 0 })
    );
    assert_eq!(
        writer.amend_open_row(64, 4, 1.0),
        Err(StoreError::RowNotOpen { row: 64, slot: 4 })
    );
    writer.amend_open_row(64, 3, 1.0).unwrap();
}

#[test]
fn the_digest_ignores_signed_zero_and_nan_payloads_but_not_values() {
    let digest_of = |values: [f64; 4]| {
        let mut writer = SegmentWriter::create(MemoryIo::new(), &mixed_schema(), &meta()).unwrap();
        writer.push_row(1.0, &values).unwrap();
        writer.finish(RunStatus::Completed).unwrap().0.digest
    };
    let base = digest_of([0.0, f64::NAN, 1.0, 2.0]);
    assert_eq!(
        digest_of([-0.0, f64::from_bits(0x7ff8_0000_0000_0001), 1.0, 2.0]),
        base
    );
    assert_ne!(digest_of([0.0, f64::NAN, 1.0, 2.5]), base);
}
