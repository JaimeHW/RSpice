//! Publication fault paths use asynchronous in-memory storage, without JS.

use super::*;
use std::{
    cell::{Cell, RefCell},
    future::poll_fn,
    task::{Context, Poll, Waker},
};

const ACCEPTED: &[u8] = b"accepted canonical bytes";
const STAGED: &[u8] = b"new project bytes";
const EXTERNAL: &[u8] = b"late external edit";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fault {
    Observe,
    Create,
    Write,
    Verify,
    Close,
    ReadBack,
    Mismatch,
}

struct Storage {
    file: RefCell<Vec<u8>>,
    staged: RefCell<Option<Vec<u8>>>,
    steps: RefCell<Vec<&'static str>>,
    reads: Cell<usize>,
    backend: Cell<Option<BrowserBindingBackend>>,
    fault: Option<Fault>,
    abort_failure: bool,
    late_change: bool,
}

impl Storage {
    fn new(fault: Option<Fault>) -> Self {
        Self {
            file: RefCell::new(ACCEPTED.to_vec()),
            staged: RefCell::new(None),
            steps: RefCell::new(Vec::new()),
            reads: Cell::new(0),
            backend: Cell::new(None),
            fault,
            abort_failure: false,
            late_change: false,
        }
    }

    async fn step(&self, name: &'static str) {
        let mut pending = true;
        poll_fn(|cx| {
            if pending {
                pending = false;
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        self.steps.borrow_mut().push(name);
    }
}

impl BrowserProjectStorage for Storage {
    type Writable = ();

    async fn create_writable(&self, backend: BrowserBindingBackend) -> Result<(), String> {
        self.step("create").await;
        self.backend.set(Some(backend));
        if self.fault == Some(Fault::Create) {
            return Err("create failed".to_owned());
        }
        Ok(())
    }
    async fn write(&self, _: &(), bytes: &[u8]) -> Result<(), String> {
        self.step("write").await;
        *self.staged.borrow_mut() = Some(bytes.to_vec());
        if self.late_change {
            *self.file.borrow_mut() = EXTERNAL.to_vec();
        }
        if self.fault == Some(Fault::Write) {
            return Err("write failed".to_owned());
        }
        Ok(())
    }
    async fn read(&self) -> Result<Vec<u8>, String> {
        let read = self.reads.get();
        self.step(match read {
            0 => "observe",
            1 => "verify",
            _ => "read-back",
        })
        .await;
        self.reads.set(read + 1);
        if read == 0 && self.fault == Some(Fault::Observe) {
            return Err("observe failed".to_owned());
        }
        if read == 1 && self.fault == Some(Fault::Verify) {
            return Err("verify failed".to_owned());
        }
        if read == 2 && self.fault == Some(Fault::ReadBack) {
            return Err("read-back failed".to_owned());
        }
        Ok(self.file.borrow().clone())
    }
    async fn abort(&self, _: &()) -> Result<(), String> {
        self.step("abort").await;
        if self.abort_failure {
            return Err("abort failed".to_owned());
        }
        self.staged.borrow_mut().take();
        Ok(())
    }
    async fn close(&self, _: &()) -> Result<(), String> {
        self.step("close").await;
        *self.file.borrow_mut() = self.staged.borrow().as_ref().unwrap().clone();
        // A rejected close may already have published. Later abort success
        // cannot prove that the predecessor bytes survived.
        if self.fault == Some(Fault::Close) {
            return Err("close failed".to_owned());
        }
        self.staged.borrow_mut().take();
        if self.fault == Some(Fault::Mismatch) {
            *self.file.borrow_mut() = EXTERNAL.to_vec();
        }
        Ok(())
    }
}

fn finish<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    // Every fixture operation yields exactly once and then wakes its caller.
    for pending in 0..16 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            assert!(pending > 0, "fixture must exercise suspended operations");
            return value;
        }
    }
    panic!("in-memory publication did not complete");
}

fn intent(
    backend: BrowserBindingBackend,
    accepted_digest: Option<ContentDigest>,
) -> BrowserWriteIntent {
    let mut intent = BrowserWriteIntent::fresh("project".to_owned(), backend);
    intent.expected_digest = accepted_digest;
    intent
}

fn publish(storage: &Storage) -> Result<ContentDigest, BrowserWriteError> {
    finish(
        intent(
            BrowserBindingBackend::ExternalFile,
            Some(digest_bytes(ACCEPTED)),
        )
        .publish_bytes(storage, STAGED),
    )
}

#[test]
fn accepted_content_is_checked_before_any_staging_or_abort() {
    let storage = Storage::new(Some(Fault::Observe));
    assert_eq!(
        publish(&storage),
        Err(BrowserWriteError::Platform("observe failed".to_owned()))
    );
    assert_eq!(*storage.steps.borrow(), ["observe"]);
    let storage = Storage::new(None);
    *storage.file.borrow_mut() = EXTERNAL.to_vec();
    assert_eq!(
        publish(&storage),
        Err(BrowserWriteError::ExternalChange(digest_bytes(EXTERNAL)))
    );
    assert_eq!(*storage.steps.borrow(), ["observe"]);
    assert!(storage.staged.borrow().is_none());
    assert_eq!(*storage.file.borrow(), EXTERNAL);
}

#[test]
fn publication_accepts_only_closed_and_verified_bytes_after_precommit_comparison() {
    for backend in [
        BrowserBindingBackend::ExternalFile,
        BrowserBindingBackend::Opfs,
    ] {
        for accepted in [None, Some(digest_bytes(ACCEPTED))] {
            let storage = Storage::new(None);
            let digest = finish(intent(backend, accepted).publish_bytes(&storage, STAGED)).unwrap();
            assert_eq!(digest, digest_bytes(STAGED));
            assert_eq!(*storage.file.borrow(), STAGED);
            assert!(storage.staged.borrow().is_none());
            assert_eq!(storage.backend.get(), Some(backend));
            assert_eq!(
                *storage.steps.borrow(),
                ["observe", "create", "write", "verify", "close", "read-back"]
            );
        }
    }
}

#[test]
fn staging_faults_preserve_abort_results_and_close_uncertainty() {
    let storage = Storage::new(Some(Fault::Create));
    assert_eq!(
        publish(&storage),
        Err(BrowserWriteError::Platform("create failed".to_owned()))
    );
    assert_eq!(*storage.steps.borrow(), ["observe", "create"]);
    for (fault, operation, steps, safe_message, abort_message) in [
        (
            Fault::Write,
            BrowserWriteOperation::Write,
            vec!["observe", "create", "write", "abort"],
            "browser project write failed and staging was aborted: write failed",
            "browser project write failed: write failed; staging abort also failed: abort failed",
        ),
        (
            Fault::Verify,
            BrowserWriteOperation::PreCommitVerification,
            vec!["observe", "create", "write", "verify", "abort"],
            "browser project pre-commit verification failed and staging was aborted: verify failed",
            "browser project pre-commit verification failed: verify failed; staging abort also failed: abort failed",
        ),
        (
            Fault::Close,
            BrowserWriteOperation::Close,
            vec!["observe", "create", "write", "verify", "close", "abort"],
            "browser project close failed and staging was aborted: close failed; publication outcome is uncertain",
            "browser project close failed: close failed; publication outcome is uncertain; staging abort also failed: abort failed",
        ),
    ] {
        for abort_failure in [false, true] {
            let storage = Storage {
                abort_failure,
                ..Storage::new(Some(fault))
            };
            let error = publish(&storage).unwrap_err();
            assert!(
                matches!(&error, BrowserWriteError::OperationFailed { operation: actual, abort_error, .. }
                if *actual == operation && abort_error.as_deref() == abort_failure.then_some("abort failed"))
            );
            assert_eq!(
                error.to_string(),
                if abort_failure {
                    abort_message
                } else {
                    safe_message
                }
            );
            assert_eq!(*storage.steps.borrow(), steps);
            assert_eq!(storage.staged.borrow().is_some(), abort_failure);
            assert_eq!(
                *storage.file.borrow(),
                if fault == Fault::Close {
                    STAGED
                } else {
                    ACCEPTED
                }
            );
        }
    }
}

#[test]
fn late_external_change_is_a_conflict_only_after_successful_abort() {
    for accepted in [None, Some(digest_bytes(ACCEPTED))] {
        for abort_failure in [false, true] {
            let storage = Storage {
                abort_failure,
                late_change: true,
                ..Storage::new(None)
            };
            let error = finish(
                intent(BrowserBindingBackend::ExternalFile, accepted)
                    .publish_bytes(&storage, STAGED),
            )
            .unwrap_err();
            if abort_failure {
                assert_eq!(
                    error,
                    BrowserWriteError::ChangedDuringStaging {
                        observed_digest: digest_bytes(EXTERNAL),
                        abort_error: "abort failed".to_owned(),
                    }
                );
                assert_eq!(
                    error.to_string(),
                    "canonical browser project changed while staged bytes were pending, and staging abort failed: abort failed"
                );
            } else {
                assert_eq!(
                    error,
                    BrowserWriteError::ExternalChange(digest_bytes(EXTERNAL))
                );
            }
            assert_eq!(
                *storage.steps.borrow(),
                ["observe", "create", "write", "verify", "abort"]
            );
            assert_eq!(*storage.file.borrow(), EXTERNAL);
            assert_eq!(storage.staged.borrow().is_some(), abort_failure);
        }
    }
}

#[test]
fn successful_close_without_matching_readback_cannot_accept_a_digest() {
    for fault in [Fault::ReadBack, Fault::Mismatch] {
        let storage = Storage::new(Some(fault));
        let error = publish(&storage).unwrap_err();
        if fault == Fault::ReadBack {
            assert_eq!(
                error,
                BrowserWriteError::ReadBackFailed("read-back failed".to_owned())
            );
            assert_eq!(
                error.to_string(),
                "browser write completed, but read-back verification failed: read-back failed"
            );
        } else {
            assert_eq!(
                error,
                BrowserWriteError::ReadBackMismatch {
                    staged_digest: digest_bytes(STAGED),
                    observed_digest: digest_bytes(EXTERNAL),
                }
            );
            assert_eq!(
                error.to_string(),
                "browser write completed, but read-back bytes do not match the staged project"
            );
        }
        assert_eq!(
            *storage.steps.borrow(),
            ["observe", "create", "write", "verify", "close", "read-back"]
        );
    }
}
