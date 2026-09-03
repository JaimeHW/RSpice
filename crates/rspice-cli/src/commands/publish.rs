//! Result-artifact publication for one CLI invocation.
//!
//! Every result file the `run` and `export` commands produce goes through
//! [`artifact`] (or [`artifact_group`] for files that only make sense
//! together). Without an open transaction each call publishes one file
//! atomically, exactly as a direct `rspice_output::write_atomic` would.
//!
//! A multi-coordinate deck opens a transaction with [`begin`] before its
//! first coordinate runs. Every publication then stages its complete bytes
//! beside its destination and joins the transaction instead of replacing the
//! destination, and [`RunTransaction::commit`] publishes the whole set — plus
//! the set manifest — after the last coordinate. A cancellation, a failure,
//! or a panic drops the guard, which removes every staging file and leaves
//! the destination directory exactly as the run found it. That is what makes
//! "either the complete coordinate set or nothing" true for a `.STEP` or
//! `.TEMP` deck.
//!
//! The active transaction is per thread, because a multi-run deck can run
//! independent decks on worker threads and each deck owns its own set. A
//! worker that publishes into a transaction opened on another thread joins it
//! explicitly with [`enter`]; there is no implicit inheritance, so a
//! publication either joins a named transaction or is its own.

use std::cell::RefCell;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rspice_output::{
    AtomicArtifactError, AtomicArtifactSet, AtomicArtifactSetError, CommitOperation,
    DestinationState, StagedArtifact, write_atomic,
};

use crate::cli::CliError;

thread_local! {
    static ACTIVE: RefCell<Option<Arc<RunTransactionState>>> = const { RefCell::new(None) };
}

/// Shared state of one open publication transaction.
#[derive(Debug)]
pub struct RunTransactionState {
    set: Mutex<AtomicArtifactSet>,
}

impl RunTransactionState {
    fn adopt(&self, staged: StagedArtifact, manifest: bool) -> Result<(), CliError> {
        let destination = staged.destination().to_path_buf();
        let mut set = self.lock()?;
        let joined = if manifest {
            set.adopt_manifest(staged)
        } else {
            set.adopt(staged)
        };
        joined.map_err(|error| CliError::output_error(&destination, membership_io_error(&error)))
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, AtomicArtifactSet>, CliError> {
        self.set.lock().map_err(|_| CliError::InternalError {
            message: "the artifact publication transaction was poisoned by a failed publisher"
                .to_string(),
        })
    }
}

/// Guard for an open transaction. Dropping it without [`Self::commit`]
/// abandons every staged member and leaves the destinations unchanged.
#[derive(Debug)]
pub struct RunTransaction {
    state: Option<Arc<RunTransactionState>>,
}

/// Guard that makes an existing transaction current on a worker thread.
pub struct EnteredTransaction {
    previous: Option<Arc<RunTransactionState>>,
}

/// Open a publication transaction on this thread.
///
/// Transactions do not nest: a deck that already publishes into one cannot
/// start a second, because the two sets would have to commit independently.
pub fn begin() -> Result<RunTransaction, CliError> {
    let state = Arc::new(RunTransactionState {
        set: Mutex::new(AtomicArtifactSet::new()),
    });
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        if active.is_some() {
            return Err(CliError::InternalError {
                message: "an artifact publication transaction is already open on this thread"
                    .to_string(),
            });
        }
        *active = Some(Arc::clone(&state));
        Ok(())
    })?;
    Ok(RunTransaction { state: Some(state) })
}

/// The transaction publications on this thread join, if any.
#[must_use]
pub fn current() -> Option<Arc<RunTransactionState>> {
    ACTIVE.with(|active| active.borrow().clone())
}

/// Make `state` the transaction this thread publishes into until the returned
/// guard is dropped. Worker pools use this to keep a fan-out inside the
/// transaction its parent opened.
#[must_use]
pub fn enter(state: Arc<RunTransactionState>) -> EnteredTransaction {
    let previous = ACTIVE.with(|active| active.borrow_mut().replace(state));
    EnteredTransaction { previous }
}

impl RunTransaction {
    /// Publish every staged member, or none of them.
    pub fn commit(mut self) -> Result<(), CliError> {
        let state = self.close();
        let Some(state) = state else {
            return Ok(());
        };
        let set = std::mem::take(&mut *state.lock()?);
        let destination = set
            .destinations()
            .next()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        set.commit()
            .map_err(|error| map_set_error(&destination, &error))
    }

    /// Detach this transaction from the thread and hand back its state.
    fn close(&mut self) -> Option<Arc<RunTransactionState>> {
        let state = self.state.take()?;
        ACTIVE.with(|active| {
            let mut active = active.borrow_mut();
            if active
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, &state))
            {
                *active = None;
            }
        });
        Some(state)
    }
}

impl Drop for RunTransaction {
    fn drop(&mut self) {
        // Abandoning the set removes every staging file; the destinations
        // keep the bytes they had before the run started.
        self.close();
    }
}

impl Drop for EnteredTransaction {
    fn drop(&mut self) {
        let previous = self.previous.take();
        ACTIVE.with(|active| *active.borrow_mut() = previous);
    }
}

/// Publish one result artifact.
///
/// With a transaction open on this thread the bytes are staged and published
/// when the transaction commits; otherwise the destination is replaced before
/// this returns. Either way the destination holds its previous bytes, or none
/// at all, until a complete artifact exists.
pub fn artifact<E, F>(path: &Path, write: F) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
{
    match current() {
        None => write_atomic(path, write),
        Some(state) => {
            let staged = StagedArtifact::stage(path, write).map_err(set_stage_error)?;
            state.adopt(staged, false).map_err(joining_error)
        }
    }
}

/// Publish the manifest that describes a transaction's coordinate set.
///
/// The manifest is committed last, so a reader that sees it sees every member
/// it names. Without an open transaction it is an ordinary single artifact.
pub fn set_manifest<E, F>(path: &Path, write: F) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
{
    match current() {
        None => write_atomic(path, write),
        Some(state) => {
            let staged = StagedArtifact::stage(path, write).map_err(set_stage_error)?;
            state.adopt(staged, true).map_err(joining_error)
        }
    }
}

/// Publish two artifacts that are only meaningful together, such as a
/// transient result and the FFT result derived from it.
///
/// Both are staged before either destination is touched. With a transaction
/// open they join it and are published with the rest of the set; without one
/// they are published as their own two-member transaction, so a failure on
/// the second leaves the first destination byte-identical.
pub fn artifact_pair<E, F, G>(
    first: &Path,
    write_first: F,
    second: &Path,
    write_second: G,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
    G: FnOnce(&mut dyn Write) -> Result<(), E>,
{
    let staged_first = StagedArtifact::stage(first, write_first).map_err(set_stage_error)?;
    let staged_second = StagedArtifact::stage(second, write_second).map_err(set_stage_error)?;

    match current() {
        Some(state) => {
            state.adopt(staged_first, false).map_err(joining_error)?;
            state.adopt(staged_second, false).map_err(joining_error)
        }
        None => {
            let mut set = AtomicArtifactSet::new();
            set.adopt(staged_first)
                .and_then(|()| set.adopt(staged_second))
                .map_err(|error| commit_error(membership_io_error(&error)))?;
            set.commit().map_err(|error| {
                commit_error(std::io::Error::other(
                    map_set_error(second, &error).to_string(),
                ))
            })
        }
    }
}

/// Remove the staging files a killed process left in `directory`, and say
/// what the pass did.
///
/// Only the files whose owning process is provably gone are removed; a stage
/// belonging to this process or to another live one is left alone.
pub fn recover_stale_artifacts(directory: &Path, quiet: bool) {
    match rspice_output::recover_stale_artifacts(directory) {
        Ok(report) => {
            if report.is_empty() {
                log::debug!(
                    "artifact recovery in {}: {}",
                    directory.display(),
                    report.summary()
                );
                return;
            }
            log::info!(
                "artifact recovery in {}: {}",
                directory.display(),
                report.summary()
            );
            if !quiet {
                for removed in &report.removed {
                    eprintln!(
                        "Removed a staging file left by an interrupted run: {}",
                        removed.display()
                    );
                }
            }
        }
        // Recovery is opportunistic: it must never fail a run that has not
        // written anything yet.
        Err(error) => log::warn!(
            "artifact recovery in {} did not run: {error}",
            directory.display()
        ),
    }
}

fn map_set_error(destination: &Path, error: &AtomicArtifactSetError<std::io::Error>) -> CliError {
    let path = match error {
        AtomicArtifactSetError::Stage { destination, .. }
        | AtomicArtifactSetError::Predecessor { destination, .. }
        | AtomicArtifactSetError::Commit { destination, .. } => destination.clone(),
        AtomicArtifactSetError::Membership(_) | AtomicArtifactSetError::Finalize { .. } => {
            destination.to_path_buf()
        }
    };
    CliError::output_error(&path, std::io::Error::other(error.to_string()))
}

/// A staged member of a set failed before any destination changed, so the
/// single-artifact vocabulary reports it as a preparation or write failure.
fn set_stage_error<E>(error: AtomicArtifactSetError<E>) -> AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    match error {
        AtomicArtifactSetError::Stage { source, .. } => source,
        other => AtomicArtifactError::Prepare(std::io::Error::other(other.to_string())),
    }
}

fn joining_error<E>(error: CliError) -> AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    commit_error(std::io::Error::other(error.to_string()))
}

fn commit_error<E>(source: std::io::Error) -> AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    AtomicArtifactError::Commit {
        operation: CommitOperation::PreCommit,
        destination_state: DestinationState::Unchanged,
        source,
    }
}

fn membership_io_error(error: &rspice_output::SetMembershipError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::AlreadyExists, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-cli-publish-{}-{id}-{tag}",
                std::process::id()
            ));
            std::fs::create_dir_all(&path).expect("create publication test directory");
            Self(path)
        }

        fn join(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn publish(path: &Path, bytes: &'static [u8]) -> Result<(), CliError> {
        artifact(path, |writer| {
            writer
                .write_all(bytes)
                .map_err(|error| CliError::output_error(path, error))
        })
        .map_err(|error| crate::cli::map_atomic_output_error(path, error))
    }

    #[test]
    fn without_a_transaction_each_artifact_is_published_immediately() {
        let directory = TestDirectory::new("standalone");
        let destination = directory.join("result.csv");

        publish(&destination, b"complete").expect("publish standalone artifact");

        assert_eq!(
            std::fs::read(&destination).expect("read published artifact"),
            b"complete"
        );
    }

    #[test]
    fn an_abandoned_transaction_publishes_no_member() {
        let directory = TestDirectory::new("abandoned");
        let first = directory.join("first.csv");
        let second = directory.join("second.csv");
        std::fs::write(&second, b"old second").expect("seed predecessor");

        {
            let _transaction = begin().expect("open transaction");
            publish(&first, b"new first").expect("stage first member");
            publish(&second, b"new second").expect("stage second member");
            assert!(!first.exists(), "a staged member replaced its destination");
            assert_eq!(
                std::fs::read(&second).expect("read predecessor"),
                b"old second"
            );
        }

        assert!(!first.exists(), "an abandoned member was published");
        assert_eq!(
            std::fs::read(&second).expect("read predecessor"),
            b"old second"
        );
        assert!(current().is_none(), "the transaction stayed open");
    }

    #[test]
    fn a_committed_transaction_publishes_every_member_and_its_manifest() {
        let directory = TestDirectory::new("committed");
        let first = directory.join("first.csv");
        let second = directory.join("second.csv");
        let manifest = directory.join("set.json");

        let transaction = begin().expect("open transaction");
        publish(&first, b"new first").expect("stage first member");
        publish(&second, b"new second").expect("stage second member");
        set_manifest(&manifest, |writer| {
            writer
                .write_all(b"{\"members\":2}")
                .map_err(|error| CliError::output_error(&manifest, error))
        })
        .map_err(|error| crate::cli::map_atomic_output_error(&manifest, error))
        .expect("stage manifest");
        transaction.commit().expect("commit transaction");

        assert_eq!(std::fs::read(&first).expect("read first"), b"new first");
        assert_eq!(std::fs::read(&second).expect("read second"), b"new second");
        assert_eq!(
            std::fs::read(&manifest).expect("read manifest"),
            b"{\"members\":2}"
        );
        assert!(current().is_none());
    }

    #[test]
    fn a_worker_thread_publishes_into_the_transaction_it_enters() {
        let directory = TestDirectory::new("worker");
        let member = directory.join("worker.csv");

        let transaction = begin().expect("open transaction");
        let handle = current().expect("the open transaction is current");
        std::thread::scope(|scope| {
            scope.spawn(|| {
                let _entered = enter(handle);
                publish(&member, b"worker member").expect("stage from a worker thread");
                assert!(!member.exists(), "a worker published before the commit");
            });
        });
        transaction.commit().expect("commit transaction");

        assert_eq!(
            std::fs::read(&member).expect("read worker member"),
            b"worker member"
        );
    }

    /// A coordinate set that fails while it is being published must leave the
    /// destination directory exactly as the run found it.
    ///
    /// The timeout and kill cases are covered end-to-end by
    /// `tests/step_set_transaction.rs`, which cancels a real swept run. What
    /// no deck can produce on demand is a failure *inside* the commit — a full
    /// device, a refused allocation, a replace that loses a race — so those
    /// come from `rspice_output`'s qualification seam, armed on this thread
    /// for the duration of the transaction.
    #[test]
    fn a_transaction_that_fails_mid_commit_leaves_every_predecessor_intact() {
        use rspice_output::fault::{ArmedFaults, ArtifactFault, ArtifactFaultPoint};

        for (tag, fault) in [
            (
                "replace",
                ArtifactFault::io(ArtifactFaultPoint::Replace).on_member(1),
            ),
            (
                "manifest",
                ArtifactFault::allocation(ArtifactFaultPoint::CommitManifest),
            ),
            (
                "predecessor",
                ArtifactFault::io(ArtifactFaultPoint::CapturePredecessor).on_member(0),
            ),
        ] {
            let directory = TestDirectory::new(tag);
            let first = directory.join("coordinate-0.csv");
            let second = directory.join("coordinate-1.csv");
            let manifest = directory.join("set.json");
            std::fs::write(&first, b"old first").expect("seed predecessor");
            std::fs::write(&manifest, b"old manifest").expect("seed manifest predecessor");

            let armed = ArmedFaults::arm(fault);
            let transaction = begin().expect("open transaction");
            publish(&first, b"new first").expect("stage first member");
            publish(&second, b"new second").expect("stage second member");
            set_manifest(&manifest, |writer| {
                writer
                    .write_all(b"{\"members\":2}")
                    .map_err(|error| CliError::output_error(&manifest, error))
            })
            .map_err(|error| crate::cli::map_atomic_output_error(&manifest, error))
            .expect("stage manifest");
            let outcome = transaction.commit();
            assert_eq!(armed.fired(), 1, "the {tag} fault never fired");
            drop(armed);

            assert!(outcome.is_err(), "the {tag} fault did not fail the commit");
            assert_eq!(
                std::fs::read(&first).expect("read the first predecessor"),
                b"old first",
                "the {tag} fault replaced a published coordinate"
            );
            assert!(
                !second.exists(),
                "the {tag} fault published a coordinate that had no predecessor"
            );
            assert_eq!(
                std::fs::read(&manifest).expect("read the manifest predecessor"),
                b"old manifest",
                "the {tag} fault replaced the set manifest"
            );
            for destination in [&first, &second, &manifest] {
                assert!(
                    rspice_output::stale_artifacts(destination)
                        .expect("list staging files")
                        .is_empty(),
                    "the {tag} fault left a staging file beside {}",
                    destination.display()
                );
            }
            assert!(current().is_none(), "the transaction stayed open");
        }
    }

    #[test]
    fn transactions_do_not_nest() {
        let _outer = begin().expect("open transaction");
        let error = begin().expect_err("a nested transaction must fail closed");
        assert!(error.to_string().contains("already open"), "{error}");
    }

    #[test]
    fn a_pair_without_a_transaction_publishes_both_or_neither() {
        let directory = TestDirectory::new("pair");
        let first = directory.join("transient.csv");
        let second = directory.join("fft.csv");
        std::fs::write(&first, b"old transient").expect("seed predecessor");
        std::fs::create_dir(&second).expect("create commit-failing destination");

        let error = artifact_pair::<CliError, _, _>(
            &first,
            |writer| {
                writer
                    .write_all(b"new transient")
                    .map_err(|error| CliError::output_error(&first, error))
            },
            &second,
            |writer| {
                writer
                    .write_all(b"new fft")
                    .map_err(|error| CliError::output_error(&second, error))
            },
        )
        .expect_err("a directory destination must fail the pair");
        assert!(
            matches!(error, AtomicArtifactError::Commit { .. }),
            "{error}"
        );
        assert_eq!(
            std::fs::read(&first).expect("read restored predecessor"),
            b"old transient"
        );
        assert!(second.is_dir());
    }
}
