use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use crate::{Digest, PlaceError, PlaceWays, Placed, kept_segment_name, place_segment};

fn folder(case: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rspice-results-store-place-{case}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

/// A source segment of `len` bytes that are not all alike.
fn source(root: &std::path::Path, len: usize) -> (PathBuf, Vec<u8>) {
    let bytes: Vec<u8> = (0..len).map(|i| (i * 31 % 251) as u8).collect();
    let path = root.join("transient-0.rsr");
    std::fs::write(&path, &bytes).unwrap();
    (path, bytes)
}

/// Every way puts the whole segment under its digest's name and leaves no partial file; a
/// segment the folder holds is not placed again. A link or a clone outlives the run's own
/// segment, which its last reader deletes.
#[test]
fn a_segment_is_kept_under_its_digest_by_each_way_and_outlives_its_source() {
    let digest = Digest([0xab; 32]);
    for (case, ways, expected) in [
        ("copy", PlaceWays::COPY, &[Placed::Copied][..]),
        (
            "link",
            PlaceWays {
                clone: false,
                link: true,
            },
            &[Placed::Linked][..],
        ),
        // Where the volume cannot clone, the next way is taken.
        ("all", PlaceWays::ALL, &[Placed::Cloned, Placed::Linked][..]),
    ] {
        let root = folder(case);
        let (path, bytes) = source(&root, (9 << 20) + 17);
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
    let digest = Digest([0x5c; 32]);
    let root = folder("cancel");
    let (path, bytes) = source(&root, (20 << 20) + 3);
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
