//! Transactional publication of native single-file artifacts.
//!
//! A writer receives a buffered stream backed by a uniquely created sibling
//! of the destination. The destination is not changed until serialization and
//! the requested file durability work have completed. Publication then uses a
//! same-directory atomic replace operation appropriate for the host platform.
//!
//! Staging files left by a process crash contain [`STAGING_MARKER`] and the
//! destination file name. [`stale_artifacts`] lists them, and
//! [`cleanup_stale_artifacts`] removes them during a controlled recovery pass
//! when the caller knows no writer for that destination is active.

use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use thiserror::Error;

/// Stable marker used to identify an RSpice staging artifact.
pub const STAGING_MARKER: &str = ".rspice-output-";

const MAX_STAGING_ATTEMPTS: u64 = 1_024;
static NEXT_STAGING_ID: AtomicU64 = AtomicU64::new(0);

/// Durability requested before and during publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Durability {
    /// Flush the Rust buffered stream before replacing the destination.
    Flush,
    /// Flush and synchronize the staging file before replacement.
    SyncFile,
    /// Synchronize the file and the published directory entry where the host
    /// exposes that operation. Windows uses `MOVEFILE_WRITE_THROUGH`; Unix
    /// additionally synchronizes the parent directory after `rename`.
    SyncFileAndParent,
}

/// What to do with a complete staging file when the atomic replace itself
/// fails. Failures before the replace attempt are always cleaned up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitFailurePolicy {
    /// Remove the staging file on a failed replace.
    Cleanup,
    /// Retain it and return its path in [`AtomicArtifactError::Commit`].
    PreserveForRecovery,
}

/// Policy for one atomic artifact publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtomicArtifactOptions {
    durability: Durability,
    commit_failure: CommitFailurePolicy,
}

impl AtomicArtifactOptions {
    /// Create an explicit publication policy.
    #[must_use]
    pub const fn new(durability: Durability) -> Self {
        Self {
            durability,
            commit_failure: CommitFailurePolicy::Cleanup,
        }
    }

    /// Select the policy used only when the atomic replace attempt fails.
    #[must_use]
    pub const fn with_commit_failure_policy(mut self, policy: CommitFailurePolicy) -> Self {
        self.commit_failure = policy;
        self
    }

    /// Requested durability.
    #[must_use]
    pub const fn durability(self) -> Durability {
        self.durability
    }
}

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
        /// Complete staging artifact retained by policy, when any.
        recovery_path: Option<PathBuf>,
        #[source]
        source: io::Error,
    },
}

/// Seekable, same-directory staging file for a long-running artifact write.
///
/// All writes target a uniquely created sibling of the destination. Calling
/// [`Self::commit`] flushes and synchronizes the completed file according to
/// the configured durability policy, then atomically replaces the
/// destination. Dropping this value without committing closes and removes the
/// staging file, leaving an existing destination byte-identical.
#[derive(Debug)]
pub struct AtomicArtifactFile {
    destination: PathBuf,
    file: Option<File>,
    cleanup: StagingCleanup,
    options: AtomicArtifactOptions,
}

impl AtomicArtifactFile {
    /// Prepare a seekable staging file beside `destination`.
    pub fn prepare(
        destination: &Path,
        options: AtomicArtifactOptions,
    ) -> Result<Self, AtomicArtifactError<io::Error>> {
        Self::prepare_impl::<io::Error, _>(destination, options, &mut NoFaults)
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

    fn prepare_impl<E, H>(
        destination: &Path,
        options: AtomicArtifactOptions,
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
            options,
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

        if matches!(
            self.options.durability,
            Durability::SyncFile | Durability::SyncFileAndParent
        ) {
            self.open_file_mut()
                .and_then(|file| file.sync_all())
                .map_err(|source| AtomicArtifactError::Flush {
                    operation: FlushOperation::SyncFile,
                    source,
                })?;
        }

        hooks
            .check(FaultPoint::AfterFlush)
            .map_err(precommit_error)?;
        self.file.take();

        hooks
            .check(FaultPoint::BeforeCommit)
            .map_err(precommit_error)?;
        reject_symlink_destination(&self.destination).map_err(precommit_error)?;

        let staging_path = self.cleanup.path().map_err(precommit_error)?.to_path_buf();
        let replace_result = hooks.check(FaultPoint::Replace).and_then(|()| {
            commit_staging_file(&staging_path, &self.destination, self.options.durability)
        });
        if let Err(source) = replace_result {
            let recovery_path = match self.options.commit_failure {
                CommitFailurePolicy::Cleanup => {
                    self.cleanup.remove_now();
                    None
                }
                CommitFailurePolicy::PreserveForRecovery => {
                    self.cleanup.disarm();
                    Some(staging_path)
                }
            };
            return Err(AtomicArtifactError::Commit {
                operation: CommitOperation::Replace,
                destination_state: DestinationState::Unchanged,
                recovery_path,
                source,
            });
        }
        self.cleanup.disarm();

        if self.options.durability == Durability::SyncFileAndParent {
            hooks
                .check(FaultPoint::SyncParent)
                .and_then(|()| sync_parent_directory(&self.destination))
                .map_err(|source| AtomicArtifactError::Commit {
                    operation: CommitOperation::SyncParent,
                    destination_state: DestinationState::PublishedDurabilityUncertain,
                    recovery_path: None,
                    source,
                })?;
        }

        Ok(())
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
pub fn write_atomic<E, F>(
    destination: &Path,
    options: AtomicArtifactOptions,
    write: F,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
{
    write_atomic_impl(destination, options, write, &mut NoFaults)
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

/// Remove crash-left staging files during a controlled recovery pass.
///
/// The caller must ensure no publication for `destination` is active. The
/// removed paths are returned for diagnostics/audit logging.
pub fn cleanup_stale_artifacts(destination: &Path) -> io::Result<Vec<PathBuf>> {
    let artifacts = stale_artifacts(destination)?;
    for artifact in &artifacts {
        std::fs::remove_file(artifact)?;
    }
    Ok(artifacts)
}

fn write_atomic_impl<E, F, H>(
    destination: &Path,
    options: AtomicArtifactOptions,
    write: F,
    hooks: &mut H,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut dyn Write) -> Result<(), E>,
    H: FaultHooks,
{
    let artifact = AtomicArtifactFile::prepare_impl::<E, _>(destination, options, hooks)?;
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
        recovery_path: None,
        source,
    }
}

fn create_staging_file(destination: &Path) -> io::Result<(PathBuf, File)> {
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
        let mut staging_name = staging_prefix(file_name);
        staging_name.push(format!("{process_id}-{id}.tmp"));
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

fn staging_prefix(destination_name: &OsStr) -> OsString {
    let mut prefix = OsString::from(".");
    prefix.push(destination_name);
    prefix.push(STAGING_MARKER);
    prefix
}

fn is_staging_name(candidate: &OsStr, destination_name: &OsStr) -> bool {
    candidate
        .as_encoded_bytes()
        .starts_with(staging_prefix(destination_name).as_encoded_bytes())
}

fn destination_parent(destination: &Path) -> &Path {
    match destination.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
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
fn commit_staging_file(
    staging: &Path,
    destination: &Path,
    _durability: Durability,
) -> io::Result<()> {
    std::fs::rename(staging, destination)
}

#[cfg(windows)]
fn commit_staging_file(
    staging: &Path,
    destination: &Path,
    durability: Durability,
) -> io::Result<()> {
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
    let mut flags = MOVEFILE_REPLACE_EXISTING;
    if durability == Durability::SyncFileAndParent {
        flags |= MOVEFILE_WRITE_THROUGH;
    }

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
fn sync_parent_directory(destination: &Path) -> io::Result<()> {
    File::open(destination_parent(destination))?.sync_all()
}

#[cfg(any(windows, not(any(unix, windows))))]
fn sync_parent_directory(_destination: &Path) -> io::Result<()> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultPoint {
    AfterPrepare,
    Flush,
    AfterFlush,
    BeforeCommit,
    Replace,
    SyncParent,
}

trait FaultHooks {
    fn check(&mut self, point: FaultPoint) -> io::Result<()>;
}

struct NoFaults;

impl FaultHooks for NoFaults {
    fn check(&mut self, _point: FaultPoint) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

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

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("rspice-output-{}-{id}-{tag}", std::process::id()));
            std::fs::create_dir(&path).expect("create unique atomic-output test directory");
            Self(path)
        }

        fn destination(&self) -> PathBuf {
            self.0.join("result.csv")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn options() -> AtomicArtifactOptions {
        AtomicArtifactOptions::new(Durability::SyncFileAndParent)
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

            let mut artifact = AtomicArtifactFile::prepare(&destination, options())
                .expect("prepare persistent staging file");
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
                let mut artifact = AtomicArtifactFile::prepare(&destination, options())
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
            let mut artifact = AtomicArtifactFile::prepare(&destination, options())
                .expect("prepare persistent staging file");
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
                options(),
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
                let error = write_atomic(&destination, options(), |writer| -> io::Result<()> {
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
                    options(),
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

            write_atomic(&destination, options(), |writer| {
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
    fn replace_failure_obeys_cleanup_or_recovery_policy() {
        for policy in [
            CommitFailurePolicy::Cleanup,
            CommitFailurePolicy::PreserveForRecovery,
        ] {
            let directory = TestDirectory::new("replace-failure");
            let destination = directory.destination();
            seed(&destination, true);
            let error = write_atomic_impl(
                &destination,
                options().with_commit_failure_policy(policy),
                |writer| writer.write_all(b"new complete artifact"),
                &mut InjectFault(FaultPoint::Replace),
            )
            .expect_err("replace failure must propagate");

            let recovery_path = match error {
                AtomicArtifactError::Commit {
                    operation: CommitOperation::Replace,
                    destination_state: DestinationState::Unchanged,
                    recovery_path,
                    ..
                } => recovery_path,
                other => panic!("unexpected error: {other}"),
            };
            assert_old_or_absent(&destination, true);
            match policy {
                CommitFailurePolicy::Cleanup => {
                    assert!(recovery_path.is_none());
                    assert_no_stages(&destination);
                }
                CommitFailurePolicy::PreserveForRecovery => {
                    let recovery_path = recovery_path.expect("recovery artifact path");
                    assert_eq!(
                        std::fs::read(&recovery_path).expect("read recovery artifact"),
                        b"new complete artifact"
                    );
                    assert_eq!(
                        stale_artifacts(&destination).expect("list recovery artifact"),
                        vec![recovery_path.clone()]
                    );
                    assert_eq!(
                        cleanup_stale_artifacts(&destination).expect("clean recovery artifact"),
                        vec![recovery_path]
                    );
                    assert_no_stages(&destination);
                }
            }
        }
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn symlink_destination_is_rejected_without_modifying_target() {
        let directory = TestDirectory::new("symlink");
        let target = directory.0.join("target.csv");
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

        let error = write_atomic(&destination, options(), |writer| {
            writer.write_all(b"replacement")
        })
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
