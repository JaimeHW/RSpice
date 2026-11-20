use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, SystemTime};

use super::support::{Rng, walk};
use crate::format::{BLOCK_ENTRY_LEN, CHUNK_HEADER_LEN, FRAME_HEADER_LEN, FrameKind, frame_crc};
use crate::{
    Axis, Digest, MemoryIo, PlaceError, PlaceWays, Placed, RunMeta, RunStatus, Schema,
    SegmentWriter, Signal, SignalKind, StoreError, kept_segment_name, place_segment,
    segment_verified, verify_segment,
};

fn folder(case: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rspice-results-store-place-{case}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

/// A finished segment of at least `len` bytes, stored plain from values that do not repeat, as
/// the run's own file `transient-<seed>.rsr` in `root`: its path, its bytes and its digest.
fn source(root: &Path, len: usize, seed: u64) -> (PathBuf, Vec<u8>, Digest) {
    const SIGNALS: usize = 32;
    let schema = Schema {
        x: Axis {
            name: "time".into(),
            unit: "s".into(),
        },
        signals: (0..SIGNALS)
            .map(|signal| Signal::new(format!("v({signal})"), "V", SignalKind::Real))
            .collect(),
    };
    let mut writer =
        SegmentWriter::with_options(MemoryIo::new(), &schema, &RunMeta::default(), 4096, false)
            .unwrap();
    let mut rng = Rng(seed);
    let mut values = [0.0; SIGNALS];
    for row in 0..len / (8 * (SIGNALS + 1)) + 1 {
        values.iter_mut().for_each(|value| *value = rng.unit());
        writer.push_row(row as f64 * 1e-9, &values).unwrap();
    }
    let (receipt, io) = writer.finish(RunStatus::Completed).unwrap();
    let bytes = io.into_bytes();
    let path = root.join(format!("transient-{seed}.rsr"));
    std::fs::write(&path, &bytes).unwrap();
    (path, bytes, receipt.digest)
}

/// Where the first chunk's value block starts and how long it is.
fn value_block(bytes: &[u8]) -> (usize, usize, usize) {
    let chunk = walk(bytes)
        .into_iter()
        .find(|frame| frame.kind == FrameKind::Chunk)
        .unwrap();
    let payload = chunk.offset as usize + FRAME_HEADER_LEN;
    let entry = payload + CHUNK_HEADER_LEN + BLOCK_ENTRY_LEN;
    let u32_at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
    (
        chunk.offset as usize,
        payload + u32_at(entry),
        u32_at(entry + 4),
    )
}

/// `bytes` with one value in the first chunk changed and every checksum over it sealed again, as
/// a tool that meant the change would leave it: each read checks out, and only the digest shows
/// the value is not the run's.
fn resealed(bytes: &[u8]) -> Vec<u8> {
    let mut bytes = bytes.to_vec();
    let (frame, block, len) = value_block(&bytes);
    bytes[block + 8] ^= 0x01;
    let entry = frame + FRAME_HEADER_LEN + CHUNK_HEADER_LEN + BLOCK_ENTRY_LEN;
    let crc = crc32fast::hash(&bytes[block..block + len]);
    bytes[entry + 12..entry + 16].copy_from_slice(&crc.to_le_bytes());
    let payload_len = u32::from_le_bytes(bytes[frame + 8..frame + 12].try_into().unwrap()) as usize;
    let header: [u8; FRAME_HEADER_LEN] = bytes[frame..frame + FRAME_HEADER_LEN].try_into().unwrap();
    let crc = frame_crc(
        &header,
        &bytes[frame + FRAME_HEADER_LEN..frame + FRAME_HEADER_LEN + payload_len],
    );
    bytes[frame + 12..frame + 16].copy_from_slice(&crc.to_le_bytes());
    bytes
}

/// Write `bytes` over the file at `path` and give it a modification time of its own, so the
/// change is seen whatever the volume's clock resolution.
fn rewrite(path: &Path, bytes: &[u8], seconds: u64) {
    std::fs::write(path, bytes).unwrap();
    std::fs::File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000 + seconds))
        .unwrap();
}

/// Every way puts the whole segment under its digest's name and leaves no partial file; a
/// segment the folder holds is not placed again. A clone or a copy outlives the run's own
/// segment, which its last reader deletes.
#[test]
fn a_segment_is_kept_under_its_digest_by_each_way_and_outlives_its_source() {
    for (case, ways, expected) in [
        ("copy", PlaceWays::COPY, &[Placed::Copied][..]),
        // Where the volume cannot clone, it copies.
        ("all", PlaceWays::ALL, &[Placed::Cloned, Placed::Copied][..]),
    ] {
        let root = folder(case);
        let (path, bytes, digest) = source(&root, (9 << 20) + 17, 1);
        let kept = root.join("project").join("results").join("store");
        let mut reports = Vec::new();
        let placed = place_segment(
            &path,
            &kept,
            &digest,
            ways,
            &AtomicBool::new(false),
            &mut |written| reports.push(written),
        )
        .unwrap();
        assert!(expected.contains(&placed), "{case}: {placed:?}");
        if placed == Placed::Copied {
            assert_eq!(reports.last().copied(), Some(bytes.len() as u64), "{case}");
            assert!(reports.windows(2).all(|pair| pair[0] < pair[1]), "{case}");
        }
        let target = kept.join(kept_segment_name(&digest));
        std::fs::remove_file(&path).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), bytes, "{case}");
        assert_eq!(
            std::fs::read_dir(&kept).unwrap().count(),
            1,
            "{case}: no partial left"
        );
        assert!(segment_verified(&target, &digest), "{case}: checked once");
        let again = place_segment(
            &path,
            &kept,
            &digest,
            ways,
            &AtomicBool::new(false),
            &mut |_| {},
        );
        assert_eq!(again, Ok(Placed::Present), "{case}");
        std::fs::remove_dir_all(&root).unwrap();
    }
}

/// A cancelled copy leaves nothing under the segment's name and no partial file, and a stale
/// partial from a copy that died is replaced, never mistaken for the segment.
#[test]
fn a_cancelled_copy_leaves_the_folder_as_it_was() {
    let root = folder("cancel");
    let (path, bytes, digest) = source(&root, (20 << 20) + 3, 2);
    let kept = root.join("store");
    std::fs::create_dir_all(&kept).unwrap();
    let partial = kept.join(format!("{}.partial", kept_segment_name(&digest)));
    std::fs::write(&partial, b"a copy that died").unwrap();
    let cancel = AtomicBool::new(false);
    let result = place_segment(
        &path,
        &kept,
        &digest,
        PlaceWays::COPY,
        &cancel,
        &mut |written| {
            assert!(
                written < bytes.len() as u64,
                "cancelled after the last block"
            );
            cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        },
    );
    assert_eq!(result, Err(PlaceError::Cancelled));
    assert_eq!(std::fs::read_dir(&kept).unwrap().count(), 0);

    let placed = place_segment(
        &path,
        &kept,
        &digest,
        PlaceWays::COPY,
        &AtomicBool::new(false),
        &mut |_| {},
    );
    assert_eq!(placed, Ok(Placed::Copied));
    assert_eq!(
        std::fs::read(kept.join(kept_segment_name(&digest))).unwrap(),
        bytes
    );
    std::fs::remove_dir_all(&root).unwrap();
}

/// A file under the digest's name that does not hold the digest (another run's segment, a
/// damaged one, a cut one, one whose bits were changed and sealed again) is not taken for the
/// segment: placing replaces it from the source.
#[test]
fn a_file_under_the_name_that_does_not_hold_the_digest_is_replaced_from_the_source() {
    let root = folder("replace");
    let (path, bytes, digest) = source(&root, 64 << 10, 3);
    let (_, other, _) = source(&root, 64 << 10, 4);
    let mut damaged = bytes.clone();
    let (_, block, len) = value_block(&bytes);
    damaged[block + len / 2] ^= 0x01;
    let kept = root.join("store");
    std::fs::create_dir_all(&kept).unwrap();
    let target = kept.join(kept_segment_name(&digest));
    for (seconds, (case, held)) in [
        ("another run", other),
        ("a flipped bit", damaged),
        ("cut short", bytes[..bytes.len() / 2].to_vec()),
        ("sealed again", resealed(&bytes)),
    ]
    .into_iter()
    .enumerate()
    {
        rewrite(&target, &held, seconds as u64);
        assert!(verify_segment(&target, &digest).is_err(), "{case}");
        let placed = place_segment(
            &path,
            &kept,
            &digest,
            PlaceWays::COPY,
            &AtomicBool::new(false),
            &mut |_| {},
        );
        assert_eq!(placed, Ok(Placed::Copied), "{case}");
        assert_eq!(std::fs::read(&target).unwrap(), bytes, "{case}");
        assert_eq!(std::fs::read_dir(&kept).unwrap().count(), 1, "{case}");
    }
    std::fs::remove_dir_all(&root).unwrap();
}

/// A source whose bits do not hash to the digest it is placed as is refused, and the folder is
/// left as it was: nothing under the name and no partial copy.
#[test]
fn a_source_that_does_not_hold_the_digest_is_never_kept() {
    let root = folder("refuse");
    let (path, bytes, digest) = source(&root, 64 << 10, 5);
    let (_, _, other) = source(&root, 64 << 10, 6);
    let kept = root.join("store");
    for (case, named) in [("another digest", other), ("its own digest", digest)] {
        if case == "its own digest" {
            std::fs::write(&path, resealed(&bytes)).unwrap();
        }
        let refused = place_segment(
            &path,
            &kept,
            &named,
            PlaceWays::ALL,
            &AtomicBool::new(false),
            &mut |_| {},
        );
        let Err(PlaceError::Io(reason)) = refused else {
            panic!("{case}: refused, not {refused:?}");
        };
        assert!(reason.contains("hash to"), "{case}: {reason}");
        assert_eq!(std::fs::read_dir(&kept).unwrap().count(), 0, "{case}");
    }
    std::fs::remove_dir_all(&root).unwrap();
}

/// A kept segment is never the run's own file under a second name: a later write to the source
/// (a resumed run appending to it) leaves the kept file as it was placed.
#[test]
fn a_kept_segment_never_shares_its_source_file() {
    let root = folder("share");
    let (path, bytes, digest) = source(&root, 64 << 10, 7);
    let kept = root.join("store");
    place_segment(
        &path,
        &kept,
        &digest,
        PlaceWays::ALL,
        &AtomicBool::new(false),
        &mut |_| {},
    )
    .unwrap();
    let target = kept.join(kept_segment_name(&digest));
    {
        use std::io::{Seek, SeekFrom, Write};
        let mut file = std::fs::File::options().write(true).open(&path).unwrap();
        file.seek(SeekFrom::Start(bytes.len() as u64 / 2)).unwrap();
        file.write_all(&[0xa5; 64]).unwrap();
        file.seek(SeekFrom::End(0)).unwrap();
        file.write_all(b"appended").unwrap();
    }
    assert_eq!(std::fs::read(&target).unwrap(), bytes);
    assert_eq!(verify_segment(&target, &digest), Ok(()));
    std::fs::remove_dir_all(&root).unwrap();
}

/// A digest is a check, not a claim: the first check of a file reads all of it and is
/// remembered while the file is unchanged; a file changed since is checked again, and one whose
/// bits were changed and sealed again fails on its digest even though every read checks out.
#[test]
fn a_kept_file_is_checked_once_and_again_after_it_changes() {
    let root = folder("verify");
    let (path, bytes, digest) = source(&root, 64 << 10, 8);
    let (_, _, other) = source(&root, 64 << 10, 9);
    assert!(!segment_verified(&path, &digest), "not checked yet");
    assert_eq!(verify_segment(&path, &digest), Ok(()));
    assert!(segment_verified(&path, &digest));
    assert!(!segment_verified(&path, &other));
    assert_eq!(
        verify_segment(&path, &other),
        Err(StoreError::DigestMismatch {
            expected: other,
            held: digest
        })
    );

    rewrite(&path, &resealed(&bytes), 1);
    assert!(!segment_verified(&path, &digest), "changed since");
    let Err(StoreError::DigestMismatch { expected, held }) = verify_segment(&path, &digest) else {
        panic!("a resealed change fails on its digest");
    };
    assert_eq!(expected, digest, "the digest its END records");
    assert_ne!(held, digest);

    let mut flipped = bytes.clone();
    let (_, block, len) = value_block(&bytes);
    flipped[block + len / 2] ^= 0x01;
    rewrite(&path, &flipped, 2);
    let Err(StoreError::Corrupt { offset, .. }) = verify_segment(&path, &digest) else {
        panic!("a flipped bit is refused at its block");
    };
    assert_eq!(offset, block as u64);

    rewrite(&path, &bytes, 3);
    assert_eq!(verify_segment(&path, &digest), Ok(()));
    std::fs::remove_file(&path).unwrap();
    assert!(!segment_verified(&path, &digest));
    assert!(matches!(
        verify_segment(&path, &digest),
        Err(StoreError::Io { .. })
    ));
    std::fs::remove_dir_all(&root).unwrap();
}

/// Only the segments a save still references stay; partial copies always go, and other files
/// are not the store's to delete.
#[test]
fn unreferenced_segments_and_partial_copies_are_removed() {
    let root = folder("remove");
    let keep = Digest([1; 32]);
    let drop = Digest([2; 32]);
    for name in [
        kept_segment_name(&keep),
        kept_segment_name(&drop),
        format!("{}.partial", kept_segment_name(&keep)),
        "notes.txt".to_owned(),
    ] {
        std::fs::write(root.join(name), b"x").unwrap();
    }
    let deleted = crate::remove_unkept_segments(&root, &HashSet::from([keep]));
    assert_eq!(deleted.len(), 2);
    let mut left: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    left.sort();
    assert_eq!(left, vec![kept_segment_name(&keep), "notes.txt".to_owned()]);
    std::fs::remove_dir_all(&root).unwrap();
}
