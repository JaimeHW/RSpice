//! One writer per store: the lock is held by one holder at a time, shared within the process
//! that holds it when it waits for it, and its file goes with its last holder.

use crate::StoreLock;

#[test]
fn a_store_lock_has_one_holder_at_a_time_and_goes_with_its_last() {
    let root = std::env::temp_dir().join(format!("rspice-store-lock-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let store = root.join("out.rsr");
    let file = StoreLock::file_for(&store);
    assert_eq!(file, root.join("out.rsr.lock"));

    let lock = StoreLock::try_acquire(&store)
        .unwrap()
        .expect("nobody holds it");
    assert!(file.is_file());
    // Another process: a handle of its own on the lock file cannot lock it.
    let other = crate::lock::open(&file).unwrap();
    assert!(matches!(
        other.try_lock(),
        Err(std::fs::TryLockError::WouldBlock)
    ));
    drop(other);
    // This process: a second try is refused, a wait shares the lock it holds.
    assert!(StoreLock::try_acquire(&store).unwrap().is_none());
    assert!(StoreLock::held(&store).is_some());
    let shared = StoreLock::acquire(&store).unwrap();
    drop(lock);
    assert!(file.is_file(), "a holder remains");
    assert!(StoreLock::try_acquire(&store).unwrap().is_none());
    drop(shared);
    assert!(!file.exists(), "the last holder removes it");
    assert!(StoreLock::held(&store).is_none());

    // A lock its holder left (a process that died) is taken by the next to ask.
    std::fs::write(&file, b"").unwrap();
    let again = StoreLock::try_acquire(&store).unwrap().expect("free again");
    drop(again);
    assert!(!file.exists());
    std::fs::remove_dir_all(&root).unwrap();
}
