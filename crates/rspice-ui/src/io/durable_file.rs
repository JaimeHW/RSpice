//! Crash-consistent native file publication.
//!
//! A successful return is the durability boundary used by primary project,
//! schematic, and recovery files: bytes are written to a unique sibling,
//! synchronized, atomically published, and the containing directory is then
//! synchronized on platforms that require it.

#![cfg(not(target_arch = "wasm32"))]

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};
use uuid::Uuid;

const UNIQUE_TEMP_ATTEMPTS: usize = 16;
const MAX_HASHED_FILE_BYTES: u64 = 512 * 1024 * 1024;
const LEASE_HEADER_MAGIC: [u8; 16] = *b"RSPICE-LEASE-V2!";
const LEASE_HEADER_PREFIX_LEN: usize = 64;
const LEASE_HEADER_LEN: usize = 96;
const RECOVERY_SLOT_A_SUFFIX: &str = ".rspice.recovery-v1.a";
const RECOVERY_SLOT_B_SUFFIX: &str = ".rspice.recovery-v1.b";
const RECOVERY_BACKUP_SUFFIX: &str = ".rspice.replace-backup-v1.";
#[cfg(windows)]
const RECOVERY_RETIRED_SUFFIX: &str = ".retired";
const RECOVERY_HEADER_MAGIC: [u8; 16] = *b"RSPICE-RECOV-V1!";
const RECOVERY_RESOLUTION_MAGIC: [u8; 16] = *b"RSPICE-RESOL-V1!";
const RECOVERY_HEADER_PREFIX_LEN: usize = 176;
const RECOVERY_HEADER_LEN: usize = 208;
const RECOVERY_RESOLUTION_PREFIX_LEN: usize = 48;
const RECOVERY_RESOLUTION_LEN: usize = 80;
const RECOVERY_FLAG_BEFORE_PRESENT: u8 = 1;
const RECOVERY_FLAG_READ_ONLY: u8 = 2;

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpectedContent {
    /// Publication is valid only while the destination does not exist.
    Missing,
    /// Publication is valid only while the destination still contains the
    /// exact bytes accepted by the caller.
    Digest([u8; 32]),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CompareExchangeError {
    #[error("destination changed before publication (expected {expected:?}, found {actual:?})")]
    Conflict {
        expected: ExpectedContent,
        actual: Option<[u8; 32]>,
    },
    #[error(
        "project publication outcome is uncertain: {message}; preserved recovery files: {recovery_paths:?}"
    )]
    PublicationUncertain {
        message: String,
        recovery_paths: Vec<PathBuf>,
    },
    #[error("another RSpice writer owns the destination lease: {0}")]
    LeaseBusy(PathBuf),
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Result of a platform publication primitive before the higher-level
/// transaction decides whether an unpublished staging file may be removed.
///
/// `Safe` is returned only when the original pathname state was observed (or
/// restored and verified) after the platform call failed. `Uncertain` always
/// retains every path that may contain predecessor or successor evidence.
#[derive(Debug, thiserror::Error)]
enum PublicationFailure {
    #[error(transparent)]
    Safe(io::Error),
    #[error("{message}; preserved recovery files: {recovery_paths:?}")]
    Uncertain {
        message: String,
        recovery_paths: Vec<PathBuf>,
    },
}

impl PublicationFailure {
    fn into_io(self) -> io::Error {
        match self {
            Self::Safe(error) => error,
            Self::Uncertain {
                message,
                recovery_paths,
            } => io::Error::other(format!(
                "publication outcome is uncertain: {message}; preserved recovery files: {recovery_paths:?}"
            )),
        }
    }

    fn into_compare_exchange(self) -> CompareExchangeError {
        match self {
            Self::Safe(error) => CompareExchangeError::Io(error),
            Self::Uncertain {
                message,
                recovery_paths,
            } => CompareExchangeError::PublicationUncertain {
                message,
                recovery_paths,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservedFileRole {
    Missing,
    Replaced,
    Replacement,
    Both,
    Other,
    Unreadable,
}

#[cfg(windows)]
impl ObservedFileRole {
    const fn is_replaced(self) -> bool {
        matches!(self, Self::Replaced | Self::Both)
    }

    const fn is_replacement(self) -> bool {
        matches!(self, Self::Replacement | Self::Both)
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReplaceObservation {
    replaced: ObservedFileRole,
    replacement: ObservedFileRole,
    backup: ObservedFileRole,
}

#[cfg(windows)]
impl ReplaceObservation {
    const fn input_is_intact(self) -> bool {
        self.replaced.is_replaced()
            && self.replacement.is_replacement()
            && matches!(self.backup, ObservedFileRole::Missing)
    }

    const fn success_is_complete(self) -> bool {
        self.replaced.is_replacement()
            && matches!(self.replacement, ObservedFileRole::Missing)
            && self.backup.is_replaced()
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplaceFailureRecovery {
    InputIntact,
    RestoreBackupIntoReplaced,
    RestoreCompletedReplacement,
    Uncertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationResolution {
    Committed,
    Aborted,
}

impl PublicationResolution {
    const fn marker(self) -> u8 {
        match self {
            Self::Committed => 1,
            Self::Aborted => 2,
        }
    }

    const fn from_marker(marker: u8) -> Option<Self> {
        match marker {
            1 => Some(Self::Committed),
            2 => Some(Self::Aborted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecoveryRecordState {
    Prepared,
    Resolved(PublicationResolution),
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecoveryReconciliation {
    Resolve(PublicationResolution),
    AcceptResolved,
    PreserveAndBlock,
}

/// Decide restart reconciliation from durable journal state and exact bytes.
/// Prepared means uncommitted: exact successor bytes are evidence, never
/// permission to infer a commit. Recovery may settle only the predecessor
/// endpoint. Existing foreign bytes and missing expected bytes are never
/// overwritten or resurrected.
#[cfg(windows)]
const fn recovery_reconciliation(
    state: RecoveryRecordState,
    observed_target: ObservedFileRole,
    before_present: bool,
) -> RecoveryReconciliation {
    match state {
        RecoveryRecordState::Prepared => match observed_target {
            ObservedFileRole::Replaced => {
                RecoveryReconciliation::Resolve(PublicationResolution::Aborted)
            }
            ObservedFileRole::Both => {
                RecoveryReconciliation::Resolve(PublicationResolution::Aborted)
            }
            ObservedFileRole::Missing if !before_present => {
                RecoveryReconciliation::Resolve(PublicationResolution::Aborted)
            }
            ObservedFileRole::Missing
            | ObservedFileRole::Replacement
            | ObservedFileRole::Other
            | ObservedFileRole::Unreadable => RecoveryReconciliation::PreserveAndBlock,
        },
        RecoveryRecordState::Resolved(PublicationResolution::Committed) => match observed_target {
            ObservedFileRole::Replacement | ObservedFileRole::Both => {
                RecoveryReconciliation::AcceptResolved
            }
            ObservedFileRole::Missing
            | ObservedFileRole::Replaced
            | ObservedFileRole::Other
            | ObservedFileRole::Unreadable => RecoveryReconciliation::PreserveAndBlock,
        },
        RecoveryRecordState::Resolved(PublicationResolution::Aborted) => match observed_target {
            ObservedFileRole::Replaced | ObservedFileRole::Both => {
                RecoveryReconciliation::AcceptResolved
            }
            ObservedFileRole::Missing if !before_present => RecoveryReconciliation::AcceptResolved,
            ObservedFileRole::Missing
            | ObservedFileRole::Replacement
            | ObservedFileRole::Other
            | ObservedFileRole::Unreadable => RecoveryReconciliation::PreserveAndBlock,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecoveryRecord {
    generation: u64,
    transaction: Uuid,
    backup_id: Uuid,
    target_binding: [u8; 32],
    before_present: bool,
    predecessor_len: u64,
    successor_len: u64,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
    predecessor_read_only: bool,
    header_checksum: [u8; 32],
    state: RecoveryRecordState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActiveRecoveryRecord {
    slot_index: usize,
    path: PathBuf,
    record: RecoveryRecord,
}

/// Decide recovery from observed bytes, never from an optimistic assumption
/// about a Win32 error code. This covers all documented ReplaceFileW partial
/// postconditions and remains fail-closed if a filesystem violates them.
#[cfg(windows)]
const fn replacement_failure_recovery(observation: ReplaceObservation) -> ReplaceFailureRecovery {
    if observation.input_is_intact() {
        ReplaceFailureRecovery::InputIntact
    } else if matches!(observation.replaced, ObservedFileRole::Missing)
        && observation.replacement.is_replacement()
        && observation.backup.is_replaced()
    {
        ReplaceFailureRecovery::RestoreBackupIntoReplaced
    } else if observation.success_is_complete() {
        ReplaceFailureRecovery::RestoreCompletedReplacement
    } else {
        ReplaceFailureRecovery::Uncertain
    }
}

struct DestinationLease {
    path: PathBuf,
    file: File,
}

impl DestinationLease {
    fn acquire(target: &Path) -> Result<Self, CompareExchangeError> {
        let lease = Self::acquire_without_reconciliation(target)?;
        #[cfg(windows)]
        reconcile_windows_recovery_slots(target)
            .map_err(PublicationFailure::into_compare_exchange)?;
        #[cfg(unix)]
        reconcile_unix_recovery_slots(target).map_err(PublicationFailure::into_compare_exchange)?;
        Ok(lease)
    }

    fn acquire_without_reconciliation(target: &Path) -> Result<Self, CompareExchangeError> {
        let path = lease_path(target);
        let target_binding = lease_target_binding(target);
        install_lease_if_missing(&path, target_binding)?;
        let mut file = open_lease_nofollow(&path)?;
        if let Err(error) = try_lock_exclusive(&file) {
            return if matches!(error.kind(), io::ErrorKind::WouldBlock) {
                Err(CompareExchangeError::LeaseBusy(path))
            } else {
                Err(error.into())
            };
        }
        validate_open_lease(&path, &mut file, target_binding)?;
        Ok(Self { path, file })
    }
}

impl Drop for DestinationLease {
    fn drop(&mut self) {
        // The inode is permanent. Removing a lock pathname after unlocking
        // has an unavoidable read-owner/remove race that can delete another
        // writer's replacement lease. Keeping it is safe and bounded: exactly
        // one small lease file exists per project destination.
        let _ = unlock_file(&self.file);
        let _ = &self.path;
    }
}

/// Reconcile durable native publication evidence before a caller parses or
/// hashes the canonical pathname. The destination lease serializes this with
/// every cooperating writer. Recovery is deliberately non-destructive: an
/// ambiguous or externally changed endpoint is preserved and reported.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn reconcile_publication(path: &Path) -> Result<(), CompareExchangeError> {
    std::fs::create_dir_all(parent_directory(path))?;
    let _lease = DestinationLease::acquire_without_reconciliation(path)?;
    #[cfg(windows)]
    reconcile_windows_recovery_slots(path).map_err(PublicationFailure::into_compare_exchange)?;
    #[cfg(unix)]
    reconcile_unix_recovery_slots(path).map_err(PublicationFailure::into_compare_exchange)?;
    Ok(())
}

/// Capture the exact picker-time destination state after reconciling any
/// durable publication evidence. Callers receive a compare-and-exchange token,
/// never overwrite authority.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn observe_expected_content(
    path: &Path,
) -> Result<ExpectedContent, CompareExchangeError> {
    std::fs::create_dir_all(parent_directory(path))?;
    let _lease = DestinationLease::acquire(path)?;
    Ok(match current_digest(path)? {
        Some(digest) => ExpectedContent::Digest(digest),
        None => ExpectedContent::Missing,
    })
}

/// Publish a file without exposing partially serialized bytes at `path`.
///
/// The callback may stream arbitrarily large payloads into the temporary file.
/// If it or any durability step fails, the previous target remains in place
/// whenever publication has not already completed, and callers receive an
/// error so they can retain dirty state and recovery evidence.
pub(crate) fn atomic_write_with<E>(
    path: &Path,
    write: impl FnOnce(&mut File) -> Result<(), E>,
) -> Result<(), E>
where
    E: From<io::Error>,
{
    let parent = parent_directory(path);
    std::fs::create_dir_all(parent).map_err(E::from)?;
    let _lease = DestinationLease::acquire(path).map_err(|error| {
        E::from(io::Error::other(format!(
            "could not acquire publication lease for '{}': {error}",
            path.display()
        )))
    })?;
    let predecessor_permissions = std::fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());

    let (temp, mut file) = create_unique_sibling(path).map_err(E::from)?;

    #[cfg(windows)]
    if path.exists()
        && let Err(error) = copy_windows_file_security(path, &temp)
    {
        drop(file);
        remove_unpublished_temp(&temp);
        return Err(E::from(io::Error::new(
            error.kind(),
            format!(
                "could not stage the security contract from '{}' onto '{}': {error}",
                path.display(),
                temp.display()
            ),
        )));
    }

    let staged = (|| {
        write(&mut file)?;
        file.flush().map_err(E::from)?;
        #[cfg(not(windows))]
        if let Some(permissions) = predecessor_permissions.as_ref() {
            file.set_permissions(permissions.clone()).map_err(E::from)?;
        }
        file.sync_all().map_err(E::from)?;
        Ok(())
    })();
    drop(file);

    if let Err(error) = staged {
        remove_unpublished_temp(&temp);
        return Err(error);
    }

    let staged_digest = match hash_file_bounded(&temp) {
        Ok(digest) => digest,
        Err(error) => {
            remove_unpublished_temp(&temp);
            return Err(E::from(error));
        }
    };
    let displaced = match publish_retaining_predecessor(&temp, path, staged_digest) {
        Ok(displaced) => displaced,
        Err(PublicationFailure::Safe(error)) => {
            if let Err(cleanup_error) = remove_transaction_temp(&temp) {
                return Err(E::from(io::Error::new(
                    cleanup_error.kind(),
                    format!(
                        "publication failed ({error}) and staged successor cleanup failed: {cleanup_error}"
                    ),
                )));
            }
            if let Err(retirement_error) = retire_publication_recovery(path) {
                return Err(E::from(retirement_error.into_io()));
            }
            return Err(E::from(error));
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => {
            return Err(E::from(error.into_io()));
        }
    };

    match current_digest(path) {
        Ok(Some(digest)) if digest == staged_digest => {}
        Ok(actual) => {
            return Err(E::from(io::Error::other(format!(
                "'{}' publication returned success, but the target digest is {actual:?} instead of the staged digest; preserved predecessor evidence: {displaced:?}",
                path.display()
            ))));
        }
        Err(error) => {
            return Err(E::from(io::Error::new(
                error.kind(),
                format!(
                    "'{}' publication returned success, but the target could not be verified: {error}; preserved predecessor evidence: {displaced:?}",
                    path.display()
                ),
            )));
        }
    }

    #[cfg(windows)]
    if let Err(error) =
        sync_windows_published_successor(path, staged_digest, predecessor_permissions.as_ref())
    {
        return Err(E::from(io::Error::new(
            error.kind(),
            format!(
                "'{}' was published and verified, but its file handle could not be synchronized; preserved predecessor evidence: {displaced:?}: {error}",
                path.display()
            ),
        )));
    }

    #[cfg(windows)]
    if !matches!(current_digest(path), Ok(Some(digest)) if digest == staged_digest) {
        return Err(E::from(io::Error::other(format!(
            "'{}' changed while its published handle was being synchronized; preserved predecessor evidence: {displaced:?}",
            path.display()
        ))));
    }

    if let Err(error) = sync_parent_directory(parent) {
        return Err(E::from(io::Error::new(
            error.kind(),
            format!(
                "'{}' was published and verified, but its containing directory could not be synchronized; preserved predecessor evidence: {displaced:?}: {error}",
                path.display()
            ),
        )));
    }

    if let Err(error) = resolve_publication_recovery(
        path,
        displaced.as_deref(),
        staged_digest,
        PublicationResolution::Committed,
    ) {
        return Err(E::from(error.into_io()));
    }

    if let Some(predecessor) = displaced {
        if let Err(error) = std::fs::remove_file(&predecessor) {
            return Err(E::from(io::Error::new(
                error.kind(),
                format!(
                    "'{}' was published and verified, but predecessor cleanup failed; preserved '{}': {error}",
                    path.display(),
                    predecessor.display()
                ),
            )));
        }
        sync_parent_directory(parent).map_err(|error| {
            E::from(io::Error::new(
                error.kind(),
                format!(
                    "'{}' was published and predecessor cleanup completed, but cleanup durability is uncertain: {error}",
                    path.display()
                ),
            ))
        })?;
    }
    retire_publication_recovery(path).map_err(|error| E::from(error.into_io()))?;
    Ok(())
}

pub(crate) fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> io::Result<()> {
    atomic_write_with(path, |file| file.write_all(bytes))
}

/// Durably publish `bytes` only if the destination still matches the
/// caller's accepted state.
///
/// A durable per-destination lease serializes cooperating RSpice writers. For
/// an existing destination, the platform atomically retains the displaced
/// predecessor, verifies its exact digest, and rolls it back on conflict.
/// Recovery evidence is never deleted when publication or rollback becomes
/// uncertain.
pub(crate) fn compare_exchange_bytes(
    path: &Path,
    expected: ExpectedContent,
    bytes: &[u8],
) -> Result<(), CompareExchangeError> {
    compare_exchange_bytes_impl(path, expected, bytes, false)
}

/// Unix-only publication for secret material. Both the staged successor and
/// canonical file are forced to owner-read/write permissions before any
/// secret-bearing predecessor evidence can be retired.
#[cfg(unix)]
pub(crate) fn compare_exchange_bytes_owner_only(
    path: &Path,
    expected: ExpectedContent,
    bytes: &[u8],
) -> Result<(), CompareExchangeError> {
    compare_exchange_bytes_impl(path, expected, bytes, true)
}

fn compare_exchange_bytes_impl(
    path: &Path,
    expected: ExpectedContent,
    bytes: &[u8],
    owner_only: bool,
) -> Result<(), CompareExchangeError> {
    let parent = parent_directory(path);
    std::fs::create_dir_all(parent)?;
    let _lease = DestinationLease::acquire(path)?;
    // Reject an already-stale caller before staging or touching the canonical
    // pathname. The durable lease serializes cooperating RSpice writers; the
    // post-publication predecessor check below remains necessary for a race
    // with an external writer after this inexpensive preflight observation.
    let initially_observed = current_digest(path)?;
    let initially_matches = match expected {
        ExpectedContent::Missing => initially_observed.is_none(),
        ExpectedContent::Digest(digest) => initially_observed == Some(digest),
    };
    if !initially_matches {
        return Err(CompareExchangeError::Conflict {
            expected,
            actual: initially_observed,
        });
    }
    let staged_digest: [u8; 32] = Sha256::digest(bytes).into();
    let predecessor_permissions = std::fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());
    let (temp, mut file) = create_unique_sibling(path)?;

    // ReplaceFileW cannot reliably merge the destination ACL when the caller
    // lacks WRITE_DAC, so the Windows publication path intentionally asks it
    // to ignore ACL merge errors. That is safe only when the staged successor
    // already carries the predecessor's complete owner/group/DACL contract.
    // `atomic_write_with` establishes the same invariant; compare-and-
    // exchange must do so as well before it can enter `replace_file_checked`.
    #[cfg(windows)]
    if initially_observed.is_some()
        && let Err(error) = copy_windows_file_security(path, &temp)
    {
        drop(file);
        remove_unpublished_temp(&temp);
        return Err(io::Error::new(
            error.kind(),
            format!(
                "could not stage the security contract from '{}' onto '{}': {error}",
                path.display(),
                temp.display()
            ),
        )
        .into());
    }

    let staged = (|| -> io::Result<()> {
        file.write_all(bytes)?;
        file.flush()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;

            if owner_only {
                file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
            } else if let Some(permissions) = predecessor_permissions.as_ref() {
                file.set_permissions(permissions.clone())?;
            }
        }
        #[cfg(all(not(windows), not(unix)))]
        if let Some(permissions) = predecessor_permissions.as_ref() {
            file.set_permissions(permissions.clone())?;
        }
        file.sync_all()
    })();
    drop(file);
    if let Err(error) = staged {
        remove_unpublished_temp(&temp);
        return Err(error.into());
    }

    let displaced = match publish_retaining_predecessor(&temp, path, staged_digest) {
        Ok(displaced) => displaced,
        Err(PublicationFailure::Safe(error)) => {
            if let Err(cleanup_error) = remove_transaction_temp(&temp) {
                return Err(CompareExchangeError::PublicationUncertain {
                    message: format!(
                        "publication failed ({error}) and staged successor cleanup failed: {cleanup_error}"
                    ),
                    recovery_paths: vec![temp],
                });
            }
            retire_publication_recovery(path).map_err(PublicationFailure::into_compare_exchange)?;
            if matches!(expected, ExpectedContent::Missing)
                && error.kind() == io::ErrorKind::AlreadyExists
            {
                return Err(CompareExchangeError::Conflict {
                    expected,
                    actual: current_digest(path)?,
                });
            }
            return Err(error.into());
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => {
            return Err(error.into_compare_exchange());
        }
    };

    let actual = match displaced.as_deref() {
        Some(predecessor) => match hash_file_bounded(predecessor) {
            Ok(digest) => Some(digest),
            Err(error) => {
                return rollback_after_publication_failure(
                    path,
                    displaced,
                    staged_digest,
                    format!("displaced predecessor could not be verified: {error}"),
                );
            }
        },
        None => None,
    };
    let matches = match expected {
        ExpectedContent::Missing => actual.is_none(),
        ExpectedContent::Digest(digest) => actual == Some(digest),
    };
    if !matches {
        return rollback_conflict(path, displaced, expected, actual, staged_digest);
    }

    if let Err(error) = sync_parent_directory(parent) {
        return rollback_after_publication_failure(
            path,
            displaced,
            staged_digest,
            format!("containing directory could not be synchronized: {error}"),
        );
    }

    match current_digest(path) {
        Ok(Some(digest)) if digest == staged_digest => {}
        Ok(actual_successor) => {
            return Err(CompareExchangeError::PublicationUncertain {
                message: format!(
                    "published target changed before predecessor cleanup (expected staged digest {staged_digest:02x?}, found {actual_successor:02x?})"
                ),
                recovery_paths: displaced.into_iter().collect(),
            });
        }
        Err(error) => {
            return Err(CompareExchangeError::PublicationUncertain {
                message: format!(
                    "published target could not be verified before predecessor cleanup: {error}"
                ),
                recovery_paths: displaced.into_iter().collect(),
            });
        }
    }

    #[cfg(unix)]
    if owner_only {
        use std::os::unix::fs::PermissionsExt as _;

        let mode = std::fs::symlink_metadata(path)?.permissions().mode() & 0o777;
        if mode != 0o600 {
            return rollback_after_publication_failure(
                path,
                displaced,
                staged_digest,
                format!("owner-only publication produced mode {mode:#05o} instead of 0o600"),
            );
        }
    }

    #[cfg(not(unix))]
    let _ = owner_only;

    #[cfg(windows)]
    if let Err(error) =
        sync_windows_published_successor(path, staged_digest, predecessor_permissions.as_ref())
    {
        return rollback_after_publication_failure(
            path,
            displaced,
            staged_digest,
            format!("published target file could not be synchronized: {error}"),
        );
    }

    #[cfg(windows)]
    if !matches!(current_digest(path), Ok(Some(digest)) if digest == staged_digest) {
        return Err(CompareExchangeError::PublicationUncertain {
            message: "published target changed while its handle was being synchronized".to_owned(),
            recovery_paths: displaced.into_iter().collect(),
        });
    }

    if let Err(error) = resolve_publication_recovery(
        path,
        displaced.as_deref(),
        staged_digest,
        PublicationResolution::Committed,
    ) {
        return Err(error.into_compare_exchange());
    }

    if let Some(predecessor) = displaced {
        if let Err(error) = std::fs::remove_file(&predecessor) {
            return Err(CompareExchangeError::PublicationUncertain {
                message: format!(
                    "successor is durable, but displaced predecessor cleanup failed: {error}"
                ),
                recovery_paths: vec![predecessor],
            });
        }
        if let Err(error) = sync_parent_directory(parent) {
            return Err(CompareExchangeError::PublicationUncertain {
                message: format!(
                    "successor is durable, but predecessor cleanup could not be synchronized: {error}"
                ),
                recovery_paths: Vec::new(),
            });
        }
    }
    retire_publication_recovery(path).map_err(PublicationFailure::into_compare_exchange)?;
    Ok(())
}

fn rollback_conflict(
    target: &Path,
    displaced: Option<PathBuf>,
    expected: ExpectedContent,
    actual: Option<[u8; 32]>,
    staged_digest: [u8; 32],
) -> Result<(), CompareExchangeError> {
    let Some(predecessor) = displaced else {
        return rollback_created_target(target, expected, actual, staged_digest);
    };
    match rollback_retaining_successor(target, &predecessor) {
        Ok(successor) => {
            let restored = match current_digest(target) {
                Ok(restored) => restored,
                Err(error) => {
                    return Err(CompareExchangeError::PublicationUncertain {
                        message: format!(
                            "conflict rollback completed, but the restored target could not be verified: {error}; target itself and captured successor were retained"
                        ),
                        recovery_paths: vec![target.to_path_buf(), successor],
                    });
                }
            };
            if restored != actual {
                return Err(CompareExchangeError::PublicationUncertain {
                    message: "conflict rollback completed, but the restored predecessor no longer matches the displaced bytes".to_owned(),
                    recovery_paths: vec![target.to_path_buf(), successor],
                });
            }
            match hash_file_bounded(&successor) {
                Ok(digest) if digest == staged_digest => {}
                Ok(digest) => {
                    return Err(CompareExchangeError::PublicationUncertain {
                        message: format!(
                            "conflict rollback captured bytes modified by an external writer (expected staged successor {staged_digest:02x?}, found {digest:02x?}); no evidence was deleted"
                        ),
                        recovery_paths: vec![target.to_path_buf(), successor],
                    });
                }
                Err(error) => {
                    return Err(CompareExchangeError::PublicationUncertain {
                        message: format!(
                            "conflict rollback captured successor could not be verified: {error}; no evidence was deleted"
                        ),
                        recovery_paths: vec![target.to_path_buf(), successor],
                    });
                }
            }
            if let Err(error) = sync_parent_of(target) {
                return Err(CompareExchangeError::PublicationUncertain {
                    message: format!("conflict rollback could not be synchronized: {error}"),
                    recovery_paths: vec![target.to_path_buf(), successor],
                });
            }
            if let Err(error) = resolve_publication_recovery(
                target,
                Some(&successor),
                staged_digest,
                PublicationResolution::Aborted,
            ) {
                return Err(rollback_failure(
                    "conflict rollback restored the predecessor, but the recovery journal could not be resolved",
                    error,
                    vec![target.to_path_buf(), successor],
                ));
            }
            if let Err(error) = remove_transaction_temp(&successor) {
                return Err(CompareExchangeError::PublicationUncertain {
                    message: format!(
                        "conflict rollback was resolved, but captured successor cleanup failed: {error}"
                    ),
                    recovery_paths: vec![successor],
                });
            }
            retire_publication_recovery(target)
                .map_err(PublicationFailure::into_compare_exchange)?;
            Err(CompareExchangeError::Conflict { expected, actual })
        }
        Err(error) => Err(rollback_failure(
            "external-change conflict was detected, but rollback failed",
            error,
            vec![target.to_path_buf(), predecessor],
        )),
    }
}

fn rollback_created_target(
    target: &Path,
    expected: ExpectedContent,
    actual: Option<[u8; 32]>,
    staged_digest: [u8; 32],
) -> Result<(), CompareExchangeError> {
    let recovery = unique_sibling(target);
    match move_no_replace_checked(target, &recovery, staged_digest) {
        Ok(()) => {
            match hash_file_bounded(&recovery) {
                Ok(digest) if digest == staged_digest => {}
                Ok(_) | Err(_) => {
                    return Err(CompareExchangeError::PublicationUncertain {
                        message: "new-target rollback captured bytes that could not be proven to be the staged successor".to_owned(),
                        recovery_paths: vec![recovery],
                    });
                }
            }
            if let Err(error) = sync_parent_of(target) {
                return Err(CompareExchangeError::PublicationUncertain {
                    message: format!(
                        "new-target conflict rollback could not be synchronized: {error}"
                    ),
                    recovery_paths: vec![recovery],
                });
            }
            remove_unpublished_temp(&recovery);
            Err(CompareExchangeError::Conflict { expected, actual })
        }
        Err(error) => Err(rollback_failure(
            "new-target conflict rollback failed",
            error,
            vec![target.to_path_buf(), recovery],
        )),
    }
}

fn rollback_after_publication_failure(
    target: &Path,
    displaced: Option<PathBuf>,
    staged_digest: [u8; 32],
    reason: String,
) -> Result<(), CompareExchangeError> {
    match displaced {
        Some(predecessor) => match rollback_retaining_successor(target, &predecessor) {
            Ok(successor) => {
                match hash_file_bounded(&successor) {
                    Ok(digest) if digest == staged_digest => {}
                    Ok(digest) => {
                        return Err(CompareExchangeError::PublicationUncertain {
                            message: format!(
                                "{reason}; rollback captured externally modified bytes (expected {staged_digest:02x?}, found {digest:02x?})"
                            ),
                            recovery_paths: vec![target.to_path_buf(), successor],
                        });
                    }
                    Err(error) => {
                        return Err(CompareExchangeError::PublicationUncertain {
                            message: format!(
                                "{reason}; rollback successor verification failed: {error}"
                            ),
                            recovery_paths: vec![target.to_path_buf(), successor],
                        });
                    }
                }
                match sync_parent_of(target) {
                    Ok(()) => {
                        if let Err(error) = resolve_publication_recovery(
                            target,
                            Some(&successor),
                            staged_digest,
                            PublicationResolution::Aborted,
                        ) {
                            return Err(rollback_failure(
                                &format!(
                                    "{reason}; predecessor was rolled back but the recovery journal could not be resolved"
                                ),
                                error,
                                vec![target.to_path_buf(), successor],
                            ));
                        }
                        if let Err(error) = remove_transaction_temp(&successor) {
                            return Err(CompareExchangeError::PublicationUncertain {
                                message: format!(
                                    "{reason}; predecessor was rolled back, but captured successor cleanup failed: {error}"
                                ),
                                recovery_paths: vec![successor],
                            });
                        }
                        retire_publication_recovery(target)
                            .map_err(PublicationFailure::into_compare_exchange)?;
                        Err(CompareExchangeError::PublicationUncertain {
                            message: format!("{reason}; predecessor was rolled back"),
                            recovery_paths: Vec::new(),
                        })
                    }
                    Err(error) => Err(CompareExchangeError::PublicationUncertain {
                        message: format!(
                            "{reason}; predecessor was rolled back but rollback sync failed: {error}"
                        ),
                        recovery_paths: vec![target.to_path_buf(), successor],
                    }),
                }
            }
            Err(error) => Err(rollback_failure(
                &format!("{reason}; predecessor rollback failed"),
                error,
                vec![target.to_path_buf(), predecessor],
            )),
        },
        None => {
            let recovery = unique_sibling(target);
            match move_no_replace_checked(target, &recovery, staged_digest) {
                Ok(()) => Err(CompareExchangeError::PublicationUncertain {
                    message: format!("{reason}; newly created target was removed from publication"),
                    recovery_paths: vec![recovery],
                }),
                Err(error) => Err(rollback_failure(
                    &format!("{reason}; new-target rollback failed"),
                    error,
                    vec![target.to_path_buf(), recovery],
                )),
            }
        }
    }
}

fn rollback_failure(
    context: &str,
    failure: PublicationFailure,
    fallback_paths: Vec<PathBuf>,
) -> CompareExchangeError {
    match failure {
        PublicationFailure::Safe(error) => CompareExchangeError::PublicationUncertain {
            message: format!("{context}: {error}"),
            recovery_paths: fallback_paths,
        },
        PublicationFailure::Uncertain {
            message,
            recovery_paths,
        } => CompareExchangeError::PublicationUncertain {
            message: format!("{context}: {message}"),
            recovery_paths: merge_recovery_paths(fallback_paths, recovery_paths),
        },
    }
}

#[cfg(windows)]
fn resolve_publication_recovery(
    target: &Path,
    displaced: Option<&Path>,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    if displaced.is_none() {
        return Ok(());
    }
    resolve_windows_recovery_slot(target, successor_digest, resolution)
}

#[cfg(unix)]
fn resolve_publication_recovery(
    target: &Path,
    displaced: Option<&Path>,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    resolve_unix_recovery_slot(target, displaced, successor_digest, resolution)
}

#[cfg(windows)]
fn retire_publication_recovery(target: &Path) -> Result<(), PublicationFailure> {
    retire_settled_windows_recovery(target)
}

#[cfg(unix)]
fn retire_publication_recovery(target: &Path) -> Result<(), PublicationFailure> {
    retire_settled_unix_recovery(target)
}

fn current_digest(path: &Path) -> io::Result<Option<[u8; 32]>> {
    match hash_file_bounded(path) {
        Ok(digest) => Ok(Some(digest)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn hash_file_bounded(path: &Path) -> io::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    hash_open_file_bounded(&mut file)
}

fn hash_open_file_bounded(file: &mut File) -> io::Result<[u8; 32]> {
    file.seek(std::io::SeekFrom::Start(0))?;
    let mut hasher = Sha256::new();
    let mut total = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total.saturating_add(read as u64);
        if total > MAX_HASHED_FILE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::FileTooLarge,
                format!(
                    "file exceeds the {} byte project publication limit",
                    MAX_HASHED_FILE_BYTES
                ),
            ));
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher.finalize().into())
}

#[cfg(windows)]
fn sync_windows_published_successor(
    path: &Path,
    expected_digest: [u8; 32],
    permissions: Option<&std::fs::Permissions>,
) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let actual = hash_open_file_bounded(&mut file)?;
    if actual != expected_digest {
        return Err(io::Error::other(format!(
            "published handle for '{}' does not contain the staged successor",
            path.display()
        )));
    }
    file.sync_all()?;
    if let Some(permissions) = permissions {
        file.set_permissions(permissions.clone())?;
        file.sync_all()?;
    }
    Ok(())
}

/// Copy `source` to `target` through the same durable publication boundary.
/// An existing target is never truncated in place.
pub(crate) fn atomic_copy(source: &Path, target: &Path) -> io::Result<()> {
    let mut source = File::open(source)?;
    atomic_write_with(target, |target| {
        std::io::copy(&mut source, target)?;
        Ok(())
    })
}

/// Persist a successful metadata-only transaction such as recovery deletion.
pub(crate) fn sync_parent_of(path: &Path) -> io::Result<()> {
    sync_parent_directory(parent_directory(path))
}

fn parent_directory(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn unique_sibling(path: &Path) -> PathBuf {
    let mut value = path.as_os_str().to_owned();
    value.push(format!(".{}.tmp", Uuid::new_v4()));
    PathBuf::from(value)
}

fn lease_path(path: &Path) -> PathBuf {
    let binding = lease_target_binding(path);
    let mut encoded = String::with_capacity(binding.len() * 2);
    for byte in binding {
        use std::fmt::Write as _;
        let _ = write!(encoded, "{byte:02x}");
    }
    parent_directory(path).join(format!(".rspice-lock-v2-{encoded}.lock"))
}

fn lease_target_binding(path: &Path) -> [u8; 32] {
    let absolute = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-destination-lease\0v2\0");
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        hasher.update(absolute.as_os_str().as_bytes());
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        for unit in absolute.as_os_str().encode_wide() {
            hasher.update(unit.to_le_bytes());
        }
    }
    hasher.finalize().into()
}

fn encode_lease_header(target_binding: [u8; 32], lease_id: Uuid) -> [u8; LEASE_HEADER_LEN] {
    let mut header = [0_u8; LEASE_HEADER_LEN];
    header[..16].copy_from_slice(&LEASE_HEADER_MAGIC);
    header[16..48].copy_from_slice(&target_binding);
    header[48..64].copy_from_slice(lease_id.as_bytes());
    let checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-destination-lease-header\0v2\0")
        .chain_update(&header[..LEASE_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    header[LEASE_HEADER_PREFIX_LEN..].copy_from_slice(&checksum);
    header
}

fn install_lease_if_missing(path: &Path, target_binding: [u8; 32]) -> io::Result<()> {
    match path.symlink_metadata() {
        Ok(_) => return Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    let header = encode_lease_header(target_binding, Uuid::new_v4());
    let (temporary, mut file) = create_unique_sibling(path)?;
    let staged = (|| -> io::Result<()> {
        file.write_all(&header)?;
        file.flush()?;
        file.sync_all()
    })();
    drop(file);
    if let Err(error) = staged {
        remove_unpublished_temp(&temporary);
        return Err(error);
    }

    match move_no_replace_raw(&temporary, path) {
        Ok(()) => sync_parent_of(path),
        Err(error) => {
            let destination_exists = match path.symlink_metadata() {
                Ok(_) => true,
                Err(observation) if observation.kind() == io::ErrorKind::NotFound => false,
                Err(observation) => {
                    remove_unpublished_temp(&temporary);
                    return Err(io::Error::new(
                        observation.kind(),
                        format!(
                            "lease installation failed ({error}) and destination existence could not be observed: {observation}"
                        ),
                    ));
                }
            };
            remove_unpublished_temp(&temporary);
            if destination_exists {
                Ok(())
            } else {
                Err(error)
            }
        }
    }
}

fn validate_open_lease(path: &Path, file: &mut File, target_binding: [u8; 32]) -> io::Result<()> {
    validate_lease_file_identity(path, file)?;
    if file.metadata()?.len() != LEASE_HEADER_LEN as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "destination lease '{}' is not an RSpice lease file",
                path.display()
            ),
        ));
    }
    let mut header = [0_u8; LEASE_HEADER_LEN];
    file.seek(std::io::SeekFrom::Start(0))?;
    file.read_exact(&mut header)?;
    let expected_checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-destination-lease-header\0v2\0")
        .chain_update(&header[..LEASE_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    if header[..16] != LEASE_HEADER_MAGIC
        || header[16..48] != target_binding
        || header[48..64] == [0_u8; 16]
        || header[LEASE_HEADER_PREFIX_LEN..] != expected_checksum
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "destination lease '{}' has foreign or corrupted ownership metadata",
                path.display()
            ),
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn open_lease_nofollow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt as _;

    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
        .open(path)
}

#[cfg(windows)]
fn open_lease_nofollow(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt as _;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ, FILE_SHARE_WRITE,
    };

    OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(unix)]
fn validate_lease_file_identity(path: &Path, file: &File) -> io::Result<()> {
    use std::os::unix::fs::MetadataExt as _;

    let opened = file.metadata()?;
    let named = std::fs::symlink_metadata(path)?;
    if !opened.file_type().is_file()
        || !named.file_type().is_file()
        || opened.nlink() != 1
        || named.nlink() != 1
        || opened.dev() != named.dev()
        || opened.ino() != named.ino()
    {
        return Err(io::Error::other(format!(
            "destination lease '{}' is not one stable, single-link regular RSpice lease file",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(windows)]
fn validate_lease_file_identity(path: &Path, file: &File) -> io::Result<()> {
    use std::os::windows::fs::FileTypeExt as _;

    let opened_metadata = file.metadata()?;
    if !opened_metadata.file_type().is_file()
        || opened_metadata.file_type().is_symlink()
        || opened_metadata.file_type().is_symlink_dir()
    {
        return Err(io::Error::other(format!(
            "destination lease '{}' is not a regular file",
            path.display()
        )));
    }
    let opened = windows_file_identity(file)?;
    if opened.2 != 1 {
        return Err(io::Error::other(format!(
            "destination lease '{}' has {} hard links; exactly one is required",
            path.display(),
            opened.2
        )));
    }
    let named_file = open_lease_nofollow(path)?;
    let named = windows_file_identity(&named_file)?;
    if opened != named {
        return Err(io::Error::other(format!(
            "destination lease '{}' changed while it was being acquired",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(windows)]
fn windows_file_identity(file: &File) -> io::Result<(u32, u64, u32)> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };

    let mut information = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::zeroed();
    let result = unsafe {
        GetFileInformationByHandle(file.as_raw_handle() as *mut _, information.as_mut_ptr())
    };
    if result == 0 {
        return Err(io::Error::last_os_error());
    }
    let information = unsafe { information.assume_init() };
    let index =
        (u64::from(information.nFileIndexHigh) << 32) | u64::from(information.nFileIndexLow);
    Ok((
        information.dwVolumeSerialNumber,
        index,
        information.nNumberOfLinks,
    ))
}

fn recovery_slot_paths(path: &Path) -> [PathBuf; 2] {
    [
        path_with_suffix(path, RECOVERY_SLOT_A_SUFFIX),
        path_with_suffix(path, RECOVERY_SLOT_B_SUFFIX),
    ]
}

fn recovery_backup_path(path: &Path, backup_id: Uuid) -> PathBuf {
    path_with_suffix(path, &format!("{RECOVERY_BACKUP_SUFFIX}{backup_id}"))
}

#[cfg(windows)]
fn recovery_rollback_path(path: &Path, backup_id: Uuid) -> PathBuf {
    path_with_suffix(
        path,
        &format!("{RECOVERY_BACKUP_SUFFIX}{backup_id}.rollback"),
    )
}

#[cfg(windows)]
fn recovery_retired_path(slot: &Path) -> PathBuf {
    path_with_suffix(slot, RECOVERY_RETIRED_SUFFIX)
}

fn path_with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut value = path.as_os_str().to_os_string();
    value.push(suffix);
    PathBuf::from(value)
}

#[cfg(unix)]
fn try_lock_exclusive(file: &File) -> io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::NonBlockingLockExclusive)
        .map_err(io::Error::from)
}

#[cfg(unix)]
fn unlock_file(file: &File) -> io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::Unlock).map_err(io::Error::from)
}

#[cfg(windows)]
fn try_lock_exclusive(file: &File) -> io::Result<()> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::{
        LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY, LockFileEx,
    };
    use windows_sys::Win32::System::IO::OVERLAPPED;

    let mut overlapped = unsafe { std::mem::zeroed::<OVERLAPPED>() };
    let locked = unsafe {
        LockFileEx(
            file.as_raw_handle() as *mut _,
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            1,
            0,
            &mut overlapped,
        )
    };
    if locked == 0 {
        let error = io::Error::last_os_error();
        // ERROR_LOCK_VIOLATION (33) and ERROR_IO_PENDING (997) both mean a
        // live owner won the non-blocking lease attempt.
        if matches!(error.raw_os_error(), Some(33 | 997)) {
            Err(io::Error::new(io::ErrorKind::WouldBlock, error))
        } else {
            Err(error)
        }
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn unlock_file(file: &File) -> io::Result<()> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::UnlockFileEx;
    use windows_sys::Win32::System::IO::OVERLAPPED;

    let mut overlapped = unsafe { std::mem::zeroed::<OVERLAPPED>() };
    let unlocked =
        unsafe { UnlockFileEx(file.as_raw_handle() as *mut _, 0, 1, 0, &mut overlapped) };
    if unlocked == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn create_unique_sibling(path: &Path) -> io::Result<(PathBuf, File)> {
    for _ in 0..UNIQUE_TEMP_ATTEMPTS {
        let candidate = unique_sibling(path);
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt as _;
            use windows_sys::Win32::Storage::FileSystem::{
                FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
            };
            options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE);
        }
        match options.open(&candidate) {
            Ok(file) => return Ok((candidate, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!(
            "could not allocate a unique sibling transaction for '{}' after {UNIQUE_TEMP_ATTEMPTS} attempts",
            path.display()
        ),
    ))
}

fn remove_unpublished_temp(path: &Path) {
    let _ = remove_transaction_temp(path);
}

fn remove_transaction_temp(path: &Path) -> io::Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => sync_parent_of(path),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn observe_file_role(
    path: &Path,
    replaced_digest: [u8; 32],
    replacement_digest: [u8; 32],
) -> ObservedFileRole {
    match current_digest(path) {
        Ok(None) => ObservedFileRole::Missing,
        Ok(Some(digest)) if digest == replaced_digest && digest == replacement_digest => {
            ObservedFileRole::Both
        }
        Ok(Some(digest)) if digest == replaced_digest => ObservedFileRole::Replaced,
        Ok(Some(digest)) if digest == replacement_digest => ObservedFileRole::Replacement,
        Ok(Some(_)) => ObservedFileRole::Other,
        Err(_) => ObservedFileRole::Unreadable,
    }
}

#[cfg(windows)]
fn observe_replace(
    replaced: &Path,
    replacement: &Path,
    backup: &Path,
    replaced_digest: [u8; 32],
    replacement_digest: [u8; 32],
) -> ReplaceObservation {
    ReplaceObservation {
        replaced: observe_file_role(replaced, replaced_digest, replacement_digest),
        replacement: observe_file_role(replacement, replaced_digest, replacement_digest),
        backup: observe_file_role(backup, replaced_digest, replacement_digest),
    }
}

fn recovery_paths(paths: &[&Path]) -> Vec<PathBuf> {
    let mut recovery_paths = Vec::new();
    for path in paths {
        if !matches!(path.try_exists(), Ok(false))
            && !recovery_paths.iter().any(|candidate| candidate == path)
        {
            recovery_paths.push((*path).to_path_buf());
        }
    }
    recovery_paths
}

fn merge_recovery_paths(
    mut left: Vec<PathBuf>,
    right: impl IntoIterator<Item = PathBuf>,
) -> Vec<PathBuf> {
    for path in right {
        if !left.contains(&path) {
            left.push(path);
        }
    }
    left
}

fn recovery_step_failure(
    context: &str,
    failure: PublicationFailure,
    paths: &[&Path],
) -> PublicationFailure {
    let fallback_paths = recovery_paths(paths);
    match failure {
        PublicationFailure::Safe(error) => PublicationFailure::Uncertain {
            message: format!("{context}: {error}"),
            recovery_paths: fallback_paths,
        },
        PublicationFailure::Uncertain {
            message,
            recovery_paths,
        } => PublicationFailure::Uncertain {
            message: format!("{context}: {message}"),
            recovery_paths: merge_recovery_paths(fallback_paths, recovery_paths),
        },
    }
}

fn move_no_replace_checked(
    from: &Path,
    to: &Path,
    expected_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    let raw_result = move_no_replace_raw(from, to);
    let from_digest = current_digest(from);
    let to_digest = current_digest(to);

    if matches!(from_digest, Ok(None))
        && matches!(to_digest, Ok(Some(digest)) if digest == expected_digest)
    {
        return Ok(());
    }

    if matches!(from_digest, Ok(Some(digest)) if digest == expected_digest) {
        let error = raw_result.err().unwrap_or_else(|| {
            io::Error::other(format!(
                "move from '{}' to '{}' reported success without consuming the source",
                from.display(),
                to.display()
            ))
        });
        return Err(PublicationFailure::Safe(error));
    }

    let raw_message = match raw_result {
        Ok(()) => "the platform call reported success".to_owned(),
        Err(error) => error.to_string(),
    };
    Err(PublicationFailure::Uncertain {
        message: format!(
            "move from '{}' to '{}' has an unverified postcondition ({raw_message}); source digest: {from_digest:?}; destination digest: {to_digest:?}",
            from.display(),
            to.display()
        ),
        recovery_paths: recovery_paths(&[from, to]),
    })
}

struct RecoveryHeaderInput {
    generation: u64,
    transaction: Uuid,
    backup_id: Uuid,
    target_binding: [u8; 32],
    before_present: bool,
    predecessor_len: u64,
    successor_len: u64,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
    predecessor_read_only: bool,
}

fn encode_recovery_header(input: RecoveryHeaderInput) -> [u8; RECOVERY_HEADER_LEN] {
    let RecoveryHeaderInput {
        generation,
        transaction,
        backup_id,
        target_binding,
        before_present,
        predecessor_len,
        successor_len,
        predecessor_digest,
        successor_digest,
        predecessor_read_only,
    } = input;
    let mut header = [0_u8; RECOVERY_HEADER_LEN];
    header[..16].copy_from_slice(&RECOVERY_HEADER_MAGIC);
    header[16..24].copy_from_slice(&generation.to_le_bytes());
    header[24..40].copy_from_slice(transaction.as_bytes());
    header[40..56].copy_from_slice(backup_id.as_bytes());
    header[56..88].copy_from_slice(&target_binding);
    header[88] = (u8::from(before_present) * RECOVERY_FLAG_BEFORE_PRESENT)
        | (u8::from(predecessor_read_only) * RECOVERY_FLAG_READ_ONLY);
    header[96..104].copy_from_slice(&predecessor_len.to_le_bytes());
    header[104..112].copy_from_slice(&successor_len.to_le_bytes());
    header[112..144].copy_from_slice(&predecessor_digest);
    header[144..176].copy_from_slice(&successor_digest);
    let checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-header\0v1\0")
        .chain_update(&header[..RECOVERY_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    header[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN].copy_from_slice(&checksum);
    header
}

fn encode_recovery_resolution(
    record: &RecoveryRecord,
    resolution: PublicationResolution,
) -> [u8; RECOVERY_RESOLUTION_LEN] {
    let mut trailer = [0_u8; RECOVERY_RESOLUTION_LEN];
    trailer[..16].copy_from_slice(&RECOVERY_RESOLUTION_MAGIC);
    trailer[16..24].copy_from_slice(&record.generation.to_le_bytes());
    trailer[24..40].copy_from_slice(record.transaction.as_bytes());
    trailer[40] = resolution.marker();
    let checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-resolution\0v1\0")
        .chain_update(&trailer[..RECOVERY_RESOLUTION_PREFIX_LEN])
        .chain_update(record.header_checksum)
        .finalize()
        .into();
    trailer[RECOVERY_RESOLUTION_PREFIX_LEN..RECOVERY_RESOLUTION_LEN].copy_from_slice(&checksum);
    trailer
}

fn decode_recovery_resolution(
    trailer: &[u8; RECOVERY_RESOLUTION_LEN],
    generation: u64,
    transaction: Uuid,
    header_checksum: [u8; 32],
) -> Option<PublicationResolution> {
    if trailer[..16] != RECOVERY_RESOLUTION_MAGIC
        || trailer[16..24] != generation.to_le_bytes()
        || trailer[24..40] != transaction.as_bytes()[..]
        || trailer[41..RECOVERY_RESOLUTION_PREFIX_LEN]
            .iter()
            .any(|byte| *byte != 0)
    {
        return None;
    }
    let resolution = PublicationResolution::from_marker(trailer[40])?;
    let expected: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-resolution\0v1\0")
        .chain_update(&trailer[..RECOVERY_RESOLUTION_PREFIX_LEN])
        .chain_update(header_checksum)
        .finalize()
        .into();
    (trailer[RECOVERY_RESOLUTION_PREFIX_LEN..] == expected).then_some(resolution)
}

fn hash_exact_section(file: &mut File, len: u64) -> io::Result<[u8; 32]> {
    let mut remaining = len;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    while remaining != 0 {
        let wanted = usize::try_from(remaining.min(buffer.len() as u64))
            .expect("bounded read length fits usize");
        let read = file.read(&mut buffer[..wanted])?;
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "recovery bundle image ended before its declared length",
            ));
        }
        hasher.update(&buffer[..read]);
        remaining -= read as u64;
    }
    Ok(hasher.finalize().into())
}

#[cfg(not(unix))]
fn sync_parent_directory(_parent: &Path) -> io::Result<()> {
    // Windows has no portable directory-fsync primitive. Staged file data is
    // flushed before publication; new-name moves use MOVEFILE_WRITE_THROUGH;
    // replacement calls retain a backup and are verified by content before
    // that evidence is released. Other supported non-Unix targets currently
    // use this same native boundary.
    Ok(())
}

mod recovery_journal;

#[cfg(unix)]
mod unix_recovery;

use recovery_journal::*;

#[cfg(unix)]
use unix_recovery::*;

#[cfg(test)]
mod tests;
