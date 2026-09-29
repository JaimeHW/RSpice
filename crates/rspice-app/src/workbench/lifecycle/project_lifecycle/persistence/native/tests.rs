//! Native adapter failures preserve writer recovery evidence and operating-system details.

#![cfg(any(windows, unix))]

use super::*;
use crate::io::durable_file::{self, ExpectedContent};

fn with_destination(test: impl FnOnce(&Path, &Path)) {
    let temp = std::fs::canonicalize(std::env::temp_dir()).unwrap();
    let root = temp.join(format!("rspice-native-storage-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&root).unwrap();
    let root = std::fs::canonicalize(root).unwrap();
    assert_eq!(root.parent(), Some(temp.as_path()));
    test(&root, &root.join("canonical.rspiceproj"));
    for entry in std::fs::read_dir(&root).unwrap() {
        let entry = entry.unwrap();
        assert_eq!(entry.path().parent(), Some(root.as_path()));
        assert!(entry.file_type().unwrap().is_file());
        std::fs::remove_file(entry.path()).unwrap();
    }
    std::fs::remove_dir(root).unwrap();
}

fn operations(path: &Path, expected: ExpectedContent) -> [Result<(), PersistenceError>; 3] {
    [
        NativeStorage.reconcile_publication(path),
        NativeStorage.observe_destination(path).map(|_| ()),
        NativeStorage
            .publish(path, expected, b"successor bytes")
            .map(|_| ()),
    ]
}

#[test]
fn storage_keeps_corrupt_recovery_evidence_and_os_error_details() {
    with_destination(|root, target| {
        let accepted = b"accepted bytes";
        std::fs::write(target, accepted).unwrap();
        let mut slot = target.as_os_str().to_owned();
        slot.push(".rspice.recovery-v1.a");
        let slot = PathBuf::from(slot);
        let damaged = b"damaged recovery metadata";
        std::fs::write(&slot, damaged).unwrap();
        let raw = durable_file::reconcile_publication(target).unwrap_err();
        let diagnostic = raw.to_string();
        let CompareExchangeError::PublicationUncertain {
            message,
            recovery_paths,
        } = raw
        else {
            panic!("damaged recovery metadata did not prevent publication")
        };
        assert!(recovery_paths.contains(&slot));
        for result in operations(
            target,
            ExpectedContent::Digest(*digest_bytes(accepted).as_bytes()),
        ) {
            let error = result.unwrap_err();
            assert_eq!(error.to_string(), diagnostic);
            let PersistenceError::PublicationUncertain {
                message: actual_message,
                recovery_paths: actual_paths,
            } = error
            else {
                panic!("publication uncertainty was lost")
            };
            assert_eq!(actual_message, message);
            assert_eq!(actual_paths, recovery_paths);
            assert_eq!(std::fs::read(target).unwrap(), accepted);
            assert_eq!(std::fs::read(&slot).unwrap(), damaged);
        }

        let blocked_parent = root.join("not-a-directory");
        std::fs::write(&blocked_parent, b"regular file").unwrap();
        let expected = std::fs::create_dir_all(&blocked_parent).unwrap_err();
        for result in operations(
            &blocked_parent.join("child.rspiceproj"),
            ExpectedContent::Missing,
        ) {
            let PersistenceError::Io(error) = result.unwrap_err() else {
                panic!("operating system error was lost")
            };
            assert_eq!(error.kind(), expected.kind());
            assert_eq!(error.raw_os_error(), expected.raw_os_error());
            assert_eq!(error.to_string(), expected.to_string());
        }
        assert_eq!(std::fs::read(blocked_parent).unwrap(), b"regular file");
    });
}

#[test]
fn storage_reports_the_owned_lease_and_recovers_after_release() {
    with_destination(|root, target| {
        let accepted = b"accepted bytes";
        std::fs::write(target, accepted).unwrap();
        let expected = NativeStorage.observe_destination(target).unwrap();
        let leases: Vec<_> = std::fs::read_dir(root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".rspice-lock-v2-")
            })
            .collect();
        assert_eq!(leases.len(), 1);
        let owned = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&leases[0])
            .unwrap();
        owned.lock().unwrap();
        let raw = durable_file::reconcile_publication(target).unwrap_err();
        let diagnostic = raw.to_string();
        let CompareExchangeError::LeaseBusy(lease) = raw else {
            panic!("the held lease did not prevent publication")
        };
        assert_eq!(lease, leases[0]);
        for result in operations(target, expected) {
            let error = result.unwrap_err();
            assert_eq!(error.to_string(), diagnostic);
            let PersistenceError::LeaseBusy(path) = error else {
                panic!("lease ownership was lost")
            };
            assert_eq!(path, lease);
            assert_eq!(std::fs::read(target).unwrap(), accepted);
        }
        drop(owned);
        assert_eq!(NativeStorage.observe_destination(target).unwrap(), expected);
        let successor = b"successful after release";
        assert_eq!(
            NativeStorage.publish(target, expected, successor).unwrap(),
            digest_bytes(successor)
        );
        assert_eq!(std::fs::read(target).unwrap(), successor);
    });
}
