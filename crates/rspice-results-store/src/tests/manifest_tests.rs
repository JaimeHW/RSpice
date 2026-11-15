//! The multi-run manifest and its atomic rewrite.

use crate::{Digest, Manifest, Member, RunStatus, StopReason, StoreDir};

fn member(index: u32) -> Member {
    Member {
        index,
        segment: StoreDir::segment_name(index),
        analysis: ".tran 1n 10u".into(),
        sweep: vec![("rload".into(), 1e3 * f64::from(index + 1))],
        seed: index.is_multiple_of(2).then_some(u64::from(index) * 31),
        corner: index.is_multiple_of(3).then(|| "ss".to_string()),
        temperature: Some(-40.0 + f64::from(index)),
        status: match index % 3 {
            0 => Some(RunStatus::Completed),
            1 => Some(RunStatus::Stopped(StopReason::Failed(
                "no convergence".into(),
            ))),
            _ => None,
        },
        digest: (index % 2 == 1).then_some(Digest([index as u8; 32])),
    }
}

#[test]
fn members_stay_in_index_order_and_round_trip() {
    let mut manifest = Manifest::default();
    for index in [5, 1, 3, 0] {
        manifest.upsert(member(index));
    }
    let mut finished = member(3);
    finished.status = Some(RunStatus::Stopped(StopReason::Cancelled));
    manifest.upsert(finished.clone());
    let indices: Vec<u32> = manifest
        .members()
        .iter()
        .map(|member| member.index)
        .collect();
    assert_eq!(indices, [0, 1, 3, 5]);
    assert_eq!(manifest.members()[2], finished);
    let bytes = manifest.encode();
    assert_eq!(Manifest::decode(&bytes).unwrap(), manifest);
    assert_eq!(
        Manifest::decode(&Manifest::default().encode()).unwrap(),
        Manifest::default()
    );
    // Any flipped byte or cut is refused.
    for at in 0..bytes.len() {
        let mut damaged = bytes.clone();
        damaged[at] ^= 0x10;
        assert!(Manifest::decode(&damaged).is_err(), "flip at {at}");
        assert!(Manifest::decode(&bytes[..at]).is_err(), "cut at {at}");
    }
}

#[test]
fn a_crash_between_writing_and_renaming_keeps_the_old_manifest() {
    let root = std::env::temp_dir().join(format!(
        "rspice-results-store-manifest-{}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }
    let store = StoreDir::new(&root);
    store.create().unwrap();
    assert!(root.join("runs").is_dir());
    assert_eq!(store.load_manifest().unwrap(), None);

    let mut old = Manifest::default();
    old.upsert(member(0));
    store.save_manifest(&old).unwrap();
    assert_eq!(store.load_manifest().unwrap(), Some(old.clone()));

    let mut new = old.clone();
    new.upsert(member(1));
    // The new manifest is written and made durable, then the process dies before the rename.
    store.stage_manifest(&new).unwrap();
    assert_eq!(store.load_manifest().unwrap(), Some(old.clone()));
    // Or dies halfway through writing it.
    let staged = root.join("manifest.rsm.new");
    let bytes = new.encode();
    std::fs::write(&staged, &bytes[..bytes.len() / 2]).unwrap();
    assert_eq!(store.load_manifest().unwrap(), Some(old.clone()));

    // The next save replaces the stale staged file and the manifest whole.
    store.save_manifest(&new).unwrap();
    assert_eq!(store.load_manifest().unwrap(), Some(new.clone()));
    assert!(!staged.exists());

    // A damaged manifest is reported, not read as empty.
    let path = root.join(StoreDir::MANIFEST);
    let mut damaged = std::fs::read(&path).unwrap();
    damaged[20] ^= 1;
    std::fs::write(&path, damaged).unwrap();
    assert!(store.load_manifest().is_err());
    std::fs::remove_dir_all(&root).unwrap();
}
