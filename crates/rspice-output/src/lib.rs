//! Transactional publication of native artifacts.
//!
//! A writer receives a buffered stream backed by a uniquely created sibling
//! of the destination. The destination is not changed until serialization and
//! the file durability work have completed. Publication then uses a
//! same-directory atomic replace operation appropriate for the host platform.
//!
//! # One publication policy
//!
//! Every artifact this crate publishes uses the same policy, so no caller can
//! weaken the durability of one result: the complete staging file is flushed
//! and synchronized, the replace itself is durable
//! (`MOVEFILE_WRITE_THROUGH` on Windows), the published directory entry is
//! synchronized where the host exposes that operation (`fsync` of the parent
//! directory on Unix), and a failed replace removes the staging file rather
//! than leaving a successor behind. The policy is deliberately not a
//! parameter.
//!
//! # One file or a set
//!
//! [`AtomicArtifactFile`] and [`write_atomic`] publish a single file.
//! [`AtomicArtifactSet`] publishes several files as one transaction: every
//! member is staged and completed first, every predecessor is snapshotted,
//! and a commit failure restores every predecessor byte-identically (or
//! removes destinations that did not exist) before the error is returned.
//!
//! # Recovery
//!
//! Staging files left by a process crash contain [`STAGING_MARKER`], the
//! destination file name, and the owning process id. [`stale_artifacts`]
//! lists them for one destination and [`recover_stale_artifacts`] removes the
//! ones whose owning process is gone during a controlled recovery pass.
//! Predecessor snapshots taken by an interrupted [`AtomicArtifactSet`] commit
//! carry [`PREDECESSOR_MARKER`] instead and are never removed automatically,
//! because such a snapshot can be the last copy of a published result.

mod recovery;
mod set;

pub use recovery::{MAX_RECOVERY_ENTRIES, StagingRecovery, recover_stale_artifacts};
pub use set::{
    AtomicArtifactSet, AtomicArtifactSetError, RollbackOutcome, SetMembershipError, StagedArtifact,
};

use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use thiserror::Error;

/// Stable marker used to identify an RSpice staging artifact.
pub const STAGING_MARKER: &str = ".rspice-output-";

/// Stable marker used to identify a predecessor snapshot taken by an
/// [`AtomicArtifactSet`] commit so that it can be restored on rollback.
pub const PREDECESSOR_MARKER: &str = ".rspice-predecessor-";

const MAX_STAGING_ATTEMPTS: u64 = 1_024;
static NEXT_STAGING_ID: AtomicU64 = AtomicU64::new(0);

/// Operation that failed while flushing or synchronizing staged data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlushOperation {
    FlushBuffer,
    SyncFile,
}

impl std::fmt::Display for FlushOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FlushBuffer => formatter.write_str("buffer flush"),
            Self::SyncFile => formatter.write_str("file synchronization"),
        }
    }
}

/// Operation that failed while publishing a complete staging artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitOperation {
    PreCommit,
    Replace,
    SyncParent,
}

impl std::fmt::Display for CommitOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreCommit => formatter.write_str("pre-commit validation"),
            Self::Replace => formatter.write_str("atomic replace"),
            Self::SyncParent => formatter.write_str("parent synchronization"),
        }
    }
}

/// Destination state when a commit operation reports an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationState {
    /// Publication did not occur; an old destination is byte-identical.
    Unchanged,
    /// Replacement occurred, but synchronizing its directory entry failed.
    PublishedDurabilityUncertain,
}

/// Typed failure phases for atomic artifact publication.
#[derive(Debug, Error)]
pub enum AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    #[error("artifact preparation failed: {0}")]
    Prepare(#[source] io::Error),
    #[error("artifact write failed: {0}")]
    Write(#[source] E),
    #[error("artifact {operation} failed: {source}")]
    Flush {
        operation: FlushOperation,
        #[source]
        source: io::Error,
    },
    #[error("artifact {operation} failed: {source}")]
    Commit {
        operation: CommitOperation,
        destination_state: DestinationState,
        #[source]
        source: io::Error,
    },
}

impl AtomicArtifactError<io::Error> {
    /// Collapse a publication failure into one [`io::Error`] that keeps the
    /// failing phase in its message and the underlying error kind.
    #[must_use]
    pub fn into_io_error(self) -> io::Error {
        let kind = match &self {
            Self::Prepare(source) | Self::Write(source) => source.kind(),
            Self::Flush { source, .. } | Self::Commit { source, .. } => source.kind(),
        };
        io::Error::new(kind, self)
    }
}

/// Seekable, same-directory staging file for a long-running artifact write.
///
/// All writes target a uniquely created sibling of the destination. Calling
/// [`Self::commit`] flushes and synchronizes the completed file, then
/// atomically replaces the destination. Dropping this value without
/// committing closes and removes the staging file, leaving an existing
/// destination byte-identical.
#[derive(Debug)]
pub struct AtomicArtifactFile {
    destination: PathBuf,
    file: Option<File>,
    cleanup: StagingCleanup,
}

/// A completely written, flushed, and synchronized artifact that has not yet
/// replaced its destination.
///
/// This split phase is useful when a logical result consists of several
/// sibling files: callers can prepare every member before publishing the
/// first one. Dropping a prepared artifact removes only its staging file and
/// leaves the destination unchanged.
#[derive(Debug)]
pub struct PreparedAtomicArtifact {
    destination: PathBuf,
    cleanup: StagingCleanup,
}

impl AtomicArtifactFile {
    /// Prepare a seekable staging file beside `destination`.
    pub fn prepare(destination: &Path) -> Result<Self, AtomicArtifactError<io::Error>> {
        Self::prepare_impl::<io::Error, _>(destination, &mut NoFaults)
    }

    /// Publish the completed staging file atomically.
    ///
    /// Errors before replacement leave the destination unchanged. A parent
    /// directory synchronization error is reported with
    /// [`DestinationState::PublishedDurabilityUncertain`] because replacement
    /// has already occurred at that point.
    pub fn commit(self) -> Result<(), AtomicArtifactError<io::Error>> {
        self.commit_impl::<io::Error, _>(&mut NoFaults)
    }

    /// Finish all fallible data flushing and synchronization without
    /// publishing the artifact.
    ///
    /// Once this succeeds, [`PreparedAtomicArtifact::commit`] performs only
    /// destination validation, atomic replacement, and parent directory
    /// synchronization.
    pub fn prepare_for_commit(
        self,
    ) -> Result<PreparedAtomicArtifact, AtomicArtifactError<io::Error>> {
        self.prepare_for_commit_impl::<io::Error, _>(&mut NoFaults)
    }

    pub(crate) fn prepare_for_commit_impl<E, H>(
        mut self,
        hooks: &mut H,
    ) -> Result<PreparedAtomicArtifact, AtomicArtifactError<E>>
    where
        E: std::error::Error + 'static,
        H: FaultHooks,
    {
        hooks
            .check(FaultPoint::Flush)
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::FlushBuffer,
                source,
            })?;
        self.open_file_mut()
            .and_then(Write::flush)
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::FlushBuffer,
                source,
            })?;
        self.open_file_mut()
            .and_then(|file| file.sync_all())
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::SyncFile,
                source,
            })?;
        hooks
            .check(FaultPoint::AfterFlush)
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::SyncFile,
                source,
            })?;
        self.file.take();
        let cleanup = std::mem::replace(&mut self.cleanup, StagingCleanup { path: None });
        Ok(PreparedAtomicArtifact {
            destination: self.destination.clone(),
            cleanup,
        })
    }

    pub(crate) fn prepare_impl<E, H>(
        destination: &Path,
        hooks: &mut H,
    ) -> Result<Self, AtomicArtifactError<E>>
    where
        E: std::error::Error + 'static,
        H: FaultHooks,
    {
        reject_symlink_destination(destination).map_err(AtomicArtifactError::Prepare)?;
        let (staging_path, file) =
            create_staging_file(destination).map_err(AtomicArtifactError::Prepare)?;
        let artifact = Self {
            destination: destination.to_path_buf(),
            file: Some(file),
            cleanup: StagingCleanup::new(staging_path),
        };

        hooks
            .check(FaultPoint::AfterPrepare)
            .map_err(AtomicArtifactError::Prepare)?;
        Ok(artifact)
    }

    fn commit_impl<E, H>(mut self, hooks: &mut H) -> Result<(), AtomicArtifactError<E>>
    where
        E: std::error::Error + 'static,
        H: FaultHooks,
    {
        hooks
            .check(FaultPoint::Flush)
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::FlushBuffer,
                source,
            })?;
        self.open_file_mut()
            .and_then(Write::flush)
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::FlushBuffer,
                source,
            })?;
        self.open_file_mut()
            .and_then(|file| file.sync_all())
            .map_err(|source| AtomicArtifactError::Flush {
                operation: FlushOperation::SyncFile,
                source,
            })?;

        hooks
            .check(FaultPoint::AfterFlush)
            .map_err(precommit_error)?;
        self.file.take();

        let cleanup = std::mem::replace(&mut self.cleanup, StagingCleanup { path: None });
        PreparedAtomicArtifact {
            destination: self.destination.clone(),
            cleanup,
        }
        .commit_impl(hooks)
    }

    fn open_file_mut(&mut self) -> io::Result<&mut File> {
        self.file.as_mut().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "atomic artifact staging file is already closed",
            )
        })
    }
}

impl PreparedAtomicArtifact {
    /// Atomically replace the destination with this fully prepared artifact.
    pub fn commit(self) -> Result<(), AtomicArtifactError<io::Error>> {
        self.commit_impl::<io::Error, _>(&mut NoFaults)
    }

    /// Destination this prepared artifact will replace.
    #[must_use]
    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub(crate) fn commit_impl<E, H>(mut self, hooks: &mut H) -> Result<(), AtomicArtifactError<E>>
    where
        E: std::error::Error + 'static,
        H: FaultHooks,
    {
        hooks
            .check(FaultPoint::BeforeCommit)
            .map_err(precommit_error)?;
        reject_symlink_destination(&self.destination).map_err(precommit_error)?;

        let staging_path = self.cleanup.path().map_err(precommit_error)?.to_path_buf();
        let replace_result = hooks
            .check(FaultPoint::Replace)
            .and_then(|()| commit_staging_file(&staging_path, &self.destination));
        if let Err(source) = replace_result {
            self.cleanup.remove_now();
            return Err(AtomicArtifactError::Commit {
                operation: CommitOperation::Replace,
                destination_state: DestinationState::Unchanged,
                source,
            });
        }
        self.cleanup.disarm();

        hooks
            .check(FaultPoint::SyncParent)
            .and_then(|()| sync_parent_directory(&self.destination))
            .map_err(|source| AtomicArtifactError::Commit {
                operation: CommitOperation::SyncParent,
                destination_state: DestinationState::PublishedDurabilityUncertain,
                source,
            })?;
        Ok(())
    }
}

impl Write for AtomicArtifactFile {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.open_file_mut()?.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.open_file_mut()?.flush()
    }

    fn write_vectored(&mut self, buffers: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.open_file_mut()?.write_vectored(buffers)
    }
}

impl Seek for AtomicArtifactFile {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        self.open_file_mut()?.seek(position)
    }
}

impl Drop for AtomicArtifactFile {
    fn drop(&mut self) {
        self.file.take();
        self.cleanup.remove_now();
    }
}

/// Stream one artifact into a staging file and publish it atomically.
///
/// The closure writes through a buffered stream. Returning an error (including
/// cancellation) leaves the destination unchanged and removes the stage.
pub fn write_atomic<E, F>(destination: &Path, write: F) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
{
    write_atomic_impl(destination, write, &mut NoFaults)
}

/// List crash-left staging files associated with `destination`.
///
/// A returned file may belong to an active writer. Callers must not infer
/// staleness solely from its name.
pub fn stale_artifacts(destination: &Path) -> io::Result<Vec<PathBuf>> {
    let destination_name = destination.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "artifact destination must name a file",
        )
    })?;
    let parent = destination_parent(destination);
    let mut artifacts = Vec::new();

    for entry in std::fs::read_dir(parent)? {
        let entry = entry?;
        if is_staging_name(&entry.file_name(), destination_name) {
            artifacts.push(entry.path());
        }
    }
    artifacts.sort();
    Ok(artifacts)
}

fn write_atomic_impl<E, F, H>(
    destination: &Path,
    write: F,
    hooks: &mut H,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
    H: FaultHooks,
{
    let artifact = AtomicArtifactFile::prepare_impl::<E, _>(destination, hooks)?;
    let mut writer = BufWriter::new(artifact);
    if let Err(source) = write(&mut writer) {
        drop(writer);
        return Err(AtomicArtifactError::Write(source));
    }

    let artifact = writer
        .into_inner()
        .map_err(|error| AtomicArtifactError::Flush {
            operation: FlushOperation::FlushBuffer,
            source: error.into_error(),
        })?;
    artifact.commit_impl::<E, _>(hooks)
}

fn precommit_error<E>(source: io::Error) -> AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    AtomicArtifactError::Commit {
        operation: CommitOperation::PreCommit,
        destination_state: DestinationState::Unchanged,
        source,
    }
}

pub(crate) fn create_staging_file(destination: &Path) -> io::Result<(PathBuf, File)> {
    create_sibling_file(destination, STAGING_MARKER, ".tmp")
}

/// Create a uniquely named sibling of `destination` whose name records the
/// marker, the owning process id, and a process-local serial number.
pub(crate) fn create_sibling_file(
    destination: &Path,
    marker: &str,
    suffix: &str,
) -> io::Result<(PathBuf, File)> {
    let file_name = destination.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "artifact destination must name a file",
        )
    })?;
    let parent = destination_parent(destination);
    let process_id = std::process::id();

    for _ in 0..MAX_STAGING_ATTEMPTS {
        let id = NEXT_STAGING_ID.fetch_add(1, Ordering::Relaxed);
        let mut staging_name = sibling_prefix(file_name, marker);
        staging_name.push(format!("{process_id}-{id}{suffix}"));
        let staging_path = parent.join(staging_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staging_path)
        {
            Ok(file) => return Ok((staging_path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!(
            "could not allocate a unique staging artifact beside {}",
            destination.display()
        ),
    ))
}

fn sibling_prefix(destination_name: &OsStr, marker: &str) -> OsString {
    let mut prefix = OsString::from(".");
    prefix.push(destination_name);
    prefix.push(marker);
    prefix
}

fn is_staging_name(candidate: &OsStr, destination_name: &OsStr) -> bool {
    candidate
        .as_encoded_bytes()
        .starts_with(sibling_prefix(destination_name, STAGING_MARKER).as_encoded_bytes())
}

pub(crate) fn destination_parent(destination: &Path) -> &Path {
    match destination.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

/// Synchronize the directory entry containing an artifact where the host
/// exposes a directory durability primitive. Multi-file publishers use this
/// after rollback or recovery namespace changes before reporting completion.
pub fn sync_artifact_parent(destination: &Path) -> io::Result<()> {
    sync_parent_directory(destination)
}

/// Replace `destination` with a complete same-filesystem recovery file using
/// the platform's durable atomic-replace path. On success, `recovery` has
/// been consumed.
pub fn restore_artifact_durably(recovery: &Path, destination: &Path) -> io::Result<()> {
    reject_symlink_destination(destination)?;
    commit_staging_file(recovery, destination)?;
    sync_parent_directory(destination)
}

/// Remove an artifact with a durable destination-absence transition.
///
/// Windows first moves the file to a uniquely created RSpice staging name
/// with `MOVEFILE_WRITE_THROUGH`, then deletes that tombstone. If the final
/// tombstone deletion is interrupted, ordinary stale-artifact recovery can
/// remove it without resurrecting the public destination.
pub fn remove_artifact_durably(destination: &Path) -> io::Result<()> {
    remove_artifact_durably_impl(destination)
}

#[cfg(not(windows))]
fn remove_artifact_durably_impl(destination: &Path) -> io::Result<()> {
    std::fs::remove_file(destination)?;
    sync_parent_directory(destination)
}

#[cfg(windows)]
fn remove_artifact_durably_impl(destination: &Path) -> io::Result<()> {
    let (tombstone, reservation) = create_staging_file(destination)?;
    drop(reservation);
    if let Err(error) = commit_staging_file(destination, &tombstone) {
        let _ = std::fs::remove_file(&tombstone);
        return Err(error);
    }
    std::fs::remove_file(tombstone)
}

fn reject_symlink_destination(destination: &Path) -> io::Result<()> {
    match std::fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace symlink artifact destination {}",
                destination.display()
            ),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    std::fs::rename(staging, destination)
}

#[cfg(windows)]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    fn wide_path(path: &Path) -> io::Result<Vec<u16>> {
        let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
        if wide.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "artifact path contains an embedded NUL",
            ));
        }
        wide.push(0);
        Ok(wide)
    }

    let staging_wide = wide_path(staging)?;
    let destination_wide = wide_path(destination)?;
    let flags = MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH;

    // SAFETY: Both buffers are NUL-terminated and remain alive through this
    // call. Staging beside the destination guarantees a same-volume move.
    let succeeded = unsafe { MoveFileExW(staging_wide.as_ptr(), destination_wide.as_ptr(), flags) };
    if succeeded == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
pub(crate) fn sync_parent_directory(destination: &Path) -> io::Result<()> {
    File::open(destination_parent(destination))?.sync_all()
}

#[cfg(any(windows, not(any(unix, windows))))]
pub(crate) fn sync_parent_directory(_destination: &Path) -> io::Result<()> {
    // Windows requests write-through on the atomic move itself. Other targets
    // without a directory synchronization primitive honor "where supported".
    Ok(())
}

#[derive(Debug)]
struct StagingCleanup {
    path: Option<PathBuf>,
}

impl StagingCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    fn remove_now(&mut self) {
        if let Some(path) = self.path.take()
            && let Err(error) = std::fs::remove_file(&path)
            && error.kind() != io::ErrorKind::NotFound
        {
            self.path = Some(path);
        }
    }

    fn path(&self) -> io::Result<&Path> {
        self.path.as_deref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "atomic artifact staging path is already disarmed",
            )
        })
    }

    fn disarm(&mut self) {
        self.path = None;
    }
}

impl Drop for StagingCleanup {
    fn drop(&mut self) {
        self.remove_now();
    }
}

/// Injection points used by the crate's transactional tests. Production code
/// always runs with [`NoFaults`], so the seam costs one inlined `Ok(())`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FaultPoint {
    AfterPrepare,
    Flush,
    AfterFlush,
    BeforeCommit,
    Replace,
    SyncParent,
    /// Staging one member of an [`AtomicArtifactSet`].
    SetStage,
    /// Snapshotting one member's predecessor before any member is committed.
    SetPredecessor,
    /// Committing the set's manifest member, which is always committed last.
    SetManifestCommit,
}

pub(crate) trait FaultHooks {
    fn check(&mut self, point: FaultPoint) -> io::Result<()>;

    /// Announce which set member the following checks belong to, so a test
    /// can target the single-file points of one member.
    fn enter_member(&mut self, _index: usize) {}
}

pub(crate) struct NoFaults;

impl FaultHooks for NoFaults {
    fn check(&mut self, _point: FaultPoint) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::{AtomicU64, Ordering, Path, PathBuf};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    /// A uniquely named directory removed when the test ends.
    pub(crate) struct TestDirectory(PathBuf);

    impl TestDirectory {
        pub(crate) fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("rspice-output-{}-{id}-{tag}", std::process::id()));
            std::fs::create_dir(&path).expect("create unique atomic-output test directory");
            Self(path)
        }

        pub(crate) fn path(&self) -> &Path {
            &self.0
        }

        pub(crate) fn destination(&self) -> PathBuf {
            self.0.join("result.csv")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tests_support::TestDirectory;
    use super::*;

    struct InjectFault(FaultPoint);

    impl FaultHooks for InjectFault {
        fn check(&mut self, point: FaultPoint) -> io::Result<()> {
            if self.0 == point {
                Err(io::Error::other(format!("injected {point:?} failure")))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn explicit_parent_synchronization_accepts_an_artifact_destination() {
        let directory = TestDirectory::new("parent-sync");
        let destination = directory.destination();
        std::fs::write(&destination, b"complete").expect("write synchronized fixture");
        sync_artifact_parent(&destination).expect("synchronize artifact parent");
    }

    #[test]
    fn durable_recovery_replace_consumes_the_predecessor_snapshot() {
        let directory = TestDirectory::new("durable-restore");
        let destination = directory.destination();
        let recovery = directory.path().join("recovery.csv");
        std::fs::write(&destination, b"successor").expect("write successor");
        std::fs::write(&recovery, b"predecessor").expect("write recovery snapshot");

        restore_artifact_durably(&recovery, &destination).expect("restore predecessor durably");

        assert_eq!(
            std::fs::read(&destination).expect("read restored predecessor"),
            b"predecessor"
        );
        assert!(!recovery.exists());
    }

    #[test]
    fn durable_remove_leaves_no_public_or_staging_artifact() {
        let directory = TestDirectory::new("durable-remove");
        let destination = directory.destination();
        std::fs::write(&destination, b"published").expect("write published artifact");

        remove_artifact_durably(&destination).expect("remove artifact durably");

        assert!(!destination.exists());
        assert!(
            stale_artifacts(&destination)
                .expect("list durable-remove stages")
                .is_empty()
        );
    }

    #[test]
    fn dropping_prepared_artifact_preserves_destination_and_cleans_stage() {
        let directory = TestDirectory::new("prepared-drop");
        let destination = directory.path().join("result.bin");
        std::fs::write(&destination, b"predecessor").expect("write predecessor");
        let mut artifact =
            AtomicArtifactFile::prepare(&destination).expect("prepare split-phase artifact");
        artifact.write_all(b"successor").expect("write successor");
        let prepared = artifact
            .prepare_for_commit()
            .expect("finish split-phase staging");
        drop(prepared);

        assert_eq!(
            std::fs::read(&destination).expect("read predecessor"),
            b"predecessor"
        );
        assert!(
            stale_artifacts(&destination)
                .expect("list stages")
                .is_empty()
        );
    }

    #[test]
    fn prepared_artifact_commit_replaces_destination() {
        let directory = TestDirectory::new("prepared-commit");
        let destination = directory.path().join("result.bin");
        std::fs::write(&destination, b"predecessor").expect("write predecessor");
        let mut artifact =
            AtomicArtifactFile::prepare(&destination).expect("prepare split-phase artifact");
        artifact.write_all(b"successor").expect("write successor");
        artifact
            .prepare_for_commit()
            .expect("finish split-phase staging")
            .commit()
            .expect("commit prepared artifact");

        assert_eq!(
            std::fs::read(&destination).expect("read successor"),
            b"successor"
        );
        assert!(
            stale_artifacts(&destination)
                .expect("list stages")
                .is_empty()
        );
    }

    #[test]
    fn prepared_commit_failure_preserves_invalid_destination_and_cleans_stage() {
        let directory = TestDirectory::new("prepared-invalid-destination");
        let destination = directory.path().join("result.bin");
        std::fs::create_dir(&destination).expect("create conflicting destination directory");
        let mut artifact =
            AtomicArtifactFile::prepare(&destination).expect("prepare split-phase artifact");
        artifact.write_all(b"successor").expect("write successor");
        let error = artifact
            .prepare_for_commit()
            .expect("finish split-phase staging")
            .commit()
            .expect_err("directory replacement must fail");

        assert!(matches!(
            error,
            AtomicArtifactError::Commit {
                destination_state: DestinationState::Unchanged,
                ..
            }
        ));
        assert!(destination.is_dir());
        assert!(
            stale_artifacts(&destination)
                .expect("list stages")
                .is_empty()
        );
    }

    fn seed(destination: &Path, preexisting: bool) {
        if preexisting {
            std::fs::write(destination, b"old complete artifact").expect("seed existing artifact");
        }
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved artifact"),
                b"old complete artifact"
            );
        } else {
            assert!(!destination.exists(), "failed write published an artifact");
        }
    }

    fn assert_no_stages(destination: &Path) {
        assert_eq!(
            stale_artifacts(destination).expect("inspect staging artifacts"),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn persistent_staging_drop_is_transactional() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("persistent-drop");
            let destination = directory.destination();
            seed(&destination, preexisting);

            let mut artifact =
                AtomicArtifactFile::prepare(&destination).expect("prepare persistent staging file");
            artifact
                .write_all(b"partial replacement")
                .expect("write staged bytes");
            assert_eq!(
                stale_artifacts(&destination)
                    .expect("inspect active persistent staging artifact")
                    .len(),
                1
            );
            drop(artifact);

            assert_old_or_absent(&destination, preexisting);
            assert_no_stages(&destination);
        }
    }

    #[test]
    fn persistent_staging_commit_faults_are_transactional() {
        for preexisting in [false, true] {
            for fault in [
                FaultPoint::Flush,
                FaultPoint::AfterFlush,
                FaultPoint::BeforeCommit,
                FaultPoint::Replace,
            ] {
                let directory = TestDirectory::new("persistent-fault");
                let destination = directory.destination();
                seed(&destination, preexisting);
                let mut artifact = AtomicArtifactFile::prepare(&destination)
                    .expect("prepare persistent staging file");
                artifact
                    .write_all(b"complete replacement")
                    .expect("write staged bytes");

                artifact
                    .commit_impl::<io::Error, _>(&mut InjectFault(fault))
                    .expect_err("injected pre-publication fault must propagate");

                assert_old_or_absent(&destination, preexisting);
                assert_no_stages(&destination);
            }
        }
    }

    #[test]
    fn persistent_staging_commit_replaces_only_with_complete_bytes() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("persistent-success");
            let destination = directory.destination();
            seed(&destination, preexisting);
            let mut artifact =
                AtomicArtifactFile::prepare(&destination).expect("prepare persistent staging file");
            artifact
                .write_all(b"new complete artifact")
                .expect("write complete staged artifact");

            artifact.commit().expect("commit persistent artifact");

            assert_eq!(
                std::fs::read(&destination).expect("read committed artifact"),
                b"new complete artifact"
            );
            assert_no_stages(&destination);
        }
    }

    #[test]
    fn after_prepare_header_and_mid_write_failures_are_transactional() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("after-prepare");
            let destination = directory.destination();
            seed(&destination, preexisting);
            let error = write_atomic_impl::<io::Error, _, _>(
                &destination,
                |_| Ok(()),
                &mut InjectFault(FaultPoint::AfterPrepare),
            )
            .expect_err("post-prepare failure must propagate");
            assert!(matches!(error, AtomicArtifactError::Prepare(_)));
            assert_old_or_absent(&destination, preexisting);
            assert_no_stages(&destination);

            for (tag, partial) in [
                ("after-header", b"time,V(out)\n".as_slice()),
                ("mid-write", b"time,V(out)\n0,1\n1,".as_slice()),
            ] {
                let directory = TestDirectory::new(tag);
                let destination = directory.destination();
                seed(&destination, preexisting);
                let error = write_atomic(&destination, |writer| -> io::Result<()> {
                    writer.write_all(partial)?;
                    Err(io::Error::other("injected serializer failure"))
                })
                .expect_err("writer failure must propagate");
                assert!(matches!(error, AtomicArtifactError::Write(_)));
                assert_old_or_absent(&destination, preexisting);
                assert_no_stages(&destination);
            }
        }
    }

    #[test]
    fn flush_after_flush_and_precommit_failures_are_transactional() {
        for preexisting in [false, true] {
            for fault in [
                FaultPoint::Flush,
                FaultPoint::AfterFlush,
                FaultPoint::BeforeCommit,
            ] {
                let directory = TestDirectory::new("publication-fault");
                let destination = directory.destination();
                seed(&destination, preexisting);
                let error = write_atomic_impl(
                    &destination,
                    |writer| writer.write_all(b"new complete artifact"),
                    &mut InjectFault(fault),
                )
                .expect_err("publication fault must propagate");
                if fault == FaultPoint::Flush {
                    assert!(matches!(error, AtomicArtifactError::Flush { .. }));
                } else {
                    assert!(matches!(
                        error,
                        AtomicArtifactError::Commit {
                            operation: CommitOperation::PreCommit,
                            destination_state: DestinationState::Unchanged,
                            ..
                        }
                    ));
                }
                assert_old_or_absent(&destination, preexisting);
                assert_no_stages(&destination);
            }
        }
    }

    #[test]
    fn successful_commit_replaces_only_with_complete_bytes() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.destination();
            seed(&destination, preexisting);

            write_atomic(&destination, |writer| {
                writer.write_all(b"new complete artifact")
            })
            .expect("atomic artifact publication succeeds");

            assert_eq!(
                std::fs::read(&destination).expect("read committed artifact"),
                b"new complete artifact"
            );
            assert_no_stages(&destination);
        }
    }

    #[test]
    fn replace_failure_removes_the_successor_and_keeps_the_predecessor() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("replace-failure");
            let destination = directory.destination();
            seed(&destination, preexisting);
            let error = write_atomic_impl(
                &destination,
                |writer| writer.write_all(b"new complete artifact"),
                &mut InjectFault(FaultPoint::Replace),
            )
            .expect_err("replace failure must propagate");

            assert!(matches!(
                error,
                AtomicArtifactError::Commit {
                    operation: CommitOperation::Replace,
                    destination_state: DestinationState::Unchanged,
                    ..
                }
            ));
            assert_old_or_absent(&destination, preexisting);
            assert_no_stages(&destination);
        }
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn symlink_destination_is_rejected_without_modifying_target() {
        let directory = TestDirectory::new("symlink");
        let target = directory.path().join("target.csv");
        let destination = directory.destination();
        std::fs::write(&target, b"symlink target").expect("write symlink target");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &destination).expect("create artifact symlink");
        #[cfg(windows)]
        if let Err(error) = std::os::windows::fs::symlink_file(&target, &destination) {
            if error.kind() == io::ErrorKind::PermissionDenied || error.raw_os_error() == Some(1314)
            {
                return;
            }
            panic!("create artifact symlink: {error}");
        }

        let error = write_atomic(&destination, |writer| writer.write_all(b"replacement"))
            .expect_err("symlink destination must be rejected");
        assert!(matches!(error, AtomicArtifactError::Prepare(_)));
        assert_eq!(
            std::fs::read(&target).expect("read symlink target"),
            b"symlink target"
        );
        assert!(destination.is_symlink());
        assert_no_stages(&destination);
    }
}
